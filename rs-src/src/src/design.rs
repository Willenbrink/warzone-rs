use ::libc;
extern "C" {
    pub type _formation;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn abort() -> !;
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
    fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_uint)
     -> *mut libc::c_char;
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
    fn heapAlloc(psHeap: *mut OBJ_HEAP, ppObject: *mut *mut libc::c_void)
     -> BOOL;
    #[no_mangle]
    fn heapFree(psHeap: *mut OBJ_HEAP, pObject: *mut libc::c_void) -> BOOL;
    /* Get the string from an ID number */
    #[no_mangle]
    fn strresGetString(psRes: *mut STR_RES, id: UDWORD) -> *mut STRING;
    #[no_mangle]
    fn HashString(String: *mut libc::c_char) -> UINT;
    /* Add a form to the widget screen */
    #[no_mangle]
    fn widgAddForm(psScreen: *mut W_SCREEN, psInit: *mut W_FORMINIT) -> BOOL;
    /* Add a label to the widget screen */
    #[no_mangle]
    fn widgAddLabel(psScreen: *mut W_SCREEN, psInit: *mut W_LABINIT) -> BOOL;
    /* Add a button to a form */
    #[no_mangle]
    fn widgAddButton(psScreen: *mut W_SCREEN, psInit: *mut W_BUTINIT) -> BOOL;
    /* Add an edit box to a form */
    #[no_mangle]
    fn widgAddEditBox(psScreen: *mut W_SCREEN, psInit: *mut W_EDBINIT)
     -> BOOL;
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
    /* Return a pointer to a buffer containing the current string of a widget if any.
 * This will always return a valid string pointer.
 * NOTE: The string must be copied out of the buffer
 */
    #[no_mangle]
    fn widgGetString(psScreen: *mut W_SCREEN, id: UDWORD) -> *mut STRING;
    /* Set the text in a widget */
    #[no_mangle]
    fn widgSetString(psScreen: *mut W_SCREEN, id: UDWORD, pText: *mut STRING);
    /* Set the current tabs for a tab form */
    #[no_mangle]
    fn widgSetTabs(psScreen: *mut W_SCREEN, id: UDWORD, major: UWORD,
                   minor: UWORD);
    /* Set the current size of a bar graph */
    #[no_mangle]
    fn widgSetBarSize(psScreen: *mut W_SCREEN, id: UDWORD, size: UDWORD);
    /* Set the current size of a minor bar on a double graph */
    #[no_mangle]
    fn widgSetMinorBarSize(psScreen: *mut W_SCREEN, id: UDWORD, size: UDWORD);
    /* Return the ID of the widget the mouse was over this frame */
    #[no_mangle]
    fn widgGetMouseOver(psScreen: *mut W_SCREEN) -> UDWORD;
    /* Get widget structure */
    /* Find a widget in a screen from its ID number */
    #[no_mangle]
    fn widgGetFromID(psScreen: *mut W_SCREEN, id: UDWORD) -> *mut WIDGET;
    /* Set tip string for a widget */
    #[no_mangle]
    fn widgSetTip(psScreen: *mut W_SCREEN, id: UDWORD, pTip: *mut STRING);
    /* Set a colour on a form */
    #[no_mangle]
    fn widgSetColour(psScreen: *mut W_SCREEN, id: UDWORD, colour: UDWORD,
                     red: UBYTE, green: UBYTE, blue: UBYTE);
    /* Set a button or clickable form's state */
    #[no_mangle]
    fn widgSetButtonState(psScreen: *mut W_SCREEN, id: UDWORD, state: UDWORD);
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
    #[no_mangle]
    fn calcTemplatePower(psTemplate: *mut DROID_TEMPLATE) -> UDWORD;
    #[no_mangle]
    fn pie_GetVideoBufferWidth() -> UDWORD;
    #[no_mangle]
    fn pie_GetVideoBufferHeight() -> UDWORD;
    #[no_mangle]
    static mut apsDroidTemplates: [*mut DROID_TEMPLATE; 8];
    #[no_mangle]
    fn calcDroidWeight(psTemplate: *mut DROID_TEMPLATE) -> UDWORD;
    #[no_mangle]
    fn calcTemplateBody(psTemplate: *mut DROID_TEMPLATE, player: UBYTE)
     -> UDWORD;
    #[no_mangle]
    fn calcDroidBaseSpeed(psTemplate: *mut DROID_TEMPLATE, weight: UDWORD,
                          player: UBYTE) -> UDWORD;
    #[no_mangle]
    fn calcDroidSpeed(baseSpeed: UDWORD, terrainType: UDWORD,
                      propIndex: UDWORD) -> UDWORD;
    #[no_mangle]
    fn calcTemplateBuild(psTemplate: *mut DROID_TEMPLATE) -> UDWORD;
    //this item can be used to design droids
    //the player does not know about this item
    //this item has been found, but is unresearched
    /* ******************************************************************************
*		Allocate stats functions
*******************************************************************************/
/* Allocate Weapon stats */
    /*Allocate Armour stats*/
//extern BOOL statsAllocArmour(UDWORD numEntries);
    /*Allocate Body stats*/
    /*Allocate Brain stats*/
    /*Allocate Power stats*/
//extern BOOL statsAllocPower(UDWORD numEntries);
    /*Allocate Propulsion stats*/
    /*Allocate Sensor stats*/
    /*Allocate Ecm Stats*/
    /*Allocate Repair Stats*/
    /*Allocate Program Stats*/
    /*Allocate Construct Stats*/
    /* ******************************************************************************
*		Load stats functions
*******************************************************************************/
/* Return the number of newlines in a file buffer */
    /*Load the weapon stats from the file exported from Access*/
    /*Load the armour stats from the file exported from Access*/
//extern BOOL loadArmourStats(void);
    /*Load the body stats from the file exported from Access*/
    /*Load the brain stats from the file exported from Access*/
    /*Load the power stats from the file exported from Access*/
//extern BOOL loadPowerStats(void);
    /*Load the propulsion stats from the file exported from Access*/
    /*Load the sensor stats from the file exported from Access*/
    /*Load the ecm stats from the file exported from Access*/
    /*Load the repair stats from the file exported from Access*/
    /*Load the program stats from the file exported from Access*/
    /*Load the construct stats from the file exported from Access*/
    /*Load the Propulsion Types from the file exported from Access*/
    /*Load the propulsion sounds from the file exported from Access*/
    /*Load the Terrain Table from the file exported from Access*/
    /*Load the Special Ability stats from the file exported from Access*/
    /* load the IMDs to use for each body-propulsion combination */
    /*Load the weapon sounds from the file exported from Access*/
    /*Load the Weapon Effect Modifiers from the file exported from Access*/
    /* ******************************************************************************
*		Set stats functions
*******************************************************************************/
/* Set the stats for a particular weapon type
 * The function uses the ref number in the stats structure to
 * index the correct array entry
 */
    /*Set the stats for a particular armour type*/
//extern void statsSetArmour(ARMOUR_STATS	*psStats, UDWORD index);
    /*Set the stats for a particular body type*/
    /*Set the stats for a particular brain type*/
    /*Set the stats for a particular power type*/
//extern void statsSetPower(POWER_STATS	*psStats, UDWORD index);
    /*Set the stats for a particular propulsion type*/
    /*Set the stats for a particular sensor type*/
    /*Set the stats for a particular ecm type*/
    /*Set the stats for a particular repair type*/
    /*Set the stats for a particular program type*/
//extern void statsSetProgram(PROGRAM_STATS	*psStats, UDWORD index);
    /*Set the stats for a particular construct type*/
    /* ******************************************************************************
*		Get stats functions
*******************************************************************************/
    //extern ARMOUR_STATS *statsGetArmour(UDWORD ref);
    //extern POWER_STATS *statsGetPower(UDWORD ref);
    //extern PROGRAM_STATS *statsGetProgram(UDWORD ref);
    /* ******************************************************************************
*		Generic stats functions
*******************************************************************************/
/*Load the stats from the Access database*/
//extern BOOL loadStats(void);
    /*calls the STATS_DEALLOC macro for each set of stats*/
    /*Deallocate the stats passed in as parameter */
    //return the type of stat this ref refers to!
    //return the REF_START value of this type of stat
    /*Returns the component type based on the string - used for reading in data */
    //get the component Inc for a stat based on the name
    //get the component Inc for a stat based on the Resource name held in Names.txt
    /*sets the tech level for the stat passed in */
    /*returns the weapon sub class based on the string name passed in */
    /*either gets the name associated with the resource (if one) or allocates space and copies pName*/
    //converts the name read in from Access into the name which is used in the Stat lists (or ignores it)
    /*return the name to display for the interface - valid for OBJECTS and STATS*/
    /*sets the store to the body size based on the name passed in - returns FALSE 
if doesn't compare with any*/
    // Pass in a stat and get its name
    /*returns the propulsion type based on the string name passed in */
    /*returns the weapon effect based on the string name passed in */
    /*Access functions for the upgradeable stats of a weapon*/
    /*Access functions for the upgradeable stats of a sensor*/
    /*Access functions for the upgradeable stats of a ECM*/
    /*Access functions for the upgradeable stats of a repair*/
    /*Access functions for the upgradeable stats of a constructor*/
    /*Access functions for the upgradeable stats of a body*/
    /*dummy function for John*/
    //Access functions for the max values to be used in the Design Screen
    #[no_mangle]
    fn getMaxPropulsionSpeed() -> UDWORD;
    #[no_mangle]
    fn getMaxWeaponDamage() -> UDWORD;
    #[no_mangle]
    fn getMaxWeaponRange() -> UDWORD;
    #[no_mangle]
    fn getMaxRepairPoints() -> UDWORD;
    #[no_mangle]
    fn getMaxConstPoints() -> UDWORD;
    #[no_mangle]
    fn getMaxECMPower() -> UDWORD;
    #[no_mangle]
    fn getMaxSensorPower() -> UDWORD;
    #[no_mangle]
    fn getMaxSensorRange() -> UDWORD;
    #[no_mangle]
    fn getMaxBodyPower() -> UDWORD;
    #[no_mangle]
    fn getMaxBodyArmour() -> UDWORD;
    #[no_mangle]
    fn getMaxComponentWeight() -> UDWORD;
    #[no_mangle]
    fn bodyArmour(psStats: *mut BODY_STATS, player: UBYTE, bodyType: UBYTE,
                  weaponClass: WEAPON_CLASS) -> UDWORD;
    #[no_mangle]
    fn bodyPower(psStats: *mut BODY_STATS, player: UBYTE, bodyType: UBYTE)
     -> UDWORD;
    #[no_mangle]
    fn constructorPoints(psStats: *mut CONSTRUCT_STATS, player: UBYTE)
     -> UDWORD;
    #[no_mangle]
    fn repairPoints(psStats: *mut REPAIR_STATS, player: UBYTE) -> UDWORD;
    #[no_mangle]
    fn ecmPower(psStats: *mut ECM_STATS, player: UBYTE) -> UDWORD;
    #[no_mangle]
    fn sensorRange(psStats: *mut SENSOR_STATS, player: UBYTE) -> UDWORD;
    #[no_mangle]
    fn sensorPower(psStats: *mut SENSOR_STATS, player: UBYTE) -> UDWORD;
    #[no_mangle]
    fn weaponDamage(psStats: *mut WEAPON_STATS, player: UBYTE) -> UDWORD;
    #[no_mangle]
    fn weaponFirePause(psStats: *mut WEAPON_STATS, player: UBYTE) -> UDWORD;
    #[no_mangle]
    fn getStatName(pStat: *mut libc::c_void) -> *mut STRING;
    #[no_mangle]
    fn statType(ref_0: UDWORD) -> UDWORD;
    // the memory heap for templates
    #[no_mangle]
    static mut psTemplateHeap: *mut OBJ_HEAP;
    #[no_mangle]
    static mut asBodyStats: *mut BODY_STATS;
    #[no_mangle]
    static mut asBrainStats: *mut BRAIN_STATS;
    #[no_mangle]
    static mut asPropulsionStats: *mut PROPULSION_STATS;
    #[no_mangle]
    static mut asSensorStats: *mut SENSOR_STATS;
    #[no_mangle]
    static mut asECMStats: *mut ECM_STATS;
    #[no_mangle]
    static mut asRepairStats: *mut REPAIR_STATS;
    #[no_mangle]
    static mut asWeaponStats: *mut WEAPON_STATS;
    #[no_mangle]
    static mut asConstructStats: *mut CONSTRUCT_STATS;
    #[no_mangle]
    static mut asPropulsionTypes: *mut PROPULSION_TYPES;
    #[no_mangle]
    static mut numBodyStats: UDWORD;
    #[no_mangle]
    static mut numBrainStats: UDWORD;
    #[no_mangle]
    static mut numPropulsionStats: UDWORD;
    #[no_mangle]
    static mut numSensorStats: UDWORD;
    #[no_mangle]
    static mut numECMStats: UDWORD;
    #[no_mangle]
    static mut numRepairStats: UDWORD;
    #[no_mangle]
    static mut numWeaponStats: UDWORD;
    #[no_mangle]
    static mut numConstructStats: UDWORD;
    #[no_mangle]
    static mut apCompLists: [[*mut UBYTE; 9]; 8];
    #[no_mangle]
    fn droidTemplateType(psTemplate: *mut DROID_TEMPLATE) -> DROID_TYPE;
    #[no_mangle]
    fn getTemplateName(psTemplate: *mut DROID_TEMPLATE) -> *mut STRING;
    /*this is called to check the weapon is 'allowed'. Check if VTOL, the weapon is 
direct fire. Also check numVTOLattackRuns for the weapon is not zero - return 
TRUE if valid weapon*/
    #[no_mangle]
    fn checkValidWeaponForProp(psTemplate: *mut DROID_TEMPLATE) -> BOOL;
    /*called when a Template is deleted in the Design screen*/
    #[no_mangle]
    fn deleteTemplateFromProduction(psTemplate: *mut DROID_TEMPLATE,
                                    player: UBYTE);
    #[no_mangle]
    fn setGameUpdatePause(state: BOOL);
    #[no_mangle]
    fn setScrollPause(state: BOOL);
    #[no_mangle]
    fn pie_ImageFileID(ImageFile: *mut IMAGEFILE, ID: UWORD, x: libc::c_int,
                       y: libc::c_int);
    #[no_mangle]
    fn pie_ImageFileIDTile(ImageFile: *mut IMAGEFILE, ID: UWORD,
                           x: libc::c_int, y: libc::c_int, x0: libc::c_int,
                           y0: libc::c_int, Width: libc::c_int,
                           Height: libc::c_int);
    #[no_mangle]
    fn iV_GetImageWidth(ImageFile: *mut IMAGEFILE, ID: UWORD) -> UWORD;
    #[no_mangle]
    fn iV_GetImageHeight(ImageFile: *mut IMAGEFILE, ID: UWORD) -> UWORD;
    #[no_mangle]
    fn pie_UploadDisplayBuffer(DisplayBuffer_0: *mut libc::c_char);
    #[no_mangle]
    fn pie_SetGeometricOffset(x: libc::c_int, y: libc::c_int);
    #[no_mangle]
    static mut radarOnScreen: BOOL;
    //extern BOOL		bScreenClose;
//extern UDWORD closingTimeStart;
//extern UDWORD screenCloseState;
//extern BOOL	bPlayerHasHQ;
    #[no_mangle]
    static mut bRender3DOnly: BOOL;
    /* Default level of sensor, repair and ECM */
    #[no_mangle]
    static mut aDefaultSensor: [UDWORD; 8];
    #[no_mangle]
    static mut aDefaultECM: [UDWORD; 8];
    #[no_mangle]
    static mut aDefaultRepair: [UDWORD; 8];
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
    /* The widget screen */
    #[no_mangle]
    static mut psWScreen: *mut W_SCREEN;
    /* the widget font */
    #[no_mangle]
    static mut WFont: libc::c_int;
    /* Which is the currently selected player */
    #[no_mangle]
    static mut selectedPlayer: UDWORD;
    /* The flag to specify if the Intelligence screen is up */
//extern BOOL				intelMapUp;
    /* The current template for the design screen to start with*/
    #[no_mangle]
    static mut psCurrTemplate: *mut DROID_TEMPLATE;
    #[no_mangle]
    static mut apsTemplateList: *mut *mut DROID_TEMPLATE;
    #[no_mangle]
    static mut DisplayBuffer: *mut libc::c_char;
    #[no_mangle]
    fn numForms(total: UDWORD, perForm: UDWORD) -> UWORD;
    // return the maximum range for a weapon
    #[no_mangle]
    fn proj_GetLongRange(psStats: *mut WEAPON_STATS, dz: SDWORD) -> SDWORD;
    #[no_mangle]
    static mut IntImages: *mut IMAGEFILE;
    // All the 2d graphics for the user interface.
    // A few useful defined frames.
    #[no_mangle]
    static mut FrameNormal: IMAGEFRAME;
    // A few useful defined tabs.
    #[no_mangle]
    static mut StandardTab: TABDEF;
    // Begin a rendering lock.
    #[no_mangle]
    fn DrawBegin();
    // End a rendering lock.
    #[no_mangle]
    fn DrawEnd();
    // Draws a transparent window
    #[no_mangle]
    fn RenderWindowFrame(Frame: *mut IMAGEFRAME, x: UDWORD, y: UDWORD,
                         Width: UDWORD, Height: UDWORD);
    #[no_mangle]
    static mut ObjectBuffers: [RENDERED_BUTTON; 40];
    #[no_mangle]
    static mut StatBuffers: [RENDERED_BUTTON; 80];
    #[no_mangle]
    static mut System0Buffers: [RENDERED_BUTTON; 80];
    #[no_mangle]
    static mut CurrentStatsTemplate: *mut BASE_STATS;
    // Get a free RENDERED_BUTTON structure for an object window button.
    #[no_mangle]
    fn GetObjectBuffer() -> SDWORD;
    // Clear ( make unused ) all RENDERED_BUTTON structures for the object window.
    #[no_mangle]
    fn ClearObjectBuffers();
    // Get a free RENDERED_BUTTON structure for a stat window button.
    #[no_mangle]
    fn GetStatBuffer() -> SDWORD;
    // Clear ( make unused ) all RENDERED_BUTTON structures for the stat window.
    #[no_mangle]
    fn ClearStatBuffers();
    #[no_mangle]
    fn intDisplayStatsButton(psWidget: *mut _widget, xOffset: UDWORD,
                             yOffset: UDWORD, pColours: *mut UDWORD);
    #[no_mangle]
    fn intDisplayObjectForm(psWidget: *mut _widget, xOffset: UDWORD,
                            yOffset: UDWORD, pColours: *mut UDWORD);
    #[no_mangle]
    fn intDisplayPlainForm(psWidget: *mut _widget, xOffset: UDWORD,
                           yOffset: UDWORD, pColours: *mut UDWORD);
    #[no_mangle]
    fn intDisplayImage(psWidget: *mut _widget, xOffset: UDWORD,
                       yOffset: UDWORD, pColours: *mut UDWORD);
    #[no_mangle]
    fn intDisplayButtonHilight(psWidget: *mut _widget, xOffset: UDWORD,
                               yOffset: UDWORD, pColours: *mut UDWORD);
    #[no_mangle]
    fn intDisplayButtonFlash(psWidget: *mut _widget, xOffset: UDWORD,
                             yOffset: UDWORD, pColours: *mut UDWORD);
    #[no_mangle]
    fn intDisplayTab(psWidget: *mut _widget, TabType: UDWORD,
                     Position: UDWORD, Number: UDWORD, Selected: BOOL,
                     Hilight: BOOL, x: UDWORD, y: UDWORD, Width: UDWORD,
                     Height: UDWORD);
    #[no_mangle]
    fn intDisplayEditBox(psWidget: *mut _widget, xOffset: UDWORD,
                         yOffset: UDWORD, pColours: *mut UDWORD);
    /* Draws a stats bar for the design screen */
    #[no_mangle]
    fn intDisplayStatsBar(psWidget: *mut _widget, xOffset: UDWORD,
                          yOffset: UDWORD, pColours: *mut UDWORD);
    /* Draws a Template Power Bar for the Design Screen */
    #[no_mangle]
    fn intDisplayDesignPowerBar(psWidget: *mut _widget, xOffset: UDWORD,
                                yOffset: UDWORD, pColours: *mut UDWORD);
    /* The string resource object */
    #[no_mangle]
    static mut psStringRes: *mut STR_RES;
    #[no_mangle]
    fn getComponentDroidTemplateRadius(psDroid: *mut DROID_TEMPLATE)
     -> UDWORD;
    #[no_mangle]
    fn getComponentRadius(psComponent: *mut BASE_STATS) -> UDWORD;
    #[no_mangle]
    fn displayComponentButton(Stat: *mut BASE_STATS, Rotation: *mut iVector,
                              Position: *mut iVector, RotXYZ: BOOL,
                              scale: SDWORD);
    #[no_mangle]
    fn displayComponentButtonTemplate(psTemplate: *mut DROID_TEMPLATE,
                                      Rotation: *mut iVector,
                                      Position: *mut iVector, RotXYZ: BOOL,
                                      scale: SDWORD);
    // Activate a callback trigger
    #[no_mangle]
    fn eventFireCallbackTrigger(callback: TRIGGER_TYPE);
    #[no_mangle]
    fn GetGameMode() -> UDWORD;
    /* Do the 3D display */
    #[no_mangle]
    fn displayWorld();
    // whether the tutorial is active
    #[no_mangle]
    static mut bInTutorial: BOOL;
    // the game description.
    #[no_mangle]
    static mut bMultiPlayer: BOOL;
    #[no_mangle]
    fn sendTemplate(t: *mut DROID_TEMPLATE) -> BOOL;
    #[no_mangle]
    fn SendDestroyTemplate(t: *mut DROID_TEMPLATE) -> BOOL;
    /* the widget screen */
    /* the widget font */
//extern PROP_FONT	*psWFont;
    #[no_mangle]
    static mut objID: UDWORD;
    #[no_mangle]
    static mut numComponent: UDWORD;
    #[no_mangle]
    static mut apsComponentList: *mut *mut COMP_BASE_STATS;
    #[no_mangle]
    static mut numExtraSys: UDWORD;
    #[no_mangle]
    static mut apsExtraSysList: *mut *mut COMP_BASE_STATS;
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
pub type UINT = libc::c_uint;
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
// Plain bar graph
// Bar graph with a trough showing empty percentage
// Double bar graph, one on top of other
/* ********** Slider styles ***************/
// Plain slider
/* **********************************************************************************/
/* Generic widget colour */
pub type W_COLOURDEF = _w_colourdef;
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
//ALL components and structures and research topics have a tech level to which they belong
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
pub type _terrain_type = libc::c_uint;
pub const TER_MAX: _terrain_type = 12;
pub const TER_SLUSH: _terrain_type = 11;
pub const TER_SHEETICE: _terrain_type = 10;
pub const TER_RUBBLE: _terrain_type = 9;
pub const TER_CLIFFFACE: _terrain_type = 8;
pub const TER_WATER: _terrain_type = 7;
pub const TER_ROAD: _terrain_type = 6;
pub const TER_PINKROCK: _terrain_type = 5;
pub const TER_REDBRUSH: _terrain_type = 4;
pub const TER_GREENMUD: _terrain_type = 3;
pub const TER_BAKEDEARTH: _terrain_type = 2;
pub const TER_SANDYBRUSH: _terrain_type = 1;
pub const TER_SAND: _terrain_type = 0;
/*
 * Map.h
 *
 * Definitions for the map structure
 *
 */
// Visibility bits - can also be accessed as a byte (as a whole).
/* The different types of terrain as far as the game is concerned */
pub type TYPE_OF_TERRAIN = _terrain_type;
/* The basic form data */
/* The common widget data */
/* Disable all child widgets if TRUE */
/* Working coords for animations. */
/* Animation counter. */
/* Animation start time */
/* Colours for the form and its widgets. signed since aColours -1 means use bitmap. */
/* The last widget to be hilited */
/* This is used to track when the mouse moves */
/* off something */
/* The widgets on the form */
/* The standard form */
pub type W_FORM = _w_form;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_minortab {
    pub psWidgets: *mut WIDGET,
    pub pTip: *mut STRING,
}
/* The common form data */
/* Information for a minor tab */
pub type W_MINORTAB = _w_minortab;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_majortab {
    pub lastMinor: UWORD,
    pub numMinor: UWORD,
    pub asMinor: [W_MINORTAB; 5],
    pub pTip: *mut STRING,
}
/* Graphics data for the tab will go here */
// Widgets on the tab
// Tool tip
/* Information for a major tab */
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
/* Graphics data for the tab will go here */
// Store which was the last selected minor tab
// Minor tab information
/* The tabbed form data structure */
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
// Button is hilited
// Button is locked down
// Button is locked but clickable
// Button flashing is enabled
// Button is flashing
/* The clickable form data structure */
pub type W_CLICKFORM = _w_clickform;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FRAMERECT {
    pub Type: UWORD,
    pub TLXOffset: SWORD,
    pub TLYOffset: SWORD,
    pub BRXOffset: SWORD,
    pub BRYOffset: SWORD,
    pub ColourIndex: UBYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IMAGEFRAME {
    pub OffsetX0: SWORD,
    pub OffsetY0: SWORD,
    pub OffsetX1: SWORD,
    pub OffsetY1: SWORD,
    pub TopLeft: SWORD,
    pub TopRight: SWORD,
    pub BottomLeft: SWORD,
    pub BottomRight: SWORD,
    pub TopEdge: SWORD,
    pub TopType: SWORD,
    pub RightEdge: SWORD,
    pub RightType: SWORD,
    pub BottomEdge: SWORD,
    pub BottomType: SWORD,
    pub LeftEdge: SWORD,
    pub LeftType: SWORD,
    pub FRect: [FRAMERECT; 5],
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
/* The common form data */
// Button state of the form
// Tip for the form
// Audio ID for form clicked sound
// Audio ID for form hilighted sound
// Pointer to audio callback function
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
/* Which component type is being selected on the design screen */
pub type DES_COMPMODE = _des_compmode;
pub type _des_compmode = libc::c_uint;
pub const IDES_NOCOMPONENT: _des_compmode = 5;
pub const IDES_PROPULSION: _des_compmode = 4;
pub const IDES_BODY: _des_compmode = 3;
pub const IDES_TURRET: _des_compmode = 2;
pub const IDES_SYSTEM: _des_compmode = 1;
pub const IDES_BRAIN: _des_compmode = 0;
// The brain for the droid
// The main system for the droid (sensor, ECM, constructor)
// The weapon for the droid
// The droid body
// The propulsion system
// No system has been selected
/*
 * CmdDroidDef.h
 *
 * Typedef's for command droids
 *
 */
// the maximum number of command droids per side
pub type COMMAND_DROID = _command_droid;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _command_droid {
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
    pub died: UDWORD,
    pub aggression: SWORD,
    pub survival: SWORD,
    pub nWeapStat: SWORD,
    pub kills: UWORD,
    pub psDroid: *mut _droid,
}
pub const STR_DES_OTHER: _fixed_str_id = 36;
pub const STR_DES_WEAPONS: _fixed_str_id = 35;
pub const IDES_AIR: _des_propmode = 1;
pub const IDES_GROUND: _des_propmode = 0;
// define the command droid as a COMPONENT
// so it fits into the design screen
/* Which type of propulsion is being selected */
pub type DES_PROPMODE = _des_propmode;
pub type _des_propmode = libc::c_uint;
pub const IDES_NOPROPULSION: _des_propmode = 2;
pub const STR_DES_WEIGHT: _fixed_str_id = 22;
pub const STR_DES_WATER: _fixed_str_id = 34;
pub const STR_DES_OFFROAD: _fixed_str_id = 33;
pub const STR_DES_ROAD: _fixed_str_id = 32;
pub const STR_DES_AIR: _fixed_str_id = 31;
pub const IDES_WEAPON: _des_sysmode = 4;
pub const IDES_COMMAND: _des_sysmode = 5;
pub const IDES_REPAIR: _des_sysmode = 3;
pub const IDES_ECM: _des_sysmode = 1;
pub const IDES_CONSTRUCT: _des_sysmode = 2;
pub const IDES_SENSOR: _des_sysmode = 0;
// Ground propulsion (wheeled, tracked, etc).
// Air propulsion
// No propulsion has be selected
/* The maximum number of characters in a design name */
/* Which type of system is displayed on the design screen */
pub type DES_SYSMODE = _des_sysmode;
pub type _des_sysmode = libc::c_uint;
pub const IDES_NOSYSTEM: _des_sysmode = 6;
pub const STR_DES_ROF: _fixed_str_id = 30;
pub const STR_DES_DAMAGE: _fixed_str_id = 29;
pub const STR_DES_RANGE: _fixed_str_id = 28;
pub const STR_DES_BUILD_POINTS: _fixed_str_id = 27;
pub const STR_DES_ECM_POWER: _fixed_str_id = 26;
pub const STR_DES_SENSOR_POWER: _fixed_str_id = 25;
pub const STR_DES_SENSOR_RANGE: _fixed_str_id = 24;
pub const STR_DES_TEMPBODY: _fixed_str_id = 42;
pub const STR_DES_TEMPPOWER: _fixed_str_id = 41;
pub const STR_DES_POWER: _fixed_str_id = 21;
pub const STR_DES_ARMOUR_HEAT: _fixed_str_id = 20;
pub const STR_DES_ARMOUR_KIN: _fixed_str_id = 19;
pub const STR_DES_DEL: _fixed_str_id = 17;
pub const STR_DES_TURRET: _fixed_str_id = 40;
pub const STR_DES_PROPULSION: _fixed_str_id = 39;
pub const STR_DES_BODY: _fixed_str_id = 38;
pub const STR_DES_POWERUSE: _fixed_str_id = 23;
// The sensor clickable is displayed
// The ECM clickable is displayed
// The Constructor clickable is displayed
// The Repair clickable is displayed
// The Weapon clickable is displayed
// The command droid clickable is displayed
// No system clickable has been displayed
// Design screen strings
pub const STR_DES_NEWVEH: _fixed_str_id = 14;
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
pub const CALL_DROIDDESIGNED: _scr_callback_types = 7;
pub const CALL_DESIGN_SYSTEM: _scr_callback_types = 30;
pub const CALL_DESIGN_COMMAND: _scr_callback_types = 31;
pub const CALL_DESIGN_PROPULSION: _scr_callback_types = 33;
pub const CALL_DESIGN_BODY: _scr_callback_types = 32;
pub const CALL_DESIGN_WEAPON: _scr_callback_types = 29;
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
pub const STR_DES_COMMAND: _fixed_str_id = 37;
pub const STR_DES_NEW: _fixed_str_id = 18;
pub const STR_DES_CANCEL: _fixed_str_id = 16;
pub const STR_DES_STORE: _fixed_str_id = 15;
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
pub const CALL_DELIVPOINTMOVED: _scr_callback_types = 6;
pub const CALL_GAMEINIT: _scr_callback_types = 5;
#[no_mangle]
pub static mut desSysMode: DES_SYSMODE = IDES_SENSOR;
#[no_mangle]
pub static mut desCompMode: DES_COMPMODE = IDES_BRAIN;
#[no_mangle]
pub static mut desPropMode: DES_PROPMODE = IDES_GROUND;
#[no_mangle]
pub static mut StringBuffer: [libc::c_char; 1920] = [0; 1920];
// unique ID creation thing..
/* default droid design template */
#[no_mangle]
pub static mut sDefaultDesignTemplate: DROID_TEMPLATE =
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
                       0 as *const _droid_template as *mut _droid_template,};
//extern iIMDShape *CurrentStatsTemplate;
//extern iIMDShape *CurrentStatsShape;
//extern SWORD CurrentStatsIndex;
#[no_mangle]
pub static mut ViewRotation: UDWORD = 0 as libc::c_int as UDWORD;
#[no_mangle]
pub static mut ViewShape: *mut iIMDShape =
    0 as *const iIMDShape as *mut iIMDShape;
/* The current name of the design */
static mut aCurrName: [STRING; 80] = [0; 80];
/* The button id of the component that is in the design */
static mut desCompID: UDWORD = 0;
static mut droidTemplID: UDWORD = 0;
/* The current design being edited on the design screen */
#[no_mangle]
pub static mut sCurrDesign: DROID_TEMPLATE =
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
                       0 as *const _droid_template as *mut _droid_template,};
