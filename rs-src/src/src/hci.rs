use ::libc;
extern "C" {
    pub type _formation;
    pub type _gateway;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn abort() -> !;
    #[no_mangle]
    fn abs(_: libc::c_int) -> libc::c_int;
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
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_uint)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_uint)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memMallocRelease(Size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn memFreeRelease(pMemToFree: *mut libc::c_void);
    /* This returns true if the key is currently depressed */
    #[no_mangle]
    fn keyDown(code: KEY_CODE) -> BOOL;
    /* Get the string from an ID number */
    #[no_mangle]
    fn strresGetString(psRes: *mut STR_RES, id: UDWORD) -> *mut STRING;
    /* Create an empty widget screen */
    #[no_mangle]
    fn widgCreateScreen(ppsScreen: *mut *mut W_SCREEN) -> BOOL;
    /* Release a screen and all its associated data */
    #[no_mangle]
    fn widgReleaseScreen(psScreen: *mut W_SCREEN);
    /* Set the tool tip font for a screen */
    #[no_mangle]
    fn widgSetTipFont(psScreen: *mut W_SCREEN, FontID: libc::c_int);
    /* Add a form to the widget screen */
    #[no_mangle]
    fn widgAddForm(psScreen: *mut W_SCREEN, psInit: *mut W_FORMINIT) -> BOOL;
    /* Add a label to the widget screen */
    #[no_mangle]
    fn widgAddLabel(psScreen: *mut W_SCREEN, psInit: *mut W_LABINIT) -> BOOL;
    /* Add a button to a form */
    #[no_mangle]
    fn widgAddButton(psScreen: *mut W_SCREEN, psInit: *mut W_BUTINIT) -> BOOL;
    /* Add a bar graph to a form */
    #[no_mangle]
    fn widgAddBarGraph(psScreen: *mut W_SCREEN, psInit: *mut W_BARINIT)
     -> BOOL;
    /* Delete a widget from the screen */
    #[no_mangle]
    fn widgDelete(psScreen: *mut W_SCREEN, id: UDWORD);
    /* Hide a widget */
    #[no_mangle]
    fn widgHide(psScreen: *mut W_SCREEN, id: UDWORD);
    /* Reveal a widget */
    #[no_mangle]
    fn widgReveal(psScreen: *mut W_SCREEN, id: UDWORD);
    /* Set the current tabs for a tab form */
    #[no_mangle]
    fn widgSetTabs(psScreen: *mut W_SCREEN, id: UDWORD, major: UWORD,
                   minor: UWORD);
    /* Get the current tabs for a tab form */
    #[no_mangle]
    fn widgGetTabs(psScreen: *mut W_SCREEN, id: UDWORD, pMajor: *mut UWORD,
                   pMinor: *mut UWORD);
    /* Return the ID of the widget the mouse was over this frame */
    #[no_mangle]
    fn widgGetMouseOver(psScreen: *mut W_SCREEN) -> UDWORD;
    /* Return the user data for a widget */
    #[no_mangle]
    fn widgGetUserData(psScreen: *mut W_SCREEN, id: UDWORD)
     -> *mut libc::c_void;
    /* Set the user data for a widget */
    #[no_mangle]
    fn widgSetUserData(psScreen: *mut W_SCREEN, id: UDWORD,
                       UserData: *mut libc::c_void);
    /* Get widget structure */
    /* Find a widget in a screen from its ID number */
    #[no_mangle]
    fn widgGetFromID(psScreen: *mut W_SCREEN, id: UDWORD) -> *mut WIDGET;
    /* Set a colour on a form */
    #[no_mangle]
    fn widgSetColour(psScreen: *mut W_SCREEN, id: UDWORD, colour: UDWORD,
                     red: UBYTE, green: UBYTE, blue: UBYTE);
    // Set the global toop tip text colour.
    #[no_mangle]
    fn widgSetTipColour(psScreen: *mut W_SCREEN, red: UBYTE, green: UBYTE,
                        blue: UBYTE);
    #[no_mangle]
    fn setWidgetsStatus(var: BOOL);
    #[no_mangle]
    fn getWidgetsStatus() -> BOOL;
    #[no_mangle]
    fn widgDisplayScreen(psScreen: *mut W_SCREEN);
    #[no_mangle]
    fn widgRunScreen(psScreen: *mut W_SCREEN) -> UDWORD;
    #[no_mangle]
    fn widgEndScreen(psScreen: *mut W_SCREEN);
    #[no_mangle]
    fn widgStartScreen(psScreen: *mut W_SCREEN);
    #[no_mangle]
    fn widgGetButtonKey(psScreen: *mut W_SCREEN) -> UDWORD;
    #[no_mangle]
    fn widgSetButtonState(psScreen: *mut W_SCREEN, id: UDWORD, state: UDWORD);
    #[no_mangle]
    fn WidgSetAudio(Callback: WIDGET_AUDIOCALLBACK, HilightID: SWORD,
                    ClickedID: SWORD);
    #[no_mangle]
    fn pal_GetNearestColour(r: uint8, g: uint8, b: uint8) -> uint8;
    #[no_mangle]
    fn pie_GetVideoBufferWidth() -> UDWORD;
    #[no_mangle]
    fn pie_GetVideoBufferHeight() -> UDWORD;
    #[no_mangle]
    fn audio_PlayTrack(iTrack: libc::c_int);
    #[no_mangle]
    static mut apsDroidTemplates: [*mut DROID_TEMPLATE; 8];
    #[no_mangle]
    fn constructorPoints(psStats: *mut CONSTRUCT_STATS, player: UBYTE)
     -> UDWORD;
    #[no_mangle]
    fn getName(pNameID: *mut STRING) -> *mut STRING;
    //stores which player the production list has been set up for
    #[no_mangle]
    static mut productionPlayer: SBYTE;
    //holder for all StructureStats
    #[no_mangle]
    static mut asStructureStats: *mut STRUCTURE_STATS;
    #[no_mangle]
    static mut numStructureStats: UDWORD;
    #[no_mangle]
    fn IsPlayerStructureLimitReached(PlayerNumber: UDWORD) -> BOOL;
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
    //fills the list with Structures that can be built
    #[no_mangle]
    fn fillStructureList(ppList: *mut *mut STRUCTURE_STATS,
                         selectedPlayer_0: UDWORD, limit: UDWORD) -> UDWORD;
    /* get demolish stat */
    #[no_mangle]
    fn structGetDemolishStat() -> *mut STRUCTURE_STATS;
    /*this is called whenever a structure has finished building*/
    #[no_mangle]
    fn buildingComplete(psBuilding: *mut STRUCTURE);
    /*Looks through the players list of structures to see if a HQ exists - will look
through the list of structures at Home Base when on an offWorld mission map*/
    #[no_mangle]
    fn radarCheckForHQ(player: UDWORD) -> BOOL;
    // Set the command droid that factory production should go to
//struct _command_droid;
    #[no_mangle]
    fn assignFactoryCommandDroid(psStruct: *mut STRUCTURE,
                                 psCommander: *mut _droid);
    // Is a structure a factory of somekind?
    #[no_mangle]
    fn StructIsFactory(Struct: *mut STRUCTURE) -> BOOL;
    // Find a factories corresonding delivery point.
    #[no_mangle]
    fn FindFactoryDelivery(Struct: *mut STRUCTURE) -> *mut FLAG_POSITION;
    /*this is called when a factory produces a droid. The Template returned is the next 
one to build - if any*/
    #[no_mangle]
    fn factoryProdUpdate(psStructure: *mut STRUCTURE,
                         psTemplate: *mut DROID_TEMPLATE)
     -> *mut DROID_TEMPLATE;
    //increment the production run for this type
    #[no_mangle]
    fn factoryProdAdjust(psStructure: *mut STRUCTURE,
                         psTemplate: *mut DROID_TEMPLATE, add: BOOL);
    //returns the quantity of a specific template in the production list
    #[no_mangle]
    fn getProductionQuantity(psStructure: *mut STRUCTURE,
                             psTemplate: *mut DROID_TEMPLATE) -> UDWORD;
    //adjust the loop quantity for this factory
    #[no_mangle]
    fn factoryLoopAdjust(psStruct: *mut STRUCTURE, add: BOOL);
    /*cancels the production run for the factory and returns any power that was 
accrued but not used*/
    #[no_mangle]
    fn cancelProduction(psBuilding: *mut STRUCTURE);
    /*set a factory's production run to hold*/
    #[no_mangle]
    fn holdProduction(psBuilding: *mut STRUCTURE);
    /*release a factory's production run from hold*/
    #[no_mangle]
    fn releaseProduction(psBuilding: *mut STRUCTURE);
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
    #[no_mangle]
    static mut numFeatureStats: UDWORD;
    /* Create a feature on the map */
    #[no_mangle]
    fn buildFeature(psStats: *mut FEATURE_STATS, x: UDWORD, y: UDWORD,
                    FromSave: BOOL) -> *mut FEATURE;
    /* The lists of objects allocated */
    #[no_mangle]
    static mut apsDroidLists: [*mut DROID; 8];
    #[no_mangle]
    static mut apsStructLists: [*mut STRUCTURE; 8];
    #[no_mangle]
    static mut apsFlagPosLists: [*mut FLAG_POSITION; 8];
    /* add the droid to the Droid Lists */
    #[no_mangle]
    fn addDroid(psDroidToAdd: *mut DROID, pList_0: *mut *mut DROID);
    /* Check no alliance has formed*/
    #[no_mangle]
    fn aiCheckAlliances(s1: UDWORD, s2: UDWORD) -> BOOL;
    #[no_mangle]
    static mut asPropulsionStats: *mut PROPULSION_STATS;
    #[no_mangle]
    static mut asWeaponStats: *mut WEAPON_STATS;
    #[no_mangle]
    static mut asConstructStats: *mut CONSTRUCT_STATS;
    #[no_mangle]
    static mut apStructTypeLists: [*mut UBYTE; 8];
    #[no_mangle]
    fn calcTemplatePower(psTemplate: *mut DROID_TEMPLATE) -> UDWORD;
    #[no_mangle]
    fn buildDroid(pTemplate: *mut DROID_TEMPLATE, x: UDWORD, y: UDWORD,
                  player: UDWORD, onMission: BOOL) -> *mut DROID;
    #[no_mangle]
    fn droidType(psDroid: *mut DROID) -> DROID_TYPE;
    #[no_mangle]
    fn fillTemplateList(pList_0: *mut *mut DROID_TEMPLATE,
                        psFactory: *mut STRUCTURE, limit: UDWORD) -> UDWORD;
    #[no_mangle]
    fn droidGetName(psDroid: *mut DROID) -> *mut STRING;
    #[no_mangle]
    fn getTemplateName(psTemplate: *mut DROID_TEMPLATE) -> *mut STRING;
    // Select a droid and do any necessary housekeeping.
    #[no_mangle]
    fn SelectDroid(psDroid: *mut DROID);
    // De-select a droid and do any necessary housekeeping.
    #[no_mangle]
    fn DeSelectDroid(psDroid: *mut DROID);
    #[no_mangle]
    static mut loopMissionState: LOOP_MISSION_STATE;
    #[no_mangle]
    static mut display3D: BOOL;
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
    fn iV_CreateFontIndirect(ImageFile: *mut IMAGEFILE,
                             AsciiTable: *mut UWORD, SpaceSize: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn iV_GetImageWidth(ImageFile: *mut IMAGEFILE, ID: UWORD) -> UWORD;
    #[no_mangle]
    fn iV_GetImageHeight(ImageFile: *mut IMAGEFILE, ID: UWORD) -> UWORD;
    #[no_mangle]
    fn pie_UploadDisplayBuffer(DisplayBuffer_0: *mut libc::c_char);
    #[no_mangle]
    fn pie_DownloadDisplayBuffer(DisplayBuffer_0: *mut libc::c_char);
    #[no_mangle]
    fn screen_RestartBackDrop();
    #[no_mangle]
    static mut mapX: UDWORD;
    #[no_mangle]
    static mut mapY: UDWORD;
    #[no_mangle]
    static mut radarOnScreen: BOOL;
    #[no_mangle]
    fn setViewPos(x: UDWORD, y: UDWORD, Pan: BOOL);
    #[no_mangle]
    fn getPlayerPos(px: *mut SDWORD, py: *mut SDWORD);
    #[no_mangle]
    fn setPlayerPos(x: SDWORD, y: SDWORD);
    #[no_mangle]
    fn doWeDrawProximitys() -> BOOL;
    //extern BOOL		bScreenClose;
//extern UDWORD closingTimeStart;
//extern UDWORD screenCloseState;
//extern BOOL	bPlayerHasHQ;
    #[no_mangle]
    static mut bRender3DOnly: BOOL;
    /*Add a messgae to the list */
    /*remove a message */
    /* Remove all Messages*/
    /* removes all the proximity displays */
    /*load the view data for the messages from the file exported from the world editor*/
    /*get the view data that contains the text message pointer passed in */
    /* Release the viewdata memory */
    //extern void storeProximityScreenCoords(MESSAGE *psMessage, SDWORD x, SDWORD y);
    /* Looks through the players list of messages to find one with the same viewData 
pointer and which is the same type of message - used in scriptFuncs */
    /*'displays' a proximity display*/
    #[no_mangle]
    fn displayProximityMessage(psProxDisp: *mut PROXIMITY_DISPLAY);
    #[no_mangle]
    static mut apsProxDisp: [*mut PROXIMITY_DISPLAY; 8];
    #[no_mangle]
    fn found3DBuilding(x: *mut UDWORD, y: *mut UDWORD) -> BOOL;
    #[no_mangle]
    fn found3DBuildLocTwo(px1: *mut UDWORD, py1: *mut UDWORD,
                          px2: *mut UDWORD, py2: *mut UDWORD) -> BOOL;
    #[no_mangle]
    fn init3DBuilding(psStats: *mut BASE_STATS, CallBack: BUILDCALLBACK,
                      UserData: *mut libc::c_void);
    #[no_mangle]
    fn kill3DBuilding();
    /* The store for the research stats */
    #[no_mangle]
    static mut asResearch: *mut RESEARCH;
    #[no_mangle]
    static mut numResearch: UDWORD;
    //List of pointers to arrays of PLAYER_RESEARCH[numResearch] for each player
    #[no_mangle]
    static mut asPlayerResList: [*mut PLAYER_RESEARCH; 8];
    /*function to check what can be researched for a particular player at any one 
  instant. Returns the number to research*/
//extern UBYTE fillResearchList(UBYTE *plist, UDWORD playerID, UWORD topic, 
//							   UWORD limit);
//needs to be UWORD sized for Patches
    #[no_mangle]
    fn fillResearchList(plist: *mut UWORD, playerID: UDWORD, topic: UWORD,
                        limit: UWORD) -> UWORD;
    /* sets the status of the topic to cancelled and stores the current research
   points accquired */
    #[no_mangle]
    fn cancelResearch(psBuilding: *mut STRUCTURE);
    #[no_mangle]
    fn mapRIDToIcon(rid: UDWORD) -> SDWORD;
    #[no_mangle]
    fn mapIconToRID(iconID: UDWORD) -> SDWORD;
    /*puts research facility on hold*/
    #[no_mangle]
    fn holdResearch(psBuilding: *mut STRUCTURE);
    /*release a research facility from hold*/
    #[no_mangle]
    fn releaseResearch(psBuilding: *mut STRUCTURE);
    /* Call this to restart the game timer after a call to gameTimeStop */
    #[no_mangle]
    fn gameTimeStart();
    #[no_mangle]
    fn saveGame(aFileName: *mut STRING, saveType: SDWORD) -> BOOL;
    /*check the available power*/
    #[no_mangle]
    fn checkPower(player: UDWORD, quantity: UDWORD, playAudio: BOOL) -> BOOL;
    /* Give a droid an order */
    #[no_mangle]
    fn orderDroid(psDroid: *mut DROID, order: DROID_ORDER);
    /* Check the order state of a droid */
    #[no_mangle]
    fn orderState(psDroid: *mut DROID, order: DROID_ORDER) -> BOOL;
    /* Get the state of a droid order with an object */
    #[no_mangle]
    fn orderStateObj(psDroid: *mut DROID, order: DROID_ORDER,
                     ppsObj: *mut *mut BASE_OBJECT) -> BOOL;
    /* Give a droid an order with a location and a stat */
    #[no_mangle]
    fn orderDroidStatsLoc(psDroid: *mut DROID, order: DROID_ORDER,
                          psStats: *mut BASE_STATS, x: UDWORD, y: UDWORD);
    /* Get the state of a droid order with a location and a stat */
    #[no_mangle]
    fn orderStateStatsLoc(psDroid: *mut DROID, order: DROID_ORDER,
                          ppsStats: *mut *mut BASE_STATS, pX: *mut UDWORD,
                          pY: *mut UDWORD) -> BOOL;
    /* Give a droid an order with a location and a stat */
    #[no_mangle]
    fn orderDroidStatsTwoLoc(psDroid: *mut DROID, order: DROID_ORDER,
                             psStats: *mut BASE_STATS, x1: UDWORD, y1: UDWORD,
                             x2: UDWORD, y2: UDWORD);
    /* order all selected droids with a location and a stat */
    #[no_mangle]
    fn orderSelectedStatsLoc(player: UDWORD, order: DROID_ORDER,
                             psStats: *mut BASE_STATS, x: UDWORD, y: UDWORD,
                             add: BOOL);
    /* order all selected droids with two a locations and a stat */
    #[no_mangle]
    fn orderSelectedStatsTwoLoc(player: UDWORD, order: DROID_ORDER,
                                psStats: *mut BASE_STATS, x1: UDWORD,
                                y1: UDWORD, x2: UDWORD, y2: UDWORD,
                                add: BOOL);
    #[no_mangle]
    static mut IntImages: *mut IMAGEFILE;
    #[no_mangle]
    static mut StandardTab: TABDEF;
    #[no_mangle]
    static mut SmallTab: TABDEF;
    // Begin a rendering lock.
    #[no_mangle]
    fn DrawBegin();
    // End a rendering lock.
    #[no_mangle]
    fn DrawEnd();
    #[no_mangle]
    static mut TopicBuffers: [RENDERED_BUTTON; 20];
    #[no_mangle]
    static mut ObjectBuffers: [RENDERED_BUTTON; 40];
    #[no_mangle]
    static mut StatBuffers: [RENDERED_BUTTON; 80];
    #[no_mangle]
    static mut ManuPower: UDWORD;
    // Initialise interface graphics.
    #[no_mangle]
    fn intInitialiseGraphics();
    // Free up interface graphics.
    #[no_mangle]
    fn intDeleteGraphics();
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
    // Clear ( make unused ) all RENDERED_BUTTON structures for the stat window.
    #[no_mangle]
    fn ClearStatBuffers();
    // callback to update the command droid size label
    #[no_mangle]
    fn intUpdateCommandSize(psWidget: *mut _widget,
                            psContext: *mut _w_context);
    // callback to update the command droid experience
    #[no_mangle]
    fn intUpdateCommandExp(psWidget: *mut _widget,
                           psContext: *mut _w_context);
    // callback to update the command droid factories
    #[no_mangle]
    fn intUpdateCommandFact(psWidget: *mut _widget,
                            psContext: *mut _w_context);
    #[no_mangle]
    fn intUpdateProgressBar(psWidget: *mut _widget,
                            psContext: *mut _w_context);
    #[no_mangle]
    fn intUpdateQuantity(psWidget: *mut _widget, psContext: *mut _w_context);
    //callback to display the factory number
    #[no_mangle]
    fn intAddFactoryInc(psWidget: *mut _widget, psContext: *mut _w_context);
    //callback to display the production quantity number for a template
    #[no_mangle]
    fn intAddProdQuantity(psWidget: *mut _widget, psContext: *mut _w_context);
    #[no_mangle]
    fn intDisplayPowerBar(psWidget: *mut _widget, xOffset: UDWORD,
                          yOffset: UDWORD, pColours: *mut UDWORD);
    #[no_mangle]
    fn intDisplayStatusButton(psWidget: *mut _widget, xOffset: UDWORD,
                              yOffset: UDWORD, pColours: *mut UDWORD);
    #[no_mangle]
    fn intDisplayObjectButton(psWidget: *mut _widget, xOffset: UDWORD,
                              yOffset: UDWORD, pColours: *mut UDWORD);
    #[no_mangle]
    fn intDisplayStatsButton(psWidget: *mut _widget, xOffset: UDWORD,
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
    fn intDisplayButtonPressed(psWidget: *mut _widget, xOffset: UDWORD,
                               yOffset: UDWORD, pColours: *mut UDWORD);
    #[no_mangle]
    fn intDisplayReticuleButton(psWidget: *mut _widget, xOffset: UDWORD,
                                yOffset: UDWORD, pColours: *mut UDWORD);
    #[no_mangle]
    fn intDisplayTab(psWidget: *mut _widget, TabType: UDWORD,
                     Position: UDWORD, Number: UDWORD, Selected: BOOL,
                     Hilight: BOOL, x: UDWORD, y: UDWORD, Width: UDWORD,
                     Height: UDWORD);
    #[no_mangle]
    fn intAddLoopQuantity(psWidget: *mut _widget, psContext: *mut _w_context);
    // Widget callback function to play an audio track.
    #[no_mangle]
    fn WidgetAudioCallback(AudioID: libc::c_int);
    /*Displays the proximity messages blips over the world*/
    #[no_mangle]
    fn intDisplayProximityBlips(psWidget: *mut _widget, xOffset: UDWORD,
                                yOffset: UDWORD, pColours: *mut UDWORD);
    #[no_mangle]
    fn intDisplayDPButton(psWidget: *mut _widget, xOffset: UDWORD,
                          yOffset: UDWORD, pColours: *mut UDWORD);
    #[no_mangle]
    fn intDisplayResSubGroup(psWidget: *mut _widget, xOffset: UDWORD,
                             yOffset: UDWORD, pColours: *mut UDWORD);
    #[no_mangle]
    fn intDisplayAllyIcon(psWidget: *mut _widget, xOffset: UDWORD,
                          yOffset: UDWORD, pColours: *mut UDWORD);
    // The tabbed form of components
    // The bottom form with the overall stats
    // The clickable form for the brain
    // The clickable form for the weapon/ecm/sensor
    // The clickable form for the body
    // The clickable form for the propulsion
    // The 3D view of the droid
    // The Brain display box
    // The bin button
    // The Name label
    // The Name box
    // The Brain label
    // The System label
    // The Body label
    // The Propulsion label
    // The 3D view label
    // The form for the power and points bars
    // The base form for the Template (left) form
    // The base form for the right form!
    // The form to hold the player' icon on
    // The power bar for the template
    // The weapon button for the Component form (right)
    // The systems (sensor/ecm) button for the Component form
    // The command button for the Component form
    // Part buttons form
    /* Design screen bar graph IDs */
    // The body armour bar graph for kinetic weapons
    // The body power plant graph
    // The body weight
    // The propulsion road speed
    // The propulsion cross country speed
    // The propulsion water speed
    // The propulsion air speed
    // The propulsion weight
    // The sensor range
    // The sensor power
    // The sensor weight
    // The ecm power
    // The ecm weight
    // The weapon range
    // The weapon damage
    // The weapon rate of fire
    // The weapon weight
    // The construction build points
    // The construction weight
    //extras added AB 3/9/97
    // The body points bar graph
    // The body armour bar graph for heat weapons
    // The Repair points
    // The repair weight
    /* Design screen bar graph labels */
    // The body armour (kinetic) bar graph label
    // The body power plant graph label
    // The body weight graph label
    // The propulsion road speed label
    // The propulsion cross country speed label
    // The propulsion water speed label
    // The propulsion air speed label
    // The propulsion weight label
    // The sensor range label
    // The sensor power label
    // The sensor weight label
    // The ecm power label
    // The ecm weight label
    // The weapon range label
    // The weapon damage label
    // The weapon rate of fire label
    // The weapon weight label
    // The construction build points label
    // The construction weight label
    //extras added AB 3/9/97
//#define IDDES_BODYPOINTSLAB		5227		// The body points label
    // The body armour (heat) bar graph label
    // The template's Power req label
    // The template's Body Points label
    // The Repair Points label
    // The Repair Weigth label
    /* Design screen buttons */
    // The first design template button
    // The last design template button
    // The first component button
    // The last component button
    // The first extra system button
    // The last extra system button
    // System button
    // Body button
    // Propulsion button
    #[no_mangle]
    fn intAddDesign(bShowCentreScreen: BOOL) -> BOOL;
    #[no_mangle]
    fn intRemoveDesign();
    #[no_mangle]
    fn intProcessDesign(id: UDWORD);
    #[no_mangle]
    fn intRunDesign();
    // The current message being displayed
    #[no_mangle]
    static mut psCurrentMsg: *mut MESSAGE;
    /* Add the Intelligence Map widgets to the widget screen */
//extern BOOL intAddIntelMap(BOOL playCurrent);
    #[no_mangle]
    fn intAddIntelMap() -> BOOL;
    /* Process return codes from the Intelligence Map */
    #[no_mangle]
    fn intProcessIntelMap(id: UDWORD);
    /* Process return code from the Message View for Tutorial Mode*/
//extern void intProcessMessageView(UDWORD id);
    /* rotate the view so looking directly down if forward = TRUE or
 back to previous view if forward = FALSE */
//extern void intelMapView(BOOL forward);
    /* Remove the Intelligence Map widgets from the screen */
    #[no_mangle]
    fn intRemoveIntelMap();
    /* Remove the Intelligence Map widgets from the screen without animation*/
    #[no_mangle]
    fn intRemoveIntelMapNoAnim();
    //initialise the text display stats for the current message
//extern void initTextDisplay(MESSAGE *psMessage, UDWORD fontID, UWORD fontColour);
    /* scroll the text message from left to right - aka tickertape messages */
//extern void scrollMessage(STRING *pText, UDWORD startX, UDWORD endX, UDWORD y, UDWORD gap);
    /*sets psCurrentMsg for the Intelligence screen*/
    #[no_mangle]
    fn setCurrentMsg();
    /*sets the flag*/
    #[no_mangle]
    fn setMessageImmediate(state: BOOL);
    #[no_mangle]
    static mut OrderUp: BOOL;
    // update already open order form
    //changed to a BASE_OBJECT to accomodate the factories - AB 21/04/99
//BOOL intAddOrder(DROID *Droid);			// create and open order form
    #[no_mangle]
    fn intAddOrder(psObj: *mut BASE_OBJECT) -> BOOL;
    // create and open order form
    #[no_mangle]
    fn intRunOrder();
    #[no_mangle]
    fn intProcessOrder(id: UDWORD);
    #[no_mangle]
    fn intRemoveOrder();
    #[no_mangle]
    fn intRemoveOrderNoAnim();
    #[no_mangle]
    fn intRefreshOrder() -> BOOL;
    #[no_mangle]
    fn drawRadar();
    /*	Sets up a map surface by allocating the necessary memory and assigning world
	variables for the renderer to work with */
    #[no_mangle]
    fn setUpMapSurface(width: UDWORD, height: UDWORD) -> *mut iSurface;
    #[no_mangle]
    fn releaseMapSurface(pSurface: *mut iSurface);
    #[no_mangle]
    fn DrawnInLastFrame(Frame: SDWORD) -> BOOL;
    // Clear all selections.
    #[no_mangle]
    fn clearSel();
    // Clear all selections and stop driver mode.
    #[no_mangle]
    fn clearSelection();
    /* Do the 3D display */
    #[no_mangle]
    fn displayWorld();
    #[no_mangle]
    fn StartDeliveryPosition(psLocation: *mut OBJECT_POSITION,
                             driveActive: BOOL);
    #[no_mangle]
    fn AddDerrickBurningMessage();
    // check whether the queue order keys are pressed
    #[no_mangle]
    fn ctrlShiftDown() -> BOOL;
    /* The string resource object */
    #[no_mangle]
    static mut psStringRes: *mut STR_RES;
    #[no_mangle]
    fn GetGameMode() -> UDWORD;
    #[no_mangle]
    static mut mission: MISSION;
    #[no_mangle]
    static mut offWorldKeepLists: BOOL;
    //returns TRUE if the mission is a Limbo Expand mission
    #[no_mangle]
    fn missionLimboExpand() -> BOOL;
    //position defines
    // status of the mission result screens.
    #[no_mangle]
    static mut MissionResUp: BOOL;
    #[no_mangle]
    fn intRemoveMissionResultNoAnim();
    #[no_mangle]
    fn intProcessMissionResult(id: UDWORD);
    #[no_mangle]
    fn intRunMissionResult();
    #[no_mangle]
    fn AllocateSnapBuffer(SnapBuffer: *mut CURSORSNAP, MaxSnaps: UWORD);
    #[no_mangle]
    fn ReleaseSnapBuffer(SnapBuffer: *mut CURSORSNAP);
    #[no_mangle]
    fn StartCursorSnap(SnapBuffer: *mut CURSORSNAP);
    #[no_mangle]
    fn intCloseInGameOptions(bPutUpLoadSave: BOOL, bResetMissionWidgets: BOOL)
     -> BOOL;
    #[no_mangle]
    fn intCloseInGameOptionsNoAnim(bResetMissionWidgets: BOOL);
    #[no_mangle]
    fn intRunInGameOptions() -> BOOL;
    #[no_mangle]
    fn intProcessInGameOptions(_: UDWORD);
    // status bools.
    #[no_mangle]
    static mut ClosingInGameOp: BOOL;
    #[no_mangle]
    static mut InGameOpUp: BOOL;
    // Refresh the transporter screen.
    #[no_mangle]
    fn intRefreshTransporter() -> BOOL;
    /*Add the Transporter Interface*/
    #[no_mangle]
    fn intAddTransporter(psSelected: *mut DROID, offWorld: BOOL) -> BOOL;
    /* Remove the Transporter widgets from the screen */
    #[no_mangle]
    fn intRemoveTrans();
    #[no_mangle]
    fn intRemoveTransNoAnim();
    /* Process return codes from the Transporter Screen*/
    #[no_mangle]
    fn intProcessTransporter(id: UDWORD);
    //process the launch transporter button click
    #[no_mangle]
    fn processLaunchTransporter();
    /*checks the order of the droid to see if its currenly flying*/
    #[no_mangle]
    fn transporterFlying(psTransporter: *mut DROID) -> BOOL;
    #[no_mangle]
    fn getWarCamStatus() -> BOOL;
    #[no_mangle]
    fn camToggleStatus();
    /* **********************************************************************************
 *
 * Compiler setup functions
 */
    /* Set the type table */
    /* Set the function table */
    /* Set the external variable table */
    /* Set the object variable table */
    /* Set the constant table */
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
    // whether the tutorial is active
    #[no_mangle]
    static mut bInTutorial: BOOL;
    // the Droid that was selected for a CALL_DROID_SELECTED
    #[no_mangle]
    static mut psCBSelectedDroid: *mut DROID;
    #[no_mangle]
    static mut ConsoleString: [libc::c_char; 255];
    #[no_mangle]
    fn addConsoleMessage(messageText: *mut STRING,
                         jusType: CONSOLE_TEXT_JUSTIFICATION) -> BOOL;
    #[no_mangle]
    fn displayConsoleMessages();
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
    #[no_mangle]
    fn displayLoadSave() -> BOOL;
    #[no_mangle]
    fn deleteSaveGame(saveGameName_0: *mut libc::c_char);
    #[no_mangle]
    fn testPlayerHasLost() -> BOOL;
    #[no_mangle]
    fn testPlayerHasWon() -> BOOL;
    //control
    #[no_mangle]
    fn seq_SetupVideoBuffers() -> BOOL;
    #[no_mangle]
    fn seq_ReleaseVideoBuffers() -> BOOL;
    // the game description.
    #[no_mangle]
    static mut bMultiPlayer: BOOL;
    #[no_mangle]
    fn getPlayerName(player: UDWORD) -> *mut STRING;
    #[no_mangle]
    fn sendReseachStatus(psBuilding: *mut STRUCTURE, index: UDWORD,
                         player: UBYTE, bStart: BOOL) -> BOOL;
    // multimenu
    #[no_mangle]
    fn intProcessMultiMenu(id: UDWORD);
    #[no_mangle]
    fn intRunMultiMenu() -> BOOL;
    #[no_mangle]
    fn intCloseMultiMenu() -> BOOL;
    #[no_mangle]
    fn intCloseMultiMenuNoAnim();
    #[no_mangle]
    static mut MultiMenuUp: BOOL;
    #[no_mangle]
    static mut ClosingMultiMenu: BOOL;
    #[no_mangle]
    static mut DirectControl: BOOL;
    #[no_mangle]
    static mut psDrivenDroid: *mut DROID;
    #[no_mangle]
    fn driveSelectionChanged();
    #[no_mangle]
    fn driveDisableControl();
    #[no_mangle]
    fn driveEnableInterface(AddReticule: BOOL);
    #[no_mangle]
    fn driveInterfaceEnabled() -> BOOL;
    #[no_mangle]
    fn driveStartBuild();
    #[no_mangle]
    fn driveDisableTactical();
    #[no_mangle]
    fn fireOnLocation(x: UDWORD, y: UDWORD) -> BOOL;
    #[no_mangle]
    fn getDebugMappingStatus() -> BOOL;
    // // Move into drive.c when we create it.
//void driveDissableControl(void)
//{
//}
//
//
//void driveEnableControl(void)
//{
//}
//
//
//void driveProcessCursorSnap(void)
//{
//	DBPRINTF(("driveProcessCursorSnap\n");
//	if(VPadPressed(VPAD_MOUSERB)) {
//		driveDissableControl();
//		widgetsOn = TRUE;
//		StartInterfaceSnap();
//	}
//}
    #[no_mangle]
    static mut AsciiLookup: [UWORD; 256];
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
pub type _widget_type = libc::c_uint;
pub const WIDG_SLIDER: _widget_type = 5;
pub const WIDG_BARGRAPH: _widget_type = 4;
pub const WIDG_EDITBOX: _widget_type = 3;
pub const WIDG_BUTTON: _widget_type = 2;
pub const WIDG_LABEL: _widget_type = 1;
pub const WIDG_FORM: _widget_type = 0;
pub type WIDGET_TYPE = _widget_type;
// The next free ID
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
/* The screen structure which stores all info for a widget screen */
pub type W_SCREEN = _w_screen;
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
// The root form of the screen
// The widget that has keyboard focus
//	PROP_FONT	*psTipFont;			// The font for tool tips
// ID of the IVIS font to use for tool tips.
// Bar graph with a trough showing empty percentage
// Double bar graph, one on top of other
/* ********** Slider styles ***************/
// Plain slider
/* **********************************************************************************/
/* Generic widget colour */
pub type W_COLOURDEF = _w_colourdef;
/* Upper limits for major and minor tabs on a tab form.
 * Not the best way to do it I know, but it keeps the memory
 * management MUCH simpler.
 */
//15		// Maximum number of major tabs on a tab form
//15		// Maximum number of minor tabs off a major
// Tab types passed into tab display callbacks.
pub type TAB_DISPLAY
    =
    Option<unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                _: UDWORD, _: BOOL, _: BOOL, _: UDWORD,
                                _: UDWORD, _: UDWORD, _: UDWORD) -> ()>;
pub type FONT_DISPLAY
    =
    Option<unsafe extern "C" fn(_: UDWORD, _: UDWORD, _: *mut STRING) -> ()>;
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
/* Form initialisation structure */
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
/* The basic init entries */
/* Data for a tabbed form */
// Position of the tabs on the form
// Size of the tabs (in pixels)
// Tab start offset.
// Tab form overlap offset.
// Tab form overlap offset.
// The thickness of the tabs
// The thickness of the tabs
// The space between tabs
// The space between tabs
// Number of major tabs
// Number of minor tabs for each major
// Tool tip for the form itself
// Tool tips for the major tabs
// Tool tips for the minor tabs
// Optional callback for displaying a tab.
// Optional callback to display the form.
/* Label initialisation structure */
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
/* The basic init entries */
// label text
// Tool tip for the label.
//	PROP_FONT	*psFont;		// label font
// ID of the IVIS font to use for this widget.
/* Button initialisation structure */
pub type W_BUTINIT = _w_butinit;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_edbinit {
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
    pub FontID: libc::c_int,
    pub pBoxDisplay: WIDGET_DISPLAY,
    pub pFontDisplay: FONT_DISPLAY,
}
/* The basic init entries */
// button text
// Tool tip text
//	PROP_FONT	*psFont;		// button font
// ID of the IVIS font to use for this widget.
/* Edit box initialisation structure */
pub type W_EDBINIT = _w_edbinit;
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
/* The basic init entries */
// initial contents of the edit box
//	PROP_FONT	*psFont;		// edit box font
// ID of the IVIS font to use for this widget.
// Optional callback to display the form.
// Optional callback to display a string.
// Bar graph fills from left to right
// Bar graph fills from right to left
// Bar graph fills from top to bottom
// Bar graph fills from bottom to top
/* Bar Graph initialisation structure */
pub type W_BARINIT = _w_barinit;
/* The basic init entries */
// Orientation of the bar on the widget
// Initial percentage of the graph that is filled
// Percentage of second bar graph if there is one
// Maximum range
// Bar colour
// Minor bar colour
// Tool tip text
/* Colour numbers */
pub type _w_colour = libc::c_uint;
// all colour numbers are less than this
// Text colour on a disabled button
pub const WCOL_MAX: _w_colour = 8;
// Background for the tool tip window
pub const WCOL_DISABLE: _w_colour = 7;
// Edit Box cursor colour
pub const WCOL_TIPBKGRND: _w_colour = 6;
// Hilite colour
pub const WCOL_CURSOR: _w_colour = 5;
// Dark colour for 3D effects
pub const WCOL_HILITE: _w_colour = 4;
// Light colour for 3D effects
pub const WCOL_DARK: _w_colour = 3;
// Text colour
pub const WCOL_LIGHT: _w_colour = 2;
// Background colours
pub const WCOL_TEXT: _w_colour = 1;
pub const WCOL_BKGRND: _w_colour = 0;
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
// only needed when generating the tree
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
pub type LOOP_MISSION_STATE = libc::c_uint;
pub const LMS_CLEAROBJECTS: LOOP_MISSION_STATE = 5;
pub const LMS_LOADGAME: LOOP_MISSION_STATE = 4;
pub const LMS_NEWLEVEL: LOOP_MISSION_STATE = 3;
pub const LMS_SAVECONTINUE: LOOP_MISSION_STATE = 2;
pub const LMS_SETUPMISSION: LOOP_MISSION_STATE = 1;
pub const LMS_NORMAL: LOOP_MISSION_STATE = 0;
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
pub type _message_type = libc::c_uint;
pub const MSG_TYPES: _message_type = 4;
pub const MSG_PROXIMITY: _message_type = 3;
pub const MSG_MISSION: _message_type = 2;
pub const MSG_CAMPAIGN: _message_type = 1;
pub const MSG_RESEARCH: _message_type = 0;
pub type MESSAGE_TYPE = _message_type;
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
//base structure for each message
pub type MESSAGE = _message;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _proximity_display {
    pub type_0: POSITION_TYPE,
    pub frameNumber: UDWORD,
    pub screenX: UDWORD,
    pub screenY: UDWORD,
    pub screenR: UDWORD,
    pub player: UDWORD,
    pub selected: BOOL,
    pub psMessage: *mut MESSAGE,
    pub radarX: UDWORD,
    pub radarY: UDWORD,
    pub timeLastDrawn: UDWORD,
    pub strobe: UDWORD,
    pub buttonID: UDWORD,
    pub psNext: *mut _proximity_display,
}
//The type of message
//ID number of the message
//VIEWDATA		*pViewData;				//Pointer to view data - if any - should be some!
//Pointer to view data - if any - should be some!
//flag to indicate whether message has been read
//which player this message belongs to
//pointer to the next in the list
//used to display the proximity messages
pub type PROXIMITY_DISPLAY = _proximity_display;
pub type BUILDCALLBACK
    =
    Option<unsafe extern "C" fn(_: UDWORD, _: UDWORD, _: *mut libc::c_void)
               -> ()>;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const RID_MAXRID: C2RustUnnamed_0 = 20;
pub const RID_GRPDAM: C2RustUnnamed_0 = 19;
pub const RID_GRPROF: C2RustUnnamed_0 = 18;
pub const RID_GRPREP: C2RustUnnamed_0 = 17;
pub const RID_GRPUPG: C2RustUnnamed_0 = 16;
pub const RID_GRPACC: C2RustUnnamed_0 = 15;
pub const RID_QUESTIONMARK: C2RustUnnamed_0 = 14;
pub const RID_DEFENCE: C2RustUnnamed_0 = 13;
pub const RID_CYBORGTECH: C2RustUnnamed_0 = 12;
pub const RID_STRUCTURETECH: C2RustUnnamed_0 = 11;
pub const RID_SYSTEMTECH: C2RustUnnamed_0 = 10;
pub const RID_POWERTECH: C2RustUnnamed_0 = 9;
pub const RID_COMPUTERTECH: C2RustUnnamed_0 = 8;
pub const RID_WEAPONTECH: C2RustUnnamed_0 = 7;
pub const RID_DROIDTECH: C2RustUnnamed_0 = 6;
pub const RID_TRACKS: C2RustUnnamed_0 = 5;
pub const RID_PLASCRETE: C2RustUnnamed_0 = 4;
pub const RID_ECM: C2RustUnnamed_0 = 3;
pub const RID_HOVERCRAFT: C2RustUnnamed_0 = 2;
pub const RID_CANNON: C2RustUnnamed_0 = 1;
pub const RID_ROCKET: C2RustUnnamed_0 = 0;
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
pub const STR_INT_POWER: _fixed_str_id = 55;
pub const IMAGE_PBAR_EMPTY: C2RustUnnamed_3 = 0;
pub const IMAGE_CANCEL_UP: C2RustUnnamed_3 = 40;
pub const STR_RET_CLOSE: _fixed_str_id = 7;
//message associated with this 'button'
//Used to store the radar coords - if to be drawn
//stores the time the 'button' was last drawn for animation
//id of image last used
//id of the button for the interface
//pointer to the next in the list
// Reticule button indecies.
pub const RETBUT_CANCEL: C2RustUnnamed_4 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BUTOFFSET {
    pub x: SWORD,
    pub y: SWORD,
}
pub const IMAGE_BUILD_UP: C2RustUnnamed_3 = 32;
pub const STR_RET_BUILD: _fixed_str_id = 6;
pub const RETBUT_BUILD: C2RustUnnamed_4 = 3;
pub const IMAGE_RESEARCH_UP: C2RustUnnamed_3 = 34;
pub const STR_RET_RESEARCH: _fixed_str_id = 5;
pub const RETBUT_RESEARCH: C2RustUnnamed_4 = 2;
pub const IMAGE_DESIGN_UP: C2RustUnnamed_3 = 30;
pub const STR_RET_DESIGN: _fixed_str_id = 4;
pub const RETBUT_DESIGN: C2RustUnnamed_4 = 4;
pub const IMAGE_MANUFACTURE_UP: C2RustUnnamed_3 = 36;
pub const STR_RET_MANUFACTURE: _fixed_str_id = 3;
pub const RETBUT_FACTORY: C2RustUnnamed_4 = 1;
pub const IMAGE_INTELMAP_UP: C2RustUnnamed_3 = 26;
pub const STR_RET_INTELLIGENCE: _fixed_str_id = 2;
pub const RETBUT_INTELMAP: C2RustUnnamed_4 = 5;
pub const IMAGE_COMMANDDROID_UP: C2RustUnnamed_3 = 28;
pub const STR_RET_COMMAND: _fixed_str_id = 8;
pub const RETBUT_COMMAND: C2RustUnnamed_4 = 6;
pub const ID_SOUND_SELECT: C2RustUnnamed_2 = 2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BUTSTATE {
    pub id: UDWORD,
    pub Enabled: BOOL,
    pub Hidden: BOOL,
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
pub type _int_retval = libc::c_uint;
pub const INT_QUIT: _int_retval = 3;
pub const INT_INTELNOSCROLL: _int_retval = 2;
pub const INT_INTERCEPT: _int_retval = 1;
pub const INT_NONE: _int_retval = 0;
/* Return codes for the widget interface */
pub type INT_RETVAL = _int_retval;
pub type TRIGGER_TYPE = _trigger_type;
pub type _trigger_type = libc::c_uint;
pub const TR_CALLBACKSTART: _trigger_type = 5;
pub const TR_PAUSE: _trigger_type = 4;
pub const TR_EVERY: _trigger_type = 3;
pub const TR_WAIT: _trigger_type = 2;
pub const TR_CODE: _trigger_type = 1;
pub const TR_INIT: _trigger_type = 0;
pub const CALL_DESIGN_QUIT: _scr_callback_types = 28;
pub const CALL_OBJECTCLOSE: _scr_callback_types = 48;
// no key clicks have been intercepted
// key clicks have been intercepted
//INT_FULLSCREENPAUSE,	// The widget interface is full screen and
							// the rest of the game should pause
	//INT_INTELPAUSE,			// The Intelligence Map is up and all update
							// routines should pause - hopefully!
//The 3DView of the intelligence screen is up
// and we don't want scroll (or update!)
// The game should quit
/* The tabbed form data structure */
pub type W_TABFORM = _w_tabform;
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
/* The common form data */
// Position of the tabs on the form
// the size of tabs horizontally and vertically
// The thickness of the tabs
// The thickness of the tabs
// The gap between tabs
// The gap between tabs
// Tab form overlap offset.
// Tab form overlap offset.
// Tab start offset.
// Tab start offset.
// which tab is selected
// Current state of the widget
// which tab is hilited.
/* NOTE: If tabHiLite is (UWORD)(-1) then there is no hilite.  A bit of a hack I know */
	/*       but I don't really have the energy to change it.  (Don't design stuff after  */
	/*       beers at lunch-time :-)                                                      */
// The number of major tabs
// The major tab information
// Optional callback for display tabs.
// Optional callback to display the form.
/* Information for a major tab */
pub type W_MAJORTAB = _w_majortab;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_majortab {
    pub lastMinor: UWORD,
    pub numMinor: UWORD,
    pub asMinor: [W_MINORTAB; 5],
    pub pTip: *mut STRING,
}
/* Graphics data for the tab will go here */
// Store which was the last selected minor tab
// Minor tab information
/* Information for a minor tab */
pub type W_MINORTAB = _w_minortab;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_minortab {
    pub psWidgets: *mut WIDGET,
    pub pTip: *mut STRING,
}
/* Graphics data for the tab will go here */
// Widgets on the tab
// Tool tip
/* Which type of object screen is being displayed. Starting value is where the intMode left off*/
pub type _obj_mode = libc::c_uint;
// maximum object mode
// the command droid screen
pub const IOBJ_MAX: _obj_mode = 22;
// The research screen
pub const IOBJ_COMMAND: _obj_mode = 21;
// The manufacture screen
pub const IOBJ_RESEARCH: _obj_mode = 20;
// Selecting a structure to demolish
pub const IOBJ_MANUFACTURE: _obj_mode = 19;
// Selecting a position for a new structure
pub const IOBJ_DEMOLISHSEL: _obj_mode = 18;
// The build screen
pub const IOBJ_BUILDSEL: _obj_mode = 17;
// Nothing doing.
pub const IOBJ_BUILD: _obj_mode = 16;
pub const IOBJ_NONE: _obj_mode = 15;
/* Status of the positioning for the object placement */
pub type _edit_pos_mode = libc::c_uint;
pub const IED_POS: _edit_pos_mode = 1;
pub const IED_NOPOS: _edit_pos_mode = 0;
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
pub const CALL_OBJECTOPEN: _scr_callback_types = 47;
pub const CALL_MANULIST: _scr_callback_types = 25;
pub const CALL_RESEARCHLIST: _scr_callback_types = 23;
pub const CALL_BUILDLIST: _scr_callback_types = 21;
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
pub struct BUTTON_SURFACE {
    pub Buffer: *mut uint8,
    pub Surface: *mut iSurface,
}
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
pub const IMAGE_CLOSE: C2RustUnnamed_3 = 43;
pub const IMAGE_CLOSEHILIGHT: C2RustUnnamed_3 = 45;
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
// Miscellaneous strings
pub const STR_MISC_CLOSE: _fixed_str_id = 9;
pub const IMAGE_LOOP_UP: C2RustUnnamed_3 = 350;
pub const IMAGE_LOOP_HI: C2RustUnnamed_3 = 352;
pub const IMAGE_LOOP_DOWN: C2RustUnnamed_3 = 351;
pub const STR_INT_LOOP: _fixed_str_id = 53;
pub const STR_INT_DPOINT: _fixed_str_id = 52;
pub const IMAGE_FDP_DOWN: C2RustUnnamed_3 = 356;
/* Function type for getting the appropriate stats for an object */
pub type OBJ_GETSTATS
    =
    Option<unsafe extern "C" fn(_: *mut BASE_OBJECT) -> *mut BASE_STATS>;
// Other interface strings
pub const STR_INT_BLDPROGRESS: _fixed_str_id = 43;
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
//UDWORD		initialPower;	    HAVEN'T FOUND A USE FOR IT YET! AB 26/8/98	
									    /* what the initial power level is - set 
									    in script not sure if will need it, but 
									    keeping for now*/
/* the current amount of power avaialble 
									    to the player*/
/* the power extracted but not converted */
/*the last object that received power 
									    before it ran out*/
/* Function type for selecting a base object while building the object screen */
pub type OBJ_SELECT
    =
    Option<unsafe extern "C" fn(_: *mut BASE_OBJECT) -> BOOL>;
/* Function type for setting the appropriate stats for an object */
pub type OBJ_SETSTATS
    =
    Option<unsafe extern "C" fn(_: *mut BASE_OBJECT, _: *mut BASE_STATS)
               -> BOOL>;
pub const CALL_BUILDGRID: _scr_callback_types = 22;
pub const ID_SOUND_WINDOWCLOSE: C2RustUnnamed_2 = 0;
pub const STR_MISC_QUIT: _fixed_str_id = 12;
pub const CALL_BUTTON_PRESSED: _scr_callback_types = 26;
pub type CONSOLE_TEXT_JUSTIFICATION = libc::c_uint;
pub const DEFAULT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 3;
pub const CENTRE_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 2;
pub const RIGHT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 1;
pub const LEFT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 0;
pub const STR_GAME_SAVED: _fixed_str_id = 298;
// Stand alone mission.
pub const GTYPE_SAVE_START: C2RustUnnamed_1 = 3;
pub const CALL_DROID_SELECTED: _scr_callback_types = 27;
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
pub type C2RustUnnamed_1 = libc::c_uint;
// User saved game - in the middle of a level
// User saved game - at the start of a level.
pub const GTYPE_SAVE_MIDMISSION: C2RustUnnamed_1 = 4;
// Scenario scroll area expansion.
pub const GTYPE_MISSION: C2RustUnnamed_1 = 2;
// Initial scenario state.
pub const GTYPE_SCENARIO_EXPAND: C2RustUnnamed_1 = 1;
pub const GTYPE_SCENARIO_START: C2RustUnnamed_1 = 0;
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
pub const ID_SOUND_BLIMP_FLIGHT: C2RustUnnamed_2 = 282;
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
pub const ID_SOUND_1_MINUTE_REMAINING: C2RustUnnamed_2 = 144;
pub const ID_SOUND_2_MINUTES_REMAINING: C2RustUnnamed_2 = 143;
pub const ID_SOUND_3_MINUTES_REMAINING: C2RustUnnamed_2 = 142;
pub const ID_SOUND_5_MINUTES_REMAINING: C2RustUnnamed_2 = 141;
pub const ID_SOUND_10_MINUTES_REMAINING: C2RustUnnamed_2 = 140;
pub const ID_SOUND_MISSION_TIMER_ACTIVATED: C2RustUnnamed_2 = 139;
pub const ID_SOUND_OBJECTIVE_FAILED: C2RustUnnamed_2 = 138;
pub const ID_SOUND_OBJECTIVE_ACCOMPLISHED: C2RustUnnamed_2 = 137;
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
pub const ID_SOUND_WINDOWOPEN: C2RustUnnamed_2 = 1;
pub const NO_SOUND: C2RustUnnamed_2 = -1;
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
// label states.
// label is hilited
pub type W_LABEL = _w_label;
/* The common widget data */
// The current button state
// Text on the label
//	PROP_FONT	*psFont;				// Font for the label
// The tool tip for the button
/* **********************************************/
/* Image ID definition file created by Framer. */
/* **********************************************/
pub type C2RustUnnamed_3 = libc::c_uint;
pub const IMAGE_ASCII64: C2RustUnnamed_3 = 483;
pub const IMAGE_ORD_FIREDES_DOWN: C2RustUnnamed_3 = 482;
pub const IMAGE_ORD_FIREDES_UP: C2RustUnnamed_3 = 481;
pub const IMAGE_TRANSETA_DOWN: C2RustUnnamed_3 = 480;
pub const IMAGE_LAUNCHUP: C2RustUnnamed_3 = 479;
pub const IMAGE_BLUE6: C2RustUnnamed_3 = 478;
pub const IMAGE_BLUE5: C2RustUnnamed_3 = 477;
pub const IMAGE_BLUE4: C2RustUnnamed_3 = 476;
pub const IMAGE_BLUE3: C2RustUnnamed_3 = 475;
pub const IMAGE_BLUE2: C2RustUnnamed_3 = 474;
pub const IMAGE_BLUE1: C2RustUnnamed_3 = 473;
pub const IMAGE_DES_ARMOUR_KINETIC: C2RustUnnamed_3 = 472;
pub const IMAGE_TABHILIGHT_SM: C2RustUnnamed_3 = 471;
pub const IMAGE_TAB1DOWN_SM: C2RustUnnamed_3 = 470;
pub const IMAGE_TAB1SELECTED_SM: C2RustUnnamed_3 = 469;
pub const IMAGE_TAB1_SM: C2RustUnnamed_3 = 468;
pub const IMAGE_TARGET5: C2RustUnnamed_3 = 467;
pub const IMAGE_TARGET4: C2RustUnnamed_3 = 466;
pub const IMAGE_TARGET3: C2RustUnnamed_3 = 465;
pub const IMAGE_TARGET2: C2RustUnnamed_3 = 464;
pub const IMAGE_TARGET1: C2RustUnnamed_3 = 463;
pub const IMAGE_MISSION_CLOCK_UP: C2RustUnnamed_3 = 462;
pub const IMAGE_RES_GRPDAM: C2RustUnnamed_3 = 461;
pub const IMAGE_RES_GRPACC: C2RustUnnamed_3 = 460;
pub const IMAGE_RES_GRPROF: C2RustUnnamed_3 = 459;
pub const IMAGE_RES_GRPREP: C2RustUnnamed_3 = 458;
pub const IMAGE_RES_GRPUPG: C2RustUnnamed_3 = 457;
pub const IMAGE_ASCII131: C2RustUnnamed_3 = 456;
pub const IMAGE_ASCII161: C2RustUnnamed_3 = 455;
pub const IMAGE_ASCII191: C2RustUnnamed_3 = 454;
pub const IMAGE_ASCII208: C2RustUnnamed_3 = 453;
pub const IMAGE_ASCII207: C2RustUnnamed_3 = 452;
pub const IMAGE_ASCII206: C2RustUnnamed_3 = 451;
pub const IMAGE_ASCII205: C2RustUnnamed_3 = 450;
pub const IMAGE_ASCII204: C2RustUnnamed_3 = 449;
pub const IMAGE_ASCII203: C2RustUnnamed_3 = 448;
pub const IMAGE_ASCII202: C2RustUnnamed_3 = 447;
pub const IMAGE_ASCII201: C2RustUnnamed_3 = 446;
pub const IMAGE_ASCII200: C2RustUnnamed_3 = 445;
pub const IMAGE_ASCII198: C2RustUnnamed_3 = 444;
pub const IMAGE_ASCII197: C2RustUnnamed_3 = 443;
pub const IMAGE_ASCII196: C2RustUnnamed_3 = 442;
pub const IMAGE_ASCII195: C2RustUnnamed_3 = 441;
pub const IMAGE_ASCII194: C2RustUnnamed_3 = 440;
pub const IMAGE_ASCII193: C2RustUnnamed_3 = 439;
pub const IMAGE_ASCII192: C2RustUnnamed_3 = 438;
pub const IMAGE_ASCII210: C2RustUnnamed_3 = 437;
pub const IMAGE_ASCII211: C2RustUnnamed_3 = 436;
pub const IMAGE_ASCII214: C2RustUnnamed_3 = 435;
pub const IMAGE_ASCII213: C2RustUnnamed_3 = 434;
pub const IMAGE_ASCII212: C2RustUnnamed_3 = 433;
pub const IMAGE_ASCII216: C2RustUnnamed_3 = 432;
pub const IMAGE_ASCII220: C2RustUnnamed_3 = 431;
pub const IMAGE_ASCII217: C2RustUnnamed_3 = 430;
pub const IMAGE_ASCII219: C2RustUnnamed_3 = 429;
pub const IMAGE_ASCII218: C2RustUnnamed_3 = 428;
pub const IMAGE_ASCII221: C2RustUnnamed_3 = 427;
pub const IMAGE_ASCII223: C2RustUnnamed_3 = 426;
pub const IMAGE_ASCII248: C2RustUnnamed_3 = 425;
pub const IMAGE_ASCII241: C2RustUnnamed_3 = 424;
pub const IMAGE_ASCII253: C2RustUnnamed_3 = 423;
pub const IMAGE_ASCII252: C2RustUnnamed_3 = 422;
pub const IMAGE_ASCII251: C2RustUnnamed_3 = 421;
pub const IMAGE_ASCII250: C2RustUnnamed_3 = 420;
pub const IMAGE_ASCII249: C2RustUnnamed_3 = 419;
pub const IMAGE_ASCII246: C2RustUnnamed_3 = 418;
pub const IMAGE_ASCII245: C2RustUnnamed_3 = 417;
pub const IMAGE_ASCII244: C2RustUnnamed_3 = 416;
pub const IMAGE_ASCII243: C2RustUnnamed_3 = 415;
pub const IMAGE_ASCII242: C2RustUnnamed_3 = 414;
pub const IMAGE_ASCII239: C2RustUnnamed_3 = 413;
pub const IMAGE_ASCII238: C2RustUnnamed_3 = 412;
pub const IMAGE_ASCII237: C2RustUnnamed_3 = 411;
pub const IMAGE_ASCII236: C2RustUnnamed_3 = 410;
pub const IMAGE_ASCII235: C2RustUnnamed_3 = 409;
pub const IMAGE_ASCII234: C2RustUnnamed_3 = 408;
pub const IMAGE_ASCII233: C2RustUnnamed_3 = 407;
pub const IMAGE_ASCII232: C2RustUnnamed_3 = 406;
pub const IMAGE_ASCII231: C2RustUnnamed_3 = 405;
pub const IMAGE_ASCII230: C2RustUnnamed_3 = 404;
pub const IMAGE_ASCII229: C2RustUnnamed_3 = 403;
pub const IMAGE_ASCII228: C2RustUnnamed_3 = 402;
pub const IMAGE_ASCII227: C2RustUnnamed_3 = 401;
pub const IMAGE_ASCII226: C2RustUnnamed_3 = 400;
pub const IMAGE_ASCII225: C2RustUnnamed_3 = 399;
pub const IMAGE_ASCII224: C2RustUnnamed_3 = 398;
pub const IMAGE_ASCII189: C2RustUnnamed_3 = 397;
pub const IMAGE_ASCII188: C2RustUnnamed_3 = 396;
pub const IMAGE_RES_DEFENCE: C2RustUnnamed_3 = 395;
pub const IMAGE_LEV_7: C2RustUnnamed_3 = 394;
pub const IMAGE_LEV_6: C2RustUnnamed_3 = 393;
pub const IMAGE_LEV_5: C2RustUnnamed_3 = 392;
pub const IMAGE_LEV_4: C2RustUnnamed_3 = 391;
pub const IMAGE_LEV_3: C2RustUnnamed_3 = 390;
pub const IMAGE_LEV_2: C2RustUnnamed_3 = 389;
pub const IMAGE_LEV_1: C2RustUnnamed_3 = 388;
pub const IMAGE_LEV_0: C2RustUnnamed_3 = 387;
pub const IMAGE_CDCHANGE_CANCEL: C2RustUnnamed_3 = 386;
pub const IMAGE_CDCHANGE_OK: C2RustUnnamed_3 = 385;
pub const IMAGE_MULTI_CHAN: C2RustUnnamed_3 = 384;
pub const IMAGE_MULTI_NOCHAN: C2RustUnnamed_3 = 383;
pub const IMAGE_ORD_EMBARKDOWN: C2RustUnnamed_3 = 382;
pub const IMAGE_ORD_EMBARKUP: C2RustUnnamed_3 = 381;
pub const IMAGE_ORD_RTRDOWN: C2RustUnnamed_3 = 380;
pub const IMAGE_ORD_RTRUP: C2RustUnnamed_3 = 379;
pub const IMAGE_ORD_GUARDDOWN: C2RustUnnamed_3 = 378;
pub const IMAGE_ORD_GUARDUP: C2RustUnnamed_3 = 377;
pub const IMAGE_ORD_PERSUEDOWN: C2RustUnnamed_3 = 376;
pub const IMAGE_ORD_PERSUEUP: C2RustUnnamed_3 = 375;
pub const IMAGE_ORD_PATROLDOWN: C2RustUnnamed_3 = 374;
pub const IMAGE_ORD_PATROLUP: C2RustUnnamed_3 = 373;
pub const IMAGE_GN_0: C2RustUnnamed_3 = 372;
pub const IMAGE_GN_1: C2RustUnnamed_3 = 371;
pub const IMAGE_GN_2: C2RustUnnamed_3 = 370;
pub const IMAGE_GN_3: C2RustUnnamed_3 = 369;
pub const IMAGE_GN_4: C2RustUnnamed_3 = 368;
pub const IMAGE_GN_5: C2RustUnnamed_3 = 367;
pub const IMAGE_GN_6: C2RustUnnamed_3 = 366;
pub const IMAGE_GN_7: C2RustUnnamed_3 = 365;
pub const IMAGE_GN_8: C2RustUnnamed_3 = 364;
pub const IMAGE_GN_9: C2RustUnnamed_3 = 363;
pub const IMAGE_GN_STAR: C2RustUnnamed_3 = 362;
pub const IMAGE_VDP_HI: C2RustUnnamed_3 = 361;
pub const IMAGE_VDP_UP: C2RustUnnamed_3 = 360;
pub const IMAGE_VDP_DOWN: C2RustUnnamed_3 = 359;
pub const IMAGE_FDP_HI: C2RustUnnamed_3 = 358;
pub const IMAGE_FDP_UP: C2RustUnnamed_3 = 357;
pub const IMAGE_CDP_HI: C2RustUnnamed_3 = 355;
pub const IMAGE_CDP_UP: C2RustUnnamed_3 = 354;
pub const IMAGE_CDP_DOWN: C2RustUnnamed_3 = 353;
pub const IMAGE_RES_CYBORGTECH: C2RustUnnamed_3 = 349;
pub const IMAGE_CURSOR_DEST: C2RustUnnamed_3 = 348;
pub const IMAGE_DES_EDITBOXRIGHTH: C2RustUnnamed_3 = 347;
pub const IMAGE_DES_EDITBOXMIDH: C2RustUnnamed_3 = 346;
pub const IMAGE_DES_EDITBOXRIGHT: C2RustUnnamed_3 = 345;
pub const IMAGE_DES_EDITBOXMID: C2RustUnnamed_3 = 344;
pub const IMAGE_DES_POWERBAR_RIGHT: C2RustUnnamed_3 = 343;
pub const IMAGE_DES_POWERBAR_LEFT: C2RustUnnamed_3 = 342;
pub const IMAGE_AUDIO_LASTSAMPLEH: C2RustUnnamed_3 = 341;
pub const IMAGE_AUDIO_LASTSAMPLE: C2RustUnnamed_3 = 340;
pub const IMAGE_DES_BINH: C2RustUnnamed_3 = 339;
pub const IMAGE_DES_BIN: C2RustUnnamed_3 = 338;
pub const IMAGE_INFINITE_HI: C2RustUnnamed_3 = 337;
pub const IMAGE_INFINITE_DOWN: C2RustUnnamed_3 = 336;
pub const IMAGE_INFINITE_UP: C2RustUnnamed_3 = 335;
pub const IMAGE_RETICULE_GREY: C2RustUnnamed_3 = 334;
pub const IMAGE_ORD_FACHILITE: C2RustUnnamed_3 = 333;
pub const IMAGE_ORD_FAC5DOWN: C2RustUnnamed_3 = 332;
pub const IMAGE_ORD_FAC5UP: C2RustUnnamed_3 = 331;
pub const IMAGE_ORD_FAC4DOWN: C2RustUnnamed_3 = 330;
pub const IMAGE_ORD_FAC4UP: C2RustUnnamed_3 = 329;
pub const IMAGE_ORD_FAC3DOWN: C2RustUnnamed_3 = 328;
pub const IMAGE_ORD_FAC3UP: C2RustUnnamed_3 = 327;
pub const IMAGE_ORD_FAC2DOWN: C2RustUnnamed_3 = 326;
pub const IMAGE_ORD_FAC2UP: C2RustUnnamed_3 = 325;
pub const IMAGE_ORD_FAC1DOWN: C2RustUnnamed_3 = 324;
pub const IMAGE_ORD_FAC1UP: C2RustUnnamed_3 = 323;
pub const IMAGE_TRANSETA_UP: C2RustUnnamed_3 = 322;
pub const IMAGE_LAUNCHDOWN: C2RustUnnamed_3 = 321;
pub const IMAGE_MISSION_CLOCK: C2RustUnnamed_3 = 320;
pub const IMAGE_NRUTER: C2RustUnnamed_3 = 319;
pub const IMAGE_CMDDROID_EXP: C2RustUnnamed_3 = 318;
pub const IMAGE_MULTI_VIS_HI: C2RustUnnamed_3 = 317;
pub const IMAGE_MULTI_POW_HI: C2RustUnnamed_3 = 316;
pub const IMAGE_MULTI_DRO_HI: C2RustUnnamed_3 = 315;
pub const IMAGE_MULTI_TEK_HI: C2RustUnnamed_3 = 314;
pub const IMAGE_MULTI_TEK: C2RustUnnamed_3 = 313;
pub const IMAGE_MULTI_DRO: C2RustUnnamed_3 = 312;
pub const IMAGE_MULTI_POW: C2RustUnnamed_3 = 311;
pub const IMAGE_MULTI_VIS: C2RustUnnamed_3 = 310;
pub const IMAGE_MULTI_AL_HI: C2RustUnnamed_3 = 309;
pub const IMAGE_MULTI_OFFAL_HI: C2RustUnnamed_3 = 308;
pub const IMAGE_MULTI_NOAL_HI: C2RustUnnamed_3 = 307;
pub const IMAGE_MULTI_AL: C2RustUnnamed_3 = 306;
pub const IMAGE_MULTI_OFFAL: C2RustUnnamed_3 = 305;
pub const IMAGE_MULTI_NOAL: C2RustUnnamed_3 = 304;
pub const IMAGE_RAD_ART3: C2RustUnnamed_3 = 303;
pub const IMAGE_RAD_ART2: C2RustUnnamed_3 = 302;
pub const IMAGE_RAD_ART1: C2RustUnnamed_3 = 301;
pub const IMAGE_RAD_ARTREAD: C2RustUnnamed_3 = 300;
pub const IMAGE_RAD_RES3: C2RustUnnamed_3 = 299;
pub const IMAGE_RAD_RES2: C2RustUnnamed_3 = 298;
pub const IMAGE_RAD_RES1: C2RustUnnamed_3 = 297;
pub const IMAGE_RAD_RESREAD: C2RustUnnamed_3 = 296;
pub const IMAGE_RAD_ENM3: C2RustUnnamed_3 = 295;
pub const IMAGE_RAD_ENM2: C2RustUnnamed_3 = 294;
pub const IMAGE_RAD_ENM1: C2RustUnnamed_3 = 293;
pub const IMAGE_RAD_ENMREAD: C2RustUnnamed_3 = 292;
pub const IMAGE_SLIDER_INFINITY: C2RustUnnamed_3 = 291;
pub const IMAGE_SLIDER_BIGBUT: C2RustUnnamed_3 = 290;
pub const IMAGE_ORD_DESTRUCT2GREY: C2RustUnnamed_3 = 289;
pub const IMAGE_ORD_DESTRUCT2DOWN: C2RustUnnamed_3 = 288;
pub const IMAGE_ORD_DESTRUCT2UP: C2RustUnnamed_3 = 287;
pub const IMAGE_ORD_GOTOHQDOWN: C2RustUnnamed_3 = 286;
pub const IMAGE_ORD_GOTOHQUP: C2RustUnnamed_3 = 285;
pub const IMAGE_ORD_HALTDOWN: C2RustUnnamed_3 = 284;
pub const IMAGE_ORD_HALTUP: C2RustUnnamed_3 = 283;
pub const IMAGE_ORD_HOLDFIREDOWN: C2RustUnnamed_3 = 282;
pub const IMAGE_ORD_HOLDFIREUP: C2RustUnnamed_3 = 281;
pub const IMAGE_ORD_RETFIREDOWN: C2RustUnnamed_3 = 280;
pub const IMAGE_ORD_RETFIREUP: C2RustUnnamed_3 = 279;
pub const IMAGE_ORD_FATWILLDOWN: C2RustUnnamed_3 = 278;
pub const IMAGE_ORD_FATWILLUP: C2RustUnnamed_3 = 277;
pub const IMAGE_ORD_REPAIR3DOWN: C2RustUnnamed_3 = 276;
pub const IMAGE_ORD_REPAIR3UP: C2RustUnnamed_3 = 275;
pub const IMAGE_ORD_REPAIR1DOWN: C2RustUnnamed_3 = 274;
pub const IMAGE_ORD_REPAIR1UP: C2RustUnnamed_3 = 273;
pub const IMAGE_ORD_REPAIR2DOWN: C2RustUnnamed_3 = 272;
pub const IMAGE_ORD_REPAIR2UP: C2RustUnnamed_3 = 271;
pub const IMAGE_ORD_RANGE3DOWN: C2RustUnnamed_3 = 270;
pub const IMAGE_ORD_RANGE3UP: C2RustUnnamed_3 = 269;
pub const IMAGE_ORD_RANGE2DOWN: C2RustUnnamed_3 = 268;
pub const IMAGE_ORD_RANGE2UP: C2RustUnnamed_3 = 267;
pub const IMAGE_ORD_RANGE1DOWN: C2RustUnnamed_3 = 266;
pub const IMAGE_ORD_RANGE1UP: C2RustUnnamed_3 = 265;
pub const IMAGE_ORD_DESTRUCT1DOWN: C2RustUnnamed_3 = 264;
pub const IMAGE_ORD_DESTRUCT1UP: C2RustUnnamed_3 = 263;
pub const IMAGE_RES_QUESTIONMARK: C2RustUnnamed_3 = 262;
pub const IMAGE_RES_STRUCTURETECH: C2RustUnnamed_3 = 261;
pub const IMAGE_RES_SYSTEMTECH: C2RustUnnamed_3 = 260;
pub const IMAGE_RES_POWERTECH: C2RustUnnamed_3 = 259;
pub const IMAGE_RES_COMPUTERTECH: C2RustUnnamed_3 = 258;
pub const IMAGE_RES_WEAPONTECH: C2RustUnnamed_3 = 257;
pub const IMAGE_RES_DROIDTECH: C2RustUnnamed_3 = 256;
pub const IMAGE_QUESTION_MARK: C2RustUnnamed_3 = 255;
pub const IMAGE_RES_MAJOR_ROCKET: C2RustUnnamed_3 = 254;
pub const IMAGE_RES_MINOR_AUTOWEAPONS: C2RustUnnamed_3 = 253;
pub const IMAGE_TRACKS: C2RustUnnamed_3 = 252;
pub const IMAGE_STAR: C2RustUnnamed_3 = 251;
pub const IMAGE_SLIDER_BIG: C2RustUnnamed_3 = 250;
pub const IMAGE_DES_STATBACKMID: C2RustUnnamed_3 = 249;
pub const IMAGE_DES_STATBACKRIGHT: C2RustUnnamed_3 = 248;
pub const IMAGE_DES_STATBACKLEFT: C2RustUnnamed_3 = 247;
pub const IMAGE_SIDETABSEL: C2RustUnnamed_3 = 246;
pub const IMAGE_SIDETABDOWN: C2RustUnnamed_3 = 245;
pub const IMAGE_SIDETABHI: C2RustUnnamed_3 = 244;
pub const IMAGE_SIDETAB: C2RustUnnamed_3 = 243;
pub const IMAGE_CURSOR_NOTPOS: C2RustUnnamed_3 = 242;
pub const IMAGE_CURSOR_PICKUP: C2RustUnnamed_3 = 241;
pub const IMAGE_CURSOR_REPAIR: C2RustUnnamed_3 = 240;
pub const IMAGE_CURSOR_FIX: C2RustUnnamed_3 = 239;
pub const IMAGE_CURSOR_ATTACH: C2RustUnnamed_3 = 238;
pub const IMAGE_CURSOR_BRIDGE: C2RustUnnamed_3 = 237;
pub const IMAGE_CURSOR_EMBARK: C2RustUnnamed_3 = 236;
pub const IMAGE_CURSOR_GUARD: C2RustUnnamed_3 = 235;
pub const IMAGE_CURSOR_MOVE: C2RustUnnamed_3 = 234;
pub const IMAGE_CURSOR_BUILD: C2RustUnnamed_3 = 233;
pub const IMAGE_CURSOR_DEFAULT: C2RustUnnamed_3 = 232;
pub const IMAGE_CURSOR_ECM: C2RustUnnamed_3 = 231;
pub const IMAGE_CURSOR_LOCKON: C2RustUnnamed_3 = 230;
pub const IMAGE_CURSOR_SELECT: C2RustUnnamed_3 = 229;
pub const IMAGE_CURSOR_ATTACK: C2RustUnnamed_3 = 228;
pub const IMAGE_ASCII126: C2RustUnnamed_3 = 227;
pub const IMAGE_ASCII125: C2RustUnnamed_3 = 226;
pub const IMAGE_ASCII124: C2RustUnnamed_3 = 225;
pub const IMAGE_ASCII123: C2RustUnnamed_3 = 224;
pub const IMAGE_ASCII122: C2RustUnnamed_3 = 223;
pub const IMAGE_ASCII121: C2RustUnnamed_3 = 222;
pub const IMAGE_ASCII120: C2RustUnnamed_3 = 221;
pub const IMAGE_ASCII119: C2RustUnnamed_3 = 220;
pub const IMAGE_ASCII118: C2RustUnnamed_3 = 219;
pub const IMAGE_ASCII117: C2RustUnnamed_3 = 218;
pub const IMAGE_ASCII116: C2RustUnnamed_3 = 217;
pub const IMAGE_ASCII115: C2RustUnnamed_3 = 216;
pub const IMAGE_ASCII114: C2RustUnnamed_3 = 215;
pub const IMAGE_ASCII113: C2RustUnnamed_3 = 214;
pub const IMAGE_ASCII112: C2RustUnnamed_3 = 213;
pub const IMAGE_ASCII111: C2RustUnnamed_3 = 212;
pub const IMAGE_ASCII110: C2RustUnnamed_3 = 211;
pub const IMAGE_ASCII109: C2RustUnnamed_3 = 210;
pub const IMAGE_ASCII108: C2RustUnnamed_3 = 209;
pub const IMAGE_ASCII107: C2RustUnnamed_3 = 208;
pub const IMAGE_ASCII106: C2RustUnnamed_3 = 207;
pub const IMAGE_ASCII105: C2RustUnnamed_3 = 206;
pub const IMAGE_ASCII104: C2RustUnnamed_3 = 205;
pub const IMAGE_ASCII103: C2RustUnnamed_3 = 204;
pub const IMAGE_ASCII102: C2RustUnnamed_3 = 203;
pub const IMAGE_ASCII101: C2RustUnnamed_3 = 202;
pub const IMAGE_ASCII100: C2RustUnnamed_3 = 201;
pub const IMAGE_ASCII99: C2RustUnnamed_3 = 200;
pub const IMAGE_ASCII98: C2RustUnnamed_3 = 199;
pub const IMAGE_ASCII97: C2RustUnnamed_3 = 198;
pub const IMAGE_ASCII96: C2RustUnnamed_3 = 197;
pub const IMAGE_ASCII95: C2RustUnnamed_3 = 196;
pub const IMAGE_ASCII94: C2RustUnnamed_3 = 195;
pub const IMAGE_ASCII93: C2RustUnnamed_3 = 194;
pub const IMAGE_ASCII92: C2RustUnnamed_3 = 193;
pub const IMAGE_ASCII91: C2RustUnnamed_3 = 192;
pub const IMAGE_ASCII90: C2RustUnnamed_3 = 191;
pub const IMAGE_ASCII89: C2RustUnnamed_3 = 190;
pub const IMAGE_ASCII88: C2RustUnnamed_3 = 189;
pub const IMAGE_ASCII87: C2RustUnnamed_3 = 188;
pub const IMAGE_ASCII86: C2RustUnnamed_3 = 187;
pub const IMAGE_ASCII85: C2RustUnnamed_3 = 186;
pub const IMAGE_ASCII84: C2RustUnnamed_3 = 185;
pub const IMAGE_ASCII83: C2RustUnnamed_3 = 184;
pub const IMAGE_ASCII82: C2RustUnnamed_3 = 183;
pub const IMAGE_ASCII81: C2RustUnnamed_3 = 182;
pub const IMAGE_ASCII80: C2RustUnnamed_3 = 181;
pub const IMAGE_ASCII79: C2RustUnnamed_3 = 180;
pub const IMAGE_ASCII78: C2RustUnnamed_3 = 179;
pub const IMAGE_ASCII77: C2RustUnnamed_3 = 178;
pub const IMAGE_ASCII76: C2RustUnnamed_3 = 177;
pub const IMAGE_ASCII75: C2RustUnnamed_3 = 176;
pub const IMAGE_ASCII74: C2RustUnnamed_3 = 175;
pub const IMAGE_ASCII73: C2RustUnnamed_3 = 174;
pub const IMAGE_ASCII72: C2RustUnnamed_3 = 173;
pub const IMAGE_ASCII71: C2RustUnnamed_3 = 172;
pub const IMAGE_ASCII70: C2RustUnnamed_3 = 171;
pub const IMAGE_ASCII69: C2RustUnnamed_3 = 170;
pub const IMAGE_ASCII68: C2RustUnnamed_3 = 169;
pub const IMAGE_ASCII67: C2RustUnnamed_3 = 168;
pub const IMAGE_ASCII66: C2RustUnnamed_3 = 167;
pub const IMAGE_ASCII65: C2RustUnnamed_3 = 166;
pub const IMAGE_ASTERISK: C2RustUnnamed_3 = 165;
pub const IMAGE_ASCII63: C2RustUnnamed_3 = 164;
pub const IMAGE_ASCII62: C2RustUnnamed_3 = 163;
pub const IMAGE_ASCII61: C2RustUnnamed_3 = 162;
pub const IMAGE_ASCII60: C2RustUnnamed_3 = 161;
pub const IMAGE_ASCII59: C2RustUnnamed_3 = 160;
pub const IMAGE_ASCII58: C2RustUnnamed_3 = 159;
pub const IMAGE_ASCII57: C2RustUnnamed_3 = 158;
pub const IMAGE_ASCII56: C2RustUnnamed_3 = 157;
pub const IMAGE_ASCII55: C2RustUnnamed_3 = 156;
pub const IMAGE_ASCII54: C2RustUnnamed_3 = 155;
pub const IMAGE_ASCII53: C2RustUnnamed_3 = 154;
pub const IMAGE_ASCII52: C2RustUnnamed_3 = 153;
pub const IMAGE_ASCII51: C2RustUnnamed_3 = 152;
pub const IMAGE_ASCII50: C2RustUnnamed_3 = 151;
pub const IMAGE_ASCII49: C2RustUnnamed_3 = 150;
pub const IMAGE_ASCII48: C2RustUnnamed_3 = 149;
pub const IMAGE_ASCII47: C2RustUnnamed_3 = 148;
pub const IMAGE_ASCII46: C2RustUnnamed_3 = 147;
pub const IMAGE_ASCII45: C2RustUnnamed_3 = 146;
pub const IMAGE_ASCII44: C2RustUnnamed_3 = 145;
pub const IMAGE_ASCII43: C2RustUnnamed_3 = 144;
pub const IMAGE_ASCII42: C2RustUnnamed_3 = 143;
pub const IMAGE_ASCII41: C2RustUnnamed_3 = 142;
pub const IMAGE_ASCII40: C2RustUnnamed_3 = 141;
pub const IMAGE_ASCII39: C2RustUnnamed_3 = 140;
pub const IMAGE_ASCII38: C2RustUnnamed_3 = 139;
pub const IMAGE_ASCII37: C2RustUnnamed_3 = 138;
pub const IMAGE_ASCII36: C2RustUnnamed_3 = 137;
pub const IMAGE_ASCII35: C2RustUnnamed_3 = 136;
pub const IMAGE_ASCII34: C2RustUnnamed_3 = 135;
pub const IMAGE_ASCII33: C2RustUnnamed_3 = 134;
pub const IMAGE_INTEL_CAMPAIGNDOWN: C2RustUnnamed_3 = 133;
pub const IMAGE_INTEL_CAMPAIGN: C2RustUnnamed_3 = 132;
pub const IMAGE_INTEL_MISSIONDOWN: C2RustUnnamed_3 = 131;
pub const IMAGE_INTEL_MISSION: C2RustUnnamed_3 = 130;
pub const IMAGE_INTEL_RESEARCHDOWN: C2RustUnnamed_3 = 129;
pub const IMAGE_INTEL_RESEARCH: C2RustUnnamed_3 = 128;
pub const IMAGE_DES_BARYELLOW: C2RustUnnamed_3 = 127;
pub const IMAGE_DES_BARRED: C2RustUnnamed_3 = 126;
pub const IMAGE_DES_BARBLUE: C2RustUnnamed_3 = 125;
pub const IMAGE_DES_BARBACK: C2RustUnnamed_3 = 124;
pub const IMAGE_9: C2RustUnnamed_3 = 123;
pub const IMAGE_8: C2RustUnnamed_3 = 122;
pub const IMAGE_7: C2RustUnnamed_3 = 121;
pub const IMAGE_6: C2RustUnnamed_3 = 120;
pub const IMAGE_5: C2RustUnnamed_3 = 119;
pub const IMAGE_4: C2RustUnnamed_3 = 118;
pub const IMAGE_3: C2RustUnnamed_3 = 117;
pub const IMAGE_2: C2RustUnnamed_3 = 116;
pub const IMAGE_1: C2RustUnnamed_3 = 115;
pub const IMAGE_0: C2RustUnnamed_3 = 114;
pub const IMAGE_RES_MAJOR_HOVER: C2RustUnnamed_3 = 113;
pub const IMAGE_RES_MAJOR_HEAVYWEP: C2RustUnnamed_3 = 112;
pub const IMAGE_RES_MAJOR_ELECTRONIC: C2RustUnnamed_3 = 111;
pub const IMAGE_RES_MAJOR_PLASCRETE: C2RustUnnamed_3 = 110;
pub const IMAGE_RES_MINOR_RADAR: C2RustUnnamed_3 = 109;
pub const IMAGE_PLASCRETE: C2RustUnnamed_3 = 108;
pub const IMAGE_ECM: C2RustUnnamed_3 = 107;
pub const IMAGE_RES_MINOR_PLASSTEEL: C2RustUnnamed_3 = 106;
pub const IMAGE_HOVERCRAFT: C2RustUnnamed_3 = 105;
pub const IMAGE_CANNON: C2RustUnnamed_3 = 104;
pub const IMAGE_ROCKET: C2RustUnnamed_3 = 103;
pub const IMAGE_DES_BODYPOINTS: C2RustUnnamed_3 = 102;
pub const IMAGE_DES_FIRERATE: C2RustUnnamed_3 = 101;
pub const IMAGE_DES_HOVER: C2RustUnnamed_3 = 100;
pub const IMAGE_DES_POWER: C2RustUnnamed_3 = 99;
pub const IMAGE_DES_DAMAGE: C2RustUnnamed_3 = 98;
pub const IMAGE_DES_WEIGHT: C2RustUnnamed_3 = 97;
pub const IMAGE_DES_ROAD: C2RustUnnamed_3 = 96;
pub const IMAGE_DES_CROSSCOUNTRY: C2RustUnnamed_3 = 95;
pub const IMAGE_DES_ARMOUR_EXPLOSIVE: C2RustUnnamed_3 = 94;
pub const IMAGE_DES_RANGE: C2RustUnnamed_3 = 93;
pub const IMAGE_DES_BUILDRATE: C2RustUnnamed_3 = 92;
pub const IMAGE_DES_TABWEAPONDOWN: C2RustUnnamed_3 = 91;
pub const IMAGE_DES_TABWEAPON: C2RustUnnamed_3 = 90;
pub const IMAGE_DES_EXTRAHI: C2RustUnnamed_3 = 89;
pub const IMAGE_DES_STATSCOMP: C2RustUnnamed_3 = 88;
pub const IMAGE_DES_BACK: C2RustUnnamed_3 = 87;
pub const IMAGE_FRAME_HC3: C2RustUnnamed_3 = 86;
pub const IMAGE_FRAME_HC2: C2RustUnnamed_3 = 85;
pub const IMAGE_FRAME_HC1: C2RustUnnamed_3 = 84;
pub const IMAGE_FRAME_HC0: C2RustUnnamed_3 = 83;
pub const IMAGE_FRAME_VC3: C2RustUnnamed_3 = 82;
pub const IMAGE_FRAME_VC2: C2RustUnnamed_3 = 81;
pub const IMAGE_FRAME_VC1: C2RustUnnamed_3 = 80;
pub const IMAGE_FRAME_VC0: C2RustUnnamed_3 = 79;
pub const IMAGE_DES_EDITBOXLEFTH: C2RustUnnamed_3 = 78;
pub const IMAGE_DES_EDITBOXLEFT: C2RustUnnamed_3 = 77;
pub const IMAGE_DES_POWERBACK: C2RustUnnamed_3 = 76;
pub const IMAGE_DES_STATSCURR: C2RustUnnamed_3 = 75;
pub const IMAGE_DES_STATSBACK: C2RustUnnamed_3 = 74;
pub const IMAGE_DES_POWERCURR: C2RustUnnamed_3 = 73;
pub const IMAGE_FRAME_HBH: C2RustUnnamed_3 = 72;
pub const IMAGE_FRAME_HTH: C2RustUnnamed_3 = 71;
pub const IMAGE_FRAME_HB2: C2RustUnnamed_3 = 70;
pub const IMAGE_FRAME_HT2: C2RustUnnamed_3 = 69;
pub const IMAGE_DES_COMMANDDOWN: C2RustUnnamed_3 = 68;
pub const IMAGE_DES_COMMAND: C2RustUnnamed_3 = 67;
pub const IMAGE_DES_SYSTEMSDOWN: C2RustUnnamed_3 = 66;
pub const IMAGE_DES_SYSTEMS: C2RustUnnamed_3 = 65;
pub const IMAGE_DES_WEAPONSDOWN: C2RustUnnamed_3 = 64;
pub const IMAGE_DES_WEAPONS: C2RustUnnamed_3 = 63;
pub const IMAGE_FRAME_VRH: C2RustUnnamed_3 = 62;
pub const IMAGE_FRAME_VLH: C2RustUnnamed_3 = 61;
pub const IMAGE_FRAME_VR2: C2RustUnnamed_3 = 60;
pub const IMAGE_FRAME_VL2: C2RustUnnamed_3 = 59;
pub const IMAGE_DES_HILIGHT: C2RustUnnamed_3 = 58;
pub const IMAGE_DES_PROPULSIONH: C2RustUnnamed_3 = 57;
pub const IMAGE_DES_PROPULSION: C2RustUnnamed_3 = 56;
pub const IMAGE_DES_BODYH: C2RustUnnamed_3 = 55;
pub const IMAGE_DES_BODY: C2RustUnnamed_3 = 54;
pub const IMAGE_DES_TURRETH: C2RustUnnamed_3 = 53;
pub const IMAGE_DES_TURRET: C2RustUnnamed_3 = 52;
pub const IMAGE_FRAME_VR: C2RustUnnamed_3 = 51;
pub const IMAGE_FRAME_VL: C2RustUnnamed_3 = 50;
pub const IMAGE_SLIDER_BACK: C2RustUnnamed_3 = 49;
pub const IMAGE_FRAME_HB: C2RustUnnamed_3 = 48;
pub const IMAGE_FRAME_HT: C2RustUnnamed_3 = 47;
pub const IMAGE_SLIDER_BUT: C2RustUnnamed_3 = 46;
pub const IMAGE_CLOSEDOWN: C2RustUnnamed_3 = 44;
pub const IMAGE_CANCEL_HILIGHT: C2RustUnnamed_3 = 42;
pub const IMAGE_CANCEL_DOWN: C2RustUnnamed_3 = 41;
pub const IMAGE_RETICULE_BUTDOWN: C2RustUnnamed_3 = 39;
pub const IMAGE_RETICULE_HILIGHT: C2RustUnnamed_3 = 38;
pub const IMAGE_MANUFACTURE_DOWN: C2RustUnnamed_3 = 37;
pub const IMAGE_RESEARCH_DOWN: C2RustUnnamed_3 = 35;
pub const IMAGE_BUILD_DOWN: C2RustUnnamed_3 = 33;
pub const IMAGE_DESIGN_DOWN: C2RustUnnamed_3 = 31;
pub const IMAGE_COMMANDDROID_DOWN: C2RustUnnamed_3 = 29;
pub const IMAGE_INTELMAP_DOWN: C2RustUnnamed_3 = 27;
pub const IMAGE_PBAR_BOTTOM: C2RustUnnamed_3 = 25;
pub const IMAGE_PBAR_TOP: C2RustUnnamed_3 = 24;
pub const IMAGE_FRAME_C3: C2RustUnnamed_3 = 23;
pub const IMAGE_FRAME_C2: C2RustUnnamed_3 = 22;
pub const IMAGE_FRAME_C1: C2RustUnnamed_3 = 21;
pub const IMAGE_FRAME_C0: C2RustUnnamed_3 = 20;
pub const IMAGE_TABHILIGHT: C2RustUnnamed_3 = 19;
pub const IMAGE_TABSELECTED: C2RustUnnamed_3 = 18;
pub const IMAGE_TAB4DOWN: C2RustUnnamed_3 = 17;
pub const IMAGE_TAB3DOWN: C2RustUnnamed_3 = 16;
pub const IMAGE_TAB2DOWN: C2RustUnnamed_3 = 15;
pub const IMAGE_TAB1DOWN: C2RustUnnamed_3 = 14;
pub const IMAGE_TAB4: C2RustUnnamed_3 = 13;
pub const IMAGE_TAB3: C2RustUnnamed_3 = 12;
pub const IMAGE_TAB2: C2RustUnnamed_3 = 11;
pub const IMAGE_TAB1: C2RustUnnamed_3 = 10;
pub const IMAGE_BUTB_HILITE: C2RustUnnamed_3 = 9;
pub const IMAGE_BUT_HILITE: C2RustUnnamed_3 = 8;
pub const IMAGE_BUTB0_DOWN: C2RustUnnamed_3 = 7;
pub const IMAGE_BUTB0_UP: C2RustUnnamed_3 = 6;
pub const IMAGE_BUT0_DOWN: C2RustUnnamed_3 = 5;
pub const IMAGE_BUT0_UP: C2RustUnnamed_3 = 4;
pub const IMAGE_PBAR_REQUIRED: C2RustUnnamed_3 = 3;
pub const IMAGE_PBAR_USED: C2RustUnnamed_3 = 2;
pub const IMAGE_PBAR_AVAIL: C2RustUnnamed_3 = 1;
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
pub const STR_INT_POWERACCRUED: _fixed_str_id = 54;
pub const STR_INT_TRANSLAUNCH: _fixed_str_id = 51;
pub const STR_INT_TRANSPORTER: _fixed_str_id = 50;
pub const STR_INT_RESCOMPLETED: _fixed_str_id = 49;
pub const STR_INT_MISMESSAGE: _fixed_str_id = 48;
pub const STR_INT_GENMESSAGE: _fixed_str_id = 47;
pub const STR_INT_RESMESSAGE: _fixed_str_id = 46;
pub const STR_INT_PWRUSAGE: _fixed_str_id = 45;
pub const STR_INT_BLDSPEED: _fixed_str_id = 44;
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
pub const STR_MISC_SAVEGAME: _fixed_str_id = 11;
pub const STR_MISC_LOADGAME: _fixed_str_id = 10;
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
pub const CALL_MANURUN: _scr_callback_types = 24;
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
// Number of reticule buttons.
pub type C2RustUnnamed_4 = libc::c_uint;
/* Which type of object is being displayed on the edit stats screen */
pub type _edit_obj_mode = libc::c_uint;
// Features
// Structures
pub const IED_FEATURE: _edit_obj_mode = 2;
// Droids
pub const IED_STRUCT: _edit_obj_mode = 1;
pub const IED_DROID: _edit_obj_mode = 0;
// Major weapon classes, used to group weapon classes for selection purposes.
pub type C2RustUnnamed_5 = libc::c_uint;
pub const WMC_EMP: C2RustUnnamed_5 = 9;
pub const WMC_COMMAND: C2RustUnnamed_5 = 8;
pub const WMC_BOMB: C2RustUnnamed_5 = 7;
pub const WMC_LAS_SAT: C2RustUnnamed_5 = 6;
pub const WMC_AA: C2RustUnnamed_5 = 5;
pub const WMC_ELECTRONIC: C2RustUnnamed_5 = 4;
pub const WMC_MUZZLE: C2RustUnnamed_5 = 3;
pub const WMC_MISSILE: C2RustUnnamed_5 = 2;
pub const WMC_SHELL: C2RustUnnamed_5 = 1;
pub const WMC_FLAME: C2RustUnnamed_5 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DROIDDIST {
    pub psDroid: *mut DROID,
    pub Dist: UDWORD,
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
pub static mut ReticuleOffsets: [BUTOFFSET; 7] =
    [{
         let mut init =
             BUTOFFSET{x: 48 as libc::c_int as SWORD,
                       y: 49 as libc::c_int as SWORD,};
         init
     },
     {
         let mut init =
             BUTOFFSET{x: 53 as libc::c_int as SWORD,
                       y: 17 as libc::c_int as SWORD,};
         init
     },
     {
         let mut init =
             BUTOFFSET{x: 87 as libc::c_int as SWORD,
                       y: 35 as libc::c_int as SWORD,};
         init
     },
     {
         let mut init =
             BUTOFFSET{x: 87 as libc::c_int as SWORD,
                       y: 70 as libc::c_int as SWORD,};
         init
     },
     {
         let mut init =
             BUTOFFSET{x: 53 as libc::c_int as SWORD,
                       y: 88 as libc::c_int as SWORD,};
         init
     },
     {
         let mut init =
             BUTOFFSET{x: 19 as libc::c_int as SWORD,
                       y: 70 as libc::c_int as SWORD,};
         init
     },
     {
         let mut init =
             BUTOFFSET{x: 19 as libc::c_int as SWORD,
                       y: 35 as libc::c_int as SWORD,};
         init
     }];
#[no_mangle]
pub static mut ReticuleEnabled: [BUTSTATE; 7] =
    [{
         let mut init =
             BUTSTATE{id: 8 as libc::c_int as UDWORD,
                      Enabled: 0 as libc::c_int,
                      Hidden: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             BUTSTATE{id: 4 as libc::c_int as UDWORD,
                      Enabled: 0 as libc::c_int,
                      Hidden: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             BUTSTATE{id: 5 as libc::c_int as UDWORD,
                      Enabled: 0 as libc::c_int,
                      Hidden: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             BUTSTATE{id: 3 as libc::c_int as UDWORD,
                      Enabled: 0 as libc::c_int,
                      Hidden: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             BUTSTATE{id: 7 as libc::c_int as UDWORD,
                      Enabled: 0 as libc::c_int,
                      Hidden: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             BUTSTATE{id: 6 as libc::c_int as UDWORD,
                      Enabled: 0 as libc::c_int,
                      Hidden: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             BUTSTATE{id: 9 as libc::c_int as UDWORD,
                      Enabled: 0 as libc::c_int,
                      Hidden: 0 as libc::c_int,};
         init
     }];
// Set the x,y members of a button widget initialiser given a reticule button index.
//
#[no_mangle]
pub unsafe extern "C" fn SetReticuleButPos(mut ButId: UWORD,
                                           mut sButInit: *mut W_BUTINIT) {
    if (ButId as libc::c_int) < 7 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"SetReticuleButPos : Bad button index\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (ButId as libc::c_int) < 7 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"hci.c\x00" as *const u8 as *const libc::c_char,
              142 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 18],
                                        &[libc::c_char; 18]>(b"SetReticuleButPos\x00")).as_ptr(),
              b"ButId < NUMRETBUTS\x00" as *const u8 as *const libc::c_char);
    };
    (*sButInit).x =
        (ReticuleOffsets[ButId as usize].x as libc::c_int + 0 as libc::c_int)
            as SWORD;
    (*sButInit).y =
        (ReticuleOffsets[ButId as usize].y as libc::c_int + 0 as libc::c_int)
            as SWORD;
}
#[no_mangle]
pub static mut InterfaceSnap: CURSORSNAP =
    CURSORSNAP{MaxSnaps: 0,
               NumSnaps: 0,
               CurrentSnap: 0,
               NewCurrentFormID: 0,
               NewCurrentID: 0,
               SnapCoords: 0 as *const SNAPCOORD as *mut SNAPCOORD,};
static mut ClosingObject: BOOL = 0 as libc::c_int;
static mut ClosingStats: BOOL = 0 as libc::c_int;
static mut keyButtonMapping: UDWORD = 0 as libc::c_int as UDWORD;
#[no_mangle]
pub static mut ClosingMessageView: BOOL = 0 as libc::c_int;
#[no_mangle]
pub static mut ClosingIntelMap: BOOL = 0 as libc::c_int;
#[no_mangle]
pub static mut ClosingOrder: BOOL = 0 as libc::c_int;
#[no_mangle]
pub static mut ClosingTrans: BOOL = 0 as libc::c_int;
#[no_mangle]
pub static mut ClosingTransCont: BOOL = 0 as libc::c_int;
#[no_mangle]
pub static mut ClosingTransDroids: BOOL = 0 as libc::c_int;
#[no_mangle]
pub static mut ReticuleUp: BOOL = 0 as libc::c_int;
#[no_mangle]
pub static mut Refreshing: BOOL = 0 as libc::c_int;
#[no_mangle]
pub static mut DisplayBuffer: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut displayBufferSize: UDWORD = 0;
/* Close strings */
static mut pCloseText: [STRING; 2] = [88, 0];
/* Player button strings */
static mut apPlayerText: [*mut STRING; 8] =
    [b"0\x00" as *const u8 as *const libc::c_char as *mut STRING,
     b"1\x00" as *const u8 as *const libc::c_char as *mut STRING,
     b"2\x00" as *const u8 as *const libc::c_char as *mut STRING,
     b"3\x00" as *const u8 as *const libc::c_char as *mut STRING,
     b"4\x00" as *const u8 as *const libc::c_char as *mut STRING,
     b"5\x00" as *const u8 as *const libc::c_char as *mut STRING,
     b"6\x00" as *const u8 as *const libc::c_char as *mut STRING,
     b"7\x00" as *const u8 as *const libc::c_char as *mut STRING];
static mut apPlayerTip: [*mut STRING; 8] =
    [b"Select Player 0\x00" as *const u8 as *const libc::c_char as
         *mut STRING,
     b"Select Player 1\x00" as *const u8 as *const libc::c_char as
         *mut STRING,
     b"Select Player 2\x00" as *const u8 as *const libc::c_char as
         *mut STRING,
     b"Select Player 3\x00" as *const u8 as *const libc::c_char as
         *mut STRING,
     b"Select Player 4\x00" as *const u8 as *const libc::c_char as
         *mut STRING,
     b"Select Player 5\x00" as *const u8 as *const libc::c_char as
         *mut STRING,
     b"Select Player 6\x00" as *const u8 as *const libc::c_char as
         *mut STRING,
     b"Select Player 7\x00" as *const u8 as *const libc::c_char as
         *mut STRING];
/* The widget screen */
#[no_mangle]
pub static mut psWScreen: *mut W_SCREEN =
    0 as *const W_SCREEN as *mut W_SCREEN;
/* the widget font */
//PROP_FONT	*psWFont;
#[no_mangle]
pub static mut WFont: libc::c_int = 0;
// Ivis Font ID.
/* The current player */
#[no_mangle]
pub static mut selectedPlayer: UDWORD = 0 as libc::c_int as UDWORD;
/* The flag to specify if the Intelligence screen is up */
//BOOL				intelMapUp = FALSE;
//two colours used for drawing the footprint outline for objects in 2D
#[no_mangle]
pub static mut outlineOK: UDWORD = 0;
#[no_mangle]
pub static mut outlineNotOK: UDWORD = 0;
//value gets set to colour used for drawing
#[no_mangle]
pub static mut outlineColour: UDWORD = 0;
#[no_mangle]
pub static mut outlineColour3D: UDWORD = 0;
// The last widget ID from widgRunScreen
#[no_mangle]
pub static mut intLastWidget: UDWORD = 0;
// /* The current mode of the widget screen */
//enum _int_mode
//{
//	INT_NORMAL,		// Standard mode (just the reticule)
//	INT_OPTION,		// Option screen
//	INT_EDITSTAT,	// Stat screen up for placing objects
//	INT_EDIT,		// Edit mode
//	INT_OBJECT,		// Object screen
//	INT_STAT,		// Object screen with stat screen
//	INT_DESIGN,		// Design screen
//	INT_INTELMAP,	// Intelligence Map
//	INT_ORDER,
//	//INT_TUTORIAL,	// Tutorial mode - message display
//} intMode;
#[no_mangle]
pub static mut intMode: INTMODE = INT_NORMAL;
#[no_mangle]
pub static mut editObjMode: _edit_obj_mode = IED_DROID;
#[no_mangle]
pub static mut editPosMode: _edit_pos_mode = IED_NOPOS;
#[no_mangle]
pub static mut objMode: _obj_mode = 0 as _obj_mode;
/* The current object list being used by the object screen */
static mut psObjList: *mut BASE_OBJECT =
    0 as *const BASE_OBJECT as *mut BASE_OBJECT;
/* functions to select and get stats from the current object list */
static mut objSelectFunc: OBJ_SELECT = None;
#[no_mangle]
pub static mut objGetStatsFunc: OBJ_GETSTATS = None;
static mut objSetStatsFunc: OBJ_SETSTATS = None;
/* Whether the objects that are on the object screen have changed this frame */
static mut objectsChanged: BOOL = 0;
/* The current stats list being used by the stats screen */
static mut ppsStatsList: *mut *mut BASE_STATS =
    0 as *const *mut BASE_STATS as *mut *mut BASE_STATS;
static mut numStatsListEntries: UDWORD = 0;
/* The selected object on the object screen when the stats screen is displayed */
static mut psObjSelected: *mut BASE_OBJECT =
    0 as *const BASE_OBJECT as *mut BASE_OBJECT;
/* The button ID of the objects stat when the stat screen is displayed */
#[no_mangle]
pub static mut objStatID: UDWORD = 0;
/* The button ID of an objects stat on the stat screen if it is locked down */
static mut statID: UDWORD = 0;
/* The stats for the current getStructPos */
static mut psPositionStats: *mut BASE_STATS =
    0 as *const BASE_STATS as *mut BASE_STATS;
/* The number of tabs on the object form (used by intObjDestroyed to tell whether */
/* the number of tabs has changed).                                             */
static mut objNumTabs: UWORD = 0;
/* The tab positions of the object form when the structure form is displayed */
static mut objMajor: UWORD = 0;
static mut objMinor: UWORD = 0;
/* The current map width and height in the new map edit boxes */
//static UDWORD			newMapWidth, newMapHeight;
/* Store a list of stats pointers from the main structure stats */
static mut apsStructStatsList: *mut *mut STRUCTURE_STATS =
    0 as *const *mut STRUCTURE_STATS as *mut *mut STRUCTURE_STATS;
/* Store a list of research pointers for topics that can be performed*/
static mut ppResearchList: *mut *mut RESEARCH =
    0 as *const *mut RESEARCH as *mut *mut RESEARCH;
/* Store a list of Template pointers for Droids that can be built */
#[no_mangle]
pub static mut apsTemplateList: *mut *mut DROID_TEMPLATE =
    0 as *const *mut DROID_TEMPLATE as *mut *mut DROID_TEMPLATE;
#[no_mangle]
pub static mut psCurrTemplate: *mut DROID_TEMPLATE =
    0 as *const DROID_TEMPLATE as *mut DROID_TEMPLATE;
/* Store a list of Feature pointers for features to be placed on the map */
static mut apsFeatureList: *mut *mut FEATURE_STATS =
    0 as *const *mut FEATURE_STATS as *mut *mut FEATURE_STATS;
/*Store a list of research indices which can be performed*/
//needs to be UWORD sized for Patches
static mut pList: *mut UWORD = 0 as *const UWORD as *mut UWORD;
static mut pSList: *mut UWORD = 0 as *const UWORD as *mut UWORD;
//static UBYTE			*pList;
//static UBYTE			*pSList;
/* Store a list of component stats pointers for the design screen */
#[no_mangle]
pub static mut numComponent: UDWORD = 0;
#[no_mangle]
pub static mut apsComponentList: *mut *mut COMP_BASE_STATS =
    0 as *const *mut COMP_BASE_STATS as *mut *mut COMP_BASE_STATS;
#[no_mangle]
pub static mut numExtraSys: UDWORD = 0;
#[no_mangle]
pub static mut apsExtraSysList: *mut *mut COMP_BASE_STATS =
    0 as *const *mut COMP_BASE_STATS as *mut *mut COMP_BASE_STATS;
//defined in HCI.h now
// store the objects that are being used for the object bar
//#define			MAX_OBJECTS		15//10 we need at least 15 for the 3 different types of factory
#[no_mangle]
pub static mut apsObjectList: *mut *mut BASE_OBJECT =
    0 as *const *mut BASE_OBJECT as *mut *mut BASE_OBJECT;
#[no_mangle]
pub static mut numObjects: SDWORD = 0;
//this list is used for sorting the objects - at the mo' this is just factories
#[no_mangle]
pub static mut apsListToOrder: *mut *mut BASE_OBJECT =
    0 as *const *mut BASE_OBJECT as *mut *mut BASE_OBJECT;
/* The button id of the component that is in the design */
//UDWORD			desCompID;
/* The button id of the droid template that has been locked down */
//UDWORD			droidTemplID;
/* Flags to check whether the power bars are currently on the screen */
static mut powerBarUp: BOOL = 0 as libc::c_int;
static mut StatsUp: BOOL = 0 as libc::c_int;
static mut psStatsScreenOwner: *mut BASE_OBJECT =
    0 as *const BASE_OBJECT as *mut BASE_OBJECT;
//Buffer to hold the 3D view for the Intelligence Screen
#[no_mangle]
pub static mut pIntelMapSurface: *mut iSurface =
    0 as *const iSurface as *mut iSurface;
/* pointer to hold the imd to use for a new template in the design screen */
//iIMDShape	*pNewDesignIMD = NULL;
/* The previous object for each object bar */
static mut apsPreviousObj: [*mut BASE_OBJECT; 22] =
    [0 as *const BASE_OBJECT as *mut BASE_OBJECT; 22];
/* The jump position for each object on the base bar */
static mut asJumpPos: [POINT; 22] = [POINT{x: 0, y: 0,}; 22];
// whether to reopen the build menu
//static BOOL				bReopenBuildMenu = FALSE;
// chnaged back to pre Mark Donald setting at Jim's request - AlexM
static mut bReopenBuildMenu: BOOL = 0 as libc::c_int;
static mut CurrentStruct: *mut STRUCTURE =
    0 as *const STRUCTURE as *mut STRUCTURE;
static mut CurrentStructType: SWORD = 0 as libc::c_int as SWORD;
static mut CurrentDroid: *mut DROID = 0 as *const DROID as *mut DROID;
static mut CurrentDroidType: SWORD = 0 as libc::c_int as SWORD;
/* Initialise the in game interface */
/* **************************GAME CODE ****************************/
/* Initialise the in game interface */
#[no_mangle]
pub unsafe extern "C" fn intInitialise() -> BOOL {
    //	UBYTE			*pFileBuffer;
//	UDWORD			fileSize;
    let mut comp: UDWORD = 0;
    let mut inc: UDWORD = 0;
    AllocateSnapBuffer(&mut InterfaceSnap, 64 as libc::c_int as UWORD);
    intInitialiseReticule();
    widgSetTipColour(psWScreen, 0 as libc::c_int as UBYTE,
                     164 as libc::c_int as UBYTE, 0 as libc::c_int as UBYTE);
    if GetGameMode() == 3 as libc::c_int as libc::c_uint {
        //		WidgSetAudio(WidgetAudioCallback,ID_SOUND_HILIGHTBUTTON,ID_SOUND_SELECT);
        WidgSetAudio(Some(WidgetAudioCallback as
                              unsafe extern "C" fn(_: libc::c_int) -> ()),
                     -(1 as libc::c_int) as SWORD,
                     ID_SOUND_SELECT as libc::c_int as SWORD);
    } else {
        //		WidgSetAudio(WidgetAudioCallback,FE_AUDIO_HILIGHTBUTTON,FE_AUDIO_SELECTBUT);
        WidgSetAudio(Some(WidgetAudioCallback as
                              unsafe extern "C" fn(_: libc::c_int) -> ()),
                     -(1 as libc::c_int) as SWORD,
                     ID_SOUND_SELECT as libc::c_int as SWORD);
    }
    /* Create storage for Structures that can be built */
    apsStructStatsList =
        memMallocRelease((::std::mem::size_of::<*mut STRUCTURE_STATS>() as
                              libc::c_ulong).wrapping_mul(80 as libc::c_int as
                                                              libc::c_uint))
            as *mut *mut STRUCTURE_STATS;
    if apsStructStatsList.is_null() {
        debug(LOG_ERROR,
              b"Out of memory\x00" as *const u8 as *const libc::c_char);
        abort();
    }
    //create the storage for Research topics - max possible size
    ppResearchList =
        memMallocRelease((::std::mem::size_of::<*mut RESEARCH>() as
                              libc::c_ulong).wrapping_mul(80 as libc::c_int as
                                                              libc::c_uint))
            as *mut *mut RESEARCH;
    if ppResearchList.is_null() {
        debug(LOG_ERROR,
              b"Unable to allocate memory for research list\x00" as *const u8
                  as *const libc::c_char);
        abort();
    }
    //create the list for the selected player
	//needs to be UWORD sized for Patches
    pList =
        memMallocRelease((::std::mem::size_of::<UWORD>() as
                              libc::c_ulong).wrapping_mul(80 as libc::c_int as
                                                              libc::c_uint))
            as *mut UWORD;
    pSList =
        memMallocRelease((::std::mem::size_of::<UWORD>() as
                              libc::c_ulong).wrapping_mul(80 as libc::c_int as
                                                              libc::c_uint))
            as *mut UWORD;
    //pList = (UBYTE *) MALLOC(sizeof (UBYTE) * MAXRESEARCH);
	//pSList = (UBYTE *) MALLOC(sizeof (UBYTE) * MAXRESEARCH);
    if pList.is_null() {
        debug(LOG_ERROR,
              b"Unable to allocate memory for research list\x00" as *const u8
                  as *const libc::c_char);
        abort();
    }
    if pSList.is_null() {
        debug(LOG_ERROR,
              b"Unable to allocate memory for sorted research list\x00" as
                  *const u8 as *const libc::c_char);
        abort();
    }
    /* Create storage for Templates that can be built */
    apsTemplateList =
        memMallocRelease((::std::mem::size_of::<*mut DROID_TEMPLATE>() as
                              libc::c_ulong).wrapping_mul(40 as libc::c_int as
                                                              libc::c_uint))
            as *mut *mut DROID_TEMPLATE;
    if apsTemplateList.is_null() {
        debug(LOG_ERROR,
              b"Unable to allocate memory for template list\x00" as *const u8
                  as *const libc::c_char);
        abort();
    }
    (GetGameMode()) == 3 as libc::c_int as libc::c_uint;
    /* Create storage for the feature list */
    apsFeatureList =
        memMallocRelease((::std::mem::size_of::<*mut FEATURE_STATS>() as
                              libc::c_ulong).wrapping_mul(40 as libc::c_int as
                                                              libc::c_uint))
            as *mut *mut FEATURE_STATS;
    if apsFeatureList.is_null() {
        debug(LOG_ERROR,
              b"Unable to allocate memory for feature list\x00" as *const u8
                  as *const libc::c_char);
        abort();
    }
    /* Create storage for the component list */
    apsComponentList =
        memMallocRelease((::std::mem::size_of::<*mut COMP_BASE_STATS>() as
                              libc::c_ulong).wrapping_mul(40 as libc::c_int as
                                                              libc::c_uint))
            as *mut *mut COMP_BASE_STATS;
    if apsComponentList.is_null() {
        debug(LOG_ERROR,
              b"Unable to allocate memory for component list\x00" as *const u8
                  as *const libc::c_char);
        abort();
    }
    /* Create storage for the extra systems list */
    apsExtraSysList =
        memMallocRelease((::std::mem::size_of::<*mut COMP_BASE_STATS>() as
                              libc::c_ulong).wrapping_mul(40 as libc::c_int as
                                                              libc::c_uint))
            as *mut *mut COMP_BASE_STATS;
    if apsExtraSysList.is_null() {
        debug(LOG_ERROR,
              b"Unable to allocate memory for extra systems list\x00" as
                  *const u8 as *const libc::c_char);
        abort();
    }
    // allocate the object list
    apsObjectList =
        memMallocRelease((::std::mem::size_of::<*mut BASE_OBJECT>() as
                              libc::c_ulong).wrapping_mul(15 as libc::c_int as
                                                              libc::c_uint))
            as *mut *mut BASE_OBJECT;
    if apsObjectList.is_null() {
        debug(LOG_ERROR,
              b"Out of memory\x00" as *const u8 as *const libc::c_char);
        abort();
    }
    //allocate the order list - ONLY SIZED FOR FACTORIES AT PRESENT!!
    apsListToOrder =
        memMallocRelease((::std::mem::size_of::<*mut BASE_OBJECT>() as
                              libc::c_ulong).wrapping_mul((3 as libc::c_int *
                                                               5 as
                                                                   libc::c_int)
                                                              as
                                                              libc::c_uint))
            as *mut *mut BASE_OBJECT;
    if apsListToOrder.is_null() {
        debug(LOG_ERROR,
              b"Out of memory\x00" as *const u8 as *const libc::c_char);
        abort();
    }
    /* Initialise the edit module */
    //	/* Load a font */
//	if (!loadFile("Serif.FNT", &pFileBuffer, &fileSize))
//	{
//		return FALSE;
//	}
//
//	if (!fontLoad(pFileBuffer, fileSize, &psWFont))
//	{
//		FREE(pFileBuffer);
//		return FALSE;
//	}
//	FREE(pFileBuffer);
    //	loadingScreenCallback();
    intInitialiseGraphics();
    //	loadingScreenCallback();
    WFont =
        iV_CreateFontIndirect(IntImages, AsciiLookup.as_mut_ptr(),
                              4 as libc::c_int);
    if widgCreateScreen(&mut psWScreen) == 0 {
        debug(LOG_ERROR,
              b"intInitialise: Couldn\'t create widget screen (Out of memory ?)\x00"
                  as *const u8 as *const libc::c_char);
        abort();
    }
    widgSetTipFont(psWScreen, WFont);
    if GetGameMode() == 3 as libc::c_int as libc::c_uint {
        if intAddReticule() == 0 {
            debug(LOG_ERROR,
                  b"intInitialise: Couldn\'t create reticule widgets (Out of memory ?)\x00"
                      as *const u8 as *const libc::c_char);
            abort();
        }
        if intAddPower() == 0 {
            debug(LOG_ERROR,
                  b"intInitialise: Couldn\'t create power Bar widget(Out of memory ?)\x00"
                      as *const u8 as *const libc::c_char);
            abort();
        }
    }
    /* Initialise the screen to be run */
    widgStartScreen(psWScreen);
    /* Note the current screen state */
    intMode = INT_NORMAL;
    objectsChanged = 0 as libc::c_int;
    //set the default colours to be used for drawing outlines in 2D
    outlineOK =
        pal_GetNearestColour(0xff as libc::c_int as uint8,
                             0xff as libc::c_int as uint8,
                             0xff as libc::c_int as uint8) as UDWORD;
    outlineNotOK =
        pal_GetNearestColour(0xff as libc::c_int as uint8,
                             0 as libc::c_int as uint8,
                             0 as libc::c_int as uint8) as UDWORD;
    //	loadingScreenCallback();
    /*initialise the messages 3D view buffer */
    pIntelMapSurface =
        setUpMapSurface(238 as libc::c_int as UDWORD,
                        169 as libc::c_int as UDWORD);
    //	loadingScreenCallback();
    /*Initialise the video playback buffer*/
    if seq_SetupVideoBuffers() == 0 {
        debug(LOG_ERROR,
              b"intInitialise: Unable to initialise video playback buffer\x00"
                  as *const u8 as *const libc::c_char);
        abort();
    }
    //	loadingScreenCallback();
    // reset the previous objects
	//memset(apsPreviousObj, 0, sizeof(apsPreviousObj));
    intResetPreviousObj();
    // reset the jump positions
    memset(asJumpPos.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<[POINT; 22]>() as libc::c_ulong);
    /* make demolish stat always available */
    if bInTutorial == 0 {
        comp = 0 as libc::c_int as UDWORD;
        while comp < numStructureStats {
            //if (!strcmp(asStructureStats[comp].pName, "Demolish Structure"))
            if (*asStructureStats.offset(comp as isize)).type_0 ==
                   REF_DEMOLISH as libc::c_int as libc::c_uint {
                inc = 0 as libc::c_int as UDWORD;
                while inc < 8 as libc::c_int as libc::c_uint {
                    *apStructTypeLists[inc as usize].offset(comp as isize) =
                        0x1 as libc::c_int as UBYTE;
                    inc = inc.wrapping_add(1)
                }
            }
            comp = comp.wrapping_add(1)
        }
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn intReopenBuild(mut reopen: BOOL) {
    bReopenBuildMenu = reopen;
}
#[no_mangle]
pub unsafe extern "C" fn intGetReopenBuild() -> BOOL {
    return bReopenBuildMenu;
}
//initialise all the previous obj - particularly useful for when go Off world!
//initialise all the previous obj - particularly useful for when go Off world!
#[no_mangle]
pub unsafe extern "C" fn intResetPreviousObj() {
    //make sure stats screen doesn't think it should be up
    StatsUp = 0 as libc::c_int;
    // reset the previous objects
    memset(apsPreviousObj.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<[*mut BASE_OBJECT; 22]>() as libc::c_ulong);
}
/* Shut down the in game interface */
/* Shut down the in game interface */
#[no_mangle]
pub unsafe extern "C" fn intShutDown() {
    //	widgEndScreen(psWScreen);
    widgReleaseScreen(psWScreen);
    //	fontFree(psWFont);
    ReleaseSnapBuffer(&mut InterfaceSnap);
    memFreeRelease(apsStructStatsList as *mut libc::c_void);
    apsStructStatsList = 0 as *mut *mut STRUCTURE_STATS;
    memFreeRelease(ppResearchList as *mut libc::c_void);
    ppResearchList = 0 as *mut *mut RESEARCH;
    memFreeRelease(pList as *mut libc::c_void);
    pList = 0 as *mut UWORD;
    memFreeRelease(pSList as *mut libc::c_void);
    pSList = 0 as *mut UWORD;
    memFreeRelease(apsTemplateList as *mut libc::c_void);
    apsTemplateList = 0 as *mut *mut DROID_TEMPLATE;
    memFreeRelease(apsFeatureList as *mut libc::c_void);
    apsFeatureList = 0 as *mut *mut FEATURE_STATS;
    memFreeRelease(apsComponentList as *mut libc::c_void);
    apsComponentList = 0 as *mut *mut COMP_BASE_STATS;
    memFreeRelease(apsExtraSysList as *mut libc::c_void);
    apsExtraSysList = 0 as *mut *mut COMP_BASE_STATS;
    memFreeRelease(apsObjectList as *mut libc::c_void);
    apsObjectList = 0 as *mut *mut BASE_OBJECT;
    memFreeRelease(apsListToOrder as *mut libc::c_void);
    apsListToOrder = 0 as *mut *mut BASE_OBJECT;
    //release the message buffer
    releaseMapSurface(pIntelMapSurface);
    //release the video buffers
    seq_ReleaseVideoBuffers();
    intDeleteGraphics();
    //obviously!
    ReticuleUp = 0 as libc::c_int;
}
static mut IntRefreshPending: BOOL = 0 as libc::c_int;
/* Refresh icons on the interface, without disturbing the layout. i.e. smartreset*/
// Set widget refresh pending flag.
//
#[no_mangle]
pub unsafe extern "C" fn intRefreshScreen() {
    IntRefreshPending = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn intSetCurrentCursorPosition(mut Snap:
                                                         *mut CURSORSNAP,
                                                     mut id: UDWORD) {
}
#[no_mangle]
pub unsafe extern "C" fn intIsRefreshing() -> BOOL { return Refreshing; }
// see if a delivery point is selected
#[no_mangle]
pub unsafe extern "C" fn intFindSelectedDelivPoint() -> *mut FLAG_POSITION {
    let mut psFlagPos: *mut FLAG_POSITION = 0 as *mut FLAG_POSITION;
    psFlagPos = apsFlagPosLists[selectedPlayer as usize];
    while !psFlagPos.is_null() {
        if (*psFlagPos).selected != 0 &&
               (*psFlagPos).type_0 as libc::c_uint ==
                   POS_DELIVERY as libc::c_int as libc::c_uint {
            return psFlagPos
        }
        psFlagPos = (*psFlagPos).psNext
    }
    return 0 as *mut FLAG_POSITION;
}
// Refresh widgets once per game cycle if pending flag is set.
//
#[no_mangle]
pub unsafe extern "C" fn intDoScreenRefresh() {
    let mut objMajor_0: UWORD = 0 as libc::c_int as UWORD;
    let mut objMinor_0: UWORD = 0 as libc::c_int as UWORD;
    let mut statMajor: UWORD = 0 as libc::c_int as UWORD;
    let mut statMinor: UWORD = 0 as libc::c_int as UWORD;
    let mut psFlag: *mut FLAG_POSITION = 0 as *mut FLAG_POSITION;
    if IntRefreshPending != 0 {
        Refreshing = 1 as libc::c_int;
        /*		if( (widgGetFromID(psWScreen,IDOBJ_FORM) != NULL)
		  && !(widgGetFromID(psWScreen,IDOBJ_FORM)->style & WIDG_HIDDEN)
		  )*/
        if (intMode as libc::c_uint ==
                INT_OBJECT as libc::c_int as libc::c_uint ||
                intMode as libc::c_uint ==
                    INT_STAT as libc::c_int as libc::c_uint ||
                intMode as libc::c_uint ==
                    INT_CMDORDER as libc::c_int as libc::c_uint ||
                intMode as libc::c_uint ==
                    INT_ORDER as libc::c_int as libc::c_uint ||
                intMode as libc::c_uint ==
                    INT_TRANSPORTER as libc::c_int as libc::c_uint) &&
               !widgGetFromID(psWScreen,
                              3000 as libc::c_int as UDWORD).is_null() &&
               (*widgGetFromID(psWScreen,
                               3000 as libc::c_int as UDWORD)).style &
                   0x8000 as libc::c_int as libc::c_uint == 0 {
            let mut StatsWasUp: BOOL = 0 as libc::c_int;
            let mut OrderWasUp: BOOL = 0 as libc::c_int;
            // If the stats form is up then remove it, but remember that it was up.
/*			if(widgGetFromID(psWScreen,IDSTAT_FORM) != NULL) {
				StatsWasUp = TRUE;
//				intRemoveStatsNoAnim();
			}*/
            if intMode as libc::c_uint ==
                   INT_STAT as libc::c_int as libc::c_uint &&
                   !widgGetFromID(psWScreen,
                                  4000 as libc::c_int as UDWORD).is_null() {
                StatsWasUp = 1 as libc::c_int
            }
            // store the current tab position
            if !widgGetFromID(psWScreen,
                              3500 as libc::c_int as UDWORD).is_null() {
                widgGetTabs(psWScreen, 3500 as libc::c_int as UDWORD,
                            &mut objMajor_0, &mut objMinor_0);
            }
            if StatsWasUp != 0 {
                widgGetTabs(psWScreen, 4004 as libc::c_int as UDWORD,
                            &mut statMajor, &mut statMinor);
            }
            // now make sure the stats screen isn't up
            if !widgGetFromID(psWScreen,
                              4000 as libc::c_int as UDWORD).is_null() {
                intRemoveStatsNoAnim();
            }
            if !psObjSelected.is_null() && (*psObjSelected).died != 0 {
                // refresh when unit dies
                psObjSelected = 0 as *mut BASE_OBJECT;
                objMinor_0 = 0 as libc::c_int as UWORD;
                objMajor_0 = objMinor_0;
                statMinor = 0 as libc::c_int as UWORD;
                statMajor = statMinor
            }
            // see if there was a delivery point being positioned
            psFlag = intFindSelectedDelivPoint();
            // see if the commander order screen is up
            if intMode as libc::c_uint ==
                   INT_CMDORDER as libc::c_int as libc::c_uint &&
                   !widgGetFromID(psWScreen,
                                  8000 as libc::c_int as UDWORD).is_null() {
                OrderWasUp = 1 as libc::c_int
            }
            let mut current_block_23: u64;
            //		if(widgGetFromID(psWScreen,IDOBJ_FORM) != NULL) {
	//			intRemoveObjectNoAnim();
	//		}
            //	DBPRINTF(("StatsWasUp %d\n",StatsWasUp);
            match objMode as libc::c_uint {
                19 | 20 => {
                    // The manufacture screen (factorys on bottom bar)
                    // The research screen
                    //intUpdateObject((BASE_OBJECT *)interfaceStructList(),NULL,StatsWasUp);
				//pass in the currently selected object
                    intUpdateObject(interfaceStructList() as *mut BASE_OBJECT,
                                    psObjSelected, StatsWasUp);
                    current_block_23 = 17788412896529399552;
                }
                16 | 21 => {
                    // the command droid screen
                    current_block_23 = 10634193843865723758;
                }
                17 | 18 => { current_block_23 = 10634193843865723758; }
                _ => { current_block_23 = 17788412896529399552; }
            }
            match current_block_23 {
                10634193843865723758 =>
                // Selecting a position for a new structure
                // Selecting a structure to demolish
                //intUpdateObject((BASE_OBJECT *)apsDroidLists[selectedPlayer],NULL,StatsWasUp);
				//pass in the currently selected object
                {
                    intUpdateObject(apsDroidLists[selectedPlayer as usize] as
                                        *mut BASE_OBJECT, psObjSelected,
                                    StatsWasUp);
                }
                _ => { }
            }
            // set the tabs again
            if !widgGetFromID(psWScreen,
                              3500 as libc::c_int as UDWORD).is_null() {
                widgSetTabs(psWScreen, 3500 as libc::c_int as UDWORD,
                            objMajor_0, objMinor_0);
            }
            if !widgGetFromID(psWScreen,
                              4004 as libc::c_int as UDWORD).is_null() {
                widgSetTabs(psWScreen, 4004 as libc::c_int as UDWORD,
                            statMajor, statMinor);
            }
            if !psFlag.is_null() {
                // need to restart the delivery point position
                StartDeliveryPosition(psFlag as *mut OBJECT_POSITION,
                                      0 as libc::c_int);
            }
            // make sure the commander order screen is in the right state
            if intMode as libc::c_uint ==
                   INT_CMDORDER as libc::c_int as libc::c_uint &&
                   OrderWasUp == 0 &&
                   !widgGetFromID(psWScreen,
                                  8000 as libc::c_int as UDWORD).is_null() {
                intRemoveOrderNoAnim();
                widgSetButtonState(psWScreen, statID,
                                   0 as libc::c_int as UDWORD);
            }
        }
        // Refresh the transporter interface.
        intRefreshTransporter();
        // Refresh the order interface.
        intRefreshOrder();
        Refreshing = 0 as libc::c_int
    }
    IntRefreshPending = 0 as libc::c_int;
}
/* Reset the widget screen to just the reticule */
/* Reset the widget screen to just the reticule */
#[no_mangle]
pub unsafe extern "C" fn intResetScreen(mut NoAnim: BOOL) {
    //	// Ensure driver mode is turned off.
//	StopDriverMode();
    if getWidgetsStatus() == 0 as libc::c_int { NoAnim = 1 as libc::c_int }
    if ReticuleUp != 0 {
        /* Reset the reticule buttons */
        widgSetButtonState(psWScreen, 9 as libc::c_int as UDWORD,
                           0 as libc::c_int as UDWORD);
        widgSetButtonState(psWScreen, 3 as libc::c_int as UDWORD,
                           0 as libc::c_int as UDWORD);
        widgSetButtonState(psWScreen, 4 as libc::c_int as UDWORD,
                           0 as libc::c_int as UDWORD);
        widgSetButtonState(psWScreen, 6 as libc::c_int as UDWORD,
                           0 as libc::c_int as UDWORD);
        widgSetButtonState(psWScreen, 5 as libc::c_int as UDWORD,
                           0 as libc::c_int as UDWORD);
        widgSetButtonState(psWScreen, 7 as libc::c_int as UDWORD,
                           0 as libc::c_int as UDWORD);
    }
    //19 #ifdef PSX
//19 	if(KeyboardIsActive()) {
//19 		intRemoveStringEntry();
//19 	}
//19 #endif
    /* Remove whatever extra screen was displayed */
    match intMode as libc::c_uint {
        1 => { intRemoveOptions(); }
        3 => {
            intStopStructPosition();
            if NoAnim != 0 {
                intRemoveStatsNoAnim();
            } else { intRemoveStats(); }
        }
        4 => {
            intStopStructPosition();
            if NoAnim != 0 {
                intRemoveObjectNoAnim();
            } else { intRemoveObject(); }
        }
        5 => {
            if NoAnim != 0 {
                intRemoveStatsNoAnim();
                intRemoveObjectNoAnim();
            } else { intRemoveStats(); intRemoveObject(); }
        }
        6 => {
            if NoAnim != 0 {
                intRemoveOrderNoAnim();
                intRemoveObjectNoAnim();
            } else { intRemoveOrder(); intRemoveObject(); }
        }
        9 => {
            if NoAnim != 0 {
                intRemoveOrderNoAnim();
            } else { intRemoveOrder(); }
        }
        10 => {
            if NoAnim != 0 {
                intCloseInGameOptionsNoAnim(1 as libc::c_int);
            } else {
                intCloseInGameOptions(0 as libc::c_int, 1 as libc::c_int);
            }
        }
        12 => {
            //		if(NoAnim)	{
            intRemoveMissionResultNoAnim();
        }
        13 => {
            if NoAnim != 0 {
                intCloseMultiMenuNoAnim();
            } else { intCloseMultiMenu(); }
        }
        7 => {
            intRemoveDesign();
            intHidePowerBar();
            if bInTutorial != 0 {
                eventFireCallbackTrigger(CALL_DESIGN_QUIT as libc::c_int as
                                             TRIGGER_TYPE);
            }
            // pc
            if bMultiPlayer == 0 { gameTimeStart(); }
        }
        8 => {
            //rotate the map back to previous view position on leaving the Intelligence Map
		//intelMapView(FALSE);
            if NoAnim != 0 {
                intRemoveIntelMapNoAnim();
            } else { intRemoveIntelMap(); }
            intHidePowerBar();
            if bMultiPlayer == 0 { gameTimeStart(); }
        }
        11 => {
            /*	case INT_TUTORIAL:
		//remove 3dView
		intRemoveMessageView();

#ifndef PSX
		if(!bMultiPlayer)
		{
#endif
			gameTimeStart();
#ifndef PSX
		}
#endif
		break;*/
            if NoAnim != 0 {
                intRemoveTransNoAnim();
            } else { intRemoveTrans(); }
        }
        _ => { }
    }
    intMode = INT_NORMAL;
    //clearSel() sets IntRefreshPending = TRUE by calling intRefreshScreen() but if we're doing this then we won't need to refresh - hopefully!
    IntRefreshPending = 0 as libc::c_int;
}
// calulate the center world coords for a structure stat given
// top left tile coords
#[no_mangle]
pub unsafe extern "C" fn intCalcStructCenter(mut psStats:
                                                 *mut STRUCTURE_STATS,
                                             mut tilex: UDWORD,
                                             mut tiley: UDWORD,
                                             mut pcx: *mut UDWORD,
                                             mut pcy: *mut UDWORD) {
    let mut width: SDWORD = 0;
    let mut height: SDWORD = 0;
    width =
        (*psStats).baseWidth.wrapping_mul(128 as libc::c_int as libc::c_uint)
            as SDWORD;
    height =
        (*psStats).baseBreadth.wrapping_mul(128 as libc::c_int as
                                                libc::c_uint) as SDWORD;
    *pcx =
        tilex.wrapping_mul(128 as libc::c_int as
                               libc::c_uint).wrapping_add((width /
                                                               2 as
                                                                   libc::c_int)
                                                              as
                                                              libc::c_uint);
    *pcy =
        tiley.wrapping_mul(128 as libc::c_int as
                               libc::c_uint).wrapping_add((height /
                                                               2 as
                                                                   libc::c_int)
                                                              as
                                                              libc::c_uint);
}
// Returns TRUE if the widget specified by id should filter input.
//
#[no_mangle]
pub unsafe extern "C" fn AllowWidgetIntercept(mut id: UDWORD) -> BOOL {
    match id {
        102 | 11000 | 11001 | 11010 => { return 0 as libc::c_int }
        _ => { }
    }
    return 1 as libc::c_int;
}
/* Run the widgets for the in game interface */
/* Run the widgets for the in game interface */
#[no_mangle]
pub unsafe extern "C" fn intRunWidgets() -> INT_RETVAL {
    let mut retID: UDWORD = 0;
    let mut retCode: INT_RETVAL = INT_NONE;
    let mut quitting: BOOL = 0 as libc::c_int;
    let mut structX: UDWORD = 0;
    let mut structY: UDWORD = 0;
    let mut structX2: UDWORD = 0;
    let mut structY2: UDWORD = 0;
    let mut objMajor_0: UWORD = 0;
    let mut objMinor_0: UWORD = 0;
    let mut psStructure: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut i: SDWORD = 0;
    let mut widgOverID: UDWORD = 0;
    intDoScreenRefresh();
    // If the widgets are turned off then why bother to process them?
//	if(!widgetsOn) {
//		return INT_NONE;
//	}
    /* Update the object list if necessary */
    if intMode as libc::c_uint == INT_OBJECT as libc::c_int as libc::c_uint ||
           intMode as libc::c_uint == INT_STAT as libc::c_int as libc::c_uint
           ||
           intMode as libc::c_uint ==
               INT_CMDORDER as libc::c_int as libc::c_uint {
        /*		switch (objMode)
		{
		case IOBJ_BUILD:
			psObjList = (BASE_OBJECT *)apsDroidLists[selectedPlayer];
			break;
		case IOBJ_MANUFACTURE:
			psObjList = (BASE_OBJECT *)apsStructLists[selectedPlayer];
			break;
		case IOBJ_RESEARCH:
			psObjList = (BASE_OBJECT *)apsStructLists[selectedPlayer];
			break;
		}*/
		// see if there is a dead object in the list
        i = 0 as libc::c_int;
        while i < numObjects {
            if !(*apsObjectList.offset(i as isize)).is_null() &&
                   (**apsObjectList.offset(i as isize)).died != 0 {
                intObjectDied((i + 3002 as libc::c_int) as UDWORD);
                let ref mut fresh0 = *apsObjectList.offset(i as isize);
                *fresh0 = 0 as *mut BASE_OBJECT
            }
            i += 1
        }
    }
    /* Update the previous object array */
    i = 0 as libc::c_int;
    while i < IOBJ_MAX as libc::c_int {
        if !apsPreviousObj[i as usize].is_null() &&
               (*apsPreviousObj[i as usize]).died != 0 {
            apsPreviousObj[i as usize] = 0 as *mut BASE_OBJECT
        }
        i += 1
    }
    /* if objects in the world have changed, may have to update the interface */
    if objectsChanged != 0 {
        /* The objects on the object screen have changed */
        if intMode as libc::c_uint ==
               INT_OBJECT as libc::c_int as libc::c_uint {
            if !widgGetFromID(psWScreen,
                              3500 as libc::c_int as UDWORD).is_null() {
            } else {
                debug(LOG_ERROR,
                      b"No object form\n\x00" as *const u8 as
                          *const libc::c_char);
            };
            if !widgGetFromID(psWScreen,
                              3500 as libc::c_int as UDWORD).is_null() {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"hci.c\x00" as *const u8 as *const libc::c_char,
                      1418 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 14],
                                                &[libc::c_char; 14]>(b"intRunWidgets\x00")).as_ptr(),
                      b"widgGetFromID(psWScreen,IDOBJ_TABFORM) != NULL\x00" as
                          *const u8 as *const libc::c_char);
            };
            /* Remove the old screen */
            widgGetTabs(psWScreen, 3500 as libc::c_int as UDWORD,
                        &mut objMajor_0, &mut objMinor_0);
            intRemoveObject();
            /* Add the new screen */
            match objMode as libc::c_uint {
                16 | 17 => { intAddBuild(0 as *mut DROID); }
                19 => { intAddManufacture(0 as *mut STRUCTURE); }
                20 => { intAddResearch(0 as *mut STRUCTURE); }
                _ => { }
            }
            /* Reset the tabs on the object screen */
            if objMajor_0 as libc::c_int > objNumTabs as libc::c_int {
                widgSetTabs(psWScreen, 3500 as libc::c_int as UDWORD,
                            objNumTabs, objMinor_0);
            } else {
                widgSetTabs(psWScreen, 3500 as libc::c_int as UDWORD,
                            objMajor_0, objMinor_0);
            }
        } else {
            (intMode as libc::c_uint) ==
                INT_STAT as libc::c_int as libc::c_uint;
        }
    }
    objectsChanged = 0 as libc::c_int;
    if bLoadSaveUp != 0 {
        if runLoadSave(1 as libc::c_int) != 0 {
            // check for file name.
            if strlen(sRequestResult.as_mut_ptr()) != 0 {
                debug(LOG_ERROR,
                      b"Returned %s\x00" as *const u8 as *const libc::c_char,
                      sRequestResult.as_mut_ptr());
                if bRequestLoad != 0 {
                    //					loadGame(sRequestResult,TRUE,FALSE,TRUE);
                    loopMissionState = LMS_LOADGAME;
                    strcpy(saveGameName.as_mut_ptr(),
                           sRequestResult.as_mut_ptr());
                } else if saveGame(sRequestResult.as_mut_ptr(),
                                   GTYPE_SAVE_START as libc::c_int) != 0 {
                    addConsoleMessage(strresGetString(psStringRes,
                                                      STR_GAME_SAVED as
                                                          libc::c_int as
                                                          UDWORD),
                                      LEFT_JUSTIFY);
                    if !widgGetFromID(psWScreen,
                                      11006 as libc::c_int as
                                          UDWORD).is_null() {
                        widgDelete(psWScreen, 11006 as libc::c_int as UDWORD);
                    }
                } else {
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"intRunWidgets: saveGame Failed\x00" as
                                  *const u8 as *const libc::c_char);
                    };
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                                  *const u8 as *const libc::c_char,
                              b"hci.c\x00" as *const u8 as
                                  *const libc::c_char, 1485 as libc::c_int,
                              (*::std::mem::transmute::<&[u8; 14],
                                                        &[libc::c_char; 14]>(b"intRunWidgets\x00")).as_ptr(),
                              b"FALSE\x00" as *const u8 as
                                  *const libc::c_char);
                    };
                    deleteSaveGame(sRequestResult.as_mut_ptr());
                }
            }
        }
    } else if InGameOpUp != 0 { intRunInGameOptions(); }
    if MissionResUp != 0 { intRunMissionResult(); }
    /* Run the current set of widgets */
    if bLoadSaveUp == 0 {
        retID = widgRunScreen(psWScreen)
    } else { retID = 0 as libc::c_int as UDWORD }
    /* We may need to trigger widgets with a key press */
    if keyButtonMapping != 0 {
        /* Set the appropriate id */
        retID = keyButtonMapping;
        /* Clear it so it doesn't trigger next time around */
        keyButtonMapping = 0 as libc::c_int as UDWORD
    }
    intLastWidget = retID;
    if bInTutorial != 0 && retID != 0 as libc::c_int as libc::c_uint {
        eventFireCallbackTrigger(CALL_BUTTON_PRESSED as libc::c_int as
                                     TRIGGER_TYPE);
    }
    /* Extra code for the power bars to deal with the shadow */
    if powerBarUp != 0 { intRunPower(); }
    if StatsUp != 0 { intRunStats(); }
    if OrderUp != 0 { intRunOrder(); }
    if MultiMenuUp != 0 { intRunMultiMenu(); }
    if retID >= 12000 as libc::c_int as libc::c_uint &&
           retID <= 12019 as libc::c_int as libc::c_uint {
        processProximityButtons(retID);
        return INT_NONE
    }
    /* Extra code for the design screen to deal with the shadow bar graphs */
    if intMode as libc::c_uint == INT_DESIGN as libc::c_int as libc::c_uint {
        intRunDesign();
    }
    let mut current_block_153: u64;
    /* Deal with any clicks */
    match retID {
        0 => { current_block_153 = 145651165234646754; }
        2 => {
            /* ****************  Reticule buttons  *****************/
            intResetScreen(0 as libc::c_int);
            //		widgSetButtonState(psWScreen, IDRET_OPTIONS, WBUT_CLICKLOCK);	// commented out by ajl, now command droids menu
            intAddOptions();
            intMode = INT_OPTION;
            current_block_153 = 145651165234646754;
        }
        9 => {
            intResetScreen(0 as libc::c_int);
            widgSetButtonState(psWScreen, 9 as libc::c_int as UDWORD,
                               0x4 as libc::c_int as UDWORD);
            intAddCommand(0 as *mut DROID);
            current_block_153 = 145651165234646754;
        }
        3 => {
            //intResetScreen(FALSE);
            intResetScreen(1 as libc::c_int);
            widgSetButtonState(psWScreen, 3 as libc::c_int as UDWORD,
                               0x4 as libc::c_int as UDWORD);
            intAddBuild(0 as *mut DROID);
            //		intMode = INT_OBJECT;
            current_block_153 = 145651165234646754;
        }
        4 => {
            //		OrderDroidsToEmbark();
//		missionDestroyObjects();
		//intResetScreen(FALSE);
            intResetScreen(1 as libc::c_int);
            widgSetButtonState(psWScreen, 4 as libc::c_int as UDWORD,
                               0x4 as libc::c_int as UDWORD);
            intAddManufacture(0 as *mut STRUCTURE);
            //		intMode = INT_OBJECT;
            current_block_153 = 145651165234646754;
        }
        5 => {
            //intResetScreen(FALSE);
            intResetScreen(1 as libc::c_int);
            widgSetButtonState(psWScreen, 5 as libc::c_int as UDWORD,
                               0x4 as libc::c_int as UDWORD);
            intAddResearch(0 as *mut STRUCTURE);
            //		intMode = INT_OBJECT;
            current_block_153 = 145651165234646754;
        }
        6 => {
            //		intResetScreen(FALSE);
//		//check if RMB was clicked
            if widgGetButtonKey(psWScreen) & 2 as libc::c_int as libc::c_uint
                   != 0 {
                //set the current message to be the last non-proximity message added
                setCurrentMsg();
                setMessageImmediate(1 as libc::c_int);
            } else { psCurrentMsg = 0 as *mut MESSAGE }
            addIntelScreen();
            current_block_153 = 145651165234646754;
        }
        7 => {
            intResetScreen(1 as libc::c_int);
            widgSetButtonState(psWScreen, 7 as libc::c_int as UDWORD,
                               0x4 as libc::c_int as UDWORD);
            /*add the power bar - for looks! */
            intShowPowerBar();
            intAddDesign(0 as libc::c_int);
            intMode = INT_DESIGN;
            current_block_153 = 145651165234646754;
        }
        8 => {
            intResetScreen(0 as libc::c_int);
            psCurrentMsg = 0 as *mut MESSAGE;
            current_block_153 = 145651165234646754;
        }
        11012 => {
            /*Transporter button pressed - OFFWORLD Mission Maps ONLY *********/
            addTransporterInterface(0 as *mut DROID, 1 as libc::c_int);
            current_block_153 = 145651165234646754;
        }
        9010 => {
            processLaunchTransporter();
            current_block_153 = 145651165234646754;
        }
        11007 => {
            // mission quit
            current_block_153 = 12582106546670454683;
        }
        10502 | 1031 => { current_block_153 = 12582106546670454683; }
        3500 => {
            // Process form tab clicks.
            // If tab clicked on in object screen then refresh all rendered buttons.
            RefreshObjectButtons();
            RefreshTopicButtons();
            current_block_153 = 145651165234646754;
        }
        4004 => {
            // If tab clicked on in stats screen then refresh all rendered buttons.
            RefreshStatsButtons();
            current_block_153 = 145651165234646754;
        }
        5002 => {
            // If tab clicked on in design template screen then refresh all rendered buttons.
            RefreshStatsButtons();
            current_block_153 = 145651165234646754;
        }
        5003 => {
            // If tab clicked on in design component screen then refresh all rendered buttons.
            RefreshObjectButtons();
            RefreshSystem0Buttons();
            current_block_153 = 145651165234646754;
        }
        _ => {
            /* Default case passes remaining IDs to appropriate function */
            match intMode as libc::c_uint {
                1 => { intProcessOptions(retID); }
                3 => { intProcessEditStats(retID); }
                5 | 6 | 4 => {
                    /* In stat mode ids get passed to processObject
			 * and then through to processStats
			 */
			// NO BREAK HERE! THIS IS CORRECT;
                    intProcessObject(retID);
                }
                9 => { intProcessOrder(retID); }
                12 => { intProcessMissionResult(retID); }
                10 => { intProcessInGameOptions(retID); }
                13 => { intProcessMultiMenu(retID); }
                7 => { intProcessDesign(retID); }
                8 => { intProcessIntelMap(retID); }
                11 => {
                    /*case INT_TUTORIAL:
			intProcessMessageView(retID);
			break;*/
                    intProcessTransporter(retID);
                }
                0 => { }
                _ => {
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"intRunWidgets: unknown interface mode\x00" as
                                  *const u8 as *const libc::c_char);
                    };
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                                  *const u8 as *const libc::c_char,
                              b"hci.c\x00" as *const u8 as
                                  *const libc::c_char, 1723 as libc::c_int,
                              (*::std::mem::transmute::<&[u8; 14],
                                                        &[libc::c_char; 14]>(b"intRunWidgets\x00")).as_ptr(),
                              b"FALSE\x00" as *const u8 as
                                  *const libc::c_char);
                    };
                }
            }
            current_block_153 = 145651165234646754;
        }
    }
    match current_block_153 {
        12582106546670454683 =>
        // esc quit confrim
        // options screen quit
        {
            intResetScreen(0 as libc::c_int);
            //clearMissionWidgets();
            quitting = 1 as libc::c_int
        }
        _ => { }
    }
    if quitting == 0 && retID == 0 {
        if !(intMode as libc::c_uint ==
                 INT_EDIT as libc::c_int as libc::c_uint) {
            if (intMode as libc::c_uint ==
                    INT_OBJECT as libc::c_int as libc::c_uint ||
                    intMode as libc::c_uint ==
                        INT_STAT as libc::c_int as libc::c_uint) &&
                   objMode as libc::c_uint ==
                       IOBJ_BUILDSEL as libc::c_int as libc::c_uint {
                // See if a position for the structure has been found
                if display3D != 0 &&
                       found3DBuildLocTwo(&mut structX, &mut structY,
                                          &mut structX2, &mut structY2) != 0 {
                    // check if it's a straight line.
                    if structX == structX2 || structY == structY2 {
                        // Send the droid off to build the structure assuming the droid
					// can get to the location chosen
                        structX =
                            (structX <<
                                 7 as
                                     libc::c_int).wrapping_add((128 as
                                                                    libc::c_int
                                                                    /
                                                                    2 as
                                                                        libc::c_int)
                                                                   as
                                                                   libc::c_uint);
                        structY =
                            (structY <<
                                 7 as
                                     libc::c_int).wrapping_add((128 as
                                                                    libc::c_int
                                                                    /
                                                                    2 as
                                                                        libc::c_int)
                                                                   as
                                                                   libc::c_uint);
                        structX2 =
                            (structX2 <<
                                 7 as
                                     libc::c_int).wrapping_add((128 as
                                                                    libc::c_int
                                                                    /
                                                                    2 as
                                                                        libc::c_int)
                                                                   as
                                                                   libc::c_uint);
                        structY2 =
                            (structY2 <<
                                 7 as
                                     libc::c_int).wrapping_add((128 as
                                                                    libc::c_int
                                                                    /
                                                                    2 as
                                                                        libc::c_int)
                                                                   as
                                                                   libc::c_uint);
                        if !(IsPlayerStructureLimitReached(selectedPlayer) !=
                                 0) {
                            // Set the droid order
                            if intNumSelectedDroids(DROID_CONSTRUCT as
                                                        libc::c_int as UDWORD)
                                   == 0 as libc::c_int &&
                                   intNumSelectedDroids(DROID_CYBORG_CONSTRUCT
                                                            as libc::c_int as
                                                            UDWORD) ==
                                       0 as libc::c_int &&
                                   !psObjSelected.is_null() {
                                orderDroidStatsTwoLoc(psObjSelected as
                                                          *mut DROID,
                                                      DORDER_LINEBUILD,
                                                      psPositionStats,
                                                      structX, structY,
                                                      structX2, structY2);
                            } else {
                                orderSelectedStatsTwoLoc(selectedPlayer,
                                                         DORDER_LINEBUILD,
                                                         psPositionStats,
                                                         structX, structY,
                                                         structX2, structY2,
                                                         ctrlShiftDown());
                            }
                        }
                    }
                    // put the build menu up again after the structure position has been chosen
				//or ctrl/shift is down and we're queing the build orders
                    if bReopenBuildMenu != 0 || ctrlShiftDown() != 0 {
                        intAddBuild(0 as *mut DROID);
                    } else {
                        // Clear the object screen
                        intResetScreen(0 as libc::c_int);
                    }
                } else if intGetStructPosition(&mut structX, &mut structY) !=
                              0 {
                    //found building
                    //check droid hasn't died
                    if psObjSelected.is_null() || (*psObjSelected).died == 0 {
                        let mut CanBuild: BOOL = 1 as libc::c_int;
                        // Send the droid off to build the structure assuming the droid
					// can get to the location chosen
//					structX = structX << TILE_SHIFT;
//					structY = structY << TILE_SHIFT;
                        intCalcStructCenter(psPositionStats as
                                                *mut STRUCTURE_STATS, structX,
                                            structY, &mut structX,
                                            &mut structY);
                        // Don't allow derrick to be built on burning ground.
                        if (*(psPositionStats as *mut STRUCTURE_STATS)).type_0
                               ==
                               REF_RESOURCE_EXTRACTOR as libc::c_int as
                                   libc::c_uint {
                            if fireOnLocation(structX, structY) != 0 {
                                AddDerrickBurningMessage();
                                CanBuild = 0 as libc::c_int
                            }
                        }
                        if CanBuild != 0 {
                            if !(IsPlayerStructureLimitReached(selectedPlayer)
                                     != 0) {
                                // Set the droid order
                                if intNumSelectedDroids(DROID_CONSTRUCT as
                                                            libc::c_int as
                                                            UDWORD) ==
                                       0 as libc::c_int &&
                                       intNumSelectedDroids(DROID_CYBORG_CONSTRUCT
                                                                as libc::c_int
                                                                as UDWORD) ==
                                           0 as libc::c_int &&
                                       !psObjSelected.is_null() {
                                    orderDroidStatsLoc(psObjSelected as
                                                           *mut DROID,
                                                       DORDER_BUILD,
                                                       psPositionStats,
                                                       structX, structY);
                                } else {
                                    orderSelectedStatsLoc(selectedPlayer,
                                                          DORDER_BUILD,
                                                          psPositionStats,
                                                          structX, structY,
                                                          ctrlShiftDown());
                                }
                            }
                        }
                    }
                    intResetScreen(0 as libc::c_int);
                }
            } else if intMode as libc::c_uint ==
                          INT_EDITSTAT as libc::c_int as libc::c_uint &&
                          editPosMode as libc::c_uint ==
                              IED_POS as libc::c_int as libc::c_uint {
                // CHECK THIS ON PSX (PD070199)
//				if(!driveModeActive()) {
//					((DROID *)psObjSelected)->selected = FALSE;//deselect the droid if build command successful
//					DeSelectDroid((DROID*)psObjSelected);
//				}
                // put the build menu up again after the structure position has been chosen
                //or ctrl/shift is down and we're queuing the build orders
                // Clear the object screen
                /* Directly positioning some type of object */
                if intGetStructPosition(&mut structX, &mut structY) != 0 {
                    /* See what type of thing is being put down */
                    if (*psPositionStats).ref_0 >=
                           0xd0000 as libc::c_int as libc::c_uint &&
                           (*psPositionStats).ref_0 <
                               (0xd0000 as libc::c_int +
                                    0x10000 as libc::c_int) as libc::c_uint {
                        intCalcStructCenter(psPositionStats as
                                                *mut STRUCTURE_STATS, structX,
                                            structY, &mut structX,
                                            &mut structY);
                        psStructure =
                            buildStructure(psPositionStats as
                                               *mut STRUCTURE_STATS, structX,
                                           structY, selectedPlayer,
                                           0 as libc::c_int);
                        if !psStructure.is_null() {
                            (*psStructure).status =
                                SS_BUILT as libc::c_int as UBYTE;
                            buildingComplete(psStructure);
                            /*if (psStructure->pStructureType->type == REF_POWER_GEN)
						{
							//initPlayerPower();
							capacityUpdate(psStructure);
						}
						else if (psStructure->pStructureType->type == REF_RESOURCE_EXTRACTOR OR
							psStructure->pStructureType->type == REF_HQ)
						{
							//initPlayerPower();
							extractedPowerUpdate(psStructure);
						}*/
                        }
                    } else if (*psPositionStats).ref_0 >=
                                  0x100000 as libc::c_int as libc::c_uint &&
                                  (*psPositionStats).ref_0 <
                                      (0x100000 as libc::c_int +
                                           0x10000 as libc::c_int) as
                                          libc::c_uint {
                        buildFeature(psPositionStats as *mut FEATURE_STATS,
                                     structX << 7 as libc::c_int,
                                     structY << 7 as libc::c_int,
                                     0 as libc::c_int);
                    } else if (*psPositionStats).ref_0 >=
                                  0xc0000 as libc::c_int as libc::c_uint &&
                                  (*psPositionStats).ref_0 <
                                      (0xc0000 as libc::c_int +
                                           0x10000 as libc::c_int) as
                                          libc::c_uint {
                        psDroid =
                            buildDroid(psPositionStats as *mut DROID_TEMPLATE,
                                       (structX <<
                                            7 as
                                                libc::c_int).wrapping_add((128
                                                                               as
                                                                               libc::c_int
                                                                               /
                                                                               2
                                                                                   as
                                                                                   libc::c_int)
                                                                              as
                                                                              libc::c_uint),
                                       (structY <<
                                            7 as
                                                libc::c_int).wrapping_add((128
                                                                               as
                                                                               libc::c_int
                                                                               /
                                                                               2
                                                                                   as
                                                                                   libc::c_int)
                                                                              as
                                                                              libc::c_uint),
                                       selectedPlayer, 0 as libc::c_int);
                        if !psDroid.is_null() {
                            addDroid(psDroid, apsDroidLists.as_mut_ptr());
                        }
                    }
                    editPosMode = IED_NOPOS
                }
            }
        }
    }
    widgOverID = widgGetMouseOver(psWScreen);
    retCode = INT_NONE;
    if quitting != 0 {
        retCode = INT_QUIT
    } else if retID != 0 ||
                  intMode as libc::c_uint ==
                      INT_EDIT as libc::c_int as libc::c_uint ||
                  intMode as libc::c_uint ==
                      INT_MISSIONRES as libc::c_int as libc::c_uint ||
                  widgOverID != 0 as libc::c_int as libc::c_uint {
        retCode = INT_INTERCEPT
    }
    //the return code has been superceded by the different pause states
	/*else if (intMode == INT_DESIGN)
	{
		retCode = INT_FULLSCREENPAUSE;
	}*/
	//else if (intMode == INT_INTELMAP)
	//{
		//Pause game in Intel Screen now - AB 18/03/98
		//retCode = INT_FULLSCREENPAUSE;
		/*//if 3D View is up don't want scroll
		psWidget = widgGetFromID(psWScreen,IDINTMAP_MSGVIEW);
		if (psWidget)
		{
			retCode = INT_INTELNOSCROLL;
		}
		else
		{
			retCode = INT_INTELPAUSE;
		}*/
	//}
	/*else if (intMode == INT_TUTORIAL)
	{
		retCode = INT_INTELNOSCROLL;
	}*/
    //#ifndef PSX
//	else if (retID || intMode == INT_EDIT || intMode == INT_MISSIONRES || widgGetMouseOver(psWScreen) != 0)
//	{
//		retCode = INT_INTERCEPT;
//	}
//#else
//	else if (retID || intMode == INT_MISSIONRES || widgGetMouseOver(psWScreen) != 0)
//	{
//		retCode = INT_INTERCEPT;
//	}
//#endif
    if (testPlayerHasLost() != 0 ||
            testPlayerHasWon() != 0 && bMultiPlayer == 0) &&
           intMode as libc::c_uint !=
               INT_MISSIONRES as libc::c_int as libc::c_uint &&
           getDebugMappingStatus() == 0 {
        debug(LOG_ERROR,
              b"PlayerHasLost Or Won\n\x00" as *const u8 as
                  *const libc::c_char);
        intResetScreen(1 as libc::c_int);
        retCode = INT_QUIT;
        quitting = 1 as libc::c_int
    }
    return retCode;
}
/* Get  and validate the new map size from the options screen */
/*static void intGetMapSize(void)
{
	SDWORD	editWidth, editHeight;
	STRING	*pStr;
	STRING	aText[WIDG_MAXSTR];
	UDWORD	i, tmp, bitCount;
	BOOL	widthChanged=FALSE, heightChanged=FALSE;

	// Get the new width
	pStr = widgGetString(psWScreen, IDOPT_MAPWIDTH);
	if (isdigit(*pStr))
	{
		// There is a number in the string
		sscanf(pStr, "%d", &editWidth);
	}
	else
	{
		// No number in the string, restore the old value
		editWidth = newMapWidth;
		widthChanged = TRUE;
	}

	// Get the new height
	pStr = widgGetString(psWScreen, IDOPT_MAPHEIGHT);
	if (isdigit(*pStr))
	{
		// There is a number in the string
		sscanf(pStr, "%d", &editHeight);
	}
	else
	{
		// No number in the string, restore the old value
		editHeight = newMapHeight;
		heightChanged = TRUE;
	}

	// now validate the sizes
	if (editWidth <= 0 || editWidth > MAP_MAXWIDTH)
	{
		editWidth = newMapWidth;
		widthChanged = TRUE;
	}
	else
	{
		// Check it is a power of 2
		bitCount = 0;
		tmp = editWidth;
		for(i=0; i<32; i++)
		{
			if (tmp & 1)
			{
				bitCount ++;
			}
			tmp = tmp >> 1;
		}
		if (bitCount != 1)
		{
			editWidth = newMapWidth;
			widthChanged = TRUE;
		}
	}
	if (editHeight <= 0 || editHeight > MAP_MAXHEIGHT)
	{
		editHeight = newMapHeight;
		heightChanged = TRUE;
	}
	else
	{
		// Check it is a power of 2
		bitCount = 0;
		tmp = editHeight;
		for(i=0; i<32; i++)
		{
			if (tmp & 1)
			{
				bitCount ++;
			}
			tmp = tmp >> 1;
		}
		if (bitCount != 1)
		{
			editHeight = newMapHeight;
			heightChanged = TRUE;
		}
	}

	// Store the new size
	newMapWidth = editWidth;
	newMapHeight = editHeight;

	// Syncronise the edit boxes if necessary
	if (widthChanged)
	{
		sprintf(aText, "%d", newMapWidth);
		widgSetString(psWScreen, IDOPT_MAPWIDTH, aText);
	}
	if (heightChanged)
	{
		sprintf(aText, "%d", newMapHeight);
		widgSetString(psWScreen, IDOPT_MAPHEIGHT, aText);
	}
}*/
#[no_mangle]
pub unsafe extern "C" fn intIncrementPlayerNumber() {
    intResetScreen(0 as libc::c_int);
    selectedPlayer = selectedPlayer.wrapping_add(1);
    if selectedPlayer >= 8 as libc::c_int as libc::c_uint {
        selectedPlayer = 0 as libc::c_int as UDWORD
    }
    sprintf(ConsoleString.as_mut_ptr(),
            b"New Player ID : %d\x00" as *const u8 as *const libc::c_char,
            selectedPlayer);
    addConsoleMessage(ConsoleString.as_mut_ptr(), DEFAULT_JUSTIFY);
}
#[no_mangle]
pub unsafe extern "C" fn intAddEditDroids() {
    let mut i: UDWORD = 0;
    let mut psTempl: *mut DROID_TEMPLATE = 0 as *mut DROID_TEMPLATE;
    i = 0 as libc::c_int as UDWORD;
    psTempl = apsDroidTemplates[selectedPlayer as usize];
    while !psTempl.is_null() && i < 40 as libc::c_int as libc::c_uint {
        let ref mut fresh1 = *apsTemplateList.offset(i as isize);
        *fresh1 = psTempl;
        psTempl = (*psTempl).psNext;
        i = i.wrapping_add(1)
    }
    ppsStatsList = apsTemplateList as *mut *mut BASE_STATS;
    objMode = IOBJ_MANUFACTURE;
    intAddStats(ppsStatsList, i, 0 as *mut BASE_STATS, 0 as *mut BASE_OBJECT);
    intMode = INT_EDITSTAT;
    editPosMode = IED_NOPOS;
}
#[no_mangle]
pub unsafe extern "C" fn intAddEditStructures() {
    let mut i: UDWORD = 0;
    i = 0 as libc::c_int as UDWORD;
    while i < numStructureStats && i < 80 as libc::c_int as libc::c_uint {
        let ref mut fresh2 = *apsStructStatsList.offset(i as isize);
        *fresh2 = asStructureStats.offset(i as isize);
        i = i.wrapping_add(1)
    }
    ppsStatsList = apsStructStatsList as *mut *mut BASE_STATS;
    objMode = IOBJ_BUILD;
    intAddStats(ppsStatsList, i, 0 as *mut BASE_STATS, 0 as *mut BASE_OBJECT);
    intMode = INT_EDITSTAT;
    editPosMode = IED_NOPOS;
}
/* Process return codes from the Options screen */
/* Process return codes from the Options screen */
unsafe extern "C" fn intProcessOptions(mut id: UDWORD) {
    let mut i: UDWORD = 0;
    let mut psTempl: *mut DROID_TEMPLATE = 0 as *mut DROID_TEMPLATE;
    if id >= 1010 as libc::c_int as libc::c_uint &&
           id <= 1030 as libc::c_int as libc::c_uint {
        widgSetButtonState(psWScreen,
                           (1010 as libc::c_int as
                                libc::c_uint).wrapping_add(selectedPlayer),
                           0 as libc::c_int as UDWORD);
        selectedPlayer = id.wrapping_sub(1010 as libc::c_int as libc::c_uint);
        widgSetButtonState(psWScreen,
                           (1010 as libc::c_int as
                                libc::c_uint).wrapping_add(selectedPlayer),
                           0x2 as libc::c_int as UDWORD);
    } else {
        match id {
            1037 => {
                /* The add object buttons */
                intRemoveOptions();
                i = 0 as libc::c_int as UDWORD;
                psTempl = apsDroidTemplates[selectedPlayer as usize];
                while !psTempl.is_null() &&
                          i < 40 as libc::c_int as libc::c_uint {
                    let ref mut fresh3 = *apsTemplateList.offset(i as isize);
                    *fresh3 = psTempl;
                    psTempl = (*psTempl).psNext;
                    i = i.wrapping_add(1)
                }
                ppsStatsList = apsTemplateList as *mut *mut BASE_STATS;
                objMode = IOBJ_MANUFACTURE;
                intAddStats(ppsStatsList, i, 0 as *mut BASE_STATS,
                            0 as *mut BASE_OBJECT);
                intMode = INT_EDITSTAT;
                editPosMode = IED_NOPOS
            }
            1038 => {
                intRemoveOptions();
                i = 0 as libc::c_int as UDWORD;
                while i < numStructureStats &&
                          i < 80 as libc::c_int as libc::c_uint {
                    let ref mut fresh4 =
                        *apsStructStatsList.offset(i as isize);
                    *fresh4 = asStructureStats.offset(i as isize);
                    i = i.wrapping_add(1)
                }
                ppsStatsList = apsStructStatsList as *mut *mut BASE_STATS;
                objMode = IOBJ_BUILD;
                intAddStats(ppsStatsList, i, 0 as *mut BASE_STATS,
                            0 as *mut BASE_OBJECT);
                intMode = INT_EDITSTAT;
                editPosMode = IED_NOPOS
            }
            1039 => {
                intRemoveOptions();
                i = 0 as libc::c_int as UDWORD;
                while i < numFeatureStats &&
                          i < 40 as libc::c_int as libc::c_uint {
                    let ref mut fresh5 = *apsFeatureList.offset(i as isize);
                    *fresh5 = asFeatureStats.offset(i as isize);
                    i = i.wrapping_add(1)
                }
                ppsStatsList = apsFeatureList as *mut *mut BASE_STATS;
                intAddStats(ppsStatsList, i, 0 as *mut BASE_STATS,
                            0 as *mut BASE_OBJECT);
                intMode = INT_EDITSTAT;
                editPosMode = IED_NOPOS
            }
            1006 => {
                /* Close window buttons */
                intRemoveOptions();
                intMode = INT_NORMAL
            }
            1036 => { }
            1000 | 1007 | 1001 | 1004 | 1008 | 1009 | 1035 => { }
            _ => {
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"intProcessOptions: Unknown return code\x00" as
                              *const u8 as *const libc::c_char);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"hci.c\x00" as *const u8 as *const libc::c_char,
                          2276 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 18],
                                                    &[libc::c_char; 18]>(b"intProcessOptions\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
            }
        }
    };
}
/* Process return codes from the object placement stats screen */
/* Process return codes from the object placement stats screen */
unsafe extern "C" fn intProcessEditStats(mut id: UDWORD) {
    if id >= 4100 as libc::c_int as libc::c_uint &&
           id <= 4179 as libc::c_int as libc::c_uint {
        /* Clicked on a stat button - need to look for a location for it */
        psPositionStats =
            *ppsStatsList.offset(id.wrapping_sub(4100 as libc::c_int as
                                                     libc::c_uint) as isize);
        /*if it is a structure - need to check there is enough power available
		to build */
        if (*psPositionStats).ref_0 >= 0xd0000 as libc::c_int as libc::c_uint
               &&
               (*psPositionStats).ref_0 <
                   (0xd0000 as libc::c_int + 0x10000 as libc::c_int) as
                       libc::c_uint {
            if checkPower(selectedPlayer,
                          (*(psPositionStats as
                                 *mut STRUCTURE_STATS)).powerToBuild,
                          1 as libc::c_int) == 0 {
                return
            }
        }
        /*if it is a template - need to check there is enough power available
		to build */
        if (*psPositionStats).ref_0 >= 0xc0000 as libc::c_int as libc::c_uint
               &&
               (*psPositionStats).ref_0 <
                   (0xc0000 as libc::c_int + 0x10000 as libc::c_int) as
                       libc::c_uint {
            if checkPower(selectedPlayer,
                          (*(psPositionStats as
                                 *mut DROID_TEMPLATE)).powerPoints,
                          1 as libc::c_int) == 0 {
                return
            }
        }
        intStartStructPosition(psPositionStats, 0 as *mut DROID);
        editPosMode = IED_POS
    } else if id == 4003 as libc::c_int as libc::c_uint {
        intRemoveStats();
        intStopStructPosition();
        intMode = INT_NORMAL;
        objMode = IOBJ_NONE
    };
}
/* Remove the power bars */
//static void intRemovePower(void);
/* Set the shadow for the PowerBar */
/* Set the shadow for the PowerBar */
unsafe extern "C" fn intRunPower() {
    let mut statID_0: UDWORD = 0;
    let mut psStat: *mut BASE_STATS = 0 as *mut BASE_STATS;
    let mut quantity: UDWORD = 0 as libc::c_int as UDWORD;
    let mut psResearch: *mut RESEARCH = 0 as *mut RESEARCH;
    /* Find out which button was hilited */
    statID_0 = widgGetMouseOver(psWScreen);
    if statID_0 >= 4100 as libc::c_int as libc::c_uint &&
           statID_0 <= 4179 as libc::c_int as libc::c_uint {
        psStat =
            *ppsStatsList.offset(statID_0.wrapping_sub(4100 as libc::c_int as
                                                           libc::c_uint) as
                                     isize);
        if (*psStat).ref_0 >= 0xd0000 as libc::c_int as libc::c_uint &&
               (*psStat).ref_0 <
                   (0xd0000 as libc::c_int + 0x10000 as libc::c_int) as
                       libc::c_uint {
            //get the structure build points
            quantity =
                (**apsStructStatsList.offset(statID_0.wrapping_sub(4100 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint)
                                                 as isize)).powerToBuild
        } else if (*psStat).ref_0 >= 0xc0000 as libc::c_int as libc::c_uint &&
                      (*psStat).ref_0 <
                          (0xc0000 as libc::c_int + 0x10000 as libc::c_int) as
                              libc::c_uint {
            //get the template build points
            quantity =
                calcTemplatePower(*apsTemplateList.offset(statID_0.wrapping_sub(4100
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint)
                                                              as isize))
        } else if (*psStat).ref_0 >= 0xb0000 as libc::c_int as libc::c_uint &&
                      (*psStat).ref_0 <
                          (0xb0000 as libc::c_int + 0x10000 as libc::c_int) as
                              libc::c_uint {
            //get the research points
            psResearch =
                *ppResearchList.offset(statID_0.wrapping_sub(4100 as
                                                                 libc::c_int
                                                                 as
                                                                 libc::c_uint)
                                           as isize);
            //			if (asPlayerResList[selectedPlayer][psResearch - asResearch].researched != CANCELLED_RESEARCH)
			// has research been not been canceled
            if (*asPlayerResList[selectedPlayer as
                                     usize].offset(psResearch.wrapping_offset_from(asResearch)
                                                       as libc::c_int as
                                                       isize)).ResearchStatus
                   as libc::c_int & 0x2 as libc::c_int == 0 as libc::c_int {
                quantity =
                    (**ppResearchList.offset(statID_0.wrapping_sub(4100 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint)
                                                 as isize)).researchPower
            }
        }
        //update the power bars
        intSetShadowPower(quantity);
    } else { intSetShadowPower(0 as libc::c_int as UDWORD); };
}
// Process stats screen.
unsafe extern "C" fn intRunStats() {
    let mut psOwner: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psFactory: *mut FACTORY = 0 as *mut FACTORY;
    // No looped production on PSX.
    if intMode as libc::c_uint != INT_EDITSTAT as libc::c_int as libc::c_uint
           &&
           objMode as libc::c_uint ==
               IOBJ_MANUFACTURE as libc::c_int as libc::c_uint {
        psOwner =
            widgGetUserData(psWScreen, 4404 as libc::c_int as UDWORD) as
                *mut BASE_OBJECT;
        if (*psOwner).type_0 as libc::c_uint ==
               OBJ_STRUCTURE as libc::c_int as libc::c_uint {
        } else {
            debug(LOG_ERROR,
                  b"intRunStats: Invalid object type\x00" as *const u8 as
                      *const libc::c_char);
        };
        if (*psOwner).type_0 as libc::c_uint ==
               OBJ_STRUCTURE as libc::c_int as libc::c_uint {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"hci.c\x00" as *const u8 as *const libc::c_char,
                  2488 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 12],
                                            &[libc::c_char; 12]>(b"intRunStats\x00")).as_ptr(),
                  b"psOwner->type == OBJ_STRUCTURE\x00" as *const u8 as
                      *const libc::c_char);
        };
        psStruct = psOwner as *mut STRUCTURE;
        if StructIsFactory(psStruct) != 0 {
        } else {
            debug(LOG_ERROR,
                  b"intRunStats: Invalid Structure type\x00" as *const u8 as
                      *const libc::c_char);
        };
        if StructIsFactory(psStruct) != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"hci.c\x00" as *const u8 as *const libc::c_char,
                  2491 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 12],
                                            &[libc::c_char; 12]>(b"intRunStats\x00")).as_ptr(),
                  b"StructIsFactory(psStruct)\x00" as *const u8 as
                      *const libc::c_char);
        };
        psFactory = (*psStruct).pFunctionality as *mut FACTORY;
        //adjust the loop button if necessary
        if !(*psFactory).psSubject.is_null() &&
               (*psFactory).quantity as libc::c_int != 0 {
            widgSetButtonState(psWScreen, 4403 as libc::c_int as UDWORD,
                               0x4 as libc::c_int as UDWORD);
        }
    };
}
/* Add the stats screen for a given object */
unsafe extern "C" fn intAddObjectStats(mut psObj: *mut BASE_OBJECT,
                                       mut id: UDWORD) {
    let mut psStats: *mut BASE_STATS = 0 as *mut BASE_STATS;
    let mut statMajor: UWORD = 0 as libc::c_int as UWORD;
    let mut statMinor: UWORD = 0 as libc::c_int as UWORD;
    let mut newStatMajor: UWORD = 0;
    let mut newStatMinor: UWORD = 0;
    let mut i: UDWORD = 0;
    let mut j: UDWORD = 0;
    let mut index: UDWORD = 0;
    let mut count: UDWORD = 0;
    let mut iconNumber: SDWORD = 0;
    let mut entryIN: SDWORD = 0;
    let mut psForm: *mut W_TABFORM = 0 as *mut W_TABFORM;
    /* Clear a previous structure pos if there is one */
    intStopStructPosition();
    /* Get the current tab pos */
    widgGetTabs(psWScreen, 3500 as libc::c_int as UDWORD, &mut objMajor,
                &mut objMinor);
    /* if there is already a stats form up, remove it */
    statMinor = 0 as libc::c_int as UWORD;
    statMajor = statMinor;
    // Store the tab positions.
    if intMode as libc::c_uint == INT_STAT as libc::c_int as libc::c_uint {
        if !widgGetFromID(psWScreen, 4000 as libc::c_int as UDWORD).is_null()
           {
            widgGetTabs(psWScreen, 4004 as libc::c_int as UDWORD,
                        &mut statMajor, &mut statMinor);
        }
        intRemoveStatsNoAnim();
    }
    /* Display the stats window
	 *  - restore the tab position if there is no stats selected
	 */
    psStats = objGetStatsFunc.expect("non-null function pointer")(psObj);
    // note the object for the screen
    apsPreviousObj[objMode as usize] = psObj;
    //determine the Structures that can be built
    if objMode as libc::c_uint == IOBJ_BUILD as libc::c_int as libc::c_uint {
        numStatsListEntries =
            fillStructureList(apsStructStatsList, selectedPlayer,
                              (80 as libc::c_int - 1 as libc::c_int) as
                                  UDWORD);
        ppsStatsList = apsStructStatsList as *mut *mut BASE_STATS
    }
    //have to determine the Template list once the factory has been chosen
    if objMode as libc::c_uint ==
           IOBJ_MANUFACTURE as libc::c_int as libc::c_uint {
        numStatsListEntries =
            fillTemplateList(apsTemplateList, psObj as *mut STRUCTURE,
                             40 as libc::c_int as UDWORD);
        ppsStatsList = apsTemplateList as *mut *mut BASE_STATS
    }
    /*have to calculate the list each time the Topic button is pressed
	  so that only one topic can be researched at a time*/
    if objMode as libc::c_uint == IOBJ_RESEARCH as libc::c_int as libc::c_uint
       {
        //set to value that won't be reached in fillResearchList
        index = numResearch.wrapping_add(1 as libc::c_int as libc::c_uint);
        if !psStats.is_null() {
            index =
                (psStats as *mut RESEARCH).wrapping_offset_from(asResearch) as
                    libc::c_int as UDWORD
        }
        //recalculate the list
        numStatsListEntries =
            fillResearchList(pList, selectedPlayer, index as UWORD,
                             80 as libc::c_int as UWORD) as UDWORD;
        //	-- Alex's reordering of the list
        count = 0 as libc::c_int as UDWORD;
        i = 0 as libc::c_int as UDWORD;
        while i < RID_MAXRID as libc::c_int as libc::c_uint {
            iconNumber = mapRIDToIcon(i);
            j = 0 as libc::c_int as UDWORD;
            while j < numStatsListEntries {
                entryIN =
                    (*asResearch.offset(*pList.offset(j as isize) as
                                            isize)).iconID as SDWORD;
                if entryIN == iconNumber {
                    let fresh6 = count;
                    count = count.wrapping_add(1);
                    *pSList.offset(fresh6 as isize) =
                        *pList.offset(j as isize)
                }
                j = j.wrapping_add(1)
            }
            i = i.wrapping_add(1)
        }
        /* Tag on the ones at the end that have no BASTARD icon IDs - why is this?!!?!?!?*/
        j = 0 as libc::c_int as UDWORD;
        while j < numStatsListEntries {
            //this can't be assumed cos we've added some more icons and they have higher #define values than QUESTIONMARK!
            //entryIN = asResearch[pList[j]].iconID;
			//if(entryIN<mapRIDToIcon(RID_ROCKET) OR entryIN>mapRIDToIcon(RID_QUESTIONMARK))
            iconNumber =
                mapIconToRID((*asResearch.offset(*pList.offset(j as isize) as
                                                     isize)).iconID as
                                 UDWORD);
            if iconNumber < 0 as libc::c_int {
                let fresh7 = count;
                count = count.wrapping_add(1);
                *pSList.offset(fresh7 as isize) = *pList.offset(j as isize)
            }
            j = j.wrapping_add(1)
        }
        //fill up the list with topics
        i = 0 as libc::c_int as UDWORD;
        while i < numStatsListEntries {
            let ref mut fresh8 = *ppResearchList.offset(i as isize);
            *fresh8 =
                asResearch.offset(*pSList.offset(i as isize) as libc::c_int as
                                      isize);
            i = i.wrapping_add(1)
            // note change from pList
        }
    }
    //DBPRINTF(("intAddStats(%p,%d,%p,%p)\n",ppsStatsList, numStatsListEntries, psStats, psObj);
    intAddStats(ppsStatsList, numStatsListEntries, psStats, psObj);
    //get the tab positions for the new stat form
    psForm =
        widgGetFromID(psWScreen, 4004 as libc::c_int as UDWORD) as
            *mut W_TABFORM;
    if !psForm.is_null() {
        newStatMajor = (*psForm).numMajor;
        newStatMinor = (*psForm).asMajor[statMajor as usize].numMinor;
        // Restore the tab positions.
        if psStats.is_null() &&
               !widgGetFromID(psWScreen,
                              4000 as libc::c_int as UDWORD).is_null() {
            //only restore if we've still got at least that many tabs
            if newStatMajor as libc::c_int > statMajor as libc::c_int &&
                   newStatMinor as libc::c_int > statMinor as libc::c_int {
                widgSetTabs(psWScreen, 4004 as libc::c_int as UDWORD,
                            statMajor, statMinor);
            }
        }
    }
    intMode = INT_STAT;
    /* Note the object */
    psObjSelected = psObj;
    objStatID = id;
    /* Reset the tabs and lock the button */
    widgSetTabs(psWScreen, 3500 as libc::c_int as UDWORD, objMajor, objMinor);
    if id != 0 as libc::c_int as libc::c_uint {
        widgSetButtonState(psWScreen, id, 0x4 as libc::c_int as UDWORD);
    };
}
unsafe extern "C" fn intSelectDroid(mut psObj: *mut BASE_OBJECT) {
    if driveModeActive() != 0 {
        clearSel();
        (*(psObj as *mut DROID)).selected = 1 as libc::c_int as UBYTE;
        driveSelectionChanged();
        //		clearSelection();
//		((DROID*)psObj)->selected = TRUE;
//		StopDriverMode();
//		StartDriverMode();
        driveDisableControl();
    } else {
        clearSelection();
        (*(psObj as *mut DROID)).selected = 1 as libc::c_int as UBYTE
    };
}
unsafe extern "C" fn intResetWindows(mut psObj: *mut BASE_OBJECT) {
    if !psObj.is_null() {
        // reset the object screen with the new object
        match objMode as libc::c_uint {
            16 | 17 | 18 => { intAddBuild(psObj as *mut DROID); }
            20 => { intAddResearch(psObj as *mut STRUCTURE); }
            19 => { intAddManufacture(psObj as *mut STRUCTURE); }
            21 => { intAddCommand(psObj as *mut DROID); }
            _ => { }
        }
        //intAddObjectStats(psObj, id);
    };
}
/* Process the object widgets */
/* Process return codes from the object screen */
unsafe extern "C" fn intProcessObject(mut id: UDWORD) {
    let mut psObj: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut IsDeliveryRepos: BOOL = 0 as libc::c_int;
    let mut butIndex: SDWORD = 0;
    let mut statButID: UDWORD = 0;
    if !widgGetFromID(psWScreen, 3500 as libc::c_int as UDWORD).is_null() {
    } else {
        debug(LOG_ERROR,
              b"intProcessObject, missing form\n\x00" as *const u8 as
                  *const libc::c_char);
    };
    if !widgGetFromID(psWScreen, 3500 as libc::c_int as UDWORD).is_null() {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"hci.c\x00" as *const u8 as *const libc::c_char,
              2702 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 17],
                                        &[libc::c_char; 17]>(b"intProcessObject\x00")).as_ptr(),
              b"widgGetFromID(psWScreen,IDOBJ_TABFORM) != NULL\x00" as
                  *const u8 as *const libc::c_char);
    };
    // deal with CRTL clicks
    if objMode as libc::c_uint == IOBJ_BUILD as libc::c_int as libc::c_uint &&
           (keyDown(KEY_LCTRL) != 0 || keyDown(KEY_RCTRL) != 0 ||
                keyDown(KEY_LSHIFT) != 0 || keyDown(KEY_RSHIFT) != 0) &&
           (id >= 3002 as libc::c_int as libc::c_uint &&
                id <= 3021 as libc::c_int as libc::c_uint ||
                id >= 3100 as libc::c_int as libc::c_uint &&
                    id <= 3199 as libc::c_int as libc::c_uint) {
        /* Find the object that the ID refers to */
        psObj = intGetObject(id);
        if id >= 3002 as libc::c_int as libc::c_uint &&
               id <= 3021 as libc::c_int as libc::c_uint {
            statButID =
                (3100 as libc::c_int as
                     libc::c_uint).wrapping_add(id).wrapping_sub(3002 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint)
        } else { statButID = id }
        if (*psObj).selected != 0 {
            (*psObj).selected = 0 as libc::c_int as UBYTE;
            widgSetButtonState(psWScreen, statButID,
                               0 as libc::c_int as UDWORD);
            if intNumSelectedDroids(DROID_CONSTRUCT as libc::c_int as UDWORD)
                   == 0 as libc::c_int &&
                   intNumSelectedDroids(DROID_CYBORG_CONSTRUCT as libc::c_int
                                            as UDWORD) == 0 as libc::c_int {
                intRemoveStats();
            }
            if psObjSelected == psObj {
                psObjSelected =
                    intCheckForDroid(DROID_CONSTRUCT as libc::c_int as UDWORD)
                        as *mut BASE_OBJECT;
                if psObjSelected.is_null() {
                    psObjSelected =
                        intCheckForDroid(DROID_CYBORG_CONSTRUCT as libc::c_int
                                             as UDWORD) as *mut BASE_OBJECT
                }
            }
        } else {
            if !psObjSelected.is_null() {
                (*psObjSelected).selected = 1 as libc::c_int as UBYTE
            }
            (*psObj).selected = 1 as libc::c_int as UBYTE;
            widgSetButtonState(psWScreen, statButID,
                               0x4 as libc::c_int as UDWORD);
            intAddObjectStats(psObj, statButID);
        }
    } else if id >= 3002 as libc::c_int as libc::c_uint &&
                  id <= 3021 as libc::c_int as libc::c_uint {
        /* deal with RMB clicks */
        if widgGetButtonKey(psWScreen) & 2 as libc::c_int as libc::c_uint != 0
           {
            intObjectRMBPressed(id);
        } else {
            /* deal with LMB clicks */
            /* An object button has been pressed */
			/* Find the object that the ID refers to */
            psObj = intGetObject(id);
            if !psObj.is_null() {
                //Only do this if not offworld - only check if a structure
				//if (!offWorldKeepLists)
                if (*psObj).type_0 as libc::c_uint ==
                       OBJ_STRUCTURE as libc::c_int as libc::c_uint &&
                       offWorldKeepLists == 0 {
                    /* Deselect old buildings */
                    psStruct = apsStructLists[selectedPlayer as usize];
                    while !psStruct.is_null() {
                        (*psStruct).selected = 0 as libc::c_int as UBYTE;
                        psStruct = (*psStruct).psNext
                    }
                    /* Select new one */
                    (*(psObj as *mut STRUCTURE)).selected =
                        1 as libc::c_int as UBYTE
                }
                if driveModeActive() == 0 {
                    //don't do this if offWorld and a structure object has been selected
                    if !((*psObj).type_0 as libc::c_uint ==
                             OBJ_STRUCTURE as libc::c_int as libc::c_uint &&
                             offWorldKeepLists != 0) {
                        // set the map position - either the object position, or the position jumped from
                        butIndex =
                            id.wrapping_sub(3002 as libc::c_int as
                                                libc::c_uint) as SDWORD;
                        if butIndex >= 0 as libc::c_int &&
                               butIndex < IOBJ_MAX as libc::c_int {
                            if asJumpPos[butIndex as usize].x ==
                                   0 as libc::c_int &&
                                   asJumpPos[butIndex as usize].y ==
                                       0 as libc::c_int ||
                                   DrawnInLastFrame((*psObj).sDisplay.frameNumber
                                                        as SDWORD) == 0 ||
                                   ((*psObj).sDisplay.screenX >
                                        pie_GetVideoBufferWidth() ||
                                        (*psObj).sDisplay.screenY >
                                            pie_GetVideoBufferHeight()) {
                                getPlayerPos(&mut (*asJumpPos.as_mut_ptr().offset(butIndex
                                                                                      as
                                                                                      isize)).x
                                                 as *mut libc::c_int as
                                                 *mut SDWORD,
                                             &mut (*asJumpPos.as_mut_ptr().offset(butIndex
                                                                                      as
                                                                                      isize)).y
                                                 as *mut libc::c_int as
                                                 *mut SDWORD);
                                setPlayerPos((*psObj).x as SDWORD,
                                             (*psObj).y as SDWORD);
                                if getWarCamStatus() != 0 {
                                    camToggleStatus();
                                }
                                //							intSetMapPos(psObj->x, psObj->y);
                            } else {
                                setPlayerPos(asJumpPos[butIndex as usize].x,
                                             asJumpPos[butIndex as usize].y);
                                if getWarCamStatus() != 0 {
                                    camToggleStatus();
                                }
                                //							intSetMapPos(asJumpPos[butIndex].x, asJumpPos[butIndex].y);
                                asJumpPos[butIndex as usize].x =
                                    0 as libc::c_int;
                                asJumpPos[butIndex as usize].y =
                                    0 as libc::c_int
                            }
                        }
                    }
                }
                psObj = intGetObject(id);
                if IsDeliveryRepos == 0 { intResetWindows(psObj); }
                // If a construction droid button was clicked then
				// clear all other selections and select it.
                if (*psObj).type_0 as libc::c_uint ==
                       OBJ_DROID as libc::c_int as libc::c_uint {
                    // If it's a droid...
                    intSelectDroid(psObj);
                    psObjSelected = psObj
                }
            }
        }
    } else if id >= 3100 as libc::c_int as libc::c_uint &&
                  id <= 3199 as libc::c_int as libc::c_uint {
        /* A object stat button has been pressed */
        /* deal with RMB clicks */
        if widgGetButtonKey(psWScreen) & 2 as libc::c_int as libc::c_uint != 0
           {
            intObjStatRMBPressed(id);
        } else {
            /* Find the object that the stats ID refers to */
            psObj = intGetObject(id);
            intResetWindows(psObj);
            // If a droid button was clicked then clear all other selections and select it.
            if (*psObj).type_0 as libc::c_uint ==
                   OBJ_DROID as libc::c_int as libc::c_uint {
                // Select the droid when the stat button (in the object window) is pressed.
                intSelectDroid(psObj);
                psObjSelected = psObj
            } else if (*psObj).type_0 as libc::c_uint ==
                          OBJ_STRUCTURE as libc::c_int as libc::c_uint {
                //				clearSelection();
//				psObj->selected = TRUE;
                if StructIsFactory(psObj as *mut STRUCTURE) != 0 {
                    //might need to cancel the hold on production
                    releaseProduction(psObj as *mut STRUCTURE);
                } else if (*(*(psObj as
                                   *mut STRUCTURE)).pStructureType).type_0 ==
                              REF_RESEARCH as libc::c_int as libc::c_uint {
                    //might need to cancel the hold on research facilty
                    releaseResearch(psObj as *mut STRUCTURE);
                }
            }
        }
    } else if id == 3001 as libc::c_int as libc::c_uint {
        intResetScreen(0 as libc::c_int);
        intMode = INT_NORMAL
    } else if objMode as libc::c_uint !=
                  IOBJ_COMMAND as libc::c_int as libc::c_uint &&
                  id != 3500 as libc::c_int as libc::c_uint {
        /* Not a button on the build form, must be on the stats form */
        intProcessStats(id);
    } else if id != 3500 as libc::c_int as libc::c_uint {
        intProcessOrder(id);
    };
}
/* Process return codes from the stats screen */
/* Process return codes from the stats screen */
unsafe extern "C" fn intProcessStats(mut id: UDWORD) {
    let mut psStats: *mut BASE_STATS = 0 as *mut BASE_STATS;
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psFlag: *mut FLAG_POSITION = 0 as *mut FLAG_POSITION;
    let mut psNext: *mut DROID_TEMPLATE = 0 as *mut DROID_TEMPLATE;
    if !widgGetFromID(psWScreen, 3500 as libc::c_int as UDWORD).is_null() {
    } else {
        debug(LOG_ERROR,
              b"intProcessStats, missing form\n\x00" as *const u8 as
                  *const libc::c_char);
    };
    if !widgGetFromID(psWScreen, 3500 as libc::c_int as UDWORD).is_null() {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"hci.c\x00" as *const u8 as *const libc::c_char,
              2912 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 16],
                                        &[libc::c_char; 16]>(b"intProcessStats\x00")).as_ptr(),
              b"widgGetFromID(psWScreen,IDOBJ_TABFORM) != NULL\x00" as
                  *const u8 as *const libc::c_char);
    };
    if id >= 4100 as libc::c_int as libc::c_uint &&
           id <= 4179 as libc::c_int as libc::c_uint {
        if id.wrapping_sub(4100 as libc::c_int as libc::c_uint) <
               numStatsListEntries {
        } else {
            debug(LOG_ERROR,
                  b"intProcessStructure: Invalid structure stats id\x00" as
                      *const u8 as *const libc::c_char);
        };
        if id.wrapping_sub(4100 as libc::c_int as libc::c_uint) <
               numStatsListEntries {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"hci.c\x00" as *const u8 as *const libc::c_char,
                  2918 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 16],
                                            &[libc::c_char; 16]>(b"intProcessStats\x00")).as_ptr(),
                  b"id - IDSTAT_START < numStatsListEntries\x00" as *const u8
                      as *const libc::c_char);
        };
        /* deal with RMB clicks */
        if widgGetButtonKey(psWScreen) & 2 as libc::c_int as libc::c_uint != 0
           {
            //printf("WKEY_SECONDARY : %d\n",id);
            intStatsRMBPressed(id);
        } else if objMode as libc::c_uint ==
                      IOBJ_MANUFACTURE as libc::c_int as libc::c_uint {
            /* deal with LMB clicks */
            //manufacture works differently!
            //get the stats
            psStats =
                *ppsStatsList.offset(id.wrapping_sub(4100 as libc::c_int as
                                                         libc::c_uint) as
                                         isize);
            if 1 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"intProcessStats: Invalid structure pointer\x00" as
                          *const u8 as *const libc::c_char);
            };
            if 1 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"hci.c\x00" as *const u8 as *const libc::c_char,
                      2936 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 16],
                                                &[libc::c_char; 16]>(b"intProcessStats\x00")).as_ptr(),
                      b"PTRVALID(psObjSelected, sizeof(STRUCTURE))\x00" as
                          *const u8 as *const libc::c_char);
            };
            if 1 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"intProcessStats: Invalid template pointer\x00" as
                          *const u8 as *const libc::c_char);
            };
            if 1 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"hci.c\x00" as *const u8 as *const libc::c_char,
                      2938 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 16],
                                                &[libc::c_char; 16]>(b"intProcessStats\x00")).as_ptr(),
                      b"PTRVALID(psStats, sizeof(DROID_TEMPLATE))\x00" as
                          *const u8 as *const libc::c_char);
            };
            if productionPlayer as libc::c_int ==
                   selectedPlayer as SBYTE as libc::c_int {
                let mut psFactory: *mut FACTORY =
                    (*(psObjSelected as *mut STRUCTURE)).pFunctionality as
                        *mut FACTORY;
                //increase the production
                factoryProdAdjust(psObjSelected as *mut STRUCTURE,
                                  psStats as *mut DROID_TEMPLATE,
                                  1 as libc::c_int);
                //need to check if this was the template that was mid-production
                if psStats == (*psFactory).psSubject {
                    //if have wrapped round to zero then cancel the production
                    if getProductionQuantity(psObjSelected as *mut STRUCTURE,
                                             psStats as *mut DROID_TEMPLATE)
                           == 0 as libc::c_int as libc::c_uint {
                        //init the factory production
                        (*psFactory).psSubject = 0 as *mut BASE_STATS;
                        //check to see if anything left to produce
                        psNext =
                            factoryProdUpdate(psObjSelected as *mut STRUCTURE,
                                              0 as *mut DROID_TEMPLATE);
                        if psNext.is_null() {
                            intManufactureFinished(psObjSelected as
                                                       *mut STRUCTURE);
                        } else if objSetStatsFunc.expect("non-null function pointer")(psObjSelected,
                                                                                      psNext
                                                                                          as
                                                                                          *mut BASE_STATS)
                                      == 0 {
                            intSetStats(objStatID, 0 as *mut BASE_STATS);
                        } else {
                            // Reset the button on the object form
                            intSetStats(objStatID, psStats);
                        }
                    }
                } else if (*psFactory).psSubject.is_null() {
                    if objSetStatsFunc.expect("non-null function pointer")(psObjSelected,
                                                                           psStats)
                           == 0 {
                        intSetStats(objStatID, 0 as *mut BASE_STATS);
                    } else {
                        //if factory wasn't currently on line then set the object button
                        // Reset the button on the object form
                        intSetStats(objStatID, psStats);
                    }
                }
            }
        } else {
            /* See if this was a click on an already selected stat */
            psStats =
                objGetStatsFunc.expect("non-null function pointer")(psObjSelected);
            //only do the cancel operation if not trying to add to the build list
            if psStats ==
                   *ppsStatsList.offset(id.wrapping_sub(4100 as libc::c_int as
                                                            libc::c_uint) as
                                            isize) &&
                   !(objMode as libc::c_uint ==
                         IOBJ_BUILD as libc::c_int as libc::c_uint &&
                         ctrlShiftDown() != 0) {
                //this needs to be done before the topic is cancelled from the structure
                    //research works differently now! - AB 5/2/99
					/* If Research then need to set topic to be cancelled */
                if objMode as libc::c_uint ==
                       IOBJ_RESEARCH as libc::c_int as libc::c_uint {
                    if (*psObjSelected).type_0 as libc::c_uint ==
                           OBJ_STRUCTURE as libc::c_int as libc::c_uint {
                        cancelResearch(psObjSelected as *mut STRUCTURE);
                    }
                }
                //research works differently now! - AB 5/2/99
					/* If Research then need to set topic to be cancelled */
					/*if (objMode == IOBJ_RESEARCH)
					{
						if (psObjSelected->type == OBJ_STRUCTURE )
						{
							cancelResearch((STRUCTURE *)psObjSelected);
						}
					}*/
                objSetStatsFunc.expect("non-null function pointer")(psObjSelected,
                                                                    0 as
                                                                        *mut BASE_STATS);
                intSetStats(objStatID, 0 as *mut BASE_STATS);
                widgSetButtonState(psWScreen, id, 0 as libc::c_int as UDWORD);
            } else {
                /* Clear the object stats */
                /* Reset the button on the object form */
                /* Unlock the button on the stats form */
                //If Research then need to set the topic - if one, to be cancelled
                if objMode as libc::c_uint ==
                       IOBJ_RESEARCH as libc::c_int as libc::c_uint {
                    if (*psObjSelected).type_0 as libc::c_uint ==
                           OBJ_STRUCTURE as libc::c_int as libc::c_uint &&
                           (*(*(psObjSelected as
                                    *mut STRUCTURE)).pStructureType).type_0 ==
                               REF_RESEARCH as libc::c_int as libc::c_uint {
                        //if there was a topic currently being researched - cancel it
                        if !(*((*(psObjSelected as
                                      *mut STRUCTURE)).pFunctionality as
                                   *mut RESEARCH_FACILITY)).psSubject.is_null()
                           {
                            cancelResearch(psObjSelected as *mut STRUCTURE);
                        }
                    }
                }
                // call the tutorial callback if necessary
                if bInTutorial != 0 &&
                       objMode as libc::c_uint ==
                           IOBJ_BUILD as libc::c_int as libc::c_uint {
                    eventFireCallbackTrigger(CALL_BUILDGRID as libc::c_int as
                                                 TRIGGER_TYPE);
                }
                // Set the object stats
                psStats =
                    *ppsStatsList.offset(id.wrapping_sub(4100 as libc::c_int
                                                             as libc::c_uint)
                                             as isize);
                // Reset the button on the object form
					//if this returns FALSE, there's a problem so set the button to NULL
                if objSetStatsFunc.expect("non-null function pointer")(psObjSelected,
                                                                       psStats)
                       == 0 {
                    intSetStats(objStatID, 0 as *mut BASE_STATS);
                } else {
                    // Reset the button on the object form
                    intSetStats(objStatID, psStats);
                }
            }
            // Get the tabs on the object form
            widgGetTabs(psWScreen, 3500 as libc::c_int as UDWORD,
                        &mut objMajor, &mut objMinor);
            // Close the stats box
            intRemoveStats();
            intMode = INT_OBJECT;
            // Reset the tabs on the object form
            widgSetTabs(psWScreen, 3500 as libc::c_int as UDWORD, objMajor,
                        objMinor);
            // Close the object box as well if selecting a location to build- no longer hide/reveal
                //or if selecting a structure to demolish
            if objMode as libc::c_uint ==
                   IOBJ_BUILDSEL as libc::c_int as libc::c_uint ||
                   objMode as libc::c_uint ==
                       IOBJ_DEMOLISHSEL as libc::c_int as libc::c_uint {
                if driveModeActive() != 0 {
                    // Make sure weve got a construction droid selected.
						//if(driveGetDriven()->droidType != DROID_CONSTRUCT) {
                    if (*driveGetDriven()).droidType as libc::c_uint !=
                           DROID_CONSTRUCT as libc::c_int as libc::c_uint &&
                           (*driveGetDriven()).droidType as libc::c_uint !=
                               DROID_CYBORG_CONSTRUCT as libc::c_int as
                                   libc::c_uint {
                        //PD30 #ifdef PSX
//PD30 							intGotoNextDroidType(DROID_CONSTRUCT);
//PD30 #endif
//PD30 							driveSelectionChanged();
                        driveDisableControl();
                    }
                    driveDisableTactical();
                    driveStartBuild();
                    intRemoveObject();
                }
                intRemoveObject();
                //hack to stop the stats window re-opening in demolish mode
                if objMode as libc::c_uint ==
                       IOBJ_DEMOLISHSEL as libc::c_int as libc::c_uint {
                    IntRefreshPending = 0 as libc::c_int
                }
            }
        }
    } else if id == 4003 as libc::c_int as libc::c_uint {
        /* Get the tabs on the object form */
        widgGetTabs(psWScreen, 3500 as libc::c_int as UDWORD, &mut objMajor,
                    &mut objMinor);
        /* Close the structure box without doing anything */
        intRemoveStats();
        intMode = INT_OBJECT;
        /* Reset the tabs on the build form */
        widgSetTabs(psWScreen, 3500 as libc::c_int as UDWORD, objMajor,
                    objMinor);
        /* Unlock the stats button */
        widgSetButtonState(psWScreen, objStatID, 0 as libc::c_int as UDWORD);
    } else if !(id == 4400 as libc::c_int as libc::c_uint) {
        if !(id >= 12000 as libc::c_int as libc::c_uint &&
                 id <= 12019 as libc::c_int as libc::c_uint) {
            // No looped production on PSX.
            if id == 4403 as libc::c_int as libc::c_uint {
                // Process the loop button.
                psStruct =
                    widgGetUserData(psWScreen, 4404 as libc::c_int as UDWORD)
                        as *mut STRUCTURE;
                if !psStruct.is_null() {
                    //LMB pressed
                    if widgGetButtonKey(psWScreen) &
                           1 as libc::c_int as libc::c_uint != 0 {
                        factoryLoopAdjust(psStruct, 1 as libc::c_int);
                    } else if widgGetButtonKey(psWScreen) &
                                  2 as libc::c_int as libc::c_uint != 0 {
                        factoryLoopAdjust(psStruct, 0 as libc::c_int);
                    }
                    if !(*((*psStruct).pFunctionality as
                               *mut FACTORY)).psSubject.is_null() &&
                           (*((*psStruct).pFunctionality as
                                  *mut FACTORY)).quantity as libc::c_int != 0
                       {
                        //RMB pressed
                        //lock the button
                        widgSetButtonState(psWScreen,
                                           4403 as libc::c_int as UDWORD,
                                           0x4 as libc::c_int as UDWORD);
                    } else {
                        //unlock
                        widgSetButtonState(psWScreen,
                                           4403 as libc::c_int as UDWORD,
                                           0 as libc::c_int as UDWORD);
                    }
                }
            } else if id == 4405 as libc::c_int as libc::c_uint {
                // Process the DP button
                psStruct =
                    widgGetUserData(psWScreen, 4405 as libc::c_int as UDWORD)
                        as *mut STRUCTURE;
                if !psStruct.is_null() {
                    // make sure that the factory isn't assigned to a commander
                    assignFactoryCommandDroid(psStruct, 0 as *mut _droid);
                    psFlag = FindFactoryDelivery(psStruct);
                    if !psFlag.is_null() {
                        StartDeliveryPosition(psFlag as *mut OBJECT_POSITION,
                                              0 as libc::c_int);
                    }
                }
            } else {
                if id == 4000 as libc::c_int as libc::c_uint ||
                       id == 4001 as libc::c_int as libc::c_uint ||
                       id == 4002 as libc::c_int as libc::c_uint ||
                       id == 4004 as libc::c_int as libc::c_uint {
                } else {
                    debug(LOG_ERROR,
                          b"intProcessStructure: Unknown widget ID\x00" as
                              *const u8 as *const libc::c_char);
                };
                if id == 4000 as libc::c_int as libc::c_uint ||
                       id == 4001 as libc::c_int as libc::c_uint ||
                       id == 4002 as libc::c_int as libc::c_uint ||
                       id == 4004 as libc::c_int as libc::c_uint {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"hci.c\x00" as *const u8 as *const libc::c_char,
                          3206 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 16],
                                                    &[libc::c_char; 16]>(b"intProcessStats\x00")).as_ptr(),
                          b"id == IDSTAT_FORM || id == IDSTAT_TITLEFORM || id == IDSTAT_LABEL || id == IDSTAT_TABFORM\x00"
                              as *const u8 as *const libc::c_char);
                };
            }
        }
    };
}
/* Set the map view point to the world coordinates x,y */
/* Set the map view point to the world coordinates x,y */
/* Set the map view point to the world coordinates x,y */
#[no_mangle]
pub unsafe extern "C" fn intSetMapPos(mut x: UDWORD, mut y: UDWORD) {
    if driveModeActive() == 0 {
        setViewPos(x >> 7 as libc::c_int, y >> 7 as libc::c_int,
                   1 as libc::c_int);
        //		DBPRINTF(("intSetMapPos\n");
        mapX = x >> 7 as libc::c_int;
        mapY = y >> 7 as libc::c_int
    };
}
//		setPlayerPos((SDWORD)x, (SDWORD)y);
/* Sync the interface to an object */
/* Sync the interface to an object */
// If psObj is NULL then reset interface displays.
//
// There should be two version of this function, one for left clicking and one got right.
//
#[no_mangle]
pub unsafe extern "C" fn intObjectSelected(mut psObj: *mut BASE_OBJECT) {
    //STRUCTURE	*psStruct;
	/* Remove whatever is up */
//	intResetScreen(FALSE);
    //DBPRINTF(("intObjectSelected\n"));
    if !psObj.is_null() {
        //		if(!widgetsOn)
//		{
//			forceWidgetsOn = TRUE;
//		}
//		intResetScreen(TRUE);
        setWidgetsStatus(1 as libc::c_int);
        match (*psObj).type_0 as libc::c_uint {
            0 => {
                /*			stop build interface appearing for constuction droids
			if (droidType((DROID *)psObj) == DROID_CONSTRUCT)
			{
				intResetScreen(FALSE);
				intAddBuild((DROID *)psObj);
			}
			else*/
                //			if(!OrderUp)
//			{
//				intResetScreen(FALSE);
//			}
//			intAddOrder((DROID *)psObj);
//			intMode = INT_ORDER;
                if OrderUp == 0 {
                    intResetScreen(0 as libc::c_int);
                    //changed to a BASE_OBJECT to accomodate the factories - AB 21/04/99
                //intAddOrder((DROID *)psObj);
                    intAddOrder(psObj);
                    intMode = INT_ORDER
                } else {
                    //changed to a BASE_OBJECT to accomodate the factories - AB 21/04/99
				//intAddOrder((DROID *)psObj);
                    intAddOrder(psObj);
                }
            }
            1 => {
                //don't do anything if structure is only partially built
                intResetScreen(0 as libc::c_int);
                if !(objMode as libc::c_uint ==
                         IOBJ_DEMOLISHSEL as libc::c_int as libc::c_uint) {
                    if (*(psObj as *mut STRUCTURE)).status as libc::c_int ==
                           SS_BUILT as libc::c_int {
                        if (*(*(psObj as
                                    *mut STRUCTURE)).pStructureType).type_0 ==
                               REF_FACTORY as libc::c_int as libc::c_uint ||
                               (*(*(psObj as
                                        *mut STRUCTURE)).pStructureType).type_0
                                   ==
                                   REF_CYBORG_FACTORY as libc::c_int as
                                       libc::c_uint ||
                               (*(*(psObj as
                                        *mut STRUCTURE)).pStructureType).type_0
                                   ==
                                   REF_VTOL_FACTORY as libc::c_int as
                                       libc::c_uint {
                            intAddManufacture(psObj as *mut STRUCTURE);
                            //widgHide(psWScreen, IDOBJ_FORM);
                        } else if (*(*(psObj as
                                           *mut STRUCTURE)).pStructureType).type_0
                                      ==
                                      REF_RESEARCH as libc::c_int as
                                          libc::c_uint {
                            intAddResearch(psObj as *mut STRUCTURE);
                            //widgHide(psWScreen, IDOBJ_FORM);
                        }
                        //		  		for(psStruct = apsStructLists[selectedPlayer]; psStruct; psStruct=psStruct->psNext)
//				{
//					psStruct->selected = FALSE;
//				}
//				((STRUCTURE*)psObj)->selected = TRUE;		// wrong place?
                    }
                }
            }
            _ => { }
        }
    } else {
        intResetScreen(0 as libc::c_int);
        //		if(OrderUp) {
//			intRemoveOrder();
//		}
    };
}
// add the construction interface if a constructor droid is selected
// add the construction interface if a constructor droid is selected
#[no_mangle]
pub unsafe extern "C" fn intConstructorSelected(mut psDroid: *mut DROID) {
    //	if(!widgetsOn)
//	{
//		forceWidgetsOn = TRUE;
//	}
//	intResetScreen(FALSE);
    setWidgetsStatus(1 as libc::c_int);
    intAddBuild(psDroid);
    widgHide(psWScreen, 3000 as libc::c_int as UDWORD);
}
// add the construction interface if a constructor droid is selected
// add the construction interface if a constructor droid is selected
#[no_mangle]
pub unsafe extern "C" fn intCommanderSelected(mut psDroid: *mut DROID) {
    setWidgetsStatus(1 as libc::c_int);
    intAddCommand(psDroid);
    widgHide(psWScreen, 3000 as libc::c_int as UDWORD);
}
/* Start looking for a structure location */
/* Start looking for a structure location */
//static void intStartStructPosition(UDWORD width, UDWORD height)
unsafe extern "C" fn intStartStructPosition(mut psStats: *mut BASE_STATS,
                                            mut psDroid: *mut DROID) {
    init3DBuilding(psStats, None, 0 as *mut libc::c_void);
    /*if ((intMode == INT_OBJECT || intMode == INT_STAT) && objMode == IOBJ_BUILDSEL) {
		widgGetTabs(psWScreen, IDOBJ_TABFORM, &objMajor, &objMinor);
		// Hide the object form while we select a position.
		//widgHide(psWScreen,IDOBJ_TABFORM);	only need to hide the top form -all else follows
		widgHide(psWScreen,IDOBJ_FORM);
	}*/
}
/* Stop looking for a structure location */
/* Stop looking for a structure location */
unsafe extern "C" fn intStopStructPosition() {
    /* Check there is still a struct position running */
//	if (intMode == INT_OBJECT && objMode == IOBJ_BUILDSEL) {
    if (intMode as libc::c_uint == INT_OBJECT as libc::c_int as libc::c_uint
            ||
            intMode as libc::c_uint ==
                INT_STAT as libc::c_int as libc::c_uint) &&
           objMode as libc::c_uint ==
               IOBJ_BUILDSEL as libc::c_int as libc::c_uint {
        // Reset the stats button
//		widgGetTabs(psWScreen, IDOBJ_FORM, &objMajor, &objMinor);
//		widgEndScreen(psWScreen);
        /*if(DroidIsBuilding((DROID *)psObjSelected)) {
			STRUCTURE *Structure = DroidGetBuildStructure((DROID *)psObjSelected);
			ASSERT( Structure!=NULL,"Bad structure pointer" );
			intSetStats(objStatID,(BASE_STATS*)Structure->pStructureType);
		} else if(DroidGoingToBuild((DROID *)psObjSelected)) {
			intSetStats(objStatID,DroidGetBuildStats((DROID *)psObjSelected));
		} else {
			intSetStats(objStatID,NULL);
		}*/
        //		widgStartScreen(psWScreen);
        objMode = IOBJ_BUILD
    }
    kill3DBuilding();
}
/* See if a structure location has been found */
/* See if a structure location has been found */
unsafe extern "C" fn intGetStructPosition(mut pX: *mut UDWORD,
                                          mut pY: *mut UDWORD) -> BOOL {
    let mut retVal: BOOL = 0 as libc::c_int;
    if display3D != 0 { retVal = found3DBuilding(pX, pY); (retVal) != 0; }
    return retVal;
}
/* Display the widgets for the in game interface */
/* Display the widgets for the in game interface */
#[no_mangle]
pub unsafe extern "C" fn intDisplayWidgets() {
    //STRUCTURE	*psStructure;
    let mut bPlayerHasHQ: BOOL = 0;
    //	int	i;
    /* Including the edit mode here is pretty nasty - but it will get
	 * ripped out for the final version.
	 */
    (intMode as libc::c_uint) == INT_EDIT as libc::c_int as libc::c_uint;
    // God only knows...
    if ReticuleUp != 0 && bInTutorial == 0 { intCheckReticuleButtons(); }
    /*draw the background for the design screen and the Intelligence screen*/
    if intMode as libc::c_uint == INT_DESIGN as libc::c_int as libc::c_uint ||
           intMode as libc::c_uint ==
               INT_INTELMAP as libc::c_int as libc::c_uint {
        // When will they ever learn!!!!
        if bMultiPlayer == 0 {
            DrawBegin();
            screen_RestartBackDrop();
            // Download buffer in system memory to the display back buffer.
            pie_DownloadDisplayBuffer(DisplayBuffer);
            //			DISP_WIDTH, DISP_HEIGHT);
			/*Add the radar to the design screen - only if player has HQ*/
			/*bPlayerHasHQ=FALSE;
			for(psStructure=apsStructLists[selectedPlayer]; psStructure AND !bPlayerHasHQ; psStructure = psStructure->psNext)
			{
				if(psStructure->pStructureType->type == REF_HQ)
				{
					bPlayerHasHQ = TRUE;
				}
			}*/
            bPlayerHasHQ = radarCheckForHQ(selectedPlayer);
            //#ifndef PSX
//			if(bPlayerHasHQ || (bMultiPlayer && (game.type == DMATCH)) )
//#else
            if bPlayerHasHQ != 0 {
                //#endif
                drawRadar();
            }
            // We need to add the console messages to the intelmap for the tutorial so that it can display messages
            if intMode as libc::c_uint ==
                   INT_DESIGN as libc::c_int as libc::c_uint ||
                   bInTutorial != 0 &&
                       intMode as libc::c_uint ==
                           INT_INTELMAP as libc::c_int as libc::c_uint {
                displayConsoleMessages();
            }
            DrawEnd();
        }
    }
    //draw the proximity blips onto the world - done as buttons on the interface now
	//drawProximityBlips();
    StartCursorSnap(&mut InterfaceSnap);
    //19 #ifdef PSX
//19 	if(KeyboardIsActive()) {
//19 		widgDisplayScreen(psKeyScreen);
//19 	}
//19 #endif
    widgDisplayScreen(psWScreen);
    if bLoadSaveUp != 0 { displayLoadSave(); };
}
/* Tell the interface when an object is created
 * - it may have to be added to a screen
 */
/* Tell the interface when an object is created - it may have to be added to a screen */
#[no_mangle]
pub unsafe extern "C" fn intNewObj(mut psObj: *mut BASE_OBJECT) {
    if intMode as libc::c_uint == INT_OBJECT as libc::c_int as libc::c_uint ||
           intMode as libc::c_uint == INT_STAT as libc::c_int as libc::c_uint
       {
        if (objMode as libc::c_uint ==
                IOBJ_BUILD as libc::c_int as libc::c_uint ||
                objMode as libc::c_uint ==
                    IOBJ_BUILDSEL as libc::c_int as libc::c_uint) &&
               (*psObj).type_0 as libc::c_uint ==
                   OBJ_DROID as libc::c_int as libc::c_uint &&
               objSelectFunc.expect("non-null function pointer")(psObj) != 0 {
            objectsChanged = 1 as libc::c_int
        } else if (objMode as libc::c_uint ==
                       IOBJ_RESEARCH as libc::c_int as libc::c_uint ||
                       objMode as libc::c_uint ==
                           IOBJ_MANUFACTURE as libc::c_int as libc::c_uint) &&
                      (*psObj).type_0 as libc::c_uint ==
                          OBJ_STRUCTURE as libc::c_int as libc::c_uint &&
                      objSelectFunc.expect("non-null function pointer")(psObj)
                          != 0 {
            objectsChanged = 1 as libc::c_int
        }
    };
}
// clean up when an object dies
// clean up when an object dies
#[no_mangle]
pub unsafe extern "C" fn intObjectDied(mut objID: UDWORD) {
    let mut psBut: *mut RENDERED_BUTTON = 0 as *mut RENDERED_BUTTON;
    let mut statsID: UDWORD = 0;
    let mut gubbinsID: UDWORD = 0;
    // clear the object button
    psBut = widgGetUserData(psWScreen, objID) as *mut RENDERED_BUTTON;
    if !psBut.is_null() {
        (*psBut).Data = 0 as *mut libc::c_void;
        // and its gubbins
        gubbinsID =
            (3600 as libc::c_int as
                 libc::c_uint).wrapping_add(objID).wrapping_sub(3002 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uint);
        widgSetUserData(psWScreen, gubbinsID, 0 as *mut libc::c_void);
        gubbinsID =
            (3400 as libc::c_int as
                 libc::c_uint).wrapping_add(objID).wrapping_sub(3002 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uint);
        widgSetUserData(psWScreen, gubbinsID, 0 as *mut libc::c_void);
        gubbinsID =
            (3200 as libc::c_int as
                 libc::c_uint).wrapping_add(objID).wrapping_sub(3002 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uint);
        widgSetUserData(psWScreen, gubbinsID, 0 as *mut libc::c_void);
        // clear the stats button
        statsID =
            (3100 as libc::c_int as
                 libc::c_uint).wrapping_add(objID).wrapping_sub(3002 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uint);
        intSetStats(statsID, 0 as *mut BASE_STATS);
        psBut = widgGetUserData(psWScreen, statsID) as *mut RENDERED_BUTTON;
        // and disable it
        widgSetButtonState(psWScreen, statsID, 0x1 as libc::c_int as UDWORD);
        // remove the stat screen if necessary
        if intMode as libc::c_uint == INT_STAT as libc::c_int as libc::c_uint
               && statsID == objStatID {
            intRemoveStatsNoAnim();
            intMode = INT_OBJECT
        }
    };
}
/* Tell the interface a construction droid has finished building */
/* Tell the interface a construction droid has finished building */
#[no_mangle]
pub unsafe extern "C" fn intBuildFinished(mut psDroid: *mut DROID) {
    let mut droidID: UDWORD = 0;
    let mut psCurr: *mut DROID = 0 as *mut DROID;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"intBuildFinished: Invalid droid pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"hci.c\x00" as *const u8 as *const libc::c_char,
              3598 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 17],
                                        &[libc::c_char; 17]>(b"intBuildFinished\x00")).as_ptr(),
              b"PTRVALID(psDroid, sizeof(DROID))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (intMode as libc::c_uint == INT_OBJECT as libc::c_int as libc::c_uint
            ||
            intMode as libc::c_uint ==
                INT_STAT as libc::c_int as libc::c_uint) &&
           objMode as libc::c_uint ==
               IOBJ_BUILD as libc::c_int as libc::c_uint {
        /* Find which button the droid is on and clear it's stats */
        droidID = 0 as libc::c_int as UDWORD;
        psCurr = apsDroidLists[selectedPlayer as usize];
        while !psCurr.is_null() {
            if objSelectFunc.expect("non-null function pointer")(psCurr as
                                                                     *mut BASE_OBJECT)
                   != 0 {
                if psCurr == psDroid {
                    intSetStats(droidID.wrapping_add(3100 as libc::c_int as
                                                         libc::c_uint),
                                0 as *mut BASE_STATS);
                    break ;
                } else { droidID = droidID.wrapping_add(1) }
            }
            psCurr = (*psCurr).psNext
        }
    };
}
/* Tell the interface a construction droid has started building*/
/* Tell the interface a construction droid has started building*/
#[no_mangle]
pub unsafe extern "C" fn intBuildStarted(mut psDroid: *mut DROID) {
    let mut droidID: UDWORD = 0;
    let mut psCurr: *mut DROID = 0 as *mut DROID;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"intBuildStarted: Invalid droid pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"hci.c\x00" as *const u8 as *const libc::c_char,
              3628 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 16],
                                        &[libc::c_char; 16]>(b"intBuildStarted\x00")).as_ptr(),
              b"PTRVALID(psDroid, sizeof(DROID))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (intMode as libc::c_uint == INT_OBJECT as libc::c_int as libc::c_uint
            ||
            intMode as libc::c_uint ==
                INT_STAT as libc::c_int as libc::c_uint) &&
           objMode as libc::c_uint ==
               IOBJ_BUILD as libc::c_int as libc::c_uint {
        /* Find which button the droid is on and clear it's stats */
        droidID = 0 as libc::c_int as UDWORD;
        psCurr = apsDroidLists[selectedPlayer as usize];
        while !psCurr.is_null() {
            if objSelectFunc.expect("non-null function pointer")(psCurr as
                                                                     *mut BASE_OBJECT)
                   != 0 {
                if psCurr == psDroid {
                    intSetStats(droidID.wrapping_add(3100 as libc::c_int as
                                                         libc::c_uint),
                                (*((*psCurr).psTarget as
                                       *mut STRUCTURE)).pStructureType as
                                    *mut BASE_STATS);
                    break ;
                } else { droidID = droidID.wrapping_add(1) }
            }
            psCurr = (*psCurr).psNext
        }
    };
}
/* Are we in build select mode*/
#[no_mangle]
pub unsafe extern "C" fn intBuildSelectMode() -> BOOL {
    return (objMode as libc::c_uint ==
                IOBJ_BUILDSEL as libc::c_int as libc::c_uint) as libc::c_int;
}
/* Are we in demolish select mode*/
#[no_mangle]
pub unsafe extern "C" fn intDemolishSelectMode() -> BOOL {
    return (objMode as libc::c_uint ==
                IOBJ_DEMOLISHSEL as libc::c_int as libc::c_uint) as
               libc::c_int;
}
//is the build interface up?
#[no_mangle]
pub unsafe extern "C" fn intBuildMode() -> BOOL {
    return (objMode as libc::c_uint ==
                IOBJ_BUILD as libc::c_int as libc::c_uint) as libc::c_int;
}
// Just tell the interface the build placement was canceled.
// currently only relevant on the Playstation.
//
#[no_mangle]
pub unsafe extern "C" fn intBuildCancel() {
    // nasty crash ... 12-3-99
//
// when positioning the factory delivery point with the production bar up causes a crash
//
// so what we need to do is check for a building mode and only clear the mode if we are placing a building
    //
    if objMode as libc::c_uint == IOBJ_BUILD as libc::c_int as libc::c_uint ||
           objMode as libc::c_uint ==
               IOBJ_BUILDSEL as libc::c_int as libc::c_uint {
        //
        objMode = IOBJ_NONE
    };
}
//Written to allow demolish order to be added to the queuing system
#[no_mangle]
pub unsafe extern "C" fn intDemolishCancel() {
    if objMode as libc::c_uint ==
           IOBJ_DEMOLISHSEL as libc::c_int as libc::c_uint {
        objMode = IOBJ_NONE
    };
}
/* Tell the interface a factory has completed building ALL droids */
/* Tell the interface a factory has completed building ALL droids */
#[no_mangle]
pub unsafe extern "C" fn intManufactureFinished(mut psBuilding:
                                                    *mut STRUCTURE) {
    let mut structureID: SDWORD = 0;
    let mut psCurr: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psObj: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"intManufactureFinished: Invalid structure pointer\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"hci.c\x00" as *const u8 as *const libc::c_char,
              3707 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 23],
                                        &[libc::c_char; 23]>(b"intManufactureFinished\x00")).as_ptr(),
              b"PTRVALID(psBuilding, sizeof(STRUCTURE))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (intMode as libc::c_uint == INT_OBJECT as libc::c_int as libc::c_uint
            ||
            intMode as libc::c_uint ==
                INT_STAT as libc::c_int as libc::c_uint) &&
           objMode as libc::c_uint ==
               IOBJ_MANUFACTURE as libc::c_int as libc::c_uint {
        /* Find which button the structure is on and clear it's stats */
        structureID = 0 as libc::c_int;
        numObjects = 0 as libc::c_int;
        memset(apsObjectList as *mut libc::c_void, 0 as libc::c_int,
               (::std::mem::size_of::<*mut BASE_OBJECT>() as
                    libc::c_ulong).wrapping_mul(15 as libc::c_int as
                                                    libc::c_uint));
        //for (psCurr = apsStructLists[selectedPlayer]; psCurr; psCurr = psCurr->psNext)
        psCurr = interfaceStructList();
        while !psCurr.is_null() {
            if objSelectFunc.expect("non-null function pointer")(psCurr as
                                                                     *mut BASE_OBJECT)
                   != 0 {
                //the list is ordered now so we have to get all possible entries and sort it before checking if this is the one!
                let ref mut fresh9 =
                    *apsObjectList.offset(numObjects as isize);
                *fresh9 = psCurr as *mut BASE_OBJECT;
                numObjects += 1
            }
            // make sure the list doesn't overflow
            if numObjects >= 15 as libc::c_int { break ; }
            psCurr = (*psCurr).psNext
        }
        //order the list
        orderFactories();
        //now look thru the list to see which one corresponds to the factory that has just finished
        structureID = 0 as libc::c_int;
        psObj = *apsObjectList.offset(structureID as isize);
        while structureID < numObjects {
            if psObj as *mut STRUCTURE == psBuilding {
                intSetStats((structureID + 3100 as libc::c_int) as UDWORD,
                            0 as *mut BASE_STATS);
                // No looped production on PSX.
        		//clear the loop button if interface is up
                if !widgGetFromID(psWScreen,
                                  4403 as libc::c_int as UDWORD).is_null() {
                    widgSetButtonState(psWScreen,
                                       4403 as libc::c_int as UDWORD,
                                       0 as libc::c_int as UDWORD);
                }
                break ;
            } else { structureID += 1 }
        }
    };
}
/* Tell the interface a research facility has completed a topic */
/* Tell the interface a research facility has completed a topic */
#[no_mangle]
pub unsafe extern "C" fn intResearchFinished(mut psBuilding: *mut STRUCTURE) {
    //SDWORD		    structureID;
	//STRUCTURE       *psCurr;
    //BASE_OBJECT     *psObj;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"intResearchFinished: Invalid structure pointer\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"hci.c\x00" as *const u8 as *const libc::c_char,
              3762 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 20],
                                        &[libc::c_char; 20]>(b"intResearchFinished\x00")).as_ptr(),
              b"PTRVALID(psBuilding, sizeof(STRUCTURE))\x00" as *const u8 as
                  *const libc::c_char);
    };
    // just do a screen refresh
    intRefreshScreen();
    /*	if ((intMode == INT_OBJECT || intMode == INT_STAT) &&
		(objMode == IOBJ_RESEARCH))
	{
		// Find which button the structure is on and clear it's stats
		structureID = 0;
    	numObjects = 0;
	    memset(apsObjectList, 0, sizeof(BASE_OBJECT *) * MAX_OBJECTS);
		//for (psCurr = apsStructLists[selectedPlayer]; psCurr; psCurr = psCurr->psNext)
		for (psCurr = interfaceStructList(); psCurr; psCurr = psCurr->psNext)
		{
			if (objSelectFunc((BASE_OBJECT *)psCurr))
			{
                //the list is ordered now so we have to get all possible entries and sort it before checking if this is the one!
                apsObjectList[numObjects] = (BASE_OBJECT *)psCurr;
			    numObjects++;
            }
			// make sure the list doesn't overflow
			if (numObjects >= MAX_OBJECTS)
			{
				break;
			}
        }
        //order the list
        orderResearch();

        //now look thru the list to see which one corresponds to the factory that has just finished
        structureID = 0;
        for (psObj = apsObjectList[structureID]; structureID < numObjects; structureID++)
        {
            if ((STRUCTURE *)psObj == psBuilding)
			{
				intSetStats(structureID + IDOBJ_STATSTART, NULL);
				break;
			}
		}
	}*/
    // refresh the research interface to update with new topics.
	//intRefreshScreen();
}
/* Do the annoying calculation for how many forms are needed
 * given the total number of buttons and the number of
 * buttons per page.
 * A simple div just doesn't quite do it....
 */
#[no_mangle]
pub unsafe extern "C" fn numForms(mut total: UDWORD, mut perForm: UDWORD)
 -> UWORD {
    /* If the buttons fit exactly, don't have to add one */
    if total != 0 as libc::c_int as libc::c_uint &&
           total.wrapping_rem(perForm) == 0 as libc::c_int as libc::c_uint {
        return total.wrapping_div(perForm) as UWORD
    }
    /* Otherwise add one to the div */
    return total.wrapping_div(perForm).wrapping_add(1 as libc::c_int as
                                                        libc::c_uint) as
               UWORD;
}
// Add...
//	Droid order button	( always )
//	Transporter button,	( if transporter available )
//	Re-enforcements button, ( if in off world map and re-enforcements available )
//
// Currently only used on Playstation.
//
#[no_mangle]
pub unsafe extern "C" fn intAddReticuleExtras() -> BOOL {
    return 1 as libc::c_int;
}
/* Add the reticule widgets to the widget screen */
#[no_mangle]
pub unsafe extern "C" fn _intAddReticule() -> BOOL {
    if ReticuleUp == 0 as libc::c_int {
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
        /* Create the basic form */
        memset(&mut sFormInit as *mut W_FORMINIT as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<W_FORMINIT>() as libc::c_ulong);
        sFormInit.formID = 0 as libc::c_int as UDWORD;
        sFormInit.id = 1 as libc::c_int as UDWORD;
        sFormInit.style = 0 as libc::c_int as UDWORD;
        sFormInit.x = 23 as libc::c_int as SWORD;
        sFormInit.y =
            (324 as libc::c_int as
                 libc::c_uint).wrapping_add(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        libc::c_uint))
                as SWORD;
        sFormInit.width = 132 as libc::c_int as UWORD;
        sFormInit.height = 132 as libc::c_int as UWORD;
        sFormInit.pDisplay =
            Some(intDisplayPlainForm as
                     unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                          _: UDWORD, _: *mut UDWORD) -> ());
        if widgAddForm(psWScreen, &mut sFormInit) == 0 {
            return 0 as libc::c_int
        }
        /* Now add the buttons */
        //set up default button data
        memset(&mut sButInit as *mut W_BUTINIT as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<W_BUTINIT>() as libc::c_ulong);
        sButInit.formID = 1 as libc::c_int as UDWORD;
        sButInit.id = 9 as libc::c_int as UDWORD;
        sButInit.width = 25 as libc::c_int as UWORD;
        sButInit.height = 28 as libc::c_int as UWORD;
        sButInit.FontID = WFont;
        //add buttons as required...
        //options button
        sButInit.style = 0 as libc::c_int as UDWORD;
        SetReticuleButPos(RETBUT_COMMAND as libc::c_int as UWORD,
                          &mut sButInit);
        //		sButInit.x = 19+RETXOFFSET;
//		sButInit.y = 35+RETYOFFSET;
	//	sButInit.pText = "O";
        sButInit.pTip =
            strresGetString(psStringRes,
                            STR_RET_COMMAND as libc::c_int as UDWORD);
        sButInit.pDisplay =
            Some(intDisplayReticuleButton as
                     unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                          _: UDWORD, _: *mut UDWORD) -> ());
        sButInit.pUserData =
            IMAGE_COMMANDDROID_UP as libc::c_int as *mut libc::c_void;
        if widgAddButton(psWScreen, &mut sButInit) == 0 {
            return 0 as libc::c_int
        }
        /* Intelligence Map button - this needs to respond to RMB as well*/
        sButInit.style = (0 as libc::c_int | 0x20 as libc::c_int) as UDWORD;
        sButInit.id = 6 as libc::c_int as UDWORD;
        SetReticuleButPos(RETBUT_INTELMAP as libc::c_int as UWORD,
                          &mut sButInit);
        //		sButInit.x = 19+RETXOFFSET;
//		sButInit.y = 70+RETYOFFSET;
	//	sButInit.pText = "S";
        sButInit.pTip =
            strresGetString(psStringRes,
                            STR_RET_INTELLIGENCE as libc::c_int as UDWORD);
        sButInit.pDisplay =
            Some(intDisplayReticuleButton as
                     unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                          _: UDWORD, _: *mut UDWORD) -> ());
        sButInit.pUserData =
            IMAGE_INTELMAP_UP as libc::c_int as *mut libc::c_void;
        if widgAddButton(psWScreen, &mut sButInit) == 0 {
            return 0 as libc::c_int
        }
        /* Manufacture button */
        sButInit.style = 0 as libc::c_int as UDWORD;
        sButInit.id = 4 as libc::c_int as UDWORD;
        SetReticuleButPos(RETBUT_FACTORY as libc::c_int as UWORD,
                          &mut sButInit);
        //		sButInit.x = 53+RETXOFFSET;
//		sButInit.y = 17+RETYOFFSET;
	//	sButInit.pText = "M";
        sButInit.pTip =
            strresGetString(psStringRes,
                            STR_RET_MANUFACTURE as libc::c_int as UDWORD);
        sButInit.pDisplay =
            Some(intDisplayReticuleButton as
                     unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                          _: UDWORD, _: *mut UDWORD) -> ());
        sButInit.pUserData =
            IMAGE_MANUFACTURE_UP as libc::c_int as *mut libc::c_void;
        if widgAddButton(psWScreen, &mut sButInit) == 0 {
            return 0 as libc::c_int
        }
        /* Design button */
        sButInit.style = 0 as libc::c_int as UDWORD;
        sButInit.id = 7 as libc::c_int as UDWORD;
        SetReticuleButPos(RETBUT_DESIGN as libc::c_int as UWORD,
                          &mut sButInit);
        //		sButInit.x = 53+RETXOFFSET;
//		sButInit.y = 88+RETYOFFSET;
	//	sButInit.pText = "D";
        sButInit.pTip =
            strresGetString(psStringRes,
                            STR_RET_DESIGN as libc::c_int as UDWORD);
        sButInit.pDisplay =
            Some(intDisplayReticuleButton as
                     unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                          _: UDWORD, _: *mut UDWORD) -> ());
        sButInit.pUserData =
            IMAGE_DESIGN_UP as libc::c_int as *mut libc::c_void;
        if widgAddButton(psWScreen, &mut sButInit) == 0 {
            return 0 as libc::c_int
        }
        /* Research button */
        sButInit.style = 0 as libc::c_int as UDWORD;
        sButInit.id = 5 as libc::c_int as UDWORD;
        SetReticuleButPos(RETBUT_RESEARCH as libc::c_int as UWORD,
                          &mut sButInit);
        //		sButInit.x = 87+RETXOFFSET;
//		sButInit.y = 35+RETYOFFSET;
	//	sButInit.pText = "R";
        sButInit.pTip =
            strresGetString(psStringRes,
                            STR_RET_RESEARCH as libc::c_int as UDWORD);
        sButInit.pDisplay =
            Some(intDisplayReticuleButton as
                     unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                          _: UDWORD, _: *mut UDWORD) -> ());
        sButInit.pUserData =
            IMAGE_RESEARCH_UP as libc::c_int as *mut libc::c_void;
        if widgAddButton(psWScreen, &mut sButInit) == 0 {
            return 0 as libc::c_int
        }
        /* Build button */
        sButInit.style = 0 as libc::c_int as UDWORD;
        sButInit.id = 3 as libc::c_int as UDWORD;
        SetReticuleButPos(RETBUT_BUILD as libc::c_int as UWORD,
                          &mut sButInit);
        //		sButInit.x = 87+RETXOFFSET;
//		sButInit.y = 70+RETYOFFSET;
	//	sButInit.pText = "B";
        sButInit.pTip =
            strresGetString(psStringRes,
                            STR_RET_BUILD as libc::c_int as UDWORD);
        sButInit.pDisplay =
            Some(intDisplayReticuleButton as
                     unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                          _: UDWORD, _: *mut UDWORD) -> ());
        sButInit.pUserData =
            IMAGE_BUILD_UP as libc::c_int as *mut libc::c_void;
        if widgAddButton(psWScreen, &mut sButInit) == 0 {
            return 0 as libc::c_int
        }
        /* Cancel button */
        sButInit.style = 0 as libc::c_int as UDWORD;
        sButInit.id = 8 as libc::c_int as UDWORD;
        SetReticuleButPos(RETBUT_CANCEL as libc::c_int as UWORD,
                          &mut sButInit);
        //		sButInit.x = 48+RETXOFFSET;
//		sButInit.y = 49+RETYOFFSET;
        sButInit.width = (25 as libc::c_int + 10 as libc::c_int) as UWORD;
        sButInit.height = (28 as libc::c_int + 8 as libc::c_int) as UWORD;
        //	sButInit.pText = "C";
        sButInit.pTip =
            strresGetString(psStringRes,
                            STR_RET_CLOSE as libc::c_int as UDWORD);
        sButInit.pDisplay =
            Some(intDisplayReticuleButton as
                     unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                          _: UDWORD, _: *mut UDWORD) -> ());
        sButInit.pUserData =
            IMAGE_CANCEL_UP as libc::c_int as *mut libc::c_void;
        //#ifdef PSX
	//	sButInit.pCallback = intUpdateReticuleButton;
	//#endif
        if widgAddButton(psWScreen, &mut sButInit) == 0 {
            return 0 as libc::c_int
        }
        //	intCheckReticuleButtons();
        ReticuleUp = 1 as libc::c_int
    } // remove reticule
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn intReticuleIsUp() -> BOOL { return ReticuleUp; }
#[no_mangle]
pub unsafe extern "C" fn intRemoveReticule() {
    if ReticuleUp == 1 as libc::c_int {
        widgDelete(psWScreen, 1 as libc::c_int as UDWORD);
        ReticuleUp = 0 as libc::c_int
    };
}
//toggles the Power Bar display on and off
//toggles the Power Bar display on and off
#[no_mangle]
pub unsafe extern "C" fn togglePowerBar() {
    //toggle the flag
    powerBarUp = (powerBarUp == 0) as libc::c_int;
    if powerBarUp != 0 { intShowPowerBar(); } else { intHidePowerBar(); };
}
/* *****************Power Bar Stuff!**************/
/* Add the power bars */
/* Add the power bars to the screen */
unsafe extern "C" fn intAddPower() -> BOOL {
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
    memset(&mut sBarInit as *mut W_BARINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_BARINIT>() as libc::c_ulong);
    /* Add the trough bar */
    sBarInit.formID = 0 as libc::c_int as UDWORD; //IDPOW_FORM;
    sBarInit.id = 102 as libc::c_int as UDWORD;
    //start the power bar off in view (default)
    sBarInit.style = 1 as libc::c_int as UDWORD;
    sBarInit.orientation = 0x1 as libc::c_int as UWORD;
    sBarInit.x =
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
    sBarInit.y =
        (324 as libc::c_int as
             libc::c_uint).wrapping_add(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint)).wrapping_add(115
                                                                                                                    as
                                                                                                                    libc::c_int
                                                                                                                    as
                                                                                                                    libc::c_uint).wrapping_add(6
                                                                                                                                                   as
                                                                                                                                                   libc::c_int
                                                                                                                                                   as
                                                                                                                                                   libc::c_uint)
            as SWORD;
    sBarInit.width = 308 as libc::c_int as UWORD;
    sBarInit.height =
        iV_GetImageHeight(IntImages,
                          IMAGE_PBAR_EMPTY as libc::c_int as UWORD);
    sBarInit.sCol.red = 0xcc as libc::c_int as UBYTE;
    sBarInit.sCol.green = 0 as libc::c_int as UBYTE;
    sBarInit.sCol.blue = 0 as libc::c_int as UBYTE;
    sBarInit.pDisplay =
        Some(intDisplayPowerBar as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    sBarInit.iRange =
        (5 as libc::c_int * 100 as libc::c_int /
             (60 as libc::c_int - 8 as libc::c_int)) as UWORD;
    sBarInit.pTip =
        strresGetString(psStringRes, STR_INT_POWER as libc::c_int as UDWORD);
    if widgAddBarGraph(psWScreen, &mut sBarInit) == 0 {
        return 0 as libc::c_int
    }
    powerBarUp = 1 as libc::c_int;
    return 1 as libc::c_int;
}
// update shadow...
/* Remove the power bar widgets */
/*void intRemovePower(void)
{
	if (powerBarUp)
	{
		widgDelete(psWScreen, IDPOW_POWERBAR_T);
		powerBarUp = FALSE;
	}
}*/
/* Set the shadow power for the selected player */
// Now just sets the global variable ManuPower which is used in the power bar display callback. PD
#[no_mangle]
pub unsafe extern "C" fn intSetShadowPower(mut quantity: UDWORD) {
    ManuPower = quantity;
}
/* Add the options widgets to the widget screen */
#[no_mangle]
pub unsafe extern "C" fn _intAddOptions() -> BOOL {
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
    let mut sEdInit: W_EDBINIT =
        W_EDBINIT{formID: 0,
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
                  FontID: 0,
                  pBoxDisplay: None,
                  pFontDisplay: None,};
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
    let mut player: UDWORD = 0;
    //	STRING		aText[WIDG_MAXSTR];//, aTip[WIDG_MAXSTR];
    //	widgEndScreen(psWScreen);
    memset(&mut sFormInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_FORMINIT>() as libc::c_ulong);
    memset(&mut sLabInit as *mut W_LABINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_LABINIT>() as libc::c_ulong);
    memset(&mut sButInit as *mut W_BUTINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_BUTINIT>() as libc::c_ulong);
    memset(&mut sEdInit as *mut W_EDBINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_EDBINIT>() as libc::c_ulong);
    /* Add the option form */
    sFormInit.formID = 0 as libc::c_int as UDWORD;
    sFormInit.id = 1000 as libc::c_int as UDWORD;
    sFormInit.style = 0 as libc::c_int as UDWORD;
    sFormInit.x = (640 as libc::c_int - 300 as libc::c_int) as SWORD;
    sFormInit.y = 20 as libc::c_int as SWORD;
    sFormInit.width = 275 as libc::c_int as UWORD;
    sFormInit.height = 350 as libc::c_int as UWORD;
    if widgAddForm(psWScreen, &mut sFormInit) == 0 { return 0 as libc::c_int }
    // set the interface mode
    intMode = INT_OPTION;
    /* Add the Option screen label */
    sLabInit.formID = 1000 as libc::c_int as UDWORD;
    sLabInit.id = 1007 as libc::c_int as UDWORD;
    sLabInit.style = 0 as libc::c_int as UDWORD;
    sLabInit.x = 5 as libc::c_int as SWORD;
    sLabInit.y = 5 as libc::c_int as SWORD;
    sLabInit.width = 60 as libc::c_int as UWORD;
    sLabInit.height = 20 as libc::c_int as UWORD;
    sLabInit.pText =
        b"Options\x00" as *const u8 as *const libc::c_char as *mut STRING;
    sLabInit.FontID = WFont;
    if widgAddLabel(psWScreen, &mut sLabInit) == 0 { return 0 as libc::c_int }
    /* Add the close box */
    sButInit.formID = 1000 as libc::c_int as UDWORD;
    sButInit.id = 1006 as libc::c_int as UDWORD;
    sButInit.style = 0 as libc::c_int as UDWORD;
    sButInit.x =
        (275 as libc::c_int - 5 as libc::c_int - 15 as libc::c_int) as SWORD;
    sButInit.y = 5 as libc::c_int as SWORD;
    sButInit.width = 15 as libc::c_int as UWORD;
    sButInit.height = 15 as libc::c_int as UWORD;
    sButInit.FontID = WFont;
    sButInit.pText = pCloseText.as_mut_ptr();
    sButInit.pTip =
        strresGetString(psStringRes, STR_MISC_CLOSE as libc::c_int as UDWORD);
    if widgAddButton(psWScreen, &mut sButInit) == 0 {
        return 0 as libc::c_int
    }
    /* Add the edit button */
    sButInit.formID = 1000 as libc::c_int as UDWORD;
    sButInit.id = 1005 as libc::c_int as UDWORD;
    sButInit.x = 5 as libc::c_int as SWORD;
    sButInit.y = 100 as libc::c_int as SWORD;
    sButInit.width = 60 as libc::c_int as UWORD;
    sButInit.height = 20 as libc::c_int as UWORD;
    sButInit.pText =
        b"Edit\x00" as *const u8 as *const libc::c_char as *mut STRING;
    sButInit.pTip =
        b"Start Edit Mode\x00" as *const u8 as *const libc::c_char as
            *mut STRING;
    if widgAddButton(psWScreen, &mut sButInit) == 0 {
        return 0 as libc::c_int
    }
    /* Add the add object buttons */
    sButInit.id = 1037 as libc::c_int as UDWORD;
    sButInit.x =
        (sButInit.x as libc::c_int + (5 as libc::c_int + 60 as libc::c_int))
            as SWORD;
    sButInit.pText =
        b"Unit\x00" as *const u8 as *const libc::c_char as *mut STRING;
    sButInit.pTip =
        b"Place Unit on map\x00" as *const u8 as *const libc::c_char as
            *mut STRING;
    if widgAddButton(psWScreen, &mut sButInit) == 0 {
        return 0 as libc::c_int
    }
    sButInit.id = 1038 as libc::c_int as UDWORD;
    sButInit.x =
        (sButInit.x as libc::c_int + (5 as libc::c_int + 60 as libc::c_int))
            as SWORD;
    sButInit.pText =
        b"Struct\x00" as *const u8 as *const libc::c_char as *mut STRING;
    sButInit.pTip =
        b"Place Structures on map\x00" as *const u8 as *const libc::c_char as
            *mut STRING;
    if widgAddButton(psWScreen, &mut sButInit) == 0 {
        return 0 as libc::c_int
    }
    sButInit.id = 1039 as libc::c_int as UDWORD;
    sButInit.x =
        (sButInit.x as libc::c_int + (5 as libc::c_int + 60 as libc::c_int))
            as SWORD;
    sButInit.pText =
        b"Feat\x00" as *const u8 as *const libc::c_char as *mut STRING;
    sButInit.pTip =
        b"Place Features on map\x00" as *const u8 as *const libc::c_char as
            *mut STRING;
    if widgAddButton(psWScreen, &mut sButInit) == 0 {
        return 0 as libc::c_int
    }
    /* Add the quit button */
    sButInit.formID = 1000 as libc::c_int as UDWORD;
    sButInit.id = 1031 as libc::c_int as UDWORD;
    sButInit.x = 5 as libc::c_int as SWORD;
    sButInit.y =
        (350 as libc::c_int - 5 as libc::c_int - 20 as libc::c_int) as SWORD;
    sButInit.width =
        (275 as libc::c_int - 5 as libc::c_int * 2 as libc::c_int) as UWORD;
    sButInit.height = 20 as libc::c_int as UWORD;
    sButInit.pText =
        b"Quit\x00" as *const u8 as *const libc::c_char as *mut STRING;
    sButInit.pTip =
        strresGetString(psStringRes, STR_MISC_QUIT as libc::c_int as UDWORD);
    if widgAddButton(psWScreen, &mut sButInit) == 0 {
        return 0 as libc::c_int
    }
    /* Add the player form */
    sFormInit.formID = 1000 as libc::c_int as UDWORD;
    sFormInit.id = 1008 as libc::c_int as UDWORD;
    sFormInit.style = 0 as libc::c_int as UDWORD;
    sFormInit.x = 5 as libc::c_int as SWORD;
    sFormInit.y = 150 as libc::c_int as SWORD;
    sFormInit.width =
        (275 as libc::c_int - 5 as libc::c_int * 2 as libc::c_int) as UWORD;
    sFormInit.height =
        (20 as libc::c_int * 3 as libc::c_int +
             5 as libc::c_int * 4 as libc::c_int) as UWORD;
    if widgAddForm(psWScreen, &mut sFormInit) == 0 { return 0 as libc::c_int }
    /* Add the player label */
    sLabInit.formID = 1008 as libc::c_int as UDWORD;
    sLabInit.id = 1009 as libc::c_int as UDWORD;
    sLabInit.style = 0 as libc::c_int as UDWORD;
    sLabInit.x = 5 as libc::c_int as SWORD;
    sLabInit.y = 5 as libc::c_int as SWORD;
    sLabInit.pText =
        b"Current Player:\x00" as *const u8 as *const libc::c_char as
            *mut STRING;
    sLabInit.FontID = WFont;
    if widgAddLabel(psWScreen, &mut sLabInit) == 0 { return 0 as libc::c_int }
    /* Add the player buttons */
    sButInit.formID = 1008 as libc::c_int as UDWORD;
    sButInit.id = 1010 as libc::c_int as UDWORD;
    sButInit.style = 0 as libc::c_int as UDWORD;
    sButInit.x = 5 as libc::c_int as SWORD;
    sButInit.y =
        (20 as libc::c_int + 5 as libc::c_int * 2 as libc::c_int) as SWORD;
    sButInit.width = 60 as libc::c_int as UWORD;
    sButInit.height = 20 as libc::c_int as UWORD;
    sButInit.FontID = WFont;
    player = 0 as libc::c_int as UDWORD;
    while player < 8 as libc::c_int as libc::c_uint {
        sButInit.pText = apPlayerText[player as usize];
        sButInit.pTip = apPlayerTip[player as usize];
        if widgAddButton(psWScreen, &mut sButInit) == 0 {
            return 0 as libc::c_int
        }
        /* Update the initialisation structure for the next button */
        sButInit.id =
            (sButInit.id as
                 libc::c_uint).wrapping_add(1 as libc::c_int as libc::c_uint)
                as UDWORD as UDWORD;
        sButInit.x =
            (sButInit.x as libc::c_int +
                 (60 as libc::c_int + 5 as libc::c_int)) as SWORD;
        if sButInit.x as libc::c_int + 60 as libc::c_int + 5 as libc::c_int >
               275 as libc::c_int - 5 as libc::c_int * 2 as libc::c_int {
            sButInit.x = 5 as libc::c_int as SWORD;
            sButInit.y =
                (sButInit.y as libc::c_int +
                     (20 as libc::c_int + 5 as libc::c_int)) as SWORD
        }
        player = player.wrapping_add(1)
    }
    //	widgStartScreen(psWScreen);
    widgSetButtonState(psWScreen,
                       (1010 as libc::c_int as
                            libc::c_uint).wrapping_add(selectedPlayer),
                       0x2 as libc::c_int as UDWORD);
    return 1 as libc::c_int;
}
/* Remove the options widgets from the widget screen */
/* Remove the options widgets from the widget screen */
unsafe extern "C" fn intRemoveOptions() {
    //	widgEndScreen(psWScreen);
    widgDelete(psWScreen, 1000 as libc::c_int as UDWORD);
    //	widgStartScreen(psWScreen);
}
/* Add the object screen widgets to the widget screen.
 * select is a pointer to a function that returns true when the object is
 * to be added to the screen.
 * getStats is a pointer to a function that returns the appropriate stats
 * for the object.
 * If psSelected != NULL it specifies which object should be hilited.
 */
unsafe extern "C" fn _intAddObjectWindow(mut psObjects: *mut BASE_OBJECT,
                                         mut psSelected: *mut BASE_OBJECT,
                                         mut bForceStats: BOOL) -> BOOL {
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
    let mut sBarInit2: W_BARINIT =
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
    let mut displayForm: UDWORD = 0;
    let mut i: UDWORD = 0;
    let mut statID_0: UDWORD = 0 as libc::c_int as UDWORD;
    let mut objLoop: SDWORD = 0;
    let mut psObj: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut psFirst: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut psStats: *mut BASE_STATS = 0 as *mut BASE_STATS;
    let mut BufferID: SDWORD = 0;
    let mut Droid: *mut DROID = 0 as *mut DROID;
    let mut Structure: *mut STRUCTURE = 0 as *mut STRUCTURE;
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
    let mut sLabIntObjText: W_LABINIT =
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
    let mut sLabInitCmdExp: W_LABINIT =
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
    let mut sLabInitCmdFac: W_LABINIT =
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
    let mut sLabInitCmdFac2: W_LABINIT =
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
    //	W_LABINIT		sLabInitCmdFacts;
    let mut IsFactory: BOOL = 0;
    let mut Animate: BOOL = 1 as libc::c_int;
    let mut FormX: UWORD = 0;
    let mut FormY: UWORD = 0;
    if psSelected.is_null() || 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"intAddObject: Invalid object pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if psSelected.is_null() || 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"hci.c\x00" as *const u8 as *const libc::c_char,
              4486 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 20],
                                        &[libc::c_char; 20]>(b"_intAddObjectWindow\x00")).as_ptr(),
              b"psSelected == NULL || PTRVALID(psSelected, sizeof(BASE_OBJECT))\x00"
                  as *const u8 as *const libc::c_char);
    };
    //#ifdef PSX
// // Is the stats form up?
//	if(widgGetFromID(psWScreen,IDSTAT_FORM) != NULL) {
//		intRemoveStatsNoAnim();
//		DBPRINTF(("Removing stats form\n");
//	}
//#endif
    // Is the form already up?
    if !widgGetFromID(psWScreen, 3000 as libc::c_int as UDWORD).is_null() {
        intRemoveObjectNoAnim();
        Animate = 0 as libc::c_int
    } else {
        // reset the object position array
        memset(asJumpPos.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<[POINT; 22]>() as libc::c_ulong);
    }
    Animate = 0 as libc::c_int;
    ClearObjectBuffers();
    ClearTopicBuffers();
    memset(&mut sFormInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_FORMINIT>() as libc::c_ulong);
    memset(&mut sBFormInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_FORMINIT>() as libc::c_ulong);
    memset(&mut sBFormInit2 as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_FORMINIT>() as libc::c_ulong);
    memset(&mut sBarInit as *mut W_BARINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_BARINIT>() as libc::c_ulong);
    /* See how many objects the player has */
    numObjects = 0 as libc::c_int;
    psFirst = 0 as *mut BASE_OBJECT;
    memset(apsObjectList as *mut libc::c_void, 0 as libc::c_int,
           (::std::mem::size_of::<*mut BASE_OBJECT>() as
                libc::c_ulong).wrapping_mul(15 as libc::c_int as
                                                libc::c_uint));
    psObj = psObjects;
    while !psObj.is_null() {
        if objSelectFunc.expect("non-null function pointer")(psObj) != 0 {
            let ref mut fresh10 = *apsObjectList.offset(numObjects as isize);
            *fresh10 = psObj;
            numObjects += 1;
            if numObjects == 1 as libc::c_int { psFirst = psObj }
            // make sure the list doesn't overflow
            if numObjects >= 15 as libc::c_int { break ; }
        }
        psObj = (*psObj).psNext
    }
    if numObjects == 0 as libc::c_int {
        // No objects so close the stats window if it's up...
        if !widgGetFromID(psWScreen, 4000 as libc::c_int as UDWORD).is_null()
           {
            //DBPRINTF(("No objects, intRemoveStatsNoAnim\n");
            intRemoveStatsNoAnim();
        }
        // and return.
        return 0 as libc::c_int
    }
    /*if psSelected != NULL then check its in the list of suitable objects for
    this instance of the interface - this could happen when a structure is upgraded*/
    objLoop = 0 as libc::c_int;
    if !psSelected.is_null() {
        objLoop = 0 as libc::c_int;
        while objLoop < numObjects {
            if psSelected == *apsObjectList.offset(objLoop as isize) {
                break ;
            }
            objLoop += 1
        }
    }
    //if have reached the end of the loop and not quit out, then can't have found the selected object in the list
    if objLoop == numObjects {
        //initialise psSelected so gets set up with an iten in the list
        psSelected = 0 as *mut BASE_OBJECT
    }
    //order the objects according to what they are
    orderObjectInterface();
    // wont ever get here cause if theres no research facility then the research reticule button
// is disabled so commented out.
//	if (numObjects == 0 && objMode == IOBJ_RESEARCH)
//	{
//		audio_QueueTrack(ID_SOUND_RESEARCH_FAC_REQ);
//		return FALSE;
//	}
    // set the selected object if necessary
    if psSelected.is_null() {
        //first check if there is an object selected of the required type
        match objMode as libc::c_uint {
            20 => {
                psSelected =
                    intCheckForStructure(REF_RESEARCH as libc::c_int as
                                             UDWORD) as *mut BASE_OBJECT
            }
            19 => {
                psSelected =
                    intCheckForStructure(REF_FACTORY as libc::c_int as UDWORD)
                        as *mut BASE_OBJECT;
                //if haven't got a Factory, check for specific types of factory
                if psSelected.is_null() {
                    psSelected =
                        intCheckForStructure(REF_CYBORG_FACTORY as libc::c_int
                                                 as UDWORD) as
                            *mut BASE_OBJECT
                }
                if psSelected.is_null() {
                    psSelected =
                        intCheckForStructure(REF_VTOL_FACTORY as libc::c_int
                                                 as UDWORD) as
                            *mut BASE_OBJECT
                }
            }
            16 => {
                psSelected =
                    intCheckForDroid(DROID_CONSTRUCT as libc::c_int as UDWORD)
                        as *mut BASE_OBJECT;
                if psSelected.is_null() {
                    psSelected =
                        intCheckForDroid(DROID_CYBORG_CONSTRUCT as libc::c_int
                                             as UDWORD) as *mut BASE_OBJECT
                }
            }
            21 => {
                psSelected =
                    intCheckForDroid(DROID_COMMAND as libc::c_int as UDWORD)
                        as *mut BASE_OBJECT
            }
            _ => { }
        }
        if psSelected.is_null() {
            if !apsPreviousObj[objMode as usize].is_null() &&
                   (*apsPreviousObj[objMode as usize]).player as libc::c_uint
                       == selectedPlayer {
                psSelected = apsPreviousObj[objMode as usize];
                //#ifndef PSX
		//make sure this matches in game once decided - DON'T!
		//clearSelection();
		//psSelected->selected = TRUE;
//#endif
                //it is possible for a structure to change status - building of modules
                if (*psSelected).type_0 as libc::c_uint ==
                       OBJ_STRUCTURE as libc::c_int as libc::c_uint {
                    if (*(psSelected as *mut STRUCTURE)).status as libc::c_int
                           != SS_BUILT as libc::c_int {
                        //structure not complete so just set selected to the first valid object
                        psSelected = psFirst
                    }
                }
            } else { psSelected = psFirst }
        }
    }
    /* Reset the current object and store the current list */
    psObjSelected = 0 as *mut BASE_OBJECT;
    psObjList = psObjects;
    /* Create the basic form */
    sFormInit.formID = 0 as libc::c_int as UDWORD;
    sFormInit.id = 3000 as libc::c_int as UDWORD;
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
    FormX = sFormInit.x as UWORD;
    FormY = sFormInit.y as UWORD;
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
    //#if defined(PSX) && defined(MOVETOFORM)
// // Position the mouse in the center of this form.
//	SetCurrentSnapFormID(&InterfaceSnap,sFormInit.id);
//#endif
    /* Add the close button */
    memset(&mut sButInit as *mut W_BUTINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_BUTINIT>() as libc::c_ulong);
    sButInit.formID = 3000 as libc::c_int as UDWORD;
    sButInit.id = 3001 as libc::c_int as UDWORD;
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
    /*add the tabbed form */
    memset(&mut sFormInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_FORMINIT>() as libc::c_ulong);
    sFormInit.formID = 3000 as libc::c_int as UDWORD;
    sFormInit.id = 3500 as libc::c_int as UDWORD;
    sFormInit.style = 1 as libc::c_int as UDWORD;
    sFormInit.x = 2 as libc::c_int as SWORD;
    sFormInit.y = 6 as libc::c_int as SWORD;
    sFormInit.width = 316 as libc::c_int as UWORD;
    sFormInit.height = 112 as libc::c_int as UWORD;
    sFormInit.numMajor =
        numForms(((60 as libc::c_int + 2 as libc::c_int) * numObjects) as
                     UDWORD,
                 (316 as libc::c_int - 2 as libc::c_int) as UDWORD);
    sFormInit.majorPos = 1 as libc::c_int as UWORD;
    sFormInit.minorPos = 0 as libc::c_int as UWORD;
    sFormInit.majorSize = 26 as libc::c_int as UWORD;
    sFormInit.majorOffset = 2 as libc::c_int as SWORD;
    sFormInit.tabVertOffset = (11 as libc::c_int / 2 as libc::c_int) as SWORD;
    sFormInit.tabMajorThickness = 11 as libc::c_int as UWORD;
    sFormInit.tabMajorGap = 2 as libc::c_int as UWORD;
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
    i = 0 as libc::c_int as UDWORD;
    while i < sFormInit.numMajor as libc::c_uint {
        sFormInit.aNumMinors[i as usize] = 1 as libc::c_int as UWORD;
        i = i.wrapping_add(1)
    }
    if widgAddForm(psWScreen, &mut sFormInit) == 0 { return 0 as libc::c_int }
    /* Store the number of tabs */
    objNumTabs = sFormInit.numMajor;
    /* Add the object and stats buttons */
    sBFormInit.formID = 3500 as libc::c_int as UDWORD;
    sBFormInit.id = 3002 as libc::c_int as UDWORD;
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
    sBFormInit2.id = 3100 as libc::c_int as UDWORD;
    sBFormInit2.y = 0 as libc::c_int as SWORD;
    //right click on a Template will put the production on hold
    sBFormInit2.style = (4 as libc::c_int | 0x20 as libc::c_int) as UDWORD;
    // Action progress bar.
    sBarInit.formID = 3002 as libc::c_int as UDWORD;
    sBarInit.id = 3200 as libc::c_int as UDWORD;
    sBarInit.style = (1 as libc::c_int | 0x8000 as libc::c_int) as UDWORD;
    sBarInit.orientation = 0x1 as libc::c_int as UWORD;
    sBarInit.x = 3 as libc::c_int as SWORD;
    sBarInit.y = 36 as libc::c_int as SWORD;
    sBarInit.width = (60 as libc::c_int - 8 as libc::c_int) as UWORD;
    sBarInit.height = 4 as libc::c_int as UWORD;
    sBarInit.size = 0 as libc::c_int as UWORD;
    sBarInit.sCol.red = 255 as libc::c_int as UBYTE;
    sBarInit.sCol.green = 235 as libc::c_int as UBYTE;
    sBarInit.sCol.blue = 19 as libc::c_int as UBYTE;
    sBarInit.sMinorCol.red = 0x55 as libc::c_int as UBYTE;
    sBarInit.sMinorCol.green = 0 as libc::c_int as UBYTE;
    sBarInit.sMinorCol.blue = 0 as libc::c_int as UBYTE;
    sBarInit.pTip =
        strresGetString(psStringRes,
                        STR_INT_BLDPROGRESS as libc::c_int as UDWORD);
    //object output bar ie manuf power o/p, research power o/p
    memcpy(&mut sBarInit2 as *mut W_BARINIT as *mut libc::c_void,
           &mut sBarInit as *mut W_BARINIT as *const libc::c_void,
           ::std::mem::size_of::<W_BARINIT>() as libc::c_ulong);
    sBarInit2.id = 3300 as libc::c_int as UDWORD;
    sBarInit2.style = 0 as libc::c_int as UDWORD;
    sBarInit2.x = 3 as libc::c_int as SWORD;
    sBarInit2.y =
        (46 as libc::c_int - 4 as libc::c_int - 6 as libc::c_int) as SWORD;
    sBarInit2.size = 50 as libc::c_int as UWORD;
    //don't set the tip cos we haven't got a suitable text string at this point - 2/2/99
	//sBarInit2.pTip = strresGetString(psStringRes, STR_INT_BLDSPEED);
    sBarInit2.pTip = 0 as *mut STRING;
    memset(&mut sLabInit as *mut W_LABINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_LABINIT>() as libc::c_ulong);
    sLabInit.id = 3400 as libc::c_int as UDWORD;
    sLabInit.style = (0 as libc::c_int | 0x8000 as libc::c_int) as UDWORD;
    sLabInit.x = 2 as libc::c_int as SWORD;
    sLabInit.y = 2 as libc::c_int as SWORD;
    sLabInit.width = 16 as libc::c_int as UWORD;
    sLabInit.height = 16 as libc::c_int as UWORD;
    sLabInit.pText =
        b"10\x00" as *const u8 as *const libc::c_char as *mut STRING;
    sLabInit.FontID = WFont;
    memset(&mut sLabInitCmdFac as *mut W_LABINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_LABINIT>() as libc::c_ulong);
    sLabInitCmdFac.id = 3750 as libc::c_int as UDWORD;
    sLabInitCmdFac.style =
        (0 as libc::c_int | 0x8000 as libc::c_int) as UDWORD;
    sLabInitCmdFac.x = 2 as libc::c_int as SWORD;
    sLabInitCmdFac.y = 14 as libc::c_int as SWORD;
    sLabInitCmdFac.width = 16 as libc::c_int as UWORD;
    sLabInitCmdFac.height = 16 as libc::c_int as UWORD;
    sLabInitCmdFac.pText =
        b"10\x00" as *const u8 as *const libc::c_char as *mut STRING;
    sLabInitCmdFac.FontID = WFont;
    memset(&mut sLabInitCmdFac2 as *mut W_LABINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_LABINIT>() as libc::c_ulong);
    sLabInitCmdFac2.id = 3800 as libc::c_int as UDWORD;
    sLabInitCmdFac2.style =
        (0 as libc::c_int | 0x8000 as libc::c_int) as UDWORD;
    sLabInitCmdFac2.x = 2 as libc::c_int as SWORD;
    sLabInitCmdFac2.y = 26 as libc::c_int as SWORD;
    sLabInitCmdFac2.width = 16 as libc::c_int as UWORD;
    sLabInitCmdFac2.height = 16 as libc::c_int as UWORD;
    sLabInitCmdFac2.pText =
        b"10\x00" as *const u8 as *const libc::c_char as *mut STRING;
    sLabInitCmdFac2.FontID = WFont;
    memset(&mut sLabIntObjText as *mut W_LABINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_LABINIT>() as libc::c_ulong);
    sLabIntObjText.id = 3600 as libc::c_int as UDWORD;
    sLabIntObjText.style =
        (0 as libc::c_int | 0x8000 as libc::c_int) as UDWORD;
    sLabIntObjText.x = 2 as libc::c_int as SWORD;
    sLabIntObjText.y = 8 as libc::c_int as SWORD;
    sLabIntObjText.width = 16 as libc::c_int as UWORD;
    sLabIntObjText.height = 16 as libc::c_int as UWORD;
    sLabIntObjText.pText =
        b"xxx/xxx - overrun\x00" as *const u8 as *const libc::c_char as
            *mut STRING;
    sLabIntObjText.FontID = WFont;
    memset(&mut sLabInitCmdExp as *mut W_LABINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_LABINIT>() as libc::c_ulong);
    sLabInitCmdExp.id = 3700 as libc::c_int as UDWORD;
    sLabInitCmdExp.style =
        (0 as libc::c_int | 0x8000 as libc::c_int) as UDWORD;
    sLabInitCmdExp.x = 3 as libc::c_int as SWORD;
    sLabInitCmdExp.y =
        (46 as libc::c_int - 4 as libc::c_int - 6 as libc::c_int) as SWORD;
    sLabInitCmdExp.width = 16 as libc::c_int as UWORD;
    sLabInitCmdExp.height = 16 as libc::c_int as UWORD;
    sLabInitCmdExp.pText =
        b"@@@@@ - overrun\x00" as *const u8 as *const libc::c_char as
            *mut STRING;
    sLabInitCmdExp.FontID = WFont;
    displayForm = 0 as libc::c_int as UDWORD;
    i = 0 as libc::c_int as UDWORD;
    while i < numObjects as UDWORD {
        psObj = *apsObjectList.offset(i as isize);
        if (*psObj).died == 0 as libc::c_int as libc::c_uint {
            // Don't add the button if the objects dead.
            IsFactory = 0 as libc::c_int;
            /* Got an object - set the text and tip for the button */
            match (*psObj).type_0 as libc::c_uint {
                0 => {
                    // Get the construction power of a construction droid.. Not convinced this is right.
                    Droid = psObj as *mut DROID;
                    if (*Droid).droidType as libc::c_uint ==
                           DROID_CONSTRUCT as libc::c_int as libc::c_uint ||
                           (*Droid).droidType as libc::c_uint ==
                               DROID_CYBORG_CONSTRUCT as libc::c_int as
                                   libc::c_uint {
                        if (*Droid).asBits[COMP_CONSTRUCT as libc::c_int as
                                               usize].nStat as libc::c_int !=
                               0 {
                        } else {
                            debug(LOG_ERROR,
                                  b"intUpdateProgressBar: invalid droid type\x00"
                                      as *const u8 as *const libc::c_char);
                        };
                        if (*Droid).asBits[COMP_CONSTRUCT as libc::c_int as
                                               usize].nStat as libc::c_int !=
                               0 {
                        } else {
                            debug(LOG_ERROR,
                                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                                      *const u8 as *const libc::c_char,
                                  b"hci.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  4844 as libc::c_int,
                                  (*::std::mem::transmute::<&[u8; 20],
                                                            &[libc::c_char; 20]>(b"_intAddObjectWindow\x00")).as_ptr(),
                                  b"Droid->asBits[COMP_CONSTRUCT].nStat\x00"
                                      as *const u8 as *const libc::c_char);
                        };
                        psStats =
                            asConstructStats.offset((*Droid).asBits[COMP_CONSTRUCT
                                                                        as
                                                                        libc::c_int
                                                                        as
                                                                        usize].nStat
                                                        as libc::c_int as
                                                        isize) as
                                *mut BASE_STATS;
                        //sBarInit2.size = (UWORD)((CONSTRUCT_STATS*)psStats)->constructPoints;	// Need to scale? YEP!
                        sBarInit2.size =
                            constructorPoints(psStats as *mut CONSTRUCT_STATS,
                                              (*Droid).player) as UWORD;
                        if sBarInit2.size as libc::c_int > 100 as libc::c_int
                           {
                            sBarInit2.size = 100 as libc::c_int as UWORD
                        }
                    }
                    //				sBFormInit.pTip = ((DROID *)psObj)->pName;
                    sBFormInit.pTip = droidGetName(psObj as *mut DROID)
                }
                1 => {
                    // Get the construction power of a structure..
                    Structure = psObj as *mut STRUCTURE; // Need to scale?
                    match (*(*Structure).pStructureType).type_0 {
                        1 | 16 | 17 => {
                            sBarInit2.size =
                                (*((*Structure).pFunctionality as
                                       *mut FACTORY)).productionOutput as
                                    UWORD;
                            if sBarInit2.size as libc::c_int >
                                   100 as libc::c_int {
                                sBarInit2.size = 100 as libc::c_int as UWORD
                            }
                            IsFactory = 1 as libc::c_int;
                            //right click on factory centres on DP
                            sBFormInit.style =
                                (4 as libc::c_int | 0x20 as libc::c_int) as
                                    UDWORD
                        }
                        10 => {
                            sBarInit2.size =
                                (*((*Structure).pFunctionality as
                                       *mut RESEARCH_FACILITY)).researchPoints
                                    as UWORD; // Need to scale?
                            if sBarInit2.size as libc::c_int >
                                   100 as libc::c_int {
                                sBarInit2.size = 100 as libc::c_int as UWORD
                            }
                        }
                        _ => {
                            if 0 as libc::c_int != 0 {
                            } else {
                                debug(LOG_ERROR,
                                      b"intAddObject: invalid structure type\x00"
                                          as *const u8 as
                                          *const libc::c_char);
                            };
                            if 0 as libc::c_int != 0 {
                            } else {
                                debug(LOG_ERROR,
                                      b"Assert in Warzone: %s:%d : %s (%s)\x00"
                                          as *const u8 as *const libc::c_char,
                                      b"hci.c\x00" as *const u8 as
                                          *const libc::c_char,
                                      4888 as libc::c_int,
                                      (*::std::mem::transmute::<&[u8; 20],
                                                                &[libc::c_char; 20]>(b"_intAddObjectWindow\x00")).as_ptr(),
                                      b"FALSE\x00" as *const u8 as
                                          *const libc::c_char);
                            };
                        }
                    }
                    sBFormInit.pTip =
                        getName((*(*(psObj as
                                         *mut STRUCTURE)).pStructureType).pName)
                }
                2 => {
                    sBFormInit.pTip =
                        getName((*(*(psObj as *mut FEATURE)).psStats).pName)
                }
                _ => { sBFormInit.pTip = 0 as *mut STRING }
            }
            //BufferID = (sBFormInit.id-IDOBJ_OBJSTART)*2;
            BufferID =
                sBFormInit.id.wrapping_sub(3002 as libc::c_int as
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
                      b"hci.c\x00" as *const u8 as *const libc::c_char,
                      4912 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 20],
                                                &[libc::c_char; 20]>(b"_intAddObjectWindow\x00")).as_ptr(),
                      b"BufferID < NUM_TOPICBUFFERS\x00" as *const u8 as
                          *const libc::c_char);
            };
            ClearTopicButtonBuffer(BufferID);
            TopicBuffers[BufferID as usize].InUse = 1 as libc::c_int;
            TopicBuffers[BufferID as usize].Data = psObj as *mut libc::c_void;
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
            if IsFactory != 0 {
                // Add a text label for the factory Inc.
                sLabIntObjText.formID = sBFormInit.id;
                sLabIntObjText.pCallback =
                    Some(intAddFactoryInc as
                             unsafe extern "C" fn(_: *mut _widget,
                                                  _: *mut _w_context) -> ());
                sLabIntObjText.pUserData = psObj as *mut libc::c_void;
                if widgAddLabel(psWScreen, &mut sLabIntObjText) == 0 {
                    return 0 as libc::c_int
                }
                sLabIntObjText.id = sLabIntObjText.id.wrapping_add(1)
            }
            // Add the power bar.
            if (*psObj).type_0 as libc::c_uint !=
                   OBJ_DROID as libc::c_int as libc::c_uint ||
                   ((*(psObj as *mut DROID)).droidType as libc::c_uint ==
                        DROID_CONSTRUCT as libc::c_int as libc::c_uint ||
                        (*(psObj as *mut DROID)).droidType as libc::c_uint ==
                            DROID_CYBORG_CONSTRUCT as libc::c_int as
                                libc::c_uint) {
                sBarInit2.formID = sBFormInit.id;
                sBarInit.iRange = 1000 as libc::c_int as UWORD;
                if widgAddBarGraph(psWScreen, &mut sBarInit2) == 0 {
                    return 0 as libc::c_int
                }
            }
            // Add command droid bits
            if (*psObj).type_0 as libc::c_uint ==
                   OBJ_DROID as libc::c_int as libc::c_uint &&
                   (*(psObj as *mut DROID)).droidType as libc::c_uint ==
                       DROID_COMMAND as libc::c_int as libc::c_uint {
                // the group size label
                sLabIntObjText.formID = sBFormInit.id;
                sLabIntObjText.pCallback =
                    Some(intUpdateCommandSize as
                             unsafe extern "C" fn(_: *mut _widget,
                                                  _: *mut _w_context) -> ());
                sLabIntObjText.pUserData = psObj as *mut libc::c_void;
                if widgAddLabel(psWScreen, &mut sLabIntObjText) == 0 {
                    return 0 as libc::c_int
                }
                sLabIntObjText.id = sLabIntObjText.id.wrapping_add(1);
                // the experience stars
                sLabInitCmdExp.formID = sBFormInit.id;
                sLabInitCmdExp.pCallback =
                    Some(intUpdateCommandExp as
                             unsafe extern "C" fn(_: *mut _widget,
                                                  _: *mut _w_context) -> ());
                //			sLabInitCmdExp.pDisplay = intDisplayCommandExp;
                sLabInitCmdExp.pUserData = psObj as *mut libc::c_void;
                if widgAddLabel(psWScreen, &mut sLabInitCmdExp) == 0 {
                    return 0 as libc::c_int
                }
                sLabInitCmdExp.id = sLabInitCmdExp.id.wrapping_add(1)
            }
            /* Now do the stats button */
            psStats =
                objGetStatsFunc.expect("non-null function pointer")(psObj);
            if !psStats.is_null() {
                //sBFormInit2.pTip = psStats->pName;
				// If it's a droid the name might not be a stringID
                if (*psStats).ref_0 >= 0xc0000 as libc::c_int as libc::c_uint
                       &&
                       (*psStats).ref_0 <
                           (0xc0000 as libc::c_int + 0x10000 as libc::c_int)
                               as libc::c_uint {
                    sBFormInit2.pTip =
                        getTemplateName(psStats as *mut DROID_TEMPLATE)
                    //printf("Tip %s\n",sBFormInit2.pTip);
                } else { sBFormInit2.pTip = getName((*psStats).pName) }
                BufferID =
                    sBFormInit2.id.wrapping_sub(3100 as libc::c_int as
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
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"hci.c\x00" as *const u8 as *const libc::c_char,
                          5001 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 20],
                                                    &[libc::c_char; 20]>(b"_intAddObjectWindow\x00")).as_ptr(),
                          b"BufferID < NUM_OBJECTBUFFERS\x00" as *const u8 as
                              *const libc::c_char);
                };
                ClearObjectButtonBuffer(BufferID);
                ObjectBuffers[BufferID as usize].InUse = 1 as libc::c_int;
                ObjectBuffers[BufferID as usize].Data =
                    psObj as *mut libc::c_void;
                ObjectBuffers[BufferID as usize].Data2 =
                    psStats as *mut libc::c_void;
                sBFormInit2.pUserData =
                    &mut *ObjectBuffers.as_mut_ptr().offset(BufferID as isize)
                        as *mut RENDERED_BUTTON as *mut libc::c_void
            } else if (*psObj).type_0 as libc::c_uint ==
                          OBJ_DROID as libc::c_int as libc::c_uint &&
                          (*(psObj as *mut DROID)).droidType as libc::c_uint
                              == DROID_COMMAND as libc::c_int as libc::c_uint
             {
                sBFormInit2.pTip = 0 as *mut STRING;
                BufferID =
                    sBFormInit2.id.wrapping_sub(3100 as libc::c_int as
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
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"hci.c\x00" as *const u8 as *const libc::c_char,
                          5013 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 20],
                                                    &[libc::c_char; 20]>(b"_intAddObjectWindow\x00")).as_ptr(),
                          b"BufferID < NUM_OBJECTBUFFERS\x00" as *const u8 as
                              *const libc::c_char);
                };
                ClearObjectButtonBuffer(BufferID);
                ObjectBuffers[BufferID as usize].InUse = 1 as libc::c_int;
                ObjectBuffers[BufferID as usize].Data =
                    psObj as *mut libc::c_void;
                ObjectBuffers[BufferID as usize].Data2 =
                    0 as *mut libc::c_void;
                sBFormInit2.pUserData =
                    &mut *ObjectBuffers.as_mut_ptr().offset(BufferID as isize)
                        as *mut RENDERED_BUTTON as *mut libc::c_void
            } else {
                sBFormInit2.pTip = 0 as *mut STRING;
                BufferID =
                    sBFormInit2.id.wrapping_sub(3100 as libc::c_int as
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
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"hci.c\x00" as *const u8 as *const libc::c_char,
                          5025 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 20],
                                                    &[libc::c_char; 20]>(b"_intAddObjectWindow\x00")).as_ptr(),
                          b"BufferID < NUM_OBJECTBUFFERS\x00" as *const u8 as
                              *const libc::c_char);
                };
                ClearObjectButtonBuffer(BufferID);
                ObjectBuffers[BufferID as usize].InUse = 1 as libc::c_int;
                sBFormInit2.pUserData =
                    &mut *ObjectBuffers.as_mut_ptr().offset(BufferID as isize)
                        as *mut RENDERED_BUTTON as *mut libc::c_void
            }
            sBFormInit2.pDisplay =
                Some(intDisplayStatusButton as
                         unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                              _: UDWORD, _: *mut UDWORD)
                             -> ());
            if widgAddForm(psWScreen, &mut sBFormInit2) == 0 {
                return 0 as libc::c_int
            }
            if (*psObj).selected != 0 {
                widgSetButtonState(psWScreen, sBFormInit2.id,
                                   0x4 as libc::c_int as UDWORD);
            }
            if (*psObj).type_0 as libc::c_uint !=
                   OBJ_DROID as libc::c_int as libc::c_uint ||
                   ((*(psObj as *mut DROID)).droidType as libc::c_uint ==
                        DROID_CONSTRUCT as libc::c_int as libc::c_uint ||
                        (*(psObj as *mut DROID)).droidType as libc::c_uint ==
                            DROID_CYBORG_CONSTRUCT as libc::c_int as
                                libc::c_uint) {
                // Set the colour for the production run size text.
                widgSetColour(psWScreen, sBFormInit2.id,
                              WCOL_TEXT as libc::c_int as UDWORD,
                              255 as libc::c_int as UBYTE,
                              255 as libc::c_int as UBYTE,
                              0 as libc::c_int as UBYTE);
                widgSetColour(psWScreen, sBFormInit2.id,
                              WCOL_BKGRND as libc::c_int as UDWORD,
                              0 as libc::c_int as UBYTE,
                              32 as libc::c_int as UBYTE,
                              64 as libc::c_int as UBYTE);
            }
            // Add command droid bits
            if (*psObj).type_0 as libc::c_uint ==
                   OBJ_DROID as libc::c_int as libc::c_uint &&
                   (*(psObj as *mut DROID)).droidType as libc::c_uint ==
                       DROID_COMMAND as libc::c_int as libc::c_uint {
                // the assigned factories label
                sLabInit.formID = sBFormInit2.id;
                sLabInit.pCallback =
                    Some(intUpdateCommandFact as
                             unsafe extern "C" fn(_: *mut _widget,
                                                  _: *mut _w_context) -> ());
                sLabInit.pUserData = psObj as *mut libc::c_void;
                // the assigned cyborg factories label
                sLabInitCmdFac.formID = sBFormInit2.id;
                sLabInitCmdFac.pCallback =
                    Some(intUpdateCommandFact as
                             unsafe extern "C" fn(_: *mut _widget,
                                                  _: *mut _w_context) -> ());
                sLabInitCmdFac.pUserData = psObj as *mut libc::c_void;
                if widgAddLabel(psWScreen, &mut sLabInitCmdFac) == 0 {
                    return 0 as libc::c_int
                }
                // the assigned VTOL factories label
                sLabInitCmdFac2.formID = sBFormInit2.id;
                sLabInitCmdFac2.pCallback =
                    Some(intUpdateCommandFact as
                             unsafe extern "C" fn(_: *mut _widget,
                                                  _: *mut _w_context) -> ());
                sLabInitCmdFac2.pUserData = psObj as *mut libc::c_void;
                if widgAddLabel(psWScreen, &mut sLabInitCmdFac2) == 0 {
                    return 0 as libc::c_int
                }
            } else {
                // Add a text label for the size of the production run.
                sLabInit.formID = sBFormInit2.id;
                sLabInit.pCallback =
                    Some(intUpdateQuantity as
                             unsafe extern "C" fn(_: *mut _widget,
                                                  _: *mut _w_context) -> ());
                sLabInit.pUserData = psObj as *mut libc::c_void
            }
            if widgAddLabel(psWScreen, &mut sLabInit) == 0 {
                return 0 as libc::c_int
            }
            // Add the progress bar.
            sBarInit.formID = sBFormInit2.id;
            // Setup widget update callback and object pointer so we can update the progress bar.
            sBarInit.pCallback =
                Some(intUpdateProgressBar as
                         unsafe extern "C" fn(_: *mut _widget,
                                              _: *mut _w_context) -> ());
            sBarInit.pUserData = psObj as *mut libc::c_void;
            sBarInit.iRange = 1000 as libc::c_int as UWORD;
            if widgAddBarGraph(psWScreen, &mut sBarInit) == 0 {
                return 0 as libc::c_int
            }
            /* If this matches psSelected note which form to display */
            if psSelected == psObj {
                displayForm = sBFormInit.majorID as UDWORD;
                statID_0 = sBFormInit2.id
                //				DBPRINTF(("Selected %d\n",statID);
            }
            /* Set up the next button (Objects) */
            sBFormInit.id =
                (sBFormInit.id as
                     libc::c_uint).wrapping_add(1 as libc::c_int as
                                                    libc::c_uint) as UDWORD as
                    UDWORD;
            if sBFormInit.id < 3021 as libc::c_int as libc::c_uint {
            } else {
                debug(LOG_ERROR,
                      b"Too many object buttons\x00" as *const u8 as
                          *const libc::c_char);
            };
            if sBFormInit.id < 3021 as libc::c_int as libc::c_uint {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"hci.c\x00" as *const u8 as *const libc::c_char,
                      5118 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 20],
                                                &[libc::c_char; 20]>(b"_intAddObjectWindow\x00")).as_ptr(),
                      b"sBFormInit.id < IDOBJ_OBJEND\x00" as *const u8 as
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
            /* Set up the next button (Stats) */
            sLabInit.id =
                (sLabInit.id as
                     libc::c_uint).wrapping_add(1 as libc::c_int as
                                                    libc::c_uint) as UDWORD as
                    UDWORD;
            sLabInitCmdFac.id =
                (sLabInitCmdFac.id as
                     libc::c_uint).wrapping_add(1 as libc::c_int as
                                                    libc::c_uint) as UDWORD as
                    UDWORD;
            sLabInitCmdFac2.id =
                (sLabInitCmdFac2.id as
                     libc::c_uint).wrapping_add(1 as libc::c_int as
                                                    libc::c_uint) as UDWORD as
                    UDWORD;
            sBarInit.id =
                (sBarInit.id as
                     libc::c_uint).wrapping_add(1 as libc::c_int as
                                                    libc::c_uint) as UDWORD as
                    UDWORD;
            if sBarInit.id < 3299 as libc::c_int as libc::c_uint {
            } else {
                debug(LOG_ERROR,
                      b"Too many progress bars\x00" as *const u8 as
                          *const libc::c_char);
            };
            if sBarInit.id < 3299 as libc::c_int as libc::c_uint {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"hci.c\x00" as *const u8 as *const libc::c_char,
                      5133 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 20],
                                                &[libc::c_char; 20]>(b"_intAddObjectWindow\x00")).as_ptr(),
                      b"sBarInit.id < IDOBJ_PROGBAREND\x00" as *const u8 as
                          *const libc::c_char);
            };
            sBarInit2.id =
                (sBarInit2.id as
                     libc::c_uint).wrapping_add(1 as libc::c_int as
                                                    libc::c_uint) as UDWORD as
                    UDWORD;
            if sBarInit2.id < 3399 as libc::c_int as libc::c_uint {
            } else {
                debug(LOG_ERROR,
                      b"Too many power bars\x00" as *const u8 as
                          *const libc::c_char);
            };
            if sBarInit2.id < 3399 as libc::c_int as libc::c_uint {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"hci.c\x00" as *const u8 as *const libc::c_char,
                      5136 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 20],
                                                &[libc::c_char; 20]>(b"_intAddObjectWindow\x00")).as_ptr(),
                      b"sBarInit2.id < IDOBJ_POWERBAREND\x00" as *const u8 as
                          *const libc::c_char);
            };
            sBFormInit2.id =
                (sBFormInit2.id as
                     libc::c_uint).wrapping_add(1 as libc::c_int as
                                                    libc::c_uint) as UDWORD as
                    UDWORD;
            if sBFormInit2.id < 3199 as libc::c_int as libc::c_uint {
            } else {
                debug(LOG_ERROR,
                      b"Too many stat buttons\x00" as *const u8 as
                          *const libc::c_char);
            };
            if sBFormInit2.id < 3199 as libc::c_int as libc::c_uint {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"hci.c\x00" as *const u8 as *const libc::c_char,
                      5139 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 20],
                                                &[libc::c_char; 20]>(b"_intAddObjectWindow\x00")).as_ptr(),
                      b"sBFormInit2.id < IDOBJ_STATEND\x00" as *const u8 as
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
            if sBFormInit.id > 3021 as libc::c_int as libc::c_uint {
                //can't fit any more on the screen!
                debug(LOG_NEVER,
                      b"This is just a Warning!\n Max buttons have been allocated\x00"
                          as *const u8 as *const libc::c_char);
                break ;
            }
        }
        //DBPRINTF(("Skipped dead object\n");
        i = i.wrapping_add(1)
    }
    //	widgStartScreen(psWScreen);
    widgSetTabs(psWScreen, 3500 as libc::c_int as UDWORD,
                displayForm as UWORD, 0 as libc::c_int as UWORD);
    // if the selected object isn't on one of the main buttons (too many objects)
	// reset the selected pointer
    if statID_0 == 0 as libc::c_int as libc::c_uint {
        psSelected = 0 as *mut BASE_OBJECT
    }
    //DBPRINTF(("%p %d\n",psSelected,bForceStats);
    if !psSelected.is_null() &&
           objMode as libc::c_uint !=
               IOBJ_COMMAND as libc::c_int as libc::c_uint {
        if bForceStats != 0 ||
               !widgGetFromID(psWScreen,
                              4000 as libc::c_int as UDWORD).is_null() {
            //DBPRINTF(("intAddObjectStats %p %d\n",psSelected,statID));
            objStatID = statID_0;
            intAddObjectStats(psSelected, statID_0);
            intMode = INT_STAT;
            if bForceStats == 0 {
                intSetCurrentCursorPosition(&mut InterfaceSnap, statID_0);
            }
        } else {
            widgSetButtonState(psWScreen, statID_0,
                               0x4 as libc::c_int as UDWORD);
            intMode = INT_OBJECT;
            intSetCurrentCursorPosition(&mut InterfaceSnap, statID_0);
        }
    } else if !psSelected.is_null() {
        /* Note the object */
        psObjSelected = psSelected;
        objStatID = statID_0;
        // We don't want to be locking the button for command droids.
//		widgSetButtonState(psWScreen, statID, WBUT_CLICKLOCK);
// Don't want it to automaticly open order screen on PSX.
        //changed to a BASE_OBJECT to accomodate the factories - AB 21/04/99
		//intAddOrder((DROID *)psSelected);
        intAddOrder(psSelected);
        widgSetButtonState(psWScreen, statID_0, 0x4 as libc::c_int as UDWORD);
        intMode = INT_CMDORDER;
        intSetCurrentCursorPosition(&mut InterfaceSnap, statID_0);
    } else {
        intMode = INT_OBJECT;
        intSetCurrentCursorPosition(&mut InterfaceSnap, statID_0);
    }
    if objMode as libc::c_uint == IOBJ_BUILD as libc::c_int as libc::c_uint ||
           objMode as libc::c_uint ==
               IOBJ_MANUFACTURE as libc::c_int as libc::c_uint ||
           objMode as libc::c_uint ==
               IOBJ_RESEARCH as libc::c_int as libc::c_uint {
        intShowPowerBar();
    }
    //	if ((objMode==IOBJ_RESEARCH) && bInTutorial)
    if bInTutorial != 0 {
        debug(LOG_NEVER,
              b"Go with object open callback!\n\x00" as *const u8 as
                  *const libc::c_char);
        eventFireCallbackTrigger(CALL_OBJECTOPEN as libc::c_int as
                                     TRIGGER_TYPE);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn _intUpdateObject(mut psObjects: *mut BASE_OBJECT,
                                      mut psSelected: *mut BASE_OBJECT,
                                      mut bForceStats: BOOL) -> BOOL {
    _intAddObjectWindow(psObjects, psSelected, bForceStats);
    // if the stats screen is up and..
    if StatsUp != 0 {
        if !psStatsScreenOwner.is_null() {
            // it's owner is dead then..
            if (*psStatsScreenOwner).died != 0 as libc::c_int as libc::c_uint
               {
                // remove it.
//DBPRINTF(("psStatsScreenOwner died\n");
                intRemoveStatsNoAnim();
            }
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn _intAddObject(mut psObjects: *mut BASE_OBJECT,
                                   mut psSelected: *mut BASE_OBJECT,
                                   mut bForceStats: BOOL) -> BOOL {
    _intAddObjectWindow(psObjects, psSelected, bForceStats);
    return 1 as libc::c_int;
}
/* Remove the object widgets from the widget screen */
/* Remove the build widgets from the widget screen */
#[no_mangle]
pub unsafe extern "C" fn intRemoveObject() {
    let mut Form: *mut W_TABFORM = 0 as *mut W_TABFORM;
    widgDelete(psWScreen, 3500 as libc::c_int as UDWORD);
    widgDelete(psWScreen, 3001 as libc::c_int as UDWORD);
    // Start the window close animation.
    Form =
        widgGetFromID(psWScreen, 3000 as libc::c_int as UDWORD) as
            *mut W_TABFORM; // Used to signal when the close anim has finished.
    if !Form.is_null() {
        (*Form).display =
            Some(intClosePlainForm as
                     unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                          _: UDWORD, _: *mut UDWORD) -> ());
        (*Form).disableChildren = 1 as libc::c_int;
        (*Form).pUserData = 0 as *mut libc::c_void;
        ClosingObject = 1 as libc::c_int
    }
    ClearObjectBuffers();
    ClearTopicBuffers();
    intHidePowerBar();
    if bInTutorial != 0 {
        debug(LOG_NEVER,
              b"Go with object close callback!\n\x00" as *const u8 as
                  *const libc::c_char);
        eventFireCallbackTrigger(CALL_OBJECTCLOSE as libc::c_int as
                                     TRIGGER_TYPE);
    };
}
/* Remove the build widgets from the widget screen */
unsafe extern "C" fn intRemoveObjectNoAnim() {
    widgDelete(psWScreen, 3500 as libc::c_int as UDWORD);
    widgDelete(psWScreen, 3001 as libc::c_int as UDWORD);
    widgDelete(psWScreen, 3000 as libc::c_int as UDWORD);
    ClearObjectBuffers();
    ClearTopicBuffers();
    intHidePowerBar();
    /*	if (bInTutorial)
	{
		DBPRINTF(("Go with object close callback!(noanim)\n"));
	 	eventFireCallbackTrigger(CALL_OBJECTCLOSE);
	}*/
}
/* Remove the stats widgets from the widget screen */
/* Remove the stats widgets from the widget screen */
/* Remove the stats widgets from the widget screen */
#[no_mangle]
pub unsafe extern "C" fn intRemoveStats() {
    let mut Form: *mut W_TABFORM = 0 as *mut W_TABFORM;
    widgDelete(psWScreen, 4003 as libc::c_int as UDWORD);
    widgDelete(psWScreen, 4004 as libc::c_int as UDWORD);
    // Start the window close animation.
    Form =
        widgGetFromID(psWScreen, 4000 as libc::c_int as UDWORD) as
            *mut W_TABFORM; // Used to signal when the close anim has finished.
    if !Form.is_null() {
        (*Form).display =
            Some(intClosePlainForm as
                     unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                          _: UDWORD, _: *mut UDWORD) -> ());
        (*Form).pUserData = 0 as *mut libc::c_void;
        (*Form).disableChildren = 1 as libc::c_int;
        ClosingStats = 1 as libc::c_int
    }
    ClearStatBuffers();
    StatsUp = 0 as libc::c_int;
    psStatsScreenOwner = 0 as *mut BASE_OBJECT;
    //DBPRINTF(("intRemoveStats\n");
}
/* Remove the stats widgets from the widget screen */
/* Remove the stats widgets from the widget screen */
#[no_mangle]
pub unsafe extern "C" fn intRemoveStatsNoAnim() {
    widgDelete(psWScreen, 4003 as libc::c_int as UDWORD);
    widgDelete(psWScreen, 4004 as libc::c_int as UDWORD);
    widgDelete(psWScreen, 4000 as libc::c_int as UDWORD);
    ClearStatBuffers();
    StatsUp = 0 as libc::c_int;
    psStatsScreenOwner = 0 as *mut BASE_OBJECT;
    //DBPRINTF(("intRemoveStatsNoAnim\n");
}
// Poll for closing windows and handle them, ensure called even if game is paused.
//
#[no_mangle]
pub unsafe extern "C" fn HandleClosingWindows() {
    let mut Widg: *mut WIDGET = 0 as *mut WIDGET;
    if ClosingObject != 0 {
        Widg = widgGetFromID(psWScreen, 3000 as libc::c_int as UDWORD);
        if !Widg.is_null() {
            // Has the window finished closing?
            if (*Widg).pUserData as UDWORD != 0 {
                widgDelete(psWScreen, 3000 as libc::c_int as UDWORD);
                ClosingObject = 0 as libc::c_int
            }
        } else { ClosingObject = 0 as libc::c_int }
    }
    if ClosingStats != 0 {
        Widg = widgGetFromID(psWScreen, 4000 as libc::c_int as UDWORD);
        if !Widg.is_null() {
            // Has the window finished closing?
            if (*Widg).pUserData as UDWORD != 0 {
                widgDelete(psWScreen, 4000 as libc::c_int as UDWORD);
                ClosingStats = 0 as libc::c_int
            }
        } else { ClosingStats = 0 as libc::c_int }
    }
    if ClosingMessageView != 0 {
        Widg = widgGetFromID(psWScreen, 6002 as libc::c_int as UDWORD);
        if !Widg.is_null() {
            // Has the window finished closing?
            if (*Widg).pUserData as UDWORD != 0 {
                widgDelete(psWScreen, 6002 as libc::c_int as UDWORD);
                ClosingMessageView = 0 as libc::c_int
            }
        } else { ClosingMessageView = 0 as libc::c_int }
    }
    if ClosingIntelMap != 0 {
        Widg = widgGetFromID(psWScreen, 6000 as libc::c_int as UDWORD);
        if !Widg.is_null() {
            // Has the window finished closing?
            if (*Widg).pUserData as UDWORD != 0 {
                widgDelete(psWScreen, 6000 as libc::c_int as UDWORD);
                ClosingIntelMap = 0 as libc::c_int
            }
        } else { ClosingIntelMap = 0 as libc::c_int }
    }
    if ClosingOrder != 0 {
        Widg = widgGetFromID(psWScreen, 8000 as libc::c_int as UDWORD);
        if !Widg.is_null() {
            // Has the window finished closing?
            if (*Widg).pUserData as UDWORD != 0 {
                widgDelete(psWScreen, 8000 as libc::c_int as UDWORD);
                ClosingOrder = 0 as libc::c_int
            }
        } else { ClosingOrder = 0 as libc::c_int }
    }
    if ClosingTrans != 0 {
        Widg = widgGetFromID(psWScreen, 9000 as libc::c_int as UDWORD);
        if !Widg.is_null() {
            // Has the window finished closing?
            if (*Widg).pUserData as UDWORD != 0 {
                widgDelete(psWScreen, 9000 as libc::c_int as UDWORD);
                ClosingTrans = 0 as libc::c_int
            }
        } else { ClosingTrans = 0 as libc::c_int }
    }
    if ClosingTransCont != 0 {
        Widg = widgGetFromID(psWScreen, 9003 as libc::c_int as UDWORD);
        if !Widg.is_null() {
            // Has the window finished closing?
            if (*Widg).pUserData as UDWORD != 0 {
                widgDelete(psWScreen, 9003 as libc::c_int as UDWORD);
                ClosingTransCont = 0 as libc::c_int
            }
        } else { ClosingTransCont = 0 as libc::c_int }
    }
    if ClosingTransDroids != 0 {
        Widg = widgGetFromID(psWScreen, 9006 as libc::c_int as UDWORD);
        if !Widg.is_null() {
            // Has the window finished closing?
            if (*Widg).pUserData as UDWORD != 0 {
                widgDelete(psWScreen, 9006 as libc::c_int as UDWORD);
                ClosingTransDroids = 0 as libc::c_int
            }
        } else { ClosingTransDroids = 0 as libc::c_int }
    }
    if ClosingInGameOp != 0 {
        Widg = widgGetFromID(psWScreen, 10500 as libc::c_int as UDWORD);
        if !Widg.is_null() {
            // Has the window finished closing?
            if (*Widg).pUserData as UDWORD != 0 {
                widgDelete(psWScreen, 10500 as libc::c_int as UDWORD);
                ClosingInGameOp = 0 as libc::c_int
            }
        } else { ClosingInGameOp = 0 as libc::c_int }
    }
    //if(ClosingMissionRes) {
	//	Widg = widgGetFromID(psWScreen,IDMISSIONRES_FORM);
	//	if(Widg) {
// Has the window finished closing?
	//		if( ((UDWORD)Widg->pUserData) ) {
	//			intRemoveMissionResultNoAnim();
	//			resetMissionPauseState();	//reset the pauses
	//		}
	//	} else {
	//		ClosingMissionRes = FALSE;
	//		//reset the pauses
	//		resetMissionPauseState();
	//	}
	//}
    if ClosingMultiMenu != 0 {
        Widg = widgGetFromID(psWScreen, 10600 as libc::c_int as UDWORD);
        if !Widg.is_null() {
            // Has the window finished closing?
            if (*Widg).pUserData as UDWORD != 0 {
                widgDelete(psWScreen, 10600 as libc::c_int as UDWORD);
                ClosingMultiMenu = 0 as libc::c_int
            }
        } else { ClosingMultiMenu = 0 as libc::c_int }
    };
    //19 #ifdef PSX
//19 	HandleKeyboardClose();
//19 #endif
}
/* Get the object refered to by a button ID on the object screen.
 * This works for droid or structure buttons
 */
/* Get the object refered to by a button ID on the object screen.
 * This works for object or stats buttons
 */
unsafe extern "C" fn intGetObject(mut id: UDWORD) -> *mut BASE_OBJECT {
    //	UDWORD			objID;
    let mut psObj: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    /* If this is a stats button, find the object button linked to it */
    if id >= 3100 as libc::c_int as libc::c_uint &&
           id <= 3199 as libc::c_int as libc::c_uint {
        id =
            (3002 as libc::c_int as
                 libc::c_uint).wrapping_add(id).wrapping_sub(3100 as
                                                                 libc::c_int
                                                                 as
                                                                 libc::c_uint)
    }
    /* Find the object that the ID refers to */
    if id as SDWORD - 3002 as libc::c_int >= 0 as libc::c_int &&
           (id as SDWORD - 3002 as libc::c_int) < numObjects {
    } else {
        debug(LOG_ERROR,
              b"intGetObject: invalid button ID\x00" as *const u8 as
                  *const libc::c_char);
    };
    if id as SDWORD - 3002 as libc::c_int >= 0 as libc::c_int &&
           (id as SDWORD - 3002 as libc::c_int) < numObjects {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"hci.c\x00" as *const u8 as *const libc::c_char,
              5538 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 13],
                                        &[libc::c_char; 13]>(b"intGetObject\x00")).as_ptr(),
              b"( (SDWORD)id - IDOBJ_OBJSTART >= 0 ) && ( (SDWORD)id - IDOBJ_OBJSTART < numObjects )\x00"
                  as *const u8 as *const libc::c_char);
    };
    psObj =
        *apsObjectList.offset(id.wrapping_sub(3002 as libc::c_int as
                                                  libc::c_uint) as isize);
    /*	objID = IDOBJ_OBJSTART;
	for(psObj = psObjList; psObj; psObj = psObj->psNext)
	{
		if (objSelectFunc(psObj))
		{
			if (objID == id)
			{
				// Found the object so jump out of the loops
				goto found;
			}
			objID++;
		}
	}
found:	// Jump to here if an object is found
	ASSERT( psObj != NULL, "intGetObject: couldn't match id to button" );
	*/
    return psObj;
}
/* Reset the stats button for an object */
unsafe extern "C" fn _intSetStats(mut id: UDWORD,
                                  mut psStats: *mut BASE_STATS) {
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
    let mut butPerForm: UDWORD = 0;
    let mut butPos: UDWORD = 0;
    let mut BufferID: SDWORD = 0;
    let mut psObj: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    /* Update the button on the object screen */
    widgDelete(psWScreen, id);
    memset(&mut sFormInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_FORMINIT>() as libc::c_ulong);
    memset(&mut sBarInit as *mut W_BARINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_BARINIT>() as libc::c_ulong);
    sFormInit.formID = 3500 as libc::c_int as UDWORD;
    butPerForm =
        ((316 as libc::c_int - 2 as libc::c_int) /
             (60 as libc::c_int + 2 as libc::c_int)) as UDWORD;
    sFormInit.majorID =
        id.wrapping_sub(3100 as libc::c_int as
                            libc::c_uint).wrapping_div(butPerForm) as UWORD;
    sFormInit.minorID = 0 as libc::c_int as UWORD;
    sFormInit.id = id;
    sFormInit.style = (4 as libc::c_int | 0x20 as libc::c_int) as UDWORD;
    butPos =
        id.wrapping_sub(3100 as libc::c_int as
                            libc::c_uint).wrapping_rem(butPerForm);
    sFormInit.x =
        butPos.wrapping_mul((60 as libc::c_int + 2 as libc::c_int) as
                                libc::c_uint).wrapping_add(2 as libc::c_int as
                                                               libc::c_uint)
            as UWORD as SWORD;
    sFormInit.y = 0 as libc::c_int as SWORD;
    sFormInit.width = 60 as libc::c_int as UWORD;
    sFormInit.height = 46 as libc::c_int as UWORD;
    // Action progress bar.
    sBarInit.formID = id;
    sBarInit.id =
        id.wrapping_sub(3100 as libc::c_int as
                            libc::c_uint).wrapping_add(3200 as libc::c_int as
                                                           libc::c_uint);
    sBarInit.style = 1 as libc::c_int as UDWORD;
    sBarInit.orientation = 0x1 as libc::c_int as UWORD;
    sBarInit.x = 3 as libc::c_int as SWORD;
    sBarInit.y = 36 as libc::c_int as SWORD;
    sBarInit.width = (60 as libc::c_int - 8 as libc::c_int) as UWORD;
    sBarInit.height = 4 as libc::c_int as UWORD;
    sBarInit.size = 0 as libc::c_int as UWORD;
    sBarInit.sCol.red = 255 as libc::c_int as UBYTE;
    sBarInit.sCol.green = 235 as libc::c_int as UBYTE;
    sBarInit.sCol.blue = 19 as libc::c_int as UBYTE;
    sBarInit.sMinorCol.red = 0x55 as libc::c_int as UBYTE;
    sBarInit.sMinorCol.green = 0 as libc::c_int as UBYTE;
    sBarInit.sMinorCol.blue = 0 as libc::c_int as UBYTE;
    sBarInit.iRange = 1000 as libc::c_int as UWORD;
    // Setup widget update callback and object pointer so we can update the progress bar.
    sBarInit.pCallback =
        Some(intUpdateProgressBar as
                 unsafe extern "C" fn(_: *mut _widget, _: *mut _w_context)
                     -> ());
    sBarInit.pUserData = intGetObject(id) as *mut libc::c_void;
    memset(&mut sLabInit as *mut W_LABINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_LABINIT>() as libc::c_ulong);
    sLabInit.formID = id;
    sLabInit.id =
        id.wrapping_sub(3100 as libc::c_int as
                            libc::c_uint).wrapping_add(3400 as libc::c_int as
                                                           libc::c_uint);
    sLabInit.style = (0 as libc::c_int | 0x8000 as libc::c_int) as UDWORD;
    sLabInit.x = 2 as libc::c_int as SWORD;
    sLabInit.y = 2 as libc::c_int as SWORD;
    sLabInit.width = 16 as libc::c_int as UWORD;
    sLabInit.height = 16 as libc::c_int as UWORD;
    sLabInit.pText =
        b"10\x00" as *const u8 as *const libc::c_char as *mut STRING;
    sLabInit.FontID = WFont;
    if !psStats.is_null() {
        //		sButInit.pText = "S";
		//sFormInit.pTip = psStats->pName;
		// If it's a droid the name might not be a stringID
        if (*psStats).ref_0 >= 0xc0000 as libc::c_int as libc::c_uint &&
               (*psStats).ref_0 <
                   (0xc0000 as libc::c_int + 0x10000 as libc::c_int) as
                       libc::c_uint {
            sFormInit.pTip = getTemplateName(psStats as *mut DROID_TEMPLATE)
            //printf("Tip2 %s\n",sFormInit.pTip);
        } else { sFormInit.pTip = getName((*psStats).pName) }
        BufferID =
            sFormInit.id.wrapping_sub(3100 as libc::c_int as
                                          libc::c_uint).wrapping_mul(2 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_uint).wrapping_add(1
                                                                                                        as
                                                                                                        libc::c_int
                                                                                                        as
                                                                                                        libc::c_uint)
                as SDWORD;
        //		sFormInit.pUserData = (void*)intGetObject(id);
        if BufferID < 10 as libc::c_int * 4 as libc::c_int {
        } else {
            debug(LOG_ERROR,
                  b"BufferID > NUM_OBJECTBUFFERS\x00" as *const u8 as
                      *const libc::c_char);
        };
        if BufferID < 10 as libc::c_int * 4 as libc::c_int {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"hci.c\x00" as *const u8 as *const libc::c_char,
                  5645 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 13],
                                            &[libc::c_char; 13]>(b"_intSetStats\x00")).as_ptr(),
                  b"BufferID < NUM_OBJECTBUFFERS\x00" as *const u8 as
                      *const libc::c_char);
        };
        ClearObjectButtonBuffer(BufferID);
        ObjectBuffers[BufferID as usize].InUse = 1 as libc::c_int;
        ObjectBuffers[BufferID as usize].Data =
            intGetObject(id) as *mut libc::c_void;
        ObjectBuffers[BufferID as usize].Data2 = psStats as *mut libc::c_void;
        sFormInit.pUserData =
            &mut *ObjectBuffers.as_mut_ptr().offset(BufferID as isize) as
                *mut RENDERED_BUTTON as *mut libc::c_void;
        sLabInit.pCallback =
            Some(intUpdateQuantity as
                     unsafe extern "C" fn(_: *mut _widget, _: *mut _w_context)
                         -> ());
        sLabInit.pUserData = sBarInit.pUserData
    } else {
        //		DBPRINTF(("2 *sFormInit.id-IDOBJ_STATSTART : %d\n",BufferID));
//		BufferID = GetObjectBuffer();
        // Add a text label for the size of the production run.
        //		sButInit.pText = "NONE";
        sFormInit.pTip = 0 as *mut STRING;
        BufferID =
            sFormInit.id.wrapping_sub(3100 as libc::c_int as
                                          libc::c_uint).wrapping_mul(2 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_uint).wrapping_add(1
                                                                                                        as
                                                                                                        libc::c_int
                                                                                                        as
                                                                                                        libc::c_uint)
                as SDWORD;
        //		DBPRINTF(("2 sFormInit.id-IDOBJ_STATSTART : %d\n",BufferID));
//		BufferID = GetObjectBuffer();
        if BufferID < 10 as libc::c_int * 4 as libc::c_int {
        } else {
            debug(LOG_ERROR,
                  b"BufferID > NUM_OBJECTBUFFERS\x00" as *const u8 as
                      *const libc::c_char);
        };
        if BufferID < 10 as libc::c_int * 4 as libc::c_int {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"hci.c\x00" as *const u8 as *const libc::c_char,
                  5665 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 13],
                                            &[libc::c_char; 13]>(b"_intSetStats\x00")).as_ptr(),
                  b"BufferID < NUM_OBJECTBUFFERS\x00" as *const u8 as
                      *const libc::c_char);
        };
        ClearObjectButtonBuffer(BufferID);
        ObjectBuffers[BufferID as usize].InUse = 1 as libc::c_int;
        sFormInit.pUserData =
            &mut *ObjectBuffers.as_mut_ptr().offset(BufferID as isize) as
                *mut RENDERED_BUTTON as *mut libc::c_void;
        //		sFormInit.pUserData = NULL;
        /* Reset the stats screen button if necessary */
        if objMode as libc::c_uint == INT_STAT as libc::c_int as libc::c_uint
               && statID != 0 as libc::c_int as libc::c_uint {
            widgSetButtonState(psWScreen, statID, 0 as libc::c_int as UDWORD);
        }
    }
    sFormInit.pDisplay =
        Some(intDisplayStatusButton as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    widgAddForm(psWScreen, &mut sFormInit);
    // Set the colour for the production run size text.
    widgSetColour(psWScreen, sFormInit.id, WCOL_TEXT as libc::c_int as UDWORD,
                  255 as libc::c_int as UBYTE, 255 as libc::c_int as UBYTE,
                  0 as libc::c_int as UBYTE);
    widgSetColour(psWScreen, sFormInit.id,
                  WCOL_BKGRND as libc::c_int as UDWORD,
                  0 as libc::c_int as UBYTE, 32 as libc::c_int as UBYTE,
                  64 as libc::c_int as UBYTE);
    widgAddLabel(psWScreen, &mut sLabInit);
    widgAddBarGraph(psWScreen, &mut sBarInit);
    psObj = intGetObject(id);
    if !psObj.is_null() && (*psObj).selected as libc::c_int != 0 {
        widgSetButtonState(psWScreen, id, 0x4 as libc::c_int as UDWORD);
    };
}
#[no_mangle]
pub unsafe extern "C" fn intUpdateManufactureLimits(mut psWidget:
                                                        *mut _widget,
                                                    mut psContext:
                                                        *mut _w_context) {
    let mut Label: *mut W_LABEL = psWidget as *mut W_LABEL;
    let mut MaxDroids: UDWORD = getMaxDroids(selectedPlayer);
    let mut CurDroids: UDWORD =
        getNumDroids(selectedPlayer).wrapping_add(getNumMissionDroids(selectedPlayer)).wrapping_add(getNumTransporterDroids(selectedPlayer));
    if CurDroids > MaxDroids { CurDroids = MaxDroids }
    (*Label).aText[0 as libc::c_int as usize] =
        ('0' as i32 as
             libc::c_uint).wrapping_add(CurDroids.wrapping_div(10 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uint))
            as UBYTE as STRING;
    (*Label).aText[1 as libc::c_int as usize] =
        ('0' as i32 as
             libc::c_uint).wrapping_add(CurDroids.wrapping_rem(10 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uint))
            as UBYTE as STRING;
    (*Label).aText[3 as libc::c_int as usize] =
        ('0' as i32 as
             libc::c_uint).wrapping_add(MaxDroids.wrapping_div(10 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uint))
            as UBYTE as STRING;
    (*Label).aText[4 as libc::c_int as usize] =
        ('0' as i32 as
             libc::c_uint).wrapping_add(MaxDroids.wrapping_rem(10 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uint))
            as UBYTE as STRING;
}
/* Add the stats widgets to the widget screen */
/* If psSelected != NULL it specifies which stat should be hilited
   psOwner specifies which object is hilighted on the object bar for this stat*/
unsafe extern "C" fn _intAddStats(mut ppsStatsList_0: *mut *mut BASE_STATS,
                                  mut numStats: UDWORD,
                                  mut psSelected: *mut BASE_STATS,
                                  mut psOwner: *mut BASE_OBJECT) -> BOOL {
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
    let mut i: UDWORD = 0;
    let mut butPerForm: UDWORD = 0;
    let mut statForm: UDWORD = 0;
    let mut BufferID: SDWORD = 0;
    let mut Stat: *mut BASE_STATS = 0 as *mut BASE_STATS;
    let mut Animate: BOOL = 1 as libc::c_int;
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
    let mut psFactory: *mut FACTORY = 0 as *mut FACTORY;
    //STRING				sCaption[6];
    // should this ever be called with psOwner == NULL?
    // Is the form already up?
    if !widgGetFromID(psWScreen, 4000 as libc::c_int as UDWORD).is_null() {
        intRemoveStatsNoAnim();
        Animate = 0 as libc::c_int
    }
    // is the order form already up ?
    if !widgGetFromID(psWScreen, 8000 as libc::c_int as UDWORD).is_null() {
        intRemoveOrderNoAnim();
    }
    Animate = 0 as libc::c_int;
    //	// return if there's no owner? Option screen calls with psOwner == NULL.
//	if(psOwner == NULL) {
//		ASSERT( FALSE,"intAddStats : psOwner == NULL" );	// Actually an error condition.
//		return FALSE;
//	}
    if !psOwner.is_null() {
        // Return if the owner is dead.
        if (*psOwner).died != 0 as libc::c_int as libc::c_uint {
            //DBPRINTF(("intAddStats : Owner is dead\n");
            return 0 as libc::c_int
        }
    }
    psStatsScreenOwner = psOwner;
    ClearStatBuffers();
    widgEndScreen(psWScreen);
    /* Create the basic form */
    memset(&mut sFormInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_FORMINIT>() as libc::c_ulong);
    sFormInit.formID = 0 as libc::c_int as UDWORD;
    sFormInit.id = 4000 as libc::c_int as UDWORD;
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
    if widgAddForm(psWScreen, &mut sFormInit) == 0 {
        //DBPRINTF(("widgAdd failed : %d\n",__LINE__);
        return 0 as libc::c_int
    }
    //#if defined(PSX) && defined(MOVETOFORM)
// // Position the mouse in the center of this form.
//	SetCurrentSnapFormID(&InterfaceSnap,sFormInit.id);
//#endif
    // Add the quantity slider ( if it's a factory ).
    if objMode as libc::c_uint ==
           IOBJ_MANUFACTURE as libc::c_int as libc::c_uint {
        // No delivery point button on PSX.
		//add the Factory DP button
        memset(&mut sButInit as *mut W_BUTINIT as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<W_BUTINIT>() as libc::c_ulong);
        sButInit.formID = 4000 as libc::c_int as UDWORD;
        sButInit.id = 4405 as libc::c_int as UDWORD;
        sButInit.style = (0 as libc::c_int | 0x20 as libc::c_int) as UDWORD;
        sButInit.x = 4 as libc::c_int as SWORD;
        sButInit.y = 4 as libc::c_int as SWORD;
        sButInit.width =
            iV_GetImageWidth(IntImages,
                             IMAGE_FDP_DOWN as libc::c_int as UWORD);
        sButInit.height =
            iV_GetImageHeight(IntImages,
                              IMAGE_FDP_DOWN as libc::c_int as UWORD);
        sButInit.pTip =
            strresGetString(psStringRes,
                            STR_INT_DPOINT as libc::c_int as UDWORD);
        sButInit.FontID = WFont;
        sButInit.pDisplay =
            Some(intDisplayDPButton as
                     unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                          _: UDWORD, _: *mut UDWORD) -> ());
        sButInit.pUserData = psOwner as *mut libc::c_void;
        if widgAddButton(psWScreen, &mut sButInit) == 0 {
            return 0 as libc::c_int
        }
        //#ifdef PSX
	//	WidgSetOTIndex(OT2D_FARFARFORE);
	//#endif
        // No looped production on PSX thank you.
		//add the Factory Loop button!
        memset(&mut sButInit as *mut W_BUTINIT as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<W_BUTINIT>() as libc::c_ulong);
        sButInit.formID = 4000 as libc::c_int as UDWORD;
        sButInit.id = 4403 as libc::c_int as UDWORD;
        sButInit.style = (0 as libc::c_int | 0x20 as libc::c_int) as UDWORD;
        sButInit.x =
            (8 as libc::c_int + 70 as libc::c_int + 2 as libc::c_int) as
                SWORD;
        sButInit.y = 4 as libc::c_int as SWORD;
        sButInit.width =
            iV_GetImageWidth(IntImages,
                             IMAGE_LOOP_DOWN as libc::c_int as UWORD);
        sButInit.height =
            iV_GetImageHeight(IntImages,
                              IMAGE_LOOP_DOWN as libc::c_int as UWORD);
        sButInit.pTip =
            strresGetString(psStringRes,
                            STR_INT_LOOP as libc::c_int as UDWORD);
        sButInit.FontID = WFont;
        sButInit.pDisplay =
            Some(intDisplayButtonPressed as
                     unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                          _: UDWORD, _: *mut UDWORD) -> ());
        sButInit.pUserData =
            ((IMAGE_LOOP_DOWN as libc::c_int & 0x3ff as libc::c_int) <<
                 20 as libc::c_int |
                 (IMAGE_LOOP_HI as libc::c_int & 0x3ff as libc::c_int) <<
                     10 as libc::c_int |
                 IMAGE_LOOP_UP as libc::c_int & 0x3ff as libc::c_int) as
                *mut libc::c_void;
        if widgAddButton(psWScreen, &mut sButInit) == 0 {
            return 0 as libc::c_int
        }
        if !psOwner.is_null() {
            psFactory =
                (*(psOwner as *mut STRUCTURE)).pFunctionality as *mut FACTORY;
            if !(*psFactory).psSubject.is_null() &&
                   (*psFactory).quantity as libc::c_int != 0 {
                widgSetButtonState(psWScreen, 4403 as libc::c_int as UDWORD,
                                   0x4 as libc::c_int as UDWORD);
            }
        }
        // create a text label for the loop quantity.
        memset(&mut sLabInit as *mut W_LABINIT as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<W_LABINIT>() as libc::c_ulong);
        sLabInit.formID = 4000 as libc::c_int as UDWORD;
        sLabInit.id = 4404 as libc::c_int as UDWORD;
        sLabInit.style = (0 as libc::c_int | 0x8000 as libc::c_int) as UDWORD;
        //#ifndef PSX
        sLabInit.x =
            (sButInit.x as libc::c_int - 15 as libc::c_int) as UWORD as SWORD;
        sLabInit.y = sButInit.y;
        //#else
//		sLabInit.x = sButInit.x - 15;
//		sLabInit.y = sButInit.y;
//#endif
        sLabInit.width = 12 as libc::c_int as UWORD;
        sLabInit.height = 15 as libc::c_int as UWORD;
        sLabInit.FontID = WFont;
        sLabInit.pUserData = psOwner as *mut libc::c_void;
        sLabInit.pCallback =
            Some(intAddLoopQuantity as
                     unsafe extern "C" fn(_: *mut _widget, _: *mut _w_context)
                         -> ());
        if widgAddLabel(psWScreen, &mut sLabInit) == 0 {
            return 0 as libc::c_int
        }
        /* store the common values for the text labels for the quantity
		to produce (on each button).*/
        memset(&mut sLabInit as *mut W_LABINIT as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<W_LABINIT>() as libc::c_ulong);
        sLabInit.id = 4600 as libc::c_int as UDWORD;
        sLabInit.style = (0 as libc::c_int | 0x8000 as libc::c_int) as UDWORD;
        sLabInit.x = (60 as libc::c_int - 12 as libc::c_int) as SWORD;
        sLabInit.y = 2 as libc::c_int as SWORD;
        sLabInit.width = 12 as libc::c_int as UWORD;
        sLabInit.height = 15 as libc::c_int as UWORD;
        sLabInit.FontID = WFont;
        sLabInit.pCallback =
            Some(intAddProdQuantity as
                     unsafe extern "C" fn(_: *mut _widget, _: *mut _w_context)
                         -> ())
    }
    /* Add the close button */
    memset(&mut sButInit as *mut W_BUTINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_BUTINIT>() as libc::c_ulong);
    sButInit.formID = 4000 as libc::c_int as UDWORD;
    sButInit.id = 4003 as libc::c_int as UDWORD;
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
    /* Calculate how many buttons will go on a form */
    butPerForm =
        ((132 as libc::c_int - 2 as libc::c_int) /
             (60 as libc::c_int + 2 as libc::c_int) *
             ((273 as libc::c_int - 2 as libc::c_int) /
                  (46 as libc::c_int + 2 as libc::c_int))) as UDWORD;
    /* Add the tabbed form */
    memset(&mut sFormInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_FORMINIT>() as libc::c_ulong);
    sFormInit.formID = 4000 as libc::c_int as UDWORD;
    sFormInit.id = 4004 as libc::c_int as UDWORD;
    sFormInit.style = 1 as libc::c_int as UDWORD;
    sFormInit.x = 0 as libc::c_int as SWORD;
    sFormInit.y = 18 as libc::c_int as SWORD;
    sFormInit.width = 132 as libc::c_int as UWORD;
    sFormInit.height = 273 as libc::c_int as UWORD;
    sFormInit.numMajor = numForms(numStats, butPerForm);
    sFormInit.majorPos = 1 as libc::c_int as UWORD;
    sFormInit.minorPos = 0 as libc::c_int as UWORD;
    sFormInit.majorSize = 26 as libc::c_int as UWORD;
    sFormInit.majorOffset = 2 as libc::c_int as SWORD;
    sFormInit.tabVertOffset = (11 as libc::c_int / 2 as libc::c_int) as SWORD;
    sFormInit.tabMajorThickness = 11 as libc::c_int as UWORD;
    sFormInit.tabMajorGap = 2 as libc::c_int as UWORD;
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
    //Build menu can have up to 80 stats - so can research now 13/09/99 AB
    if (objMode as libc::c_uint == IOBJ_BUILD as libc::c_int as libc::c_uint
            ||
            objMode as libc::c_uint ==
                IOBJ_RESEARCH as libc::c_int as libc::c_uint) &&
           sFormInit.numMajor as libc::c_int > 4 as libc::c_int {
        sFormInit.pUserData =
            &mut SmallTab as *mut TABDEF as *mut libc::c_void;
        sFormInit.majorSize =
            (sFormInit.majorSize as libc::c_int / 2 as libc::c_int) as UWORD
    }
    i = 0 as libc::c_int as UDWORD;
    while i < sFormInit.numMajor as libc::c_uint {
        sFormInit.aNumMinors[i as usize] = 1 as libc::c_int as UWORD;
        i = i.wrapping_add(1)
    }
    if widgAddForm(psWScreen, &mut sFormInit) == 0 { return 0 as libc::c_int }
    /* Add the stat buttons */
    memset(&mut sBFormInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_FORMINIT>() as libc::c_ulong);
    sBFormInit.formID = 4004 as libc::c_int as UDWORD;
    sBFormInit.majorID = 0 as libc::c_int as UWORD;
    sBFormInit.minorID = 0 as libc::c_int as UWORD;
    sBFormInit.id = 4100 as libc::c_int as UDWORD;
    sBFormInit.style = (4 as libc::c_int | 0x20 as libc::c_int) as UDWORD;
    sBFormInit.x = 4 as libc::c_int as SWORD;
    sBFormInit.y = 2 as libc::c_int as SWORD;
    sBFormInit.width = 60 as libc::c_int as UWORD;
    sBFormInit.height = 46 as libc::c_int as UWORD;
    memset(&mut sBarInit as *mut W_BARINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_BARINIT>() as libc::c_ulong);
    sBarInit.id = 4300 as libc::c_int as UDWORD;
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
    //sBarInit.pTip = strresGetString(psStringRes, STR_INT_PWRUSAGE);
    statID = 0 as libc::c_int as UDWORD;
    statForm = 0 as libc::c_int as UDWORD;
    i = 0 as libc::c_int as UDWORD;
    while i < numStats {
        if sBFormInit.id > 4179 as libc::c_int as libc::c_uint {
            //can't fit any more on the screen!
            debug(LOG_NEVER,
                  b"This is just a Warning!\n Max buttons have been allocated\x00"
                      as *const u8 as *const libc::c_char);
            break ;
        } else {
            Stat = *ppsStatsList_0.offset(i as isize);
            // If it's a droid the name might not be a stringID
            if (*Stat).ref_0 >= 0xc0000 as libc::c_int as libc::c_uint &&
                   (*Stat).ref_0 <
                       (0xc0000 as libc::c_int + 0x10000 as libc::c_int) as
                           libc::c_uint {
                sBFormInit.pTip =
                    getTemplateName(*ppsStatsList_0.offset(i as isize) as
                                        *mut DROID_TEMPLATE)
                //printf("Tip3 %s\n",sBFormInit.pTip);
            } else {
                sBFormInit.pTip =
                    getName((**ppsStatsList_0.offset(i as isize)).pName)
            }
            BufferID = i as SDWORD;
            if BufferID < 20 as libc::c_int * 4 as libc::c_int {
            } else {
                debug(LOG_ERROR,
                      b"BufferID > NUM_STATBUFFERS\x00" as *const u8 as
                          *const libc::c_char);
            };
            if BufferID < 20 as libc::c_int * 4 as libc::c_int {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"hci.c\x00" as *const u8 as *const libc::c_char,
                      6110 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 13],
                                                &[libc::c_char; 13]>(b"_intAddStats\x00")).as_ptr(),
                      b"BufferID < NUM_STATBUFFERS\x00" as *const u8 as
                          *const libc::c_char);
            };
            StatBuffers[BufferID as usize].InUse = 1 as libc::c_int;
            StatBuffers[BufferID as usize].Data =
                *ppsStatsList_0.offset(i as isize) as *mut libc::c_void;
            sBFormInit.pUserData =
                &mut *StatBuffers.as_mut_ptr().offset(BufferID as isize) as
                    *mut RENDERED_BUTTON as *mut libc::c_void;
            sBFormInit.pDisplay =
                Some(intDisplayStatsButton as
                         unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                              _: UDWORD, _: *mut UDWORD)
                             -> ());
            if widgAddForm(psWScreen, &mut sBFormInit) == 0 {
                return 0 as libc::c_int
            }
            widgSetColour(psWScreen, sBFormInit.id,
                          WCOL_BKGRND as libc::c_int as UDWORD,
                          0 as libc::c_int as UBYTE,
                          0 as libc::c_int as UBYTE,
                          0 as libc::c_int as UBYTE);
            //Stat = ppsStatsList[i];
            if (*Stat).ref_0 >= 0xd0000 as libc::c_int as libc::c_uint &&
                   (*Stat).ref_0 <
                       (0xd0000 as libc::c_int + 0x10000 as libc::c_int) as
                           libc::c_uint {
                // It's a structure.
                //sBarInit.pTip = strresGetString(psStringRes, STR_INT_BLDSPEED);
			//sBarInit.size = (UWORD)(((STRUCTURE_STATS*)Stat)->buildPoints / BUILDPOINTS_STRUCTDIV);
                sBarInit.size =
                    (*(Stat as
                           *mut STRUCTURE_STATS)).powerToBuild.wrapping_div(5
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_uint)
                        as UWORD;
                if sBarInit.size as libc::c_int > 100 as libc::c_int {
                    sBarInit.size = 100 as libc::c_int as UWORD
                }
                sBarInit.formID = sBFormInit.id;
                sBarInit.iRange = 1000 as libc::c_int as UWORD;
                if widgAddBarGraph(psWScreen, &mut sBarInit) == 0 {
                    return 0 as libc::c_int
                }
            } else if (*Stat).ref_0 >= 0xc0000 as libc::c_int as libc::c_uint
                          &&
                          (*Stat).ref_0 <
                              (0xc0000 as libc::c_int +
                                   0x10000 as libc::c_int) as libc::c_uint {
                // It's a droid.
                //sBarInit.size = (UWORD)(((DROID_TEMPLATE*)Stat)->buildPoints  / BUILDPOINTS_DROIDDIV);
                sBarInit.size =
                    (*(Stat as
                           *mut DROID_TEMPLATE)).powerPoints.wrapping_div(5 as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_uint)
                        as UWORD;
                //sBarInit.pTip = strresGetString(psStringRes, STR_INT_PWRUSAGE);
                if sBarInit.size as libc::c_int > 100 as libc::c_int {
                    sBarInit.size = 100 as libc::c_int as UWORD
                }
                sBarInit.formID = sBFormInit.id;
                sBarInit.iRange = 1000 as libc::c_int as UWORD;
                if widgAddBarGraph(psWScreen, &mut sBarInit) == 0 {
                    return 0 as libc::c_int
                }
                // Add a text label for the quantity to produce.
                sLabInit.formID = sBFormInit.id;
                sLabInit.pUserData = Stat as *mut libc::c_void;
                if widgAddLabel(psWScreen, &mut sLabInit) == 0 {
                    return 0 as libc::c_int
                }
                sLabInit.id = sLabInit.id.wrapping_add(1)
            } else if (*Stat).ref_0 >= 0xb0000 as libc::c_int as libc::c_uint
                          &&
                          (*Stat).ref_0 <
                              (0xb0000 as libc::c_int +
                                   0x10000 as libc::c_int) as libc::c_uint {
                // It's a Research topic.
                //new icon in for groups - AB 12/01/99
                memset(&mut sLabInit as *mut W_LABINIT as *mut libc::c_void,
                       0 as libc::c_int,
                       ::std::mem::size_of::<W_LABINIT>() as libc::c_ulong);
                sLabInit.formID = sBFormInit.id;
                sLabInit.id =
                    (4500 as libc::c_int as
                         libc::c_uint).wrapping_add(sBFormInit.id.wrapping_sub(4100
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   libc::c_uint));
                sLabInit.style = 0 as libc::c_int as UDWORD;
                sLabInit.x = (60 as libc::c_int - 16 as libc::c_int) as SWORD;
                sLabInit.y = 3 as libc::c_int as SWORD;
                sLabInit.width = 12 as libc::c_int as UWORD;
                sLabInit.height = 15 as libc::c_int as UWORD;
                sLabInit.pUserData = Stat as *mut libc::c_void;
                sLabInit.pDisplay =
                    Some(intDisplayResSubGroup as
                             unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                                  _: UDWORD, _: *mut UDWORD)
                                 -> ());
                widgAddLabel(psWScreen, &mut sLabInit);
                sBarInit.size =
                    (*(Stat as
                           *mut RESEARCH)).researchPower.wrapping_div(5 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_uint)
                        as UWORD;
                if sBarInit.size as libc::c_int > 100 as libc::c_int {
                    sBarInit.size = 100 as libc::c_int as UWORD
                }
                if bMultiPlayer != 0 {
                    let mut psOtherStruct: *mut STRUCTURE =
                        0 as *mut STRUCTURE;
                    let mut ii: UBYTE = 0;
                    ii = 0 as libc::c_int as UBYTE;
                    's_999:
                        while (ii as libc::c_int) < 8 as libc::c_int {
                            if ii as libc::c_uint != selectedPlayer &&
                                   aiCheckAlliances(selectedPlayer,
                                                    ii as UDWORD) != 0 {
                                //add power bar as well
                                //sBarInit.pTip = strresGetString(psStringRes, STR_INT_PWRUSAGE);
                                //check each research facility to see if they are doing this topic.
                                psOtherStruct = apsStructLists[ii as usize];
                                while !psOtherStruct.is_null() {
                                    if (*(*psOtherStruct).pStructureType).type_0
                                           ==
                                           REF_RESEARCH as libc::c_int as
                                               libc::c_uint &&
                                           (*psOtherStruct).status as
                                               libc::c_int ==
                                               SS_BUILT as libc::c_int &&
                                           !(*((*psOtherStruct).pFunctionality
                                                   as
                                                   *mut RESEARCH_FACILITY)).psSubject.is_null()
                                           &&
                                           (*(*((*psOtherStruct).pFunctionality
                                                    as
                                                    *mut RESEARCH_FACILITY)).psSubject).ref_0
                                               == (*Stat).ref_0 {
                                        // add a label.
							//	DBPRINTF(("!"));
                                        memset(&mut sLabInit as *mut W_LABINIT
                                                   as *mut libc::c_void,
                                               0 as libc::c_int,
                                               ::std::mem::size_of::<W_LABINIT>()
                                                   as libc::c_ulong);
                                        sLabInit.formID = sBFormInit.id;
                                        sLabInit.id =
                                            (4800 as libc::c_int as
                                                 libc::c_uint).wrapping_add(sBFormInit.id.wrapping_sub(4100
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_uint));
                                        sLabInit.style =
                                            0 as libc::c_int as UDWORD;
                                        sLabInit.x =
                                            (60 as libc::c_int -
                                                 19 as libc::c_int) as SWORD;
                                        sLabInit.y =
                                            (46 as libc::c_int -
                                                 19 as libc::c_int) as SWORD;
                                        sLabInit.width =
                                            12 as libc::c_int as UWORD;
                                        sLabInit.height =
                                            15 as libc::c_int as UWORD;
                                        sLabInit.pUserData =
                                            ii as libc::c_int as
                                                *mut libc::c_void;
                                        sLabInit.pTip =
                                            getPlayerName(ii as UDWORD);
                                        sLabInit.pDisplay =
                                            Some(intDisplayAllyIcon as
                                                     unsafe extern "C" fn(_:
                                                                              *mut _widget,
                                                                          _:
                                                                              UDWORD,
                                                                          _:
                                                                              UDWORD,
                                                                          _:
                                                                              *mut UDWORD)
                                                         -> ());
                                        widgAddLabel(psWScreen,
                                                     &mut sLabInit);
                                        break 's_999 ;
                                    } else {
                                        psOtherStruct =
                                            (*psOtherStruct).psNext
                                    }
                                }
                            }
                            ii = ii.wrapping_add(1)
                        }
                }
                sBarInit.formID = sBFormInit.id;
                if widgAddBarGraph(psWScreen, &mut sBarInit) == 0 {
                    return 0 as libc::c_int
                }
            }
            // if multiplayer, if research topic is being done by another ally then mark as such..
            /* If this matches psSelected note the form and button */
            if *ppsStatsList_0.offset(i as isize) == psSelected {
                statID = sBFormInit.id;
                statForm = sBFormInit.majorID as UDWORD
            }
            /* Update the init struct for the next button */
            sBFormInit.id =
                (sBFormInit.id as
                     libc::c_uint).wrapping_add(1 as libc::c_int as
                                                    libc::c_uint) as UDWORD as
                    UDWORD;
            sBFormInit.x =
                (sBFormInit.x as libc::c_int +
                     (60 as libc::c_int + 2 as libc::c_int)) as SWORD;
            if sBFormInit.x as libc::c_int + 60 as libc::c_int +
                   2 as libc::c_int > 132 as libc::c_int {
                // - STAT_TABWIDTH)
                sBFormInit.x = 4 as libc::c_int as SWORD; //STAT_GAP;
                sBFormInit.y =
                    (sBFormInit.y as libc::c_int +
                         (46 as libc::c_int + 2 as libc::c_int)) as SWORD
            }
            if sBFormInit.y as libc::c_int + 46 as libc::c_int +
                   2 as libc::c_int > 273 as libc::c_int {
                // - STAT_TITLEHEIGHT)
                sBFormInit.y = 2 as libc::c_int as SWORD; //STAT_GAP;
                sBFormInit.majorID =
                    (sBFormInit.majorID as libc::c_int + 1 as libc::c_int) as
                        UWORD
            }
            sBarInit.id =
                (sBarInit.id as
                     libc::c_uint).wrapping_add(1 as libc::c_int as
                                                    libc::c_uint) as UDWORD as
                    UDWORD;
            i = i.wrapping_add(1)
        }
    }
    //	widgStartScreen(psWScreen);
    /* Set the correct page and button if necessary */
    if statID != 0 {
        widgSetTabs(psWScreen, 4004 as libc::c_int as UDWORD,
                    statForm as UWORD, 0 as libc::c_int as UWORD);
        widgSetButtonState(psWScreen, statID, 0x4 as libc::c_int as UDWORD);
        intSetCurrentCursorPosition(&mut InterfaceSnap, statID);
    }
    StatsUp = 1 as libc::c_int;
    // call the tutorial callbacks if necessary
    if bInTutorial != 0 {
        match objMode as libc::c_uint {
            16 => {
                eventFireCallbackTrigger(CALL_BUILDLIST as libc::c_int as
                                             TRIGGER_TYPE);
            }
            20 => {
                eventFireCallbackTrigger(CALL_RESEARCHLIST as libc::c_int as
                                             TRIGGER_TYPE);
            }
            19 => {
                eventFireCallbackTrigger(CALL_MANULIST as libc::c_int as
                                             TRIGGER_TYPE);
            }
            _ => { }
        }
    }
    //DBPRINTF(("intAddStats OK\n");
    return 1 as libc::c_int;
}
/* Select a command droid */
unsafe extern "C" fn selectCommand(mut psObj: *mut BASE_OBJECT) -> BOOL {
    //	UDWORD	i;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    if 1 as libc::c_int != 0 &&
           (*psObj).type_0 as libc::c_uint ==
               OBJ_DROID as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"selectConstruction: invalid droid pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 &&
           (*psObj).type_0 as libc::c_uint ==
               OBJ_DROID as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"hci.c\x00" as *const u8 as *const libc::c_char,
              6310 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"selectCommand\x00")).as_ptr(),
              b"PTRVALID(psObj, sizeof(DROID)) && psObj->type == OBJ_DROID\x00"
                  as *const u8 as *const libc::c_char);
    };
    psDroid = psObj as *mut DROID;
    //check the droid type
    if (*psDroid).droidType as libc::c_uint ==
           DROID_COMMAND as libc::c_int as libc::c_uint &&
           (*psDroid).died == 0 as libc::c_int as libc::c_uint {
        return 1 as libc::c_int
    }
    /*for (i=0; i < psDroid->numProgs; i++)
	{
		if (psDroid->asProgs[i].psStats->order == ORDER_BUILD)
		{
			return TRUE;
		}
	}*/
    return 0 as libc::c_int;
}
/* Return the stats for a command droid */
unsafe extern "C" fn getCommandStats(mut psObj: *mut BASE_OBJECT)
 -> *mut BASE_STATS {
    return 0 as *mut BASE_STATS;
}
/* Set the stats for a command droid */
unsafe extern "C" fn setCommandStats(mut psObj: *mut BASE_OBJECT,
                                     mut psStats: *mut BASE_STATS) -> BOOL {
    return 1 as libc::c_int;
}
/* Select a construction droid */
unsafe extern "C" fn selectConstruction(mut psObj: *mut BASE_OBJECT) -> BOOL {
    //	UDWORD	i;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    if 1 as libc::c_int != 0 &&
           (*psObj).type_0 as libc::c_uint ==
               OBJ_DROID as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"selectConstruction: invalid droid pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 &&
           (*psObj).type_0 as libc::c_uint ==
               OBJ_DROID as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"hci.c\x00" as *const u8 as *const libc::c_char,
              6348 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 19],
                                        &[libc::c_char; 19]>(b"selectConstruction\x00")).as_ptr(),
              b"PTRVALID(psObj, sizeof(DROID)) && psObj->type == OBJ_DROID\x00"
                  as *const u8 as *const libc::c_char);
    };
    psDroid = psObj as *mut DROID;
    //check the droid type
	//if ( (psDroid->droidType == DROID_CONSTRUCT) && (psDroid->died == 0) )
    if ((*psDroid).droidType as libc::c_uint ==
            DROID_CONSTRUCT as libc::c_int as libc::c_uint ||
            (*psDroid).droidType as libc::c_uint ==
                DROID_CYBORG_CONSTRUCT as libc::c_int as libc::c_uint) &&
           (*psDroid).died == 0 as libc::c_int as libc::c_uint {
        return 1 as libc::c_int
    }
    /*for (i=0; i < psDroid->numProgs; i++)
	{
		if (psDroid->asProgs[i].psStats->order == ORDER_BUILD)
		{
			return TRUE;
		}
	}*/
    return 0 as libc::c_int;
}
/* Return the stats for a construction droid */
unsafe extern "C" fn getConstructionStats(mut psObj: *mut BASE_OBJECT)
 -> *mut BASE_STATS {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut Stats: *mut BASE_STATS = 0 as *mut BASE_STATS;
    let mut Structure: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut x: UDWORD = 0;
    let mut y: UDWORD = 0;
    if 1 as libc::c_int != 0 &&
           (*psObj).type_0 as libc::c_uint ==
               OBJ_DROID as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"getConstructionStats: invalid droid pointer\x00" as *const u8
                  as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 &&
           (*psObj).type_0 as libc::c_uint ==
               OBJ_DROID as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"hci.c\x00" as *const u8 as *const libc::c_char,
              6378 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 21],
                                        &[libc::c_char; 21]>(b"getConstructionStats\x00")).as_ptr(),
              b"PTRVALID(psObj, sizeof(DROID)) && psObj->type == OBJ_DROID\x00"
                  as *const u8 as *const libc::c_char);
    };
    psDroid = psObj as *mut DROID;
    //if(droidType(psDroid) != DROID_CONSTRUCT) return NULL;
    if !(droidType(psDroid) as libc::c_uint ==
             DROID_CONSTRUCT as libc::c_int as libc::c_uint ||
             droidType(psDroid) as libc::c_uint ==
                 DROID_CYBORG_CONSTRUCT as libc::c_int as libc::c_uint) {
        return 0 as *mut BASE_STATS
    }
    if orderStateStatsLoc(psDroid, DORDER_BUILD, &mut Stats, &mut x, &mut y)
           != 0 {
        // Moving to build location?
        return Stats
    } else {
        if orderStateObj(psDroid, DORDER_BUILD,
                         &mut Structure as *mut *mut STRUCTURE as
                             *mut *mut BASE_OBJECT) != 0 &&
               (*psDroid).order == DORDER_BUILD as libc::c_int {
            // Is building
            //		DBPRINTF(("BUILDING %p : %d %p %p\n",psDroid,psDroid->order,psDroid->psTarStats,psDroid->psTarget));
            return (*psDroid).psTarStats
        } else {
            if orderStateObj(psDroid, DORDER_HELPBUILD,
                             &mut Structure as *mut *mut STRUCTURE as
                                 *mut *mut BASE_OBJECT) != 0 &&
                   ((*psDroid).order == DORDER_HELPBUILD as libc::c_int ||
                        (*psDroid).order == DORDER_LINEBUILD as libc::c_int) {
                //Is helping
                //		DBPRINTF(("HELPING  %p : %d %p %p\n",psDroid,psDroid->order,psDroid->psTarStats,psDroid->psTarget));
//		return (BASE_STATS*)((STRUCTURE*)psDroid->psTarget)->pStructureType;
                return (*Structure).pStructureType as *mut BASE_STATS
            } else {
                if orderState(psDroid, DORDER_DEMOLISH) != 0 {
                    return structGetDemolishStat() as *mut BASE_STATS
                }
            }
        }
    }
    return 0 as *mut BASE_STATS;
}
/* Set the stats for a construction droid */
unsafe extern "C" fn setConstructionStats(mut psObj: *mut BASE_OBJECT,
                                          mut psStats: *mut BASE_STATS)
 -> BOOL {
    let mut psSStats: *mut STRUCTURE_STATS = 0 as *mut STRUCTURE_STATS;
    //UDWORD				i;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    if 1 as libc::c_int != 0 &&
           (*psObj).type_0 as libc::c_uint ==
               OBJ_DROID as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"setConstructionStats: invalid droid pointer\x00" as *const u8
                  as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 &&
           (*psObj).type_0 as libc::c_uint ==
               OBJ_DROID as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"hci.c\x00" as *const u8 as *const libc::c_char,
              6415 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 21],
                                        &[libc::c_char; 21]>(b"setConstructionStats\x00")).as_ptr(),
              b"PTRVALID(psObj, sizeof(DROID)) && psObj->type == OBJ_DROID\x00"
                  as *const u8 as *const libc::c_char);
    };
    /* psStats might be NULL if the operation is canceled in the middle */
    if psStats.is_null() || 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"setConstructionStats: invalid stats pointer\x00" as *const u8
                  as *const libc::c_char);
    };
    if psStats.is_null() || 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"hci.c\x00" as *const u8 as *const libc::c_char,
              6418 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 21],
                                        &[libc::c_char; 21]>(b"setConstructionStats\x00")).as_ptr(),
              b"psStats == NULL || PTRVALID(psStats, sizeof(STRUCTURE_STATS))\x00"
                  as *const u8 as *const libc::c_char);
    };
    if !psStats.is_null() {
        psSStats = psStats as *mut STRUCTURE_STATS;
        psDroid = psObj as *mut DROID;
        //check for demolish first
        if psSStats == structGetDemolishStat() {
            objMode = IOBJ_DEMOLISHSEL;
            // When demolish requested, need to select a construction droid, not really any
			// choice in this as demolishing uses the droid targeting interface rather than
			// the build positioning interface and therefore requires a construction droid
			// to be selected.
            clearSel();
            //			psDroid->selected = TRUE;
            SelectDroid(psDroid);
            if driveModeActive() != 0 { driveSelectionChanged(); }
            /* Re-written to allow demolish order to be added to the queuing system

            //make sure its not on the way to build something
   		    orderDroid(psDroid,DORDER_STOP);
            //clear out target (but not if queuing)
            psDroid->psTarget = NULL;
    		psDroid->psTarStats = (BASE_STATS *) structGetDemolishStat();

            */
            return 1 as libc::c_int
        }
        //Power is obtained gradually so no need to check
		/* check enough power to build*/
		/*if (!checkPower(selectedPlayer, psSStats->powerToBuild, TRUE))
		{
			return FALSE;
		}*/
        /* Store the stats for future use */
        psPositionStats = psStats;
        /* Now start looking for a location for the structure */
        if !psSStats.is_null() {
            //			if ( psSStats == structGetDemolishStat() )
//			{
//				objMode = IOBJ_DEMOLISHSEL;
//				psDroid->psTarStats = (BASE_STATS *) structGetDemolishStat();
            //				//set the droids current program
//				/*for (i=0; i < psDroid->numProgs; i++)
//				{
//					if (psDroid->asProgs[i].psStats->order == ORDER_DEMOLISH)
//					{
//						psDroid->activeProg = i;
//					}
//				}*/
//			}
//			else
            objMode = IOBJ_BUILDSEL;
            intStartStructPosition(psStats, psDroid);
            //set the droids current program
				/*for (i=0; i < psDroid->numProgs; i++)
				{
					if (psDroid->asProgs[i].psStats->order == ORDER_BUILD)
					{
						psDroid->activeProg = i;
					}
				}*/
        } else { orderDroid(psDroid, DORDER_STOP); }
    } else {
        psDroid = psObj as *mut DROID;
        orderDroid(psDroid, DORDER_STOP);
    }
    return 1 as libc::c_int;
}
/* Select a research facility */
unsafe extern "C" fn selectResearch(mut psObj: *mut BASE_OBJECT) -> BOOL {
    let mut psResFacility: *mut STRUCTURE = 0 as *mut STRUCTURE;
    if 1 as libc::c_int != 0 &&
           (*psObj).type_0 as libc::c_uint ==
               OBJ_STRUCTURE as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"selectResearch: invalid Structure pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 &&
           (*psObj).type_0 as libc::c_uint ==
               OBJ_STRUCTURE as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"hci.c\x00" as *const u8 as *const libc::c_char,
              6514 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 15],
                                        &[libc::c_char; 15]>(b"selectResearch\x00")).as_ptr(),
              b"PTRVALID(psObj, sizeof(STRUCTURE)) && psObj->type == OBJ_STRUCTURE\x00"
                  as *const u8 as *const libc::c_char);
    };
    psResFacility = psObj as *mut STRUCTURE;
    /* A Structure is a research facility if its type = REF_RESEARCH and is
	   completely built*/
    if (*(*psResFacility).pStructureType).type_0 ==
           REF_RESEARCH as libc::c_int as libc::c_uint &&
           (*psResFacility).status as libc::c_int == SS_BUILT as libc::c_int
           && (*psResFacility).died == 0 as libc::c_int as libc::c_uint {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
/* Return the stats for a research facility */
unsafe extern "C" fn getResearchStats(mut psObj: *mut BASE_OBJECT)
 -> *mut BASE_STATS {
    let mut psBuilding: *mut STRUCTURE = 0 as *mut STRUCTURE;
    if 1 as libc::c_int != 0 &&
           (*psObj).type_0 as libc::c_uint ==
               OBJ_STRUCTURE as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"getResearchTip: invalid Structure pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 &&
           (*psObj).type_0 as libc::c_uint ==
               OBJ_STRUCTURE as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"hci.c\x00" as *const u8 as *const libc::c_char,
              6534 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 17],
                                        &[libc::c_char; 17]>(b"getResearchStats\x00")).as_ptr(),
              b"PTRVALID(psObj, sizeof(STRUCTURE)) && psObj->type == OBJ_STRUCTURE\x00"
                  as *const u8 as *const libc::c_char);
    };
    psBuilding = psObj as *mut STRUCTURE;
    return (*((*psBuilding).pFunctionality as
                  *mut RESEARCH_FACILITY)).psSubject as *mut BASE_STATS;
}
/* Set the stats for a research facility */
unsafe extern "C" fn setResearchStats(mut psObj: *mut BASE_OBJECT,
                                      mut psStats: *mut BASE_STATS) -> BOOL {
    let mut psBuilding: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut pResearch: *mut RESEARCH = 0 as *mut RESEARCH;
    let mut pPlayerRes: *mut PLAYER_RESEARCH = 0 as *mut PLAYER_RESEARCH;
    let mut count: UDWORD = 0;
    let mut psResFacilty: *mut RESEARCH_FACILITY =
        0 as *mut RESEARCH_FACILITY;
    if 1 as libc::c_int != 0 &&
           (*psObj).type_0 as libc::c_uint ==
               OBJ_STRUCTURE as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"setResearchStats: invalid Structure pointer\x00" as *const u8
                  as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 &&
           (*psObj).type_0 as libc::c_uint ==
               OBJ_STRUCTURE as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"hci.c\x00" as *const u8 as *const libc::c_char,
              6551 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 17],
                                        &[libc::c_char; 17]>(b"setResearchStats\x00")).as_ptr(),
              b"PTRVALID(psObj, sizeof(STRUCTURE)) && psObj->type == OBJ_STRUCTURE\x00"
                  as *const u8 as *const libc::c_char);
    };
    /* psStats might be NULL if the operation is canceled in the middle */
    if psStats.is_null() || 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"setResearchStats: invalid stats pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if psStats.is_null() || 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"hci.c\x00" as *const u8 as *const libc::c_char,
              6554 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 17],
                                        &[libc::c_char; 17]>(b"setResearchStats\x00")).as_ptr(),
              b"psStats == NULL || PTRVALID(psStats, sizeof(RESEARCH))\x00" as
                  *const u8 as *const libc::c_char);
    };
    psBuilding = psObj as *mut STRUCTURE;
    psResFacilty = (*psBuilding).pFunctionality as *mut RESEARCH_FACILITY;
    //initialise the subject
    (*psResFacilty).psSubject = 0 as *mut _base_stats;
    //set up the player_research
    if !psStats.is_null() {
        pResearch = psStats as *mut RESEARCH;
        count =
            (*pResearch).ref_0.wrapping_sub(0xb0000 as libc::c_int as
                                                libc::c_uint);
        //meant to still be in the list but greyed out
        pPlayerRes =
            asPlayerResList[selectedPlayer as usize].offset(count as isize);
        /*subtract the power required to research*/
		/*if (pPlayerRes->researched != CANCELLED_RESEARCH)
		{
			if (!usePower(selectedPlayer, pResearch->researchPower))
			{
				addConsoleMessage("Research: No Power",DEFAULT_JUSTIFY);
				return FALSE;
			}
		}*/
        //set the subject up
        (*psResFacilty).psSubject = psStats;
        if (*pPlayerRes).ResearchStatus as libc::c_int & 0x2 as libc::c_int !=
               0 {
            //set up as if all power available for cancelled topics
            (*psResFacilty).powerAccrued = (*pResearch).researchPower
        } else {
            (*psResFacilty).powerAccrued = 0 as libc::c_int as UDWORD
        } // inform others, I'm researching this.
        sendReseachStatus(psBuilding, count, selectedPlayer as UBYTE,
                          1 as libc::c_int);
        (*pPlayerRes).ResearchStatus =
            ((*pPlayerRes).ResearchStatus as libc::c_int &
                 !(0x1 as libc::c_int | 0x2 as libc::c_int |
                       0x4 as libc::c_int) | 0x1 as libc::c_int) as UBYTE;
        //psResFacilty->timeStarted = gameTime;
        (*psResFacilty).timeStarted = 0 as libc::c_int as UDWORD;
        (*psResFacilty).timeStartHold = 0 as libc::c_int as UDWORD;
        //this is no longer used...AB 30/06/99
        (*psResFacilty).timeToResearch =
            ((*pResearch).researchPoints as
                 libc::c_uint).wrapping_div((*psResFacilty).researchPoints);
        //check for zero research time - usually caused by 'silly' data!
        if (*psResFacilty).timeToResearch == 0 as libc::c_int as libc::c_uint
           {
            //set to 1/1000th sec - ie very fast!
            (*psResFacilty).timeToResearch = 1 as libc::c_int as UDWORD
        }
        //stop the button from flashing once a topic has been chosen
        stopReticuleButtonFlash(5 as libc::c_int as UDWORD);
    }
    return 1 as libc::c_int;
}
/* Select a Factory */
unsafe extern "C" fn selectManufacture(mut psObj: *mut BASE_OBJECT) -> BOOL {
    let mut psBuilding: *mut STRUCTURE = 0 as *mut STRUCTURE;
    if 1 as libc::c_int != 0 &&
           (*psObj).type_0 as libc::c_uint ==
               OBJ_STRUCTURE as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"selectManufacture: invalid Structure pointer\x00" as *const u8
                  as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 &&
           (*psObj).type_0 as libc::c_uint ==
               OBJ_STRUCTURE as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"hci.c\x00" as *const u8 as *const libc::c_char,
              6620 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 18],
                                        &[libc::c_char; 18]>(b"selectManufacture\x00")).as_ptr(),
              b"PTRVALID(psObj, sizeof(STRUCTURE)) && psObj->type == OBJ_STRUCTURE\x00"
                  as *const u8 as *const libc::c_char);
    };
    psBuilding = psObj as *mut STRUCTURE;
    /* A Structure is a Factory if its type = REF_FACTORY or REF_CYBORG_FACTORY or
	REF_VTOL_FACTORY and it is completely built*/
    if ((*(*psBuilding).pStructureType).type_0 ==
            REF_FACTORY as libc::c_int as libc::c_uint ||
            (*(*psBuilding).pStructureType).type_0 ==
                REF_CYBORG_FACTORY as libc::c_int as libc::c_uint ||
            (*(*psBuilding).pStructureType).type_0 ==
                REF_VTOL_FACTORY as libc::c_int as libc::c_uint) &&
           (*psBuilding).status as libc::c_int == SS_BUILT as libc::c_int &&
           (*psBuilding).died == 0 as libc::c_int as libc::c_uint {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
/* Return the stats for a Factory */
unsafe extern "C" fn getManufactureStats(mut psObj: *mut BASE_OBJECT)
 -> *mut BASE_STATS {
    let mut psBuilding: *mut STRUCTURE = 0 as *mut STRUCTURE;
    if 1 as libc::c_int != 0 &&
           (*psObj).type_0 as libc::c_uint ==
               OBJ_STRUCTURE as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"getManufactureTip: invalid Structure pointer\x00" as *const u8
                  as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 &&
           (*psObj).type_0 as libc::c_uint ==
               OBJ_STRUCTURE as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"hci.c\x00" as *const u8 as *const libc::c_char,
              6642 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 20],
                                        &[libc::c_char; 20]>(b"getManufactureStats\x00")).as_ptr(),
              b"PTRVALID(psObj, sizeof(STRUCTURE)) && psObj->type == OBJ_STRUCTURE\x00"
                  as *const u8 as *const libc::c_char);
    };
    psBuilding = psObj as *mut STRUCTURE;
    return (*((*psBuilding).pFunctionality as *mut FACTORY)).psSubject;
}
/* Set the stats for a Factory */
unsafe extern "C" fn setManufactureStats(mut psObj: *mut BASE_OBJECT,
                                         mut psStats: *mut BASE_STATS)
 -> BOOL {
    let mut Structure: *mut STRUCTURE = 0 as *mut STRUCTURE;
    if 1 as libc::c_int != 0 &&
           (*psObj).type_0 as libc::c_uint ==
               OBJ_STRUCTURE as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"setManufactureStats: invalid Structure pointer\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 &&
           (*psObj).type_0 as libc::c_uint ==
               OBJ_STRUCTURE as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"hci.c\x00" as *const u8 as *const libc::c_char,
              6655 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 20],
                                        &[libc::c_char; 20]>(b"setManufactureStats\x00")).as_ptr(),
              b"PTRVALID(psObj, sizeof(STRUCTURE)) && psObj->type == OBJ_STRUCTURE\x00"
                  as *const u8 as *const libc::c_char);
    };
    /* psStats might be NULL if the operation is canceled in the middle */
    if psStats.is_null() || 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"setManufactureStats: invalid stats pointer\x00" as *const u8
                  as *const libc::c_char);
    };
    if psStats.is_null() || 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"hci.c\x00" as *const u8 as *const libc::c_char,
              6658 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 20],
                                        &[libc::c_char; 20]>(b"setManufactureStats\x00")).as_ptr(),
              b"psStats == NULL || PTRVALID(psStats, sizeof(DROID_TEMPLATE))\x00"
                  as *const u8 as *const libc::c_char);
    };
    Structure = psObj as *mut STRUCTURE;
    //check to see if the factory was already building something
    if (*((*Structure).pFunctionality as *mut FACTORY)).psSubject.is_null() {
        //factory not currently building so set up the factory stats
        if !psStats.is_null() {
            /* Set the factory to build droid(s) */
            if structSetManufacture(Structure, psStats as *mut DROID_TEMPLATE,
                                    1 as libc::c_int as UBYTE) == 0 {
                return 0 as libc::c_int
            }
        }
    }
    /*else
	{
		// Stop manufacturing.
		//return half the power cost if cancelled mid production
		if (((FACTORY*)Structure->pFunctionality)->timeStarted != ACTION_START_TIME)
		{
			if (((FACTORY*)Structure->pFunctionality)->psSubject != NULL)
			{
				addPower(Structure->player, ((DROID_TEMPLATE *)((FACTORY*)Structure->
					pFunctionality)->psSubject)->powerPoints / 2);
			}
		}
		else
		{
			//return the power accrued
			addPower(Structure->player, ((FACTORY*)Structure->pFunctionality)->powerAccrued);
		}
		((FACTORY*)Structure->pFunctionality)->quantity = 0;
		((FACTORY*)Structure->pFunctionality)->psSubject = NULL;
		((FACTORY*)Structure->pFunctionality)->powerAccrued = 0;
		intManufactureFinished(Structure);
	}*/
    return 1 as libc::c_int;
}
/* Add the build widgets to the widget screen */
/* If psSelected != NULL it specifies which droid should be hilited */
/* Add the build widgets to the widget screen */
/* If psSelected != NULL it specifies which droid should be hilited */
unsafe extern "C" fn intAddBuild(mut psSelected: *mut DROID) -> BOOL {
    /* Store the correct stats list for future reference */
    ppsStatsList = apsStructStatsList as *mut *mut BASE_STATS;
    objSelectFunc =
        Some(selectConstruction as
                 unsafe extern "C" fn(_: *mut BASE_OBJECT) -> BOOL);
    objGetStatsFunc =
        Some(getConstructionStats as
                 unsafe extern "C" fn(_: *mut BASE_OBJECT)
                     -> *mut BASE_STATS);
    objSetStatsFunc =
        Some(setConstructionStats as
                 unsafe extern "C" fn(_: *mut BASE_OBJECT, _: *mut BASE_STATS)
                     -> BOOL);
    /* Set the sub mode */
    objMode = IOBJ_BUILD;
    /* Create the object screen with the required data */
    return intAddObject(apsDroidLists[selectedPlayer as usize] as
                            *mut BASE_OBJECT, psSelected as *mut BASE_OBJECT,
                        1 as libc::c_int);
}
/* Add the manufacture widgets to the widget screen */
/* If psSelected != NULL it specifies which factory should be hilited */
/* Add the manufacture widgets to the widget screen */
/* If psSelected != NULL it specifies which factory should be hilited */
unsafe extern "C" fn intAddManufacture(mut psSelected: *mut STRUCTURE)
 -> BOOL {
    /* Store the correct stats list for future reference */
    ppsStatsList = apsTemplateList as *mut *mut BASE_STATS;
    objSelectFunc =
        Some(selectManufacture as
                 unsafe extern "C" fn(_: *mut BASE_OBJECT) -> BOOL);
    objGetStatsFunc =
        Some(getManufactureStats as
                 unsafe extern "C" fn(_: *mut BASE_OBJECT)
                     -> *mut BASE_STATS);
    objSetStatsFunc =
        Some(setManufactureStats as
                 unsafe extern "C" fn(_: *mut BASE_OBJECT, _: *mut BASE_STATS)
                     -> BOOL);
    /* Set the sub mode */
    objMode = IOBJ_MANUFACTURE;
    /* Create the object screen with the required data */
	//return intAddObject((BASE_OBJECT *)apsStructLists[selectedPlayer],
    return intAddObject(interfaceStructList() as *mut BASE_OBJECT,
                        psSelected as *mut BASE_OBJECT, 1 as libc::c_int);
}
/* Add the research widgets to the widget screen */
/* If psSelected != NULL it specifies which droid should be hilited */
/* Add the research widgets to the widget screen */
/* If psSelected != NULL it specifies which droid should be hilited */
unsafe extern "C" fn intAddResearch(mut psSelected: *mut STRUCTURE) -> BOOL {
    ppsStatsList = ppResearchList as *mut *mut BASE_STATS;
    objSelectFunc =
        Some(selectResearch as
                 unsafe extern "C" fn(_: *mut BASE_OBJECT) -> BOOL);
    objGetStatsFunc =
        Some(getResearchStats as
                 unsafe extern "C" fn(_: *mut BASE_OBJECT)
                     -> *mut BASE_STATS);
    objSetStatsFunc =
        Some(setResearchStats as
                 unsafe extern "C" fn(_: *mut BASE_OBJECT, _: *mut BASE_STATS)
                     -> BOOL);
    /* Set the sub mode */
    objMode = IOBJ_RESEARCH;
    /* Create the object screen with the required data */
	//return intAddObject((BASE_OBJECT *)apsStructLists[selectedPlayer],
    return intAddObject(interfaceStructList() as *mut BASE_OBJECT,
                        psSelected as *mut BASE_OBJECT, 1 as libc::c_int);
}
/* Add the command droid widgets to the widget screen */
/* If psSelected != NULL it specifies which droid should be hilited */
/* Add the command droid widgets to the widget screen */
/* If psSelected != NULL it specifies which droid should be hilited */
unsafe extern "C" fn intAddCommand(mut psSelected: *mut DROID) -> BOOL {
    ppsStatsList = 0 as *mut *mut BASE_STATS; //(BASE_STATS **)ppResearchList;
    objSelectFunc =
        Some(selectCommand as
                 unsafe extern "C" fn(_: *mut BASE_OBJECT) -> BOOL);
    objGetStatsFunc =
        Some(getCommandStats as
                 unsafe extern "C" fn(_: *mut BASE_OBJECT)
                     -> *mut BASE_STATS);
    objSetStatsFunc =
        Some(setCommandStats as
                 unsafe extern "C" fn(_: *mut BASE_OBJECT, _: *mut BASE_STATS)
                     -> BOOL);
    /* Set the sub mode */
    objMode = IOBJ_COMMAND;
    /* Create the object screen with the required data */
	//return intAddObject((BASE_OBJECT *)apsStructLists[selectedPlayer],
    return intAddObject(apsDroidLists[selectedPlayer as usize] as
                            *mut BASE_OBJECT, psSelected as *mut BASE_OBJECT,
                        1 as libc::c_int);
}
/*Deals with the RMB click for the stats screen */
/*Deals with the RMB click for the stats screen */
unsafe extern "C" fn intStatsRMBPressed(mut id: UDWORD) {
    let mut psStats: *mut DROID_TEMPLATE = 0 as *mut DROID_TEMPLATE;
    let mut psNext: *mut DROID_TEMPLATE = 0 as *mut DROID_TEMPLATE;
    if id.wrapping_sub(4100 as libc::c_int as libc::c_uint) <
           numStatsListEntries {
    } else {
        debug(LOG_ERROR,
              b"intStatsRMBPressed: Invalid structure stats id\x00" as
                  *const u8 as *const libc::c_char);
    };
    if id.wrapping_sub(4100 as libc::c_int as libc::c_uint) <
           numStatsListEntries {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"hci.c\x00" as *const u8 as *const libc::c_char,
              6864 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 19],
                                        &[libc::c_char; 19]>(b"intStatsRMBPressed\x00")).as_ptr(),
              b"id - IDSTAT_START < numStatsListEntries\x00" as *const u8 as
                  *const libc::c_char);
    };
    if objMode as libc::c_uint ==
           IOBJ_MANUFACTURE as libc::c_int as libc::c_uint {
        psStats =
            *ppsStatsList.offset(id.wrapping_sub(4100 as libc::c_int as
                                                     libc::c_uint) as isize)
                as *mut DROID_TEMPLATE;
        //this now causes the production run to be decreased by one
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"intStatsRMBPressed: Invalid structure pointer\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"hci.c\x00" as *const u8 as *const libc::c_char,
                  6874 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 19],
                                            &[libc::c_char; 19]>(b"intStatsRMBPressed\x00")).as_ptr(),
                  b"PTRVALID(psObjSelected, sizeof(STRUCTURE))\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"intStatsRMBPressed: Invalid template pointer\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"hci.c\x00" as *const u8 as *const libc::c_char,
                  6876 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 19],
                                            &[libc::c_char; 19]>(b"intStatsRMBPressed\x00")).as_ptr(),
                  b"PTRVALID(psStats, sizeof(DROID_TEMPLATE))\x00" as
                      *const u8 as *const libc::c_char);
        };
        if productionPlayer as libc::c_int ==
               selectedPlayer as SBYTE as libc::c_int {
            let mut psFactory: *mut FACTORY =
                (*(psObjSelected as *mut STRUCTURE)).pFunctionality as
                    *mut FACTORY;
            //decrease the production
            factoryProdAdjust(psObjSelected as *mut STRUCTURE, psStats,
                              0 as libc::c_int);
            //need to check if this was the template that was mid-production
            if psStats == (*psFactory).psSubject as *mut DROID_TEMPLATE {
                //if have decreased to zero then cancel the production
                if getProductionQuantity(psObjSelected as *mut STRUCTURE,
                                         psStats) ==
                       0 as libc::c_int as libc::c_uint {
                    //init the factory production
                    (*psFactory).psSubject = 0 as *mut BASE_STATS;
                    //check to see if anything left to produce
                    psNext =
                        factoryProdUpdate(psObjSelected as *mut STRUCTURE,
                                          0 as *mut DROID_TEMPLATE);
                    if psNext.is_null() {
                        intManufactureFinished(psObjSelected as
                                                   *mut STRUCTURE);
                    } else if objSetStatsFunc.expect("non-null function pointer")(psObjSelected,
                                                                                  psNext
                                                                                      as
                                                                                      *mut BASE_STATS)
                                  == 0 {
                        intSetStats(objStatID, 0 as *mut BASE_STATS);
                    } else {
                        // Reset the button on the object form
                        intSetStats(objStatID, psStats as *mut BASE_STATS);
                    }
                }
            } else if (*psFactory).psSubject.is_null() {
                if objSetStatsFunc.expect("non-null function pointer")(psObjSelected,
                                                                       psStats
                                                                           as
                                                                           *mut BASE_STATS)
                       == 0 {
                    intSetStats(objStatID, 0 as *mut BASE_STATS);
                } else {
                    //if factory wasn't currently on line then set the object button
                    // Reset the button on the object form
                    intSetStats(objStatID, psStats as *mut BASE_STATS);
                }
            }
        }
    };
}
/*Deals with the RMB click for the object screen */
/*Deals with the RMB click for the Object screen */
unsafe extern "C" fn intObjectRMBPressed(mut id: UDWORD) {
    let mut psObj: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut psStructure: *mut STRUCTURE = 0 as *mut STRUCTURE;
    if (id as SDWORD - 3002 as libc::c_int) < numObjects {
    } else {
        debug(LOG_ERROR,
              b"intObjectRMBPressed: Invalid object id\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (id as SDWORD - 3002 as libc::c_int) < numObjects {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"hci.c\x00" as *const u8 as *const libc::c_char,
              6971 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 20],
                                        &[libc::c_char; 20]>(b"intObjectRMBPressed\x00")).as_ptr(),
              b"(SDWORD)id - IDOBJ_OBJSTART < numObjects\x00" as *const u8 as
                  *const libc::c_char);
    };
    /* Find the object that the ID refers to */
    psObj = intGetObject(id);
    if !psObj.is_null() {
        //don't jump around when offworld
        if (*psObj).type_0 as libc::c_uint ==
               OBJ_STRUCTURE as libc::c_int as libc::c_uint &&
               offWorldKeepLists == 0 {
            psStructure = psObj as *mut STRUCTURE;
            if (*(*psStructure).pStructureType).type_0 ==
                   REF_FACTORY as libc::c_int as libc::c_uint ||
                   (*(*psStructure).pStructureType).type_0 ==
                       REF_CYBORG_FACTORY as libc::c_int as libc::c_uint ||
                   (*(*psStructure).pStructureType).type_0 ==
                       REF_VTOL_FACTORY as libc::c_int as libc::c_uint {
                //centre the view on the delivery point
                setViewPos(((*(*((*psStructure).pFunctionality as
                                     *mut FACTORY)).psAssemblyPoint).coords.x
                                >> 7 as libc::c_int) as UDWORD,
                           ((*(*((*psStructure).pFunctionality as
                                     *mut FACTORY)).psAssemblyPoint).coords.y
                                >> 7 as libc::c_int) as UDWORD,
                           1 as libc::c_int);
            }
        }
    };
}
/*Deals with the RMB click for the Object Stats buttons */
/*Deals with the RMB click for the Object Stats buttons */
unsafe extern "C" fn intObjStatRMBPressed(mut id: UDWORD) {
    let mut psObj: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut psStructure: *mut STRUCTURE = 0 as *mut STRUCTURE;
    if (id as SDWORD - 3100 as libc::c_int) < numObjects {
    } else {
        debug(LOG_ERROR,
              b"intObjStatRMBPressed: Invalid stat id\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (id as SDWORD - 3100 as libc::c_int) < numObjects {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"hci.c\x00" as *const u8 as *const libc::c_char,
              7002 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 21],
                                        &[libc::c_char; 21]>(b"intObjStatRMBPressed\x00")).as_ptr(),
              b"(SDWORD)id - IDOBJ_STATSTART < numObjects\x00" as *const u8 as
                  *const libc::c_char);
    };
    /* Find the object that the ID refers to */
    psObj = intGetObject(id);
    if !psObj.is_null() {
        //#ifndef PSX
        intResetWindows(psObj);
        //#else
//		intAddObjectStats(psObj, id);
//#endif
        if (*psObj).type_0 as libc::c_uint ==
               OBJ_STRUCTURE as libc::c_int as libc::c_uint {
            psStructure = psObj as *mut STRUCTURE;
            if StructIsFactory(psStructure) != 0 {
                //check if active
                if !(*((*psStructure).pFunctionality as
                           *mut FACTORY)).psSubject.is_null() {
                    //if not curently on hold, set it
                    if (*((*psStructure).pFunctionality as
                              *mut FACTORY)).timeStartHold ==
                           0 as libc::c_int as libc::c_uint {
                        holdProduction(psStructure);
                    } else {
                        //cancel if have RMB-clicked twice
                        cancelProduction(psStructure);
                        //play audio to indicate cancelled
                        audio_PlayTrack(ID_SOUND_WINDOWCLOSE as libc::c_int);
                    }
                }
            } else if (*(*psStructure).pStructureType).type_0 ==
                          REF_RESEARCH as libc::c_int as libc::c_uint {
                //check if active
                if !(*((*psStructure).pFunctionality as
                           *mut RESEARCH_FACILITY)).psSubject.is_null() {
                    //if not curently on hold, set it
                    if (*((*psStructure).pFunctionality as
                              *mut RESEARCH_FACILITY)).timeStartHold ==
                           0 as libc::c_int as libc::c_uint {
                        holdResearch(psStructure);
                    } else {
                        //cancel if have RMB-clicked twice
                        cancelResearch(psStructure);
                        //play audio to indicate cancelled
                        audio_PlayTrack(ID_SOUND_WINDOWCLOSE as libc::c_int);
                    }
                }
            }
        }
    };
}
//sets up the Intelligence Screen as far as the interface is concerned
//extern void addIntelScreen(BOOL playImmediate);
//sets up the Intelligence Screen as far as the interface is concerned
//void addIntelScreen(BOOL playImmediate)
#[no_mangle]
pub unsafe extern "C" fn addIntelScreen() {
    let mut radOnScreen: BOOL = 0;
    if driveModeActive() != 0 && driveInterfaceEnabled() == 0 {
        driveDisableControl();
        driveEnableInterface(1 as libc::c_int);
    }
    intResetScreen(0 as libc::c_int);
    /*#ifndef PSX
	if(!bMultiPlayer)
	{
#endif
		gameTimeStop();

#ifndef PSX
	}
#endif*/
    //done in intAddIntelMap()
	//setIntelligencePauseState();
    //lock the reticule button
    widgSetButtonState(psWScreen, 6 as libc::c_int as UDWORD,
                       0x4 as libc::c_int as UDWORD);
    //add the power bar - for looks!
    intShowPowerBar();
    //get the background image for the Intelligence screen
    // Only do this in main game.
    if GetGameMode() == 3 as libc::c_int as libc::c_uint && bMultiPlayer == 0
       {
        radOnScreen = radarOnScreen;
        bRender3DOnly = 1 as libc::c_int;
        radarOnScreen = 0 as libc::c_int;
        // Just display the 3d, no interface
        displayWorld();
        // Upload the current display back buffer into system memory.
        pie_UploadDisplayBuffer(DisplayBuffer);
        radarOnScreen = radOnScreen;
        bRender3DOnly = 0 as libc::c_int
    }
    //add all the intelligence screen interface
	//(void)intAddIntelMap(playImmediate);
    intAddIntelMap();
    intMode = INT_INTELMAP;
    /*if (psCurrentMsg AND psCurrentMsg->type == MSG_TUTORIAL)
	{
		//just display the message
		if (psCurrentMsg->pViewData)
		{
			intAddMessageView(psCurrentMsg->type);
			if (psCurrentMsg->pViewData->audioID != NO_AUDIO_MSG)
			{
				audio_PlayTrack(psCurrentMsg->pViewData->audioID);
			}
			intMode = INT_TUTORIAL;
		}
	}
	else
	{
		widgSetButtonState(psWScreen, IDRET_INTEL_MAP, WBUT_CLICKLOCK);
		//add the power bar - for looks!
		(void)intAddPower();
		intelMapView(TRUE);
		(void)intAddIntelMap(playImmediate);
		intMode = INT_INTELMAP;
	}*/
}
//sets up the Transporter Screen as far as the interface is concerned
//sets up the Transporter Screen as far as the interface is concerned
#[no_mangle]
pub unsafe extern "C" fn addTransporterInterface(mut psSelected: *mut DROID,
                                                 mut onMission: BOOL) {
    //if psSelected = NULL add interface but if psSelected != NULL make sure its not flying
    if psSelected.is_null() ||
           !psSelected.is_null() && transporterFlying(psSelected) == 0 {
        intResetScreen(0 as libc::c_int);
        intAddTransporter(psSelected, onMission);
        intMode = INT_TRANSPORTER
    };
}
/*sets which list of structures to use for the interface*/
/*sets which list of structures to use for the interface*/
#[no_mangle]
pub unsafe extern "C" fn interfaceStructList() -> *mut STRUCTURE {
    if offWorldKeepLists != 0 {
        return mission.apsStructLists[selectedPlayer as usize]
    } else { return apsStructLists[selectedPlayer as usize] };
}
/*causes a reticule button to start flashing*/
/*causes a reticule button to start flashing*/
#[no_mangle]
pub unsafe extern "C" fn flashReticuleButton(mut buttonID: UDWORD) {
    let mut psButton: *mut W_TABFORM = 0 as *mut W_TABFORM;
    let mut flash: UDWORD = 0;
    //get the button for the id
    psButton = widgGetFromID(psWScreen, buttonID) as *mut W_TABFORM;
    if !psButton.is_null() {
        //set flashing byte to true
        flash =
            ((1 as libc::c_int as UBYTE as libc::c_int & 0xff as libc::c_int)
                 << 24 as libc::c_int) as UDWORD;
        (*psButton).pUserData =
            (flash | (*psButton).pUserData as UDWORD) as *mut libc::c_void
    };
}
// stop a reticule button flashing
// stop a reticule button flashing
#[no_mangle]
pub unsafe extern "C" fn stopReticuleButtonFlash(mut buttonID: UDWORD) {
    let mut psButton: *mut WIDGET = 0 as *mut WIDGET;
    let mut DownTime: UBYTE = 0;
    let mut Index: UBYTE = 0;
    let mut flashing: UBYTE = 0;
    let mut flashTime: UBYTE = 0;
    psButton = widgGetFromID(psWScreen, buttonID);
    if !psButton.is_null() {
        // clear flashing byte
        DownTime =
            ((*psButton).pUserData as UDWORD >> 8 as libc::c_int &
                 0xff as libc::c_int as libc::c_uint) as UBYTE;
        Index =
            ((*psButton).pUserData as UDWORD &
                 0xff as libc::c_int as libc::c_uint) as UBYTE;
        flashing =
            ((*psButton).pUserData as UDWORD >> 24 as libc::c_int &
                 0xff as libc::c_int as libc::c_uint) as UBYTE;
        flashTime =
            ((*psButton).pUserData as UDWORD >> 16 as libc::c_int &
                 0xff as libc::c_int as libc::c_uint) as UBYTE;
        flashing = 0 as libc::c_int as UBYTE;
        flashTime = 0 as libc::c_int as UBYTE;
        (*psButton).pUserData =
            ((flashTime as libc::c_int & 0xff as libc::c_int) <<
                 24 as libc::c_int |
                 (flashing as libc::c_int & 0xff as libc::c_int) <<
                     16 as libc::c_int |
                 (DownTime as libc::c_int & 0xff as libc::c_int) <<
                     8 as libc::c_int |
                 Index as libc::c_int & 0xff as libc::c_int) as
                *mut libc::c_void
        //psButton->pUserData = (void *)(((UDWORD)psButton->pUserData) & 0x00ffffff);
    };
}
//displays the Power Bar
//displays the Power Bar
#[no_mangle]
pub unsafe extern "C" fn intShowPowerBar() {
    //if its not already on display
    if !widgGetFromID(psWScreen, 102 as libc::c_int as UDWORD).is_null() {
        widgReveal(psWScreen, 102 as libc::c_int as UDWORD);
    };
}
//hides the power bar from the display - NB static function now
//hides the power bar from the display
unsafe extern "C" fn intHidePowerBar() {
    //only hides the power bar if the player has requested no power bar
    if powerBarUp == 0 {
        if !widgGetFromID(psWScreen, 102 as libc::c_int as UDWORD).is_null() {
            widgHide(psWScreen, 102 as libc::c_int as UDWORD);
        }
    };
}
//hides the power bar from the display
//extern void intHidePowerBar(void);
//hides the power bar from the display - regardless of what player requested!
//hides the power bar from the display - regardless of what player requested!
#[no_mangle]
pub unsafe extern "C" fn forceHidePowerBar() {
    if !widgGetFromID(psWScreen, 102 as libc::c_int as UDWORD).is_null() {
        widgHide(psWScreen, 102 as libc::c_int as UDWORD);
    };
}
/* Add the Proximity message buttons */
unsafe extern "C" fn _intAddProximityButton(mut psProxDisp:
                                                *mut PROXIMITY_DISPLAY,
                                            mut inc: UDWORD) -> BOOL {
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
    let mut psProxDisp2: *mut PROXIMITY_DISPLAY = 0 as *mut PROXIMITY_DISPLAY;
    let mut cnt: UDWORD = 0;
    //#ifdef PSX
//	WidgSetOTIndex(OT2D_FARFORE);
//#endif
    memset(&mut sBFormInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_FORMINIT>() as libc::c_ulong);
    sBFormInit.formID = 0 as libc::c_int as UDWORD;
    sBFormInit.id = (12000 as libc::c_int as libc::c_uint).wrapping_add(inc);
    //store the ID so we can detect which one has been clicked on
    (*psProxDisp).buttonID = sBFormInit.id;
    //	loop back and find a free one!
//	ASSERT( sBFormInit.id < IDPROX_END,"Too many proximity message buttons" );
    if sBFormInit.id >= 12019 as libc::c_int as libc::c_uint {
        cnt = 12000 as libc::c_int as UDWORD;
        while cnt < 12019 as libc::c_int as libc::c_uint {
            // go down the prox msgs and see if it's free.
            psProxDisp2 = apsProxDisp[selectedPlayer as usize];
            while !psProxDisp2.is_null() && (*psProxDisp2).buttonID != cnt {
                psProxDisp2 = (*psProxDisp2).psNext
            }
            if psProxDisp.is_null() {
                // value was unused.
                sBFormInit.id = cnt;
                break ;
            } else { cnt = cnt.wrapping_add(1) }
        }
        if cnt == 12019 as libc::c_int as libc::c_uint {
            return 0 as libc::c_int
            // no slot was found.
        }
    }
    sBFormInit.majorID = 0 as libc::c_int as UWORD;
    sBFormInit.minorID = 0 as libc::c_int as UWORD;
    sBFormInit.style = 4 as libc::c_int as UDWORD;
    //sBFormInit.width = iV_GetImageWidth(IntImages,IMAGE_GAM_ENMREAD);
	//sBFormInit.height = iV_GetImageHeight(IntImages,IMAGE_GAM_ENMREAD);
    sBFormInit.width = 9 as libc::c_int as UWORD;
    sBFormInit.height = 9 as libc::c_int as UWORD;
    //the x and y need to be set up each time the button is drawn - see intDisplayProximityBlips
    sBFormInit.pDisplay =
        Some(intDisplayProximityBlips as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    //set the data for this button
    sBFormInit.pUserData = psProxDisp as *mut libc::c_void;
    if widgAddForm(psWScreen, &mut sBFormInit) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
/* Add the Proximity message buttons */
/* Add the Proximity message buttons */
#[no_mangle]
pub unsafe extern "C" fn intAddProximityButton(mut psProxDisp:
                                                   *mut PROXIMITY_DISPLAY,
                                               mut inc: UDWORD) -> BOOL {
    return _intAddProximityButton(psProxDisp, inc);
}
/*Remove a Proximity Button - when the message is deleted*/
/*Remove a Proximity Button - when the message is deleted*/
#[no_mangle]
pub unsafe extern "C" fn intRemoveProximityButton(mut psProxDisp:
                                                      *mut PROXIMITY_DISPLAY) {
    widgDelete(psWScreen, (*psProxDisp).buttonID);
}
//proximity display stuff
/*deals with the proximity message when clicked on*/
unsafe extern "C" fn processProximityButtons(mut id: UDWORD) {
    let mut psProxDisp: *mut PROXIMITY_DISPLAY = 0 as *mut PROXIMITY_DISPLAY;
    if doWeDrawProximitys() == 0 { return }
    //find which proximity display this relates to
    psProxDisp = 0 as *mut PROXIMITY_DISPLAY;
    psProxDisp = apsProxDisp[selectedPlayer as usize];
    while !psProxDisp.is_null() {
        if (*psProxDisp).buttonID == id { break ; }
        psProxDisp = (*psProxDisp).psNext
    }
    if !psProxDisp.is_null() {
        //if not been read - display info
        if (*(*psProxDisp).psMessage).read == 0 {
            displayProximityMessage(psProxDisp);
        }
    };
}
/* Allows us to fool the widgets with a keypress */
/*	Fools the widgets by setting a key value */
#[no_mangle]
pub unsafe extern "C" fn setKeyButtonMapping(mut val: UDWORD) {
    keyButtonMapping = val;
}
/*Looks through the players list of structures to see if there is one selected
of the required type. If there is more than one, they are all deselected and
the first one reselected*/
unsafe extern "C" fn intCheckForStructure(mut structType: UDWORD)
 -> *mut STRUCTURE {
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psSel: *mut STRUCTURE = 0 as *mut STRUCTURE;
    //	for (psStruct = apsStructLists[player]; psStruct != NULL; psStruct =
    psStruct = interfaceStructList();
    while !psStruct.is_null() {
        if (*psStruct).selected as libc::c_int != 0 &&
               (*(*psStruct).pStructureType).type_0 == structType &&
               (*psStruct).status as libc::c_int == SS_BUILT as libc::c_int {
            if !psSel.is_null() {
                clearSelection();
                (*psSel).selected = 1 as libc::c_int as UBYTE;
                break ;
            } else { psSel = psStruct }
        }
        psStruct = (*psStruct).psNext
    }
    return psSel;
}
/*Looks through the players list of droids to see if there is one selected
of the required type. If there is more than one, they are all deselected and
the first one reselected*/
// no longer do this for constructor droids - (gleeful its-near-the-end-of-the-project hack - JOHN)
unsafe extern "C" fn intCheckForDroid(mut droidType_0: UDWORD) -> *mut DROID {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut psSel: *mut DROID = 0 as *mut DROID;
    //	clearSelection();
    psDroid = apsDroidLists[selectedPlayer as usize];
    while !psDroid.is_null() {
        if (*psDroid).selected as libc::c_int != 0 &&
               (*psDroid).droidType as libc::c_uint ==
                   droidType_0 as SDWORD as libc::c_uint {
            if !psSel.is_null() {
                /* Was...
				clearSelection();
				SelectDroid(psSel);
*/
                if droidType_0 !=
                       DROID_CONSTRUCT as libc::c_int as libc::c_uint &&
                       droidType_0 !=
                           DROID_CYBORG_CONSTRUCT as libc::c_int as
                               libc::c_uint {
                    clearSelection();
                }
                SelectDroid(psSel);
                break ;
            } else { psSel = psDroid }
        }
        psDroid = (*psDroid).psNext
    }
    return psSel;
}
// count the number of selected droids of a type
// count the number of selected droids of a type
#[no_mangle]
pub unsafe extern "C" fn intNumSelectedDroids(mut droidType_0: UDWORD)
 -> SDWORD {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut num: SDWORD = 0;
    num = 0 as libc::c_int;
    psDroid = apsDroidLists[selectedPlayer as usize];
    while !psDroid.is_null() {
        if (*psDroid).selected as libc::c_int != 0 &&
               (*psDroid).droidType as libc::c_uint ==
                   droidType_0 as SDWORD as libc::c_uint {
            num += 1 as libc::c_int
        }
        psDroid = (*psDroid).psNext
    }
    return num;
}
#[no_mangle]
pub unsafe extern "C" fn intShowReticuleButton(mut id: UDWORD,
                                               mut Show: BOOL) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 7 as libc::c_int {
        if ReticuleEnabled[i as usize].id == id {
            ReticuleEnabled[i as usize].Hidden = (Show == 0) as libc::c_int;
            break ;
        } else { i += 1 }
    };
}
unsafe extern "C" fn intInitialiseReticule() {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 7 as libc::c_int {
        ReticuleEnabled[i as usize].Hidden = 0 as libc::c_int;
        i += 1
    };
}
// Check that each reticule button has the structure or droid required for it and
// enable/disable accordingly.
//
unsafe extern "C" fn intCheckReticuleButtons() {
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut i: libc::c_int = 0;
    //#ifndef PSX
    ReticuleEnabled[RETBUT_CANCEL as libc::c_int as usize].Enabled =
        1 as libc::c_int;
    //#endif
    ReticuleEnabled[RETBUT_FACTORY as libc::c_int as usize].Enabled =
        0 as libc::c_int;
    ReticuleEnabled[RETBUT_RESEARCH as libc::c_int as usize].Enabled =
        0 as libc::c_int;
    ReticuleEnabled[RETBUT_BUILD as libc::c_int as usize].Enabled =
        0 as libc::c_int;
    ReticuleEnabled[RETBUT_DESIGN as libc::c_int as usize].Enabled =
        0 as libc::c_int;
    ReticuleEnabled[RETBUT_INTELMAP as libc::c_int as usize].Enabled =
        1 as libc::c_int;
    ReticuleEnabled[RETBUT_COMMAND as libc::c_int as usize].Enabled =
        0 as libc::c_int;
    psStruct = interfaceStructList();
    while !psStruct.is_null() {
        if (*psStruct).status as libc::c_int == SS_BUILT as libc::c_int {
            match (*(*psStruct).pStructureType).type_0 {
                10 => {
                    if missionLimboExpand() == 0 {
                        ReticuleEnabled[RETBUT_RESEARCH as libc::c_int as
                                            usize].Enabled = 1 as libc::c_int
                    }
                }
                1 | 16 | 17 => {
                    if missionLimboExpand() == 0 {
                        ReticuleEnabled[RETBUT_FACTORY as libc::c_int as
                                            usize].Enabled = 1 as libc::c_int
                    }
                }
                0 => {
                    ReticuleEnabled[RETBUT_DESIGN as libc::c_int as
                                        usize].Enabled = 1 as libc::c_int
                }
                _ => { }
            }
        }
        psStruct = (*psStruct).psNext
    }
    psDroid = apsDroidLists[selectedPlayer as usize];
    while !psDroid.is_null() {
        match (*psDroid).droidType as libc::c_uint {
            3 | 10 => {
                ReticuleEnabled[RETBUT_BUILD as libc::c_int as usize].Enabled
                    = 1 as libc::c_int
            }
            7 => {
                ReticuleEnabled[RETBUT_COMMAND as libc::c_int as
                                    usize].Enabled = 1 as libc::c_int
            }
            _ => { }
        }
        psDroid = (*psDroid).psNext
    }
    i = 0 as libc::c_int;
    while i < 7 as libc::c_int {
        let mut psWidget: *mut WIDGET =
            widgGetFromID(psWScreen, ReticuleEnabled[i as usize].id);
        if !psWidget.is_null() {
            if (*psWidget).type_0 as libc::c_uint !=
                   WIDG_LABEL as libc::c_int as libc::c_uint {
                if ReticuleEnabled[i as usize].Enabled != 0 {
                    widgSetButtonState(psWScreen,
                                       ReticuleEnabled[i as usize].id,
                                       0 as libc::c_int as UDWORD);
                } else {
                    widgSetButtonState(psWScreen,
                                       ReticuleEnabled[i as usize].id,
                                       0x1 as libc::c_int as UDWORD);
                }
                if ReticuleEnabled[i as usize].Hidden != 0 {
                    widgHide(psWScreen, ReticuleEnabled[i as usize].id);
                } else {
                    widgReveal(psWScreen, ReticuleEnabled[i as usize].id);
                }
            }
        }
        i += 1
    };
}
/*Checks to see if there are any research topics to do and flashes the button*/
/*Checks to see if there are any research topics to do and flashes the button -
only if research facility is free*/
#[no_mangle]
pub unsafe extern "C" fn intCheckResearchButton() {
    let mut index: UWORD = 0;
    let mut count: UWORD = 0;
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut resFree: BOOL = 0 as libc::c_int;
    psStruct = interfaceStructList();
    while !psStruct.is_null() {
        if (*(*psStruct).pStructureType).type_0 ==
               REF_RESEARCH as libc::c_int as libc::c_uint &&
               (*psStruct).status as libc::c_int == SS_BUILT as libc::c_int &&
               (*((*psStruct).pFunctionality as
                      *mut RESEARCH_FACILITY)).psSubject.is_null() {
            resFree = 1 as libc::c_int;
            break ;
        } else { psStruct = (*psStruct).psNext }
    }
    if resFree != 0 {
        //set to value that won't be reached in fillResearchList
        //needs to be UWORD sized for the Patches
        index =
            numResearch.wrapping_add(1 as libc::c_int as libc::c_uint) as
                UWORD;
        //index = (UBYTE)(numResearch + 1);
		//calculate the list
        count =
            fillResearchList(pList, selectedPlayer, index,
                             80 as libc::c_int as UWORD);
        if count != 0 {
            //set the research reticule button to flash
            flashReticuleButton(5 as libc::c_int as UDWORD);
        }
    };
}
// see if a reticule button is enabled
// see if a reticule button is enabled
#[no_mangle]
pub unsafe extern "C" fn intCheckReticuleButEnabled(mut id: UDWORD) -> BOOL {
    let mut i: SDWORD = 0;
    i = 0 as libc::c_int;
    while i < 7 as libc::c_int {
        if ReticuleEnabled[i as usize].id == id {
            return ReticuleEnabled[i as usize].Enabled
        }
        i += 1
    }
    return 0 as libc::c_int;
}
//#ifdef PSX
#[no_mangle]
pub unsafe extern "C" fn InterfaceIsUp(mut Type: UWORD) -> BOOL {
    return ((intMode as libc::c_uint ==
                 INT_OBJECT as libc::c_int as libc::c_uint ||
                 intMode as libc::c_uint ==
                     INT_STAT as libc::c_int as libc::c_uint) &&
                objMode as libc::c_uint == Type as libc::c_uint) as
               libc::c_int;
}
// Do any maintenance on the interface that's needed when a structure is destroyed.
//
#[no_mangle]
pub unsafe extern "C" fn intDestroyStructure(mut psStruct: *mut STRUCTURE) {
    if psStruct == CurrentStruct { intInitStructureCycle(); };
}
// Do any maintenance on the interface that's needed when a droid is destroyed.
//
#[no_mangle]
pub unsafe extern "C" fn intDestroyDroid(mut psDroid: *mut DROID) {
    if psDroid == CurrentDroid { intInitDroidCycle(); };
}
#[no_mangle]
pub unsafe extern "C" fn intInitObjectCycle() {
    intInitStructureCycle();
    intInitDroidCycle();
}
unsafe extern "C" fn intInitStructureCycle() {
    CurrentStruct = 0 as *mut STRUCTURE;
    CurrentStructType = 0 as libc::c_int as SWORD;
}
unsafe extern "C" fn intInitDroidCycle() {
    CurrentDroid = 0 as *mut DROID;
    CurrentDroidType = 0 as libc::c_int as SWORD;
}
// Begin drive mode.
//
// Selects a construction droid and activates the droid cam and driving mode.
// If it can't find a construction droid then it tries for a weapon droid.
//
//void BeginDriveMode(void)
//{
//	DROID *psDroid;
//
//	if( (psDroid = intGotoNextDroidType(DROID_CONSTRUCT)) == NULL) {
//		psDroid = intGotoNextDroidType(DROID_WEAPON);
//	}
//
//	camToggleStatus();
//}
// Find any structure. Returns NULL if none found.
//
#[no_mangle]
pub unsafe extern "C" fn intFindAStructure() -> *mut STRUCTURE {
    let mut Struct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    // First try and find a factory.
    Struct =
        intGotoNextStructureType(REF_FACTORY as libc::c_int as UDWORD,
                                 0 as libc::c_int, 0 as libc::c_int);
    if Struct.is_null() {
        // If that fails then look for a command center.
        Struct =
            intGotoNextStructureType(REF_HQ as libc::c_int as UDWORD,
                                     0 as libc::c_int, 0 as libc::c_int);
        if Struct.is_null() {
            // If that fails then look for a any structure.
            Struct =
                intGotoNextStructureType(255 as libc::c_int as UDWORD,
                                         0 as libc::c_int, 0 as libc::c_int)
        }
    }
    return Struct;
}
// Look through the players structures and find the next one of type structType.
//
#[no_mangle]
pub unsafe extern "C" fn intGotoNextStructureType(mut structType: UDWORD,
                                                  mut JumpTo: BOOL,
                                                  mut CancelDrive: BOOL)
 -> *mut STRUCTURE {
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut Found: BOOL = 0 as libc::c_int;
    if structType as SWORD as libc::c_int != CurrentStructType as libc::c_int
       {
        CurrentStruct = 0 as *mut STRUCTURE;
        CurrentStructType = structType as SWORD
    }
    if !CurrentStruct.is_null() {
        psStruct = CurrentStruct
    } else { psStruct = interfaceStructList() }
    while !psStruct.is_null() {
        if ((*(*psStruct).pStructureType).type_0 == structType ||
                structType == 255 as libc::c_int as libc::c_uint) &&
               (*psStruct).status as libc::c_int == SS_BUILT as libc::c_int {
            if psStruct != CurrentStruct {
                if CancelDrive != 0 { clearSelection(); } else { clearSel(); }
                (*psStruct).selected = 1 as libc::c_int as UBYTE;
                CurrentStruct = psStruct;
                Found = 1 as libc::c_int;
                break ;
            }
        }
        psStruct = (*psStruct).psNext
    }
    // Start back at the begining?
    if Found == 0 && !CurrentStruct.is_null() {
        psStruct = interfaceStructList();
        while psStruct != CurrentStruct && !psStruct.is_null() {
            if ((*(*psStruct).pStructureType).type_0 == structType ||
                    structType == 255 as libc::c_int as libc::c_uint) &&
                   (*psStruct).status as libc::c_int ==
                       SS_BUILT as libc::c_int {
                if psStruct != CurrentStruct {
                    if CancelDrive != 0 {
                        clearSelection();
                    } else { clearSel(); }
                    (*psStruct).selected = 1 as libc::c_int as UBYTE;
                    CurrentStruct = psStruct;
                    Found = 1 as libc::c_int;
                    break ;
                }
            }
            psStruct = (*psStruct).psNext
        }
    }
    // Center it on screen.
    if !CurrentStruct.is_null() && JumpTo != 0 {
        intSetMapPos((*CurrentStruct).x as UDWORD,
                     (*CurrentStruct).y as UDWORD);
    }
    return CurrentStruct;
}
#[no_mangle]
pub unsafe extern "C" fn GetWeaponMajorClass(mut psWeapStats:
                                                 *mut WEAPON_STATS)
 -> UDWORD {
    match (*psWeapStats).weaponSubClass as libc::c_uint {
        7 => { return WMC_FLAME as libc::c_int as UDWORD }
        2 | 8 => { return WMC_SHELL as libc::c_int as UDWORD }
        3 | 4 | 11 | 12 => { return WMC_MISSILE as libc::c_int as UDWORD }
        0 | 1 | 5 | 6 => { return WMC_MUZZLE as libc::c_int as UDWORD }
        9 => { return WMC_ELECTRONIC as libc::c_int as UDWORD }
        10 => { return WMC_AA as libc::c_int as UDWORD }
        13 => { return WMC_LAS_SAT as libc::c_int as UDWORD }
        14 => { return WMC_BOMB as libc::c_int as UDWORD }
        15 => { return WMC_COMMAND as libc::c_int as UDWORD }
        16 => { return WMC_EMP as libc::c_int as UDWORD }
        _ => { }
    }
    if 0 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Unknown weapon class\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 0 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"hci.c\x00" as *const u8 as *const libc::c_char,
              7821 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 20],
                                        &[libc::c_char; 20]>(b"GetWeaponMajorClass\x00")).as_ptr(),
              b"FALSE\x00" as *const u8 as *const libc::c_char);
    };
    return 0 as libc::c_int as UDWORD;
}
// Used for selecting similar types of units.
//
#[no_mangle]
pub unsafe extern "C" fn DroidTypesMatch(mut psDroid1: *mut DROID,
                                         mut psDroid2: *mut DROID) -> BOOL {
    let mut Type1: UDWORD = (*psDroid1).droidType as UDWORD;
    let mut Type2: UDWORD = (*psDroid2).droidType as UDWORD;
    let mut IsLift1: BOOL =
        ((*asPropulsionStats.offset((*psDroid1).asBits[COMP_PROPULSION as
                                                           libc::c_int as
                                                           usize].nStat as
                                        libc::c_int as isize)).propulsionType
             as libc::c_int == LIFT as libc::c_int) as libc::c_int;
    let mut IsLift2: BOOL =
        ((*asPropulsionStats.offset((*psDroid2).asBits[COMP_PROPULSION as
                                                           libc::c_int as
                                                           usize].nStat as
                                        libc::c_int as isize)).propulsionType
             as libc::c_int == LIFT as libc::c_int) as libc::c_int;
    // Don't match ground units with flying units.
    if IsLift1 != IsLift2 { return 0 as libc::c_int }
    // Allow matching cyborgs with tanks.
    if Type1 == DROID_WEAPON as libc::c_int as libc::c_uint ||
           Type1 == DROID_CYBORG as libc::c_int as libc::c_uint ||
           Type1 == DROID_CYBORG_SUPER as libc::c_int as libc::c_uint {
        if Type2 == DROID_WEAPON as libc::c_int as libc::c_uint ||
               Type2 == DROID_CYBORG as libc::c_int as libc::c_uint ||
               Type2 == DROID_CYBORG_SUPER as libc::c_int as libc::c_uint {
            return 1 as libc::c_int
        }
    }
    if Type1 == Type2 { return 1 as libc::c_int }
    return 0 as libc::c_int;
}
// Select all droids of the same type with the same cluster id as
// the specified droid, deselects all others.
//
#[no_mangle]
pub unsafe extern "C" fn intSelectDroidsInDroidCluster(mut psCurDroid:
                                                           *mut DROID)
 -> BOOL {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut NumSelected: UWORD = 0 as libc::c_int as UWORD;
    let mut NearDroids: [DROIDDIST; 10] =
        [DROIDDIST{psDroid: 0 as *mut DROID, Dist: 0,}; 10];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut psWeapStats: *mut WEAPON_STATS =
        asWeaponStats.offset((*psCurDroid).asWeaps[0 as libc::c_int as
                                                       usize].nStat as isize);
    let mut WeapClass: UDWORD = GetWeaponMajorClass(psWeapStats);
    // Can't select a transporter.
    if (*psCurDroid).droidType as libc::c_uint ==
           DROID_TRANSPORTER as libc::c_int as libc::c_uint {
        return 1 as libc::c_int
    } //,k;
    psDroid = apsDroidLists[selectedPlayer as usize];
    while !psDroid.is_null() {
        if (*psDroid).cluster as libc::c_int ==
               (*psCurDroid).cluster as libc::c_int &&
               DroidTypesMatch(psDroid, psCurDroid) != 0 {
            let mut dx: UDWORD =
                abs((*psCurDroid).x as libc::c_int -
                        (*psDroid).x as libc::c_int) as UDWORD;
            let mut dy: UDWORD =
                abs((*psCurDroid).y as libc::c_int -
                        (*psDroid).y as libc::c_int) as UDWORD;
            let mut dist: UDWORD =
                dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy));
            let mut Index: libc::c_int = 0;
            let mut psWStats: *mut WEAPON_STATS =
                asWeaponStats.offset((*psDroid).asWeaps[0 as libc::c_int as
                                                            usize].nStat as
                                         isize);
            if !(GetWeaponMajorClass(psWStats) != WeapClass) {
                DeSelectDroid(psDroid);
                Index = -(1 as libc::c_int);
                // Maintain a sorted list of the MAX_GROUP_SIZE nearest droids.
                i = 0 as libc::c_int;
                while i < NumSelected as libc::c_int {
                    if dist < NearDroids[i as usize].Dist {
                        // Nearer?
                        // Move the rest down.
                        j = 10 as libc::c_int - 2 as libc::c_int;
                        while j >= i {
                            NearDroids[(j + 1 as libc::c_int) as usize] =
                                NearDroids[j as usize];
                            j -= 1
                        }
                        Index = i;
                        break ;
                    } else { i += 1 }
                }
                if Index >= 0 as libc::c_int {
                    NearDroids[Index as usize].Dist = dist;
                    NearDroids[Index as usize].psDroid = psDroid;
                    if (NumSelected as libc::c_int) < 10 as libc::c_int {
                        NumSelected = NumSelected.wrapping_add(1)
                    }
                } else if i < 10 as libc::c_int {
                    NearDroids[i as usize].Dist = dist;
                    NearDroids[i as usize].psDroid = psDroid;
                    NumSelected = NumSelected.wrapping_add(1)
                }
            }
        }
        psDroid = (*psDroid).psNext
    }
    i = 0 as libc::c_int;
    while i < NumSelected as libc::c_int {
        SelectDroid(NearDroids[i as usize].psDroid);
        i += 1
    }
    //	intRefreshScreen();
    debug(LOG_NEVER,
          b"Selected %d droids\n\x00" as *const u8 as *const libc::c_char,
          NumSelected as libc::c_int);
    return 1 as libc::c_int;
}
// Look through the players droids and find the next one of type droidType.
// If Current=NULL then start at the beginning overwise start at Current.
//
#[no_mangle]
pub unsafe extern "C" fn intGotoNextDroidType(mut CurrDroid: *mut DROID,
                                              mut droidType_0: UDWORD,
                                              mut AllowGroup: BOOL)
 -> *mut DROID {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut Found: BOOL = 0 as libc::c_int;
    if !CurrDroid.is_null() { CurrentDroid = CurrDroid }
    if droidType_0 as SWORD as libc::c_int != CurrentDroidType as libc::c_int
           && droidType_0 != DROID_ANY as libc::c_int as libc::c_uint {
        CurrentDroid = 0 as *mut DROID;
        CurrentDroidType = droidType_0 as SWORD
    }
    if !CurrentDroid.is_null() {
        psDroid = CurrentDroid
    } else { psDroid = apsDroidLists[selectedPlayer as usize] }
    while !psDroid.is_null() {
        if ((*psDroid).droidType as UDWORD == droidType_0 ||
                droidType_0 == DROID_ANY as libc::c_int as libc::c_uint &&
                    (*psDroid).droidType as libc::c_uint !=
                        DROID_TRANSPORTER as libc::c_int as libc::c_uint) &&
               ((*psDroid).group as libc::c_int == 0xff as libc::c_int ||
                    AllowGroup != 0) {
            if psDroid != CurrentDroid {
                clearSel();
                SelectDroid(psDroid);
                //				psDroid->selected = TRUE;
                CurrentDroid = psDroid;
                Found = 1 as libc::c_int;
                break ;
            }
        }
        psDroid = (*psDroid).psNext
    }
    // Start back at the begining?
    if Found == 0 && !CurrentDroid.is_null() {
        psDroid = apsDroidLists[selectedPlayer as usize];
        while psDroid != CurrentDroid && !psDroid.is_null() {
            if ((*psDroid).droidType as UDWORD == droidType_0 ||
                    droidType_0 == DROID_ANY as libc::c_int as libc::c_uint &&
                        (*psDroid).droidType as libc::c_uint !=
                            DROID_TRANSPORTER as libc::c_int as libc::c_uint)
                   &&
                   ((*psDroid).group as libc::c_int == 0xff as libc::c_int ||
                        AllowGroup != 0) {
                if psDroid != CurrentDroid {
                    clearSel();
                    SelectDroid(psDroid);
                    //					psDroid->selected = TRUE;
                    CurrentDroid = psDroid;
                    Found = 1 as libc::c_int;
                    break ;
                }
            }
            psDroid = (*psDroid).psNext
        }
    }
    if Found == 1 as libc::c_int {
        if bInTutorial != 0 {
            psCBSelectedDroid = CurrentDroid;
            eventFireCallbackTrigger(CALL_DROID_SELECTED as libc::c_int as
                                         TRIGGER_TYPE);
            psCBSelectedDroid = 0 as *mut DROID
        }
        // Center it on screen.
        if !CurrentDroid.is_null() {
            intSetMapPos((*CurrentDroid).x as UDWORD,
                         (*CurrentDroid).y as UDWORD);
        }
        return CurrentDroid
    }
    return 0 as *mut DROID;
}
/*order the objects in the bottom bar according to their type*/
unsafe extern "C" fn orderObjectInterface() {
    if apsObjectList.is_null() {
        //no objects so nothing to order!
        return
    }
    match (**apsObjectList.offset(0 as libc::c_int as isize)).type_0 as
              libc::c_uint {
        1 => {
            //if (((STRUCTURE *)apsObjectList[0])->pStructureType->type == REF_FACTORY OR
		//	((STRUCTURE *)apsObjectList[0])->pStructureType->type == REF_CYBORG_FACTORY OR
		//	((STRUCTURE *)apsObjectList[0])->pStructureType->type == REF_VTOL_FACTORY)
            if StructIsFactory(*apsObjectList.offset(0 as libc::c_int as
                                                         isize) as
                                   *mut STRUCTURE) != 0 {
                orderFactories();
            } else if (*(*(*apsObjectList.offset(0 as libc::c_int as isize) as
                               *mut STRUCTURE)).pStructureType).type_0 ==
                          REF_RESEARCH as libc::c_int as libc::c_uint {
                orderResearch();
            }
        }
        0 => { orderDroids(); }
        _ => { }
    };
}
/*puts the selected players factories in order - Standard factories 1-5, then
cyborg factories 1-5 and then Vtol factories 1-5*/
unsafe extern "C" fn orderFactories() {
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psNext: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut entry: SDWORD = 0;
    let mut inc: UDWORD = 0;
    let mut type_0: UDWORD = 0;
    let mut objectInc: UDWORD = 0;
    if numObjects <= 3 as libc::c_int * 5 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"orderFactories : too many factories!\x00" as *const u8 as
                  *const libc::c_char);
    };
    if numObjects <= 3 as libc::c_int * 5 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"hci.c\x00" as *const u8 as *const libc::c_char,
              8232 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 15],
                                        &[libc::c_char; 15]>(b"orderFactories\x00")).as_ptr(),
              b"numObjects <= NUM_FACTORY_TYPES * MAX_FACTORY\x00" as
                  *const u8 as *const libc::c_char);
    };
    //copy the object list into the list to order
    memcpy(apsListToOrder as *mut libc::c_void,
           apsObjectList as *const libc::c_void,
           (::std::mem::size_of::<*mut BASE_OBJECT>() as
                libc::c_ulong).wrapping_mul((3 as libc::c_int *
                                                 5 as libc::c_int) as
                                                libc::c_uint));
    inc = 0 as libc::c_int as UDWORD;
    objectInc = inc;
    entry = objectInc as SDWORD;
    type_0 = 0 as libc::c_int as UDWORD;
    //go through the list of structures and extract them in order
    while entry < numObjects {
        psStruct =
            *apsListToOrder.offset(objectInc as isize) as *mut STRUCTURE;
        while !psStruct.is_null() {
            objectInc = objectInc.wrapping_add(1);
            psNext =
                *apsListToOrder.offset(objectInc as isize) as *mut STRUCTURE;
            if objectInc as SDWORD >= numObjects {
                psNext = 0 as *mut STRUCTURE
            }
            if StructIsFactory(psStruct) != 0 {
            } else {
                debug(LOG_ERROR,
                      b"orderFactories: structure is not a factory\x00" as
                          *const u8 as *const libc::c_char);
            };
            if StructIsFactory(psStruct) != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"hci.c\x00" as *const u8 as *const libc::c_char,
                      8252 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 15],
                                                &[libc::c_char; 15]>(b"orderFactories\x00")).as_ptr(),
                      b"StructIsFactory(psStruct)\x00" as *const u8 as
                          *const libc::c_char);
            };
            if (*(*((*psStruct).pFunctionality as
                        *mut FACTORY)).psAssemblyPoint).factoryInc as
                   libc::c_uint == inc &&
                   (*(*((*psStruct).pFunctionality as
                            *mut FACTORY)).psAssemblyPoint).factoryType as
                       libc::c_uint == type_0 {
                let fresh11 = entry;
                entry = entry + 1;
                let ref mut fresh12 = *apsObjectList.offset(fresh11 as isize);
                *fresh12 = psStruct as *mut BASE_OBJECT;
                //quick check that don't end up with more!
                if entry > numObjects {
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"orderFactories: too many objects!\x00" as
                                  *const u8 as *const libc::c_char);
                    };
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                                  *const u8 as *const libc::c_char,
                              b"hci.c\x00" as *const u8 as
                                  *const libc::c_char, 8263 as libc::c_int,
                              (*::std::mem::transmute::<&[u8; 15],
                                                        &[libc::c_char; 15]>(b"orderFactories\x00")).as_ptr(),
                              b"FALSE\x00" as *const u8 as
                                  *const libc::c_char);
                    };
                    return
                }
                break ;
            } else { psStruct = psNext }
        }
        inc = inc.wrapping_add(1);
        if inc > 5 as libc::c_int as libc::c_uint {
            inc = 0 as libc::c_int as UDWORD;
            type_0 = type_0.wrapping_add(1)
        }
        objectInc = 0 as libc::c_int as UDWORD
    };
}
//reorder the research facilities so that first built is first in the list
unsafe extern "C" fn orderResearch() {
    let mut psTemp: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut i: UDWORD = 0;
    let mut maxLoop: UDWORD = 0;
    maxLoop = (numObjects / 2 as libc::c_int) as UDWORD;
    i = 0 as libc::c_int as UDWORD;
    while i < maxLoop {
        psTemp = *apsObjectList.offset(i as isize);
        let ref mut fresh13 = *apsObjectList.offset(i as isize);
        *fresh13 =
            *apsObjectList.offset(((numObjects - 1 as libc::c_int) as
                                       libc::c_uint).wrapping_sub(i) as
                                      isize);
        let ref mut fresh14 =
            *apsObjectList.offset(((numObjects - 1 as libc::c_int) as
                                       libc::c_uint).wrapping_sub(i) as
                                      isize);
        *fresh14 = psTemp;
        i = i.wrapping_add(1)
    };
}
// reorder the commanders
unsafe extern "C" fn orderDroids() {
    let mut i: SDWORD = 0;
    let mut j: SDWORD = 0;
    let mut psTemp: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    debug(LOG_NEVER, b"orderUnit\n\x00" as *const u8 as *const libc::c_char);
    // bubble sort on the ID - first built will always be first in the list
    i = 0 as libc::c_int;
    while i < 15 as libc::c_int {
        j = i + 1 as libc::c_int;
        while j < 15 as libc::c_int {
            if !(*apsObjectList.offset(i as isize)).is_null() &&
                   !(*apsObjectList.offset(j as isize)).is_null() &&
                   (**apsObjectList.offset(i as isize)).id >
                       (**apsObjectList.offset(j as isize)).id {
                psTemp = *apsObjectList.offset(i as isize);
                let ref mut fresh15 = *apsObjectList.offset(i as isize);
                *fresh15 = *apsObjectList.offset(j as isize);
                let ref mut fresh16 = *apsObjectList.offset(j as isize);
                *fresh16 = psTemp
            }
            j += 1
        }
        i += 1
    };
}
//access function for selected object in the interface
//access function for selected object in the interface
#[no_mangle]
pub unsafe extern "C" fn getCurrentSelected() -> *mut BASE_OBJECT {
    return psObjSelected;
}
/* Add the options widgets to the widget screen */
/* **************************************************************************************/
/*              Function Prototypes                                                    */
/* Add the options widgets to the widget screen */
//
// Stack friendly wrappers for those greedy interface initialisation functions.
//
#[no_mangle]
pub unsafe extern "C" fn intAddOptions() -> BOOL {
    //#ifdef PSX
// If the stacks in the dcache then..
//	if(SpInDCache()) {
//		static BOOL ret;
//		// Set the stack pointer to point to the alternative stack which is'nt limited to 1k.
//		SetSpAlt();
//		ret = _intAddOptions();
//		SetSpAltNormal();
//		return ret;
//	}
//#endif
    return _intAddOptions();
}
/* Add the reticule widgets to the widget screen */
#[no_mangle]
pub unsafe extern "C" fn intAddReticule() -> BOOL {
    return _intAddReticule();
}
/* The int AddObject function is only called by :
 * intAddBuild, intAddManufacture and intAddResearch
 */
unsafe extern "C" fn intAddObject(mut psObjects: *mut BASE_OBJECT,
                                  mut psSelected: *mut BASE_OBJECT,
                                  mut bForceStats: BOOL) -> BOOL {
    return _intAddObject(psObjects, psSelected, bForceStats);
}
unsafe extern "C" fn intUpdateObject(mut psObjects: *mut BASE_OBJECT,
                                     mut psSelected: *mut BASE_OBJECT,
                                     mut bForceStats: BOOL) -> BOOL {
    return _intUpdateObject(psObjects, psSelected, bForceStats);
}
/* Reset the stats button for an object */
unsafe extern "C" fn intSetStats(mut id: UDWORD,
                                 mut psStats: *mut BASE_STATS) {
    _intSetStats(id, psStats);
}
/* Add the stats widgets to the widget screen */
/* If psSelected != NULL it specifies which stat should be hilited */
unsafe extern "C" fn intAddStats(mut ppsStatsList_0: *mut *mut BASE_STATS,
                                 mut numStats: UDWORD,
                                 mut psSelected: *mut BASE_STATS,
                                 mut psOwner: *mut BASE_OBJECT) -> BOOL {
    return _intAddStats(ppsStatsList_0, numStats, psSelected, psOwner);
}
