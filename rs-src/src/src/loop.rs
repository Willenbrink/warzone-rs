use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _w_context;
    pub type _formation;
    pub type _gateway;
    #[no_mangle]
    fn mouseY() -> SDWORD;
    #[no_mangle]
    fn mouseX() -> SDWORD;
    #[no_mangle]
    fn keyPressed(code: KEY_CODE) -> BOOL;
    #[no_mangle]
    fn keyDown(code: KEY_CODE) -> BOOL;
    #[no_mangle]
    fn SDL_Delay(ms: Uint32);
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
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    /* Get the string from an ID number */
    #[no_mangle]
    fn strresGetString(psRes: *mut STR_RES, id: UDWORD) -> *mut STRING;
    #[no_mangle]
    fn frameSetCursorFromRes(resID: WORD);
    //for byte fields in a DWORD
    //screen rectangle
    //an area of texture
    //render style for pie draw functions
    // This is the new resource loaded structure (TEXPAGE)
    /* **************************************************************************/
/*
 *	Global Variables
 */
/* **************************************************************************/
    /* **************************************************************************/
/*
 *	Global ProtoTypes
 */
/* **************************************************************************/
    //extern void pie_Draw3DIntelShape(iIMDShape *shape, int frame, int team, UDWORD colour, UDWORD specular, int pieFlag, int pieData);
//extern void pie_Draw3DNowShape(iIMDShape *shape, int frame, int team, UDWORD col, UDWORD spec, int pieFlag, int pieFlagData);
    //PIEVERTEX line draw for all hardware modes
    //iVetrex triangle draw for software modes
    //PIEVERTEX poly draw for all hardware modes
    //PIEVERTEX triangle draw (glide specific)
    #[no_mangle]
    fn pie_GetResetCounts(pPieCount: *mut SDWORD, pTileCount: *mut SDWORD,
                          pPolyCount: *mut SDWORD, pStateCount: *mut SDWORD);
    #[no_mangle]
    fn pie_LoadBackDrop(screenType: SCREENTYPE, b3DFX: BOOL);
    //Sets all states
    #[no_mangle]
    fn pie_SetDepthBufferStatus(depthMode: DEPTH_MODE);
    //fog currently on
    #[no_mangle]
    fn pie_SetFogStatus(val: BOOL);
    //mouse states
    #[no_mangle]
    fn pie_SetMouse(ImageFile: *mut IMAGEFILE, ImageID: UWORD);
    #[no_mangle]
    fn pie_ScreenFlip(ClearMode: CLEAR_MODE);
    #[no_mangle]
    fn pie_GlobalRenderBegin();
    #[no_mangle]
    fn pie_GlobalRenderEnd(bForceClearToBlack: BOOL);
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
    #[no_mangle]
    fn getHQExists(player: UDWORD) -> BOOL;
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
    #[no_mangle]
    fn moveUpdateBaseSpeed();
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
    static mut apsDroidLists: [*mut DROID; 8];
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
    #[no_mangle]
    fn droidUpdate(psDroid: *mut DROID);
    #[no_mangle]
    static mut apsStructLists: [*mut STRUCTURE; 8];
    #[no_mangle]
    fn structureUpdate(psBuilding: *mut STRUCTURE);
    #[no_mangle]
    fn setHQExists(state: BOOL, player: UDWORD);
    /*sets the flag to indicate a SatUplink Exists - so draw everything!*/
    #[no_mangle]
    fn setSatUplinkExists(state: BOOL, player: UDWORD);
    #[no_mangle]
    static mut asWeaponStats: *mut WEAPON_STATS;
    /*returns the status of the flag*/
    /*sets the flag to indicate a Las Sat Exists - ONLY EVER WANT ONE*/
    #[no_mangle]
    fn setLasSatExists(state: BOOL, player: UDWORD);
    #[no_mangle]
    static mut apsFeatureLists: [*mut FEATURE; 8];
    /*
 * Feature.h
 *
 * Definitions for the feature structures.
 *
 */
    //they're just not there anymore!!!!! Ye ha!
    /* The statistics for the features */
    //Value is stored for easy access to this feature in destroyDroid()/destroyStruct()
//extern UDWORD			droidFeature;
    /* Load the feature stats */
    /* Release the feature stats memory */
    // Set the tile no draw flags for a structure
    /* Create a feature on the map */
    /* Release the resources associated with a feature */
    /* Update routine for features */
    #[no_mangle]
    fn featureUpdate(psFeat: *mut FEATURE);
    /* **************************************************************************/
    #[no_mangle]
    fn animObj_Update();
    /* The list of destroyed objects */
    /* Initialise the object heaps */
    /* Release the object heaps */
    /* General housekeeping for the object system */
    #[no_mangle]
    fn objmemUpdate();
    /* **************************************************************************/
    /* **************************************************************************/
    /* **************************************************************************/
    #[no_mangle]
    fn audio_StopAll();
    #[no_mangle]
    fn audio_Update() -> BOOL;
    /* Tidy up after a mode change */
    #[no_mangle]
    fn dispModeChange() -> BOOL;
    /* Process the user input. This just processes the key input and jumping around the radar*/
//extern BOOL processInput(void);
    #[no_mangle]
    fn ProcessRadarInput();
    #[no_mangle]
    fn processInput();
    /*don't want to do any of these whilst in the Intelligence Screen*/
    #[no_mangle]
    fn processMouseClickInput();
    #[no_mangle]
    fn scroll();
    /* Do the 3D display */
    #[no_mangle]
    fn displayWorld();
    #[no_mangle]
    static mut dragBox3D: _dragBox;
    #[no_mangle]
    static mut wallDrag: _dragBox;
    //extern BOOL	widgetsOn;
    #[no_mangle]
    static mut rotActive: BOOL;
    /*
 * HCI.h
 *
 * Function definitions for the in game interface code.
 */
    //#include "intimage.h"
    // store the objects that are being used for the object bar
    //10 we need at least 15 for the 3 different types of factory
    // The reticule form
    // option button
    // build button
    // manufacture button
    // research button
    // intelligence map button
    // design droids button
    // central cancel button
    // command droid button
    // transporter button
    // droid order button
    // power bar - trough
    //transporter button on timer display
    /* Object screen IDs */
    // The object back form for build/manufacture/research
    // The form for the close button
    // The first ID for droids/factories/research
    // The last ID for droids/factories/research
    // The first ID for stats
    // The last ID for stats
    // The first ID for stats progress bars.
    // The last ID for stats progress bars.
    // The first ID for power bars.
    // The first ID for power bars.
    // The first ID for progress number labels.
    // The last ID for progress number labels.
    // The object tab form for build/manufacture/research
    // The first ID for factory number labels
    // The last ID for factory number labels
    // The first ID for factory number labels
    // The last ID for factory number labels
    // The first ID for factory number labels
    // The last ID for factory number labels
    // The first ID for VTOL factory number labels
    // The last ID for VTOL factory number labels
    // The stats form for structure/droid/research type
    // The form for the close box
    // Unused
    // The stats close box
    // The tab form with the stats buttons
    // The first stats ID
    // The last stats ID
    //#define IDSTAT_BARSTART		4200
    // Reticule position.
    /* Option positions */
    // Object screen position.
    // X coord of object screen back form.
    // Y coord of object screen back form.
    //316		// Width of object screen back form.
    // Height of object screen back form.
    /* Build screen positions */
    // X coord of object screen tab form.
    // Y coord of object screen tab form.
    //312//310	// Width of object screen tab form.
    // Height of object screen tab form.
    // Gap between buttons.
    // Offset of first obj button from left of tab form.
    //44	// Offset of first obj button from top of tab form.
    //slider bar positions
    // Slider x.
    // Slider y.
    // Slider width.
    //4	// Slider height.
    // Power bar position.
    //tab details
    /* close button data */
    // Stat screen position.
    // Width of the tab form.
    // Height of the tab form.
    // Offset of the tab form within the main form.
    // Offset of the tab form within the main form.
    // 2 16 bit values packed into a DWORD.
    // 3 10 bit values packed into a DWORD.
    // 4 8 bit values packed into a DWORD.
    //#define BUILDPOINTS_STRUCTDIV 1
//#define BUILDPOINTS_DROIDDIV 5
//#define POWERPOINTS_STRUCTDIV 1
    //3
    // Button width.
    // Button height.
    // Stat window slider offset.
    // Slider number of stops.
    //0xcc
    //0
    //0
    /* maximum array sizes */
    //40 can have 80 topic displayed at one now AB 13/09/99
    //20
    // Standard mode (just the reticule)
    // Option screen
    // Edit mode
    // Stat screen up for placing objects
    // Object screen
    // Object screen with stat screen
    // Object screen with command droids and orders screen
    // Design screen
    // Intelligence Map
    // in game options.
    //INT_TUTORIAL,	// Tutorial mode - message display
    //Loading/unloading a Transporter
    // Results of a mission display.
    // multiplayer only, player stats etc...
    // CD Change message box
    //leave as last so we can start the objMode at this value
    //NOT ANYMORE! 10/08/98 AB
//#ifndef PSX
//#define INCLUDE_PRODSLIDER	// Include quantity slider in manufacture window.
//#endif
    //#ifndef PSX
    //#endif
    /* The widget screen */
    /* the widget font */
    /* Which is the currently selected player */
    // The last widget ID from widgRunScreen
    /* The button ID of the objects stat when the stat screen is displayed */
    /* The flag to specify if the Intelligence screen is up */
