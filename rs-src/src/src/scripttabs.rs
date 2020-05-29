use ::libc;
extern "C" {
    // If this is defined we save out the compiled scripts
    // Initialise the script library
    #[no_mangle]
    fn scriptInitialise(psInit: *mut EVENT_INIT) -> BOOL;
    // Shutdown the script library
    #[no_mangle]
    fn scriptShutDown();
    /* **********************************************************************************
 *
 * Compiler setup functions
 */
    /* Set the type table */
    #[no_mangle]
    fn scriptSetTypeTab(psTypeTab: *mut TYPE_SYMBOL);
    /* Set the function table */
    #[no_mangle]
    fn scriptSetFuncTab(psFuncTab: *mut FUNC_SYMBOL);
    /* Set the external variable table */
    #[no_mangle]
    fn scriptSetExternalTab(psExtTab: *mut VAR_SYMBOL);
    /* Set the object variable table */
    #[no_mangle]
    fn scriptSetObjectTab(psObjTab: *mut VAR_SYMBOL);
    /* Set the constant table */
    #[no_mangle]
    fn scriptSetConstTab(psConstTab: *mut CONST_SYMBOL);
    /* Set the callback table */
    #[no_mangle]
    fn scriptSetCallbackTab(psCallTab: *mut CALLBACK_SYMBOL);
    /* Set the type equivalence table */
    #[no_mangle]
    fn scriptSetTypeEquiv(psTypeTab: *mut TYPE_EQUIV);
    // Initialise the create/release function array - specify the maximum value type
    #[no_mangle]
    fn eventInitValueFuncs(maxType: SDWORD) -> BOOL;
    // Add a new value create function
    #[no_mangle]
    fn eventAddValueCreate(type_0: INTERP_TYPE, create: VAL_CREATE_FUNC)
     -> BOOL;
    // Add a new value release function
    #[no_mangle]
    fn eventAddValueRelease(type_0: INTERP_TYPE, release: VAL_RELEASE_FUNC)
     -> BOOL;
    /* Push a value onto the stack without using a value structure */
    /* **********************************************************************************
 *
 * Script library instinct functions
 *
 * These would be declared in the function symbol array:
 *
 *	{ "traceOn",				interpTraceOn,			VAL_VOID,
 *		0, { VAL_VOID } },
 *	{ "traceOff",				interpTraceOff,			VAL_VOID,
 *		0, { VAL_VOID } },
 *	{ "setEventTrigger",		eventSetTrigger,		VAL_VOID,
 *		2, { VAL_EVENT, VAL_TRIGGER } },
 *	{ "eventTraceLevel",		eventSetTraceLevel,		VAL_VOID,
 *		1, { VAL_INT } },
 *
 */
    /* Instinct function to turn on tracing */
    /* Instinct function to turn off tracing */
    // Change the trigger assigned to an event
// This is an instinct function that takes a VAL_EVENT and VAL_TRIGGER as parameters
    // set the event tracing level
//   0 - no tracing
//   1 - only fired triggers
//   2 - added and fired triggers
//   3 - as 2 but show tested but not fired triggers as well
    #[no_mangle]
    fn eventSetTraceLevel() -> BOOL;
    #[no_mangle]
    fn eventSetTrigger() -> BOOL;
    #[no_mangle]
    fn interpTraceOff() -> BOOL;
    #[no_mangle]
    fn interpTraceOn() -> BOOL;
    // Check for any player object being within a certain range of a position
    #[no_mangle]
    fn scrObjectInRange() -> BOOL;
    // Check for a droid being within a certain range of a position
    #[no_mangle]
    fn scrDroidInRange() -> BOOL;
    // Check for a struct being within a certain range of a position
    #[no_mangle]
    fn scrStructInRange() -> BOOL;
    // return power of a player.
    #[no_mangle]
    fn scrPlayerPower() -> BOOL;
    // Check for any player object being within a certain area
    #[no_mangle]
    fn scrObjectInArea() -> BOOL;
    // Check for a droid being within a certain area
    #[no_mangle]
    fn scrDroidInArea() -> BOOL;
    // Check for a struct being within a certain Area of a position
    #[no_mangle]
    fn scrStructInArea() -> BOOL;
    // as above, but only visible structures.
    #[no_mangle]
    fn scrSeenStructInArea() -> BOOL;
    // Check for a players structures but no walls being within a certain area
    #[no_mangle]
    fn scrStructButNoWallsInArea() -> BOOL;
    // Count the number of player objects within a certain area
    #[no_mangle]
    fn scrNumObjectsInArea() -> BOOL;
    // Count the number of player droids within a certain area
    #[no_mangle]
    fn scrNumDroidsInArea() -> BOOL;
    // Count the number of player structures within a certain area
    #[no_mangle]
    fn scrNumStructsInArea() -> BOOL;
    // Count the number of player structures but not walls within a certain area
    #[no_mangle]
    fn scrNumStructsButNotWallsInArea() -> BOOL;
    // Count the number of structures in an area of a certain type
    #[no_mangle]
    fn scrNumStructsByTypeInArea() -> BOOL;
    // Check for a droid having seen a certain object
    #[no_mangle]
    fn scrDroidHasSeen() -> BOOL;
    // Enable a component to be researched
    #[no_mangle]
    fn scrEnableComponent() -> BOOL;
    // Make a component available
    #[no_mangle]
    fn scrMakeComponentAvailable() -> BOOL;
    //Enable a structure type to be built
    #[no_mangle]
    fn scrEnableStructure() -> BOOL;
    // true if structure is available.
    #[no_mangle]
    fn scrIsStructureAvailable() -> BOOL;
    // Build a droid
    #[no_mangle]
    fn scrAddDroid() -> BOOL;
    // Build a droid
    #[no_mangle]
    fn scrAddDroidToMissionList() -> BOOL;
    //builds a droid in the specified factory//
    #[no_mangle]
    fn scrBuildDroid() -> BOOL;
    //check for a building to have been destroyed
    #[no_mangle]
    fn scrBuildingDestroyed() -> BOOL;
    // Add a reticule button to the interface
    #[no_mangle]
    fn scrAddReticuleButton() -> BOOL;
    //Remove a reticule button from the interface
    #[no_mangle]
    fn scrRemoveReticuleButton() -> BOOL;
    // add a message to the Intelligence Display
    #[no_mangle]
    fn scrAddMessage() -> BOOL;
    // add a tutorial message to the Intelligence Display
//extern BOOL scrAddTutorialMessage(void);
    //make the droid with the matching id the currently selected droid
    #[no_mangle]
    fn scrSelectDroidByID() -> BOOL;
    // for a specified player, set the assembly point droids go to when built
    #[no_mangle]
    fn scrSetAssemblyPoint() -> BOOL;
    // test for structure is idle or not
    #[no_mangle]
    fn scrStructureIdle() -> BOOL;
    // sends a players droids to a location to attack
    #[no_mangle]
    fn scrAttackLocation() -> BOOL;
    // enumerate features;
    #[no_mangle]
    fn scrInitGetFeature() -> BOOL;
    #[no_mangle]
    fn scrGetFeature() -> BOOL;
    //Add a feature
    #[no_mangle]
    fn scrAddFeature() -> BOOL;
    //Destroy a feature
    #[no_mangle]
    fn scrDestroyFeature() -> BOOL;
    //Add a structure
    #[no_mangle]
    fn scrAddStructure() -> BOOL;
    //Destroy a structure
    #[no_mangle]
    fn scrDestroyStructure() -> BOOL;
    // enumerate structures
    #[no_mangle]
    fn scrInitEnumStruct() -> BOOL;
    #[no_mangle]
    fn scrEnumStruct() -> BOOL;
    /*looks to see if a structure (specified by type) exists */
    #[no_mangle]
    fn scrStructureBeingBuilt() -> BOOL;
    /* almost the same as above, but only for a specific struct*/
// pc multiplayer only for now.
    #[no_mangle]
    fn scrStructureComplete() -> BOOL;
    /*looks to see if a structure (specified by type) exists and built*/
    #[no_mangle]
    fn scrStructureBuilt() -> BOOL;
    /*centre theview on an object - can be droid/structure or feature */
    #[no_mangle]
    fn scrCentreView() -> BOOL;
    /*centre the view on a position */
    #[no_mangle]
    fn scrCentreViewPos() -> BOOL;
    // Get a pointer to a structure based on a stat - returns NULL if cannot find one
    #[no_mangle]
    fn scrGetStructure() -> BOOL;
    // Get a pointer to a template based on a component stat - returns NULL if cannot find one
    #[no_mangle]
    fn scrGetTemplate() -> BOOL;
    // Get a pointer to a droid based on a component stat - returns NULL if cannot find one
    #[no_mangle]
    fn scrGetDroid() -> BOOL;
    // Sets all the scroll params for the map
    #[no_mangle]
    fn scrSetScrollParams() -> BOOL;
    // Sets the scroll minX separately for the map
    #[no_mangle]
    fn scrSetScrollMinX() -> BOOL;
    // Sets the scroll minY separately for the map
    #[no_mangle]
    fn scrSetScrollMinY() -> BOOL;
    // Sets the scroll maxX separately for the map
    #[no_mangle]
    fn scrSetScrollMaxX() -> BOOL;
    // Sets the scroll maxY separately for the map
    #[no_mangle]
    fn scrSetScrollMaxY() -> BOOL;
    // Sets which sensor will be used as the default for a player
    #[no_mangle]
    fn scrSetDefaultSensor() -> BOOL;
    // Sets which ECM will be used as the default for a player
    #[no_mangle]
    fn scrSetDefaultECM() -> BOOL;
    // Sets which RepairUnit will be used as the default for a player
    #[no_mangle]
    fn scrSetDefaultRepair() -> BOOL;
    // Sets the structure limits for a player
    #[no_mangle]
    fn scrSetStructureLimits() -> BOOL;
    // Sets all structure limits for a player to a specified value
    #[no_mangle]
    fn scrSetAllStructureLimits() -> BOOL;
    //multiplayer limit handler
    #[no_mangle]
    fn scrApplyLimitSet() -> BOOL;
    // plays a sound for the specified player - only plays the sound if the
//specified player = selectedPlayer
    #[no_mangle]
    fn scrPlaySound() -> BOOL;
    // plays a sound for the specified player - only plays the sound if the
// specified player = selectedPlayer - saves position
    #[no_mangle]
    fn scrPlaySoundPos() -> BOOL;
    /* add a text message tothe top of the screen for the selected player*/
    #[no_mangle]
    fn scrAddConsoleText() -> BOOL;
    // same as above - but it doesn't clear what's there and isn't permanent
    #[no_mangle]
    fn scrShowConsoleText() -> BOOL;
    /* Adds console text without clearing old */
    #[no_mangle]
    fn scrTagConsoleText() -> BOOL;
    //demo functions for turning the power on
    #[no_mangle]
    fn scrTurnPowerOff() -> BOOL;
    //demo functions for turning the power off
    #[no_mangle]
    fn scrTurnPowerOn() -> BOOL;
    //flags when the tutorial is over so that console messages can be turned on again
    #[no_mangle]
    fn scrTutorialEnd() -> BOOL;
    //function to play a full-screen video in the middle of the game for the selected player
    #[no_mangle]
    fn scrPlayVideo() -> BOOL;
    //checks to see if there are any droids for the specified player
    #[no_mangle]
    fn scrAnyDroidsLeft() -> BOOL;
    //checks to see if there are any structures (except walls) for the specified player
    #[no_mangle]
    fn scrAnyStructButWallsLeft() -> BOOL;
    #[no_mangle]
    fn scrAnyFactoriesLeft() -> BOOL;
    //function to call when the game is over, plays a message.
    #[no_mangle]
    fn scrGameOverMessage() -> BOOL;
    //function to call when the game is over
    #[no_mangle]
    fn scrGameOver() -> BOOL;
    //defines the background audio to play
    #[no_mangle]
    fn scrPlayBackgroundAudio() -> BOOL;
    // cd audio funcs
    #[no_mangle]
    fn scrPlayCDAudio() -> BOOL;
    #[no_mangle]
    fn scrStopCDAudio() -> BOOL;
    #[no_mangle]
    fn scrPauseCDAudio() -> BOOL;
    #[no_mangle]
    fn scrResumeCDAudio() -> BOOL;
    // set the retreat point for a player
    #[no_mangle]
    fn scrSetRetreatPoint() -> BOOL;
    // set the retreat force level
    #[no_mangle]
    fn scrSetRetreatForce() -> BOOL;
    // set the retreat leadership
    #[no_mangle]
    fn scrSetRetreatLeadership() -> BOOL;
    // set the retreat point for a group
    #[no_mangle]
    fn scrSetGroupRetreatPoint() -> BOOL;
    #[no_mangle]
    fn scrSetGroupRetreatForce() -> BOOL;
    // set the retreat leadership
    #[no_mangle]
    fn scrSetGroupRetreatLeadership() -> BOOL;
    // set the retreat health level
    #[no_mangle]
    fn scrSetRetreatHealth() -> BOOL;
    #[no_mangle]
    fn scrSetGroupRetreatHealth() -> BOOL;
    //start a Mission
    #[no_mangle]
    fn scrStartMission() -> BOOL;
    //end a mission NO LONGER CALLED FROM SCRIPT
//extern BOOL scrEndMission(void);
    //set Snow (enable disable snow)
    #[no_mangle]
    fn scrSetSnow() -> BOOL;
    //set Rain (enable disable Rain)
    #[no_mangle]
    fn scrSetRain() -> BOOL;
    //set Background Fog (replace fade out with fog)
    #[no_mangle]
    fn scrSetBackgroundFog() -> BOOL;
    //set Depth Fog (gradual fog from mid range to edge of world)
    #[no_mangle]
    fn scrSetDepthFog() -> BOOL;
    //set Mission Fog colour, may be modified by weather effects
    #[no_mangle]
    fn scrSetFogColour() -> BOOL;
    // remove a message from the Intelligence Display
    #[no_mangle]
    fn scrRemoveMessage() -> BOOL;
    // Pop up a message box with a number value in it
    #[no_mangle]
    fn scrNumMB() -> BOOL;
    // Do an approximation to a square root
    #[no_mangle]
    fn scrApproxRoot() -> BOOL;
    #[no_mangle]
    fn scrRefTest() -> BOOL;
    // is <player> human or a computer? (multiplayer)
    #[no_mangle]
    fn scrIsHumanPlayer() -> BOOL;
    // Set an alliance between two players
    #[no_mangle]
    fn scrCreateAlliance() -> BOOL;
    #[no_mangle]
    fn scrOfferAlliance() -> BOOL;
    // Break an alliance between two players
    #[no_mangle]
    fn scrBreakAlliance() -> BOOL;
    // push true if an alliance still exists.
    #[no_mangle]
    fn scrAllianceExists() -> BOOL;
    #[no_mangle]
    fn scrAllianceExistsBetween() -> BOOL;
    // true if player is allied.
    #[no_mangle]
    fn scrPlayerInAlliance() -> BOOL;
    // push true if group wins are allowed.
//extern BOOL scrAllianceState(void);
    // push true if a single alliance is dominant.
    #[no_mangle]
    fn scrDominatingAlliance() -> BOOL;
    // push true if human player is responsible for 'player'
    #[no_mangle]
    fn scrMyResponsibility() -> BOOL;
    /*checks to see if a structure of the type specified exists within the
specified range of an XY location */
    #[no_mangle]
    fn scrStructureBuiltInRange() -> BOOL;
    // generate a random number
    #[no_mangle]
    fn scrRandom() -> BOOL;
    // randomise the random number seed
    #[no_mangle]
    fn scrRandomiseSeed() -> BOOL;
    //explicitly enables a research topic
    #[no_mangle]
    fn scrEnableResearch() -> BOOL;
    //acts as if the research topic was completed - used to jump into the tree
    #[no_mangle]
    fn scrCompleteResearch() -> BOOL;
    // start a reticule button flashing
    #[no_mangle]
    fn scrFlashOn() -> BOOL;
    // stop a reticule button flashing
    #[no_mangle]
    fn scrFlashOff() -> BOOL;
    //set the initial power level settings for a player
    #[no_mangle]
    fn scrSetPowerLevel() -> BOOL;
    //add some power for a player
    #[no_mangle]
    fn scrAddPower() -> BOOL;
    //set the landing Zone position for the map
    #[no_mangle]
    fn scrSetLandingZone() -> BOOL;
    /*set the landing Zone position for the Limbo droids*/
    #[no_mangle]
    fn scrSetLimboLanding() -> BOOL;
    //initialises all the no go areas
    #[no_mangle]
    fn scrInitAllNoGoAreas() -> BOOL;
    //set a no go area for the map - landing zones for the enemy, or player 0
    #[no_mangle]
    fn scrSetNoGoArea() -> BOOL;
    // set the zoom level for the radar
    #[no_mangle]
    fn scrSetRadarZoom() -> BOOL;
    //set the time delay for reinforcements for an offworld mission
    #[no_mangle]
    fn scrSetReinforcementTime() -> BOOL;
    //set how long an offworld mission can last -1 = no limit
    #[no_mangle]
    fn scrSetMissionTime() -> BOOL;
    // this returns how long is left for the current mission time is 1/100th sec - same units as passed in
    #[no_mangle]
    fn scrMissionTimeRemaining() -> BOOL;
    // clear all the console messages
    #[no_mangle]
    fn scrFlushConsoleMessages() -> BOOL;
    // find and manipulate a position to build a structure.
    #[no_mangle]
    fn scrPickStructLocation() -> BOOL;
    // establish the distance between two points in world coordinates - approximate bounded to 11% out
    #[no_mangle]
    fn scrDistanceTwoPts() -> BOOL;
    // decides if a base object can see another - you can select whether walls matter to line of sight
    #[no_mangle]
    fn scrLOSTwoBaseObjects() -> BOOL;
    // destroys all structures of a certain type within a certain area and gives a gfx effect if you want it
    #[no_mangle]
    fn scrDestroyStructuresInArea() -> BOOL;
    // Estimates a threat from droids within a certain area
    #[no_mangle]
    fn scrThreatInArea() -> BOOL;
    // gets the nearest gateway to a list of points
    #[no_mangle]
    fn scrGetNearestGateway() -> BOOL;
    // Lets the user specify which tile goes under water.
    #[no_mangle]
    fn scrSetWaterTile() -> BOOL;
    // lets the user specify which tile	is used for rubble on skyscraper destruction
    #[no_mangle]
    fn scrSetRubbleTile() -> BOOL;
    // Tells the game what campaign it's in
    #[no_mangle]
    fn scrSetCampaignNumber() -> BOOL;
    // tests whether a structure has a module. If structure is null, then any structure
    #[no_mangle]
    fn scrTestStructureModule() -> BOOL;
    // give a player a template from another player
    #[no_mangle]
    fn scrAddTemplate() -> BOOL;
    // Sets the transporter entry and exit points for the map
    #[no_mangle]
    fn scrSetTransporterExit() -> BOOL;
    // Fly transporters in at start of map
    #[no_mangle]
    fn scrFlyTransporterIn() -> BOOL;
    // Add droid to transporter
    #[no_mangle]
    fn scrAddDroidToTransporter() -> BOOL;
    #[no_mangle]
    fn scrDestroyUnitsInArea() -> BOOL;
    // Removes a droid from thr world without all the graphical hoo ha.
    #[no_mangle]
    fn scrRemoveDroid() -> BOOL;
    // Sets an object to be a certain percent damaged
    #[no_mangle]
    fn scrForceDamage() -> BOOL;
    #[no_mangle]
    fn scrGetGameStatus() -> BOOL;
    //get the colour number used by a player
    #[no_mangle]
    fn scrGetPlayerColour() -> BOOL;
    //set the colour number to use for a player
    #[no_mangle]
    fn scrSetPlayerColour() -> BOOL;
    //set all droids in an area to belong to a different player
    #[no_mangle]
    fn scrTakeOverDroidsInArea() -> BOOL;
    /*this takes over a single droid and passes a pointer back to the new one*/
    #[no_mangle]
    fn scrTakeOverSingleDroid() -> BOOL;
    // set all droids in an area of a certain experience level or less to belong to
// a different player - returns the number of droids changed
    #[no_mangle]
    fn scrTakeOverDroidsInAreaExp() -> BOOL;
    /*this takes over a single structure and passes a pointer back to the new one*/
    #[no_mangle]
    fn scrTakeOverSingleStructure() -> BOOL;
    //set all structures in an area to belong to a different player - returns the number of droids changed
//will not work on factories for the selectedPlayer
    #[no_mangle]
    fn scrTakeOverStructsInArea() -> BOOL;
    //set Flag for defining what happens to the droids in a Transporter
    #[no_mangle]
    fn scrSetDroidsToSafetyFlag() -> BOOL;
    //set Flag for defining whether the coded countDown is called
    #[no_mangle]
    fn scrSetPlayCountDown() -> BOOL;
    //get the number of droids currently onthe map for a player
    #[no_mangle]
    fn scrGetDroidCount() -> BOOL;
    // fire a weapon stat at an object
    #[no_mangle]
    fn scrFireWeaponAtObj() -> BOOL;
    // fire a weapon stat at a location
    #[no_mangle]
    fn scrFireWeaponAtLoc() -> BOOL;
    #[no_mangle]
    fn scrClearConsole() -> BOOL;
    // set the number of kills for a droid
    #[no_mangle]
    fn scrSetDroidKills() -> BOOL;
    // reset the visibility for a player
    #[no_mangle]
    fn scrResetPlayerVisibility() -> BOOL;
    // set the vtol return pos for a player
    #[no_mangle]
    fn scrSetVTOLReturnPos() -> BOOL;
    // skirmish function **NOT PSX**
    #[no_mangle]
    fn scrIsVtol() -> BOOL;
    // init templates for tutorial.
    #[no_mangle]
    fn scrTutorialTemplates() -> BOOL;
    //called via the script in a Limbo Expand level to set the level to plain ol' expand
    #[no_mangle]
    fn scrResetLimboMission() -> BOOL;
    // skirmish lassat fire.
    #[no_mangle]
    fn scrSkFireLassat() -> BOOL;
    //-----------------------------------------
//New functions
//-----------------------------------------
    #[no_mangle]
    fn scrStrcmp() -> BOOL;
    #[no_mangle]
    fn scrConsole() -> BOOL;
    #[no_mangle]
    fn scrDbgMsgOn() -> BOOL;
    #[no_mangle]
    fn scrDbg() -> BOOL;
    #[no_mangle]
    fn scrMsg() -> BOOL;
    #[no_mangle]
    fn scrDebugFile() -> BOOL;
    #[no_mangle]
    fn scrActionDroidObj() -> BOOL;
    #[no_mangle]
    fn scrInitEnumDroids() -> BOOL;
    #[no_mangle]
    fn scrEnumDroid() -> BOOL;
    #[no_mangle]
    fn scrInitIterateGroupB() -> BOOL;
    #[no_mangle]
    fn scrIterateGroupB() -> BOOL;
    #[no_mangle]
    fn scrFactoryGetTemplate() -> BOOL;
    #[no_mangle]
    fn scrNumTemplatesInProduction() -> BOOL;
    #[no_mangle]
    fn scrNumDroidsByComponent() -> BOOL;
    #[no_mangle]
    fn scrGetStructureLimit() -> BOOL;
    #[no_mangle]
    fn scrStructureLimitReached() -> BOOL;
    #[no_mangle]
    fn scrGetNumStructures() -> BOOL;
    #[no_mangle]
    fn scrGetUnitLimit() -> BOOL;
    #[no_mangle]
    fn scrMin() -> BOOL;
    #[no_mangle]
    fn scrMax() -> BOOL;
    #[no_mangle]
    fn scrFogTileInRange() -> BOOL;
    #[no_mangle]
    fn scrMapRevealedInRange() -> BOOL;
    #[no_mangle]
    fn scrNumResearchLeft() -> BOOL;
    #[no_mangle]
    fn scrResearchCompleted() -> BOOL;
    #[no_mangle]
    fn scrResearchStarted() -> BOOL;
    #[no_mangle]
    fn scrThreatInRange() -> BOOL;
    #[no_mangle]
    fn scrNumEnemyWeapObjInRange() -> BOOL;
    #[no_mangle]
    fn scrNumEnemyWeapDroidsInRange() -> BOOL;
    #[no_mangle]
    fn scrNumEnemyWeapStructsInRange() -> BOOL;
    #[no_mangle]
    fn scrNumFriendlyWeapObjInRange() -> BOOL;
    #[no_mangle]
    fn scrNumFriendlyWeapDroidsInRange() -> BOOL;
    #[no_mangle]
    fn scrNumFriendlyWeapStructsInRange() -> BOOL;
    #[no_mangle]
    fn scrNumPlayerWeapObjInRange() -> BOOL;
    #[no_mangle]
    fn scrNumEnemyObjInRange() -> BOOL;
    #[no_mangle]
    fn scrNumStructsByStatInRange() -> BOOL;
    #[no_mangle]
    fn scrNumStructsByStatInArea() -> BOOL;
    #[no_mangle]
    fn scrNumStructsByTypeInRange() -> BOOL;
    #[no_mangle]
    fn scrNumFeatByTypeInRange() -> BOOL;
    #[no_mangle]
    fn scrNumStructsButNotWallsInRangeVis() -> BOOL;
    #[no_mangle]
    fn scrGetStructureVis() -> BOOL;
    #[no_mangle]
    fn scrChooseValidLoc() -> BOOL;
    #[no_mangle]
    fn scrGetClosestEnemy() -> BOOL;
    #[no_mangle]
    fn scrTransporterCapacity() -> BOOL;
    #[no_mangle]
    fn scrTransporterFlying() -> BOOL;
    #[no_mangle]
    fn scrUnloadTransporter() -> BOOL;
    #[no_mangle]
    fn scrHasGroup() -> BOOL;
    #[no_mangle]
    fn scrObjWeaponMaxRange() -> BOOL;
    #[no_mangle]
    fn scrObjHasWeapon() -> BOOL;
    #[no_mangle]
    fn scrObjectHasIndirectWeapon() -> BOOL;
    #[no_mangle]
    fn scrGetClosestEnemyDroidByType() -> BOOL;
    #[no_mangle]
    fn scrGetClosestEnemyStructByType() -> BOOL;
    #[no_mangle]
    fn scrSkDefenseLocationB() -> BOOL;
    #[no_mangle]
    fn scrCirclePerimPoint() -> BOOL;
    #[no_mangle]
    fn scrGiftRadar() -> BOOL;
    #[no_mangle]
    fn scrNumAllies() -> BOOL;
    #[no_mangle]
    fn scrNumAAinRange() -> BOOL;
    #[no_mangle]
    fn scrSelectDroid() -> BOOL;
    #[no_mangle]
    fn scrSelectGroup() -> BOOL;
    #[no_mangle]
    fn scrModulo() -> BOOL;
    #[no_mangle]
    fn scrPlayerLoaded() -> BOOL;
    /*
 * ScriptExtern.h
 *
 * All game variable access functions for the scripts
 *
 */
    // current game level
    #[no_mangle]
    static mut scrGameLevel: SDWORD;
    // whether the tutorial is active
    #[no_mangle]
    static mut bInTutorial: BOOL;
    // General function to get some basic game values
    #[no_mangle]
    fn scrGenExternGet(index: UDWORD) -> BOOL;
    // General function to set some basic game values
    #[no_mangle]
    fn scrGenExternSet(index: UDWORD) -> BOOL;
    // Get values from a base object
    #[no_mangle]
    fn scrBaseObjGet(index: UDWORD) -> BOOL;
    // convert a base object to a droid if it is the right type
    #[no_mangle]
    fn scrObjToDroid() -> BOOL;
    // convert a base object to a structure if it is the right type
    #[no_mangle]
    fn scrObjToStructure() -> BOOL;
    // convert a base object to a feature if it is the right type
    #[no_mangle]
    fn scrObjToFeature() -> BOOL;
    // Get values from a group
    #[no_mangle]
    fn scrGroupObjGet(index: UDWORD) -> BOOL;
    // default value save routine
    #[no_mangle]
    fn scrValDefSave(type_0: INTERP_TYPE, data: UDWORD,
                     pBuffer: *mut libc::c_char, pSize: *mut UDWORD) -> BOOL;
    // default value load routine
    #[no_mangle]
    fn scrValDefLoad(version: SDWORD, type_0: INTERP_TYPE,
                     pBuffer: *mut libc::c_char, size: UDWORD,
                     pData: *mut UDWORD) -> BOOL;
    // Add a new base pointer variable
    #[no_mangle]
    fn scrvAddBasePointer(psVal: *mut INTERP_VAL) -> BOOL;
    // remove a base pointer from the list
    #[no_mangle]
    fn scrvReleaseBasePointer(psVal: *mut INTERP_VAL);
    // create a group structure for a ST_GROUP variable
    #[no_mangle]
    fn scrvNewGroup(psVal: *mut INTERP_VAL) -> BOOL;
    // release a ST_GROUP variable
    #[no_mangle]
    fn scrvReleaseGroup(psVal: *mut INTERP_VAL);
    // Initialise the script value module
    #[no_mangle]
    fn scrvInitialise() -> BOOL;
    // Shut down the script value module
    #[no_mangle]
    fn scrvShutDown();
    // deal with unit takover(2)
    #[no_mangle]
    fn scrCBDroidTaken() -> BOOL;
    // Deal with a CALL_NEWDROID
    #[no_mangle]
    fn scrCBNewDroid() -> BOOL;
    // Deal with a CALL_STRUCT_ATTACKED
    #[no_mangle]
    fn scrCBStructAttacked() -> BOOL;
    // Deal with a CALL_DROID_ATTACKED
    #[no_mangle]
    fn scrCBDroidAttacked() -> BOOL;
    // Deal with a CALL_ATTACKED
    #[no_mangle]
    fn scrCBAttacked() -> BOOL;
    // deal with CALL_BUTTON_PRESSED
    #[no_mangle]
    fn scrCBButtonPressed() -> BOOL;
    // deal with CALL_DROID_SELECTED
    #[no_mangle]
    fn scrCBDroidSelected() -> BOOL;
    // deal with a CALL_OBJ_DESTROYED
    #[no_mangle]
    fn scrCBObjDestroyed() -> BOOL;
    // deal with a CALL_STRUCT_DESTROYED
    #[no_mangle]
    fn scrCBStructDestroyed() -> BOOL;
    // deal with a CALL_DROID_DESTROYED
    #[no_mangle]
    fn scrCBDroidDestroyed() -> BOOL;
    // deal with a CALL_FEATURE_DESTROYED
    #[no_mangle]
    fn scrCBFeatureDestroyed() -> BOOL;
    // deal with a CALL_OBJ_SEEN
    #[no_mangle]
    fn scrCBObjSeen() -> BOOL;
    // deal with a CALL_DROID_SEEN
    #[no_mangle]
    fn scrCBDroidSeen() -> BOOL;
    // deal with a CALL_STRUCT_SEEN
    #[no_mangle]
    fn scrCBStructSeen() -> BOOL;
    // deal with a CALL_FEATURE_SEEN
    #[no_mangle]
    fn scrCBFeatureSeen() -> BOOL;
    // deal with a CALL_TRANSPORTER_OFFMAP
    #[no_mangle]
    fn scrCBTransporterOffMap() -> BOOL;
    // deal with a CALL_TRANSPORTER_LANDED
    #[no_mangle]
    fn scrCBTransporterLanded() -> BOOL;
    #[no_mangle]
    fn scrCBClusterEmpty() -> BOOL;
    #[no_mangle]
    fn scrCBVtolOffMap() -> BOOL;
    /*called when selectedPlayer completes some research*/
    #[no_mangle]
    fn scrCBResCompleted() -> BOOL;
    /* when a player leaves the game*/
    #[no_mangle]
    fn scrCBPlayerLeft() -> BOOL;
    // alliance offered.
    #[no_mangle]
    fn scrCBAllianceOffer() -> BOOL;
    //Console callback
    #[no_mangle]
    fn scrCallConsole() -> BOOL;
    #[no_mangle]
    fn scrCBStructBuilt() -> BOOL;
    #[no_mangle]
    fn scrCallMultiMsg() -> BOOL;
    //extern BOOL scrCallBeacon(void);
    #[no_mangle]
    fn scrCBTransporterLandedB() -> BOOL;
    /*
 * ScriptAI.h
 *
 * Script functions to support the AI system
 *
 */
    // Add a droid to a group
    #[no_mangle]
    fn scrGroupAddDroid() -> BOOL;
    // Add droids in an area to a group
    #[no_mangle]
    fn scrGroupAddArea() -> BOOL;
    // Move the droids from one group to another
    #[no_mangle]
    fn scrGroupAddGroup() -> BOOL;
    // check if a droid is a member of a group
    #[no_mangle]
    fn scrGroupMember() -> BOOL;
    // return number of idle droids in group.
    #[no_mangle]
    fn scrIdleGroup() -> BOOL;
    // initialise iterating a groups members
    #[no_mangle]
    fn scrInitIterateGroup() -> BOOL;
    // iterate through a groups members
    #[no_mangle]
    fn scrIterateGroup() -> BOOL;
    // remove a droid from a group
    #[no_mangle]
    fn scrDroidLeaveGroup() -> BOOL;
    // Give a group an order
    #[no_mangle]
    fn scrOrderGroup() -> BOOL;
    // Give a group an order to a location
    #[no_mangle]
    fn scrOrderGroupLoc() -> BOOL;
    // Give a group an order to an object
    #[no_mangle]
    fn scrOrderGroupObj() -> BOOL;
    // Give a Droid an order
    #[no_mangle]
    fn scrOrderDroid() -> BOOL;
    // Give a Droid an order to a location
    #[no_mangle]
    fn scrOrderDroidLoc() -> BOOL;
    // Give a Droid an order to an object
    #[no_mangle]
    fn scrOrderDroidObj() -> BOOL;
    // Give a Droid an order with a stat
    #[no_mangle]
    fn scrOrderDroidStatsLoc() -> BOOL;
    // set the secondary state for a droid
    #[no_mangle]
    fn scrSetDroidSecondary() -> BOOL;
    // set the secondary state for a droid
    #[no_mangle]
    fn scrSetGroupSecondary() -> BOOL;
    // initialise iterating a cluster
    #[no_mangle]
    fn scrInitIterateCluster() -> BOOL;
    // iterate a cluster
    #[no_mangle]
    fn scrIterateCluster() -> BOOL;
    // add a droid to a commander
    #[no_mangle]
    fn scrCmdDroidAddDroid() -> BOOL;
    // reset the structure preferences
    #[no_mangle]
    fn scrResetStructTargets() -> BOOL;
    // reset the droid preferences
    #[no_mangle]
    fn scrResetDroidTargets() -> BOOL;
    // set prefered structure target types
    #[no_mangle]
    fn scrSetStructTarPref() -> BOOL;
    // set structure target ignore types
    #[no_mangle]
    fn scrSetStructTarIgnore() -> BOOL;
    // set prefered droid target types
    #[no_mangle]
    fn scrSetDroidTarPref() -> BOOL;
    // set droid target ignore types
    #[no_mangle]
    fn scrSetDroidTarIgnore() -> BOOL;
    // get a structure target in an area using the preferences
    #[no_mangle]
    fn scrStructTargetInArea() -> BOOL;
    // get a structure target on the map using the preferences
    #[no_mangle]
    fn scrStructTargetOnMap() -> BOOL;
    // get a droid target in an area using the preferences
    #[no_mangle]
    fn scrDroidTargetInArea() -> BOOL;
    // get a droid target on the map using the preferences
    #[no_mangle]
    fn scrDroidTargetOnMap() -> BOOL;
    // get a target from a cluster using the preferences
    #[no_mangle]
    fn scrTargetInCluster() -> BOOL;
    // Skirmish funcs may99
    // choose and do research
    #[no_mangle]
    fn scrSkDoResearch() -> BOOL;
    // find the human players
    #[no_mangle]
    fn scrSkLocateEnemy() -> BOOL;
    // check a template
    #[no_mangle]
    fn scrSkCanBuildTemplate() -> BOOL;
    // check for vtol availability
    #[no_mangle]
    fn scrSkVtolEnableCheck() -> BOOL;
    // check capacity
    #[no_mangle]
    fn scrSkGetFactoryCapacity() -> BOOL;
    // help/hinder player.
    #[no_mangle]
    fn scrSkDifficultyModifier() -> BOOL;
    // pick good spots.
    #[no_mangle]
    fn scrSkDefenseLocation() -> BOOL;
}
pub type UBYTE = libc::c_uchar;
pub type STRING = libc::c_char;
pub type UWORD = libc::c_ushort;
pub type SWORD = libc::c_short;
pub type UDWORD = libc::c_uint;
pub type SDWORD = libc::c_int;
pub type BOOL = libc::c_int;
pub type _object_type = libc::c_uint;
pub const OBJ_TARGET: _object_type = 4;
pub const OBJ_BULLET: _object_type = 3;
pub const OBJ_FEATURE: _object_type = 2;
pub const OBJ_STRUCTURE: _object_type = 1;
pub const OBJ_DROID: _object_type = 0;
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
    pub v: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub bval: BOOL,
    pub ival: SDWORD,
    pub sval: *mut STRING,
    pub oval: *mut libc::c_void,
    pub pVoid: *mut libc::c_void,
}
pub type INTERP_VAL = _interp_val;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _interp_typeequiv {
    pub base: INTERP_TYPE,
    pub numEquiv: SDWORD,
    pub aEquivTypes: [INTERP_TYPE; 10],
}
pub type TYPE_EQUIV = _interp_typeequiv;
pub type SCRIPT_FUNC = Option<unsafe extern "C" fn() -> BOOL>;
pub type SCRIPT_VARFUNC = Option<unsafe extern "C" fn(_: UDWORD) -> BOOL>;
pub type _storage_type = libc::c_uint;
pub const ST_LOCAL: _storage_type = 4;
pub const ST_EXTERN: _storage_type = 3;
pub const ST_OBJECT: _storage_type = 2;
pub const ST_PRIVATE: _storage_type = 1;
pub const ST_PUBLIC: _storage_type = 0;
pub type STORAGE_TYPE = UBYTE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _script_debug {
    pub offset: UDWORD,
    pub line: UDWORD,
    pub pLabel: *mut STRING,
}
pub type SCRIPT_DEBUG = _script_debug;
pub type _trigger_type = libc::c_uint;
pub const TR_CALLBACKSTART: _trigger_type = 5;
pub const TR_PAUSE: _trigger_type = 4;
pub const TR_EVERY: _trigger_type = 3;
pub const TR_WAIT: _trigger_type = 2;
pub const TR_CODE: _trigger_type = 1;
pub const TR_INIT: _trigger_type = 0;
pub type TRIGGER_TYPE = _trigger_type;
/* The possible access types for a type */
pub type _access_type = libc::c_uint;
// The type represents an object
// The type represents a simple data value
pub const AT_OBJECT: _access_type = 1;
pub const AT_SIMPLE: _access_type = 0;
// function pointer for script variable saving
// if pBuffer is NULL the script system is just asking how much space the saved variable will require
// otherwise pBuffer points to an array to store the value in
pub type SCR_VAL_SAVE
    =
    Option<unsafe extern "C" fn(_: INTERP_TYPE, _: UDWORD,
                                _: *mut libc::c_char, _: *mut UDWORD)
               -> BOOL>;