/* Flag to indictate whether a 'spare' template button is required */
static mut newTemplate: BOOL = 0 as libc::c_int;
/* Add the design widgets to the widget screen */
#[no_mangle]
pub unsafe extern "C" fn _intAddDesign(mut bShowCentreScreen: BOOL) -> BOOL {
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
    desSetupDesignTemplates();
    //set which states are to be paused while design screen is up
    setDesignPauseState();
    if GetGameMode() == 3 as libc::c_int as libc::c_uint && bMultiPlayer == 0
       {
        // Only do this in main game.
        let mut radOnScreen: BOOL = radarOnScreen;
        bRender3DOnly = 1 as libc::c_int;
        radarOnScreen = 0 as libc::c_int;
        // Just display the 3d, no interface
        displayWorld();
        // Upload the current display back buffer into system memory.
        pie_UploadDisplayBuffer(DisplayBuffer);
        radarOnScreen = radOnScreen;
        bRender3DOnly = 0 as libc::c_int
    }
    //initialise flags
    newTemplate = 0 as libc::c_int;
    memset(&mut sFormInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_FORMINIT>() as libc::c_ulong);
    memset(&mut sLabInit as *mut W_LABINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_LABINIT>() as libc::c_ulong);
    memset(&mut sEdInit as *mut W_EDBINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_EDBINIT>() as libc::c_ulong);
    memset(&mut sButInit as *mut W_BUTINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_BUTINIT>() as libc::c_ulong);
    memset(&mut sBarInit as *mut W_BARINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_BARINIT>() as libc::c_ulong);
    /* Add the main design form */
    sFormInit.formID = 0 as libc::c_int as UDWORD; //0;
    sFormInit.id = 5000 as libc::c_int as UDWORD; //0;
    sFormInit.style = 0 as libc::c_int as UDWORD; //DISP_WIDTH-1;
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
            as SWORD; //DES_BASEHEIGHT;
    sFormInit.y =
        (59 as libc::c_int as
             libc::c_uint).wrapping_add(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint).wrapping_div(2
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   libc::c_uint))
            as SWORD; //intDisplayStatusForm;
    sFormInit.width = 315 as libc::c_int as UWORD;
    sFormInit.height = 262 as libc::c_int as UWORD;
    sFormInit.pDisplay =
        Some(intDisplayPlainForm as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    if widgAddForm(psWScreen, &mut sFormInit) == 0 { return 0 as libc::c_int }
    //	widgSetColour(psWScreen, 0, WCOL_TEXT,
//				255,255,255);
//	widgSetColour(psWScreen, 0, WCOL_CURSOR,
//				255,255,0);
    /* add the edit name box */
    sEdInit.formID = 5000 as libc::c_int as UDWORD;
    sEdInit.id = 5013 as libc::c_int as UDWORD;
    sEdInit.style = 0 as libc::c_int as UDWORD;
    sEdInit.x = 8 as libc::c_int as SWORD;
    sEdInit.y = 6 as libc::c_int as SWORD;
    sEdInit.width =
        (315 as libc::c_int - 2 as libc::c_int * 8 as libc::c_int) as UWORD;
    sEdInit.height = 14 as libc::c_int as UWORD;
    sEdInit.pText =
        strresGetString(psStringRes, STR_DES_NEWVEH as libc::c_int as UDWORD);
    sEdInit.FontID = WFont;
    sEdInit.pBoxDisplay =
        Some(intDisplayEditBox as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    if widgAddEditBox(psWScreen, &mut sEdInit) == 0 {
        return 0 as libc::c_int
    }
    CurrentStatsTemplate = 0 as *mut BASE_STATS;
    //	CurrentStatsShape = NULL;
//	CurrentStatsIndex = -1;
    /* Initialise the current design */
    if !psCurrTemplate.is_null() {
        memcpy(&mut sCurrDesign as *mut DROID_TEMPLATE as *mut libc::c_void,
               psCurrTemplate as *const libc::c_void,
               ::std::mem::size_of::<DROID_TEMPLATE>() as libc::c_ulong);
        strncpy(aCurrName.as_mut_ptr(),
                getStatName(psCurrTemplate as *mut libc::c_void),
                (80 as libc::c_int - 1 as libc::c_int) as libc::c_uint);
        strcpy(sCurrDesign.aName.as_mut_ptr(), aCurrName.as_mut_ptr());
    } else {
        memcpy(&mut sCurrDesign as *mut DROID_TEMPLATE as *mut libc::c_void,
               &mut sDefaultDesignTemplate as *mut DROID_TEMPLATE as
                   *const libc::c_void,
               ::std::mem::size_of::<DROID_TEMPLATE>() as libc::c_ulong);
        strcpy(aCurrName.as_mut_ptr(),
               strresGetString(psStringRes,
                               STR_DES_NEWVEH as libc::c_int as UDWORD));
        strcpy(sCurrDesign.aName.as_mut_ptr(), aCurrName.as_mut_ptr());
    }
    /* Add the design templates form */
    if intAddTemplateForm(psCurrTemplate) == 0 { return 0 as libc::c_int }
    /* Add the 3D View form */
    sFormInit.formID = 5000 as libc::c_int as UDWORD;
    sFormInit.id = 5009 as libc::c_int as UDWORD;
    sFormInit.style = 0 as libc::c_int as UDWORD;
    sFormInit.x = 8 as libc::c_int as SWORD;
    sFormInit.y = 25 as libc::c_int as SWORD;
    sFormInit.width = 236 as libc::c_int as UWORD;
    sFormInit.height = 192 as libc::c_int as UWORD;
    sFormInit.pDisplay =
        Some(intDisplayViewForm as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    if widgAddForm(psWScreen, &mut sFormInit) == 0 { return 0 as libc::c_int }
    /* Add the part button form */
    sFormInit.formID = 5000 as libc::c_int as UDWORD;
    sFormInit.id = 5027 as libc::c_int as UDWORD;
    sFormInit.style = 0 as libc::c_int as UDWORD;
    sFormInit.x =
        (8 as libc::c_int + 236 as libc::c_int + 2 as libc::c_int) as SWORD;
    sFormInit.y = 25 as libc::c_int as SWORD;
    sFormInit.width =
        (iV_GetImageWidth(IntImages, IMAGE_DES_TURRET as libc::c_int as UWORD)
             as libc::c_int + 2 as libc::c_int * 6 as libc::c_int) as UWORD;
    sFormInit.height = 192 as libc::c_int as UWORD;
    sFormInit.pDisplay =
        Some(intDisplayDesignForm as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    if widgAddForm(psWScreen, &mut sFormInit) == 0 { return 0 as libc::c_int }
    // add the body part button
    sButInit.formID = 5027 as libc::c_int as UDWORD;
    sButInit.id = 5901 as libc::c_int as UDWORD;
    sButInit.style = 0 as libc::c_int as UDWORD;
    sButInit.x = 6 as libc::c_int as SWORD;
    sButInit.y = 6 as libc::c_int as SWORD;
    sButInit.width =
        iV_GetImageWidth(IntImages, IMAGE_DES_BODY as libc::c_int as UWORD);
    sButInit.height =
        iV_GetImageHeight(IntImages, IMAGE_DES_BODY as libc::c_int as UWORD);
    sButInit.pTip =
        strresGetString(psStringRes, STR_DES_BODY as libc::c_int as UDWORD);
    sButInit.FontID = WFont;
    sButInit.pDisplay =
        Some(intDisplayButtonFlash as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    sButInit.pUserData =
        ((1 as libc::c_int & 0x3ff as libc::c_int) << 20 as libc::c_int |
             (IMAGE_DES_BODYH as libc::c_int & 0x3ff as libc::c_int) <<
                 10 as libc::c_int |
             IMAGE_DES_BODY as libc::c_int & 0x3ff as libc::c_int) as
            *mut libc::c_void;
    if widgAddButton(psWScreen, &mut sButInit) == 0 {
        return 0 as libc::c_int
    }
    // add the propulsion part button
    sButInit.formID = 5027 as libc::c_int as UDWORD;
    sButInit.id = 5902 as libc::c_int as UDWORD;
    sButInit.style = 0 as libc::c_int as UDWORD;
    sButInit.x = 6 as libc::c_int as SWORD;
    sButInit.y =
        (iV_GetImageHeight(IntImages,
                           IMAGE_DES_PROPULSION as libc::c_int as UWORD) as
             libc::c_int + 2 as libc::c_int * 6 as libc::c_int) as UWORD as
            SWORD;
    sButInit.width =
        iV_GetImageWidth(IntImages,
                         IMAGE_DES_PROPULSION as libc::c_int as UWORD);
    sButInit.height =
        iV_GetImageHeight(IntImages,
                          IMAGE_DES_PROPULSION as libc::c_int as UWORD);
    sButInit.pTip =
        strresGetString(psStringRes,
                        STR_DES_PROPULSION as libc::c_int as UDWORD);
    sButInit.FontID = WFont;
    sButInit.pDisplay =
        Some(intDisplayButtonFlash as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    sButInit.pUserData =
        ((1 as libc::c_int & 0x3ff as libc::c_int) << 20 as libc::c_int |
             (IMAGE_DES_PROPULSIONH as libc::c_int & 0x3ff as libc::c_int) <<
                 10 as libc::c_int |
             IMAGE_DES_PROPULSION as libc::c_int & 0x3ff as libc::c_int) as
            *mut libc::c_void;
    if widgAddButton(psWScreen, &mut sButInit) == 0 {
        return 0 as libc::c_int
    }
    // add the turret part button
    sButInit.formID = 5027 as libc::c_int as UDWORD;
    sButInit.id = 5900 as libc::c_int as UDWORD;
    sButInit.style = 0 as libc::c_int as UDWORD;
    sButInit.x = 6 as libc::c_int as SWORD;
    sButInit.y =
        (iV_GetImageHeight(IntImages,
                           IMAGE_DES_PROPULSION as libc::c_int as UWORD) as
             libc::c_int +
             iV_GetImageHeight(IntImages,
                               IMAGE_DES_BODY as libc::c_int as UWORD) as
                 libc::c_int + 3 as libc::c_int * 6 as libc::c_int) as UWORD
            as SWORD;
    sButInit.width =
        iV_GetImageWidth(IntImages, IMAGE_DES_TURRET as libc::c_int as UWORD);
    sButInit.height =
        iV_GetImageHeight(IntImages,
                          IMAGE_DES_TURRET as libc::c_int as UWORD);
    sButInit.pTip =
        strresGetString(psStringRes, STR_DES_TURRET as libc::c_int as UDWORD);
    sButInit.FontID = WFont;
    sButInit.pDisplay =
        Some(intDisplayButtonFlash as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    sButInit.pUserData =
        ((1 as libc::c_int & 0x3ff as libc::c_int) << 20 as libc::c_int |
             (IMAGE_DES_TURRETH as libc::c_int & 0x3ff as libc::c_int) <<
                 10 as libc::c_int |
             IMAGE_DES_TURRET as libc::c_int & 0x3ff as libc::c_int) as
            *mut libc::c_void;
    if widgAddButton(psWScreen, &mut sButInit) == 0 {
        return 0 as libc::c_int
    }
    /* add the delete button */
    sButInit.formID = 5027 as libc::c_int as UDWORD;
    sButInit.id = 5011 as libc::c_int as UDWORD;
    sButInit.style = 0 as libc::c_int as UDWORD;
    sButInit.width =
        iV_GetImageWidth(IntImages, IMAGE_DES_BIN as libc::c_int as UWORD);
    sButInit.height =
        iV_GetImageHeight(IntImages, IMAGE_DES_BIN as libc::c_int as UWORD);
    sButInit.x = 6 as libc::c_int as SWORD;
    sButInit.y =
        (192 as libc::c_int - sButInit.height as libc::c_int -
             6 as libc::c_int) as UWORD as SWORD;
    sButInit.pTip =
        strresGetString(psStringRes, STR_DES_DEL as libc::c_int as UDWORD);
    sButInit.FontID = WFont;
    sButInit.pDisplay =
        Some(intDisplayButtonHilight as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    sButInit.pUserData =
        ((0 as libc::c_int & 0x3ff as libc::c_int) << 20 as libc::c_int |
             (IMAGE_DES_BINH as libc::c_int & 0x3ff as libc::c_int) <<
                 10 as libc::c_int |
             IMAGE_DES_BIN as libc::c_int & 0x3ff as libc::c_int) as
            *mut libc::c_void;
    if widgAddButton(psWScreen, &mut sButInit) == 0 {
        return 0 as libc::c_int
    }
    /* add central stats form */
    sFormInit.formID = 0 as libc::c_int as UDWORD;
    sFormInit.id = 5001 as libc::c_int as UDWORD;
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
        (59 as libc::c_int as
             libc::c_uint).wrapping_add(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint).wrapping_div(2
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   libc::c_uint)).wrapping_add(262
                                                                                                                                                   as
                                                                                                                                                   libc::c_int
                                                                                                                                                   as
                                                                                                                                                   libc::c_uint).wrapping_add(3
                                                                                                                                                                                  as
                                                                                                                                                                                  libc::c_int
                                                                                                                                                                                  as
                                                                                                                                                                                  libc::c_uint)
            as SWORD;
    sFormInit.width = 315 as libc::c_int as UWORD;
    sFormInit.height = 100 as libc::c_int as UWORD;
    sFormInit.pDisplay =
        Some(intDisplayPlainForm as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    if widgAddForm(psWScreen, &mut sFormInit) == 0 { return 0 as libc::c_int }
    /* Add the body form */
    sFormInit.formID = 5001 as libc::c_int as UDWORD;
    sFormInit.id = 5007 as libc::c_int as UDWORD;
    sFormInit.style = (4 as libc::c_int | 8 as libc::c_int) as UDWORD;
    sFormInit.width = 300 as libc::c_int as UWORD;
    sFormInit.height = 85 as libc::c_int as UWORD;
    sFormInit.x = 6 as libc::c_int as SWORD;
    sFormInit.y = 6 as libc::c_int as SWORD;
    sFormInit.pDisplay =
        Some(intDisplayStatForm as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    if widgAddForm(psWScreen, &mut sFormInit) == 0 { return 0 as libc::c_int }
    /* Add the graphs for the Body */
    sBarInit.formID = 5007 as libc::c_int as UDWORD; //DES_CLICKBARY;
    sBarInit.id = 5100 as libc::c_int as UDWORD; //DBAR_BODYMAXARMOUR;
    sBarInit.style =
        0 as libc::c_int as UDWORD; //+= DES_CLICKBARHEIGHT + DES_CLICKGAP;
    sBarInit.orientation = 0x1 as libc::c_int as UWORD; //DBAR_BODYMAXARMOUR;
    sBarInit.x = 154 as libc::c_int as SWORD;
    sBarInit.y = 7 as libc::c_int as SWORD;
    sBarInit.width = 140 as libc::c_int as UWORD;
    sBarInit.height = 11 as libc::c_int as UWORD;
    sBarInit.size = 50 as libc::c_int as UWORD;
    sBarInit.sCol.red = 255 as libc::c_int as UBYTE;
    sBarInit.sCol.green = 235 as libc::c_int as UBYTE;
    sBarInit.sCol.blue = 19 as libc::c_int as UBYTE;
    sBarInit.sMinorCol.red = 0x55 as libc::c_int as UBYTE;
    sBarInit.sMinorCol.green = 0 as libc::c_int as UBYTE;
    sBarInit.sMinorCol.blue = 0 as libc::c_int as UBYTE;
    sBarInit.pDisplay =
        Some(intDisplayStatsBar as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    sBarInit.pTip =
        strresGetString(psStringRes,
                        STR_DES_ARMOUR_KIN as libc::c_int as UDWORD);
    sBarInit.iRange = getMaxBodyArmour() as UWORD;
    if widgAddBarGraph(psWScreen, &mut sBarInit) == 0 {
        return 1 as libc::c_int
    }
    sBarInit.id = 5128 as libc::c_int as UDWORD;
    sBarInit.y =
        (7 as libc::c_int + 11 as libc::c_int + 9 as libc::c_int) as SWORD;
    sBarInit.pTip =
        strresGetString(psStringRes,
                        STR_DES_ARMOUR_HEAT as libc::c_int as UDWORD);
    sBarInit.iRange = getMaxBodyArmour() as UWORD;
    if widgAddBarGraph(psWScreen, &mut sBarInit) == 0 {
        return 1 as libc::c_int
    }
    //body points added AB 3/9/97
	//sBarInit.id = IDDES_BODYPOINTS;
	//sBarInit.y += DES_CLICKBARHEIGHT + DES_CLICKGAP;
	//if (!widgAddBarGraph(psWScreen, &sBarInit))
	//{
	//	return TRUE;
	//}
    sBarInit.id =
        5101 as libc::c_int as UDWORD; //+= DES_CLICKBARHEIGHT + DES_CLICKGAP;
    sBarInit.y =
        (7 as libc::c_int +
             (11 as libc::c_int + 9 as libc::c_int) * 2 as libc::c_int) as
            SWORD; //DBAR_BODYMAXPOWER;
    sBarInit.pTip =
        strresGetString(psStringRes,
                        STR_DES_POWER as libc::c_int as
                            UDWORD); //+= DES_CLICKBARHEIGHT + DES_CLICKGAP;
    sBarInit.iRange = getMaxBodyPower() as UWORD; //DBAR_MAXWEIGHT;
    if widgAddBarGraph(psWScreen, &mut sBarInit) == 0 {
        return 1 as libc::c_int
    }
    sBarInit.id = 5102 as libc::c_int as UDWORD;
    sBarInit.y =
        (7 as libc::c_int +
             (11 as libc::c_int + 9 as libc::c_int) * 3 as libc::c_int) as
            SWORD;
    sBarInit.pTip =
        strresGetString(psStringRes, STR_DES_WEIGHT as libc::c_int as UDWORD);
    sBarInit.iRange = getMaxComponentWeight() as UWORD;
    if widgAddBarGraph(psWScreen, &mut sBarInit) == 0 {
        return 1 as libc::c_int
    }
    /* Add the labels for the Body */
    sLabInit.formID = 5007 as libc::c_int as UDWORD;
    sLabInit.id = 5200 as libc::c_int as UDWORD;
    sLabInit.style = 0 as libc::c_int as UDWORD;
    sLabInit.x = 126 as libc::c_int as SWORD;
    sLabInit.y =
        (7 as libc::c_int - 11 as libc::c_int / 3 as libc::c_int) as SWORD;
    sLabInit.width = 20 as libc::c_int as UWORD;
    sLabInit.height = 11 as libc::c_int as UWORD;
    //	sLabInit.pText = "Armour against Kinetic weapons";
    sLabInit.pTip =
        strresGetString(psStringRes,
                        STR_DES_ARMOUR_KIN as libc::c_int as UDWORD);
    sLabInit.FontID = WFont;
    sLabInit.pDisplay =
        Some(intDisplayImage as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    //just to confuse things even more - the graphics were named incorrectly!
    sLabInit.pUserData =
        IMAGE_DES_ARMOUR_EXPLOSIVE as libc::c_int as
            *mut libc::c_void; //IMAGE_DES_ARMOUR_KINETIC;
    if widgAddLabel(psWScreen, &mut sLabInit) == 0 { return 1 as libc::c_int }
    sLabInit.id = 5228 as libc::c_int as UDWORD;
    sLabInit.y =
        (sLabInit.y as libc::c_int + (11 as libc::c_int + 9 as libc::c_int))
            as SWORD;
    //	sLabInit.pText = "Armour against Heat weapons";
    sLabInit.pTip =
        strresGetString(psStringRes,
                        STR_DES_ARMOUR_HEAT as libc::c_int as
                            UDWORD); //IMAGE_DES_ARMOUR_EXPLOSIVE;
    sLabInit.pDisplay =
        Some(intDisplayImage as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    sLabInit.pUserData =
        IMAGE_DES_ARMOUR_KINETIC as libc::c_int as *mut libc::c_void;
    if widgAddLabel(psWScreen, &mut sLabInit) == 0 { return 1 as libc::c_int }
    //body points added AB 3/9/97
	//sLabInit.id = IDDES_BODYPOINTSLAB;
	//sLabInit.y += DES_CLICKBARHEIGHT + DES_CLICKGAP;
	//sLabInit.pText = "Body Points";
	//sLabInit.pTip = sLabInit.pText;
	//sLabInit.pDisplay = intDisplayImage;
	//sLabInit.pUserData = (void*)IMAGE_DES_BODYPOINTS;
	//if (!widgAddLabel(psWScreen, &sLabInit))
	//{
	//	return TRUE;
	//}
    sLabInit.id = 5201 as libc::c_int as UDWORD;
    sLabInit.y =
        (sLabInit.y as libc::c_int + (11 as libc::c_int + 9 as libc::c_int))
            as SWORD;
    //	sLabInit.pText = "Power";
    sLabInit.pTip =
        strresGetString(psStringRes, STR_DES_POWER as libc::c_int as UDWORD);
    sLabInit.pDisplay =
        Some(intDisplayImage as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    sLabInit.pUserData = IMAGE_DES_POWER as libc::c_int as *mut libc::c_void;
    if widgAddLabel(psWScreen, &mut sLabInit) == 0 { return 1 as libc::c_int }
    sLabInit.id = 5202 as libc::c_int as UDWORD;
    sLabInit.y =
        (sLabInit.y as libc::c_int + (11 as libc::c_int + 9 as libc::c_int))
            as SWORD;
    //	sLabInit.pText = "Weight";
    sLabInit.pTip =
        strresGetString(psStringRes, STR_DES_WEIGHT as libc::c_int as UDWORD);
    sLabInit.pDisplay =
        Some(intDisplayImage as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    sLabInit.pUserData = IMAGE_DES_WEIGHT as libc::c_int as *mut libc::c_void;
    if widgAddLabel(psWScreen, &mut sLabInit) == 0 { return 1 as libc::c_int }
    /* add power/points bar subform */
    memset(&mut sFormInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_FORMINIT>() as libc::c_ulong);
    sFormInit.formID = 5000 as libc::c_int as UDWORD;
    sFormInit.id = 5019 as libc::c_int as UDWORD;
    sFormInit.style = 0 as libc::c_int as UDWORD;
    sFormInit.x = 8 as libc::c_int as SWORD;
    sFormInit.y =
        (25 as libc::c_int + 192 as libc::c_int + 2 as libc::c_int) as SWORD;
    sFormInit.width =
        (315 as libc::c_int - 2 as libc::c_int * 8 as libc::c_int) as UWORD;
    sFormInit.height = 40 as libc::c_int as UWORD;
    sFormInit.pDisplay =
        Some(intDisplayDesignForm as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    if widgAddForm(psWScreen, &mut sFormInit) == 0 { return 0 as libc::c_int }
    /* Set the text colour for the form */
    widgSetColour(psWScreen, 5019 as libc::c_int as UDWORD,
                  WCOL_TEXT as libc::c_int as UDWORD,
                  0 as libc::c_int as UBYTE, 164 as libc::c_int as UBYTE,
                  0 as libc::c_int as UBYTE);
    /* Add the design template power bar and label*/
    sLabInit.formID = 5019 as libc::c_int as UDWORD; //intDisplayStatsBar;
    sLabInit.id = 5229 as libc::c_int as UDWORD; //WBAR_SCALE;
    sLabInit.x = 1 as libc::c_int as SWORD;
    sLabInit.y = 6 as libc::c_int as SWORD;
    sLabInit.pTip =
        strresGetString(psStringRes,
                        STR_DES_TEMPPOWER as libc::c_int as UDWORD);
    sLabInit.pDisplay =
        Some(intDisplayImage as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    sLabInit.pUserData = IMAGE_DES_POWER as libc::c_int as *mut libc::c_void;
    if widgAddLabel(psWScreen, &mut sLabInit) == 0 { return 1 as libc::c_int }
    memset(&mut sBarInit as *mut W_BARINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_BARINIT>() as libc::c_ulong);
    sBarInit.formID = 5019 as libc::c_int as UDWORD;
    sBarInit.id = 5023 as libc::c_int as UDWORD;
    sBarInit.style = 0 as libc::c_int as UDWORD;
    sBarInit.orientation = 0x1 as libc::c_int as UWORD;
    sBarInit.x =
        (1 as libc::c_int + 4 as libc::c_int +
             iV_GetImageWidth(IntImages,
                              IMAGE_DES_BODYPOINTS as libc::c_int as UWORD) as
                 libc::c_int) as SWORD;
    sBarInit.y = 6 as libc::c_int as SWORD;
    sBarInit.width =
        (315 as libc::c_int - 2 as libc::c_int * 8 as libc::c_int -
             15 as libc::c_int -
             iV_GetImageWidth(IntImages,
                              IMAGE_DES_BODYPOINTS as libc::c_int as UWORD) as
                 libc::c_int) as SWORD as UWORD;
    sBarInit.height =
        iV_GetImageHeight(IntImages,
                          IMAGE_DES_POWERBACK as libc::c_int as UWORD);
    sBarInit.pDisplay =
        Some(intDisplayDesignPowerBar as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    sBarInit.pTip =
        strresGetString(psStringRes,
                        STR_DES_TEMPPOWER as libc::c_int as UDWORD);
    sBarInit.iRange = 1000 as libc::c_int as UWORD;
    if widgAddBarGraph(psWScreen, &mut sBarInit) == 0 {
        return 0 as libc::c_int
    }
    /* Add the design template body points bar and label*/
    sLabInit.formID = 5019 as libc::c_int as UDWORD; //intDisplayStatsBar;
    sLabInit.id =
        5230 as libc::c_int as
            UDWORD; //(UWORD)getMaxBodyPoints();//DBAR_BODYMAXPOINTS;
    sLabInit.x = 1 as libc::c_int as SWORD;
    sLabInit.y =
        (6 as libc::c_int + 2 as libc::c_int +
             iV_GetImageHeight(IntImages,
                               IMAGE_DES_BODYPOINTS as libc::c_int as UWORD)
                 as libc::c_int) as SWORD;
    sLabInit.pTip =
        strresGetString(psStringRes,
                        STR_DES_TEMPBODY as libc::c_int as UDWORD);
    sLabInit.pDisplay =
        Some(intDisplayImage as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    sLabInit.pUserData =
        IMAGE_DES_BODYPOINTS as libc::c_int as *mut libc::c_void;
    if widgAddLabel(psWScreen, &mut sLabInit) == 0 { return 1 as libc::c_int }
    memset(&mut sBarInit as *mut W_BARINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_BARINIT>() as libc::c_ulong);
    sBarInit.formID = 5019 as libc::c_int as UDWORD;
    sBarInit.id = 5127 as libc::c_int as UDWORD;
    sBarInit.style = 0 as libc::c_int as UDWORD;
    sBarInit.orientation = 0x1 as libc::c_int as UWORD;
    sBarInit.x =
        (1 as libc::c_int + 4 as libc::c_int +
             iV_GetImageWidth(IntImages,
                              IMAGE_DES_BODYPOINTS as libc::c_int as UWORD) as
                 libc::c_int) as SWORD;
    sBarInit.y =
        (6 as libc::c_int + 2 as libc::c_int + 4 as libc::c_int +
             iV_GetImageHeight(IntImages,
                               IMAGE_DES_BODYPOINTS as libc::c_int as UWORD)
                 as libc::c_int) as SWORD;
    sBarInit.width =
        (315 as libc::c_int - 2 as libc::c_int * 8 as libc::c_int -
             15 as libc::c_int -
             iV_GetImageWidth(IntImages,
                              IMAGE_DES_BODYPOINTS as libc::c_int as UWORD) as
                 libc::c_int) as SWORD as UWORD;
    sBarInit.height =
        iV_GetImageHeight(IntImages,
                          IMAGE_DES_POWERBACK as libc::c_int as UWORD);
    sBarInit.pDisplay =
        Some(intDisplayDesignPowerBar as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    sBarInit.pTip =
        strresGetString(psStringRes,
                        STR_DES_TEMPBODY as libc::c_int as UDWORD);
    sBarInit.iRange = 8400 as libc::c_int as UWORD;
    if widgAddBarGraph(psWScreen, &mut sBarInit) == 0 {
        return 0 as libc::c_int
    }
    /* Add the variable bits of the design screen and set the bar graphs */
    desCompMode = IDES_NOCOMPONENT;
    desSysMode = IDES_NOSYSTEM;
    desPropMode = IDES_NOPROPULSION;
    intSetDesignStats(&mut sCurrDesign);
    intSetBodyPoints(&mut sCurrDesign);
    intSetDesignPower(&mut sCurrDesign);
    intSetDesignMode(IDES_BODY);
    /* hide design and component forms until required */
    if bShowCentreScreen == 0 as libc::c_int {
        widgHide(psWScreen, 5000 as libc::c_int as UDWORD);
    }
    widgHide(psWScreen, 5001 as libc::c_int as UDWORD);
    widgHide(psWScreen, 5021 as libc::c_int as UDWORD);
    return 1 as libc::c_int;
}
/* set up droid templates before going into design screen */
#[no_mangle]
pub unsafe extern "C" fn desSetupDesignTemplates() {
    let mut psTempl: *mut DROID_TEMPLATE = 0 as *mut DROID_TEMPLATE;
    let mut i: UDWORD = 0;
    /* init template list */
    memset(apsTemplateList as *mut libc::c_void, 0 as libc::c_int,
           (::std::mem::size_of::<*mut DROID_TEMPLATE>() as
                libc::c_ulong).wrapping_mul(40 as libc::c_int as
                                                libc::c_uint));
    let ref mut fresh0 = *apsTemplateList.offset(0 as libc::c_int as isize);
    *fresh0 = &mut sDefaultDesignTemplate;
    i = 1 as libc::c_int as UDWORD;
    psTempl = apsDroidTemplates[selectedPlayer as usize];
    while !psTempl.is_null() && i < 40 as libc::c_int as libc::c_uint {
        /* add template to list if not a transporter,
		 * cyborg, person or command droid
		 */
        if (*psTempl).droidType as libc::c_uint !=
               DROID_TRANSPORTER as libc::c_int as libc::c_uint &&
               (*psTempl).droidType as libc::c_uint !=
                   DROID_CYBORG as libc::c_int as libc::c_uint &&
               (*psTempl).droidType as libc::c_uint !=
                   DROID_CYBORG_SUPER as libc::c_int as libc::c_uint &&
               (*psTempl).droidType as libc::c_uint !=
                   DROID_CYBORG_CONSTRUCT as libc::c_int as libc::c_uint &&
               (*psTempl).droidType as libc::c_uint !=
                   DROID_CYBORG_REPAIR as libc::c_int as libc::c_uint &&
               (*psTempl).droidType as libc::c_uint !=
                   DROID_PERSON as libc::c_int as libc::c_uint {
            let ref mut fresh1 = *apsTemplateList.offset(i as isize);
            *fresh1 = psTempl;
            i = i.wrapping_add(1)
        }
        /* next template */
        psTempl = (*psTempl).psNext
    };
}
/* Add the design template form */
unsafe extern "C" fn _intAddTemplateForm(mut psSelected: *mut DROID_TEMPLATE)
 -> BOOL {
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
    let mut numButtons: UDWORD = 0;
    let mut butPerForm: UDWORD = 0;
    let mut i: UDWORD = 0;
    /* Count the number of minor tabs needed for the template form */
    numButtons = 0 as libc::c_int as UDWORD;
    i = 0 as libc::c_int as UDWORD;
    while i < 40 as libc::c_int as libc::c_uint {
        if !(*apsTemplateList.offset(i as isize)).is_null() {
            numButtons = numButtons.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    }
    /* Calculate how many buttons will go on a single form */
    butPerForm =
        ((132 as libc::c_int - 0 as libc::c_int - 2 as libc::c_int) /
             (60 as libc::c_int + 2 as libc::c_int) *
             ((258 as libc::c_int - 0 as libc::c_int - 2 as libc::c_int) /
                  (46 as libc::c_int + 2 as libc::c_int))) as UDWORD;
    /* add a form to place the tabbed form on */
    memset(&mut sFormInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_FORMINIT>() as libc::c_ulong);
    sFormInit.formID = 0 as libc::c_int as UDWORD;
    sFormInit.id = 5020 as libc::c_int as UDWORD;
    sFormInit.style = 0 as libc::c_int as UDWORD;
    sFormInit.x = 23 as libc::c_int as SWORD;
    sFormInit.y =
        (59 as libc::c_int as
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
    sFormInit.width = 132 as libc::c_int as UWORD;
    sFormInit.height = (258 as libc::c_int + 4 as libc::c_int) as UWORD;
    sFormInit.pDisplay =
        Some(intDisplayPlainForm as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    if widgAddForm(psWScreen, &mut sFormInit) == 0 { return 0 as libc::c_int }
    /* Add the design templates form */
    memset(&mut sFormInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_FORMINIT>() as
               libc::c_ulong); //IDDES_FORM;
    sFormInit.formID =
        5020 as libc::c_int as UDWORD; //DES_LEFTFORMX;	//DES_TEMPLX;
    sFormInit.id =
        5002 as libc::c_int as UDWORD; //DES_LEFTFORMY;		//DES_TEMPLY;
    sFormInit.style = 1 as libc::c_int as UDWORD; //DES_TEMPLWIDTH;
    sFormInit.x = 2 as libc::c_int as SWORD; //DES_TEMPLHEIGHT;
    sFormInit.y = 2 as libc::c_int as SWORD; //(DES_TAB_HEIGHT/2)+2;
    sFormInit.width = 132 as libc::c_int as UWORD; //intDisplayTemplateForm;
    sFormInit.height = 258 as libc::c_int as UWORD;
    sFormInit.numMajor = numForms(numButtons, butPerForm);
    sFormInit.majorPos = 1 as libc::c_int as UWORD;
    sFormInit.minorPos = 0 as libc::c_int as UWORD;
    sFormInit.majorSize = 26 as libc::c_int as UWORD;
    sFormInit.majorOffset = 2 as libc::c_int as SWORD;
    sFormInit.tabVertOffset = (11 as libc::c_int / 2 as libc::c_int) as SWORD;
    sFormInit.tabMajorThickness = 11 as libc::c_int as UWORD;
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
    /* Put the buttons on it */
    if intAddTemplateButtons(5002 as libc::c_int as UDWORD,
                             (132 as libc::c_int - 0 as libc::c_int) as
                                 UDWORD,
                             (258 as libc::c_int - 0 as libc::c_int) as
                                 UDWORD, 60 as libc::c_int as UDWORD,
                             46 as libc::c_int as UDWORD,
                             2 as libc::c_int as UDWORD, psSelected) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
/* Add the droid template buttons to a form */
/* Add the droid template buttons to a form */
#[no_mangle]
pub unsafe extern "C" fn intAddTemplateButtons(mut formID: UDWORD,
                                               mut formWidth: UDWORD,
                                               mut formHeight: UDWORD,
                                               mut butWidth: UDWORD,
                                               mut butHeight: UDWORD,
                                               mut gap: UDWORD,
                                               mut psSelected:
                                                   *mut DROID_TEMPLATE)
 -> BOOL {
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
    let mut psTempl: *mut DROID_TEMPLATE = 0 as *mut DROID_TEMPLATE;
    let mut aButText: [STRING; 6] = [0; 6];
    let mut BufferID: SDWORD = 0;
    let mut i: UDWORD = 0;
    let mut TempString: [libc::c_char; 256] = [0; 256];
    let mut BufferPos: libc::c_int = 0 as libc::c_int;
    ClearStatBuffers();
    memset(aButText.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           (5 as libc::c_int + 1 as libc::c_int) as libc::c_uint);
    memset(&mut sButInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_BUTINIT>() as libc::c_ulong);
    /* Set up the button struct */
    sButInit.formID = formID; //DES_TABBUTWIDTH;
    sButInit.id = 5300 as libc::c_int as UDWORD; //DES_TABBUTHEIGHT;
    sButInit.style = 4 as libc::c_int as UDWORD;
    sButInit.x = 2 as libc::c_int as SWORD;
    sButInit.y = 2 as libc::c_int as SWORD;
    sButInit.width = 60 as libc::c_int as UWORD;
    sButInit.height = 46 as libc::c_int as UWORD;
    /* Add each button */
    memset(&mut sBarInit as *mut W_BARINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_BARINIT>() as libc::c_ulong);
    sBarInit.id = 5400 as libc::c_int as UDWORD;
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
    sBarInit.pTip =
        strresGetString(psStringRes,
                        STR_DES_POWERUSE as libc::c_int as UDWORD);
    droidTemplID = 0 as libc::c_int as UDWORD;
    i = 0 as libc::c_int as UDWORD;
    while i < 40 as libc::c_int as libc::c_uint {
        if !(*apsTemplateList.offset(i as isize)).is_null() {
            psTempl = *apsTemplateList.offset(i as isize);
            /* Set the tip and add the button */
            // On the playstation the tips are additionaly setup when they are displayed ... because we only have one text name buffer
            strncpy(aButText.as_mut_ptr(), getTemplateName(psTempl),
                    5 as libc::c_int as libc::c_uint);
            sButInit.pTip = getTemplateName(psTempl);
            BufferID = GetStatBuffer();
            if BufferID >= 0 as libc::c_int {
            } else {
                debug(LOG_ERROR,
                      b"Unable to aquire stat buffer.\x00" as *const u8 as
                          *const libc::c_char);
            };
            if BufferID >= 0 as libc::c_int {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"design.c\x00" as *const u8 as *const libc::c_char,
                      1069 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 22],
                                                &[libc::c_char; 22]>(b"intAddTemplateButtons\x00")).as_ptr(),
                      b"BufferID >= 0\x00" as *const u8 as
                          *const libc::c_char);
            };
            StatBuffers[BufferID as usize].InUse = 1 as libc::c_int;
            StatBuffers[BufferID as usize].Data =
                psTempl as *mut libc::c_void;
            sButInit.pUserData =
                &mut *StatBuffers.as_mut_ptr().offset(BufferID as isize) as
                    *mut RENDERED_BUTTON as *mut libc::c_void;
            sButInit.pDisplay =
                Some(intDisplayTemplateButton as
                         unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                              _: UDWORD, _: *mut UDWORD)
                             -> ());
            if widgAddForm(psWScreen, &mut sButInit) == 0 {
                return 0 as libc::c_int
            }
            sBarInit.iRange = 5 as libc::c_int as UWORD;
            sBarInit.size =
                (*psTempl).powerPoints.wrapping_div(5 as libc::c_int as
                                                        libc::c_uint) as
                    UWORD;
            if sBarInit.size as libc::c_int > 100 as libc::c_int {
                sBarInit.size = 100 as libc::c_int as UWORD
            }
            sprintf(TempString.as_mut_ptr(),
                    b"%s - %d\x00" as *const u8 as *const libc::c_char,
                    strresGetString(psStringRes,
                                    STR_DES_POWERUSE as libc::c_int as
                                        UDWORD), (*psTempl).powerPoints);
            if (BufferPos as
                    libc::c_uint).wrapping_add(strlen(TempString.as_mut_ptr())).wrapping_add(1
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_uint)
                   < (32 as libc::c_int * 60 as libc::c_int) as libc::c_uint {
            } else {
                debug(LOG_ERROR,
                      b"String Buffer Overrun\x00" as *const u8 as
                          *const libc::c_char);
            };
            if (BufferPos as
                    libc::c_uint).wrapping_add(strlen(TempString.as_mut_ptr())).wrapping_add(1
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_uint)
                   < (32 as libc::c_int * 60 as libc::c_int) as libc::c_uint {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"design.c\x00" as *const u8 as *const libc::c_char,
                      1085 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 22],
                                                &[libc::c_char; 22]>(b"intAddTemplateButtons\x00")).as_ptr(),
                      b"BufferPos+strlen(TempString)+1 < STRING_BUFFER_SIZE\x00"
                          as *const u8 as *const libc::c_char);
            };
            strcpy(&mut *StringBuffer.as_mut_ptr().offset(BufferPos as isize),
                   TempString.as_mut_ptr());
            sBarInit.pTip =
                &mut *StringBuffer.as_mut_ptr().offset(BufferPos as isize) as
                    *mut libc::c_char;
            BufferPos =
                (BufferPos as
                     libc::c_uint).wrapping_add(strlen(TempString.as_mut_ptr()).wrapping_add(1
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_uint))
                    as libc::c_int as libc::c_int;
            sBarInit.formID = sButInit.id;
            if widgAddBarGraph(psWScreen, &mut sBarInit) == 0 {
                return 0 as libc::c_int
            }
            /* if the current template matches psSelected lock the button */
            if psTempl == psSelected {
                droidTemplID = sButInit.id;
                widgSetButtonState(psWScreen, droidTemplID,
                                   0x2 as libc::c_int as UDWORD);
                widgSetTabs(psWScreen, 5002 as libc::c_int as UDWORD,
                            sButInit.majorID, 0 as libc::c_int as UWORD);
            }
            /* Update the init struct for the next button */
            sBarInit.id =
                (sBarInit.id as
                     libc::c_uint).wrapping_add(1 as libc::c_int as
                                                    libc::c_uint) as UDWORD as
                    UDWORD;
            sButInit.id =
                (sButInit.id as
                     libc::c_uint).wrapping_add(1 as libc::c_int as
                                                    libc::c_uint) as UDWORD as
                    UDWORD;
            sButInit.x =
                (sButInit.x as
                     libc::c_uint).wrapping_add(butWidth).wrapping_add(gap) as
                    SWORD;
            if (sButInit.x as
                    libc::c_uint).wrapping_add(butWidth).wrapping_add(gap) >
                   formWidth {
                sButInit.x = 2 as libc::c_int as SWORD;
                sButInit.y =
                    (sButInit.y as
                         libc::c_uint).wrapping_add(butHeight).wrapping_add(gap)
                        as SWORD
            }
            if (sButInit.y as
                    libc::c_uint).wrapping_add(butHeight).wrapping_add(gap) >
                   formHeight {
                sButInit.y = 2 as libc::c_int as SWORD;
                sButInit.majorID =
                    (sButInit.majorID as libc::c_int + 1 as libc::c_int) as
                        UWORD
            }
            //check don't go over max templates that can fit on the form
            if sButInit.id >= 5339 as libc::c_int as libc::c_uint { break ; }
        }
        i = i.wrapping_add(1)
    }
    return 1 as libc::c_int;
}
/* Set the current mode of the design screen, and display the appropriate component lists */
/*static UDWORD MaxComponents(UDWORD NumStats)
{
	if(NumStats > MAX_DESIGN_COMPONENTS) {
		return MAX_DESIGN_COMPONENTS;
	}

	return NumStats;
}*/
/* Set the current mode of the design screen, and display the appropriate
 * component lists
 */
unsafe extern "C" fn intSetDesignMode(mut newCompMode: DES_COMPMODE) {
    let mut weaponIndex: UDWORD = 0;
    //	UBYTE	aCmdAvailable[MAX_CMDDROIDS];
//	SDWORD	i;
	//UDWORD	NumComponents;
	//UDWORD	NumSensors;
	//UDWORD	NumECMs;
	//UDWORD	NumConstructs;
	//UDWORD	NumWeapons;
    if newCompMode as libc::c_uint != desCompMode as libc::c_uint {
        /* Have to change the component display - remove the old one */
        match desCompMode as libc::c_uint {
            0 | 1 => {
                /*
			widgDelete(psWScreen, IDDES_COMPFORM);
			widgDelete(psWScreen, IDDES_RIGHTBASE);
			widgSetButtonState(psWScreen, IDDES_SYSTEMFORM, 0);
*/
                widgDelete(psWScreen, 5003 as libc::c_int as UDWORD);
                widgDelete(psWScreen, 5021 as libc::c_int as UDWORD);
                widgSetButtonState(psWScreen, 5006 as libc::c_int as UDWORD,
                                   0 as libc::c_int as UDWORD);
            }
            2 => {
                widgDelete(psWScreen, 5003 as libc::c_int as UDWORD);
                widgDelete(psWScreen, 5021 as libc::c_int as UDWORD);
                widgSetButtonState(psWScreen, 5006 as libc::c_int as UDWORD,
                                   0 as libc::c_int as UDWORD);
            }
            3 => {
                widgDelete(psWScreen, 5003 as libc::c_int as UDWORD);
                widgDelete(psWScreen, 5021 as libc::c_int as UDWORD);
                widgSetButtonState(psWScreen, 5007 as libc::c_int as UDWORD,
                                   0 as libc::c_int as UDWORD);
            }
            4 => {
                widgDelete(psWScreen, 5003 as libc::c_int as UDWORD);
                widgDelete(psWScreen, 5021 as libc::c_int as UDWORD);
                widgSetButtonState(psWScreen, 5008 as libc::c_int as UDWORD,
                                   0 as libc::c_int as UDWORD);
            }
            5 | _ => { }
        }
        /* Set up the display for the new mode */
        desCompMode = newCompMode;
        match desCompMode as libc::c_uint {
            0 | 1 => {
                // initialise the available array
/*			aCmdAvailable[0] = 0;
			for(i=1; i<MAX_CMDDROIDS; i++)
			{
				if (i<=numCommandDroids[selectedPlayer])
				{
					aCmdAvailable[i] = AVAILABLE;
				}
				else
				{
					aCmdAvailable[i] = 0;
				}
			}
			intAddComponentForm(numCommandDroids[selectedPlayer]);
			intAddComponentButtons((COMP_BASE_STATS *)asCommandDroids[selectedPlayer],
								   sizeof(COMMAND_DROID),
								   aCmdAvailable,MAX_CMDDROIDS,
								   sCurrDesign.asParts[COMP_BRAIN],TAB_USEMAJOR);
			intAddSystemButtons(IDES_BRAIN);
			widgSetButtonState(psWScreen, IDDES_SYSTEMFORM, WBUT_LOCK);*/
                intAddComponentForm(intNumAvailable(apCompLists[selectedPlayer
                                                                    as
                                                                    usize][COMP_SENSOR
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               usize],
                                                    numSensorStats,
                                                    asSensorStats as
                                                        *mut COMP_BASE_STATS,
                                                    ::std::mem::size_of::<SENSOR_STATS>()
                                                        as
                                                        libc::c_ulong).wrapping_add(intNumAvailable(apCompLists[selectedPlayer
                                                                                                                    as
                                                                                                                    usize][COMP_ECM
                                                                                                                               as
                                                                                                                               libc::c_int
                                                                                                                               as
                                                                                                                               usize],
                                                                                                    numECMStats,
                                                                                                    asECMStats
                                                                                                        as
                                                                                                        *mut COMP_BASE_STATS,
                                                                                                    ::std::mem::size_of::<ECM_STATS>()
                                                                                                        as
                                                                                                        libc::c_ulong)).wrapping_add(intNumAvailable(apCompLists[selectedPlayer
                                                                                                                                                                     as
                                                                                                                                                                     usize][COMP_BRAIN
                                                                                                                                                                                as
                                                                                                                                                                                libc::c_int
                                                                                                                                                                                as
                                                                                                                                                                                usize],
                                                                                                                                                     numBrainStats,
                                                                                                                                                     asBrainStats
                                                                                                                                                         as
                                                                                                                                                         *mut COMP_BASE_STATS,
                                                                                                                                                     ::std::mem::size_of::<BRAIN_STATS>()
                                                                                                                                                         as
                                                                                                                                                         libc::c_ulong)).wrapping_add(intNumAvailable(apCompLists[selectedPlayer
                                                                                                                                                                                                                      as
                                                                                                                                                                                                                      usize][COMP_CONSTRUCT
                                                                                                                                                                                                                                 as
                                                                                                                                                                                                                                 libc::c_int
                                                                                                                                                                                                                                 as
                                                                                                                                                                                                                                 usize],
                                                                                                                                                                                                      numConstructStats,
                                                                                                                                                                                                      asConstructStats
                                                                                                                                                                                                          as
                                                                                                                                                                                                          *mut COMP_BASE_STATS,
                                                                                                                                                                                                      ::std::mem::size_of::<CONSTRUCT_STATS>()
                                                                                                                                                                                                          as
                                                                                                                                                                                                          libc::c_ulong)).wrapping_add(intNumAvailable(apCompLists[selectedPlayer
                                                                                                                                                                                                                                                                       as
                                                                                                                                                                                                                                                                       usize][COMP_REPAIRUNIT
                                                                                                                                                                                                                                                                                  as
                                                                                                                                                                                                                                                                                  libc::c_int
                                                                                                                                                                                                                                                                                  as
                                                                                                                                                                                                                                                                                  usize],
                                                                                                                                                                                                                                                       numRepairStats,
                                                                                                                                                                                                                                                       asRepairStats
                                                                                                                                                                                                                                                           as
                                                                                                                                                                                                                                                           *mut COMP_BASE_STATS,
                                                                                                                                                                                                                                                       ::std::mem::size_of::<REPAIR_STATS>()
                                                                                                                                                                                                                                                           as
                                                                                                                                                                                                                                                           libc::c_ulong)));
                intAddExtraSystemButtons(sCurrDesign.asParts[COMP_SENSOR as
                                                                 libc::c_int
                                                                 as usize] as
                                             UDWORD,
                                         sCurrDesign.asParts[COMP_ECM as
                                                                 libc::c_int
                                                                 as usize] as
                                             UDWORD,
                                         sCurrDesign.asParts[COMP_CONSTRUCT as
                                                                 libc::c_int
                                                                 as usize] as
                                             UDWORD,
                                         sCurrDesign.asParts[COMP_REPAIRUNIT
                                                                 as
                                                                 libc::c_int
                                                                 as usize] as
                                             UDWORD,
                                         sCurrDesign.asParts[COMP_BRAIN as
                                                                 libc::c_int
                                                                 as usize] as
                                             UDWORD);
                intAddSystemButtons(IDES_SYSTEM as libc::c_int);
                widgSetButtonState(psWScreen, 5006 as libc::c_int as UDWORD,
                                   0x2 as libc::c_int as UDWORD);
            }
            2 => {
                intAddComponentForm(intNumAvailable(apCompLists[selectedPlayer
                                                                    as
                                                                    usize][COMP_WEAPON
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               usize],
                                                    numWeaponStats,
                                                    asWeaponStats as
                                                        *mut COMP_BASE_STATS,
                                                    ::std::mem::size_of::<WEAPON_STATS>()
                                                        as libc::c_ulong));
                weaponIndex =
                    if sCurrDesign.numWeaps > 0 as libc::c_int as libc::c_uint
                       {
                        sCurrDesign.asWeaps[0 as libc::c_int as usize]
                    } else { 0 as libc::c_int as libc::c_uint };
                intAddComponentButtons(asWeaponStats as *mut COMP_BASE_STATS,
                                       ::std::mem::size_of::<WEAPON_STATS>()
                                           as libc::c_ulong,
                                       apCompLists[selectedPlayer as
                                                       usize][COMP_WEAPON as
                                                                  libc::c_int
                                                                  as usize],
                                       numWeaponStats, weaponIndex,
                                       0 as libc::c_int as UDWORD);
                intAddSystemButtons(IDES_TURRET as libc::c_int);
                widgSetButtonState(psWScreen, 5006 as libc::c_int as UDWORD,
                                   0x2 as libc::c_int as UDWORD);
            }
            3 => {
                //NumSensors = MaxComponents(numSensorStats);
			//NumECMs = MaxComponents(numECMStats);
			//NumConstructs = MaxComponents(numConstructStats);
			//NumWeapons = MaxComponents(numWeaponStats);
/*			intAddMultiComponentForm(apCompLists[selectedPlayer][COMP_SENSOR],
									 numSensorStats,//NumSensors,
									 apCompLists[selectedPlayer][COMP_ECM],
									 numECMStats,//NumECMs,
									 apCompLists[selectedPlayer][COMP_CONSTRUCT],
									 numConstructStats,//NumConstructs,
									 apCompLists[selectedPlayer][COMP_WEAPON],
									 numWeaponStats);//NumWeapons);
			if (sCurrDesign.numWeaps > 0)
			{
				weaponIndex = sCurrDesign.asWeaps[0];
			}
			else
			{
				weaponIndex = 0;
			}
			intAddComponentButtons((COMP_BASE_STATS *)asWeaponStats,
								   sizeof(WEAPON_STATS),
								   apCompLists[selectedPlayer][COMP_WEAPON],
								   //NumWeapons, weaponIndex,TAB_USEMINOR);
								   numWeaponStats, weaponIndex,TAB_USEMINOR);
			intAddExtraSystemButtons(sCurrDesign.asParts[COMP_SENSOR],
									 sCurrDesign.asParts[COMP_ECM],
									 sCurrDesign.asParts[COMP_CONSTRUCT],
									 sCurrDesign.asParts[COMP_REPAIRUNIT]);
			widgSetButtonState(psWScreen, IDDES_SYSTEMFORM, WBUT_LOCK);
			break;*/
                /*NumComponents = numBodyStats;
			if(NumComponents > MAX_DESIGN_COMPONENTS) {
				NumComponents = MAX_DESIGN_COMPONENTS;
			}*/
                intAddComponentForm(intNumAvailable(apCompLists[selectedPlayer
                                                                    as
                                                                    usize][COMP_BODY
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               usize],
                                                    numBodyStats,
                                                    asBodyStats as
                                                        *mut COMP_BASE_STATS,
                                                    ::std::mem::size_of::<BODY_STATS>()
                                                        as libc::c_ulong));
                intAddComponentButtons(asBodyStats as *mut COMP_BASE_STATS,
                                       ::std::mem::size_of::<BODY_STATS>() as
                                           libc::c_ulong,
                                       apCompLists[selectedPlayer as
                                                       usize][COMP_BODY as
                                                                  libc::c_int
                                                                  as usize],
                                       numBodyStats,
                                       sCurrDesign.asParts[COMP_BODY as
                                                               libc::c_int as
                                                               usize] as
                                           UDWORD,
                                       0 as libc::c_int as UDWORD);
                widgSetButtonState(psWScreen, 5007 as libc::c_int as UDWORD,
                                   0x2 as libc::c_int as UDWORD);
            }
            4 => {
                /*NumComponents = numPropulsionStats;
			if(NumComponents > MAX_DESIGN_COMPONENTS) {
				NumComponents = MAX_DESIGN_COMPONENTS;
			}*/
                intAddComponentForm(intNumAvailable(apCompLists[selectedPlayer
                                                                    as
                                                                    usize][COMP_PROPULSION
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               usize],
                                                    numPropulsionStats,
                                                    asPropulsionStats as
                                                        *mut COMP_BASE_STATS,
                                                    ::std::mem::size_of::<PROPULSION_STATS>()
                                                        as libc::c_ulong));
                intAddComponentButtons(asPropulsionStats as
                                           *mut COMP_BASE_STATS,
                                       ::std::mem::size_of::<PROPULSION_STATS>()
                                           as libc::c_ulong,
                                       apCompLists[selectedPlayer as
                                                       usize][COMP_PROPULSION
                                                                  as
                                                                  libc::c_int
                                                                  as usize],
                                       numPropulsionStats,
                                       sCurrDesign.asParts[COMP_PROPULSION as
                                                               libc::c_int as
                                                               usize] as
                                           UDWORD,
                                       0 as libc::c_int as UDWORD);
                widgSetButtonState(psWScreen, 5008 as libc::c_int as UDWORD,
                                   0x2 as libc::c_int as UDWORD);
            }
            5 | _ => { }
        }
    };
}
unsafe extern "C" fn intChooseSystemStats(mut psTemplate: *mut DROID_TEMPLATE)
 -> *mut COMP_BASE_STATS {
    let mut psStats: *mut COMP_BASE_STATS = 0 as *mut COMP_BASE_STATS;
    // Check for a command droid
/*	if (psTemplate->asParts[COMP_BRAIN] != 0)
	{
		return (COMP_BASE_STATS *)(asCommandDroids[selectedPlayer] +
								psTemplate->asParts[COMP_BRAIN]);
	}*/
    // Choose correct system stats
    match droidTemplateType(psTemplate) as libc::c_uint {
        7 => {
            //		psStats = (COMP_BASE_STATS *)(asCommandDroids[selectedPlayer] +
//								psTemplate->asParts[COMP_BRAIN]);
            psStats =
                asBrainStats.offset((*psTemplate).asParts[COMP_BRAIN as
                                                              libc::c_int as
                                                              usize] as isize)
                    as *mut COMP_BASE_STATS
        }
        1 => {
            psStats =
                asSensorStats.offset((*psTemplate).asParts[COMP_SENSOR as
                                                               libc::c_int as
                                                               usize] as
                                         isize) as *mut COMP_BASE_STATS
        }
        2 => {
            psStats =
                asECMStats.offset((*psTemplate).asParts[COMP_ECM as
                                                            libc::c_int as
                                                            usize] as isize)
                    as *mut COMP_BASE_STATS
        }
        3 | 10 => {
            psStats =
                asConstructStats.offset((*psTemplate).asParts[COMP_CONSTRUCT
                                                                  as
                                                                  libc::c_int
                                                                  as usize] as
                                            isize) as *mut COMP_BASE_STATS
        }
        8 | 11 => {
            psStats =
                asRepairStats.offset((*psTemplate).asParts[COMP_REPAIRUNIT as
                                                               libc::c_int as
                                                               usize] as
                                         isize) as *mut COMP_BASE_STATS
        }
        0 | 4 | 5 | 12 | 9 => {
            psStats =
                asWeaponStats.offset((*psTemplate).asWeaps[0 as libc::c_int as
                                                               usize] as
                                         isize) as *mut COMP_BASE_STATS
        }
        _ => {
            debug(LOG_ERROR,
                  b"intSetDesignStats: unrecognised droid type\x00" as
                      *const u8 as *const libc::c_char);
            abort();
        }
    }
    return psStats;
}
/* set SHOWTEMPLATENAME to 0 to show template components in edit box */
/*
	Go through the template for player 0 matching up all the
	components from SourceTemplate


	NULL is returned if no match is found

	= Matches Body,Proulsion & weapon

	- This is used for generating cyborg names
*/
#[no_mangle]
pub unsafe extern "C" fn MatchTemplate(mut SourceTemplate:
                                           *mut DROID_TEMPLATE,
                                       mut player: UDWORD)
 -> *mut DROID_TEMPLATE {
    let mut pDroidDesign: *mut DROID_TEMPLATE = 0 as *mut DROID_TEMPLATE;
    pDroidDesign = apsDroidTemplates[player as usize];
    while !pDroidDesign.is_null() {
        if (*pDroidDesign).asParts[COMP_BODY as libc::c_int as usize] ==
               (*SourceTemplate).asParts[COMP_BODY as libc::c_int as usize] {
            if (*pDroidDesign).asParts[COMP_PROPULSION as libc::c_int as
                                           usize] ==
                   (*SourceTemplate).asParts[COMP_PROPULSION as libc::c_int as
                                                 usize] {
                if (*pDroidDesign).numWeaps != 0 {
                    if (*pDroidDesign).asWeaps[0 as libc::c_int as usize] ==
                           (*SourceTemplate).asWeaps[0 as libc::c_int as
                                                         usize] {
                        return pDroidDesign
                    }
                }
            }
        }
        pDroidDesign = (*pDroidDesign).psNext
    }
    return 0 as *mut DROID_TEMPLATE;
}
#[no_mangle]
pub unsafe extern "C" fn GetDefaultTemplateName(mut psTemplate:
                                                    *mut DROID_TEMPLATE)
 -> *mut STRING {
    let mut psStats: *mut COMP_BASE_STATS = 0 as *mut COMP_BASE_STATS;
    let mut pStr: *mut STRING = 0 as *mut STRING;
    /*
		First we check for the special cases of the Transporter & Cyborgs
	*/
    if (*psTemplate).droidType as libc::c_uint ==
           DROID_TRANSPORTER as libc::c_int as libc::c_uint {
        return strresGetString(0 as *mut STR_RES,
                               HashString(b"Transporter\x00" as *const u8 as
                                              *const libc::c_char as
                                              *mut libc::c_char))
    }
    /*
		Now get the normal default droid name based on its components

	*/
    aCurrName[0 as libc::c_int as usize] =
        0 as libc::c_int as STRING; // Reset string to null
    psStats = intChooseSystemStats(psTemplate);
    if (*psTemplate).asWeaps[0 as libc::c_int as usize] !=
           0 as libc::c_int as libc::c_uint ||
           (*psTemplate).asParts[COMP_CONSTRUCT as libc::c_int as usize] !=
               0 as libc::c_int ||
           (*psTemplate).asParts[COMP_SENSOR as libc::c_int as usize] !=
               0 as libc::c_int ||
           (*psTemplate).asParts[COMP_ECM as libc::c_int as usize] !=
               0 as libc::c_int ||
           (*psTemplate).asParts[COMP_REPAIRUNIT as libc::c_int as usize] !=
               0 as libc::c_int ||
           (*psTemplate).asParts[COMP_BRAIN as libc::c_int as usize] !=
               0 as libc::c_int {
        pStr = getStatName(psStats as *mut libc::c_void);
        strcpy(aCurrName.as_mut_ptr(), pStr);
        strcat(aCurrName.as_mut_ptr(),
               b" \x00" as *const u8 as *const libc::c_char);
        //		DBPRINTF(("%s",aCurrName));
//	DBPRINTF(("a) templ=%p stat=%p name=%s\n",psTemplate,psStats,pStr);
    }
    psStats =
        asBodyStats.offset((*psTemplate).asParts[COMP_BODY as libc::c_int as
                                                     usize] as isize) as
            *mut COMP_BASE_STATS;
    if (*psTemplate).asParts[COMP_BODY as libc::c_int as usize] !=
           0 as libc::c_int {
        pStr = getStatName(psStats as *mut libc::c_void);
        if strlen(aCurrName.as_mut_ptr()).wrapping_add(strlen(pStr)) >
               80 as libc::c_int as libc::c_uint {
            debug(LOG_NEVER,
                  b"GetDefaultTemplateName: name string too long %s+%s\n\x00"
                      as *const u8 as *const libc::c_char,
                  aCurrName.as_mut_ptr(), pStr);
            return 0 as *mut STRING
        }
        strcat(aCurrName.as_mut_ptr(), pStr);
        strcat(aCurrName.as_mut_ptr(),
               b" \x00" as *const u8 as *const libc::c_char);
        //		DBPRINTF(("b) templ=%p stat=%p name=%s\n",psTemplate,psStats,pStr);
    }
    psStats =
        asPropulsionStats.offset((*psTemplate).asParts[COMP_PROPULSION as
                                                           libc::c_int as
                                                           usize] as isize) as
            *mut COMP_BASE_STATS;
    if (*psTemplate).asParts[COMP_PROPULSION as libc::c_int as usize] !=
           0 as libc::c_int {
        pStr = getStatName(psStats as *mut libc::c_void);
        if strlen(aCurrName.as_mut_ptr()).wrapping_add(strlen(pStr)) >
               80 as libc::c_int as libc::c_uint {
            debug(LOG_NEVER,
                  b"GetDefaultTemplateName: name string too long %s+%s\n\x00"
                      as *const u8 as *const libc::c_char,
                  aCurrName.as_mut_ptr(), pStr);
            return 0 as *mut STRING
        }
        strcat(aCurrName.as_mut_ptr(), pStr);
        //		DBPRINTF(("c) templ=%p stat=%p name=%s\n",psTemplate,psStats,pStr);
    }
    //	DBPRINTF(("TEMPLATE NAME [%s]\n",aCurrName);
    return aCurrName.as_mut_ptr();
}
unsafe extern "C" fn intSetEditBoxTextFromTemplate(mut psTemplate:
                                                       *mut DROID_TEMPLATE) {
    strcpy(aCurrName.as_mut_ptr(),
           b"\x00" as *const u8 as *const libc::c_char);
    /* show component names if default template else show stat name */
    if (*psTemplate).droidType as libc::c_uint !=
           DROID_DEFAULT as libc::c_int as libc::c_uint {
        strcpy(aCurrName.as_mut_ptr(), getTemplateName(psTemplate));
    } else { GetDefaultTemplateName(psTemplate); }
    widgSetString(psWScreen, 5013 as libc::c_int as UDWORD,
                  aCurrName.as_mut_ptr());
}
/* Set all the design bar graphs from a design template */
/* Set all the design bar graphs from a design template */
unsafe extern "C" fn intSetDesignStats(mut psTemplate: *mut DROID_TEMPLATE) {
    let mut psStats: *mut COMP_BASE_STATS = intChooseSystemStats(psTemplate);
    /* Set system stats */
    intSetSystemForm(psStats);
    /* Set the body stats */
    intSetBodyStats(asBodyStats.offset((*psTemplate).asParts[COMP_BODY as
                                                                 libc::c_int
                                                                 as usize] as
                                           isize));
    /* Set the propulsion stats */
    intSetPropulsionForm(asPropulsionStats.offset((*psTemplate).asParts[COMP_PROPULSION
                                                                            as
                                                                            libc::c_int
                                                                            as
                                                                            usize]
                                                      as isize));
    /* Set the name in the edit box */
    intSetEditBoxTextFromTemplate(psTemplate);
}
/* Set up the system clickable form of the design screen given a set of stats */
unsafe extern "C" fn _intSetSystemForm(mut psStats: *mut COMP_BASE_STATS)
 -> BOOL {
    let mut psSensor: *mut SENSOR_STATS = 0 as *mut SENSOR_STATS;
    let mut psECM: *mut ECM_STATS = 0 as *mut ECM_STATS;
    let mut psConst: *mut CONSTRUCT_STATS = 0 as *mut CONSTRUCT_STATS;
    let mut psWeapon: *mut WEAPON_STATS = 0 as *mut WEAPON_STATS;
    let mut psRepair: *mut REPAIR_STATS = 0 as *mut REPAIR_STATS;
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
    //	W_LABINIT		sTitleInit;
    let mut newSysMode: DES_SYSMODE = IDES_SENSOR;
    memset(&mut sFormInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_FORMINIT>() as libc::c_ulong);
    memset(&mut sLabInit as *mut W_LABINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_LABINIT>() as libc::c_ulong);
    //	memset(&sTitleInit, 0, sizeof(W_LABINIT));
    memset(&mut sBarInit as *mut W_BARINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_BARINIT>() as libc::c_ulong);
    /* Figure out what the new mode should be */
    match statType((*psStats).ref_0) {
        8 => { newSysMode = IDES_WEAPON }
        6 => { newSysMode = IDES_SENSOR }
        5 => { newSysMode = IDES_ECM }
        7 => { newSysMode = IDES_CONSTRUCT }
        2 => { newSysMode = IDES_COMMAND }
        4 => { newSysMode = IDES_REPAIR }
        _ => { }
    }
    /* If the correct form is already displayed just set the stats */
    if newSysMode as libc::c_uint == desSysMode as libc::c_uint {
        intSetSystemStats(psStats);
        return 1 as libc::c_int
    }
    /* Remove the old form if necessary */
    if desSysMode as libc::c_uint !=
           IDES_NOSYSTEM as libc::c_int as libc::c_uint {
        widgDelete(psWScreen, 5006 as libc::c_int as UDWORD);
    }
    /* Set the new mode */
    desSysMode = newSysMode;
    /* Add the system form */
    sFormInit.formID = 5001 as libc::c_int as UDWORD; //COMPBUTWIDTH;
    sFormInit.id = 5006 as libc::c_int as UDWORD; //COMPBUTHEIGHT;
    sFormInit.style =
        (4 as libc::c_int | 8 as libc::c_int) as
            UDWORD; /* set form tip to stats string */
    sFormInit.x = 6 as libc::c_int as SWORD; /* store component stats */
    sFormInit.y = 6 as libc::c_int as SWORD;
    sFormInit.width = 300 as libc::c_int as UWORD;
    sFormInit.height = 85 as libc::c_int as UWORD;
    sFormInit.pTip = getStatName(psStats as *mut libc::c_void);
    sFormInit.pUserData = psStats as *mut libc::c_void;
    sFormInit.pDisplay =
        Some(intDisplayStatForm as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    if widgAddForm(psWScreen, &mut sFormInit) == 0 { return 0 as libc::c_int }
    /* Initialise the bargraph struct */
    sBarInit.formID = 5006 as libc::c_int as UDWORD; //WBAR_DOUBLE;
    sBarInit.style = 0 as libc::c_int as UDWORD; //DES_CLICKBARY;
    sBarInit.orientation = 0x1 as libc::c_int as UWORD;
    sBarInit.x = 154 as libc::c_int as SWORD;
    sBarInit.y = 7 as libc::c_int as SWORD;
    sBarInit.width = 140 as libc::c_int as UWORD;
    sBarInit.height = 11 as libc::c_int as UWORD;
    sBarInit.sCol.red = 255 as libc::c_int as UBYTE;
    sBarInit.sCol.green = 235 as libc::c_int as UBYTE;
    sBarInit.sCol.blue = 19 as libc::c_int as UBYTE;
    sBarInit.sMinorCol.red = 0x55 as libc::c_int as UBYTE;
    sBarInit.sMinorCol.green = 0 as libc::c_int as UBYTE;
    sBarInit.sMinorCol.blue = 0 as libc::c_int as UBYTE;
    sBarInit.pDisplay =
        Some(intDisplayStatsBar as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    /* Initialise the label struct */
    sLabInit.formID = 5006 as libc::c_int as UDWORD;
    sLabInit.style = 0 as libc::c_int as UDWORD;
    sLabInit.x = 126 as libc::c_int as SWORD;
    sLabInit.y =
        (7 as libc::c_int - 11 as libc::c_int / 3 as libc::c_int) as SWORD;
    sLabInit.width = 20 as libc::c_int as UWORD;
    sLabInit.height = 11 as libc::c_int as UWORD;
    sLabInit.FontID = WFont;
    //	/* Initialise the title label struct */
//	sTitleInit.formID = IDDES_SYSTEMFORM;
//	sTitleInit.style = WLAB_ALIGNCENTRE;
//	sTitleInit.x = DES_CLICKGAP;
//	sTitleInit.y = DES_CLICKGAP;
//	sTitleInit.width = DES_COMPBUTWIDTH - DES_CLICKGAP*2;
//	sTitleInit.height = DES_CLICKLABELHEIGHT;
//	sTitleInit.psFont = psWFont;
    /* See what type of system stats we've got */
    if (*psStats).ref_0 >= 0x50000 as libc::c_int as libc::c_uint &&
           (*psStats).ref_0 <
               (0x50000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"intAddSystemForm: Invalid sensor stats pointer\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"design.c\x00" as *const u8 as *const libc::c_char,
                  1645 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 18],
                                            &[libc::c_char; 18]>(b"_intSetSystemForm\x00")).as_ptr(),
                  b"PTRVALID(psStats, sizeof(SENSOR_STATS))\x00" as *const u8
                      as *const libc::c_char);
        };
        psSensor = psStats as *mut SENSOR_STATS;
        //		/* Add the title label */
//		sTitleInit.id = IDDES_SYSTEMLAB;
//		sTitleInit.pText = "Sensor";
//		if (!widgAddLabel(psWScreen, &sTitleInit))
//		{
//			return FALSE;
//		}
        sBarInit.id = 5110 as libc::c_int as UDWORD;
        sBarInit.iRange = getMaxSensorRange() as UWORD;
        if widgAddBarGraph(psWScreen, &mut sBarInit) == 0 {
            return 0 as libc::c_int
        }
        sBarInit.id = 5111 as libc::c_int as UDWORD;
        sBarInit.y =
            (7 as libc::c_int + 11 as libc::c_int + 9 as libc::c_int) as
                SWORD;
        sBarInit.iRange = getMaxSensorPower() as UWORD;
        if widgAddBarGraph(psWScreen, &mut sBarInit) == 0 {
            return 0 as libc::c_int
        }
        sBarInit.id = 5112 as libc::c_int as UDWORD;
        sBarInit.y =
            (7 as libc::c_int +
                 (11 as libc::c_int + 9 as libc::c_int) * 2 as libc::c_int) as
                SWORD;
        sBarInit.iRange = getMaxComponentWeight() as UWORD;
        if widgAddBarGraph(psWScreen, &mut sBarInit) == 0 {
            return 0 as libc::c_int
        }
        sLabInit.id = 5210 as libc::c_int as UDWORD;
        sLabInit.pTip =
            strresGetString(psStringRes,
                            STR_DES_SENSOR_RANGE as libc::c_int as UDWORD);
        sLabInit.pDisplay =
            Some(intDisplayImage as
                     unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                          _: UDWORD, _: *mut UDWORD) -> ());
        sLabInit.pUserData =
            IMAGE_DES_RANGE as libc::c_int as *mut libc::c_void;
        if widgAddLabel(psWScreen, &mut sLabInit) == 0 {
            return 0 as libc::c_int
        }
        sLabInit.id = 5211 as libc::c_int as UDWORD;
        sLabInit.y =
            (sLabInit.y as libc::c_int +
                 (11 as libc::c_int + 9 as libc::c_int)) as SWORD;
        sLabInit.pTip =
            strresGetString(psStringRes,
                            STR_DES_SENSOR_POWER as libc::c_int as UDWORD);
        sLabInit.pDisplay =
            Some(intDisplayImage as
                     unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                          _: UDWORD, _: *mut UDWORD) -> ());
        sLabInit.pUserData =
            IMAGE_DES_POWER as libc::c_int as *mut libc::c_void;
        if widgAddLabel(psWScreen, &mut sLabInit) == 0 {
            return 0 as libc::c_int
        }
        sLabInit.id = 5212 as libc::c_int as UDWORD;
        sLabInit.y =
            (sLabInit.y as libc::c_int +
                 (11 as libc::c_int + 9 as libc::c_int)) as SWORD;
        sLabInit.pTip =
            strresGetString(psStringRes,
                            STR_DES_WEIGHT as libc::c_int as UDWORD);
        sLabInit.pDisplay =
            Some(intDisplayImage as
                     unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                          _: UDWORD, _: *mut UDWORD) -> ());
        sLabInit.pUserData =
            IMAGE_DES_WEIGHT as libc::c_int as *mut libc::c_void;
        if widgAddLabel(psWScreen, &mut sLabInit) == 0 {
            return 0 as libc::c_int
        }
    } else if (*psStats).ref_0 >= 0x60000 as libc::c_int as libc::c_uint &&
                  (*psStats).ref_0 <
                      (0x60000 as libc::c_int + 0x10000 as libc::c_int) as
                          libc::c_uint {
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"intAddSystemForm: Invalid ecm stats pointer\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"design.c\x00" as *const u8 as *const libc::c_char,
                  1714 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 18],
                                            &[libc::c_char; 18]>(b"_intSetSystemForm\x00")).as_ptr(),
                  b"PTRVALID(psStats, sizeof(ECM_STATS))\x00" as *const u8 as
                      *const libc::c_char);
        };
        psECM = psStats as *mut ECM_STATS;
        /* Add the bar graphs*/
        //DBAR_SENSORMAXRANGE;
        //+= DES_CLICKBARHEIGHT + DES_CLICKGAP;
        //printf("%d\n",DES_STATBAR_Y2);	//+= DES_CLICKBARHEIGHT + DES_CLICKGAP;
        //DBAR_SENSORMAXPOWER;
        //+= DES_CLICKBARHEIGHT + DES_CLICKGAP;
        //DBAR_MAXWEIGHT;
        /* Add the labels */
        //		sLabInit.pText = "Sensor Range";
        //		sLabInit.pText = " Sensor Power";
        //		sLabInit.pText = "Weight";
        //		/* Add the title label */
//		sTitleInit.id = IDDES_SYSTEMLAB;
//		sTitleInit.pText = "ECM";
//		if (!widgAddLabel(psWScreen, &sTitleInit))
//		{
//			return FALSE;
//		}
        sBarInit.id = 5115 as libc::c_int as UDWORD;
        sBarInit.iRange = getMaxECMPower() as UWORD;
        if widgAddBarGraph(psWScreen, &mut sBarInit) == 0 {
            return 0 as libc::c_int
        }
        sBarInit.id = 5116 as libc::c_int as UDWORD;
        sBarInit.y =
            (7 as libc::c_int + 11 as libc::c_int + 9 as libc::c_int) as
                SWORD;
        sBarInit.iRange = getMaxComponentWeight() as UWORD;
        if widgAddBarGraph(psWScreen, &mut sBarInit) == 0 {
            return 0 as libc::c_int
        }
        sLabInit.id = 5215 as libc::c_int as UDWORD;
        sLabInit.pTip =
            strresGetString(psStringRes,
                            STR_DES_ECM_POWER as libc::c_int as UDWORD);
        sLabInit.pDisplay =
            Some(intDisplayImage as
                     unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                          _: UDWORD, _: *mut UDWORD) -> ());
        sLabInit.pUserData =
            IMAGE_DES_POWER as libc::c_int as *mut libc::c_void;
        if widgAddLabel(psWScreen, &mut sLabInit) == 0 {
            return 0 as libc::c_int
        }
        sLabInit.id = 5216 as libc::c_int as UDWORD;
        sLabInit.y =
            (sLabInit.y as libc::c_int +
                 (11 as libc::c_int + 9 as libc::c_int)) as SWORD;
        sLabInit.pTip =
            strresGetString(psStringRes,
                            STR_DES_WEIGHT as libc::c_int as UDWORD);
        sLabInit.pDisplay =
            Some(intDisplayImage as
                     unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                          _: UDWORD, _: *mut UDWORD) -> ());
        sLabInit.pUserData =
            IMAGE_DES_WEIGHT as libc::c_int as *mut libc::c_void;
        if widgAddLabel(psWScreen, &mut sLabInit) == 0 {
            return 0 as libc::c_int
        }
    } else if (*psStats).ref_0 >= 0xf0000 as libc::c_int as libc::c_uint &&
                  (*psStats).ref_0 <
                      (0xf0000 as libc::c_int + 0x10000 as libc::c_int) as
                          libc::c_uint {
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"intAddSystemForm: Invalid constructor stats pointer\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"design.c\x00" as *const u8 as *const libc::c_char,
                  1765 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 18],
                                            &[libc::c_char; 18]>(b"_intSetSystemForm\x00")).as_ptr(),
                  b"PTRVALID(psStats, sizeof(CONSTRUCT_STATS))\x00" as
                      *const u8 as *const libc::c_char);
        };
        psConst = psStats as *mut CONSTRUCT_STATS;
        /* Add the bar graphs */
        //DBAR_ECMMAXPOWER;
        //+= DES_CLICKBARHEIGHT + DES_CLICKGAP;
        //DBAR_MAXWEIGHT;
        /* Add the labels */
        //		sLabInit.pText = "ECM Power";
        //		sLabInit.pText = "Weight";
        //		/* Add the title label */
//		sTitleInit.id = IDDES_SYSTEMLAB;
//		sTitleInit.pText = "Constructor";
//		if (!widgAddLabel(psWScreen, &sTitleInit))
//		{
//			return FALSE;
//		}
        sBarInit.id = 5125 as libc::c_int as UDWORD;
        sBarInit.pTip =
            strresGetString(psStringRes,
                            STR_DES_BUILD_POINTS as libc::c_int as UDWORD);
        sBarInit.iRange = getMaxConstPoints() as UWORD;
        if widgAddBarGraph(psWScreen, &mut sBarInit) == 0 {
            return 0 as libc::c_int
        }
        sBarInit.id = 5126 as libc::c_int as UDWORD;
        sBarInit.y =
            (7 as libc::c_int + 11 as libc::c_int + 9 as libc::c_int) as
                SWORD;
        sBarInit.pTip =
            strresGetString(psStringRes,
                            STR_DES_WEIGHT as libc::c_int as UDWORD);
        sBarInit.iRange = getMaxComponentWeight() as UWORD;
        if widgAddBarGraph(psWScreen, &mut sBarInit) == 0 {
            return 0 as libc::c_int
        }
        sLabInit.id = 5225 as libc::c_int as UDWORD;
        sLabInit.pTip =
            strresGetString(psStringRes,
                            STR_DES_BUILD_POINTS as libc::c_int as UDWORD);
        sLabInit.pDisplay =
            Some(intDisplayImage as
                     unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                          _: UDWORD, _: *mut UDWORD) -> ());
        sLabInit.pUserData =
            IMAGE_DES_BUILDRATE as libc::c_int as *mut libc::c_void;
        if widgAddLabel(psWScreen, &mut sLabInit) == 0 {
            return 0 as libc::c_int
        }
        sLabInit.id = 5226 as libc::c_int as UDWORD;
        sLabInit.y =
            (sLabInit.y as libc::c_int +
                 (11 as libc::c_int + 9 as libc::c_int)) as SWORD;
        sLabInit.pTip =
            strresGetString(psStringRes,
                            STR_DES_WEIGHT as libc::c_int as UDWORD);
        sLabInit.pDisplay =
            Some(intDisplayImage as
                     unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                          _: UDWORD, _: *mut UDWORD) -> ());
        sLabInit.pUserData =
            IMAGE_DES_WEIGHT as libc::c_int as *mut libc::c_void;
        if widgAddLabel(psWScreen, &mut sLabInit) == 0 {
            return 0 as libc::c_int
        }
    } else if (*psStats).ref_0 >= 0x80000 as libc::c_int as libc::c_uint &&
                  (*psStats).ref_0 <
                      (0x80000 as libc::c_int + 0x10000 as libc::c_int) as
                          libc::c_uint {
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"intAddSystemForm: Invalid repair stats pointer\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"design.c\x00" as *const u8 as *const libc::c_char,
                  1816 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 18],
                                            &[libc::c_char; 18]>(b"_intSetSystemForm\x00")).as_ptr(),
                  b"PTRVALID(psStats, sizeof(REPAIR_STATS))\x00" as *const u8
                      as *const libc::c_char);
        };
        psRepair = psStats as *mut REPAIR_STATS;
        /* Add the bar graphs */
        //DBAR_CONSTMAXPOINTS;
        //+= DES_CLICKBARHEIGHT + DES_CLICKGAP;
        //DBAR_MAXWEIGHT;
        /* Add the labels */
        /* Add the bar graphs */
        sBarInit.id = 5129 as libc::c_int as UDWORD; //DBAR_REPAIRMAXPOINTS;
        sBarInit.pTip =
            strresGetString(psStringRes,
                            STR_DES_BUILD_POINTS as libc::c_int as
                                UDWORD); //+= DES_CLICKBARHEIGHT + DES_CLICKGAP;
        sBarInit.iRange = getMaxRepairPoints() as UWORD; //DBAR_MAXWEIGHT;
        if widgAddBarGraph(psWScreen, &mut sBarInit) == 0 {
            return 0 as libc::c_int
        }
        sBarInit.id = 5130 as libc::c_int as UDWORD;
        sBarInit.y =
            (7 as libc::c_int + 11 as libc::c_int + 9 as libc::c_int) as
                SWORD;
        sBarInit.pTip =
            strresGetString(psStringRes,
                            STR_DES_WEIGHT as libc::c_int as UDWORD);
        sBarInit.iRange = getMaxComponentWeight() as UWORD;
        if widgAddBarGraph(psWScreen, &mut sBarInit) == 0 {
            return 0 as libc::c_int
        }
        /* Add the labels */
        sLabInit.id = 5231 as libc::c_int as UDWORD;
        sLabInit.pTip =
            strresGetString(psStringRes,
                            STR_DES_BUILD_POINTS as libc::c_int as UDWORD);
        sLabInit.pDisplay =
            Some(intDisplayImage as
                     unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                          _: UDWORD, _: *mut UDWORD) -> ());
        sLabInit.pUserData =
            IMAGE_DES_BUILDRATE as libc::c_int as *mut libc::c_void;
        if widgAddLabel(psWScreen, &mut sLabInit) == 0 {
            return 0 as libc::c_int
        }
        sLabInit.id = 5232 as libc::c_int as UDWORD;
        sLabInit.y =
            (sLabInit.y as libc::c_int +
                 (11 as libc::c_int + 9 as libc::c_int)) as SWORD;
        sLabInit.pTip =
            strresGetString(psStringRes,
                            STR_DES_WEIGHT as libc::c_int as UDWORD);
        sLabInit.pDisplay =
            Some(intDisplayImage as
                     unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                          _: UDWORD, _: *mut UDWORD) -> ());
        sLabInit.pUserData =
            IMAGE_DES_WEIGHT as libc::c_int as *mut libc::c_void;
        if widgAddLabel(psWScreen, &mut sLabInit) == 0 {
            return 0 as libc::c_int
        }
    } else if (*psStats).ref_0 >= 0xa0000 as libc::c_int as libc::c_uint &&
                  (*psStats).ref_0 <
                      (0xa0000 as libc::c_int + 0x10000 as libc::c_int) as
                          libc::c_uint {
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"intAddSystemForm: Invalid ecm stats pointer\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"design.c\x00" as *const u8 as *const libc::c_char,
                  1859 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 18],
                                            &[libc::c_char; 18]>(b"_intSetSystemForm\x00")).as_ptr(),
                  b"PTRVALID(psStats, sizeof(WEAPON_STATS))\x00" as *const u8
                      as *const libc::c_char);
        };
        psWeapon = psStats as *mut WEAPON_STATS;
        //		/* Add the title label */