//extern BOOL				intelMapUp;
    /* The current template for the design screen to start with*/
    //two colours used for drawing the footprint outline for objects in 2D
    //two colours used for drawing the footprint outline for objects in 3D
    //arbitary value!
    //value gets set to colour used for drawing
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
    #[no_mangle]
    fn intDisplayWidgets();
    /* Initialise the set of widgets that make up a screen.
 * Call this once before calling widgRunScreen and widgDisplayScreen.
 * This should only be called once before calling Run and Display as many times
 * as is required.
 */
    /* Clean up after a screen has been run.
 * Call this after the widgRunScreen / widgDisplayScreen cycle.
 */
    /* Execute a set of widgets for one cycle.
 * Return the id of the widget that was activated, or 0 for none.
 */
    /* Display the screen's widgets in their current state
 * (Call after calling widgRunScreen, this allows the input
 *  processing to be seperated from the display of the widgets).
 */
    // Set the current audio callback function and audio id's.
    // Get pointer to current audio callback function.
    // Get current audio ID for hilight.
    // Get current audio ID for clicked.
    #[no_mangle]
    fn getWidgetsStatus() -> BOOL;
    #[no_mangle]
    fn setWidgetsStatus(var: BOOL);
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
    #[no_mangle]
    fn intResetScreen(NoAnim: BOOL);
    /* Refresh icons on the interface, without disturbing the layout. i.e. smartreset*/
    /* Add the options widgets to the widget screen */
    /* Remove the stats widgets from the widget screen */
    /* Remove the stats widgets from the widget screen */
    /*sets which list of structures to use for the interface*/
    //sets up the Transporter Screen as far as the interface is concerned
    /*causes a reticule button to start flashing*/
    // stop a reticule button flashing
    //toggles the Power Bar display on and off
    //displays the Power Bar
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
    fn HandleClosingWindows();
    #[no_mangle]
    fn intRunWidgets() -> INT_RETVAL;
    #[no_mangle]
    static mut selectedPlayer: UDWORD;
    #[no_mangle]
    static mut intMode: INTMODE;
    #[no_mangle]
    fn widgRunScreen(psScreen: *mut W_SCREEN) -> UDWORD;
    #[no_mangle]
    static mut psWScreen: *mut W_SCREEN;
    #[no_mangle]
    fn intRunInGameOptions() -> BOOL;
    #[no_mangle]
    fn intProcessInGameOptions(_: UDWORD);
    #[no_mangle]
    static mut InGameOpUp: BOOL;
    /* Update the AI for a player */
    #[no_mangle]
    fn playerUpdate(player: UDWORD);
    /*
 * GTime.h
 *
 * Interface to the game clock.
 *
 */
    /* The number of ticks per second for the game clock */
    // The maximum time for one frame (stops the clock running away when debugging)
// changed to /6 by ajl. if this needs to go back to ticks/10 then tell me. 
    /* The current time in the game world */
    #[no_mangle]
    static mut gameTime: UDWORD;
    /* The current time in the game world */
    #[no_mangle]
    static mut gameTime2: UDWORD;
    /* Call this to stop the game timer */
    #[no_mangle]
    fn gameTimeStop();
    /* Call this to restart the game timer after a call to gameTimeStop */
    #[no_mangle]
    fn gameTimeStart();
    #[no_mangle]
    fn CoordInRadar(x: libc::c_int, y: libc::c_int) -> BOOL;
    #[no_mangle]
    fn downloadAtStartOfFrame();
    #[no_mangle]
    fn proj_UpdateAll();
    #[no_mangle]
    fn addConsoleMessage(messageText: *mut STRING,
                         jusType: CONSOLE_TEXT_JUSTIFICATION) -> BOOL;
    #[no_mangle]
    fn updateConsoleMessages();
    /* Update current power based on what was extracted during the last cycle and 
   what Power Generators exist */
    #[no_mangle]
    fn updatePlayerPower(player: UDWORD);
    //won't bother with this on PSX unless starts being used too much!
    //this is a check cos there is a problem with the power but not sure where!!
    #[no_mangle]
    fn powerCheck(bBeforePowerUsed: BOOL, player: UBYTE);
    #[no_mangle]
    static mut radarOnScreen: BOOL;
    // the game description.
    #[no_mangle]
    static mut bMultiPlayer: BOOL;
    #[no_mangle]
    static mut bDisplayMultiJoiningStatus: UBYTE;
    #[no_mangle]
    fn multiPlayerLoop() -> BOOL;
    #[no_mangle]
    fn eventProcessTriggers(currTime: UDWORD);
    #[no_mangle]
    fn eventFireCallbackTrigger(callback: TRIGGER_TYPE);
    // update the visibility reduction
    #[no_mangle]
    fn visUpdateLevel();
    // return whether a message is immediate
    #[no_mangle]
    fn messageIsImmediate() -> BOOL;
    /*sets the flag*/
    #[no_mangle]
    fn setMessageImmediate(state: BOOL);
    /* **************************************************************************/
/*
 *	Global Variables
 */
/* **************************************************************************/
    #[no_mangle]
    static mut bLoadSaveUp: BOOL;
    // true when interface is up and should be run.
    //the name of the save game to load from the front end
    #[no_mangle]
    static mut saveGameName: [STRING; 256];
    #[no_mangle]
    static mut sRequestResult: [STRING; 255];
    #[no_mangle]
    static mut bRequestLoad: BOOL;
    #[no_mangle]
    fn runLoadSave(bResetMissionWidgets: BOOL) -> BOOL;
    // return whether the save screen was displayed in the mission results screen
    #[no_mangle]
    fn saveInMissionRes() -> BOOL;
    // return whether the save screen was displayed in the middle of a mission
    #[no_mangle]
    fn saveMidMission() -> BOOL;
    #[no_mangle]
    fn deleteSaveGame(saveGameName_0: *mut libc::c_char);
    #[no_mangle]
    fn saveGame(aFileName: *mut STRING, saveType: SDWORD) -> BOOL;
    /* The string resource object */
    #[no_mangle]
    static mut psStringRes: *mut STR_RES;
    #[no_mangle]
    fn screen_StopBackDrop();
    #[no_mangle]
    fn screen_RestartBackDrop();
    #[no_mangle]
    fn screen_GetBackDrop() -> BOOL;
    /* Toggle the display between full screen or windowed */
    #[no_mangle]
    fn screenToggleMode();
    #[no_mangle]
    fn intDisplayMultiJoiningStatus(joinCount: UBYTE) -> BOOL;
    #[no_mangle]
    static mut IntImages: *mut IMAGEFILE;
    //full screen render
//extern BOOL seq_PlayVideo(char* pSeq, char* pAudio);
//extern BOOL seq_StartFullScreenVideo(char* sequenceFile, char* audioFile);//start videos through seqList
    #[no_mangle]
    fn seq_UpdateFullScreenVideo(bClear: *mut CLEAR_MODE) -> BOOL;
    #[no_mangle]
    fn seq_StopFullScreenVideo() -> BOOL;
    /*checks to see if there are any sequences left in the list to play*/
    #[no_mangle]
    fn seq_AnySeqLeft() -> BOOL;
    /*returns the next sequence in the list to play*/
    #[no_mangle]
    fn seq_StartNextFullScreenVideo();
    #[no_mangle]
    fn cdAudio_Resume() -> BOOL;
    #[no_mangle]
    static mut mission: MISSION;
    #[no_mangle]
    static mut apsLimboDroids: [*mut DROID; 8];
    #[no_mangle]
    fn missionDestroyObjects();
    //sets up the game to start a new mission
//extern BOOL setUpMission(MISSION_TYPE type);
    #[no_mangle]
    fn setUpMission(type_0: UDWORD) -> BOOL;
    /* The update routine for all Structures left back at base during a Mission*/
    #[no_mangle]
    fn missionStructureUpdate(psBuilding: *mut STRUCTURE);
    /* The update routine for all droids left back at home base
Only interested in Transporters at present*/
    #[no_mangle]
    fn missionDroidUpdate(psDroid: *mut DROID);
    /*update routine for mission details */
    #[no_mangle]
    fn missionTimerUpdate();
    #[no_mangle]
    fn getWarCamStatus() -> BOOL;
    #[no_mangle]
    static mut fogStatus: UDWORD;
    // compact some of the grid arrays
    #[no_mangle]
    fn gridGarbageCollect();
    #[no_mangle]
    fn process3DBuilding() -> BOOL;
    #[no_mangle]
    fn driveUpdate();
    #[no_mangle]
    fn getDrivingStatus() -> BOOL;
    // update the findpath system each frame
    #[no_mangle]
    fn fpathUpdate();
    // whether the tutorial is active
    #[no_mangle]
    static mut bInTutorial: BOOL;
    // update routine for the cluster system
    #[no_mangle]
    fn clusterUpdate();
    #[no_mangle]
    fn mixer_GetWavVolume() -> SDWORD;
    #[no_mangle]
    fn mixer_SetWavVolume(iVol: SDWORD);
    // create a command droid
//extern BOOL buildCommandDroid(SDWORD player, SDWORD droid, DROID *psDroid);
    // destroy a command droid
//extern void destroyCommandDroid(SDWORD player, SDWORD droid);
    // update the command droids
    #[no_mangle]
    fn cmdDroidUpdate();
    #[no_mangle]
    fn kf_TogglePauseMode();
    #[no_mangle]
    fn displayGameOver(success: BOOL) -> BOOL;
    #[no_mangle]
    fn getScriptWinLoseVideo() -> UBYTE;
    // check the valid flags and scream if the power isn't valid
    #[no_mangle]
    fn pwrcUpdate();
}
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type UBYTE = libc::c_uchar;
pub type SBYTE = libc::c_schar;
pub type STRING = libc::c_char;
pub type UWORD = libc::c_ushort;
pub type SWORD = libc::c_short;
pub type UDWORD = libc::c_uint;
pub type SDWORD = libc::c_int;
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
// win32 functions to POSIX
// WIN32
pub type BOOL = libc::c_int;
pub type WORD = libc::c_short;
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
    SDL - Simple DirectMedia Layer
    Copyright (C) 1997-2012 Sam Lantinga

    This library is free software; you can redistribute it and/or
    modify it under the terms of the GNU Lesser General Public
    License as published by the Free Software Foundation; either
    version 2.1 of the License, or (at your option) any later version.

    This library is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
    Lesser General Public License for more details.

    You should have received a copy of the GNU Lesser General Public
    License along with this library; if not, write to the Free Software
    Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA

    Sam Lantinga
    slouken@libsdl.org