// function pointer for script variable loading
pub type SCR_VAL_LOAD
    =
    Option<unsafe extern "C" fn(_: SDWORD, _: INTERP_TYPE,
                                _: *mut libc::c_char, _: UDWORD,
                                _: *mut UDWORD) -> BOOL>;
/* Type for a user type symbol */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _type_symbol {
    pub typeID: SWORD,
    pub accessType: SWORD,
    pub pIdent: *mut STRING,
    pub saveFunc: SCR_VAL_SAVE,
    pub loadFunc: SCR_VAL_LOAD,
}
pub type TYPE_SYMBOL = _type_symbol;
// 
/* Type for a variable symbol */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _var_symbol {
    pub pIdent: *mut STRING,
    pub type_0: INTERP_TYPE,
    pub storage: STORAGE_TYPE,
    pub objType: INTERP_TYPE,
    pub index: UDWORD,
    pub get: SCRIPT_VARFUNC,
    pub set: SCRIPT_VARFUNC,
    pub dimensions: SDWORD,
    pub elements: [SDWORD; 4],
    pub psNext: *mut _var_symbol,
}
pub type VAR_SYMBOL = _var_symbol;
/* Type for a constant symbol */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _const_symbol {
    pub pIdent: *mut STRING,
    pub type_0: INTERP_TYPE,
    pub bval: BOOL,
    pub ival: SDWORD,
    pub oval: *mut libc::c_void,
    pub sval: *mut STRING,
}
pub type CONST_SYMBOL = _const_symbol;
//String values
//	float			fval;
/* The maximum number of parameters for an instinct function */
/* Type for a function symbol */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _func_symbol {
    pub pIdent: *mut STRING,
    pub pFunc: SCRIPT_FUNC,
    pub type_0: INTERP_TYPE,
    pub numParams: UDWORD,
    pub aParams: [INTERP_TYPE; 20],
    pub script: BOOL,
    pub size: UDWORD,
    pub pCode: *mut UDWORD,
    pub location: UDWORD,
    pub debugEntries: UDWORD,
    pub psDebug: *mut SCRIPT_DEBUG,
    pub psNext: *mut _func_symbol,
}
pub type FUNC_SYMBOL = _func_symbol;
/* The type for a callback trigger symbol */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _callback_symbol {
    pub pIdent: *mut STRING,
    pub type_0: TRIGGER_TYPE,
    pub pFunc: SCRIPT_FUNC,
    pub numParams: UDWORD,
    pub aParams: [INTERP_TYPE; 20],
}
pub type CALLBACK_SYMBOL = _callback_symbol;
// List of parameter types
// The Event initialisation data
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _event_init {
    pub valInit: UWORD,
    pub valExt: UWORD,
    pub trigInit: UWORD,
    pub trigExt: UWORD,
    pub contInit: UWORD,
    pub contExt: UWORD,
}
pub type EVENT_INIT = _event_init;
// context chunk init values
// ID numbers for each user type
pub type _scr_user_types = libc::c_uint;
// maximum possible type - should always be last
// NULL stats
pub const ST_MAXTYPE: _scr_user_types = 34;
//used so we can check for NULL templates
pub const ST_POINTER_S: _scr_user_types = 33;
//used so we can check for NULL objects etc
pub const ST_POINTER_T: _scr_user_types = 32;
// A research topic
//private types for game code - not for use in script
pub const ST_POINTER_O: _scr_user_types = 31;
// A group of droids
pub const ST_RESEARCH: _scr_user_types = 30;
// The name of a game level
pub const ST_GROUP: _scr_user_types = 29;
pub const ST_LEVEL: _scr_user_types = 28;
// text string for display messages in tutorial
pub const ST_SOUND: _scr_user_types = 27;
// ID of a droid
pub const ST_TEXTSTRING: _scr_user_types = 26;
// feature stat type
pub const ST_DROIDID: _scr_user_types = 25;
// structure stat type
pub const ST_FEATURESTAT: _scr_user_types = 24;
/* A structure ID number (don't really 
											   need this since just a number?)*/
pub const ST_STRUCTURESTAT: _scr_user_types = 23;
// Template object
pub const ST_STRUCTUREID: _scr_user_types = 22;
pub const ST_TEMPLATE: _scr_user_types = 21;
pub const ST_BRAIN: _scr_user_types = 20;
pub const ST_REPAIR: _scr_user_types = 19;
pub const ST_WEAPON: _scr_user_types = 18;
pub const ST_CONSTRUCT: _scr_user_types = 17;
pub const ST_SENSOR: _scr_user_types = 16;
pub const ST_ECM: _scr_user_types = 15;
// Component types
pub const ST_PROPULSION: _scr_user_types = 14;
// General component
pub const ST_BODY: _scr_user_types = 13;
// General stats type
pub const ST_COMPONENT: _scr_user_types = 12;
// Feature object
pub const ST_BASESTATS: _scr_user_types = 11;
// Structure object
pub const ST_FEATURE: _scr_user_types = 10;
// Droid object
pub const ST_STRUCTURE: _scr_user_types = 9;
// Base object
pub const ST_DROID: _scr_user_types = 8;
// Intelligence message ?? (6)
pub const ST_BASEOBJECT: _scr_user_types = 7;
pub const ST_INTMESSAGE: _scr_user_types = 6;
// a create function for data stored in an INTERP_VAL
pub type VAL_CREATE_FUNC
    =
    Option<unsafe extern "C" fn(_: *mut INTERP_VAL) -> BOOL>;
// a release function for data stored in an INTERP_VAL
pub type VAL_RELEASE_FUNC
    =
    Option<unsafe extern "C" fn(_: *mut INTERP_VAL) -> ()>;
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
pub const EXTID_MULTIGAMEBASETYPE: _externids = 9;
pub const EXTID_MULTIGAMEHUMANMAX: _externids = 8;
pub const EXTID_MULTIGAMETYPE: _externids = 7;
pub const EXTID_EXTRAFAILFLAG: _externids = 14;
// IHATESCRIPTSANDEVERYTHINGTHEYSTANDFOR(ESPECIALLYONSUNDAYS)
pub const EXTID_EXTRAVICTORYFLAG: _externids = 13;
pub const EXTID_TARGETTYPE: _externids = 12;
pub const EXTID_INTMODE: _externids = 11;
pub const EXTID_CURSOR: _externids = 10;
pub const EXTID_TUTORIAL: _externids = 6;
pub const EXTID_GAMELEVEL: _externids = 4;
pub const EXTID_GAMETIME: _externids = 5;
pub const EXTID_SELECTEDPLAYER: _externids = 3;
pub const EXTID_GAMEINIT: _externids = 2;
pub const EXTID_MAPHEIGHT: _externids = 1;
pub const EXTID_MAPWIDTH: _externids = 0;
pub const EXTID_TRACKTRANSPORTER: _externids = 15;
pub const EXTID_ISPSX: _externids = 16;
pub const OBJID_SELECTED: _objids = 18;
// the stat of a structure
pub const OBJID_STRUCTSTATTYPE: _objids = 14;
//if droid is selected (humans only)
pub const OBJID_TARGET: _objids = 19;
// the weapon component
pub const OBJID_STRUCTSTAT: _objids = 13;
pub const OBJID_ACTION: _objids = 17;
// number of units in a group
pub const GROUPID_HEALTH: _groupids = 3;
// average y of a group
pub const GROUPID_MEMBERS: _groupids = 2;
// average x of a group
pub const GROUPID_POSY: _groupids = 1;
pub const GROUPID_POSX: _groupids = 0;
// order coords.106
pub const OBJID_ORDERY: _objids = 16;
//new
pub const OBJID_ORDERX: _objids = 15;
// the propulsion component
pub const OBJID_WEAPON: _objids = 12;
// the body component
pub const OBJID_PROPULSION: _objids = 11;
// %age damage level
pub const OBJID_BODY: _objids = 10;
// current droid order
pub const OBJID_DROIDTYPE: _objids = 7;
// type of a base object
pub const OBJID_ORDER: _objids = 6;
// which cluster the object is a member of
pub const OBJID_HEALTH: _objids = 9;
// what type of droid
pub const OBJID_CLUSTERID: _objids = 8;
// player of a base object
pub const OBJID_TYPE: _objids = 5;
// id of a base object
pub const OBJID_PLAYER: _objids = 4;
pub const OBJID_ID: _objids = 3;
pub const OBJID_POSZ: _objids = 2;
// Position of a base object
pub const OBJID_POSY: _objids = 1;
pub const OBJID_POSX: _objids = 0;
pub const SCR_DT_HOVER: _scr_droid_tar = 65536;
pub const SCR_DT_VTOL: _scr_droid_tar = 32768;
pub const SCR_DT_GROUND: _scr_droid_tar = 30720;
pub const SCR_DT_LEGS: _scr_droid_tar = 16384;
pub const SCR_DT_WHEEL: _scr_droid_tar = 8192;
pub const SCR_DT_HTRACK: _scr_droid_tar = 4096;
pub const SCR_DT_TRACK: _scr_droid_tar = 2048;
pub const SCR_DT_SUPER_HEAVY: _scr_droid_tar = 1024;
pub const SCR_DT_HEAVY: _scr_droid_tar = 512;
pub const SCR_DT_MEDIUM: _scr_droid_tar = 256;
pub const SCR_DT_LIGHT: _scr_droid_tar = 128;
pub const SCR_DT_WEAP_ALL: _scr_droid_tar = 112;
pub const SCR_DT_WEAP_IDF: _scr_droid_tar = 64;
pub const SCR_DT_WEAP_AIR: _scr_droid_tar = 32;
pub const SCR_DT_WEAP_GROUND: _scr_droid_tar = 16;
pub const SCR_DT_REPAIR: _scr_droid_tar = 8;
pub const SCR_DT_CONSTRUCT: _scr_droid_tar = 4;
pub const SCR_DT_SENSOR: _scr_droid_tar = 2;
pub const SCR_DT_COMMAND: _scr_droid_tar = 1;
pub const SCR_ST_DEF_ALL: _scr_struct_tar = 28672;
pub const SCR_ST_DEF_IDF: _scr_struct_tar = 16384;
pub const SCR_ST_DEF_AIR: _scr_struct_tar = 8192;
pub const SCR_ST_DEF_GROUND: _scr_struct_tar = 4096;
pub const SCR_ST_SENSOR: _scr_struct_tar = 2048;
pub const SCR_ST_REARM_PAD: _scr_struct_tar = 1024;
pub const SCR_ST_VTOL_FACTORY: _scr_struct_tar = 512;
pub const SCR_ST_CYBORG_FACTORY: _scr_struct_tar = 256;
pub const SCR_ST_COMMAND_CONTROL: _scr_struct_tar = 128;
pub const SCR_ST_REPAIR_FACILITY: _scr_struct_tar = 64;
pub const SCR_ST_RESEARCH: _scr_struct_tar = 32;
pub const SCR_ST_WALL: _scr_struct_tar = 16;
pub const SCR_ST_RESOURCE_EXTRACTOR: _scr_struct_tar = 8;
pub const SCR_ST_POWER_GEN: _scr_struct_tar = 4;
pub const SCR_ST_FACTORY: _scr_struct_tar = 2;
pub const SCR_ST_HQ: _scr_struct_tar = 1;
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
pub const STATUS_DeliveryReposInProgress: gamestatus = 2;
pub const STATUS_BattleMapViewEnabled: gamestatus = 1;
pub const STATUS_ReticuleIsOpen: gamestatus = 0;
pub const INT_MULTIMENU: C2RustUnnamed_1 = 13;
pub const INT_MISSIONRES: C2RustUnnamed_1 = 12;
pub const INT_TRANSPORTER: C2RustUnnamed_1 = 11;
pub const INT_INGAMEOP: C2RustUnnamed_1 = 10;
pub const INT_ORDER: C2RustUnnamed_1 = 9;
pub const INT_INTELMAP: C2RustUnnamed_1 = 8;
pub const INT_DESIGN: C2RustUnnamed_1 = 7;
pub const INT_CMDORDER: C2RustUnnamed_1 = 6;
pub const INT_STAT: C2RustUnnamed_1 = 5;
pub const INT_OBJECT: C2RustUnnamed_1 = 4;
pub const INT_EDIT: C2RustUnnamed_1 = 2;
pub const INT_EDITSTAT: C2RustUnnamed_1 = 3;
pub const INT_OPTION: C2RustUnnamed_1 = 1;
pub const INT_NORMAL: C2RustUnnamed_1 = 0;
pub const IMAGE_CURSOR_EMBARK: C2RustUnnamed_2 = 236;
pub const IMAGE_CURSOR_FIX: C2RustUnnamed_2 = 239;
pub const IMAGE_CURSOR_LOCKON: C2RustUnnamed_2 = 230;
pub const IMAGE_CURSOR_ATTACH: C2RustUnnamed_2 = 238;
pub const IMAGE_CURSOR_BRIDGE: C2RustUnnamed_2 = 237;
pub const IMAGE_CURSOR_GUARD: C2RustUnnamed_2 = 235;
pub const IMAGE_CURSOR_BUILD: C2RustUnnamed_2 = 233;
pub const IMAGE_CURSOR_DEFAULT: C2RustUnnamed_2 = 232;
pub const IMAGE_CURSOR_PICKUP: C2RustUnnamed_2 = 241;
pub const IMAGE_CURSOR_REPAIR: C2RustUnnamed_2 = 240;
pub const IMAGE_CURSOR_ECM: C2RustUnnamed_2 = 231;
pub const IMAGE_CURSOR_MOVE: C2RustUnnamed_2 = 234;
pub const IMAGE_CURSOR_ATTACK: C2RustUnnamed_2 = 228;
pub const IMAGE_CURSOR_SELECT: C2RustUnnamed_2 = 229;
pub const DSS_PATROL_SET: _secondary_state = 4194304;
pub const DSS_RTL_TRANSPORT: _secondary_state = 2097152;
pub const DSS_RTL_BASE: _secondary_state = 1048576;
pub const DSS_RTL_REPAIR: _secondary_state = 524288;
pub const DSS_ASSPROD_END: _secondary_state = 262144;
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
pub const DSO_RETURN_TO_LOC: _secondary_order = 9;
pub const DSO_HALTTYPE: _secondary_order = 8;
pub const DSO_PATROL: _secondary_order = 7;
pub const DSO_RECYCLE: _secondary_order = 6;
pub const DSO_ATTACK_LEVEL: _secondary_order = 2;
pub const DSO_REPAIR_LEVEL: _secondary_order = 1;
pub const DSO_ATTACK_RANGE: _secondary_order = 0;
pub const DACTION_FIRESUPPORT_RETREAT: _droid_action = 39;
pub const DACTION_RETURNTOPOS: _droid_action = 38;
pub const DACTION_CLEARREARMPAD: _droid_action = 37;
pub const DACTION_VTOLATTACK: _droid_action = 36;
pub const DACTION_WAITDURINGREARM: _droid_action = 35;
pub const DACTION_MOVETOREARMPOINT: _droid_action = 34;
pub const DACTION_WAITFORREARM: _droid_action = 33;
pub const DACTION_MOVETOREARM: _droid_action = 32;
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
pub const DACTION_NONE: _droid_action = 0;
pub const DORDER_DROIDREPAIR: _droid_order = 26;
pub const DORDER_SCOUT: _droid_order = 28;
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
// extra data for expanding a campaign map
pub const LDS_BETWEEN: _level_type = 5;
// off map mission (extra map data)
pub const LDS_MCLEAR: _level_type = 7;
// pause between missions
pub const LDS_MKEEP: _level_type = 6;
// data for changing between levels
pub const LDS_EXPAND: _level_type = 4;
// the data set for a campaign (no map data)
pub const LDS_CAMSTART: _level_type = 2;
pub const MSG_PROXIMITY: _message_type = 3;
pub const MSG_MISSION: _message_type = 2;
pub const MSG_CAMPAIGN: _message_type = 1;
pub const MSG_RESEARCH: _message_type = 0;
pub type gamestatus = libc::c_uint;
// ID numbers for external variables
pub type _externids = libc::c_uint;
/*
 * ScriptObj.h
 *
 * Object access functions for the script library
 *
 */