//		sTitleInit.id = IDDES_SYSTEMLAB;
//		sTitleInit.pText = "Weapon";
//		if (!widgAddLabel(psWScreen, &sTitleInit))
//		{
//			return FALSE;
//		}
        sBarInit.id = 5120 as libc::c_int as UDWORD;
        sBarInit.iRange = getMaxWeaponRange() as UWORD;
        sBarInit.pTip =
            strresGetString(psStringRes,
                            STR_DES_RANGE as libc::c_int as UDWORD);
        if widgAddBarGraph(psWScreen, &mut sBarInit) == 0 {
            return 0 as libc::c_int
        }
        sBarInit.id = 5121 as libc::c_int as UDWORD;
        sBarInit.y =
            (7 as libc::c_int + 11 as libc::c_int + 9 as libc::c_int) as
                SWORD;
        sBarInit.iRange = getMaxWeaponDamage() as UWORD;
        sBarInit.pTip =
            strresGetString(psStringRes,
                            STR_DES_DAMAGE as libc::c_int as UDWORD);
        if widgAddBarGraph(psWScreen, &mut sBarInit) == 0 {
            return 0 as libc::c_int
        }
        sBarInit.id = 5122 as libc::c_int as UDWORD;
        sBarInit.y =
            (7 as libc::c_int +
                 (11 as libc::c_int + 9 as libc::c_int) * 2 as libc::c_int) as
                SWORD;
        sBarInit.iRange = 240 as libc::c_int as UWORD;
        sBarInit.pTip =
            strresGetString(psStringRes,
                            STR_DES_ROF as libc::c_int as UDWORD);
        if widgAddBarGraph(psWScreen, &mut sBarInit) == 0 {
            return 0 as libc::c_int
        }
        sBarInit.id = 5123 as libc::c_int as UDWORD;
        sBarInit.y =
            (7 as libc::c_int +
                 (11 as libc::c_int + 9 as libc::c_int) * 3 as libc::c_int) as
                SWORD;
        sBarInit.iRange = getMaxComponentWeight() as UWORD;
        sBarInit.pTip =
            strresGetString(psStringRes,
                            STR_DES_WEIGHT as libc::c_int as UDWORD);
        if widgAddBarGraph(psWScreen, &mut sBarInit) == 0 {
            return 0 as libc::c_int
        }
        sLabInit.id = 5220 as libc::c_int as UDWORD;
        sLabInit.pTip =
            strresGetString(psStringRes,
                            STR_DES_RANGE as libc::c_int as UDWORD);
        sLabInit.pDisplay =
            Some(intDisplayImage as
                     unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                          _: UDWORD, _: *mut UDWORD) -> ());
        sLabInit.pUserData =
            IMAGE_DES_RANGE as libc::c_int as *mut libc::c_void;
        if widgAddLabel(psWScreen, &mut sLabInit) == 0 {
            return 0 as libc::c_int
        }
        sLabInit.id = 5221 as libc::c_int as UDWORD;
        sLabInit.y =
            (sLabInit.y as libc::c_int +
                 (11 as libc::c_int + 9 as libc::c_int)) as SWORD;
        sLabInit.pTip =
            strresGetString(psStringRes,
                            STR_DES_DAMAGE as libc::c_int as UDWORD);
        sLabInit.pDisplay =
            Some(intDisplayImage as
                     unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                          _: UDWORD, _: *mut UDWORD) -> ());
        sLabInit.pUserData =
            IMAGE_DES_DAMAGE as libc::c_int as *mut libc::c_void;
        if widgAddLabel(psWScreen, &mut sLabInit) == 0 {
            return 0 as libc::c_int
        }
        sLabInit.id = 5222 as libc::c_int as UDWORD;
        sLabInit.y =
            (sLabInit.y as libc::c_int +
                 (11 as libc::c_int + 9 as libc::c_int)) as SWORD;
        sLabInit.pTip =
            strresGetString(psStringRes,
                            STR_DES_ROF as libc::c_int as UDWORD);
        sLabInit.pDisplay =
            Some(intDisplayImage as
                     unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                          _: UDWORD, _: *mut UDWORD) -> ());
        sLabInit.pUserData =
            IMAGE_DES_FIRERATE as libc::c_int as *mut libc::c_void;
        if widgAddLabel(psWScreen, &mut sLabInit) == 0 {
            return 0 as libc::c_int
        }
        sLabInit.id = 5223 as libc::c_int as UDWORD;
        sLabInit.y =
            (sLabInit.y as libc::c_int +
                 (11 as libc::c_int + 9 as libc::c_int)) as SWORD;
        sLabInit.pTip =
            strresGetString(psStringRes,
                            STR_DES_WEIGHT as libc::c_int as UDWORD);
        sLabInit.pDisplay =
            Some(intDisplayImage as
                     unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                          _: UDWORD, _: *mut UDWORD) -> ());
        sLabInit.pUserData =
            IMAGE_DES_WEIGHT as libc::c_int as *mut libc::c_void;
        if widgAddLabel(psWScreen, &mut sLabInit) == 0 {
            return 0 as libc::c_int
        }
    }
    /* Add the bar graphs */
    //DBAR_WEAPMAXRANGE;
    //+= DES_CLICKBARHEIGHT + DES_CLICKGAP;
    //DBAR_WEAPMAXDAMAGE;
    //+= DES_CLICKBARHEIGHT + DES_CLICKGAP;
    //+= DES_CLICKBARHEIGHT + DES_CLICKGAP;
    //DBAR_MAXWEIGHT;
    /* Add the labels */
    //		sLabInit.pText = "Range";
    //		sLabInit.pText = "Dam";
    //		sLabInit.pText = "ROF";
    //		sLabInit.pText = "Weight";
    // Add the correct component form
    match desSysMode as libc::c_uint {
        0 | 2 | 1 | 3 | 5 => {
            //	case IDES_COMMAND:
//		intSetDesignMode(IDES_BRAIN);
//		break;
            intSetDesignMode(IDES_SYSTEM);
        }
        4 => { intSetDesignMode(IDES_TURRET); }
        _ => { }
    }
    /* Set the stats */
    intSetSystemStats(psStats);
    /* Lock the form down if necessary */
    if desCompMode as libc::c_uint ==
           IDES_SYSTEM as libc::c_int as libc::c_uint {
        widgSetButtonState(psWScreen, 5006 as libc::c_int as UDWORD,
                           0x2 as libc::c_int as UDWORD);
    }
    return 1 as libc::c_int;
}
/* Set up the propulsion clickable form of the design screen given a set of stats */
/* Set up the propulsion clickable form of the design screen given a set of stats */
unsafe extern "C" fn intSetPropulsionForm(mut psStats: *mut PROPULSION_STATS)
 -> BOOL {
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
    //	W_LABINIT		sTitleInit;
    let mut newPropMode: DES_PROPMODE = IDES_GROUND;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"intAddPropulsionForm: Invalid propulsion stats pointer\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"design.c\x00" as *const u8 as *const libc::c_char,
              1988 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 21],
                                        &[libc::c_char; 21]>(b"intSetPropulsionForm\x00")).as_ptr(),
              b"PTRVALID(psStats, sizeof(PROPULSION_STATS))\x00" as *const u8
                  as *const libc::c_char);
    };
    memset(&mut sFormInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_FORMINIT>() as libc::c_ulong);
    memset(&mut sLabInit as *mut W_LABINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_LABINIT>() as libc::c_ulong);
    //	memset(&sTitleInit, 0, sizeof(W_LABINIT));
    memset(&mut sBarInit as *mut W_BARINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_BARINIT>() as libc::c_ulong);
    /* figure out what the new mode should be */
    match (*asPropulsionTypes.offset((*psStats).propulsionType as
                                         isize)).travel {
        0 => { newPropMode = IDES_GROUND }
        1 => { newPropMode = IDES_AIR }
        _ => { }
    }
    /* If the mode hasn't changed, just set the stats */
    if desPropMode as libc::c_uint == newPropMode as libc::c_uint {
        intSetPropulsionStats(psStats);
        return 1 as libc::c_int
    }
    /* Remove the old form if necessary */
    if desPropMode as libc::c_uint !=
           IDES_NOPROPULSION as libc::c_int as libc::c_uint {
        widgDelete(psWScreen, 5008 as libc::c_int as UDWORD);
    }
    /* Set the new mode */
    desPropMode = newPropMode;
    /* Add the propulsion form */
    sFormInit.formID = 5001 as libc::c_int as UDWORD; //DES_COMPBUTWIDTH;
    sFormInit.id = 5008 as libc::c_int as UDWORD; //DES_COMPBUTHEIGHT;
    sFormInit.style =
        (4 as libc::c_int | 8 as libc::c_int) as
            UDWORD; /* set form tip to stats string */
    sFormInit.x = 6 as libc::c_int as SWORD;
    sFormInit.y = 6 as libc::c_int as SWORD;
    sFormInit.width = 300 as libc::c_int as UWORD;
    sFormInit.height = 85 as libc::c_int as UWORD;
    sFormInit.pTip = getStatName(psStats as *mut libc::c_void);
    sFormInit.pDisplay =
        Some(intDisplayStatForm as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    if widgAddForm(psWScreen, &mut sFormInit) == 0 { return 0 as libc::c_int }
    /* Initialise the bargraph struct */
    sBarInit.formID = 5008 as libc::c_int as UDWORD; //WBAR_DOUBLE;
    sBarInit.style = 0 as libc::c_int as UDWORD; //DES_CLICKBARY;
    sBarInit.orientation = 0x1 as libc::c_int as UWORD;
    sBarInit.x = 154 as libc::c_int as SWORD;
    sBarInit.y = 7 as libc::c_int as SWORD;
    sBarInit.width = 140 as libc::c_int as UWORD;
    sBarInit.height = 11 as libc::c_int as UWORD;
    sBarInit.sCol.red = 255 as libc::c_int as UBYTE;
    sBarInit.sCol.green = 235 as libc::c_int as UBYTE;
    sBarInit.sCol.blue = 19 as libc::c_int as UBYTE;
    sBarInit.sMinorCol.red = 0x55 as libc::c_int as UBYTE;
    sBarInit.sMinorCol.green = 0 as libc::c_int as UBYTE;
    sBarInit.sMinorCol.blue = 0 as libc::c_int as UBYTE;
    sBarInit.pDisplay =
        Some(intDisplayStatsBar as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    /* Initialise the label struct */
    sLabInit.formID = 5008 as libc::c_int as UDWORD; //DES_CLICKBARHEIGHT;
    sLabInit.style = 0 as libc::c_int as UDWORD;
    sLabInit.x = 126 as libc::c_int as SWORD;
    sLabInit.y =
        (7 as libc::c_int - 11 as libc::c_int / 3 as libc::c_int) as SWORD;
    sLabInit.width = 20 as libc::c_int as UWORD;
    sLabInit.height = 19 as libc::c_int as UWORD;
    sLabInit.FontID = WFont;
    //	/* Initialise the title label struct */
//	sTitleInit.formID = IDDES_PROPFORM;
//	sTitleInit.style = WLAB_ALIGNCENTRE;
//	sTitleInit.x = DES_CLICKGAP;
//	sTitleInit.y = DES_CLICKGAP;
//	sTitleInit.width = DES_COMPBUTWIDTH - DES_CLICKGAP*2;
//	sTitleInit.height = DES_CLICKLABELHEIGHT;
//	sTitleInit.psFont = psWFont;
    /* See what type of propulsion we've got */
    match desPropMode as libc::c_uint {
        1 => {
            /* Add the bar graphs */
            sBarInit.id = 5108 as libc::c_int as UDWORD; //DBAR_PROPMAXSPEED;
            sBarInit.iRange =
                getMaxPropulsionSpeed() as
                    UWORD; //+= DES_CLICKBARHEIGHT + DES_CLICKGAP;
            sBarInit.pTip =
                strresGetString(psStringRes,
                                STR_DES_AIR as libc::c_int as
                                    UDWORD); //DBAR_MAXWEIGHT;
            if widgAddBarGraph(psWScreen, &mut sBarInit) == 0 {
                return 0 as libc::c_int
            }
            sBarInit.id = 5109 as libc::c_int as UDWORD;
            sBarInit.y =
                (7 as libc::c_int + 11 as libc::c_int + 9 as libc::c_int) as
                    SWORD;
            sBarInit.iRange = getMaxComponentWeight() as UWORD;
            sBarInit.pTip =
                strresGetString(psStringRes,
                                STR_DES_WEIGHT as libc::c_int as UDWORD);
            if widgAddBarGraph(psWScreen, &mut sBarInit) == 0 {
                return 0 as libc::c_int
            }
            /* Add the labels */
            sLabInit.id = 5208 as libc::c_int as UDWORD;
            sLabInit.pTip =
                strresGetString(psStringRes,
                                STR_DES_AIR as libc::c_int as UDWORD);
            sLabInit.pDisplay =
                Some(intDisplayImage as
                         unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                              _: UDWORD, _: *mut UDWORD)
                             -> ());
            sLabInit.pUserData =
                IMAGE_DES_HOVER as libc::c_int as *mut libc::c_void;
            if widgAddLabel(psWScreen, &mut sLabInit) == 0 {
                return 0 as libc::c_int
            }
            sLabInit.id = 5209 as libc::c_int as UDWORD;
            sLabInit.y =
                (sLabInit.y as libc::c_int +
                     (11 as libc::c_int + 9 as libc::c_int)) as SWORD;
            sLabInit.pTip =
                strresGetString(psStringRes,
                                STR_DES_WEIGHT as libc::c_int as UDWORD);
            sLabInit.pDisplay =
                Some(intDisplayImage as
                         unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                              _: UDWORD, _: *mut UDWORD)
                             -> ());
            sLabInit.pUserData =
                IMAGE_DES_WEIGHT as libc::c_int as *mut libc::c_void;
            if widgAddLabel(psWScreen, &mut sLabInit) == 0 {
                return 0 as libc::c_int
            }
        }
        0 => {
            /* Add the bar graphs */
            sBarInit.id = 5105 as libc::c_int as UDWORD; //DBAR_PROPMAXSPEED;
            sBarInit.pTip =
                strresGetString(psStringRes,
                                STR_DES_ROAD as libc::c_int as
                                    UDWORD); //+= DES_CLICKBARHEIGHT + DES_CLICKGAP;
            sBarInit.iRange =
                getMaxPropulsionSpeed() as UWORD; //DBAR_PROPMAXSPEED;
            if widgAddBarGraph(psWScreen, &mut sBarInit) == 0 {
                return 0 as libc::c_int
            } //+= DES_CLICKBARHEIGHT + DES_CLICKGAP;
            sBarInit.id = 5106 as libc::c_int as UDWORD; //DBAR_PROPMAXSPEED;
            sBarInit.y =
                (7 as libc::c_int + 11 as libc::c_int + 9 as libc::c_int) as
                    SWORD; //+= DES_CLICKBARHEIGHT + DES_CLICKGAP;
            sBarInit.pTip =
                strresGetString(psStringRes,
                                STR_DES_OFFROAD as libc::c_int as
                                    UDWORD); //DBAR_MAXWEIGHT;
            sBarInit.iRange = getMaxPropulsionSpeed() as UWORD;
            if widgAddBarGraph(psWScreen, &mut sBarInit) == 0 {
                return 0 as libc::c_int
            }
            sBarInit.id = 5107 as libc::c_int as UDWORD;
            sBarInit.y =
                (7 as libc::c_int +
                     (11 as libc::c_int + 9 as libc::c_int) *
                         2 as libc::c_int) as SWORD;
            sBarInit.pTip =
                strresGetString(psStringRes,
                                STR_DES_WATER as libc::c_int as UDWORD);
            sBarInit.iRange = getMaxPropulsionSpeed() as UWORD;
            if widgAddBarGraph(psWScreen, &mut sBarInit) == 0 {
                return 0 as libc::c_int
            }
            sBarInit.id = 5109 as libc::c_int as UDWORD;
            sBarInit.y =
                (7 as libc::c_int +
                     (11 as libc::c_int + 9 as libc::c_int) *
                         3 as libc::c_int) as SWORD;
            sBarInit.pTip =
                strresGetString(psStringRes,
                                STR_DES_WEIGHT as libc::c_int as UDWORD);
            sBarInit.iRange = getMaxComponentWeight() as UWORD;
            if widgAddBarGraph(psWScreen, &mut sBarInit) == 0 {
                return 0 as libc::c_int
            }
            /* Add the labels */
            sLabInit.id = 5205 as libc::c_int as UDWORD; //WATER;
            sLabInit.pTip =
                strresGetString(psStringRes,
                                STR_DES_ROAD as libc::c_int as UDWORD);
            sLabInit.pDisplay =
                Some(intDisplayImage as
                         unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                              _: UDWORD, _: *mut UDWORD)
                             -> ());
            sLabInit.pUserData =
                IMAGE_DES_ROAD as libc::c_int as *mut libc::c_void;
            if widgAddLabel(psWScreen, &mut sLabInit) == 0 {
                return 0 as libc::c_int
            }
            sLabInit.id = 5206 as libc::c_int as UDWORD;
            sLabInit.y =
                (sLabInit.y as libc::c_int +
                     (11 as libc::c_int + 9 as libc::c_int)) as SWORD;
            sLabInit.pTip =
                strresGetString(psStringRes,
                                STR_DES_OFFROAD as libc::c_int as UDWORD);
            sLabInit.pDisplay =
                Some(intDisplayImage as
                         unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                              _: UDWORD, _: *mut UDWORD)
                             -> ());
            sLabInit.pUserData =
                IMAGE_DES_CROSSCOUNTRY as libc::c_int as *mut libc::c_void;
            if widgAddLabel(psWScreen, &mut sLabInit) == 0 {
                return 0 as libc::c_int
            }
            sLabInit.id = 5207 as libc::c_int as UDWORD;
            sLabInit.y =
                (sLabInit.y as libc::c_int +
                     (11 as libc::c_int + 9 as libc::c_int)) as SWORD;
            sLabInit.pTip =
                strresGetString(psStringRes,
                                STR_DES_WATER as libc::c_int as UDWORD);
            sLabInit.pDisplay =
                Some(intDisplayImage as
                         unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                              _: UDWORD, _: *mut UDWORD)
                             -> ());
            sLabInit.pUserData =
                IMAGE_DES_HOVER as libc::c_int as *mut libc::c_void;
            if widgAddLabel(psWScreen, &mut sLabInit) == 0 {
                return 0 as libc::c_int
            }
            sLabInit.id = 5209 as libc::c_int as UDWORD;
            sLabInit.y =
                (sLabInit.y as libc::c_int +
                     (11 as libc::c_int + 9 as libc::c_int)) as SWORD;
            sLabInit.pTip =
                strresGetString(psStringRes,
                                STR_DES_WEIGHT as libc::c_int as UDWORD);
            sLabInit.pDisplay =
                Some(intDisplayImage as
                         unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                              _: UDWORD, _: *mut UDWORD)
                             -> ());
            sLabInit.pUserData =
                IMAGE_DES_WEIGHT as libc::c_int as *mut libc::c_void;
            if widgAddLabel(psWScreen, &mut sLabInit) == 0 {
                return 0 as libc::c_int
            }
        }
        _ => { }
    }
    /* Set the stats */
    intSetPropulsionStats(psStats);
    /* Lock the form down if necessary */
    if desCompMode as libc::c_uint ==
           IDES_PROPULSION as libc::c_int as libc::c_uint {
        widgSetButtonState(psWScreen, 5008 as libc::c_int as UDWORD,
                           0x2 as libc::c_int as UDWORD);
    }
    return 1 as libc::c_int;
}
/* Add the Major system tab form to the design screen */
//static BOOL intAddMultiComponentForm(UBYTE *aSensor, UDWORD numSensor,
//							 UBYTE *aECM, UDWORD numECM,
//							 UBYTE	*aConstruct, UDWORD numConstruct,
//							 UBYTE *aWeapon, UDWORD numWeapon);
// count the number of available components
// count the number of available components
unsafe extern "C" fn intNumAvailable(mut aAvailable: *mut UBYTE,
                                     mut numEntries: UDWORD,
                                     mut asStats: *mut COMP_BASE_STATS,
                                     mut size: UDWORD) -> UDWORD {
    let mut numButtons: UDWORD = 0;
    let mut i: UDWORD = 0;
    let mut psCurrStats: *mut COMP_BASE_STATS = 0 as *mut COMP_BASE_STATS;
    numButtons = 0 as libc::c_int as UDWORD;
    psCurrStats = asStats;
    i = 0 as libc::c_int as UDWORD;
    while i < numEntries {
        /*all components have a flag which indicates whether they can be seen
		in design screen now so don't need to check for default location*/
		//if ((aAvailable[i] & AVAILABLE) &&
		//	intGetLocation(psCurrStats) != LOC_DEFAULT)
        if (*psCurrStats).design != 0 &&
               *aAvailable.offset(i as isize) as libc::c_int &
                   0x1 as libc::c_int != 0 {
            numButtons = numButtons.wrapping_add(1)
        }
        psCurrStats =
            (psCurrStats as *mut UBYTE).offset(size as isize) as
                *mut COMP_BASE_STATS;
        i = i.wrapping_add(1)
    }
    return numButtons;
}
/* Add the component tab form to the design screen */
/* Add the component tab form to the design screen */
unsafe extern "C" fn intAddComponentForm(mut numButtons: UDWORD) -> BOOL {
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
    let mut i: UDWORD = 0;
    let mut butPerForm: UDWORD = 0;
    let mut numFrm: UDWORD = 0;
    memset(&mut sFormInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_FORMINIT>() as libc::c_ulong);
    /* Count the number of buttons that will be on the tabbed form */
/*	numButtons = 0;
	for(i=0; i < numEntries; i++)
	{
		if (aAvailable[i] & AVAILABLE)
		{
			numButtons++;
		}
	}*/
    /* Calculate how many buttons will go on a single form */
/*	butPerForm = ((DES_RIGHTFORMWIDTH - DES_TABTHICKNESS - DES_TABBUTGAP) /
						(DES_TABBUTWIDTH + DES_TABBUTGAP)) *
				 ((DES_RIGHTFORMHEIGHT - DES_TABTHICKNESS - DES_TABBUTGAP) /
						(DES_TABBUTHEIGHT + DES_TABBUTGAP));*/
    butPerForm = 8 as libc::c_int as UDWORD;
    /* add a form to place the tabbed form on */
    memset(&mut sFormInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_FORMINIT>() as libc::c_ulong);
    sFormInit.formID = 0 as libc::c_int as UDWORD;
    sFormInit.id = 5021 as libc::c_int as UDWORD;
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
                                                                                                                                                                                                                                                                                                                     libc::c_uint)).wrapping_sub(2
                                                                                                                                                                                                                                                                                                                                                     as
                                                                                                                                                                                                                                                                                                                                                     libc::c_int
                                                                                                                                                                                                                                                                                                                                                     as
                                                                                                                                                                                                                                                                                                                                                     libc::c_uint)
            as SWORD;
    sFormInit.y =
        (59 as libc::c_int as
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
    sFormInit.width = 132 as libc::c_int as UWORD;
    sFormInit.height = (258 as libc::c_int + 4 as libc::c_int) as UWORD;
    sFormInit.pDisplay =
        Some(intDisplayPlainForm as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    if widgAddForm(psWScreen, &mut sFormInit) == 0 { return 0 as libc::c_int }
    //now a single form
    memset(&mut sFormInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_FORMINIT>() as libc::c_ulong);
    sFormInit.formID = 5021 as libc::c_int as UDWORD;
    sFormInit.id = 5003 as libc::c_int as UDWORD;
    sFormInit.style = 1 as libc::c_int as UDWORD;
    sFormInit.x = 2 as libc::c_int as SWORD;
    sFormInit.y = 40 as libc::c_int as SWORD;
    sFormInit.width = (132 as libc::c_int + 20 as libc::c_int) as UWORD;
    sFormInit.height = 258 as libc::c_int as UWORD;
    numFrm = numForms(numButtons, butPerForm) as UDWORD;
    sFormInit.numMajor =
        if numFrm >= 9 as libc::c_int as libc::c_uint {
            (9 as libc::c_int - 1 as libc::c_int) as libc::c_uint
        } else { numFrm } as UWORD;
    sFormInit.majorPos = 1 as libc::c_int as UWORD;
    sFormInit.minorPos = 0 as libc::c_int as UWORD;
    sFormInit.majorSize = 26 as libc::c_int as UWORD;
    sFormInit.majorOffset = 2 as libc::c_int as SWORD;
    sFormInit.tabVertOffset = (11 as libc::c_int / 2 as libc::c_int) as SWORD;
    sFormInit.tabMajorThickness = 11 as libc::c_int as UWORD;
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
    return 1 as libc::c_int;
}
/* Add the system buttons (weapons, command droid, etc) to the design screen */
/* Add the Major system tab form to the design screen */
/*static BOOL intAddMultiComponentForm(UBYTE *aSensor, UDWORD numSensor,
									 UBYTE *aECM, UDWORD numECM,
									 UBYTE	*aConstruct, UDWORD numConstruct,
									 UBYTE *aWeapon, UDWORD numWeapon)
{
	W_FORMINIT		sFormInit;
	UDWORD			i, numButtons, butPerForm;

	memset(&sFormInit, 0, sizeof(W_FORMINIT));

	// Calculate how many buttons will go on a single form
	butPerForm = ((DES_RIGHTFORMWIDTH - DES_MINORSIZE - DES_TABBUTGAP) /
						(DES_TABBUTWIDTH + DES_TABBUTGAP)) *
				 ((DES_RIGHTFORMHEIGHT - DES_MAJORSIZE - DES_TABBUTGAP) /
						(DES_TABBUTHEIGHT + DES_TABBUTGAP));

	// add a form to place the tabbed form on
	memset(&sFormInit, 0, sizeof(W_FORMINIT));
	sFormInit.formID = 0;
	sFormInit.id = IDDES_RIGHTBASE;
	sFormInit.style = WFORM_PLAIN;
	sFormInit.x = RADTLX-2;
	sFormInit.y = DESIGN_Y;
	sFormInit.width = DES_RIGHTFORMWIDTH;//RET_FORMWIDTH;
	sFormInit.height = DES_RIGHTFORMHEIGHT + 4;
	sFormInit.pDisplay = intDisplayPlainForm;
	if (!widgAddForm(psWScreen, &sFormInit))
	{
		return FALSE;
	}

	// Initialise the form structure
	memset(&sFormInit, 0, sizeof(W_FORMINIT));
	sFormInit.formID = IDDES_RIGHTBASE;			//0;
	sFormInit.id = IDDES_COMPFORM;
	sFormInit.style = WFORM_TABBED;
	sFormInit.x = 2;
	sFormInit.y = 10;
	sFormInit.width = DES_RIGHTFORMWIDTH;
	sFormInit.height = DES_RIGHTFORMHEIGHT;
	sFormInit.majorPos = WFORM_TABTOP;
	sFormInit.minorPos = WFORM_TABRIGHT;//WFORM_TABNONE;
	sFormInit.majorSize = 35;
	sFormInit.minorSize = 26;//0;
	sFormInit.minorOffset = 10;//0;
	sFormInit.majorOffset = 4;
	sFormInit.tabVertOffset = 0;//20;
	sFormInit.tabHorzOffset = -5;//0;
	sFormInit.tabMajorThickness = 44;
	sFormInit.tabMinorThickness = 11;//0;
	sFormInit.tabMajorGap = DES_TAB_SYSGAP;
	sFormInit.tabMinorGap = 0;//DES_TAB_SYSGAP - 2;
	sFormInit.numMajor = 3;
	sFormInit.apMajorTips[IDES_MAINTAB] = strresGetString(psStringRes, STR_DES_WEAPONS);
	sFormInit.apMajorTips[IDES_EXTRATAB] = strresGetString(psStringRes, STR_DES_OTHER);
	sFormInit.apMajorTips[IDES_EXTRATAB2] = strresGetString(psStringRes, STR_DES_COMMAND);
	sFormInit.pFormDisplay = intDisplayObjectForm;
	sFormInit.pUserData = (void*)&SystemTab;
	sFormInit.pTabDisplay = intDisplaySystemTab;

	// Calculate the number of minor tabs for each major
	numButtons = 0;
	for(i=0; i < numWeapon; i++)
	{
		if (aWeapon[i] & AVAILABLE)
		{
			numButtons++;
			if (numButtons == MAX_SYSTEM_COMPONENTS)
			{
				break;
			}
		}
	}
	sFormInit.aNumMinors[0] = numForms(numButtons, butPerForm);
// Hack so we can use design screen after pressing 'A'.
//	if(sFormInit.aNumMinors[0] >= WFORM_MAXMINOR) {
//		sFormInit.aNumMinors[0] = WFORM_MAXMINOR-1;
//	}

	numButtons = 0;
    for(i=0; i < numSensor; i++)
	{
		if ((aSensor[i] & AVAILABLE) AND
			intGetLocation((COMP_BASE_STATS *)&asSensorStats[i]) != LOC_DEFAULT)
		{
			numButtons++;
			if (numButtons == MAX_SYSTEM_COMPONENTS)
			{
				break;
			}
		}
	}
	if (numButtons < MAX_SYSTEM_COMPONENTS)
	{
		for(i=0; i < numECM; i++)
		{
			if ((aECM[i] & AVAILABLE) AND
				intGetLocation((COMP_BASE_STATS *)&asECMStats[i]) != LOC_DEFAULT)
			{
				numButtons++;
			}
		}
		for(i=0; i < numConstruct; i++)
		{
			if (aConstruct[i] & AVAILABLE)
			{
				numButtons++;
			}
		}
	}

	sFormInit.aNumMinors[1] = numForms(numButtons, butPerForm);
	sFormInit.aNumMinors[2] = 1;
// Hack so we can use design screen after pressing 'A'.
//	if(sFormInit.aNumMinors[1] >= WFORM_MAXMINOR) {
//		sFormInit.aNumMinors[1] = WFORM_MAXMINOR-1;
//	}


	if (!widgAddForm(psWScreen, &sFormInit))
	{
		return FALSE;
	}

	return TRUE;
}*/
/* Add the system buttons (weapons, command droid, etc) to the design screen */
unsafe extern "C" fn intAddSystemButtons(mut mode: SDWORD) -> BOOL {
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
    memset(&mut sButInit as *mut W_BUTINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_BUTINIT>() as libc::c_ulong);
    // add the weapon button
    sButInit.formID = 5021 as libc::c_int as UDWORD;
    sButInit.id = 5024 as libc::c_int as UDWORD;
    sButInit.style = 0 as libc::c_int as UDWORD;
    sButInit.x = 26 as libc::c_int as SWORD;
    sButInit.y = 10 as libc::c_int as SWORD;
    sButInit.width =
        iV_GetImageWidth(IntImages,
                         IMAGE_DES_WEAPONS as libc::c_int as UWORD);
    sButInit.height =
        iV_GetImageHeight(IntImages,
                          IMAGE_DES_WEAPONS as libc::c_int as UWORD);
    sButInit.pTip =
        strresGetString(psStringRes,
                        STR_DES_WEAPONS as libc::c_int as UDWORD);
    sButInit.FontID = WFont;
    sButInit.pDisplay =
        Some(intDisplayButtonHilight as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    sButInit.pUserData =
        ((0 as libc::c_int & 0x3ff as libc::c_int) << 20 as libc::c_int |
             (IMAGE_DES_EXTRAHI as libc::c_int & 0x3ff as libc::c_int) <<
                 10 as libc::c_int |
             IMAGE_DES_WEAPONS as libc::c_int & 0x3ff as libc::c_int) as
            *mut libc::c_void;
    if widgAddButton(psWScreen, &mut sButInit) == 0 {
        return 0 as libc::c_int
    }
    //if currently got a VTOL proplusion attached then don't add the system buttons
    if checkTemplateIsVtol(&mut sCurrDesign) == 0 {
        // add the system button
        sButInit.formID = 5021 as libc::c_int as UDWORD;
        sButInit.id = 5025 as libc::c_int as UDWORD;
        sButInit.style = 0 as libc::c_int as UDWORD;
        sButInit.x = 68 as libc::c_int as SWORD;
        sButInit.y = 10 as libc::c_int as SWORD;
        sButInit.width =
            iV_GetImageWidth(IntImages,
                             IMAGE_DES_SYSTEMS as libc::c_int as UWORD);
        sButInit.height =
            iV_GetImageHeight(IntImages,
                              IMAGE_DES_SYSTEMS as libc::c_int as UWORD);
        sButInit.pTip =
            strresGetString(psStringRes,
                            STR_DES_OTHER as libc::c_int as UDWORD);
        sButInit.FontID = WFont;
        sButInit.pDisplay =
            Some(intDisplayButtonHilight as
                     unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                          _: UDWORD, _: *mut UDWORD) -> ());
        sButInit.pUserData =
            ((0 as libc::c_int & 0x3ff as libc::c_int) << 20 as libc::c_int |
                 (IMAGE_DES_EXTRAHI as libc::c_int & 0x3ff as libc::c_int) <<
                     10 as libc::c_int |
                 IMAGE_DES_SYSTEMS as libc::c_int & 0x3ff as libc::c_int) as
                *mut libc::c_void;
        if widgAddButton(psWScreen, &mut sButInit) == 0 {
            return 0 as libc::c_int
        }
    }
    // command turrets now in systems
    // lock down the correct button
    match mode {
        2 => {
            widgSetButtonState(psWScreen, 5024 as libc::c_int as UDWORD,
                               0x2 as libc::c_int as UDWORD);
        }
        0 | 1 => {
            /*
		widgSetButtonState(psWScreen, IDDES_COMMAND, WBUT_LOCK);
		break;
*/
            widgSetButtonState(psWScreen, 5025 as libc::c_int as UDWORD,
                               0x2 as libc::c_int as UDWORD);
        }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"intAddSystemButtons: unexpected mode\x00" as *const u8
                          as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"design.c\x00" as *const u8 as *const libc::c_char,
                      2525 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 20],
                                                &[libc::c_char; 20]>(b"intAddSystemButtons\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
        }
    }
    return 1 as libc::c_int;
}
/* Add the component buttons to the main tab of the system or component form */
/* Add the component buttons to the main tab of the component form */
unsafe extern "C" fn intAddComponentButtons(mut psStats: *mut COMP_BASE_STATS,
                                            mut size: UDWORD,
                                            mut aAvailable: *mut UBYTE,
                                            mut numEntries: UDWORD,
                                            mut compID: UDWORD,
                                            mut WhichTab: UDWORD) -> BOOL {
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
                   pFormDisplay: None,};
    let mut psTabForm: *mut W_TABFORM = 0 as *mut W_TABFORM;
    let mut i: UDWORD = 0;
    let mut maxComponents: UDWORD = 0;
    let mut psCurrStats: *mut COMP_BASE_STATS = 0 as *mut COMP_BASE_STATS;
    let mut aButText: [STRING; 6] = [0; 6];
    let mut BufferID: SDWORD = 0;
    let mut psPropStats: *mut PROPULSION_STATS = 0 as *mut PROPULSION_STATS;
    let mut bVTol: BOOL = 0;
    let mut bWeapon: BOOL = 0;
    let mut bVtolWeapon: BOOL = 0;
    let mut numTabs: UWORD = 0;
    ClearObjectBuffers();
    memset(aButText.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           (5 as libc::c_int + 1 as libc::c_int) as libc::c_uint);
    memset(&mut sButInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_BUTINIT>() as libc::c_ulong);
    /* Set up the button struct */
    sButInit.formID = 5003 as libc::c_int as UDWORD;
    sButInit.majorID = 0 as libc::c_int as UWORD;
    sButInit.minorID = 0 as libc::c_int as UWORD;
    sButInit.id = 5500 as libc::c_int as UDWORD;
    sButInit.style = 4 as libc::c_int as UDWORD;
    sButInit.x = 2 as libc::c_int as SWORD;
    sButInit.y = 2 as libc::c_int as SWORD;
    sButInit.width = 60 as libc::c_int as UWORD;
    sButInit.height = 46 as libc::c_int as UWORD;
    //need to set max number of buttons possible
    if (*psStats).ref_0 >= 0xa0000 as libc::c_int as libc::c_uint &&
           (*psStats).ref_0 <
               (0xa0000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
        maxComponents = 32 as libc::c_int as UDWORD
    } else { maxComponents = 40 as libc::c_int as UDWORD }
    /*if adding weapons - need to check if the propulsion is a VTOL*/
    bVTol = 0 as libc::c_int;
    if (*psStats).ref_0 >= 0xa0000 as libc::c_int as libc::c_uint &&
           (*psStats).ref_0 <
               (0xa0000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
        bWeapon = 1 as libc::c_int
    } else { bWeapon = 0 as libc::c_int }
    if bWeapon != 0 {
        //check if the current Template propulsion has been set
        if sCurrDesign.asParts[COMP_PROPULSION as libc::c_int as usize] != 0 {
            psPropStats =
                asPropulsionStats.offset(sCurrDesign.asParts[COMP_PROPULSION
                                                                 as
                                                                 libc::c_int
                                                                 as usize] as
                                             isize);
            if 1 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"intAddComponentButtons: invalid propulsion stats pointer\x00"
                          as *const u8 as *const libc::c_char);
            };
            if 1 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"design.c\x00" as *const u8 as *const libc::c_char,
                      2596 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 23],
                                                &[libc::c_char; 23]>(b"intAddComponentButtons\x00")).as_ptr(),
                      b"PTRVALID(psPropStats, sizeof(PROPULSION_STATS))\x00"
                          as *const u8 as *const libc::c_char);
            };
            if (*asPropulsionTypes.offset((*psPropStats).propulsionType as
                                              isize)).travel ==
                   AIR as libc::c_int as libc::c_uint {
                bVTol = 1 as libc::c_int
            }
        }
    }
    /* Add each button */
    desCompID = 0 as libc::c_int as UDWORD;
    numComponent = 0 as libc::c_int as UDWORD;
    psCurrStats = psStats;
    let mut current_block_102: u64;
    i = 0 as libc::c_int as UDWORD;
    while i < numEntries {
        /* If we are out of space in the list - stop */
        if numComponent >= maxComponents { break ; }
        /* Skip unavailable entries and non-design ones*/
        if *aAvailable.offset(i as isize) as libc::c_int & 0x1 as libc::c_int
               == 0 || (*psCurrStats).design == 0 {
            /* Update the stats pointer for the next button */
            psCurrStats =
                (psCurrStats as *mut UBYTE).offset(size as isize) as
                    *mut COMP_BASE_STATS
        } else {
            /*skip indirect weapons if VTOL propulsion or numVTOLattackRuns for the weapon is zero*/
            if bWeapon != 0 {
                if (*(psCurrStats as *mut WEAPON_STATS)).vtolAttackRuns != 0 {
                    bVtolWeapon = 1 as libc::c_int
                } else { bVtolWeapon = 0 as libc::c_int }
                if bVTol != 0 && bVtolWeapon == 0 ||
                       bVTol == 0 && bVtolWeapon != 0 {
                    /* Update the stats pointer for the next button */
                    psCurrStats =
                        (psCurrStats as *mut UBYTE).offset(size as isize) as
                            *mut COMP_BASE_STATS;
                    current_block_102 = 14072441030219150333;
                } else { current_block_102 = 17747245473264231573; }
            } else { current_block_102 = 17747245473264231573; }
            match current_block_102 {
                14072441030219150333 => { }
                _ => {
                    /* Set the tip and add the button */
                    strncpy(aButText.as_mut_ptr(),
                            getStatName(psCurrStats as *mut libc::c_void),
                            5 as libc::c_int as libc::c_uint);
                    sButInit.pTip =
                        getStatName(psCurrStats as *mut libc::c_void);
                    BufferID = GetObjectBuffer();
                    if BufferID >= 0 as libc::c_int {
                    } else {
                        debug(LOG_ERROR,
                              b"Unable to acquire Topic buffer.\x00" as
                                  *const u8 as *const libc::c_char);
                    };
                    if BufferID >= 0 as libc::c_int {
                    } else {
                        debug(LOG_ERROR,
                              b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                                  *const u8 as *const libc::c_char,
                              b"design.c\x00" as *const u8 as
                                  *const libc::c_char, 2652 as libc::c_int,
                              (*::std::mem::transmute::<&[u8; 23],
                                                        &[libc::c_char; 23]>(b"intAddComponentButtons\x00")).as_ptr(),
                              b"BufferID >= 0\x00" as *const u8 as
                                  *const libc::c_char);
                    };
                    ObjectBuffers[BufferID as usize].InUse = 1 as libc::c_int;
                    ObjectBuffers[BufferID as usize].Data =
                        psCurrStats as *mut libc::c_void;
                    sButInit.pUserData =
                        &mut *ObjectBuffers.as_mut_ptr().offset(BufferID as
                                                                    isize) as
                            *mut RENDERED_BUTTON as *mut libc::c_void;
                    sButInit.pDisplay =
                        Some(intDisplayComponentButton as
                                 unsafe extern "C" fn(_: *mut _widget,
                                                      _: UDWORD, _: UDWORD,
                                                      _: *mut UDWORD) -> ());
                    if widgAddForm(psWScreen, &mut sButInit) == 0 {
                        return 0 as libc::c_int
                    }
                    /* Store the stat pointer in the list */
                    let fresh2 = numComponent;
                    numComponent = numComponent.wrapping_add(1);
                    let ref mut fresh3 =
                        *apsComponentList.offset(fresh2 as isize);
                    *fresh3 = psCurrStats;
                    /* If this matches the component ID lock the button */
                    if i == compID {
                        desCompID = sButInit.id;
                        widgSetButtonState(psWScreen, sButInit.id,
                                           0x2 as libc::c_int as UDWORD);
                        widgSetTabs(psWScreen, 5003 as libc::c_int as UDWORD,
                                    sButInit.majorID, sButInit.minorID);
                    }
                    // if this is a command droid that is in use or dead - make it unavailable
                    if statType((*psCurrStats).ref_0) ==
                           COMP_BRAIN as libc::c_int as libc::c_uint {
                        if !(*(psCurrStats as
                                   *mut COMMAND_DROID)).psDroid.is_null() ||
                               (*(psCurrStats as *mut COMMAND_DROID)).died !=
                                   0 {
                            widgSetButtonState(psWScreen, sButInit.id,
                                               0x1 as libc::c_int as UDWORD);
                        }
                    }
                    if WhichTab == 0 as libc::c_int as libc::c_uint {
                        /* Update the init struct for the next button */
                        sButInit.id =
                            (sButInit.id as
                                 libc::c_uint).wrapping_add(1 as libc::c_int
                                                                as
                                                                libc::c_uint)
                                as UDWORD as UDWORD;
                        sButInit.x =
                            (sButInit.x as libc::c_int +
                                 (60 as libc::c_int + 2 as libc::c_int)) as
                                SWORD;
                        if sButInit.x as libc::c_int + 60 as libc::c_int +
                               2 as libc::c_int >
                               132 as libc::c_int + 20 as libc::c_int -
                                   0 as libc::c_int {
                            sButInit.x = 2 as libc::c_int as SWORD;
                            sButInit.y =
                                (sButInit.y as libc::c_int +
                                     (46 as libc::c_int + 2 as libc::c_int))
                                    as SWORD
                        }
                        if sButInit.y as libc::c_int + 46 as libc::c_int +
                               2 as libc::c_int >
                               258 as libc::c_int - 40 as libc::c_int {
                            sButInit.y = 2 as libc::c_int as SWORD;
                            sButInit.majorID =
                                (sButInit.majorID as libc::c_int +
                                     1 as libc::c_int) as UWORD;
                            if sButInit.majorID as libc::c_int >=
                                   9 as libc::c_int {
                                debug(LOG_NEVER,
                                      b"Too many buttons for component form\x00"
                                          as *const u8 as
                                          *const libc::c_char);
                                return 0 as libc::c_int
                            }
                        }
                    } else {
                        /* Update the init struct for the next button */
                        sButInit.id =
                            (sButInit.id as
                                 libc::c_uint).wrapping_add(1 as libc::c_int
                                                                as
                                                                libc::c_uint)
                                as UDWORD as UDWORD;
                        sButInit.x =
                            (sButInit.x as libc::c_int +
                                 (60 as libc::c_int + 2 as libc::c_int)) as
                                SWORD;
                        if sButInit.x as libc::c_int + 60 as libc::c_int +
                               2 as libc::c_int >
                               132 as libc::c_int + 20 as libc::c_int -
                                   11 as libc::c_int {
                            sButInit.x = 2 as libc::c_int as SWORD;
                            sButInit.y =
                                (sButInit.y as libc::c_int +
                                     (46 as libc::c_int + 2 as libc::c_int))
                                    as SWORD
                        }
                        if sButInit.y as libc::c_int + 46 as libc::c_int +
                               2 as libc::c_int >
                               258 as libc::c_int - 40 as libc::c_int {
                            sButInit.y = 2 as libc::c_int as SWORD;
                            sButInit.minorID =
                                (sButInit.minorID as libc::c_int +
                                     1 as libc::c_int) as UWORD;
                            if sButInit.minorID as libc::c_int >=
                                   5 as libc::c_int {
                                debug(LOG_NEVER,
                                      b"Too many buttons for component form\x00"
                                          as *const u8 as
                                          *const libc::c_char);
                                return 0 as libc::c_int
                            }
                        }
                    }
                    /* Update the stats pointer for the next button */
                    psCurrStats =
                        (psCurrStats as *mut UBYTE).offset(size as isize) as
                            *mut COMP_BASE_STATS
                }
            }
        }
        i = i.wrapping_add(1)
    }
    //hack to sort out the tabs on the weapon form
    //need to check how many buttons have been added to see if need all the tabs that are there
    psTabForm =
        widgGetFromID(psWScreen, 5003 as libc::c_int as UDWORD) as
            *mut W_TABFORM;
    if !psTabForm.is_null() {
        numTabs = (*psTabForm).numMajor;
        if numComponent <
               (numTabs as libc::c_int * 8 as libc::c_int) as UDWORD {
            (*psTabForm).numMajor =
                numForms(numComponent, 8 as libc::c_int as UDWORD)
        }
    }
    return 1 as libc::c_int;
}
/* Add the component buttons to the main tab of the component form */
/* Return the location of a COMP_BASE_STATS */
/*static LOC intGetLocation(COMP_BASE_STATS *psStats)
{
	switch (statType(psStats->ref))
	{
	case COMP_SENSOR:
		return ((SENSOR_STATS *)psStats)->location;
		break;
	case COMP_ECM:
		return ((ECM_STATS *)psStats)->location;
		break;
	case COMP_REPAIRUNIT:
		return ((REPAIR_STATS *)psStats)->location;
		break;
	}

	// Nothing else has a location, so return Turret so it can be
	// selected on the design screen.
	//
	return LOC_TURRET;
}*/
/* Add the component buttons to the main tab of the component form */
unsafe extern "C" fn intAddExtraSystemButtons(mut sensorIndex: UDWORD,
                                              mut ecmIndex: UDWORD,
                                              mut constIndex: UDWORD,
                                              mut repairIndex: UDWORD,
                                              mut brainIndex: UDWORD)
 -> BOOL {
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
                   pFormDisplay: None,};
    let mut i: UDWORD = 0;
    let mut buttonType: UDWORD = 0;
    let mut size: UDWORD = 0 as libc::c_int as UDWORD;
    let mut compIndex: UDWORD = 0 as libc::c_int as UDWORD;
    let mut numStats: UDWORD = 0 as libc::c_int as UDWORD;
    let mut psCurrStats: *mut COMP_BASE_STATS = 0 as *mut COMP_BASE_STATS;
    let mut aAvailable: *mut UBYTE = 0 as *mut UBYTE;
    let mut aButText: [STRING; 6] = [0; 6];
    let mut BufferID: SDWORD = 0;
    memset(aButText.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           (5 as libc::c_int + 1 as libc::c_int) as libc::c_uint);
    memset(&mut sButInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_BUTINIT>() as libc::c_ulong);
    // Set up the button struct
    sButInit.formID = 5003 as libc::c_int as UDWORD;
    sButInit.majorID = 0 as libc::c_int as UWORD;
    sButInit.minorID = 0 as libc::c_int as UWORD;
    sButInit.id = 5700 as libc::c_int as UDWORD;
    sButInit.style = 4 as libc::c_int as UDWORD;
    sButInit.x = 2 as libc::c_int as SWORD;
    sButInit.y = 2 as libc::c_int as SWORD;
    sButInit.width = 60 as libc::c_int as UWORD;
    sButInit.height = 46 as libc::c_int as UWORD;
    // Add the buttons :
	// buttonType == 0  -  Sensor Buttons
	// buttonType == 1  -  ECM Buttons
	// buttonType == 2  -  Constructor Buttons
	// buttonType == 3  -  Repair Buttons
	// buttonType == 4  -  Brain Buttons
    numExtraSys = 0 as libc::c_int as UDWORD;
    buttonType = 0 as libc::c_int as UDWORD;
    while buttonType < 5 as libc::c_int as libc::c_uint {
        match buttonType {
            0 => {
                // Sensor Buttons
                psCurrStats = asSensorStats as *mut COMP_BASE_STATS;
                size = ::std::mem::size_of::<SENSOR_STATS>() as libc::c_ulong;
                aAvailable =
                    apCompLists[selectedPlayer as
                                    usize][COMP_SENSOR as libc::c_int as
                                               usize];
                //numStats = MaxComponents(numSensorStats);
                numStats = numSensorStats;
                compIndex = sensorIndex
            }
            1 => {
                // ECM Buttons
                psCurrStats = asECMStats as *mut COMP_BASE_STATS;
                size = ::std::mem::size_of::<ECM_STATS>() as libc::c_ulong;
                aAvailable =
                    apCompLists[selectedPlayer as
                                    usize][COMP_ECM as libc::c_int as usize];
                //numStats = MaxComponents(numECMStats);
                numStats = numECMStats;
                compIndex = ecmIndex
            }
            2 => {
                // Constructor Buttons
                psCurrStats = asConstructStats as *mut COMP_BASE_STATS;
                size =
                    ::std::mem::size_of::<CONSTRUCT_STATS>() as libc::c_ulong;
                aAvailable =
                    apCompLists[selectedPlayer as
                                    usize][COMP_CONSTRUCT as libc::c_int as
                                               usize];
                //numStats = MaxComponents(numConstructStats);
                numStats = numConstructStats;
                compIndex = constIndex
            }
            3 => {
                // Repair Buttons
                psCurrStats = asRepairStats as *mut COMP_BASE_STATS;
                size = ::std::mem::size_of::<REPAIR_STATS>() as libc::c_ulong;
                aAvailable =
                    apCompLists[selectedPlayer as
                                    usize][COMP_REPAIRUNIT as libc::c_int as
                                               usize];
                //numStats = MaxComponents(numECMStats);
                numStats = numRepairStats;
                compIndex = repairIndex
            }
            4 => {
                // Repair Buttons
                psCurrStats = asBrainStats as *mut COMP_BASE_STATS;
                size = ::std::mem::size_of::<BRAIN_STATS>() as libc::c_ulong;
                aAvailable =
                    apCompLists[selectedPlayer as
                                    usize][COMP_BRAIN as libc::c_int as
                                               usize];
                //numStats = MaxComponents(numECMStats);
                numStats = numBrainStats;
                compIndex = brainIndex
            }
            _ => { }
        }
        i = 0 as libc::c_int as UDWORD;
        while i < numStats {
            // If we are out of space in the list - stop
            if numExtraSys >= 40 as libc::c_int as libc::c_uint {
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"intAddExtraSystemButtons: Too many components for the list\x00"
                              as *const u8 as *const libc::c_char);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"design.c\x00" as *const u8 as *const libc::c_char,
                          2856 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 25],
                                                    &[libc::c_char; 25]>(b"intAddExtraSystemButtons\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
                return 0 as libc::c_int
            }
            // Skip unavailable entries or non-design ones
            if *aAvailable.offset(i as isize) as libc::c_int &
                   0x1 as libc::c_int == 0 || (*psCurrStats).design == 0 {
                // Update the stats pointer for the next button
                psCurrStats =
                    (psCurrStats as *mut UBYTE).offset(size as isize) as
                        *mut COMP_BASE_STATS
            } else {
                // Set the tip and add the button
                strncpy(aButText.as_mut_ptr(),
                        getStatName(psCurrStats as *mut libc::c_void),
                        5 as libc::c_int as libc::c_uint);
                sButInit.pTip = getStatName(psCurrStats as *mut libc::c_void);
                BufferID =
                    sButInit.id.wrapping_sub(5700 as libc::c_int as
                                                 libc::c_uint) as SDWORD;
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
                          b"design.c\x00" as *const u8 as *const libc::c_char,
                          2876 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 25],
                                                    &[libc::c_char; 25]>(b"intAddExtraSystemButtons\x00")).as_ptr(),
                          b"BufferID < NUM_OBJECTBUFFERS\x00" as *const u8 as
                              *const libc::c_char);
                };
                /*switch(buttonType) {
				case 0:
					RENDERBUTTON_INUSE(&System0Buffers[BufferID]);
					System0Buffers[BufferID].Data = psCurrStats;
					sButInit.pUserData = (void*)&System0Buffers[BufferID];
					break;
				case 1:
					RENDERBUTTON_INUSE(&System1Buffers[BufferID]);
					System1Buffers[BufferID].Data = psCurrStats;
					sButInit.pUserData = (void*)&System1Buffers[BufferID];
					break;
				case 2:
					RENDERBUTTON_INUSE(&System2Buffers[BufferID]);
					System2Buffers[BufferID].Data = psCurrStats;
					sButInit.pUserData = (void*)&System2Buffers[BufferID];
					break;
			}*/
			//just use one set of buffers for mixed system form
                System0Buffers[BufferID as usize].InUse = 1 as libc::c_int;
                if statType((*psCurrStats).ref_0) ==
                       COMP_BRAIN as libc::c_int as libc::c_uint {
                    System0Buffers[BufferID as usize].Data =
                        (*(psCurrStats as *mut BRAIN_STATS)).psWeaponStat as
                            *mut libc::c_void
                } else {
                    System0Buffers[BufferID as usize].Data =
                        psCurrStats as *mut libc::c_void
                }
                sButInit.pUserData =
                    &mut *System0Buffers.as_mut_ptr().offset(BufferID as
                                                                 isize) as
                        *mut RENDERED_BUTTON as *mut libc::c_void;
                sButInit.pDisplay =
                    Some(intDisplayComponentButton as
                             unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                                  _: UDWORD, _: *mut UDWORD)
                                 -> ());
                if widgAddForm(psWScreen, &mut sButInit) == 0 {
                    return 0 as libc::c_int
                }
                // Store the stat pointer in the list
                let fresh4 = numExtraSys;
                numExtraSys = numExtraSys.wrapping_add(1);
                let ref mut fresh5 = *apsExtraSysList.offset(fresh4 as isize);
                *fresh5 = psCurrStats;
                // If this matches the sensorIndex note the form and button
                if i == compIndex {
                    desCompID = sButInit.id;
                    widgSetButtonState(psWScreen, sButInit.id,
                                       0x2 as libc::c_int as UDWORD);
                    widgSetTabs(psWScreen, 5003 as libc::c_int as UDWORD,
                                sButInit.majorID, sButInit.minorID);
                }
                // Update the init struct for the next button
                sButInit.id =
                    (sButInit.id as
                         libc::c_uint).wrapping_add(1 as libc::c_int as
                                                        libc::c_uint) as
                        UDWORD as UDWORD;
                sButInit.x =
                    (sButInit.x as libc::c_int +
                         (60 as libc::c_int + 2 as libc::c_int)) as SWORD;
                if sButInit.x as libc::c_int + 60 as libc::c_int +
                       2 as libc::c_int >
                       132 as libc::c_int + 20 as libc::c_int -
                           11 as libc::c_int {
                    sButInit.x = 2 as libc::c_int as SWORD;
                    sButInit.y =
                        (sButInit.y as libc::c_int +
                             (46 as libc::c_int + 2 as libc::c_int)) as SWORD
                }
                if sButInit.y as libc::c_int + 46 as libc::c_int +
                       2 as libc::c_int >
                       258 as libc::c_int - 40 as libc::c_int {
                    sButInit.y = 2 as libc::c_int as SWORD;
                    sButInit.majorID =
                        (sButInit.majorID as libc::c_int + 1 as libc::c_int)
                            as UWORD
                }
                // Update the stats pointer for the next button
                psCurrStats =
                    (psCurrStats as *mut UBYTE).offset(size as isize) as
                        *mut COMP_BASE_STATS
            }
            i = i.wrapping_add(1)
        }
        buttonType = buttonType.wrapping_add(1)
    }
    return 1 as libc::c_int;
}
/* Set the bar graphs for the system clickable */
/* Set the bar graphs for the system clickable */
unsafe extern "C" fn intSetSystemStats(mut psStats: *mut COMP_BASE_STATS) {
    let mut psForm: *mut W_FORM = 0 as *mut W_FORM;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"intSetSystemStats: Invalid stats pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"design.c\x00" as *const u8 as *const libc::c_char,
              2958 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 18],
                                        &[libc::c_char; 18]>(b"intSetSystemStats\x00")).as_ptr(),
              b"PTRVALID(psStats, sizeof(COMP_BASE_STATS))\x00" as *const u8
                  as *const libc::c_char);
    };
    /* set form tip to stats string */
    widgSetTip(psWScreen, 5006 as libc::c_int as UDWORD,
               getStatName(psStats as *mut libc::c_void));
    /* set form stats for later display in intDisplayStatForm */
    psForm =
        widgGetFromID(psWScreen, 5006 as libc::c_int as UDWORD) as
            *mut W_FORM;
    if !psForm.is_null() {
        (*psForm).pUserData = psStats as *mut libc::c_void
    }
    /* Set the correct system stats */
    match statType((*psStats).ref_0) {
        6 => { intSetSensorStats(psStats as *mut SENSOR_STATS); }
        5 => { intSetECMStats(psStats as *mut ECM_STATS); }
        8 => { intSetWeaponStats(psStats as *mut WEAPON_STATS); }
        7 => { intSetConstructStats(psStats as *mut CONSTRUCT_STATS); }
        4 => { intSetRepairStats(psStats as *mut REPAIR_STATS); }
        _ => { }
    };
}
/* Set the shadow bar graphs for the system clickable */
/* Set the shadow bar graphs for the system clickable */
unsafe extern "C" fn intSetSystemShadowStats(mut psStats:
                                                 *mut COMP_BASE_STATS) {
    /* Set the correct system stats - psStats can be set to NULL if
	 * desSysMode does not match the type of the stats.
	 */
    if !psStats.is_null() {
        match statType((*psStats).ref_0) {
            6 => {
                if desSysMode as libc::c_uint ==
                       IDES_SENSOR as libc::c_int as libc::c_uint {
                    intSetSensorShadowStats(psStats as *mut SENSOR_STATS);
                } else { psStats = 0 as *mut COMP_BASE_STATS }
            }
            5 => {
                if desSysMode as libc::c_uint ==
                       IDES_ECM as libc::c_int as libc::c_uint {
                    intSetECMShadowStats(psStats as *mut ECM_STATS);
                } else { psStats = 0 as *mut COMP_BASE_STATS }
            }
            8 => {
                if desSysMode as libc::c_uint ==
                       IDES_WEAPON as libc::c_int as libc::c_uint {
                    intSetWeaponShadowStats(psStats as *mut WEAPON_STATS);
                } else { psStats = 0 as *mut COMP_BASE_STATS }
            }
            7 => {
                if desSysMode as libc::c_uint ==
                       IDES_CONSTRUCT as libc::c_int as libc::c_uint {
                    intSetConstructShadowStats(psStats as
                                                   *mut CONSTRUCT_STATS);
                } else { psStats = 0 as *mut COMP_BASE_STATS }
            }
            2 => { psStats = 0 as *mut COMP_BASE_STATS }
            4 => {
                if desSysMode as libc::c_uint ==
                       IDES_REPAIR as libc::c_int as libc::c_uint {
                    intSetRepairShadowStats(psStats as *mut REPAIR_STATS);
                } else { psStats = 0 as *mut COMP_BASE_STATS }
            }
            _ => { }
        }
    }
    if psStats.is_null() {
        match desSysMode as libc::c_uint {
            0 => { intSetSensorShadowStats(0 as *mut SENSOR_STATS); }
            1 => { intSetECMShadowStats(0 as *mut ECM_STATS); }
            4 => { intSetWeaponShadowStats(0 as *mut WEAPON_STATS); }
            2 => { intSetConstructShadowStats(0 as *mut CONSTRUCT_STATS); }
            3 => { intSetRepairShadowStats(0 as *mut REPAIR_STATS); }
            _ => { }
        }
    };
}
/* Set the bar graphs for the sensor stats */
/* Set the bar graphs for the sensor stats */
unsafe extern "C" fn intSetSensorStats(mut psStats: *mut SENSOR_STATS) {
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"intSetSensorStats: Invalid stats pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"design.c\x00" as *const u8 as *const libc::c_char,
              3086 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 18],
                                        &[libc::c_char; 18]>(b"intSetSensorStats\x00")).as_ptr(),
              b"PTRVALID(psStats, sizeof(SENSOR_STATS))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (*psStats).ref_0 >= 0x50000 as libc::c_int as libc::c_uint &&
           (*psStats).ref_0 <
               (0x50000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"intSetSensorStats: stats ref is out of range\x00" as *const u8
                  as *const libc::c_char);
    };
    if (*psStats).ref_0 >= 0x50000 as libc::c_int as libc::c_uint &&
           (*psStats).ref_0 <
               (0x50000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"design.c\x00" as *const u8 as *const libc::c_char,
              3089 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 18],
                                        &[libc::c_char; 18]>(b"intSetSensorStats\x00")).as_ptr(),
              b"(psStats->ref >= REF_SENSOR_START) && (psStats->ref < REF_SENSOR_START + REF_RANGE)\x00"
                  as *const u8 as *const libc::c_char);
    };
    /* range */
	//widgSetBarSize(psWScreen, IDDES_SENSORRANGE, psStats->range);
    widgSetBarSize(psWScreen, 5110 as libc::c_int as UDWORD,
                   sensorRange(psStats, selectedPlayer as UBYTE));
    /* power */
	//widgSetBarSize(psWScreen, IDDES_SENSORPOWER, psStats->power);
    widgSetBarSize(psWScreen, 5111 as libc::c_int as UDWORD,
                   sensorPower(psStats, selectedPlayer as UBYTE));
    /* weight */
    widgSetBarSize(psWScreen, 5112 as libc::c_int as UDWORD,
                   (*psStats).weight);
}
/* Set the shadow bar graphs for the sensor stats */
/* Set the shadow bar graphs for the sensor stats */
unsafe extern "C" fn intSetSensorShadowStats(mut psStats: *mut SENSOR_STATS) {
    if psStats.is_null() || 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"intSetSensorShadowStats: Invalid stats pointer\x00" as
                  *const u8 as *const libc::c_char);
    };
    if psStats.is_null() || 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"design.c\x00" as *const u8 as *const libc::c_char,
              3107 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 24],
                                        &[libc::c_char; 24]>(b"intSetSensorShadowStats\x00")).as_ptr(),
              b"psStats == NULL || PTRVALID(psStats, sizeof(SENSOR_STATS))\x00"
                  as *const u8 as *const libc::c_char);
    };
    if psStats.is_null() ||
           (*psStats).ref_0 >= 0x50000 as libc::c_int as libc::c_uint &&
               (*psStats).ref_0 <
                   (0x50000 as libc::c_int + 0x10000 as libc::c_int) as
                       libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"intSetSensorShadowStats: stats ref is out of range\x00" as
                  *const u8 as *const libc::c_char);
    };
    if psStats.is_null() ||
           (*psStats).ref_0 >= 0x50000 as libc::c_int as libc::c_uint &&
               (*psStats).ref_0 <
                   (0x50000 as libc::c_int + 0x10000 as libc::c_int) as
                       libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"design.c\x00" as *const u8 as *const libc::c_char,
              3111 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 24],
                                        &[libc::c_char; 24]>(b"intSetSensorShadowStats\x00")).as_ptr(),
              b"psStats == NULL || ((psStats->ref >= REF_SENSOR_START) && (psStats->ref < REF_SENSOR_START + REF_RANGE))\x00"
                  as *const u8 as *const libc::c_char);
    };
    if !psStats.is_null() {
        /* range */
		//widgSetMinorBarSize(psWScreen, IDDES_SENSORRANGE, psStats->range);
        widgSetMinorBarSize(psWScreen, 5110 as libc::c_int as UDWORD,
                            sensorRange(psStats, selectedPlayer as UBYTE));
        /* power */
		//widgSetMinorBarSize(psWScreen, IDDES_SENSORPOWER, psStats->power);
        widgSetMinorBarSize(psWScreen, 5111 as libc::c_int as UDWORD,
                            sensorPower(psStats, selectedPlayer as UBYTE));
        /* weight */
        widgSetMinorBarSize(psWScreen, 5112 as libc::c_int as UDWORD,
                            (*psStats).weight);
    } else {
        /* Remove the shadow bars */
        widgSetMinorBarSize(psWScreen, 5110 as libc::c_int as UDWORD,
                            0 as libc::c_int as UDWORD);
        widgSetMinorBarSize(psWScreen, 5111 as libc::c_int as UDWORD,
                            0 as libc::c_int as UDWORD);
        widgSetMinorBarSize(psWScreen, 5112 as libc::c_int as UDWORD,
                            0 as libc::c_int as UDWORD);
    };
}
/* Set the bar graphs for the ECM stats */
/* Set the bar graphs for the ECM stats */
unsafe extern "C" fn intSetECMStats(mut psStats: *mut ECM_STATS) {
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"intSetECMStats: Invalid stats pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"design.c\x00" as *const u8 as *const libc::c_char,
              3140 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 15],
                                        &[libc::c_char; 15]>(b"intSetECMStats\x00")).as_ptr(),
              b"PTRVALID(psStats, sizeof(ECM_STATS))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (*psStats).ref_0 >= 0x60000 as libc::c_int as libc::c_uint &&
           (*psStats).ref_0 <
               (0x60000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"intSetECMStats: stats ref is out of range\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (*psStats).ref_0 >= 0x60000 as libc::c_int as libc::c_uint &&
           (*psStats).ref_0 <
               (0x60000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"design.c\x00" as *const u8 as *const libc::c_char,
              3143 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 15],
                                        &[libc::c_char; 15]>(b"intSetECMStats\x00")).as_ptr(),
              b"(psStats->ref >= REF_ECM_START) && (psStats->ref < REF_ECM_START + REF_RANGE)\x00"
                  as *const u8 as *const libc::c_char);
    };
    /* power */
	//widgSetBarSize(psWScreen, IDDES_ECMPOWER, psStats->power);
    widgSetBarSize(psWScreen, 5115 as libc::c_int as UDWORD,
                   ecmPower(psStats, selectedPlayer as UBYTE));
    /* weight */
    widgSetBarSize(psWScreen, 5116 as libc::c_int as UDWORD,
                   (*psStats).weight);
}
/* Set the shadow bar graphs for the ECM stats */
/* Set the shadow bar graphs for the ECM stats */
unsafe extern "C" fn intSetECMShadowStats(mut psStats: *mut ECM_STATS) {
    if psStats.is_null() || 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"intSetECMShadowStats: Invalid stats pointer\x00" as *const u8
                  as *const libc::c_char);
    };
    if psStats.is_null() || 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"design.c\x00" as *const u8 as *const libc::c_char,
              3157 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 21],
                                        &[libc::c_char; 21]>(b"intSetECMShadowStats\x00")).as_ptr(),
              b"psStats == NULL || PTRVALID(psStats, sizeof(ECM_STATS))\x00"
                  as *const u8 as *const libc::c_char);
    };
    if psStats.is_null() ||
           (*psStats).ref_0 >= 0x60000 as libc::c_int as libc::c_uint &&
               (*psStats).ref_0 <
                   (0x60000 as libc::c_int + 0x10000 as libc::c_int) as
                       libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"intSetECMShadowStats: stats ref is out of range\x00" as
                  *const u8 as *const libc::c_char);
    };
    if psStats.is_null() ||
           (*psStats).ref_0 >= 0x60000 as libc::c_int as libc::c_uint &&
               (*psStats).ref_0 <
                   (0x60000 as libc::c_int + 0x10000 as libc::c_int) as
                       libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"design.c\x00" as *const u8 as *const libc::c_char,
              3161 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 21],
                                        &[libc::c_char; 21]>(b"intSetECMShadowStats\x00")).as_ptr(),
              b"psStats == NULL || ((psStats->ref >= REF_ECM_START) && (psStats->ref < REF_ECM_START + REF_RANGE))\x00"
                  as *const u8 as *const libc::c_char);
    };
    if !psStats.is_null() {
        /* power */
		//widgSetMinorBarSize(psWScreen, IDDES_ECMPOWER, psStats->power);
        widgSetMinorBarSize(psWScreen, 5115 as libc::c_int as UDWORD,
                            ecmPower(psStats, selectedPlayer as UBYTE));
        /* weight */
        widgSetMinorBarSize(psWScreen, 5116 as libc::c_int as UDWORD,
                            (*psStats).weight);
    } else {
        /* Remove the shadow bars */
        widgSetMinorBarSize(psWScreen, 5115 as libc::c_int as UDWORD,
                            0 as libc::c_int as UDWORD);
        widgSetMinorBarSize(psWScreen, 5116 as libc::c_int as UDWORD,
                            0 as libc::c_int as UDWORD);
    };
}
/* Set the bar graphs for the Constructor stats */
/* Set the bar graphs for the Constructor stats */
unsafe extern "C" fn intSetConstructStats(mut psStats: *mut CONSTRUCT_STATS) {
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"intSetConstructStats: Invalid stats pointer\x00" as *const u8
                  as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"design.c\x00" as *const u8 as *const libc::c_char,
              3185 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 21],
                                        &[libc::c_char; 21]>(b"intSetConstructStats\x00")).as_ptr(),
              b"PTRVALID(psStats, sizeof(CONSTRUCT_STATS))\x00" as *const u8
                  as *const libc::c_char);
    };
    if (*psStats).ref_0 >= 0xf0000 as libc::c_int as libc::c_uint &&
           (*psStats).ref_0 <
               (0xf0000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"intSetConstructStats: stats ref is out of range\x00" as
                  *const u8 as *const libc::c_char);
    };
    if (*psStats).ref_0 >= 0xf0000 as libc::c_int as libc::c_uint &&
           (*psStats).ref_0 <
               (0xf0000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"design.c\x00" as *const u8 as *const libc::c_char,
              3188 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 21],
                                        &[libc::c_char; 21]>(b"intSetConstructStats\x00")).as_ptr(),
              b"(psStats->ref >= REF_CONSTRUCT_START) && (psStats->ref < REF_CONSTRUCT_START + REF_RANGE)\x00"
                  as *const u8 as *const libc::c_char);
    };
    /* power */
	//widgSetBarSize(psWScreen, IDDES_CONSTPOINTS, psStats->constructPoints);
    widgSetBarSize(psWScreen, 5125 as libc::c_int as UDWORD,
                   constructorPoints(psStats, selectedPlayer as UBYTE));
    /* weight */
    widgSetBarSize(psWScreen, 5126 as libc::c_int as UDWORD,
                   (*psStats).weight);
}
/* Set the shadow bar graphs for the Constructor stats */
/* Set the shadow bar graphs for the Constructor stats */
unsafe extern "C" fn intSetConstructShadowStats(mut psStats:
                                                    *mut CONSTRUCT_STATS) {
    if psStats.is_null() || 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"intSetConstructShadowStats: Invalid stats pointer\x00" as
                  *const u8 as *const libc::c_char);
    };
    if psStats.is_null() || 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"design.c\x00" as *const u8 as *const libc::c_char,
              3203 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 27],
                                        &[libc::c_char; 27]>(b"intSetConstructShadowStats\x00")).as_ptr(),
              b"psStats == NULL || PTRVALID(psStats, sizeof(CONSTRUCT_STATS))\x00"
                  as *const u8 as *const libc::c_char);
    };
    if psStats.is_null() ||
           (*psStats).ref_0 >= 0xf0000 as libc::c_int as libc::c_uint &&
               (*psStats).ref_0 <
                   (0xf0000 as libc::c_int + 0x10000 as libc::c_int) as
                       libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"intSetConstructShadowStats: stats ref is out of range\x00" as
                  *const u8 as *const libc::c_char);
    };
    if psStats.is_null() ||
           (*psStats).ref_0 >= 0xf0000 as libc::c_int as libc::c_uint &&
               (*psStats).ref_0 <
                   (0xf0000 as libc::c_int + 0x10000 as libc::c_int) as
                       libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"design.c\x00" as *const u8 as *const libc::c_char,
              3207 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 27],
                                        &[libc::c_char; 27]>(b"intSetConstructShadowStats\x00")).as_ptr(),
              b"psStats == NULL || ((psStats->ref >= REF_CONSTRUCT_START) && (psStats->ref < REF_CONSTRUCT_START + REF_RANGE))\x00"
                  as *const u8 as *const libc::c_char);
    };
    if !psStats.is_null() {
        /* power */
		//widgSetMinorBarSize(psWScreen, IDDES_CONSTPOINTS, psStats->constructPoints);
        widgSetMinorBarSize(psWScreen, 5125 as libc::c_int as UDWORD,
                            constructorPoints(psStats,
                                              selectedPlayer as UBYTE));
        /* weight */
        widgSetMinorBarSize(psWScreen, 5126 as libc::c_int as UDWORD,
                            (*psStats).weight);
    } else {
        /* reset the shadow bars */
        widgSetMinorBarSize(psWScreen, 5125 as libc::c_int as UDWORD,
                            0 as libc::c_int as UDWORD);
        widgSetMinorBarSize(psWScreen, 5126 as libc::c_int as UDWORD,
                            0 as libc::c_int as UDWORD);
    };
}
/* Set the bar graphs for the Repair stats */
/* Set the bar graphs for the Repair stats */
unsafe extern "C" fn intSetRepairStats(mut psStats: *mut REPAIR_STATS) {
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"intSetRepairStats: Invalid stats pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"design.c\x00" as *const u8 as *const libc::c_char,
              3230 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 18],
                                        &[libc::c_char; 18]>(b"intSetRepairStats\x00")).as_ptr(),
              b"PTRVALID(psStats, sizeof(REPAIR_STATS))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (*psStats).ref_0 >= 0x80000 as libc::c_int as libc::c_uint &&
           (*psStats).ref_0 <
               (0x80000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"intSetRepairStats: stats ref is out of range\x00" as *const u8
                  as *const libc::c_char);
    };
    if (*psStats).ref_0 >= 0x80000 as libc::c_int as libc::c_uint &&
           (*psStats).ref_0 <
               (0x80000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"design.c\x00" as *const u8 as *const libc::c_char,
              3233 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 18],
                                        &[libc::c_char; 18]>(b"intSetRepairStats\x00")).as_ptr(),
              b"(psStats->ref >= REF_REPAIR_START) && (psStats->ref < REF_REPAIR_START + REF_RANGE)\x00"
                  as *const u8 as *const libc::c_char);
    };
    /* power */
	//widgSetBarSize(psWScreen, IDDES_REPAIRPOINTS, psStats->repairPoints);
    widgSetBarSize(psWScreen, 5129 as libc::c_int as UDWORD,
                   repairPoints(psStats, selectedPlayer as UBYTE));
    /* weight */
    widgSetBarSize(psWScreen, 5130 as libc::c_int as UDWORD,
                   (*psStats).weight);
}
/* Set the shadow bar graphs for the Repair stats */
/* Set the shadow bar graphs for the Repair stats */
unsafe extern "C" fn intSetRepairShadowStats(mut psStats: *mut REPAIR_STATS) {
    if psStats.is_null() || 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"intSetRepairShadowStats: Invalid stats pointer\x00" as
                  *const u8 as *const libc::c_char);
    };
    if psStats.is_null() || 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"design.c\x00" as *const u8 as *const libc::c_char,
              3248 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 24],
                                        &[libc::c_char; 24]>(b"intSetRepairShadowStats\x00")).as_ptr(),
              b"psStats == NULL || PTRVALID(psStats, sizeof(REPAIR_STATS))\x00"
                  as *const u8 as *const libc::c_char);
    };
    if psStats.is_null() ||
           (*psStats).ref_0 >= 0x80000 as libc::c_int as libc::c_uint &&
               (*psStats).ref_0 <
                   (0x80000 as libc::c_int + 0x10000 as libc::c_int) as
                       libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"intSetRepairShadowStats: stats ref is out of range\x00" as
                  *const u8 as *const libc::c_char);
    };
    if psStats.is_null() ||
           (*psStats).ref_0 >= 0x80000 as libc::c_int as libc::c_uint &&
               (*psStats).ref_0 <
                   (0x80000 as libc::c_int + 0x10000 as libc::c_int) as
                       libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"design.c\x00" as *const u8 as *const libc::c_char,
              3252 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 24],
                                        &[libc::c_char; 24]>(b"intSetRepairShadowStats\x00")).as_ptr(),
              b"psStats == NULL || ((psStats->ref >= REF_REPAIR_START) && (psStats->ref < REF_REPAIR_START + REF_RANGE))\x00"
                  as *const u8 as *const libc::c_char);
    };
    if !psStats.is_null() {
        /* power */
		//widgSetMinorBarSize(psWScreen, IDDES_REPAIRPOINTS, psStats->repairPoints);
        widgSetMinorBarSize(psWScreen, 5129 as libc::c_int as UDWORD,
                            repairPoints(psStats, selectedPlayer as UBYTE));
        /* weight */
        widgSetMinorBarSize(psWScreen, 5130 as libc::c_int as UDWORD,
                            (*psStats).weight);
    } else {
        /* reset the shadow bars */
        widgSetMinorBarSize(psWScreen, 5129 as libc::c_int as UDWORD,
                            0 as libc::c_int as UDWORD);
        widgSetMinorBarSize(psWScreen, 5130 as libc::c_int as UDWORD,
                            0 as libc::c_int as UDWORD);
    };
}
/* Set the bar graphs for the Weapon stats */
/* Set the bar graphs for the Weapon stats */
unsafe extern "C" fn intSetWeaponStats(mut psStats: *mut WEAPON_STATS) {
    let mut size: UDWORD = 0;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"intSetWeaponStats: Invalid stats pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"design.c\x00" as *const u8 as *const libc::c_char,
              3278 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 18],
                                        &[libc::c_char; 18]>(b"intSetWeaponStats\x00")).as_ptr(),
              b"PTRVALID(psStats, sizeof(WEAPON_STATS))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (*psStats).ref_0 >= 0xa0000 as libc::c_int as libc::c_uint &&
           (*psStats).ref_0 <
               (0xa0000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"intSetWeaponStats: stats ref is out of range\x00" as *const u8
                  as *const libc::c_char);
    };
    if (*psStats).ref_0 >= 0xa0000 as libc::c_int as libc::c_uint &&
           (*psStats).ref_0 <
               (0xa0000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"design.c\x00" as *const u8 as *const libc::c_char,
              3281 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 18],
                                        &[libc::c_char; 18]>(b"intSetWeaponStats\x00")).as_ptr(),
              b"(psStats->ref >= REF_WEAPON_START) && (psStats->ref < REF_WEAPON_START + REF_RANGE)\x00"
                  as *const u8 as *const libc::c_char);
    };
    /* range */
    widgSetBarSize(psWScreen, 5120 as libc::c_int as UDWORD,
                   proj_GetLongRange(psStats, 0 as libc::c_int) as UDWORD);
    /* rate of fire */
	/*size = (DBAR_WEAPMAXROF - (psStats->firePause + (psStats->
		firePause *	asWeaponUpgrade[selectedPlayer][psStats->weaponSubClass].
		firePause)/100));*/
	//size = DBAR_WEAPMAXROF - weaponFirePause(psStats, (UBYTE)selectedPlayer);
	//size = weaponFirePause(psStats, (UBYTE)selectedPlayer);
    size = weaponROF(psStats) as UDWORD;
    /*if (size != 0)
	{
		size = ONEMIN / size;
	}*/
    //This Hack not needed anymore!!!
	/* Hack to set the ROF to zero for the NULL weapon */
	/*if (psStats == asWeaponStats)
	{
		size = 0;
	}*/
    widgSetBarSize(psWScreen, 5122 as libc::c_int as UDWORD, size);
    /* damage */
	//widgSetBarSize(psWScreen, IDDES_WEAPDAMAGE, psStats->damage);
    widgSetBarSize(psWScreen, 5121 as libc::c_int as UDWORD,
                   weaponDamage(psStats, selectedPlayer as UBYTE) as UWORD as
                       UDWORD);
    /* weight */
    widgSetBarSize(psWScreen, 5123 as libc::c_int as UDWORD,
                   (*psStats).weight);
}
/* Set the shadow bar graphs for the weapon stats */
/* Set the shadow bar graphs for the Weapon stats */
unsafe extern "C" fn intSetWeaponShadowStats(mut psStats: *mut WEAPON_STATS) {
    let mut size: UDWORD = 0;
    if psStats.is_null() || 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"intSetWeaponShadowStats: Invalid stats pointer\x00" as
                  *const u8 as *const libc::c_char);
    };
    if psStats.is_null() || 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"design.c\x00" as *const u8 as *const libc::c_char,
              3317 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 24],
                                        &[libc::c_char; 24]>(b"intSetWeaponShadowStats\x00")).as_ptr(),
              b"psStats == NULL || PTRVALID(psStats, sizeof(WEAPON_STATS))\x00"
                  as *const u8 as *const libc::c_char);
    };
    if psStats.is_null() ||
           (*psStats).ref_0 >= 0xa0000 as libc::c_int as libc::c_uint &&
               (*psStats).ref_0 <
                   (0xa0000 as libc::c_int + 0x10000 as libc::c_int) as
                       libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"intSetWeaponShadowStats: stats ref is out of range\x00" as
                  *const u8 as *const libc::c_char);
    };
    if psStats.is_null() ||
           (*psStats).ref_0 >= 0xa0000 as libc::c_int as libc::c_uint &&
               (*psStats).ref_0 <
                   (0xa0000 as libc::c_int + 0x10000 as libc::c_int) as
                       libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"design.c\x00" as *const u8 as *const libc::c_char,
              3321 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 24],
                                        &[libc::c_char; 24]>(b"intSetWeaponShadowStats\x00")).as_ptr(),
              b"psStats == NULL || ((psStats->ref >= REF_WEAPON_START) && (psStats->ref < REF_WEAPON_START + REF_RANGE))\x00"
                  as *const u8 as *const libc::c_char);
    };
    if !psStats.is_null() {
        /* range */
        widgSetMinorBarSize(psWScreen, 5120 as libc::c_int as UDWORD,
                            proj_GetLongRange(psStats, 0 as libc::c_int) as
                                UDWORD);
        /* rate of fire */
		/*size = (DBAR_WEAPMAXROF - (psStats->firePause + (psStats->
			firePause *	asWeaponUpgrade[selectedPlayer][psStats->weaponSubClass].
			firePause)/100));*/
		//size = DBAR_WEAPMAXROF - weaponFirePause(psStats, (UBYTE)selectedPlayer);
		//widgSetMinorBarSize(psWScreen, IDDES_WEAPROF, size);
		/*size = weaponFirePause(psStats, (UBYTE)selectedPlayer);
		if (size != 0)
		{
			size = ONEMIN / size;
		}*/
        size = weaponROF(psStats) as UDWORD;
        //This Hack not needed anymore!!!
    	/* Hack to set the ROF to zero for the NULL weapon */
	    /*if (psStats == asWeaponStats)
	    {
		    size = 0;
	    }*/
        widgSetMinorBarSize(psWScreen, 5122 as libc::c_int as UDWORD, size);
        /* damage */
		//widgSetMinorBarSize(psWScreen, IDDES_WEAPDAMAGE, psStats->damage);
        widgSetMinorBarSize(psWScreen, 5121 as libc::c_int as UDWORD,
                            weaponDamage(psStats, selectedPlayer as UBYTE) as
                                UWORD as UDWORD);
        /* weight */
        widgSetMinorBarSize(psWScreen, 5123 as libc::c_int as UDWORD,
                            (*psStats).weight);
    } else {
        /* Reset the shadow bars */
        widgSetMinorBarSize(psWScreen, 5120 as libc::c_int as UDWORD,
                            0 as libc::c_int as UDWORD);
        widgSetMinorBarSize(psWScreen, 5122 as libc::c_int as UDWORD,
                            0 as libc::c_int as UDWORD);
        widgSetMinorBarSize(psWScreen, 5121 as libc::c_int as UDWORD,
                            0 as libc::c_int as UDWORD);
        widgSetMinorBarSize(psWScreen, 5123 as libc::c_int as UDWORD,
                            0 as libc::c_int as UDWORD);
    };
}
/* Set the bar graphs for the Body stats */
/* Set the bar graphs for the Body stats */
unsafe extern "C" fn intSetBodyStats(mut psStats: *mut BODY_STATS) {
    let mut psForm: *mut W_FORM = 0 as *mut W_FORM;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"intSetBodyStats: Invalid stats pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"design.c\x00" as *const u8 as *const libc::c_char,
              3369 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 16],
                                        &[libc::c_char; 16]>(b"intSetBodyStats\x00")).as_ptr(),
              b"PTRVALID(psStats, sizeof(BODY_STATS))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (*psStats).ref_0 >= 0x10000 as libc::c_int as libc::c_uint &&
           (*psStats).ref_0 <
               (0x10000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"intSetBodyStats: stats ref is out of range\x00" as *const u8
                  as *const libc::c_char);
    };
    if (*psStats).ref_0 >= 0x10000 as libc::c_int as libc::c_uint &&
           (*psStats).ref_0 <
               (0x10000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"design.c\x00" as *const u8 as *const libc::c_char,
              3372 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 16],
                                        &[libc::c_char; 16]>(b"intSetBodyStats\x00")).as_ptr(),
              b"(psStats->ref >= REF_BODY_START) && (psStats->ref < REF_BODY_START + REF_RANGE)\x00"
                  as *const u8 as *const libc::c_char);
    };
    /* set form tip to stats string */
    widgSetTip(psWScreen, 5007 as libc::c_int as UDWORD,
               getStatName(psStats as *mut libc::c_void));
    /* armour */
	//	size = WBAR_SCALE * psStats->armourValue/DBAR_BODYMAXARMOUR;
	//do kinetic armour
	//widgSetBarSize(psWScreen, IDDES_BODYARMOUR_K, psStats->armourValue[WC_KINETIC]);
    widgSetBarSize(psWScreen, 5100 as libc::c_int as UDWORD,
                   bodyArmour(psStats, selectedPlayer as UBYTE,
                              0 as libc::c_int as UBYTE, WC_KINETIC));
    //do heat armour
	//widgSetBarSize(psWScreen, IDDES_BODYARMOUR_H, psStats->armourValue[WC_HEAT] );
    widgSetBarSize(psWScreen, 5128 as libc::c_int as UDWORD,
                   bodyArmour(psStats, selectedPlayer as UBYTE,
                              0 as libc::c_int as UBYTE, WC_HEAT));
    /* body points */
	/*size = WBAR_SCALE * psStats->body/DBAR_BODYMAXPOINTS;
	if (size > WBAR_SCALE)
	{
		size = WBAR_SCALE;
	}
	widgSetBarSize(psWScreen, IDDES_BODYPOINTS, size);*/
	/* power */
	//widgSetBarSize(psWScreen, IDDES_BODYPOWER, psStats->powerOutput);
    widgSetBarSize(psWScreen, 5101 as libc::c_int as UDWORD,
                   bodyPower(psStats, selectedPlayer as UBYTE,
                             0 as libc::c_int as UBYTE));
    /* weight */
    widgSetBarSize(psWScreen, 5102 as libc::c_int as UDWORD,
                   (*psStats).weight);
    /* set form stats for later display in intDisplayStatForm */
    psForm =
        widgGetFromID(psWScreen, 5007 as libc::c_int as UDWORD) as
            *mut W_FORM;
    if !psForm.is_null() {
        (*psForm).pUserData = psStats as *mut libc::c_void
    };
}
/* Set the shadow bar graphs for the Body stats */
/* Set the shadow bar graphs for the Body stats */
unsafe extern "C" fn intSetBodyShadowStats(mut psStats: *mut BODY_STATS) {
    if psStats.is_null() || 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"intSetBodyShadowStats: Invalid stats pointer\x00" as *const u8
                  as *const libc::c_char);
    };
    if psStats.is_null() || 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"design.c\x00" as *const u8 as *const libc::c_char,
              3414 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 22],
                                        &[libc::c_char; 22]>(b"intSetBodyShadowStats\x00")).as_ptr(),
              b"psStats == NULL || PTRVALID(psStats, sizeof(BODY_STATS))\x00"
                  as *const u8 as *const libc::c_char);
    };
    if psStats.is_null() ||
           (*psStats).ref_0 >= 0x10000 as libc::c_int as libc::c_uint &&
               (*psStats).ref_0 <
                   (0x10000 as libc::c_int + 0x10000 as libc::c_int) as
                       libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"intSetBodyShadowStats: stats ref is out of range\x00" as
                  *const u8 as *const libc::c_char);
    };
    if psStats.is_null() ||
           (*psStats).ref_0 >= 0x10000 as libc::c_int as libc::c_uint &&
               (*psStats).ref_0 <
                   (0x10000 as libc::c_int + 0x10000 as libc::c_int) as
                       libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"design.c\x00" as *const u8 as *const libc::c_char,
              3418 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 22],
                                        &[libc::c_char; 22]>(b"intSetBodyShadowStats\x00")).as_ptr(),
              b"psStats == NULL || ((psStats->ref >= REF_BODY_START) && (psStats->ref < REF_BODY_START + REF_RANGE))\x00"
                  as *const u8 as *const libc::c_char);
    };
    if !psStats.is_null() {
        /* armour - kinetic*/
		//size = WBAR_SCALE * psStats->armourValue/DBAR_BODYMAXARMOUR;
		//widgSetMinorBarSize(psWScreen, IDDES_BODYARMOUR_K,psStats->armourValue[WC_KINETIC]);
        widgSetMinorBarSize(psWScreen, 5100 as libc::c_int as UDWORD,
                            bodyArmour(psStats, selectedPlayer as UBYTE,
                                       0 as libc::c_int as UBYTE,
                                       WC_KINETIC));
        //armour - heat
		//widgSetMinorBarSize(psWScreen, IDDES_BODYARMOUR_H,psStats->armourValue[WC_HEAT]);
        widgSetMinorBarSize(psWScreen, 5128 as libc::c_int as UDWORD,
                            bodyArmour(psStats, selectedPlayer as UBYTE,
                                       0 as libc::c_int as UBYTE, WC_HEAT));
        /* body points */
//			size = WBAR_SCALE * psStats->bodyPoints/DBAR_BODYMAXPOINTS;
//			if (size > WBAR_SCALE)
//			{
//				size = WBAR_SCALE;
//			}
//			widgSetMinorBarSize(psWScreen, IDDES_BODYPOINTS, size);
		/* power */
		//widgSetMinorBarSize(psWScreen, IDDES_BODYPOWER, psStats->powerOutput);
        widgSetMinorBarSize(psWScreen, 5101 as libc::c_int as UDWORD,
                            bodyPower(psStats, selectedPlayer as UBYTE,
                                      0 as libc::c_int as UBYTE));
        /* weight */
        widgSetMinorBarSize(psWScreen, 5102 as libc::c_int as UDWORD,
                            (*psStats).weight);
    } else {
        /* Reset the shadow bars */
        widgSetMinorBarSize(psWScreen, 5100 as libc::c_int as UDWORD,
                            0 as libc::c_int as UDWORD);
        widgSetMinorBarSize(psWScreen, 5128 as libc::c_int as UDWORD,
                            0 as libc::c_int as UDWORD);
        //		widgSetMinorBarSize(psWScreen, IDDES_BODYPOINTS, 0);
        widgSetMinorBarSize(psWScreen, 5101 as libc::c_int as UDWORD,
                            0 as libc::c_int as UDWORD);
        widgSetMinorBarSize(psWScreen, 5102 as libc::c_int as UDWORD,
                            0 as libc::c_int as UDWORD);
    };
}
/* Sets the Design Power Bar for a given Template */
/* Sets the Design Power Bar for a given Template */
unsafe extern "C" fn intSetDesignPower(mut psTemplate: *mut DROID_TEMPLATE) {
    /* use the same scale as PowerBar in main window so values are relative */
    widgSetBarSize(psWScreen, 5023 as libc::c_int as UDWORD,
                   calcTemplatePower(psTemplate));
}
/* Sets the Power shadow Bar for the current Template with new stat*/
/* Set the shadow bar graphs for the template power points - psStats is new hilited stats*/
unsafe extern "C" fn intSetTemplatePowerShadowStats(mut psStats:
                                                        *mut COMP_BASE_STATS) {
    let mut type_0: UDWORD = 0;
    //SDWORD				Avail, Used, Total;
    let mut compTempl: DROID_TEMPLATE =
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
    if !(&mut sCurrDesign as *mut DROID_TEMPLATE).is_null() &&
           !psStats.is_null() {
        //create the comparison Template
        memcpy(&mut compTempl as *mut DROID_TEMPLATE as *mut libc::c_void,
               &mut sCurrDesign as *mut DROID_TEMPLATE as *const libc::c_void,
               ::std::mem::size_of::<DROID_TEMPLATE>() as libc::c_ulong);
        type_0 = statType((*psStats).ref_0);
        /*if type = BODY or PROPULSION can do a straight comparison but if the new stat is
		a 'system' stat then need to find out which 'system' is currently in place so the
		comparison is meaningful*/
        if desCompMode as libc::c_uint ==
               IDES_SYSTEM as libc::c_int as libc::c_uint {
            //work out current system component
            if sCurrDesign.asParts[COMP_ECM as libc::c_int as usize] != 0 {
                type_0 = COMP_ECM as libc::c_int as UDWORD
            } else if sCurrDesign.asParts[COMP_SENSOR as libc::c_int as usize]
                          != 0 {
                type_0 = COMP_SENSOR as libc::c_int as UDWORD
            } else if sCurrDesign.asParts[COMP_CONSTRUCT as libc::c_int as
                                              usize] != 0 {
                type_0 = COMP_CONSTRUCT as libc::c_int as UDWORD
            } else if sCurrDesign.asParts[COMP_REPAIRUNIT as libc::c_int as
                                              usize] != 0 {
                type_0 = COMP_REPAIRUNIT as libc::c_int as UDWORD
            } else if sCurrDesign.asWeaps[0 as libc::c_int as usize] != 0 {
                type_0 = COMP_WEAPON as libc::c_int as UDWORD
            } else { type_0 = COMP_UNKNOWN as libc::c_int as UDWORD }
        }
        match type_0 {
            1 => {
                compTempl.asParts[COMP_BODY as libc::c_int as usize] =
                    (psStats as
                         *mut BODY_STATS).wrapping_offset_from(asBodyStats) as
                        libc::c_int
                //default:
			//don't want to draw for unknown comp
            }
            3 => {
                compTempl.asParts[COMP_PROPULSION as libc::c_int as usize] =
                    (psStats as
                         *mut PROPULSION_STATS).wrapping_offset_from(asPropulsionStats)
                        as libc::c_int
            }
            5 => {
                compTempl.asParts[COMP_ECM as libc::c_int as usize] =
                    (psStats as
                         *mut ECM_STATS).wrapping_offset_from(asECMStats) as
                        libc::c_int
            }
            6 => {
                compTempl.asParts[COMP_SENSOR as libc::c_int as usize] =
                    (psStats as
                         *mut SENSOR_STATS).wrapping_offset_from(asSensorStats)
                        as libc::c_int
            }
            7 => {
                compTempl.asParts[COMP_CONSTRUCT as libc::c_int as usize] =
                    (psStats as
                         *mut CONSTRUCT_STATS).wrapping_offset_from(asConstructStats)
                        as libc::c_int
            }
            4 => {
                compTempl.asParts[COMP_REPAIRUNIT as libc::c_int as usize] =
                    (psStats as
                         *mut REPAIR_STATS).wrapping_offset_from(asRepairStats)
                        as libc::c_int
            }
            8 => {
                compTempl.asWeaps[0 as libc::c_int as usize] =
                    (psStats as
                         *mut WEAPON_STATS).wrapping_offset_from(asWeaponStats)
                        as libc::c_int as UDWORD
            }
            _ => { }
        }
        widgSetMinorBarSize(psWScreen, 5023 as libc::c_int as UDWORD,
                            calcTemplatePower(&mut compTempl));
    } else {
        /* Reset the shadow bar */
        widgSetMinorBarSize(psWScreen, 5023 as libc::c_int as UDWORD,
                            0 as libc::c_int as UDWORD);
    };
}
/* Sets the Body Points Bar for a given Template */
/* Sets the Body Points Bar for a given Template */
unsafe extern "C" fn intSetBodyPoints(mut psTemplate: *mut DROID_TEMPLATE) {
    // If total greater than Body Bar size then scale values.
    widgSetBarSize(psWScreen, 5127 as libc::c_int as UDWORD,
                   calcTemplateBody(psTemplate, selectedPlayer as UBYTE));
}
/* Sets the Body Points shadow Bar for the current Template with new stat*/
/* Set the shadow bar graphs for the template Body points - psStats is new hilited stats*/
unsafe extern "C" fn intSetTemplateBodyShadowStats(mut psStats:
                                                       *mut COMP_BASE_STATS) {
    let mut type_0: UDWORD = 0;
    let mut compTempl: DROID_TEMPLATE =
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
    if !(&mut sCurrDesign as *mut DROID_TEMPLATE).is_null() &&
           !psStats.is_null() {
        //create the comparison Template
        memcpy(&mut compTempl as *mut DROID_TEMPLATE as *mut libc::c_void,
               &mut sCurrDesign as *mut DROID_TEMPLATE as *const libc::c_void,
               ::std::mem::size_of::<DROID_TEMPLATE>() as libc::c_ulong);
        type_0 = statType((*psStats).ref_0);
        /*if type = BODY or PROPULSION can do a straight comparison but if the new stat is
		a 'system' stat then need to find out which 'system' is currently in place so the
		comparison is meaningful*/
        if desCompMode as libc::c_uint ==
               IDES_SYSTEM as libc::c_int as libc::c_uint {
            //work out current system component
            if sCurrDesign.asParts[COMP_ECM as libc::c_int as usize] != 0 {
                type_0 = COMP_ECM as libc::c_int as UDWORD
            } else if sCurrDesign.asParts[COMP_SENSOR as libc::c_int as usize]
                          != 0 {
                type_0 = COMP_SENSOR as libc::c_int as UDWORD
            } else if sCurrDesign.asParts[COMP_CONSTRUCT as libc::c_int as
                                              usize] != 0 {
                type_0 = COMP_CONSTRUCT as libc::c_int as UDWORD
            } else if sCurrDesign.asParts[COMP_REPAIRUNIT as libc::c_int as
                                              usize] != 0 {
                type_0 = COMP_REPAIRUNIT as libc::c_int as UDWORD
            } else if sCurrDesign.asWeaps[0 as libc::c_int as usize] != 0 {
                type_0 = COMP_WEAPON as libc::c_int as UDWORD
            } else { type_0 = COMP_UNKNOWN as libc::c_int as UDWORD }
        }
        match type_0 {
            1 => {
                compTempl.asParts[COMP_BODY as libc::c_int as usize] =
                    (psStats as
                         *mut BODY_STATS).wrapping_offset_from(asBodyStats) as
                        libc::c_int
                //default:
			//don't want to draw for unknown comp
            }
            3 => {
                compTempl.asParts[COMP_PROPULSION as libc::c_int as usize] =
                    (psStats as
                         *mut PROPULSION_STATS).wrapping_offset_from(asPropulsionStats)
                        as libc::c_int
            }
            5 => {
                compTempl.asParts[COMP_ECM as libc::c_int as usize] =
                    (psStats as
                         *mut ECM_STATS).wrapping_offset_from(asECMStats) as
                        libc::c_int
            }
            6 => {
                compTempl.asParts[COMP_SENSOR as libc::c_int as usize] =
                    (psStats as
                         *mut SENSOR_STATS).wrapping_offset_from(asSensorStats)
                        as libc::c_int
            }
            7 => {
                compTempl.asParts[COMP_CONSTRUCT as libc::c_int as usize] =
                    (psStats as
                         *mut CONSTRUCT_STATS).wrapping_offset_from(asConstructStats)
                        as libc::c_int
            }
            4 => {
                compTempl.asParts[COMP_REPAIRUNIT as libc::c_int as usize] =
                    (psStats as
                         *mut REPAIR_STATS).wrapping_offset_from(asRepairStats)
                        as libc::c_int
            }
            8 => {
                //			compTempl.asWeaps[COMP_WEAPON] = (WEAPON_STATS *)psStats - asWeaponStats;
                compTempl.asWeaps[0 as libc::c_int as usize] =
                    (psStats as
                         *mut WEAPON_STATS).wrapping_offset_from(asWeaponStats)
                        as libc::c_int as UDWORD
            }
            _ => { }
        }
        widgSetMinorBarSize(psWScreen, 5127 as libc::c_int as UDWORD,
                            calcTemplateBody(&mut compTempl,
                                             selectedPlayer as UBYTE));
    } else {
        /* Reset the shadow bar */
        widgSetMinorBarSize(psWScreen, 5127 as libc::c_int as UDWORD,
                            0 as libc::c_int as UDWORD);
    };
}
/* Calculate the speed of a droid over a type of terrain */
unsafe extern "C" fn intCalcSpeed(mut type_0: TYPE_OF_TERRAIN,
                                  mut psProp: *mut PROPULSION_STATS)
 -> UDWORD {
    let mut weight: UDWORD = 0;
    /* Calculate the weight */
    weight = calcDroidWeight(&mut sCurrDesign);
    if weight == 0 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int as UDWORD
    }
    //we want the design screen to show zero speed over water for all prop types except Hover and Vtol
    if type_0 as libc::c_uint == TER_WATER as libc::c_int as libc::c_uint {
        if !((*psProp).propulsionType as libc::c_int == HOVER as libc::c_int
                 ||
                 (*psProp).propulsionType as libc::c_int ==
                     LIFT as libc::c_int) {
            return 0 as libc::c_int as UDWORD
        }
    }
    return calcDroidSpeed(calcDroidBaseSpeed(&mut sCurrDesign, weight,
                                             selectedPlayer as UBYTE),
                          type_0 as UDWORD,
                          psProp.wrapping_offset_from(asPropulsionStats) as
                              libc::c_int as UDWORD);
}
/* Set the bar graphs for the Propulsion stats */
/* Set the bar graphs for the Propulsion stats */
unsafe extern "C" fn intSetPropulsionStats(mut psStats:
                                               *mut PROPULSION_STATS) {
    let mut psForm: *mut W_FORM = 0 as *mut W_FORM;
    let mut weight: UDWORD = 0;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"intSetPropulsionStats: Invalid stats pointer\x00" as *const u8
                  as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"design.c\x00" as *const u8 as *const libc::c_char,
              3676 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 22],
                                        &[libc::c_char; 22]>(b"intSetPropulsionStats\x00")).as_ptr(),
              b"PTRVALID(psStats, sizeof(PROPULSION_STATS))\x00" as *const u8
                  as *const libc::c_char);
    };
    if (*psStats).ref_0 >= 0x40000 as libc::c_int as libc::c_uint &&
           (*psStats).ref_0 <
               (0x40000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"intSetPropulsionStats: stats ref is out of range\x00" as
                  *const u8 as *const libc::c_char);
    };
    if (*psStats).ref_0 >= 0x40000 as libc::c_int as libc::c_uint &&
           (*psStats).ref_0 <
               (0x40000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"design.c\x00" as *const u8 as *const libc::c_char,
              3679 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 22],
                                        &[libc::c_char; 22]>(b"intSetPropulsionStats\x00")).as_ptr(),
              b"(psStats->ref >= REF_PROPULSION_START) && (psStats->ref < REF_PROPULSION_START + REF_RANGE)\x00"
                  as *const u8 as *const libc::c_char);
    };
    /* set form tip to stats string */
    widgSetTip(psWScreen, 5008 as libc::c_int as UDWORD,
               getStatName(psStats as *mut libc::c_void));
    /* set form stats for later display in intDisplayStatForm */
    psForm =
        widgGetFromID(psWScreen, 5008 as libc::c_int as UDWORD) as
            *mut W_FORM;
    if !psForm.is_null() {
        (*psForm).pUserData = psStats as *mut libc::c_void
    }
    match desPropMode as libc::c_uint {
        0 => {
            /* Road speed */
            widgSetBarSize(psWScreen, 5105 as libc::c_int as UDWORD,
                           intCalcSpeed(TER_ROAD, psStats));
            /* Cross country speed - grass */
            widgSetBarSize(psWScreen, 5106 as libc::c_int as UDWORD,
                           intCalcSpeed(TER_SANDYBRUSH, psStats));
            /* Water speed */
            widgSetBarSize(psWScreen, 5107 as libc::c_int as UDWORD,
                           intCalcSpeed(TER_WATER, psStats));
        }
        1 => {
            /* Air speed - terrain type doesn't matter, use road */
            widgSetBarSize(psWScreen, 5108 as libc::c_int as UDWORD,
                           intCalcSpeed(TER_ROAD, psStats));
        }
        _ => { }
    }
    /* weight */
	//widgSetBarSize(psWScreen, IDDES_PROPWEIGHT, psStats->weight);
    /* propulsion weight is a percentage of the body weight */
    if sCurrDesign.asParts[COMP_BODY as libc::c_int as usize] !=
           0 as libc::c_int {
        weight =
            (*psStats).weight.wrapping_mul((*asBodyStats.offset(sCurrDesign.asParts[COMP_BODY
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        usize]
                                                                    as
                                                                    isize)).weight).wrapping_div(100
                                                                                                     as
                                                                                                     libc::c_int
                                                                                                     as
                                                                                                     libc::c_uint)
    } else {
        //if haven't got a body - can't calculate a value
        weight = 0 as libc::c_int as UDWORD
    }
    widgSetBarSize(psWScreen, 5109 as libc::c_int as UDWORD, weight);
}
/* Set the shadow bar graphs for the Propulsion stats */
/* Set the shadow bar graphs for the Propulsion stats */
unsafe extern "C" fn intSetPropulsionShadowStats(mut psStats:
                                                     *mut PROPULSION_STATS) {
    let mut weight: UDWORD = 0;
    if psStats.is_null() || 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"intSetPropulsionShadowStats: Invalid stats pointer\x00" as
                  *const u8 as *const libc::c_char);
    };
    if psStats.is_null() || 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"design.c\x00" as *const u8 as *const libc::c_char,
              3734 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 28],
                                        &[libc::c_char; 28]>(b"intSetPropulsionShadowStats\x00")).as_ptr(),
              b"psStats == NULL || PTRVALID(psStats, sizeof(PROPULSION_STATS))\x00"
                  as *const u8 as *const libc::c_char);
    };
    if psStats.is_null() ||
           (*psStats).ref_0 >= 0x40000 as libc::c_int as libc::c_uint &&
               (*psStats).ref_0 <
                   (0x40000 as libc::c_int + 0x10000 as libc::c_int) as
                       libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"intSetPropulsionShadowStats: stats ref is out of range\x00" as
                  *const u8 as *const libc::c_char);
    };
    if psStats.is_null() ||
           (*psStats).ref_0 >= 0x40000 as libc::c_int as libc::c_uint &&
               (*psStats).ref_0 <
                   (0x40000 as libc::c_int + 0x10000 as libc::c_int) as
                       libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"design.c\x00" as *const u8 as *const libc::c_char,
              3738 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 28],
                                        &[libc::c_char; 28]>(b"intSetPropulsionShadowStats\x00")).as_ptr(),
              b"psStats == NULL || ((psStats->ref >= REF_PROPULSION_START) && (psStats->ref < REF_PROPULSION_START + REF_RANGE))\x00"
                  as *const u8 as *const libc::c_char);
    };
    /* Only set the shadow stats if they are the right type */
    if !psStats.is_null() &&
           ((*asPropulsionTypes.offset((*psStats).propulsionType as
                                           isize)).travel ==
                GROUND as libc::c_int as libc::c_uint &&
                desPropMode as libc::c_uint !=
                    IDES_GROUND as libc::c_int as libc::c_uint ||
                (*asPropulsionTypes.offset((*psStats).propulsionType as
                                               isize)).travel ==
                    AIR as libc::c_int as libc::c_uint &&
                    desPropMode as libc::c_uint !=
                        IDES_AIR as libc::c_int as libc::c_uint) {
        return
    }
    match desPropMode as libc::c_uint {
        0 => {
            if !psStats.is_null() {
                /* Road speed */
                widgSetMinorBarSize(psWScreen, 5105 as libc::c_int as UDWORD,
                                    intCalcSpeed(TER_ROAD, psStats));
                /* Cross country speed - grass */
                widgSetMinorBarSize(psWScreen, 5106 as libc::c_int as UDWORD,
                                    intCalcSpeed(TER_SANDYBRUSH, psStats));
                /* Water speed */
                widgSetMinorBarSize(psWScreen, 5107 as libc::c_int as UDWORD,
                                    intCalcSpeed(TER_WATER, psStats));
            } else {
                /* Reset the shadow bars */
                widgSetMinorBarSize(psWScreen, 5105 as libc::c_int as UDWORD,
                                    0 as libc::c_int as UDWORD);
                widgSetMinorBarSize(psWScreen, 5106 as libc::c_int as UDWORD,
                                    0 as libc::c_int as UDWORD);
                widgSetMinorBarSize(psWScreen, 5107 as libc::c_int as UDWORD,
                                    0 as libc::c_int as UDWORD);
            }
        }
        1 => {
            if !psStats.is_null() {
                /* Air speed - terrain type doesn't matter, use ROAD */
                widgSetMinorBarSize(psWScreen, 5108 as libc::c_int as UDWORD,
                                    intCalcSpeed(TER_ROAD, psStats));
            } else {
                /* Reset the shadow bar */
                widgSetMinorBarSize(psWScreen, 5108 as libc::c_int as UDWORD,
                                    0 as libc::c_int as UDWORD);
            }
        }
        _ => { }
    }
    if !psStats.is_null() {
        /* weight */
		//widgSetMinorBarSize(psWScreen, IDDES_PROPWEIGHT, psStats->weight);
        /* propulsion weight is a percentage of the body weight */
        if sCurrDesign.asParts[COMP_BODY as libc::c_int as usize] !=
               0 as libc::c_int {
            weight =
                (*psStats).weight.wrapping_mul((*asBodyStats.offset(sCurrDesign.asParts[COMP_BODY
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            usize]
                                                                        as
                                                                        isize)).weight).wrapping_div(100
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         libc::c_uint)
        } else {
            //if haven't got a body - can't calculate a value
            weight = 0 as libc::c_int as UDWORD
        }
        widgSetMinorBarSize(psWScreen, 5109 as libc::c_int as UDWORD, weight);
    } else {
        /* Reset the shadow bar */
        widgSetMinorBarSize(psWScreen, 5109 as libc::c_int as UDWORD,
                            0 as libc::c_int as UDWORD);
    };
}
/* Check whether a droid template is valid */
/* Return a program for a specific order */
/*static UDWORD intGetProgram(PROGRAM_ORDERS order)
{
	UDWORD		index;

	for (index = 0; index < numProgramStats; index++)
	{
		if (asProgramStats[index].order == (UDWORD)order)
		{
			break;
		}
	}

	if (index == numProgramStats)
	{
		return 0;
	}

	return index;
}*/
/* Check whether a droid template is valid */
unsafe extern "C" fn intValidTemplate(mut psTempl: *mut DROID_TEMPLATE)
 -> BOOL {
    let mut i: UDWORD = 0;
    // set the weapon for a command droid
    if (*psTempl).asParts[COMP_BRAIN as libc::c_int as usize] !=
           0 as libc::c_int {
        (*psTempl).numWeaps = 1 as libc::c_int as UDWORD;
        (*psTempl).asWeaps[0 as libc::c_int as usize] =
            (*asBrainStats.offset((*psTempl).asParts[COMP_BRAIN as libc::c_int
                                                         as usize] as
                                      isize)).psWeaponStat.wrapping_offset_from(asWeaponStats)
                as libc::c_int as UDWORD
        //			asCommandDroids[selectedPlayer][psTempl->asParts[COMP_BRAIN]].nWeapStat;
    }
    /* Check all the components have been set */
    if (*psTempl).asParts[COMP_BODY as libc::c_int as usize] ==
           0 as libc::c_int {
        return 0 as libc::c_int
    } else {
        if (*psTempl).asParts[COMP_PROPULSION as libc::c_int as usize] ==
               0 as libc::c_int {
            return 0 as libc::c_int
        }
    }
    // Check a turret has been installed
    if (*psTempl).numWeaps == 0 as libc::c_int as libc::c_uint &&
           (*psTempl).asParts[COMP_SENSOR as libc::c_int as usize] ==
               0 as libc::c_int &&
           (*psTempl).asParts[COMP_ECM as libc::c_int as usize] ==
               0 as libc::c_int &&
           (*psTempl).asParts[COMP_BRAIN as libc::c_int as usize] ==
               0 as libc::c_int &&
           (*psTempl).asParts[COMP_REPAIRUNIT as libc::c_int as usize] ==
               0 as libc::c_int &&
           (*psTempl).asParts[COMP_CONSTRUCT as libc::c_int as usize] ==
               0 as libc::c_int {
        return 0 as libc::c_int
    }
    /* Check the weapons */
    i = 0 as libc::c_int as UDWORD;
    while i < (*psTempl).numWeaps {
        if (*psTempl).asWeaps[i as usize] == 0 as libc::c_int as libc::c_uint
           {
            return 0 as libc::c_int
        }
        i = i.wrapping_add(1)
    }
    //can only have a weapon on a VTOL propulsion
    if checkTemplateIsVtol(psTempl) != 0 {
        if (*psTempl).numWeaps == 0 as libc::c_int as libc::c_uint {
            return 0 as libc::c_int
        }
    }
    if (*psTempl).asParts[COMP_SENSOR as libc::c_int as usize] ==
           0 as libc::c_int {
        /* Set the default Sensor */
        (*psTempl).asParts[COMP_SENSOR as libc::c_int as usize] =
            aDefaultSensor[selectedPlayer as usize] as SDWORD
    }
    if (*psTempl).asParts[COMP_ECM as libc::c_int as usize] ==
           0 as libc::c_int {
        /* Set the default ECM */
        (*psTempl).asParts[COMP_ECM as libc::c_int as usize] =
            aDefaultECM[selectedPlayer as usize] as SDWORD
    }
    if (*psTempl).asParts[COMP_REPAIRUNIT as libc::c_int as usize] ==
           0 as libc::c_int {
        /* Set the default Repair */
        (*psTempl).asParts[COMP_REPAIRUNIT as libc::c_int as usize] =
            aDefaultRepair[selectedPlayer as usize] as SDWORD
    }
    (*psTempl).ref_0 = 0xc0000 as libc::c_int as UDWORD;
    /* Set a default program */
	/*psTempl->numProgs = 1;
	psTempl->droidType = droidTemplateType(psTempl);
	switch ( psTempl->droidType )
	{
	case DROID_WEAPON:
	case DROID_PERSON:
	case DROID_CYBORG:
	case DROID_DEFAULT:
		psTempl->asProgs[0] = intGetProgram(ORDER_ATTACK);
		break;
	case DROID_SENSOR:
		psTempl->asProgs[0] = intGetProgram(ORDER_GUARD);
		break;
	case DROID_ECM:
		psTempl->asProgs[0] = intGetProgram(ORDER_GUARD);
		break;
	case DROID_CONSTRUCT:
		psTempl->asProgs[0] = intGetProgram(ORDER_BUILD);
		break;
	case DROID_REPAIR:
		psTempl->asProgs[0] = intGetProgram(ORDER_REPAIR);
		break;
	}*/
    /* Calculate build points */
    (*psTempl).buildPoints = calcTemplateBuild(psTempl);
    (*psTempl).powerPoints = calcTemplatePower(psTempl);
    //set the droidtype
    (*psTempl).droidType = droidTemplateType(psTempl);
    /* copy current name into template */
    strcpy(sCurrDesign.aName.as_mut_ptr(), aCurrName.as_mut_ptr());
    return 1 as libc::c_int;
}
// ajl. above function is static. A quick wrapper for the net stuff
#[no_mangle]
pub unsafe extern "C" fn MultiPlayValidTemplate(mut psTempl:
                                                    *mut DROID_TEMPLATE)
 -> BOOL {
    return intValidTemplate(psTempl);
}
#[no_mangle]
pub unsafe extern "C" fn desCreateDefaultTemplate() {
    /* set current design to default */
    memcpy(&mut sCurrDesign as *mut DROID_TEMPLATE as *mut libc::c_void,
           &mut sDefaultDesignTemplate as *mut DROID_TEMPLATE as
               *const libc::c_void,
           ::std::mem::size_of::<DROID_TEMPLATE>() as libc::c_ulong);
    /* reset stats */
    intSetDesignStats(&mut sCurrDesign);
    widgDelete(psWScreen, 5006 as libc::c_int as UDWORD);
    desSysMode = IDES_NOSYSTEM;
    CurrentStatsTemplate =
        &mut sCurrDesign as *mut DROID_TEMPLATE as *mut BASE_STATS;
}
/* Remove the design widgets from the widget screen */
#[no_mangle]
pub unsafe extern "C" fn intRemoveDesign() {
    //save the current design on exit if it is valid
    saveTemplate();
    newTemplate = 0 as libc::c_int;
    widgDelete(psWScreen, 5019 as libc::c_int as UDWORD);
    widgDelete(psWScreen, 5013 as libc::c_int as UDWORD);
    widgDelete(psWScreen, 5002 as libc::c_int as UDWORD);
    widgDelete(psWScreen, 5020 as libc::c_int as UDWORD);
    widgDelete(psWScreen, 5003 as libc::c_int as UDWORD);
    widgDelete(psWScreen, 5021 as libc::c_int as UDWORD);
    widgDelete(psWScreen, 5007 as libc::c_int as UDWORD);
    widgDelete(psWScreen, 5008 as libc::c_int as UDWORD);
    widgDelete(psWScreen, 5006 as libc::c_int as UDWORD);
    widgDelete(psWScreen, 5000 as libc::c_int as UDWORD);
    widgDelete(psWScreen, 5001 as libc::c_int as UDWORD);
    resetDesignPauseState();
}
/* set flashing flag for button */
#[no_mangle]
pub unsafe extern "C" fn intSetButtonFlash(mut id: UDWORD, mut bFlash: BOOL) {
    let mut psWidget: *mut WIDGET = widgGetFromID(psWScreen, id);
    if (*psWidget).type_0 as libc::c_uint ==
           WIDG_BUTTON as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"intSetButtonFlash : Not a button\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (*psWidget).type_0 as libc::c_uint ==
           WIDG_BUTTON as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"design.c\x00" as *const u8 as *const libc::c_char,
              4001 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 18],
                                        &[libc::c_char; 18]>(b"intSetButtonFlash\x00")).as_ptr(),
              b"psWidget->type == WIDG_BUTTON\x00" as *const u8 as
                  *const libc::c_char);
    };
    if bFlash == 1 as libc::c_int {
        (*psWidget).display =
            Some(intDisplayButtonFlash as
                     unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                          _: UDWORD, _: *mut UDWORD) -> ())
    } else {
        (*psWidget).display =
            Some(intDisplayButtonHilight as
                     unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                          _: UDWORD, _: *mut UDWORD) -> ())
    };
}
/*
 * desTemplateNameCustomised
 *
 * Checks whether user has customised template name : template not
 * customised if not complete or if generated name same as current.
 */