*/
/* * @file SDL_stdinc.h
 *  This is a general header that includes C language support
 */
/* * The number of elements in an array */
/* Use proper C++ casts when compiled as C++ to be compatible with the option
 -Wold-style-cast of GCC (and -Werror=old-style-cast in GCC 4.2 and above. */
/* * @name Basic data types */
/*@{*/
pub type Uint32 = uint32_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _free_object {
    pub psNext: *mut _free_object,
}
pub type FREE_OBJECT = _free_object;
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
pub type TREAP_CMP
    =
    Option<unsafe extern "C" fn(_: UDWORD, _: UDWORD) -> SDWORD>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _treap {
    pub cmp: TREAP_CMP,
    pub psNodes: *mut OBJ_HEAP,
    pub psRoot: *mut TREAP_NODE,
}
pub type TREAP = _treap;
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
pub type GAMECODE = libc::c_uint;
pub const GAMECODE_LOADGAME: GAMECODE = 6;
pub const GAMECODE_FASTEXIT: GAMECODE = 5;
pub const GAMECODE_NEWLEVEL: GAMECODE = 4;
pub const GAMECODE_PLAYVIDEO: GAMECODE = 3;
pub const GAMECODE_QUITGAME: GAMECODE = 2;
pub const GAMECODE_RESTARTGAME: GAMECODE = 1;
pub const GAMECODE_CONTINUE: GAMECODE = 0;
pub type LOOP_MISSION_STATE = libc::c_uint;
pub const LMS_CLEAROBJECTS: LOOP_MISSION_STATE = 5;
pub const LMS_LOADGAME: LOOP_MISSION_STATE = 4;
pub const LMS_NEWLEVEL: LOOP_MISSION_STATE = 3;
pub const LMS_SAVECONTINUE: LOOP_MISSION_STATE = 2;
pub const LMS_SETUPMISSION: LOOP_MISSION_STATE = 1;
pub const LMS_NORMAL: LOOP_MISSION_STATE = 0;
// The next free ID
// off map saving any droids (selectedPlayer) at end into apsLimboDroids
pub const LDS_NONE: _level_type = 10;
pub type CLEAR_MODE = libc::c_uint;
pub const CLEAR_FOG: CLEAR_MODE = 3;
pub const CLEAR_BLACK: CLEAR_MODE = 2;
pub const CLEAR_OFF_AND_NO_BUFFER_DOWNLOAD: CLEAR_MODE = 1;
pub const CLEAR_OFF: CLEAR_MODE = 0;
//holds which pause is valid at any one time
pub type PAUSE_STATE = _pause_state;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _pause_state {
    #[bitfield(name = "gameUpdatePause", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "audioPause", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "scriptPause", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "scrollPause", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "consolePause", ty = "libc::c_uint", bits = "4..=4")]
    pub gameUpdatePause_audioPause_scriptPause_scrollPause_consolePause: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
pub type DEPTH_MODE = libc::c_uint;
pub const DEPTH_CMP_ALWAYS_WRT_OFF: DEPTH_MODE = 3;
pub const DEPTH_CMP_LEQ_WRT_OFF: DEPTH_MODE = 2;
pub const DEPTH_CMP_ALWAYS_WRT_ON: DEPTH_MODE = 1;
pub const DEPTH_CMP_LEQ_WRT_ON: DEPTH_MODE = 0;
pub const INT_NONE: _int_retval = 0;
pub type INT_RETVAL = _int_retval;
pub type _int_retval = libc::c_uint;
pub const INT_QUIT: _int_retval = 3;
pub const INT_INTELNOSCROLL: _int_retval = 2;
pub const INT_INTERCEPT: _int_retval = 1;
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
pub type CONSOLE_TEXT_JUSTIFICATION = libc::c_uint;
pub const DEFAULT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 3;
pub const CENTRE_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 2;
pub const RIGHT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 1;
pub const LEFT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 0;
pub const STR_GAME_SAVED: _fixed_str_id = 298;
// User saved game - at the start of a level.
pub const GTYPE_SAVE_MIDMISSION: C2RustUnnamed_0 = 4;
// Stand alone mission.
pub const GTYPE_SAVE_START: C2RustUnnamed_0 = 3;
pub type W_SCREEN = _w_screen;
// ID of the IVIS font to use for tool tips.
/* The screen structure which stores all info for a widget screen */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_screen {
    pub psForm: *mut WIDGET,
    pub psFocus: *mut WIDGET,
    pub TipFontID: libc::c_int,
}
pub type WIDGET = _widget;
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
pub type WIDGET_TYPE = _widget_type;
pub type _widget_type = libc::c_uint;
pub const WIDG_SLIDER: _widget_type = 5;
pub const WIDG_BARGRAPH: _widget_type = 4;
pub const WIDG_EDITBOX: _widget_type = 3;
pub const WIDG_BUTTON: _widget_type = 2;
pub const WIDG_LABEL: _widget_type = 1;
pub const WIDG_FORM: _widget_type = 0;
pub const INT_INGAMEOP: INTMODE = 10;
pub type INTMODE = libc::c_uint;
pub const INT_MAXMODE: INTMODE = 15;
pub const INT_CDCHANGE: INTMODE = 14;
pub const INT_MULTIMENU: INTMODE = 13;
pub const INT_MISSIONRES: INTMODE = 12;
pub const INT_TRANSPORTER: INTMODE = 11;
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
pub type FEATURE = _feature;
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
pub type FEATURE_STATS = _feature_stats;
// Feature armour
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
pub type uint16 = libc::c_ushort;
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
pub struct iVector {
    pub x: int32,
    pub y: int32,
    pub z: int32,
}
pub type int32 = libc::c_int;
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
pub type uint8 = libc::c_uchar;
pub type VERTEXID = libc::c_int;
pub type uint32 = libc::c_uint;
pub type FEATURE_TYPE = _feature_type;
// points to first polygon in the BSP tree entry ... BSP_NextPoly in the iIMDPoly structure will point to the next entry
// id of the first polygon in the list ... or BSPPOLYID_TERMINATE for none
// id of the first polygon in the list ... or BSPPOLYID_TERMINATE for none
// only needed when generating the tree
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
pub type SCREEN_DISP_DATA = _screen_disp_data;
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
pub type OBJECT_TYPE = _object_type;
pub type _object_type = libc::c_uint;
pub const OBJ_TARGET: _object_type = 4;
pub const OBJ_BULLET: _object_type = 3;
pub const OBJ_FEATURE: _object_type = 2;
pub const OBJ_STRUCTURE: _object_type = 1;
pub const OBJ_DROID: _object_type = 0;
pub const WSC_LAS_SAT: _weapon_subclass = 13;
// weapon subclasses used to define which weapons are affected by weapon upgrade functions
pub type WEAPON_SUBCLASS = _weapon_subclass;
pub type _weapon_subclass = libc::c_uint;
pub const NUM_WEAPON_SUBCLASS: _weapon_subclass = 17;
pub const WSC_EMP: _weapon_subclass = 16;
pub const WSC_COMMAND: _weapon_subclass = 15;
pub const WSC_BOMB: _weapon_subclass = 14;
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
pub type WEAPON_STATS = _weapon_stats;
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
//WSC_ARTILLARY,
//WSC_CLOSECOMBAT,
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
//used to modify the damage to a propuslion type (or structure) based on weapon
pub type WEAPON_EFFECT = _weapon_effect;
pub type _weapon_effect = libc::c_uint;
pub const WE_NUMEFFECTS: _weapon_effect = 6;
pub const WE_ANTI_AIRCRAFT: _weapon_effect = 5;
pub const WE_FLAMER: _weapon_effect = 4;
pub const WE_ARTILLERY_ROUND: _weapon_effect = 3;
pub const WE_BUNKER_BUSTER: _weapon_effect = 2;
pub const WE_ANTI_TANK: _weapon_effect = 1;
pub const WE_ANTI_PERSONNEL: _weapon_effect = 0;
// used to define which projectile model to use for the weapon
pub type MOVEMENT_MODEL = _movement_model;
pub type _movement_model = libc::c_uint;
pub const NUM_MOVEMENT_MODEL: _movement_model = 6;
pub const MM_SWEEP: _movement_model = 5;
pub const MM_ERRATICDIRECT: _movement_model = 4;
pub const MM_HOMINGINDIRECT: _movement_model = 3;
pub const MM_HOMINGDIRECT: _movement_model = 2;
pub const MM_INDIRECT: _movement_model = 1;
pub const MM_DIRECT: _movement_model = 0;
//only using KINETIC and HEAT for now
pub type WEAPON_CLASS = _weapon_class;
pub type _weapon_class = libc::c_uint;
pub const NUM_WEAPON_CLASS: _weapon_class = 2;
pub const WC_HEAT: _weapon_class = 1;
pub const WC_KINETIC: _weapon_class = 0;
//bullets etc
//WC_EXPLOSIVE,	//rockets etc - classed as WC_KINETIC now to save space in struct _droid AB 25/11/98 
//laser etc
//WC_MISC,		//others we haven't thought of! - classed as WC_HEAT now to save space in struct _droid AB 25/11/98 
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
//no capability - droid must stop
//partial capability - droid has 50% chance to hit
//full capability - droid fires normally on move
//ALL components and structures and research topics have a tech level to which they belong
pub type TECH_LEVEL = _tech_level;
pub type _tech_level = libc::c_uint;
pub const MAX_TECH_LEVELS: _tech_level = 6;
pub const TECH_LEVEL_ALL: _tech_level = 5;
pub const TECH_LEVEL_TWO_THREE: _tech_level = 4;
pub const TECH_LEVEL_ONE_TWO: _tech_level = 3;
pub const TECH_LEVEL_THREE: _tech_level = 2;
pub const TECH_LEVEL_TWO: _tech_level = 1;
pub const TECH_LEVEL_ONE: _tech_level = 0;
pub type WEAPON = _weapon;
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
pub type STRUCTURE = _structure;
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
/* **************************************************************************/
/* struct member macros */
/* this must be the last entry in this structure */
/* **************************************************************************/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct COMPONENT_OBJECT {
    pub position: VECTOR3D,
    pub orientation: VECTOR3D,
    pub psParent: *mut libc::c_void,
    pub psShape: *mut iIMDShape,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VECTOR3D {
    pub x: SDWORD,
    pub y: SDWORD,
    pub z: SDWORD,
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
pub type ANIMOBJDONEFUNC
    =
    Option<unsafe extern "C" fn(_: *mut ANIM_OBJECT) -> ()>;
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
/* **************************************************************************/
/*
 * Anim.h
 *
 * Animation types and function headers
 *
 * Gareth Jones 11/7/97
 */
/* **************************************************************************/
/* **************************************************************************/
/* **************************************************************************/
/* **************************************************************************/
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
pub type BASE_OBJECT = _base_object;
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
pub type FUNCTIONALITY = [UBYTE; 40];
pub type STRUCTURE_STATS = _structure_stats;
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
 * FunctionDef.h
 *
 * Structure defs for functions.
 *
 */
//DEFENSIVE_STRUCTURE_TYPE,
	//RADAR_MAP_TYPE,
	//POWER_REG_TYPE,
	//POWER_RELAY_TYPE,
	//ARMOUR_UPGRADE_TYPE,
	//REPAIR_UPGRADE_TYPE,
	//RESISTANCE_UPGRADE_TYPE,
	//DROID_DESIGN_TYPE,
	//MAP_MARKER_TYPE,
	//SKY_DOME_MAP_TYPE,
	//BODY_UPGRADE_TYPE,
	//HQ_TYPE,
/* The number of function types */
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
pub type SENSOR_TYPE = _sensor_type;
pub type _sensor_type = libc::c_uint;
pub const SUPER_SENSOR: _sensor_type = 4;
pub const VTOL_INTERCEPT_SENSOR: _sensor_type = 3;
pub const VTOL_CB_SENSOR: _sensor_type = 2;
pub const INDIRECT_CB_SENSOR: _sensor_type = 1;
pub const STANDARD_SENSOR: _sensor_type = 0;
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
pub type STRUCT_STRENGTH = _struct_strength;
pub type _struct_strength = libc::c_uint;
pub const NUM_STRUCT_STRENGTH: _struct_strength = 4;
pub const STRENGTH_BUNKER: _struct_strength = 3;
pub const STRENGTH_HARD: _struct_strength = 2;
pub const STRENGTH_MEDIUM: _struct_strength = 1;
pub const STRENGTH_SOFT: _struct_strength = 0;
pub const SS_BUILT: _struct_states = 1;
pub const REF_SAT_UPLINK: C2RustUnnamed = 21;
pub const REF_HQ: C2RustUnnamed = 0;
//works as all of the above together! - new for updates - added 11/06/99 AB
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
pub type FLAG_POSITION = _flag_position;
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
pub type POSITION_TYPE = _position_type;
pub type _position_type = libc::c_uint;
pub const POS_TEMPDELIVERY: _position_type = 3;
pub const POS_PROXOBJ: _position_type = 2;
pub const POS_PROXDATA: _position_type = 1;
pub const POS_DELIVERY: _position_type = 0;
pub type DROID = _droid;
//UDWORD		initialPower;	    HAVEN'T FOUND A USE FOR IT YET! AB 26/8/98	
									    /* what the initial power level is - set 
									    in script not sure if will need it, but 
									    keeping for now*/
/* the current amount of power avaialble 
									    to the player*/
/* the power extracted but not converted */
/*the last object that received power 
									    before it ran out*/
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
pub type MOVE_CONTROL = _move_control;
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
pub type PATH_POINT = _path_point;
/* Only needed for Alex's movement update ? */
//	UDWORD	timeStarted;
//	UDWORD	arrivalTime;
//	UDWORD	pathStarted;
//	BOOL	startedMoving;
//	UDWORD	lastTime;
//	BOOL	speedChange;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _base_stats {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
}
pub type ORDER_LIST = _order_list;
//line build requires two sets of coords
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
pub type COMPONENT = _component;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _component {
    pub nStat: UBYTE,
}
pub type DROID_TYPE = _droid_type;
// Allowing a maximum of 255 stats per file
//UDWORD					hitPoints; NOT USED?
/* Possibly a set of function pointers here to be called when
	 * damage is done to a component, power is low ...
	 */
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
/* Store a map coordinate and it's associated tile, used by mapCalcLine */
pub type TILE_COORD = _tile_coord;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _tile_coord {
    pub x: UDWORD,
    pub y: UDWORD,
    pub psTile: *mut MAPTILE,
}
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
pub const IMAGE_CURSOR_DEFAULT: C2RustUnnamed_1 = 232;
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
pub struct iSprite {
    pub width: libc::c_uint,
    pub height: libc::c_uint,
    pub bmp: *mut iBitmap,
}
pub type iBitmap = uint8;
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
pub const CALL_VIDEO_QUIT: _scr_callback_types = 15;
pub type C2RustUnnamed = libc::c_uint;
pub const NUM_DIFF_BUILDINGS: C2RustUnnamed = 22;
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
pub type _struct_states = libc::c_uint;
pub const SS_BEING_DEMOLISHED: _struct_states = 2;
pub const SS_BEING_BUILT: _struct_states = 0;
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
// Trigger fires when the script is first run
// Trigger uses script code
// Trigger after a time pause
// Trigger at repeated intervals
// Event has paused for an interval and will restart in the middle of it's code
// The user defined callback triggers should start with this id
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
/*
 * game.h	  
 *
 * load and save game routines.
 * Very likely to ALL change! All the headers are separately defined at the 
 * moment - they probably don't need to be - if no difference make into one. 
 * Also the struct defintions throughout the game could be re-ordered to contain 
 * the variables required for saving so that don't need to create a load more here!
 */
/* **************************************************************************/
/*
 *	Global Definitions
 */
/* **************************************************************************/
//the version number used in save games
//#define VERSION_1				1				//demo save games
//#define VERSION_2				2				//names saved for components/structures/features
//#define VERSION_3				3				//changes to SAVE_GAME
//#define VERSION_4				4				//changes to SAVE_GAME
//#define VERSION_5				5				//different types of save game added
//#define VERSION_6				6				//level name added to a user save game
//string ID names saved for comps an objects
//research status saved out for user saved games
//power and experience saved out for user saved games
//includes gateways and zones in .map file.
//newstyle save game with extending structure checked in 13 Nov.
//mission and order stuff checked in 20 Nov.
//odds and ends to 24 Nov. and hashed scripts
//
//
//beta save game
//objId and new struct stats included
//droid name savegame validity stamps
//alliances, colours, radar zoom
//MAX_NAME_ SIZE replaced by MAX_SAVE_NAME_SIZE
//timeStartHold saved out for research facilities
//asRundData
//limbo droids and camstart droids saved properly, no cluster save
//reinforceTime, droid move, droid resistance
//limbo droid, research module hold fixed, cleaned by Alex
//reArm pads
//unit not the "d" word, experience and repair facility currentPtsAdded
//rearm pads currentPtsAdded saved
//mission scroll limits saved
//script external variables saved
//mission cheat time saved
//factory secondary order saved
//skirmish save 
//used in the loadGame
pub type C2RustUnnamed_0 = libc::c_uint;
// User saved game - in the middle of a level
// Scenario scroll area expansion.
pub const GTYPE_MISSION: C2RustUnnamed_0 = 2;
// Initial scenario state.
pub const GTYPE_SCENARIO_EXPAND: C2RustUnnamed_0 = 1;
pub const GTYPE_SCENARIO_START: C2RustUnnamed_0 = 0;
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
/*
 * Loop.c
 *
 * The main game loop
 *
 */
/* loop position printf's */
//#define DEBUG_GROUP1
// FIXME Direct iVis implementation include!
//ivis render code
// FIXME Direct iVis implementation inlcude!
/*
 * Global variables
 */
#[no_mangle]
pub static mut loopPieCount: SDWORD = 0;
#[no_mangle]
pub static mut loopTileCount: SDWORD = 0;
#[no_mangle]
pub static mut loopPolyCount: SDWORD = 0;
#[no_mangle]
pub static mut loopStateChanges: SDWORD = 0;
/*
 * local variables
 */
static mut paused: BOOL = 0 as libc::c_int;
static mut video: BOOL = 0 as libc::c_int;
static mut bQuitVideo: BOOL = 0 as libc::c_int;
static mut clearCount: SDWORD = 0 as libc::c_int;
static mut pauseState: PAUSE_STATE =
    PAUSE_STATE{gameUpdatePause_audioPause_scriptPause_scrollPause_consolePause:
                    [0; 1],
                c2rust_padding: [0; 3],};
static mut numDroids: [UDWORD; 8] = [0; 8];
static mut numMissionDroids: [UDWORD; 8] = [0; 8];
static mut numTransporterDroids: [UDWORD; 8] = [0; 8];
static mut numCommandDroids: [UDWORD; 8] = [0; 8];
static mut numConstructorDroids: [UDWORD; 8] = [0; 8];
// flag to signal a quick exit from the game
static mut fastExit: BOOL = 0;
static mut videoMode: SDWORD = 0;
static mut g_iGlobalVol: SDWORD = 0;
#[no_mangle]
pub static mut loopMissionState: LOOP_MISSION_STATE = LMS_NORMAL;
// this is set by scrStartMission to say what type of new level is to be started
#[no_mangle]
pub static mut nextMissionType: SDWORD = LDS_NONE as libc::c_int;
//MISSION_NONE;
/* Force 3D display */
#[no_mangle]
pub static mut mcTime: UDWORD = 0;
#[no_mangle]
pub static mut display3D: BOOL = 1 as libc::c_int;
// signal a fast exit from the game
#[no_mangle]
pub unsafe extern "C" fn loopFastExit() { fastExit = 1 as libc::c_int; }
// this is set by scrStartMission to say what type of new level is to be started
/* The main game loop */
#[no_mangle]
pub unsafe extern "C" fn gameLoop() -> GAMECODE {
    let mut psCurr: *mut DROID = 0 as *mut DROID;
    let mut psNext: *mut DROID = 0 as *mut DROID;
    let mut psCBuilding: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psNBuilding: *mut STRUCTURE = 0 as *mut STRUCTURE;
    //	BOOL		bPlayerHasHQ = FALSE;
    let mut psCFeat: *mut FEATURE = 0 as *mut FEATURE;
    let mut psNFeat: *mut FEATURE = 0 as *mut FEATURE;
    let mut i: UDWORD = 0;
    let mut widgval: UDWORD = 0;
    let mut quitting: BOOL = 0 as libc::c_int;
    let mut intRetVal: INT_RETVAL = INT_NONE;
    let mut clearMode: CLEAR_MODE = CLEAR_OFF;
    //	DumpVRAM();	// use mouse to scroll through vram
    //	dumpimdpoints();
    //	heapIntegrityCheck(psDroidHeap);
    //JPS 24 feb???
    if fogStatus & 1 as libc::c_int as libc::c_uint != 0 {
        clearMode = CLEAR_FOG; //screen clear to fog colour D3D
        if loopMissionState as libc::c_uint ==
               LMS_SAVECONTINUE as libc::c_int as libc::c_uint {
            pie_SetFogStatus(0 as libc::c_int);
            clearMode = CLEAR_BLACK
        }
    } else {
        clearMode = CLEAR_BLACK
        //force to black 3DFX
    } //gameloopflip
    pie_ScreenFlip(clearMode);
    //JPS 24 feb???
    fastExit = 0 as libc::c_int; // Needs to be done outside the pause case.
    pie_GlobalRenderBegin();
    HandleClosingWindows();
    audio_Update();
    if paused == 0 {
        if scriptPaused() == 0 {
            /* Update the event system */
            if bInTutorial == 0 {
                eventProcessTriggers(gameTime.wrapping_div(100 as libc::c_int
                                                               as
                                                               libc::c_uint));
            } else {
                eventProcessTriggers(gameTime2.wrapping_div(100 as libc::c_int
                                                                as
                                                                libc::c_uint));
            }
        }
        /* Run the in game interface and see if it grabbed any mouse clicks */
        if rotActive == 0 && getWidgetsStatus() != 0 &&
               dragBox3D.status != 1 as libc::c_int as libc::c_uint &&
               wallDrag.status != 1 as libc::c_int as libc::c_uint {
            intRetVal = intRunWidgets()
        } else { intRetVal = INT_NONE }
        //don't process the object lists if paused or about to quit to the front end
        if !(gameUpdatePaused() != 0 ||
                 intRetVal as libc::c_uint ==
                     INT_QUIT as libc::c_int as libc::c_uint) {
            if dragBox3D.status != 1 as libc::c_int as libc::c_uint &&
                   wallDrag.status != 1 as libc::c_int as libc::c_uint {
                if intRetVal as libc::c_uint ==
                       INT_INTERCEPT as libc::c_int as libc::c_uint ||
                       radarOnScreen != 0 &&
                           CoordInRadar(mouseX(), mouseY()) != 0 &&
                           getHQExists(selectedPlayer) != 0 {
                    pie_SetMouse(IntImages,
                                 IMAGE_CURSOR_DEFAULT as libc::c_int as
                                     UWORD);
                    frameSetCursorFromRes(108 as libc::c_int as WORD);
                    //}
                    intRetVal = INT_INTERCEPT
                }
            }
            //if( (intRetVal != INT_FULLSCREENPAUSE) && (
					//	intRetVal != INT_INTELPAUSE) )
					//{
            //}
            //		intRetVal = INT_NONE;
            // we need two versions of the loop conditions, since PSX doesn't have
		// multiplayer stuff.
            // Don't update the game world if the design screen is up and single player game
		//if (((intRetVal != INT_FULLSCREENPAUSE) || bMultiPlayer) AND ((intRetVal !=
		//	INT_INTELPAUSE) || bMultiPlayer))
            //handles callbacks for positioning of DP's - like PSX
            process3DBuilding();
            // Update the base movement stuff
            moveUpdateBaseSpeed();
            // Update the visibility change stuff
            visUpdateLevel();
            // do the grid garbage collection
            gridGarbageCollect();
            //update the findpath system
            fpathUpdate();
            // update the cluster system
            clusterUpdate();
            // update the command droids
            cmdDroidUpdate();
            /* Update the AI for a player */
            i = 0 as libc::c_int as UDWORD;
            while i < 8 as libc::c_int as libc::c_uint {
                playerUpdate(i);
                i = i.wrapping_add(1)
            }
            if getDrivingStatus() != 0 { driveUpdate(); }
            //ajl. get the incoming netgame messages and process them.
            if bMultiPlayer != 0 {
                multiPlayerLoop();
                //		RecvMessage();
            }
            i = 0 as libc::c_int as UDWORD;
            while i < 8 as libc::c_int as libc::c_uint {
                //update the current power available for a player
                updatePlayerPower(i);
                //this is a check cos there is a problem with the power but not sure where!!
                powerCheck(1 as libc::c_int, i as UBYTE);
                //spread the power out...done in aiUpdateStructure now
				//spreadPower((UBYTE)i);
                //set the flag for each player
				//setPowerGenExists(FALSE, i);
                setHQExists(0 as libc::c_int, i);
                setSatUplinkExists(0 as libc::c_int, i);
                numCommandDroids[i as usize] = 0 as libc::c_int as UDWORD;
                numConstructorDroids[i as usize] = 0 as libc::c_int as UDWORD;
                numDroids[i as usize] = 0 as libc::c_int as UDWORD;
                numTransporterDroids[i as usize] = 0 as libc::c_int as UDWORD;
                psCurr = apsDroidLists[i as usize];
                while !psCurr.is_null() {
                    /* Copy the next pointer - not 100% sure if the droid could get destroyed
					   but this covers us anyway */
                    psNext = (*psCurr).psNext;
                    droidUpdate(psCurr);
                    // update the droid counts
                    numDroids[i as usize] =
                        numDroids[i as usize].wrapping_add(1);
                    match (*psCurr).droidType as libc::c_uint {
                        7 => {
                            numCommandDroids[i as usize] =
                                (numCommandDroids[i as usize] as
                                     libc::c_uint).wrapping_add(1 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uint)
                                    as UDWORD as UDWORD
                        }
                        3 | 10 => {
                            numConstructorDroids[i as usize] =
                                (numConstructorDroids[i as usize] as
                                     libc::c_uint).wrapping_add(1 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uint)
                                    as UDWORD as UDWORD
                        }
                        6 => {
                            if !(*psCurr).psGroup.is_null() {
                                numTransporterDroids[i as usize] =
                                    (numTransporterDroids[i as usize] as
                                         libc::c_uint).wrapping_add(((*(*psCurr).psGroup).refCount
                                                                         as
                                                                         libc::c_int
                                                                         -
                                                                         1 as
                                                                             libc::c_int)
                                                                        as
                                                                        libc::c_uint)
                                        as UDWORD as UDWORD
                            }
                        }
                        _ => { }
                    }
                    psCurr = psNext
                }
                numMissionDroids[i as usize] = 0 as libc::c_int as UDWORD;
                psCurr = mission.apsDroidLists[i as usize];
                while !psCurr.is_null() {
                    /* Copy the next pointer - not 100% sure if the droid could
					get destroyed but this covers us anyway */
                    psNext = (*psCurr).psNext;
                    missionDroidUpdate(psCurr);
                    numMissionDroids[i as usize] =
                        numMissionDroids[i as usize].wrapping_add(1);
                    match (*psCurr).droidType as libc::c_uint {
                        7 => {
                            numCommandDroids[i as usize] =
                                (numCommandDroids[i as usize] as
                                     libc::c_uint).wrapping_add(1 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uint)
                                    as UDWORD as UDWORD
                        }
                        3 | 10 => {
                            numConstructorDroids[i as usize] =
                                (numConstructorDroids[i as usize] as
                                     libc::c_uint).wrapping_add(1 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uint)
                                    as UDWORD as UDWORD
                        }
                        6 => {
                            if !(*psCurr).psGroup.is_null() {
                                numTransporterDroids[i as usize] =
                                    (numTransporterDroids[i as usize] as
                                         libc::c_uint).wrapping_add(((*(*psCurr).psGroup).refCount
                                                                         as
                                                                         libc::c_int
                                                                         -
                                                                         1 as
                                                                             libc::c_int)
                                                                        as
                                                                        libc::c_uint)
                                        as UDWORD as UDWORD
                            }
                        }
                        _ => { }
                    }
                    psCurr = psNext
                }
                psCurr = apsLimboDroids[i as usize];
                while !psCurr.is_null() {
                    /* Copy the next pointer - not 100% sure if the droid could
					get destroyed but this covers us anyway */
                    psNext = (*psCurr).psNext;
                    // count the type of units
                    match (*psCurr).droidType as libc::c_uint {
                        7 => {
                            numCommandDroids[i as usize] =
                                (numCommandDroids[i as usize] as
                                     libc::c_uint).wrapping_add(1 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uint)
                                    as UDWORD as UDWORD
                        }
                        3 | 10 => {
                            numConstructorDroids[i as usize] =
                                (numConstructorDroids[i as usize] as
                                     libc::c_uint).wrapping_add(1 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uint)
                                    as UDWORD as UDWORD
                        }
                        _ => { }
                    }
                    psCurr = psNext
                }
                /*set this up AFTER droidUpdate so that if trying to building a
                new one, we know whether one exists already*/
                setLasSatExists(0 as libc::c_int, i);
                psCBuilding = apsStructLists[i as usize];
                while !psCBuilding.is_null() {
                    /* Copy the next pointer - not 100% sure if the structure could get destroyed
					   but this covers us anyway */
                    psNBuilding = (*psCBuilding).psNext;
                    structureUpdate(psCBuilding);
                    //set animation flag
					/*if (psCBuilding->pStructureType->type == REF_POWER_GEN AND
						psCBuilding->status == SS_BUILT)
					{
						setPowerGenExists(TRUE, i);
					}*/
                    if (*(*psCBuilding).pStructureType).type_0 ==
                           REF_HQ as libc::c_int as libc::c_uint &&
                           (*psCBuilding).status as libc::c_int ==
                               SS_BUILT as libc::c_int {
                        setHQExists(1 as libc::c_int, i);
                    }
                    if (*(*psCBuilding).pStructureType).type_0 ==
                           REF_SAT_UPLINK as libc::c_int as libc::c_uint &&
                           (*psCBuilding).status as libc::c_int ==
                               SS_BUILT as libc::c_int {
                        setSatUplinkExists(1 as libc::c_int, i);
                    }
                    //don't wait for the Las Sat to be built - can't build another if one is partially built
                    if (*asWeaponStats.offset((*psCBuilding).asWeaps[0 as
                                                                         libc::c_int
                                                                         as
                                                                         usize].nStat
                                                  as isize)).weaponSubClass as
                           libc::c_uint ==
                           WSC_LAS_SAT as libc::c_int as libc::c_uint {
                        setLasSatExists(1 as libc::c_int, i);
                    }
                    psCBuilding = psNBuilding
                }
                psCBuilding = mission.apsStructLists[i as usize];
                while !psCBuilding.is_null() {
                    /* Copy the next pointer - not 100% sure if the structure could get destroyed
					   but this covers us anyway It shouldn't do since its not even on the map!*/
                    psNBuilding = (*psCBuilding).psNext;
                    missionStructureUpdate(psCBuilding);
                    if (*(*psCBuilding).pStructureType).type_0 ==
                           REF_HQ as libc::c_int as libc::c_uint &&
                           (*psCBuilding).status as libc::c_int ==
                               SS_BUILT as libc::c_int {
                        setHQExists(1 as libc::c_int, i);
                    }
                    if (*(*psCBuilding).pStructureType).type_0 ==
                           REF_SAT_UPLINK as libc::c_int as libc::c_uint &&
                           (*psCBuilding).status as libc::c_int ==
                               SS_BUILT as libc::c_int {
                        setSatUplinkExists(1 as libc::c_int, i);
                    }
                    //don't wait for the Las Sat to be built - can't build another if one is partially built
                    if (*asWeaponStats.offset((*psCBuilding).asWeaps[0 as
                                                                         libc::c_int
                                                                         as
                                                                         usize].nStat
                                                  as isize)).weaponSubClass as
                           libc::c_uint ==
                           WSC_LAS_SAT as libc::c_int as libc::c_uint {
                        setLasSatExists(1 as libc::c_int, i);
                    }
                    psCBuilding = psNBuilding
                }
                //this is a check cos there is a problem with the power but not sure where!!
                powerCheck(0 as libc::c_int, i as UBYTE);
                i = i.wrapping_add(1)
            }
            pwrcUpdate();
            missionTimerUpdate();
            proj_UpdateAll();
            psCFeat = apsFeatureLists[0 as libc::c_int as usize];
            while !psCFeat.is_null() {
                psNFeat = (*psCFeat).psNext;
                featureUpdate(psCFeat);
                psCFeat = psNFeat
            }
            /* Ensure smoke drifts up! */
//			raiseSmoke();
//			updateGravitons();
            /* update animations */
            animObj_Update();
            /* Raise and increase frames of explosions */
//			updateExplosions();
			/* Update all the temporary world effects */
//			processEffects();
            //			flushConsoleMessages();
//			clustDisplay();
            //}
		// Don't update the game world if the design screen is up and single player game
		//if (((intRetVal != INT_FULLSCREENPAUSE ) || bMultiPlayer) AND ((intRetVal !=
		//	INT_INTELNOSCROLL) || bMultiPlayer))
		//{
			//not any more!
			//need to be able to scroll and have radar still in Intelligence Screen -
			//but only if 3D View is not up
			/*if(!getWarCamStatus())
			{
				scroll();
			}*/
		//}
		// Don't update the game world if the design screen is up and single player game
		//if ((intRetVal != INT_FULLSCREENPAUSE ) || bMultiPlayer)
		//{
//			DBP1(("Radar update \n"));
//			/* Make radar line sweep and colour cycle */
//			updateRadar();
		//}
            // Don't update the game world if the design screen is up and single player game
		//if ((intRetVal != INT_FULLSCREENPAUSE AND intRetVal !=
		//	INT_INTELPAUSE) || bMultiPlayer)
		//{
            objmemUpdate();
        }
        if consolePaused() == 0 {
            /* Process all the console messages */
            updateConsoleMessages();
        }
        if scrollPaused() == 0 {
            if getWarCamStatus() == 0 {
                //this could set scrollPause?
                if dragBox3D.status != 1 as libc::c_int as libc::c_uint &&
                       intMode as libc::c_uint !=
                           INT_INGAMEOP as libc::c_int as libc::c_uint {
                    scroll();
                }
            }
        }
    } else {
        //paused
        intRetVal = INT_NONE;
        if video != 0 {
            bQuitVideo =
                (seq_UpdateFullScreenVideo(0 as *mut CLEAR_MODE) == 0) as
                    libc::c_int
        }
        if dragBox3D.status != 1 as libc::c_int as libc::c_uint { scroll(); }
        if InGameOpUp != 0 {
            //Added to prevent busy loop, and get CPU time back when paused!
            // ingame options menu up, run it!
            intRunInGameOptions();
            //			processFrontendSnap(FALSE);
            widgval = widgRunScreen(psWScreen);
            intProcessInGameOptions(widgval);
            if widgval ==
                   (10500 as libc::c_int + 2 as libc::c_int) as libc::c_uint {
                if gamePaused() != 0 { kf_TogglePauseMode(); }
                intRetVal = INT_QUIT
            }
        }
        if bLoadSaveUp != 0 {
            if runLoadSave(1 as libc::c_int) != 0 {
                // check for file name.
                if strlen(sRequestResult.as_mut_ptr()) != 0 {
                    debug(LOG_NEVER,
                          b"Returned %s\x00" as *const u8 as
                              *const libc::c_char,
                          sRequestResult.as_mut_ptr());
                    if bRequestLoad != 0 {
                        loopMissionState = LMS_LOADGAME;
                        strcpy(saveGameName.as_mut_ptr(),
                               sRequestResult.as_mut_ptr());
                    } else if saveInMissionRes() != 0 {
                        if saveGame(sRequestResult.as_mut_ptr(),
                                    GTYPE_SAVE_START as libc::c_int) != 0 {
                            addConsoleMessage(strresGetString(psStringRes,
                                                              STR_GAME_SAVED
                                                                  as
                                                                  libc::c_int
                                                                  as UDWORD),
                                              LEFT_JUSTIFY);
                        } else {
                            if 0 as libc::c_int != 0 {
                            } else {
                                debug(LOG_ERROR,
                                      b"Mission Results: saveGame Failed\x00"
                                          as *const u8 as
                                          *const libc::c_char);
                            };
                            if 0 as libc::c_int != 0 {
                            } else {
                                debug(LOG_ERROR,
                                      b"Assert in Warzone: %s:%d : %s (%s)\x00"
                                          as *const u8 as *const libc::c_char,
                                      b"loop.c\x00" as *const u8 as
                                          *const libc::c_char,
                                      579 as libc::c_int,
                                      (*::std::mem::transmute::<&[u8; 9],
                                                                &[libc::c_char; 9]>(b"gameLoop\x00")).as_ptr(),
                                      b"FALSE\x00" as *const u8 as
                                          *const libc::c_char);
                            };
                            deleteSaveGame(sRequestResult.as_mut_ptr());
                        }
                    } else if bMultiPlayer != 0 || saveMidMission() != 0 {
                        if saveGame(sRequestResult.as_mut_ptr(),
                                    GTYPE_SAVE_MIDMISSION as libc::c_int) != 0
                           {
                            //mid mission from [esc] menu
                            addConsoleMessage(strresGetString(psStringRes,
                                                              STR_GAME_SAVED
                                                                  as
                                                                  libc::c_int
                                                                  as UDWORD),
                                              LEFT_JUSTIFY);
                        } else {
                            if 0 as libc::c_int != 0 {
                            } else {
                                debug(LOG_ERROR,
                                      b"Mid Mission: saveGame Failed\x00" as
                                          *const u8 as *const libc::c_char);
                            };
                            if 0 as libc::c_int != 0 {
                            } else {
                                debug(LOG_ERROR,
                                      b"Assert in Warzone: %s:%d : %s (%s)\x00"
                                          as *const u8 as *const libc::c_char,
                                      b"loop.c\x00" as *const u8 as
                                          *const libc::c_char,
                                      591 as libc::c_int,
                                      (*::std::mem::transmute::<&[u8; 9],
                                                                &[libc::c_char; 9]>(b"gameLoop\x00")).as_ptr(),
                                      b"FALSE\x00" as *const u8 as
                                          *const libc::c_char);
                            };
                            deleteSaveGame(sRequestResult.as_mut_ptr());
                        }
                    } else {
                        if 0 as libc::c_int != 0 {
                        } else {
                            debug(LOG_ERROR,
                                  b"Attempt to save game with incorrect load/save mode\x00"
                                      as *const u8 as *const libc::c_char);
                        };
                        if 0 as libc::c_int != 0 {
                        } else {
                            debug(LOG_ERROR,
                                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                                      *const u8 as *const libc::c_char,
                                  b"loop.c\x00" as *const u8 as
                                      *const libc::c_char, 597 as libc::c_int,
                                  (*::std::mem::transmute::<&[u8; 9],
                                                            &[libc::c_char; 9]>(b"gameLoop\x00")).as_ptr(),
                                  b"FALSE\x00" as *const u8 as
                                      *const libc::c_char);
                        };
                    }
                }
            }
        }
        SDL_Delay(20 as libc::c_int as Uint32);
    }
    /* Check for quit */
//	if (keyPressed(KEY_ESC) || intRetVal == INT_QUIT)
    if intRetVal as libc::c_uint == INT_QUIT as libc::c_int as libc::c_uint ||
           fastExit != 0 {
        if video == 0 {
            //quitting from the game to the front end
			//so get a new backdrop
            quitting = 1 as libc::c_int;
            pie_LoadBackDrop(SCREEN_RANDOMBDROP, 0 as libc::c_int);
        } else {
            //if in video mode esc kill video
            bQuitVideo = 1 as libc::c_int
        }
    }
    if video == 0 {
        //if (!quitting && intRetVal != INT_FULLSCREENPAUSE)
        if quitting == 0 {
            if gameUpdatePaused() == 0 {
                if display3D != 0 {
                    /*bPlayerHasHQ=FALSE;
					for (psStructure = apsStructLists[selectedPlayer]; psStructure AND
						!bPlayerHasHQ; psStructure = psStructure->psNext)
					{
						if (psStructure->pStructureType->type == REF_HQ)
						{
							bPlayerHasHQ = TRUE;
						}
					}
					*/
				  //	bPlayerHasHQ = radarCheckForHQ(selectedPlayer);
                    if dragBox3D.status != 1 as libc::c_int as libc::c_uint &&
                           wallDrag.status != 1 as libc::c_int as libc::c_uint
                       {
                        ProcessRadarInput();
                    }
                    processInput();
                    //no key clicks or in Intelligence Screen
	//				if (intRetVal == INT_INTELPAUSE)
                    if intRetVal as libc::c_uint ==
                           INT_NONE as libc::c_int as libc::c_uint &&
                           InGameOpUp == 0 {
                        // OR intRetVal == INT_INTELPAUSE)
                        //quitting = processInput();
						//don't want to handle the mouse input here when in intelligence screen
						//if (intRetVal != INT_INTELPAUSE)
						//{
                        processMouseClickInput();
                    }
                    downloadAtStartOfFrame();
                    displayWorld();
                } else {
                    //no key clicks or in Intelligence Screen
                    (intRetVal as libc::c_uint) ==
                        INT_NONE as libc::c_int as libc::c_uint;
                }
            }
            //			}
            //#endif
            pie_SetDepthBufferStatus(DEPTH_CMP_ALWAYS_WRT_ON);
            pie_SetFogStatus(0 as libc::c_int);
            if bMultiPlayer != 0 {
                /* Display the in game interface */
//			if(widgetsOn OR forceWidgetsOn)
//			{
                //				if((game.type == DMATCH) && !MultiMenuUp)
//				{
//					intDisplayMiniMultiMenu();
//				}
                if bDisplayMultiJoiningStatus != 0 {
                    intDisplayMultiJoiningStatus(bDisplayMultiJoiningStatus);
                    setWidgetsStatus(0 as libc::c_int);
                }
            }
            if getWidgetsStatus() != 0 { intDisplayWidgets(); }
            pie_SetDepthBufferStatus(DEPTH_CMP_LEQ_WRT_ON);
            pie_SetFogStatus(1 as libc::c_int);
        }
        /*else if (!quitting)
		{
			// Display the in game interface
	//#ifdef PSX
	//		DrawMousePointer(mouseX(),mouseY());	// add the mouse pointer as a primative
	//#endif

			DBP1(("loop: Display widgets\n"));
			if(widgetsOn)
			{
				pie_SetDepthBufferStatus(DEPTH_CMP_ALWAYS_WRT_ON);
				pie_SetFogStatus(FALSE);

				intDisplayWidgets();

				pie_SetDepthBufferStatus(DEPTH_CMP_LEQ_WRT_ON);
				pie_SetFogStatus(TRUE);
			}
		}*/
    }
    /* Check for toggling video playbackmode */
    if bQuitVideo != 0 {
        if !(video == 0) {
            seq_StopFullScreenVideo();
            bQuitVideo = 0 as libc::c_int
        }
    }
    /* Check for pause */
//	if (!video)
//	{
//		if (keyPressed(KEY_F12) && !paused)
//		{
//			paused = TRUE;
//			gameTimeStop();
	//		addGameMessage("Game Status : PAUSED",1000, TRUE);
//			addConsoleMessage("Game has been paused",DEFAULT_JUSTIFY);
//		}
//		else if (keyPressed(KEY_F12) && paused)
//		{
//			paused = FALSE;
//			gameTimeStart();
//		}
//	}		// ALL THIS GUBBINS DONE IN A PROPER KEYMAPPING NOW (A DEBUG ONE THOUGH!).
    pie_GetResetCounts(&mut loopPieCount, &mut loopTileCount,
                       &mut loopPolyCount,
                       &mut loopStateChanges); //screen clear to fog colour 3DFX
    if fogStatus & 1 as libc::c_int as libc::c_uint != 0 {
        pie_GlobalRenderEnd(0 as
                                libc::c_int); //screen clear to fog colour D3D
        clearMode = CLEAR_FOG; //force to black 3DFX
        if loopMissionState as libc::c_uint ==
               LMS_SAVECONTINUE as libc::c_int as libc::c_uint {
            pie_SetFogStatus(0 as libc::c_int);
            clearMode = CLEAR_BLACK
        }
    } else {
        pie_GlobalRenderEnd(1 as libc::c_int);
        clearMode = CLEAR_BLACK
        //force to black 3DFX
    }
    if quitting == 0 && fastExit == 0 {
        //JPS 24 feb???		pie_ScreenFlip(clearMode);//gameloopflip
        /* Check for toggling display mode */
        if (keyDown(KEY_LALT) != 0 || keyDown(KEY_RALT) != 0) &&
               keyPressed(KEY_RETURN) != 0 {
            screenToggleMode();
            dispModeChange();
        }
    }
    /*	if(missionComplete)
	{
		if (gameTime>(mcTime+MISSION_COMPLETE_DELAY))
		{
			quitting = TRUE;
		}
	}*/
    // deal with the mission state
    match loopMissionState as libc::c_uint {
        5 => {
            missionDestroyObjects();
            setScriptPause(1 as libc::c_int);
            loopMissionState = LMS_SETUPMISSION
        }
        0 => { }
        1 => {
            setScriptPause(0 as libc::c_int);
            if setUpMission(nextMissionType as UDWORD) == 0 {
                return GAMECODE_QUITGAME
            }
        }
        2 => {
            // just wait for this to be changed when the new mission starts
            clearMode = CLEAR_BLACK
        }
        3 => {
            //nextMissionType = MISSION_NONE;
            nextMissionType = LDS_NONE as libc::c_int; //gameloopflip
            return GAMECODE_NEWLEVEL
        }
        4 => { return GAMECODE_LOADGAME }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"unknown loopMissionState\x00" as *const u8 as
                          *const libc::c_char); //gameloopflip
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"loop.c\x00" as *const u8 as *const libc::c_char,
                      852 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 9],
                                                &[libc::c_char; 9]>(b"gameLoop\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
        }
    }
    if fastExit != 0 {
        pie_SetFogStatus(0 as libc::c_int);
        pie_ScreenFlip(CLEAR_BLACK);
        pie_ScreenFlip(CLEAR_BLACK);
        /* Check for toggling display mode */
        if (keyDown(KEY_LALT) != 0 || keyDown(KEY_RALT) != 0) &&
               keyPressed(KEY_RETURN) != 0 {
            screenToggleMode(); //gameloopflip
            dispModeChange(); //gameloopflip
        }
        return GAMECODE_FASTEXIT
    } else {
        if quitting != 0 {
            pie_SetFogStatus(0 as libc::c_int);
            pie_ScreenFlip(CLEAR_BLACK);
            pie_ScreenFlip(CLEAR_BLACK);
            /* Check for toggling display mode */
            if (keyDown(KEY_LALT) != 0 || keyDown(KEY_RALT) != 0) &&
                   keyPressed(KEY_RETURN) != 0 {
                screenToggleMode();
                dispModeChange();
            }
            return GAMECODE_QUITGAME
        } else {
            if video != 0 { audio_StopAll(); return GAMECODE_PLAYVIDEO }
        }
    }
    /*
	if( (intMode == INT_NORMAL) AND (forceWidgetsOn == TRUE) )
	{
		forceWidgetsOn = FALSE;
	}
	*/
    return GAMECODE_CONTINUE;
}
/* The video playback loop */
#[no_mangle]
pub unsafe extern "C" fn videoLoop() -> GAMECODE {
    let mut bVolKilled: BOOL = 0 as libc::c_int;
    let mut bClear: CLEAR_MODE = CLEAR_OFF;
    static mut bActiveBackDrop: BOOL = 0 as libc::c_int;
    pie_GlobalRenderBegin();
    bClear = CLEAR_OFF;
    if video != 0 {
        bQuitVideo =
            (seq_UpdateFullScreenVideo(&mut bClear) == 0) as libc::c_int
    }
    if (keyPressed(KEY_ESC) != 0 || bQuitVideo != 0) && seq_AnySeqLeft() == 0
       {
        /* zero volume before video quit - restore later */
        g_iGlobalVol = mixer_GetWavVolume();
        mixer_SetWavVolume(0 as libc::c_int);
        bVolKilled = 1 as libc::c_int
    }
    //toggling display mode disabled in video mode
	// Check for quit
    if keyPressed(KEY_ESC) != 0 {
        // || bQuitVideo)
        seq_StopFullScreenVideo();
        bQuitVideo = 0 as libc::c_int;
        // remove the intelligence screen if necessary
		/*if (messageIsImmediate())
		{
			intResetScreen(TRUE);
		}*/
        if messageIsImmediate() != 0 {
            intResetScreen(1 as libc::c_int);
            setMessageImmediate(0 as libc::c_int);
        }
        if getScriptWinLoseVideo() == 0 {
            eventFireCallbackTrigger(CALL_VIDEO_QUIT as libc::c_int as
                                         TRIGGER_TYPE);
        } else {
            displayGameOver((getScriptWinLoseVideo() as libc::c_int ==
                                 1 as libc::c_int) as libc::c_int);
        }
        pie_ScreenFlip(CLEAR_BLACK);
        if bActiveBackDrop != 0 { screen_RestartBackDrop(); }
    }
    // remove the intelligence screen if necessary
    //Script callback for video over
    //don't do the callback if we're playing the win/lose video
    //		clearCount = 0;
    // videoloopflip extra mar10
    //if the video finished...
    if bQuitVideo != 0 {
        seq_StopFullScreenVideo();
        bQuitVideo = 0 as libc::c_int;
        //set the next video off - if any
        if seq_AnySeqLeft() != 0 {
            pie_ScreenFlip(CLEAR_BLACK); // videoloopflip extra mar10
            if bActiveBackDrop != 0 { screen_RestartBackDrop(); }
            //bClear = CLEAR_BLACK;
            seq_StartNextFullScreenVideo();
        } else {
            // remove the intelligence screen if necessary
            if messageIsImmediate() != 0 {
                intResetScreen(1 as libc::c_int);
                setMessageImmediate(0 as libc::c_int);
            }
            // remove the intelligence screen if necessary
			/*if (messageIsImmediate())
			{
				intResetScreen(TRUE);
			}*/
            if getScriptWinLoseVideo() == 0 {
                eventFireCallbackTrigger(CALL_VIDEO_QUIT as libc::c_int as
                                             TRIGGER_TYPE);
            } else {
                displayGameOver((getScriptWinLoseVideo() as libc::c_int ==
                                     1 as libc::c_int) as libc::c_int);
            }
            pie_ScreenFlip(CLEAR_BLACK);
            if bActiveBackDrop != 0 { screen_RestartBackDrop(); }
        }
    }
    //don't do the callback if we're playing the win/lose video
    //		    clearCount = 0;
    // videoloopflip extra mar10
    pie_GlobalRenderEnd(1 as libc::c_int); //force to black
    if clearCount < 1 as libc::c_int {
        bClear = CLEAR_BLACK; // videoloopflip
        if !(screen_GetBackDrop() as *mut libc::c_void).is_null() {
            bActiveBackDrop = 1 as libc::c_int;
            screen_StopBackDrop();
        } else { bActiveBackDrop = 0 as libc::c_int; screen_StopBackDrop(); }
    } else if clearCount < 2 as libc::c_int { bClear = CLEAR_BLACK }
    clearCount += 1;
    pie_ScreenFlip(bClear);
    /* restore volume after video quit */
    if bVolKilled == 1 as libc::c_int { mixer_SetWavVolume(g_iGlobalVol); }
    return GAMECODE_CONTINUE;
}
#[no_mangle]
pub unsafe extern "C" fn loop_SetVideoPlaybackMode() {
    videoMode += 1 as libc::c_int;
    paused = 1 as libc::c_int;
    video = 1 as libc::c_int;
    clearCount = 0 as libc::c_int;
    gameTimeStop();
    pie_SetFogStatus(0 as libc::c_int);
    audio_StopAll();
}
#[no_mangle]
pub unsafe extern "C" fn loop_ClearVideoPlaybackMode() {
    videoMode -= 1 as libc::c_int;
    paused = 0 as libc::c_int;
    video = 0 as libc::c_int;
    gameTimeStart();
    //	pie_SetFogStatus(TRUE);
    cdAudio_Resume();
    if videoMode == 0 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"loop_ClearVideoPlaybackMode: out of sync.\x00" as *const u8 as
                  *const libc::c_char);
    };
    if videoMode == 0 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"loop.c\x00" as *const u8 as *const libc::c_char,
              1087 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 28],
                                        &[libc::c_char; 28]>(b"loop_ClearVideoPlaybackMode\x00")).as_ptr(),
              b"videoMode == 0\x00" as *const u8 as *const libc::c_char);
    };
}
#[no_mangle]
pub unsafe extern "C" fn loop_GetVideoMode() -> SDWORD { return videoMode; }
#[no_mangle]
pub unsafe extern "C" fn loop_GetVideoStatus() -> BOOL { return video; }
#[no_mangle]
pub unsafe extern "C" fn gamePaused() -> BOOL { return paused; }
#[no_mangle]
pub unsafe extern "C" fn setGamePauseStatus(mut val: BOOL) { paused = val; }
#[no_mangle]
pub unsafe extern "C" fn gameUpdatePaused() -> BOOL {
    return pauseState.gameUpdatePause() as BOOL;
}
#[no_mangle]
pub unsafe extern "C" fn audioPaused() -> BOOL {
    return pauseState.audioPause() as BOOL;
}
#[no_mangle]
pub unsafe extern "C" fn scriptPaused() -> BOOL {
    return pauseState.scriptPause() as BOOL;
}
#[no_mangle]
pub unsafe extern "C" fn scrollPaused() -> BOOL {
    return pauseState.scrollPause() as BOOL;
}
#[no_mangle]
pub unsafe extern "C" fn consolePaused() -> BOOL {
    return pauseState.consolePause() as BOOL;
}
#[no_mangle]
pub unsafe extern "C" fn setGameUpdatePause(mut state: BOOL) {
    pauseState.set_gameUpdatePause(state as libc::c_uint);
    if state != 0 {
        screen_RestartBackDrop();
    } else { screen_StopBackDrop(); };
}
#[no_mangle]
pub unsafe extern "C" fn setAudioPause(mut state: BOOL) {
    pauseState.set_audioPause(state as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn setScriptPause(mut state: BOOL) {
    pauseState.set_scriptPause(state as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn setScrollPause(mut state: BOOL) {
    pauseState.set_scrollPause(state as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn setConsolePause(mut state: BOOL) {
    pauseState.set_consolePause(state as libc::c_uint);
}
//set all the pause states to the state value
//set all the pause states to the state value
#[no_mangle]
pub unsafe extern "C" fn setAllPauseStates(mut state: BOOL) {
    setGameUpdatePause(state);
    setAudioPause(state);
    setScriptPause(state);
    setScrollPause(state);
    setConsolePause(state);
}
// Number of units in the current list.
#[no_mangle]
pub unsafe extern "C" fn getNumDroids(mut player: UDWORD) -> UDWORD {
    return numDroids[player as usize];
}
// Number of units on transporters.
#[no_mangle]
pub unsafe extern "C" fn getNumTransporterDroids(mut player: UDWORD)
 -> UDWORD {
    return numTransporterDroids[player as usize];
}
// Number of units in the mission list.
#[no_mangle]
pub unsafe extern "C" fn getNumMissionDroids(mut player: UDWORD) -> UDWORD {
    return numMissionDroids[player as usize];
}
#[no_mangle]
pub unsafe extern "C" fn getNumCommandDroids(mut player: UDWORD) -> UDWORD {
    return numCommandDroids[player as usize];
}
#[no_mangle]
pub unsafe extern "C" fn getNumConstructorDroids(mut player: UDWORD)
 -> UDWORD {
    return numConstructorDroids[player as usize];
}
// increase the droid counts - used by update factory to keep the counts in sync
// increase the droid counts - used by update factory to keep the counts in sync
#[no_mangle]
pub unsafe extern "C" fn incNumDroids(mut player: UDWORD) {
    numDroids[player as usize] =
        (numDroids[player as usize] as
             libc::c_uint).wrapping_add(1 as libc::c_int as libc::c_uint) as
            UDWORD as UDWORD;
}
#[no_mangle]
pub unsafe extern "C" fn incNumCommandDroids(mut player: UDWORD) {
    numCommandDroids[player as usize] =
        (numCommandDroids[player as usize] as
             libc::c_uint).wrapping_add(1 as libc::c_int as libc::c_uint) as
            UDWORD as UDWORD;
}
#[no_mangle]
pub unsafe extern "C" fn incNumConstructorDroids(mut player: UDWORD) {
    numConstructorDroids[player as usize] =
        (numConstructorDroids[player as usize] as
             libc::c_uint).wrapping_add(1 as libc::c_int as libc::c_uint) as
            UDWORD as UDWORD;
}