// id's for object variables
pub type _objids = libc::c_uint;
//added object->psTarget
// id's for group variables
pub type _groupids = libc::c_uint;
pub type _scr_struct_tar = libc::c_uint;
pub type _scr_droid_tar = libc::c_uint;
pub type _message_type = libc::c_uint;
pub const MSG_TYPES: _message_type = 4;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const INT_MAXMODE: C2RustUnnamed_1 = 15;
pub const INT_CDCHANGE: C2RustUnnamed_1 = 14;
// average health of a group
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
// off map saving any droids (selectedPlayer) at end into apsLimboDroids
pub const LDS_NONE: _level_type = 10;
// expand campaign map using droids held in apsLimboDroids
pub const LDS_MKEEP_LIMBO: _level_type = 9;
// off map mission (extra map data)
pub const LDS_EXPAND_LIMBO: _level_type = 8;
// mapdata for the start of a campaign
pub const LDS_CAMCHANGE: _level_type = 3;
// all data required for a stand alone level
pub const LDS_CAMPAIGN: _level_type = 1;
pub const LDS_COMPLETE: _level_type = 0;
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
pub const DORDER_RESTORE: _droid_order = 27;
pub const DORDER_GUARD: _droid_order = 25;
pub const DORDER_TRANSPORTRETURN: _droid_order = 24;
pub const DORDER_TRANSPORTIN: _droid_order = 23;
pub const DORDER_TRANSPORTOUT: _droid_order = 22;
pub const DORDER_RECYCLE: _droid_order = 21;
pub const DORDER_BUILDMODULE: _droid_order = 20;
pub const DORDER_COMMAND: _droid_order = 19;
pub const DORDER_ATTACKTARGET: _droid_order = 18;
pub type _secondary_order = libc::c_uint;
pub const DSO_ASSIGN_VTOL_PRODUCTION: _secondary_order = 11;
pub const DSO_FIRE_DESIGNATOR: _secondary_order = 10;
pub const DSO_CLEAR_PRODUCTION: _secondary_order = 5;
pub const DSO_ASSIGN_CYBORG_PRODUCTION: _secondary_order = 4;
pub const DSO_ASSIGN_PRODUCTION: _secondary_order = 3;
pub type _secondary_state = libc::c_uint;
pub const DSS_VTOLPROD_END: _secondary_state = 268435456;
pub const DSS_VTOLPROD_START: _secondary_state = 16777216;
pub const DSS_FIREDES_SET: _secondary_state = 8388608;
pub const DSS_ASSPROD_MID: _secondary_state = 8192;
pub type _droid_action = libc::c_uint;
pub const DACTION_MOVETOCLEAR: _droid_action = 31;
pub const DACTION_CLEARWRECK: _droid_action = 16;
pub type _targets = libc::c_uint;
pub const MT_NOTARGET: _targets = 23;
pub const MT_SENSORSTRUCTDAM: _targets = 22;
pub const MT_SENSORSTRUCT: _targets = 21;
pub const MT_CONSTRUCT: _targets = 20;
pub const MT_WRECKFEATURE: _targets = 19;
/* **********************************************/
/* Image ID definition file created by Framer. */
/* **********************************************/
pub type C2RustUnnamed_2 = libc::c_uint;
pub const IMAGE_ASCII64: C2RustUnnamed_2 = 483;
pub const IMAGE_ORD_FIREDES_DOWN: C2RustUnnamed_2 = 482;
pub const IMAGE_ORD_FIREDES_UP: C2RustUnnamed_2 = 481;
pub const IMAGE_TRANSETA_DOWN: C2RustUnnamed_2 = 480;
pub const IMAGE_LAUNCHUP: C2RustUnnamed_2 = 479;
pub const IMAGE_BLUE6: C2RustUnnamed_2 = 478;
pub const IMAGE_BLUE5: C2RustUnnamed_2 = 477;
pub const IMAGE_BLUE4: C2RustUnnamed_2 = 476;
pub const IMAGE_BLUE3: C2RustUnnamed_2 = 475;
pub const IMAGE_BLUE2: C2RustUnnamed_2 = 474;
pub const IMAGE_BLUE1: C2RustUnnamed_2 = 473;
pub const IMAGE_DES_ARMOUR_KINETIC: C2RustUnnamed_2 = 472;
pub const IMAGE_TABHILIGHT_SM: C2RustUnnamed_2 = 471;
pub const IMAGE_TAB1DOWN_SM: C2RustUnnamed_2 = 470;
pub const IMAGE_TAB1SELECTED_SM: C2RustUnnamed_2 = 469;
pub const IMAGE_TAB1_SM: C2RustUnnamed_2 = 468;
pub const IMAGE_TARGET5: C2RustUnnamed_2 = 467;
pub const IMAGE_TARGET4: C2RustUnnamed_2 = 466;
pub const IMAGE_TARGET3: C2RustUnnamed_2 = 465;
pub const IMAGE_TARGET2: C2RustUnnamed_2 = 464;
pub const IMAGE_TARGET1: C2RustUnnamed_2 = 463;
pub const IMAGE_MISSION_CLOCK_UP: C2RustUnnamed_2 = 462;
pub const IMAGE_RES_GRPDAM: C2RustUnnamed_2 = 461;
pub const IMAGE_RES_GRPACC: C2RustUnnamed_2 = 460;
pub const IMAGE_RES_GRPROF: C2RustUnnamed_2 = 459;
pub const IMAGE_RES_GRPREP: C2RustUnnamed_2 = 458;
pub const IMAGE_RES_GRPUPG: C2RustUnnamed_2 = 457;
pub const IMAGE_ASCII131: C2RustUnnamed_2 = 456;
pub const IMAGE_ASCII161: C2RustUnnamed_2 = 455;
pub const IMAGE_ASCII191: C2RustUnnamed_2 = 454;
pub const IMAGE_ASCII208: C2RustUnnamed_2 = 453;
pub const IMAGE_ASCII207: C2RustUnnamed_2 = 452;
pub const IMAGE_ASCII206: C2RustUnnamed_2 = 451;
pub const IMAGE_ASCII205: C2RustUnnamed_2 = 450;
pub const IMAGE_ASCII204: C2RustUnnamed_2 = 449;
pub const IMAGE_ASCII203: C2RustUnnamed_2 = 448;
pub const IMAGE_ASCII202: C2RustUnnamed_2 = 447;
pub const IMAGE_ASCII201: C2RustUnnamed_2 = 446;
pub const IMAGE_ASCII200: C2RustUnnamed_2 = 445;
pub const IMAGE_ASCII198: C2RustUnnamed_2 = 444;
pub const IMAGE_ASCII197: C2RustUnnamed_2 = 443;
pub const IMAGE_ASCII196: C2RustUnnamed_2 = 442;
pub const IMAGE_ASCII195: C2RustUnnamed_2 = 441;
pub const IMAGE_ASCII194: C2RustUnnamed_2 = 440;
pub const IMAGE_ASCII193: C2RustUnnamed_2 = 439;
pub const IMAGE_ASCII192: C2RustUnnamed_2 = 438;
pub const IMAGE_ASCII210: C2RustUnnamed_2 = 437;
pub const IMAGE_ASCII211: C2RustUnnamed_2 = 436;
pub const IMAGE_ASCII214: C2RustUnnamed_2 = 435;
pub const IMAGE_ASCII213: C2RustUnnamed_2 = 434;
pub const IMAGE_ASCII212: C2RustUnnamed_2 = 433;
pub const IMAGE_ASCII216: C2RustUnnamed_2 = 432;
pub const IMAGE_ASCII220: C2RustUnnamed_2 = 431;
pub const IMAGE_ASCII217: C2RustUnnamed_2 = 430;
pub const IMAGE_ASCII219: C2RustUnnamed_2 = 429;
pub const IMAGE_ASCII218: C2RustUnnamed_2 = 428;
pub const IMAGE_ASCII221: C2RustUnnamed_2 = 427;
pub const IMAGE_ASCII223: C2RustUnnamed_2 = 426;
pub const IMAGE_ASCII248: C2RustUnnamed_2 = 425;
pub const IMAGE_ASCII241: C2RustUnnamed_2 = 424;
pub const IMAGE_ASCII253: C2RustUnnamed_2 = 423;
pub const IMAGE_ASCII252: C2RustUnnamed_2 = 422;
pub const IMAGE_ASCII251: C2RustUnnamed_2 = 421;
pub const IMAGE_ASCII250: C2RustUnnamed_2 = 420;
pub const IMAGE_ASCII249: C2RustUnnamed_2 = 419;
pub const IMAGE_ASCII246: C2RustUnnamed_2 = 418;
pub const IMAGE_ASCII245: C2RustUnnamed_2 = 417;
pub const IMAGE_ASCII244: C2RustUnnamed_2 = 416;
pub const IMAGE_ASCII243: C2RustUnnamed_2 = 415;
pub const IMAGE_ASCII242: C2RustUnnamed_2 = 414;
pub const IMAGE_ASCII239: C2RustUnnamed_2 = 413;
pub const IMAGE_ASCII238: C2RustUnnamed_2 = 412;
pub const IMAGE_ASCII237: C2RustUnnamed_2 = 411;
pub const IMAGE_ASCII236: C2RustUnnamed_2 = 410;
pub const IMAGE_ASCII235: C2RustUnnamed_2 = 409;
pub const IMAGE_ASCII234: C2RustUnnamed_2 = 408;
pub const IMAGE_ASCII233: C2RustUnnamed_2 = 407;
pub const IMAGE_ASCII232: C2RustUnnamed_2 = 406;
pub const IMAGE_ASCII231: C2RustUnnamed_2 = 405;
pub const IMAGE_ASCII230: C2RustUnnamed_2 = 404;
pub const IMAGE_ASCII229: C2RustUnnamed_2 = 403;
pub const IMAGE_ASCII228: C2RustUnnamed_2 = 402;
pub const IMAGE_ASCII227: C2RustUnnamed_2 = 401;
pub const IMAGE_ASCII226: C2RustUnnamed_2 = 400;
pub const IMAGE_ASCII225: C2RustUnnamed_2 = 399;
pub const IMAGE_ASCII224: C2RustUnnamed_2 = 398;
pub const IMAGE_ASCII189: C2RustUnnamed_2 = 397;
pub const IMAGE_ASCII188: C2RustUnnamed_2 = 396;
pub const IMAGE_RES_DEFENCE: C2RustUnnamed_2 = 395;
pub const IMAGE_LEV_7: C2RustUnnamed_2 = 394;
pub const IMAGE_LEV_6: C2RustUnnamed_2 = 393;
pub const IMAGE_LEV_5: C2RustUnnamed_2 = 392;
pub const IMAGE_LEV_4: C2RustUnnamed_2 = 391;
pub const IMAGE_LEV_3: C2RustUnnamed_2 = 390;
pub const IMAGE_LEV_2: C2RustUnnamed_2 = 389;
pub const IMAGE_LEV_1: C2RustUnnamed_2 = 388;
pub const IMAGE_LEV_0: C2RustUnnamed_2 = 387;
pub const IMAGE_CDCHANGE_CANCEL: C2RustUnnamed_2 = 386;
pub const IMAGE_CDCHANGE_OK: C2RustUnnamed_2 = 385;
pub const IMAGE_MULTI_CHAN: C2RustUnnamed_2 = 384;
pub const IMAGE_MULTI_NOCHAN: C2RustUnnamed_2 = 383;
pub const IMAGE_ORD_EMBARKDOWN: C2RustUnnamed_2 = 382;
pub const IMAGE_ORD_EMBARKUP: C2RustUnnamed_2 = 381;
pub const IMAGE_ORD_RTRDOWN: C2RustUnnamed_2 = 380;
pub const IMAGE_ORD_RTRUP: C2RustUnnamed_2 = 379;
pub const IMAGE_ORD_GUARDDOWN: C2RustUnnamed_2 = 378;
pub const IMAGE_ORD_GUARDUP: C2RustUnnamed_2 = 377;
pub const IMAGE_ORD_PERSUEDOWN: C2RustUnnamed_2 = 376;
pub const IMAGE_ORD_PERSUEUP: C2RustUnnamed_2 = 375;
pub const IMAGE_ORD_PATROLDOWN: C2RustUnnamed_2 = 374;
pub const IMAGE_ORD_PATROLUP: C2RustUnnamed_2 = 373;
pub const IMAGE_GN_0: C2RustUnnamed_2 = 372;
pub const IMAGE_GN_1: C2RustUnnamed_2 = 371;
pub const IMAGE_GN_2: C2RustUnnamed_2 = 370;
pub const IMAGE_GN_3: C2RustUnnamed_2 = 369;
pub const IMAGE_GN_4: C2RustUnnamed_2 = 368;
pub const IMAGE_GN_5: C2RustUnnamed_2 = 367;
pub const IMAGE_GN_6: C2RustUnnamed_2 = 366;
pub const IMAGE_GN_7: C2RustUnnamed_2 = 365;
pub const IMAGE_GN_8: C2RustUnnamed_2 = 364;
pub const IMAGE_GN_9: C2RustUnnamed_2 = 363;
pub const IMAGE_GN_STAR: C2RustUnnamed_2 = 362;
pub const IMAGE_VDP_HI: C2RustUnnamed_2 = 361;
pub const IMAGE_VDP_UP: C2RustUnnamed_2 = 360;
pub const IMAGE_VDP_DOWN: C2RustUnnamed_2 = 359;
pub const IMAGE_FDP_HI: C2RustUnnamed_2 = 358;
pub const IMAGE_FDP_UP: C2RustUnnamed_2 = 357;
pub const IMAGE_FDP_DOWN: C2RustUnnamed_2 = 356;
pub const IMAGE_CDP_HI: C2RustUnnamed_2 = 355;
pub const IMAGE_CDP_UP: C2RustUnnamed_2 = 354;
pub const IMAGE_CDP_DOWN: C2RustUnnamed_2 = 353;
pub const IMAGE_LOOP_HI: C2RustUnnamed_2 = 352;
pub const IMAGE_LOOP_DOWN: C2RustUnnamed_2 = 351;
pub const IMAGE_LOOP_UP: C2RustUnnamed_2 = 350;
pub const IMAGE_RES_CYBORGTECH: C2RustUnnamed_2 = 349;
pub const IMAGE_CURSOR_DEST: C2RustUnnamed_2 = 348;
pub const IMAGE_DES_EDITBOXRIGHTH: C2RustUnnamed_2 = 347;
pub const IMAGE_DES_EDITBOXMIDH: C2RustUnnamed_2 = 346;
pub const IMAGE_DES_EDITBOXRIGHT: C2RustUnnamed_2 = 345;
pub const IMAGE_DES_EDITBOXMID: C2RustUnnamed_2 = 344;
pub const IMAGE_DES_POWERBAR_RIGHT: C2RustUnnamed_2 = 343;
pub const IMAGE_DES_POWERBAR_LEFT: C2RustUnnamed_2 = 342;
pub const IMAGE_AUDIO_LASTSAMPLEH: C2RustUnnamed_2 = 341;
pub const IMAGE_AUDIO_LASTSAMPLE: C2RustUnnamed_2 = 340;
pub const IMAGE_DES_BINH: C2RustUnnamed_2 = 339;
pub const IMAGE_DES_BIN: C2RustUnnamed_2 = 338;
pub const IMAGE_INFINITE_HI: C2RustUnnamed_2 = 337;
pub const IMAGE_INFINITE_DOWN: C2RustUnnamed_2 = 336;
pub const IMAGE_INFINITE_UP: C2RustUnnamed_2 = 335;
pub const IMAGE_RETICULE_GREY: C2RustUnnamed_2 = 334;
pub const IMAGE_ORD_FACHILITE: C2RustUnnamed_2 = 333;
pub const IMAGE_ORD_FAC5DOWN: C2RustUnnamed_2 = 332;
pub const IMAGE_ORD_FAC5UP: C2RustUnnamed_2 = 331;
pub const IMAGE_ORD_FAC4DOWN: C2RustUnnamed_2 = 330;
pub const IMAGE_ORD_FAC4UP: C2RustUnnamed_2 = 329;
pub const IMAGE_ORD_FAC3DOWN: C2RustUnnamed_2 = 328;
pub const IMAGE_ORD_FAC3UP: C2RustUnnamed_2 = 327;
pub const IMAGE_ORD_FAC2DOWN: C2RustUnnamed_2 = 326;
pub const IMAGE_ORD_FAC2UP: C2RustUnnamed_2 = 325;
pub const IMAGE_ORD_FAC1DOWN: C2RustUnnamed_2 = 324;
pub const IMAGE_ORD_FAC1UP: C2RustUnnamed_2 = 323;
pub const IMAGE_TRANSETA_UP: C2RustUnnamed_2 = 322;
pub const IMAGE_LAUNCHDOWN: C2RustUnnamed_2 = 321;
pub const IMAGE_MISSION_CLOCK: C2RustUnnamed_2 = 320;
pub const IMAGE_NRUTER: C2RustUnnamed_2 = 319;
pub const IMAGE_CMDDROID_EXP: C2RustUnnamed_2 = 318;
pub const IMAGE_MULTI_VIS_HI: C2RustUnnamed_2 = 317;
pub const IMAGE_MULTI_POW_HI: C2RustUnnamed_2 = 316;
pub const IMAGE_MULTI_DRO_HI: C2RustUnnamed_2 = 315;
pub const IMAGE_MULTI_TEK_HI: C2RustUnnamed_2 = 314;
pub const IMAGE_MULTI_TEK: C2RustUnnamed_2 = 313;
pub const IMAGE_MULTI_DRO: C2RustUnnamed_2 = 312;
pub const IMAGE_MULTI_POW: C2RustUnnamed_2 = 311;
pub const IMAGE_MULTI_VIS: C2RustUnnamed_2 = 310;
pub const IMAGE_MULTI_AL_HI: C2RustUnnamed_2 = 309;
pub const IMAGE_MULTI_OFFAL_HI: C2RustUnnamed_2 = 308;
pub const IMAGE_MULTI_NOAL_HI: C2RustUnnamed_2 = 307;
pub const IMAGE_MULTI_AL: C2RustUnnamed_2 = 306;
pub const IMAGE_MULTI_OFFAL: C2RustUnnamed_2 = 305;
pub const IMAGE_MULTI_NOAL: C2RustUnnamed_2 = 304;
pub const IMAGE_RAD_ART3: C2RustUnnamed_2 = 303;
pub const IMAGE_RAD_ART2: C2RustUnnamed_2 = 302;
pub const IMAGE_RAD_ART1: C2RustUnnamed_2 = 301;
pub const IMAGE_RAD_ARTREAD: C2RustUnnamed_2 = 300;
pub const IMAGE_RAD_RES3: C2RustUnnamed_2 = 299;
pub const IMAGE_RAD_RES2: C2RustUnnamed_2 = 298;
pub const IMAGE_RAD_RES1: C2RustUnnamed_2 = 297;
pub const IMAGE_RAD_RESREAD: C2RustUnnamed_2 = 296;
pub const IMAGE_RAD_ENM3: C2RustUnnamed_2 = 295;
pub const IMAGE_RAD_ENM2: C2RustUnnamed_2 = 294;
pub const IMAGE_RAD_ENM1: C2RustUnnamed_2 = 293;
pub const IMAGE_RAD_ENMREAD: C2RustUnnamed_2 = 292;
pub const IMAGE_SLIDER_INFINITY: C2RustUnnamed_2 = 291;
pub const IMAGE_SLIDER_BIGBUT: C2RustUnnamed_2 = 290;
pub const IMAGE_ORD_DESTRUCT2GREY: C2RustUnnamed_2 = 289;
pub const IMAGE_ORD_DESTRUCT2DOWN: C2RustUnnamed_2 = 288;
pub const IMAGE_ORD_DESTRUCT2UP: C2RustUnnamed_2 = 287;
pub const IMAGE_ORD_GOTOHQDOWN: C2RustUnnamed_2 = 286;
pub const IMAGE_ORD_GOTOHQUP: C2RustUnnamed_2 = 285;
pub const IMAGE_ORD_HALTDOWN: C2RustUnnamed_2 = 284;
pub const IMAGE_ORD_HALTUP: C2RustUnnamed_2 = 283;
pub const IMAGE_ORD_HOLDFIREDOWN: C2RustUnnamed_2 = 282;
pub const IMAGE_ORD_HOLDFIREUP: C2RustUnnamed_2 = 281;
pub const IMAGE_ORD_RETFIREDOWN: C2RustUnnamed_2 = 280;
pub const IMAGE_ORD_RETFIREUP: C2RustUnnamed_2 = 279;
pub const IMAGE_ORD_FATWILLDOWN: C2RustUnnamed_2 = 278;
pub const IMAGE_ORD_FATWILLUP: C2RustUnnamed_2 = 277;
pub const IMAGE_ORD_REPAIR3DOWN: C2RustUnnamed_2 = 276;
pub const IMAGE_ORD_REPAIR3UP: C2RustUnnamed_2 = 275;
pub const IMAGE_ORD_REPAIR1DOWN: C2RustUnnamed_2 = 274;
pub const IMAGE_ORD_REPAIR1UP: C2RustUnnamed_2 = 273;
pub const IMAGE_ORD_REPAIR2DOWN: C2RustUnnamed_2 = 272;
pub const IMAGE_ORD_REPAIR2UP: C2RustUnnamed_2 = 271;
pub const IMAGE_ORD_RANGE3DOWN: C2RustUnnamed_2 = 270;
pub const IMAGE_ORD_RANGE3UP: C2RustUnnamed_2 = 269;
pub const IMAGE_ORD_RANGE2DOWN: C2RustUnnamed_2 = 268;
pub const IMAGE_ORD_RANGE2UP: C2RustUnnamed_2 = 267;
pub const IMAGE_ORD_RANGE1DOWN: C2RustUnnamed_2 = 266;
pub const IMAGE_ORD_RANGE1UP: C2RustUnnamed_2 = 265;
pub const IMAGE_ORD_DESTRUCT1DOWN: C2RustUnnamed_2 = 264;
pub const IMAGE_ORD_DESTRUCT1UP: C2RustUnnamed_2 = 263;
pub const IMAGE_RES_QUESTIONMARK: C2RustUnnamed_2 = 262;
pub const IMAGE_RES_STRUCTURETECH: C2RustUnnamed_2 = 261;
pub const IMAGE_RES_SYSTEMTECH: C2RustUnnamed_2 = 260;
pub const IMAGE_RES_POWERTECH: C2RustUnnamed_2 = 259;
pub const IMAGE_RES_COMPUTERTECH: C2RustUnnamed_2 = 258;
pub const IMAGE_RES_WEAPONTECH: C2RustUnnamed_2 = 257;
pub const IMAGE_RES_DROIDTECH: C2RustUnnamed_2 = 256;
pub const IMAGE_QUESTION_MARK: C2RustUnnamed_2 = 255;
pub const IMAGE_RES_MAJOR_ROCKET: C2RustUnnamed_2 = 254;
pub const IMAGE_RES_MINOR_AUTOWEAPONS: C2RustUnnamed_2 = 253;
pub const IMAGE_TRACKS: C2RustUnnamed_2 = 252;
pub const IMAGE_STAR: C2RustUnnamed_2 = 251;
pub const IMAGE_SLIDER_BIG: C2RustUnnamed_2 = 250;
pub const IMAGE_DES_STATBACKMID: C2RustUnnamed_2 = 249;
pub const IMAGE_DES_STATBACKRIGHT: C2RustUnnamed_2 = 248;
pub const IMAGE_DES_STATBACKLEFT: C2RustUnnamed_2 = 247;
pub const IMAGE_SIDETABSEL: C2RustUnnamed_2 = 246;
pub const IMAGE_SIDETABDOWN: C2RustUnnamed_2 = 245;
pub const IMAGE_SIDETABHI: C2RustUnnamed_2 = 244;
pub const IMAGE_SIDETAB: C2RustUnnamed_2 = 243;
pub const IMAGE_CURSOR_NOTPOS: C2RustUnnamed_2 = 242;
pub const IMAGE_ASCII126: C2RustUnnamed_2 = 227;
pub const IMAGE_ASCII125: C2RustUnnamed_2 = 226;
pub const IMAGE_ASCII124: C2RustUnnamed_2 = 225;
pub const IMAGE_ASCII123: C2RustUnnamed_2 = 224;
pub const IMAGE_ASCII122: C2RustUnnamed_2 = 223;
pub const IMAGE_ASCII121: C2RustUnnamed_2 = 222;
pub const IMAGE_ASCII120: C2RustUnnamed_2 = 221;
pub const IMAGE_ASCII119: C2RustUnnamed_2 = 220;
pub const IMAGE_ASCII118: C2RustUnnamed_2 = 219;
pub const IMAGE_ASCII117: C2RustUnnamed_2 = 218;
pub const IMAGE_ASCII116: C2RustUnnamed_2 = 217;
pub const IMAGE_ASCII115: C2RustUnnamed_2 = 216;
pub const IMAGE_ASCII114: C2RustUnnamed_2 = 215;
pub const IMAGE_ASCII113: C2RustUnnamed_2 = 214;
pub const IMAGE_ASCII112: C2RustUnnamed_2 = 213;
pub const IMAGE_ASCII111: C2RustUnnamed_2 = 212;
pub const IMAGE_ASCII110: C2RustUnnamed_2 = 211;
pub const IMAGE_ASCII109: C2RustUnnamed_2 = 210;
pub const IMAGE_ASCII108: C2RustUnnamed_2 = 209;
pub const IMAGE_ASCII107: C2RustUnnamed_2 = 208;
pub const IMAGE_ASCII106: C2RustUnnamed_2 = 207;
pub const IMAGE_ASCII105: C2RustUnnamed_2 = 206;
pub const IMAGE_ASCII104: C2RustUnnamed_2 = 205;
pub const IMAGE_ASCII103: C2RustUnnamed_2 = 204;
pub const IMAGE_ASCII102: C2RustUnnamed_2 = 203;
pub const IMAGE_ASCII101: C2RustUnnamed_2 = 202;
pub const IMAGE_ASCII100: C2RustUnnamed_2 = 201;
pub const IMAGE_ASCII99: C2RustUnnamed_2 = 200;
pub const IMAGE_ASCII98: C2RustUnnamed_2 = 199;
pub const IMAGE_ASCII97: C2RustUnnamed_2 = 198;
pub const IMAGE_ASCII96: C2RustUnnamed_2 = 197;
pub const IMAGE_ASCII95: C2RustUnnamed_2 = 196;
pub const IMAGE_ASCII94: C2RustUnnamed_2 = 195;
pub const IMAGE_ASCII93: C2RustUnnamed_2 = 194;
pub const IMAGE_ASCII92: C2RustUnnamed_2 = 193;
pub const IMAGE_ASCII91: C2RustUnnamed_2 = 192;
pub const IMAGE_ASCII90: C2RustUnnamed_2 = 191;
pub const IMAGE_ASCII89: C2RustUnnamed_2 = 190;
pub const IMAGE_ASCII88: C2RustUnnamed_2 = 189;
pub const IMAGE_ASCII87: C2RustUnnamed_2 = 188;
pub const IMAGE_ASCII86: C2RustUnnamed_2 = 187;
pub const IMAGE_ASCII85: C2RustUnnamed_2 = 186;
pub const IMAGE_ASCII84: C2RustUnnamed_2 = 185;
pub const IMAGE_ASCII83: C2RustUnnamed_2 = 184;
pub const IMAGE_ASCII82: C2RustUnnamed_2 = 183;
pub const IMAGE_ASCII81: C2RustUnnamed_2 = 182;
pub const IMAGE_ASCII80: C2RustUnnamed_2 = 181;
pub const IMAGE_ASCII79: C2RustUnnamed_2 = 180;
pub const IMAGE_ASCII78: C2RustUnnamed_2 = 179;
pub const IMAGE_ASCII77: C2RustUnnamed_2 = 178;
pub const IMAGE_ASCII76: C2RustUnnamed_2 = 177;
pub const IMAGE_ASCII75: C2RustUnnamed_2 = 176;
pub const IMAGE_ASCII74: C2RustUnnamed_2 = 175;
pub const IMAGE_ASCII73: C2RustUnnamed_2 = 174;
pub const IMAGE_ASCII72: C2RustUnnamed_2 = 173;
pub const IMAGE_ASCII71: C2RustUnnamed_2 = 172;
pub const IMAGE_ASCII70: C2RustUnnamed_2 = 171;
pub const IMAGE_ASCII69: C2RustUnnamed_2 = 170;
pub const IMAGE_ASCII68: C2RustUnnamed_2 = 169;
pub const IMAGE_ASCII67: C2RustUnnamed_2 = 168;
pub const IMAGE_ASCII66: C2RustUnnamed_2 = 167;
pub const IMAGE_ASCII65: C2RustUnnamed_2 = 166;
pub const IMAGE_ASTERISK: C2RustUnnamed_2 = 165;
pub const IMAGE_ASCII63: C2RustUnnamed_2 = 164;
pub const IMAGE_ASCII62: C2RustUnnamed_2 = 163;
pub const IMAGE_ASCII61: C2RustUnnamed_2 = 162;
pub const IMAGE_ASCII60: C2RustUnnamed_2 = 161;
pub const IMAGE_ASCII59: C2RustUnnamed_2 = 160;
pub const IMAGE_ASCII58: C2RustUnnamed_2 = 159;
pub const IMAGE_ASCII57: C2RustUnnamed_2 = 158;
pub const IMAGE_ASCII56: C2RustUnnamed_2 = 157;
pub const IMAGE_ASCII55: C2RustUnnamed_2 = 156;
pub const IMAGE_ASCII54: C2RustUnnamed_2 = 155;
pub const IMAGE_ASCII53: C2RustUnnamed_2 = 154;
pub const IMAGE_ASCII52: C2RustUnnamed_2 = 153;
pub const IMAGE_ASCII51: C2RustUnnamed_2 = 152;
pub const IMAGE_ASCII50: C2RustUnnamed_2 = 151;
pub const IMAGE_ASCII49: C2RustUnnamed_2 = 150;
pub const IMAGE_ASCII48: C2RustUnnamed_2 = 149;
pub const IMAGE_ASCII47: C2RustUnnamed_2 = 148;
pub const IMAGE_ASCII46: C2RustUnnamed_2 = 147;
pub const IMAGE_ASCII45: C2RustUnnamed_2 = 146;
pub const IMAGE_ASCII44: C2RustUnnamed_2 = 145;
pub const IMAGE_ASCII43: C2RustUnnamed_2 = 144;
pub const IMAGE_ASCII42: C2RustUnnamed_2 = 143;
pub const IMAGE_ASCII41: C2RustUnnamed_2 = 142;
pub const IMAGE_ASCII40: C2RustUnnamed_2 = 141;
pub const IMAGE_ASCII39: C2RustUnnamed_2 = 140;
pub const IMAGE_ASCII38: C2RustUnnamed_2 = 139;
pub const IMAGE_ASCII37: C2RustUnnamed_2 = 138;
pub const IMAGE_ASCII36: C2RustUnnamed_2 = 137;
pub const IMAGE_ASCII35: C2RustUnnamed_2 = 136;
pub const IMAGE_ASCII34: C2RustUnnamed_2 = 135;
pub const IMAGE_ASCII33: C2RustUnnamed_2 = 134;
pub const IMAGE_INTEL_CAMPAIGNDOWN: C2RustUnnamed_2 = 133;
pub const IMAGE_INTEL_CAMPAIGN: C2RustUnnamed_2 = 132;
pub const IMAGE_INTEL_MISSIONDOWN: C2RustUnnamed_2 = 131;
pub const IMAGE_INTEL_MISSION: C2RustUnnamed_2 = 130;
pub const IMAGE_INTEL_RESEARCHDOWN: C2RustUnnamed_2 = 129;
pub const IMAGE_INTEL_RESEARCH: C2RustUnnamed_2 = 128;
pub const IMAGE_DES_BARYELLOW: C2RustUnnamed_2 = 127;
pub const IMAGE_DES_BARRED: C2RustUnnamed_2 = 126;
pub const IMAGE_DES_BARBLUE: C2RustUnnamed_2 = 125;
pub const IMAGE_DES_BARBACK: C2RustUnnamed_2 = 124;
pub const IMAGE_9: C2RustUnnamed_2 = 123;
pub const IMAGE_8: C2RustUnnamed_2 = 122;
pub const IMAGE_7: C2RustUnnamed_2 = 121;
pub const IMAGE_6: C2RustUnnamed_2 = 120;
pub const IMAGE_5: C2RustUnnamed_2 = 119;
pub const IMAGE_4: C2RustUnnamed_2 = 118;
pub const IMAGE_3: C2RustUnnamed_2 = 117;
pub const IMAGE_2: C2RustUnnamed_2 = 116;
pub const IMAGE_1: C2RustUnnamed_2 = 115;
pub const IMAGE_0: C2RustUnnamed_2 = 114;
pub const IMAGE_RES_MAJOR_HOVER: C2RustUnnamed_2 = 113;
pub const IMAGE_RES_MAJOR_HEAVYWEP: C2RustUnnamed_2 = 112;
pub const IMAGE_RES_MAJOR_ELECTRONIC: C2RustUnnamed_2 = 111;
pub const IMAGE_RES_MAJOR_PLASCRETE: C2RustUnnamed_2 = 110;
pub const IMAGE_RES_MINOR_RADAR: C2RustUnnamed_2 = 109;
pub const IMAGE_PLASCRETE: C2RustUnnamed_2 = 108;
pub const IMAGE_ECM: C2RustUnnamed_2 = 107;
pub const IMAGE_RES_MINOR_PLASSTEEL: C2RustUnnamed_2 = 106;
pub const IMAGE_HOVERCRAFT: C2RustUnnamed_2 = 105;
pub const IMAGE_CANNON: C2RustUnnamed_2 = 104;
pub const IMAGE_ROCKET: C2RustUnnamed_2 = 103;
pub const IMAGE_DES_BODYPOINTS: C2RustUnnamed_2 = 102;
pub const IMAGE_DES_FIRERATE: C2RustUnnamed_2 = 101;
pub const IMAGE_DES_HOVER: C2RustUnnamed_2 = 100;
pub const IMAGE_DES_POWER: C2RustUnnamed_2 = 99;
pub const IMAGE_DES_DAMAGE: C2RustUnnamed_2 = 98;
pub const IMAGE_DES_WEIGHT: C2RustUnnamed_2 = 97;
pub const IMAGE_DES_ROAD: C2RustUnnamed_2 = 96;
pub const IMAGE_DES_CROSSCOUNTRY: C2RustUnnamed_2 = 95;
pub const IMAGE_DES_ARMOUR_EXPLOSIVE: C2RustUnnamed_2 = 94;
pub const IMAGE_DES_RANGE: C2RustUnnamed_2 = 93;
pub const IMAGE_DES_BUILDRATE: C2RustUnnamed_2 = 92;
pub const IMAGE_DES_TABWEAPONDOWN: C2RustUnnamed_2 = 91;
pub const IMAGE_DES_TABWEAPON: C2RustUnnamed_2 = 90;
pub const IMAGE_DES_EXTRAHI: C2RustUnnamed_2 = 89;
pub const IMAGE_DES_STATSCOMP: C2RustUnnamed_2 = 88;
pub const IMAGE_DES_BACK: C2RustUnnamed_2 = 87;
pub const IMAGE_FRAME_HC3: C2RustUnnamed_2 = 86;
pub const IMAGE_FRAME_HC2: C2RustUnnamed_2 = 85;
pub const IMAGE_FRAME_HC1: C2RustUnnamed_2 = 84;
pub const IMAGE_FRAME_HC0: C2RustUnnamed_2 = 83;
pub const IMAGE_FRAME_VC3: C2RustUnnamed_2 = 82;
pub const IMAGE_FRAME_VC2: C2RustUnnamed_2 = 81;
pub const IMAGE_FRAME_VC1: C2RustUnnamed_2 = 80;
pub const IMAGE_FRAME_VC0: C2RustUnnamed_2 = 79;
pub const IMAGE_DES_EDITBOXLEFTH: C2RustUnnamed_2 = 78;
pub const IMAGE_DES_EDITBOXLEFT: C2RustUnnamed_2 = 77;
pub const IMAGE_DES_POWERBACK: C2RustUnnamed_2 = 76;
pub const IMAGE_DES_STATSCURR: C2RustUnnamed_2 = 75;
pub const IMAGE_DES_STATSBACK: C2RustUnnamed_2 = 74;
pub const IMAGE_DES_POWERCURR: C2RustUnnamed_2 = 73;
pub const IMAGE_FRAME_HBH: C2RustUnnamed_2 = 72;
pub const IMAGE_FRAME_HTH: C2RustUnnamed_2 = 71;
pub const IMAGE_FRAME_HB2: C2RustUnnamed_2 = 70;
pub const IMAGE_FRAME_HT2: C2RustUnnamed_2 = 69;
pub const IMAGE_DES_COMMANDDOWN: C2RustUnnamed_2 = 68;
pub const IMAGE_DES_COMMAND: C2RustUnnamed_2 = 67;
pub const IMAGE_DES_SYSTEMSDOWN: C2RustUnnamed_2 = 66;
pub const IMAGE_DES_SYSTEMS: C2RustUnnamed_2 = 65;
pub const IMAGE_DES_WEAPONSDOWN: C2RustUnnamed_2 = 64;
pub const IMAGE_DES_WEAPONS: C2RustUnnamed_2 = 63;
pub const IMAGE_FRAME_VRH: C2RustUnnamed_2 = 62;
pub const IMAGE_FRAME_VLH: C2RustUnnamed_2 = 61;
pub const IMAGE_FRAME_VR2: C2RustUnnamed_2 = 60;
pub const IMAGE_FRAME_VL2: C2RustUnnamed_2 = 59;
pub const IMAGE_DES_HILIGHT: C2RustUnnamed_2 = 58;
pub const IMAGE_DES_PROPULSIONH: C2RustUnnamed_2 = 57;
pub const IMAGE_DES_PROPULSION: C2RustUnnamed_2 = 56;
pub const IMAGE_DES_BODYH: C2RustUnnamed_2 = 55;
pub const IMAGE_DES_BODY: C2RustUnnamed_2 = 54;
pub const IMAGE_DES_TURRETH: C2RustUnnamed_2 = 53;
pub const IMAGE_DES_TURRET: C2RustUnnamed_2 = 52;
pub const IMAGE_FRAME_VR: C2RustUnnamed_2 = 51;
pub const IMAGE_FRAME_VL: C2RustUnnamed_2 = 50;
pub const IMAGE_SLIDER_BACK: C2RustUnnamed_2 = 49;
pub const IMAGE_FRAME_HB: C2RustUnnamed_2 = 48;
pub const IMAGE_FRAME_HT: C2RustUnnamed_2 = 47;
pub const IMAGE_SLIDER_BUT: C2RustUnnamed_2 = 46;
pub const IMAGE_CLOSEHILIGHT: C2RustUnnamed_2 = 45;
pub const IMAGE_CLOSEDOWN: C2RustUnnamed_2 = 44;
pub const IMAGE_CLOSE: C2RustUnnamed_2 = 43;
pub const IMAGE_CANCEL_HILIGHT: C2RustUnnamed_2 = 42;
pub const IMAGE_CANCEL_DOWN: C2RustUnnamed_2 = 41;
pub const IMAGE_CANCEL_UP: C2RustUnnamed_2 = 40;
pub const IMAGE_RETICULE_BUTDOWN: C2RustUnnamed_2 = 39;
pub const IMAGE_RETICULE_HILIGHT: C2RustUnnamed_2 = 38;
pub const IMAGE_MANUFACTURE_DOWN: C2RustUnnamed_2 = 37;
pub const IMAGE_MANUFACTURE_UP: C2RustUnnamed_2 = 36;
pub const IMAGE_RESEARCH_DOWN: C2RustUnnamed_2 = 35;
pub const IMAGE_RESEARCH_UP: C2RustUnnamed_2 = 34;
pub const IMAGE_BUILD_DOWN: C2RustUnnamed_2 = 33;
pub const IMAGE_BUILD_UP: C2RustUnnamed_2 = 32;
pub const IMAGE_DESIGN_DOWN: C2RustUnnamed_2 = 31;
pub const IMAGE_DESIGN_UP: C2RustUnnamed_2 = 30;
pub const IMAGE_COMMANDDROID_DOWN: C2RustUnnamed_2 = 29;
pub const IMAGE_COMMANDDROID_UP: C2RustUnnamed_2 = 28;
pub const IMAGE_INTELMAP_DOWN: C2RustUnnamed_2 = 27;
pub const IMAGE_INTELMAP_UP: C2RustUnnamed_2 = 26;
pub const IMAGE_PBAR_BOTTOM: C2RustUnnamed_2 = 25;
pub const IMAGE_PBAR_TOP: C2RustUnnamed_2 = 24;
pub const IMAGE_FRAME_C3: C2RustUnnamed_2 = 23;
pub const IMAGE_FRAME_C2: C2RustUnnamed_2 = 22;
pub const IMAGE_FRAME_C1: C2RustUnnamed_2 = 21;
pub const IMAGE_FRAME_C0: C2RustUnnamed_2 = 20;
pub const IMAGE_TABHILIGHT: C2RustUnnamed_2 = 19;
pub const IMAGE_TABSELECTED: C2RustUnnamed_2 = 18;
pub const IMAGE_TAB4DOWN: C2RustUnnamed_2 = 17;
pub const IMAGE_TAB3DOWN: C2RustUnnamed_2 = 16;
pub const IMAGE_TAB2DOWN: C2RustUnnamed_2 = 15;
pub const IMAGE_TAB1DOWN: C2RustUnnamed_2 = 14;
pub const IMAGE_TAB4: C2RustUnnamed_2 = 13;
pub const IMAGE_TAB3: C2RustUnnamed_2 = 12;
pub const IMAGE_TAB2: C2RustUnnamed_2 = 11;
pub const IMAGE_TAB1: C2RustUnnamed_2 = 10;
pub const IMAGE_BUTB_HILITE: C2RustUnnamed_2 = 9;
pub const IMAGE_BUT_HILITE: C2RustUnnamed_2 = 8;
pub const IMAGE_BUTB0_DOWN: C2RustUnnamed_2 = 7;
pub const IMAGE_BUTB0_UP: C2RustUnnamed_2 = 6;
pub const IMAGE_BUT0_DOWN: C2RustUnnamed_2 = 5;
pub const IMAGE_BUT0_UP: C2RustUnnamed_2 = 4;
pub const IMAGE_PBAR_REQUIRED: C2RustUnnamed_2 = 3;
pub const IMAGE_PBAR_USED: C2RustUnnamed_2 = 2;
pub const IMAGE_PBAR_AVAIL: C2RustUnnamed_2 = 1;
pub const IMAGE_PBAR_EMPTY: C2RustUnnamed_2 = 0;
/*
 * ScriptTabs.c
 *
 * All the tables for the script compiler
 *
 */