#[no_mangle]
pub unsafe extern "C" fn desTemplateNameCustomised(mut psTemplate:
                                                       *mut DROID_TEMPLATE)
 -> BOOL {
    if (*psTemplate).droidType as libc::c_uint ==
           DROID_DEFAULT as libc::c_int as libc::c_uint ||
           strcmp(getTemplateName(psTemplate),
                  GetDefaultTemplateName(psTemplate)) == 0 as libc::c_int {
        return 0 as libc::c_int
    } else { return 1 as libc::c_int };
}
/* checks whether to update name or has user already changed it */
#[no_mangle]
pub unsafe extern "C" fn desUpdateDesignName(mut psTemplate:
                                                 *mut DROID_TEMPLATE,
                                             mut szCurrName: *mut STRING) {
}
/* Process return codes from the design screen */
#[no_mangle]
pub unsafe extern "C" fn intProcessDesign(mut id: UDWORD) {
    let mut psTempl: *mut DROID_TEMPLATE = 0 as *mut DROID_TEMPLATE;
    let mut psCurr: *mut DROID_TEMPLATE = 0 as *mut DROID_TEMPLATE;
    let mut psPrev: *mut DROID_TEMPLATE = 0 as *mut DROID_TEMPLATE;
    //DROID_TEMPLATE	*psTempPrev;
    let mut currID: UDWORD = 0;
    //	DES_COMPMODE	currCompMode;
    let mut i: UDWORD = 0;
    let mut bTemplateNameCustomised: BOOL = 0;
    //	if (pie_GetRenderEngine() == ENGINE_GLIDE)
//	{
		/* Dirty hack to allow screen dumps from the 3dfx during design!!! */
//		if(keyPressed(KEY_D))
//		{
//			CONPRINTF(ConsoleString,(ConsoleString,"Hackety hack - Alex has written screen dump to disk - %s",iV_ScreenDumpToDisk()));
//		}
//	}
    /* check template button pressed */
    if id >= 5300 as libc::c_int as libc::c_uint &&
           id <= 5339 as libc::c_int as libc::c_uint {
        /* if first template create blank design */
        if id == 5300 as libc::c_int as libc::c_uint {
            desCreateDefaultTemplate();
            strncpy(aCurrName.as_mut_ptr(),
                    strresGetString(psStringRes,
                                    STR_DES_NEWVEH as libc::c_int as UDWORD),
                    (80 as libc::c_int - 1 as libc::c_int) as libc::c_uint);
            strcpy(sCurrDesign.aName.as_mut_ptr(), aCurrName.as_mut_ptr());
            //			strncpy(aCurrName, strresGetString(psStringRes, STR_DES_NEWVEH),
//				WIDG_MAXSTR-1);
            /* hide body and system component buttons */
            widgHide(psWScreen, 5900 as libc::c_int as UDWORD);
            //			widgHide( psWScreen, IDDES_BODYBUTTON );
            widgHide(psWScreen, 5902 as libc::c_int as UDWORD);
            /* set button render routines to flash */
            intSetButtonFlash(5900 as libc::c_int as UDWORD,
                              1 as libc::c_int);
            intSetButtonFlash(5901 as libc::c_int as UDWORD,
                              1 as libc::c_int);
            intSetButtonFlash(5902 as libc::c_int as UDWORD,
                              1 as libc::c_int);
        } else {
            /* Find the template for the new button */
            currID = 5300 as libc::c_int as UDWORD;
            i = 0 as libc::c_int as UDWORD;
            while i < 40 as libc::c_int as libc::c_uint {
                psTempl = *apsTemplateList.offset(i as isize);
                if currID == id { break ; }
                currID = currID.wrapping_add(1);
                i = i.wrapping_add(1)
            }
            if !psTempl.is_null() {
            } else {
                debug(LOG_ERROR,
                      b"intProcessDesign: template not found!\n\x00" as
                          *const u8 as *const libc::c_char);
            };
            if !psTempl.is_null() {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"design.c\x00" as *const u8 as *const libc::c_char,
                      4107 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 17],
                                                &[libc::c_char; 17]>(b"intProcessDesign\x00")).as_ptr(),
                      b"psTempl != NULL\x00" as *const u8 as
                          *const libc::c_char);
            };
            if !psTempl.is_null() {
                /* Set the new template */
                memcpy(&mut sCurrDesign as *mut DROID_TEMPLATE as
                           *mut libc::c_void, psTempl as *const libc::c_void,
                       ::std::mem::size_of::<DROID_TEMPLATE>() as
                           libc::c_ulong);
                //strcpy( sCurrDesign.aName, aCurrName );
                strncpy(aCurrName.as_mut_ptr(), getTemplateName(psTempl),
                        (80 as libc::c_int - 1 as libc::c_int) as
                            libc::c_uint);
                /* reveal body and propulsion component buttons */
                widgReveal(psWScreen, 5901 as libc::c_int as UDWORD);
                widgReveal(psWScreen, 5902 as libc::c_int as UDWORD);
                widgReveal(psWScreen, 5900 as libc::c_int as UDWORD);
                /* turn off button flashes */
                intSetButtonFlash(5900 as libc::c_int as UDWORD,
                                  0 as libc::c_int);
                intSetButtonFlash(5901 as libc::c_int as UDWORD,
                                  0 as libc::c_int);
                intSetButtonFlash(5902 as libc::c_int as UDWORD,
                                  0 as libc::c_int);
                /* reset button states */
                widgSetButtonState(psWScreen, 5900 as libc::c_int as UDWORD,
                                   0 as libc::c_int as UDWORD);
                widgSetButtonState(psWScreen, 5901 as libc::c_int as UDWORD,
                                   0 as libc::c_int as UDWORD);
                widgSetButtonState(psWScreen, 5902 as libc::c_int as UDWORD,
                                   0 as libc::c_int as UDWORD);
            }
            intSetDesignMode(IDES_BODY);
        }
        /* reveal and flash body component button */
        widgReveal(psWScreen, 5901 as libc::c_int as UDWORD);
        widgSetButtonState(psWScreen, 5901 as libc::c_int as UDWORD,
                           0x4 as libc::c_int as UDWORD);
        /* reveal design form if not already on-screen */
        widgReveal(psWScreen, 5000 as libc::c_int as UDWORD);
        /* Droid template button has been pressed - clear the old button */
        if droidTemplID != 0 as libc::c_int as libc::c_uint {
            widgSetButtonState(psWScreen, droidTemplID,
                               0 as libc::c_int as UDWORD);
        }
        intSetDesignStats(&mut sCurrDesign);
        /* show body stats only */
        widgReveal(psWScreen, 5001 as libc::c_int as UDWORD);
        widgReveal(psWScreen, 5007 as libc::c_int as UDWORD);
        widgHide(psWScreen, 5008 as libc::c_int as UDWORD);
        widgHide(psWScreen, 5006 as libc::c_int as UDWORD);
        /*Update the Power bar stats as the power to build will have changed */
        intSetDesignPower(&mut sCurrDesign);
        /*Update the body points */
        intSetBodyPoints(&mut sCurrDesign);
        /* Lock the button */
        widgSetButtonState(psWScreen, id, 0x2 as libc::c_int as UDWORD);
        droidTemplID = id;
        /* Update the component form */
