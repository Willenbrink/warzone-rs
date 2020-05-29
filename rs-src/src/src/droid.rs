use ::libc;
extern "C" {
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    /* Allocate an object from a heap
 * Returns a pointer to the object if successful
 */
    #[no_mangle]
    fn heapAlloc(psHeap: *mut OBJ_HEAP, ppObject: *mut *mut libc::c_void)
     -> BOOL;
    /* Return an object to the heap */
    #[no_mangle]
    fn heapFree(psHeap: *mut OBJ_HEAP, pObject: *mut libc::c_void) -> BOOL;
    #[no_mangle]
    fn abort() -> !;
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
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_uint)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_uint)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_uint)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_uint)
     -> *mut libc::c_void;
    /* Return the ID number for an ID string */
    #[no_mangle]
    fn strresGetIDNum(psRes: *mut STR_RES, pIDStr: *mut STRING,
                      pIDNum: *mut UDWORD) -> BOOL;
    /* Get the string from an ID number */
    #[no_mangle]
    fn strresGetString(psRes: *mut STR_RES, id: UDWORD) -> *mut STRING;
    #[no_mangle]
    fn frameGetFrameNumber() -> UDWORD;
    #[no_mangle]
    fn animObj_SetDoneFunc(psObj: *mut ANIM_OBJECT,
                           pDoneFunc: ANIMOBJDONEFUNC);
    /* uwCycles=0 for infinite looping */
    #[no_mangle]
    fn animObj_Add(pParentObj: *mut libc::c_void, iAnimID: libc::c_int,
                   udwStartDelay: UDWORD, uwCycles: UWORD)
     -> *mut ANIM_OBJECT;
    #[no_mangle]
    fn animObj_Remove(ppsObj: *mut *mut ANIM_OBJECT, iAnimID: libc::c_int)
     -> BOOL;
    #[no_mangle]
    fn audio_PlayStaticTrack(iX: SDWORD, iY: SDWORD, iTrack: libc::c_int)
     -> BOOL;
    #[no_mangle]
    fn audio_PlayObjStaticTrackCallback(psObj: *mut libc::c_void,
                                        iTrack: libc::c_int,
                                        pUserCallback: AUDIO_CALLBACK)
     -> BOOL;
    #[no_mangle]
    fn audio_PlayObjDynamicTrack(psObj: *mut libc::c_void,
                                 iTrack: libc::c_int,
                                 pUserCallback: AUDIO_CALLBACK) -> BOOL;
    #[no_mangle]
    fn audio_StopObjTrack(psObj: *mut libc::c_void, iTrack: libc::c_int);
    #[no_mangle]
    fn audio_QueueTrackMinDelayPos(iTrack: SDWORD, iMinDelay: UDWORD,
                                   iX: SDWORD, iY: SDWORD, iZ: SDWORD);
    #[no_mangle]
    fn audio_QueueTrackPos(iTrack: SDWORD, iX: SDWORD, iY: SDWORD,
                           iZ: SDWORD);
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
    /* The list of destroyed objects */
    /* Initialise the object heaps */
    /* Release the object heaps */
    /* General housekeeping for the object system */
    /* Create a new droid */
    /* add the droid to the Droid Lists */
    /*destroy a droid */
    #[no_mangle]
    fn killDroid(psDel: *mut DROID);
    /*
 * Structure.h
 *
 * Definitions for the structures.
 *
 */
    // how long to wait between CALL_STRUCT_ATTACKED's - plus how long to flash on radar for
    // This should really be logarithmic
    /* explosion data for when a structure is blown up - used by features as well*/
    //production loop max
    //10
    /*This should correspond to the structLimits! */
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
    #[no_mangle]
    fn assignFactoryCommandDroid(psStruct: *mut STRUCTURE,
                                 psCommander: *mut _droid);
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
    #[no_mangle]
    fn StructIsFactory(Struct: *mut STRUCTURE) -> BOOL;
    #[no_mangle]
    static mut apsStructLists: [*mut STRUCTURE; 8];
    #[no_mangle]
    static mut apsDroidLists: [*mut DROID; 8];
    #[no_mangle]
    fn addDroid(psDroidToAdd: *mut DROID, pList: *mut *mut DROID);
    #[no_mangle]
    static mut psTemplateHeap: *mut OBJ_HEAP;
    #[no_mangle]
    static mut asBodyStats: *mut BODY_STATS;
    #[no_mangle]
    static mut numSensorStats: UDWORD;
    #[no_mangle]
    fn getResourceName(pName: *mut STRING) -> BOOL;
    #[no_mangle]
    static mut asSensorStats: *mut SENSOR_STATS;
    #[no_mangle]
    static mut numRepairStats: UDWORD;
    #[no_mangle]
    static mut asRepairStats: *mut REPAIR_STATS;
    #[no_mangle]
    static mut numPropulsionStats: UDWORD;
    #[no_mangle]
    static mut asPropulsionStats: *mut PROPULSION_STATS;
    #[no_mangle]
    static mut numECMStats: UDWORD;
    #[no_mangle]
    static mut asECMStats: *mut ECM_STATS;
    #[no_mangle]
    static mut numConstructStats: UDWORD;
    #[no_mangle]
    static mut asConstructStats: *mut CONSTRUCT_STATS;
    #[no_mangle]
    static mut numBrainStats: UDWORD;
    #[no_mangle]
    static mut asBrainStats: *mut BRAIN_STATS;
    #[no_mangle]
    static mut numBodyStats: UDWORD;
    #[no_mangle]
    fn numCR(pFileBuffer: *mut libc::c_char, fileSize: UDWORD) -> UDWORD;
    #[no_mangle]
    static mut asWeaponStats: *mut WEAPON_STATS;
    #[no_mangle]
    static mut asPropulsionTypes: *mut PROPULSION_TYPES;
    #[no_mangle]
    fn getCompFromName(compType: UDWORD, pName: *mut STRING) -> SDWORD;
    #[no_mangle]
    static mut numWeaponStats: UDWORD;
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
    fn bodyArmour(psStats: *mut BODY_STATS, player: UBYTE, bodyType: UBYTE,
                  weaponClass: WEAPON_CLASS) -> UDWORD;
    #[no_mangle]
    static mut asBodyUpgrade: [[BODY_UPGRADE; 2]; 8];
    #[no_mangle]
    fn ecmPower(psStats: *mut ECM_STATS, player: UBYTE) -> UDWORD;
    #[no_mangle]
    fn sensorPower(psStats: *mut SENSOR_STATS, player: UBYTE) -> UDWORD;
    #[no_mangle]
    fn sensorRange(psStats: *mut SENSOR_STATS, player: UBYTE) -> UDWORD;
    #[no_mangle]
    fn bodyPower(psStats: *mut BODY_STATS, player: UBYTE, bodyType: UBYTE)
     -> UDWORD;
    #[no_mangle]
    static mut psDroidHeap: *mut OBJ_HEAP;
    #[no_mangle]
    fn createDroid(player: UDWORD, ppsNew: *mut *mut DROID) -> BOOL;
    #[no_mangle]
    fn getSpeedFactor(terrainType: UDWORD, propulsionType: UDWORD) -> UDWORD;
    #[no_mangle]
    static mut asStructureUpgrade: [STRUCTURE_UPGRADE; 8];
    #[no_mangle]
    fn moveUpdateDroid(psDroid: *mut DROID);
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
    /* Check no alliance has formed*/
    /* Initialise the AI system */
    /* Shutdown the AI system */
    /* Initialise a droid structure for AI */
    /* Do the AI for a droid */
    #[no_mangle]
    fn aiUpdateDroid(psDroid: *mut DROID);
    #[no_mangle]
    static mut asStructureStats: *mut STRUCTURE_STATS;
    #[no_mangle]
    static mut asStructLimits: [*mut STRUCTURE_LIMITS; 8];
    #[no_mangle]
    fn buildStructure(pStructureType: *mut STRUCTURE_STATS, x: UDWORD,
                      y: UDWORD, player: UDWORD, FromSave: BOOL)
     -> *mut STRUCTURE;
    #[no_mangle]
    fn releasePowerGen(psRelease: *mut STRUCTURE);
    #[no_mangle]
    fn removeStruct(psDel: *mut STRUCTURE, bDestroy: BOOL) -> BOOL;
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
    #[no_mangle]
    fn structPowerToBuild(psStruct: *mut STRUCTURE) -> UDWORD;
    #[no_mangle]
    fn constructorPoints(psStats: *mut CONSTRUCT_STATS, player: UBYTE)
     -> UDWORD;
    #[no_mangle]
    fn structureBody(psStruct: *mut STRUCTURE) -> UDWORD;
    #[no_mangle]
    fn repairPoints(psStats: *mut REPAIR_STATS, player: UBYTE) -> UDWORD;
    #[no_mangle]
    fn structureCompletedCallback(psStructType: *mut STRUCTURE_STATS);
    #[no_mangle]
    fn buildingComplete(psBuilding: *mut STRUCTURE);
    #[no_mangle]
    fn structureResistance(psStats: *mut STRUCTURE_STATS, player: UBYTE)
     -> UDWORD;
    #[no_mangle]
    fn weaponDamage(psStats: *mut WEAPON_STATS, player: UBYTE) -> UDWORD;
    /* Remove all droids */
    /*Remove a single Droid from its list*/
    #[no_mangle]
    fn removeDroid(psDroidToRemove: *mut DROID, pList: *mut *mut DROID);
    #[no_mangle]
    static mut apCompLists: [[*mut UBYTE; 9]; 8];
    #[no_mangle]
    fn validTemplateForFactory(psTemplate: *mut DROID_TEMPLATE,
                               psFactory: *mut STRUCTURE) -> BOOL;
    #[no_mangle]
    fn checkProductionForCommand(player: UBYTE) -> UBYTE;
    #[no_mangle]
    fn getProductionQuantity(psStructure: *mut STRUCTURE,
                             psTemplate: *mut DROID_TEMPLATE) -> UDWORD;
    #[no_mangle]
    static mut apsFlagPosLists: [*mut FLAG_POSITION; 8];
    #[no_mangle]
    static mut apStructTypeLists: [*mut UBYTE; 8];
    #[no_mangle]
    static mut numStructureStats: UDWORD;
    #[no_mangle]
    static mut researchModuleStat: UDWORD;
    #[no_mangle]
    static mut factoryModuleStat: UDWORD;
    #[no_mangle]
    static mut powerModuleStat: UDWORD;
    #[no_mangle]
    fn removeFeature(psDel: *mut FEATURE);
    #[no_mangle]
    fn vtolOnRearmPad(psStruct: *mut STRUCTURE, psDroid: *mut DROID) -> BOOL;
    #[no_mangle]
    static mut asProductionRun: [[[PRODUCTION_RUN; 20]; 5]; 3];
    #[no_mangle]
    fn structSetManufacture(psStruct: *mut STRUCTURE,
                            psTempl: *mut DROID_TEMPLATE, quantity: UBYTE)
     -> BOOL;
    #[no_mangle]
    fn factoryProdUpdate(psStructure: *mut STRUCTURE,
                         psTemplate: *mut DROID_TEMPLATE)
     -> *mut DROID_TEMPLATE;
    #[no_mangle]
    fn factoryProdAdjust(psStructure: *mut STRUCTURE,
                         psTemplate: *mut DROID_TEMPLATE, add: BOOL);
    #[no_mangle]
    static mut productionPlayer: SBYTE;
    /* Check which tiles can be seen by an object */
    #[no_mangle]
    fn visTilesUpdate(psObj: *mut BASE_OBJECT, SpreadLoad: BOOL);
    #[no_mangle]
    fn processVisibility(psCurr: *mut BASE_OBJECT);
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
    #[no_mangle]
    static mut DirectControl: BOOL;
    #[no_mangle]
    fn driveDroidKilled(psDroid: *mut DROID) -> BOOL;
    /* Which is the currently selected player */
    #[no_mangle]
    static mut selectedPlayer: UDWORD;
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
    fn intBuildStarted(psDroid: *mut DROID);
    #[no_mangle]
    fn intManufactureFinished(psBuilding: *mut STRUCTURE);
    #[no_mangle]
    fn intBuildFinished(psDroid: *mut DROID);
    // The maximum time for one frame (stops the clock running away when debugging)
// changed to /6 by ajl. if this needs to go back to ticks/10 then tell me. 
    /* The current time in the game world */
    #[no_mangle]
    static mut gameTime: UDWORD;
    #[no_mangle]
    fn RemapPlayerNumber(OldNumber: UDWORD) -> UDWORD;
    /*check the available power*/
    #[no_mangle]
    fn checkPower(player: UDWORD, quantity: UDWORD, playAudio: BOOL) -> BOOL;
    //return the power when a structure/droid is deliberately destroyed
    #[no_mangle]
    fn addPower(player: UDWORD, quantity: UDWORD);
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
    /*defines which droid types draw power - returns TRUE if use power*/
    #[no_mangle]
    fn droidUsesPower(psDroid: *mut DROID) -> BOOL;
    /* Maximum number of effects in the world - need to investigate what this should be */
/* EXTERNAL REFERENCES */
    #[no_mangle]
    fn effectGiveAuxVar(var: UDWORD);
    #[no_mangle]
    fn addEffect(pos: *mut iVector, group: EFFECT_GROUP, type_0: EFFECT_TYPE,
                 specified: BOOL, imd: *mut iIMDShape, lit: BOOL);
    // after failing a route ... this is the amount of time that the droid goes all defensive untill it can start going aggressive
    // 1.5 sec
    // 4 secs
    //this is how long a droid is disabled for when its been attacked by an EMP weapon
    // 10 secs
    /* Update the action state for a droid */
    #[no_mangle]
    fn actionUpdateDroid(psDroid: *mut DROID);
    /* Update a droids order state */
    #[no_mangle]
    fn orderUpdateDroid(psDroid: *mut DROID);
    /* Give a droid an order */
    #[no_mangle]
    fn orderDroid(psDroid: *mut DROID, order: DROID_ORDER);
    /* Get the state of a droid order with an object */
    #[no_mangle]
    fn orderStateObj(psDroid: *mut DROID, order: DROID_ORDER,
                     ppsObj: *mut *mut BASE_OBJECT) -> BOOL;
    // get the state of a secondary order, return FALSE if unsupported
    #[no_mangle]
    fn secondaryGetState(psDroid: *mut DROID, sec: SECONDARY_ORDER,
                         pState: *mut SECONDARY_STATE) -> BOOL;
    // set the state of a secondary order, return FALSE if failed.
    #[no_mangle]
    fn secondarySetState(psDroid: *mut DROID, sec: SECONDARY_ORDER,
                         State: SECONDARY_STATE) -> BOOL;
    // check the damage level of a droid against it's secondary state
    #[no_mangle]
    fn secondaryCheckDamageLevel(psDroid: *mut DROID);
    // make all the members of a numeric group have the same secondary states
    #[no_mangle]
    fn secondarySetAverageGroupState(player: UDWORD, group: UDWORD);
    // do a moral check for a player
    #[no_mangle]
    fn orderMoralCheck(player: UDWORD);
    // do a moral check for a group
    #[no_mangle]
    fn orderGroupMoralCheck(psGroup: *mut _droid_group);
    // do a health check for a droid
    #[no_mangle]
    fn orderHealthCheck(psDroid: *mut DROID);
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
    #[no_mangle]
    fn getTileStructure(x: UDWORD, y: UDWORD) -> *mut STRUCTURE;
    // Clear all selections and stop driver mode.
    #[no_mangle]
    fn clearSelection();
    #[no_mangle]
    static mut ConsoleString: [libc::c_char; 255];
    #[no_mangle]
    fn addConsoleMessage(messageText: *mut STRING,
                         jusType: CONSOLE_TEXT_JUSTIFICATION) -> BOOL;
    #[no_mangle]
    fn compPersonToBits(psDroid: *mut DROID);
    #[no_mangle]
    fn destroyFXDroid(psDroid: *mut DROID);
    #[no_mangle]
    fn calcDroidIllumination(psDroid: *mut DROID);
    // Look up the zone for a coordinate
    #[no_mangle]
    fn gwGetZone(x: SDWORD, y: SDWORD) -> SDWORD;
    // see if a zone is reachable
    #[no_mangle]
    fn gwZoneReachable(zone: SDWORD) -> BOOL;
    // the game description.
    #[no_mangle]
    static mut bMultiPlayer: BOOL;
    #[no_mangle]
    fn isHumanPlayer(player: UDWORD) -> BOOL;
    //to tell if the player is a computer or not.
    #[no_mangle]
    fn myResponsibility(player: UDWORD) -> BOOL;
    // ///////////////////////////////////////////////////////
// definitions of functions in multiplay's other c files.
    // Buildings . multistruct
    #[no_mangle]
    fn sendBuildStarted(psStruct: *mut STRUCTURE, psDroid: *mut DROID)
     -> BOOL;
    #[no_mangle]
    fn SendBuildFinished(psStruct: *mut STRUCTURE) -> BOOL;
    // droids . multibot
    #[no_mangle]
    fn SendDroid(pTemplate: *mut DROID_TEMPLATE, x: UDWORD, y: UDWORD,
                 player: UBYTE, id: UDWORD) -> BOOL;
    #[no_mangle]
    fn SendDestroyDroid(d: *mut DROID) -> BOOL;
    #[no_mangle]
    fn SendDemolishFinished(psS: *mut STRUCTURE, psD: *mut DROID) -> BOOL;
    // initialise the group system
    // shutdown the group system
    // create a new group
    #[no_mangle]
    fn grpCreate(ppsGroup: *mut *mut DROID_GROUP) -> BOOL;
    #[no_mangle]
    fn grpJoin(psGroup: *mut DROID_GROUP, psDroid: *mut DROID);
    #[no_mangle]
    fn grpLeave(psGroup: *mut DROID_GROUP, psDroid: *mut DROID);
    // Remove a unit from a formation
    #[no_mangle]
    fn formationLeave(psFormation: *mut FORMATION, psObj: *mut BASE_OBJECT);
    #[no_mangle]
    fn getWarCamStatus() -> BOOL;
    #[no_mangle]
    fn camToggleStatus();
    #[no_mangle]
    fn processWarCam() -> BOOL;
    #[no_mangle]
    static mut mapX: UDWORD;
    #[no_mangle]
    static mut mapY: UDWORD;
    #[no_mangle]
    fn setViewPos(x: UDWORD, y: UDWORD, Pan: BOOL);
    /* The string resource object */
    #[no_mangle]
    static mut psStringRes: *mut STR_RES;
    // Activate a callback trigger
    #[no_mangle]
    fn eventFireCallbackTrigger(callback: TRIGGER_TYPE);
    //for scrCBStructBuilt callback
    // The pointer to the droid that was just built for a CALL_NEWDROID
    #[no_mangle]
    static mut psScrCBDroidTaken: *mut DROID;
    // get the experience level of a command droid
    #[no_mangle]
    fn cmdDroidGetLevel(psCommander: *mut DROID) -> SDWORD;
    // get the level of a droids commander, if any
    #[no_mangle]
    fn cmdGetCommanderLevel(psDroid: *mut DROID) -> SDWORD;
    #[no_mangle]
    static mut fpathBlockingTile:
           Option<unsafe extern "C" fn(_: SDWORD, _: SDWORD) -> BOOL>;
    // add an object to the grid system
    #[no_mangle]
    fn gridAddObject(psObj: *mut BASE_OBJECT);
    // remove an object from the grid system
    #[no_mangle]
    fn gridRemoveObject(psObj: *mut BASE_OBJECT);
    // initialise the grid system to start iterating through units that
// could affect a location (x,y in world coords)
    #[no_mangle]
    fn gridStartIterate(x: SDWORD, y: SDWORD);
    // get the next object that could affect a location,
// should only be called after gridStartIterate
    #[no_mangle]
    fn gridIterate() -> *mut BASE_OBJECT;
    // return whether a weapon is direct or indirect
    #[no_mangle]
    fn proj_Direct(psStats: *mut WEAPON_STATS) -> BOOL;
    /*
// The fattest macro around - change this little bastard at your peril
#define GFX_VISIBLE(psObj)	(							\
	(													\
		(psObj->player == selectedPlayer)				\
	)													\
	OR													\
	(													\
		(psObj->psSource != NULL)						\
		AND												\
		(												\
			(psObj->psSource->type == OBJ_STRUCTURE && psObj->psSource->player == selectedPlayer) OR\
			(psObj->psSource->type == OBJ_STRUCTURE && psObj->psSource->visible[selectedPlayer]) OR\
			(psObj->psSource->type == OBJ_DROID     && psObj->psSource->visible[selectedPlayer]) OR\
			(psObj->psSource->type == OBJ_DROID     && psObj->psSource->player == selectedPlayer) \
		)												\
	)													\
	OR													\
	(													\
		(psObj->psDest != NULL)							\
		AND												\
		(												\
			(psObj->psDest->type == OBJ_STRUCTURE && psObj->psDest->player == selectedPlayer) OR\
			(psObj->psDest->type == OBJ_STRUCTURE && psObj->psSource->visible[selectedPlayer]) OR\
			(psObj->psDest->type == OBJ_DROID     && psObj->psDest->visible[selectedPlayer]) OR\
			(psObj->psDest->type == OBJ_DROID     && psObj->psSource->player == selectedPlayer) \
		)												\
	)													\
	OR													\
	(													\
		godMode											\
	)													\
)
*/
    #[no_mangle]
    fn calcDamage(baseDamage: UDWORD, weaponEffect: WEAPON_EFFECT,
                  psTarget: *mut BASE_OBJECT) -> UDWORD;
    // tell the cluster system about a new droid
    #[no_mangle]
    fn clustNewDroid(psDroid: *mut DROID);
    // update the cluster information for an object
    #[no_mangle]
    fn clustUpdateObject(psObj: *mut BASE_OBJECT);
    // remove an object from the cluster system
    #[no_mangle]
    fn clustRemoveObject(psObj: *mut BASE_OBJECT);
    // tell the cluster system that an object has been attacked
    #[no_mangle]
    fn clustObjectAttacked(psObj: *mut BASE_OBJECT);
    #[no_mangle]
    static mut mission: MISSION;
    #[no_mangle]
    fn missionCanReEnforce() -> BOOL;
    /*This is used to display the transporter button and capacity when at the home base ONLY*/
    #[no_mangle]
    fn intAddTransporterLaunch(psDroid: *mut DROID) -> BOOL;
    #[no_mangle]
    fn selNumSelected(player: UDWORD) -> UDWORD;
    #[no_mangle]
    fn modifyForDifficultyLevel(basicVal: SDWORD, IsPlayer: BOOL) -> SDWORD;
    #[no_mangle]
    fn kill3DBuilding();
    /*returns true if the build state is not equal to BUILD3D_NONE*/
    #[no_mangle]
    fn tryingToGetLocation() -> BOOL;
    #[no_mangle]
    fn scoreUpdateVar(var: DATA_INDEX);
    /* Default level of sensor, repair and ECM */
    #[no_mangle]
    static mut aDefaultSensor: [UDWORD; 8];
    #[no_mangle]
    static mut aDefaultECM: [UDWORD; 8];
    #[no_mangle]
    static mut aDefaultRepair: [UDWORD; 8];
    /* Output text to the display screen at location x,y. The remaining arguments are as printf. */
    #[no_mangle]
    fn screenTextOut(x: UDWORD, y: UDWORD, pFormat: *mut STRING, _: ...);
    #[no_mangle]
    fn ThreatInRange(player: SDWORD, range: SDWORD, rangeX: SDWORD,
                     rangeY: SDWORD, bVTOLs: BOOL) -> BOOL;
    /* default droid design template */
    #[no_mangle]
    static mut sDefaultDesignTemplate: DROID_TEMPLATE;
}
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
/*
if any types are added BEFORE 'COMP_BODY' - then Save/Load Game will have to be 
altered since it loops through the components from 1->MAX_COMP
*/
pub type _component_type = libc::c_uint;
pub const COMP_NUMCOMPONENTS: _component_type = 9;
//COMP_PROGRAM,		//this needs to be removed when save games changes
pub const COMP_WEAPON: _component_type = 8;
pub const COMP_CONSTRUCT: _component_type = 7;
pub const COMP_SENSOR: _component_type = 6;
pub const COMP_ECM: _component_type = 5;
pub const COMP_REPAIRUNIT: _component_type = 4;
//COMP_POWERPLANT,
pub const COMP_PROPULSION: _component_type = 3;
pub const COMP_BRAIN: _component_type = 2;
//COMP_ARMOUR,
pub const COMP_BODY: _component_type = 1;
pub const COMP_UNKNOWN: _component_type = 0;
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
pub struct _brain_stats {
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
    pub progCap: UDWORD,
    pub psWeaponStat: *mut _weapon_stats,
}
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
pub type BRAIN_STATS = _brain_stats;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _construct_stats {
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
    pub constructPoints: UDWORD,
    pub pMountGraphic: *mut iIMDShape,
}
pub type CONSTRUCT_STATS = _construct_stats;
//no capability - droid must stop
//partial capability - droid has 50% chance to hit
//full capability - droid fires normally on move
// The turret mount to use
/* ***********************************************************************************
*	Additional stats tables
************************************************************************************/
pub type _travel_medium = libc::c_uint;
pub const AIR: _travel_medium = 1;
pub const GROUND: _travel_medium = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _propulsion_types {
    pub powerRatioMult: UWORD,
    pub travel: UDWORD,
    pub startID: SWORD,
    pub idleID: SWORD,
    pub moveOffID: SWORD,
    pub moveID: SWORD,
    pub hissID: SWORD,
    pub shutDownID: SWORD,
}
pub type PROPULSION_TYPES = _propulsion_types;
//sound to play when this prop type shuts down
/*body stats which can be upgraded by research*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _body_upgrade {
    pub powerOutput: UWORD,
    pub body: UWORD,
    pub armourValue: [UWORD; 2],
}
pub type BODY_UPGRADE = _body_upgrade;
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
// which line this member is on
// the next member on this line
// distance along the line
// the member unit
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
pub type BASE_OBJECT = _base_object;
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
pub type MOVE_CONTROL = _move_control;
// position relative to center
// orientation of line
// first member in the 'linked list' of members
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
//the three different types of factory (currently) - FACTORY, CYBORG_FACTORY, VTOL_FACTORY
// added repair facilities as they need an assebly point as well
//seperate the numfactory from numflag
//this is used for module graphics - factory and vtol factory
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _production_run {
    pub quantity: UBYTE,
    pub built: UBYTE,
    pub psTemplate: *mut _droid_template,
}
pub type PRODUCTION_RUN = _production_run;
//template to build
/* structure stats which can be upgraded by research*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _structure_upgrade {
    pub armour: UWORD,
    pub body: UWORD,
    pub resistance: UWORD,
}
pub type STRUCTURE_UPGRADE = _structure_upgrade;
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
pub struct _naybor_info {
    pub psObj: *mut BASE_OBJECT,
    pub distSqr: UDWORD,
}
pub type NAYBOR_INFO = _naybor_info;
pub type PICKTILE = libc::c_uint;
pub const HALF_FREE_TILE: PICKTILE = 2;
pub const FREE_TILE: PICKTILE = 1;
pub const NO_FREE_TILE: PICKTILE = 0;
// Feature armour
/*
 * Group.h
 *
 * Link droids together into a group for AI etc.
 *
 */
// standard group
// command droid group
// transporter group
pub type DROID_GROUP = _droid_group;
// list of droids in the group
// the command droid of a command group
// where the group should retreat to
// information about a formation
pub type FORMATION = _formation;
pub const DORDER_RECYCLE: _droid_order = 21;
// number of units using the formation
// maximum length of the lines
// seperation between the ranks
// direction of the formation
// position of the front of the formation
// the lines that make up a formation
// the units that have a position allocated in the formation
// formation speed (currently speed of slowest member) - GJ - sorry.
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
pub const NO_SOUND: C2RustUnnamed_0 = -1;
pub const DACTION_NONE: _droid_action = 0;
pub const DORDER_NONE: _droid_order = 0;
pub const ID_SOUND_EXPLOSION: C2RustUnnamed_0 = 271;
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
pub const ID_SOUND_BARB_SQUISH: C2RustUnnamed_0 = 276;
pub const DORDER_RUNBURN: _droid_order = 29;
// assign production to a command droid - state is the factory number
// remove production from a command droid
// patrol between current pos and next move target
// what to do when stopped
// return to various locations
// command droid controlling IDF structures
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
pub const ID_SOUND_UNIT_DESTROYED: C2RustUnnamed_0 = 77;
pub type CONSOLE_TEXT_JUSTIFICATION = libc::c_uint;
pub const DEFAULT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 3;
pub const CENTRE_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 2;
pub const RIGHT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 1;
pub const LEFT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 0;
pub const STR_GAM_UNITLOST: _fixed_str_id = 222;
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
pub const DORDER_SCOUT: _droid_order = 28;
pub const DORDER_RESTORE: _droid_order = 27;
pub const DORDER_DROIDREPAIR: _droid_order = 26;
pub const DORDER_GUARD: _droid_order = 25;
pub const DORDER_TRANSPORTRETURN: _droid_order = 24;
pub const DORDER_TRANSPORTIN: _droid_order = 23;
pub const DORDER_TRANSPORTOUT: _droid_order = 22;
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
pub const ID_SOUND_BARB_SCREAM: C2RustUnnamed_0 = 301;
pub const DACTION_BUILD: _droid_action = 2;
pub const ID_SOUND_CONSTRUCTION_LOOP: C2RustUnnamed_0 = 264;
pub const ID_SOUND_CONSTRUCTION_START: C2RustUnnamed_0 = 263;
pub const ID_SOUND_CONSTRUCTION_1: C2RustUnnamed_0 = 265;
pub const DACTION_DEMOLISH: _droid_action = 4;
pub const DACTION_REPAIR: _droid_action = 5;
pub const DACTION_DROIDREPAIR: _droid_action = 14;
pub const ID_SOUND_STRUCTURE_COMPLETED: C2RustUnnamed_0 = 14;
pub const DACTION_BUILD_FOUNDATION: _droid_action = 3;
pub const STR_GAM_STRREST: _fixed_str_id = 229;
pub const DACTION_RESTORE: _droid_action = 15;
pub const STR_GP_ASSIGNED: _fixed_str_id = 247;
pub const STR_GP_SELECTED: _fixed_str_id = 246;
pub const STR_GP_ALLIGN: _fixed_str_id = 249;
pub const STR_GP_CENTERED: _fixed_str_id = 248;
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
pub const STR_DL_LEVEL_ACE: _fixed_str_id = 294;
pub const STR_DL_LEVEL_SPECIAL: _fixed_str_id = 293;
pub const STR_DL_LEVEL_ELITE: _fixed_str_id = 292;
pub const STR_DL_LEVEL_CRACK: _fixed_str_id = 291;
pub const STR_DL_LEVEL_VETERAN: _fixed_str_id = 290;
pub const STR_DL_LEVEL_REGULAR: _fixed_str_id = 288;
pub const STR_DL_LEVEL_TRAINED: _fixed_str_id = 287;
pub const STR_DL_LEVEL_GREEN: _fixed_str_id = 286;
pub const STR_DL_LEVEL_ROOKIE: _fixed_str_id = 285;
pub const DACTION_CLEARWRECK: _droid_action = 16;
pub const DACTION_WAITFORREARM: _droid_action = 33;
pub const DACTION_WAITDURINGREARM: _droid_action = 35;
pub const DACTION_MOVETOREARMPOINT: _droid_action = 34;
pub const DACTION_MOVETOREARM: _droid_action = 32;
pub const DACTION_MOVEFIRE: _droid_action = 17;
pub const DACTION_VTOLATTACK: _droid_action = 36;
pub const DACTION_ROTATETOATTACK: _droid_action = 24;
pub const DACTION_MOVETOATTACK: _droid_action = 23;
pub const DACTION_ATTACK: _droid_action = 6;
pub const ID_SOUND_NEXUS_UNIT_ABSORBED: C2RustUnnamed_0 = 345;
pub type TRIGGER_TYPE = _trigger_type;
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
pub const CALL_UNITTAKEOVER: _scr_callback_types = 55;
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
pub type GATEWAY_LINK = _gateway_link;
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
pub const ID_SOUND_RICOCHET_3: C2RustUnnamed_0 = 275;
pub const ID_SOUND_RICOCHET_2: C2RustUnnamed_0 = 274;
pub const ID_SOUND_RICOCHET_1: C2RustUnnamed_0 = 273;
pub const ID_SOUND_EXPLOSION_ANTITANK: C2RustUnnamed_0 = 272;
pub const ID_SOUND_EXPLOSION_LASER: C2RustUnnamed_0 = 270;
pub const ID_SOUND_EXPLOSION_SMALL: C2RustUnnamed_0 = 269;
pub const ID_SOUND_CONSTRUCTION_4: C2RustUnnamed_0 = 268;
pub const ID_SOUND_CONSTRUCTION_3: C2RustUnnamed_0 = 267;
pub const ID_SOUND_CONSTRUCTION_2: C2RustUnnamed_0 = 266;
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
pub type _droid_action = libc::c_uint;
pub const DACTION_FIRESUPPORT_RETREAT: _droid_action = 39;
pub const DACTION_RETURNTOPOS: _droid_action = 38;
pub const DACTION_CLEARREARMPAD: _droid_action = 37;
pub const DACTION_MOVETOCLEAR: _droid_action = 31;
pub const DACTION_MOVETORESTORE: _droid_action = 30;
pub const DACTION_MOVETODROIDREPAIR: _droid_action = 29;
pub const DACTION_WAITDURINGREPAIR: _droid_action = 28;
pub const DACTION_MOVETOREPAIRPOINT: _droid_action = 27;
pub const DACTION_WAITFORREPAIR: _droid_action = 26;
pub const DACTION_MOVETOOBSERVE: _droid_action = 25;
pub const DACTION_FOUNDATION_WANDER: _droid_action = 22;
pub const DACTION_BUILDWANDER: _droid_action = 21;
pub const DACTION_MOVETOREPAIR: _droid_action = 20;
pub const DACTION_MOVETODEMOLISH: _droid_action = 19;
pub const DACTION_MOVETOBUILD: _droid_action = 18;
pub const DACTION_TRANSPORTIN: _droid_action = 13;
pub const DACTION_TRANSPORTWAITTOFLYIN: _droid_action = 12;
pub const DACTION_TRANSPORTOUT: _droid_action = 11;
pub const DACTION_DESTRUCT: _droid_action = 10;
pub const DACTION_SULK: _droid_action = 9;
pub const DACTION_FIRESUPPORT: _droid_action = 8;
pub const DACTION_OBSERVE: _droid_action = 7;
pub const DACTION_MOVE: _droid_action = 1;
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
pub const STR_DL_LEVEL_PROF: _fixed_str_id = 289;
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
pub const STR_GAM_ELECDAM: _fixed_str_id = 228;
pub const STR_GAM_STRDAM: _fixed_str_id = 227;
pub const STR_GAM_POWCON: _fixed_str_id = 226;
pub const STR_GAM_RESREM: _fixed_str_id = 225;
pub const STR_GAM_ENERGY: _fixed_str_id = 224;
pub const STR_GAM_NORTH: _fixed_str_id = 223;
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
pub type _scr_callback_types = libc::c_uint;
pub const CALL_BEACON: _scr_callback_types = 62;
pub const CALL_TRANSPORTER_LANDED_B: _scr_callback_types = 61;
pub const CALL_STRUCTBUILT: _scr_callback_types = 60;
pub const CALL_AI_MSG: _scr_callback_types = 59;
pub const CALL_CONSOLE: _scr_callback_types = 58;
pub const CALL_ALLIANCEOFFER: _scr_callback_types = 57;
pub const CALL_PLAYERLEFT: _scr_callback_types = 56;
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
// Returns TRUE if drive mode is active.
//
#[inline]
unsafe extern "C" fn driveModeActive() -> BOOL { return DirectControl; }
//storage
#[no_mangle]
pub static mut apsDroidTemplates: [*mut DROID_TEMPLATE; 8] =
    [0 as *const DROID_TEMPLATE as *mut DROID_TEMPLATE; 8];
// store the experience of recently recycled droids
#[no_mangle]
pub static mut aDroidExperience: [[UWORD; 32]; 8] = [[0; 32]; 8];
#[no_mangle]
pub static mut selectedGroup: UDWORD = 0xff as libc::c_int as UDWORD;
#[no_mangle]
pub static mut selectedCommander: UDWORD = 0xff as libc::c_int as UDWORD;
//how far round a repair droid looks for a damaged droid
//8)
/* Store for the objects near the droid currently being updated
 * NAYBOR = neighbour - thanks to Keith for a great abreviation
 */
#[no_mangle]
pub static mut asDroidNaybors: [NAYBOR_INFO; 100] =
    [NAYBOR_INFO{psObj: 0 as *const BASE_OBJECT as *mut BASE_OBJECT,
                 distSqr: 0,}; 100];
#[no_mangle]
pub static mut numNaybors: UDWORD = 0 as libc::c_int as UDWORD;
// store the last time a structure was hit for a side
// this controls when the CALL_STRUCT_ATTACKED is made
//UDWORD	aLastDroidHit[MAX_PLAYERS];
// the structure that was last hit
#[no_mangle]
pub static mut psLastDroidHit: *mut DROID = 0 as *const DROID as *mut DROID;
/*time to move to a new location (when building foundation) */
//static void moveToNewTile(DROID *psDroid);
// initialise droid module
#[no_mangle]
pub unsafe extern "C" fn droidInit() -> BOOL {
    memset(aDroidExperience.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           (::std::mem::size_of::<UWORD>() as
                libc::c_ulong).wrapping_mul(8 as libc::c_int as
                                                libc::c_uint).wrapping_mul(32
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_uint));
    //	memset(aLastDroidHit, 0, sizeof(UDWORD) * MAX_PLAYERS);
    psLastDroidHit = 0 as *mut DROID;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn droidDamage(mut psDroid: *mut DROID,
                                     mut damage: UDWORD,
                                     mut weaponClass: UDWORD,
                                     mut weaponSubClass: UDWORD) -> BOOL {
    let mut penDamage: UDWORD = 0;
    let mut armourDamage: UDWORD = 0;
    let mut penetrated: BOOL = 0 as libc::c_int;
    let mut armour: UDWORD = 0 as libc::c_int as UDWORD;
    let mut state: SECONDARY_STATE = 0 as SECONDARY_STATE;
    let mut level: SDWORD = 0;
    let mut cmdLevel: SDWORD = 0;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"unitDamage: Invalid Unit pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"droid.c\x00" as *const u8 as *const libc::c_char,
              154 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 12],
                                        &[libc::c_char; 12]>(b"droidDamage\x00")).as_ptr(),
              b"PTRVALID(psDroid, sizeof(DROID))\x00" as *const u8 as
                  *const libc::c_char);
    };
    //EMP cannons do not do body damage
    if weaponSubClass == WSC_EMP as libc::c_int as libc::c_uint {
        //store the time
        (*psDroid).timeLastHit = gameTime;
        (*psDroid).lastHitWeapon = weaponSubClass;
        //quit early
        return 0 as libc::c_int
    }
    //only overwrite if the last weapon to hit was not an EMP - need the time value for this
    if (*psDroid).lastHitWeapon != WSC_EMP as libc::c_int as libc::c_uint {
        (*psDroid).timeLastHit = gameTime;
        (*psDroid).lastHitWeapon = weaponSubClass
    }
    //	if(selectedPlayer==0)
    if (*psDroid).player as libc::c_uint != selectedPlayer {
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
    // vtols on the ground take triple damage
    if vtolDroid(psDroid) != 0 &&
           (*psDroid).sMove.Status as libc::c_int == 0 as libc::c_int {
        damage =
            (damage as
                 libc::c_uint).wrapping_mul(3 as libc::c_int as libc::c_uint)
                as UDWORD as UDWORD
    }
    // reset the attack level
    if secondaryGetState(psDroid, DSO_ATTACK_LEVEL, &mut state) != 0 {
        if state as libc::c_uint ==
               DSS_ALEV_ATTACKED as libc::c_int as libc::c_uint {
            secondarySetState(psDroid, DSO_ATTACK_LEVEL, DSS_ALEV_ALWAYS);
        }
    }
    match weaponClass {
        0 => {
            //case WC_EXPLOSIVE:
            if damage > (*psDroid).armour[WC_KINETIC as libc::c_int as usize]
               {
                penetrated = 1 as libc::c_int
            }
            armour = (*psDroid).armour[WC_KINETIC as libc::c_int as usize]
        }
        1 => {
            //case WC_MISC:
            if damage > (*psDroid).armour[WC_HEAT as libc::c_int as usize] {
                penetrated = 1 as libc::c_int
            }
            armour = (*psDroid).armour[WC_HEAT as libc::c_int as usize]
        }
        _ => { }
    }
    clustObjectAttacked(psDroid as *mut BASE_OBJECT);
    //	if (damage > psDroid->armour[WC_KINETIC])
    if penetrated != 0 {
        /* Damage has penetrated - reduce armour and body points */
		//penDamage = damage - psDroid->armour;
        penDamage = damage.wrapping_sub(armour);
        level = getDroidLevel(psDroid) as SDWORD;
        cmdLevel = cmdGetCommanderLevel(psDroid);
        if level > cmdLevel {
            /* Do damage to armour */
//		armourDamage = (damage / PEN_ARMOUR_DAMAGE_FACTOR) + 1;
            //		DBP1(("penetrated: %d, armour: %d\n", penDamage, armourDamage));
            //penDamage = (penDamage * (100 - 5 * level)) / 100;
            penDamage =
                penDamage.wrapping_mul((100 as libc::c_int -
                                            6 as libc::c_int * level) as
                                           libc::c_uint).wrapping_div(100 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_uint)
        } else {
            //penDamage = (penDamage * (100 - 5 * cmdLevel)) / 100;
            penDamage =
                penDamage.wrapping_mul((100 as libc::c_int -
                                            6 as libc::c_int * cmdLevel) as
                                           libc::c_uint).wrapping_div(100 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_uint)
        }
        if penDamage >= (*psDroid).body {
            //we don't want this in multiPlayer
            if bMultiPlayer == 0 {
                //hack to prevent Transporter's being blown up
                if (*psDroid).droidType as libc::c_uint ==
                       DROID_TRANSPORTER as libc::c_int as libc::c_uint {
                    (*psDroid).body = 1 as libc::c_int as UDWORD;
                    return 0 as libc::c_int
                }
            }
            /* Droid destroyed */
            if (*psDroid).player as libc::c_uint == selectedPlayer {
                sprintf(ConsoleString.as_mut_ptr(),
                        strresGetString(psStringRes,
                                        STR_GAM_UNITLOST as libc::c_int as
                                            UDWORD));
                addConsoleMessage(ConsoleString.as_mut_ptr(),
                                  DEFAULT_JUSTIFY);
                scoreUpdateVar(WD_UNITS_LOST);
                audio_QueueTrackMinDelayPos(ID_SOUND_UNIT_DESTROYED as
                                                libc::c_int,
                                            (5 as libc::c_int *
                                                 1000 as libc::c_int) as
                                                UDWORD,
                                            (*psDroid).x as SDWORD,
                                            (*psDroid).y as SDWORD,
                                            (*psDroid).z as SDWORD);
                //				"INTEL REPORT : Unit Lost!"));
            } else { scoreUpdateVar(WD_UNITS_KILLED); }
            if (*psDroid).droidType as libc::c_uint ==
                   DROID_PERSON as libc::c_int as libc::c_uint &&
                   weaponClass == WC_HEAT as libc::c_int as libc::c_uint {
                droidBurn(psDroid);
            } else { destroyDroid(psDroid); }
            return 1 as libc::c_int
        } else {
            (*psDroid).body =
                ((*psDroid).body as libc::c_uint).wrapping_sub(penDamage) as
                    UDWORD as UDWORD
        }
    } else {
        /* Damage didn't penetrate - only reduce armour */
        armourDamage =
            damage.wrapping_div(50 as libc::c_int as
                                    libc::c_uint).wrapping_add(1 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uint);
        //		DBP1(("armour: %d\n", armourDamage));
        if (*psDroid).droidType as libc::c_uint ==
               DROID_PERSON as libc::c_int as libc::c_uint &&
               weaponClass == WC_HEAT as libc::c_int as libc::c_uint {
            droidBurn(psDroid);
        }
        if (*psDroid).body == 1 as libc::c_int as libc::c_uint {
            /* Do one point of damage to body */
            //we don't want this in multiPlayer
            if bMultiPlayer == 0 {
                //hack to prevent Transporter's being blown up
                if (*psDroid).droidType as libc::c_uint ==
                       DROID_TRANSPORTER as libc::c_int as libc::c_uint {
                    return 0 as libc::c_int
                }
            }
            if (*psDroid).player as libc::c_uint == selectedPlayer {
                sprintf(ConsoleString.as_mut_ptr(),
                        strresGetString(psStringRes,
                                        STR_GAM_UNITLOST as libc::c_int as
                                            UDWORD));
                addConsoleMessage(ConsoleString.as_mut_ptr(),
                                  DEFAULT_JUSTIFY);
                scoreUpdateVar(WD_UNITS_LOST);
                audio_QueueTrackMinDelayPos(ID_SOUND_UNIT_DESTROYED as
                                                libc::c_int,
                                            (5 as libc::c_int *
                                                 1000 as libc::c_int) as
                                                UDWORD,
                                            (*psDroid).x as SDWORD,
                                            (*psDroid).y as SDWORD,
                                            (*psDroid).z as SDWORD);
            } else { scoreUpdateVar(WD_UNITS_KILLED); }
            destroyDroid(psDroid);
            return 1 as libc::c_int
        } else {
            (*psDroid).body =
                ((*psDroid).body as
                     libc::c_uint).wrapping_sub(1 as libc::c_int as
                                                    libc::c_uint) as UDWORD as
                    UDWORD
        }
    }
    /* Actually reduce the droids armour */
/*	if (armourDamage >= psDroid->armour)
	{
		psDroid->armour = 0;
	}
	else
	{
		psDroid->armour -= armourDamage;
	}*/
    /* now check for auto return on droid's secondary orders */
    secondaryCheckDamageLevel(psDroid);
    /* now check for scripted run-away based on health */
    orderHealthCheck(psDroid);
    //only overwrite if the last weapon to hit was not an EMP - need the time value for this
    if (*psDroid).lastHitWeapon != WSC_EMP as libc::c_int as libc::c_uint {
        (*psDroid).timeLastHit = gameTime;
        (*psDroid).lastHitWeapon = weaponSubClass
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn droidDeleteName(mut psDroid: *mut DROID) { }
/* droidRelease: release all resources associated with a droid -
 * should only be called by objmem - use vanishDroid preferably
 */
#[no_mangle]
pub unsafe extern "C" fn droidRelease(mut psDroid: *mut DROID) {
    let mut psCurr: *mut DROID = 0 as *mut DROID;
    let mut psNext: *mut DROID = 0 as *mut DROID;
    droidDeleteName(psDroid);
    if (*psDroid).droidType as libc::c_uint ==
           DROID_TRANSPORTER as libc::c_int as libc::c_uint {
        if !(*psDroid).psGroup.is_null() {
            //free all droids associated with this Transporter
            psCurr = (*(*psDroid).psGroup).psList;
            while !psCurr.is_null() && psCurr != psDroid {
                psNext = (*psCurr).psGrpNext;
                droidRelease(psCurr);
                heapFree(psDroidHeap, psCurr as *mut libc::c_void);
                psCurr = psNext
            }
        }
    }
    //FREE(psDroid->pName);
    // leave the current formation if any
    if !(*psDroid).sMove.psFormation.is_null() {
        formationLeave((*psDroid).sMove.psFormation,
                       psDroid as *mut BASE_OBJECT);
    }
    // leave the current group if any
    if !(*psDroid).psGroup.is_null() {
        grpLeave((*psDroid).psGroup, psDroid);
    }
    // remove the object from the grid
    gridRemoveObject(psDroid as *mut BASE_OBJECT);
    // remove the droid from the cluster systerm
    clustRemoveObject(psDroid as *mut BASE_OBJECT);
}
// recycle a droid (retain it's experience and some of it's cost)
#[no_mangle]
pub unsafe extern "C" fn recycleDroid(mut psDroid: *mut DROID) {
    let mut numKills: UDWORD = 0;
    let mut minKills: UDWORD = 0;
    let mut i: SDWORD = 0;
    let mut cost: SDWORD = 0;
    let mut storeIndex: SDWORD = 0;
    let mut position: iVector = iVector{x: 0, y: 0, z: 0,};
    // store the droids kills
    numKills = (*psDroid).numKills as UDWORD;
    if numKills > 0xffff as libc::c_int as libc::c_uint {
        numKills = 0xffff as libc::c_int as UDWORD
    }
    minKills = 0xffff as libc::c_int as UDWORD;
    storeIndex = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if (aDroidExperience[(*psDroid).player as usize][i as usize] as
                libc::c_int) < minKills as UWORD as libc::c_int {
            storeIndex = i;
            minKills =
                aDroidExperience[(*psDroid).player as usize][i as usize] as
                    UDWORD
        }
        i += 1
    }
    aDroidExperience[(*psDroid).player as usize][storeIndex as usize] =
        numKills as UWORD;
    // return part of the cost of the droid
    cost = calcDroidPower(psDroid) as SDWORD;
    cost =
        ((cost / 2 as libc::c_int) as
             libc::c_uint).wrapping_mul((*psDroid).body).wrapping_div((*psDroid).originalBody)
            as SDWORD;
    addPower((*psDroid).player as UDWORD, cost as UDWORD);
    // hide the droid
    memset((*psDroid).visible.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<[UBYTE; 8]>() as libc::c_ulong);
    // stop any group moral checks
    if !(*psDroid).psGroup.is_null() {
        grpLeave((*psDroid).psGroup, psDroid); // Add an effect
    }
    position.x = (*psDroid).x as int32;
    position.z = (*psDroid).y as int32;
    position.y = (*psDroid).z as int32;
    //	destroyDroid(psDroid);
    vanishDroid(psDroid);
    addEffect(&mut position, EFFECT_EXPLOSION, EXPLOSION_TYPE_DISCOVERY,
              0 as libc::c_int, 0 as *mut iIMDShape, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn removeDroidBase(mut psDel: *mut DROID) {
    let mut psCurr: *mut DROID = 0 as *mut DROID;
    let mut psNext: *mut DROID = 0 as *mut DROID;
    let mut bRet: BOOL = 0;
    let mut psGroup: *mut DROID_GROUP = 0 as *mut DROID_GROUP;
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    (driveDroidKilled(psDel)) == 0;
    //tell the power system its gone
    powerDestroyObject(psDel as *mut BASE_OBJECT);
    if (*psDel).died != 0 && (*psDel).died != 1 as libc::c_int as libc::c_uint
       {
        // droid has already been killed, quit
        return
    }
    //ajl, inform others of destruction.
    if bMultiPlayer != 0 {
        if !((*psDel).player as libc::c_uint != selectedPlayer &&
                 (*psDel).order == DORDER_RECYCLE as libc::c_int) {
            SendDestroyDroid(psDel);
        }
    }
    /* remove animation if present */
    if !(*psDel).psCurAnim.is_null() {
        bRet =
            animObj_Remove(&mut (*psDel).psCurAnim,
                           (*(*(*psDel).psCurAnim).psAnim).uwID as
                               libc::c_int);
        if bRet == 1 as libc::c_int {
        } else {
            debug(LOG_ERROR,
                  b"destroyUnit: animObj_Remove failed\x00" as *const u8 as
                      *const libc::c_char);
        };
        if bRet == 1 as libc::c_int {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"droid.c\x00" as *const u8 as *const libc::c_char,
                  512 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 16],
                                            &[libc::c_char; 16]>(b"removeDroidBase\x00")).as_ptr(),
                  b"bRet == TRUE\x00" as *const u8 as *const libc::c_char);
        };
        (*psDel).psCurAnim = 0 as *mut ANIM_OBJECT
    }
    //put back the power required to maintain this droid (=power to build)
	//returnPower(psDel->player, psDel->power);
    // leave the current formation if any
    if !(*psDel).sMove.psFormation.is_null() {
        formationLeave((*psDel).sMove.psFormation, psDel as *mut BASE_OBJECT);
        (*psDel).sMove.psFormation = 0 as *mut _formation
    }
    //kill all the droids inside the transporter
    if (*psDel).droidType as libc::c_uint ==
           DROID_TRANSPORTER as libc::c_int as libc::c_uint {
        if !(*psDel).psGroup.is_null() {
            //free all droids associated with this Transporter
            psCurr = (*(*psDel).psGroup).psList;
            while !psCurr.is_null() && psCurr != psDel {
                psNext = (*psCurr).psGrpNext;
                /* add droid to droid list then vanish it - hope this works! - GJ */
                addDroid(psCurr, apsDroidLists.as_mut_ptr());
                vanishDroid(psCurr);
                psCurr = psNext
            }
        }
    }
    // check moral
    if !(*psDel).psGroup.is_null() &&
           (*(*psDel).psGroup).refCount as libc::c_int > 1 as libc::c_int {
        psGroup = (*psDel).psGroup;
        grpLeave((*psDel).psGroup, psDel);
        orderGroupMoralCheck(psGroup);
    } else {
        //else if (psDel->player == BARB1 || psDel->player == BARB2)
        orderMoralCheck((*psDel).player as UDWORD);
    }
    // leave the current group if any
    if !(*psDel).psGroup.is_null() {
        grpLeave((*psDel).psGroup, psDel);
        (*psDel).psGroup = 0 as *mut _droid_group
    }
    //PUT THIS IN removeDroidFX()
	//once a droid is destroyed - it leaves a wrecked droid FEATURE in its place
//	buildFeature((asFeatureStats + droidFeature), psDel->x, psDel->y);
	//if( (psDel->droidType == DROID_PERSON || psDel->droidType == DROID_CYBORG) &&
	//	(psDel->order != DORDER_RUNBURN) )
	//{
	//	/* blow person up into blood and guts */
	//	compPersonToBits(psDel);
	//}
    /* Put Deliv. Pts back into world when a command droid dies */
    if (*psDel).droidType as libc::c_uint ==
           DROID_COMMAND as libc::c_int as libc::c_uint {
        psStruct = apsStructLists[(*psDel).player as usize];
        while !psStruct.is_null() {
            /* Replace the delivery points for the factories assigned to this command droid */
//			if ( psStruct->pStructureType->type == REF_FACTORY )	/* Is it a factory? */
//			{
//				assignFactoryCommandDroid(psStruct, NULL);	/* Return d. pt. */
//			}
            // alexl's stab at a right answer.
            if StructIsFactory(psStruct) != 0 &&
                   (*((*psStruct).pFunctionality as *mut FACTORY)).psCommander
                       == psDel {
                assignFactoryCommandDroid(psStruct, 0 as *mut _droid);
            }
            psStruct = (*psStruct).psNext
        }
    }
    //check to see if constructor droid currently trying to find a location to build
    //if (psDel->droidType == DROID_CONSTRUCT AND psDel->player ==
    if ((*psDel).droidType as libc::c_uint ==
            DROID_CONSTRUCT as libc::c_int as libc::c_uint ||
            (*psDel).droidType as libc::c_uint ==
                DROID_CYBORG_CONSTRUCT as libc::c_int as libc::c_uint) &&
           (*psDel).player as libc::c_uint == selectedPlayer &&
           (*psDel).selected as libc::c_int != 0 {
        //if currently trying to build, kill off the placement
        if tryingToGetLocation() != 0 { kill3DBuilding(); }
    }
    // kill the command droid if any
/*	- no seperate COMMAND_DROID structure now
	if (psDel->asBits[COMP_BRAIN].nStat != 0)
	{
		destroyCommandDroid((SDWORD)psDel->player,
							(SDWORD)psDel->asBits[COMP_BRAIN].nStat);
	}*/
    // remove the droid from the grid
    gridRemoveObject(psDel as *mut BASE_OBJECT);
    // remove the droid from the cluster systerm
    clustRemoveObject(psDel as *mut BASE_OBJECT);
    if (*psDel).player as libc::c_uint == selectedPlayer {
        intRefreshScreen();
    }
    killDroid(psDel);
}
/* Put this back in if everything starts blowing up at once!!!
// -------------------------------------------------------------------------------
UDWORD	lastDroidRemove=0;
UDWORD	droidRemoveKills=0;
// -------------------------------------------------------------------------------
*/
#[no_mangle]
pub unsafe extern "C" fn removeDroidFX(mut psDel: *mut DROID) {
    let mut pos: iVector = iVector{x: 0, y: 0, z: 0,};
    // only display anything if the droid is visible
    if (*psDel).visible[selectedPlayer as usize] == 0 { return }
    /*
	// -------------------------------------------------------------------------------
	if(psDel->player == selectedPlayer)
	{
		if(gameTime - lastDroidRemove < 100)	//assume 10 fames a sec min
		{
			droidRemoveKills++;
			if(droidRemoveKills>=2)
			{
				ASSERT( FALSE,"3 of your droids killed in less than a tenth of a second?" );
			}
		}
		else
		{
			droidRemoveKills = 0;
		}
		lastDroidRemove = gameTime;
	}
	// -------------------------------------------------------------------------------

  */
    //if( (psDel->droidType == DROID_PERSON || psDel->droidType == DROID_CYBORG) &&
    if ((*psDel).droidType as libc::c_uint ==
            DROID_PERSON as libc::c_int as libc::c_uint ||
            cyborgDroid(psDel) != 0) &&
           (*psDel).order != DORDER_RUNBURN as libc::c_int {
        /* blow person up into blood and guts */
        compPersonToBits(psDel);
    }
    /* if baba and not running (on fire) then squish */
    if (*psDel).droidType as libc::c_uint ==
           DROID_PERSON as libc::c_int as libc::c_uint {
        if (*psDel).order != DORDER_RUNBURN as libc::c_int {
            if (*psDel).visible[selectedPlayer as usize] != 0 {
                // The babarian has been run over ...
                audio_PlayStaticTrack((*psDel).x as SDWORD,
                                      (*psDel).y as SDWORD,
                                      ID_SOUND_BARB_SQUISH as libc::c_int);
            }
        }
    } else if (*psDel).visible[selectedPlayer as usize] != 0 {
        destroyFXDroid(psDel);
        // this sounds a bit crap on the psx
        pos.x = (*psDel).x as int32;
        pos.z = (*psDel).y as int32;
        pos.y = (*psDel).z as int32;
        addEffect(&mut pos, EFFECT_DESTRUCTION, DESTRUCTION_TYPE_DROID,
                  0 as libc::c_int, 0 as *mut iIMDShape, 0 as libc::c_int);
        audio_PlayStaticTrack((*psDel).x as SDWORD, (*psDel).y as SDWORD,
                              ID_SOUND_EXPLOSION as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn destroyDroid(mut psDel: *mut DROID) {
    if (*psDel).lastHitWeapon == WSC_LAS_SAT as libc::c_int as libc::c_uint {
        // darken tile if lassat.
        let mut width: UDWORD = 0;
        let mut breadth: UDWORD = 0;
        let mut psTile: *mut MAPTILE = 0 as *mut MAPTILE;
        mapX = ((*psDel).x as libc::c_int >> 7 as libc::c_int) as UDWORD;
        mapY = ((*psDel).y as libc::c_int >> 7 as libc::c_int) as UDWORD;
        width = mapX.wrapping_sub(1 as libc::c_int as libc::c_uint);
        while width <= mapX.wrapping_add(1 as libc::c_int as libc::c_uint) {
            breadth = mapY.wrapping_sub(1 as libc::c_int as libc::c_uint);
            while breadth <=
                      mapY.wrapping_add(1 as libc::c_int as libc::c_uint) {
                psTile = mapTile(width, breadth);
                if (*psTile).tileVisBits as libc::c_int &
                       (1 as libc::c_int) << selectedPlayer != 0 {
                    (*psTile).illumination =
                        ((*psTile).illumination as libc::c_int /
                             2 as libc::c_int) as UBYTE;
                    if (*psTile).bMaxed as libc::c_int != 0 &&
                           (*psTile).level as libc::c_int !=
                               0xff as libc::c_int {
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
    removeDroidFX(psDel);
    removeDroidBase(psDel);
}
#[no_mangle]
pub unsafe extern "C" fn vanishDroid(mut psDel: *mut DROID) {
    removeDroidBase(psDel);
}
/* Remove a droid from the List so doesn't update or get drawn etc
TAKE CARE with removeDroid() - usually want droidRemove since it deal with cluster and grid code*/
//returns FALSE if the droid wasn't removed - because it died!
#[no_mangle]
pub unsafe extern "C" fn droidRemove(mut psDroid: *mut DROID,
                                     mut pList: *mut *mut DROID) -> BOOL {
    //	BOOL	bRet;
    (driveDroidKilled(psDroid)) == 0;
    //tell the power system its gone
    powerDestroyObject(psDroid as *mut BASE_OBJECT);
    if (*psDroid).died != 0 &&
           (*psDroid).died != 1 as libc::c_int as libc::c_uint {
        // droid has already been killed, quit
        return 0 as libc::c_int
    }
    /* remove animation if present  - do this when the droid is destroyed*/
	/*if ( psDroid->psCurAnim != NULL )
	{
		bRet = animObj_Remove( psDroid->psCurAnim, psDroid->psCurAnim->psAnim->uwID );
		ASSERT( bRet == TRUE, "droidRemove: animObj_Remove failed" );
		psDroid->psCurAnim = NULL;
	}*/
    // leave the current formation if any
    if !(*psDroid).sMove.psFormation.is_null() {
        formationLeave((*psDroid).sMove.psFormation,
                       psDroid as *mut BASE_OBJECT);
        (*psDroid).sMove.psFormation = 0 as *mut _formation
    }
    // leave the current group if any - not if its a Transporter droid
    if (*psDroid).droidType as libc::c_uint !=
           DROID_TRANSPORTER as libc::c_int as libc::c_uint &&
           !(*psDroid).psGroup.is_null() {
        grpLeave((*psDroid).psGroup, psDroid);
        (*psDroid).psGroup = 0 as *mut _droid_group
    }
    //reset the baseStruct
    (*psDroid).psBaseStruct = 0 as *mut _structure;
    // remove the droid from the cluster systerm
    clustRemoveObject(psDroid as *mut BASE_OBJECT);
    // remove the droid from the grid
    gridRemoveObject(psDroid as *mut BASE_OBJECT);
    droidDeleteName(psDroid);
    removeDroid(psDroid, pList);
    //tell the power system its gone
    powerDestroyObject(psDroid as *mut BASE_OBJECT);
    if (*psDroid).player as libc::c_uint == selectedPlayer {
        intRefreshScreen();
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn droidFlameFallCallback(mut psObj: *mut ANIM_OBJECT) {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"unitFlameFallCallback: invalid anim object pointer\n\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"droid.c\x00" as *const u8 as *const libc::c_char,
              1022 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 23],
                                        &[libc::c_char; 23]>(b"droidFlameFallCallback\x00")).as_ptr(),
              b"PTRVALID(psObj, sizeof(ANIM_OBJECT))\x00" as *const u8 as
                  *const libc::c_char);
    };
    psDroid = (*psObj).psParent as *mut DROID;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"unitFlameFallCallback: invalid Unit pointer\n\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"droid.c\x00" as *const u8 as *const libc::c_char,
              1025 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 23],
                                        &[libc::c_char; 23]>(b"droidFlameFallCallback\x00")).as_ptr(),
              b"PTRVALID(psDroid, sizeof(DROID))\x00" as *const u8 as
                  *const libc::c_char);
    };
    (*psDroid).psCurAnim = 0 as *mut ANIM_OBJECT;
    destroyDroid(psDroid);
}
#[no_mangle]
pub unsafe extern "C" fn droidBurntCallback(mut psObj: *mut ANIM_OBJECT) {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"unitBurntCallback: invalid anim object pointer\n\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"droid.c\x00" as *const u8 as *const libc::c_char,
              1037 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 19],
                                        &[libc::c_char; 19]>(b"droidBurntCallback\x00")).as_ptr(),
              b"PTRVALID(psObj, sizeof(ANIM_OBJECT))\x00" as *const u8 as
                  *const libc::c_char);
    };
    psDroid = (*psObj).psParent as *mut DROID;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"unitBurntCallback: invalid Unit pointer\n\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"droid.c\x00" as *const u8 as *const libc::c_char,
              1040 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 19],
                                        &[libc::c_char; 19]>(b"droidBurntCallback\x00")).as_ptr(),
              b"PTRVALID(psDroid, sizeof(DROID))\x00" as *const u8 as
                  *const libc::c_char);
    };
    /* add falling anim */
    (*psDroid).psCurAnim =
        animObj_Add(psDroid as *mut BASE_OBJECT as *mut libc::c_void,
                    4 as libc::c_int, 0 as libc::c_int as UDWORD,
                    1 as libc::c_int as UWORD);
    if (*psDroid).psCurAnim.is_null() {
        debug(LOG_ERROR,
              b"unitBurntCallback: couldn\'t add fall over anim\n\x00" as
                  *const u8 as *const libc::c_char);
        abort();
    }
    animObj_SetDoneFunc((*psDroid).psCurAnim,
                        Some(droidFlameFallCallback as
                                 unsafe extern "C" fn(_: *mut ANIM_OBJECT)
                                     -> ()));
}
#[no_mangle]
pub unsafe extern "C" fn droidBurn(mut psDroid: *mut DROID) {
    let mut bRet: BOOL = 0;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"unitBurn: invalid Unit pointer\n\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"droid.c\x00" as *const u8 as *const libc::c_char,
              1060 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 10],
                                        &[libc::c_char; 10]>(b"droidBurn\x00")).as_ptr(),
              b"PTRVALID(psDroid, sizeof(DROID))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (*psDroid).droidType as libc::c_uint !=
           DROID_PERSON as libc::c_int as libc::c_uint {
        debug(LOG_ERROR,
              b"unitBurn: can\'t burn anything except babarians currently!\n\x00"
                  as *const u8 as *const libc::c_char);
        abort();
    }
    /* if already burning return else remove currently-attached anim if present */
    if !(*psDroid).psCurAnim.is_null() {
        /* return if already burning */
        if (*(*(*psDroid).psCurAnim).psAnim).uwID as libc::c_int ==
               3 as libc::c_int {
            return
        } else {
            bRet =
                animObj_Remove(&mut (*psDroid).psCurAnim,
                               (*(*(*psDroid).psCurAnim).psAnim).uwID as
                                   libc::c_int);
            if bRet == 1 as libc::c_int {
            } else {
                debug(LOG_ERROR,
                      b"unitBurn: animObj_Remove failed\x00" as *const u8 as
                          *const libc::c_char);
            };
            if bRet == 1 as libc::c_int {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"droid.c\x00" as *const u8 as *const libc::c_char,
                      1080 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 10],
                                                &[libc::c_char; 10]>(b"droidBurn\x00")).as_ptr(),
                      b"bRet == TRUE\x00" as *const u8 as
                          *const libc::c_char);
            };
            (*psDroid).psCurAnim = 0 as *mut ANIM_OBJECT
        }
    }
    /* add burning anim */
    (*psDroid).psCurAnim =
        animObj_Add(psDroid as *mut BASE_OBJECT as *mut libc::c_void,
                    3 as libc::c_int, 0 as libc::c_int as UDWORD,
                    3 as libc::c_int as UWORD);
    if (*psDroid).psCurAnim.is_null() {
        debug(LOG_ERROR,
              b"unitBurn: couldn\'t add burn anim\n\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    /* set callback */
    animObj_SetDoneFunc((*psDroid).psCurAnim,
                        Some(droidBurntCallback as
                                 unsafe extern "C" fn(_: *mut ANIM_OBJECT)
                                     -> ()));
    /* add scream */
    debug(LOG_NEVER, b"baba burn\n\x00" as *const u8 as *const libc::c_char);
    audio_PlayObjDynamicTrack(psDroid as *mut libc::c_void,
                              ID_SOUND_BARB_SCREAM as libc::c_int +
                                  rand() % 3 as libc::c_int, None);
    /* set droid running */
    orderDroid(psDroid, DORDER_RUNBURN);
}
/* Add a new object to the naybor list */
unsafe extern "C" fn addNaybor(mut psObj: *mut BASE_OBJECT,
                               mut distSqr: UDWORD) {
    let mut pos: UDWORD = 0;
    if numNaybors >= 100 as libc::c_int as libc::c_uint {
        //		DBPRINTF(("Naybor list maxed out for id %d\n", psObj->id));
        return
    } else {
        if numNaybors == 0 as libc::c_int as libc::c_uint {
            // No objects in the list
            asDroidNaybors[0 as libc::c_int as usize].psObj = psObj;
            asDroidNaybors[0 as libc::c_int as usize].distSqr = distSqr;
            numNaybors = numNaybors.wrapping_add(1)
        } else if distSqr >=
                      asDroidNaybors[numNaybors.wrapping_sub(1 as libc::c_int
                                                                 as
                                                                 libc::c_uint)
                                         as usize].distSqr {
            // Simple case - this is the most distant object
            asDroidNaybors[numNaybors as usize].psObj = psObj;
            asDroidNaybors[numNaybors as usize].distSqr = distSqr;
            numNaybors = numNaybors.wrapping_add(1)
        } else {
            // Move all the objects further away up the list
            pos = numNaybors;
            while pos > 0 as libc::c_int as libc::c_uint &&
                      asDroidNaybors[pos.wrapping_sub(1 as libc::c_int as
                                                          libc::c_uint) as
                                         usize].distSqr > distSqr {
                memcpy(asDroidNaybors.as_mut_ptr().offset(pos as isize) as
                           *mut libc::c_void,
                       asDroidNaybors.as_mut_ptr().offset(pos.wrapping_sub(1
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_uint)
                                                              as isize) as
                           *const libc::c_void,
                       ::std::mem::size_of::<NAYBOR_INFO>() as libc::c_ulong);
                pos = pos.wrapping_sub(1)
            }
            // Insert the object at the correct position
            asDroidNaybors[pos as usize].psObj = psObj;
            asDroidNaybors[pos as usize].distSqr = distSqr;
            numNaybors = numNaybors.wrapping_add(1)
        }
    }
    if numNaybors <= 100 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"addNaybor: numNaybors > MAX_NAYBORS\x00" as *const u8 as
                  *const libc::c_char);
    };
    if numNaybors <= 100 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"droid.c\x00" as *const u8 as *const libc::c_char,
              1149 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 10],
                                        &[libc::c_char; 10]>(b"addNaybor\x00")).as_ptr(),
              b"numNaybors <= MAX_NAYBORS\x00" as *const u8 as
                  *const libc::c_char);
    };
}
static mut CurrentNaybors: *mut DROID = 0 as *const DROID as *mut DROID;
static mut nayborTime: UDWORD = 0 as libc::c_int as UDWORD;
#[no_mangle]
pub unsafe extern "C" fn droidResetNaybors() {
    CurrentNaybors = 0 as *mut DROID;
}
//#ifndef PSX
//
//void droidGetNaybors(DROID *psDroid)
//{
//	droidGetNayb(psDroid);
//}
//
//#else
//
//void droidGetNaybors(DROID *psDroid)
//{
//	static DROID *psTmpDroid;
//	psTmpDroid = psDroid;
//
//	SetSpDCache();
//	droidGetNayb(psTmpDroid);
//	SetSpNormal();
//}
//
//#endif
// macro to see if an object is in NAYBOR_RANGE
// used by droidGetNayb
/* Find all the objects close to the droid */
#[no_mangle]
pub unsafe extern "C" fn droidGetNaybors(mut psDroid: *mut DROID) {
    //	DROID		*psCurrD;
//	STRUCTURE	*psCurrS;
//	FEATURE		*psCurrF;
    let mut xdiff: SDWORD = 0;
    let mut ydiff: SDWORD = 0;
    //	UDWORD		player;
    let mut dx: UDWORD = 0;
    let mut dy: UDWORD = 0;
    let mut distSqr: UDWORD = 0;
    let mut psObj: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    // Ensure only called max of once per droid per game cycle.
    if CurrentNaybors == psDroid && nayborTime == gameTime { return }
    CurrentNaybors = psDroid;
    nayborTime = gameTime;
    // reset the naybor array
    numNaybors = 0 as libc::c_int as UDWORD;
    // search for naybor objects
    dx = (*psDroid).x as UDWORD;
    dy = (*psDroid).y as UDWORD;
    /*	for(player = 0; player < MAX_PLAYERS; player++)
	{
		for (psCurrD = apsDroidLists[player]; psCurrD; psCurrD = psCurrD->psNext)
		{
			if (psCurrD != psDroid)
			{
				IN_NAYBOR_RANGE(psCurrD);

				addNaybor((BASE_OBJECT *)psCurrD, distSqr);
			}
		}
		for (psCurrS = apsStructLists[player]; psCurrS; psCurrS = psCurrS->psNext)
		{
			IN_NAYBOR_RANGE(psCurrS);

			addNaybor((BASE_OBJECT *)psCurrS, distSqr);
		}
	}
	for (psCurrF = apsFeatureLists[0]; psCurrF; psCurrF = psCurrF->psNext)
	{
		IN_NAYBOR_RANGE(psCurrF);

		addNaybor((BASE_OBJECT *)psCurrF, distSqr);
	}*/
    gridStartIterate(dx as SDWORD, dy as SDWORD);
    psObj = gridIterate();
    while !psObj.is_null() {
        if psObj != psDroid as *mut BASE_OBJECT {
            xdiff =
                dx.wrapping_sub((*psObj).x as SDWORD as libc::c_uint) as
                    SDWORD;
            if xdiff < 0 as libc::c_int { xdiff = -xdiff }
            if !(xdiff > 128 as libc::c_int * 8 as libc::c_int) {
                ydiff =
                    dy.wrapping_sub((*psObj).y as SDWORD as libc::c_uint) as
                        SDWORD;
                if ydiff < 0 as libc::c_int { ydiff = -ydiff }
                if !(ydiff > 128 as libc::c_int * 8 as libc::c_int) {
                    distSqr = (xdiff * xdiff + ydiff * ydiff) as UDWORD;
                    if !(distSqr >
                             (128 as libc::c_int * 8 as libc::c_int *
                                  (128 as libc::c_int * 8 as libc::c_int)) as
                                 libc::c_uint) {
                        addNaybor(psObj, distSqr);
                    }
                }
            }
        }
        psObj = gridIterate()
    };
}
/* Display the neigbours of a droid */
#[no_mangle]
pub unsafe extern "C" fn displayNaybors() {
    let mut count: UDWORD = 0;
    let mut y: UDWORD = 0;
    let mut pType: *mut STRING = 0 as *mut STRING;
    y = 100 as libc::c_int as UDWORD;
    count = 0 as libc::c_int as UDWORD;
    while count < numNaybors {
        match (*asDroidNaybors[count as usize].psObj).type_0 as libc::c_uint {
            0 => {
                pType =
                    b"UNIT  \x00" as *const u8 as *const libc::c_char as
                        *mut STRING
            }
            1 => {
                pType =
                    b"STRUCT \x00" as *const u8 as *const libc::c_char as
                        *mut STRING
            }
            2 => {
                pType =
                    b"FEATURE\x00" as *const u8 as *const libc::c_char as
                        *mut STRING
            }
            _ => { }
        }
        screenTextOut(450 as libc::c_int as UDWORD, y,
                      b"%s %d\x00" as *const u8 as *const libc::c_char as
                          *mut STRING, pType,
                      (*asDroidNaybors[count as usize].psObj).id);
        y =
            (y as
                 libc::c_uint).wrapping_add(15 as libc::c_int as libc::c_uint)
                as UDWORD as UDWORD;
        count = count.wrapping_add(1)
    };
}
//static DROID *psTmpDroid;
/* The main update routine for all droids */
#[no_mangle]
pub unsafe extern "C" fn droidUpdate(mut psDroid: *mut DROID) {
    let mut dv: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut percentDamage: UDWORD = 0;
    let mut emissionInterval: UDWORD = 0;
    let mut psBeingTargetted: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut damageToDo: SDWORD = 0;
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
              b"droid.c\x00" as *const u8 as *const libc::c_char,
              1328 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 12],
                                        &[libc::c_char; 12]>(b"droidUpdate\x00")).as_ptr(),
              b"PTRVALID(psDroid, sizeof(DROID))\x00" as *const u8 as
                  *const libc::c_char);
    };
    //	ASSERT( psDroid->x != 0 && psDroid->y != 0,
//		"unitUpdate: Unit at (0,0)" );
    // Slap the stack in the DCache.
    // Find all the objects close to the droid
//	droidGetNaybors(psTmpDroid);	// Now done when needed.
    /* Clear down every droid as attacker could get killed */
//	psDroid->bTargetted = FALSE;
    // update the cluster of the droid
    if (*psDroid).id.wrapping_rem(20 as libc::c_int as libc::c_uint) ==
           frameGetFrameNumber().wrapping_rem(20 as libc::c_int as
                                                  libc::c_uint) {
        clustUpdateObject(psDroid as *mut BASE_OBJECT);
    }
    //may need power
    if droidUsesPower(psDroid) != 0 {
        //	    if ((asPower[psDroid->player]->currentPower > POWER_PER_CYCLE) OR
//		    (!powerCalculated))
        if checkPower((*psDroid).player as UDWORD, 5 as libc::c_int as UDWORD,
                      0 as libc::c_int) != 0 {
            //check if this droid is due some power
            if getLastPowered(psDroid as *mut BASE_OBJECT) != 0 {
                //get some power if necessary
                if accruePower(psDroid as *mut BASE_OBJECT) != 0 {
                    updateLastPowered(psDroid as *mut BASE_OBJECT,
                                      (*psDroid).player);
                }
            }
        }
    }
    // ai update droid
//#ifndef PSX
    aiUpdateDroid(psDroid);
    //#else
//	PROFILE_START(4);
//	psTmpDroid = psDroid;
//	SetSpDCache();
//	aiUpdateDroid(psTmpDroid);
//	SetSpNormal();
//	PROFILE_END(4);
//#endif
    // update the droids order
    orderUpdateDroid(psDroid);
    // update the action of the droid
    actionUpdateDroid(psDroid);
    //#ifndef PSX
	// update the move system
    moveUpdateDroid(psDroid);
    //#else
//	PROFILE_START(5);
//	psTmpDroid = psDroid;
//	SetSpDCache();
//	moveUpdateDroid(psTmpDroid);
//	SetSpNormal();
//	PROFILE_END(5);
//#endif
    /* Only add smoke if they're visible */
    if (*psDroid).visible[selectedPlayer as usize] as libc::c_int != 0 &&
           (*psDroid).droidType as libc::c_uint !=
               DROID_PERSON as libc::c_int as libc::c_uint {
        //percentDamage= PERCENT(psDroid->body,(asBodyStats + psDroid->asBits[COMP_BODY].nStat)->bodyPoints );
        percentDamage =
            (100 as libc::c_int as
                 libc::c_uint).wrapping_sub((*psDroid).body.wrapping_mul(100
                                                                             as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint).wrapping_div((*psDroid).originalBody));
        // Is there any damage?
        if percentDamage >= 25 as libc::c_int as libc::c_uint {
            if percentDamage >= 100 as libc::c_int as libc::c_uint {
                percentDamage = 99 as libc::c_int as UDWORD
            }
            //psDroid->emissionInterval = CALC_DROID_SMOKE_INTERVAL(percentDamage);
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
            //if(gameTime > (psDroid->lastEmission+psDroid->emissionInterval))
            if gameTime >
                   (*psDroid).lastEmission.wrapping_add(emissionInterval) {
                dv.x =
                    (*psDroid).x as libc::c_int +
                        (16 as libc::c_int - rand() % 32 as libc::c_int);
                dv.z =
                    (*psDroid).y as libc::c_int +
                        (16 as libc::c_int - rand() % 32 as libc::c_int);
                dv.y = (*psDroid).z as int32;
                dv.y += (*(*psDroid).sDisplay.imd).ymax * 2 as libc::c_int;
                addEffect(&mut dv, EFFECT_SMOKE, SMOKE_TYPE_DRIFTING_SMALL,
                          0 as libc::c_int, 0 as *mut iIMDShape,
                          0 as libc::c_int);
                // FIXFX		addExplosion(&dv,TYPE_EXPLOSION_SMOKE_CLOUD,NULL);
                (*psDroid).lastEmission = gameTime
            }
        }
    }
    processVisibility(psDroid as *mut BASE_OBJECT);
    // -----------------
	/* Are we a sensor droid or a command droid? */
    if (*psDroid).droidType as libc::c_uint ==
           DROID_SENSOR as libc::c_int as libc::c_uint ||
           (*psDroid).droidType as libc::c_uint ==
               DROID_COMMAND as libc::c_int as libc::c_uint {
        /* Nothing yet... */
        psBeingTargetted = 0 as *mut BASE_OBJECT;
        /* If we're attacking or sensing (observing), then... */
        if orderStateObj(psDroid, DORDER_ATTACK, &mut psBeingTargetted) != 0
               ||
               orderStateObj(psDroid, DORDER_OBSERVE, &mut psBeingTargetted)
                   != 0 {
            /* If it's a structure */
            if (*psBeingTargetted).type_0 as libc::c_uint ==
                   OBJ_STRUCTURE as libc::c_int as libc::c_uint {
                /* And it's your structure or your droid... */
                if (*(psBeingTargetted as *mut STRUCTURE)).player as
                       libc::c_uint == selectedPlayer ||
                       (*psDroid).player as libc::c_uint == selectedPlayer {
                    /* Highlight the structure in question */
                    (*(psBeingTargetted as *mut STRUCTURE)).targetted =
                        1 as libc::c_int as UBYTE
                }
            } else if (*psBeingTargetted).type_0 as libc::c_uint ==
                          OBJ_DROID as libc::c_int as libc::c_uint {
                // ffs AM
                /* And it's your  your droid... */
                if (*(psBeingTargetted as *mut DROID)).player as libc::c_uint
                       == selectedPlayer ||
                       (*psDroid).player as libc::c_uint == selectedPlayer {
                    (*(psBeingTargetted as *mut DROID)).bTargetted =
                        1 as libc::c_int
                }
            } else if (*psBeingTargetted).type_0 as libc::c_uint ==
                          OBJ_FEATURE as libc::c_int as libc::c_uint {
                if (*psDroid).player as libc::c_uint == selectedPlayer {
                    (*(psBeingTargetted as *mut FEATURE)).bTargetted =
                        1 as libc::c_int
                }
            }
        }
    }
    // -----------------
//	visTilesUpdate((BASE_OBJECT *)psDroid);
    /* Update the fire damage data */
    if (*psDroid).inFire & 0x1 as libc::c_int != 0 {
        /* Still in a fire, reset the fire flag to see if we get out this turn */
        (*psDroid).inFire = 0 as libc::c_int
    } else if (*psDroid).inFire & 0x2 as libc::c_int != 0 {
        if (*psDroid).burnStart.wrapping_add(10000 as libc::c_int as
                                                 libc::c_uint) < gameTime {
            /* The fire flag has not been set so we must be out of the fire */
            // stop burning
            (*psDroid).inFire = 0 as libc::c_int;
            (*psDroid).burnStart = 0 as libc::c_int as UDWORD;
            (*psDroid).burnDamage = 0 as libc::c_int as UDWORD
        } else {
            // do burn damage
            damageToDo =
                15 as libc::c_int *
                    (gameTime as SDWORD - (*psDroid).burnStart as SDWORD) /
                    1000 as libc::c_int;
            damageToDo -= (*psDroid).burnDamage as SDWORD;
            if damageToDo > 0 as libc::c_int {
                (*psDroid).burnDamage =
                    ((*psDroid).burnDamage as
                         libc::c_uint).wrapping_add(damageToDo as
                                                        libc::c_uint) as
                        UDWORD as UDWORD;
                //psDroid->damage(psDroid, damageToDo, WC_HEAT);
                droidDamage(psDroid, damageToDo as UDWORD,
                            WC_HEAT as libc::c_int as UDWORD,
                            WSC_FLAME as libc::c_int as UDWORD);
            }
        }
    } else if (*psDroid).burnStart != 0 as libc::c_int as libc::c_uint {
        // just left the fire
        (*psDroid).inFire |= 0x2 as libc::c_int;
        (*psDroid).burnStart = gameTime;
        (*psDroid).burnDamage = 0 as libc::c_int as UDWORD
    }
    droidUpdateRecoil(psDroid);
    // on the psx version, we do this in DisplayCompObj() in component.c
    calcDroidIllumination(psDroid);
    //check the resistance level of the droid
    if (*psDroid).id.wrapping_rem(50 as libc::c_int as libc::c_uint) ==
           frameGetFrameNumber().wrapping_rem(50 as libc::c_int as
                                                  libc::c_uint) {
        //zero resistance means not currently been attacked - ignore these
        if (*psDroid).resistance as libc::c_int != 0 &&
               ((*psDroid).resistance as libc::c_int) <
                   droidResistance(psDroid) as libc::c_int {
            //increase over time if low
            (*psDroid).resistance += 1
        }
    };
    //    ASSERT( psDroid->x != 0 && psDroid->y != 0,
//		"unitUpdate (end): Unit at (0,0)" );
}
//#ifndef PSX
//void droidUpdate(DROID *psDroid)
//{
//	droidUpd(psDroid);
//}
//#else
//void droidUpdate(DROID *psDroid)
//{
//	static DROID *psTmpDroid;
//	psTmpDroid = psDroid;
//
//	SetSpDCache();
//	droidUpd(psTmpDroid);
//	SetSpNormal();
//}
//#endif
// calculate the experience level of a droid
/*SDWORD droidCalcExp(DROID *psDroid)
{
	// figure out a to hit mod based on kills
	if (psDroid->numKills > 100)
	{
		return 6;
	}
	else if (psDroid->numKills > 50)
	{
		return 5;
	}
	else if (psDroid->numKills > 25)
	{
		return 4;
	}
	else if (psDroid->numKills > 10)
	{
		return 3;
	}
	else if (psDroid->numKills > 5)
	{
		return 2;
	}
	else if (psDroid->numKills > 1)
	{
		return 1;
	}

	return 0;
}*/
/* See if a droid is next to a structure */
unsafe extern "C" fn droidNextToStruct(mut psDroid: *mut DROID,
                                       mut psStruct: *mut BASE_OBJECT)
 -> BOOL {
    let mut minX: SDWORD = 0;
    let mut maxX: SDWORD = 0;
    let mut maxY: SDWORD = 0;
    let mut x: SDWORD = 0;
    let mut y: SDWORD = 0;
    minX =
        ((*psDroid).x as libc::c_int >> 7 as libc::c_int) - 1 as libc::c_int;
    y = ((*psDroid).y as libc::c_int >> 7 as libc::c_int) - 1 as libc::c_int;
    maxX = minX + 2 as libc::c_int;
    maxY = y + 2 as libc::c_int;
    if minX < 0 as libc::c_int { minX = 0 as libc::c_int }
    if maxX >= mapWidth as SDWORD { maxX = mapWidth as SDWORD }
    if y < 0 as libc::c_int { y = 0 as libc::c_int }
    if maxY >= mapHeight as SDWORD { maxY = mapHeight as SDWORD }
    while y <= maxY {
        x = minX;
        while x <= maxX {
            //psor			if (mapTile(x,y)->psObject == psStruct)
            if (*mapTile(x as UWORD as UDWORD,
                         y as UWORD as UDWORD)).tileInfoBits as libc::c_int &
                   0x1 as libc::c_int != 0 &&
                   getTileStructure(x as UDWORD, y as UDWORD) ==
                       psStruct as *mut STRUCTURE {
                return 1 as libc::c_int
            }
            x += 1
        }
        y += 1
    }
    return 0 as libc::c_int;
}
/* Set up a droid to build a foundation - returns true if successful */
#[no_mangle]
pub unsafe extern "C" fn droidStartFoundation(mut psDroid: *mut DROID)
 -> BOOL {
    //	SDWORD	height;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"unitStartFoundation: invalid unit pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"droid.c\x00" as *const u8 as *const libc::c_char,
              1637 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 21],
                                        &[libc::c_char; 21]>(b"droidStartFoundation\x00")).as_ptr(),
              b"PTRVALID(psDroid, sizeof(DROID))\x00" as *const u8 as
                  *const libc::c_char);
    };
    /* See if we are starting a new structure */
    if (*psDroid).order == DORDER_BUILD as libc::c_int ||
           (*psDroid).order == DORDER_LINEBUILD as libc::c_int {
        //buildStructure() calls this now so don't need it twice! AB 14/12/98
		//height = buildFoundation((STRUCTURE_STATS *)psDroid->psTarStats,
		//				psDroid->orderX,psDroid->orderY);
		//if (height >= 0)
        //psDroid->actionHeight = (UWORD)height;
            //psDroid->actionHeight = 0;  //not used here anymore
        (*psDroid).actionStarted = gameTime;
        (*psDroid).actionPoints = 0 as libc::c_int as UDWORD;
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn droidCheckBuildStillInProgress(mut psSample:
                                                            *mut AUDIO_SAMPLE)
 -> BOOL {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"unitCheckBuildStillInProgress: audio sample pointer invalid\n\x00"
                  as *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"droid.c\x00" as *const u8 as *const libc::c_char,
              1666 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 31],
                                        &[libc::c_char; 31]>(b"droidCheckBuildStillInProgress\x00")).as_ptr(),
              b"PTRVALID(psSample, sizeof(AUDIO_SAMPLE))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (*psSample).psObj.is_null() {
        return 0 as libc::c_int
    } else {
        psDroid = (*psSample).psObj as *mut DROID;
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"unitCheckBuildStillInProgress: unit pointer invalid\n\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"droid.c\x00" as *const u8 as *const libc::c_char,
                  1676 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 31],
                                            &[libc::c_char; 31]>(b"droidCheckBuildStillInProgress\x00")).as_ptr(),
                  b"PTRVALID(psDroid, sizeof(DROID))\x00" as *const u8 as
                      *const libc::c_char);
        };
    }
    if (*psDroid).died == 0 &&
           (*psDroid).action == DACTION_BUILD as libc::c_int {
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
}
unsafe extern "C" fn droidBuildStartAudioCallback(mut psSample:
                                                      *mut AUDIO_SAMPLE)
 -> BOOL {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"unitBuildStartAudioCallback: audio sample pointer invalid\n\x00"
                  as *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"droid.c\x00" as *const u8 as *const libc::c_char,
              1695 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 29],
                                        &[libc::c_char; 29]>(b"droidBuildStartAudioCallback\x00")).as_ptr(),
              b"PTRVALID(psSample, sizeof(AUDIO_SAMPLE))\x00" as *const u8 as
                  *const libc::c_char);
    };
    psDroid = (*psSample).psObj as *mut DROID;
    if !psDroid.is_null() {
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"unitBuildStartAudioCallback: unit pointer invalid\n\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"droid.c\x00" as *const u8 as *const libc::c_char,
                  1702 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 29],
                                            &[libc::c_char; 29]>(b"droidBuildStartAudioCallback\x00")).as_ptr(),
                  b"PTRVALID(psDroid, sizeof(DROID))\x00" as *const u8 as
                      *const libc::c_char);
        };
        if (*psDroid).visible[selectedPlayer as usize] != 0 {
            audio_PlayObjDynamicTrack(psDroid as *mut libc::c_void,
                                      ID_SOUND_CONSTRUCTION_LOOP as
                                          libc::c_int,
                                      Some(droidCheckBuildStillInProgress as
                                               unsafe extern "C" fn(_:
                                                                        *mut AUDIO_SAMPLE)
                                                   -> BOOL));
        }
    }
    return 1 as libc::c_int;
}
//psDroid->tileNumber = 0;
/* Set up a droid to build a structure - returns true if successful */
#[no_mangle]
pub unsafe extern "C" fn droidStartBuild(mut psDroid: *mut DROID) -> BOOL {
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    //FEATURE				*psCurr;
    let mut psStructStat: *mut STRUCTURE_STATS = 0 as *mut STRUCTURE_STATS;
    //MESSAGE				*psMessage;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"unitStartBuild: invalid unit pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"droid.c\x00" as *const u8 as *const libc::c_char,
              1724 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 16],
                                        &[libc::c_char; 16]>(b"droidStartBuild\x00")).as_ptr(),
              b"PTRVALID(psDroid, sizeof(DROID))\x00" as *const u8 as
                  *const libc::c_char);
    };
    /* See if we are starting a new structure */
    if (*psDroid).psTarget.is_null() &&
           ((*psDroid).order == DORDER_BUILD as libc::c_int ||
                (*psDroid).order == DORDER_LINEBUILD as libc::c_int) {
        //need to check structLimits have not been exceeded
        psStructStat = (*psDroid).psTarStats as *mut STRUCTURE_STATS;
        if (*asStructLimits[(*psDroid).player as
                                usize].offset(psStructStat.wrapping_offset_from(asStructureStats)
                                                  as libc::c_int as
                                                  isize)).currentQuantity as
               libc::c_int >=
               (*asStructLimits[(*psDroid).player as
                                    usize].offset(psStructStat.wrapping_offset_from(asStructureStats)
                                                      as libc::c_int as
                                                      isize)).limit as
                   libc::c_int {
            intBuildFinished(psDroid);
            return 0 as libc::c_int
        }
        //ok to build
        psStruct =
            buildStructure(psStructStat, (*psDroid).orderX as UDWORD,
                           (*psDroid).orderY as UDWORD,
                           (*psDroid).player as UDWORD, 0 as libc::c_int);
        if psStruct.is_null() {
            intBuildFinished(psDroid);
            return 0 as libc::c_int
        }
        //add one to current quantity for this player
        let ref mut fresh0 =
            (*asStructLimits[(*psDroid).player as
                                 usize].offset(psStructStat.wrapping_offset_from(asStructureStats)
                                                   as libc::c_int as
                                                   isize)).currentQuantity;
        *fresh0 = (*fresh0).wrapping_add(1);
        //The following is done in buildStructure now - AB 3/9/98
		//if resource extractor - need to remove oil feature and prox Msg
		/*if (psStruct->pStructureType->type == REF_RESOURCE_EXTRACTOR)
		{
			//find the resource at this point
			for (psCurr = apsFeatureLists[0]; psCurr != NULL; psCurr = psCurr->psNext)
			{
				if (psCurr->psStats->subType == FEAT_OIL_RESOURCE)
				{
					if ((psCurr->x == psStruct->x) AND (psCurr->y == psStruct->y))
					{
						//see if there is a proximity message at this location
						psMessage = findMessage(psCurr, MSG_PROXIMITY, psStruct->player);
						if (psMessage)
						{
							removeMessage(psMessage, psStruct->player);
						}
						//remove it from the map
						removeFeature(psCurr);
						//set the map to hold the resource extractor again
						SET_TILE_STRUCTURE(mapTile(psCurr->x >> TILE_SHIFT,
							psCurr->y >> TILE_SHIFT));
					}
				}
			}
		}*/
        //commented out for demo - 2/1/98
		//ASSERT( droidNextToStruct(psDroid, (BASE_OBJECT *)psStruct),
		//	"droidStartBuild: did not build structure next to droid" );
        if bMultiPlayer != 0 {
            //			psStruct->id = psDroid->buildingId;// Change the id to the constructor droids required id.
            if myResponsibility((*psDroid).player as UDWORD) != 0 {
                sendBuildStarted(psStruct, psDroid);
            }
        }
    } else {
        /* Check the structure is still there to build (joining a partially built struct) */
        psStruct = (*psDroid).psTarget as *mut STRUCTURE;
        if droidNextToStruct(psDroid, psStruct as *mut BASE_OBJECT) == 0 {
            /* Nope - stop building */
            debug(LOG_NEVER,
                  b"unitStartBuild: not next to structure\n\x00" as *const u8
                      as *const libc::c_char);
        }
    }
    //check structure not already built
    if (*psStruct).status as libc::c_int != SS_BUILT as libc::c_int {
        (*psDroid).actionStarted = gameTime;
        (*psDroid).actionPoints = 0 as libc::c_int as UDWORD;
        (*psDroid).psTarget = psStruct as *mut BASE_OBJECT;
        (*psDroid).psActionTarget = psStruct as *mut BASE_OBJECT
    }
    if (*psStruct).visible[selectedPlayer as usize] != 0 {
        audio_PlayObjStaticTrackCallback(psDroid as *mut libc::c_void,
                                         ID_SOUND_CONSTRUCTION_START as
                                             libc::c_int,
                                         Some(droidBuildStartAudioCallback as
                                                  unsafe extern "C" fn(_:
                                                                           *mut AUDIO_SAMPLE)
                                                      -> BOOL));
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn droidAddWeldSound(mut iVecEffect: iVector) {
    let mut iAudioID: SDWORD = 0;
    iAudioID =
        ID_SOUND_CONSTRUCTION_1 as libc::c_int + rand() % 4 as libc::c_int;
    audio_PlayStaticTrack(iVecEffect.x, iVecEffect.z, iAudioID);
}
unsafe extern "C" fn addConstructorEffect(mut psStruct: *mut STRUCTURE) {
    let mut widthRange: UDWORD = 0;
    let mut breadthRange: UDWORD = 0;
    let mut temp: iVector = iVector{x: 0, y: 0, z: 0,};
    // We definitly still want this called on the PSX.
    //FIXME
    if rand() % 10 as libc::c_int == 0 as libc::c_int &&
           (*psStruct).visible[selectedPlayer as usize] as libc::c_int != 0 {
        /* This needs fixing - it's an arse effect! */
        widthRange =
            (*(*psStruct).pStructureType).baseWidth.wrapping_mul(128 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint).wrapping_div(4
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    libc::c_uint);
        breadthRange =
            (*(*psStruct).pStructureType).baseBreadth.wrapping_mul(128 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint).wrapping_div(4
                                                                                                      as
                                                                                                      libc::c_int
                                                                                                      as
                                                                                                      libc::c_uint);
        temp.x =
            ((*psStruct).x as
                 libc::c_uint).wrapping_add((rand() as
                                                 libc::c_uint).wrapping_rem((2
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_uint).wrapping_mul(widthRange)).wrapping_sub(widthRange))
                as int32;
        temp.y =
            map_TileHeight(((*psStruct).x as libc::c_int >> 7 as libc::c_int)
                               as UDWORD,
                           ((*psStruct).y as libc::c_int >> 7 as libc::c_int)
                               as UDWORD) as libc::c_int +
                (*(*psStruct).sDisplay.imd).ymax / 6 as libc::c_int;
        temp.z =
            ((*psStruct).y as
                 libc::c_uint).wrapping_add((rand() as
                                                 libc::c_uint).wrapping_rem((2
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_uint).wrapping_mul(breadthRange)).wrapping_sub(breadthRange))
                as int32;
        //FIXFX				addExplosion(&temp,TYPE_EXPLOSION_SMOKE_CLOUD,NULL);
        if rand() % 2 as libc::c_int != 0 { droidAddWeldSound(temp); }
    };
}
/* Update a construction droid while it is building
   returns TRUE while building continues */
#[no_mangle]
pub unsafe extern "C" fn droidUpdateBuild(mut psDroid: *mut DROID) -> BOOL {
    //	UDWORD		widthRange,breadthRange;
//	iVector		temp;
    let mut pointsToAdd: UDWORD = 0; //, powerPercent, buildPercent;
    let mut constructPoints: UDWORD = 0;
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    //UDWORD		mapX, mapY, i, j;
	//UBYTE		prevScale, currScale, current = 0, prev = 0;
    if (*psDroid).action == DACTION_BUILD as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"unitUpdateBuild: unit is not building\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (*psDroid).action == DACTION_BUILD as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"droid.c\x00" as *const u8 as *const libc::c_char,
              1873 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 17],
                                        &[libc::c_char; 17]>(b"droidUpdateBuild\x00")).as_ptr(),
              b"psDroid->action == DACTION_BUILD\x00" as *const u8 as
                  *const libc::c_char);
    };
    psStruct = (*psDroid).psTarget as *mut STRUCTURE;
    if (*psStruct).type_0 as libc::c_uint ==
           OBJ_STRUCTURE as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"unitUpdateBuild: target is not a structure\x00" as *const u8
                  as *const libc::c_char);
    };
    if (*psStruct).type_0 as libc::c_uint ==
           OBJ_STRUCTURE as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"droid.c\x00" as *const u8 as *const libc::c_char,
              1876 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 17],
                                        &[libc::c_char; 17]>(b"droidUpdateBuild\x00")).as_ptr(),
              b"psStruct->type == OBJ_STRUCTURE\x00" as *const u8 as
                  *const libc::c_char);
    };
    if ((*psDroid).asBits[COMP_CONSTRUCT as libc::c_int as usize].nStat as
            libc::c_uint) < numConstructStats {
    } else {
        debug(LOG_ERROR,
              b"unitUpdateBuild: Invalid construct pointer for unit\x00" as
                  *const u8 as *const libc::c_char);
    };
    if ((*psDroid).asBits[COMP_CONSTRUCT as libc::c_int as usize].nStat as
            libc::c_uint) < numConstructStats {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"droid.c\x00" as *const u8 as *const libc::c_char,
              1878 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 17],
                                        &[libc::c_char; 17]>(b"droidUpdateBuild\x00")).as_ptr(),
              b"psDroid->asBits[COMP_CONSTRUCT].nStat < numConstructStats\x00"
                  as *const u8 as *const libc::c_char);
    };
    /* Check the structure is still there to build */
	// FIXME - need to ensure construction droids move nearer to structures before building
/*	if (!droidNextToStruct(psDroid, (BASE_OBJECT *)psStruct))
	{
		// Nope - stop building
		return FALSE;
	}*/
    //first check the structure hasn't been completed by another droid
    if (*psStruct).status as libc::c_int == SS_BUILT as libc::c_int {
        //update the interface
        intBuildFinished(psDroid);
        return 0 as libc::c_int
    }
    //for now wait until have enough power to build
    //if (psStruct->currentPowerAccrued < (SWORD)psStruct->pStructureType->powerToBuild)
    if ((*psStruct).currentPowerAccrued as libc::c_int) <
           structPowerToBuild(psStruct) as SWORD as libc::c_int {
        (*psDroid).actionStarted = gameTime;
        return 1 as libc::c_int
    }
    //the amount of power accrued limits how much of the work can have been done
    /*if (psStruct->pStructureType->powerToBuild)
    {
        powerPercent = 100 * psStruct->currentPowerAccrued / psStruct->
            pStructureType->powerToBuild;
    }
    else
    {
        powerPercent = 100;
    }
    buildPercent = 100 * psStruct->currentBuildPts / psStruct->
        pStructureType->buildPoints;

    //check if enough power to do more
    if (buildPercent > powerPercent)
    {
        //reset the actionStarted time and actionPoints added so the correct
        //amount of points are added when there is more power
        psDroid->actionStarted = gameTime;
        psDroid->actionPoints = 0;
        return TRUE;
    }*/
    //constructPoints = (asConstructStats + psDroid->asBits[COMP_CONSTRUCT].nStat)->
	//	constructPoints;
    constructPoints =
        constructorPoints(asConstructStats.offset((*psDroid).asBits[COMP_CONSTRUCT
                                                                        as
                                                                        libc::c_int
                                                                        as
                                                                        usize].nStat
                                                      as libc::c_int as
                                                      isize),
                          (*psDroid).player);
    pointsToAdd =
        constructPoints.wrapping_mul(gameTime.wrapping_sub((*psDroid).actionStarted)).wrapping_div(1000
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_uint);
    //these two lines needs to be removed if illumination is put pack in
    (*psStruct).currentBuildPts =
        ((*psStruct).currentBuildPts as
             libc::c_uint).wrapping_add(pointsToAdd).wrapping_sub((*psDroid).actionPoints)
            as SWORD;
    //psStruct->heightScale = (MAKEFRACT(psStruct->currentBuildPts))/(psStruct->pStructureType->buildPoints);
	//ILLUMINATION ISN'T BEING DONE ANYMORE
	/*
	//reserve the previous value
	prevScale= (UBYTE) MAKEINT(psStruct->heightScale*100);
	prev = (UBYTE)(prevScale / (UBYTE)10);

	psStruct->currentBuildPts += (pointsToAdd - psDroid->actionPoints);
	psStruct->heightScale = (MAKEFRACT(psStruct->currentBuildPts))/(psStruct->pStructureType->buildPoints);

	currScale = (UBYTE) MAKEINT(psStruct->heightScale * 100);
	current = (UBYTE)(currScale / (UBYTE)10);

	if (current != prev)
	{
		prev *= 10;
		current *= 10;
	}

	if (current != prev)
	{
#ifdef PSX
		FRACT divisor1,divisor2;
		divisor1= (MAKEFRACT(FOUNDATION_ILLUMIN + prev) - ((MAKEFRACT(FOUNDATION_ILLUMIN * prev))/100))/100;
		divisor2= (MAKEFRACT(FOUNDATION_ILLUMIN + current) - ((MAKEFRACT(FOUNDATION_ILLUMIN * current))/100))/100;
#endif
		//set the illumination of the tiles back to original value as building is completed

		mapX = (psStruct->x - psStruct->pStructureType->baseWidth *
			TILE_UNITS / 2) >> TILE_SHIFT;
		mapY = (psStruct->y - psStruct->pStructureType->baseBreadth *
			TILE_UNITS / 2) >> TILE_SHIFT;



		for (i = 0; i < psStruct->pStructureType->baseWidth+1; i++)
		{
			for (j = 0; j < psStruct->pStructureType->baseBreadth+1; j++)
			{
#ifndef PSX
				FRACT	divisor,illumin, currentIllumin;

				divisor = (FOUNDATION_ILLUMIN + prev -
					(FOUNDATION_ILLUMIN * prev)/(FRACT)100) / (FRACT)100;
				//work out what the initial value was before modifier was applied
				currentIllumin = mapTile(mapX+i, mapY+j)->illumination;
				illumin = currentIllumin / divisor;
				divisor = ( FOUNDATION_ILLUMIN+current-(FOUNDATION_ILLUMIN*current)/(FRACT)100 )
																			/ (FRACT)100;
				illumin = illumin * divisor;
				mapTile(mapX+i, mapY+j)->illumination = (UBYTE)illumin;

#else
				FRACT	illumin;
				MAPTILE *map;

				map=mapTile(mapX+i,mapY+j);
				illumin = FRACTdiv_1(MAKEFRACT(map->illumination),divisor1);	// Calculate the old illumination before we built
				map->illumination = (UBYTE)MAKEINT(FRACTmul_1(illumin,divisor2)); // new calculate the new adjust illumination
#endif
			}
		}
		prev = current;
	}
	*/
    //store the amount just added
    (*psDroid).actionPoints = pointsToAdd;
    //check if structure is built
    if (*psStruct).currentBuildPts as libc::c_int >
           (*(*psStruct).pStructureType).buildPoints as SDWORD {
        (*psStruct).currentBuildPts =
            (*(*psStruct).pStructureType).buildPoints as SWORD;
        (*psStruct).status = SS_BUILT as libc::c_int as UBYTE;
        buildingComplete(psStruct);
        // update the power if necessary
		/*if (psStruct->pStructureType->type == REF_POWER_GEN)
		{
			capacityUpdate(psStruct);
		}
		else if (psStruct->pStructureType->type == REF_RESOURCE_EXTRACTOR OR
			psStruct->pStructureType->type == REF_HQ)
		{
			extractedPowerUpdate(psStruct);
		}*/
        intBuildFinished(psDroid);
        /* This is done in buildingComplete now - AB 14/09/98
		GJ HACK! - add anim to deriks
		if ( psStruct->pStructureType->type == REF_RESOURCE_EXTRACTOR &&
				psStruct->psCurAnim == NULL )
		{
			psStruct->psCurAnim = animObj_Add( psStruct, ID_ANIM_DERIK, 0, 0 );
		}*/
        if bMultiPlayer != 0 &&
               myResponsibility((*psStruct).player as UDWORD) != 0 {
            SendBuildFinished(psStruct);
        }
        //only play the sound if selected player
        if (*psStruct).player as libc::c_uint == selectedPlayer &&
               ((*psDroid).order != DORDER_LINEBUILD as libc::c_int ||
                    (*psDroid).orderX as libc::c_int >> 7 as libc::c_int ==
                        (*psDroid).orderX2 as libc::c_int >> 7 as libc::c_int
                        &&
                        (*psDroid).orderY as libc::c_int >> 7 as libc::c_int
                            ==
                            (*psDroid).orderY2 as libc::c_int >>
                                7 as libc::c_int) {
            audio_QueueTrackPos(ID_SOUND_STRUCTURE_COMPLETED as libc::c_int,
                                (*psStruct).x as SDWORD,
                                (*psStruct).y as SDWORD,
                                (*psStruct).z as SDWORD);
            intRefreshScreen();
            // update any open interface bars.
        }
        structureCompletedCallback((*psStruct).pStructureType);
        audio_StopObjTrack(psDroid as *mut libc::c_void,
                           ID_SOUND_CONSTRUCTION_LOOP as libc::c_int);
        return 0 as libc::c_int
    } else {
        addConstructorEffect(psStruct);
        //		//FIXME
//		if((ONEINTEN) AND (psStruct->visible[selectedPlayer]))
//			{
//				/* This needs fixing - it's an arse effect! */
//				widthRange = (psStruct->pStructureType->baseWidth*TILE_UNITS)/3;
//				breadthRange = (psStruct->pStructureType->baseBreadth*TILE_UNITS)/3;
//				temp.x = psStruct->x+((rand()%(2*widthRange)) - widthRange);
//				temp.y = map_TileHeight(psStruct->x>>TILE_SHIFT, psStruct->y>>TILE_SHIFT)+
//								((psStruct->sDisplay.imd->ymax)/2);
//				temp.z = psStruct->y+((rand()%(2*breadthRange)) - breadthRange);
// //FIXFX				addExplosion(&temp,TYPE_EXPLOSION_SMOKE_CLOUD,NULL);
//				if(rand()%2)
//				{
//					addEffect(&temp,EFFECT_EXPLOSION,EXPLOSION_TYPE_PLASMA,FALSE,NULL,0);
//				}
//				else
//				{
//					addEffect(&temp,EFFECT_CONSTRUCTION,CONSTRUCTION_TYPE_DRIFTING,FALSE,NULL,0);
//				}
//			}
    }
    return 1 as libc::c_int;
}
//NEW VERSION WHEREBY THE GROUND IS LEVELLED IN ONE GO
/* Update a construction droid while it is building a
   foundation. Returns TRUE whilst foundation continues */
#[no_mangle]
pub unsafe extern "C" fn droidUpdateFoundation(mut psDroid: *mut DROID)
 -> BOOL {
    //UBYTE				width, breadth;
//	STRUCTURE_STATS		*psStructStats = (STRUCTURE_STATS *)psDroid->psTarStats;
	//UDWORD				x;
	//UDWORD				y;
    if (*psDroid).action == DACTION_BUILD_FOUNDATION as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"unitUpdateFoundation: unit is not building foundation\x00" as
                  *const u8 as *const libc::c_char);
    };
    if (*psDroid).action == DACTION_BUILD_FOUNDATION as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"droid.c\x00" as *const u8 as *const libc::c_char,
              2094 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 22],
                                        &[libc::c_char; 22]>(b"droidUpdateFoundation\x00")).as_ptr(),
              b"psDroid->action == DACTION_BUILD_FOUNDATION\x00" as *const u8
                  as *const libc::c_char);
    };
    /*x = (SDWORD)psDroid->orderX - (SDWORD)(psStructStats->baseWidth * TILE_UNITS)/2;
	y = (SDWORD)psDroid->orderY - (SDWORD)(psStructStats->baseBreadth * TILE_UNITS)/2;

	x = (x >> TILE_SHIFT);
	y = (y >> TILE_SHIFT);

	//find a section that is not at the actionHeight
	for (breadth = 0; breadth < (UBYTE)(psStructStats->baseBreadth + 1); breadth++)
	{
		for (width = 0; width < (UBYTE)(psStructStats->baseWidth + 1); width++)
		{
			//set the tile to the required foundation height
			setTileHeight(x + width, y + breadth, psDroid->actionHeight);
			// We need to raise features on raised tiles to the new height
			if(TILE_HAS_FEATURE(mapTile(x+width,y+breadth)))
			{
				getTileFeature(x+width, y+breadth)->z =
					psDroid->actionHeight;
			}
		}
	}
	//all must be at actionHeight
	return FALSE;*/
    //don't do this anymore since happens in one go and is called by buildStructure - see buildFlatten()!!
    return 0 as libc::c_int;
}
//OLD VERSION WHEREBY THE GROUND IS LOWERED/RAISED TILE BY TILE
/* Update a construction droid while it is building a
   foundation. Returns TRUE whilst foundation continues */
/*BOOL droidUpdateFoundation(DROID *psDroid)
{
	UBYTE				width, breadth, buildSpeed = 160, illumin;
	STRUCTURE_STATS		*psStructStats = (STRUCTURE_STATS *)psDroid->psTarStats;
	UDWORD				x = psDroid->orderX >> TILE_SHIFT;
	UDWORD				y = psDroid->orderY >> TILE_SHIFT;
	UDWORD				pointsToAdd, height;
	SDWORD				newHeight;

	ASSERT( psDroid->action == DACTION_BUILD_FOUNDATION,
		"droidUpdateFoundation: droid is not building foundation" );

	//we want this to happen almost immediately...so add lots each cycle
	//pointsToAdd = 40 * (gameTime - psDroid->actionStarted) / GAME_TICKS_PER_SEC;
	pointsToAdd = gameTime - psDroid->actionStarted;

	//if time to raise a new section
	if (pointsToAdd > psDroid->actionPoints)
	{
		//find a section that is not at the actionHeight
		breadth = (UBYTE)(psDroid->tileNumber / (psStructStats->baseBreadth + 1));
		for (; breadth < (UBYTE)(psStructStats->baseBreadth + 1); breadth++)
		{
			width = (UBYTE)(psDroid->tileNumber % (psStructStats->baseWidth + 1));
			for (; width < (UBYTE)(psStructStats->baseWidth + 1); width++)
			{

				//set the illumination of the tile to FOUNDATION_ILLUM% darker
				illumin = mapTile(x + width, y + breadth)->illumination;
				DBPRINTF(("Start illum=%d\n",illumin);
				illumin = (UBYTE)((FOUNDATION_ILLUMIN * illumin) / 100);
				mapTile(x + width, y + breadth)->illumination = illumin;

				height = map_TileHeight(x + width, y + breadth);
				if (height < psDroid->actionHeight)
				{
					newHeight = height + (buildSpeed * (pointsToAdd -
						psDroid->actionPoints));
					if (newHeight > (SDWORD)psDroid->actionHeight)
					{
						//got to the right height
						newHeight = psDroid->actionHeight;
						//psDroid->tileNumber++;
						moveToNewTile(psDroid);
					}
					setTileHeight(x + width, y + breadth, newHeight);
					// We need to raise features on raised tiles to the new height
					if(TILE_HAS_FEATURE(mapTile(x+width,y+breadth)))
					{
						getTileFeature(x+width,y+breadth)->z = newHeight;
					}
					psDroid->actionPoints = pointsToAdd;
					return TRUE;
				}
				else if (height > psDroid->actionHeight)
				{
					newHeight = (SDWORD)(height - (buildSpeed * (pointsToAdd -
						psDroid->actionPoints)));
					if (newHeight < (SDWORD)psDroid->actionHeight)
					{
						//got to the right height
						newHeight = psDroid->actionHeight;
						//psDroid->tileNumber++;
						moveToNewTile(psDroid);
					}
					setTileHeight(x + width, y + breadth, newHeight);
					if(TILE_HAS_FEATURE(mapTile(x+width,y+breadth)))
					{
						getTileFeature(x+width,y+breadth)->z = newHeight;
					}
					psDroid->actionPoints = pointsToAdd;
					return TRUE;
				}
				else
				{
					//already at the right height
					//psDroid->tileNumber++;
					moveToNewTile(psDroid);
				}
			}
		}
		//all must be at actionHeight
		return FALSE;
	}
	//still building
	return TRUE;
}*/
/*time to move to a new location (when building foundation) */
/*void moveToNewTile(DROID *psDroid)
{
	UBYTE				breadth, width;
	UDWORD				newX, newY;
	STRUCTURE_STATS		*psStructStats = (STRUCTURE_STATS *)psDroid->psTarStats;
	UDWORD				x = psDroid->orderX >> TILE_SHIFT;
	UDWORD				y = psDroid->orderY >> TILE_SHIFT;

	//increment the tile index
	psDroid->tileNumber++;

	//determine the tile now working on
	breadth = (UBYTE)(psDroid->tileNumber / (psStructStats->baseBreadth + 1));
	width = (UBYTE)(psDroid->tileNumber % (psStructStats->baseWidth + 1));
	newX = x + width;
	newY = y + breadth;

	//find a free location near the tile
	if (!pickATile(&newX, &newY,LOOK_FOR_EMPTY_TILE))
	{
		ASSERT( FALSE, "moveToNewTile: Unable to find a free location" );
	}

	//order the droid to move
	psDroid->action = DACTION_FOUNDATION_WANDER;
	newX = newX << TILE_SHIFT;
	newY = newY << TILE_SHIFT;
	psDroid->actionX = newX;
	psDroid->actionY = newY;
	moveDroidTo(psDroid, newX, newY);
}*/
#[no_mangle]
pub unsafe extern "C" fn droidStartDemolishing(mut psDroid: *mut DROID)
 -> BOOL {
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    if (*psDroid).order == DORDER_DEMOLISH as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"unitStartDemolishing: unit is not demolishing\x00" as
                  *const u8 as *const libc::c_char);
    };
    if (*psDroid).order == DORDER_DEMOLISH as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"droid.c\x00" as *const u8 as *const libc::c_char,
              2253 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 22],
                                        &[libc::c_char; 22]>(b"droidStartDemolishing\x00")).as_ptr(),
              b"psDroid->order == DORDER_DEMOLISH\x00" as *const u8 as
                  *const libc::c_char);
    };
    psStruct = (*psDroid).psTarget as *mut STRUCTURE;
    if (*psStruct).type_0 as libc::c_uint ==
           OBJ_STRUCTURE as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"unitStartDemolishing: target is not a structure\x00" as
                  *const u8 as *const libc::c_char);
    };
    if (*psStruct).type_0 as libc::c_uint ==
           OBJ_STRUCTURE as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"droid.c\x00" as *const u8 as *const libc::c_char,
              2256 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 22],
                                        &[libc::c_char; 22]>(b"droidStartDemolishing\x00")).as_ptr(),
              b"psStruct->type == OBJ_STRUCTURE\x00" as *const u8 as
                  *const libc::c_char);
    };
    (*psDroid).actionStarted = gameTime;
    (*psDroid).actionPoints = 0 as libc::c_int as UDWORD;
    /* init build points Don't - could be partially demolished*/
	//psStruct->currentBuildPts = psStruct->pStructureType->buildPoints;
    (*psStruct).status = SS_BEING_DEMOLISHED as libc::c_int as UBYTE;
    // Set height scale for demolishing
	//psStruct->heightScale = (MAKEFRACT(psStruct->currentBuildPts))/(
	//	psStruct->pStructureType->buildPoints);
    //if start to demolish a power gen need to inform the derricks
    if (*(*psStruct).pStructureType).type_0 ==
           REF_POWER_GEN as libc::c_int as libc::c_uint {
        releasePowerGen(psStruct);
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn droidUpdateDemolishing(mut psDroid: *mut DROID)
 -> BOOL {
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut pointsToAdd: UDWORD = 0;
    let mut constructPoints: UDWORD = 0;
    if (*psDroid).action == DACTION_DEMOLISH as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"unitUpdateDemolishing: unit is not demolishing\x00" as
                  *const u8 as *const libc::c_char);
    };
    if (*psDroid).action == DACTION_DEMOLISH as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"droid.c\x00" as *const u8 as *const libc::c_char,
              2284 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 23],
                                        &[libc::c_char; 23]>(b"droidUpdateDemolishing\x00")).as_ptr(),
              b"psDroid->action == DACTION_DEMOLISH\x00" as *const u8 as
                  *const libc::c_char);
    };
    psStruct = (*psDroid).psTarget as *mut STRUCTURE;
    if (*psStruct).type_0 as libc::c_uint ==
           OBJ_STRUCTURE as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"unitUpdateDemolishing: target is not a structure\x00" as
                  *const u8 as *const libc::c_char);
    };
    if (*psStruct).type_0 as libc::c_uint ==
           OBJ_STRUCTURE as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"droid.c\x00" as *const u8 as *const libc::c_char,
              2287 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 23],
                                        &[libc::c_char; 23]>(b"droidUpdateDemolishing\x00")).as_ptr(),
              b"psStruct->type == OBJ_STRUCTURE\x00" as *const u8 as
                  *const libc::c_char);
    };
    //constructPoints = (asConstructStats + psDroid->asBits[COMP_CONSTRUCT].nStat)->
	//	constructPoints;
    constructPoints =
        constructorPoints(asConstructStats.offset((*psDroid).asBits[COMP_CONSTRUCT
                                                                        as
                                                                        libc::c_int
                                                                        as
                                                                        usize].nStat
                                                      as libc::c_int as
                                                      isize),
                          (*psDroid).player);
    pointsToAdd =
        constructPoints.wrapping_mul(gameTime.wrapping_sub((*psDroid).actionStarted)).wrapping_div(1000
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_uint);
    (*psStruct).currentBuildPts =
        ((*psStruct).currentBuildPts as
             libc::c_uint).wrapping_sub(pointsToAdd).wrapping_sub((*psDroid).actionPoints)
            as SWORD;
    //psStruct->heightScale = (MAKEFRACT(psStruct->currentBuildPts))/(psStruct->pStructureType->buildPoints);
    //store the amount just subtracted
    (*psDroid).actionPoints = pointsToAdd;
    /* check if structure is demolished */
    if (*psStruct).currentBuildPts as libc::c_int <= 0 as libc::c_int {
        if bMultiPlayer != 0 { SendDemolishFinished(psStruct, psDroid); }
        if (*(*psStruct).pStructureType).type_0 ==
               REF_POWER_GEN as libc::c_int as libc::c_uint {
            //can't assume it was completely built!!
			// put back all the power
			/*addPower(psStruct->player, structPowerToBuild(psStruct));
            //if it had a module attached, need to add the power for the base struct as well
            if (((POWER_GEN *)psStruct->pFunctionality)->capacity)
            {
                addPower(psStruct->player, psStruct->pStructureType->powerToBuild);
            }*/
            //if had module attached - the base must have been completely built
            if (*((*psStruct).pFunctionality as *mut POWER_GEN)).capacity != 0
               {
                //so add the power required to build the base struct
                addPower((*psStruct).player as UDWORD,
                         (*(*psStruct).pStructureType).powerToBuild);
            }
            //add the currentAccruedPower since this may or may not be all required
            addPower((*psStruct).player as UDWORD,
                     (*psStruct).currentPowerAccrued as UDWORD);
        } else {
            /* CAN'T assume completely built
			//put back half the power required to build this structure (=power to build)
			addPower(psStruct->player, structPowerToBuild(psStruct) / 2);
            //if it had a module attached, need to add the power for the base struct as well
            if (StructIsFactory(psStruct))
            {
                if (((FACTORY *)psStruct->pFunctionality)->capacity)
                {
                    addPower(psStruct->player, psStruct->pStructureType->
                        powerToBuild / 2);
                }
            }
            else if (psStruct->pStructureType->type == REF_RESEARCH)
            {
                if (((RESEARCH_FACILITY *)psStruct->pFunctionality)->capacity)
                {
                    addPower(psStruct->player, psStruct->pStructureType->
                        powerToBuild / 2);
                }
            }*/
            //if it had a module attached, need to add the power for the base struct as well
            if StructIsFactory(psStruct) != 0 {
                if (*((*psStruct).pFunctionality as *mut FACTORY)).capacity !=
                       0 {
                    //add half power for base struct
                    addPower((*psStruct).player as UDWORD,
                             (*(*psStruct).pStructureType).powerToBuild.wrapping_div(2
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_uint));
                    //if large factory - add half power for one upgrade
                    if (*((*psStruct).pFunctionality as
                              *mut FACTORY)).capacity as libc::c_int >
                           SIZE_MEDIUM as libc::c_int {
                        addPower((*psStruct).player as UDWORD,
                                 structPowerToBuild(psStruct).wrapping_div(2
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_uint));
                    }
                }
            } else if (*(*psStruct).pStructureType).type_0 ==
                          REF_RESEARCH as libc::c_int as libc::c_uint {
                if (*((*psStruct).pFunctionality as
                          *mut RESEARCH_FACILITY)).capacity != 0 {
                    //add half power for base struct
                    addPower((*psStruct).player as UDWORD,
                             (*(*psStruct).pStructureType).powerToBuild.wrapping_div(2
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_uint));
                }
            }
            //add currentAccrued for the current layer of the structure
            addPower((*psStruct).player as UDWORD,
                     ((*psStruct).currentPowerAccrued as libc::c_int /
                          2 as libc::c_int) as UDWORD);
        }
        /* remove structure and foundation */
        removeStruct(psStruct, 1 as libc::c_int);
        /* reset target stats*/
        (*psDroid).psTarStats = 0 as *mut _base_stats;
        return 0 as libc::c_int
    } else { addConstructorEffect(psStruct); }
    return 1 as libc::c_int;
}
/* Set up a droid to clear a wrecked building feature - returns true if successful */
#[no_mangle]
pub unsafe extern "C" fn droidStartClearing(mut psDroid: *mut DROID) -> BOOL {
    let mut psFeature: *mut FEATURE = 0 as *mut FEATURE;
    if (*psDroid).order == DORDER_CLEARWRECK as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"unitStartClearing: unit is not clearing wreckage\x00" as
                  *const u8 as *const libc::c_char);
    };
    if (*psDroid).order == DORDER_CLEARWRECK as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"droid.c\x00" as *const u8 as *const libc::c_char,
              2404 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 19],
                                        &[libc::c_char; 19]>(b"droidStartClearing\x00")).as_ptr(),
              b"psDroid->order == DORDER_CLEARWRECK\x00" as *const u8 as
                  *const libc::c_char);
    };
    psFeature = (*psDroid).psTarget as *mut FEATURE;
    if (*psFeature).type_0 as libc::c_uint ==
           OBJ_FEATURE as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"unitStartClearing: target is not a feature\x00" as *const u8
                  as *const libc::c_char);
    };
    if (*psFeature).type_0 as libc::c_uint ==
           OBJ_FEATURE as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"droid.c\x00" as *const u8 as *const libc::c_char,
              2407 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 19],
                                        &[libc::c_char; 19]>(b"droidStartClearing\x00")).as_ptr(),
              b"psFeature->type == OBJ_FEATURE\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (*(*psFeature).psStats).subType as libc::c_uint ==
           FEAT_BUILD_WRECK as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"unitStartClearing: feature is not a wrecked building\x00" as
                  *const u8 as *const libc::c_char);
    };
    if (*(*psFeature).psStats).subType as libc::c_uint ==
           FEAT_BUILD_WRECK as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"droid.c\x00" as *const u8 as *const libc::c_char,
              2409 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 19],
                                        &[libc::c_char; 19]>(b"droidStartClearing\x00")).as_ptr(),
              b"psFeature->psStats->subType == FEAT_BUILD_WRECK\x00" as
                  *const u8 as *const libc::c_char);
    };
    (*psDroid).actionStarted = gameTime;
    (*psDroid).actionPoints = 0 as libc::c_int as UDWORD;
    return 1 as libc::c_int;
}
/* Update a construction droid while it is clearing
   returns TRUE while continues */
#[no_mangle]
pub unsafe extern "C" fn droidUpdateClearing(mut psDroid: *mut DROID)
 -> BOOL {
    let mut psFeature: *mut FEATURE = 0 as *mut FEATURE;
    let mut pointsToAdd: UDWORD = 0;
    let mut constructPoints: UDWORD = 0;
    if (*psDroid).action == DACTION_CLEARWRECK as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"unitUpdateClearing: unit is not clearing wreckage\x00" as
                  *const u8 as *const libc::c_char);
    };
    if (*psDroid).action == DACTION_CLEARWRECK as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"droid.c\x00" as *const u8 as *const libc::c_char,
              2425 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 20],
                                        &[libc::c_char; 20]>(b"droidUpdateClearing\x00")).as_ptr(),
              b"psDroid->action == DACTION_CLEARWRECK\x00" as *const u8 as
                  *const libc::c_char);
    };
    psFeature = (*psDroid).psTarget as *mut FEATURE;
    if (*psFeature).type_0 as libc::c_uint ==
           OBJ_FEATURE as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"unitStartClearing: target is not a feature\x00" as *const u8
                  as *const libc::c_char);
    };
    if (*psFeature).type_0 as libc::c_uint ==
           OBJ_FEATURE as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"droid.c\x00" as *const u8 as *const libc::c_char,
              2428 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 20],
                                        &[libc::c_char; 20]>(b"droidUpdateClearing\x00")).as_ptr(),
              b"psFeature->type == OBJ_FEATURE\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (*(*psFeature).psStats).subType as libc::c_uint ==
           FEAT_BUILD_WRECK as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"unitStartClearing: feature is not a wrecked building\x00" as
                  *const u8 as *const libc::c_char);
    };
    if (*(*psFeature).psStats).subType as libc::c_uint ==
           FEAT_BUILD_WRECK as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"droid.c\x00" as *const u8 as *const libc::c_char,
              2430 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 20],
                                        &[libc::c_char; 20]>(b"droidUpdateClearing\x00")).as_ptr(),
              b"psFeature->psStats->subType == FEAT_BUILD_WRECK\x00" as
                  *const u8 as *const libc::c_char);
    };
    if (*psFeature).body > 0 as libc::c_int as libc::c_uint {
        constructPoints =
            constructorPoints(asConstructStats.offset((*psDroid).asBits[COMP_CONSTRUCT
                                                                            as
                                                                            libc::c_int
                                                                            as
                                                                            usize].nStat
                                                          as libc::c_int as
                                                          isize),
                              (*psDroid).player);
        pointsToAdd =
            constructPoints.wrapping_mul(gameTime.wrapping_sub((*psDroid).actionStarted)).wrapping_div(1000
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_uint);
        (*psFeature).body =
            ((*psFeature).body as
                 libc::c_uint).wrapping_sub(pointsToAdd.wrapping_sub((*psDroid).actionPoints))
                as UDWORD as UDWORD;
        //store the amount just subtracted
        (*psDroid).actionPoints = pointsToAdd
    }
    /* check if structure is demolished */
    if (*psFeature).body <= 0 as libc::c_int as libc::c_uint {
        /* remove feature */
        removeFeature(psFeature);
        /* reset target stats */
        (*psDroid).psTarStats = 0 as *mut _base_stats;
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn droidStartRepair(mut psDroid: *mut DROID) -> BOOL {
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    //ASSERT( psDroid->order == DORDER_REPAIR,
	//	"droidStartRepair: droid does not have repair order" );
	//psStruct = (STRUCTURE *)psDroid->psTarget;
    psStruct = (*psDroid).psActionTarget as *mut STRUCTURE;
    if (*psStruct).type_0 as libc::c_uint ==
           OBJ_STRUCTURE as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"unitStartRepair: target is not a structure\x00" as *const u8
                  as *const libc::c_char);
    };
    if (*psStruct).type_0 as libc::c_uint ==
           OBJ_STRUCTURE as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"droid.c\x00" as *const u8 as *const libc::c_char,
              2470 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 17],
                                        &[libc::c_char; 17]>(b"droidStartRepair\x00")).as_ptr(),
              b"psStruct->type == OBJ_STRUCTURE\x00" as *const u8 as
                  *const libc::c_char);
    };
    (*psDroid).actionStarted = gameTime;
    (*psDroid).actionPoints = 0 as libc::c_int as UDWORD;
    return 1 as libc::c_int;
}
/*Start a Repair Droid working on a damaged droid*/
#[no_mangle]
pub unsafe extern "C" fn droidStartDroidRepair(mut psDroid: *mut DROID)
 -> BOOL {
    let mut psDroidToRepair: *mut DROID = 0 as *mut DROID;
    //	ASSERT( psDroid->order == DORDER_DROIDREPAIR,
//		"droidStartDroidRepair: droid does not have droid repair order" );
    psDroidToRepair = (*psDroid).psActionTarget as *mut DROID;
    if (*psDroidToRepair).type_0 as libc::c_uint ==
           OBJ_DROID as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"unitStartUnitRepair: target is not a unit\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (*psDroidToRepair).type_0 as libc::c_uint ==
           OBJ_DROID as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"droid.c\x00" as *const u8 as *const libc::c_char,
              2488 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 22],
                                        &[libc::c_char; 22]>(b"droidStartDroidRepair\x00")).as_ptr(),
              b"psDroidToRepair->type == OBJ_DROID\x00" as *const u8 as
                  *const libc::c_char);
    };
    (*psDroid).actionStarted = gameTime;
    (*psDroid).actionPoints = 0 as libc::c_int as UDWORD;
    return 1 as libc::c_int;
}
/*checks a droids current body points to see if need to self repair*/
#[no_mangle]
pub unsafe extern "C" fn droidSelfRepair(mut psDroid: *mut DROID) {
    if vtolDroid(psDroid) == 0 {
        if (*psDroid).body < (*psDroid).originalBody {
            if (*psDroid).asBits[COMP_REPAIRUNIT as libc::c_int as
                                     usize].nStat as libc::c_int !=
                   0 as libc::c_int {
                (*psDroid).action = DACTION_DROIDREPAIR as libc::c_int;
                //			    psDroid->psTarget = (BASE_OBJECT *)psDroid;
                (*psDroid).psActionTarget = psDroid as *mut BASE_OBJECT;
                (*psDroid).actionStarted = gameTime;
                (*psDroid).actionPoints = 0 as libc::c_int as UDWORD
            }
        }
    };
}
/*Start a EW weapon droid working on a low resistance structure*/
#[no_mangle]
pub unsafe extern "C" fn droidStartRestore(mut psDroid: *mut DROID) -> BOOL {
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    if (*psDroid).order == DORDER_RESTORE as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"unitStartRestore: unit is not restoring\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (*psDroid).order == DORDER_RESTORE as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"droid.c\x00" as *const u8 as *const libc::c_char,
              2522 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 18],
                                        &[libc::c_char; 18]>(b"droidStartRestore\x00")).as_ptr(),
              b"psDroid->order == DORDER_RESTORE\x00" as *const u8 as
                  *const libc::c_char);
    };
    psStruct = (*psDroid).psTarget as *mut STRUCTURE;
    if (*psStruct).type_0 as libc::c_uint ==
           OBJ_STRUCTURE as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"unitStartRestore: target is not a structure\x00" as *const u8
                  as *const libc::c_char);
    };
    if (*psStruct).type_0 as libc::c_uint ==
           OBJ_STRUCTURE as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"droid.c\x00" as *const u8 as *const libc::c_char,
              2525 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 18],
                                        &[libc::c_char; 18]>(b"droidStartRestore\x00")).as_ptr(),
              b"psStruct->type == OBJ_STRUCTURE\x00" as *const u8 as
                  *const libc::c_char);
    };
    (*psDroid).actionStarted = gameTime;
    (*psDroid).actionPoints = 0 as libc::c_int as UDWORD;
    return 1 as libc::c_int;
}
/*continue restoring a structure*/
#[no_mangle]
pub unsafe extern "C" fn droidUpdateRestore(mut psDroid: *mut DROID) -> BOOL {
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut pointsToAdd: UDWORD = 0;
    let mut restorePoints: UDWORD = 0;
    let mut psStats: *mut WEAPON_STATS = 0 as *mut WEAPON_STATS;
    if (*psDroid).action == DACTION_RESTORE as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"unitUpdateRestore: unit is not restoring\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (*psDroid).action == DACTION_RESTORE as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"droid.c\x00" as *const u8 as *const libc::c_char,
              2541 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 19],
                                        &[libc::c_char; 19]>(b"droidUpdateRestore\x00")).as_ptr(),
              b"psDroid->action == DACTION_RESTORE\x00" as *const u8 as
                  *const libc::c_char);
    };
    psStruct = (*psDroid).psTarget as *mut STRUCTURE;
    if (*psStruct).type_0 as libc::c_uint ==
           OBJ_STRUCTURE as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"unitUpdateRestore: target is not a structure\x00" as *const u8
                  as *const libc::c_char);
    };
    if (*psStruct).type_0 as libc::c_uint ==
           OBJ_STRUCTURE as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"droid.c\x00" as *const u8 as *const libc::c_char,
              2544 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 19],
                                        &[libc::c_char; 19]>(b"droidUpdateRestore\x00")).as_ptr(),
              b"psStruct->type == OBJ_STRUCTURE\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (*(*psStruct).pStructureType).resistance !=
           0 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"unitUpdateRestore: invalid structure for EW\x00" as *const u8
                  as *const libc::c_char);
    };
    if (*(*psStruct).pStructureType).resistance !=
           0 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"droid.c\x00" as *const u8 as *const libc::c_char,
              2546 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 19],
                                        &[libc::c_char; 19]>(b"droidUpdateRestore\x00")).as_ptr(),
              b"psStruct->pStructureType->resistance != 0\x00" as *const u8 as
                  *const libc::c_char);
    };
    //ASSERT( psDroid->numWeaps != 0,
    if (*psDroid).asWeaps[0 as libc::c_int as usize].nStat >
           0 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"unitUpdateRestore: droid doean\'t have any weapons\x00" as
                  *const u8 as *const libc::c_char);
    };
    if (*psDroid).asWeaps[0 as libc::c_int as usize].nStat >
           0 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"droid.c\x00" as *const u8 as *const libc::c_char,
              2550 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 19],
                                        &[libc::c_char; 19]>(b"droidUpdateRestore\x00")).as_ptr(),
              b"psDroid->asWeaps[0].nStat > 0\x00" as *const u8 as
                  *const libc::c_char);
    };
    psStats =
        asWeaponStats.offset((*psDroid).asWeaps[0 as libc::c_int as
                                                    usize].nStat as isize);
    if (*psStats).weaponSubClass as libc::c_uint ==
           WSC_ELECTRONIC as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"unitUpdateRestore: unit\'s weapon is not EW\x00" as *const u8
                  as *const libc::c_char);
    };
    if (*psStats).weaponSubClass as libc::c_uint ==
           WSC_ELECTRONIC as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"droid.c\x00" as *const u8 as *const libc::c_char,
              2555 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 19],
                                        &[libc::c_char; 19]>(b"droidUpdateRestore\x00")).as_ptr(),
              b"psStats->weaponSubClass == WSC_ELECTRONIC\x00" as *const u8 as
                  *const libc::c_char);
    };
    //restorePoints = calcDamage(psStats->damage, psStats->weaponEffect,(BASE_OBJECT *)psStruct);
    restorePoints =
        calcDamage(weaponDamage(psStats, (*psDroid).player),
                   (*psStats).weaponEffect, psStruct as *mut BASE_OBJECT);
    pointsToAdd =
        restorePoints.wrapping_mul(gameTime.wrapping_sub((*psDroid).actionStarted)).wrapping_div(1000
                                                                                                     as
                                                                                                     libc::c_int
                                                                                                     as
                                                                                                     libc::c_uint);
    (*psStruct).resistance =
        ((*psStruct).resistance as
             libc::c_uint).wrapping_add(pointsToAdd.wrapping_sub((*psDroid).actionPoints))
            as SWORD;
    //store the amount just added
    (*psDroid).actionPoints = pointsToAdd;
    /* check if structure is restored */
	//if ( psStruct->resistance < (SDWORD)(psStruct->pStructureType->resistance))
    if ((*psStruct).resistance as libc::c_int) <
           structureResistance((*psStruct).pStructureType, (*psStruct).player)
               as SDWORD {
        return 1 as libc::c_int
    } else {
        addConsoleMessage(strresGetString(psStringRes,
                                          STR_GAM_STRREST as libc::c_int as
                                              UDWORD), DEFAULT_JUSTIFY);
        //psStruct->resistance = psStruct->pStructureType->resistance;
        (*psStruct).resistance =
            structureResistance((*psStruct).pStructureType,
                                (*psStruct).player) as UWORD as SWORD;
        return 0 as libc::c_int
    };
}
/* Code to have the droid's weapon assembly rock back upon firing */
#[no_mangle]
pub unsafe extern "C" fn droidUpdateRecoil(mut psDroid: *mut DROID) {
    let mut percent: UDWORD = 0;
    let mut recoil: UDWORD = 0;
    let mut fraction: FRACT = 0.;
    /* Check it's actually got a weapon */
	//if(psDroid->numWeaps == 0)
    if (*psDroid).asWeaps[0 as libc::c_int as usize].nStat ==
           0 as libc::c_int as libc::c_uint {
        return
    }
    /* We have a weapon */
    if gameTime >
           (*psDroid).asWeaps[0 as libc::c_int as
                                  usize].lastFired.wrapping_add((1000 as
                                                                     libc::c_int
                                                                     /
                                                                     4 as
                                                                         libc::c_int)
                                                                    as
                                                                    libc::c_uint)
       {
        /* Recoil effect is over */
        (*psDroid).asWeaps[0 as libc::c_int as usize].recoilValue =
            0 as libc::c_int as UDWORD;
        return
    }
    /* Where should the assembly be? */
    percent =
        gameTime.wrapping_sub((*psDroid).asWeaps[0 as libc::c_int as
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
        (*asWeaponStats.offset((*psDroid).asWeaps[0 as libc::c_int as
                                                      usize].nStat as
                                   isize)).recoilValue as FRACT /
            100 as libc::c_int as FRACT;
    recoil = (recoil as FRACT * fraction) as SDWORD as UDWORD;
    /* Put it into the weapon data */
    (*psDroid).asWeaps[0 as libc::c_int as usize].recoilValue = recoil;
}
#[no_mangle]
pub unsafe extern "C" fn droidUpdateRepair(mut psDroid: *mut DROID) -> BOOL {
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut iPointsToAdd: UDWORD = 0;
    let mut iRepairPoints: UDWORD = 0;
    if (*psDroid).action == DACTION_REPAIR as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"unitUpdateRepair: unit does not have repair order\x00" as
                  *const u8 as *const libc::c_char);
    };
    if (*psDroid).action == DACTION_REPAIR as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"droid.c\x00" as *const u8 as *const libc::c_char,
              2640 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 18],
                                        &[libc::c_char; 18]>(b"droidUpdateRepair\x00")).as_ptr(),
              b"psDroid->action == DACTION_REPAIR\x00" as *const u8 as
                  *const libc::c_char);
    };
    //psStruct = (STRUCTURE *)psDroid->psTarget;
    psStruct = (*psDroid).psActionTarget as *mut STRUCTURE;
    if (*psStruct).type_0 as libc::c_uint ==
           OBJ_STRUCTURE as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"unitUpdateRepair: target is not a structure\x00" as *const u8
                  as *const libc::c_char);
    };
    if (*psStruct).type_0 as libc::c_uint ==
           OBJ_STRUCTURE as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"droid.c\x00" as *const u8 as *const libc::c_char,
              2644 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 18],
                                        &[libc::c_char; 18]>(b"droidUpdateRepair\x00")).as_ptr(),
              b"psStruct->type == OBJ_STRUCTURE\x00" as *const u8 as
                  *const libc::c_char);
    };
    //iRepairPoints = asConstructStats + psDroid->asBits[COMP_CONSTRUCT].nStat)->
	//	constructPoints;
    iRepairPoints =
        constructorPoints(asConstructStats.offset((*psDroid).asBits[COMP_CONSTRUCT
                                                                        as
                                                                        libc::c_int
                                                                        as
                                                                        usize].nStat
                                                      as libc::c_int as
                                                      isize),
                          (*psDroid).player);
    iPointsToAdd =
        iRepairPoints.wrapping_mul(gameTime.wrapping_sub((*psDroid).actionStarted)).wrapping_div(1000
                                                                                                     as
                                                                                                     libc::c_int
                                                                                                     as
                                                                                                     libc::c_uint);
    /* add points to structure */
    (*psStruct).body =
        ((*psStruct).body as
             libc::c_uint).wrapping_add(iPointsToAdd.wrapping_sub((*psDroid).actionPoints))
            as UWORD;
    /* store the amount just added */
    (*psDroid).actionPoints = iPointsToAdd;
    /* if not finished repair return TRUE else complete repair and return FALSE */
	//if ( psStruct->body < psStruct->baseBodyPoints )
    if ((*psStruct).body as libc::c_uint) < structureBody(psStruct) {
        return 1 as libc::c_int
    } else {
        //psStruct->body = psStruct->baseBodyPoints;
        (*psStruct).body = structureBody(psStruct) as UWORD;
        return 0 as libc::c_int
    };
}
/*Updates a Repair Droid working on a damaged droid*/
#[no_mangle]
pub unsafe extern "C" fn droidUpdateDroidRepair(mut psRepairDroid: *mut DROID)
 -> BOOL {
    let mut psDroidToRepair: *mut DROID = 0 as *mut DROID;
    let mut iPointsToAdd: UDWORD = 0;
    let mut iRepairPoints: UDWORD = 0;
    let mut powerCost: UDWORD = 0;
    let mut iVecEffect: iVector = iVector{x: 0, y: 0, z: 0,};
    if (*psRepairDroid).action == DACTION_DROIDREPAIR as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"unitUpdateUnitRepair: unit does not have unit repair order\x00"
                  as *const u8 as *const libc::c_char);
    };
    if (*psRepairDroid).action == DACTION_DROIDREPAIR as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"droid.c\x00" as *const u8 as *const libc::c_char,
              2682 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 23],
                                        &[libc::c_char; 23]>(b"droidUpdateDroidRepair\x00")).as_ptr(),
              b"psRepairDroid->action == DACTION_DROIDREPAIR\x00" as *const u8
                  as *const libc::c_char);
    };
    if (*psRepairDroid).asBits[COMP_REPAIRUNIT as libc::c_int as usize].nStat
           as libc::c_int != 0 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"unitUpdateUnitRepair: unit does not have a repair turret\x00"
                  as *const u8 as *const libc::c_char);
    };
    if (*psRepairDroid).asBits[COMP_REPAIRUNIT as libc::c_int as usize].nStat
           as libc::c_int != 0 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"droid.c\x00" as *const u8 as *const libc::c_char,
              2684 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 23],
                                        &[libc::c_char; 23]>(b"droidUpdateDroidRepair\x00")).as_ptr(),
              b"psRepairDroid->asBits[COMP_REPAIRUNIT].nStat != 0\x00" as
                  *const u8 as *const libc::c_char);
    };
    psDroidToRepair = (*psRepairDroid).psActionTarget as *mut DROID;
    if (*psDroidToRepair).type_0 as libc::c_uint ==
           OBJ_DROID as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"unitUpdateUnitRepair: target is not a unit\x00" as *const u8
                  as *const libc::c_char);
    };
    if (*psDroidToRepair).type_0 as libc::c_uint ==
           OBJ_DROID as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"droid.c\x00" as *const u8 as *const libc::c_char,
              2688 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 23],
                                        &[libc::c_char; 23]>(b"droidUpdateDroidRepair\x00")).as_ptr(),
              b"psDroidToRepair->type == OBJ_DROID\x00" as *const u8 as
                  *const libc::c_char);
    };
    //nah - once more unto the breach my friend...or something like that...
    //Changed again so waits until got enough power to do the repair work - AB 5/3/99
    //Do repair gradually as power comes in
    //for now wait until have enough power to repair
    /*if (powerReqForDroidRepair(psDroidToRepair) > psDroidToRepair->powerAccrued)
    {
        psRepairDroid->actionStarted = gameTime;
        psRepairDroid->actionPoints = 0;
        return TRUE;
    }*/
    //the amount of power accrued limits how much of the work can be done
    //self-repair doesn't cost power
    if psRepairDroid == psDroidToRepair {
        powerCost = 0 as libc::c_int as UDWORD
    } else {
        //check if enough power to do any
        //powerCost = (powerReqForDroidRepair(psDroidToRepair) -
        //    psDroidToRepair->powerAccrued) / POWER_FACTOR;
        powerCost = powerReqForDroidRepair(psDroidToRepair) as UDWORD;
        if powerCost > (*psDroidToRepair).powerAccrued as libc::c_uint {
            powerCost =
                powerCost.wrapping_sub((*psDroidToRepair).powerAccrued as
                                           libc::c_uint).wrapping_div(100 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_uint)
        } else { powerCost = 0 as libc::c_int as UDWORD }
    }
    //if the power cost is 0 (due to rounding) then do for free!
    if powerCost != 0 {
        if (*psDroidToRepair).powerAccrued == 0 {
            //reset the actionStarted time and actionPoints added so the correct
            //amount of points are added when there is more power
            (*psRepairDroid).actionStarted = gameTime;
            //init so repair points to add won't be huge when start up again
            (*psRepairDroid).actionPoints = 0 as libc::c_int as UDWORD;
            return 1 as libc::c_int
        }
    }
    //iRepairPoints = asRepairStats[psRepairDroid->asBits[COMP_REPAIRUNIT].
	//	nStat].repairPoints;
    iRepairPoints =
        repairPoints(asRepairStats.offset((*psRepairDroid).asBits[COMP_REPAIRUNIT
                                                                      as
                                                                      libc::c_int
                                                                      as
                                                                      usize].nStat
                                              as libc::c_int as isize),
                     (*psRepairDroid).player);
    //if self repair then add repair points depending on the time delay for the stat
    if psRepairDroid == psDroidToRepair {
        iPointsToAdd =
            iRepairPoints.wrapping_mul(gameTime.wrapping_sub((*psRepairDroid).actionStarted)).wrapping_div((*asRepairStats.offset((*psRepairDroid).asBits[COMP_REPAIRUNIT
                                                                                                                                                              as
                                                                                                                                                              libc::c_int
                                                                                                                                                              as
                                                                                                                                                              usize].nStat
                                                                                                                                      as
                                                                                                                                      libc::c_int
                                                                                                                                      as
                                                                                                                                      isize)).time)
    } else {
        iPointsToAdd =
            iRepairPoints.wrapping_mul(gameTime.wrapping_sub((*psRepairDroid).actionStarted)).wrapping_div(1000
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                               as
                                                                                                               libc::c_uint)
    }
    iPointsToAdd =
        (iPointsToAdd as
             libc::c_uint).wrapping_sub((*psRepairDroid).actionPoints) as
            UDWORD as UDWORD;
    if iPointsToAdd != 0 {
        //just add the points if the power cost is negligable
        //if these points would make the droid healthy again then just add
        if powerCost == 0 ||
               (*psDroidToRepair).body.wrapping_add(iPointsToAdd) >=
                   (*psDroidToRepair).originalBody {
            //anothe HACK but sorts out all the rounding errors when values get small
            (*psDroidToRepair).body =
                ((*psDroidToRepair).body as
                     libc::c_uint).wrapping_add(iPointsToAdd) as UDWORD as
                    UDWORD;
            (*psRepairDroid).actionPoints =
                ((*psRepairDroid).actionPoints as
                     libc::c_uint).wrapping_add(iPointsToAdd) as UDWORD as
                    UDWORD
        } else {
            //see if we have enough power to do this amount of repair
            powerCost =
                iPointsToAdd.wrapping_mul(repairPowerPoint(psDroidToRepair) as
                                              libc::c_uint);
            if powerCost <= (*psDroidToRepair).powerAccrued as libc::c_uint {
                (*psDroidToRepair).body =
                    ((*psDroidToRepair).body as
                         libc::c_uint).wrapping_add(iPointsToAdd) as UDWORD as
                        UDWORD;
                (*psRepairDroid).actionPoints =
                    ((*psRepairDroid).actionPoints as
                         libc::c_uint).wrapping_add(iPointsToAdd) as UDWORD as
                        UDWORD;
                //subtract the power cost for these points
                (*psDroidToRepair).powerAccrued =
                    ((*psDroidToRepair).powerAccrued as
                         libc::c_uint).wrapping_sub(powerCost) as UWORD as
                        UWORD
            } else {
                /*reset the actionStarted time and actionPoints added so the correct
                amount of points are added when there is more power*/
                (*psRepairDroid).actionStarted = gameTime;
                (*psRepairDroid).actionPoints = 0 as libc::c_int as UDWORD
            }
        }
    }
    /* add points to droid */
	//psDroidToRepair->body += (iPointsToAdd - psRepairDroid->actionPoints);
    /* store the amount just added */
	//psRepairDroid->actionPoints = iPointsToAdd;
    /* add plasma repair effect whilst being repaired */
    if rand() % 5 as libc::c_int == 0 as libc::c_int &&
           (*psDroidToRepair).visible[selectedPlayer as usize] as libc::c_int
               != 0 {
        iVecEffect.x =
            (*psDroidToRepair).x as libc::c_int +
                (20 as libc::c_int - rand() % 40 as libc::c_int);
        iVecEffect.y =
            (*psDroidToRepair).z as libc::c_int + rand() % 8 as libc::c_int;
        iVecEffect.z =
            (*psDroidToRepair).y as libc::c_int +
                (20 as libc::c_int - rand() % 40 as libc::c_int);
        effectGiveAuxVar((90 as libc::c_int + rand() % 20 as libc::c_int) as
                             UDWORD);
        addEffect(&mut iVecEffect, EFFECT_EXPLOSION, EXPLOSION_TYPE_LASER,
                  0 as libc::c_int, 0 as *mut iIMDShape, 0 as libc::c_int);
        droidAddWeldSound(iVecEffect);
    }
    /* if not finished repair return TRUE else complete repair and return FALSE */
    if (*psDroidToRepair).body < (*psDroidToRepair).originalBody {
        return 1 as libc::c_int
    } else {
        //reset the power accrued
        (*psDroidToRepair).powerAccrued = 0 as libc::c_int as UWORD;
        (*psDroidToRepair).body = (*psDroidToRepair).originalBody;
        return 0 as libc::c_int
    };
}
/* load the Droid stats for the components from the Access database */
#[no_mangle]
pub unsafe extern "C" fn loadDroidTemplates(mut pDroidData: *mut libc::c_char,
                                            mut bufferSize: UDWORD) -> BOOL {
    let mut pStartDroidData: *mut libc::c_char =
        0 as *mut libc::c_char; //,EndOfFile;
    let mut cnt: libc::c_int = 0;
    let mut NumDroids: UDWORD = 0 as libc::c_int as UDWORD;
    let mut i: UDWORD = 0;
    let mut player: UDWORD = 0;
    let mut componentName: [STRING; 60] = [0; 60];
    let mut found: BOOL = 0 as libc::c_int;
    let mut pDroidDesign: *mut DROID_TEMPLATE = 0 as *mut DROID_TEMPLATE;
    let mut pStats: *mut COMP_BASE_STATS = 0 as *mut COMP_BASE_STATS;
    let mut size: UDWORD = 0;
    let mut inc: UDWORD = 0;
    let mut templateID: UDWORD = 0;
    let mut bDefaultTemplateFound: BOOL = 0 as libc::c_int;
    //	STRING				*pDroidName = droidName;
    let mut id: UDWORD = 0;
    /* init default template */
    memset(&mut sDefaultDesignTemplate as *mut DROID_TEMPLATE as
               *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<DROID_TEMPLATE>() as libc::c_ulong);
    pStartDroidData = pDroidData;
    NumDroids = numCR(pDroidData, bufferSize);
    i = 0 as libc::c_int as UDWORD;
    while i < NumDroids {
        if heapAlloc(psTemplateHeap,
                     &mut pDroidDesign as *mut *mut DROID_TEMPLATE as
                         *mut libc::c_void as *mut *mut libc::c_void) == 0 {
            debug(LOG_ERROR,
                  b"Out of memory - Droid Templates\x00" as *const u8 as
                      *const libc::c_char);
            abort();
        }
        memset(pDroidDesign as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<DROID_TEMPLATE>() as libc::c_ulong);
        //pDroidDesign->pName = pDroidDesign->aName;
		//only fill in aName now
        (*pDroidDesign).pName = 0 as *mut STRING;
        //read the data into the storage - the data is delimited using comma's
        componentName[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        sscanf(pDroidData,
               b"%[^\',\'],%d,%n\x00" as *const u8 as *const libc::c_char,
               componentName.as_mut_ptr(), &mut templateID as *mut UDWORD,
               &mut cnt as *mut libc::c_int);
        pDroidData = pDroidData.offset(cnt as isize);
        // Hideous mishmash of ifdef's ... sorry about that
        //We ain't EVER going back to the way it was so..just store the long (translated) name in aName
/*#ifdef RESOURCE_NAMES
		//get the name associated with the resource
		if (!strresGetIDNum(psStringRes, componentName, &id))
		{
			DBERROR(("Unable to find string resource for %s", componentName));
			return FALSE;
		}

		//get the string from the id and copy into the MALLOC'ed area
		strcpy(droidName,strresGetString(psStringRes, id));

#elif defined STORE_RESOURCE_ID
		//find the pointer where the string ID is stored
		if (!strresGetIDString(psStringRes, componentName, &pDroidName))
		{
			DBERROR(("Unable to find string resource for %s", componentName));
			return FALSE;
		}
		strcpy(droidName,pDroidName);
#else
		strcpy(droidName,componentName);
#endif
		//get the string from the id and copy into the MALLOC'ed area
		strcpy(droidName,strresGetString(psStringRes, id));

		//allocate storage for the name - constant array now (aName)
		strncpy(pDroidDesign->pName,droidName, DROID_MAXNAME);
		pDroidDesign->pName[DROID_MAXNAME-1]=0;*/
        /*ALWAYS get the name associated with the resource for PC regardless
		of STORE_RESOURCE_ID or RESOURCE_NAMES! - 25/06/98 AB*/
        if strresGetIDNum(psStringRes, componentName.as_mut_ptr(), &mut id) ==
               0 {
            debug(LOG_ERROR,
                  b"Unable to find string resource for %s\x00" as *const u8 as
                      *const libc::c_char, componentName.as_mut_ptr());
            abort();
        }
        //get the string from the id and copy into the Name space
        strcpy((*pDroidDesign).aName.as_mut_ptr(),
               strresGetString(psStringRes, id));
        (*pDroidDesign).aName[(60 as libc::c_int - 1 as libc::c_int) as usize]
            = 0 as libc::c_int as STRING;
        //store the unique template id - NOT ON PSX
        (*pDroidDesign).multiPlayerID = templateID;
        //read in Body Name
        componentName[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        sscanf(pDroidData,
               b"%[^\',\'],%n\x00" as *const u8 as *const libc::c_char,
               componentName.as_mut_ptr(), &mut cnt as *mut libc::c_int);
        pDroidData = pDroidData.offset(cnt as isize);
        found = 0 as libc::c_int;
        //get the Body stats pointer
        if strcmp(componentName.as_mut_ptr(),
                  b"0\x00" as *const u8 as *const libc::c_char) == 0 {
            (*pDroidDesign).asParts[COMP_BODY as libc::c_int as usize] =
                -(1 as libc::c_int)
        } else {
            pStats = asBodyStats as *mut COMP_BASE_STATS;
            size = ::std::mem::size_of::<BODY_STATS>() as libc::c_ulong;
            if getResourceName(componentName.as_mut_ptr()) == 0 {
                return 0 as libc::c_int
            }
            inc = 0 as libc::c_int as UDWORD;
            while inc < numBodyStats {
                //compare the names
                if strcmp(componentName.as_mut_ptr(), (*pStats).pName) == 0 {
                    (*pDroidDesign).asParts[COMP_BODY as libc::c_int as usize]
                        = inc as SDWORD;
                    found = 1 as libc::c_int;
                    break ;
                } else {
                    pStats =
                        (pStats as *mut UBYTE).offset(size as isize) as
                            *mut COMP_BASE_STATS;
                    inc = inc.wrapping_add(1)
                }
            }
            if found == 0 {
                debug(LOG_ERROR,
                      b"Body component not found for droid %s\x00" as
                          *const u8 as *const libc::c_char,
                      getTemplateName(pDroidDesign));
                abort();
            }
        }
        //read in Brain Name
        found = 0 as libc::c_int;
        componentName[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        sscanf(pDroidData,
               b"%[^\',\'],%n\x00" as *const u8 as *const libc::c_char,
               componentName.as_mut_ptr(), &mut cnt as *mut libc::c_int);
        pDroidData = pDroidData.offset(cnt as isize);
        //get the Brain stats pointer
        if strcmp(componentName.as_mut_ptr(),
                  b"0\x00" as *const u8 as *const libc::c_char) == 0 {
            (*pDroidDesign).asParts[COMP_BRAIN as libc::c_int as usize] =
                -(1 as libc::c_int)
        } else {
            pStats = asBrainStats as *mut COMP_BASE_STATS;
            size = ::std::mem::size_of::<BRAIN_STATS>() as libc::c_ulong;
            if getResourceName(componentName.as_mut_ptr()) == 0 {
                return 0 as libc::c_int
            }
            inc = 0 as libc::c_int as UDWORD;
            while inc < numBrainStats {
                //compare the names
                if strcmp(componentName.as_mut_ptr(), (*pStats).pName) == 0 {
                    (*pDroidDesign).asParts[COMP_BRAIN as libc::c_int as
                                                usize] = inc as SDWORD;
                    found = 1 as libc::c_int;
                    break ;
                } else {
                    pStats =
                        (pStats as *mut UBYTE).offset(size as isize) as
                            *mut COMP_BASE_STATS;
                    inc = inc.wrapping_add(1)
                }
            }
            if found == 0 {
                debug(LOG_ERROR,
                      b"Brain component not found for droid %s\x00" as
                          *const u8 as *const libc::c_char,
                      getTemplateName(pDroidDesign));
                abort();
            }
        }
        //read in Construct Name
        found = 0 as libc::c_int;
        componentName[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        sscanf(pDroidData,
               b"%[^\',\'],%n\x00" as *const u8 as *const libc::c_char,
               componentName.as_mut_ptr(), &mut cnt as *mut libc::c_int);
        pDroidData = pDroidData.offset(cnt as isize);
        //get the Construct stats pointer
        if strcmp(componentName.as_mut_ptr(),
                  b"0\x00" as *const u8 as *const libc::c_char) == 0 {
            (*pDroidDesign).asParts[COMP_CONSTRUCT as libc::c_int as usize] =
                -(1 as libc::c_int)
        } else {
            pStats = asConstructStats as *mut COMP_BASE_STATS;
            size = ::std::mem::size_of::<CONSTRUCT_STATS>() as libc::c_ulong;
            if getResourceName(componentName.as_mut_ptr()) == 0 {
                return 0 as libc::c_int
            }
            inc = 0 as libc::c_int as UDWORD;
            while inc < numConstructStats {
                //compare the names
                if strcmp(componentName.as_mut_ptr(), (*pStats).pName) == 0 {
                    (*pDroidDesign).asParts[COMP_CONSTRUCT as libc::c_int as
                                                usize] = inc as SDWORD;
                    found = 1 as libc::c_int;
                    break ;
                } else {
                    pStats =
                        (pStats as *mut UBYTE).offset(size as isize) as
                            *mut COMP_BASE_STATS;
                    inc = inc.wrapping_add(1)
                }
            }
            if found == 0 {
                debug(LOG_ERROR,
                      b"Construct component not found for droid %s\x00" as
                          *const u8 as *const libc::c_char,
                      getTemplateName(pDroidDesign));
                abort();
            }
        }
        //read in Ecm Name
        found = 0 as libc::c_int;
        componentName[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        sscanf(pDroidData,
               b"%[^\',\'],%n\x00" as *const u8 as *const libc::c_char,
               componentName.as_mut_ptr(), &mut cnt as *mut libc::c_int);
        pDroidData = pDroidData.offset(cnt as isize);
        //get the Ecm stats pointer
        if strcmp(componentName.as_mut_ptr(),
                  b"0\x00" as *const u8 as *const libc::c_char) == 0 {
            (*pDroidDesign).asParts[COMP_ECM as libc::c_int as usize] =
                -(1 as libc::c_int)
        } else {
            pStats = asECMStats as *mut COMP_BASE_STATS;
            size = ::std::mem::size_of::<ECM_STATS>() as libc::c_ulong;
            if getResourceName(componentName.as_mut_ptr()) == 0 {
                return 0 as libc::c_int
            }
            inc = 0 as libc::c_int as UDWORD;
            while inc < numECMStats {
                //compare the names
                if strcmp(componentName.as_mut_ptr(), (*pStats).pName) == 0 {
                    (*pDroidDesign).asParts[COMP_ECM as libc::c_int as usize]
                        = inc as SDWORD;
                    found = 1 as libc::c_int;
                    break ;
                } else {
                    pStats =
                        (pStats as *mut UBYTE).offset(size as isize) as
                            *mut COMP_BASE_STATS;
                    inc = inc.wrapping_add(1)
                }
            }
            if found == 0 {
                debug(LOG_ERROR,
                      b"ECM component not found for droid %s\x00" as *const u8
                          as *const libc::c_char,
                      getTemplateName(pDroidDesign));
                abort();
            }
        }
        //read in player id - Access decides the order -crap hey?
        sscanf(pDroidData, b"%d,%n\x00" as *const u8 as *const libc::c_char,
               &mut player as *mut UDWORD, &mut cnt as *mut libc::c_int);
        pDroidData = pDroidData.offset(cnt as isize);
        //read in Propulsion Name
        found = 0 as libc::c_int;
        componentName[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        sscanf(pDroidData,
               b"%[^\',\'],%n\x00" as *const u8 as *const libc::c_char,
               componentName.as_mut_ptr(), &mut cnt as *mut libc::c_int);
        pDroidData = pDroidData.offset(cnt as isize);
        //get the Propulsion stats pointer
        if strcmp(componentName.as_mut_ptr(),
                  b"0\x00" as *const u8 as *const libc::c_char) == 0 {
            (*pDroidDesign).asParts[COMP_PROPULSION as libc::c_int as usize] =
                -(1 as libc::c_int)
        } else {
            pStats = asPropulsionStats as *mut COMP_BASE_STATS;
            size = ::std::mem::size_of::<PROPULSION_STATS>() as libc::c_ulong;
            if getResourceName(componentName.as_mut_ptr()) == 0 {
                return 0 as libc::c_int
            }
            inc = 0 as libc::c_int as UDWORD;
            while inc < numPropulsionStats {
                //compare the names
                if strcmp(componentName.as_mut_ptr(), (*pStats).pName) == 0 {
                    (*pDroidDesign).asParts[COMP_PROPULSION as libc::c_int as
                                                usize] = inc as SDWORD;
                    found = 1 as libc::c_int;
                    break ;
                } else {
                    pStats =
                        (pStats as *mut UBYTE).offset(size as isize) as
                            *mut COMP_BASE_STATS;
                    inc = inc.wrapping_add(1)
                }
            }
            if found == 0 {
                debug(LOG_ERROR,
                      b"Propulsion component not found for droid %s\x00" as
                          *const u8 as *const libc::c_char,
                      getTemplateName(pDroidDesign));
                abort();
            }
        }
        //read in Repair Name
        found = 0 as libc::c_int;
        componentName[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        sscanf(pDroidData,
               b"%[^\',\'],%n\x00" as *const u8 as *const libc::c_char,
               componentName.as_mut_ptr(), &mut cnt as *mut libc::c_int);
        pDroidData = pDroidData.offset(cnt as isize);
        //get the Repair stats pointer
        if strcmp(componentName.as_mut_ptr(),
                  b"0\x00" as *const u8 as *const libc::c_char) == 0 {
            (*pDroidDesign).asParts[COMP_REPAIRUNIT as libc::c_int as usize] =
                -(1 as libc::c_int)
        } else {
            pStats = asRepairStats as *mut COMP_BASE_STATS;
            size = ::std::mem::size_of::<REPAIR_STATS>() as libc::c_ulong;
            if getResourceName(componentName.as_mut_ptr()) == 0 {
                return 0 as libc::c_int
            }
            inc = 0 as libc::c_int as UDWORD;
            while inc < numRepairStats {
                //compare the names
                if strcmp(componentName.as_mut_ptr(), (*pStats).pName) == 0 {
                    (*pDroidDesign).asParts[COMP_REPAIRUNIT as libc::c_int as
                                                usize] = inc as SDWORD;
                    found = 1 as libc::c_int;
                    break ;
                } else {
                    pStats =
                        (pStats as *mut UBYTE).offset(size as isize) as
                            *mut COMP_BASE_STATS;
                    inc = inc.wrapping_add(1)
                }
            }
            if found == 0 {
                debug(LOG_ERROR,
                      b"Repair component not found for droid %s\x00" as
                          *const u8 as *const libc::c_char,
                      getTemplateName(pDroidDesign));
                abort();
            }
        }
        //read in droid type - only interested if set to PERSON
        componentName[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        sscanf(pDroidData,
               b"%[^\',\'],%n\x00" as *const u8 as *const libc::c_char,
               componentName.as_mut_ptr(), &mut cnt as *mut libc::c_int);
        pDroidData = pDroidData.offset(cnt as isize);
        if strcmp(componentName.as_mut_ptr(),
                  b"PERSON\x00" as *const u8 as *const libc::c_char) == 0 {
            (*pDroidDesign).droidType = DROID_PERSON
        }
        if strcmp(componentName.as_mut_ptr(),
                  b"CYBORG\x00" as *const u8 as *const libc::c_char) == 0 {
            (*pDroidDesign).droidType = DROID_CYBORG
        }
        if strcmp(componentName.as_mut_ptr(),
                  b"CYBORG_SUPER\x00" as *const u8 as *const libc::c_char) ==
               0 {
            (*pDroidDesign).droidType = DROID_CYBORG_SUPER
        }
        if strcmp(componentName.as_mut_ptr(),
                  b"CYBORG_CONSTRUCT\x00" as *const u8 as *const libc::c_char)
               == 0 {
            (*pDroidDesign).droidType = DROID_CYBORG_CONSTRUCT
        }
        if strcmp(componentName.as_mut_ptr(),
                  b"CYBORG_REPAIR\x00" as *const u8 as *const libc::c_char) ==
               0 {
            (*pDroidDesign).droidType = DROID_CYBORG_REPAIR
        }
        if strcmp(componentName.as_mut_ptr(),
                  b"TRANSPORTER\x00" as *const u8 as *const libc::c_char) == 0
           {
            (*pDroidDesign).droidType = DROID_TRANSPORTER
        }
        if strcmp(componentName.as_mut_ptr(),
                  b"ZNULLDROID\x00" as *const u8 as *const libc::c_char) == 0
           {
            (*pDroidDesign).droidType = DROID_DEFAULT;
            bDefaultTemplateFound = 1 as libc::c_int
        }
        //pDroidData += (strlen(componentName)+1);
        //read in Sensor Name
        found = 0 as libc::c_int;
        componentName[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        sscanf(pDroidData,
               b"%[^\',\'],%n\x00" as *const u8 as *const libc::c_char,
               componentName.as_mut_ptr(), &mut cnt as *mut libc::c_int);
        pDroidData = pDroidData.offset(cnt as isize);
        //get the Sensor stats pointer
        if strcmp(componentName.as_mut_ptr(),
                  b"0\x00" as *const u8 as *const libc::c_char) == 0 {
            (*pDroidDesign).asParts[COMP_SENSOR as libc::c_int as usize] =
                -(1 as libc::c_int)
        } else {
            pStats = asSensorStats as *mut COMP_BASE_STATS;
            size = ::std::mem::size_of::<SENSOR_STATS>() as libc::c_ulong;
            if getResourceName(componentName.as_mut_ptr()) == 0 {
                return 0 as libc::c_int
            }
            inc = 0 as libc::c_int as UDWORD;
            while inc < numSensorStats {
                //compare the names
                if strcmp(componentName.as_mut_ptr(), (*pStats).pName) == 0 {
                    (*pDroidDesign).asParts[COMP_SENSOR as libc::c_int as
                                                usize] = inc as SDWORD;
                    found = 1 as libc::c_int;
                    break ;
                } else {
                    pStats =
                        (pStats as *mut UBYTE).offset(size as isize) as
                            *mut COMP_BASE_STATS;
                    inc = inc.wrapping_add(1)
                }
            }
            if found == 0 {
                debug(LOG_ERROR,
                      b"Sensor not found for droid Template: %s\x00" as
                          *const u8 as *const libc::c_char,
                      (*pDroidDesign).aName.as_mut_ptr());
                abort();
            }
        }
        //read in totals
		/*sscanf(pDroidData,"%d,%d",
			&pDroidDesign->numProgs, &pDroidDesign->numWeaps);*/
        sscanf(pDroidData, b"%d\x00" as *const u8 as *const libc::c_char,
               &mut (*pDroidDesign).numWeaps as *mut UDWORD);
        //check that not allocating more weapons than allowed
        if (*asBodyStats.offset((*pDroidDesign).asParts[COMP_BODY as
                                                            libc::c_int as
                                                            usize] as
                                    isize)).weaponSlots <
               (*pDroidDesign).numWeaps ||
               (*pDroidDesign).numWeaps > 1 as libc::c_int as libc::c_uint {
            debug(LOG_ERROR,
                  b"Too many weapons have been allocated for droid Template: %s\x00"
                      as *const u8 as *const libc::c_char,
                  (*pDroidDesign).aName.as_mut_ptr());
            abort();
        }
        //check that not allocating more programs than allowed
		/*if (((asBrainStats + pDroidDesign->asParts[COMP_BRAIN])->progCap <
			 pDroidDesign->numProgs) ||
			pDroidDesign->numProgs > DROID_MAXPROGS)
		{
#ifdef HASH_NAMES
			DBERROR(("Too many programs have been allocated for droid Template: %s",strresGetString(NULL,pDroidDesign->NameHash)));
#else
			DBERROR(("Too many programs have been allocated for droid Template: %s",getName(pDroidDesign->pName)));
#endif
			return FALSE;
		}*/
        (*pDroidDesign).ref_0 =
            (0xc0000 as libc::c_int as libc::c_uint).wrapping_add(i);
        /*	Loaded in from the database now AB 29/10/98
#ifndef PSX
			pDroidDesign->multiPlayerID = i;			// another unique number, just for multiplayer stuff.
#endif
*/
		/* store global default design if found else
		 * store in the appropriate array
		 */
        if (*pDroidDesign).droidType as libc::c_uint ==
               DROID_DEFAULT as libc::c_int as libc::c_uint {
            memcpy(&mut sDefaultDesignTemplate as *mut DROID_TEMPLATE as
                       *mut libc::c_void, pDroidDesign as *const libc::c_void,
                   ::std::mem::size_of::<DROID_TEMPLATE>() as libc::c_ulong);
            heapFree(psTemplateHeap, pDroidDesign as *mut libc::c_void);
        } else {
            (*pDroidDesign).psNext = apsDroidTemplates[player as usize];
            apsDroidTemplates[player as usize] = pDroidDesign
        }
        //increment the pointer to the start of the next record
        pDroidData =
            strchr(pDroidData, '\n' as i32).offset(1 as libc::c_int as isize);
        pDroidDesign = pDroidDesign.offset(1);
        i = i.wrapping_add(1)
    }
    //	FREE(pStartDroidData);
    if bDefaultTemplateFound == 0 as libc::c_int {
        debug(LOG_ERROR,
              b"loadUnitTemplates: default template not found\n\x00" as
                  *const u8 as *const libc::c_char);
        abort();
    }
    return 1 as libc::c_int;
}
/*initialise the template build and power points */
#[no_mangle]
pub unsafe extern "C" fn initTemplatePoints() {
    let mut player: UDWORD = 0;
    let mut pDroidDesign: *mut DROID_TEMPLATE = 0 as *mut DROID_TEMPLATE;
    player = 0 as libc::c_int as UDWORD;
    while player < 8 as libc::c_int as libc::c_uint {
        pDroidDesign = apsDroidTemplates[player as usize];
        while !pDroidDesign.is_null() {
            //calculate the total build points
            (*pDroidDesign).buildPoints = calcTemplateBuild(pDroidDesign);
            //calc the total power points
            (*pDroidDesign).powerPoints = calcTemplatePower(pDroidDesign);
            pDroidDesign = (*pDroidDesign).psNext
        }
        player = player.wrapping_add(1)
    };
}
// return whether a droid is IDF
#[no_mangle]
pub unsafe extern "C" fn idfDroid(mut psDroid: *mut DROID) -> BOOL {
    //add Cyborgs
	//if (psDroid->droidType != DROID_WEAPON)
    if !((*psDroid).droidType as libc::c_uint ==
             DROID_WEAPON as libc::c_int as libc::c_uint ||
             (*psDroid).droidType as libc::c_uint ==
                 DROID_CYBORG as libc::c_int as libc::c_uint ||
             (*psDroid).droidType as libc::c_uint ==
                 DROID_CYBORG_SUPER as libc::c_int as libc::c_uint) {
        return 0 as libc::c_int
    }
    if proj_Direct(asWeaponStats.offset((*psDroid).asWeaps[0 as libc::c_int as
                                                               usize].nStat as
                                            isize)) != 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
// return whether a template is for an IDF droid
#[no_mangle]
pub unsafe extern "C" fn templateIsIDF(mut psTemplate: *mut DROID_TEMPLATE)
 -> BOOL {
    //add Cyborgs
	//if (psTemplate->droidType != DROID_WEAPON)
    if !((*psTemplate).droidType as libc::c_uint ==
             DROID_WEAPON as libc::c_int as libc::c_uint ||
             (*psTemplate).droidType as libc::c_uint ==
                 DROID_CYBORG as libc::c_int as libc::c_uint ||
             (*psTemplate).droidType as libc::c_uint ==
                 DROID_CYBORG_SUPER as libc::c_int as libc::c_uint) {
        return 0 as libc::c_int
    }
    if proj_Direct(asWeaponStats.offset((*psTemplate).asWeaps[0 as libc::c_int
                                                                  as usize] as
                                            isize)) != 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
/* Return the type of a droid */
#[no_mangle]
pub unsafe extern "C" fn droidType(mut psDroid: *mut DROID) -> DROID_TYPE {
    /*	DROID_TYPE	type;

	if ((asSensorStats + psDroid->asBits[COMP_SENSOR].nStat)->location == LOC_TURRET)
	{
		type = DROID_SENSOR;
	}
	else if ((asECMStats + psDroid->asBits[COMP_ECM].nStat)->location == LOC_TURRET)
	{
		type = DROID_ECM;
	}
	else if (psDroid->asBits[COMP_CONSTRUCT].nStat != 0)
	{
		type = DROID_CONSTRUCT;
	}
	else
	{
		type = DROID_WEAPON;
	}*/
    return (*psDroid).droidType;
}
/* Return the type of a droid from it's template */
#[no_mangle]
pub unsafe extern "C" fn droidTemplateType(mut psTemplate:
                                               *mut DROID_TEMPLATE)
 -> DROID_TYPE {
    let mut type_0: DROID_TYPE = DROID_WEAPON;
    //	if (strcmp(psTemplate->pName, "BaBa People") == 0)
//	{
//		type = DROID_PERSON;
//	}
    if (*psTemplate).droidType as libc::c_uint ==
           DROID_PERSON as libc::c_int as libc::c_uint {
        type_0 = DROID_PERSON
    } else if (*psTemplate).droidType as libc::c_uint ==
                  DROID_CYBORG as libc::c_int as libc::c_uint {
        type_0 = DROID_CYBORG
    } else if (*psTemplate).droidType as libc::c_uint ==
                  DROID_CYBORG_SUPER as libc::c_int as libc::c_uint {
        type_0 = DROID_CYBORG_SUPER
    } else if (*psTemplate).droidType as libc::c_uint ==
                  DROID_CYBORG_CONSTRUCT as libc::c_int as libc::c_uint {
        type_0 = DROID_CYBORG_CONSTRUCT
    } else if (*psTemplate).droidType as libc::c_uint ==
                  DROID_CYBORG_REPAIR as libc::c_int as libc::c_uint {
        type_0 = DROID_CYBORG_REPAIR
    } else if (*psTemplate).droidType as libc::c_uint ==
                  DROID_TRANSPORTER as libc::c_int as libc::c_uint {
        type_0 = DROID_TRANSPORTER
    } else if (*psTemplate).asParts[COMP_BRAIN as libc::c_int as usize] !=
                  0 as libc::c_int {
        type_0 = DROID_COMMAND
    } else if (*asSensorStats.offset((*psTemplate).asParts[COMP_SENSOR as
                                                               libc::c_int as
                                                               usize] as
                                         isize)).location ==
                  LOC_TURRET as libc::c_int as libc::c_uint {
        type_0 = DROID_SENSOR
    } else if (*asECMStats.offset((*psTemplate).asParts[COMP_ECM as
                                                            libc::c_int as
                                                            usize] as
                                      isize)).location ==
                  LOC_TURRET as libc::c_int as libc::c_uint {
        type_0 = DROID_ECM
    } else if (*psTemplate).asParts[COMP_CONSTRUCT as libc::c_int as usize] !=
                  0 as libc::c_int {
        type_0 = DROID_CONSTRUCT
    } else if (*asRepairStats.offset((*psTemplate).asParts[COMP_REPAIRUNIT as
                                                               libc::c_int as
                                                               usize] as
                                         isize)).location ==
                  LOC_TURRET as libc::c_int as libc::c_uint {
        type_0 = DROID_REPAIR
    } else if (*psTemplate).asWeaps[0 as libc::c_int as usize] !=
                  0 as libc::c_int as libc::c_uint {
        type_0 = DROID_WEAPON
    } else { type_0 = DROID_DEFAULT }
    return type_0;
}
//else if (psTemplate->asParts[COMP_REPAIRUNIT] != 0)
//Load the weapons assigned to Droids in the Access database
#[no_mangle]
pub unsafe extern "C" fn loadDroidWeapons(mut pWeaponData: *mut libc::c_char,
                                          mut bufferSize: UDWORD) -> BOOL {
    let mut pStartWeaponData: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut NumWeapons: UDWORD = 0 as libc::c_int as UDWORD;
    let mut i: UDWORD = 0;
    let mut player: UDWORD = 0;
    let mut WeaponName: [STRING; 60] = [0; 60];
    let mut TemplateName: [STRING; 60] = [0; 60];
    let mut pTemplate: *mut DROID_TEMPLATE = 0 as *mut DROID_TEMPLATE;
    let mut recFound: BOOL = 0;
    let mut SkippedWeaponCount: UWORD = 0 as libc::c_int as UWORD;
    let mut incW: SDWORD = 0;
    //initialise the store count variable
    player = 0 as libc::c_int as UDWORD;
    while player < 8 as libc::c_int as libc::c_uint {
        pTemplate = apsDroidTemplates[player as usize];
        while !pTemplate.is_null() {
            //clear the storage flags
            (*pTemplate).storeCount = 0 as libc::c_int as UDWORD;
            pTemplate = (*pTemplate).psNext
        }
        player = player.wrapping_add(1)
    }
    pStartWeaponData = pWeaponData;
    NumWeapons = numCR(pWeaponData, bufferSize);
    i = 0 as libc::c_int as UDWORD;
    while i < NumWeapons {
        recFound = 0 as libc::c_int;
        //read the data into the storage - the data is delimeted using comma's
        TemplateName[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        WeaponName[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        sscanf(pWeaponData,
               b"%[^\',\'],%[^\',\'],%d\x00" as *const u8 as
                   *const libc::c_char, TemplateName.as_mut_ptr(),
               WeaponName.as_mut_ptr(), &mut player as *mut UDWORD);
        //loop through each droid to compare the name
        player = RemapPlayerNumber(player); // for psx ...
        /*if (!getResourceName(TemplateName))
		{
			return FALSE;
		}*/
        if getDroidResourceName(TemplateName.as_mut_ptr()) == 0 {
            return 0 as libc::c_int
        }
        if getResourceName(WeaponName.as_mut_ptr()) == 0 {
            return 0 as libc::c_int
        }
        if player < 8 as libc::c_int as libc::c_uint {
            pTemplate = apsDroidTemplates[player as usize];
            while !pTemplate.is_null() {
                //if (!(strcmp(TemplateName, pTemplate->pName)))
                if strcmp(TemplateName.as_mut_ptr(),
                          (*pTemplate).aName.as_mut_ptr()) == 0 {
                    //Template found
                    recFound = 1 as libc::c_int;
                    break ;
                } else { pTemplate = (*pTemplate).psNext }
            }
            /* if Template not found - try default design */
            if recFound == 0 {
                pTemplate = &mut sDefaultDesignTemplate;
                //if ( strcmp(TemplateName, pTemplate->pName) )
                if strcmp(TemplateName.as_mut_ptr(),
                          (*pTemplate).aName.as_mut_ptr()) != 0 {
                    debug(LOG_ERROR,
                          b"Unable to find Template - %s\x00" as *const u8 as
                              *const libc::c_char, TemplateName.as_mut_ptr());
                    abort();
                }
            }
            incW =
                getCompFromName(COMP_WEAPON as libc::c_int as UDWORD,
                                WeaponName.as_mut_ptr());
            //if weapon not found - error
            if incW == -(1 as libc::c_int) {
                debug(LOG_ERROR,
                      b"Unable to find Weapon %s for template %s\x00" as
                          *const u8 as *const libc::c_char,
                      WeaponName.as_mut_ptr(), TemplateName.as_mut_ptr());
                abort();
            } else {
                //Weapon found, alloc this to the current Template
                (*pTemplate).asWeaps[(*pTemplate).storeCount as usize] =
                    incW as UDWORD;
                //check not allocating more than allowed
                if (*pTemplate).storeCount >
                       (*pTemplate).numWeaps as SDWORD as libc::c_uint {
                    debug(LOG_ERROR,
                          b"Trying to allocate more weapons than allowed for Template - %s\x00"
                              as *const u8 as *const libc::c_char,
                          TemplateName.as_mut_ptr());
                    abort();
                }
                //check valid weapon/propulsion
                if checkValidWeaponForProp(pTemplate) == 0 {
                    // ffs
                    debug(LOG_ERROR,
                          b"Weapon is invalid for air propulsion for template %s\x00"
                              as *const u8 as *const libc::c_char,
                          (*pTemplate).aName.as_mut_ptr());
                    abort();
                }
                (*pTemplate).storeCount =
                    (*pTemplate).storeCount.wrapping_add(1)
            }
        } else { SkippedWeaponCount = SkippedWeaponCount.wrapping_add(1) }
        //increment the pointer to the start of the next record
        pWeaponData =
            strchr(pWeaponData,
                   '\n' as i32).offset(1 as libc::c_int as isize);
        i = i.wrapping_add(1)
    }
    if SkippedWeaponCount as libc::c_int > 0 as libc::c_int {
        debug(LOG_ERROR,
              b"Illegal player number in %d droid weapons\x00" as *const u8 as
                  *const libc::c_char, SkippedWeaponCount as libc::c_int);
        abort();
    }
    //	FREE(pStartWeaponData);
    return 1 as libc::c_int;
}
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
/* The range for neighbouring objects */
//used to stop structures being built too near the edge and droids being placed down - pickATile
/* Info stored for each droid neighbour */
// The neighbouring object
// The square of the distance to the object
//UDWORD			dist;			// The distance to the object
/* Store for the objects near the droid currently being updated */
// the structure that was last hit
// initialise droid module
// refresh the naybor list
// this only does anything if the naybor list is out of date
//extern BOOL loadDroidPrograms(car *pProgramData, UDWORD bufferSize);
/*initialise the template build and power points */
/*Builds an instance of a Structure - the x/y passed in are in world coords.*/
/* Set the asBits in a DROID structure given it's template. */
// calculate the experience level of a droid
/* Calculate the weight of a droid from it's template */
/* Calculate the power points required to build/maintain a droid */
/* Calculate the body points of a droid from it's template */
/* Calculate the base body points of a droid without upgrades*/
/* Calculate the base speed of a droid from it's template */
/* Calculate the speed of a droid over a terrain */
/* Calculate the points required to build the template */
/* Calculate the points required to build the droid */
//UDWORD calcDroidBuild(DROID *psDroid);
/* Calculate the power points required to build/maintain the droid */
// return whether a template is for an IDF droid
// return whether a droid is IDF
/* Do damage to a droid */
/* The main update routine for all droids */
/* Set up a droid to build a structure - returns true if successful */
/* Set up a droid to build a foundation - returns true if successful */
/* Sets a droid to start demolishing - returns true if successful */
/* Update a construction droid while it is demolishing 
   returns TRUE while demolishing */
/* Sets a droid to start repairing - returns true if successful */
/* Update a construction droid while it is repairing 
   returns TRUE while repairing */
/*Start a Repair Droid working on a damaged droid - returns TRUE if successful*/
/*Updates a Repair Droid working on a damaged droid - returns TRUE whilst repairing*/
/*checks a droids current body points to see if need to self repair*/
/* Update a construction droid while it is building 
   returns TRUE while building continues */
/* Update a construction droid while it is building a
   foundation. Returns TRUE whilst foundation continues */
/*Start a EW weapon droid working on a low resistance structure*/
/*continue restoring a structure*/
// recycle a droid (retain it's experience and some of it's cost)
/* Release all resources associated with a droid */
/* Remove a droid and free it's memory */
/* Same as destroy droid except no graphical effects */
/* Burn a barbarian then destroy it */
/* Remove a droid from the apsDroidLists so doesn't update or get drawn etc*/
//returns TRUE if successfully removed from the list
//free the storage for the droid templates
//Load the programs assigned to Droids in the Access database
/*BOOL loadDroidPrograms(char *pProgramData, UDWORD bufferSize)
{
	char				*pStartProgramData;
	UDWORD				NumPrograms = 0, i, incP, player;
	STRING				ProgramName[MAX_NAME_SIZE], TemplateName[MAX_NAME_SIZE];
	DROID_TEMPLATE		*pTemplate;
	PROGRAM_STATS		*pPrograms = asProgramStats;
	BOOL				recFound;
	UWORD				SkippedProgramCount=0;

#ifdef HASH_NAMES
	UDWORD				HashedTemplateName,HashedProgramName;
#endif

	//initialise the store count variable
	for (player=0; player < MAX_PLAYERS; player++)
	{
		for(pTemplate = apsDroidTemplates[player]; pTemplate != NULL;
			pTemplate = pTemplate->psNext)
		{
			//clear the storage flags
			pTemplate->storeCount = 0;
		}
	}

	pStartProgramData = pProgramData;

	NumPrograms = numCR(pProgramData, bufferSize);

	for (i=0; i < NumPrograms; i++)
	{
		recFound = FALSE;
		//read the data into the storage - the data is delimeted using comma's
		TemplateName[0] = '\0';
		ProgramName[0] = '\0';
		sscanf(pProgramData,"%[^','],%[^','],%d", &TemplateName, &ProgramName, &player);

		//if (!getResourceName(TemplateName))
		//{
		//	return FALSE;
		//}
#ifndef PSX
		if (!getDroidResourceName(TemplateName))
		{
			return FALSE;
		}
#endif
		if (!getResourceName(ProgramName))
		{
			return FALSE;
		}
#ifdef HASH_NAMES
		HashedTemplateName=HashString(TemplateName);
		HashedProgramName=HashString(ProgramName);
#endif

		if (player < MAX_PLAYERS)
		{
			//loop through each droid to compare the name
			for(pTemplate = apsDroidTemplates[player]; pTemplate != NULL; pTemplate =
				pTemplate->psNext)
			{

#ifdef HASH_NAMES
				if (pTemplate->NameHash==HashedTemplateName)
#else
				//if (!(strcmp(TemplateName, pTemplate->pName)))
				if (!(strcmp(TemplateName, pTemplate->aName)))
#endif
				{

					//Template found
					for (incP=0; incP < numProgramStats; incP++)
					{
#ifdef HASH_NAMES
						if (pPrograms[incP].NameHash==HashedProgramName)
#else
						if (!(strcmp(ProgramName, pPrograms[incP].pName)))
#endif
						{
							//Program found, alloc this to the current Template
							pTemplate->asProgs[pTemplate->storeCount] = incP;
							recFound = TRUE;
							//check not allocating more than allowed
							if (pTemplate->storeCount > (SDWORD)pTemplate->numProgs)
							{
								DBERROR(("Trying to allocate more programs than allowed for Template"));
								return FALSE;
							}
							pTemplate->storeCount++;
							break;
						}
					}
					//if program not found - error
					if (!recFound)
					{
						DBERROR(("Unable to find Program"));
						return FALSE;
					}
					else
					{
						break;
					}
				}
			}

			//if Template not found - error
			if (!recFound)
			{
				DBERROR(("Unable to allocate all Template programs"));
				return FALSE;
			}

		}
		else
		{
			SkippedProgramCount++;
		}

		//increment the pointer to the start of the next record
		pProgramData = strchr(pProgramData,'\n') + 1;
	}

	if (SkippedProgramCount>0)
	{
		DBERROR(("Illegal player number in %d droid programs",SkippedProgramCount));
	}

//	FREE(pStartProgramData);
	return TRUE;
}*/
//free the storage for the droid templates
#[no_mangle]
pub unsafe extern "C" fn droidTemplateShutDown() -> BOOL {
    let mut pTemplate: *mut DROID_TEMPLATE = 0 as *mut DROID_TEMPLATE;
    let mut pNext: *mut DROID_TEMPLATE = 0 as *mut DROID_TEMPLATE;
    let mut player: UDWORD = 0;
    player = 0 as libc::c_int as UDWORD;
    while player < 8 as libc::c_int as libc::c_uint {
        pTemplate = apsDroidTemplates[player as usize];
        while !pTemplate.is_null() {
            pNext = (*pTemplate).psNext;
            heapFree(psTemplateHeap, pTemplate as *mut libc::c_void);
            pTemplate = pNext
        }
        apsDroidTemplates[player as usize] = 0 as *mut DROID_TEMPLATE;
        player = player.wrapping_add(1)
    }
    return 1 as libc::c_int;
}
/* Calculate the weight of a droid from it's template */
#[no_mangle]
pub unsafe extern "C" fn calcDroidWeight(mut psTemplate: *mut DROID_TEMPLATE)
 -> UDWORD {
    let mut weight: UDWORD = 0;
    let mut i: UDWORD = 0;
    /* Get the basic component weight */
    weight =
        (*asBodyStats.offset((*psTemplate).asParts[COMP_BODY as libc::c_int as
                                                       usize] as
                                 isize)).weight.wrapping_add((*asBrainStats.offset((*psTemplate).asParts[COMP_BRAIN
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             usize]
                                                                                       as
                                                                                       isize)).weight).wrapping_add((*asSensorStats.offset((*psTemplate).asParts[COMP_SENSOR
                                                                                                                                                                     as
                                                                                                                                                                     libc::c_int
                                                                                                                                                                     as
                                                                                                                                                                     usize]
                                                                                                                                               as
                                                                                                                                               isize)).weight).wrapping_add((*asECMStats.offset((*psTemplate).asParts[COMP_ECM
                                                                                                                                                                                                                          as
                                                                                                                                                                                                                          libc::c_int
                                                                                                                                                                                                                          as
                                                                                                                                                                                                                          usize]
                                                                                                                                                                                                    as
                                                                                                                                                                                                    isize)).weight).wrapping_add((*asRepairStats.offset((*psTemplate).asParts[COMP_REPAIRUNIT
                                                                                                                                                                                                                                                                                  as
                                                                                                                                                                                                                                                                                  libc::c_int
                                                                                                                                                                                                                                                                                  as
                                                                                                                                                                                                                                                                                  usize]
                                                                                                                                                                                                                                                            as
                                                                                                                                                                                                                                                            isize)).weight).wrapping_add((*asConstructStats.offset((*psTemplate).asParts[COMP_CONSTRUCT
                                                                                                                                                                                                                                                                                                                                             as
                                                                                                                                                                                                                                                                                                                                             libc::c_int
                                                                                                                                                                                                                                                                                                                                             as
                                                                                                                                                                                                                                                                                                                                             usize]
                                                                                                                                                                                                                                                                                                                       as
                                                                                                                                                                                                                                                                                                                       isize)).weight);
    /* propulsion weight is a percentage of the body weight */
    weight =
        (weight as
             libc::c_uint).wrapping_add((*asPropulsionStats.offset((*psTemplate).asParts[COMP_PROPULSION
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             usize]
                                                                       as
                                                                       isize)).weight.wrapping_mul((*asBodyStats.offset((*psTemplate).asParts[COMP_BODY
                                                                                                                                                  as
                                                                                                                                                  libc::c_int
                                                                                                                                                  as
                                                                                                                                                  usize]
                                                                                                                            as
                                                                                                                            isize)).weight).wrapping_div(100
                                                                                                                                                             as
                                                                                                                                                             libc::c_int
                                                                                                                                                             as
                                                                                                                                                             libc::c_uint))
            as UDWORD as UDWORD;
    /* Add the weapon weight */
    i = 0 as libc::c_int as UDWORD;
    while i < (*psTemplate).numWeaps {
        weight =
            (weight as
                 libc::c_uint).wrapping_add((*asWeaponStats.offset((*psTemplate).asWeaps[i
                                                                                             as
                                                                                             usize]
                                                                       as
                                                                       isize)).weight)
                as UDWORD as UDWORD;
        i = i.wrapping_add(1)
    }
    return weight;
}
/* Calculate the body points of a droid from it's template */
#[no_mangle]
pub unsafe extern "C" fn calcTemplateBody(mut psTemplate: *mut DROID_TEMPLATE,
                                          mut player: UBYTE) -> UDWORD {
    let mut body: UDWORD = 0;
    let mut i: UDWORD = 0;
    if psTemplate.is_null() { return 0 as libc::c_int as UDWORD }
    /* Get the basic component body points */
    body =
        (*asBodyStats.offset((*psTemplate).asParts[COMP_BODY as libc::c_int as
                                                       usize] as
                                 isize)).body.wrapping_add((*asBrainStats.offset((*psTemplate).asParts[COMP_BRAIN
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           usize]
                                                                                     as
                                                                                     isize)).body).wrapping_add((*asSensorStats.offset((*psTemplate).asParts[COMP_SENSOR
                                                                                                                                                                 as
                                                                                                                                                                 libc::c_int
                                                                                                                                                                 as
                                                                                                                                                                 usize]
                                                                                                                                           as
                                                                                                                                           isize)).body).wrapping_add((*asECMStats.offset((*psTemplate).asParts[COMP_ECM
                                                                                                                                                                                                                    as
                                                                                                                                                                                                                    libc::c_int
                                                                                                                                                                                                                    as
                                                                                                                                                                                                                    usize]
                                                                                                                                                                                              as
                                                                                                                                                                                              isize)).body).wrapping_add((*asRepairStats.offset((*psTemplate).asParts[COMP_REPAIRUNIT
                                                                                                                                                                                                                                                                          as
                                                                                                                                                                                                                                                                          libc::c_int
                                                                                                                                                                                                                                                                          as
                                                                                                                                                                                                                                                                          usize]
                                                                                                                                                                                                                                                    as
                                                                                                                                                                                                                                                    isize)).body).wrapping_add((*asConstructStats.offset((*psTemplate).asParts[COMP_CONSTRUCT
                                                                                                                                                                                                                                                                                                                                   as
                                                                                                                                                                                                                                                                                                                                   libc::c_int
                                                                                                                                                                                                                                                                                                                                   as
                                                                                                                                                                                                                                                                                                                                   usize]
                                                                                                                                                                                                                                                                                                             as
                                                                                                                                                                                                                                                                                                             isize)).body);
    /* propulsion body points are a percentage of the bodys' body points */
    body =
        (body as
             libc::c_uint).wrapping_add((*asPropulsionStats.offset((*psTemplate).asParts[COMP_PROPULSION
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             usize]
                                                                       as
                                                                       isize)).body.wrapping_mul((*asBodyStats.offset((*psTemplate).asParts[COMP_BODY
                                                                                                                                                as
                                                                                                                                                libc::c_int
                                                                                                                                                as
                                                                                                                                                usize]
                                                                                                                          as
                                                                                                                          isize)).body).wrapping_div(100
                                                                                                                                                         as
                                                                                                                                                         libc::c_int
                                                                                                                                                         as
                                                                                                                                                         libc::c_uint))
            as UDWORD as UDWORD;
    /* Add the weapon body points */
    i = 0 as libc::c_int as UDWORD;
    while i < (*psTemplate).numWeaps {
        body =
            (body as
                 libc::c_uint).wrapping_add((*asWeaponStats.offset((*psTemplate).asWeaps[i
                                                                                             as
                                                                                             usize]
                                                                       as
                                                                       isize)).body)
                as UDWORD as UDWORD;
        i = i.wrapping_add(1)
    }
    //add on any upgrade value that may need to be applied
    body =
        (body as
             libc::c_uint).wrapping_add(body.wrapping_mul((*asBodyUpgrade[player
                                                                              as
                                                                              usize].as_mut_ptr()).body
                                                              as
                                                              libc::c_uint).wrapping_div(100
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_uint))
            as UDWORD as UDWORD;
    return body;
}
/* Calculate the base body points of a droid without upgrades*/
#[no_mangle]
pub unsafe extern "C" fn calcDroidBaseBody(mut psDroid: *mut DROID)
 -> UDWORD {
    let mut body: UDWORD = 0; //, i;
    if psDroid.is_null() { return 0 as libc::c_int as UDWORD }
    /* Get the basic component body points */
    body =
        (*asBodyStats.offset((*psDroid).asBits[COMP_BODY as libc::c_int as
                                                   usize].nStat as libc::c_int
                                 as
                                 isize)).body.wrapping_add((*asBrainStats.offset((*psDroid).asBits[COMP_BRAIN
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       usize].nStat
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize)).body).wrapping_add((*asSensorStats.offset((*psDroid).asBits[COMP_SENSOR
                                                                                                                                                             as
                                                                                                                                                             libc::c_int
                                                                                                                                                             as
                                                                                                                                                             usize].nStat
                                                                                                                                           as
                                                                                                                                           libc::c_int
                                                                                                                                           as
                                                                                                                                           isize)).body).wrapping_add((*asECMStats.offset((*psDroid).asBits[COMP_ECM
                                                                                                                                                                                                                as
                                                                                                                                                                                                                libc::c_int
                                                                                                                                                                                                                as
                                                                                                                                                                                                                usize].nStat
                                                                                                                                                                                              as
                                                                                                                                                                                              libc::c_int
                                                                                                                                                                                              as
                                                                                                                                                                                              isize)).body).wrapping_add((*asRepairStats.offset((*psDroid).asBits[COMP_REPAIRUNIT
                                                                                                                                                                                                                                                                      as
                                                                                                                                                                                                                                                                      libc::c_int
                                                                                                                                                                                                                                                                      as
                                                                                                                                                                                                                                                                      usize].nStat
                                                                                                                                                                                                                                                    as
                                                                                                                                                                                                                                                    libc::c_int
                                                                                                                                                                                                                                                    as
                                                                                                                                                                                                                                                    isize)).body).wrapping_add((*asConstructStats.offset((*psDroid).asBits[COMP_CONSTRUCT
                                                                                                                                                                                                                                                                                                                               as
                                                                                                                                                                                                                                                                                                                               libc::c_int
                                                                                                                                                                                                                                                                                                                               as
                                                                                                                                                                                                                                                                                                                               usize].nStat
                                                                                                                                                                                                                                                                                                             as
                                                                                                                                                                                                                                                                                                             libc::c_int
                                                                                                                                                                                                                                                                                                             as
                                                                                                                                                                                                                                                                                                             isize)).body);
    /* propulsion body points are a percentage of the bodys' body points */
    body =
        (body as
             libc::c_uint).wrapping_add((*asPropulsionStats.offset((*psDroid).asBits[COMP_PROPULSION
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         usize].nStat
                                                                       as
                                                                       libc::c_int
                                                                       as
                                                                       isize)).body.wrapping_mul((*asBodyStats.offset((*psDroid).asBits[COMP_BODY
                                                                                                                                            as
                                                                                                                                            libc::c_int
                                                                                                                                            as
                                                                                                                                            usize].nStat
                                                                                                                          as
                                                                                                                          libc::c_int
                                                                                                                          as
                                                                                                                          isize)).body).wrapping_div(100
                                                                                                                                                         as
                                                                                                                                                         libc::c_int
                                                                                                                                                         as
                                                                                                                                                         libc::c_uint))
            as UDWORD as UDWORD;
    /* Add the weapon body points */
	//for(i=0; i<psDroid->numWeaps; i++)
    if (*psDroid).asWeaps[0 as libc::c_int as usize].nStat >
           0 as libc::c_int as libc::c_uint {
        //body += (asWeaponStats + psDroid->asWeaps[i].nStat)->body;
        body =
            (body as
                 libc::c_uint).wrapping_add((*asWeaponStats.offset((*psDroid).asWeaps[0
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          usize].nStat
                                                                       as
                                                                       isize)).body)
                as UDWORD as UDWORD
    }
    return body;
}
/* Calculate the base speed of a droid from it's template */
#[no_mangle]
pub unsafe extern "C" fn calcDroidBaseSpeed(mut psTemplate:
                                                *mut DROID_TEMPLATE,
                                            mut weight: UDWORD,
                                            mut player: UBYTE) -> UDWORD {
    let mut speed: UDWORD = 0;
    //return ((asBodyStats + psTemplate->asParts[COMP_BODY])->
	//	powerOutput * 100) / weight;
	/*if (psTemplate->droidType == DROID_CYBORG)
	{
		return (bodyPower(asBodyStats + psTemplate->asParts[COMP_BODY],
			player, CYBORG_BODY_UPGRADE) * 100) / weight;
	}
	else
	{
		return (bodyPower(asBodyStats + psTemplate->asParts[COMP_BODY],
			player, DROID_BODY_UPGRADE) * 100) / weight;
	}*/
    if (*psTemplate).droidType as libc::c_uint ==
           DROID_CYBORG as libc::c_int as libc::c_uint ||
           (*psTemplate).droidType as libc::c_uint ==
               DROID_CYBORG_SUPER as libc::c_int as libc::c_uint ||
           (*psTemplate).droidType as libc::c_uint ==
               DROID_CYBORG_CONSTRUCT as libc::c_int as libc::c_uint ||
           (*psTemplate).droidType as libc::c_uint ==
               DROID_CYBORG_REPAIR as libc::c_int as libc::c_uint {
        speed =
            ((*asPropulsionTypes.offset((*asPropulsionStats.offset((*psTemplate).asParts[COMP_PROPULSION
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             usize]
                                                                       as
                                                                       isize)).propulsionType
                                            as isize)).powerRatioMult as
                 libc::c_uint).wrapping_mul(bodyPower(asBodyStats.offset((*psTemplate).asParts[COMP_BODY
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   usize]
                                                                             as
                                                                             isize),
                                                      player,
                                                      1 as libc::c_int as
                                                          UBYTE)).wrapping_div(weight)
    } else {
        speed =
            ((*asPropulsionTypes.offset((*asPropulsionStats.offset((*psTemplate).asParts[COMP_PROPULSION
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             usize]
                                                                       as
                                                                       isize)).propulsionType
                                            as isize)).powerRatioMult as
                 libc::c_uint).wrapping_mul(bodyPower(asBodyStats.offset((*psTemplate).asParts[COMP_BODY
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   usize]
                                                                             as
                                                                             isize),
                                                      player,
                                                      0 as libc::c_int as
                                                          UBYTE)).wrapping_div(weight)
    }
    // reduce the speed of medium/heavy VTOLs
    if (*asPropulsionStats.offset((*psTemplate).asParts[COMP_PROPULSION as
                                                            libc::c_int as
                                                            usize] as
                                      isize)).propulsionType as libc::c_int ==
           LIFT as libc::c_int {
        if (*asBodyStats.offset((*psTemplate).asParts[COMP_BODY as libc::c_int
                                                          as usize] as
                                    isize)).size as libc::c_int ==
               SIZE_HEAVY as libc::c_int {
            speed =
                (speed as
                     libc::c_uint).wrapping_div(4 as libc::c_int as
                                                    libc::c_uint) as UDWORD as
                    UDWORD
        } else if (*asBodyStats.offset((*psTemplate).asParts[COMP_BODY as
                                                                 libc::c_int
                                                                 as usize] as
                                           isize)).size as libc::c_int ==
                      SIZE_MEDIUM as libc::c_int {
            speed =
                (3 as libc::c_int as
                     libc::c_uint).wrapping_mul(speed).wrapping_div(4 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_uint)
        }
    }
    return speed;
}
/* Calculate the speed of a droid over a terrain */
#[no_mangle]
pub unsafe extern "C" fn calcDroidSpeed(mut baseSpeed: UDWORD,
                                        mut terrainType: UDWORD,
                                        mut propIndex: UDWORD) -> UDWORD {
    let mut droidSpeed: UDWORD = 0;
    //	return baseSpeed * getSpeedFactor(terrainType,
//		(asPropulsionStats + propIndex)->propulsionType) / 100;
    //need to ensure doesn't go over the max speed possible for this propulsion
    droidSpeed =
        baseSpeed.wrapping_mul(getSpeedFactor(terrainType,
                                              (*asPropulsionStats.offset(propIndex
                                                                             as
                                                                             isize)).propulsionType
                                                  as
                                                  UDWORD)).wrapping_div(100 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint);
    if droidSpeed > (*asPropulsionStats.offset(propIndex as isize)).maxSpeed {
        droidSpeed = (*asPropulsionStats.offset(propIndex as isize)).maxSpeed
    }
    return droidSpeed;
}
/* Calculate the points required to build the template - used to calculate time*/
#[no_mangle]
pub unsafe extern "C" fn calcTemplateBuild(mut psTemplate:
                                               *mut DROID_TEMPLATE)
 -> UDWORD {
    let mut build: UDWORD = 0;
    let mut i: UDWORD = 0;
    build =
        (*asBodyStats.offset((*psTemplate).asParts[COMP_BODY as libc::c_int as
                                                       usize] as
                                 isize)).buildPoints.wrapping_add((*asBrainStats.offset((*psTemplate).asParts[COMP_BRAIN
                                                                                                                  as
                                                                                                                  libc::c_int
                                                                                                                  as
                                                                                                                  usize]
                                                                                            as
                                                                                            isize)).buildPoints).wrapping_add((*asSensorStats.offset((*psTemplate).asParts[COMP_SENSOR
                                                                                                                                                                               as
                                                                                                                                                                               libc::c_int
                                                                                                                                                                               as
                                                                                                                                                                               usize]
                                                                                                                                                         as
                                                                                                                                                         isize)).buildPoints).wrapping_add((*asECMStats.offset((*psTemplate).asParts[COMP_ECM
                                                                                                                                                                                                                                         as
                                                                                                                                                                                                                                         libc::c_int
                                                                                                                                                                                                                                         as
                                                                                                                                                                                                                                         usize]
                                                                                                                                                                                                                   as
                                                                                                                                                                                                                   isize)).buildPoints).wrapping_add((*asRepairStats.offset((*psTemplate).asParts[COMP_REPAIRUNIT
                                                                                                                                                                                                                                                                                                      as
                                                                                                                                                                                                                                                                                                      libc::c_int
                                                                                                                                                                                                                                                                                                      as
                                                                                                                                                                                                                                                                                                      usize]
                                                                                                                                                                                                                                                                                as
                                                                                                                                                                                                                                                                                isize)).buildPoints).wrapping_add((*asConstructStats.offset((*psTemplate).asParts[COMP_CONSTRUCT
                                                                                                                                                                                                                                                                                                                                                                      as
                                                                                                                                                                                                                                                                                                                                                                      libc::c_int
                                                                                                                                                                                                                                                                                                                                                                      as
                                                                                                                                                                                                                                                                                                                                                                      usize]
                                                                                                                                                                                                                                                                                                                                                as
                                                                                                                                                                                                                                                                                                                                                isize)).buildPoints);
    /* propulsion build points are a percentage of the bodys' build points */
    build =
        (build as
             libc::c_uint).wrapping_add((*asPropulsionStats.offset((*psTemplate).asParts[COMP_PROPULSION
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             usize]
                                                                       as
                                                                       isize)).buildPoints.wrapping_mul((*asBodyStats.offset((*psTemplate).asParts[COMP_BODY
                                                                                                                                                       as
                                                                                                                                                       libc::c_int
                                                                                                                                                       as
                                                                                                                                                       usize]
                                                                                                                                 as
                                                                                                                                 isize)).buildPoints).wrapping_div(100
                                                                                                                                                                       as
                                                                                                                                                                       libc::c_int
                                                                                                                                                                       as
                                                                                                                                                                       libc::c_uint))
            as UDWORD as UDWORD;
    //add weapon power
    i = 0 as libc::c_int as UDWORD;
    while i < (*psTemplate).numWeaps {
        if (*psTemplate).asWeaps[i as usize] < numWeaponStats {
        } else {
            debug(LOG_ERROR,
                  b"Invalid Template weapon for %s\x00" as *const u8 as
                      *const libc::c_char, getTemplateName(psTemplate));
        };
        if (*psTemplate).asWeaps[i as usize] < numWeaponStats {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"droid.c\x00" as *const u8 as *const libc::c_char,
                  4007 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 18],
                                            &[libc::c_char; 18]>(b"calcTemplateBuild\x00")).as_ptr(),
                  b"psTemplate->asWeaps[i]<numWeaponStats\x00" as *const u8 as
                      *const libc::c_char);
        };
        build =
            (build as
                 libc::c_uint).wrapping_add((*asWeaponStats.offset((*psTemplate).asWeaps[i
                                                                                             as
                                                                                             usize]
                                                                       as
                                                                       isize)).buildPoints)
                as UDWORD as UDWORD;
        i = i.wrapping_add(1)
    }
    //add program power
	/*for(i=0; i<psTemplate->numProgs; i++)
	{
		ASSERT( psTemplate->asProgs[i]<numProgramStats,
			//"Invalid Template program for %s", psTemplate->pName );
			"Invalid Template program for %s", getTemplateName(psTemplate)));
		build += (asProgramStats + psTemplate->asProgs[i])->buildPoints;
	}*/
    return build;
}
/* Calculate the points required to build the droid */
/*UDWORD calcDroidBuild(DROID *psDroid)
{
	UDWORD	build, i;

	build = (asBodyStats + psDroid->asBits[COMP_BODY].nStat)->buildPoints +
	(asBrainStats + psDroid->asBits[COMP_BRAIN].nStat)->buildPoints +
	(asPropulsionStats + psDroid->asBits[COMP_PROPULSION].nStat)->buildPoints +
	(asSensorStats + psDroid->asBits[COMP_SENSOR].nStat)->buildPoints +
	(asECMStats + psDroid->asBits[COMP_ECM].nStat)->buildPoints +
	(asRepairStats + psDroid->asBits[COMP_REPAIRUNIT].nStat)->buildPoints;

	//add weapon power
	for(i=0; i<psDroid->numWeaps; i++)
	{
		build += (asWeaponStats + psDroid->asWeaps[i].nStat)->buildPoints;
	}

	//add program power
	for(i=0; i<psDroid->numProgs; i++)
	{
		build += psDroid->asProgs[i].psStats->buildPoints;
	}

	return build;
}*/
/* Calculate the power points required to build/maintain a template */
#[no_mangle]
pub unsafe extern "C" fn calcTemplatePower(mut psTemplate:
                                               *mut DROID_TEMPLATE)
 -> UDWORD {
    let mut power: UDWORD = 0;
    let mut i: UDWORD = 0;
    //get the component power
    power =
        (*asBodyStats.offset((*psTemplate).asParts[COMP_BODY as libc::c_int as
                                                       usize] as
                                 isize)).buildPower.wrapping_add((*asBrainStats.offset((*psTemplate).asParts[COMP_BRAIN
                                                                                                                 as
                                                                                                                 libc::c_int
                                                                                                                 as
                                                                                                                 usize]
                                                                                           as
                                                                                           isize)).buildPower).wrapping_add((*asSensorStats.offset((*psTemplate).asParts[COMP_SENSOR
                                                                                                                                                                             as
                                                                                                                                                                             libc::c_int
                                                                                                                                                                             as
                                                                                                                                                                             usize]
                                                                                                                                                       as
                                                                                                                                                       isize)).buildPower).wrapping_add((*asECMStats.offset((*psTemplate).asParts[COMP_ECM
                                                                                                                                                                                                                                      as
                                                                                                                                                                                                                                      libc::c_int
                                                                                                                                                                                                                                      as
                                                                                                                                                                                                                                      usize]
                                                                                                                                                                                                                as
                                                                                                                                                                                                                isize)).buildPower).wrapping_add((*asRepairStats.offset((*psTemplate).asParts[COMP_REPAIRUNIT
                                                                                                                                                                                                                                                                                                  as
                                                                                                                                                                                                                                                                                                  libc::c_int
                                                                                                                                                                                                                                                                                                  as
                                                                                                                                                                                                                                                                                                  usize]
                                                                                                                                                                                                                                                                            as
                                                                                                                                                                                                                                                                            isize)).buildPower).wrapping_add((*asConstructStats.offset((*psTemplate).asParts[COMP_CONSTRUCT
                                                                                                                                                                                                                                                                                                                                                                 as
                                                                                                                                                                                                                                                                                                                                                                 libc::c_int
                                                                                                                                                                                                                                                                                                                                                                 as
                                                                                                                                                                                                                                                                                                                                                                 usize]
                                                                                                                                                                                                                                                                                                                                           as
                                                                                                                                                                                                                                                                                                                                           isize)).buildPower);
    /* propulsion power points are a percentage of the bodys' power points */
    power =
        (power as
             libc::c_uint).wrapping_add((*asPropulsionStats.offset((*psTemplate).asParts[COMP_PROPULSION
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             usize]
                                                                       as
                                                                       isize)).buildPower.wrapping_mul((*asBodyStats.offset((*psTemplate).asParts[COMP_BODY
                                                                                                                                                      as
                                                                                                                                                      libc::c_int
                                                                                                                                                      as
                                                                                                                                                      usize]
                                                                                                                                as
                                                                                                                                isize)).buildPower).wrapping_div(100
                                                                                                                                                                     as
                                                                                                                                                                     libc::c_int
                                                                                                                                                                     as
                                                                                                                                                                     libc::c_uint))
            as UDWORD as UDWORD;
    //add weapon power
    i = 0 as libc::c_int as UDWORD;
    while i < (*psTemplate).numWeaps {
        power =
            (power as
                 libc::c_uint).wrapping_add((*asWeaponStats.offset((*psTemplate).asWeaps[i
                                                                                             as
                                                                                             usize]
                                                                       as
                                                                       isize)).buildPower)
                as UDWORD as UDWORD;
        i = i.wrapping_add(1)
    }
    //add program power
	/*for(i=0; i<psTemplate->numProgs; i++)
	{
		power += (asProgramStats + psTemplate->asProgs[i])->buildPower;
	}*/
    return power;
}
/* Calculate the power points required to build/maintain a droid */
#[no_mangle]
pub unsafe extern "C" fn calcDroidPower(mut psDroid: *mut DROID) -> UDWORD {
    let mut power: UDWORD = 0; //, i;
    //get the component power
    power =
        (*asBodyStats.offset((*psDroid).asBits[COMP_BODY as libc::c_int as
                                                   usize].nStat as libc::c_int
                                 as
                                 isize)).buildPower.wrapping_add((*asBrainStats.offset((*psDroid).asBits[COMP_BRAIN
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             usize].nStat
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           isize)).buildPower).wrapping_add((*asSensorStats.offset((*psDroid).asBits[COMP_SENSOR
                                                                                                                                                                         as
                                                                                                                                                                         libc::c_int
                                                                                                                                                                         as
                                                                                                                                                                         usize].nStat
                                                                                                                                                       as
                                                                                                                                                       libc::c_int
                                                                                                                                                       as
                                                                                                                                                       isize)).buildPower).wrapping_add((*asECMStats.offset((*psDroid).asBits[COMP_ECM
                                                                                                                                                                                                                                  as
                                                                                                                                                                                                                                  libc::c_int
                                                                                                                                                                                                                                  as
                                                                                                                                                                                                                                  usize].nStat
                                                                                                                                                                                                                as
                                                                                                                                                                                                                libc::c_int
                                                                                                                                                                                                                as
                                                                                                                                                                                                                isize)).buildPower).wrapping_add((*asRepairStats.offset((*psDroid).asBits[COMP_REPAIRUNIT
                                                                                                                                                                                                                                                                                              as
                                                                                                                                                                                                                                                                                              libc::c_int
                                                                                                                                                                                                                                                                                              as
                                                                                                                                                                                                                                                                                              usize].nStat
                                                                                                                                                                                                                                                                            as
                                                                                                                                                                                                                                                                            libc::c_int
                                                                                                                                                                                                                                                                            as
                                                                                                                                                                                                                                                                            isize)).buildPower).wrapping_add((*asConstructStats.offset((*psDroid).asBits[COMP_CONSTRUCT
                                                                                                                                                                                                                                                                                                                                                             as
                                                                                                                                                                                                                                                                                                                                                             libc::c_int
                                                                                                                                                                                                                                                                                                                                                             as
                                                                                                                                                                                                                                                                                                                                                             usize].nStat
                                                                                                                                                                                                                                                                                                                                           as
                                                                                                                                                                                                                                                                                                                                           libc::c_int
                                                                                                                                                                                                                                                                                                                                           as
                                                                                                                                                                                                                                                                                                                                           isize)).buildPower);
    /* propulsion power points are a percentage of the bodys' power points */
    power =
        (power as
             libc::c_uint).wrapping_add((*asPropulsionStats.offset((*psDroid).asBits[COMP_PROPULSION
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         usize].nStat
                                                                       as
                                                                       libc::c_int
                                                                       as
                                                                       isize)).buildPower.wrapping_mul((*asBodyStats.offset((*psDroid).asBits[COMP_BODY
                                                                                                                                                  as
                                                                                                                                                  libc::c_int
                                                                                                                                                  as
                                                                                                                                                  usize].nStat
                                                                                                                                as
                                                                                                                                libc::c_int
                                                                                                                                as
                                                                                                                                isize)).buildPower).wrapping_div(100
                                                                                                                                                                     as
                                                                                                                                                                     libc::c_int
                                                                                                                                                                     as
                                                                                                                                                                     libc::c_uint))
            as UDWORD as UDWORD;
    //add weapon power
	//for(i=0; i<psDroid->numWeaps; i++)
    if (*psDroid).asWeaps[0 as libc::c_int as usize].nStat >
           0 as libc::c_int as libc::c_uint {
        //power += (asWeaponStats + psDroid->asWeaps[i].nStat)->buildPower;
        power =
            (power as
                 libc::c_uint).wrapping_add((*asWeaponStats.offset((*psDroid).asWeaps[0
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          usize].nStat
                                                                       as
                                                                       isize)).buildPower)
                as UDWORD as UDWORD
    }
    //add program power
/*	for(i=0; i<psDroid->numProgs; i++)
	{
		power += (asProgramStats + psDroid->asProgs[i])->buildPower;
	}*/
    return power;
}
//Builds an instance of a Droid - the x/y passed in are in world coords.
#[no_mangle]
pub unsafe extern "C" fn buildDroid(mut pTemplate: *mut DROID_TEMPLATE,
                                    mut x: UDWORD, mut y: UDWORD,
                                    mut player: UDWORD, mut onMission: BOOL)
 -> *mut DROID {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut psGrp: *mut DROID_GROUP = 0 as *mut DROID_GROUP;
    //DROID_TEMPLATE	*psCurrTempl, *psPrevTempl;
    let mut inc: UDWORD = 0;
    //	UDWORD			mapX, mapY;
    let mut numKills: UDWORD = 0;
    let mut i: SDWORD = 0;
    let mut experienceLoc: SDWORD = 0;
    //	UDWORD			tileX,tileY;
//	BOOL			gotPos;
//	UDWORD			numIts;
    /*
	if(bMultiPlayer)
	{
		for(psDroid=apsDroidLists[selectedPlayer],i=0;
		psDroid; psDroid = psDroid->psNext,i++)
		{
		}
		if(i>=80)
		{
			if(player == selectedPlayer)
			{
				addConsoleMessage("Cannot build new unit - Command Control limit reached (DEMO)",LEFT_JUSTIFY);
				return(NULL);
			}
		}
	}
	*/
    //don't worry if not on homebase cos not being drawn yet
// this check no longer required coz John says so JPS 7Jan99
/*
	if (!onMission)
	{
		if ( TILE_OCCUPIED(mapTile(x >> TILE_SHIFT, y >> TILE_SHIFT)) )
		{
			DBPRINTF(("droid build: tile occupied\n"));
			ASSERT( FALSE,"Can't build a droid cos there's somthing here" );
			return NULL;
		}
	}
*/
    //allocate memory
    if createDroid(player, &mut psDroid) == 0 {
        debug(LOG_NEVER,
              b"unit build: unable to create\n\x00" as *const u8 as
                  *const libc::c_char);
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Cannot get the memory for the unit\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"droid.c\x00" as *const u8 as *const libc::c_char,
                  4172 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 11],
                                            &[libc::c_char; 11]>(b"buildDroid\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as *mut DROID
    }
    //THIS IS DONE BEFORE STARTING TO BUILD NOW IN STRUCTURE.C AB 14/05/98
	/*subtract the power required to build*/
	/*if (!usePower(player, pTemplate->powerPoints))
	{
//		addGameMessage("droid build: no power", 1000, TRUE);
		// Added next line so that we don't see this message for other players
		if(player == selectedPlayer)
		{
			addConsoleMessage("Droid build: No Power",DEFAULT_JUSTIFY);
		}
		HEAP_FREE(psDroidHeap, psDroid);
		return NULL;
	}*/
    //fill in other details
	//psDroid->pTemplate = pTemplate;
    //droidSetName(psDroid,pTemplate->pName);
    droidSetName(psDroid, (*pTemplate).aName.as_mut_ptr());
    // Set the droids type
    (*psDroid).droidType = droidTemplateType(pTemplate);
    //legacy code
//	psDroid->x = (UWORD)((x & (~TILE_MASK)) + TILE_UNITS/2);
//	psDroid->y = (UWORD)((y & (~TILE_MASK)) + TILE_UNITS/2);
    (*psDroid).x = x as UWORD;
    (*psDroid).y = y as UWORD;
    //don't worry if not on homebase cos not being drawn yet
    if onMission == 0 {
        //		mapX = psDroid->x >> TILE_SHIFT;
//		mapY = psDroid->y >> TILE_SHIFT;
//		psDroid->lastTile = mapTile(mapX,mapY);
		//set droid height
        (*psDroid).z =
            map_Height((*psDroid).x as UDWORD, (*psDroid).y as UDWORD) as
                UWORD
    }
    //	psDroid->x = (UWORD)((x & (~TILE_MASK)) + TILE_UNITS/2);
//	psDroid->y = (UWORD)((y & (~TILE_MASK)) + TILE_UNITS/2);
//	psDroid->x = (UWORD)x;
//	psDroid->y = (UWORD)y;
//	DBPRINTF(("new droid = %p height=%d\n",psDroid,psDroid->z);
    (*psDroid).cluster = 0 as libc::c_int as UBYTE;
    (*psDroid).psGroup = 0 as *mut _droid_group;
    (*psDroid).psGrpNext = 0 as *mut _droid;
    if (*psDroid).droidType as libc::c_uint ==
           DROID_TRANSPORTER as libc::c_int as libc::c_uint ||
           (*psDroid).droidType as libc::c_uint ==
               DROID_COMMAND as libc::c_int as libc::c_uint {
        if grpCreate(&mut psGrp) == 0 {
            debug(LOG_NEVER,
                  b"unit build: unable to create group\n\x00" as *const u8 as
                      *const libc::c_char);
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Can\'t create unit because can\'t create group\x00" as
                          *const u8 as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"droid.c\x00" as *const u8 as *const libc::c_char,
                      4234 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 11],
                                                &[libc::c_char; 11]>(b"buildDroid\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            heapFree(psDroidHeap, psDroid as *mut libc::c_void);
            return 0 as *mut DROID
        }
        grpJoin(psGrp, psDroid);
    }
    (*psDroid).order = DORDER_NONE as libc::c_int;
    (*psDroid).orderX = 0 as libc::c_int as UWORD;
    (*psDroid).orderY = 0 as libc::c_int as UWORD;
    (*psDroid).orderX2 = 0 as libc::c_int as UWORD;
    (*psDroid).orderY2 = 0 as libc::c_int as UWORD;
    (*psDroid).psTarget = 0 as *mut _base_object;
    (*psDroid).psTarStats = 0 as *mut _base_stats;
    (*psDroid).secondaryOrder =
        (DSS_ARANGE_DEFAULT as libc::c_int | DSS_REPLEV_NEVER as libc::c_int |
             DSS_ALEV_ALWAYS as libc::c_int | DSS_HALT_GUARD as libc::c_int)
            as UDWORD;
    (*psDroid).action = DACTION_NONE as libc::c_int;
    (*psDroid).actionX = 0 as libc::c_int as UDWORD;
    (*psDroid).actionY = 0 as libc::c_int as UDWORD;
    (*psDroid).psActionTarget = 0 as *mut _base_object;
    // ffs je
    (*psDroid).listSize = 0 as libc::c_int;
    memset((*psDroid).asOrderList.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           (::std::mem::size_of::<ORDER_LIST>() as
                libc::c_ulong).wrapping_mul(10 as libc::c_int as
                                                libc::c_uint));
    (*psDroid).iAudioID = NO_SOUND as libc::c_int;
    (*psDroid).lastSync = 0 as libc::c_int as UDWORD;
    //	psDroid->activeWeapon = -1;
	//psDroid->activeProg = -1;
    (*psDroid).psCurAnim = 0 as *mut ANIM_OBJECT;
    (*psDroid).group = 0xff as libc::c_int as UBYTE;
    (*psDroid).psBaseStruct = 0 as *mut _structure;
    // find the highest stored experience
    if (*psDroid).droidType as libc::c_uint !=
           DROID_CONSTRUCT as libc::c_int as libc::c_uint &&
           (*psDroid).droidType as libc::c_uint !=
               DROID_CYBORG_CONSTRUCT as libc::c_int as libc::c_uint &&
           (*psDroid).droidType as libc::c_uint !=
               DROID_REPAIR as libc::c_int as libc::c_uint &&
           (*psDroid).droidType as libc::c_uint !=
               DROID_CYBORG_REPAIR as libc::c_int as libc::c_uint &&
           (*psDroid).droidType as libc::c_uint !=
               DROID_TRANSPORTER as libc::c_int as libc::c_uint {
        numKills = 0 as libc::c_int as UDWORD;
        experienceLoc = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < 32 as libc::c_int {
            if aDroidExperience[player as usize][i as usize] as libc::c_uint >
                   numKills {
                numKills =
                    aDroidExperience[player as usize][i as usize] as UDWORD;
                experienceLoc = i
            }
            i += 1
        }
        aDroidExperience[player as usize][experienceLoc as usize] =
            0 as libc::c_int as UWORD;
        (*psDroid).numKills = numKills as UWORD
    } else { (*psDroid).numKills = 0 as libc::c_int as UWORD }
    //	//create the droids weapons	- done in droidSetBits()
    //create the droids programs
	/*psDroid->numProgs = pTemplate->numProgs;
	if (pTemplate->numProgs > 0)
	{
		for (inc=0; inc < pTemplate->numProgs; inc++)
		{
			psDroid->asProgs[inc].psStats = asProgramStats + pTemplate->asProgs[inc];
		}
		//set the first program to be the current one
		psDroid->activeProg = 0;
	}*/
    droidSetBits(pTemplate, psDroid);
    //calculate the droids total weight
    (*psDroid).weight = calcDroidWeight(pTemplate);
    // Initialise the movement stuff
    (*psDroid).baseSpeed =
        calcDroidBaseSpeed(pTemplate, (*psDroid).weight, player as UBYTE);
    initDroidMovement(psDroid);
    /*psDroid->sMove.fx = MAKEFRACT(psDroid->x);
	psDroid->sMove.fy = MAKEFRACT(psDroid->y);
//	psDroid->sMove.Speed = 0;
	psDroid->sMove.speed = MAKEFRACT(0);
	psDroid->sMove.dir = 0;
	psDroid->sMove.bumpDir = 0;
	psDroid->sMove.speed = MAKEFRACT(0);
	psDroid->sMove.Status = MOVEINACTIVE;
//	psDroid->sMove.speedChange = FALSE;
//	psDroid->sMove.lastTime=0;
	psDroid->sMove.Direction=0;
	psDroid->sMove.Direction3D=0;
	psDroid->sMove.psFormation = NULL;*/
    (*psDroid).direction = 0 as libc::c_int as UWORD;
    (*psDroid).pitch = 0 as libc::c_int as SWORD;
    (*psDroid).roll = 0 as libc::c_int as SWORD;
    //psDroid->turretRotRate = 360;
    (*psDroid).turretRotation = 0 as libc::c_int as UWORD;
    (*psDroid).turretPitch = 0 as libc::c_int as UWORD;
    (*psDroid).selected = 0 as libc::c_int as UBYTE;
    (*psDroid).lastEmission = 0 as libc::c_int as UDWORD;
    // ffs AM
    (*psDroid).bTargetted = 0 as libc::c_int; // no such weapon
    (*psDroid).timeLastHit = 0xffffffff as libc::c_uint;
    (*psDroid).lastHitWeapon = 0xffffffff as libc::c_uint;
    //allocate 'easy-access' data!
	//psDroid->sensorRange = (asSensorStats + pTemplate->asParts
	//	[COMP_SENSOR])->range;
	//psDroid->sensorPower = (asSensorStats + pTemplate->asParts
	//	[COMP_SENSOR])->power;
    (*psDroid).sensorRange =
        sensorRange(asSensorStats.offset((*pTemplate).asParts[COMP_SENSOR as
                                                                  libc::c_int
                                                                  as usize] as
                                             isize), player as UBYTE);
    (*psDroid).sensorPower =
        sensorPower(asSensorStats.offset((*pTemplate).asParts[COMP_SENSOR as
                                                                  libc::c_int
                                                                  as usize] as
                                             isize), player as UBYTE);
    /*psDroid->power = (asPowerStats + pTemplate->asParts
		[COMP_POWERPLANT])->output;
	psDroid->power = (asBodyStats + pTemplate->asParts[COMP_BODY])->
		powerOutput;*/
    //psDroid->ECMMod = (asECMStats + pTemplate->asParts[COMP_ECM])->power;
    (*psDroid).ECMMod =
        ecmPower(asECMStats.offset((*pTemplate).asParts[COMP_ECM as
                                                            libc::c_int as
                                                            usize] as isize),
                 player as UBYTE);
    //psDroid->body = (asBodyStats + pTemplate->asParts[COMP_BODY])->bodyPoints;
    (*psDroid).body = calcTemplateBody(pTemplate, player as UBYTE);
    (*psDroid).originalBody = (*psDroid).body;
    //if (psDroid->droidType == DROID_CYBORG)
    if cyborgDroid(psDroid) != 0 {
        inc = 0 as libc::c_int as UDWORD;
        while inc < NUM_WEAPON_CLASS as libc::c_int as libc::c_uint {
            //psDroid->armour[inc] = (asBodyStats + pTemplate->asParts[COMP_BODY])->armourValue[inc];
            (*psDroid).armour[inc as usize] =
                bodyArmour(asBodyStats.offset((*pTemplate).asParts[COMP_BODY
                                                                       as
                                                                       libc::c_int
                                                                       as
                                                                       usize]
                                                  as isize), player as UBYTE,
                           1 as libc::c_int as UBYTE, inc as WEAPON_CLASS);
            inc = inc.wrapping_add(1)
        }
    } else {
        inc = 0 as libc::c_int as UDWORD;
        while inc < NUM_WEAPON_CLASS as libc::c_int as libc::c_uint {
            (*psDroid).armour[inc as usize] =
                bodyArmour(asBodyStats.offset((*pTemplate).asParts[COMP_BODY
                                                                       as
                                                                       libc::c_int
                                                                       as
                                                                       usize]
                                                  as isize), player as UBYTE,
                           0 as libc::c_int as UBYTE, inc as WEAPON_CLASS);
            inc = inc.wrapping_add(1)
        }
    }
    /*(asArmourStats + pTemplate->asParts[COMP_ARMOUR])->
		strength;*/
    //allocate the power points
	//psDroid->power = pTemplate->powerPoints;
    //init the resistance to indicate no EW performed on this droid
    (*psDroid).resistance = 0 as libc::c_int as SWORD;
    memset((*psDroid).visible.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<[UBYTE; 8]>() as libc::c_ulong);
    (*psDroid).visible[(*psDroid).player as usize] =
        0xff as libc::c_int as UBYTE;
    //psDroid->damage = droidDamage;
    (*psDroid).died = 0 as libc::c_int as UDWORD;
    (*psDroid).inFire = 0 as libc::c_int;
    (*psDroid).burnStart = 0 as libc::c_int as UDWORD;
    (*psDroid).burnDamage = 0 as libc::c_int as UDWORD;
    //	psDroid->sAI.state = AI_PAUSE;
//	psDroid->sAI.psTarget = NULL;
//	psDroid->sAI.psSelectedWeapon = NULL;
//	psDroid->sAI.psStructToBuild = NULL;
    (*psDroid).sDisplay.screenX = 9999 as libc::c_int as UDWORD;
    (*psDroid).sDisplay.screenY = 9999 as libc::c_int as UDWORD;
    (*psDroid).sDisplay.screenR = 0 as libc::c_int as UDWORD;
    /* Set droid's initial illumination */
    (*psDroid).illumination = 0xff as libc::c_int as UBYTE;
    (*psDroid).sDisplay.imd =
        (*asBodyStats.offset((*psDroid).asBits[COMP_BODY as libc::c_int as
                                                   usize].nStat as
                                 isize)).pIMD;
    //don't worry if not on homebase cos not being drawn yet
    if onMission == 0 {
        /* People always stand upright */
        if (*psDroid).droidType as libc::c_uint !=
               DROID_PERSON as libc::c_int as libc::c_uint {
            updateDroidOrientation(psDroid);
        }
        visTilesUpdate(psDroid as *mut BASE_OBJECT, 0 as libc::c_int);
        gridAddObject(psDroid as *mut BASE_OBJECT);
        clustNewDroid(psDroid);
    }
    // create the command droid if necessary
    ((*pTemplate).asParts[COMP_BRAIN as libc::c_int as usize]) !=
        0 as libc::c_int;
    // ajl. droid will be created, so inform others
    if bMultiPlayer != 0 {
        if SendDroid(pTemplate, x, y, player as UBYTE, (*psDroid).id) ==
               0 as libc::c_int {
            return 0 as *mut DROID
        }
    }
    /* transporter-specific stuff */
    if (*psDroid).droidType as libc::c_uint ==
           DROID_TRANSPORTER as libc::c_int as libc::c_uint {
        //add transporter launch button if selected player and not a reinforcable situation
        if player == selectedPlayer && missionCanReEnforce() == 0 {
            intAddTransporterLaunch(psDroid);
        }
        //set droid height to be above the terrain
        (*psDroid).z =
            ((*psDroid).z as libc::c_int + 10 as libc::c_int) as UWORD;
        /* reset halt secondary order from guard to hold */
        secondarySetState(psDroid, DSO_HALTTYPE, DSS_HALT_HOLD);
    }
    if player == selectedPlayer { scoreUpdateVar(WD_UNITS_BUILT); }
    return psDroid;
}
//initialises the droid movement model
#[no_mangle]
pub unsafe extern "C" fn initDroidMovement(mut psDroid: *mut DROID) {
    memset(&mut (*psDroid).sMove as *mut MOVE_CONTROL as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<MOVE_CONTROL>() as libc::c_ulong);
    (*psDroid).sMove.fx = (*psDroid).x as FRACT;
    (*psDroid).sMove.fy = (*psDroid).y as FRACT;
    (*psDroid).sMove.fz = (*psDroid).z as FRACT;
}
// Set the asBits in a DROID structure given it's template.
//
#[no_mangle]
pub unsafe extern "C" fn droidSetBits(mut pTemplate: *mut DROID_TEMPLATE,
                                      mut psDroid: *mut DROID) {
    let mut inc: UDWORD = 0;
    (*psDroid).droidType = droidTemplateType(pTemplate);
    (*psDroid).direction = 0 as libc::c_int as UWORD;
    (*psDroid).pitch = 0 as libc::c_int as SWORD;
    (*psDroid).roll = 0 as libc::c_int as SWORD;
    //psDroid->turretRotRate = 360;
    (*psDroid).turretRotation = 0 as libc::c_int as UWORD;
    (*psDroid).turretPitch = 0 as libc::c_int as UWORD;
    //	psDroid->ECMMod = (asECMStats + pTemplate->asParts[COMP_ECM])->power;
    (*psDroid).body = calcTemplateBody(pTemplate, (*psDroid).player);
    (*psDroid).originalBody = (*psDroid).body;
    //create the droids weapons
	//psDroid->numWeaps = pTemplate->numWeaps;
    (*psDroid).asWeaps[0 as libc::c_int as usize].nStat =
        0 as libc::c_int as UDWORD;
    if (*pTemplate).numWeaps > 0 as libc::c_int as libc::c_uint {
        //for (inc=0; inc < pTemplate->numWeaps; inc++)
        //can only have one weapon now
        inc = 0 as libc::c_int as UDWORD;
        (*psDroid).asWeaps[inc as usize].lastFired =
            0 as libc::c_int as UDWORD;
        (*psDroid).asWeaps[inc as usize].nStat =
            (*pTemplate).asWeaps[inc as usize];
        (*psDroid).asWeaps[inc as usize].hitPoints =
            (*asWeaponStats.offset((*psDroid).asWeaps[inc as usize].nStat as
                                       isize)).hitPoints;
        (*psDroid).asWeaps[inc as usize].recoilValue =
            0 as libc::c_int as UDWORD;
        (*psDroid).asWeaps[inc as usize].ammo =
            (*asWeaponStats.offset((*psDroid).asWeaps[inc as usize].nStat as
                                       isize)).numRounds as UDWORD
        //set the first weapon to be the current one
//		psDroid->activeWeapon = 0;
    }
    //allocate the components hit points
    (*psDroid).asBits[COMP_BODY as libc::c_int as usize].nStat =
        (*pTemplate).asParts[COMP_BODY as libc::c_int as usize] as UBYTE;
    //psDroid->asBits[COMP_BODY].hitPoints =
	//	(asBodyStats + pTemplate->asParts[COMP_BODY])->hitPoints;
    // ajl - changed this to init brains for all droids (crashed)
    (*psDroid).asBits[COMP_BRAIN as libc::c_int as usize].nStat =
        0 as libc::c_int as UBYTE;
    // This is done by the Command droid stuff - John.
	// Not any more - John.
    (*psDroid).asBits[COMP_BRAIN as libc::c_int as usize].nStat =
        (*pTemplate).asParts[COMP_BRAIN as libc::c_int as usize] as UBYTE;
    //	psDroid->asBits[COMP_BRAIN].hitPoints =
//		(asBrainStats + pTemplate->asParts[COMP_BRAIN])->hitPoints;
    /*psDroid->asBits[COMP_POWERPLANT].hitPoints =
		(asPowerStats + pTemplate->asParts[COMP_POWERPLANT])->hitPoints;*/
    (*psDroid).asBits[COMP_PROPULSION as libc::c_int as usize].nStat =
        (*pTemplate).asParts[COMP_PROPULSION as libc::c_int as usize] as
            UBYTE;
    //psDroid->asBits[COMP_PROPULSION].hitPoints =
	//	(asPropulsionStats + pTemplate->asParts[COMP_PROPULSION])->hitPoints;
    (*psDroid).asBits[COMP_SENSOR as libc::c_int as usize].nStat =
        (*pTemplate).asParts[COMP_SENSOR as libc::c_int as usize] as UBYTE;
    //psDroid->asBits[COMP_SENSOR].hitPoints =
	//	(asSensorStats + pTemplate->asParts[COMP_SENSOR])->hitPoints;
    (*psDroid).asBits[COMP_ECM as libc::c_int as usize].nStat =
        (*pTemplate).asParts[COMP_ECM as libc::c_int as usize] as UBYTE;
    //psDroid->asBits[COMP_ECM].hitPoints =
	//	(asECMStats + pTemplate->asParts[COMP_ECM])->hitPoints;
    /*psDroid->asBits[COMP_ARMOUR].hitPoints =
		(asArmourStats + pTemplate->asParts[COMP_ARMOUR])->hitPoints; */
    (*psDroid).asBits[COMP_REPAIRUNIT as libc::c_int as usize].nStat =
        (*pTemplate).asParts[COMP_REPAIRUNIT as libc::c_int as usize] as
            UBYTE;
    //psDroid->asBits[COMP_REPAIRUNIT].hitPoints =
	//	(asRepairStats + pTemplate->asParts[COMP_REPAIRUNIT])->hitPoints;
    (*psDroid).asBits[COMP_CONSTRUCT as libc::c_int as usize].nStat =
        (*pTemplate).asParts[COMP_CONSTRUCT as libc::c_int as usize] as UBYTE;
    //psDroid->asBits[COMP_CONSTRUCT].hitPoints =
	//	(asConstructStats + pTemplate->asParts[COMP_CONSTRUCT])->hitPoints;
}
// Sets the parts array in a template given a droid.
#[no_mangle]
pub unsafe extern "C" fn templateSetParts(mut psDroid: *mut DROID,
                                          mut psTemplate:
                                              *mut DROID_TEMPLATE) {
    //	UDWORD inc;
    (*psTemplate).droidType = (*psDroid).droidType;
    //can only have one weapon now
    if (*psDroid).asWeaps[0 as libc::c_int as usize].nStat >
           0 as libc::c_int as libc::c_uint {
        (*psTemplate).numWeaps = 1 as libc::c_int as UDWORD;
        (*psTemplate).asWeaps[0 as libc::c_int as usize] =
            (*psDroid).asWeaps[0 as libc::c_int as usize].nStat
        // setup the weapon stat
    } else { (*psTemplate).numWeaps = 0 as libc::c_int as UDWORD }
    (*psTemplate).asParts[COMP_BODY as libc::c_int as usize] =
        (*psDroid).asBits[COMP_BODY as libc::c_int as usize].nStat as SDWORD;
    (*psTemplate).asParts[COMP_BRAIN as libc::c_int as usize] =
        (*psDroid).asBits[COMP_BRAIN as libc::c_int as usize].nStat as SDWORD;
    (*psTemplate).asParts[COMP_PROPULSION as libc::c_int as usize] =
        (*psDroid).asBits[COMP_PROPULSION as libc::c_int as usize].nStat as
            SDWORD;
    (*psTemplate).asParts[COMP_SENSOR as libc::c_int as usize] =
        (*psDroid).asBits[COMP_SENSOR as libc::c_int as usize].nStat as
            SDWORD;
    (*psTemplate).asParts[COMP_ECM as libc::c_int as usize] =
        (*psDroid).asBits[COMP_ECM as libc::c_int as usize].nStat as SDWORD;
    (*psTemplate).asParts[COMP_REPAIRUNIT as libc::c_int as usize] =
        (*psDroid).asBits[COMP_REPAIRUNIT as libc::c_int as usize].nStat as
            SDWORD;
    (*psTemplate).asParts[COMP_CONSTRUCT as libc::c_int as usize] =
        (*psDroid).asBits[COMP_CONSTRUCT as libc::c_int as usize].nStat as
            SDWORD;
}
/*
fills the list with Templates that can be manufactured
in the Factory - based on size. There is a limit on how many can be manufactured
at any one time. Pass back the number available.
*/
#[no_mangle]
pub unsafe extern "C" fn fillTemplateList(mut ppList:
                                              *mut *mut DROID_TEMPLATE,
                                          mut psFactory: *mut STRUCTURE,
                                          mut limit: UDWORD) -> UDWORD {
    let mut psCurr: *mut DROID_TEMPLATE = 0 as *mut DROID_TEMPLATE;
    let mut count: UDWORD = 0 as libc::c_int as UDWORD;
    let mut iCapacity: UDWORD =
        (*((*psFactory).pFunctionality as *mut FACTORY)).capacity as UDWORD;
    let mut current_block_5: u64;
    //BOOL			bCyborg = FALSE, bVtol = FALSE;
    /*if (psFactory->pStructureType->type == REF_CYBORG_FACTORY)
	{
		bCyborg = TRUE;
	}
	else if (psFactory->pStructureType->type == REF_VTOL_FACTORY)
	{
		bVtol = TRUE;
	}*/
    /* Add the templates to the list*/
    psCurr = apsDroidTemplates[(*psFactory).player as usize];
    while !psCurr.is_null() {
        //if on offworld mission, then can't build a Command Droid
		//if (mission.type == LDS_MKEEP OR mission.type == LDS_MCLEAR)
/*        if (missionIsOffworld())
		{
			if (psCurr->droidType == DROID_COMMAND)
			{
				continue;
			}
		}*/
        //must add Command Droid if currently in production
        if getProductionQuantity(psFactory, psCurr) == 0 {
            //can only have (MAX_CMDDROIDS) in the world at any one time
            if (*psCurr).droidType as libc::c_uint ==
                   DROID_COMMAND as libc::c_int as libc::c_uint {
                if checkProductionForCommand((*psFactory).player) as
                       libc::c_int +
                       checkCommandExist((*psFactory).player) as libc::c_int
                       >= 5 as libc::c_int {
                    current_block_5 = 14916268686031723178;
                } else { current_block_5 = 8515828400728868193; }
            } else { current_block_5 = 8515828400728868193; }
        } else { current_block_5 = 8515828400728868193; }
        match current_block_5 {
            8515828400728868193 => {
                if !(validTemplateForFactory(psCurr, psFactory) == 0) {
                    //check the factory can cope with this sized body
                    if !((*asBodyStats.offset((*psCurr).asParts[COMP_BODY as
                                                                    libc::c_int
                                                                    as usize]
                                                  as isize)).size as
                             libc::c_uint > iCapacity) {
                        //cyborg templates are available when the body has been research
            //-same for Transporter in multiPlayer
                        if (*psCurr).droidType as libc::c_uint ==
                               DROID_CYBORG as libc::c_int as libc::c_uint ||
                               (*psCurr).droidType as libc::c_uint ==
                                   DROID_CYBORG_SUPER as libc::c_int as
                                       libc::c_uint ||
                               (*psCurr).droidType as libc::c_uint ==
                                   DROID_CYBORG_CONSTRUCT as libc::c_int as
                                       libc::c_uint ||
                               (*psCurr).droidType as libc::c_uint ==
                                   DROID_CYBORG_REPAIR as libc::c_int as
                                       libc::c_uint ||
                               (*psCurr).droidType as libc::c_uint ==
                                   DROID_TRANSPORTER as libc::c_int as
                                       libc::c_uint {
                            if *apCompLists[(*psFactory).player as
                                                usize][COMP_BODY as
                                                           libc::c_int as
                                                           usize].offset((*psCurr).asParts[COMP_BODY
                                                                                               as
                                                                                               libc::c_int
                                                                                               as
                                                                                               usize]
                                                                             as
                                                                             isize)
                                   as libc::c_int != 0x1 as libc::c_int {
                                current_block_5 = 14916268686031723178;
                            } else { current_block_5 = 10048703153582371463; }
                        } else { current_block_5 = 10048703153582371463; }
                        match current_block_5 {
                            14916268686031723178 => { }
                            _ => {
                                let fresh1 = ppList;
                                ppList = ppList.offset(1);
                                *fresh1 = psCurr;
                                count = count.wrapping_add(1);
                                //once reached the limit, stop adding any more to the list
                                if count == limit { return count }
                            }
                        }
                    }
                }
            }
            _ => { }
        }
        //ignore if not research yet
        psCurr = (*psCurr).psNext
    }
    return count;
}
//determines the best IMD to draw for the droid - A TEMP MEASURE!
/*void setDroidIMD(DROID *psDroid)
{
	UDWORD	imdNum;

	//Big droid for end of Demo
	if (psDroid->player == 1)
	{
		imdNum = BARBARIANS + 3;
	}
	else
	{
		//Barbarian droids!!
		if (psDroid->player == DROIDPLAYER1 OR psDroid->player == DROIDPLAYER2)
		{
			imdNum = BARBARIANS;
			if (!strcmp(asBodyStats[psDroid->asBits[COMP_BODY].nStat].pName,
				"BaBa Body"))
			{
				imdNum = BARBARIANS + 1;
			}
			else if (!strcmp(asBodyStats[psDroid->asBits[COMP_BODY].nStat].pName,
				"Buggy Body"))
			{
				imdNum = BARBARIANS + 2;
			}
		}
		else
		{
			if(psDroid->numWeaps)
			{
				if(asWeaponStats[psDroid->asWeaps[0].nStat].direct)
				{
					//either a light or heavy cannon or a machine gun
					if (!strcmp(asWeaponStats[psDroid->asWeaps[0].nStat].pName,
						"Light Cannon"))
					{
						imdNum = CANNON_DROIDS;
					}
					else if (!strcmp(asWeaponStats[psDroid->asWeaps[0].nStat].pName,
						"Heavy Cannon"))
					{
						imdNum = CANNON_HEAVY_DROIDS;
					}
					else
					{
						imdNum = MACHGUN_DROIDS;
					}
				}
				else
				{
					//either  rocket1 or rocket2
					if (!strcmp(asWeaponStats[psDroid->asWeaps[0].nStat].pName,
						"Single Rocket"))
					{
						imdNum = ROCKET_DROIDS;
					}
					else
					{
						imdNum = ROCKET2_DROIDS;
					}
				}
			}
			else
			// No weapon droid
			{
				if(psDroid->asBits[COMP_CONSTRUCT].nStat)
				{
					// Constructor droid
					imdNum = CONST_DROIDS;
				}
				else
				{
					imdNum = RADAR_DROIDS;
				}
			}

			//sub group droids by body type and propulsion
			if (asBodyStats[psDroid->asBits[COMP_BODY].nStat].size == SIZE_MEDIUM)
			{
				imdNum += 3;
			}
			//propulsion types 0=wheeled, 1=tracked, 3=hover
			if (asPropulsionStats[psDroid->asBits[COMP_PROPULSION].nStat].
				propulsionType == 1)
			{
				imdNum += 1;
			}
			else if (asPropulsionStats[psDroid->asBits[COMP_PROPULSION].nStat].
				propulsionType == 3)
			{
				imdNum +=2;
			}
		}
	}
	//fail safe in case haven't managed to work out a valid number!
#ifndef PSX		// currently psx only has 2 droid imds .. so this check will not work
	if (imdNum > NUM_DROID_TYPES - 1)
	{
		imdNum = 0;
	}
#endif
	psDroid->sDisplay.imd = BODY_IMD(psDroid, psDroid->player);
	psDroid->imdNum = imdNum; // use the imdnum to define which body and which turret
}*/
// Determine the best IMD to draw for the droid - A TEMP MEASURE!
//
// Same as setDroidIMD() but uses a template structure instead
// of a droid structure.
//
/*UDWORD GetIMDFromTemplate(DROID_TEMPLATE *Template,UDWORD Player)
{
	UDWORD	imdNum;

	//Big droid for end of Demo
	if (Player == 1)
	{
		imdNum = BARBARIANS + 3;
	}
	else
	{
		//Barbarian droids!!
		if (Player == 6 OR Player == 7)
		{
			imdNum = BARBARIANS;
			if (!strcmp(asBodyStats[Template->asParts[COMP_BODY]].pName,
				"BaBa Body"))
			{
				imdNum = BARBARIANS + 1;
			}
			else if (!strcmp(asBodyStats[Template->asParts[COMP_BODY]].pName,
				"Buggy Body"))
			{
				imdNum = BARBARIANS + 2;
			}
		}
		else
		{
			if(Template->numWeaps)
			{
				if(asWeaponStats[Template->asWeaps[0]].direct)
				{
					//either a light or heavy cannon or a machine gun
					if (!strcmp(asWeaponStats[Template->asWeaps[0]].pName,
						"Light Cannon"))
					{
						imdNum = CANNON_DROIDS;
					}
					else if (!strcmp(asWeaponStats[Template->asWeaps[0]].pName,
						"Heavy Cannon"))
					{
						imdNum = CANNON_HEAVY_DROIDS;
					}
					else
					{
						imdNum = MACHGUN_DROIDS;
					}
				}
				else
				{
					//either  rocket1 or rocket2
					if (!strcmp(asWeaponStats[Template->asWeaps[0]].pName,
						"Single Rocket"))
					{
						imdNum = ROCKET_DROIDS;
					}
					else
					{
						imdNum = ROCKET2_DROIDS;
					}
				}
			}
			else
			// No weapon droid
			{
				if(Template->asParts[COMP_CONSTRUCT])
				{
					// Constructor droid
					imdNum = CONST_DROIDS;
				}
				else
				{
					imdNum = RADAR_DROIDS;
				}
			}

			//sub group droids by body type and propulsion
			if (asBodyStats[Template->asParts[COMP_BODY]].size == SIZE_MEDIUM)
			{
				imdNum += 3;
			}
			//propulsion types 0=wheeled, 1=tracked, 3=hover
			if (asPropulsionStats[Template->asParts[COMP_PROPULSION]].
				propulsionType == 1)
			{
				imdNum += 1;
			}
			else if (asPropulsionStats[Template->asParts[COMP_PROPULSION]].
				propulsionType == 3)
			{
				imdNum +=2;
			}
		}
	}
	//fail safe in case haven't managed to work out a valid number!
	if (imdNum > NUM_DROID_TYPES - 1)
	{
		imdNum = 0;
	}

	return imdNum;
}*/
// set the keyboard group for a droid
/* Make all the droids for a certain player a member of a specific group */
#[no_mangle]
pub unsafe extern "C" fn assignDroidsToGroup(mut playerNumber: UDWORD,
                                             mut groupNumber: UDWORD) {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut bAtLeastOne: BOOL = 0 as libc::c_int;
    let mut psFlagPos: *mut FLAG_POSITION = 0 as *mut FLAG_POSITION;
    if groupNumber < 0xff as libc::c_int as libc::c_uint {
        /* Run through all the droids */
        psDroid = apsDroidLists[playerNumber as usize];
        while !psDroid.is_null() {
            /* Clear out the old ones */
            if (*psDroid).group as libc::c_uint == groupNumber {
                (*psDroid).group = 0xff as libc::c_int as UBYTE
            }
            /* Only assign the currently selected ones */
            if (*psDroid).selected != 0 {
                /* Set them to the right group - they can only be a member of one group */
                (*psDroid).group = groupNumber as UBYTE;
                bAtLeastOne = 1 as libc::c_int
            }
            psDroid = (*psDroid).psNext
        }
    }
    if bAtLeastOne != 0 {
        setSelectedGroup(groupNumber);
        //clear the Deliv Point if one
        psFlagPos = apsFlagPosLists[selectedPlayer as usize];
        while !psFlagPos.is_null() {
            (*psFlagPos).selected = 0 as libc::c_int;
            psFlagPos = (*psFlagPos).psNext
        }
        groupConsoleInformOfCreation(groupNumber);
        secondarySetAverageGroupState(selectedPlayer, groupNumber);
    };
}
#[no_mangle]
pub unsafe extern "C" fn activateGroupAndMove(mut playerNumber: UDWORD,
                                              mut groupNumber: UDWORD)
 -> BOOL {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut psCentreDroid: *mut DROID = 0 as *mut DROID;
    let mut Selected: BOOL = 0 as libc::c_int;
    let mut psFlagPos: *mut FLAG_POSITION = 0 as *mut FLAG_POSITION;
    if groupNumber < 0xff as libc::c_int as libc::c_uint {
        psCentreDroid = 0 as *mut DROID;
        psDroid = apsDroidLists[playerNumber as usize];
        while !psDroid.is_null() {
            /* Wipe out the ones in the wrong group */
            if (*psDroid).selected as libc::c_int != 0 &&
                   (*psDroid).group as libc::c_uint != groupNumber {
                //				psDroid->selected = FALSE;
                DeSelectDroid(psDroid);
            }
            /* Get the right ones */
            if (*psDroid).group as libc::c_uint == groupNumber {
                //				psDroid->selected = TRUE;
                SelectDroid(psDroid);
                psCentreDroid = psDroid
            }
            psDroid = (*psDroid).psNext
        }
        /* There was at least one in the group */
        if !psCentreDroid.is_null() {
            //clear the Deliv Point if one
            psFlagPos =
                apsFlagPosLists[selectedPlayer as usize]; // messy - fix this
            while !psFlagPos.is_null() {
                (*psFlagPos).selected = 0 as libc::c_int;
                psFlagPos = (*psFlagPos).psNext
            }
            Selected = 1 as libc::c_int;
            if driveModeActive() == 0 {
                if getWarCamStatus() != 0 {
                    camToggleStatus();
                    // messy - FIXME
                    processWarCam();
                    camToggleStatus();
                } else if getWarCamStatus() == 0 {
                    //		setViewPos(psCentreDroid->x>>TILE_SHIFT,psCentreDroid->y>>TILE_SHIFT);
                    //odd, but necessary
                    //				camToggleStatus();
					/* Centre display on him if warcam isn't active */
//#ifndef PSX
                    setViewPos(((*psCentreDroid).x as libc::c_int >>
                                    7 as libc::c_int) as UDWORD,
                               ((*psCentreDroid).y as libc::c_int >>
                                    7 as libc::c_int) as UDWORD,
                               1 as libc::c_int);
                    //#else
//					camPanToLocation(psCentreDroid->x, psCentreDroid->y);
//#endif
                }
            }
        }
    }
    if Selected != 0 {
        setSelectedGroup(groupNumber);
        groupConsoleInformOfCentering(groupNumber);
    } else { setSelectedGroup(0xff as libc::c_int as UDWORD); }
    return Selected;
}
#[no_mangle]
pub unsafe extern "C" fn activateGroup(mut playerNumber: UDWORD,
                                       mut groupNumber: UDWORD) -> BOOL {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut Selected: BOOL = 0 as libc::c_int;
    let mut psFlagPos: *mut FLAG_POSITION = 0 as *mut FLAG_POSITION;
    if groupNumber < 0xff as libc::c_int as libc::c_uint {
        psDroid = apsDroidLists[playerNumber as usize];
        while !psDroid.is_null() {
            /* Wipe out the ones in the wrong group */
            if (*psDroid).selected as libc::c_int != 0 &&
                   (*psDroid).group as libc::c_uint != groupNumber {
                //				psDroid->selected = FALSE;
                DeSelectDroid(psDroid);
            }
            /* Get the right ones */
            if (*psDroid).group as libc::c_uint == groupNumber {
                //				psDroid->selected = TRUE;
                SelectDroid(psDroid);
                Selected = 1 as libc::c_int
            }
            psDroid = (*psDroid).psNext
        }
    }
    if Selected != 0 {
        //		if(getWarCamStatus())
//		{
//			camToggleStatus();
//		}
        setSelectedGroup(groupNumber);
        //clear the Deliv Point if one
        psFlagPos = apsFlagPosLists[selectedPlayer as usize];
        while !psFlagPos.is_null() {
            (*psFlagPos).selected = 0 as libc::c_int;
            psFlagPos = (*psFlagPos).psNext
        }
        groupConsoleInformOfSelection(groupNumber);
    } else { setSelectedGroup(0xff as libc::c_int as UDWORD); }
    return Selected;
}
//determines the best IMD to draw for the droid - A TEMP MEASURE!
//static void setDroidIMD(DROID *psDroid);
#[no_mangle]
pub unsafe extern "C" fn groupConsoleInformOfSelection(mut groupNumber:
                                                           UDWORD) {
    // ffs am
    let mut groupInfo: [libc::c_char; 255] = [0; 255];
    //	if(!getWarCamStatus())
//	{
    sprintf(groupInfo.as_mut_ptr(),
            strresGetString(psStringRes,
                            STR_GP_SELECTED as libc::c_int as UDWORD),
            groupNumber, selNumSelected(selectedPlayer));
    addConsoleMessage(groupInfo.as_mut_ptr(), RIGHT_JUSTIFY);
    //	}
}
#[no_mangle]
pub unsafe extern "C" fn groupConsoleInformOfCreation(mut groupNumber:
                                                          UDWORD) {
    let mut groupInfo: [libc::c_char; 255] = [0; 255];
    if getWarCamStatus() == 0 {
        sprintf(groupInfo.as_mut_ptr(),
                strresGetString(psStringRes,
                                STR_GP_ASSIGNED as libc::c_int as UDWORD),
                selNumSelected(selectedPlayer), groupNumber);
        addConsoleMessage(groupInfo.as_mut_ptr(), RIGHT_JUSTIFY);
    };
}
#[no_mangle]
pub unsafe extern "C" fn groupConsoleInformOfCentering(mut groupNumber:
                                                           UDWORD) {
    let mut groupInfo: [libc::c_char; 255] = [0; 255];
    if getWarCamStatus() == 0 {
        sprintf(groupInfo.as_mut_ptr(),
                strresGetString(psStringRes,
                                STR_GP_CENTERED as libc::c_int as UDWORD),
                groupNumber, selNumSelected(selectedPlayer));
    } else {
        sprintf(groupInfo.as_mut_ptr(),
                strresGetString(psStringRes,
                                STR_GP_ALLIGN as libc::c_int as UDWORD),
                groupNumber, selNumSelected(selectedPlayer));
    }
    addConsoleMessage(groupInfo.as_mut_ptr(), RIGHT_JUSTIFY);
}
#[no_mangle]
pub unsafe extern "C" fn getSelectedGroup() -> UDWORD {
    return selectedGroup;
}
#[no_mangle]
pub unsafe extern "C" fn setSelectedGroup(mut groupNumber: UDWORD) {
    selectedGroup = groupNumber;
    selectedCommander = 0xff as libc::c_int as UDWORD;
}
#[no_mangle]
pub unsafe extern "C" fn getSelectedCommander() -> UDWORD {
    return selectedCommander;
}
#[no_mangle]
pub unsafe extern "C" fn setSelectedCommander(mut commander: UDWORD) {
    selectedGroup = 0xff as libc::c_int as UDWORD;
    selectedCommander = commander;
}
/* calculate muzzle tip location in 3d world */
#[no_mangle]
pub unsafe extern "C" fn calcDroidMuzzleLocation(mut psDroid: *mut DROID,
                                                 mut muzzle: *mut iVector)
 -> BOOL {
    //	UDWORD turretType;
//	UDWORD bodyType;
    let mut barrel: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut psShape: *mut iIMDShape = 0 as *mut iIMDShape;
    let mut psWeapon: *mut iIMDShape = 0 as *mut iIMDShape;
    let mut psWeaponMount: *mut iIMDShape = 0 as *mut iIMDShape;
    psShape =
        (*asBodyStats.offset((*psDroid).asBits[COMP_BODY as libc::c_int as
                                                   usize].nStat as
                                 isize)).pIMD;
    psWeapon =
        (*asWeaponStats.offset((*psDroid).asWeaps[0 as libc::c_int as
                                                      usize].nStat as
                                   isize)).pIMD;
    psWeaponMount =
        (*asWeaponStats.offset((*psDroid).asWeaps[0 as libc::c_int as
                                                      usize].nStat as
                                   isize)).pMountGraphic;
    if !psShape.is_null() && (*psShape).nconnectors != 0 {
        // This code has not been translated to the PSX Yet !!!!                                     (sorry)
        pie_MatBegin();
        pie_TRANSLATE((*psDroid).x as libc::c_int, -((*psDroid).z as SDWORD),
                      (*psDroid).y as libc::c_int);
        //matrix = the center of droid
        pie_MatRotY(65536 as libc::c_int / 360 as libc::c_int *
                        (*psDroid).direction as SDWORD);
        pie_MatRotX(65536 as libc::c_int / 360 as libc::c_int *
                        (*psDroid).pitch as libc::c_int);
        pie_MatRotZ(65536 as libc::c_int / 360 as libc::c_int *
                        -((*psDroid).roll as SDWORD));
        //		pie_TRANSLATE(100,0,0);			//	(left,-height,forward)
        pie_TRANSLATE((*(*psShape).connectors).x, -(*(*psShape).connectors).z,
                      -(*(*psShape).connectors).y); //note y and z flipped
        //matrix = the gun and turret mount on the body
        pie_MatRotY(65536 as libc::c_int / 360 as libc::c_int *
                        (*psDroid).turretRotation as
                            SDWORD); //+ve anticlockwise
        pie_MatRotX(65536 as libc::c_int / 360 as libc::c_int *
                        (*psDroid).turretPitch as libc::c_int); //+ve up
        pie_MatRotZ(65536 as libc::c_int / 360 as libc::c_int *
                        0 as libc::c_int);
        //matrix = the muzzle mount on turret
        if !psWeapon.is_null() && (*psWeapon).nconnectors != 0 {
            barrel.x = (*(*psWeapon).connectors).x;
            barrel.y = -(*(*psWeapon).connectors).y;
            barrel.z = -(*(*psWeapon).connectors).z
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
        (*muzzle).x = (*psDroid).x as int32;
        (*muzzle).y = (*psDroid).y as int32;
        (*muzzle).z = (*psDroid).z as libc::c_int + 32 as libc::c_int
    }
    return 1 as libc::c_int;
}
/* IF YOU USE THIS FUNCTION - NOTE THAT selectedPlayer's TEMPLATES ARE NOT USED!!!!
   gets a template from its name - relies on the name being unique (or it will
   return the first one it finds!! */
#[no_mangle]
pub unsafe extern "C" fn getTemplateFromName(mut pName: *mut STRING)
 -> *mut DROID_TEMPLATE {
    let mut player: UDWORD = 0;
    let mut psCurr: *mut DROID_TEMPLATE = 0 as *mut DROID_TEMPLATE;
    /*all droid and template names are now stored as the translated
	name regardless of RESOURCE_NAMES and STORE_RESOURCE_ID! - AB 26/06/98*/
    getDroidResourceName(pName);
    player = 0 as libc::c_int as UDWORD;
    while player < 8 as libc::c_int as libc::c_uint {
        let mut current_block_3: u64;
        //OK so we want selectedPlayer's CYBORG templates since they cannot be edited
        //and we don't want to duplicate them for the sake of it! (ha!)
        //don't use selectedPlayer's templates if not multiplayer
        //this was written for use in the scripts and we don't want the scripts to use
        //selectedPlayer's templates because we cannot guarentee they will exist!
/*#ifndef PSX
        if (!bMultiPlayer)
#endif
        {
            if (player == selectedPlayer)
            {
                continue;
            }
        }
*/
        psCurr = apsDroidTemplates[player as usize];
        while !psCurr.is_null() {
            //if (!strcmp(psCurr->pName, pName))
            if strcmp((*psCurr).aName.as_mut_ptr(), pName) == 0 {
                //if template is selectedPlayers' it must be a CYBORG or we ignore it
                if bMultiPlayer == 0 {
                    //if (player == selectedPlayer AND psCurr->droidType != DROID_CYBORG)
                    if player == selectedPlayer &&
                           !((*psCurr).droidType as libc::c_uint ==
                                 DROID_CYBORG as libc::c_int as libc::c_uint
                                 ||
                                 (*psCurr).droidType as libc::c_uint ==
                                     DROID_CYBORG_SUPER as libc::c_int as
                                         libc::c_uint ||
                                 (*psCurr).droidType as libc::c_uint ==
                                     DROID_CYBORG_CONSTRUCT as libc::c_int as
                                         libc::c_uint ||
                                 (*psCurr).droidType as libc::c_uint ==
                                     DROID_CYBORG_REPAIR as libc::c_int as
                                         libc::c_uint) {
                        current_block_3 = 7502529970979898288;
                    } else { current_block_3 = 11812396948646013369; }
                } else { current_block_3 = 11812396948646013369; }
                match current_block_3 {
                    7502529970979898288 => { }
                    _ => { return psCurr }
                }
            }
            //ignore
            psCurr = (*psCurr).psNext
        }
        player = player.wrapping_add(1)
    }
    return 0 as *mut DROID_TEMPLATE;
}
/*getTemplatefFromSinglePlayerID gets template for unique ID  searching one players list */
#[no_mangle]
pub unsafe extern "C" fn getTemplateFromSinglePlayerID(mut multiPlayerID:
                                                           UDWORD,
                                                       mut player: UDWORD)
 -> *mut DROID_TEMPLATE {
    let mut pDroidDesign: *mut DROID_TEMPLATE = 0 as *mut DROID_TEMPLATE;
    pDroidDesign = apsDroidTemplates[player as usize];
    while !pDroidDesign.is_null() {
        if (*pDroidDesign).multiPlayerID == multiPlayerID {
            return pDroidDesign
        }
        pDroidDesign = (*pDroidDesign).psNext
    }
    return 0 as *mut DROID_TEMPLATE;
}
/*getTemplatefFromMultiPlayerID gets template for unique ID  searching all lists */
#[no_mangle]
pub unsafe extern "C" fn getTemplateFromMultiPlayerID(mut multiPlayerID:
                                                          UDWORD)
 -> *mut DROID_TEMPLATE {
    let mut player: UDWORD = 0;
    let mut pDroidDesign: *mut DROID_TEMPLATE = 0 as *mut DROID_TEMPLATE;
    player = 0 as libc::c_int as UDWORD;
    while player < 8 as libc::c_int as libc::c_uint {
        pDroidDesign = getTemplateFromSinglePlayerID(multiPlayerID, player);
        if !pDroidDesign.is_null() { return pDroidDesign }
        player = player.wrapping_add(1)
    }
    return 0 as *mut DROID_TEMPLATE;
}
// finds a droid for the player and sets it to be the current selected droid
#[no_mangle]
pub unsafe extern "C" fn selectDroidByID(mut id: UDWORD, mut player: UDWORD)
 -> BOOL {
    let mut psCurr: *mut DROID = 0 as *mut DROID;
    //look through the list of droids for the player and find the matching id
    psCurr = apsDroidLists[player as usize];
    while !psCurr.is_null() {
        if (*psCurr).id == id { break ; }
        psCurr = (*psCurr).psNext
    }
    if !psCurr.is_null() {
        clearSelection();
        //		psCurr->selected = TRUE;
        SelectDroid(psCurr);
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn getDroidLevel(mut psDroid: *mut DROID) -> UDWORD {
    if (*psDroid).droidType as libc::c_uint ==
           DROID_COMMAND as libc::c_int as libc::c_uint ||
           (*psDroid).droidType as libc::c_uint ==
               DROID_SENSOR as libc::c_int as libc::c_uint {
        return cmdDroidGetLevel(psDroid) as UDWORD
    } else if ((*psDroid).numKills as libc::c_int) < 4 as libc::c_int {
        return 0 as libc::c_int as UDWORD
    } else if ((*psDroid).numKills as libc::c_int) < 8 as libc::c_int {
        return 1 as libc::c_int as UDWORD
    } else if ((*psDroid).numKills as libc::c_int) < 16 as libc::c_int {
        return 2 as libc::c_int as UDWORD
    } else if ((*psDroid).numKills as libc::c_int) < 32 as libc::c_int {
        return 3 as libc::c_int as UDWORD
    } else if ((*psDroid).numKills as libc::c_int) < 64 as libc::c_int {
        return 4 as libc::c_int as UDWORD
    } else if ((*psDroid).numKills as libc::c_int) < 128 as libc::c_int {
        return 5 as libc::c_int as UDWORD
    } else if ((*psDroid).numKills as libc::c_int) < 256 as libc::c_int {
        return 6 as libc::c_int as UDWORD
    } else if ((*psDroid).numKills as libc::c_int) < 512 as libc::c_int {
        return 7 as libc::c_int as UDWORD
    } else { return 8 as libc::c_int as UDWORD };
}
#[no_mangle]
pub unsafe extern "C" fn getDroidNameForRank(mut rank: UDWORD)
 -> *mut STRING {
    match rank {
        0 => {
            return strresGetString(psStringRes,
                                   STR_DL_LEVEL_ROOKIE as libc::c_int as
                                       UDWORD)
        }
        1 => {
            return strresGetString(psStringRes,
                                   STR_DL_LEVEL_GREEN as libc::c_int as
                                       UDWORD)
        }
        2 => {
            return strresGetString(psStringRes,
                                   STR_DL_LEVEL_TRAINED as libc::c_int as
                                       UDWORD)
        }
        3 => {
            return strresGetString(psStringRes,
                                   STR_DL_LEVEL_REGULAR as libc::c_int as
                                       UDWORD)
        }
        4 => {
            return strresGetString(psStringRes,
                                   STR_DL_LEVEL_VETERAN as libc::c_int as
                                       UDWORD)
        }
        5 => {
            return strresGetString(psStringRes,
                                   STR_DL_LEVEL_CRACK as libc::c_int as
                                       UDWORD)
        }
        6 => {
            return strresGetString(psStringRes,
                                   STR_DL_LEVEL_ELITE as libc::c_int as
                                       UDWORD)
        }
        7 => {
            return strresGetString(psStringRes,
                                   STR_DL_LEVEL_SPECIAL as libc::c_int as
                                       UDWORD)
        }
        8 => {
            return strresGetString(psStringRes,
                                   STR_DL_LEVEL_ACE as libc::c_int as UDWORD)
        }
        _ => { }
    }
    return 0 as *mut STRING;
}
#[no_mangle]
pub unsafe extern "C" fn getDroidLevelName(mut psDroid: *mut DROID)
 -> *mut STRING {
    //#ifndef PSX
    return getDroidNameForRank(getDroidLevel(psDroid));
    /*
	switch (getDroidLevel(psDroid))
	{
	case 0:
		return strresGetString(psStringRes, STR_DL_LEVEL_ROOKIE);
	case 1:
		return strresGetString(psStringRes, STR_DL_LEVEL_GREEN);
	case 2:
		return strresGetString(psStringRes, STR_DL_LEVEL_TRAINED);
	case 3:
		return strresGetString(psStringRes, STR_DL_LEVEL_REGULAR);
	case 4:
		return strresGetString(psStringRes, STR_DL_LEVEL_VETERAN);
	case 5:
		return strresGetString(psStringRes, STR_DL_LEVEL_CRACK);
	case 6:
		return strresGetString(psStringRes, STR_DL_LEVEL_ELITE);
	case 7:
		return strresGetString(psStringRes, STR_DL_LEVEL_SPECIAL);
	case 8:
		return strresGetString(psStringRes, STR_DL_LEVEL_ACE);
	}
	*/
//#else
//		return "Rank";
//#endif
}
#[no_mangle]
pub unsafe extern "C" fn getNumDroidsForLevel(mut level: UDWORD) -> UDWORD {
    //UDWORD	lower,upper;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut count: UDWORD = 0;
    //UDWORD	numKills;
    /*	if(level)
	{
		lower = getBound(level-1);
		upper = getBound(level);
	}
	else
	{
		lower = upper = 0;
	}*/
    psDroid = apsDroidLists[selectedPlayer as usize];
    count = 0 as libc::c_int as UDWORD;
    while !psDroid.is_null() {
        /*		numKills = psDroid->numKills;
		if(level ? (numKills > lower AND numKills <=upper) : (numKills ==0) )*/
        if getDroidLevel(psDroid) == level { count = count.wrapping_add(1) }
        psDroid = (*psDroid).psNext
    }
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn getBound(mut level: UDWORD) -> UDWORD {
    if level == 0 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int as UDWORD
    } else if level < 8 as libc::c_int as libc::c_uint {
        return ((1 as libc::c_int) <<
                    level.wrapping_add(2 as libc::c_int as libc::c_uint)) as
                   UDWORD
    } else { return 0xffffffff as libc::c_uint };
}
/* Calculate the system points used by a template - NOT USED AT PRESENT*/
#[no_mangle]
pub unsafe extern "C" fn calcTemplateSystemPoints(mut psTemplate:
                                                      *mut DROID_TEMPLATE)
 -> UDWORD {
    let mut system: UDWORD = 0;
    let mut i: UDWORD = 0;
    //get the component system points
    system =
        (*asBodyStats.offset((*psTemplate).asParts[COMP_BODY as libc::c_int as
                                                       usize] as
                                 isize)).systemPoints.wrapping_add((*asBrainStats.offset((*psTemplate).asParts[COMP_BRAIN
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   usize]
                                                                                             as
                                                                                             isize)).systemPoints).wrapping_add((*asSensorStats.offset((*psTemplate).asParts[COMP_SENSOR
                                                                                                                                                                                 as
                                                                                                                                                                                 libc::c_int
                                                                                                                                                                                 as
                                                                                                                                                                                 usize]
                                                                                                                                                           as
                                                                                                                                                           isize)).systemPoints).wrapping_add((*asECMStats.offset((*psTemplate).asParts[COMP_ECM
                                                                                                                                                                                                                                            as
                                                                                                                                                                                                                                            libc::c_int
                                                                                                                                                                                                                                            as
                                                                                                                                                                                                                                            usize]
                                                                                                                                                                                                                      as
                                                                                                                                                                                                                      isize)).systemPoints).wrapping_add((*asRepairStats.offset((*psTemplate).asParts[COMP_REPAIRUNIT
                                                                                                                                                                                                                                                                                                          as
                                                                                                                                                                                                                                                                                                          libc::c_int
                                                                                                                                                                                                                                                                                                          as
                                                                                                                                                                                                                                                                                                          usize]
                                                                                                                                                                                                                                                                                    as
                                                                                                                                                                                                                                                                                    isize)).systemPoints).wrapping_add((*asConstructStats.offset((*psTemplate).asParts[COMP_CONSTRUCT
                                                                                                                                                                                                                                                                                                                                                                           as
                                                                                                                                                                                                                                                                                                                                                                           libc::c_int
                                                                                                                                                                                                                                                                                                                                                                           as
                                                                                                                                                                                                                                                                                                                                                                           usize]
                                                                                                                                                                                                                                                                                                                                                     as
                                                                                                                                                                                                                                                                                                                                                     isize)).systemPoints);
    /* propulsion system points are a percentage of the bodys' system points */
    system =
        (system as
             libc::c_uint).wrapping_add((*asPropulsionStats.offset((*psTemplate).asParts[COMP_PROPULSION
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             usize]
                                                                       as
                                                                       isize)).systemPoints.wrapping_mul((*asBodyStats.offset((*psTemplate).asParts[COMP_BODY
                                                                                                                                                        as
                                                                                                                                                        libc::c_int
                                                                                                                                                        as
                                                                                                                                                        usize]
                                                                                                                                  as
                                                                                                                                  isize)).systemPoints).wrapping_div(100
                                                                                                                                                                         as
                                                                                                                                                                         libc::c_int
                                                                                                                                                                         as
                                                                                                                                                                         libc::c_uint))
            as UDWORD as UDWORD;
    //add weapon system
    i = 0 as libc::c_int as UDWORD;
    while i < (*psTemplate).numWeaps {
        system =
            (system as
                 libc::c_uint).wrapping_add((*asWeaponStats.offset((*psTemplate).asWeaps[i
                                                                                             as
                                                                                             usize]
                                                                       as
                                                                       isize)).systemPoints)
                as UDWORD as UDWORD;
        i = i.wrapping_add(1)
    }
    //add program system
	/*for(i=0; i<psTemplate->numProgs; i++)
	{
		system += (asProgramStats + psTemplate->asProgs[i])->systemPoints;
	}*/
    return system;
}
/* Calculate the system points used by a droid - NOT USED AT PRESENT*/
#[no_mangle]
pub unsafe extern "C" fn calcDroidSystemPoints(mut psDroid: *mut DROID)
 -> UDWORD {
    let mut system: UDWORD = 0; //, i;
    //get the component system points
    system =
        (*asBodyStats.offset((*psDroid).asBits[COMP_BODY as libc::c_int as
                                                   usize].nStat as libc::c_int
                                 as
                                 isize)).systemPoints.wrapping_add((*asBrainStats.offset((*psDroid).asBits[COMP_BRAIN
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                               as
                                                                                                               usize].nStat
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             isize)).systemPoints).wrapping_add((*asSensorStats.offset((*psDroid).asBits[COMP_SENSOR
                                                                                                                                                                             as
                                                                                                                                                                             libc::c_int
                                                                                                                                                                             as
                                                                                                                                                                             usize].nStat
                                                                                                                                                           as
                                                                                                                                                           libc::c_int
                                                                                                                                                           as
                                                                                                                                                           isize)).systemPoints).wrapping_add((*asECMStats.offset((*psDroid).asBits[COMP_ECM
                                                                                                                                                                                                                                        as
                                                                                                                                                                                                                                        libc::c_int
                                                                                                                                                                                                                                        as
                                                                                                                                                                                                                                        usize].nStat
                                                                                                                                                                                                                      as
                                                                                                                                                                                                                      libc::c_int
                                                                                                                                                                                                                      as
                                                                                                                                                                                                                      isize)).systemPoints).wrapping_add((*asRepairStats.offset((*psDroid).asBits[COMP_REPAIRUNIT
                                                                                                                                                                                                                                                                                                      as
                                                                                                                                                                                                                                                                                                      libc::c_int
                                                                                                                                                                                                                                                                                                      as
                                                                                                                                                                                                                                                                                                      usize].nStat
                                                                                                                                                                                                                                                                                    as
                                                                                                                                                                                                                                                                                    libc::c_int
                                                                                                                                                                                                                                                                                    as
                                                                                                                                                                                                                                                                                    isize)).systemPoints).wrapping_add((*asConstructStats.offset((*psDroid).asBits[COMP_CONSTRUCT
                                                                                                                                                                                                                                                                                                                                                                       as
                                                                                                                                                                                                                                                                                                                                                                       libc::c_int
                                                                                                                                                                                                                                                                                                                                                                       as
                                                                                                                                                                                                                                                                                                                                                                       usize].nStat
                                                                                                                                                                                                                                                                                                                                                     as
                                                                                                                                                                                                                                                                                                                                                     libc::c_int
                                                                                                                                                                                                                                                                                                                                                     as
                                                                                                                                                                                                                                                                                                                                                     isize)).systemPoints);
    /* propulsion system points are a percentage of the bodys' system points */
    system =
        (system as
             libc::c_uint).wrapping_add((*asPropulsionStats.offset((*psDroid).asBits[COMP_PROPULSION
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         usize].nStat
                                                                       as
                                                                       libc::c_int
                                                                       as
                                                                       isize)).systemPoints.wrapping_mul((*asBodyStats.offset((*psDroid).asBits[COMP_BODY
                                                                                                                                                    as
                                                                                                                                                    libc::c_int
                                                                                                                                                    as
                                                                                                                                                    usize].nStat
                                                                                                                                  as
                                                                                                                                  libc::c_int
                                                                                                                                  as
                                                                                                                                  isize)).systemPoints).wrapping_div(100
                                                                                                                                                                         as
                                                                                                                                                                         libc::c_int
                                                                                                                                                                         as
                                                                                                                                                                         libc::c_uint))
            as UDWORD as UDWORD;
    //add weapon system
	//for(i=0; i<psDroid->numWeaps; i++)
    if (*psDroid).asWeaps[0 as libc::c_int as usize].nStat >
           0 as libc::c_int as libc::c_uint {
        //system += (asWeaponStats + psDroid->asWeaps[i].nStat)->systemPoints;
        system =
            (system as
                 libc::c_uint).wrapping_add((*asWeaponStats.offset((*psDroid).asWeaps[0
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          usize].nStat
                                                                       as
                                                                       isize)).systemPoints)
                as UDWORD as UDWORD
    }
    //add program system
	/*for(i=0; i<psDroid->numProgs; i++)
	{
		system += psDroid->asProgs[i].psStats->systemPoints;
	}*/
    return system;
}
// Get the name of a droid from it's DROID structure.
//
#[no_mangle]
pub unsafe extern "C" fn droidGetName(mut psDroid: *mut DROID)
 -> *mut STRING {
    return (*psDroid).aName.as_mut_ptr();
}
//
// Set the name of a droid in it's DROID structure.
//
// - only possible on the PC where you can adjust the names,
//
#[no_mangle]
pub unsafe extern "C" fn droidSetName(mut psDroid: *mut DROID,
                                      mut pName: *mut STRING) {
    strncpy((*psDroid).aName.as_mut_ptr(), pName,
            60 as libc::c_int as libc::c_uint);
    (*psDroid).aName[(60 as libc::c_int - 1 as libc::c_int) as usize] =
        0 as libc::c_int as STRING;
}
// ////////////////////////////////////////////////////////////////////////////
// returns true when no droid on x,y square.
#[no_mangle]
pub unsafe extern "C" fn noDroid(mut x: UDWORD, mut y: UDWORD) -> BOOL {
    let mut i: UDWORD = 0;
    let mut pD: *mut DROID = 0 as *mut DROID;
    // check each droid list
    i = 0 as libc::c_int as UDWORD;
    while i < 8 as libc::c_int as libc::c_uint {
        pD = apsDroidLists[i as usize];
        while !pD.is_null() {
            if ((*pD).x as libc::c_int >> 7 as libc::c_int) as UDWORD == x {
                if ((*pD).y as libc::c_int >> 7 as libc::c_int) as UDWORD == y
                   {
                    return 0 as libc::c_int
                }
            }
            pD = (*pD).psNext
        }
        i = i.wrapping_add(1)
    }
    return 1 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////////
// returns true when one droid on x,y square.
#[no_mangle]
pub unsafe extern "C" fn oneDroid(mut x: UDWORD, mut y: UDWORD) -> BOOL {
    let mut i: UDWORD = 0;
    let mut bFound: BOOL = 0 as libc::c_int;
    let mut pD: *mut DROID = 0 as *mut DROID;
    // check each droid list
    i = 0 as libc::c_int as UDWORD;
    while i < 8 as libc::c_int as libc::c_uint {
        pD = apsDroidLists[i as usize];
        while !pD.is_null() {
            if ((*pD).x as libc::c_int >> 7 as libc::c_int) as UDWORD == x {
                if ((*pD).y as libc::c_int >> 7 as libc::c_int) as UDWORD == y
                   {
                    if bFound != 0 {
                        return 0 as libc::c_int
                    } else {
                        bFound = 1 as libc::c_int
                        //first droid on this square so continue
                    }
                }
            }
            pD = (*pD).psNext
        }
        i = i.wrapping_add(1)
    }
    return 1 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////////
// returns true if it's a sensible place to put that droid.
#[no_mangle]
pub unsafe extern "C" fn sensiblePlace(mut x: SDWORD, mut y: SDWORD) -> BOOL {
    let mut count: UDWORD = 0 as libc::c_int as UDWORD;
    // not too near the edges.
    if x < 3 as libc::c_int ||
           x >
               mapWidth.wrapping_sub(3 as libc::c_int as libc::c_uint) as
                   SDWORD {
        return 0 as libc::c_int
    }
    if y < 3 as libc::c_int ||
           y >
               mapHeight.wrapping_sub(3 as libc::c_int as libc::c_uint) as
                   SDWORD {
        return 0 as libc::c_int
    }
    //check no features there
    if (*mapTile(x as UDWORD, y as UDWORD)).tileInfoBits as libc::c_int &
           0x2 as libc::c_int != 0 {
        return 0 as libc::c_int
    }
    // not on a blocking tile.
    if fpathBlockingTile.expect("non-null function pointer")(x, y) != 0 {
        return 0 as libc::c_int
    }
    // shouldn't next to more than one blocking tile, to avoid windy paths.
    if fpathBlockingTile.expect("non-null function pointer")(x -
                                                                 1 as
                                                                     libc::c_int,
                                                             y -
                                                                 1 as
                                                                     libc::c_int)
           != 0 {
        count = count.wrapping_add(1)
    }
    if fpathBlockingTile.expect("non-null function pointer")(x,
                                                             y -
                                                                 1 as
                                                                     libc::c_int)
           != 0 {
        count = count.wrapping_add(1)
    }
    if fpathBlockingTile.expect("non-null function pointer")(x +
                                                                 1 as
                                                                     libc::c_int,
                                                             y -
                                                                 1 as
                                                                     libc::c_int)
           != 0 {
        count = count.wrapping_add(1)
    }
    if fpathBlockingTile.expect("non-null function pointer")(x -
                                                                 1 as
                                                                     libc::c_int,
                                                             y) != 0 {
        count = count.wrapping_add(1)
    }
    if fpathBlockingTile.expect("non-null function pointer")(x +
                                                                 1 as
                                                                     libc::c_int,
                                                             y) != 0 {
        count = count.wrapping_add(1)
    }
    if fpathBlockingTile.expect("non-null function pointer")(x -
                                                                 1 as
                                                                     libc::c_int,
                                                             y +
                                                                 1 as
                                                                     libc::c_int)
           != 0 {
        count = count.wrapping_add(1)
    }
    if fpathBlockingTile.expect("non-null function pointer")(x,
                                                             y +
                                                                 1 as
                                                                     libc::c_int)
           != 0 {
        count = count.wrapping_add(1)
    }
    if fpathBlockingTile.expect("non-null function pointer")(x +
                                                                 1 as
                                                                     libc::c_int,
                                                             y +
                                                                 1 as
                                                                     libc::c_int)
           != 0 {
        count = count.wrapping_add(1)
    }
    if count > 1 as libc::c_int as libc::c_uint { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
// ------------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn normalPAT(mut x: UDWORD, mut y: UDWORD) -> BOOL {
    if sensiblePlace(x as SDWORD, y as SDWORD) != 0 && noDroid(x, y) != 0 {
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
}
// ------------------------------------------------------------------------------------
// Should stop things being placed in inaccessible areas?
#[no_mangle]
pub unsafe extern "C" fn zonedPAT(mut x: UDWORD, mut y: UDWORD) -> BOOL {
    if sensiblePlace(x as SDWORD, y as SDWORD) != 0 && noDroid(x, y) != 0 &&
           gwZoneReachable(gwGetZone(x as SDWORD, y as SDWORD)) != 0 {
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
}
// ------------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn halfPAT(mut x: UDWORD, mut y: UDWORD) -> BOOL {
    if sensiblePlace(x as SDWORD, y as SDWORD) != 0 && oneDroid(x, y) != 0 {
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
}
// ------------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn pickATileGen(mut x: *mut UDWORD, mut y: *mut UDWORD,
                                      mut numIterations: UBYTE,
                                      mut function:
                                          Option<unsafe extern "C" fn(_:
                                                                          UDWORD,
                                                                      _:
                                                                          UDWORD)
                                                     -> BOOL>) -> BOOL {
    let mut i: SDWORD = 0;
    let mut j: SDWORD = 0;
    let mut startX: SDWORD = 0;
    let mut endX: SDWORD = 0;
    let mut startY: SDWORD = 0;
    let mut endY: SDWORD = 0;
    let mut passes: UDWORD = 0;
    if *x >= 0 as libc::c_int as libc::c_uint && *x < mapWidth {
    } else {
        debug(LOG_ERROR,
              b"x coordinate is off-map for pickATileGen\x00" as *const u8 as
                  *const libc::c_char);
    };
    if *x >= 0 as libc::c_int as libc::c_uint && *x < mapWidth {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"droid.c\x00" as *const u8 as *const libc::c_char,
              5805 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 13],
                                        &[libc::c_char; 13]>(b"pickATileGen\x00")).as_ptr(),
              b"*x>=0 AND *x<mapWidth\x00" as *const u8 as
                  *const libc::c_char);
    };
    if *y >= 0 as libc::c_int as libc::c_uint && *y < mapHeight {
    } else {
        debug(LOG_ERROR,
              b"y coordinate is off-map for pickATileGen\x00" as *const u8 as
                  *const libc::c_char);
    };
    if *y >= 0 as libc::c_int as libc::c_uint && *y < mapHeight {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"droid.c\x00" as *const u8 as *const libc::c_char,
              5806 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 13],
                                        &[libc::c_char; 13]>(b"pickATileGen\x00")).as_ptr(),
              b"*y>=0 AND *y<mapHeight\x00" as *const u8 as
                  *const libc::c_char);
    };
    /* Exit if they're fine! */
    if sensiblePlace(*x as SDWORD, *y as SDWORD) != 0 && noDroid(*x, *y) != 0
       {
        return 1 as libc::c_int
    }
    /* Initial box dimensions and set iteration count to zero */
    endX = *x as SDWORD;
    startX = endX;
    endY = *y as SDWORD;
    startY = endY;
    passes = 0 as libc::c_int as UDWORD;
    /* Keep going until we get a tile or we exceed distance */
    while passes < numIterations as libc::c_uint {
        /* Process whole box */
        i = startX;
        while i <= endX {
            j = startY;
            while j <= endY {
                /* Test only perimeter as internal tested previous iteration */
                if i == startX || i == endX || j == startY || j == endY {
                    /* Good enough? */
                    if function.expect("non-null function pointer")(i as
                                                                        UDWORD,
                                                                    j as
                                                                        UDWORD)
                           != 0 {
                        /* Set exit conditions and get out NOW */
                        *x = i as UDWORD;
                        *y = j as UDWORD;
                        return 1 as libc::c_int
                    }
                }
                j += 1
            }
            i += 1
        }
        /* Expand the box out in all directions - off map handled by tileAcceptable */
        startX -= 1;
        startY -= 1;
        endX += 1;
        endY += 1;
        passes = passes.wrapping_add(1)
    }
    /* If we got this far, then we failed - passed in values will be unchanged */
    return 0 as libc::c_int;
}
//same as orig, but with threat check
#[no_mangle]
pub unsafe extern "C" fn pickATileGenThreat(mut x: *mut UDWORD,
                                            mut y: *mut UDWORD,
                                            mut numIterations: UBYTE,
                                            mut threatRange: SDWORD,
                                            mut player: SDWORD,
                                            mut function:
                                                Option<unsafe extern "C" fn(_:
                                                                                UDWORD,
                                                                            _:
                                                                                UDWORD)
                                                           -> BOOL>) -> BOOL {
    let mut i: SDWORD = 0;
    let mut j: SDWORD = 0;
    let mut startX: SDWORD = 0;
    let mut endX: SDWORD = 0;
    let mut startY: SDWORD = 0;
    let mut endY: SDWORD = 0;
    let mut passes: UDWORD = 0;
    if *x >= 0 as libc::c_int as libc::c_uint && *x < mapWidth {
    } else {
        debug(LOG_ERROR,
              b"x coordinate is off-map for pickATileGen\x00" as *const u8 as
                  *const libc::c_char);
    };
    if *x >= 0 as libc::c_int as libc::c_uint && *x < mapWidth {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"droid.c\x00" as *const u8 as *const libc::c_char,
              5857 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 19],
                                        &[libc::c_char; 19]>(b"pickATileGenThreat\x00")).as_ptr(),
              b"*x>=0 AND *x<mapWidth\x00" as *const u8 as
                  *const libc::c_char);
    };
    if *y >= 0 as libc::c_int as libc::c_uint && *y < mapHeight {
    } else {
        debug(LOG_ERROR,
              b"y coordinate is off-map for pickATileGen\x00" as *const u8 as
                  *const libc::c_char);
    };
    if *y >= 0 as libc::c_int as libc::c_uint && *y < mapHeight {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"droid.c\x00" as *const u8 as *const libc::c_char,
              5858 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 19],
                                        &[libc::c_char; 19]>(b"pickATileGenThreat\x00")).as_ptr(),
              b"*y>=0 AND *y<mapHeight\x00" as *const u8 as
                  *const libc::c_char);
    };
    if function.expect("non-null function pointer")(*x, *y) != 0 &&
           (threatRange <= 0 as libc::c_int ||
                ThreatInRange(player, threatRange, *x as SDWORD, *y as SDWORD,
                              0 as libc::c_int) == 0) {
        //TODO: vtol check really not needed?
        return 1 as libc::c_int
    }
    /* Initial box dimensions and set iteration count to zero */
    endX = *x as SDWORD;
    startX = endX;
    endY = *y as SDWORD;
    startY = endY;
    passes = 0 as libc::c_int as UDWORD;
    /* Keep going until we get a tile or we exceed distance */
    while passes < numIterations as libc::c_uint {
        /* Process whole box */
        i = startX;
        while i <= endX {
            j = startY;
            while j <= endY {
                /* Test only perimeter as internal tested previous iteration */
                if i == startX || i == endX || j == startY || j == endY {
                    /* Good enough? */
                    if function.expect("non-null function pointer")(i as
                                                                        UDWORD,
                                                                    j as
                                                                        UDWORD)
                           != 0 &&
                           (threatRange <= 0 as libc::c_int ||
                                ThreatInRange(player, threatRange,
                                              i << 7 as libc::c_int,
                                              j << 7 as libc::c_int,
                                              0 as libc::c_int) == 0) {
                        //TODO: vtols check really not needed?
                        /* Set exit conditions and get out NOW */
                        *x = i as UDWORD;
                        *y = j as UDWORD;
                        return 1 as libc::c_int
                    }
                }
                j += 1
            }
            i += 1
        }
        /* Expand the box out in all directions - off map handled by tileAcceptable */
        startX -= 1;
        startY -= 1;
        endX += 1;
        endY += 1;
        passes = passes.wrapping_add(1)
    }
    /* If we got this far, then we failed - passed in values will be unchanged */
    return 0 as libc::c_int;
}
// ------------------------------------------------------------------------------------
/* Improved pickATile - Replaces truly scary existing one. */
/* AM 22 - 10 - 98 */
#[no_mangle]
pub unsafe extern "C" fn pickATile(mut x: *mut UDWORD, mut y: *mut UDWORD,
                                   mut numIterations: UBYTE) -> BOOL {
    let mut i: SDWORD = 0;
    let mut j: SDWORD = 0;
    let mut startX: SDWORD = 0;
    let mut endX: SDWORD = 0;
    let mut startY: SDWORD = 0;
    let mut endY: SDWORD = 0;
    let mut passes: UDWORD = 0;
    if *x >= 0 as libc::c_int as libc::c_uint && *x < mapWidth {
    } else {
        debug(LOG_ERROR,
              b"x coordinate is off-map for pickATile\x00" as *const u8 as
                  *const libc::c_char);
    };
    if *x >= 0 as libc::c_int as libc::c_uint && *x < mapWidth {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"droid.c\x00" as *const u8 as *const libc::c_char,
              5907 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 10],
                                        &[libc::c_char; 10]>(b"pickATile\x00")).as_ptr(),
              b"*x>=0 AND *x<mapWidth\x00" as *const u8 as
                  *const libc::c_char);
    };
    if *y >= 0 as libc::c_int as libc::c_uint && *y < mapHeight {
    } else {
        debug(LOG_ERROR,
              b"y coordinate is off-map for pickATile\x00" as *const u8 as
                  *const libc::c_char);
    };
    if *y >= 0 as libc::c_int as libc::c_uint && *y < mapHeight {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"droid.c\x00" as *const u8 as *const libc::c_char,
              5908 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 10],
                                        &[libc::c_char; 10]>(b"pickATile\x00")).as_ptr(),
              b"*y>=0 AND *y<mapHeight\x00" as *const u8 as
                  *const libc::c_char);
    };
    /* Exit if they're fine! */
    if sensiblePlace(*x as SDWORD, *y as SDWORD) != 0 && noDroid(*x, *y) != 0
       {
        return 1 as libc::c_int
    }
    /* Initial box dimensions and set iteration count to zero */
    endX = *x as SDWORD;
    startX = endX;
    endY = *y as SDWORD;
    startY = endY;
    passes = 0 as libc::c_int as UDWORD;
    /* Keep going until we get a tile or we exceed distance */
    while passes < numIterations as libc::c_uint {
        /* Process whole box */
        i = startX;
        while i <= endX {
            j = startY;
            while j <= endY {
                /* Test only perimeter as internal tested previous iteration */
                if i == startX || i == endX || j == startY || j == endY {
                    /* Good enough? */
                    if sensiblePlace(i, j) != 0 &&
                           noDroid(i as UDWORD, j as UDWORD) != 0 {
                        /* Set exit conditions and get out NOW */
                        *x = i as UDWORD;
                        *y = j as UDWORD;
                        return 1 as libc::c_int
                    }
                }
                j += 1
            }
            i += 1
        }
        /* Expand the box out in all directions - off map handled by tileAcceptable */
        startX -= 1;
        startY -= 1;
        endX += 1;
        endY += 1;
        passes = passes.wrapping_add(1)
    }
    /* If we got this far, then we failed - passed in values will be unchanged */
    return 0 as libc::c_int;
}
// pickHalfATile just like improved pickATile but with Double Density Droid Placement
#[no_mangle]
pub unsafe extern "C" fn pickHalfATile(mut x: *mut UDWORD, mut y: *mut UDWORD,
                                       mut numIterations: UBYTE) -> PICKTILE {
    let mut i: SDWORD = 0;
    let mut j: SDWORD = 0;
    let mut startX: SDWORD = 0;
    let mut endX: SDWORD = 0;
    let mut startY: SDWORD = 0;
    let mut endY: SDWORD = 0;
    let mut passes: UDWORD = 0;
    /*
		Why was this written - I wrote pickATileGen to take a function
		pointer for what qualified as a valid tile - could use that.
		I'm not going to change it in case I'm missing the point */
    if pickATileGen(x, y, numIterations,
                    Some(zonedPAT as
                             unsafe extern "C" fn(_: UDWORD, _: UDWORD)
                                 -> BOOL)) != 0 {
        return FREE_TILE
    }
    /* Exit if they're fine! */
    if sensiblePlace(*x as SDWORD, *y as SDWORD) != 0 && oneDroid(*x, *y) != 0
       {
        return FREE_TILE
    }
    /* Initial box dimensions and set iteration count to zero */
    endX = *x as SDWORD;
    startX = endX;
    endY = *y as SDWORD;
    startY = endY;
    passes = 0 as libc::c_int as UDWORD;
    /* Keep going until we get a tile or we exceed distance */
    while passes < numIterations as libc::c_uint {
        /* Process whole box */
        i = startX;
        while i <= endX {
            j = startY;
            while j <= endY {
                /* Test only perimeter as internal tested previous iteration */
                if i == startX || i == endX || j == startY || j == endY {
                    /* Good enough? */
                    if sensiblePlace(i, j) != 0 &&
                           oneDroid(i as UDWORD, j as UDWORD) != 0 {
                        /* Set exit conditions and get out NOW */
                        *x = i as UDWORD;
                        *y = j as UDWORD;
                        return HALF_FREE_TILE
                    }
                }
                j += 1
            }
            i += 1
        }
        /* Expand the box out in all directions - off map handled by tileAcceptable */
        startX -= 1;
        startY -= 1;
        endX += 1;
        endY += 1;
        passes = passes.wrapping_add(1)
    }
    /* If we got this far, then we failed - passed in values will be unchanged */
    return NO_FREE_TILE;
}
// ////////////////////////////////////////////////////////////////////////////
// returns an x/y tile coord to place a droid
/* works round the location in a clockwise direction  Returns FALSE if a valid
location isn't found*/
/*BOOL pickATile(UDWORD *pX0,UDWORD *pY0, UBYTE numIterations)
{
	BOOL		found = FALSE;
	UDWORD		startX, startY, incX, incY;
	SDWORD		x=0, y=0;

	startX = *pX0;
	startY = *pY0;


	for (incX = 1, incY = 1; incX < numIterations; incX++, incY++)
	{
		if (!found)
		{
			y = startY - incY;
			for(x = startX - incX; x < (SDWORD)(startX + incX); x++)
			{
				if (sensiblePlace(x,y) AND noDroid(x,y) )
				{
					found = TRUE;
					break;
				}
			}
		}
		if (!found)
		{
			x = startX + incX;
			for(y = startY - incY; y < (SDWORD)(startY + incY); y++)
			{
				if (sensiblePlace(x,y) AND noDroid(x,y) )
				{
					found = TRUE;
					break;
				}
			}
		}
		if (!found)
		{
			y = startY + incY;
			for(x = startX + incX; x > (SDWORD)(startX - incX); x--)
			{
				if (sensiblePlace(x,y) AND noDroid(x,y) )
				{
					found = TRUE;
					break;
				}
			}
		}
		if (!found)
		{
			x = startX - incX;
			for(y = startY + incY; y > (SDWORD)(startY - incY); y--)
			{
				if (sensiblePlace(x,y) AND noDroid(x,y) )
				{
					found = TRUE;
					break;
				}
			}
		}
		if (found)
		{
			break;
		}
	}
	if (!found)
	{
		*pX0 = startX;
		*pY0 = startY;
		return FALSE;
	}
	else
	{
		*pX0 = x;
		*pY0 = y;
		return TRUE;
	}
}
*/
/* Looks through the players list of droids to see if any of them are
building the specified structure - returns TRUE if finds one*/
#[no_mangle]
pub unsafe extern "C" fn checkDroidsBuilding(mut psStructure: *mut STRUCTURE)
 -> BOOL {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    psDroid = apsDroidLists[(*psStructure).player as usize];
    while !psDroid.is_null() {
        //check DORDER_BUILD, HELP_BUILD is handled the same
        orderStateObj(psDroid, DORDER_BUILD,
                      &mut psStruct as *mut *mut STRUCTURE as
                          *mut *mut BASE_OBJECT);
        if psStruct == psStructure { return 1 as libc::c_int }
        psDroid = (*psDroid).psNext
    }
    return 0 as libc::c_int;
}
/* Looks through the players list of droids to see if any of them are
demolishing the specified structure - returns TRUE if finds one*/
#[no_mangle]
pub unsafe extern "C" fn checkDroidsDemolishing(mut psStructure:
                                                    *mut STRUCTURE) -> BOOL {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    psDroid = apsDroidLists[(*psStructure).player as usize];
    while !psDroid.is_null() {
        //check DORDER_DEMOLISH
        orderStateObj(psDroid, DORDER_DEMOLISH,
                      &mut psStruct as *mut *mut STRUCTURE as
                          *mut *mut BASE_OBJECT);
        if psStruct == psStructure { return 1 as libc::c_int }
        psDroid = (*psDroid).psNext
    }
    return 0 as libc::c_int;
}
/* checks the structure for type and capacity and **NOT orders the droid*** to build
a module if it can - returns TRUE if order is set */
#[no_mangle]
pub unsafe extern "C" fn buildModule(mut psDroid: *mut DROID,
                                     mut psStruct: *mut STRUCTURE,
                                     mut bCheckPower: BOOL) -> BOOL {
    let mut order: BOOL = 0;
    let mut i: UDWORD = 0 as libc::c_int as UDWORD;
    //	ASSERT( PTRVALID(psDroid, sizeof(DROID)),
//		"buildModule: Invalid droid pointer" );
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"buildModule: Invalid structure pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"droid.c\x00" as *const u8 as *const libc::c_char,
              6139 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 12],
                                        &[libc::c_char; 12]>(b"buildModule\x00")).as_ptr(),
              b"PTRVALID(psStruct, sizeof(STRUCTURE))\x00" as *const u8 as
                  *const libc::c_char);
    };
    order = 0 as libc::c_int;
    match (*(*psStruct).pStructureType).type_0 {
        3 => {
            //check room for one more!
            if (*((*psStruct).pFunctionality as *mut POWER_GEN)).capacity <
                   4 as libc::c_int as libc::c_uint {
                /*for (i = 0; (i < numStructureStats) && (asStructureStats[i].type !=
				REF_POWER_MODULE);i++)
			{
				//keep looking for the Power Module stat...
			}*/
                i = powerModuleStat;
                order = 1 as libc::c_int
            }
        }
        1 | 17 => {
            //check room for one more!
            if ((*((*psStruct).pFunctionality as *mut FACTORY)).capacity as
                    libc::c_int) < 2 as libc::c_int {
                /*for (i = 0; (i < numStructureStats) && (asStructureStats[i].type !=
				REF_FACTORY_MODULE);i++)
			{
				//keep looking for the Factory Module stat...
			}*/
                i = factoryModuleStat;
                order = 1 as libc::c_int
            }
        }
        10 => {
            //check room for one more!
            if (*((*psStruct).pFunctionality as
                      *mut RESEARCH_FACILITY)).capacity <
                   4 as libc::c_int as libc::c_uint {
                /*for (i = 0; (i < numStructureStats) && (asStructureStats[i].type !=
				REF_RESEARCH_MODULE);i++)
			{
				//keep looking for the Research Module stat...
			}*/
                i = researchModuleStat;
                order = 1 as libc::c_int
            }
        }
        _ => { }
    }
    if order != 0 {
        //check availability of Module
        if !(i < numStructureStats &&
                 *apStructTypeLists[(*psDroid).player as
                                        usize].offset(i as isize) as
                     libc::c_int == 0x1 as libc::c_int) {
            order = 0 as libc::c_int
        }
        //Power is obtained gradually now, so allow order
		/*if(bCheckPower)
		{
			// check enough power to build
			if (!checkPower(selectedPlayer, asStructureStats[i].powerToBuild, TRUE))
			{
				order = FALSE;
			}
		}*/
    }
    return order;
}
/*Deals with building a module - checking if any droid is currently doing this
 - if so, helping to build the current one*/
#[no_mangle]
pub unsafe extern "C" fn setUpBuildModule(mut psDroid: *mut DROID) {
    let mut tileX: UDWORD = 0;
    let mut tileY: UDWORD = 0;
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    tileX = ((*psDroid).orderX as libc::c_int >> 7 as libc::c_int) as UDWORD;
    tileY = ((*psDroid).orderY as libc::c_int >> 7 as libc::c_int) as UDWORD;
    //check not another Truck started
    psStruct = getTileStructure(tileX, tileY);
    if !psStruct.is_null() {
        if checkDroidsBuilding(psStruct) != 0 {
            //set up the help build scenario
            (*psDroid).order = DORDER_HELPBUILD as libc::c_int;
            (*psDroid).psTarget = psStruct as *mut BASE_OBJECT;
            if droidStartBuild(psDroid) != 0 {
                (*psDroid).action = DACTION_BUILD as libc::c_int;
                intBuildStarted(psDroid);
            } else { (*psDroid).action = DACTION_NONE as libc::c_int }
        } else if buildModule(psDroid, psStruct, 0 as libc::c_int) != 0 {
            //no other droids building so just start it off
            if droidStartBuild(psDroid) != 0 {
                (*psDroid).action = DACTION_BUILD as libc::c_int;
                intBuildStarted(psDroid);
            } else { (*psDroid).action = DACTION_NONE as libc::c_int }
        } else { (*psDroid).action = DACTION_NONE as libc::c_int }
    } else {
        //we've got a problem if it didn't find a structure
        (*psDroid).action = DACTION_NONE as libc::c_int
    };
}
// not written yet - needs merging with code in Dr Jones' Design.c
#[no_mangle]
pub unsafe extern "C" fn BuildNameFromDroid(mut psDroid: *mut DROID,
                                            mut ConstructedName:
                                                *mut STRING) {
}
// We just need 1 buffer for the current displayed droid (or template) name
//(32)
#[no_mangle]
pub unsafe extern "C" fn getDroidName(mut psDroid: *mut DROID)
 -> *mut STRING {
    let mut sTemplate: DROID_TEMPLATE =
        DROID_TEMPLATE{ref_0: 0,
                       pName: 0 as *const STRING as *mut STRING,
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
                       psNext:
                           0 as *const _droid_template as
                               *mut _droid_template,};
    templateSetParts(psDroid, &mut sTemplate);
    return getTemplateName(&mut sTemplate);
}
/*return the name to display for the interface - we don't know if this is
a string ID or something the user types in*/
#[no_mangle]
pub unsafe extern "C" fn getTemplateName(mut psTemplate: *mut DROID_TEMPLATE)
 -> *mut STRING {
    //STRING *pNameID=psTemplate->pName;
    let mut pNameID: *mut STRING = (*psTemplate).aName.as_mut_ptr();
    let mut id: UDWORD = 0;
    let mut pName: *mut STRING = 0 as *mut STRING;
    /*see if the name has a resource associated with it by trying to get
	the ID for the string*/
    if strresGetIDNum(psStringRes, pNameID, &mut id) != 0 {
        //get the string from the id
        pName = strresGetString(psStringRes, id);
        if !pName.is_null() { return pName }
    }
    //if haven't found a resource, return the name passed in
    if pName.is_null() { return pNameID }
    return 0 as *mut STRING;
}
/* Just returns true if the droid's present body points aren't as high as the original*/
#[no_mangle]
pub unsafe extern "C" fn droidIsDamaged(mut psDroid: *mut DROID) -> BOOL {
    if (*psDroid).body < (*psDroid).originalBody {
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn getDroidResourceName(mut pName: *mut STRING)
 -> BOOL {
    let mut id: UDWORD = 0;
    //see if the name has a resource associated with it by trying to get the ID for the string
    if strresGetIDNum(psStringRes, pName, &mut id) == 0 {
        debug(LOG_ERROR,
              b"Unable to find string resource for %s\x00" as *const u8 as
                  *const libc::c_char, pName);
        abort();
    }
    //get the string from the id
    strcpy(pName, strresGetString(psStringRes, id));
    return 1 as libc::c_int;
}
/*checks to see if an electronic warfare weapon is attached to the droid*/
#[no_mangle]
pub unsafe extern "C" fn electronicDroid(mut psDroid: *mut DROID) -> BOOL {
    let mut psCurr: *mut DROID = 0 as *mut DROID;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"electronicUnit: Invalid unit pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"droid.c\x00" as *const u8 as *const libc::c_char,
              6407 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 16],
                                        &[libc::c_char; 16]>(b"electronicDroid\x00")).as_ptr(),
              b"PTRVALID(psDroid, sizeof(DROID))\x00" as *const u8 as
                  *const libc::c_char);
    };
    //if (psDroid->numWeaps AND asWeaponStats[psDroid->asWeaps[0].nStat].
    if (*psDroid).asWeaps[0 as libc::c_int as usize].nStat >
           0 as libc::c_int as libc::c_uint &&
           (*asWeaponStats.offset((*psDroid).asWeaps[0 as libc::c_int as
                                                         usize].nStat as
                                      isize)).weaponSubClass as libc::c_uint
               == WSC_ELECTRONIC as libc::c_int as libc::c_uint {
        return 1 as libc::c_int
    }
    if (*psDroid).droidType as libc::c_uint ==
           DROID_COMMAND as libc::c_int as libc::c_uint &&
           !(*psDroid).psGroup.is_null() {
        // if a commander has EW units attached it is electronic
        psCurr = (*(*psDroid).psGroup).psList;
        while !psCurr.is_null() {
            if electronicDroid(psCurr) != 0 { return 1 as libc::c_int }
            psCurr = (*psCurr).psGrpNext
        }
    }
    return 0 as libc::c_int;
}
/*checks to see if the droid is currently being repaired by another*/
#[no_mangle]
pub unsafe extern "C" fn droidUnderRepair(mut psDroid: *mut DROID) -> BOOL {
    let mut psCurr: *mut DROID = 0 as *mut DROID;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"unitUnderRepair: Invalid unit pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"droid.c\x00" as *const u8 as *const libc::c_char,
              6437 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 17],
                                        &[libc::c_char; 17]>(b"droidUnderRepair\x00")).as_ptr(),
              b"PTRVALID(psDroid, sizeof(DROID))\x00" as *const u8 as
                  *const libc::c_char);
    };
    //droid must be damaged
    if droidIsDamaged(psDroid) != 0 {
        //look thru the list of players droids to see if any are repairing this droid
        psCurr = apsDroidLists[(*psDroid).player as usize];
        while !psCurr.is_null() {
            //if (psCurr->droidType == DROID_REPAIR AND psCurr->action ==
            if ((*psCurr).droidType as libc::c_uint ==
                    DROID_REPAIR as libc::c_int as libc::c_uint ||
                    (*psCurr).droidType as libc::c_uint ==
                        DROID_CYBORG_REPAIR as libc::c_int as libc::c_uint) &&
                   (*psCurr).action == DACTION_DROIDREPAIR as libc::c_int &&
                   (*psCurr).psTarget == psDroid as *mut BASE_OBJECT {
                return 1 as libc::c_int
            }
            psCurr = (*psCurr).psNext
        }
    }
    return 0 as libc::c_int;
}
//count how many Command Droids exist in the world at any one moment
#[no_mangle]
pub unsafe extern "C" fn checkCommandExist(mut player: UBYTE) -> UBYTE {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut quantity: UBYTE = 0 as libc::c_int as UBYTE;
    psDroid = apsDroidLists[player as usize];
    while !psDroid.is_null() {
        if (*psDroid).droidType as libc::c_uint ==
               DROID_COMMAND as libc::c_int as libc::c_uint {
            quantity = quantity.wrapping_add(1)
        }
        psDroid = (*psDroid).psNext
    }
    return quantity;
}
//access functions for vtols
#[no_mangle]
pub unsafe extern "C" fn vtolDroid(mut psDroid: *mut DROID) -> BOOL {
    return ((*asPropulsionStats.offset((*psDroid).asBits[COMP_PROPULSION as
                                                             libc::c_int as
                                                             usize].nStat as
                                           isize)).propulsionType as
                libc::c_int == LIFT as libc::c_int &&
                (*psDroid).droidType as libc::c_uint !=
                    DROID_TRANSPORTER as libc::c_int as libc::c_uint) as
               libc::c_int;
}
/*returns TRUE if a VTOL Weapon Droid which has completed all runs*/
#[no_mangle]
pub unsafe extern "C" fn vtolEmpty(mut psDroid: *mut DROID) -> BOOL {
    if vtolDroid(psDroid) == 0 { return 0 as libc::c_int }
    if (*psDroid).droidType as libc::c_uint !=
           DROID_WEAPON as libc::c_int as libc::c_uint {
        return 0 as libc::c_int
    }
    if (*psDroid).sMove.iAttackRuns as libc::c_int >=
           getNumAttackRuns(psDroid) as libc::c_int {
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
}
// true if a vtol is waiting to be rearmed by a particular rearm pad
#[no_mangle]
pub unsafe extern "C" fn vtolReadyToRearm(mut psDroid: *mut DROID,
                                          mut psStruct: *mut STRUCTURE)
 -> BOOL {
    let mut psRearmPad: *mut STRUCTURE = 0 as *mut STRUCTURE;
    if vtolDroid(psDroid) == 0 ||
           (*psDroid).action != DACTION_WAITFORREARM as libc::c_int {
        return 0 as libc::c_int
    }
    // If a unit has been ordered to rearm make sure it goes to the correct base
    if orderStateObj(psDroid, DORDER_REARM,
                     &mut psRearmPad as *mut *mut STRUCTURE as
                         *mut *mut BASE_OBJECT) != 0 {
        if psRearmPad != psStruct && vtolOnRearmPad(psRearmPad, psDroid) == 0
           {
            // target rearm pad is clear - let it go there
            return 0 as libc::c_int
        }
    }
    if vtolHappy(psDroid) != 0 && vtolOnRearmPad(psStruct, psDroid) != 0 {
        // there is a vtol on the pad and this vtol is already rearmed
		// don't bother shifting the other vtol off
        return 0 as libc::c_int
    }
    if !(*psDroid).psActionTarget.is_null() &&
           (*(*psDroid).psActionTarget).cluster as libc::c_int !=
               (*psStruct).cluster as libc::c_int {
        // vtol is rearming at a different base
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
// true if a vtol droid currently returning to be rearmed
#[no_mangle]
pub unsafe extern "C" fn vtolRearming(mut psDroid: *mut DROID) -> BOOL {
    if vtolDroid(psDroid) == 0 { return 0 as libc::c_int }
    if (*psDroid).droidType as libc::c_uint !=
           DROID_WEAPON as libc::c_int as libc::c_uint {
        return 0 as libc::c_int
    }
    if (*psDroid).action == DACTION_MOVETOREARM as libc::c_int ||
           (*psDroid).action == DACTION_WAITFORREARM as libc::c_int ||
           (*psDroid).action == DACTION_MOVETOREARMPOINT as libc::c_int ||
           (*psDroid).action == DACTION_WAITDURINGREARM as libc::c_int {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
// true if a droid is currently attacking
#[no_mangle]
pub unsafe extern "C" fn droidAttacking(mut psDroid: *mut DROID) -> BOOL {
    //what about cyborgs?
	//if (psDroid->droidType != DROID_WEAPON)
    if !((*psDroid).droidType as libc::c_uint ==
             DROID_WEAPON as libc::c_int as libc::c_uint ||
             (*psDroid).droidType as libc::c_uint ==
                 DROID_CYBORG as libc::c_int as libc::c_uint ||
             (*psDroid).droidType as libc::c_uint ==
                 DROID_CYBORG_SUPER as libc::c_int as libc::c_uint) {
        return 0 as libc::c_int
    }
    if (*psDroid).action == DACTION_ATTACK as libc::c_int ||
           (*psDroid).action == DACTION_MOVETOATTACK as libc::c_int ||
           (*psDroid).action == DACTION_ROTATETOATTACK as libc::c_int ||
           (*psDroid).action == DACTION_VTOLATTACK as libc::c_int ||
           (*psDroid).action == DACTION_MOVEFIRE as libc::c_int {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
// see if there are any other vtols attacking the same target
// but still rearming
#[no_mangle]
pub unsafe extern "C" fn allVtolsRearmed(mut psDroid: *mut DROID) -> BOOL {
    let mut psCurr: *mut DROID = 0 as *mut DROID;
    let mut stillRearming: BOOL = 0;
    // ignore all non vtols
    if vtolDroid(psDroid) == 0 { return 1 as libc::c_int }
    stillRearming = 0 as libc::c_int;
    psCurr = apsDroidLists[(*psDroid).player as usize];
    while !psCurr.is_null() {
        if vtolRearming(psCurr) != 0 && (*psCurr).order == (*psDroid).order &&
               (*psCurr).psTarget == (*psDroid).psTarget {
            stillRearming = 1 as libc::c_int;
            break ;
        } else { psCurr = (*psCurr).psNext }
    }
    return (stillRearming == 0) as libc::c_int;
}
/*returns a count of the base number of attack runs for the weapon attached to the droid*/
#[no_mangle]
pub unsafe extern "C" fn getNumAttackRuns(mut psDroid: *mut DROID) -> UWORD {
    let mut numAttackRuns: UWORD = 0;
    if vtolDroid(psDroid) != 0 {
    } else {
        debug(LOG_ERROR,
              b"numAttackRuns:not a VTOL Droid\x00" as *const u8 as
                  *const libc::c_char);
    };
    if vtolDroid(psDroid) != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"droid.c\x00" as *const u8 as *const libc::c_char,
              6622 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 17],
                                        &[libc::c_char; 17]>(b"getNumAttackRuns\x00")).as_ptr(),
              b"vtolDroid(psDroid)\x00" as *const u8 as *const libc::c_char);
    };
    /*if weapon attached to the droid is a salvo weapon, then number of shots that
    can be fired = vtolAttackRuns*numRounds */
    if (*asWeaponStats.offset((*psDroid).asWeaps[0 as libc::c_int as
                                                     usize].nStat as
                                  isize)).reloadTime != 0 {
        numAttackRuns =
            ((*asWeaponStats.offset((*psDroid).asWeaps[0 as libc::c_int as
                                                           usize].nStat as
                                        isize)).numRounds as libc::c_int *
                 (*asWeaponStats.offset((*psDroid).asWeaps[0 as libc::c_int as
                                                               usize].nStat as
                                            isize)).vtolAttackRuns as
                     libc::c_int) as UWORD
    } else {
        numAttackRuns =
            (*asWeaponStats.offset((*psDroid).asWeaps[0 as libc::c_int as
                                                          usize].nStat as
                                       isize)).vtolAttackRuns as UWORD
    }
    return numAttackRuns;
}
/*Checks a vtol for being fully armed and fully repaired to see if ready to
leave reArm pad */
#[no_mangle]
pub unsafe extern "C" fn vtolHappy(mut psDroid: *mut DROID) -> BOOL {
    if vtolDroid(psDroid) != 0 {
    } else {
        debug(LOG_ERROR,
              b"vtolHappy: not a VTOL droid\x00" as *const u8 as
                  *const libc::c_char);
    };
    if vtolDroid(psDroid) != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"droid.c\x00" as *const u8 as *const libc::c_char,
              6643 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 10],
                                        &[libc::c_char; 10]>(b"vtolHappy\x00")).as_ptr(),
              b"vtolDroid(psDroid)\x00" as *const u8 as *const libc::c_char);
    };
    if (*psDroid).droidType as libc::c_uint ==
           DROID_WEAPON as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"vtolHappy: not a weapon droid\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (*psDroid).droidType as libc::c_uint ==
           DROID_WEAPON as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"droid.c\x00" as *const u8 as *const libc::c_char,
              6644 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 10],
                                        &[libc::c_char; 10]>(b"vtolHappy\x00")).as_ptr(),
              b"psDroid->droidType == DROID_WEAPON\x00" as *const u8 as
                  *const libc::c_char);
    };
    //check full complement of ammo
    if (*psDroid).sMove.iAttackRuns as libc::c_int == 0 as libc::c_int {
        //check fully repaired
        if (*psDroid).body == (*psDroid).originalBody {
            return 1 as libc::c_int
        }
    }
    return 0 as libc::c_int;
}
/*checks if the droid is a VTOL droid and updates the attack runs as required*/
#[no_mangle]
pub unsafe extern "C" fn updateVtolAttackRun(mut psDroid: *mut DROID) {
    if vtolDroid(psDroid) != 0 {
        (*psDroid).sMove.iAttackRuns =
            (*psDroid).sMove.iAttackRuns.wrapping_add(1);
        //quick check doesn't go over limit
        if ((*psDroid).sMove.iAttackRuns as libc::c_int) <
               0xffff as libc::c_int {
        } else {
            debug(LOG_ERROR,
                  b"updateVtolAttackRun: too many attack runs\x00" as
                      *const u8 as *const libc::c_char);
        };
        if ((*psDroid).sMove.iAttackRuns as libc::c_int) <
               0xffff as libc::c_int {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"droid.c\x00" as *const u8 as *const libc::c_char,
                  6665 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 20],
                                            &[libc::c_char; 20]>(b"updateVtolAttackRun\x00")).as_ptr(),
                  b"psDroid->sMove.iAttackRuns < UWORD_MAX\x00" as *const u8
                      as *const libc::c_char);
        };
    };
}
/*this mends the VTOL when it has been returned to home base whilst on an
offworld mission*/
#[no_mangle]
pub unsafe extern "C" fn mendVtol(mut psDroid: *mut DROID) {
    if vtolEmpty(psDroid) != 0 {
    } else {
        debug(LOG_ERROR,
              b"mendVtol: droid is not an empty weapon VTOL!\x00" as *const u8
                  as *const libc::c_char);
    };
    if vtolEmpty(psDroid) != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"droid.c\x00" as *const u8 as *const libc::c_char,
              6673 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 9],
                                        &[libc::c_char; 9]>(b"mendVtol\x00")).as_ptr(),
              b"vtolEmpty(psDroid)\x00" as *const u8 as *const libc::c_char);
    };
    /* set rearm value to no runs made */
    (*psDroid).sMove.iAttackRuns = 0 as libc::c_int as UWORD;
    //reset ammo and lastTimeFired
    (*psDroid).asWeaps[0 as libc::c_int as usize].ammo =
        (*asWeaponStats.offset((*psDroid).asWeaps[0 as libc::c_int as
                                                      usize].nStat as
                                   isize)).numRounds as UDWORD;
    (*psDroid).asWeaps[0 as libc::c_int as usize].lastFired =
        0 as libc::c_int as UDWORD;
    /* set droid points to max */
    (*psDroid).body = (*psDroid).originalBody;
}
//assign rearmPad to the VTOL
#[no_mangle]
pub unsafe extern "C" fn assignVTOLPad(mut psNewDroid: *mut DROID,
                                       mut psReArmPad: *mut STRUCTURE) {
    if vtolDroid(psNewDroid) != 0 {
    } else {
        debug(LOG_ERROR,
              b"assignVTOLPad: not a vtol droid\x00" as *const u8 as
                  *const libc::c_char);
    };
    if vtolDroid(psNewDroid) != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"droid.c\x00" as *const u8 as *const libc::c_char,
              6688 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"assignVTOLPad\x00")).as_ptr(),
              b"vtolDroid(psNewDroid)\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (*psReArmPad).type_0 as libc::c_uint ==
           OBJ_STRUCTURE as libc::c_int as libc::c_uint &&
           (*(*psReArmPad).pStructureType).type_0 ==
               REF_REARM_PAD as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"assignVTOLPad: not a ReArm Pad\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (*psReArmPad).type_0 as libc::c_uint ==
           OBJ_STRUCTURE as libc::c_int as libc::c_uint &&
           (*(*psReArmPad).pStructureType).type_0 ==
               REF_REARM_PAD as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"droid.c\x00" as *const u8 as *const libc::c_char,
              6691 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"assignVTOLPad\x00")).as_ptr(),
              b"psReArmPad->type == OBJ_STRUCTURE && psReArmPad->pStructureType->type == REF_REARM_PAD\x00"
                  as *const u8 as *const libc::c_char);
    };
    (*psNewDroid).psBaseStruct = psReArmPad;
}
//don't use this function any more - the droid checks each frame for this to have died
//look through all droids to see if any are associated with the ReArming Pad
/*void releaseVTOLPad(STRUCTURE *psReArmPad)
{
    DROID       *psDroid;

    ASSERT( psReArmPad->pStructureType->type == REF_REARM_PAD,
        "releaseVTOLPad: not a ReArm Pad" );

    //go thru the structure's player's list of droids looking for VTOLs
    for (psDroid = apsDroidLists[psReArmPad->player]; psDroid != NULL; psDroid = psDroid->psNext)
    {
        if (vtolDroid(psDroid))
        {
            //check this droid was assigned to the rearming pad
            if (psDroid->psBaseStruct == psReArmPad)
            {
                //need to find a new rearming pad - get the nearest to this one
                psDroid->psBaseStruct = NULL;
                psDroid->psBaseStruct = findNearestReArmPad(psDroid, psReArmPad, FALSE);
            }
        }
    }
}*/
/*compares the droid sensor type with the droid weapon type to see if the
FIRE_SUPPORT order can be assigned*/
#[no_mangle]
pub unsafe extern "C" fn droidSensorDroidWeapon(mut psObj: *mut BASE_OBJECT,
                                                mut psDroid: *mut DROID)
 -> BOOL {
    let mut psStats: *mut SENSOR_STATS = 0 as *mut SENSOR_STATS;
    //first check if the object is a droid or a structure
    if (*psObj).type_0 as libc::c_uint !=
           OBJ_DROID as libc::c_int as libc::c_uint &&
           (*psObj).type_0 as libc::c_uint !=
               OBJ_STRUCTURE as libc::c_int as libc::c_uint {
        return 0 as libc::c_int
    }
    //check same player
    if (*psObj).player as libc::c_int != (*psDroid).player as libc::c_int {
        return 0 as libc::c_int
    }
    //check obj is a sensor droid/structure
    match (*psObj).type_0 as libc::c_uint {
        0 => {
            if (*(psObj as *mut DROID)).droidType as libc::c_uint !=
                   DROID_SENSOR as libc::c_int as libc::c_uint &&
                   (*(psObj as *mut DROID)).droidType as libc::c_uint !=
                       DROID_COMMAND as libc::c_int as libc::c_uint {
                return 0 as libc::c_int
            }
            psStats =
                asSensorStats.offset((*(psObj as
                                            *mut DROID)).asBits[COMP_SENSOR as
                                                                    libc::c_int
                                                                    as
                                                                    usize].nStat
                                         as libc::c_int as isize)
        }
        1 => {
            psStats = (*(*(psObj as *mut STRUCTURE)).pStructureType).pSensor;
            if psStats.is_null() ||
                   (*psStats).location !=
                       LOC_TURRET as libc::c_int as libc::c_uint {
                return 0 as libc::c_int
            }
        }
        _ => { }
    }
    //check droid is a weapon droid - or Cyborg!!
	//if (psDroid->droidType != DROID_WEAPON)
    if !((*psDroid).droidType as libc::c_uint ==
             DROID_WEAPON as libc::c_int as libc::c_uint ||
             (*psDroid).droidType as libc::c_uint ==
                 DROID_CYBORG as libc::c_int as libc::c_uint ||
             (*psDroid).droidType as libc::c_uint ==
                 DROID_CYBORG_SUPER as libc::c_int as libc::c_uint) {
        return 0 as libc::c_int
    }
    //finally check the right droid/sensor combination
	// check vtol droid with commander
    if (vtolDroid(psDroid) != 0 ||
            proj_Direct(asWeaponStats.offset((*psDroid).asWeaps[0 as
                                                                    libc::c_int
                                                                    as
                                                                    usize].nStat
                                                 as isize)) == 0) &&
           (*(psObj as *mut DROID)).droidType as libc::c_uint ==
               DROID_COMMAND as libc::c_int as libc::c_uint {
        return 1 as libc::c_int
    }
    //check vtol droid with vtol sensor
	//if (vtolDroid(psDroid) AND psDroid->numWeaps > 0)
    if vtolDroid(psDroid) != 0 &&
           (*psDroid).asWeaps[0 as libc::c_int as usize].nStat >
               0 as libc::c_int as libc::c_uint {
        if (*psStats).type_0 as libc::c_uint ==
               VTOL_INTERCEPT_SENSOR as libc::c_int as libc::c_uint ||
               (*psStats).type_0 as libc::c_uint ==
                   VTOL_CB_SENSOR as libc::c_int as libc::c_uint ||
               (*psStats).type_0 as libc::c_uint ==
                   SUPER_SENSOR as libc::c_int as libc::c_uint {
            return 1 as libc::c_int
        }
        return 0 as libc::c_int
    }
    //check indirect weapon droid with standard/cb sensor
    /*Super Sensor works as any type*/
    if proj_Direct(asWeaponStats.offset((*psDroid).asWeaps[0 as libc::c_int as
                                                               usize].nStat as
                                            isize)) == 0 {
        if (*psStats).type_0 as libc::c_uint ==
               STANDARD_SENSOR as libc::c_int as libc::c_uint ||
               (*psStats).type_0 as libc::c_uint ==
                   INDIRECT_CB_SENSOR as libc::c_int as libc::c_uint ||
               (*psStats).type_0 as libc::c_uint ==
                   SUPER_SENSOR as libc::c_int as libc::c_uint {
            return 1 as libc::c_int
        }
        return 0 as libc::c_int
    }
    return 0 as libc::c_int;
}
// return whether a droid has a CB sensor on it
#[no_mangle]
pub unsafe extern "C" fn cbSensorDroid(mut psDroid: *mut DROID) -> BOOL {
    if (*psDroid).droidType as libc::c_uint !=
           DROID_SENSOR as libc::c_int as libc::c_uint {
        return 0 as libc::c_int
    }
    /*Super Sensor works as any type*/
    if (*asSensorStats.offset((*psDroid).asBits[COMP_SENSOR as libc::c_int as
                                                    usize].nStat as
                                  isize)).type_0 as libc::c_uint ==
           VTOL_CB_SENSOR as libc::c_int as libc::c_uint ||
           (*asSensorStats.offset((*psDroid).asBits[COMP_SENSOR as libc::c_int
                                                        as usize].nStat as
                                      isize)).type_0 as libc::c_uint ==
               INDIRECT_CB_SENSOR as libc::c_int as libc::c_uint ||
           (*asSensorStats.offset((*psDroid).asBits[COMP_SENSOR as libc::c_int
                                                        as usize].nStat as
                                      isize)).type_0 as libc::c_uint ==
               SUPER_SENSOR as libc::c_int as libc::c_uint {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
//testing the new electronic warfare for multiPlayer - AB don't want to release with this in the game!!!!!!
//#define TEST_EW 1
// ////////////////////////////////////////////////////////////////////////////
// give a droid from one player to another - used in Electronic Warfare and multiplayer
//returns the droid created - for single player
#[no_mangle]
pub unsafe extern "C" fn giftSingleDroid(mut psD: *mut DROID, mut to: UDWORD)
 -> *mut DROID {
    let mut sTemplate: DROID_TEMPLATE =
        DROID_TEMPLATE{ref_0: 0,
                       pName: 0 as *const STRING as *mut STRING,
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
                       psNext:
                           0 as *const _droid_template as
                               *mut _droid_template,};
    let mut x: UWORD = 0;
    let mut y: UWORD = 0;
    let mut numKills: UWORD = 0;
    let mut direction: UWORD = 0;
    let mut i: UWORD = 0;
    let mut psNewDroid: *mut DROID = 0 as *mut DROID;
    let mut psCurr: *mut DROID = 0 as *mut DROID;
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut body: UDWORD = 0;
    let mut armourK: UDWORD = 0;
    let mut armourH: UDWORD = 0;
    //leave any group it belongs to  - this gets called in droidRemove()
	/*if(psD->psGroup)
	{
		grpLeave( psD->psGroup, psD);
		psD->psGroup = NULL;
	}*/
	// remove the droid from the cluster systerm - this gets called in droidRemove()
	//clustRemoveObject((BASE_OBJECT *)psD);
    if (*psD).player as libc::c_uint == to { return psD }
    if bMultiPlayer != 0 {
        //reset order
        orderDroid(psD, DORDER_STOP);
        if droidRemove(psD, apsDroidLists.as_mut_ptr()) != 0 {
            // remove droid from one list
            if isHumanPlayer((*psD).player as UDWORD) == 0 {
                droidSetName(psD,
                             b"Enemy Unit\x00" as *const u8 as
                                 *const libc::c_char as *mut STRING);
            }
            //if successfully removed the droid from the players list add it to new player's list
            (*psD).selected = 0 as libc::c_int as UBYTE; // move droid
            (*psD).player = to as UBYTE; // add to other list.
            addDroid(psD, apsDroidLists.as_mut_ptr());
            //the new player may have different default sensor/ecm/repair components
            if (*asSensorStats.offset((*psD).asBits[COMP_SENSOR as libc::c_int
                                                        as usize].nStat as
                                          libc::c_int as isize)).location ==
                   LOC_DEFAULT as libc::c_int as libc::c_uint {
                if (*psD).asBits[COMP_SENSOR as libc::c_int as usize].nStat as
                       libc::c_uint != aDefaultSensor[(*psD).player as usize]
                   {
                    (*psD).asBits[COMP_SENSOR as libc::c_int as usize].nStat =
                        aDefaultSensor[(*psD).player as usize] as UBYTE
                }
            }
            if (*asECMStats.offset((*psD).asBits[COMP_ECM as libc::c_int as
                                                     usize].nStat as
                                       libc::c_int as isize)).location ==
                   LOC_DEFAULT as libc::c_int as libc::c_uint {
                if (*psD).asBits[COMP_ECM as libc::c_int as usize].nStat as
                       libc::c_uint != aDefaultECM[(*psD).player as usize] {
                    (*psD).asBits[COMP_ECM as libc::c_int as usize].nStat =
                        aDefaultECM[(*psD).player as usize] as UBYTE
                }
            }
            if (*asRepairStats.offset((*psD).asBits[COMP_REPAIRUNIT as
                                                        libc::c_int as
                                                        usize].nStat as
                                          libc::c_int as isize)).location ==
                   LOC_DEFAULT as libc::c_int as libc::c_uint {
                if (*psD).asBits[COMP_REPAIRUNIT as libc::c_int as
                                     usize].nStat as libc::c_uint !=
                       aDefaultRepair[(*psD).player as usize] {
                    (*psD).asBits[COMP_REPAIRUNIT as libc::c_int as
                                      usize].nStat =
                        aDefaultRepair[(*psD).player as usize] as UBYTE
                }
            }
        }
        //add back into cluster system
        clustNewDroid(psD);
        //add back into the grid system
        gridAddObject(psD as *mut BASE_OBJECT);
        //check through the 'to' players list of droids to see if any are targetting it
        psCurr = apsDroidLists[to as usize];
        while !psCurr.is_null() {
            if (*psCurr).psTarget == psD as *mut BASE_OBJECT ||
                   (*psCurr).psActionTarget == psD as *mut BASE_OBJECT {
                orderDroid(psCurr, DORDER_STOP);
            }
            //check through order list
            i = 0 as libc::c_int as UWORD;
            while (i as libc::c_int) < (*psCurr).listSize {
                if (*psCurr).asOrderList[i as usize].psOrderTarget ==
                       psD as *mut BASE_OBJECT as *mut libc::c_void {
                    // move the rest of the list down
                    memmove(&mut *(*psCurr).asOrderList.as_mut_ptr().offset(i
                                                                                as
                                                                                isize)
                                as *mut ORDER_LIST as *mut libc::c_void,
                            (&mut *(*psCurr).asOrderList.as_mut_ptr().offset(i
                                                                                 as
                                                                                 isize)
                                 as
                                 *mut ORDER_LIST).offset(1 as libc::c_int as
                                                             isize) as
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
                i = i.wrapping_add(1)
            }
            psCurr = (*psCurr).psNext
        }
        //check through the 'to' players list of structures to see if any are targetting it
        psStruct = apsStructLists[to as usize];
        while !psStruct.is_null() {
            if (*psStruct).psTarget == psD as *mut BASE_OBJECT {
                (*psStruct).psTarget = 0 as *mut BASE_OBJECT
            }
            psStruct = (*psStruct).psNext
        }
        // skirmish callback!
        psScrCBDroidTaken = psD;
        eventFireCallbackTrigger(CALL_UNITTAKEOVER as libc::c_int as
                                     TRIGGER_TYPE);
        psScrCBDroidTaken = 0 as *mut DROID;
        return 0 as *mut DROID
    }
    //got to destroy the droid and build another since there are too many complications re order/action!
    //create a template based on the droid
    templateSetParts(psD, &mut sTemplate);
    //copy the name across
    strcpy(sTemplate.aName.as_mut_ptr(), (*psD).aName.as_mut_ptr());
    x = (*psD).x;
    y = (*psD).y;
    body = (*psD).body;
    armourK = (*psD).armour[WC_KINETIC as libc::c_int as usize];
    armourH = (*psD).armour[WC_HEAT as libc::c_int as usize];
    numKills = (*psD).numKills;
    direction = (*psD).direction;
    //only play the sound if unit being taken over is selectedPlayer's but not going to the selectedPlayer
        //if ((psD->player == selectedPlayer) &&
		//	(psD->player != to))
    if (*psD).player as libc::c_uint == selectedPlayer && to != selectedPlayer
       {
        scoreUpdateVar(WD_UNITS_LOST);
        audio_QueueTrackPos(ID_SOUND_NEXUS_UNIT_ABSORBED as libc::c_int,
                            x as SDWORD, y as SDWORD, (*psD).z as SDWORD);
    }
    //make the old droid vanish
    vanishDroid(psD);
    //create a new droid
    psNewDroid =
        buildDroid(&mut sTemplate, x as UDWORD, y as UDWORD, to,
                   0 as libc::c_int);
    if !psNewDroid.is_null() {
        addDroid(psNewDroid, apsDroidLists.as_mut_ptr());
        (*psNewDroid).body = body;
        (*psNewDroid).armour[WC_KINETIC as libc::c_int as usize] = armourK;
        (*psNewDroid).armour[WC_HEAT as libc::c_int as usize] = armourH;
        (*psNewDroid).numKills = numKills;
        (*psNewDroid).direction = direction;
        if !((*psNewDroid).droidType as libc::c_uint ==
                 DROID_PERSON as libc::c_int as libc::c_uint ||
                 cyborgDroid(psNewDroid) != 0 ||
                 (*psNewDroid).droidType as libc::c_uint ==
                     DROID_TRANSPORTER as libc::c_int as libc::c_uint) {
            updateDroidOrientation(psNewDroid);
        }
    } else {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"giftSingleUnit: unable to build a unit\x00" as *const u8
                      as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"droid.c\x00" as *const u8 as *const libc::c_char,
                  7001 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 16],
                                            &[libc::c_char; 16]>(b"giftSingleDroid\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
    }
    return psNewDroid;
}
/*calculates the electronic resistance of a droid based on its experience level*/
#[no_mangle]
pub unsafe extern "C" fn droidResistance(mut psDroid: *mut DROID) -> SWORD {
    let mut resistance: SWORD = 0;
    resistance =
        ((*psDroid).numKills as libc::c_int * 30 as libc::c_int) as SWORD;
    //ensure base minimum in MP before the upgrade effect
    if bMultiPlayer != 0 {
        //ensure resistance is a base minimum
        if (resistance as libc::c_int) < 30 as libc::c_int {
            resistance = 30 as libc::c_int as SWORD
        }
    }
    //structure resistance upgrades are passed on to droids
    resistance =
        (resistance as libc::c_int +
             resistance as libc::c_int *
                 (asStructureUpgrade[(*psDroid).player as usize].resistance as
                      libc::c_int / 100 as libc::c_int)) as SWORD;
    //ensure resistance is a base minimum
    if (resistance as libc::c_int) < 30 as libc::c_int {
        resistance = 30 as libc::c_int as SWORD
    }
    return resistance;
}
/*this is called to check the weapon is 'allowed'. Check if VTOL, the weapon is
direct fire. Also check numVTOLattackRuns for the weapon is not zero - return
TRUE if valid weapon*/
#[no_mangle]
pub unsafe extern "C" fn checkValidWeaponForProp(mut psTemplate:
                                                     *mut DROID_TEMPLATE)
 -> BOOL {
    let mut psPropStats: *mut PROPULSION_STATS = 0 as *mut PROPULSION_STATS;
    let mut bValid: BOOL = 0;
    bValid = 1 as libc::c_int;
    //check propulsion stat for vtol
    psPropStats =
        asPropulsionStats.offset((*psTemplate).asParts[COMP_PROPULSION as
                                                           libc::c_int as
                                                           usize] as isize);
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"checkValidWeaponForProp: invalid propulsion stats pointer\x00"
                  as *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"droid.c\x00" as *const u8 as *const libc::c_char,
              7049 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 24],
                                        &[libc::c_char; 24]>(b"checkValidWeaponForProp\x00")).as_ptr(),
              b"PTRVALID(psPropStats, sizeof(PROPULSION_STATS))\x00" as
                  *const u8 as *const libc::c_char);
    };
    if (*asPropulsionTypes.offset((*psPropStats).propulsionType as
                                      isize)).travel ==
           AIR as libc::c_int as libc::c_uint {
        //check weapon stat for indirect
		//if (!(asWeaponStats + sCurrDesign.asWeaps[0])->direct)
        if proj_Direct(asWeaponStats.offset((*psTemplate).asWeaps[0 as
                                                                      libc::c_int
                                                                      as
                                                                      usize]
                                                as isize)) == 0 ||
               (*asWeaponStats.offset((*psTemplate).asWeaps[0 as libc::c_int
                                                                as usize] as
                                          isize)).vtolAttackRuns == 0 {
            bValid = 0 as libc::c_int
        }
    } else if (*asWeaponStats.offset((*psTemplate).asWeaps[0 as libc::c_int as
                                                               usize] as
                                         isize)).vtolAttackRuns != 0 {
        bValid = 0 as libc::c_int
    }
    //also checks that there is only a weapon attached and no other system component
    if (*psTemplate).numWeaps == 0 as libc::c_int as libc::c_uint {
        bValid = 0 as libc::c_int
    }
    if (*psTemplate).asParts[COMP_BRAIN as libc::c_int as usize] !=
           0 as libc::c_int &&
           (*psTemplate).asParts[COMP_WEAPON as libc::c_int as usize] !=
               0 as libc::c_int {
        bValid = 0 as libc::c_int
    }
    return bValid;
}
/*called when a Template is deleted in the Design screen*/
#[no_mangle]
pub unsafe extern "C" fn deleteTemplateFromProduction(mut psTemplate:
                                                          *mut DROID_TEMPLATE,
                                                      mut player: UBYTE) {
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut inc: UDWORD = 0;
    let mut i: UDWORD = 0;
    let mut psList: *mut STRUCTURE = 0 as *mut STRUCTURE;
    //see if any factory is currently using the template
    i = 0 as libc::c_int as UDWORD;
    while i < 2 as libc::c_int as libc::c_uint {
        psList = 0 as *mut STRUCTURE;
        match i {
            0 => { psList = apsStructLists[player as usize] }
            1 => { psList = mission.apsStructLists[player as usize] }
            _ => { }
        }
        psStruct = psList;
        while !psStruct.is_null() {
            if StructIsFactory(psStruct) != 0 {
                let mut psFactory: *mut FACTORY =
                    (*psStruct).pFunctionality as *mut FACTORY;
                let mut psNextTemplate: *mut DROID_TEMPLATE =
                    0 as *mut DROID_TEMPLATE;
                //if template belongs to the production player - check thru the production list (if struct is busy)
                if player as libc::c_int == productionPlayer as libc::c_int &&
                       !(*psFactory).psSubject.is_null() {
                    inc = 0 as libc::c_int as UDWORD;
                    while inc < 20 as libc::c_int as libc::c_uint {
                        if asProductionRun[(*(*psFactory).psAssemblyPoint).factoryType
                                               as
                                               usize][(*(*psFactory).psAssemblyPoint).factoryInc
                                                          as
                                                          usize][inc as
                                                                     usize].psTemplate
                               == psTemplate {
                            //if this is the template currently being worked on
                            if psTemplate ==
                                   (*psFactory).psSubject as
                                       *mut DROID_TEMPLATE {
                                //set the quantity to 1 and then use factoryProdAdjust to subtract it
                                asProductionRun[(*(*psFactory).psAssemblyPoint).factoryType
                                                    as
                                                    usize][(*(*psFactory).psAssemblyPoint).factoryInc
                                                               as
                                                               usize][inc as
                                                                          usize].quantity
                                    = 1 as libc::c_int as UBYTE;
                                factoryProdAdjust(psStruct, psTemplate,
                                                  0 as libc::c_int);
                                //init the factory production
                                (*psFactory).psSubject = 0 as *mut BASE_STATS;
                                //check to see if anything left to produce
                                psNextTemplate =
                                    factoryProdUpdate(psStruct,
                                                      0 as
                                                          *mut DROID_TEMPLATE);
                                //power is returned by factoryProdAdjust()
                                if !psNextTemplate.is_null() {
                                    structSetManufacture(psStruct,
                                                         psNextTemplate,
                                                         (*psFactory).quantity);
                                } else {
                                    //nothing more to manufacture - reset the Subject and Tab on HCI Form
                                    intManufactureFinished(psStruct);
                                    //power is returned by factoryProdAdjust()
                                }
                            } else {
                                //just need to initialise this production run
                                asProductionRun[(*(*psFactory).psAssemblyPoint).factoryType
                                                    as
                                                    usize][(*(*psFactory).psAssemblyPoint).factoryInc
                                                               as
                                                               usize][inc as
                                                                          usize].psTemplate
                                    = 0 as *mut _droid_template;
                                asProductionRun[(*(*psFactory).psAssemblyPoint).factoryType
                                                    as
                                                    usize][(*(*psFactory).psAssemblyPoint).factoryInc
                                                               as
                                                               usize][inc as
                                                                          usize].quantity
                                    = 0 as libc::c_int as UBYTE;
                                asProductionRun[(*(*psFactory).psAssemblyPoint).factoryType
                                                    as
                                                    usize][(*(*psFactory).psAssemblyPoint).factoryInc
                                                               as
                                                               usize][inc as
                                                                          usize].built
                                    = 0 as libc::c_int as UBYTE
                            }
                        }
                        inc = inc.wrapping_add(1)
                    }
                } else if (*psFactory).psSubject ==
                              psTemplate as *mut BASE_STATS {
                    //not the production player, so check not being built in the factory for the template player
                    //clear the factories subject and quantity
                    (*psFactory).psSubject = 0 as *mut BASE_STATS;
                    (*psFactory).quantity = 0 as libc::c_int as UBYTE;
                    //return any accrued power
                    if (*psFactory).powerAccrued != 0 {
                        addPower((*psStruct).player as UDWORD,
                                 (*psFactory).powerAccrued);
                    }
                    //tell the interface
                    intManufactureFinished(psStruct);
                }
            }
            psStruct = (*psStruct).psNext
        }
        i = i.wrapping_add(1)
    };
}
/* Return the type of a droid */
/* Return the type of a droid from it's template */
//fills the list with Templates that can be manufactured in the Factory - based on size
// Get an IMD index from a droid template.
/* calculate muzzle tip location in 3d world */
/* gets a template from its name - relies on the name being unique */
/*getTemplateFromSinglePlayerID gets template for unique ID  searching one players list */
/*getTemplateFromMultiPlayerID gets template for unique ID  searching all lists */
// finds a droid for the player and sets it to be the current selected droid
/* Droid experience stuff */
// Get a droid's name.
// Set a droid's name.
// Delete the name from a droid structure.
// Set a templates name.
// returns true when no droid on x,y square.
// true if no droid at x,y
// returns true if it's a sensible place to put that droid.
// true if x,y is an ok place
// returns an x/y coord to place a droid
//initialises the droid movement model
/* Looks through the players list of droids to see if any of them are 
building the specified structure - returns TRUE if finds one*/
/* Looks through the players list of droids to see if any of them are 
demolishing the specified structure - returns TRUE if finds one*/
/* checks the structure for type and capacity and orders the droid to build
a module if it can - returns TRUE if order is set */
/*Deals with building a module - checking if any droid is currently doing this
 - if so, helping to build the current one*/
/*return the name to display for the interface given a DROID structure*/
/*return the name to display for the interface - we don't know if this is 
a string ID or something the user types in*/
/* Just returns true if the droid's present body points aren't as high as the original*/
/* Returns currently active (selected) group */
/*checks to see if an electronic warfare weapon is attached to the droid*/
/*checks to see if the droid is currently being repaired by another*/
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
// Select a droid and do any necessary housekeeping.
//
#[no_mangle]
pub unsafe extern "C" fn SelectDroid(mut psDroid: *mut DROID) {
    (*psDroid).selected = 1 as libc::c_int as UBYTE;
    intRefreshScreen();
}
// De-select a droid and do any necessary housekeeping.
// De-select a droid and do any necessary housekeeping.
//
#[no_mangle]
pub unsafe extern "C" fn DeSelectDroid(mut psDroid: *mut DROID) {
    (*psDroid).selected = 0 as libc::c_int as UBYTE;
    intRefreshScreen();
}
/*calculate the power cost to repair a droid*/
/*calculate the power cost to repair a droid*/
#[no_mangle]
pub unsafe extern "C" fn powerReqForDroidRepair(mut psDroid: *mut DROID)
 -> UWORD {
    let mut powerReq: UWORD = 0; //powerPercent;
    powerReq =
        (repairPowerPoint(psDroid) as
             libc::c_uint).wrapping_mul((*psDroid).originalBody.wrapping_sub((*psDroid).body))
            as UWORD;
    //powerPercent = (UWORD)(100 - PERCENT(psDroid->body, psDroid->originalBody));
    //by including POWER_FACTOR don't have to worry about rounding errors
	//powerReq = (UWORD)((POWER_FACTOR * powerPercent * calcDroidPower(psDroid) *
    //    REPAIR_POWER_FACTOR) / 100);
    return powerReq;
}
/*power cost for One repair point*/
/*power cost for One repair point*/
#[no_mangle]
pub unsafe extern "C" fn repairPowerPoint(mut psDroid: *mut DROID) -> UWORD {
    return (100 as libc::c_int as
                libc::c_uint).wrapping_mul(calcDroidPower(psDroid)).wrapping_div((*psDroid).originalBody).wrapping_mul(1
                                                                                                                           as
                                                                                                                           libc::c_int
                                                                                                                           as
                                                                                                                           libc::c_uint).wrapping_div(5
                                                                                                                                                          as
                                                                                                                                                          libc::c_int
                                                                                                                                                          as
                                                                                                                                                          libc::c_uint)
               as UWORD;
}
/* audio finished callback */
#[no_mangle]
pub unsafe extern "C" fn droidAudioTrackStopped(mut psSample:
                                                    *mut AUDIO_SAMPLE)
 -> BOOL {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"unitAudioTrackStopped: audio sample pointer invalid\n\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"droid.c\x00" as *const u8 as *const libc::c_char,
              7221 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 23],
                                        &[libc::c_char; 23]>(b"droidAudioTrackStopped\x00")).as_ptr(),
              b"PTRVALID(psSample, sizeof(AUDIO_SAMPLE))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if !(*psSample).psObj.is_null() {
        psDroid = (*psSample).psObj as *mut DROID;
        if (*psDroid).type_0 as libc::c_uint ==
               OBJ_DROID as libc::c_int as libc::c_uint &&
               (*psDroid).died == 0 {
            if 1 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"unitAudioTrackStopped: unit pointer invalid\n\x00" as
                          *const u8 as *const libc::c_char);
            };
            if 1 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"droid.c\x00" as *const u8 as *const libc::c_char,
                      7230 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 23],
                                                &[libc::c_char; 23]>(b"droidAudioTrackStopped\x00")).as_ptr(),
                      b"PTRVALID(psDroid, sizeof(DROID))\x00" as *const u8 as
                          *const libc::c_char);
            };
            (*psDroid).iAudioID = NO_SOUND as libc::c_int
        }
    }
    return 1 as libc::c_int;
}
/*returns TRUE if droid type is one of the Cyborg types*/
/*returns TRUE if droid type is one of the Cyborg types*/
#[no_mangle]
pub unsafe extern "C" fn cyborgDroid(mut psDroid: *mut DROID) -> BOOL {
    if (*psDroid).droidType as libc::c_uint ==
           DROID_CYBORG as libc::c_int as libc::c_uint ||
           (*psDroid).droidType as libc::c_uint ==
               DROID_CYBORG_CONSTRUCT as libc::c_int as libc::c_uint ||
           (*psDroid).droidType as libc::c_uint ==
               DROID_CYBORG_REPAIR as libc::c_int as libc::c_uint ||
           (*psDroid).droidType as libc::c_uint ==
               DROID_CYBORG_SUPER as libc::c_int as libc::c_uint {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