// Get all the function prototypes
//#include "mission.h"
/* The table of user defined types
 * The format is :
 *         <type id no.>,  "Type name",   <access type>
 *
 * The type id no. should start at VAL_USERTYPESTART and increase by one per type.
 * The access type controls whether the type is a simple data type or an object type.
 */
#[no_mangle]
pub static mut asTypeTable: [TYPE_SYMBOL; 29] =
    unsafe {
        [{
             let mut init =
                 _type_symbol{typeID:
                                  ST_INTMESSAGE as libc::c_int as INTERP_TYPE
                                      as SWORD,
                              accessType: AT_SIMPLE as libc::c_int as SWORD,
                              pIdent:
                                  b"INTMESSAGE\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              saveFunc:
                                  Some(scrValDefSave as
                                           unsafe extern "C" fn(_:
                                                                    INTERP_TYPE,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut libc::c_char,
                                                                _:
                                                                    *mut UDWORD)
                                               -> BOOL),
                              loadFunc:
                                  Some(scrValDefLoad as
                                           unsafe extern "C" fn(_: SDWORD,
                                                                _:
                                                                    INTERP_TYPE,
                                                                _:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut UDWORD)
                                               -> BOOL),};
             init
         },
         {
             let mut init =
                 _type_symbol{typeID:
                                  ST_BASEOBJECT as libc::c_int as INTERP_TYPE
                                      as SWORD,
                              accessType: AT_OBJECT as libc::c_int as SWORD,
                              pIdent:
                                  b"BASEOBJ\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              saveFunc:
                                  Some(scrValDefSave as
                                           unsafe extern "C" fn(_:
                                                                    INTERP_TYPE,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut libc::c_char,
                                                                _:
                                                                    *mut UDWORD)
                                               -> BOOL),
                              loadFunc:
                                  Some(scrValDefLoad as
                                           unsafe extern "C" fn(_: SDWORD,
                                                                _:
                                                                    INTERP_TYPE,
                                                                _:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut UDWORD)
                                               -> BOOL),};
             init
         },
         {
             let mut init =
                 _type_symbol{typeID:
                                  ST_DROID as libc::c_int as INTERP_TYPE as
                                      SWORD,
                              accessType: AT_OBJECT as libc::c_int as SWORD,
                              pIdent:
                                  b"DROID\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              saveFunc:
                                  Some(scrValDefSave as
                                           unsafe extern "C" fn(_:
                                                                    INTERP_TYPE,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut libc::c_char,
                                                                _:
                                                                    *mut UDWORD)
                                               -> BOOL),
                              loadFunc:
                                  Some(scrValDefLoad as
                                           unsafe extern "C" fn(_: SDWORD,
                                                                _:
                                                                    INTERP_TYPE,
                                                                _:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut UDWORD)
                                               -> BOOL),};
             init
         },
         {
             let mut init =
                 _type_symbol{typeID:
                                  ST_STRUCTURE as libc::c_int as INTERP_TYPE
                                      as SWORD,
                              accessType: AT_OBJECT as libc::c_int as SWORD,
                              pIdent:
                                  b"STRUCTURE\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              saveFunc:
                                  Some(scrValDefSave as
                                           unsafe extern "C" fn(_:
                                                                    INTERP_TYPE,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut libc::c_char,
                                                                _:
                                                                    *mut UDWORD)
                                               -> BOOL),
                              loadFunc:
                                  Some(scrValDefLoad as
                                           unsafe extern "C" fn(_: SDWORD,
                                                                _:
                                                                    INTERP_TYPE,
                                                                _:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut UDWORD)
                                               -> BOOL),};
             init
         },
         {
             let mut init =
                 _type_symbol{typeID:
                                  ST_FEATURE as libc::c_int as INTERP_TYPE as
                                      SWORD,
                              accessType: AT_OBJECT as libc::c_int as SWORD,
                              pIdent:
                                  b"FEATURE\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              saveFunc:
                                  Some(scrValDefSave as
                                           unsafe extern "C" fn(_:
                                                                    INTERP_TYPE,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut libc::c_char,
                                                                _:
                                                                    *mut UDWORD)
                                               -> BOOL),
                              loadFunc:
                                  Some(scrValDefLoad as
                                           unsafe extern "C" fn(_: SDWORD,
                                                                _:
                                                                    INTERP_TYPE,
                                                                _:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut UDWORD)
                                               -> BOOL),};
             init
         },
         {
             let mut init =
                 _type_symbol{typeID:
                                  ST_BASESTATS as libc::c_int as INTERP_TYPE
                                      as SWORD,
                              accessType: AT_SIMPLE as libc::c_int as SWORD,
                              pIdent:
                                  b"BASESTATS\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              saveFunc:
                                  Some(scrValDefSave as
                                           unsafe extern "C" fn(_:
                                                                    INTERP_TYPE,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut libc::c_char,
                                                                _:
                                                                    *mut UDWORD)
                                               -> BOOL),
                              loadFunc:
                                  Some(scrValDefLoad as
                                           unsafe extern "C" fn(_: SDWORD,
                                                                _:
                                                                    INTERP_TYPE,
                                                                _:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut UDWORD)
                                               -> BOOL),};
             init
         },
         {
             let mut init =
                 _type_symbol{typeID:
                                  ST_COMPONENT as libc::c_int as INTERP_TYPE
                                      as SWORD,
                              accessType: AT_SIMPLE as libc::c_int as SWORD,
                              pIdent:
                                  b"COMPONENT\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              saveFunc:
                                  Some(scrValDefSave as
                                           unsafe extern "C" fn(_:
                                                                    INTERP_TYPE,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut libc::c_char,
                                                                _:
                                                                    *mut UDWORD)
                                               -> BOOL),
                              loadFunc:
                                  Some(scrValDefLoad as
                                           unsafe extern "C" fn(_: SDWORD,
                                                                _:
                                                                    INTERP_TYPE,
                                                                _:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut UDWORD)
                                               -> BOOL),};
             init
         },
         {
             let mut init =
                 _type_symbol{typeID:
                                  ST_BODY as libc::c_int as INTERP_TYPE as
                                      SWORD,
                              accessType: AT_SIMPLE as libc::c_int as SWORD,
                              pIdent:
                                  b"BODY\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              saveFunc:
                                  Some(scrValDefSave as
                                           unsafe extern "C" fn(_:
                                                                    INTERP_TYPE,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut libc::c_char,
                                                                _:
                                                                    *mut UDWORD)
                                               -> BOOL),
                              loadFunc:
                                  Some(scrValDefLoad as
                                           unsafe extern "C" fn(_: SDWORD,
                                                                _:
                                                                    INTERP_TYPE,
                                                                _:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut UDWORD)
                                               -> BOOL),};
             init
         },
         {
             let mut init =
                 _type_symbol{typeID:
                                  ST_PROPULSION as libc::c_int as INTERP_TYPE
                                      as SWORD,
                              accessType: AT_SIMPLE as libc::c_int as SWORD,
                              pIdent:
                                  b"PROPULSION\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              saveFunc:
                                  Some(scrValDefSave as
                                           unsafe extern "C" fn(_:
                                                                    INTERP_TYPE,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut libc::c_char,
                                                                _:
                                                                    *mut UDWORD)
                                               -> BOOL),
                              loadFunc:
                                  Some(scrValDefLoad as
                                           unsafe extern "C" fn(_: SDWORD,
                                                                _:
                                                                    INTERP_TYPE,
                                                                _:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut UDWORD)
                                               -> BOOL),};
             init
         },
         {
             let mut init =
                 _type_symbol{typeID:
                                  ST_ECM as libc::c_int as INTERP_TYPE as
                                      SWORD,
                              accessType: AT_SIMPLE as libc::c_int as SWORD,
                              pIdent:
                                  b"ECM\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              saveFunc:
                                  Some(scrValDefSave as
                                           unsafe extern "C" fn(_:
                                                                    INTERP_TYPE,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut libc::c_char,
                                                                _:
                                                                    *mut UDWORD)
                                               -> BOOL),
                              loadFunc:
                                  Some(scrValDefLoad as
                                           unsafe extern "C" fn(_: SDWORD,
                                                                _:
                                                                    INTERP_TYPE,
                                                                _:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut UDWORD)
                                               -> BOOL),};
             init
         },
         {
             let mut init =
                 _type_symbol{typeID:
                                  ST_SENSOR as libc::c_int as INTERP_TYPE as
                                      SWORD,
                              accessType: AT_SIMPLE as libc::c_int as SWORD,
                              pIdent:
                                  b"SENSOR\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              saveFunc:
                                  Some(scrValDefSave as
                                           unsafe extern "C" fn(_:
                                                                    INTERP_TYPE,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut libc::c_char,
                                                                _:
                                                                    *mut UDWORD)
                                               -> BOOL),
                              loadFunc:
                                  Some(scrValDefLoad as
                                           unsafe extern "C" fn(_: SDWORD,
                                                                _:
                                                                    INTERP_TYPE,
                                                                _:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut UDWORD)
                                               -> BOOL),};
             init
         },
         {
             let mut init =
                 _type_symbol{typeID:
                                  ST_CONSTRUCT as libc::c_int as INTERP_TYPE
                                      as SWORD,
                              accessType: AT_SIMPLE as libc::c_int as SWORD,
                              pIdent:
                                  b"CONSTRUCT\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              saveFunc:
                                  Some(scrValDefSave as
                                           unsafe extern "C" fn(_:
                                                                    INTERP_TYPE,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut libc::c_char,
                                                                _:
                                                                    *mut UDWORD)
                                               -> BOOL),
                              loadFunc:
                                  Some(scrValDefLoad as
                                           unsafe extern "C" fn(_: SDWORD,
                                                                _:
                                                                    INTERP_TYPE,
                                                                _:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut UDWORD)
                                               -> BOOL),};
             init
         },
         {
             let mut init =
                 _type_symbol{typeID:
                                  ST_WEAPON as libc::c_int as INTERP_TYPE as
                                      SWORD,
                              accessType: AT_SIMPLE as libc::c_int as SWORD,
                              pIdent:
                                  b"WEAPON\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              saveFunc:
                                  Some(scrValDefSave as
                                           unsafe extern "C" fn(_:
                                                                    INTERP_TYPE,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut libc::c_char,
                                                                _:
                                                                    *mut UDWORD)
                                               -> BOOL),
                              loadFunc:
                                  Some(scrValDefLoad as
                                           unsafe extern "C" fn(_: SDWORD,
                                                                _:
                                                                    INTERP_TYPE,
                                                                _:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut UDWORD)
                                               -> BOOL),};
             init
         },
         {
             let mut init =
                 _type_symbol{typeID:
                                  ST_REPAIR as libc::c_int as INTERP_TYPE as
                                      SWORD,
                              accessType: AT_SIMPLE as libc::c_int as SWORD,
                              pIdent:
                                  b"REPAIR\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              saveFunc:
                                  Some(scrValDefSave as
                                           unsafe extern "C" fn(_:
                                                                    INTERP_TYPE,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut libc::c_char,
                                                                _:
                                                                    *mut UDWORD)
                                               -> BOOL),
                              loadFunc:
                                  Some(scrValDefLoad as
                                           unsafe extern "C" fn(_: SDWORD,
                                                                _:
                                                                    INTERP_TYPE,
                                                                _:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut UDWORD)
                                               -> BOOL),};
             init
         },
         {
             let mut init =
                 _type_symbol{typeID:
                                  ST_BRAIN as libc::c_int as INTERP_TYPE as
                                      SWORD,
                              accessType: AT_SIMPLE as libc::c_int as SWORD,
                              pIdent:
                                  b"BRAIN\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              saveFunc:
                                  Some(scrValDefSave as
                                           unsafe extern "C" fn(_:
                                                                    INTERP_TYPE,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut libc::c_char,
                                                                _:
                                                                    *mut UDWORD)
                                               -> BOOL),
                              loadFunc:
                                  Some(scrValDefLoad as
                                           unsafe extern "C" fn(_: SDWORD,
                                                                _:
                                                                    INTERP_TYPE,
                                                                _:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut UDWORD)
                                               -> BOOL),};
             init
         },
         {
             let mut init =
                 _type_symbol{typeID:
                                  ST_TEMPLATE as libc::c_int as INTERP_TYPE as
                                      SWORD,
                              accessType: AT_SIMPLE as libc::c_int as SWORD,
                              pIdent:
                                  b"TEMPLATE\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              saveFunc:
                                  Some(scrValDefSave as
                                           unsafe extern "C" fn(_:
                                                                    INTERP_TYPE,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut libc::c_char,
                                                                _:
                                                                    *mut UDWORD)
                                               -> BOOL),
                              loadFunc:
                                  Some(scrValDefLoad as
                                           unsafe extern "C" fn(_: SDWORD,
                                                                _:
                                                                    INTERP_TYPE,
                                                                _:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut UDWORD)
                                               -> BOOL),};
             init
         },
         {
             let mut init =
                 _type_symbol{typeID:
                                  ST_STRUCTUREID as libc::c_int as INTERP_TYPE
                                      as SWORD,
                              accessType: AT_SIMPLE as libc::c_int as SWORD,
                              pIdent:
                                  b"STRUCTUREID\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              saveFunc:
                                  Some(scrValDefSave as
                                           unsafe extern "C" fn(_:
                                                                    INTERP_TYPE,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut libc::c_char,
                                                                _:
                                                                    *mut UDWORD)
                                               -> BOOL),
                              loadFunc:
                                  Some(scrValDefLoad as
                                           unsafe extern "C" fn(_: SDWORD,
                                                                _:
                                                                    INTERP_TYPE,
                                                                _:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut UDWORD)
                                               -> BOOL),};
             init
         },
         {
             let mut init =
                 _type_symbol{typeID:
                                  ST_STRUCTURESTAT as libc::c_int as
                                      INTERP_TYPE as SWORD,
                              accessType: AT_SIMPLE as libc::c_int as SWORD,
                              pIdent:
                                  b"STRUCTURESTAT\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              saveFunc:
                                  Some(scrValDefSave as
                                           unsafe extern "C" fn(_:
                                                                    INTERP_TYPE,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut libc::c_char,
                                                                _:
                                                                    *mut UDWORD)
                                               -> BOOL),
                              loadFunc:
                                  Some(scrValDefLoad as
                                           unsafe extern "C" fn(_: SDWORD,
                                                                _:
                                                                    INTERP_TYPE,
                                                                _:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut UDWORD)
                                               -> BOOL),};
             init
         },
         {
             let mut init =
                 _type_symbol{typeID:
                                  ST_FEATURESTAT as libc::c_int as INTERP_TYPE
                                      as SWORD,
                              accessType: AT_SIMPLE as libc::c_int as SWORD,
                              pIdent:
                                  b"FEATURESTAT\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              saveFunc:
                                  Some(scrValDefSave as
                                           unsafe extern "C" fn(_:
                                                                    INTERP_TYPE,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut libc::c_char,
                                                                _:
                                                                    *mut UDWORD)
                                               -> BOOL),
                              loadFunc:
                                  Some(scrValDefLoad as
                                           unsafe extern "C" fn(_: SDWORD,
                                                                _:
                                                                    INTERP_TYPE,
                                                                _:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut UDWORD)
                                               -> BOOL),};
             init
         },
         {
             let mut init =
                 _type_symbol{typeID:
                                  ST_DROIDID as libc::c_int as INTERP_TYPE as
                                      SWORD,
                              accessType: AT_SIMPLE as libc::c_int as SWORD,
                              pIdent:
                                  b"DROIDID\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              saveFunc:
                                  Some(scrValDefSave as
                                           unsafe extern "C" fn(_:
                                                                    INTERP_TYPE,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut libc::c_char,
                                                                _:
                                                                    *mut UDWORD)
                                               -> BOOL),
                              loadFunc:
                                  Some(scrValDefLoad as
                                           unsafe extern "C" fn(_: SDWORD,
                                                                _:
                                                                    INTERP_TYPE,
                                                                _:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut UDWORD)
                                               -> BOOL),};
             init
         },
         {
             let mut init =
                 _type_symbol{typeID:
                                  ST_TEXTSTRING as libc::c_int as INTERP_TYPE
                                      as SWORD,
                              accessType: AT_SIMPLE as libc::c_int as SWORD,
                              pIdent:
                                  b"TEXTSTRING\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              saveFunc:
                                  Some(scrValDefSave as
                                           unsafe extern "C" fn(_:
                                                                    INTERP_TYPE,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut libc::c_char,
                                                                _:
                                                                    *mut UDWORD)
                                               -> BOOL),
                              loadFunc:
                                  Some(scrValDefLoad as
                                           unsafe extern "C" fn(_: SDWORD,
                                                                _:
                                                                    INTERP_TYPE,
                                                                _:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut UDWORD)
                                               -> BOOL),};
             init
         },
         {
             let mut init =
                 _type_symbol{typeID:
                                  ST_SOUND as libc::c_int as INTERP_TYPE as
                                      SWORD,
                              accessType: AT_SIMPLE as libc::c_int as SWORD,
                              pIdent:
                                  b"SOUND\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              saveFunc:
                                  Some(scrValDefSave as
                                           unsafe extern "C" fn(_:
                                                                    INTERP_TYPE,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut libc::c_char,
                                                                _:
                                                                    *mut UDWORD)
                                               -> BOOL),
                              loadFunc:
                                  Some(scrValDefLoad as
                                           unsafe extern "C" fn(_: SDWORD,
                                                                _:
                                                                    INTERP_TYPE,
                                                                _:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut UDWORD)
                                               -> BOOL),};
             init
         },
         {
             let mut init =
                 _type_symbol{typeID:
                                  ST_LEVEL as libc::c_int as INTERP_TYPE as
                                      SWORD,
                              accessType: AT_SIMPLE as libc::c_int as SWORD,
                              pIdent:
                                  b"LEVEL\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              saveFunc:
                                  Some(scrValDefSave as
                                           unsafe extern "C" fn(_:
                                                                    INTERP_TYPE,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut libc::c_char,
                                                                _:
                                                                    *mut UDWORD)
                                               -> BOOL),
                              loadFunc:
                                  Some(scrValDefLoad as
                                           unsafe extern "C" fn(_: SDWORD,
                                                                _:
                                                                    INTERP_TYPE,
                                                                _:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut UDWORD)
                                               -> BOOL),};
             init
         },
         {
             let mut init =
                 _type_symbol{typeID:
                                  ST_GROUP as libc::c_int as INTERP_TYPE as
                                      SWORD,
                              accessType: AT_OBJECT as libc::c_int as SWORD,
                              pIdent:
                                  b"GROUP\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              saveFunc:
                                  Some(scrValDefSave as
                                           unsafe extern "C" fn(_:
                                                                    INTERP_TYPE,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut libc::c_char,
                                                                _:
                                                                    *mut UDWORD)
                                               -> BOOL),
                              loadFunc:
                                  Some(scrValDefLoad as
                                           unsafe extern "C" fn(_: SDWORD,
                                                                _:
                                                                    INTERP_TYPE,
                                                                _:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut UDWORD)
                                               -> BOOL),};
             init
         },
         {
             let mut init =
                 _type_symbol{typeID:
                                  ST_RESEARCH as libc::c_int as INTERP_TYPE as
                                      SWORD,
                              accessType: AT_SIMPLE as libc::c_int as SWORD,
                              pIdent:
                                  b"RESEARCHSTAT\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              saveFunc:
                                  Some(scrValDefSave as
                                           unsafe extern "C" fn(_:
                                                                    INTERP_TYPE,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut libc::c_char,
                                                                _:
                                                                    *mut UDWORD)
                                               -> BOOL),
                              loadFunc:
                                  Some(scrValDefLoad as
                                           unsafe extern "C" fn(_: SDWORD,
                                                                _:
                                                                    INTERP_TYPE,
                                                                _:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut UDWORD)
                                               -> BOOL),};
             init
         },
         {
             let mut init =
                 _type_symbol{typeID:
                                  ST_POINTER_O as libc::c_int as INTERP_TYPE
                                      as SWORD,
                              accessType: AT_OBJECT as libc::c_int as SWORD,
                              pIdent:
                                  b"\x00" as *const u8 as *const libc::c_char
                                      as *mut STRING,
                              saveFunc: None,
                              loadFunc: None,};
             init
         },
         {
             let mut init =
                 _type_symbol{typeID:
                                  ST_POINTER_T as libc::c_int as INTERP_TYPE
                                      as SWORD,
                              accessType: AT_SIMPLE as libc::c_int as SWORD,
                              pIdent:
                                  b"\x00" as *const u8 as *const libc::c_char
                                      as *mut STRING,
                              saveFunc: None,
                              loadFunc: None,};
             init
         },
         {
             let mut init =
                 _type_symbol{typeID:
                                  ST_POINTER_S as libc::c_int as INTERP_TYPE
                                      as SWORD,
                              accessType: AT_SIMPLE as libc::c_int as SWORD,
                              pIdent:
                                  b"\x00" as *const u8 as *const libc::c_char
                                      as *mut STRING,
                              saveFunc: None,
                              loadFunc: None,};
             init
         },
         {
             let mut init =
                 _type_symbol{typeID: 0 as libc::c_int as SWORD,
                              accessType: AT_SIMPLE as libc::c_int as SWORD,
                              pIdent:
                                  b"END OF TYPE LIST\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              saveFunc: None,
                              loadFunc: None,};
             init
         }]
    };
/* The table of script callable C functions
 * This is laid out :
 *     "ScriptName",   function_Pointer,  <function return type>
 *      <number of parameters>,  <parameter types>,  FALSE
 */