//		currCompMode = desCompMode;
        widgDelete(psWScreen, 5003 as libc::c_int as UDWORD);
        widgDelete(psWScreen, 5021 as libc::c_int as UDWORD);
        desCompMode = IDES_NOCOMPONENT;
        intSetDesignMode(IDES_BODY);
    } else if id >= 5500 as libc::c_int as libc::c_uint &&
                  id <= 5699 as libc::c_int as libc::c_uint {
        /* check whether can change template name */
        bTemplateNameCustomised = desTemplateNameCustomised(&mut sCurrDesign);
        /* Component stats button has been pressed - clear the old button */
        if desCompID != 0 as libc::c_int as libc::c_uint {
            widgSetButtonState(psWScreen, desCompID,
                               0 as libc::c_int as UDWORD);
        }
        /* Set the stats in the template */
        match desCompMode as libc::c_uint {
            2 => {
                /* Calculate the index of the component */
                sCurrDesign.asWeaps[0 as libc::c_int as usize] =
                    (*apsComponentList.offset(id.wrapping_sub(5500 as
                                                                  libc::c_int
                                                                  as
                                                                  libc::c_uint)
                                                  as isize) as
                         *mut WEAPON_STATS).wrapping_offset_from(asWeaponStats)
                        as libc::c_int as UDWORD;
                sCurrDesign.numWeaps = 1 as libc::c_int as UDWORD;
                /* Reset the sensor, ECM and constructor and repair
				- defaults will be set when OK is hit */
                sCurrDesign.asParts[COMP_SENSOR as libc::c_int as usize] =
                    0 as libc::c_int;
                sCurrDesign.asParts[COMP_ECM as libc::c_int as usize] =
                    0 as libc::c_int;
                sCurrDesign.asParts[COMP_CONSTRUCT as libc::c_int as usize] =
                    0 as libc::c_int;
                sCurrDesign.asParts[COMP_REPAIRUNIT as libc::c_int as usize] =
                    0 as libc::c_int;
                sCurrDesign.asParts[COMP_BRAIN as libc::c_int as usize] =
                    0 as libc::c_int;
                /* Set the new stats on the display */
                intSetSystemForm(*apsComponentList.offset(id.wrapping_sub(5500
                                                                              as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_uint)
                                                              as isize));
                // do the callback if in the tutorial
                if bInTutorial != 0 {
                    eventFireCallbackTrigger(CALL_DESIGN_WEAPON as libc::c_int
                                                 as TRIGGER_TYPE);
                }
            }
            3 => {
                /* reveal propulsion button if hidden */
                widgReveal(psWScreen, 5902 as libc::c_int as UDWORD);
                /* Calculate the index of the component */
                sCurrDesign.asParts[COMP_BODY as libc::c_int as usize] =
                    (*apsComponentList.offset(id.wrapping_sub(5500 as
                                                                  libc::c_int
                                                                  as
                                                                  libc::c_uint)
                                                  as isize) as
                         *mut BODY_STATS).wrapping_offset_from(asBodyStats) as
                        libc::c_int;
                /* Set the new stats on the display */
                intSetBodyStats(*apsComponentList.offset(id.wrapping_sub(5500
                                                                             as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint)
                                                             as isize) as
                                    *mut BODY_STATS);
                // do the callback if in the tutorial
                if bInTutorial != 0 {
                    eventFireCallbackTrigger(CALL_DESIGN_BODY as libc::c_int
                                                 as TRIGGER_TYPE);
                }
            }
            4 => {
                /* Calculate the index of the component */
                sCurrDesign.asParts[COMP_PROPULSION as libc::c_int as usize] =
                    (*apsComponentList.offset(id.wrapping_sub(5500 as
                                                                  libc::c_int
                                                                  as
                                                                  libc::c_uint)
                                                  as isize) as
                         *mut PROPULSION_STATS).wrapping_offset_from(asPropulsionStats)
                        as libc::c_int;
                /* Set the new stats on the display */
                intSetPropulsionStats(*apsComponentList.offset(id.wrapping_sub(5500
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   libc::c_uint)
                                                                   as isize)
                                          as *mut PROPULSION_STATS);
                //check that the weapon is valid for this propulsion
                if intCheckValidWeaponForProp() == 0 {
                    //no way of allocating more than one weapon is there?
                    if sCurrDesign.numWeaps > 1 as libc::c_int as libc::c_uint
                       {
                        if 0 as libc::c_int != 0 {
                        } else {
                            debug(LOG_ERROR,
                                  b"designScreen: More than one weapon on droid - how?\x00"
                                      as *const u8 as *const libc::c_char);
                        };
                        if 0 as libc::c_int != 0 {
                        } else {
                            debug(LOG_ERROR,
                                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                                      *const u8 as *const libc::c_char,
                                  b"design.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  4267 as libc::c_int,
                                  (*::std::mem::transmute::<&[u8; 17],
                                                            &[libc::c_char; 17]>(b"intProcessDesign\x00")).as_ptr(),
                                  b"FALSE\x00" as *const u8 as
                                      *const libc::c_char);
                        };
                    }
                    //not valid weapon so initialise the weapon stat
                    sCurrDesign.asWeaps[0 as libc::c_int as usize] =
                        0 as libc::c_int as UDWORD;
                    //init all other stats as well!
                    sCurrDesign.asParts[COMP_SENSOR as libc::c_int as usize] =
                        0 as libc::c_int;
                    sCurrDesign.asParts[COMP_BRAIN as libc::c_int as usize] =
                        0 as libc::c_int;
                    sCurrDesign.asParts[COMP_REPAIRUNIT as libc::c_int as
                                            usize] = 0 as libc::c_int;
                    sCurrDesign.asParts[COMP_CONSTRUCT as libc::c_int as
                                            usize] = 0 as libc::c_int;
                    sCurrDesign.asParts[COMP_ECM as libc::c_int as usize] =
                        0 as libc::c_int;
                    /* Reset the weapon stats on the display */
                    intSetSystemForm(asWeaponStats.offset(sCurrDesign.asWeaps[0
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  usize]
                                                              as isize) as
                                         *mut COMP_BASE_STATS);
                    intSetDesignMode(IDES_PROPULSION);
                }
                // do the callback if in the tutorial
                if bInTutorial != 0 {
                    eventFireCallbackTrigger(CALL_DESIGN_PROPULSION as
                                                 libc::c_int as TRIGGER_TYPE);
                }
            }
            0 | 1 | _ => { }
        }
        /* Lock the new button */
        widgSetButtonState(psWScreen, id, 0x2 as libc::c_int as UDWORD);
        desCompID = id;
        /* Update the propulsion stats as the droid weight will have changed */
        intSetPropulsionStats(asPropulsionStats.offset(sCurrDesign.asParts[COMP_PROPULSION
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               usize]
                                                           as isize));
        /*Update the Power bar stats as the power to build will have changed */
        intSetDesignPower(&mut sCurrDesign);
        /*Update the body points */
        intSetBodyPoints(&mut sCurrDesign);
        /* update name if not customised */
        if bTemplateNameCustomised == 0 as libc::c_int {
            strcpy(sCurrDesign.aName.as_mut_ptr(),
                   GetDefaultTemplateName(&mut sCurrDesign));
        }
        /* Update the name in the edit box */
        intSetEditBoxTextFromTemplate(&mut sCurrDesign);
        /* flash next button if design not complete */
        if intValidTemplate(&mut sCurrDesign) == 0 as libc::c_int {
            /* reset button states */
            widgSetButtonState(psWScreen, 5900 as libc::c_int as UDWORD,
                               0 as libc::c_int as UDWORD);
            widgSetButtonState(psWScreen, 5901 as libc::c_int as UDWORD,
                               0 as libc::c_int as UDWORD);
            widgSetButtonState(psWScreen, 5902 as libc::c_int as UDWORD,
                               0 as libc::c_int as UDWORD);
            match desCompMode as libc::c_uint {
                3 => {
                    widgSetButtonState(psWScreen,
                                       5902 as libc::c_int as UDWORD,
                                       0x4 as libc::c_int as UDWORD);
                }
                4 => {
                    widgSetButtonState(psWScreen,
                                       5900 as libc::c_int as UDWORD,
                                       0x4 as libc::c_int as UDWORD);
                }
                0 | 1 | 2 => {
                    widgSetButtonState(psWScreen,
                                       5901 as libc::c_int as UDWORD,
                                       0x4 as libc::c_int as UDWORD);
                }
                _ => { }
            }
        }
    } else if id >= 5700 as libc::c_int as libc::c_uint &&
                  id <= 5899 as libc::c_int as libc::c_uint {
        /* check whether can change template name */
        bTemplateNameCustomised = desTemplateNameCustomised(&mut sCurrDesign);
        /* flash body button */
//		widgSetButtonState(psWScreen, IDDES_SYSTEMBUTTON, 0);
//		widgSetButtonState(psWScreen, IDDES_BODYBUTTON, WBUT_CLICKLOCK);
//		widgSetButtonState(psWScreen, IDDES_PROPBUTTON, 0);
        if desCompID != 0 as libc::c_int as libc::c_uint {
            widgSetButtonState(psWScreen, desCompID,
                               0 as libc::c_int as UDWORD);
        }
        match statType((**apsExtraSysList.offset(id.wrapping_sub(5700 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint)
                                                     as isize)).ref_0) {
            6 => {
                // Extra component stats button has been pressed - clear the old button
                // Now store the new stats
                // Calculate the index of the component
                sCurrDesign.asParts[COMP_SENSOR as libc::c_int as usize] =
                    (*apsExtraSysList.offset(id.wrapping_sub(5700 as
                                                                 libc::c_int
                                                                 as
                                                                 libc::c_uint)
                                                 as isize) as
                         *mut SENSOR_STATS).wrapping_offset_from(asSensorStats)
                        as libc::c_int;
                // Reset the ECM, constructor and weapon and repair
			//	- defaults will be set when OK is hit
                sCurrDesign.numWeaps = 0 as libc::c_int as UDWORD;
                sCurrDesign.asWeaps[0 as libc::c_int as usize] =
                    0 as libc::c_int as UDWORD;
                sCurrDesign.asParts[COMP_ECM as libc::c_int as usize] =
                    0 as libc::c_int;
                sCurrDesign.asParts[COMP_CONSTRUCT as libc::c_int as usize] =
                    0 as libc::c_int;
                sCurrDesign.asParts[COMP_REPAIRUNIT as libc::c_int as usize] =
                    0 as libc::c_int;
                sCurrDesign.asParts[COMP_BRAIN as libc::c_int as usize] =
                    0 as libc::c_int;
                // Set the new stats on the display
                intSetSystemForm(*apsExtraSysList.offset(id.wrapping_sub(5700
                                                                             as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint)
                                                             as isize));
            }
            5 => {
                // Calculate the index of the component
                sCurrDesign.asParts[COMP_ECM as libc::c_int as usize] =
                    (*apsExtraSysList.offset(id.wrapping_sub(5700 as
                                                                 libc::c_int
                                                                 as
                                                                 libc::c_uint)
                                                 as isize) as
                         *mut ECM_STATS).wrapping_offset_from(asECMStats) as
                        libc::c_int;
                // Reset the Sensor, constructor and weapon and repair
			//	- defaults will be set when OK is hit
                sCurrDesign.numWeaps = 0 as libc::c_int as UDWORD;
                sCurrDesign.asWeaps[0 as libc::c_int as usize] =
                    0 as libc::c_int as UDWORD;
                sCurrDesign.asParts[COMP_SENSOR as libc::c_int as usize] =
                    0 as libc::c_int;
                sCurrDesign.asParts[COMP_CONSTRUCT as libc::c_int as usize] =
                    0 as libc::c_int;
                sCurrDesign.asParts[COMP_REPAIRUNIT as libc::c_int as usize] =
                    0 as libc::c_int;
                sCurrDesign.asParts[COMP_BRAIN as libc::c_int as usize] =
                    0 as libc::c_int;
                // Set the new stats on the display
                intSetSystemForm(*apsExtraSysList.offset(id.wrapping_sub(5700
                                                                             as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint)
                                                             as isize));
            }
            7 => {
                // Calculate the index of the component and repair
                sCurrDesign.asParts[COMP_CONSTRUCT as libc::c_int as usize] =
                    (*apsExtraSysList.offset(id.wrapping_sub(5700 as
                                                                 libc::c_int
                                                                 as
                                                                 libc::c_uint)
                                                 as isize) as
                         *mut CONSTRUCT_STATS).wrapping_offset_from(asConstructStats)
                        as libc::c_int;
                // Reset the Sensor, ECM and weapon
			//	- defaults will be set when OK is hit
                sCurrDesign.numWeaps = 0 as libc::c_int as UDWORD;
                sCurrDesign.asWeaps[0 as libc::c_int as usize] =
                    0 as libc::c_int as UDWORD;
                sCurrDesign.asParts[COMP_ECM as libc::c_int as usize] =
                    0 as libc::c_int;
                sCurrDesign.asParts[COMP_SENSOR as libc::c_int as usize] =
                    0 as libc::c_int;
                sCurrDesign.asParts[COMP_REPAIRUNIT as libc::c_int as usize] =
                    0 as libc::c_int;
                sCurrDesign.asParts[COMP_BRAIN as libc::c_int as usize] =
                    0 as libc::c_int;
                // Set the new stats on the display
                intSetSystemForm(*apsExtraSysList.offset(id.wrapping_sub(5700
                                                                             as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint)
                                                             as isize));
            }
            4 => {
                // Calculate the index of the component
                sCurrDesign.asParts[COMP_REPAIRUNIT as libc::c_int as usize] =
                    (*apsExtraSysList.offset(id.wrapping_sub(5700 as
                                                                 libc::c_int
                                                                 as
                                                                 libc::c_uint)
                                                 as isize) as
                         *mut REPAIR_STATS).wrapping_offset_from(asRepairStats)
                        as libc::c_int;
                // Reset the Sensor, ECM and weapon and construct
			//	- defaults will be set when OK is hit
                sCurrDesign.numWeaps = 0 as libc::c_int as UDWORD;
                sCurrDesign.asWeaps[0 as libc::c_int as usize] =
                    0 as libc::c_int as UDWORD;
                sCurrDesign.asParts[COMP_ECM as libc::c_int as usize] =
                    0 as libc::c_int;
                sCurrDesign.asParts[COMP_SENSOR as libc::c_int as usize] =
                    0 as libc::c_int;
                sCurrDesign.asParts[COMP_CONSTRUCT as libc::c_int as usize] =
                    0 as libc::c_int;
                sCurrDesign.asParts[COMP_BRAIN as libc::c_int as usize] =
                    0 as libc::c_int;
                // Set the new stats on the display
                intSetSystemForm(*apsExtraSysList.offset(id.wrapping_sub(5700
                                                                             as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint)
                                                             as isize));
            }
            2 => {
                /* Calculate the index of the brain */
                sCurrDesign.asParts[COMP_BRAIN as libc::c_int as usize] =
                    (*apsExtraSysList.offset(id.wrapping_sub(5700 as
                                                                 libc::c_int
                                                                 as
                                                                 libc::c_uint)
                                                 as isize) as
                         *mut BRAIN_STATS).wrapping_offset_from(asBrainStats)
                        as libc::c_int;
                /* Reset the sensor, ECM and constructor and repair
				- defaults will be set when OK is hit */
                sCurrDesign.asParts[COMP_SENSOR as libc::c_int as usize] =
                    0 as libc::c_int;
                sCurrDesign.asParts[COMP_ECM as libc::c_int as usize] =
                    0 as libc::c_int;
                sCurrDesign.asParts[COMP_CONSTRUCT as libc::c_int as usize] =
                    0 as libc::c_int;
                sCurrDesign.asParts[COMP_REPAIRUNIT as libc::c_int as usize] =
                    0 as libc::c_int;
                sCurrDesign.numWeaps = 0 as libc::c_int as UDWORD;
                /* Set the new stats on the display */
                intSetSystemForm(*apsExtraSysList.offset(id.wrapping_sub(5700
                                                                             as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint)
                                                             as isize));
            }
            _ => { }
        }
        widgSetButtonState(psWScreen, id, 0x2 as libc::c_int as UDWORD);
        desCompID = id;
        intSetPropulsionStats(asPropulsionStats.offset(sCurrDesign.asParts[COMP_PROPULSION
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               usize]
                                                           as isize));
        intSetDesignPower(&mut sCurrDesign);
        intSetBodyPoints(&mut sCurrDesign);
        if bTemplateNameCustomised == 0 as libc::c_int {
            strcpy(sCurrDesign.aName.as_mut_ptr(),
                   GetDefaultTemplateName(&mut sCurrDesign));
        }
        intSetEditBoxTextFromTemplate(&mut sCurrDesign);
        if bInTutorial != 0 {
            if statType((**apsExtraSysList.offset(id.wrapping_sub(5700 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint)
                                                      as isize)).ref_0) ==
                   COMP_BRAIN as libc::c_int as libc::c_uint {
                eventFireCallbackTrigger(CALL_DESIGN_COMMAND as libc::c_int as
                                             TRIGGER_TYPE);
            } else {
                eventFireCallbackTrigger(CALL_DESIGN_SYSTEM as libc::c_int as
                                             TRIGGER_TYPE);
            }
        }
    } else {
        match id {
            5024 => {
                // Lock the new button
                // Update the propulsion stats as the droid weight will have changed
                // Update the Power bar stats as the power to build will have changed
                // Update the body points
                /* update name if not customised */
                /* Update the name in the edit box */
                // do the callback if in the tutorial
                /* The four component clickable forms */
                desCompID = 0 as libc::c_int as UDWORD;
                intSetDesignMode(IDES_TURRET);
            }
            5026 => {
                desCompID = 0 as libc::c_int as UDWORD;
                intSetDesignMode(IDES_BRAIN);
            }
            5025 => {
                desCompID = 0 as libc::c_int as UDWORD;
                intSetDesignMode(IDES_SYSTEM);
            }
            5013 => {
                /* The name edit box */
                strncpy(sCurrDesign.aName.as_mut_ptr(),
                        widgGetString(psWScreen,
                                      5013 as libc::c_int as UDWORD),
                        60 as libc::c_int as libc::c_uint);
                strncpy(aCurrName.as_mut_ptr(),
                        sCurrDesign.aName.as_mut_ptr(),
                        (80 as libc::c_int - 1 as libc::c_int) as
                            libc::c_uint);
            }
            5011 => {
                /* Find the template for the current button */
                currID = (5300 as libc::c_int + 1 as libc::c_int) as UDWORD;
                //psTempPrev = NULL;
                i = 1 as libc::c_int as UDWORD;
                while i < 40 as libc::c_int as libc::c_uint {
                    psTempl = *apsTemplateList.offset(i as isize);
                    if currID == droidTemplID &&
                           psTempl != &mut sCurrDesign as *mut DROID_TEMPLATE
                       {
                        break ;
                    }
                    currID = currID.wrapping_add(1);
                    i = i.wrapping_add(1)
                    //psTempPrev = psTempl;
                }
                /* remove template if found */
                if !psTempl.is_null() {
                    if bMultiPlayer != 0 {
                        //ajl. inform others of template destruction.
                        SendDestroyTemplate(psTempl);
                    }
                    /*CAN'T ASSUME THIS - there are some templates that don't get passed
				into the design screen*/
				/* update player template list.
				if( psTempPrev )
				{
					psTempPrev->psNext = psTempl->psNext;
				}
				else
				{
					apsDroidTemplates[selectedPlayer] = psTempl->psNext;
				}*/
                    //update player template list.
                    psCurr = apsDroidTemplates[selectedPlayer as usize];
                    psPrev = 0 as *mut DROID_TEMPLATE;
                    while !psCurr.is_null() {
                        if psCurr == psTempl {
                            if !psPrev.is_null() {
                                (*psPrev).psNext = (*psCurr).psNext
                            } else {
                                apsDroidTemplates[selectedPlayer as usize] =
                                    (*psCurr).psNext
                            }
                            break ;
                        } else { psPrev = psCurr; psCurr = (*psCurr).psNext }
                    }
                    // Delete the template.
                //before deleting the template, need to make sure not being used in production
                    deleteTemplateFromProduction(psTempl,
                                                 selectedPlayer as UBYTE);
                    heapFree(psTemplateHeap, psTempl as *mut libc::c_void);
                    /* get previous template and set as current */
                    psTempl =
                        *apsTemplateList.offset(i.wrapping_sub(1 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uint)
                                                    as isize);
                    /* update local list */
                    desSetupDesignTemplates();
                    /* Now update the droid template form */
                    newTemplate = 0 as libc::c_int;
                    widgDelete(psWScreen, 5002 as libc::c_int as UDWORD);
                    widgDelete(psWScreen, 5020 as libc::c_int as UDWORD);
                    intAddTemplateForm(psTempl);
                    /* Set the new template */
                    memcpy(&mut sCurrDesign as *mut DROID_TEMPLATE as
                               *mut libc::c_void,
                           psTempl as *const libc::c_void,
                           ::std::mem::size_of::<DROID_TEMPLATE>() as
                               libc::c_ulong);
                    //strcpy( sCurrDesign.aName, aCurrName );
                    strncpy(aCurrName.as_mut_ptr(), getTemplateName(psTempl),
                            (80 as libc::c_int - 1 as libc::c_int) as
                                libc::c_uint);
                    intSetEditBoxTextFromTemplate(psTempl);
                    intSetDesignStats(&mut sCurrDesign);
                    /* show body stats only */
                    widgReveal(psWScreen, 5001 as libc::c_int as UDWORD);
                    widgReveal(psWScreen, 5007 as libc::c_int as UDWORD);
                    widgHide(psWScreen, 5008 as libc::c_int as UDWORD);
                    widgHide(psWScreen, 5006 as libc::c_int as UDWORD);
                    /*Update the Power bar stats as the power to build will have changed */
                    intSetDesignPower(&mut sCurrDesign);
                    /*Update the body points */
                    intSetBodyPoints(&mut sCurrDesign);
                    /* show correct body component highlight */
                    widgDelete(psWScreen, 5003 as libc::c_int as UDWORD);
                    widgDelete(psWScreen, 5021 as libc::c_int as UDWORD);
                    desCompMode = IDES_NOCOMPONENT;
                    intSetDesignMode(IDES_BODY);
                }
            }
            5900 => {
                // Add the correct component form
                match droidTemplateType(&mut sCurrDesign) as libc::c_uint {
                    7 | 1 | 3 | 2 | 8 => {
                        /*
				intSetDesignMode(IDES_BRAIN);
				break;
*/
                        intSetDesignMode(IDES_SYSTEM);
                    }
                    _ => { intSetDesignMode(IDES_TURRET); }
                }
                /* reveal components if not already onscreen */
                widgReveal(psWScreen, 5001 as libc::c_int as UDWORD);
                widgReveal(psWScreen, 5021 as libc::c_int as UDWORD);
                widgReveal(psWScreen, 5006 as libc::c_int as UDWORD);
                widgHide(psWScreen, 5007 as libc::c_int as UDWORD);
                widgHide(psWScreen, 5008 as libc::c_int as UDWORD);
                /* lock button if design complete */
                if intValidTemplate(&mut sCurrDesign) == 1 as libc::c_int {
                    widgSetButtonState(psWScreen,
                                       5900 as libc::c_int as UDWORD,
                                       0x4 as libc::c_int as UDWORD);
                    widgSetButtonState(psWScreen,
                                       5901 as libc::c_int as UDWORD,
                                       0 as libc::c_int as UDWORD);
                    widgSetButtonState(psWScreen,
                                       5902 as libc::c_int as UDWORD,
                                       0 as libc::c_int as UDWORD);
                }
            }
            5901 => {
                /* reveal components if not already onscreen */
                widgReveal(psWScreen, 5021 as libc::c_int as UDWORD);
                intSetDesignMode(IDES_BODY);
                widgReveal(psWScreen, 5001 as libc::c_int as UDWORD);
                widgHide(psWScreen, 5006 as libc::c_int as UDWORD);
                widgReveal(psWScreen, 5007 as libc::c_int as UDWORD);
                widgHide(psWScreen, 5008 as libc::c_int as UDWORD);
                /* lock button if design complete */
                if intValidTemplate(&mut sCurrDesign) == 1 as libc::c_int {
                    widgSetButtonState(psWScreen,
                                       5900 as libc::c_int as UDWORD,
                                       0 as libc::c_int as UDWORD);
                    widgSetButtonState(psWScreen,
                                       5901 as libc::c_int as UDWORD,
                                       0x4 as libc::c_int as UDWORD);
                    widgSetButtonState(psWScreen,
                                       5902 as libc::c_int as UDWORD,
                                       0 as libc::c_int as UDWORD);
                }
            }
            5902 => {
                /* reveal components if not already onscreen */
                widgReveal(psWScreen, 5021 as libc::c_int as UDWORD);
                intSetDesignMode(IDES_PROPULSION);
                widgReveal(psWScreen, 5001 as libc::c_int as UDWORD);
                widgHide(psWScreen, 5006 as libc::c_int as UDWORD);
                widgHide(psWScreen, 5007 as libc::c_int as UDWORD);
                widgReveal(psWScreen, 5008 as libc::c_int as UDWORD);
                /* lock button if design complete */
                if intValidTemplate(&mut sCurrDesign) == 1 as libc::c_int {
                    widgSetButtonState(psWScreen,
                                       5900 as libc::c_int as UDWORD,
                                       0 as libc::c_int as UDWORD);
                    widgSetButtonState(psWScreen,
                                       5901 as libc::c_int as UDWORD,
                                       0 as libc::c_int as UDWORD);
                    widgSetButtonState(psWScreen,
                                       5902 as libc::c_int as UDWORD,
                                       0x4 as libc::c_int as UDWORD);
                }
            }
            _ => { }
        }
    }
    /* show body button if component button pressed and
	 * save template if valid
	 */
    if id >= 5500 as libc::c_int as libc::c_uint &&
           id <= 5699 as libc::c_int as libc::c_uint ||
           id >= 5700 as libc::c_int as libc::c_uint &&
               id <= 5899 as libc::c_int as libc::c_uint {
        /* reveal body button if hidden */
        widgReveal(psWScreen, 5901 as libc::c_int as UDWORD);
        /* save template if valid */
        if saveTemplate() != 0 {
            eventFireCallbackTrigger(CALL_DROIDDESIGNED as libc::c_int as
                                         TRIGGER_TYPE);
        }
        match desCompMode as libc::c_uint {
            3 => {
                widgReveal(psWScreen, 5007 as libc::c_int as UDWORD);
                widgHide(psWScreen, 5008 as libc::c_int as UDWORD);
                widgHide(psWScreen, 5006 as libc::c_int as UDWORD);
            }
            4 => {
                widgHide(psWScreen, 5007 as libc::c_int as UDWORD);
                widgReveal(psWScreen, 5008 as libc::c_int as UDWORD);
                widgHide(psWScreen, 5006 as libc::c_int as UDWORD);
            }
            0 | 1 | 2 => {
                widgHide(psWScreen, 5007 as libc::c_int as UDWORD);
                widgHide(psWScreen, 5008 as libc::c_int as UDWORD);
                widgReveal(psWScreen, 5006 as libc::c_int as UDWORD);
            }
            _ => { }
        }
        widgReveal(psWScreen, 5001 as libc::c_int as UDWORD);
        /* switch automatically to next component type if initial design */
        if intValidTemplate(&mut sCurrDesign) == 0 {
            /* show next component design screen */
            match desCompMode as libc::c_uint {
                3 => {
                    intSetDesignMode(IDES_PROPULSION);
                    widgReveal(psWScreen, 5902 as libc::c_int as UDWORD);
                    widgSetButtonState(psWScreen,
                                       5902 as libc::c_int as UDWORD,
                                       0x4 as libc::c_int as UDWORD);
                }
                4 => {
                    intSetDesignMode(IDES_TURRET);
                    widgReveal(psWScreen, 5900 as libc::c_int as UDWORD);
                    widgSetButtonState(psWScreen,
                                       5900 as libc::c_int as UDWORD,
                                       0x4 as libc::c_int as UDWORD);
                }
                0 | 1 | 2 => { intSetDesignMode(IDES_BODY); }
                _ => { }
            }
        }
    }
    //save the template if the name gets edited
    if id == 5013 as libc::c_int as libc::c_uint { saveTemplate(); };
}
/* Set the shadow bar graphs for the design screen */
#[no_mangle]
pub unsafe extern "C" fn intRunDesign() {
    let mut statID: UDWORD = 0;
    let mut psStats: *mut COMP_BASE_STATS = 0 as *mut COMP_BASE_STATS;
    let mut templateButton: BOOL = 0;
    /* Find out which button was hilited */
    templateButton = 0 as libc::c_int;
    statID = widgGetMouseOver(psWScreen);
    // Somut around here is casuing a nasty crash.....
	/* If a component button is hilited get the stats for it */
    if statID == desCompID {
        /* The mouse is over the selected component - no shadow stats */
        psStats = 0 as *mut COMP_BASE_STATS
    } else if statID >= 5500 as libc::c_int as libc::c_uint &&
                  statID <= 5699 as libc::c_int as libc::c_uint {
        //DBPRINTF(("1 %p\n",psStats);
        psStats =
            *apsComponentList.offset(statID.wrapping_sub(5500 as libc::c_int
                                                             as libc::c_uint)
                                         as isize)
    } else if statID >= 5700 as libc::c_int as libc::c_uint &&
                  statID <= 5899 as libc::c_int as libc::c_uint {
        //DBPRINTF(("2 %p\n",psStats);
        psStats =
            *apsExtraSysList.offset(statID.wrapping_sub(5700 as libc::c_int as
                                                            libc::c_uint) as
                                        isize)
    } else if statID >= 5300 as libc::c_int as libc::c_uint &&
                  statID <= 5339 as libc::c_int as libc::c_uint {
        //DBPRINTF(("3 %d\n",statID);
        runTemplateShadowStats(statID);
        templateButton = 1 as libc::c_int;
        psStats = 0 as *mut COMP_BASE_STATS
        //DBPRINTF(("4 %p\n",psStats);
    } else {
        /* No component button so reset the stats to nothing */
        psStats = 0 as *mut COMP_BASE_STATS
    }
    /* Now set the bar graphs for the stats - don't bother if over template
	since setting them all!*/
    if templateButton == 0 {
        match desCompMode as libc::c_uint {
            1 | 2 => {
                intSetSystemShadowStats(psStats);
                intSetBodyShadowStats(0 as *mut BODY_STATS);
                intSetPropulsionShadowStats(0 as *mut PROPULSION_STATS);
            }
            3 => {
                intSetSystemShadowStats(0 as *mut COMP_BASE_STATS);
                intSetBodyShadowStats(psStats as *mut BODY_STATS);
                intSetPropulsionShadowStats(0 as *mut PROPULSION_STATS);
            }
            4 => {
                intSetSystemShadowStats(0 as *mut COMP_BASE_STATS);
                intSetBodyShadowStats(0 as *mut BODY_STATS);
                intSetPropulsionShadowStats(psStats as *mut PROPULSION_STATS);
            }
            0 | _ => { }
        }
        //set the template shadow stats
        intSetTemplateBodyShadowStats(psStats);
        intSetTemplatePowerShadowStats(psStats);
    };
}
#[no_mangle]
pub unsafe extern "C" fn intDisplayStatForm(mut psWidget: *mut _widget,
                                            mut xOffset: UDWORD,
                                            mut yOffset: UDWORD,
                                            mut pColours: *mut UDWORD) {
    let mut Form: *mut W_CLICKFORM = psWidget as *mut W_CLICKFORM;
    let mut x0: UWORD = 0;
    let mut y0: UWORD = 0;
    static mut iRY: UDWORD = 45 as libc::c_int as UDWORD;
    //	BOOL			Hilight = FALSE;
    let mut psStats: *mut BASE_STATS = 0 as *mut BASE_STATS;
    let mut Rotation: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut Position: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut templateRadius: SWORD = 0;
    let mut falseScale: SDWORD = 0;
    /* get stats from userdata pointer in widget stored in
	 * intSetSystemStats, intSetBodyStats, intSetPropulsionStats
	 */
    psStats = (*Form).pUserData as *mut BASE_STATS;
    x0 = xOffset.wrapping_add((*Form).x as libc::c_uint) as UWORD;
    y0 = yOffset.wrapping_add((*Form).y as libc::c_uint) as UWORD;
    DrawBegin();
    pie_ImageFileID(IntImages, IMAGE_DES_STATBACKLEFT as libc::c_int as UWORD,
                    x0 as libc::c_int, y0 as libc::c_int);
    pie_ImageFileIDTile(IntImages,
                        IMAGE_DES_STATBACKMID as libc::c_int as UWORD,
                        x0 as libc::c_int +
                            iV_GetImageWidth(IntImages,
                                             IMAGE_DES_STATBACKLEFT as
                                                 libc::c_int as UWORD) as
                                libc::c_int, y0 as libc::c_int,
                        0 as libc::c_int, 0 as libc::c_int,
                        (*Form).width as libc::c_int -
                            iV_GetImageWidth(IntImages,
                                             IMAGE_DES_STATBACKLEFT as
                                                 libc::c_int as UWORD) as
                                libc::c_int -
                            iV_GetImageWidth(IntImages,
                                             IMAGE_DES_STATBACKRIGHT as
                                                 libc::c_int as UWORD) as
                                libc::c_int,
                        iV_GetImageHeight(IntImages,
                                          IMAGE_DES_STATBACKMID as libc::c_int
                                              as UWORD) as libc::c_int);
    pie_ImageFileID(IntImages,
                    IMAGE_DES_STATBACKRIGHT as libc::c_int as UWORD,
                    x0 as libc::c_int + (*Form).width as libc::c_int -
                        iV_GetImageWidth(IntImages,
                                         IMAGE_DES_STATBACKRIGHT as
                                             libc::c_int as UWORD) as
                            libc::c_int, y0 as libc::c_int);
    /* display current component */
    pie_SetGeometricOffset(xOffset.wrapping_add(((*psWidget).width as
                                                     libc::c_int /
                                                     4 as libc::c_int) as
                                                    libc::c_uint) as
                               libc::c_int,
                           yOffset.wrapping_add(((*psWidget).height as
                                                     libc::c_int /
                                                     2 as libc::c_int) as
                                                    libc::c_uint) as
                               libc::c_int);
    Rotation.x = -(30 as libc::c_int);
    Rotation.y = iRY as int32;
    Rotation.z = 0 as libc::c_int;
    /* inc rotation if highlighted */
    if (*Form).state & 0x4 as libc::c_int as libc::c_uint != 0 {
        iRY =
            (iRY as
                 libc::c_uint).wrapping_add((90 as libc::c_int as
                                                 libc::c_uint).wrapping_mul(frameTime2).wrapping_div(1000
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         libc::c_uint))
                as UDWORD as UDWORD;
        iRY =
            (iRY as
                 libc::c_uint).wrapping_rem(360 as libc::c_int as
                                                libc::c_uint) as UDWORD as
                UDWORD
    }
    templateRadius = getComponentRadius(psStats) as SWORD;
    Position.x = 0 as libc::c_int;
    Position.y = -(templateRadius as libc::c_int) / 4 as libc::c_int;
    //	Position.z = templateRadius * 12;
    Position.z = 2000 as libc::c_int;
    //scale the object around the BUTTON_RADIUS so that half size objects are draw are draw 75% the size of normal objects
    falseScale =
        150 as libc::c_int * 64 as libc::c_int /
            templateRadius as libc::c_int;
    falseScale =
        falseScale / 2 as libc::c_int + 150 as libc::c_int / 2 as libc::c_int;
    //display component in bottom design screen window
    displayComponentButton(psStats, &mut Rotation, &mut Position,
                           1 as libc::c_int, falseScale);
    DrawEnd();
}
/* Displays the 3D view of the droid in a window on the design form */
#[no_mangle]
pub unsafe extern "C" fn intDisplayViewForm(mut psWidget: *mut _widget,
                                            mut xOffset: UDWORD,
                                            mut yOffset: UDWORD,
                                            mut pColours: *mut UDWORD) {
    let mut Form: *mut W_FORM = psWidget as *mut W_FORM;
    let mut x0: UDWORD = 0;
    let mut y0: UDWORD = 0;
    let mut x1: UDWORD = 0;
    let mut y1: UDWORD = 0;
    static mut iRY: UDWORD = 45 as libc::c_int as UDWORD;
    let mut Rotation: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut Position: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut templateRadius: SWORD = 0;
    let mut falseScale: SDWORD = 0;
    x0 = xOffset.wrapping_add((*Form).x as libc::c_uint);
    y0 = yOffset.wrapping_add((*Form).y as libc::c_uint);
    x1 = x0.wrapping_add((*Form).width as libc::c_uint);
    y1 = y0.wrapping_add((*Form).height as libc::c_uint);
    RenderWindowFrame(&mut FrameNormal, x0, y0, x1.wrapping_sub(x0),
                      y1.wrapping_sub(y0));
    if !CurrentStatsTemplate.is_null() {
        pie_SetGeometricOffset(((23 as libc::c_int + 132 as libc::c_int +
                                     6 as libc::c_int) as
                                    libc::c_uint).wrapping_add(pie_GetVideoBufferWidth().wrapping_sub(640
                                                                                                          as
                                                                                                          libc::c_int
                                                                                                          as
                                                                                                          libc::c_uint).wrapping_div(2
                                                                                                                                         as
                                                                                                                                         libc::c_int
                                                                                                                                         as
                                                                                                                                         libc::c_uint)).wrapping_add(8
                                                                                                                                                                         as
                                                                                                                                                                         libc::c_int
                                                                                                                                                                         as
                                                                                                                                                                         libc::c_uint).wrapping_add((236
                                                                                                                                                                                                         as
                                                                                                                                                                                                         libc::c_int
                                                                                                                                                                                                         /
                                                                                                                                                                                                         2
                                                                                                                                                                                                             as
                                                                                                                                                                                                             libc::c_int)
                                                                                                                                                                                                        as
                                                                                                                                                                                                        libc::c_uint)
                                   as libc::c_int,
                               (59 as libc::c_int as
                                    libc::c_uint).wrapping_add(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_uint).wrapping_div(2
                                                                                                                                          as
                                                                                                                                          libc::c_int
                                                                                                                                          as
                                                                                                                                          libc::c_uint)).wrapping_add(25
                                                                                                                                                                          as
                                                                                                                                                                          libc::c_int
                                                                                                                                                                          as
                                                                                                                                                                          libc::c_uint).wrapping_add((192
                                                                                                                                                                                                          as
                                                                                                                                                                                                          libc::c_int
                                                                                                                                                                                                          /
                                                                                                                                                                                                          4
                                                                                                                                                                                                              as
                                                                                                                                                                                                              libc::c_int)
                                                                                                                                                                                                         as
                                                                                                                                                                                                         libc::c_uint).wrapping_add(32
                                                                                                                                                                                                                                        as
                                                                                                                                                                                                                                        libc::c_int
                                                                                                                                                                                                                                        as
                                                                                                                                                                                                                                        libc::c_uint)
                                   as libc::c_int);
        Rotation.x = -(30 as libc::c_int);
        //		Rotation.y = ViewRotation;
        Rotation.y = iRY as int32;
        Rotation.z = 0 as libc::c_int;
        /* inc rotation */
        iRY =
            (iRY as
                 libc::c_uint).wrapping_add((90 as libc::c_int as
                                                 libc::c_uint).wrapping_mul(frameTime2).wrapping_div(1000
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         libc::c_uint))
                as UDWORD as UDWORD;
        iRY =
            (iRY as
                 libc::c_uint).wrapping_rem(360 as libc::c_int as
                                                libc::c_uint) as UDWORD as
                UDWORD;
        //fixed depth scale the pie
        Position.x = 0 as libc::c_int;
        Position.y = -(100 as libc::c_int);
        Position.z = 2000 as libc::c_int;
        templateRadius =
            getComponentDroidTemplateRadius(CurrentStatsTemplate as
                                                *mut DROID_TEMPLATE) as SWORD;
        //scale the object around the OBJECT_RADIUS so that half size objects are draw are draw 75% the size of normal objects
        falseScale =
            200 as libc::c_int * 128 as libc::c_int /
                templateRadius as libc::c_int;
        //display large droid view in the design screen
        displayComponentButtonTemplate(&mut sCurrDesign as
                                           *mut DROID_TEMPLATE, &mut Rotation,
                                       &mut Position, 1 as libc::c_int,
                                       falseScale);
    };
    //	ViewRotation+=2;
}
#[no_mangle]
pub unsafe extern "C" fn intDisplayTemplateButton(mut psWidget: *mut _widget,
                                                  mut xOffset: UDWORD,
                                                  mut yOffset: UDWORD,
                                                  mut pColours: *mut UDWORD) {
    intDisplayStatsButton(psWidget, xOffset, yOffset, pColours);
}
#[no_mangle]
pub unsafe extern "C" fn intDisplayComponentButton(mut psWidget: *mut _widget,
                                                   mut xOffset: UDWORD,
                                                   mut yOffset: UDWORD,
                                                   mut pColours:
                                                       *mut UDWORD) {
    //	iIMDShape *OldCurShape = CurrentStatsShape;
//	SWORD OldCurIndex = CurrentStatsIndex;
    let mut OldCurStatsTemplate: *mut BASE_STATS = CurrentStatsTemplate;
    intDisplayStatsButton(psWidget, xOffset, yOffset, pColours);
    CurrentStatsTemplate = OldCurStatsTemplate;
    //	CurrentStatsShape = OldCurShape;
//	CurrentStatsIndex = OldCurIndex;
}
/* General display window for the design form */
/* General display window for the design form  SOLID BACKGROUND - NOT TRANSPARENT*/
#[no_mangle]
pub unsafe extern "C" fn intDisplayDesignForm(mut psWidget: *mut _widget,
                                              mut xOffset: UDWORD,
                                              mut yOffset: UDWORD,
                                              mut pColours: *mut UDWORD) {
    let mut Form: *mut W_TABFORM = psWidget as *mut W_TABFORM;
    let mut x0: UDWORD = 0;
    let mut y0: UDWORD = 0;
    let mut x1: UDWORD = 0;
    let mut y1: UDWORD = 0;
    x0 = xOffset.wrapping_add((*Form).x as libc::c_uint);
    y0 = yOffset.wrapping_add((*Form).y as libc::c_uint);
    x1 = x0.wrapping_add((*Form).width as libc::c_uint);
    y1 = y0.wrapping_add((*Form).height as libc::c_uint);
    //AdjustTabFormSize(Form,&x0,&y0,&x1,&y1);
    //RenderWindowFrame(&FrameDesignView,x0,y0,x1-x0,y1-y0);
    RenderWindowFrame(&mut FrameNormal, x0, y0, x1.wrapping_sub(x0),
                      y1.wrapping_sub(y0));
}
/* save the current Template if valid. Return TRUE if stored */
// NOTE!! if(when) this function is changed, please mail alexlee.
/* save the current Template if valid. Return TRUE if stored */
#[no_mangle]
pub unsafe extern "C" fn saveTemplate() -> BOOL {
    let mut psTempl: *mut DROID_TEMPLATE = 0 as *mut DROID_TEMPLATE;
    let mut psPlayerTempl: *mut DROID_TEMPLATE = 0 as *mut DROID_TEMPLATE;
    let mut psPrevTempl: *mut DROID_TEMPLATE = 0 as *mut DROID_TEMPLATE;
    let mut stored: BOOL = 0 as libc::c_int;
    let mut bTemplateFound: BOOL = 0 as libc::c_int;
    let mut i: UDWORD = 0;
    let mut iCurrID: UDWORD = 0;
    /* if first (New Design) button selected find empty template
	 * else find current button template
	 */
    if droidTemplID == 5300 as libc::c_int as libc::c_uint {
        /* find empty template and point to that */
        i = 1 as libc::c_int as UDWORD;
        while i < 40 as libc::c_int as libc::c_uint {
            psTempl = *apsTemplateList.offset(i as isize);
            if psTempl.is_null() {
                bTemplateFound = 1 as libc::c_int;
                break ;
            } else { i = i.wrapping_add(1) }
        }
    } else {
        /* Find the template for the current button */
        iCurrID = (5300 as libc::c_int + 1 as libc::c_int) as UDWORD;
        i = 1 as libc::c_int as UDWORD;
        while i < 40 as libc::c_int as libc::c_uint {
            psTempl = *apsTemplateList.offset(i as isize);
            if iCurrID == droidTemplID {
                bTemplateFound = 1 as libc::c_int;
                break ;
            } else {
                iCurrID = iCurrID.wrapping_add(1);
                i = i.wrapping_add(1)
            }
        }
    }
    if bTemplateFound == 1 as libc::c_int &&
           intValidTemplate(&mut sCurrDesign) != 0 {
        /* create new template if button is NULL,
		 * else store changes to existing template */
        if psTempl.is_null() {
            /* The design needs a new template in the list */
            if heapAlloc(psTemplateHeap,
                         &mut psTempl as *mut *mut DROID_TEMPLATE as
                             *mut *mut libc::c_void) == 0 {
                debug(LOG_NEVER,
                      b"saveTemplate: heap alloc failed\n\x00" as *const u8 as
                          *const libc::c_char);
                return 0 as libc::c_int
            }
            (*psTempl).ref_0 = 0xc0000 as libc::c_int as UDWORD;
            newTemplate = 1 as libc::c_int;
            /* Add it to temp array */
            let ref mut fresh6 = *apsTemplateList.offset(i as isize);
            *fresh6 = psTempl;
            /* update player template list */
            psPlayerTempl = apsDroidTemplates[selectedPlayer as usize];
            psPrevTempl = 0 as *mut DROID_TEMPLATE;
            while !psPlayerTempl.is_null() {
                psPrevTempl = psPlayerTempl;
                psPlayerTempl = (*psPlayerTempl).psNext
            }
            if psPrevTempl.is_null() {
                apsDroidTemplates[selectedPlayer as usize] = psTempl
            } else { (*psPrevTempl).psNext = psTempl }
            /* set button render routines to highlight, not flash */
            intSetButtonFlash(5900 as libc::c_int as UDWORD,
                              0 as libc::c_int);
            intSetButtonFlash(5901 as libc::c_int as UDWORD,
                              0 as libc::c_int);
            intSetButtonFlash(5902 as libc::c_int as UDWORD,
                              0 as libc::c_int);
            /* reset all button states */
            widgSetButtonState(psWScreen, 5900 as libc::c_int as UDWORD,
                               0 as libc::c_int as UDWORD);
            widgSetButtonState(psWScreen, 5901 as libc::c_int as UDWORD,
                               0 as libc::c_int as UDWORD);
            widgSetButtonState(psWScreen, 5902 as libc::c_int as UDWORD,
                               0 as libc::c_int as UDWORD);
        } else {
            /* Get existing template */
            psTempl = *apsTemplateList.offset(i as isize);
            newTemplate = 0 as libc::c_int;
            /*ANY change to the template affect the production - even if the
            template is changed and then changed back again!*/
            deleteTemplateFromProduction(psTempl, selectedPlayer as UBYTE);
        }
        /* Copy the template */
        memcpy(psTempl as *mut libc::c_void,
               &mut sCurrDesign as *mut DROID_TEMPLATE as *const libc::c_void,
               ::std::mem::size_of::<DROID_TEMPLATE>() as libc::c_ulong);
        strncpy((*psTempl).aName.as_mut_ptr(), aCurrName.as_mut_ptr(),
                60 as libc::c_int as libc::c_uint);
        (*psTempl).aName[(60 as libc::c_int - 1 as libc::c_int) as usize] =
            0 as libc::c_int as STRING;
        /* Now update the droid template form */
        widgDelete(psWScreen, 5002 as libc::c_int as UDWORD);
        widgDelete(psWScreen, 5020 as libc::c_int as UDWORD);
        intAddTemplateForm(psTempl);
        stored = 1 as libc::c_int
    }
    if stored != 0 {
        (*psTempl).multiPlayerID = objID << 3 as libc::c_int | selectedPlayer;
        objID = objID.wrapping_add(1);
        if bMultiPlayer != 0 { sendTemplate(psTempl); }
    }
    return stored;
}
/* Return the location of a COMP_BASE_STATS */
//static LOC intGetLocation(COMP_BASE_STATS *psStats);
/*Function to set the shadow bars for all the stats when the mouse is over
the Template buttons*/
/*Function to set the shadow bars for all the stats when the mouse is over
the Template buttons*/
unsafe extern "C" fn runTemplateShadowStats(mut id: UDWORD) {
    let mut currID: UDWORD = 0;
    let mut psTempl: *mut DROID_TEMPLATE = 0 as *mut DROID_TEMPLATE;
    let mut psStats: *mut COMP_BASE_STATS = 0 as *mut COMP_BASE_STATS;
    let mut templType: DROID_TYPE = DROID_WEAPON;
    let mut i: UDWORD = 0;
    /* Find the template for the new button */
	//currID = IDDES_TEMPLSTART;
    //we're ignoring the Blank Design so start at the second button
    currID = (5300 as libc::c_int + 1 as libc::c_int) as UDWORD;
    i = 1 as libc::c_int as UDWORD;
    while i < 40 as libc::c_int as libc::c_uint {
        psTempl = *apsTemplateList.offset(i as isize);
        if currID == id { break ; }
        currID = currID.wrapping_add(1);
        i = i.wrapping_add(1)
    }
    //if we're over a different template
    if !psTempl.is_null() &&
           psTempl != &mut sCurrDesign as *mut DROID_TEMPLATE {
        /* Now set the bar graphs for the stats */
        intSetBodyShadowStats(asBodyStats.offset((*psTempl).asParts[COMP_BODY
                                                                        as
                                                                        libc::c_int
                                                                        as
                                                                        usize]
                                                     as isize));
        intSetPropulsionShadowStats(asPropulsionStats.offset((*psTempl).asParts[COMP_PROPULSION
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    usize]
                                                                 as isize));
        //only set the system shadow bar if the same type of droid
        psStats = 0 as *mut COMP_BASE_STATS;
        templType = droidTemplateType(psTempl);
        if templType as libc::c_uint ==
               droidTemplateType(&mut sCurrDesign) as libc::c_uint {
            match templType as libc::c_uint {
                0 => {
                    psStats =
                        asWeaponStats.offset((*psTempl).asWeaps[0 as
                                                                    libc::c_int
                                                                    as usize]
                                                 as isize) as
                            *mut COMP_BASE_STATS
                }
                1 => {
                    psStats =
                        asSensorStats.offset((*psTempl).asParts[COMP_SENSOR as
                                                                    libc::c_int
                                                                    as usize]
                                                 as isize) as
                            *mut COMP_BASE_STATS
                }
                2 => {
                    psStats =
                        asECMStats.offset((*psTempl).asParts[COMP_ECM as
                                                                 libc::c_int
                                                                 as usize] as
                                              isize) as *mut COMP_BASE_STATS
                }
                3 => {
                    psStats =
                        asConstructStats.offset((*psTempl).asParts[COMP_CONSTRUCT
                                                                       as
                                                                       libc::c_int
                                                                       as
                                                                       usize]
                                                    as isize) as
                            *mut COMP_BASE_STATS
                }
                8 => {
                    psStats =
                        asRepairStats.offset((*psTempl).asParts[COMP_REPAIRUNIT
                                                                    as
                                                                    libc::c_int
                                                                    as usize]
                                                 as isize) as
                            *mut COMP_BASE_STATS
                }
                _ => { }
            }
        }
        if !psStats.is_null() { intSetSystemShadowStats(psStats); }
        //set the template shadow stats
		//intSetTemplateBodyShadowStats(psStats);
		//haven't got a stat so just do the code required here...
        widgSetMinorBarSize(psWScreen, 5127 as libc::c_int as UDWORD,
                            calcTemplateBody(psTempl,
                                             selectedPlayer as UBYTE));
        //intSetTemplatePowerShadowStats(psStats);
        widgSetMinorBarSize(psWScreen, 5023 as libc::c_int as UDWORD,
                            calcTemplatePower(psTempl));
    };
}
/*sets which states need to be paused when the design screen is up*/
/*sets which states need to be paused when the design screen is up*/
#[no_mangle]
pub unsafe extern "C" fn setDesignPauseState() {
    if bMultiPlayer == 0 {
        gameTimeStop();
        setGameUpdatePause(1 as libc::c_int);
        setScrollPause(1 as libc::c_int);
    };
}
/*resets the pause states */
/*resets the pause states */
#[no_mangle]
pub unsafe extern "C" fn resetDesignPauseState() {
    if bMultiPlayer == 0 {
        setGameUpdatePause(0 as libc::c_int);
        setScrollPause(0 as libc::c_int);
        gameTimeStart();
    };
}
/*this is called when a new propulsion type is added to the current design
to check the weapon is 'allowed'. Check if VTOL, the weapon is direct fire.
Also check numVTOLattackRuns for the weapon is not zero - return TRUE if valid weapon*/
unsafe extern "C" fn intCheckValidWeaponForProp() -> BOOL {
    return checkValidWeaponForProp(&mut sCurrDesign);
}
// Propulsion button
#[no_mangle]
pub unsafe extern "C" fn intAddDesign(mut bShowCentreScreen: BOOL) -> BOOL {
    return _intAddDesign(bShowCentreScreen);
}
/* Set up the system clickable form of the design screen given a set of stats */
/* Set up the system clickable form of the design screen given a set of stats */
unsafe extern "C" fn intSetSystemForm(mut psStats: *mut COMP_BASE_STATS)
 -> BOOL {
    return _intSetSystemForm(psStats);
}
/* Add the template tab form to the design screen */
unsafe extern "C" fn intAddTemplateForm(mut psSelected: *mut DROID_TEMPLATE)
 -> BOOL {
    return _intAddTemplateForm(psSelected);
}
//checks if the template has LIFT propulsion attached - returns TRUE if it does
unsafe extern "C" fn checkTemplateIsVtol(mut psTemplate: *mut DROID_TEMPLATE)
 -> BOOL {
    if (*asPropulsionStats.offset((*psTemplate).asParts[COMP_PROPULSION as
                                                            libc::c_int as
                                                            usize] as
                                      isize)).propulsionType as libc::c_int ==
           LIFT as libc::c_int {
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
}
/*goes thru' the list passed in reversing the order so the first entry becomes
the last and the last entry becomes the first!*/
#[no_mangle]
pub unsafe extern "C" fn reverseTemplateList(mut ppsList:
                                                 *mut *mut DROID_TEMPLATE) {
    let mut psPrev: *mut DROID_TEMPLATE = 0 as *mut DROID_TEMPLATE;
    let mut psNext: *mut DROID_TEMPLATE = 0 as *mut DROID_TEMPLATE;
    let mut psCurrent: *mut DROID_TEMPLATE = 0 as *mut DROID_TEMPLATE;
    let mut psObjList: *mut DROID_TEMPLATE = 0 as *mut DROID_TEMPLATE;
    //initialise the pointers
    psObjList = *ppsList;
    psNext = 0 as *mut DROID_TEMPLATE;
    psPrev = psNext;
    psCurrent = psObjList;
    while !psCurrent.is_null() {
        psNext = (*psCurrent).psNext;
        (*psCurrent).psNext = psPrev;
        psPrev = psCurrent;
        psCurrent = psNext
    }
    //set the list passed in to point to the new top
    *ppsList = psPrev;
}
//calculates the weapons ROF based on the fire pause and the salvos
unsafe extern "C" fn weaponROF(mut psStat: *mut WEAPON_STATS) -> UWORD {
    let mut rof: UWORD = 0 as libc::c_int as UWORD;
    //if there are salvos
    if (*psStat).numRounds != 0 {
        if (*psStat).reloadTime != 0 as libc::c_int as libc::c_uint {
            rof =
                ((60 as libc::c_int * 1000 as libc::c_int) as
                     libc::c_uint).wrapping_div((*psStat).reloadTime) as
                    UWORD;
            //multiply by the number of salvos/shot
            rof =
                (rof as libc::c_int * (*psStat).numRounds as libc::c_int) as
                    UWORD
        }
    }
    if rof == 0 {
        rof = weaponFirePause(psStat, selectedPlayer as UBYTE) as UWORD;
        if rof as libc::c_int != 0 as libc::c_int {
            rof =
                (60 as libc::c_int * 1000 as libc::c_int / rof as libc::c_int)
                    as UWORD
        }
        //else leave it at 0
    }
    return rof;
}