#[no_mangle]
pub static mut asFuncTable: [FUNC_SYMBOL; 269] =
    unsafe {
        [{
             let mut init =
                 _func_symbol{pIdent:
                                  b"InitEnumDroids\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrInitEnumDroids as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"EnumDroid\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrEnumDroid as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: ST_DROID as libc::c_int as INTERP_TYPE,
                              numParams: 0 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_VOID, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"traceOn\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(interpTraceOn as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 0 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_VOID, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"traceOff\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(interpTraceOff as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 0 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_VOID, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"setEventTrigger\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(eventSetTrigger as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_EVENT, VAL_TRIGGER, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"eventTraceLevel\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(eventSetTraceLevel as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"objectInRange\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrObjectInRange as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_BOOL,
                              numParams: 4 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_INT, VAL_INT,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"droidInRange\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrDroidInRange as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_BOOL,
                              numParams: 4 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_INT, VAL_INT,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"structInRange\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrStructInRange as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_BOOL,
                              numParams: 4 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_INT, VAL_INT,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"playerPower\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrPlayerPower as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_INT,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"objectInArea\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrObjectInArea as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_BOOL,
                              numParams: 5 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_INT, VAL_INT,
                                   VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"droidInArea\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrDroidInArea as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_BOOL,
                              numParams: 5 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_INT, VAL_INT,
                                   VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"structInArea\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrStructInArea as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_BOOL,
                              numParams: 5 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_INT, VAL_INT,
                                   VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"seenStructInArea\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrSeenStructInArea as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_BOOL,
                              numParams: 7 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_BOOL, VAL_INT,
                                   VAL_INT, VAL_INT, VAL_INT, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"structButNoWallsInArea\x00" as *const u8
                                      as *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrStructButNoWallsInArea as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_BOOL,
                              numParams: 5 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_INT, VAL_INT,
                                   VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"numObjectsInArea\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrNumObjectsInArea as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_INT,
                              numParams: 5 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_INT, VAL_INT,
                                   VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"numDroidsInArea\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrNumDroidsInArea as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_INT,
                              numParams: 5 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_INT, VAL_INT,
                                   VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"numStructsInArea\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrNumStructsInArea as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_INT,
                              numParams: 5 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_INT, VAL_INT,
                                   VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"numStructsButNotWallsInArea\x00" as
                                      *const u8 as *const libc::c_char as
                                      *mut STRING,
                              pFunc:
                                  Some(scrNumStructsButNotWallsInArea as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_INT,
                              numParams: 5 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_INT, VAL_INT,
                                   VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"numStructsByTypeInArea\x00" as *const u8
                                      as *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrNumStructsByTypeInArea as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_INT,
                              numParams: 6 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_INT, VAL_INT,
                                   VAL_INT, VAL_INT, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"droidHasSeen\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrDroidHasSeen as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_BOOL,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_BASEOBJECT as libc::c_int as
                                       INTERP_TYPE, VAL_INT, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"buildingDestroyed\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrBuildingDestroyed as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_BOOL,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_STRUCTUREID as libc::c_int as
                                       INTERP_TYPE, VAL_INT, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"structureIdle\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrStructureIdle as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_BOOL,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_STRUCTURE as libc::c_int as INTERP_TYPE,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"initEnumStruct\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrInitEnumStruct as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 4 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_BOOL,
                                   ST_STRUCTURESTAT as libc::c_int as
                                       INTERP_TYPE, VAL_INT, VAL_INT,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"enumStruct\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrEnumStruct as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0:
                                  ST_STRUCTURE as libc::c_int as INTERP_TYPE,
                              numParams: 0 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_VOID, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"structureBeingBuilt\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrStructureBeingBuilt as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_BOOL,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_STRUCTURESTAT as libc::c_int as
                                       INTERP_TYPE, VAL_INT, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"structureComplete\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrStructureComplete as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_BOOL,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_STRUCTURE as libc::c_int as INTERP_TYPE,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"structureBuilt\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrStructureBuilt as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_BOOL,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_STRUCTURESTAT as libc::c_int as
                                       INTERP_TYPE, VAL_INT, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"anyDroidsLeft\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrAnyDroidsLeft as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_BOOL,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"anyStructButWallsLeft\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrAnyStructButWallsLeft as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_BOOL,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"anyFactoriesLeft\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrAnyFactoriesLeft as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_BOOL,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"enableComponent\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrEnableComponent as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_COMPONENT as libc::c_int as INTERP_TYPE,
                                   VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"makeComponentAvailable\x00" as *const u8
                                      as *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrMakeComponentAvailable as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_COMPONENT as libc::c_int as INTERP_TYPE,
                                   VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"enableStructure\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrEnableStructure as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_STRUCTURESTAT as libc::c_int as
                                       INTERP_TYPE, VAL_INT, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"isStructureAvailable\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrIsStructureAvailable as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_BOOL,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_STRUCTURESTAT as libc::c_int as
                                       INTERP_TYPE, VAL_INT, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"addDroid\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrAddDroid as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: ST_DROID as libc::c_int as INTERP_TYPE,
                              numParams: 4 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_TEMPLATE as libc::c_int as INTERP_TYPE,
                                   VAL_INT, VAL_INT, VAL_INT, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"addDroidToMissionList\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrAddDroidToMissionList as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: ST_DROID as libc::c_int as INTERP_TYPE,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_TEMPLATE as libc::c_int as INTERP_TYPE,
                                   VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"buildDroid\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrBuildDroid as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 4 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_TEMPLATE as libc::c_int as INTERP_TYPE,
                                   ST_STRUCTURE as libc::c_int as INTERP_TYPE,
                                   VAL_INT, VAL_INT, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"addTemplate\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrAddTemplate as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_BOOL,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_TEMPLATE as libc::c_int as INTERP_TYPE,
                                   VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"addReticuleButton\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrAddReticuleButton as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"removeReticuleButton\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrRemoveReticuleButton as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"addMessage\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrAddMessage as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 4 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_INTMESSAGE as libc::c_int as
                                       INTERP_TYPE, VAL_INT, VAL_INT,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"removeMessage\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrRemoveMessage as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 3 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_INTMESSAGE as libc::c_int as
                                       INTERP_TYPE, VAL_INT, VAL_INT,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"selectDroidByID\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrSelectDroidByID as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_BOOL,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_DROIDID as libc::c_int as INTERP_TYPE,
                                   VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"setAssemblyPoint\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrSetAssemblyPoint as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 3 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_STRUCTURE as libc::c_int as INTERP_TYPE,
                                   VAL_INT, VAL_INT, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"attackLocation\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrAttackLocation as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 3 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_INT, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"initGetFeature\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrInitGetFeature as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 3 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_FEATURESTAT as libc::c_int as
                                       INTERP_TYPE, VAL_INT, VAL_INT,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"getFeature\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrGetFeature as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0:
                                  ST_FEATURE as libc::c_int as INTERP_TYPE,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"addFeature\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrAddFeature as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0:
                                  ST_FEATURE as libc::c_int as INTERP_TYPE,
                              numParams: 3 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_FEATURESTAT as libc::c_int as
                                       INTERP_TYPE, VAL_INT, VAL_INT,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"destroyFeature\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrDestroyFeature as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_FEATURE as libc::c_int as INTERP_TYPE,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"addStructure\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrAddStructure as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0:
                                  ST_STRUCTURE as libc::c_int as INTERP_TYPE,
                              numParams: 4 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_STRUCTURESTAT as libc::c_int as
                                       INTERP_TYPE, VAL_INT, VAL_INT, VAL_INT,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"destroyStructure\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrDestroyStructure as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_STRUCTURE as libc::c_int as INTERP_TYPE,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"centreView\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrCentreView as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_BASEOBJECT as libc::c_int as
                                       INTERP_TYPE, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"centreViewPos\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrCentreViewPos as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"getStructure\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrGetStructure as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0:
                                  ST_STRUCTURE as libc::c_int as INTERP_TYPE,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_STRUCTURESTAT as libc::c_int as
                                       INTERP_TYPE, VAL_INT, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"getTemplate\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrGetTemplate as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0:
                                  ST_TEMPLATE as libc::c_int as INTERP_TYPE,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_COMPONENT as libc::c_int as INTERP_TYPE,
                                   VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"getDroid\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrGetDroid as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: ST_DROID as libc::c_int as INTERP_TYPE,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_COMPONENT as libc::c_int as INTERP_TYPE,
                                   VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"setScrollParams\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrSetScrollParams as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 4 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_INT, VAL_INT,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"setScrollMinX\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrSetScrollMinX as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"setScrollMinY\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrSetScrollMinY as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"setScrollMaxX\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrSetScrollMaxX as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"setScrollMaxY\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrSetScrollMaxY as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"setDefaultSensor\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrSetDefaultSensor as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_SENSOR as libc::c_int as INTERP_TYPE,
                                   VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"setDefaultECM\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrSetDefaultECM as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_ECM as libc::c_int as INTERP_TYPE,
                                   VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"setDefaultRepair\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrSetDefaultRepair as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_REPAIR as libc::c_int as INTERP_TYPE,
                                   VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"setStructureLimits\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrSetStructureLimits as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 3 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_STRUCTURESTAT as libc::c_int as
                                       INTERP_TYPE, VAL_INT, VAL_INT,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"setAllStructureLimits\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrSetAllStructureLimits as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"applyLimitSet\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrApplyLimitSet as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 0 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_VOID, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"playSound\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrPlaySound as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_SOUND as libc::c_int as INTERP_TYPE,
                                   VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"playSoundPos\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrPlaySoundPos as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 5 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_SOUND as libc::c_int as INTERP_TYPE,
                                   VAL_INT, VAL_INT, VAL_INT, VAL_INT,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"addConsoleText\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrAddConsoleText as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_TEXTSTRING as libc::c_int as
                                       INTERP_TYPE, VAL_INT, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"showConsoleText\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrShowConsoleText as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_TEXTSTRING as libc::c_int as
                                       INTERP_TYPE, VAL_INT, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"tagConsoleText\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrTagConsoleText as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_TEXTSTRING as libc::c_int as
                                       INTERP_TYPE, VAL_INT, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"turnPowerOn\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrTurnPowerOn as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 0 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_VOID, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"turnPowerOff\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrTurnPowerOff as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 0 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_VOID, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"tutorialEnd\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrTutorialEnd as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 0 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_VOID, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"clearConsole\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrClearConsole as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 0 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_VOID, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"playVideo\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrPlayVideo as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_TEXTSTRING as libc::c_int as
                                       INTERP_TYPE,
                                   ST_TEXTSTRING as libc::c_int as
                                       INTERP_TYPE, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"gameOverMessage\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrGameOverMessage as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 4 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_INTMESSAGE as libc::c_int as
                                       INTERP_TYPE, VAL_INT, VAL_INT,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"gameOver\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrGameOver as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"playBackgroundAudio\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrPlayBackgroundAudio as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_TEXTSTRING as libc::c_int as
                                       INTERP_TYPE, VAL_INT, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"playCDAudio\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrPlayCDAudio as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"stopCDAudio\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrStopCDAudio as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 0 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_VOID, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"pauseCDAudio\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrPauseCDAudio as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 0 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_VOID, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"resumeCDAudio\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrResumeCDAudio as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 0 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_VOID, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"setRetreatPoint\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrSetRetreatPoint as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 3 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_INT, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"setRetreatForce\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrSetRetreatForce as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"setRetreatLeadership\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrSetRetreatLeadership as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"setGroupRetreatForce\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrSetGroupRetreatForce as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_GROUP as libc::c_int as INTERP_TYPE,
                                   VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"setGroupRetreatLeadership\x00" as
                                      *const u8 as *const libc::c_char as
                                      *mut STRING,
                              pFunc:
                                  Some(scrSetGroupRetreatLeadership as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_GROUP as libc::c_int as INTERP_TYPE,
                                   VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"setGroupRetreatPoint\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrSetGroupRetreatPoint as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 3 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_GROUP as libc::c_int as INTERP_TYPE,
                                   VAL_INT, VAL_INT, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"setRetreatHealth\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrSetRetreatHealth as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"setGroupRetreatHealth\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrSetGroupRetreatHealth as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_GROUP as libc::c_int as INTERP_TYPE,
                                   VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"startMission\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrStartMission as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT,
                                   ST_LEVEL as libc::c_int as INTERP_TYPE,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"setSnow\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrSetSnow as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"setRain\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrSetRain as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"setBackgroundFog\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrSetBackgroundFog as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"setDepthFog\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrSetDepthFog as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"setFogColour\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrSetFogColour as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 3 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_INT, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"setTransporterExit\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrSetTransporterExit as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 3 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_INT, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"flyTransporterIn\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrFlyTransporterIn as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 4 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_INT, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"addDroidToTransporter\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrAddDroidToTransporter as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_DROID as libc::c_int as INTERP_TYPE,
                                   ST_DROID as libc::c_int as INTERP_TYPE,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"structureBuiltInRange\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrStructureBuiltInRange as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0:
                                  ST_STRUCTURE as libc::c_int as INTERP_TYPE,
                              numParams: 5 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_STRUCTURESTAT as libc::c_int as
                                       INTERP_TYPE, VAL_INT, VAL_INT, VAL_INT,
                                   VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"random\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrRandom as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_INT,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"randomiseSeed\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrRandomiseSeed as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 0 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_VOID, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"enableResearch\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrEnableResearch as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_RESEARCH as libc::c_int as INTERP_TYPE,
                                   VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"completeResearch\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrCompleteResearch as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_RESEARCH as libc::c_int as INTERP_TYPE,
                                   VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"flashOn\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrFlashOn as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"flashOff\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrFlashOff as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"setPowerLevel\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrSetPowerLevel as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"addPower\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrAddPower as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"setLandingZone\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrSetLandingZone as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 4 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_INT, VAL_INT,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"setLimboLanding\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrSetLimboLanding as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 4 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_INT, VAL_INT,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"setNoGoArea\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrSetNoGoArea as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 5 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_INT, VAL_INT,
                                   VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"initAllNoGoAreas\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrInitAllNoGoAreas as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 0 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_VOID, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"setRadarZoom\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrSetRadarZoom as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"setReinforcementTime\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrSetReinforcementTime as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"setMissionTime\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrSetMissionTime as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"missionTimeRemaining\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrMissionTimeRemaining as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_INT,
                              numParams: 0 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_VOID, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"flushConsoleMessages\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrFlushConsoleMessages as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 0 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_VOID, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"pickStructLocation\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrPickStructLocation as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_BOOL,
                              numParams: 4 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_STRUCTURESTAT as libc::c_int as
                                       INTERP_TYPE,
                                   (0x100000 as libc::c_int |
                                        VAL_INT as libc::c_int) as
                                       INTERP_TYPE,
                                   (0x100000 as libc::c_int |
                                        VAL_INT as libc::c_int) as
                                       INTERP_TYPE, VAL_INT, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"groupAddDroid\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrGroupAddDroid as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_GROUP as libc::c_int as INTERP_TYPE,
                                   ST_DROID as libc::c_int as INTERP_TYPE,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"groupAddArea\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrGroupAddArea as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 6 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_GROUP as libc::c_int as INTERP_TYPE,
                                   VAL_INT, VAL_INT, VAL_INT, VAL_INT,
                                   VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"groupAddAreaNoGroup\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrGroupAddArea as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 6 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_GROUP as libc::c_int as INTERP_TYPE,
                                   VAL_INT, VAL_INT, VAL_INT, VAL_INT,
                                   VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"groupAddGroup\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrGroupAddGroup as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_GROUP as libc::c_int as INTERP_TYPE,
                                   ST_GROUP as libc::c_int as INTERP_TYPE,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"groupMember\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrGroupMember as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_BOOL,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_GROUP as libc::c_int as INTERP_TYPE,
                                   ST_DROID as libc::c_int as INTERP_TYPE,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"idleGroup\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrIdleGroup as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_INT,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_GROUP as libc::c_int as INTERP_TYPE,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"initIterateGroup\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrInitIterateGroup as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_GROUP as libc::c_int as INTERP_TYPE,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"iterateGroup\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrIterateGroup as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: ST_DROID as libc::c_int as INTERP_TYPE,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_GROUP as libc::c_int as INTERP_TYPE,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"droidLeaveGroup\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrDroidLeaveGroup as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_DROID as libc::c_int as INTERP_TYPE,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"orderDroid\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrOrderDroid as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_DROID as libc::c_int as INTERP_TYPE,
                                   VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"orderDroidLoc\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrOrderDroidLoc as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 4 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_DROID as libc::c_int as INTERP_TYPE,
                                   VAL_INT, VAL_INT, VAL_INT, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"orderDroidObj\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrOrderDroidObj as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 3 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_DROID as libc::c_int as INTERP_TYPE,
                                   VAL_INT,
                                   ST_BASEOBJECT as libc::c_int as
                                       INTERP_TYPE, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"orderDroidStatsLoc\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrOrderDroidStatsLoc as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 5 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_DROID as libc::c_int as INTERP_TYPE,
                                   VAL_INT,
                                   ST_BASESTATS as libc::c_int as INTERP_TYPE,
                                   VAL_INT, VAL_INT, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"orderGroup\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrOrderGroup as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_GROUP as libc::c_int as INTERP_TYPE,
                                   VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"orderGroupLoc\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrOrderGroupLoc as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 4 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_GROUP as libc::c_int as INTERP_TYPE,
                                   VAL_INT, VAL_INT, VAL_INT, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"orderGroupObj\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrOrderGroupObj as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 3 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_GROUP as libc::c_int as INTERP_TYPE,
                                   VAL_INT,
                                   ST_BASEOBJECT as libc::c_int as
                                       INTERP_TYPE, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"setDroidSecondary\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrSetDroidSecondary as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 3 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_DROID as libc::c_int as INTERP_TYPE,
                                   VAL_INT, VAL_INT, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"setGroupSecondary\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrSetGroupSecondary as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 3 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_GROUP as libc::c_int as INTERP_TYPE,
                                   VAL_INT, VAL_INT, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"initIterateCluster\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrInitIterateCluster as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"iterateCluster\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrIterateCluster as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0:
                                  ST_BASEOBJECT as libc::c_int as INTERP_TYPE,
                              numParams: 0 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_VOID, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"cmdDroidAddDroid\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrCmdDroidAddDroid as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_DROID as libc::c_int as INTERP_TYPE,
                                   ST_DROID as libc::c_int as INTERP_TYPE,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"numMessageBox\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrNumMB as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"debugBox\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrNumMB as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"approxRoot\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrApproxRoot as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_INT,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"refTest\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrRefTest as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"distBetweenTwoPoints\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrDistanceTwoPts as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_INT,
                              numParams: 4 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_INT, VAL_INT,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"losTwoObjects\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrLOSTwoBaseObjects as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_BOOL,
                              numParams: 3 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_BASEOBJECT as libc::c_int as
                                       INTERP_TYPE,
                                   ST_BASEOBJECT as libc::c_int as
                                       INTERP_TYPE, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"killStructsInArea\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrDestroyStructuresInArea as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 8 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_INT, VAL_INT,
                                   VAL_INT, VAL_INT, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"getThreatInArea\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrThreatInArea as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_INT,
                              numParams: 10 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_INT, VAL_INT,
                                   VAL_INT, VAL_INT, VAL_INT, VAL_INT,
                                   VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"getNearestGateway\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrGetNearestGateway as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_BOOL,
                              numParams: 4 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT,
                                   (0x100000 as libc::c_int |
                                        VAL_INT as libc::c_int) as
                                       INTERP_TYPE,
                                   (0x100000 as libc::c_int |
                                        VAL_INT as libc::c_int) as
                                       INTERP_TYPE, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"setWaterTile\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrSetWaterTile as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"setRubbleTile\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrSetRubbleTile as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"setCampaignNumber\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrSetCampaignNumber as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"testStructureModule\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrTestStructureModule as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_BOOL,
                              numParams: 3 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT,
                                   ST_STRUCTURE as libc::c_int as INTERP_TYPE,
                                   VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"killDroidsInArea\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrDestroyUnitsInArea as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_INT,
                              numParams: 5 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_INT, VAL_INT,
                                   VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"vanishUnit\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrRemoveDroid as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_DROID as libc::c_int as INTERP_TYPE,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"forceDamageObject\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrForceDamage as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_BASEOBJECT as libc::c_int as
                                       INTERP_TYPE, VAL_INT, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"isHumanPlayer\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrIsHumanPlayer as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_BOOL,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"offerAlliance\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrOfferAlliance as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"createAlliance\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrCreateAlliance as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"breakAlliance\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrBreakAlliance as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"allianceExists\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrAllianceExists as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_BOOL,
                              numParams: 0 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_VOID, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"allianceExistsBetween\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrAllianceExistsBetween as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_BOOL,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"playerInAlliance\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrPlayerInAlliance as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_BOOL,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"dominatingAlliance\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrDominatingAlliance as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_BOOL,
                              numParams: 0 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_VOID, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"myResponsibility\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrMyResponsibility as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_BOOL,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"objToDroid\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrObjToDroid as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: ST_DROID as libc::c_int as INTERP_TYPE,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_BASEOBJECT as libc::c_int as
                                       INTERP_TYPE, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"objToStructure\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrObjToStructure as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0:
                                  ST_STRUCTURE as libc::c_int as INTERP_TYPE,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_BASEOBJECT as libc::c_int as
                                       INTERP_TYPE, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"objToFeature\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrObjToFeature as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0:
                                  ST_FEATURE as libc::c_int as INTERP_TYPE,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_BASEOBJECT as libc::c_int as
                                       INTERP_TYPE, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"getGameStatus\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrGetGameStatus as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_BOOL,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"getPlayerColour\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrGetPlayerColour as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_INT,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"setPlayerColour\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrSetPlayerColour as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"takeOverDroidsInArea\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrTakeOverDroidsInArea as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_INT,
                              numParams: 6 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_INT, VAL_INT,
                                   VAL_INT, VAL_INT, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"takeOverDroidsInAreaExp\x00" as *const u8
                                      as *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrTakeOverDroidsInAreaExp as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_INT,
                              numParams: 8 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_INT, VAL_INT,
                                   VAL_INT, VAL_INT, VAL_INT, VAL_INT,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"takeOverStructsInArea\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrTakeOverStructsInArea as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_INT,
                              numParams: 6 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_INT, VAL_INT,
                                   VAL_INT, VAL_INT, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"takeOverSingleDroid\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrTakeOverSingleDroid as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: ST_DROID as libc::c_int as INTERP_TYPE,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_DROID as libc::c_int as INTERP_TYPE,
                                   VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"takeOverSingleStructure\x00" as *const u8
                                      as *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrTakeOverSingleStructure as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0:
                                  ST_STRUCTURE as libc::c_int as INTERP_TYPE,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_STRUCTURE as libc::c_int as INTERP_TYPE,
                                   VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"resetStructTargets\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrResetStructTargets as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 0 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_VOID, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"resetDroidTargets\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrResetDroidTargets as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 0 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_VOID, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"setStructTarPref\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrSetStructTarPref as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"setStructTarIgnore\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrSetStructTarIgnore as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"setDroidTarPref\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrSetDroidTarPref as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"setDroidTarIgnore\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrSetDroidTarIgnore as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"structTargetInArea\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrStructTargetInArea as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0:
                                  ST_STRUCTURE as libc::c_int as INTERP_TYPE,
                              numParams: 6 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_INT, VAL_INT,
                                   VAL_INT, VAL_INT, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"structTargetOnMap\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrStructTargetOnMap as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0:
                                  ST_STRUCTURE as libc::c_int as INTERP_TYPE,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"droidTargetInArea\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrDroidTargetInArea as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: ST_DROID as libc::c_int as INTERP_TYPE,
                              numParams: 6 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_INT, VAL_INT,
                                   VAL_INT, VAL_INT, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"droidTargetOnMap\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrDroidTargetOnMap as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: ST_DROID as libc::c_int as INTERP_TYPE,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"targetInCluster\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrTargetInCluster as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: ST_DROID as libc::c_int as INTERP_TYPE,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"setDroidsToSafetyFlag\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrSetDroidsToSafetyFlag as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"setPlayCountDown\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrSetPlayCountDown as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"getDroidCount\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrGetDroidCount as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_INT,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"fireWeaponAtObj\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrFireWeaponAtObj as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_WEAPON as libc::c_int as INTERP_TYPE,
                                   ST_BASEOBJECT as libc::c_int as
                                       INTERP_TYPE, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"fireWeaponAtLoc\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrFireWeaponAtLoc as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 3 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_WEAPON as libc::c_int as INTERP_TYPE,
                                   VAL_INT, VAL_INT, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"setDroidKills\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrSetDroidKills as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_DROID as libc::c_int as INTERP_TYPE,
                                   VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"resetPlayerVisibility\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrResetPlayerVisibility as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"setVTOLReturnPos\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrSetVTOLReturnPos as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 3 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_INT, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"isVtol\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrIsVtol as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_BOOL,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_DROID as libc::c_int as INTERP_TYPE,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"tutorialTemplates\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrTutorialTemplates as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 0 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_VOID, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"resetLimboMission\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrResetLimboMission as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 0 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_VOID, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"skDoResearch\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrSkDoResearch as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 3 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_STRUCTURE as libc::c_int as INTERP_TYPE,
                                   VAL_INT, VAL_INT, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"skLocateEnemy\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrSkLocateEnemy as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0:
                                  ST_BASEOBJECT as libc::c_int as INTERP_TYPE,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"skCanBuildTemplate\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrSkCanBuildTemplate as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_BOOL,
                              numParams: 3 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT,
                                   ST_STRUCTURE as libc::c_int as INTERP_TYPE,
                                   ST_TEMPLATE as libc::c_int as INTERP_TYPE,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"skVtolEnableCheck\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrSkVtolEnableCheck as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_BOOL,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"skGetFactoryCapacity\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrSkGetFactoryCapacity as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_INT,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_STRUCTURE as libc::c_int as INTERP_TYPE,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"skDifficultyModifier\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrSkDifficultyModifier as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"skDefenseLocation\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrSkDefenseLocation as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_BOOL,
                              numParams: 6 as libc::c_int as UDWORD,
                              aParams:
                                  [(0x100000 as libc::c_int |
                                        VAL_INT as libc::c_int) as
                                       INTERP_TYPE,
                                   (0x100000 as libc::c_int |
                                        VAL_INT as libc::c_int) as
                                       INTERP_TYPE,
                                   ST_STRUCTURESTAT as libc::c_int as
                                       INTERP_TYPE,
                                   ST_STRUCTURESTAT as libc::c_int as
                                       INTERP_TYPE,
                                   ST_DROID as libc::c_int as INTERP_TYPE,
                                   VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"skFireLassat\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrSkFireLassat as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT,
                                   ST_BASEOBJECT as libc::c_int as
                                       INTERP_TYPE, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"strcmp\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrStrcmp as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_BOOL,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_STRING, VAL_STRING, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"console\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrConsole as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_STRING, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"dbgMsgOn\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrDbgMsgOn as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"dbg\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrDbg as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_STRING, VAL_INT, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"msg\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrMsg as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 3 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_STRING, VAL_INT, VAL_INT, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"debug\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrDebugFile as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_STRING, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"actionDroidObj\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrActionDroidObj as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 3 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_DROID as libc::c_int as INTERP_TYPE,
                                   VAL_INT,
                                   ST_BASEOBJECT as libc::c_int as
                                       INTERP_TYPE, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"InitEnumDroids\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrInitEnumDroids as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"EnumDroid\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrEnumDroid as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: ST_DROID as libc::c_int as INTERP_TYPE,
                              numParams: 0 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_VOID, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"initIterateGroupB\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrInitIterateGroupB as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_GROUP as libc::c_int as INTERP_TYPE,
                                   VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"iterateGroupB\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrIterateGroupB as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: ST_DROID as libc::c_int as INTERP_TYPE,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_GROUP as libc::c_int as INTERP_TYPE,
                                   VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"factoryGetTemplate\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrFactoryGetTemplate as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0:
                                  ST_TEMPLATE as libc::c_int as INTERP_TYPE,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_STRUCTURE as libc::c_int as INTERP_TYPE,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"numTemplatesInProduction\x00" as *const u8
                                      as *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrNumTemplatesInProduction as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_INT,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_TEMPLATE as libc::c_int as INTERP_TYPE,
                                   VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"numDroidsByComponent\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrNumDroidsByComponent as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_INT,
                              numParams: 3 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_COMPONENT as libc::c_int as INTERP_TYPE,
                                   VAL_INT, VAL_INT, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"getStructureLimit\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrGetStructureLimit as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_INT,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_STRUCTURESTAT as libc::c_int as
                                       INTERP_TYPE, VAL_INT, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"structureLimitReached\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrStructureLimitReached as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_BOOL,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_STRUCTURESTAT as libc::c_int as
                                       INTERP_TYPE, VAL_INT, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"getNumStructures\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrGetNumStructures as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_INT,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_STRUCTURESTAT as libc::c_int as
                                       INTERP_TYPE, VAL_INT, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"getUnitLimit\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrGetUnitLimit as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_INT,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"min\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrMin as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_INT,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"max\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrMax as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_INT,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"fogTileInRange\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrFogTileInRange as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_BOOL,
                              numParams: 9 as libc::c_int as UDWORD,
                              aParams:
                                  [(0x100000 as libc::c_int |
                                        VAL_INT as libc::c_int) as
                                       INTERP_TYPE,
                                   (0x100000 as libc::c_int |
                                        VAL_INT as libc::c_int) as
                                       INTERP_TYPE, VAL_INT, VAL_INT, VAL_INT,
                                   VAL_INT, VAL_INT, VAL_INT, VAL_INT,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"mapRevealedInRange\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrMapRevealedInRange as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_BOOL,
                              numParams: 4 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_INT, VAL_INT,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"numResearchLeft\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrNumResearchLeft as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_INT,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT,
                                   ST_RESEARCH as libc::c_int as INTERP_TYPE,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"researchCompleted\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrResearchCompleted as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_BOOL,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_RESEARCH as libc::c_int as INTERP_TYPE,
                                   VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"researchStarted\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrResearchStarted as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_BOOL,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_RESEARCH as libc::c_int as INTERP_TYPE,
                                   VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"threatInRange\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrThreatInRange as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_BOOL,
                              numParams: 5 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_INT, VAL_INT,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"numEnemyWeapObjInRange\x00" as *const u8
                                      as *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrNumEnemyWeapObjInRange as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_INT,
                              numParams: 5 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_INT, VAL_INT,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"numEnemyWeapDroidsInRange\x00" as
                                      *const u8 as *const libc::c_char as
                                      *mut STRING,
                              pFunc:
                                  Some(scrNumEnemyWeapDroidsInRange as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_INT,
                              numParams: 5 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_INT, VAL_INT,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"numEnemyWeapStructsInRange\x00" as
                                      *const u8 as *const libc::c_char as
                                      *mut STRING,
                              pFunc:
                                  Some(scrNumEnemyWeapStructsInRange as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_INT,
                              numParams: 4 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_INT, VAL_INT,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"numFriendlyWeapObjInRange\x00" as
                                      *const u8 as *const libc::c_char as
                                      *mut STRING,
                              pFunc:
                                  Some(scrNumFriendlyWeapObjInRange as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_INT,
                              numParams: 5 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_INT, VAL_INT,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"numFriendlyWeapDroidsInRange\x00" as
                                      *const u8 as *const libc::c_char as
                                      *mut STRING,
                              pFunc:
                                  Some(scrNumFriendlyWeapDroidsInRange as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_INT,
                              numParams: 5 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_INT, VAL_INT,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"numFriendlyWeapStructsInRange\x00" as
                                      *const u8 as *const libc::c_char as
                                      *mut STRING,
                              pFunc:
                                  Some(scrNumFriendlyWeapStructsInRange as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_INT,
                              numParams: 4 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_INT, VAL_INT,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"numPlayerWeapObjInRange\x00" as *const u8
                                      as *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrNumPlayerWeapObjInRange as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_INT,
                              numParams: 6 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_INT, VAL_INT,
                                   VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"numEnemyObjInRange\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrNumEnemyObjInRange as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_INT,
                              numParams: 5 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_INT, VAL_INT,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"numStructsByStatInRange\x00" as *const u8
                                      as *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrNumStructsByStatInRange as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_INT,
                              numParams: 6 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_STRUCTURESTAT as libc::c_int as
                                       INTERP_TYPE, VAL_INT, VAL_INT, VAL_INT,
                                   VAL_INT, VAL_INT, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"numStructsByStatInArea\x00" as *const u8
                                      as *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrNumStructsByStatInArea as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_INT,
                              numParams: 7 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_STRUCTURESTAT as libc::c_int as
                                       INTERP_TYPE, VAL_INT, VAL_INT, VAL_INT,
                                   VAL_INT, VAL_INT, VAL_INT, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"numStructsByTypeInRange\x00" as *const u8
                                      as *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrNumStructsByTypeInRange as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_INT,
                              numParams: 6 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_INT, VAL_INT,
                                   VAL_INT, VAL_INT, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"numFeatByTypeInRange\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrNumFeatByTypeInRange as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_INT,
                              numParams: 5 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_INT, VAL_INT,
                                   VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"numStructsButNotWallsInRangeVis\x00" as
                                      *const u8 as *const libc::c_char as
                                      *mut STRING,
                              pFunc:
                                  Some(scrNumStructsButNotWallsInRangeVis as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_INT,
                              numParams: 5 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_INT, VAL_INT,
                                   VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"getStructureVis\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrGetStructureVis as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0:
                                  ST_STRUCTURE as libc::c_int as INTERP_TYPE,
                              numParams: 3 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_STRUCTURESTAT as libc::c_int as
                                       INTERP_TYPE, VAL_INT, VAL_INT,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"chooseValidLoc\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrChooseValidLoc as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_BOOL,
                              numParams: 6 as libc::c_int as UDWORD,
                              aParams:
                                  [(0x100000 as libc::c_int |
                                        VAL_INT as libc::c_int) as
                                       INTERP_TYPE,
                                   (0x100000 as libc::c_int |
                                        VAL_INT as libc::c_int) as
                                       INTERP_TYPE, VAL_INT, VAL_INT, VAL_INT,
                                   VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"getClosestEnemy\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrGetClosestEnemy as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0:
                                  ST_BASEOBJECT as libc::c_int as INTERP_TYPE,
                              numParams: 6 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_INT, VAL_BOOL,
                                   VAL_BOOL, VAL_INT, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"transporterCapacity\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrTransporterCapacity as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_INT,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_DROID as libc::c_int as INTERP_TYPE,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"transporterFlying\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrTransporterFlying as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_BOOL,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_DROID as libc::c_int as INTERP_TYPE,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"unloadTransporter\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrUnloadTransporter as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 3 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_DROID as libc::c_int as INTERP_TYPE,
                                   VAL_INT, VAL_INT, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"hasGroup\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrHasGroup as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_BOOL,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_DROID as libc::c_int as INTERP_TYPE,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"objWeaponMaxRange\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrObjWeaponMaxRange as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_INT,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_BASEOBJECT as libc::c_int as
                                       INTERP_TYPE, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"objHasWeapon\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrObjHasWeapon as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_BOOL,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_BASEOBJECT as libc::c_int as
                                       INTERP_TYPE, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"objectHasIndirectWeapon\x00" as *const u8
                                      as *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrObjectHasIndirectWeapon as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_BOOL,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_BASEOBJECT as libc::c_int as
                                       INTERP_TYPE, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"getClosestEnemyDroidByType\x00" as
                                      *const u8 as *const libc::c_char as
                                      *mut STRING,
                              pFunc:
                                  Some(scrGetClosestEnemyDroidByType as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: ST_DROID as libc::c_int as INTERP_TYPE,
                              numParams: 6 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_INT, VAL_INT,
                                   VAL_BOOL, VAL_INT, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"getClosestEnemyStructByType\x00" as
                                      *const u8 as *const libc::c_char as
                                      *mut STRING,
                              pFunc:
                                  Some(scrGetClosestEnemyStructByType as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0:
                                  ST_STRUCTURE as libc::c_int as INTERP_TYPE,
                              numParams: 5 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_INT, VAL_INT,
                                   VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"skDefenseLocationB\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrSkDefenseLocationB as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_BOOL,
                              numParams: 6 as libc::c_int as UDWORD,
                              aParams:
                                  [(0x100000 as libc::c_int |
                                        VAL_INT as libc::c_int) as
                                       INTERP_TYPE,
                                   (0x100000 as libc::c_int |
                                        VAL_INT as libc::c_int) as
                                       INTERP_TYPE,
                                   ST_STRUCTURESTAT as libc::c_int as
                                       INTERP_TYPE,
                                   ST_STRUCTURESTAT as libc::c_int as
                                       INTERP_TYPE,
                                   ST_DROID as libc::c_int as INTERP_TYPE,
                                   VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"circlePerimPoint\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrCirclePerimPoint as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 5 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT,
                                   (0x100000 as libc::c_int |
                                        VAL_INT as libc::c_int) as
                                       INTERP_TYPE,
                                   (0x100000 as libc::c_int |
                                        VAL_INT as libc::c_int) as
                                       INTERP_TYPE, VAL_INT, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"giftRadar\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrGiftRadar as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 3 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"numAllies\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrNumAllies as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_INT,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"numAAinRange\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrNumAAinRange as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_INT,
                              numParams: 5 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_INT, VAL_INT,
                                   VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"selectDroid\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrSelectDroid as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_DROID as libc::c_int as INTERP_TYPE,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"selectGroup\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrSelectGroup as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_VOID,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [ST_GROUP as libc::c_int as INTERP_TYPE,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"modulo\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrModulo as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_INT,
                              numParams: 2 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_INT, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"playerLoaded\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc:
                                  Some(scrPlayerLoaded as
                                           unsafe extern "C" fn() -> BOOL),
                              type_0: VAL_BOOL,
                              numParams: 1 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         },
         {
             let mut init =
                 _func_symbol{pIdent:
                                  b"FUNCTION LIST END\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              pFunc: None,
                              type_0: VAL_VOID,
                              numParams: 0 as libc::c_int as UDWORD,
                              aParams:
                                  [VAL_VOID, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                   VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL],
                              script: 0,
                              size: 0,
                              pCode: 0 as *const UDWORD as *mut UDWORD,
                              location: 0,
                              debugEntries: 0,
                              psDebug:
                                  0 as *const SCRIPT_DEBUG as
                                      *mut SCRIPT_DEBUG,
                              psNext:
                                  0 as *const _func_symbol as
                                      *mut _func_symbol,};
             init
         }]
    };
/*
 * The table of external variables and their access functions.
 * The table is laid out as follows :
 *
 *   "variable_identifier", <variable type>, (INTERP_TYPE)ST_EXTERN, 0, <index>,
 *		<get_function>, <set_function>
 *
 * The Storage type for an external variable is always (INTERP_TYPE)ST_EXTERN.
 * The index is not used by the compiler but is passed to the access function
 * to allow one function to deal with a number of variables
 */
#[no_mangle]
pub static mut asExternTable: [VAR_SYMBOL; 18] =
    unsafe {
        [{
             let mut init =
                 _var_symbol{pIdent:
                                 b"isPSX\x00" as *const u8 as
                                     *const libc::c_char as *mut STRING,
                             type_0: VAL_BOOL,
                             storage:
                                 ST_EXTERN as libc::c_int as INTERP_TYPE as
                                     STORAGE_TYPE,
                             objType: VAL_BOOL,
                             index: EXTID_ISPSX as libc::c_int as UDWORD,
                             get:
                                 Some(scrGenExternGet as
                                          unsafe extern "C" fn(_: UDWORD)
                                              -> BOOL),
                             set: None,
                             dimensions: 0,
                             elements: [0; 4],
                             psNext:
                                 0 as *const _var_symbol as
                                     *mut _var_symbol,};
             init
         },
         {
             let mut init =
                 _var_symbol{pIdent:
                                 b"trackTransporter\x00" as *const u8 as
                                     *const libc::c_char as *mut STRING,
                             type_0: VAL_BOOL,
                             storage:
                                 ST_EXTERN as libc::c_int as INTERP_TYPE as
                                     STORAGE_TYPE,
                             objType: VAL_BOOL,
                             index:
                                 EXTID_TRACKTRANSPORTER as libc::c_int as
                                     UDWORD,
                             get:
                                 Some(scrGenExternGet as
                                          unsafe extern "C" fn(_: UDWORD)
                                              -> BOOL),
                             set: None,
                             dimensions: 0,
                             elements: [0; 4],
                             psNext:
                                 0 as *const _var_symbol as
                                     *mut _var_symbol,};
             init
         },
         {
             let mut init =
                 _var_symbol{pIdent:
                                 b"mapWidth\x00" as *const u8 as
                                     *const libc::c_char as *mut STRING,
                             type_0: VAL_INT,
                             storage:
                                 ST_EXTERN as libc::c_int as INTERP_TYPE as
                                     STORAGE_TYPE,
                             objType: VAL_BOOL,
                             index: EXTID_MAPWIDTH as libc::c_int as UDWORD,
                             get:
                                 Some(scrGenExternGet as
                                          unsafe extern "C" fn(_: UDWORD)
                                              -> BOOL),
                             set: None,
                             dimensions: 0,
                             elements: [0; 4],
                             psNext:
                                 0 as *const _var_symbol as
                                     *mut _var_symbol,};
             init
         },
         {
             let mut init =
                 _var_symbol{pIdent:
                                 b"mapHeight\x00" as *const u8 as
                                     *const libc::c_char as *mut STRING,
                             type_0: VAL_INT,
                             storage:
                                 ST_EXTERN as libc::c_int as INTERP_TYPE as
                                     STORAGE_TYPE,
                             objType: VAL_BOOL,
                             index: EXTID_MAPHEIGHT as libc::c_int as UDWORD,
                             get:
                                 Some(scrGenExternGet as
                                          unsafe extern "C" fn(_: UDWORD)
                                              -> BOOL),
                             set: None,
                             dimensions: 0,
                             elements: [0; 4],
                             psNext:
                                 0 as *const _var_symbol as
                                     *mut _var_symbol,};
             init
         },
         {
             let mut init =
                 _var_symbol{pIdent:
                                 b"gameInitialised\x00" as *const u8 as
                                     *const libc::c_char as *mut STRING,
                             type_0: VAL_BOOL,
                             storage:
                                 ST_EXTERN as libc::c_int as INTERP_TYPE as
                                     STORAGE_TYPE,
                             objType: VAL_BOOL,
                             index: EXTID_GAMEINIT as libc::c_int as UDWORD,
                             get:
                                 Some(scrGenExternGet as
                                          unsafe extern "C" fn(_: UDWORD)
                                              -> BOOL),
                             set: None,
                             dimensions: 0,
                             elements: [0; 4],
                             psNext:
                                 0 as *const _var_symbol as
                                     *mut _var_symbol,};
             init
         },
         {
             let mut init =
                 _var_symbol{pIdent:
                                 b"selectedPlayer\x00" as *const u8 as
                                     *const libc::c_char as *mut STRING,
                             type_0: VAL_INT,
                             storage:
                                 ST_EXTERN as libc::c_int as INTERP_TYPE as
                                     STORAGE_TYPE,
                             objType: VAL_BOOL,
                             index:
                                 EXTID_SELECTEDPLAYER as libc::c_int as
                                     UDWORD,
                             get:
                                 Some(scrGenExternGet as
                                          unsafe extern "C" fn(_: UDWORD)
                                              -> BOOL),
                             set: None,
                             dimensions: 0,
                             elements: [0; 4],
                             psNext:
                                 0 as *const _var_symbol as
                                     *mut _var_symbol,};
             init
         },
         {
             let mut init =
                 _var_symbol{pIdent:
                                 b"gameTime\x00" as *const u8 as
                                     *const libc::c_char as *mut STRING,
                             type_0: VAL_INT,
                             storage:
                                 ST_EXTERN as libc::c_int as INTERP_TYPE as
                                     STORAGE_TYPE,
                             objType: VAL_BOOL,
                             index: EXTID_GAMETIME as libc::c_int as UDWORD,
                             get:
                                 Some(scrGenExternGet as
                                          unsafe extern "C" fn(_: UDWORD)
                                              -> BOOL),
                             set: None,
                             dimensions: 0,
                             elements: [0; 4],
                             psNext:
                                 0 as *const _var_symbol as
                                     *mut _var_symbol,};
             init
         },
         {
             let mut init =
                 _var_symbol{pIdent:
                                 b"gameLevel\x00" as *const u8 as
                                     *const libc::c_char as *mut STRING,
                             type_0: VAL_INT,
                             storage:
                                 ST_EXTERN as libc::c_int as INTERP_TYPE as
                                     STORAGE_TYPE,
                             objType: VAL_BOOL,
                             index: EXTID_GAMELEVEL as libc::c_int as UDWORD,
                             get:
                                 Some(scrGenExternGet as
                                          unsafe extern "C" fn(_: UDWORD)
                                              -> BOOL),
                             set:
                                 Some(scrGenExternSet as
                                          unsafe extern "C" fn(_: UDWORD)
                                              -> BOOL),
                             dimensions: 0,
                             elements: [0; 4],
                             psNext:
                                 0 as *const _var_symbol as
                                     *mut _var_symbol,};
             init
         },
         {
             let mut init =
                 _var_symbol{pIdent:
                                 b"inTutorial\x00" as *const u8 as
                                     *const libc::c_char as *mut STRING,
                             type_0: VAL_BOOL,
                             storage:
                                 ST_EXTERN as libc::c_int as INTERP_TYPE as
                                     STORAGE_TYPE,
                             objType: VAL_BOOL,
                             index: EXTID_TUTORIAL as libc::c_int as UDWORD,
                             get:
                                 Some(scrGenExternGet as
                                          unsafe extern "C" fn(_: UDWORD)
                                              -> BOOL),
                             set:
                                 Some(scrGenExternSet as
                                          unsafe extern "C" fn(_: UDWORD)
                                              -> BOOL),
                             dimensions: 0,
                             elements: [0; 4],
                             psNext:
                                 0 as *const _var_symbol as
                                     *mut _var_symbol,};
             init
         },
         {
             let mut init =
                 _var_symbol{pIdent:
                                 b"cursorType\x00" as *const u8 as
                                     *const libc::c_char as *mut STRING,
                             type_0: VAL_INT,
                             storage:
                                 ST_EXTERN as libc::c_int as INTERP_TYPE as
                                     STORAGE_TYPE,
                             objType: VAL_BOOL,
                             index: EXTID_CURSOR as libc::c_int as UDWORD,
                             get:
                                 Some(scrGenExternGet as
                                          unsafe extern "C" fn(_: UDWORD)
                                              -> BOOL),
                             set: None,
                             dimensions: 0,
                             elements: [0; 4],
                             psNext:
                                 0 as *const _var_symbol as
                                     *mut _var_symbol,};
             init
         },
         {
             let mut init =
                 _var_symbol{pIdent:
                                 b"intMode\x00" as *const u8 as
                                     *const libc::c_char as *mut STRING,
                             type_0: VAL_INT,
                             storage:
                                 ST_EXTERN as libc::c_int as INTERP_TYPE as
                                     STORAGE_TYPE,
                             objType: VAL_BOOL,
                             index: EXTID_INTMODE as libc::c_int as UDWORD,
                             get:
                                 Some(scrGenExternGet as
                                          unsafe extern "C" fn(_: UDWORD)
                                              -> BOOL),
                             set: None,
                             dimensions: 0,
                             elements: [0; 4],
                             psNext:
                                 0 as *const _var_symbol as
                                     *mut _var_symbol,};
             init
         },
         {
             let mut init =
                 _var_symbol{pIdent:
                                 b"targetedObjectType\x00" as *const u8 as
                                     *const libc::c_char as *mut STRING,
                             type_0: VAL_INT,
                             storage:
                                 ST_EXTERN as libc::c_int as INTERP_TYPE as
                                     STORAGE_TYPE,
                             objType: VAL_BOOL,
                             index: EXTID_TARGETTYPE as libc::c_int as UDWORD,
                             get:
                                 Some(scrGenExternGet as
                                          unsafe extern "C" fn(_: UDWORD)
                                              -> BOOL),
                             set: None,
                             dimensions: 0,
                             elements: [0; 4],
                             psNext:
                                 0 as *const _var_symbol as
                                     *mut _var_symbol,};
             init
         },
         {
             let mut init =
                 _var_symbol{pIdent:
                                 b"extraVictoryFlag\x00" as *const u8 as
                                     *const libc::c_char as *mut STRING,
                             type_0: VAL_BOOL,
                             storage:
                                 ST_EXTERN as libc::c_int as INTERP_TYPE as
                                     STORAGE_TYPE,
                             objType: VAL_BOOL,
                             index:
                                 EXTID_EXTRAVICTORYFLAG as libc::c_int as
                                     UDWORD,
                             get:
                                 Some(scrGenExternGet as
                                          unsafe extern "C" fn(_: UDWORD)
                                              -> BOOL),
                             set:
                                 Some(scrGenExternSet as
                                          unsafe extern "C" fn(_: UDWORD)
                                              -> BOOL),
                             dimensions: 0,
                             elements: [0; 4],
                             psNext:
                                 0 as *const _var_symbol as
                                     *mut _var_symbol,};
             init
         },
         {
             let mut init =
                 _var_symbol{pIdent:
                                 b"extraFailFlag\x00" as *const u8 as
                                     *const libc::c_char as *mut STRING,
                             type_0: VAL_BOOL,
                             storage:
                                 ST_EXTERN as libc::c_int as INTERP_TYPE as
                                     STORAGE_TYPE,
                             objType: VAL_BOOL,
                             index:
                                 EXTID_EXTRAFAILFLAG as libc::c_int as UDWORD,
                             get:
                                 Some(scrGenExternGet as
                                          unsafe extern "C" fn(_: UDWORD)
                                              -> BOOL),
                             set:
                                 Some(scrGenExternSet as
                                          unsafe extern "C" fn(_: UDWORD)
                                              -> BOOL),
                             dimensions: 0,
                             elements: [0; 4],
                             psNext:
                                 0 as *const _var_symbol as
                                     *mut _var_symbol,};
             init
         },
         {
             let mut init =
                 _var_symbol{pIdent:
                                 b"multiPlayerGameType\x00" as *const u8 as
                                     *const libc::c_char as *mut STRING,
                             type_0: VAL_INT,
                             storage:
                                 ST_EXTERN as libc::c_int as INTERP_TYPE as
                                     STORAGE_TYPE,
                             objType: VAL_BOOL,
                             index:
                                 EXTID_MULTIGAMETYPE as libc::c_int as UDWORD,
                             get:
                                 Some(scrGenExternGet as
                                          unsafe extern "C" fn(_: UDWORD)
                                              -> BOOL),
                             set: None,
                             dimensions: 0,
                             elements: [0; 4],
                             psNext:
                                 0 as *const _var_symbol as
                                     *mut _var_symbol,};
             init
         },
         {
             let mut init =
                 _var_symbol{pIdent:
                                 b"multiPlayerMaxPlayers\x00" as *const u8 as
                                     *const libc::c_char as *mut STRING,
                             type_0: VAL_INT,
                             storage:
                                 ST_EXTERN as libc::c_int as INTERP_TYPE as
                                     STORAGE_TYPE,
                             objType: VAL_BOOL,
                             index:
                                 EXTID_MULTIGAMEHUMANMAX as libc::c_int as
                                     UDWORD,
                             get:
                                 Some(scrGenExternGet as
                                          unsafe extern "C" fn(_: UDWORD)
                                              -> BOOL),
                             set: None,
                             dimensions: 0,
                             elements: [0; 4],
                             psNext:
                                 0 as *const _var_symbol as
                                     *mut _var_symbol,};
             init
         },
         {
             let mut init =
                 _var_symbol{pIdent:
                                 b"multiPlayerBaseType\x00" as *const u8 as
                                     *const libc::c_char as *mut STRING,
                             type_0: VAL_INT,
                             storage:
                                 ST_EXTERN as libc::c_int as INTERP_TYPE as
                                     STORAGE_TYPE,
                             objType: VAL_BOOL,
                             index:
                                 EXTID_MULTIGAMEBASETYPE as libc::c_int as
                                     UDWORD,
                             get:
                                 Some(scrGenExternGet as
                                          unsafe extern "C" fn(_: UDWORD)
                                              -> BOOL),
                             set: None,
                             dimensions: 0,
                             elements: [0; 4],
                             psNext:
                                 0 as *const _var_symbol as
                                     *mut _var_symbol,};
             init
         },
         {
             let mut init =
                 _var_symbol{pIdent: 0 as *const STRING as *mut STRING,
                             type_0: VAL_VOID,
                             storage:
                                 ST_EXTERN as libc::c_int as INTERP_TYPE as
                                     STORAGE_TYPE,
                             objType: VAL_BOOL,
                             index: 0 as libc::c_int as UDWORD,
                             get: None,
                             set: None,
                             dimensions: 0,
                             elements: [0; 4],
                             psNext:
                                 0 as *const _var_symbol as
                                     *mut _var_symbol,};
             init
         }]
    };
/*
 * The table of object variables and their access functions.
 * The table is laid out as follows :
 *
 *   "variable_identifier", <variable type>, (INTERP_TYPE)ST_OBJECT, <object type>, <index>,
 *		<get_function>, <set_function>
 *
 * The Storage type for an object variable is always (INTERP_TYPE)ST_OBJECT.
 * The object type is the type of the object this is a member of.
 * The index is not used by the compiler but is passed to the access function
 * to allow one function to deal with a number of variables
 */
#[no_mangle]
pub static mut asObjTable: [VAR_SYMBOL; 25] =
    unsafe {
        [{
             let mut init =
                 _var_symbol{pIdent:
                                 b"x\x00" as *const u8 as *const libc::c_char
                                     as *mut STRING,
                             type_0: VAL_INT,
                             storage:
                                 ST_OBJECT as libc::c_int as INTERP_TYPE as
                                     STORAGE_TYPE,
                             objType:
                                 ST_BASEOBJECT as libc::c_int as INTERP_TYPE,
                             index: OBJID_POSX as libc::c_int as UDWORD,
                             get:
                                 Some(scrBaseObjGet as
                                          unsafe extern "C" fn(_: UDWORD)
                                              -> BOOL),
                             set: None,
                             dimensions: 0,
                             elements: [0; 4],
                             psNext:
                                 0 as *const _var_symbol as
                                     *mut _var_symbol,};
             init
         },
         {
             let mut init =
                 _var_symbol{pIdent:
                                 b"y\x00" as *const u8 as *const libc::c_char
                                     as *mut STRING,
                             type_0: VAL_INT,
                             storage:
                                 ST_OBJECT as libc::c_int as INTERP_TYPE as
                                     STORAGE_TYPE,
                             objType:
                                 ST_BASEOBJECT as libc::c_int as INTERP_TYPE,
                             index: OBJID_POSY as libc::c_int as UDWORD,
                             get:
                                 Some(scrBaseObjGet as
                                          unsafe extern "C" fn(_: UDWORD)
                                              -> BOOL),
                             set: None,
                             dimensions: 0,
                             elements: [0; 4],
                             psNext:
                                 0 as *const _var_symbol as
                                     *mut _var_symbol,};
             init
         },
         {
             let mut init =
                 _var_symbol{pIdent:
                                 b"z\x00" as *const u8 as *const libc::c_char
                                     as *mut STRING,
                             type_0: VAL_INT,
                             storage:
                                 ST_OBJECT as libc::c_int as INTERP_TYPE as
                                     STORAGE_TYPE,
                             objType:
                                 ST_BASEOBJECT as libc::c_int as INTERP_TYPE,
                             index: OBJID_POSZ as libc::c_int as UDWORD,
                             get:
                                 Some(scrBaseObjGet as
                                          unsafe extern "C" fn(_: UDWORD)
                                              -> BOOL),
                             set: None,
                             dimensions: 0,
                             elements: [0; 4],
                             psNext:
                                 0 as *const _var_symbol as
                                     *mut _var_symbol,};
             init
         },
         {
             let mut init =
                 _var_symbol{pIdent:
                                 b"id\x00" as *const u8 as *const libc::c_char
                                     as *mut STRING,
                             type_0: VAL_INT,
                             storage:
                                 ST_OBJECT as libc::c_int as INTERP_TYPE as
                                     STORAGE_TYPE,
                             objType:
                                 ST_BASEOBJECT as libc::c_int as INTERP_TYPE,
                             index: OBJID_ID as libc::c_int as UDWORD,
                             get:
                                 Some(scrBaseObjGet as
                                          unsafe extern "C" fn(_: UDWORD)
                                              -> BOOL),
                             set: None,
                             dimensions: 0,
                             elements: [0; 4],
                             psNext:
                                 0 as *const _var_symbol as
                                     *mut _var_symbol,};
             init
         },
         {
             let mut init =
                 _var_symbol{pIdent:
                                 b"player\x00" as *const u8 as
                                     *const libc::c_char as *mut STRING,
                             type_0: VAL_INT,
                             storage:
                                 ST_OBJECT as libc::c_int as INTERP_TYPE as
                                     STORAGE_TYPE,
                             objType:
                                 ST_BASEOBJECT as libc::c_int as INTERP_TYPE,
                             index: OBJID_PLAYER as libc::c_int as UDWORD,
                             get:
                                 Some(scrBaseObjGet as
                                          unsafe extern "C" fn(_: UDWORD)
                                              -> BOOL),
                             set: None,
                             dimensions: 0,
                             elements: [0; 4],
                             psNext:
                                 0 as *const _var_symbol as
                                     *mut _var_symbol,};
             init
         },
         {
             let mut init =
                 _var_symbol{pIdent:
                                 b"type\x00" as *const u8 as
                                     *const libc::c_char as *mut STRING,
                             type_0: VAL_INT,
                             storage:
                                 ST_OBJECT as libc::c_int as INTERP_TYPE as
                                     STORAGE_TYPE,
                             objType:
                                 ST_BASEOBJECT as libc::c_int as INTERP_TYPE,
                             index: OBJID_TYPE as libc::c_int as UDWORD,
                             get:
                                 Some(scrBaseObjGet as
                                          unsafe extern "C" fn(_: UDWORD)
                                              -> BOOL),
                             set: None,
                             dimensions: 0,
                             elements: [0; 4],
                             psNext:
                                 0 as *const _var_symbol as
                                     *mut _var_symbol,};
             init
         },
         {
             let mut init =
                 _var_symbol{pIdent:
                                 b"clusterID\x00" as *const u8 as
                                     *const libc::c_char as *mut STRING,
                             type_0: VAL_INT,
                             storage:
                                 ST_OBJECT as libc::c_int as INTERP_TYPE as
                                     STORAGE_TYPE,
                             objType:
                                 ST_BASEOBJECT as libc::c_int as INTERP_TYPE,
                             index: OBJID_CLUSTERID as libc::c_int as UDWORD,
                             get:
                                 Some(scrBaseObjGet as
                                          unsafe extern "C" fn(_: UDWORD)
                                              -> BOOL),
                             set: None,
                             dimensions: 0,
                             elements: [0; 4],
                             psNext:
                                 0 as *const _var_symbol as
                                     *mut _var_symbol,};
             init
         },
         {
             let mut init =
                 _var_symbol{pIdent:
                                 b"health\x00" as *const u8 as
                                     *const libc::c_char as *mut STRING,
                             type_0: VAL_INT,
                             storage:
                                 ST_OBJECT as libc::c_int as INTERP_TYPE as
                                     STORAGE_TYPE,
                             objType:
                                 ST_BASEOBJECT as libc::c_int as INTERP_TYPE,
                             index: OBJID_HEALTH as libc::c_int as UDWORD,
                             get:
                                 Some(scrBaseObjGet as
                                          unsafe extern "C" fn(_: UDWORD)
                                              -> BOOL),
                             set: None,
                             dimensions: 0,
                             elements: [0; 4],
                             psNext:
                                 0 as *const _var_symbol as
                                     *mut _var_symbol,};
             init
         },
         {
             let mut init =
                 _var_symbol{pIdent:
                                 b"order\x00" as *const u8 as
                                     *const libc::c_char as *mut STRING,
                             type_0: VAL_INT,
                             storage:
                                 ST_OBJECT as libc::c_int as INTERP_TYPE as
                                     STORAGE_TYPE,
                             objType: ST_DROID as libc::c_int as INTERP_TYPE,
                             index: OBJID_ORDER as libc::c_int as UDWORD,
                             get:
                                 Some(scrBaseObjGet as
                                          unsafe extern "C" fn(_: UDWORD)
                                              -> BOOL),
                             set: None,
                             dimensions: 0,
                             elements: [0; 4],
                             psNext:
                                 0 as *const _var_symbol as
                                     *mut _var_symbol,};
             init
         },
         {
             let mut init =
                 _var_symbol{pIdent:
                                 b"droidType\x00" as *const u8 as
                                     *const libc::c_char as *mut STRING,
                             type_0: VAL_INT,
                             storage:
                                 ST_OBJECT as libc::c_int as INTERP_TYPE as
                                     STORAGE_TYPE,
                             objType: ST_DROID as libc::c_int as INTERP_TYPE,
                             index: OBJID_DROIDTYPE as libc::c_int as UDWORD,
                             get:
                                 Some(scrBaseObjGet as
                                          unsafe extern "C" fn(_: UDWORD)
                                              -> BOOL),
                             set: None,
                             dimensions: 0,
                             elements: [0; 4],
                             psNext:
                                 0 as *const _var_symbol as
                                     *mut _var_symbol,};
             init
         },
         {
             let mut init =
                 _var_symbol{pIdent:
                                 b"body\x00" as *const u8 as
                                     *const libc::c_char as *mut STRING,
                             type_0: ST_BODY as libc::c_int as INTERP_TYPE,
                             storage:
                                 ST_OBJECT as libc::c_int as INTERP_TYPE as
                                     STORAGE_TYPE,
                             objType: ST_DROID as libc::c_int as INTERP_TYPE,
                             index: OBJID_BODY as libc::c_int as UDWORD,
                             get:
                                 Some(scrBaseObjGet as
                                          unsafe extern "C" fn(_: UDWORD)
                                              -> BOOL),
                             set: None,
                             dimensions: 0,
                             elements: [0; 4],
                             psNext:
                                 0 as *const _var_symbol as
                                     *mut _var_symbol,};
             init
         },
         {
             let mut init =
                 _var_symbol{pIdent:
                                 b"propulsion\x00" as *const u8 as
                                     *const libc::c_char as *mut STRING,
                             type_0:
                                 ST_PROPULSION as libc::c_int as INTERP_TYPE,
                             storage:
                                 ST_OBJECT as libc::c_int as INTERP_TYPE as
                                     STORAGE_TYPE,
                             objType: ST_DROID as libc::c_int as INTERP_TYPE,
                             index: OBJID_PROPULSION as libc::c_int as UDWORD,
                             get:
                                 Some(scrBaseObjGet as
                                          unsafe extern "C" fn(_: UDWORD)
                                              -> BOOL),
                             set: None,
                             dimensions: 0,
                             elements: [0; 4],
                             psNext:
                                 0 as *const _var_symbol as
                                     *mut _var_symbol,};
             init
         },
         {
             let mut init =
                 _var_symbol{pIdent:
                                 b"weapon\x00" as *const u8 as
                                     *const libc::c_char as *mut STRING,
                             type_0: ST_WEAPON as libc::c_int as INTERP_TYPE,
                             storage:
                                 ST_OBJECT as libc::c_int as INTERP_TYPE as
                                     STORAGE_TYPE,
                             objType: ST_DROID as libc::c_int as INTERP_TYPE,
                             index: OBJID_WEAPON as libc::c_int as UDWORD,
                             get:
                                 Some(scrBaseObjGet as
                                          unsafe extern "C" fn(_: UDWORD)
                                              -> BOOL),
                             set: None,
                             dimensions: 0,
                             elements: [0; 4],
                             psNext:
                                 0 as *const _var_symbol as
                                     *mut _var_symbol,};
             init
         },
         {
             let mut init =
                 _var_symbol{pIdent:
                                 b"orderx\x00" as *const u8 as
                                     *const libc::c_char as *mut STRING,
                             type_0: VAL_INT,
                             storage:
                                 ST_OBJECT as libc::c_int as INTERP_TYPE as
                                     STORAGE_TYPE,
                             objType: ST_DROID as libc::c_int as INTERP_TYPE,
                             index: OBJID_ORDERX as libc::c_int as UDWORD,
                             get:
                                 Some(scrBaseObjGet as
                                          unsafe extern "C" fn(_: UDWORD)
                                              -> BOOL),
                             set: None,
                             dimensions: 0,
                             elements: [0; 4],
                             psNext:
                                 0 as *const _var_symbol as
                                     *mut _var_symbol,};
             init
         },
         {
             let mut init =
                 _var_symbol{pIdent:
                                 b"ordery\x00" as *const u8 as
                                     *const libc::c_char as *mut STRING,
                             type_0: VAL_INT,
                             storage:
                                 ST_OBJECT as libc::c_int as INTERP_TYPE as
                                     STORAGE_TYPE,
                             objType: ST_DROID as libc::c_int as INTERP_TYPE,
                             index: OBJID_ORDERY as libc::c_int as UDWORD,
                             get:
                                 Some(scrBaseObjGet as
                                          unsafe extern "C" fn(_: UDWORD)
                                              -> BOOL),
                             set: None,
                             dimensions: 0,
                             elements: [0; 4],
                             psNext:
                                 0 as *const _var_symbol as
                                     *mut _var_symbol,};
             init
         },
         {
             let mut init =
                 _var_symbol{pIdent:
                                 b"x\x00" as *const u8 as *const libc::c_char
                                     as *mut STRING,
                             type_0: VAL_INT,
                             storage:
                                 ST_OBJECT as libc::c_int as INTERP_TYPE as
                                     STORAGE_TYPE,
                             objType: ST_GROUP as libc::c_int as INTERP_TYPE,
                             index: GROUPID_POSX as libc::c_int as UDWORD,
                             get:
                                 Some(scrGroupObjGet as
                                          unsafe extern "C" fn(_: UDWORD)
                                              -> BOOL),
                             set: None,
                             dimensions: 0,
                             elements: [0; 4],
                             psNext:
                                 0 as *const _var_symbol as
                                     *mut _var_symbol,};
             init
         },
         {
             let mut init =
                 _var_symbol{pIdent:
                                 b"y\x00" as *const u8 as *const libc::c_char
                                     as *mut STRING,
                             type_0: VAL_INT,
                             storage:
                                 ST_OBJECT as libc::c_int as INTERP_TYPE as
                                     STORAGE_TYPE,
                             objType: ST_GROUP as libc::c_int as INTERP_TYPE,
                             index: GROUPID_POSY as libc::c_int as UDWORD,
                             get:
                                 Some(scrGroupObjGet as
                                          unsafe extern "C" fn(_: UDWORD)
                                              -> BOOL),
                             set: None,
                             dimensions: 0,
                             elements: [0; 4],
                             psNext:
                                 0 as *const _var_symbol as
                                     *mut _var_symbol,};
             init
         },
         {
             let mut init =
                 _var_symbol{pIdent:
                                 b"members\x00" as *const u8 as
                                     *const libc::c_char as *mut STRING,
                             type_0: VAL_INT,
                             storage:
                                 ST_OBJECT as libc::c_int as INTERP_TYPE as
                                     STORAGE_TYPE,
                             objType: ST_GROUP as libc::c_int as INTERP_TYPE,
                             index: GROUPID_MEMBERS as libc::c_int as UDWORD,
                             get:
                                 Some(scrGroupObjGet as
                                          unsafe extern "C" fn(_: UDWORD)
                                              -> BOOL),
                             set: None,
                             dimensions: 0,
                             elements: [0; 4],
                             psNext:
                                 0 as *const _var_symbol as
                                     *mut _var_symbol,};
             init
         },
         {
             let mut init =
                 _var_symbol{pIdent:
                                 b"health\x00" as *const u8 as
                                     *const libc::c_char as *mut STRING,
                             type_0: VAL_INT,
                             storage:
                                 ST_OBJECT as libc::c_int as INTERP_TYPE as
                                     STORAGE_TYPE,
                             objType: ST_GROUP as libc::c_int as INTERP_TYPE,
                             index: GROUPID_HEALTH as libc::c_int as UDWORD,
                             get:
                                 Some(scrGroupObjGet as
                                          unsafe extern "C" fn(_: UDWORD)
                                              -> BOOL),
                             set: None,
                             dimensions: 0,
                             elements: [0; 4],
                             psNext:
                                 0 as *const _var_symbol as
                                     *mut _var_symbol,};
             init
         },
         {
             let mut init =
                 _var_symbol{pIdent:
                                 b"action\x00" as *const u8 as
                                     *const libc::c_char as *mut STRING,
                             type_0: VAL_INT,
                             storage:
                                 ST_OBJECT as libc::c_int as INTERP_TYPE as
                                     STORAGE_TYPE,
                             objType: ST_DROID as libc::c_int as INTERP_TYPE,
                             index: OBJID_ACTION as libc::c_int as UDWORD,
                             get:
                                 Some(scrBaseObjGet as
                                          unsafe extern "C" fn(_: UDWORD)
                                              -> BOOL),
                             set: None,
                             dimensions: 0,
                             elements: [0; 4],
                             psNext:
                                 0 as *const _var_symbol as
                                     *mut _var_symbol,};
             init
         },
         {
             let mut init =
                 _var_symbol{pIdent:
                                 b"stat\x00" as *const u8 as
                                     *const libc::c_char as *mut STRING,
                             type_0:
                                 ST_STRUCTURESTAT as libc::c_int as
                                     INTERP_TYPE,
                             storage:
                                 ST_OBJECT as libc::c_int as INTERP_TYPE as
                                     STORAGE_TYPE,
                             objType:
                                 ST_BASEOBJECT as libc::c_int as INTERP_TYPE,
                             index: OBJID_STRUCTSTAT as libc::c_int as UDWORD,
                             get:
                                 Some(scrBaseObjGet as
                                          unsafe extern "C" fn(_: UDWORD)
                                              -> BOOL),
                             set: None,
                             dimensions: 0,
                             elements: [0; 4],
                             psNext:
                                 0 as *const _var_symbol as
                                     *mut _var_symbol,};
             init
         },
         {
             let mut init =
                 _var_symbol{pIdent:
                                 b"target\x00" as *const u8 as
                                     *const libc::c_char as *mut STRING,
                             type_0:
                                 ST_BASEOBJECT as libc::c_int as INTERP_TYPE,
                             storage:
                                 ST_OBJECT as libc::c_int as INTERP_TYPE as
                                     STORAGE_TYPE,
                             objType:
                                 ST_BASEOBJECT as libc::c_int as INTERP_TYPE,
                             index: OBJID_TARGET as libc::c_int as UDWORD,
                             get:
                                 Some(scrBaseObjGet as
                                          unsafe extern "C" fn(_: UDWORD)
                                              -> BOOL),
                             set: None,
                             dimensions: 0,
                             elements: [0; 4],
                             psNext:
                                 0 as *const _var_symbol as
                                     *mut _var_symbol,};
             init
         },
         {
             let mut init =
                 _var_symbol{pIdent:
                                 b"stattype\x00" as *const u8 as
                                     *const libc::c_char as *mut STRING,
                             type_0: VAL_INT,
                             storage:
                                 ST_OBJECT as libc::c_int as INTERP_TYPE as
                                     STORAGE_TYPE,
                             objType:
                                 ST_STRUCTURE as libc::c_int as INTERP_TYPE,
                             index:
                                 OBJID_STRUCTSTATTYPE as libc::c_int as
                                     UDWORD,
                             get:
                                 Some(scrBaseObjGet as
                                          unsafe extern "C" fn(_: UDWORD)
                                              -> BOOL),
                             set: None,
                             dimensions: 0,
                             elements: [0; 4],
                             psNext:
                                 0 as *const _var_symbol as
                                     *mut _var_symbol,};
             init
         },
         {
             let mut init =
                 _var_symbol{pIdent:
                                 b"selected\x00" as *const u8 as
                                     *const libc::c_char as *mut STRING,
                             type_0: VAL_BOOL,
                             storage:
                                 ST_OBJECT as libc::c_int as INTERP_TYPE as
                                     STORAGE_TYPE,
                             objType: ST_DROID as libc::c_int as INTERP_TYPE,
                             index: OBJID_SELECTED as libc::c_int as UDWORD,
                             get:
                                 Some(scrBaseObjGet as
                                          unsafe extern "C" fn(_: UDWORD)
                                              -> BOOL),
                             set: None,
                             dimensions: 0,
                             elements: [0; 4],
                             psNext:
                                 0 as *const _var_symbol as
                                     *mut _var_symbol,};
             init
         },
         {
             let mut init =
                 _var_symbol{pIdent: 0 as *const STRING as *mut STRING,
                             type_0: VAL_VOID,
                             storage:
                                 ST_OBJECT as libc::c_int as INTERP_TYPE as
                                     STORAGE_TYPE,
                             objType: VAL_VOID,
                             index: 0 as libc::c_int as UDWORD,
                             get: None,
                             set: None,
                             dimensions: 0,
                             elements: [0; 4],
                             psNext:
                                 0 as *const _var_symbol as
                                     *mut _var_symbol,};
             init
         }]
    };
/* The table of constant variables
 * The format is :
 *
 *	"variable name", <variable type>, <bool value>, <int value>,
 *						<object pointer value>
 *
 * Only the value corresponding to the type should be set, all other values
 * should be 0.
 *
 * Any user-type constants should use the object pointer value.
 */
#[no_mangle]
pub static mut asConstantTable: [CONST_SYMBOL; 254] =
    [{
         let mut init =
             _const_symbol{pIdent:
                               b"OPTIONS\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: 2 as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"CANCEL\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: 8 as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"BUILD\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: 3 as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"MANUFACTURE\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: 4 as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"RESEARCH\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: 5 as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"INTELMAP\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: 6 as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DESIGN\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: 7 as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"COMMAND\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: 9 as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"IDRET_OPTIONS\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: 2 as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"IDRET_CANCEL\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: 8 as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"IDRET_BUILD\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: 3 as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"IDRET_MANUFACTURE\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: 4 as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"IDRET_RESEARCH\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: 5 as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"IDRET_INTELMAP\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: 6 as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"IDRET_DESIGN\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: 7 as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"IDRET_COMMAND\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: 9 as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"IDRET_ORDER\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: 11 as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"IDRET_TRANSPORTER\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: 10 as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"IDDES_TEMPLSTART\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: 5300 as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"IDDES_SYSTEMBUTTON\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: 5900 as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"IDDES_BODYBUTTON\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: 5901 as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"IDDES_PROPBUTTON\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: 5902 as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"IDOBJ_STATSTART\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: 3100 as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"IDOBJ_OBJSTART\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: 3002 as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"IDSTAT_START\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: 4100 as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"RES_MSG\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: MSG_RESEARCH as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"CAMP_MSG\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: MSG_CAMPAIGN as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"MISS_MSG\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: MSG_MISSION as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"PROX_MSG\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: MSG_PROXIMITY as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"NULLTEMPLATE\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: ST_POINTER_T as libc::c_int as INTERP_TYPE,
                           bval: 0 as libc::c_int,
                           ival: 0 as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"NULLOBJECT\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: ST_POINTER_O as libc::c_int as INTERP_TYPE,
                           bval: 0 as libc::c_int,
                           ival: 0 as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"NULLSTAT\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: ST_POINTER_S as libc::c_int as INTERP_TYPE,
                           bval: 0 as libc::c_int,
                           ival: 0 as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"NULLSTRING\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0:
                               ST_TEXTSTRING as libc::c_int as INTERP_TYPE,
                           bval: 0 as libc::c_int,
                           ival: 0 as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"BARBARIAN1\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: 6 as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"BARBARIAN2\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: 7 as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"LZ_COMPROMISED_TIME\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: 999999 as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"OBJ_DROID\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: OBJ_DROID as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"OBJ_STRUCTURE\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: OBJ_STRUCTURE as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"OBJ_FEATURE\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: OBJ_FEATURE as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"CAMP_START\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: LDS_CAMSTART as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"CAMP_EXPAND\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: LDS_EXPAND as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"OFF_KEEP\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: LDS_MKEEP as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"OFF_CLEAR\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: LDS_MCLEAR as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"BETWEEN\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: LDS_BETWEEN as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DROID_WEAPON\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DROID_WEAPON as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DROID_SENSOR\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DROID_SENSOR as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DROID_ECM\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DROID_ECM as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DROID_CONSTRUCT\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DROID_CONSTRUCT as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DROID_PERSON\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DROID_PERSON as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DROID_CYBORG\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DROID_CYBORG as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DROID_TRANSPORTER\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DROID_TRANSPORTER as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DROID_COMMAND\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DROID_COMMAND as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DROID_REPAIR\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DROID_REPAIR as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"REF_HQ\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: REF_HQ as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"REF_FACTORY\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: REF_FACTORY as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"REF_FACTORY_MODULE\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: REF_FACTORY_MODULE as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"REF_POWER_GEN\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: REF_POWER_GEN as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"REF_POWER_MODULE\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: REF_POWER_MODULE as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"REF_RESOURCE_EXTRACTOR\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: REF_RESOURCE_EXTRACTOR as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"REF_DEFENSE\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: REF_DEFENSE as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"REF_WALL\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: REF_WALL as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"REF_WALLCORNER\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: REF_WALLCORNER as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"REF_RESEARCH\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: REF_RESEARCH as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"REF_RESEARCH_MODULE\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: REF_RESEARCH_MODULE as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"REF_REPAIR_FACILITY\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: REF_REPAIR_FACILITY as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"REF_COMMAND_CONTROL\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: REF_COMMAND_CONTROL as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"REF_CYBORG_FACTORY\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: REF_CYBORG_FACTORY as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"REF_VTOL_FACTORY\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: REF_VTOL_FACTORY as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"REF_REARM_PAD\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: REF_REARM_PAD as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"REF_MISSILE_SILO\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: REF_MISSILE_SILO as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DORDER_NONE\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DORDER_NONE as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DORDER_STOP\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DORDER_STOP as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DORDER_MOVE\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DORDER_MOVE as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DORDER_ATTACK\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DORDER_ATTACK as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DORDER_BUILD\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DORDER_BUILD as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DORDER_HELPBUILD\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DORDER_HELPBUILD as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DORDER_LINEBUILD\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DORDER_LINEBUILD as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DORDER_DEMOLISH\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DORDER_DEMOLISH as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DORDER_REPAIR\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DORDER_REPAIR as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DORDER_OBSERVE\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DORDER_OBSERVE as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DORDER_FIRESUPPORT\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DORDER_FIRESUPPORT as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DORDER_RETREAT\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DORDER_RETREAT as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DORDER_DESTRUCT\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DORDER_DESTRUCT as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DORDER_RTB\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DORDER_RTB as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DORDER_RTR\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DORDER_RTR as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DORDER_RUN\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DORDER_RUN as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DORDER_EMBARK\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DORDER_EMBARK as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DORDER_DISEMBARK\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DORDER_DISEMBARK as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DORDER_SCOUT\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DORDER_SCOUT as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DORDER_DROIDREPAIR\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DORDER_DROIDREPAIR as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DACTION_NONE\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DACTION_NONE as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DACTION_MOVE\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DACTION_MOVE as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DACTION_BUILD\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DACTION_BUILD as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DACTION_BUILD_FOUNDATION\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DACTION_BUILD_FOUNDATION as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DACTION_DEMOLISH\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DACTION_DEMOLISH as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DACTION_REPAIR\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DACTION_REPAIR as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DACTION_ATTACK\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DACTION_ATTACK as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DACTION_OBSERVE\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DACTION_OBSERVE as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DACTION_FIRESUPPORT\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DACTION_FIRESUPPORT as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DACTION_SULK\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DACTION_SULK as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DACTION_DESTRUCT\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DACTION_DESTRUCT as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DACTION_TRANSPORTOUT\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DACTION_TRANSPORTOUT as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DACTION_TRANSPORTWAITTOFLYIN\x00" as
                                   *const u8 as *const libc::c_char as
                                   *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DACTION_TRANSPORTWAITTOFLYIN as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DACTION_TRANSPORTIN\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DACTION_TRANSPORTIN as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DACTION_DROIDREPAIR\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DACTION_DROIDREPAIR as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DACTION_RESTORE\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DACTION_RESTORE as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DACTION_MOVEFIRE\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DACTION_MOVEFIRE as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DACTION_MOVETOBUILD\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DACTION_MOVETOBUILD as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DACTION_MOVETODEMOLISH\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DACTION_MOVETODEMOLISH as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DACTION_MOVETOREPAIR\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DACTION_MOVETOREPAIR as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DACTION_BUILDWANDER\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DACTION_BUILDWANDER as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DACTION_FOUNDATION_WANDER\x00" as *const u8
                                   as *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DACTION_FOUNDATION_WANDER as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DACTION_MOVETOATTACK\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DACTION_MOVETOATTACK as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DACTION_ROTATETOATTACK\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DACTION_ROTATETOATTACK as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DACTION_MOVETOOBSERVE\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DACTION_MOVETOOBSERVE as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DACTION_WAITFORREPAIR\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DACTION_WAITFORREPAIR as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DACTION_MOVETOREPAIRPOINT\x00" as *const u8
                                   as *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DACTION_MOVETOREPAIRPOINT as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DACTION_WAITDURINGREPAIR\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DACTION_WAITDURINGREPAIR as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DACTION_MOVETODROIDREPAIR\x00" as *const u8
                                   as *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DACTION_MOVETODROIDREPAIR as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DACTION_MOVETORESTORE\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DACTION_MOVETORESTORE as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DACTION_MOVETOREARM\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DACTION_MOVETOREARM as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DACTION_WAITFORREARM\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DACTION_WAITFORREARM as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DACTION_MOVETOREARMPOINT\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DACTION_MOVETOREARMPOINT as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DACTION_WAITDURINGREARM\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DACTION_WAITDURINGREARM as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DACTION_VTOLATTACK\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DACTION_VTOLATTACK as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DACTION_CLEARREARMPAD\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DACTION_CLEARREARMPAD as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DACTION_RETURNTOPOS\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DACTION_RETURNTOPOS as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DACTION_FIRESUPPORT_RETREAT\x00" as *const u8
                                   as *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DACTION_FIRESUPPORT_RETREAT as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DSO_ATTACK_RANGE\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DSO_ATTACK_RANGE as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DSO_REPAIR_LEVEL\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DSO_REPAIR_LEVEL as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DSO_ATTACK_LEVEL\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DSO_ATTACK_LEVEL as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DSO_RECYCLE\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DSO_RECYCLE as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DSO_PATROL\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DSO_PATROL as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DSO_HALTTYPE\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DSO_HALTTYPE as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DSO_RETURN_TO_LOC\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DSO_RETURN_TO_LOC as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DSS_ARANGE_SHORT\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DSS_ARANGE_SHORT as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DSS_ARANGE_LONG\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DSS_ARANGE_LONG as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DSS_ARANGE_DEFAULT\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DSS_ARANGE_DEFAULT as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DSS_REPLEV_LOW\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DSS_REPLEV_LOW as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DSS_REPLEV_HIGH\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DSS_REPLEV_HIGH as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DSS_REPLEV_NEVER\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DSS_REPLEV_NEVER as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DSS_ALEV_ALWAYS\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DSS_ALEV_ALWAYS as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DSS_ALEV_ATTACKED\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DSS_ALEV_ATTACKED as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DSS_ALEV_NEVER\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DSS_ALEV_NEVER as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DSS_HALT_HOLD\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DSS_HALT_HOLD as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DSS_HALT_GUARD\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DSS_HALT_GUARD as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DSS_HALT_PERSUE\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DSS_HALT_PERSUE as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DSS_RECYCLE_SET\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DSS_RECYCLE_SET as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DSS_ASSPROD_START\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DSS_ASSPROD_START as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DSS_ASSPROD_END \x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DSS_ASSPROD_END as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DSS_RTL_REPAIR\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DSS_RTL_REPAIR as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DSS_RTL_BASE\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DSS_RTL_BASE as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DSS_RTL_TRANSPORT\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DSS_RTL_TRANSPORT as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DSS_PATROL_SET\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: DSS_PATROL_SET as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"IDRET_OPTIONS\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: 2 as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"IDRET_BUILD\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: 3 as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"IDRET_MANUFACTURE\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: 4 as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"IDRET_RESEARCH\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: 5 as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"IDRET_INTELMAP\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: 6 as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"IDRET_DESIGN\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: 7 as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"IDRET_CANCEL\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: 8 as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"IDRET_COMMAND\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: 9 as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"IMAGE_CURSOR_SELECT\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: IMAGE_CURSOR_SELECT as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"IMAGE_CURSOR_ATTACK\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: IMAGE_CURSOR_ATTACK as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"IMAGE_CURSOR_MOVE\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: IMAGE_CURSOR_MOVE as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"IMAGE_CURSOR_ECM\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: IMAGE_CURSOR_ECM as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"IMAGE_CURSOR_REPAIR\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: IMAGE_CURSOR_REPAIR as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"IMAGE_CURSOR_PICKUP\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: IMAGE_CURSOR_PICKUP as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"IMAGE_CURSOR_DEFAULT\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: IMAGE_CURSOR_DEFAULT as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"IMAGE_CURSOR_BUILD\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: IMAGE_CURSOR_BUILD as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"IMAGE_CURSOR_GUARD\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: IMAGE_CURSOR_GUARD as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"IMAGE_CURSOR_BRIDGE\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: IMAGE_CURSOR_BRIDGE as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"IMAGE_CURSOR_ATTACH\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: IMAGE_CURSOR_ATTACH as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"IMAGE_CURSOR_LOCKON\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: IMAGE_CURSOR_LOCKON as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"IMAGE_CURSOR_FIX\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: IMAGE_CURSOR_FIX as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"IMAGE_CURSOR_EMBARK\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: IMAGE_CURSOR_EMBARK as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"INT_NORMAL\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: INT_NORMAL as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"INT_OPTION\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: INT_OPTION as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"INT_EDITSTAT\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: INT_EDITSTAT as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"INT_EDIT\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: INT_EDIT as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"INT_OBJECT\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: INT_OBJECT as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"INT_STAT\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: INT_STAT as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"INT_CMDORDER\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: INT_CMDORDER as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"INT_DESIGN\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: INT_DESIGN as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"INT_INTELMAP\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: INT_INTELMAP as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"INT_ORDER\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: INT_ORDER as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"INT_INGAMEOP\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: INT_INGAMEOP as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"INT_TRANSPORTER\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: INT_TRANSPORTER as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"INT_MISSIONRES\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: INT_MISSIONRES as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"INT_MULTIMENU\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: INT_MULTIMENU as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"STATUS_ReticuleIsOpen\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: STATUS_ReticuleIsOpen as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"STATUS_BattleMapViewEnabled\x00" as *const u8
                                   as *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: STATUS_BattleMapViewEnabled as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"STATUS_DeliveryReposInProgress\x00" as
                                   *const u8 as *const libc::c_char as
                                   *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival:
                               STATUS_DeliveryReposInProgress as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"MT_TERRAIN\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: MT_TERRAIN as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"MT_RESOURCE\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: MT_RESOURCE as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"MT_BLOCKING\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: MT_BLOCKING as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"MT_RIVER\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: MT_RIVER as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"MT_TRENCH\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: MT_TRENCH as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"MT_OWNSTRDAM\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: MT_OWNSTRDAM as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"MT_OWNSTROK\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: MT_OWNSTROK as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"MT_OWNSTRINCOMP\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: MT_OWNSTRINCOMP as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"MT_REPAIR\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: MT_REPAIR as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"MT_REPAIRDAM\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: MT_REPAIRDAM as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"MT_ENEMYSTR\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: MT_ENEMYSTR as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"MT_TRANDROID\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: MT_TRANDROID as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"MT_OWNDROID\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: MT_OWNDROID as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"MT_OWNDROIDDAM\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: MT_OWNDROIDDAM as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"MT_ENEMYDROID\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: MT_ENEMYDROID as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"MT_COMMAND\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: MT_COMMAND as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"MT_ARTIFACT\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: MT_ARTIFACT as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"MT_DAMFEATURE\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: MT_DAMFEATURE as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"MT_SENSOR\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: MT_SENSOR as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"ST_HQ\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: SCR_ST_HQ as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"ST_FACTORY\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: SCR_ST_FACTORY as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"ST_POWER_GEN\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: SCR_ST_POWER_GEN as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"ST_RESOURCE_EXTRACTOR\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: SCR_ST_RESOURCE_EXTRACTOR as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"ST_WALL\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: SCR_ST_WALL as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"ST_RESEARCH\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: SCR_ST_RESEARCH as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"ST_REPAIR_FACILITY\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: SCR_ST_REPAIR_FACILITY as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"ST_COMMAND_CONTROL\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: SCR_ST_COMMAND_CONTROL as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"ST_CYBORG_FACTORY\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: SCR_ST_CYBORG_FACTORY as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"ST_VTOL_FACTORY\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: SCR_ST_VTOL_FACTORY as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"ST_REARM_PAD\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: SCR_ST_REARM_PAD as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"ST_SENSOR\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: SCR_ST_SENSOR as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"ST_DEF_GROUND\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: SCR_ST_DEF_GROUND as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"ST_DEF_AIR\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: SCR_ST_DEF_AIR as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"ST_DEF_IDF\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: SCR_ST_DEF_IDF as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"ST_DEF_ALL\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: SCR_ST_DEF_ALL as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DT_COMMAND\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: SCR_DT_COMMAND as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DT_SENSOR\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: SCR_DT_SENSOR as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DT_CONSTRUCT\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: SCR_DT_CONSTRUCT as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DT_REPAIR\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: SCR_DT_REPAIR as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DT_WEAP_GROUND\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: SCR_DT_WEAP_GROUND as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DT_WEAP_AIR\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: SCR_DT_WEAP_AIR as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DT_WEAP_IDF\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: SCR_DT_WEAP_IDF as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DT_WEAP_ALL\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: SCR_DT_WEAP_ALL as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DT_LIGHT\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: SCR_DT_LIGHT as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DT_MEDIUM\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: SCR_DT_MEDIUM as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DT_HEAVY\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: SCR_DT_HEAVY as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DT_SUPER_HEAVY\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: SCR_DT_SUPER_HEAVY as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DT_TRACK\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: SCR_DT_TRACK as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DT_HTRACK\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: SCR_DT_HTRACK as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DT_WHEEL\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: SCR_DT_WHEEL as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DT_LEGS\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: SCR_DT_LEGS as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DT_GROUND\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: SCR_DT_GROUND as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DT_VTOL\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: SCR_DT_VTOL as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"DT_HOVER\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: SCR_DT_HOVER as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"CAMPAIGN\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: 12 as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"TEAMPLAY\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: 13 as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"SKIRMISH\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: 14 as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"CAMP_CLEAN\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: 0 as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"CAMP_BASE\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: 1 as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"CAMP_WALLS\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_INT,
                           bval: 0 as libc::c_int,
                           ival: 2 as libc::c_int,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     },
     {
         let mut init =
             _const_symbol{pIdent:
                               b"CONSTANT LIST END\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                           type_0: VAL_VOID,
                           bval: 0,
                           ival: 0,
                           oval:
                               0 as *const libc::c_void as *mut libc::c_void,
                           sval: 0 as *const STRING as *mut STRING,};
         init
     }];
/* The Table of callback triggers
 * The format is :
 *
 * "callback name", <callback id>
 *
 * The callback id should be a unique id number starting at TR_CALLBACKSTART
 * and increasing sequentially by 1
 */
#[no_mangle]
pub static mut asCallbackTable: [CALLBACK_SYMBOL; 58] =
    unsafe {
        [{
             let mut init =
                 _callback_symbol{pIdent:
                                      b"CALL_GAMEINIT\x00" as *const u8 as
                                          *const libc::c_char as *mut STRING,
                                  type_0:
                                      CALL_GAMEINIT as libc::c_int as
                                          TRIGGER_TYPE,
                                  pFunc: None,
                                  numParams: 0 as libc::c_int as UDWORD,
                                  aParams: [VAL_BOOL; 20],};
             init
         },
         {
             let mut init =
                 _callback_symbol{pIdent:
                                      b"CALL_DELIVPOINTMOVED\x00" as *const u8
                                          as *const libc::c_char as
                                          *mut STRING,
                                  type_0:
                                      CALL_DELIVPOINTMOVED as libc::c_int as
                                          TRIGGER_TYPE,
                                  pFunc: None,
                                  numParams: 0 as libc::c_int as UDWORD,
                                  aParams: [VAL_BOOL; 20],};
             init
         },
         {
             let mut init =
                 _callback_symbol{pIdent:
                                      b"CALL_DROIDDESIGNED\x00" as *const u8
                                          as *const libc::c_char as
                                          *mut STRING,
                                  type_0:
                                      CALL_DROIDDESIGNED as libc::c_int as
                                          TRIGGER_TYPE,
                                  pFunc: None,
                                  numParams: 0 as libc::c_int as UDWORD,
                                  aParams: [VAL_BOOL; 20],};
             init
         },
         {
             let mut init =
                 _callback_symbol{pIdent:
                                      b"CALL_DROIDBUILT\x00" as *const u8 as
                                          *const libc::c_char as *mut STRING,
                                  type_0:
                                      CALL_DROIDBUILT as libc::c_int as
                                          TRIGGER_TYPE,
                                  pFunc: None,
                                  numParams: 0 as libc::c_int as UDWORD,
                                  aParams: [VAL_BOOL; 20],};
             init
         },
         {
             let mut init =
                 _callback_symbol{pIdent:
                                      b"CALL_POWERGEN_BUILT\x00" as *const u8
                                          as *const libc::c_char as
                                          *mut STRING,
                                  type_0:
                                      CALL_POWERGEN_BUILT as libc::c_int as
                                          TRIGGER_TYPE,
                                  pFunc: None,
                                  numParams: 0 as libc::c_int as UDWORD,
                                  aParams: [VAL_BOOL; 20],};
             init
         },
         {
             let mut init =
                 _callback_symbol{pIdent:
                                      b"CALL_RESEX_BUILT\x00" as *const u8 as
                                          *const libc::c_char as *mut STRING,
                                  type_0:
                                      CALL_RESEX_BUILT as libc::c_int as
                                          TRIGGER_TYPE,
                                  pFunc: None,
                                  numParams: 0 as libc::c_int as UDWORD,
                                  aParams: [VAL_BOOL; 20],};
             init
         },
         {
             let mut init =
                 _callback_symbol{pIdent:
                                      b"CALL_RESEARCH_BUILT\x00" as *const u8
                                          as *const libc::c_char as
                                          *mut STRING,
                                  type_0:
                                      CALL_RESEARCH_BUILT as libc::c_int as
                                          TRIGGER_TYPE,
                                  pFunc: None,
                                  numParams: 0 as libc::c_int as UDWORD,
                                  aParams: [VAL_BOOL; 20],};
             init
         },
         {
             let mut init =
                 _callback_symbol{pIdent:
                                      b"CALL_FACTORY_BUILT\x00" as *const u8
                                          as *const libc::c_char as
                                          *mut STRING,
                                  type_0:
                                      CALL_FACTORY_BUILT as libc::c_int as
                                          TRIGGER_TYPE,
                                  pFunc: None,
                                  numParams: 0 as libc::c_int as UDWORD,
                                  aParams: [VAL_BOOL; 20],};
             init
         },
         {
             let mut init =
                 _callback_symbol{pIdent:
                                      b"CALL_MISSION_START\x00" as *const u8
                                          as *const libc::c_char as
                                          *mut STRING,
                                  type_0:
                                      CALL_MISSION_START as libc::c_int as
                                          TRIGGER_TYPE,
                                  pFunc: None,
                                  numParams: 0 as libc::c_int as UDWORD,
                                  aParams: [VAL_BOOL; 20],};
             init
         },
         {
             let mut init =
                 _callback_symbol{pIdent:
                                      b"CALL_MISSION_END\x00" as *const u8 as
                                          *const libc::c_char as *mut STRING,
                                  type_0:
                                      CALL_MISSION_END as libc::c_int as
                                          TRIGGER_TYPE,
                                  pFunc: None,
                                  numParams: 0 as libc::c_int as UDWORD,
                                  aParams: [VAL_BOOL; 20],};
             init
         },
         {
             let mut init =
                 _callback_symbol{pIdent:
                                      b"CALL_VIDEO_QUIT\x00" as *const u8 as
                                          *const libc::c_char as *mut STRING,
                                  type_0:
                                      CALL_VIDEO_QUIT as libc::c_int as
                                          TRIGGER_TYPE,
                                  pFunc: None,
                                  numParams: 0 as libc::c_int as UDWORD,
                                  aParams: [VAL_BOOL; 20],};
             init
         },
         {
             let mut init =
                 _callback_symbol{pIdent:
                                      b"CALL_LAUNCH_TRANSPORTER\x00" as
                                          *const u8 as *const libc::c_char as
                                          *mut STRING,
                                  type_0:
                                      CALL_LAUNCH_TRANSPORTER as libc::c_int
                                          as TRIGGER_TYPE,
                                  pFunc: None,
                                  numParams: 0 as libc::c_int as UDWORD,
                                  aParams: [VAL_BOOL; 20],};
             init
         },
         {
             let mut init =
                 _callback_symbol{pIdent:
                                      b"CALL_START_NEXT_LEVEL\x00" as
                                          *const u8 as *const libc::c_char as
                                          *mut STRING,
                                  type_0:
                                      CALL_START_NEXT_LEVEL as libc::c_int as
                                          TRIGGER_TYPE,
                                  pFunc: None,
                                  numParams: 0 as libc::c_int as UDWORD,
                                  aParams: [VAL_BOOL; 20],};
             init
         },
         {
             let mut init =
                 _callback_symbol{pIdent:
                                      b"CALL_TRANSPORTER_REINFORCE\x00" as
                                          *const u8 as *const libc::c_char as
                                          *mut STRING,
                                  type_0:
                                      CALL_TRANSPORTER_REINFORCE as
                                          libc::c_int as TRIGGER_TYPE,
                                  pFunc: None,
                                  numParams: 0 as libc::c_int as UDWORD,
                                  aParams: [VAL_BOOL; 20],};
             init
         },
         {
             let mut init =
                 _callback_symbol{pIdent:
                                      b"CALL_MISSION_TIME\x00" as *const u8 as
                                          *const libc::c_char as *mut STRING,
                                  type_0:
                                      CALL_MISSION_TIME as libc::c_int as
                                          TRIGGER_TYPE,
                                  pFunc: None,
                                  numParams: 0 as libc::c_int as UDWORD,
                                  aParams: [VAL_BOOL; 20],};
             init
         },
         {
             let mut init =
                 _callback_symbol{pIdent:
                                      b"CALL_ELECTRONIC_TAKEOVER\x00" as
                                          *const u8 as *const libc::c_char as
                                          *mut STRING,
                                  type_0:
                                      CALL_ELECTRONIC_TAKEOVER as libc::c_int
                                          as TRIGGER_TYPE,
                                  pFunc: None,
                                  numParams: 0 as libc::c_int as UDWORD,
                                  aParams: [VAL_BOOL; 20],};
             init
         },
         {
             let mut init =
                 _callback_symbol{pIdent:
                                      b"CALL_BUILDLIST\x00" as *const u8 as
                                          *const libc::c_char as *mut STRING,
                                  type_0:
                                      CALL_BUILDLIST as libc::c_int as
                                          TRIGGER_TYPE,
                                  pFunc: None,
                                  numParams: 0 as libc::c_int as UDWORD,
                                  aParams: [VAL_BOOL; 20],};
             init
         },
         {
             let mut init =
                 _callback_symbol{pIdent:
                                      b"CALL_BUILDGRID\x00" as *const u8 as
                                          *const libc::c_char as *mut STRING,
                                  type_0:
                                      CALL_BUILDGRID as libc::c_int as
                                          TRIGGER_TYPE,
                                  pFunc: None,
                                  numParams: 0 as libc::c_int as UDWORD,
                                  aParams: [VAL_BOOL; 20],};
             init
         },
         {
             let mut init =
                 _callback_symbol{pIdent:
                                      b"CALL_RESEARCHLIST\x00" as *const u8 as
                                          *const libc::c_char as *mut STRING,
                                  type_0:
                                      CALL_RESEARCHLIST as libc::c_int as
                                          TRIGGER_TYPE,
                                  pFunc: None,
                                  numParams: 0 as libc::c_int as UDWORD,
                                  aParams: [VAL_BOOL; 20],};
             init
         },
         {
             let mut init =
                 _callback_symbol{pIdent:
                                      b"CALL_MANURUN\x00" as *const u8 as
                                          *const libc::c_char as *mut STRING,
                                  type_0:
                                      CALL_MANURUN as libc::c_int as
                                          TRIGGER_TYPE,
                                  pFunc: None,
                                  numParams: 0 as libc::c_int as UDWORD,
                                  aParams: [VAL_BOOL; 20],};
             init
         },
         {
             let mut init =
                 _callback_symbol{pIdent:
                                      b"CALL_MANULIST\x00" as *const u8 as
                                          *const libc::c_char as *mut STRING,
                                  type_0:
                                      CALL_MANULIST as libc::c_int as
                                          TRIGGER_TYPE,
                                  pFunc: None,
                                  numParams: 0 as libc::c_int as UDWORD,
                                  aParams: [VAL_BOOL; 20],};
             init
         },
         {
             let mut init =
                 _callback_symbol{pIdent:
                                      b"CALL_BUTTON_PRESSED\x00" as *const u8
                                          as *const libc::c_char as
                                          *mut STRING,
                                  type_0:
                                      CALL_BUTTON_PRESSED as libc::c_int as
                                          TRIGGER_TYPE,
                                  pFunc:
                                      Some(scrCBButtonPressed as
                                               unsafe extern "C" fn()
                                                   -> BOOL),
                                  numParams: 1 as libc::c_int as UDWORD,
                                  aParams:
                                      [VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL],};
             init
         },
         {
             let mut init =
                 _callback_symbol{pIdent:
                                      b"CALL_DROID_SELECTED\x00" as *const u8
                                          as *const libc::c_char as
                                          *mut STRING,
                                  type_0:
                                      CALL_DROID_SELECTED as libc::c_int as
                                          TRIGGER_TYPE,
                                  pFunc:
                                      Some(scrCBDroidSelected as
                                               unsafe extern "C" fn()
                                                   -> BOOL),
                                  numParams: 1 as libc::c_int as UDWORD,
                                  aParams:
                                      [(0x100000 as libc::c_int as
                                            libc::c_uint |
                                            ST_DROID as libc::c_int as
                                                INTERP_TYPE as libc::c_uint)
                                           as INTERP_TYPE, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL],};
             init
         },
         {
             let mut init =
                 _callback_symbol{pIdent:
                                      b"CALL_DESIGN_QUIT\x00" as *const u8 as
                                          *const libc::c_char as *mut STRING,
                                  type_0:
                                      CALL_DESIGN_QUIT as libc::c_int as
                                          TRIGGER_TYPE,
                                  pFunc: None,
                                  numParams: 0 as libc::c_int as UDWORD,
                                  aParams: [VAL_BOOL; 20],};
             init
         },
         {
             let mut init =
                 _callback_symbol{pIdent:
                                      b"CALL_DESIGN_WEAPON\x00" as *const u8
                                          as *const libc::c_char as
                                          *mut STRING,
                                  type_0:
                                      CALL_DESIGN_WEAPON as libc::c_int as
                                          TRIGGER_TYPE,
                                  pFunc: None,
                                  numParams: 0 as libc::c_int as UDWORD,
                                  aParams: [VAL_BOOL; 20],};
             init
         },
         {
             let mut init =
                 _callback_symbol{pIdent:
                                      b"CALL_DESIGN_SYSTEM\x00" as *const u8
                                          as *const libc::c_char as
                                          *mut STRING,
                                  type_0:
                                      CALL_DESIGN_SYSTEM as libc::c_int as
                                          TRIGGER_TYPE,
                                  pFunc: None,
                                  numParams: 0 as libc::c_int as UDWORD,
                                  aParams: [VAL_BOOL; 20],};
             init
         },
         {
             let mut init =
                 _callback_symbol{pIdent:
                                      b"CALL_DESIGN_COMMAND\x00" as *const u8
                                          as *const libc::c_char as
                                          *mut STRING,
                                  type_0:
                                      CALL_DESIGN_COMMAND as libc::c_int as
                                          TRIGGER_TYPE,
                                  pFunc: None,
                                  numParams: 0 as libc::c_int as UDWORD,
                                  aParams: [VAL_BOOL; 20],};
             init
         },
         {
             let mut init =
                 _callback_symbol{pIdent:
                                      b"CALL_DESIGN_BODY\x00" as *const u8 as
                                          *const libc::c_char as *mut STRING,
                                  type_0:
                                      CALL_DESIGN_BODY as libc::c_int as
                                          TRIGGER_TYPE,
                                  pFunc: None,
                                  numParams: 0 as libc::c_int as UDWORD,
                                  aParams: [VAL_BOOL; 20],};
             init
         },
         {
             let mut init =
                 _callback_symbol{pIdent:
                                      b"CALL_DESIGN_PROPULSION\x00" as
                                          *const u8 as *const libc::c_char as
                                          *mut STRING,
                                  type_0:
                                      CALL_DESIGN_PROPULSION as libc::c_int as
                                          TRIGGER_TYPE,
                                  pFunc: None,
                                  numParams: 0 as libc::c_int as UDWORD,
                                  aParams: [VAL_BOOL; 20],};
             init
         },
         {
             let mut init =
                 _callback_symbol{pIdent:
                                      b"CALL_RESEARCHCOMPLETED\x00" as
                                          *const u8 as *const libc::c_char as
                                          *mut STRING,
                                  type_0:
                                      CALL_RESEARCHCOMPLETED as libc::c_int as
                                          TRIGGER_TYPE,
                                  pFunc:
                                      Some(scrCBResCompleted as
                                               unsafe extern "C" fn()
                                                   -> BOOL),
                                  numParams: 1 as libc::c_int as UDWORD,
                                  aParams:
                                      [(0x100000 as libc::c_int as
                                            libc::c_uint |
                                            ST_RESEARCH as libc::c_int as
                                                INTERP_TYPE as libc::c_uint)
                                           as INTERP_TYPE, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL],};
             init
         },
         {
             let mut init =
                 _callback_symbol{pIdent:
                                      b"CALL_NEWDROID\x00" as *const u8 as
                                          *const libc::c_char as *mut STRING,
                                  type_0:
                                      CALL_NEWDROID as libc::c_int as
                                          TRIGGER_TYPE,
                                  pFunc:
                                      Some(scrCBNewDroid as
                                               unsafe extern "C" fn()
                                                   -> BOOL),
                                  numParams: 3 as libc::c_int as UDWORD,
                                  aParams:
                                      [VAL_INT,
                                       (0x100000 as libc::c_int as
                                            libc::c_uint |
                                            ST_DROID as libc::c_int as
                                                INTERP_TYPE as libc::c_uint)
                                           as INTERP_TYPE,
                                       (0x100000 as libc::c_int as
                                            libc::c_uint |
                                            ST_STRUCTURE as libc::c_int as
                                                INTERP_TYPE as libc::c_uint)
                                           as INTERP_TYPE, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL],};
             init
         },
         {
             let mut init =
                 _callback_symbol{pIdent:
                                      b"CALL_STRUCT_ATTACKED\x00" as *const u8
                                          as *const libc::c_char as
                                          *mut STRING,
                                  type_0:
                                      CALL_STRUCT_ATTACKED as libc::c_int as
                                          TRIGGER_TYPE,
                                  pFunc:
                                      Some(scrCBStructAttacked as
                                               unsafe extern "C" fn()
                                                   -> BOOL),
                                  numParams: 3 as libc::c_int as UDWORD,
                                  aParams:
                                      [VAL_INT,
                                       (0x100000 as libc::c_int as
                                            libc::c_uint |
                                            ST_STRUCTURE as libc::c_int as
                                                INTERP_TYPE as libc::c_uint)
                                           as INTERP_TYPE,
                                       (0x100000 as libc::c_int as
                                            libc::c_uint |
                                            ST_BASEOBJECT as libc::c_int as
                                                INTERP_TYPE as libc::c_uint)
                                           as INTERP_TYPE, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL],};
             init
         },
         {
             let mut init =
                 _callback_symbol{pIdent:
                                      b"CALL_DROID_ATTACKED\x00" as *const u8
                                          as *const libc::c_char as
                                          *mut STRING,
                                  type_0:
                                      CALL_DROID_ATTACKED as libc::c_int as
                                          TRIGGER_TYPE,
                                  pFunc:
                                      Some(scrCBDroidAttacked as
                                               unsafe extern "C" fn()
                                                   -> BOOL),
                                  numParams: 3 as libc::c_int as UDWORD,
                                  aParams:
                                      [VAL_INT,
                                       (0x100000 as libc::c_int as
                                            libc::c_uint |
                                            ST_DROID as libc::c_int as
                                                INTERP_TYPE as libc::c_uint)
                                           as INTERP_TYPE,
                                       (0x100000 as libc::c_int as
                                            libc::c_uint |
                                            ST_BASEOBJECT as libc::c_int as
                                                INTERP_TYPE as libc::c_uint)
                                           as INTERP_TYPE, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL],};
             init
         },
         {
             let mut init =
                 _callback_symbol{pIdent:
                                      b"CALL_ATTACKED\x00" as *const u8 as
                                          *const libc::c_char as *mut STRING,
                                  type_0:
                                      CALL_ATTACKED as libc::c_int as
                                          TRIGGER_TYPE,
                                  pFunc:
                                      Some(scrCBAttacked as
                                               unsafe extern "C" fn()
                                                   -> BOOL),
                                  numParams: 3 as libc::c_int as UDWORD,
                                  aParams:
                                      [VAL_INT,
                                       (0x100000 as libc::c_int as
                                            libc::c_uint |
                                            ST_BASEOBJECT as libc::c_int as
                                                INTERP_TYPE as libc::c_uint)
                                           as INTERP_TYPE,
                                       (0x100000 as libc::c_int as
                                            libc::c_uint |
                                            ST_BASEOBJECT as libc::c_int as
                                                INTERP_TYPE as libc::c_uint)
                                           as INTERP_TYPE, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL],};
             init
         },
         {
             let mut init =
                 _callback_symbol{pIdent:
                                      b"CALL_STRUCT_SEEN\x00" as *const u8 as
                                          *const libc::c_char as *mut STRING,
                                  type_0:
                                      CALL_STRUCT_SEEN as libc::c_int as
                                          TRIGGER_TYPE,
                                  pFunc:
                                      Some(scrCBStructSeen as
                                               unsafe extern "C" fn()
                                                   -> BOOL),
                                  numParams: 3 as libc::c_int as UDWORD,
                                  aParams:
                                      [VAL_INT,
                                       (0x100000 as libc::c_int as
                                            libc::c_uint |
                                            ST_STRUCTURE as libc::c_int as
                                                INTERP_TYPE as libc::c_uint)
                                           as INTERP_TYPE,
                                       (0x100000 as libc::c_int as
                                            libc::c_uint |
                                            ST_BASEOBJECT as libc::c_int as
                                                INTERP_TYPE as libc::c_uint)
                                           as INTERP_TYPE, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL],};
             init
         },
         {
             let mut init =
                 _callback_symbol{pIdent:
                                      b"CALL_DROID_SEEN\x00" as *const u8 as
                                          *const libc::c_char as *mut STRING,
                                  type_0:
                                      CALL_DROID_SEEN as libc::c_int as
                                          TRIGGER_TYPE,
                                  pFunc:
                                      Some(scrCBDroidSeen as
                                               unsafe extern "C" fn()
                                                   -> BOOL),
                                  numParams: 3 as libc::c_int as UDWORD,
                                  aParams:
                                      [VAL_INT,
                                       (0x100000 as libc::c_int as
                                            libc::c_uint |
                                            ST_DROID as libc::c_int as
                                                INTERP_TYPE as libc::c_uint)
                                           as INTERP_TYPE,
                                       (0x100000 as libc::c_int as
                                            libc::c_uint |
                                            ST_BASEOBJECT as libc::c_int as
                                                INTERP_TYPE as libc::c_uint)
                                           as INTERP_TYPE, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL],};
             init
         },
         {
             let mut init =
                 _callback_symbol{pIdent:
                                      b"CALL_FEATURE_SEEN\x00" as *const u8 as
                                          *const libc::c_char as *mut STRING,
                                  type_0:
                                      CALL_FEATURE_SEEN as libc::c_int as
                                          TRIGGER_TYPE,
                                  pFunc:
                                      Some(scrCBFeatureSeen as
                                               unsafe extern "C" fn()
                                                   -> BOOL),
                                  numParams: 3 as libc::c_int as UDWORD,
                                  aParams:
                                      [VAL_INT,
                                       (0x100000 as libc::c_int as
                                            libc::c_uint |
                                            ST_FEATURE as libc::c_int as
                                                INTERP_TYPE as libc::c_uint)
                                           as INTERP_TYPE,
                                       (0x100000 as libc::c_int as
                                            libc::c_uint |
                                            ST_BASEOBJECT as libc::c_int as
                                                INTERP_TYPE as libc::c_uint)
                                           as INTERP_TYPE, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL],};
             init
         },
         {
             let mut init =
                 _callback_symbol{pIdent:
                                      b"CALL_OBJ_SEEN\x00" as *const u8 as
                                          *const libc::c_char as *mut STRING,
                                  type_0:
                                      CALL_OBJ_SEEN as libc::c_int as
                                          TRIGGER_TYPE,
                                  pFunc:
                                      Some(scrCBObjSeen as
                                               unsafe extern "C" fn()
                                                   -> BOOL),
                                  numParams: 3 as libc::c_int as UDWORD,
                                  aParams:
                                      [VAL_INT,
                                       (0x100000 as libc::c_int as
                                            libc::c_uint |
                                            ST_BASEOBJECT as libc::c_int as
                                                INTERP_TYPE as libc::c_uint)
                                           as INTERP_TYPE,
                                       (0x100000 as libc::c_int as
                                            libc::c_uint |
                                            ST_BASEOBJECT as libc::c_int as
                                                INTERP_TYPE as libc::c_uint)
                                           as INTERP_TYPE, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL],};
             init
         },
         {
             let mut init =
                 _callback_symbol{pIdent:
                                      b"CALL_OBJ_DESTROYED\x00" as *const u8
                                          as *const libc::c_char as
                                          *mut STRING,
                                  type_0:
                                      CALL_OBJ_DESTROYED as libc::c_int as
                                          TRIGGER_TYPE,
                                  pFunc:
                                      Some(scrCBObjDestroyed as
                                               unsafe extern "C" fn()
                                                   -> BOOL),
                                  numParams: 2 as libc::c_int as UDWORD,
                                  aParams:
                                      [VAL_INT,
                                       (0x100000 as libc::c_int as
                                            libc::c_uint |
                                            ST_BASEOBJECT as libc::c_int as
                                                INTERP_TYPE as libc::c_uint)
                                           as INTERP_TYPE, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL],};
             init
         },
         {
             let mut init =
                 _callback_symbol{pIdent:
                                      b"CALL_STRUCT_DESTROYED\x00" as
                                          *const u8 as *const libc::c_char as
                                          *mut STRING,
                                  type_0:
                                      CALL_STRUCT_DESTROYED as libc::c_int as
                                          TRIGGER_TYPE,
                                  pFunc:
                                      Some(scrCBStructDestroyed as
                                               unsafe extern "C" fn()
                                                   -> BOOL),
                                  numParams: 2 as libc::c_int as UDWORD,
                                  aParams:
                                      [VAL_INT,
                                       (0x100000 as libc::c_int as
                                            libc::c_uint |
                                            ST_STRUCTURE as libc::c_int as
                                                INTERP_TYPE as libc::c_uint)
                                           as INTERP_TYPE, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL],};
             init
         },
         {
             let mut init =
                 _callback_symbol{pIdent:
                                      b"CALL_DROID_DESTROYED\x00" as *const u8
                                          as *const libc::c_char as
                                          *mut STRING,
                                  type_0:
                                      CALL_DROID_DESTROYED as libc::c_int as
                                          TRIGGER_TYPE,
                                  pFunc:
                                      Some(scrCBDroidDestroyed as
                                               unsafe extern "C" fn()
                                                   -> BOOL),
                                  numParams: 2 as libc::c_int as UDWORD,
                                  aParams:
                                      [VAL_INT,
                                       (0x100000 as libc::c_int as
                                            libc::c_uint |
                                            ST_DROID as libc::c_int as
                                                INTERP_TYPE as libc::c_uint)
                                           as INTERP_TYPE, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL],};
             init
         },
         {
             let mut init =
                 _callback_symbol{pIdent:
                                      b"CALL_FEATURE_DESTROYED\x00" as
                                          *const u8 as *const libc::c_char as
                                          *mut STRING,
                                  type_0:
                                      CALL_FEATURE_DESTROYED as libc::c_int as
                                          TRIGGER_TYPE,
                                  pFunc:
                                      Some(scrCBFeatureDestroyed as
                                               unsafe extern "C" fn()
                                                   -> BOOL),
                                  numParams: 1 as libc::c_int as UDWORD,
                                  aParams:
                                      [(0x100000 as libc::c_int as
                                            libc::c_uint |
                                            ST_FEATURE as libc::c_int as
                                                INTERP_TYPE as libc::c_uint)
                                           as INTERP_TYPE, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL],};
             init
         },
         {
             let mut init =
                 _callback_symbol{pIdent:
                                      b"CALL_OBJECTOPEN\x00" as *const u8 as
                                          *const libc::c_char as *mut STRING,
                                  type_0:
                                      CALL_OBJECTOPEN as libc::c_int as
                                          TRIGGER_TYPE,
                                  pFunc: None,
                                  numParams: 0 as libc::c_int as UDWORD,
                                  aParams: [VAL_BOOL; 20],};
             init
         },
         {
             let mut init =
                 _callback_symbol{pIdent:
                                      b"CALL_OBJECTCLOSE\x00" as *const u8 as
                                          *const libc::c_char as *mut STRING,
                                  type_0:
                                      CALL_OBJECTCLOSE as libc::c_int as
                                          TRIGGER_TYPE,
                                  pFunc: None,
                                  numParams: 0 as libc::c_int as UDWORD,
                                  aParams: [VAL_BOOL; 20],};
             init
         },
         {
             let mut init =
                 _callback_symbol{pIdent:
                                      b"CALL_TRANSPORTER_OFFMAP\x00" as
                                          *const u8 as *const libc::c_char as
                                          *mut STRING,
                                  type_0:
                                      CALL_TRANSPORTER_OFFMAP as libc::c_int
                                          as TRIGGER_TYPE,
                                  pFunc:
                                      Some(scrCBTransporterOffMap as
                                               unsafe extern "C" fn()
                                                   -> BOOL),
                                  numParams: 1 as libc::c_int as UDWORD,
                                  aParams:
                                      [VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL],};
             init
         },
         {
             let mut init =
                 _callback_symbol{pIdent:
                                      b"CALL_TRANSPORTER_LANDED\x00" as
                                          *const u8 as *const libc::c_char as
                                          *mut STRING,
                                  type_0:
                                      CALL_TRANSPORTER_LANDED as libc::c_int
                                          as TRIGGER_TYPE,
                                  pFunc:
                                      Some(scrCBTransporterLanded as
                                               unsafe extern "C" fn()
                                                   -> BOOL),
                                  numParams: 2 as libc::c_int as UDWORD,
                                  aParams:
                                      [ST_GROUP as libc::c_int as INTERP_TYPE,
                                       VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL],};
             init
         },
         {
             let mut init =
                 _callback_symbol{pIdent:
                                      b"CALL_ALL_ONSCREEN_DROIDS_SELECTED\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut STRING,
                                  type_0:
                                      CALL_ALL_ONSCREEN_DROIDS_SELECTED as
                                          libc::c_int as TRIGGER_TYPE,
                                  pFunc: None,
                                  numParams: 0 as libc::c_int as UDWORD,
                                  aParams: [VAL_BOOL; 20],};
             init
         },
         {
             let mut init =
                 _callback_symbol{pIdent:
                                      b"CALL_NO_REINFORCEMENTS_LEFT\x00" as
                                          *const u8 as *const libc::c_char as
                                          *mut STRING,
                                  type_0:
                                      CALL_NO_REINFORCEMENTS_LEFT as
                                          libc::c_int as TRIGGER_TYPE,
                                  pFunc: None,
                                  numParams: 0 as libc::c_int as UDWORD,
                                  aParams: [VAL_BOOL; 20],};
             init
         },
         {
             let mut init =
                 _callback_symbol{pIdent:
                                      b"CALL_CLUSTER_EMPTY\x00" as *const u8
                                          as *const libc::c_char as
                                          *mut STRING,
                                  type_0:
                                      CALL_CLUSTER_EMPTY as libc::c_int as
                                          TRIGGER_TYPE,
                                  pFunc:
                                      Some(scrCBClusterEmpty as
                                               unsafe extern "C" fn()
                                                   -> BOOL),
                                  numParams: 1 as libc::c_int as UDWORD,
                                  aParams:
                                      [(0x100000 as libc::c_int |
                                            VAL_INT as libc::c_int) as
                                           INTERP_TYPE, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL],};
             init
         },
         {
             let mut init =
                 _callback_symbol{pIdent:
                                      b"CALL_VTOL_OFF_MAP\x00" as *const u8 as
                                          *const libc::c_char as *mut STRING,
                                  type_0:
                                      CALL_VTOL_OFF_MAP as libc::c_int as
                                          TRIGGER_TYPE,
                                  pFunc:
                                      Some(scrCBVtolOffMap as
                                               unsafe extern "C" fn()
                                                   -> BOOL),
                                  numParams: 2 as libc::c_int as UDWORD,
                                  aParams:
                                      [VAL_INT,
                                       (0x100000 as libc::c_int as
                                            libc::c_uint |
                                            ST_DROID as libc::c_int as
                                                INTERP_TYPE as libc::c_uint)
                                           as INTERP_TYPE, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL],};
             init
         },
         {
             let mut init =
                 _callback_symbol{pIdent:
                                      b"CALL_UNITTAKEOVER\x00" as *const u8 as
                                          *const libc::c_char as *mut STRING,
                                  type_0:
                                      CALL_UNITTAKEOVER as libc::c_int as
                                          TRIGGER_TYPE,
                                  pFunc:
                                      Some(scrCBDroidTaken as
                                               unsafe extern "C" fn()
                                                   -> BOOL),
                                  numParams: 1 as libc::c_int as UDWORD,
                                  aParams:
                                      [(0x100000 as libc::c_int as
                                            libc::c_uint |
                                            ST_DROID as libc::c_int as
                                                INTERP_TYPE as libc::c_uint)
                                           as INTERP_TYPE, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL],};
             init
         },
         {
             let mut init =
                 _callback_symbol{pIdent:
                                      b"CALL_PLAYERLEFT\x00" as *const u8 as
                                          *const libc::c_char as *mut STRING,
                                  type_0:
                                      CALL_PLAYERLEFT as libc::c_int as
                                          TRIGGER_TYPE,
                                  pFunc:
                                      Some(scrCBPlayerLeft as
                                               unsafe extern "C" fn()
                                                   -> BOOL),
                                  numParams: 1 as libc::c_int as UDWORD,
                                  aParams:
                                      [VAL_INT, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL],};
             init
         },
         {
             let mut init =
                 _callback_symbol{pIdent:
                                      b"CALL_ALLIANCEOFFER\x00" as *const u8
                                          as *const libc::c_char as
                                          *mut STRING,
                                  type_0:
                                      CALL_ALLIANCEOFFER as libc::c_int as
                                          TRIGGER_TYPE,
                                  pFunc:
                                      Some(scrCBAllianceOffer as
                                               unsafe extern "C" fn()
                                                   -> BOOL),
                                  numParams: 2 as libc::c_int as UDWORD,
                                  aParams:
                                      [(0x100000 as libc::c_int |
                                            VAL_INT as libc::c_int) as
                                           INTERP_TYPE,
                                       (0x100000 as libc::c_int |
                                            VAL_INT as libc::c_int) as
                                           INTERP_TYPE, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL],};
             init
         },
         {
             let mut init =
                 _callback_symbol{pIdent:
                                      b"CALL_CONSOLE\x00" as *const u8 as
                                          *const libc::c_char as *mut STRING,
                                  type_0:
                                      CALL_CONSOLE as libc::c_int as
                                          TRIGGER_TYPE,
                                  pFunc:
                                      Some(scrCallConsole as
                                               unsafe extern "C" fn()
                                                   -> BOOL),
                                  numParams: 2 as libc::c_int as UDWORD,
                                  aParams:
                                      [(0x100000 as libc::c_int |
                                            VAL_INT as libc::c_int) as
                                           INTERP_TYPE,
                                       (0x100000 as libc::c_int |
                                            VAL_STRING as libc::c_int) as
                                           INTERP_TYPE, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL],};
             init
         },
         {
             let mut init =
                 _callback_symbol{pIdent:
                                      b"CALL_AI_MSG\x00" as *const u8 as
                                          *const libc::c_char as *mut STRING,
                                  type_0:
                                      CALL_AI_MSG as libc::c_int as
                                          TRIGGER_TYPE,
                                  pFunc:
                                      Some(scrCallMultiMsg as
                                               unsafe extern "C" fn()
                                                   -> BOOL),
                                  numParams: 3 as libc::c_int as UDWORD,
                                  aParams:
                                      [VAL_INT,
                                       (0x100000 as libc::c_int |
                                            VAL_INT as libc::c_int) as
                                           INTERP_TYPE,
                                       (0x100000 as libc::c_int |
                                            VAL_STRING as libc::c_int) as
                                           INTERP_TYPE, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL],};
             init
         },
         {
             let mut init =
                 _callback_symbol{pIdent:
                                      b"CALL_STRUCTBUILT\x00" as *const u8 as
                                          *const libc::c_char as *mut STRING,
                                  type_0:
                                      CALL_STRUCTBUILT as libc::c_int as
                                          TRIGGER_TYPE,
                                  pFunc:
                                      Some(scrCBStructBuilt as
                                               unsafe extern "C" fn()
                                                   -> BOOL),
                                  numParams: 3 as libc::c_int as UDWORD,
                                  aParams:
                                      [VAL_INT,
                                       (0x100000 as libc::c_int |
                                            ST_DROID as libc::c_int) as
                                           INTERP_TYPE,
                                       (0x100000 as libc::c_int |
                                            ST_STRUCTURE as libc::c_int) as
                                           INTERP_TYPE, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL],};
             init
         },
         {
             let mut init =
                 _callback_symbol{pIdent:
                                      b"CALL_TRANSPORTER_LANDED_B\x00" as
                                          *const u8 as *const libc::c_char as
                                          *mut STRING,
                                  type_0:
                                      CALL_TRANSPORTER_LANDED_B as libc::c_int
                                          as TRIGGER_TYPE,
                                  pFunc:
                                      Some(scrCBTransporterLandedB as
                                               unsafe extern "C" fn()
                                                   -> BOOL),
                                  numParams: 3 as libc::c_int as UDWORD,
                                  aParams:
                                      [ST_GROUP as libc::c_int as INTERP_TYPE,
                                       VAL_INT,
                                       (0x100000 as libc::c_int |
                                            ST_DROID as libc::c_int) as
                                           INTERP_TYPE, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                       VAL_BOOL, VAL_BOOL, VAL_BOOL],};
             init
         },
         {
             let mut init =
                 _callback_symbol{pIdent:
                                      b"CALLBACK LIST END\x00" as *const u8 as
                                          *const libc::c_char as *mut STRING,
                                  type_0: TR_INIT,
                                  pFunc: None,
                                  numParams: 0,
                                  aParams: [VAL_BOOL; 20],};
             init
         }]
    };
/* The table of type equivalence
 * The format is :
 *
 *       <base type>  <num equivalents>  <eqivalent types>
 *
 */
#[no_mangle]
pub static mut asEquivTable: [TYPE_EQUIV; 17] =
    [{
         let mut init =
             _interp_typeequiv{base:
                                   ST_BASEOBJECT as libc::c_int as
                                       INTERP_TYPE,
                               numEquiv: 3 as libc::c_int,
                               aEquivTypes:
                                   [ST_DROID as libc::c_int as INTERP_TYPE,
                                    ST_STRUCTURE as libc::c_int as
                                        INTERP_TYPE,
                                    ST_FEATURE as libc::c_int as INTERP_TYPE,
                                    VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                    VAL_BOOL, VAL_BOOL, VAL_BOOL],};
         init
     },
     {
         let mut init =
             _interp_typeequiv{base:
                                   ST_COMPONENT as libc::c_int as INTERP_TYPE,
                               numEquiv: 8 as libc::c_int,
                               aEquivTypes:
                                   [ST_BODY as libc::c_int as INTERP_TYPE,
                                    ST_PROPULSION as libc::c_int as
                                        INTERP_TYPE,
                                    ST_ECM as libc::c_int as INTERP_TYPE,
                                    ST_SENSOR as libc::c_int as INTERP_TYPE,
                                    ST_CONSTRUCT as libc::c_int as
                                        INTERP_TYPE,
                                    ST_WEAPON as libc::c_int as INTERP_TYPE,
                                    ST_REPAIR as libc::c_int as INTERP_TYPE,
                                    ST_BRAIN as libc::c_int as INTERP_TYPE,
                                    VAL_BOOL, VAL_BOOL],};
         init
     },
     {
         let mut init =
             _interp_typeequiv{base:
                                   ST_BASESTATS as libc::c_int as INTERP_TYPE,
                               numEquiv: 2 as libc::c_int,
                               aEquivTypes:
                                   [ST_STRUCTURESTAT as libc::c_int as
                                        INTERP_TYPE,
                                    ST_FEATURESTAT as libc::c_int as
                                        INTERP_TYPE, VAL_BOOL, VAL_BOOL,
                                    VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                    VAL_BOOL, VAL_BOOL],};
         init
     },
     {
         let mut init =
             _interp_typeequiv{base: ST_DROID as libc::c_int as INTERP_TYPE,
                               numEquiv: 1 as libc::c_int,
                               aEquivTypes:
                                   [ST_POINTER_O as libc::c_int as
                                        INTERP_TYPE, VAL_BOOL, VAL_BOOL,
                                    VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                    VAL_BOOL, VAL_BOOL, VAL_BOOL],};
         init
     },
     {
         let mut init =
             _interp_typeequiv{base:
                                   ST_STRUCTURE as libc::c_int as INTERP_TYPE,
                               numEquiv: 1 as libc::c_int,
                               aEquivTypes:
                                   [ST_POINTER_O as libc::c_int as
                                        INTERP_TYPE, VAL_BOOL, VAL_BOOL,
                                    VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                    VAL_BOOL, VAL_BOOL, VAL_BOOL],};
         init
     },
     {
         let mut init =
             _interp_typeequiv{base: ST_FEATURE as libc::c_int as INTERP_TYPE,
                               numEquiv: 1 as libc::c_int,
                               aEquivTypes:
                                   [ST_POINTER_O as libc::c_int as
                                        INTERP_TYPE, VAL_BOOL, VAL_BOOL,
                                    VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                    VAL_BOOL, VAL_BOOL, VAL_BOOL],};
         init
     },
     {
         let mut init =
             _interp_typeequiv{base:
                                   ST_BASEOBJECT as libc::c_int as
                                       INTERP_TYPE,
                               numEquiv: 1 as libc::c_int,
                               aEquivTypes:
                                   [ST_POINTER_O as libc::c_int as
                                        INTERP_TYPE, VAL_BOOL, VAL_BOOL,
                                    VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                    VAL_BOOL, VAL_BOOL, VAL_BOOL],};
         init
     },
     {
         let mut init =
             _interp_typeequiv{base:
                                   ST_TEMPLATE as libc::c_int as INTERP_TYPE,
                               numEquiv: 1 as libc::c_int,
                               aEquivTypes:
                                   [ST_POINTER_T as libc::c_int as
                                        INTERP_TYPE, VAL_BOOL, VAL_BOOL,
                                    VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                    VAL_BOOL, VAL_BOOL, VAL_BOOL],};
         init
     },
     {
         let mut init =
             _interp_typeequiv{base: ST_BODY as libc::c_int as INTERP_TYPE,
                               numEquiv: 1 as libc::c_int,
                               aEquivTypes:
                                   [ST_POINTER_S as libc::c_int as
                                        INTERP_TYPE, VAL_BOOL, VAL_BOOL,
                                    VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                    VAL_BOOL, VAL_BOOL, VAL_BOOL],};
         init
     },
     {
         let mut init =
             _interp_typeequiv{base:
                                   ST_PROPULSION as libc::c_int as
                                       INTERP_TYPE,
                               numEquiv: 1 as libc::c_int,
                               aEquivTypes:
                                   [ST_POINTER_S as libc::c_int as
                                        INTERP_TYPE, VAL_BOOL, VAL_BOOL,
                                    VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                    VAL_BOOL, VAL_BOOL, VAL_BOOL],};
         init
     },
     {
         let mut init =
             _interp_typeequiv{base: ST_WEAPON as libc::c_int as INTERP_TYPE,
                               numEquiv: 1 as libc::c_int,
                               aEquivTypes:
                                   [ST_POINTER_S as libc::c_int as
                                        INTERP_TYPE, VAL_BOOL, VAL_BOOL,
                                    VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                    VAL_BOOL, VAL_BOOL, VAL_BOOL],};
         init
     },
     {
         let mut init =
             _interp_typeequiv{base: ST_ECM as libc::c_int as INTERP_TYPE,
                               numEquiv: 1 as libc::c_int,
                               aEquivTypes:
                                   [ST_POINTER_S as libc::c_int as
                                        INTERP_TYPE, VAL_BOOL, VAL_BOOL,
                                    VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                    VAL_BOOL, VAL_BOOL, VAL_BOOL],};
         init
     },
     {
         let mut init =
             _interp_typeequiv{base: ST_SENSOR as libc::c_int as INTERP_TYPE,
                               numEquiv: 1 as libc::c_int,
                               aEquivTypes:
                                   [ST_POINTER_S as libc::c_int as
                                        INTERP_TYPE, VAL_BOOL, VAL_BOOL,
                                    VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                    VAL_BOOL, VAL_BOOL, VAL_BOOL],};
         init
     },
     {
         let mut init =
             _interp_typeequiv{base:
                                   ST_CONSTRUCT as libc::c_int as INTERP_TYPE,
                               numEquiv: 1 as libc::c_int,
                               aEquivTypes:
                                   [ST_POINTER_S as libc::c_int as
                                        INTERP_TYPE, VAL_BOOL, VAL_BOOL,
                                    VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                    VAL_BOOL, VAL_BOOL, VAL_BOOL],};
         init
     },
     {
         let mut init =
             _interp_typeequiv{base: ST_REPAIR as libc::c_int as INTERP_TYPE,
                               numEquiv: 1 as libc::c_int,
                               aEquivTypes:
                                   [ST_POINTER_S as libc::c_int as
                                        INTERP_TYPE, VAL_BOOL, VAL_BOOL,
                                    VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                    VAL_BOOL, VAL_BOOL, VAL_BOOL],};
         init
     },
     {
         let mut init =
             _interp_typeequiv{base: ST_BRAIN as libc::c_int as INTERP_TYPE,
                               numEquiv: 1 as libc::c_int,
                               aEquivTypes:
                                   [ST_POINTER_S as libc::c_int as
                                        INTERP_TYPE, VAL_BOOL, VAL_BOOL,
                                    VAL_BOOL, VAL_BOOL, VAL_BOOL, VAL_BOOL,
                                    VAL_BOOL, VAL_BOOL, VAL_BOOL],};
         init
     },
     {
         let mut init =
             _interp_typeequiv{base: VAL_BOOL,
                               numEquiv: 0 as libc::c_int,
                               aEquivTypes: [VAL_BOOL; 10],};
         init
     }];
// The table of user types for the compiler
// The table of script callable functions
// The table of external variables
// The table of object variables
// The table of constant values
// Initialise the script system
// Initialise the script system
#[no_mangle]
pub unsafe extern "C" fn scrTabInitialise() -> BOOL {
    let mut sInit: EVENT_INIT =
        EVENT_INIT{valInit: 0,
                   valExt: 0,
                   trigInit: 0,
                   trigExt: 0,
                   contInit: 0,
                   contExt: 0,}; // was 20 ... not enough
    sInit.valInit = 50 as libc::c_int as UWORD;
    sInit.valExt = 5 as libc::c_int as UWORD;
    sInit.trigInit = 35 as libc::c_int as UWORD;
    sInit.trigExt = 5 as libc::c_int as UWORD;
    sInit.contInit = 50 as libc::c_int as UWORD;
    sInit.contExt = 5 as libc::c_int as UWORD;
    if scriptInitialise(&mut sInit) == 0 { return 0 as libc::c_int }
    if eventInitValueFuncs(ST_MAXTYPE as libc::c_int as INTERP_TYPE as SDWORD)
           == 0 {
        return 0 as libc::c_int
    }
    scrvInitialise();
    // Set the constant table
    scriptSetConstTab(asConstantTable.as_mut_ptr());
    // Set the function table
    scriptSetFuncTab(asFuncTable.as_mut_ptr());
    // Set the type table
    scriptSetTypeTab(asTypeTable.as_mut_ptr());
    // Set the external variable table
    scriptSetExternalTab(asExternTable.as_mut_ptr());
    // Set the object variable table
    scriptSetObjectTab(asObjTable.as_mut_ptr());
    // Set the callback table
    scriptSetCallbackTab(asCallbackTable.as_mut_ptr());
    // Set the type equivalence table
    scriptSetTypeEquiv(asEquivTable.as_mut_ptr());
    // Set the create and release functions
    if eventAddValueCreate(ST_BASEOBJECT as libc::c_int as INTERP_TYPE,
                           Some(scrvAddBasePointer as
                                    unsafe extern "C" fn(_: *mut INTERP_VAL)
                                        -> BOOL)) == 0 {
        return 0 as libc::c_int
    }
    if eventAddValueRelease(ST_BASEOBJECT as libc::c_int as INTERP_TYPE,
                            Some(scrvReleaseBasePointer as
                                     unsafe extern "C" fn(_: *mut INTERP_VAL)
                                         -> ())) == 0 {
        return 0 as libc::c_int
    }
    if eventAddValueCreate(ST_DROID as libc::c_int as INTERP_TYPE,
                           Some(scrvAddBasePointer as
                                    unsafe extern "C" fn(_: *mut INTERP_VAL)
                                        -> BOOL)) == 0 {
        return 0 as libc::c_int
    }
    if eventAddValueRelease(ST_DROID as libc::c_int as INTERP_TYPE,
                            Some(scrvReleaseBasePointer as
                                     unsafe extern "C" fn(_: *mut INTERP_VAL)
                                         -> ())) == 0 {
        return 0 as libc::c_int
    }
    if eventAddValueCreate(ST_STRUCTURE as libc::c_int as INTERP_TYPE,
                           Some(scrvAddBasePointer as
                                    unsafe extern "C" fn(_: *mut INTERP_VAL)
                                        -> BOOL)) == 0 {
        return 0 as libc::c_int
    }
    if eventAddValueRelease(ST_STRUCTURE as libc::c_int as INTERP_TYPE,
                            Some(scrvReleaseBasePointer as
                                     unsafe extern "C" fn(_: *mut INTERP_VAL)
                                         -> ())) == 0 {
        return 0 as libc::c_int
    }
    if eventAddValueCreate(ST_FEATURE as libc::c_int as INTERP_TYPE,
                           Some(scrvAddBasePointer as
                                    unsafe extern "C" fn(_: *mut INTERP_VAL)
                                        -> BOOL)) == 0 {
        return 0 as libc::c_int
    }
    if eventAddValueRelease(ST_FEATURE as libc::c_int as INTERP_TYPE,
                            Some(scrvReleaseBasePointer as
                                     unsafe extern "C" fn(_: *mut INTERP_VAL)
                                         -> ())) == 0 {
        return 0 as libc::c_int
    }
    if eventAddValueCreate(ST_GROUP as libc::c_int as INTERP_TYPE,
                           Some(scrvNewGroup as
                                    unsafe extern "C" fn(_: *mut INTERP_VAL)
                                        -> BOOL)) == 0 {
        return 0 as libc::c_int
    }
    if eventAddValueRelease(ST_GROUP as libc::c_int as INTERP_TYPE,
                            Some(scrvReleaseGroup as
                                     unsafe extern "C" fn(_: *mut INTERP_VAL)
                                         -> ())) == 0 {
        return 0 as libc::c_int
    }
    // initialise various variables
    scrGameLevel = 0 as libc::c_int;
    bInTutorial = 0 as libc::c_int;
    return 1 as libc::c_int;
}
// Shut down the script system
// Shut down the script system
#[no_mangle]
pub unsafe extern "C" fn scrShutDown() { scrvShutDown(); scriptShutDown(); }
