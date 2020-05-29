use ::libc;
extern "C" {
    /* The optional user callback function */
    pub type _w_context;
    #[no_mangle]
    fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    /* This returns true if the key is currently depressed */
    #[no_mangle]
    fn keyDown(code: KEY_CODE) -> BOOL;
    #[no_mangle]
    fn abs(_: libc::c_int) -> libc::c_int;
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
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_uint)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_uint)
     -> *mut libc::c_void;
    #[no_mangle]
    fn frameGetFrameNumber() -> UDWORD;
    #[no_mangle]
    fn audio_QueueTrack(iTrack: SDWORD);
    #[no_mangle]
    fn audio_QueueTrackMinDelay(iTrack: SDWORD, iMinDelay: UDWORD);
    #[no_mangle]
    fn audio_QueueTrackPos(iTrack: SDWORD, iX: SDWORD, iY: SDWORD,
                           iZ: SDWORD);
    #[no_mangle]
    fn weaponFirePause(psStats: *mut WEAPON_STATS, player: UBYTE) -> UDWORD;
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
    /* get demolish stat */
    #[no_mangle]
    fn structGetDemolishStat() -> *mut STRUCTURE_STATS;
    // Set the command droid that factory production should go to
//struct _command_droid;
    #[no_mangle]
    fn assignFactoryCommandDroid(psStruct: *mut STRUCTURE,
                                 psCommander: *mut _droid);
    /*for a given structure, return a pointer to its module stat */
    #[no_mangle]
    fn getModuleStat(psStruct: *mut STRUCTURE) -> *mut STRUCTURE_STATS;
    /*Access functions for the upgradeable stats of a structure*/
    #[no_mangle]
    fn structureBody(psStruct: *mut STRUCTURE) -> UDWORD;
    #[no_mangle]
    fn structureResistance(psStats: *mut STRUCTURE_STATS, player: UBYTE)
     -> UDWORD;
    // Is a structure a factory of somekind?
    #[no_mangle]
    fn StructIsFactory(Struct: *mut STRUCTURE) -> BOOL;
    /*compares the structure sensor type with the droid weapon type to see if the 
FIRE_SUPPORT order can be assigned*/
    #[no_mangle]
    fn structSensorDroidWeapon(psStruct: *mut STRUCTURE, psDroid: *mut DROID)
     -> BOOL;
    /*checks if the structure has a Counter Battery sensor attached - returns 
TRUE if it has*/
    #[no_mangle]
    fn structCBSensor(psStruct: *mut STRUCTURE) -> BOOL;
    /*checks if the structure has a Standard Turret sensor attached - returns 
TRUE if it has*/
    #[no_mangle]
    fn structStandardSensor(psStruct: *mut STRUCTURE) -> BOOL;
    /*checks if the structure has a VTOL Intercept sensor attached - returns 
TRUE if it has*/
    #[no_mangle]
    fn structVTOLSensor(psStruct: *mut STRUCTURE) -> BOOL;
    /*checks if the structure has a VTOL Counter Battery sensor attached - returns 
TRUE if it has*/
    #[no_mangle]
    fn structVTOLCBSensor(psStruct: *mut STRUCTURE) -> BOOL;
    /* Just returns true if the structure's present body points aren't as high as the original*/
    #[no_mangle]
    fn structIsDamaged(psStruct: *mut STRUCTURE) -> BOOL;
    /*checks the structure passed in is a Las Sat structure which is currently 
selected - returns TRUE if valid*/
    #[no_mangle]
    fn lasSatStructSelected(psStruct: *mut STRUCTURE) -> BOOL;
    #[no_mangle]
    static mut apsStructLists: [*mut STRUCTURE; 8];
    /* Check no alliance has formed*/
    #[no_mangle]
    fn aiCheckAlliances(s1: UDWORD, s2: UDWORD) -> BOOL;
    // Find the nearest target to a droid
    #[no_mangle]
    fn aiNearestTarget(psDroid: *mut DROID, ppsObj: *mut *mut BASE_OBJECT)
     -> BOOL;
    /*set of rules which determine whether the weapon associated with the object
can fire on the propulsion type of the target*/
    #[no_mangle]
    fn validTarget(psObject: *mut BASE_OBJECT, psTarget: *mut BASE_OBJECT)
     -> BOOL;
    #[no_mangle]
    static mut asSensorStats: *mut SENSOR_STATS;
    #[no_mangle]
    static mut asWeaponStats: *mut WEAPON_STATS;
    #[no_mangle]
    fn recycleDroid(psDel: *mut DROID);
    #[no_mangle]
    fn destroyDroid(psDel: *mut DROID);
    #[no_mangle]
    fn droidIsDamaged(psDroid: *mut DROID) -> BOOL;
    #[no_mangle]
    fn buildModule(psDroid: *mut DROID, psStruct: *mut STRUCTURE,
                   bCheckPower: BOOL) -> BOOL;
    #[no_mangle]
    fn checkDroidsDemolishing(psStructure: *mut STRUCTURE) -> BOOL;
    #[no_mangle]
    fn checkDroidsBuilding(psStructure: *mut STRUCTURE) -> BOOL;
    #[no_mangle]
    fn electronicDroid(psDroid: *mut DROID) -> BOOL;
    #[no_mangle]
    fn vtolEmpty(psDroid: *mut DROID) -> BOOL;
    #[no_mangle]
    fn vtolDroid(psDroid: *mut DROID) -> BOOL;
    #[no_mangle]
    fn droidSensorDroidWeapon(psObj: *mut BASE_OBJECT, psDroid: *mut DROID)
     -> BOOL;
    #[no_mangle]
    fn allVtolsRearmed(psDroid: *mut DROID) -> BOOL;
    #[no_mangle]
    fn droidAttacking(psDroid: *mut DROID) -> BOOL;
    #[no_mangle]
    fn vtolRearming(psDroid: *mut DROID) -> BOOL;
    #[no_mangle]
    fn assignVTOLPad(psNewDroid: *mut DROID, psReArmPad: *mut STRUCTURE);
    #[no_mangle]
    fn DeSelectDroid(psDroid: *mut DROID);
    /*returns TRUE if droid type is one of the Cyborg types*/
    #[no_mangle]
    fn cyborgDroid(psDroid: *mut DROID) -> BOOL;
    /* Give a droid an action */
    #[no_mangle]
    fn actionDroid(psDroid: *mut DROID, action: DROID_ACTION);
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
    // check if a target is within weapon range
    #[no_mangle]
    fn actionInRange(psDroid: *mut DROID, psObj: *mut BASE_OBJECT) -> BOOL;
    // check if a target is inside minimum weapon range
    #[no_mangle]
    fn actionInsideMinRange(psDroid: *mut DROID, psObj: *mut BASE_OBJECT)
     -> BOOL;
    // return whether a droid can see a target to fire on it
    #[no_mangle]
    fn actionVisibleTarget(psDroid: *mut DROID, psTarget: *mut BASE_OBJECT)
     -> BOOL;
    // check whether a droid is in the neighboring tile to a build position
    #[no_mangle]
    fn actionReachedBuildPos(psDroid: *mut DROID, x: SDWORD, y: SDWORD,
                             psStats: *mut BASE_STATS) -> BOOL;
    /*send the vtol droid back to the nearest rearming pad - if one otherwise
return to base*/
    #[no_mangle]
    fn moveToRearm(psDroid: *mut DROID);
    // choose a landing position for a VTOL when it goes to rearm
    #[no_mangle]
    fn actionVTOLLandingPos(psDroid: *mut DROID, px: *mut UDWORD,
                            py: *mut UDWORD) -> BOOL;
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
    // Remove a unit from a formation
    #[no_mangle]
    fn formationLeave(psFormation: *mut FORMATION, psObj: *mut BASE_OBJECT);
    #[no_mangle]
    static mut selectedPlayer: UDWORD;
    #[no_mangle]
    fn proj_SendProjectile(psWeap: *mut WEAPON, psAttacker: *mut BASE_OBJECT,
                           player: SDWORD, tarX: UDWORD, tarY: UDWORD,
                           tarZ: UDWORD, psTarget: *mut BASE_OBJECT,
                           bVisible: BOOL) -> BOOL;
    // return whether a weapon is direct or indirect
    #[no_mangle]
    fn proj_Direct(psStats: *mut WEAPON_STATS) -> BOOL;
    #[no_mangle]
    fn addEffect(pos: *mut iVector, group: EFFECT_GROUP, type_0: EFFECT_TYPE,
                 specified: BOOL, imd: *mut iIMDShape, lit: BOOL);
    // The maximum time for one frame (stops the clock running away when debugging)
// changed to /6 by ajl. if this needs to go back to ticks/10 then tell me. 
    /* The current time in the game world */
    #[no_mangle]
    static mut gameTime: UDWORD;
    #[no_mangle]
    fn widgGetFromID(psScreen: *mut W_SCREEN, id: UDWORD) -> *mut WIDGET;
    /*Add the Transporter Interface*/
    /* Remove the Transporter widgets from the screen */
    #[no_mangle]
    fn intRemoveTrans();
    #[no_mangle]
    fn transporterAddDroid(psTransporter: *mut DROID,
                           psDroidToAdd: *mut DROID);
    /*calculates how much space is remaining on the transporter - allows droids to take
up different amount depending on their body size - currently all are set to one!*/
    /*launches the defined transporter to the offworld map*/
    /*checks how long the transporter has been travelling to see if it should
have arrived - returns TRUE when there*/
    // Order all selected droids to embark all avaialable transporters.
    // Order a single droid to embark any available transporters.
    /* Remove the Transporter Launch widget from the screen*/
    //process the launch transporter button click
    /*This is used to display the transporter button and capacity when at the home base ONLY*/
    /* set current transporter (for script callbacks) */
    /* get current transporter (for script callbacks) */
    /* check whether transporter on mission */
//extern BOOL transporterOnMission( void );
    /*called when a Transporter has arrived back at the LZ when sending droids to safety*/
    /* get time transporter launch button was pressed */
    /*set the time for the Launch*/
    /*checks the order of the droid to see if its currenly flying*/
    #[no_mangle]
    fn transporterFlying(psTransporter: *mut DROID) -> BOOL;
    #[no_mangle]
    fn resetTransporter(psTransporter: *mut DROID);
    // remove a droid from a group
    #[no_mangle]
    fn grpLeave(psGroup: *mut DROID_GROUP, psDroid: *mut DROID);
    /* set the secondary state for a group of droids */
    #[no_mangle]
    fn grpSetSecondary(psGroup: *mut DROID_GROUP, sec: SECONDARY_ORDER,
                       state: SECONDARY_STATE);
    // add a droid to a command group
    // return the current target designator for a player
    #[no_mangle]
    fn cmdDroidGetDesignator(player: UDWORD) -> *mut DROID;
    // set the current target designator for a player
    #[no_mangle]
    fn cmdDroidSetDesignator(psDroid: *mut DROID);
    // set the current target designator for a player
    #[no_mangle]
    fn cmdDroidClearDesignator(player: UDWORD);
    #[no_mangle]
    fn cmdDroidAddDroid(psCommander: *mut DROID, psDroid: *mut DROID);
    #[no_mangle]
    fn eventFireCallbackTrigger(callback: TRIGGER_TYPE);
    // note when a vtol has finished returning to base - used to vanish
// vtols when they are attacking from off map
    #[no_mangle]
    static mut psScrCBVtolOffMap: *mut DROID;
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
    //send AI message
    #[no_mangle]
    fn turnOffMultiMsg(bDoit: BOOL) -> BOOL;
    #[no_mangle]
    fn sendLasSat(player: UBYTE, psStruct: *mut STRUCTURE,
                  psObj: *mut BASE_OBJECT) -> BOOL;
    #[no_mangle]
    fn SendDroidInfo(psDroid: *mut DROID, order: DROID_ORDER, x: UDWORD,
                     y: UDWORD, psObj: *mut BASE_OBJECT) -> BOOL;
    #[no_mangle]
    fn SendGroupOrderSelected(player: UBYTE, x: UDWORD, y: UDWORD,
                              psObj: *mut BASE_OBJECT) -> BOOL;
    #[no_mangle]
    fn SendCmdGroup(psGroup: *mut DROID_GROUP, x: UWORD, y: UWORD,
                    psObj: *mut BASE_OBJECT) -> BOOL;
    //extern BOOL SendDroidWaypoint	(UBYTE player, UDWORD	x, UDWORD y);
//extern BOOL SendSingleDroidWaypoint(DROID *psDroid, UDWORD x,UDWORD y);
    #[no_mangle]
    fn sendDroidSecondary(psDroid: *mut DROID, sec: SECONDARY_ORDER,
                          state: SECONDARY_STATE) -> BOOL;
    // return positions for vtols
    #[no_mangle]
    static mut asVTOLReturnPos: [POINT; 8];
    //returns TRUE if the mission is a Limbo Expand mission
    #[no_mangle]
    fn missionLimboExpand() -> BOOL;
    #[no_mangle]
    fn unloadTransporter(psTransporter: *mut DROID, x: UDWORD, y: UDWORD,
                         goingHome: BOOL);
    //returns the x coord for where the Transporter can land
    #[no_mangle]
    fn getLandingX(iPlayer: SDWORD) -> UWORD;
    //returns the y coord for where the Transporter can land
    #[no_mangle]
    fn getLandingY(iPlayer: SDWORD) -> UWORD;
    /* move transporter offworld */
    #[no_mangle]
    fn missionMoveTransporterOffWorld(psTransporter: *mut DROID);
    #[no_mangle]
    fn missionSetReinforcementTime(iTime: UDWORD);
    #[no_mangle]
    fn getDroidsToSafetyFlag() -> BOOL;
    //checks to see if the player has any droids (except Transporters left)
    #[no_mangle]
    fn missionDroidsRemaining(player: UDWORD) -> BOOL;
    /*called when a Transporter gets to the edge of the world and the droids are 
being flown to safety. The droids inside the Transporter are placed into the 
mission list for later use*/
    #[no_mangle]
    fn moveDroidsToSafety(psTransporter: *mut DROID);
    /* The widget screen */
    #[no_mangle]
    static mut psWScreen: *mut W_SCREEN;
    // add the construction interface if a constructor droid is selected
    #[no_mangle]
    fn intConstructorSelected(psDroid: *mut DROID);
    #[no_mangle]
    fn intBuildSelectMode() -> BOOL;
    #[no_mangle]
    fn intDemolishSelectMode() -> BOOL;
    /* Refresh icons on the interface, without disturbing the layout. i.e. smartreset*/
    #[no_mangle]
    fn intRefreshScreen();
    #[no_mangle]
    fn intDemolishCancel();
    /* Check whether psViewer can see psTarget
 * psViewer should be an object that has some form of sensor,
 * currently droids and structures.
 * psTarget can be any type of BASE_OBJECT (e.g. a tree).
 */
    #[no_mangle]
    fn visibleObject(psViewer: *mut BASE_OBJECT, psTarget: *mut BASE_OBJECT)
     -> BOOL;
    // Find the wall that is blocking LOS to a target (if any)
    #[no_mangle]
    fn visGetBlockingWall(psViewer: *mut BASE_OBJECT,
                          psTarget: *mut BASE_OBJECT,
                          ppsWall: *mut *mut STRUCTURE) -> BOOL;
    //access function for bSensorAssigned variable
    #[no_mangle]
    fn setSensorAssigned();
    // check whether the queue order keys are pressed
    #[no_mangle]
    fn ctrlShiftDown() -> BOOL;
    #[no_mangle]
    fn getWarCamStatus() -> BOOL;
    #[no_mangle]
    fn camToggleStatus();
    #[no_mangle]
    fn getTrackingDroid() -> *mut DROID;
    // Check if the map tile at a location blocks a droid
    #[no_mangle]
    fn fpathGroundBlockingTile(x: SDWORD, y: SDWORD) -> BOOL;
    //extern void	assignSensorTarget( DROID *psDroid );
    #[no_mangle]
    fn assignSensorTarget(psObj: *mut BASE_OBJECT);
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
pub type FRACT = libc::c_float;
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
// Feature armour
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
pub const DACTION_MOVETOBUILD: _droid_action = 18;
pub const DACTION_BUILDWANDER: _droid_action = 21;
pub const DACTION_BUILD: _droid_action = 2;
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
/* Give a group an order */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _droid_order_data {
    pub order: SDWORD,
    pub x: UWORD,
    pub y: UWORD,
    pub x2: UWORD,
    pub y2: UWORD,
    pub psObj: *mut BASE_OBJECT,
    pub psStats: *mut BASE_STATS,
}
pub type DROID_ORDER_DATA = _droid_order_data;
//UDWORD			x,y;
	//UDWORD			x2,y2;
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
pub const DACTION_MOVETOREPAIR: _droid_action = 20;
pub const DACTION_MOVETODEMOLISH: _droid_action = 19;
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
pub const DACTION_MOVE: _droid_action = 1;
pub const DACTION_NONE: _droid_action = 0;
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
/* The common widget data */
/* Button colours */
// Colour for button text
// Colour for button background
// Colour for button border
// 2nd border colour
// Hilite colour
/* The display function prototype */
pub type WIDGET_DISPLAY
    =
    Option<unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                _: *mut UDWORD) -> ()>;
pub type WIDGET_TYPE = _widget_type;
/*
 * WidgBase.h
 *
 * Definitions for the basic widget types.
 */
/* The different base types of widget */
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
// The root form of the screen
// The widget that has keyboard focus
//	PROP_FONT	*psTipFont;			// The font for tool tips
// ID of the IVIS font to use for tool tips.
// information about a formation
pub type FORMATION = _formation;
pub const NO_SOUND: C2RustUnnamed_2 = -1;
pub const ID_SOUND_ASSIGNED_TO_COUNTER_RADAR: C2RustUnnamed_2 = 82;
pub const ID_SOUND_ASSIGNED_TO_SENSOR: C2RustUnnamed_2 = 80;
pub const ID_SOUND_ASSIGNED_TO_COMMANDER: C2RustUnnamed_2 = 112;
pub const DSS_HALT_HOLD: _secondary_state = 64;
// number of units using the formation
// maximum length of the lines
// seperation between the ranks
// direction of the formation
// position of the front of the formation
// the lines that make up a formation
// the units that have a position allocated in the formation
// formation speed (currently speed of slowest member) - GJ - sorry.
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
pub type DROID_GROUP = _droid_group;
pub const GT_COMMAND: _group_type = 1;
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
// list of droids in the group
// the command droid of a command group
// where the group should retreat to
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
pub const CALL_START_NEXT_LEVEL: _scr_callback_types = 17;
pub const CALL_VTOL_OFF_MAP: _scr_callback_types = 54;
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
pub const ID_SOUND_ON_OUR_WAY2: C2RustUnnamed_2 = 219;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub order: UDWORD,
    pub mask: UDWORD,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub state: UDWORD,
    pub num: UDWORD,
}
pub const ID_SOUND_LAS_SAT_COUNTDOWN: C2RustUnnamed_2 = 353;
pub type _group_type = libc::c_uint;
pub const GT_TRANSPORTER: _group_type = 2;
pub const GT_NORMAL: _group_type = 0;
pub type _scr_callback_types = libc::c_uint;
pub const CALL_BEACON: _scr_callback_types = 62;
pub const CALL_TRANSPORTER_LANDED_B: _scr_callback_types = 61;
pub const CALL_STRUCTBUILT: _scr_callback_types = 60;
pub const CALL_AI_MSG: _scr_callback_types = 59;
pub const CALL_CONSOLE: _scr_callback_types = 58;
pub const CALL_ALLIANCEOFFER: _scr_callback_types = 57;
pub const CALL_PLAYERLEFT: _scr_callback_types = 56;
pub const CALL_UNITTAKEOVER: _scr_callback_types = 55;
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
pub const ID_SOUND_SENSOR_LOCKED_ON: C2RustUnnamed_2 = 81;
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
// Trigger fires when the script is first run
// Trigger uses script code
// Trigger after a time pause
// Trigger at repeated intervals
// Event has paused for an interval and will restart in the middle of it's code
// The user defined callback triggers should start with this id
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
// retreat positions for the players
#[no_mangle]
pub static mut asRunData: [RUN_DATA; 8] =
    [RUN_DATA{sPos: POINT{x: 0, y: 0,},
              forceLevel: 0,
              healthLevel: 0,
              leadership: 0,}; 8];
//call this *AFTER* every mission so it gets reset
// ////////////////////////////////////////////////////////////////
// ////////////////////////////////////////////////////////////////
//call this *AFTER* every mission so it gets reset
#[no_mangle]
pub unsafe extern "C" fn initRunData() {
    let mut i: UBYTE = 0;
    i = 0 as libc::c_int as UBYTE;
    while (i as libc::c_int) < 8 as libc::c_int {
        memset(&mut *asRunData.as_mut_ptr().offset(i as isize) as
                   *mut RUN_DATA as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<RUN_DATA>() as libc::c_ulong);
        i = i.wrapping_add(1)
    };
}
// check whether a droid has to move back to the thing it is guarding
#[no_mangle]
pub unsafe extern "C" fn orderCheckGuardPosition(mut psDroid: *mut DROID,
                                                 mut range: SDWORD) {
    let mut xdiff: SDWORD = 0;
    let mut ydiff: SDWORD = 0;
    let mut x: UDWORD = 0;
    let mut y: UDWORD = 0;
    if !(*psDroid).psTarget.is_null() {
        //if ((psDroid->droidType != DROID_REPAIR) && // repair droids always follow behind - don't want them jumping into the line of fire
        // repair droids always follow behind - don't want them jumping into the line of fire
        if !((*psDroid).droidType as libc::c_uint ==
                 DROID_REPAIR as libc::c_int as libc::c_uint ||
                 (*psDroid).droidType as libc::c_uint ==
                     DROID_CYBORG_REPAIR as libc::c_int as libc::c_uint) &&
               (*(*psDroid).psTarget).type_0 as libc::c_uint ==
                   OBJ_DROID as libc::c_int as libc::c_uint &&
               orderStateLoc((*psDroid).psTarget as *mut DROID, DORDER_MOVE,
                             &mut x, &mut y) != 0 {
            // got a moving droid - check against where the unit is going
            (*psDroid).orderX = x as UWORD;
            (*psDroid).orderY = y as UWORD
        } else {
            (*psDroid).orderX = (*(*psDroid).psTarget).x;
            (*psDroid).orderY = (*(*psDroid).psTarget).y
        }
    }
    xdiff = (*psDroid).x as SDWORD - (*psDroid).orderX as SDWORD;
    ydiff = (*psDroid).y as SDWORD - (*psDroid).orderY as SDWORD;
    if xdiff * xdiff + ydiff * ydiff > range * range {
        turnOffMultiMsg(1 as libc::c_int);
        if (*psDroid).sMove.Status as libc::c_int != 0 as libc::c_int &&
               ((*psDroid).action == DACTION_MOVE as libc::c_int ||
                    (*psDroid).action == DACTION_MOVEFIRE as libc::c_int) {
            xdiff =
                (*psDroid).sMove.DestinationX - (*psDroid).orderX as SDWORD;
            ydiff =
                (*psDroid).sMove.DestinationY - (*psDroid).orderY as SDWORD;
            if xdiff * xdiff + ydiff * ydiff > range * range {
                actionDroidLoc(psDroid, DACTION_MOVE,
                               (*psDroid).orderX as UDWORD,
                               (*psDroid).orderY as UDWORD);
            }
        } else {
            actionDroidLoc(psDroid, DACTION_MOVE, (*psDroid).orderX as UDWORD,
                           (*psDroid).orderY as UDWORD);
        }
        turnOffMultiMsg(0 as libc::c_int);
    };
}
/*For a given repair droid, check if there are any damaged droids within
a defined range*/
#[no_mangle]
pub unsafe extern "C" fn checkForRepairRange(mut psDroid: *mut DROID,
                                             mut psTarget: *mut DROID)
 -> *mut BASE_OBJECT {
    let mut psCurr: *mut DROID = 0 as *mut DROID;
    let mut xdiff: SDWORD = 0;
    let mut ydiff: SDWORD = 0;
    //UDWORD		sRange;
//	UDWORD		sensorDist;
    if (*psDroid).droidType as libc::c_uint ==
           DROID_REPAIR as libc::c_int as libc::c_uint ||
           (*psDroid).droidType as libc::c_uint ==
               DROID_CYBORG_REPAIR as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"checkForRepairRange:Invalid droid type\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (*psDroid).droidType as libc::c_uint ==
           DROID_REPAIR as libc::c_int as libc::c_uint ||
           (*psDroid).droidType as libc::c_uint ==
               DROID_CYBORG_REPAIR as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"order.c\x00" as *const u8 as *const libc::c_char,
              157 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 20],
                                        &[libc::c_char; 20]>(b"checkForRepairRange\x00")).as_ptr(),
              b"psDroid->droidType == DROID_REPAIR OR psDroid->droidType == DROID_CYBORG_REPAIR\x00"
                  as *const u8 as *const libc::c_char);
    };
    if !psTarget.is_null() && (*psTarget).died != 0 {
        psTarget = 0 as *mut DROID
    }
    // if guarding a unit - always check that first
    if orderStateObj(psDroid, DORDER_GUARD,
                     &mut psCurr as *mut *mut DROID as *mut *mut BASE_OBJECT)
           != 0 && !psCurr.is_null() &&
           (*psCurr).type_0 as libc::c_uint ==
               OBJ_DROID as libc::c_int as libc::c_uint &&
           droidIsDamaged(psCurr) != 0 {
        return psCurr as *mut BASE_OBJECT
    }
    /* Used to be a define. Now uses their sensor range */
	//sRange = asSensorStats[psDroid->asBits[COMP_SENSOR].nStat].range;
	//sensorDist = (sRange * sRange);
	//back to the define!
//	sensorDist = REPAIR_DIST * REPAIR_DIST;
    if !psTarget.is_null() &&
           (*psTarget).type_0 as libc::c_uint ==
               OBJ_DROID as libc::c_int as libc::c_uint &&
           (*psTarget).player as libc::c_int ==
               (*psDroid).player as libc::c_int {
        psCurr = (*psTarget).psNext
    } else { psCurr = apsDroidLists[(*psDroid).player as usize] }
    while !psCurr.is_null() {
        //check for damage
        if droidIsDamaged(psCurr) != 0 &&
               visibleObject(psDroid as *mut BASE_OBJECT,
                             psCurr as *mut BASE_OBJECT) != 0 {
            //check for within range
            xdiff = (*psDroid).x as SDWORD - (*psCurr).x as SDWORD;
            ydiff = (*psDroid).y as SDWORD - (*psCurr).y as SDWORD;
            if xdiff * xdiff + ydiff * ydiff <
                   128 as libc::c_int * 5 as libc::c_int *
                       (128 as libc::c_int * 5 as libc::c_int) {
                return psCurr as *mut BASE_OBJECT
            }
        }
        psCurr = (*psCurr).psNext
    }
    return 0 as *mut BASE_OBJECT;
}
/*For a given constructor droid, check if there are any damaged buildings within 
a defined range*/
/*For a given constructor droid, check if there are any damaged buildings within
a defined range*/
#[no_mangle]
pub unsafe extern "C" fn checkForDamagedStruct(mut psDroid: *mut DROID,
                                               mut psTarget: *mut STRUCTURE)
 -> *mut BASE_OBJECT {
    let mut psCurr: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut xdiff: SDWORD = 0;
    let mut ydiff: SDWORD = 0;
    //ASSERT( psDroid->droidType == DROID_CONSTRUCT,
    if (*psDroid).droidType as libc::c_uint ==
           DROID_CONSTRUCT as libc::c_int as libc::c_uint ||
           (*psDroid).droidType as libc::c_uint ==
               DROID_CYBORG_CONSTRUCT as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"checkForDamagedStruct: Invalid unit type\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (*psDroid).droidType as libc::c_uint ==
           DROID_CONSTRUCT as libc::c_int as libc::c_uint ||
           (*psDroid).droidType as libc::c_uint ==
               DROID_CYBORG_CONSTRUCT as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"order.c\x00" as *const u8 as *const libc::c_char,
              214 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 22],
                                        &[libc::c_char; 22]>(b"checkForDamagedStruct\x00")).as_ptr(),
              b"psDroid->droidType == DROID_CONSTRUCT || psDroid->droidType == DROID_CYBORG_CONSTRUCT\x00"
                  as *const u8 as *const libc::c_char);
    };
    if !psTarget.is_null() && (*psTarget).died != 0 {
        psTarget = 0 as *mut STRUCTURE
    }
    if !psTarget.is_null() &&
           (*psTarget).type_0 as libc::c_uint ==
               OBJ_STRUCTURE as libc::c_int as libc::c_uint &&
           (*psTarget).player as libc::c_int ==
               (*psDroid).player as libc::c_int {
        psCurr = (*psTarget).psNext
    } else { psCurr = apsStructLists[(*psDroid).player as usize] }
    while !psCurr.is_null() {
        //check for damage
        if structIsDamaged(psCurr) != 0 &&
               visibleObject(psDroid as *mut BASE_OBJECT,
                             psCurr as *mut BASE_OBJECT) != 0 {
            //check for within range
            xdiff = (*psDroid).x as SDWORD - (*psCurr).x as SDWORD;
            ydiff = (*psDroid).y as SDWORD - (*psCurr).y as SDWORD;
            //check for repair distance and not construct_dist - this allows for structures being up to 3 tiles across
            if xdiff * xdiff + ydiff * ydiff <
                   128 as libc::c_int * 5 as libc::c_int *
                       (128 as libc::c_int * 5 as libc::c_int) {
                return psCurr as *mut BASE_OBJECT
            }
        }
        psCurr = (*psCurr).psNext
    }
    return 0 as *mut BASE_OBJECT;
}
/* Update a droids order state */
/* Update a droids order state */
#[no_mangle]
pub unsafe extern "C" fn orderUpdateDroid(mut psDroid: *mut DROID) {
    let mut actionX: UDWORD = 0;
    let mut actionY: UDWORD = 0;
    let mut psFireTarget: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut psObj: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psWall: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psRepairFac: *mut REPAIR_FACILITY = 0 as *mut REPAIR_FACILITY;
    let mut xdiff: SDWORD = 0;
    let mut ydiff: SDWORD = 0;
    let mut temp: SDWORD = 0;
    let mut state: SECONDARY_STATE = 0 as SECONDARY_STATE;
    let mut bAttack: BOOL = 0;
    let mut psSpotter: *mut DROID = 0 as *mut DROID;
    // clear the target if it has died
    if !(*psDroid).psTarget.is_null() {
        if (*(*psDroid).psTarget).died != 0 {
            (*psDroid).psTarget = 0 as *mut _base_object
        }
    }
    //clear its base struct if its died
    if !(*psDroid).psBaseStruct.is_null() {
        if (*(*psDroid).psBaseStruct).died != 0 {
            (*psDroid).psBaseStruct = 0 as *mut _structure
        }
    }
    // check for died objects in the list
    orderCheckList(psDroid);
    let mut current_block_422: u64;
    match (*psDroid).order {
        0 => {
            psObj = 0 as *mut BASE_OBJECT;
            // see if there are any orders queued up
            if !(orderDroidList(psDroid) != 0) {
                // if you are in a command group, default to guarding the commander
                if (*psDroid).droidType as libc::c_uint !=
                       DROID_COMMAND as libc::c_int as libc::c_uint &&
                       !(*psDroid).psGroup.is_null() &&
                       (*(*psDroid).psGroup).type_0 as libc::c_int ==
                           GT_COMMAND as libc::c_int &&
                       (*psDroid).psTarStats !=
                           structGetDemolishStat() as *mut BASE_STATS {
                    // stop the constructor auto repairing when it is about to demolish
                    orderDroidObj(psDroid, DORDER_GUARD,
                                  (*(*psDroid).psGroup).psCommander as
                                      *mut BASE_OBJECT);
                } else if (*psDroid).droidType as libc::c_uint ==
                              DROID_TRANSPORTER as libc::c_int as libc::c_uint
                 {
                    //check transporter isn't sitting there waiting to be filled when nothing exists!
                    if (*psDroid).player as libc::c_uint == selectedPlayer &&
                           getDroidsToSafetyFlag() != 0 &&
                           missionDroidsRemaining(selectedPlayer) == 0 {
                        //check that nothing is on the transporter (transporter counts as first in group)
                        if !(*psDroid).psGroup.is_null() &&
                               ((*(*psDroid).psGroup).refCount as libc::c_int)
                                   < 2 as libc::c_int {
                            //the script can call startMission for this callback for offworld missions
                            eventFireCallbackTrigger(CALL_START_NEXT_LEVEL as
                                                         libc::c_int as
                                                         TRIGGER_TYPE);
                        }
                    }
                } else if (*psDroid).player as libc::c_uint == selectedPlayer
                              &&
                              (*psDroid).psTarStats !=
                                  structGetDemolishStat() as *mut BASE_STATS
                              &&
                              secondaryGetState(psDroid, DSO_HALTTYPE,
                                                &mut state) != 0 &&
                              vtolDroid(psDroid) == 0 {
                    if state as libc::c_uint ==
                           DSS_HALT_GUARD as libc::c_int as libc::c_uint {
                        actionX = (*psDroid).x as UDWORD;
                        actionY = (*psDroid).y as UDWORD;
                        turnOffMultiMsg(1 as libc::c_int);
                        orderDroidLoc(psDroid, DORDER_GUARD, actionX,
                                      actionY);
                        turnOffMultiMsg(0 as libc::c_int);
                    }
                } else if ((*psDroid).droidType as libc::c_uint ==
                               DROID_REPAIR as libc::c_int as libc::c_uint ||
                               (*psDroid).droidType as libc::c_uint ==
                                   DROID_CYBORG_REPAIR as libc::c_int as
                                       libc::c_uint) &&
                              orderState(psDroid, DORDER_GUARD) == 0 {
                    psObj = checkForRepairRange(psDroid, 0 as *mut DROID);
                    if !psObj.is_null() {
                        if bMultiPlayer == 0 ||
                               myResponsibility((*psDroid).player as UDWORD)
                                   != 0 {
                            orderDroidObj(psDroid, DORDER_DROIDREPAIR, psObj);
                        }
                    }
                } else if ((*psDroid).droidType as libc::c_uint ==
                               DROID_CONSTRUCT as libc::c_int as libc::c_uint
                               ||
                               (*psDroid).droidType as libc::c_uint ==
                                   DROID_CYBORG_CONSTRUCT as libc::c_int as
                                       libc::c_uint) &&
                              orderState(psDroid, DORDER_GUARD) == 0 &&
                              (*psDroid).psTarStats !=
                                  structGetDemolishStat() as *mut BASE_STATS {
                    psObj =
                        checkForDamagedStruct(psDroid, 0 as *mut STRUCTURE);
                    if !psObj.is_null() {
                        if bMultiPlayer == 0 ||
                               myResponsibility((*psDroid).player as UDWORD)
                                   != 0 {
                            orderDroidObj(psDroid, DORDER_REPAIR, psObj);
                        }
                    }
                }
            }
        }
        24 => {
            if (*psDroid).action == DACTION_NONE as libc::c_int {
                missionMoveTransporterOffWorld(psDroid);
                // default to guarding if the correct secondary order is set
		// only do this for the human player in single player mode
                //repair droids default to repairing droids within a given range
                //constructor droids default to repairing structures within a given range
		//else if ((psDroid->droidType == DROID_CONSTRUCT) &&
                /* clear order */
                (*psDroid).order = DORDER_NONE as libc::c_int;
                (*psDroid).psTarget = 0 as *mut _base_object;
                (*psDroid).psTarStats = 0 as *mut _base_stats
            }
        }
        22 => {
            if (*psDroid).action == DACTION_NONE as libc::c_int {
                //if moving droids to safety and still got some droids left don't do callback
                if (*psDroid).player as libc::c_uint == selectedPlayer &&
                       getDroidsToSafetyFlag() != 0 &&
                       missionDroidsRemaining(selectedPlayer) != 0 {
                    //move droids in Transporter into holding list
                    moveDroidsToSafety(psDroid);
                    //don't do this until waited for the required time
                //fly Transporter back to get some more droids
	    		//orderDroidLoc( psDroid, DORDER_TRANSPORTIN,
                //    getLandingX(selectedPlayer), getLandingY(selectedPlayer));
                    orderDroid(psDroid, DORDER_TRANSPORTIN);
                    actionDroid(psDroid, DACTION_TRANSPORTWAITTOFLYIN);
                    missionSetReinforcementTime(gameTime);
                } else {
                    //we need the transporter to just sit off world for a while...
                    /* set action transporter waits for timer */
                    //the script can call startMission for this callback for offworld missions
                    eventFireCallbackTrigger(CALL_START_NEXT_LEVEL as
                                                 libc::c_int as TRIGGER_TYPE);
                    /* clear order */
                    (*psDroid).order = DORDER_NONE as libc::c_int;
                    (*psDroid).psTarget = 0 as *mut _base_object;
                    (*psDroid).psTarStats = 0 as *mut _base_stats
                }
            }
        }
        23 => {
            if (*psDroid).action == DACTION_NONE as libc::c_int &&
                   (*psDroid).sMove.Status as libc::c_int == 0 as libc::c_int
               {
                /* clear order */
                (*psDroid).order = DORDER_NONE as libc::c_int;
                (*psDroid).psTarget = 0 as *mut _base_object;
                (*psDroid).psTarStats = 0 as *mut _base_stats;
                //FFS! You only wan't to do this if the droid being tracked IS the transporter! Not all the time!
// What about if your happily playing the game and tracking a droid, and re-enforcements come in!
// And suddenly BLAM!!!! It drops you out of camera mode for no apparent reason! TOTALY CONFUSING
// THE PLAYER!
//
// Just had to get that off my chest....end of rant.....
//
                if psDroid == getTrackingDroid() {
                    // Thats better...
                    /* deselect transporter if have been tracking */
                    if getWarCamStatus() != 0 { camToggleStatus(); }
                }
                DeSelectDroid(psDroid);
                /*don't try the unload if moving droids to safety and still got some
            droids left  - wait until full and then launch again*/
                if (*psDroid).player as libc::c_uint == selectedPlayer &&
                       getDroidsToSafetyFlag() != 0 &&
                       missionDroidsRemaining(selectedPlayer) != 0 {
                    resetTransporter(psDroid);
                } else {
                    unloadTransporter(psDroid, (*psDroid).x as UDWORD,
                                      (*psDroid).y as UDWORD,
                                      0 as libc::c_int);
                }
            }
        }
        2 | 11 | 12 => {
            // Just wait for the action to finish then clear the order
            if (*psDroid).action == DACTION_NONE as libc::c_int {
                (*psDroid).order = DORDER_NONE as libc::c_int;
                (*psDroid).psTarget = 0 as *mut _base_object;
                (*psDroid).psTarStats = 0 as *mut _base_stats
            }
        }
        35 => {
            if (*psDroid).psTarget.is_null() {
                (*psDroid).order = DORDER_NONE as libc::c_int
            } else if (*psDroid).action == DACTION_NONE as libc::c_int {
                // stoped moving, but still havn't got the artifact
                actionDroidLoc(psDroid, DACTION_MOVE,
                               (*(*psDroid).psTarget).x as UDWORD,
                               (*(*psDroid).psTarget).y as UDWORD);
            }
        }
        33 | 34 => {
            if (*psDroid).psTarget.is_null() {
                if (*psDroid).order == DORDER_MOVE_ATTACKWALL as libc::c_int {
                    (*psDroid).order = DORDER_MOVE as libc::c_int
                } else { (*psDroid).order = DORDER_SCOUT as libc::c_int }
                actionDroidLoc(psDroid, DACTION_MOVE,
                               (*psDroid).orderX as UDWORD,
                               (*psDroid).orderY as UDWORD);
            } else if ((*psDroid).action != DACTION_ATTACK as libc::c_int &&
                           (*psDroid).action !=
                               DACTION_MOVETOATTACK as libc::c_int &&
                           (*psDroid).action !=
                               DACTION_ROTATETOATTACK as libc::c_int ||
                           (*psDroid).psActionTarget != (*psDroid).psTarget)
                          && actionInRange(psDroid, (*psDroid).psTarget) != 0
             {
                actionDroidObj(psDroid, DACTION_ATTACK, (*psDroid).psTarget);
            } else if (*psDroid).action == DACTION_NONE as libc::c_int {
                if (*psDroid).order == DORDER_SCOUT_ATTACKWALL as libc::c_int
                   {
                    (*psDroid).order = DORDER_SCOUT as libc::c_int
                }
                actionDroidLoc(psDroid, DACTION_MOVE,
                               (*(*psDroid).psTarget).x as UDWORD,
                               (*(*psDroid).psTarget).y as UDWORD);
            }
        }
        28 | 31 => {
            // if there is an enemy around, attack it
            if (*psDroid).action == DACTION_MOVE as libc::c_int &&
                   (secondaryGetState(psDroid, DSO_ATTACK_LEVEL, &mut state)
                        != 0 &&
                        state as libc::c_uint ==
                            DSS_ALEV_ALWAYS as libc::c_int as libc::c_uint) &&
                   aiNearestTarget(psDroid, &mut psObj) != 0 {
                match (*psDroid).droidType as libc::c_uint {
                    0 | 5 | 12 | 4 | 7 => {
                        actionDroidObj(psDroid, DACTION_ATTACK, psObj);
                    }
                    1 => { actionDroidObj(psDroid, DACTION_OBSERVE, psObj); }
                    _ => { actionDroid(psDroid, DACTION_NONE); }
                }
            } else if (*psDroid).action == DACTION_NONE as libc::c_int {
                xdiff = (*psDroid).x as SDWORD - (*psDroid).orderX as SDWORD;
                ydiff = (*psDroid).y as SDWORD - (*psDroid).orderY as SDWORD;
                if xdiff * xdiff + ydiff * ydiff <
                       128 as libc::c_int * 8 as libc::c_int *
                           (128 as libc::c_int * 8 as libc::c_int) {
                    if (*psDroid).order == DORDER_PATROL as libc::c_int {
                        // head back to the other point
                        temp = (*psDroid).orderX as SDWORD;
                        (*psDroid).orderX = (*psDroid).orderX2;
                        (*psDroid).orderX2 = temp as UWORD;
                        temp = (*psDroid).orderY as SDWORD;
                        (*psDroid).orderY = (*psDroid).orderY2;
                        (*psDroid).orderY2 = temp as UWORD;
                        actionDroidLoc(psDroid, DACTION_MOVE,
                                       (*psDroid).orderX as UDWORD,
                                       (*psDroid).orderY as UDWORD);
                    } else { (*psDroid).order = DORDER_NONE as libc::c_int }
                } else {
                    actionDroidLoc(psDroid, DACTION_MOVE,
                                   (*psDroid).orderX as UDWORD,
                                   (*psDroid).orderY as UDWORD);
                }
            } else if (*psDroid).action == DACTION_ATTACK as libc::c_int ||
                          (*psDroid).action ==
                              DACTION_MOVETOATTACK as libc::c_int ||
                          (*psDroid).action ==
                              DACTION_ROTATETOATTACK as libc::c_int ||
                          (*psDroid).action == DACTION_OBSERVE as libc::c_int
                          ||
                          (*psDroid).action ==
                              DACTION_MOVETOOBSERVE as libc::c_int {
                // attacking something - see if the droid has gone too far
                xdiff = (*psDroid).x as SDWORD - (*psDroid).actionX as SDWORD;
                ydiff = (*psDroid).y as SDWORD - (*psDroid).actionY as SDWORD;
                if xdiff * xdiff + ydiff * ydiff >
                       128 as libc::c_int * 5 as libc::c_int *
                           (128 as libc::c_int * 5 as libc::c_int) {
                    actionDroidLoc(psDroid, DACTION_RETURNTOPOS,
                                   (*psDroid).actionX, (*psDroid).actionY);
                }
            }
        }
        5 | 7 | 9 | 8 | 26 | 27 | 30 => {
            if (*psDroid).action == DACTION_NONE as libc::c_int ||
                   (*psDroid).psTarget.is_null() {
                (*psDroid).order = DORDER_NONE as libc::c_int;
                actionDroid(psDroid, DACTION_NONE);
                if (*psDroid).player as libc::c_uint == selectedPlayer {
                    intRefreshScreen();
                }
            }
        }
        32 => {
            if (*psDroid).psTarget.is_null() ||
                   (*psDroid).psActionTarget.is_null() {
                // arm pad destroyed find another
                (*psDroid).order = DORDER_NONE as libc::c_int;
                moveToRearm(psDroid);
            } else if (*psDroid).action == DACTION_NONE as libc::c_int {
                (*psDroid).order = DORDER_NONE as libc::c_int
            }
        }
        3 | 18 => {
            if (*psDroid).psTarget.is_null() {
                // ||
                //			(secondaryGetState(psDroid, DSO_ATTACK_LEVEL, &state) && (state != DSS_ALEV_ALWAYS)))
                // if vtol then return to rearm pad as long as there are no other
			// orders queued up
                if vtolDroid(psDroid) != 0 {
                    if orderDroidList(psDroid) == 0 {
                        (*psDroid).order = DORDER_NONE as libc::c_int;
                        moveToRearm(psDroid);
                    }
                } else {
                    (*psDroid).order = DORDER_NONE as libc::c_int;
                    actionDroid(psDroid, DACTION_NONE);
                }
            } else if ((*psDroid).action == DACTION_MOVE as libc::c_int ||
                           (*psDroid).action ==
                               DACTION_MOVEFIRE as libc::c_int) &&
                          actionVisibleTarget(psDroid, (*psDroid).psTarget) !=
                              0 && vtolDroid(psDroid) == 0 {
                // moved near enough to attack change to attack action
                actionDroidObj(psDroid, DACTION_ATTACK, (*psDroid).psTarget);
            } else if (*psDroid).action == DACTION_MOVETOATTACK as libc::c_int
                          && vtolDroid(psDroid) == 0 &&
                          actionVisibleTarget(psDroid, (*psDroid).psTarget) ==
                              0 {
                // lost sight of the target while chasing it - change to a move action so
			// that the unit will fire on other things while moving
                actionDroidLoc(psDroid, DACTION_MOVE,
                               (*(*psDroid).psTarget).x as UDWORD,
                               (*(*psDroid).psTarget).y as UDWORD);
            } else if vtolDroid(psDroid) == 0 &&
                          (*psDroid).psTarget == (*psDroid).psActionTarget &&
                          actionInRange(psDroid, (*psDroid).psTarget) != 0 &&
                          visGetBlockingWall(psDroid as *mut BASE_OBJECT,
                                             (*psDroid).psTarget, &mut psWall)
                              != 0 &&
                          (*psWall).player as libc::c_int !=
                              (*psDroid).player as libc::c_int {
                // there is a wall in the way - attack that
                actionDroidObj(psDroid, DACTION_ATTACK,
                               psWall as *mut BASE_OBJECT);
            } else if (*psDroid).action == DACTION_NONE as libc::c_int ||
                          (*psDroid).action ==
                              DACTION_CLEARREARMPAD as libc::c_int {
                if (*psDroid).order == DORDER_ATTACKTARGET as libc::c_int &&
                       secondaryGetState(psDroid, DSO_HALTTYPE, &mut state) !=
                           0 &&
                       state as libc::c_uint ==
                           DSS_HALT_HOLD as libc::c_int as libc::c_uint &&
                       actionInRange(psDroid, (*psDroid).psTarget) == 0 {
                    // on hold orders give up
                    (*psDroid).order = DORDER_NONE as libc::c_int;
                    (*psDroid).psTarget = 0 as *mut _base_object
                } else if vtolDroid(psDroid) == 0 ||
                              allVtolsRearmed(psDroid) != 0 {
                    actionDroidObj(psDroid, DACTION_ATTACK,
                                   (*psDroid).psTarget);
                }
            }
        }
        4 => {
            if (*psDroid).action == DACTION_BUILD as libc::c_int &&
                   (*psDroid).psTarget.is_null() {
                (*psDroid).order = DORDER_NONE as libc::c_int;
                actionDroid(psDroid, DACTION_NONE);
            } else if (*psDroid).action == DACTION_NONE as libc::c_int {
                (*psDroid).order = DORDER_NONE as libc::c_int
            }
        }
        16 => {
            //only place it can be trapped - in multiPlayer can only put cyborgs onto a Transporter
            if bMultiPlayer != 0 && cyborgDroid(psDroid) == 0 {
                (*psDroid).order = DORDER_NONE as libc::c_int;
                actionDroid(psDroid, DACTION_NONE);
            } else {
                // don't want the droids to go into a formation for this order
                if !(*psDroid).sMove.psFormation.is_null() {
                    formationLeave((*psDroid).sMove.psFormation,
                                   psDroid as *mut BASE_OBJECT);
                    (*psDroid).sMove.psFormation = 0 as *mut _formation
                }
                // Wait for the action to finish then assign to Transporter (if not already flying)
                if (*psDroid).psTarget.is_null() ||
                       transporterFlying((*psDroid).psTarget as *mut DROID) !=
                           0 {
                    (*psDroid).order = DORDER_NONE as libc::c_int;
                    actionDroid(psDroid, DACTION_NONE);
                } else if abs((*psDroid).x as SDWORD -
                                  (*(*psDroid).psTarget).x as SDWORD) <
                              128 as libc::c_int &&
                              abs((*psDroid).y as SDWORD -
                                      (*(*psDroid).psTarget).y as SDWORD) <
                                  128 as libc::c_int {
                    //if in multiPlayer, only want to process if this player's droid
                    if bMultiPlayer == 0 ||
                           (*psDroid).player as libc::c_uint == selectedPlayer
                       {
                        //psDroid->order = DORDER_NONE;
                        transporterAddDroid((*psDroid).psTarget as *mut DROID,
                                            psDroid);
                        //order the droid to stop so moveUpdateDroid does not process this unit
                        orderDroid(psDroid, DORDER_STOP);
                        (*psDroid).psTarget = 0 as *mut _base_object;
                        (*psDroid).psTarStats = 0 as *mut _base_stats;
                        secondarySetState(psDroid, DSO_RETURN_TO_LOC,
                                          0 as SECONDARY_STATE);
                    }
                } else if (*psDroid).action == DACTION_NONE as libc::c_int {
                    actionDroidLoc(psDroid, DACTION_MOVE,
                                   (*(*psDroid).psTarget).x as UDWORD,
                                   (*(*psDroid).psTarget).y as UDWORD);
                }
            }
        }
        17 => {
            //only valid in multiPlayer mode
            if bMultiPlayer != 0 {
                //this order can only be given to Transporter droids
                if (*psDroid).droidType as libc::c_uint ==
                       DROID_TRANSPORTER as libc::c_int as libc::c_uint {
                    /*once the Transporter has reached its destination (and landed),
                get all the units to disembark*/
                    if (*psDroid).action == DACTION_NONE as libc::c_int &&
                           (*psDroid).sMove.Status as libc::c_int ==
                               0 as libc::c_int &&
                           (*psDroid).sMove.iVertSpeed as libc::c_int ==
                               0 as libc::c_int {
                        //only need to unload if this player's droid
                        if (*psDroid).player as libc::c_uint == selectedPlayer
                           {
                            unloadTransporter(psDroid, (*psDroid).x as UDWORD,
                                              (*psDroid).y as UDWORD,
                                              0 as libc::c_int);
                        }
                        //reset the transporter's order
                        (*psDroid).order = DORDER_NONE as libc::c_int
                    }
                }
            }
        }
        13 => {
            // Just wait for the action to finish then clear the order
            if (*psDroid).action == DACTION_NONE as libc::c_int {
                (*psDroid).order = DORDER_NONE as libc::c_int;
                secondarySetState(psDroid, DSO_RETURN_TO_LOC,
                                  0 as SECONDARY_STATE);
            }
        }
        36 => {
            if ((*psDroid).x as libc::c_int) <
                   128 as libc::c_int * 2 as libc::c_int ||
                   (*psDroid).x as libc::c_uint >
                       mapWidth.wrapping_sub(2 as libc::c_int as
                                                 libc::c_uint).wrapping_mul(128
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_uint)
                   ||
                   ((*psDroid).y as libc::c_int) <
                       128 as libc::c_int * 2 as libc::c_int ||
                   (*psDroid).y as libc::c_uint >
                       mapHeight.wrapping_sub(2 as libc::c_int as
                                                  libc::c_uint).wrapping_mul(128
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_uint)
                   || (*psDroid).action == DACTION_NONE as libc::c_int {
                (*psDroid).order = DORDER_NONE as libc::c_int;
                psScrCBVtolOffMap = psDroid;
                eventFireCallbackTrigger(CALL_VTOL_OFF_MAP as libc::c_int as
                                             TRIGGER_TYPE);
            }
        }
        14 | 37 => {
            if (*psDroid).psTarget.is_null() {
                (*psDroid).order = DORDER_NONE as libc::c_int;
                actionDroid(psDroid, DACTION_NONE);
            } else if (*psDroid).action == DACTION_NONE as libc::c_int {
                /* get repair facility pointer */
                psStruct = (*psDroid).psTarget as *mut STRUCTURE;
                if 1 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"orderUpdateUnit: invalid structure pointer\x00" as
                              *const u8 as *const libc::c_char);
                };
                if 1 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"order.c\x00" as *const u8 as *const libc::c_char,
                          795 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 17],
                                                    &[libc::c_char; 17]>(b"orderUpdateDroid\x00")).as_ptr(),
                          b"PTRVALID(psStruct, sizeof(STRUCTURE))\x00" as
                              *const u8 as *const libc::c_char);
                };
                psRepairFac =
                    (*psStruct).pFunctionality as *mut REPAIR_FACILITY;
                if 1 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"orderUpdateUnit: invalid repair facility pointer\x00"
                              as *const u8 as *const libc::c_char);
                };
                if 1 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"order.c\x00" as *const u8 as *const libc::c_char,
                          798 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 17],
                                                    &[libc::c_char; 17]>(b"orderUpdateDroid\x00")).as_ptr(),
                          b"PTRVALID(psRepairFac, sizeof(REPAIR_FACILITY))\x00"
                              as *const u8 as *const libc::c_char);
                };
                xdiff =
                    (*psDroid).x as SDWORD -
                        (*(*psDroid).psTarget).x as SDWORD;
                ydiff =
                    (*psDroid).y as SDWORD -
                        (*(*psDroid).psTarget).y as SDWORD;
                if xdiff * xdiff + ydiff * ydiff <
                       128 as libc::c_int * 8 as libc::c_int *
                           (128 as libc::c_int * 8 as libc::c_int) {
                    /* action droid to wait */
                    actionDroid(psDroid, DACTION_WAITFORREPAIR);
                } else {
                    // move the droid closer to the repair facility
                    actionDroidLoc(psDroid, DACTION_MOVE,
                                   (*(*psDroid).psTarget).x as UDWORD,
                                   (*(*psDroid).psTarget).y as UDWORD);
                }
            } else if (*psDroid).order == DORDER_RTR as libc::c_int &&
                          ((*psDroid).action == DACTION_MOVE as libc::c_int ||
                               (*psDroid).action ==
                                   DACTION_MOVEFIRE as libc::c_int) &&
                          (*psDroid).id.wrapping_rem(50 as libc::c_int as
                                                         libc::c_uint) ==
                              frameGetFrameNumber().wrapping_rem(50 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint)
             {
                // see if there is a repair facility nearer than the one currently selected
                orderDroid(psDroid, DORDER_RTR);
            }
        }
        29 => {
            if (*psDroid).actionStarted.wrapping_add(10000 as libc::c_int as
                                                         libc::c_uint) <
                   gameTime {
                destroyDroid(psDroid);
            }
        }
        15 => {
            if (*psDroid).action == DACTION_NONE as libc::c_int {
                // got there so stop running
                (*psDroid).order = DORDER_NONE as libc::c_int
            }
            if (*psDroid).actionStarted.wrapping_add(8000 as libc::c_int as
                                                         libc::c_uint) <
                   gameTime {
                // been running long enough
                actionDroid(psDroid, DACTION_NONE);
                (*psDroid).order = DORDER_NONE as libc::c_int
            }
        }
        6 => {
            if (*psDroid).action == DACTION_NONE as libc::c_int ||
                   (*psDroid).action == DACTION_BUILD as libc::c_int &&
                       (*psDroid).psTarget.is_null() {
                // finished building the current structure
                if (*psDroid).orderX as libc::c_int >> 7 as libc::c_int ==
                       (*psDroid).orderX2 as libc::c_int >> 7 as libc::c_int
                       &&
                       (*psDroid).orderY as libc::c_int >> 7 as libc::c_int ==
                           (*psDroid).orderY2 as libc::c_int >>
                               7 as libc::c_int {
                    // finished all the structures - done
                    (*psDroid).order = DORDER_NONE as libc::c_int;
                    (*psDroid).psTarget = 0 as *mut _base_object;
                    (*psDroid).psTarStats = 0 as *mut _base_stats
                } else {
                    // update the position for another structure
                    if (*psDroid).orderX as libc::c_int >> 7 as libc::c_int ==
                           (*psDroid).orderX2 as libc::c_int >>
                               7 as libc::c_int {
                        // still got building to do - working vertically
                        if ((*psDroid).orderY as libc::c_int) <
                               (*psDroid).orderY2 as libc::c_int {
                            (*psDroid).orderY =
                                ((*psDroid).orderY as libc::c_int +
                                     128 as libc::c_int) as UWORD
                        } else {
                            (*psDroid).orderY =
                                ((*psDroid).orderY as libc::c_int -
                                     128 as libc::c_int) as UWORD
                        }
                        current_block_422 = 15932507887445511105;
                    } else if (*psDroid).orderY as libc::c_int >>
                                  7 as libc::c_int ==
                                  (*psDroid).orderY2 as libc::c_int >>
                                      7 as libc::c_int {
                        // still got building to do - working horizontally
                        if ((*psDroid).orderX as libc::c_int) <
                               (*psDroid).orderX2 as libc::c_int {
                            (*psDroid).orderX =
                                ((*psDroid).orderX as libc::c_int +
                                     128 as libc::c_int) as UWORD
                        } else {
                            (*psDroid).orderX =
                                ((*psDroid).orderX as libc::c_int -
                                     128 as libc::c_int) as UWORD
                        }
                        current_block_422 = 15932507887445511105;
                    } else {
                        if 0 as libc::c_int != 0 {
                        } else {
                            debug(LOG_ERROR,
                                  b"orderUpdateUnit: LINEBUILD order on diagonal line\x00"
                                      as *const u8 as *const libc::c_char);
                        };
                        if 0 as libc::c_int != 0 {
                        } else {
                            debug(LOG_ERROR,
                                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                                      *const u8 as *const libc::c_char,
                                  b"order.c\x00" as *const u8 as
                                      *const libc::c_char, 883 as libc::c_int,
                                  (*::std::mem::transmute::<&[u8; 17],
                                                            &[libc::c_char; 17]>(b"orderUpdateDroid\x00")).as_ptr(),
                                  b"FALSE\x00" as *const u8 as
                                      *const libc::c_char);
                        };
                        current_block_422 = 2968497183096422187;
                    }
                    match current_block_422 {
                        2968497183096422187 => { }
                        _ => {
                            // build another structure
                            (*psDroid).psTarget = 0 as *mut _base_object;
                            actionDroidLoc(psDroid, DACTION_BUILD,
                                           (*psDroid).orderX as UDWORD,
                                           (*psDroid).orderY as UDWORD);
                        }
                    }
                }
                //intRefreshScreen();
            }
        }
        10 => {
            if (*psDroid).psTarget.is_null() {
                (*psDroid).order = DORDER_NONE as libc::c_int;
                if vtolDroid(psDroid) != 0 {
                    moveToRearm(psDroid);
                } else { actionDroid(psDroid, DACTION_NONE); }
            } else if vtolEmpty(psDroid) != 0 {
                moveToRearm(psDroid);
            } else {
                //before targetting - check VTOL's are fully armed
                //indirect weapon droid attached to (standard)sensor droid
                psFireTarget = 0 as *mut BASE_OBJECT;
                if (*(*psDroid).psTarget).type_0 as libc::c_uint ==
                       OBJ_DROID as libc::c_int as libc::c_uint {
                    psSpotter = (*psDroid).psTarget as *mut DROID;
                    //				orderStateObj((DROID *)psDroid->psTarget, DORDER_OBSERVE, &psFireTarget);
                    if (*psSpotter).action == DACTION_OBSERVE as libc::c_int
                           ||
                           (*psSpotter).droidType as libc::c_uint ==
                               DROID_COMMAND as libc::c_int as libc::c_uint &&
                               (*psSpotter).action ==
                                   DACTION_ATTACK as libc::c_int {
                        psFireTarget =
                            (*((*psDroid).psTarget as
                                   *mut DROID)).psActionTarget
                    }
                } else if (*(*psDroid).psTarget).type_0 as libc::c_uint ==
                              OBJ_STRUCTURE as libc::c_int as libc::c_uint {
                    psFireTarget =
                        (*((*psDroid).psTarget as *mut STRUCTURE)).psTarget
                }
                if !psFireTarget.is_null() {
                    bAttack = 0 as libc::c_int;
                    if vtolDroid(psDroid) != 0 {
                        if vtolEmpty(psDroid) == 0 &&
                               ((*psDroid).action ==
                                    DACTION_MOVETOREARM as libc::c_int ||
                                    (*psDroid).action ==
                                        DACTION_WAITFORREARM as libc::c_int)
                               &&
                               (*psDroid).sMove.Status as libc::c_int !=
                                   0 as libc::c_int {
                            // catch vtols that were attacking another target which was destroyed
						// get them to attack the new target rather than returning to rearm
                            bAttack = 1 as libc::c_int
                        } else if allVtolsRearmed(psDroid) != 0 {
                            bAttack = 1 as libc::c_int
                        }
                    } else { bAttack = 1 as libc::c_int }
                    //if not currently attacking or target has changed
                    if bAttack != 0 &&
                           (droidAttacking(psDroid) == 0 ||
                                psFireTarget != (*psDroid).psActionTarget) {
                        //get the droid to attack
                        actionDroidObj(psDroid, DACTION_ATTACK, psFireTarget);
                    }
                } else if vtolDroid(psDroid) != 0 &&
                              (*psDroid).action != DACTION_NONE as libc::c_int
                              &&
                              (*psDroid).action !=
                                  DACTION_FIRESUPPORT as libc::c_int {
                    moveToRearm(psDroid);
                } else if (*psDroid).action !=
                              DACTION_FIRESUPPORT as libc::c_int &&
                              (*psDroid).action !=
                                  DACTION_FIRESUPPORT_RETREAT as libc::c_int {
                    actionDroidObj(psDroid, DACTION_FIRESUPPORT,
                                   (*psDroid).psTarget);
                }
            }
        }
        21 => {
            // don't bother with formations for this order
            if !(*psDroid).sMove.psFormation.is_null() {
                formationLeave((*psDroid).sMove.psFormation,
                               psDroid as *mut BASE_OBJECT);
                (*psDroid).sMove.psFormation = 0 as *mut _formation
            }
            if (*psDroid).psTarget.is_null() {
                (*psDroid).order = DORDER_NONE as libc::c_int;
                actionDroid(psDroid, DACTION_NONE);
            } else if actionReachedBuildPos(psDroid,
                                            (*(*psDroid).psTarget).x as
                                                SDWORD,
                                            (*(*psDroid).psTarget).y as
                                                SDWORD,
                                            (*((*psDroid).psTarget as
                                                   *mut STRUCTURE)).pStructureType
                                                as *mut BASE_STATS) != 0 {
                recycleDroid(psDroid);
            } else if (*psDroid).action == DACTION_NONE as libc::c_int {
                actionDroidLoc(psDroid, DACTION_MOVE,
                               (*(*psDroid).psTarget).x as UDWORD,
                               (*(*psDroid).psTarget).y as UDWORD);
            }
        }
        25 => {
            /*		if (psDroid->psTarget == NULL)
		{
			psDroid->order = DORDER_NONE;
			actionDroid(psDroid, DACTION_NONE);
		}
		else*/
            if !(orderDroidList(psDroid) != 0) {
                if (*psDroid).action == DACTION_NONE as libc::c_int ||
                       (*psDroid).action == DACTION_MOVE as libc::c_int ||
                       (*psDroid).action == DACTION_MOVEFIRE as libc::c_int {
                    // not doing anything, make sure the droid is close enough
			// to the thing it is defending
			//if ((psDroid->droidType != DROID_REPAIR) &&
                    if !((*psDroid).droidType as libc::c_uint ==
                             DROID_REPAIR as libc::c_int as libc::c_uint ||
                             (*psDroid).droidType as libc::c_uint ==
                                 DROID_CYBORG_REPAIR as libc::c_int as
                                     libc::c_uint) &&
                           !(*psDroid).psTarget.is_null() &&
                           (*(*psDroid).psTarget).type_0 as libc::c_uint ==
                               OBJ_DROID as libc::c_int as libc::c_uint &&
                           (*((*psDroid).psTarget as *mut DROID)).droidType as
                               libc::c_uint ==
                               DROID_COMMAND as libc::c_int as libc::c_uint {
                        // guarding a commander, allow more space
                        orderCheckGuardPosition(psDroid,
                                                128 as libc::c_int *
                                                    5 as libc::c_int);
                    } else {
                        orderCheckGuardPosition(psDroid,
                                                128 as libc::c_int *
                                                    3 as libc::c_int);
                    }
                } else if (*psDroid).droidType as libc::c_uint ==
                              DROID_REPAIR as libc::c_int as libc::c_uint ||
                              (*psDroid).droidType as libc::c_uint ==
                                  DROID_CYBORG_REPAIR as libc::c_int as
                                      libc::c_uint {
                    // repairing something, make sure the droid doesn't go too far
                    orderCheckGuardPosition(psDroid,
                                            128 as libc::c_int *
                                                5 as libc::c_int);
                } else if (*psDroid).droidType as libc::c_uint ==
                              DROID_CONSTRUCT as libc::c_int as libc::c_uint
                              ||
                              (*psDroid).droidType as libc::c_uint ==
                                  DROID_CYBORG_CONSTRUCT as libc::c_int as
                                      libc::c_uint {
                    // repairing something, make sure the droid doesn't go too far
                    orderCheckGuardPosition(psDroid,
                                            128 as libc::c_int *
                                                8 as libc::c_int);
                } else if (*psDroid).droidType as libc::c_uint ==
                              DROID_TRANSPORTER as libc::c_int as libc::c_uint
                 {
                    //check transporter isn't sitting there waiting to be filled when nothing exists!
                    if (*psDroid).player as libc::c_uint == selectedPlayer &&
                           getDroidsToSafetyFlag() != 0 &&
                           missionDroidsRemaining(selectedPlayer) == 0 {
                        //check that nothing is on the transporter (transporter counts as first in group)
                        if !(*psDroid).psGroup.is_null() &&
                               ((*(*psDroid).psGroup).refCount as libc::c_int)
                                   < 2 as libc::c_int {
                            //the script can call startMission for this callback for offworld missions
                            eventFireCallbackTrigger(CALL_START_NEXT_LEVEL as
                                                         libc::c_int as
                                                         TRIGGER_TYPE);
                        }
                    }
                } else if vtolRearming(psDroid) == 0 {
                    //let vtols return to rearm
                    // attacking something, make sure the droid doesn't go too far
                    if !(*psDroid).psTarget.is_null() &&
                           (*(*psDroid).psTarget).type_0 as libc::c_uint ==
                               OBJ_DROID as libc::c_int as libc::c_uint &&
                           (*((*psDroid).psTarget as *mut DROID)).droidType as
                               libc::c_uint ==
                               DROID_COMMAND as libc::c_int as libc::c_uint {
                        // guarding a commander, allow more space
                        orderCheckGuardPosition(psDroid,
                                                128 as libc::c_int *
                                                    8 as libc::c_int);
                    } else {
                        orderCheckGuardPosition(psDroid,
                                                128 as libc::c_int *
                                                    3 as libc::c_int);
                    }
                }
                // get units in a command group to attack the commanders target
                if (*psDroid).droidType as libc::c_uint !=
                       DROID_COMMAND as libc::c_int as libc::c_uint &&
                       !(*psDroid).psGroup.is_null() &&
                       (*(*psDroid).psGroup).type_0 as libc::c_int ==
                           GT_COMMAND as libc::c_int {
                    if (*(*(*psDroid).psGroup).psCommander).action ==
                           DACTION_ATTACK as libc::c_int &&
                           !(*(*(*psDroid).psGroup).psCommander).psActionTarget.is_null()
                       {
                        psObj =
                            (*(*(*psDroid).psGroup).psCommander).psActionTarget;
                        if (*psDroid).action == DACTION_ATTACK as libc::c_int
                               ||
                               (*psDroid).action ==
                                   DACTION_MOVETOATTACK as libc::c_int {
                            if (*psDroid).psActionTarget != psObj {
                                actionDroidObj(psDroid, DACTION_ATTACK,
                                               psObj);
                            }
                        } else if (*psDroid).action !=
                                      DACTION_MOVE as libc::c_int {
                            actionDroidObj(psDroid, DACTION_ATTACK, psObj);
                        }
                    }
                    // make sure units in a command group are actually guarding the commander
                    if orderStateObj(psDroid, DORDER_GUARD, &mut psObj) == 0
                           ||
                           psObj !=
                               (*(*psDroid).psGroup).psCommander as
                                   *mut BASE_OBJECT {
                        orderDroidObj(psDroid, DORDER_GUARD,
                                      (*(*psDroid).psGroup).psCommander as
                                          *mut BASE_OBJECT);
                    }
                }
                //repair droids default to repairing droids within a given range
                psObj = 0 as *mut BASE_OBJECT;
                if ((*psDroid).droidType as libc::c_uint ==
                        DROID_REPAIR as libc::c_int as libc::c_uint ||
                        (*psDroid).droidType as libc::c_uint ==
                            DROID_CYBORG_REPAIR as libc::c_int as
                                libc::c_uint) &&
                       secondaryGetState(psDroid, DSO_HALTTYPE, &mut state) !=
                           0 &&
                       state as libc::c_uint !=
                           DSS_HALT_HOLD as libc::c_int as libc::c_uint {
                    if (*psDroid).action == DACTION_NONE as libc::c_int {
                        psObj = checkForRepairRange(psDroid, 0 as *mut DROID)
                    } else if (*psDroid).action == DACTION_SULK as libc::c_int
                     {
                        psObj =
                            checkForRepairRange(psDroid,
                                                (*psDroid).psActionTarget as
                                                    *mut DROID)
                    }
                    if !psObj.is_null() {
                        actionDroidObj(psDroid, DACTION_DROIDREPAIR, psObj);
                    }
                }
                //construct droids default to repairing structures within a given range
                psObj = 0 as *mut BASE_OBJECT;
                if ((*psDroid).droidType as libc::c_uint ==
                        DROID_CONSTRUCT as libc::c_int as libc::c_uint ||
                        (*psDroid).droidType as libc::c_uint ==
                            DROID_CYBORG_CONSTRUCT as libc::c_int as
                                libc::c_uint) &&
                       secondaryGetState(psDroid, DSO_HALTTYPE, &mut state) !=
                           0 &&
                       state as libc::c_uint !=
                           DSS_HALT_HOLD as libc::c_int as libc::c_uint {
                    if (*psDroid).action == DACTION_NONE as libc::c_int {
                        psObj =
                            checkForDamagedStruct(psDroid,
                                                  0 as *mut STRUCTURE)
                    } else if (*psDroid).action == DACTION_SULK as libc::c_int
                     {
                        psObj =
                            checkForDamagedStruct(psDroid,
                                                  (*psDroid).psActionTarget as
                                                      *mut STRUCTURE)
                    }
                    if !psObj.is_null() {
                        actionDroidObj(psDroid, DACTION_REPAIR, psObj);
                    }
                }
            }
        }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"orderUpdateUnit: unknown order\x00" as *const u8 as
                          *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"order.c\x00" as *const u8 as *const libc::c_char,
                      1150 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 17],
                                                &[libc::c_char; 17]>(b"orderUpdateDroid\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
        }
    }
    // catch any vtol that is rearming but has finished his order
    if (*psDroid).order == DORDER_NONE as libc::c_int &&
           vtolRearming(psDroid) != 0 {
        //		DBPRINTF(("VTOL %d: reseting to rearm order\n", psDroid->id));
        (*psDroid).order = DORDER_REARM as libc::c_int;
        (*psDroid).psTarget = (*psDroid).psActionTarget
    };
    //	ASSERT( psDroid->x != 0 && psDroid->y != 0,
//		"orderUpdateUnit: unit at (0,0)" );
}
/* Give a command group an order */
#[no_mangle]
pub unsafe extern "C" fn orderCmdGroupBase(mut psGroup: *mut DROID_GROUP,
                                           mut psData:
                                               *mut DROID_ORDER_DATA) {
    let mut psCurr: *mut DROID = 0 as *mut DROID;
    let mut psChosen: *mut DROID = 0 as *mut DROID;
    let mut xdiff: SDWORD = 0;
    let mut ydiff: SDWORD = 0;
    let mut currdist: SDWORD = 0;
    let mut mindist: SDWORD = 0;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"cmdUnitOrderGroupBase: invalid unit group\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"order.c\x00" as *const u8 as *const libc::c_char,
              1175 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 18],
                                        &[libc::c_char; 18]>(b"orderCmdGroupBase\x00")).as_ptr(),
              b"PTRVALID(psGroup, sizeof(DROID_GROUP))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if bMultiPlayer != 0 &&
           SendCmdGroup(psGroup, (*psData).x, (*psData).y, (*psData).psObj) !=
               0 {
        // turn off multiplay messages,since we've send a group one instead.
        turnOffMultiMsg(1 as libc::c_int);
    }
    if (*psData).order == DORDER_RECOVER as libc::c_int {
        // picking up an artifact - only need to send one unit
        psChosen = 0 as *mut DROID;
        mindist = 0x7fffffff as libc::c_int;
        psCurr = (*psGroup).psList;
        while !psCurr.is_null() {
            xdiff = (*psCurr).x as SDWORD - (*(*psData).psObj).x as SDWORD;
            ydiff = (*psCurr).y as SDWORD - (*(*psData).psObj).y as SDWORD;
            currdist = xdiff * xdiff + ydiff * ydiff;
            if currdist < mindist { psChosen = psCurr; mindist = currdist }
            psCurr = (*psCurr).psGrpNext
        }
        if !psChosen.is_null() { orderDroidBase(psChosen, psData); }
    } else {
        psCurr = (*psGroup).psList;
        while !psCurr.is_null() {
            if orderState(psCurr, DORDER_RTR) == 0 {
                // if you change this, youll need to change sendcmdgroup()
                orderDroidBase(psCurr, psData);
            }
            psCurr = (*psCurr).psGrpNext
        }
    }
    turnOffMultiMsg(0 as libc::c_int);
}
// check the position of units giving fire support to this unit and tell
// them to pull back if the sensor is going to move through them
#[no_mangle]
pub unsafe extern "C" fn orderCheckFireSupportPos(mut psSensor: *mut DROID,
                                                  mut psOrder:
                                                      *mut DROID_ORDER_DATA) {
    let mut fsx: SDWORD = 0;
    let mut fsy: SDWORD = 0;
    let mut fsnum: SDWORD = 0;
    let mut sensorVX: SDWORD = 0;
    let mut sensorVY: SDWORD = 0;
    let mut fsVX: SDWORD = 0;
    let mut fsVY: SDWORD = 0;
    let mut sensorAngle: FRACT = 0.;
    let mut fsAngle: FRACT = 0.;
    let mut adiff: FRACT = 0.;
    let mut xdiff: SDWORD = 0;
    let mut ydiff: SDWORD = 0;
    let mut state: SECONDARY_STATE = 0 as SECONDARY_STATE;
    let mut psCurr: *mut DROID = 0 as *mut DROID;
    let mut psTarget: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut bRetreat: BOOL = 0;
    // find the middle of and droids doing firesupport
    fsnum = 0 as libc::c_int;
    fsy = fsnum;
    fsx = fsy;
    psCurr = apsDroidLists[(*psSensor).player as usize];
    while !psCurr.is_null() {
        if vtolDroid(psCurr) == 0 &&
               orderStateObj(psCurr, DORDER_FIRESUPPORT, &mut psTarget) != 0
               && psTarget == psSensor as *mut BASE_OBJECT &&
               (secondaryGetState(psCurr, DSO_HALTTYPE, &mut state) != 0 &&
                    state as libc::c_uint !=
                        DSS_HALT_HOLD as libc::c_int as libc::c_uint) {
            // got a unit doing fire support
            fsnum += 1 as libc::c_int;
            fsx += (*psCurr).x as SDWORD;
            fsy += (*psCurr).y as SDWORD
        }
        psCurr = (*psCurr).psNext
    }
    bRetreat = 0 as libc::c_int;
    if fsnum != 0 as libc::c_int {
        // there are some units to check the position of
        fsx /= fsnum;
        fsy /= fsnum;
        // don't do it if too near to the firesupport units
        xdiff = fsx - (*psSensor).x as SDWORD;
        ydiff = fsy - (*psSensor).y as SDWORD;
        if !(xdiff * xdiff + ydiff * ydiff <
                 128 as libc::c_int * 5 as libc::c_int *
                     (128 as libc::c_int * 5 as libc::c_int)) {
            sensorVX = (*psOrder).x as SDWORD - (*psSensor).x as SDWORD;
            sensorVY = (*psOrder).y as SDWORD - (*psSensor).y as SDWORD;
            fsVX = fsx - (*psSensor).x as SDWORD;
            fsVY = fsy - (*psSensor).y as SDWORD;
            // now check if the move position is further away than the firesupport units
            if !(sensorVX * sensorVX + sensorVY * sensorVY <
                     fsVX * fsVX + fsVY * fsVY) {
                // now get the angle between the firesupport units and the sensor move
                sensorAngle =
                    atan2(sensorVY as libc::c_double,
                          sensorVX as libc::c_double) as FRACT;
                fsAngle =
                    atan2(fsVY as libc::c_double, fsVX as libc::c_double) as
                        FRACT;
                adiff = fsAngle - sensorAngle;
                if adiff < 0 as libc::c_int as libc::c_float {
                    adiff +=
                        (3.141592654f64 * 2 as libc::c_int as libc::c_double)
                            as FRACT
                }
                if adiff as libc::c_double > 3.141592654f64 {
                    adiff -= 3.141592654f64 as FRACT
                }
                // if the angle between the firesupport units and the sensor move is bigger
		// than 45 degrees don't retreat
                if !(adiff as libc::c_double >
                         3.141592654f64 / 4 as libc::c_int as libc::c_double)
                   {
                    bRetreat = 1 as libc::c_int
                }
            }
        }
    }
    // made a decision whether to retreat
    // now move the firesupport units
    psCurr = apsDroidLists[(*psSensor).player as usize];
    while !psCurr.is_null() {
        if vtolDroid(psCurr) == 0 &&
               orderStateObj(psCurr, DORDER_FIRESUPPORT, &mut psTarget) != 0
               && psTarget == psSensor as *mut BASE_OBJECT &&
               (secondaryGetState(psCurr, DSO_HALTTYPE, &mut state) != 0 &&
                    state as libc::c_uint !=
                        DSS_HALT_HOLD as libc::c_int as libc::c_uint) {
            if bRetreat != 0 {
                actionDroidLoc(psCurr, DACTION_FIRESUPPORT_RETREAT,
                               (*psOrder).x as UDWORD,
                               (*psOrder).y as UDWORD);
            } else if (*psCurr).action ==
                          DACTION_FIRESUPPORT_RETREAT as libc::c_int {
                actionDroid(psCurr, DACTION_NONE);
            }
        }
        psCurr = (*psCurr).psNext
    };
}
unsafe extern "C" fn orderPlayFireSupportAudio(mut psObj: *mut BASE_OBJECT) {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut iAudioID: SDWORD = NO_SOUND as libc::c_int;
    /* play appropriate speech */
    match (*psObj).type_0 as libc::c_uint {
        0 => {
            psDroid = psObj as *mut DROID;
            if 1 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"orderPlayFireSupportAudio: invalid droid pointer\x00"
                          as *const u8 as *const libc::c_char);
            };
            if 1 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"order.c\x00" as *const u8 as *const libc::c_char,
                      1338 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 26],
                                                &[libc::c_char; 26]>(b"orderPlayFireSupportAudio\x00")).as_ptr(),
                      b"PTRVALID(psObj, sizeof(DROID))\x00" as *const u8 as
                          *const libc::c_char);
            };
            if (*psDroid).droidType as libc::c_uint ==
                   DROID_COMMAND as libc::c_int as libc::c_uint {
                iAudioID = ID_SOUND_ASSIGNED_TO_COMMANDER as libc::c_int
            } else if (*psDroid).droidType as libc::c_uint ==
                          DROID_SENSOR as libc::c_int as libc::c_uint {
                iAudioID = ID_SOUND_ASSIGNED_TO_SENSOR as libc::c_int
            }
        }
        1 => {
            if 1 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"orderPlayFireSupportAudio: invalid structure pointer\x00"
                          as *const u8 as *const libc::c_char);
            };
            if 1 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"order.c\x00" as *const u8 as *const libc::c_char,
                      1351 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 26],
                                                &[libc::c_char; 26]>(b"orderPlayFireSupportAudio\x00")).as_ptr(),
                      b"PTRVALID(psObj, sizeof(STRUCTURE))\x00" as *const u8
                          as *const libc::c_char);
            };
            psStruct = psObj as *mut STRUCTURE;
            //check for non-CB first
            if structStandardSensor(psStruct) != 0 ||
                   structVTOLSensor(psStruct) != 0 {
                iAudioID = ID_SOUND_ASSIGNED_TO_SENSOR as libc::c_int
            } else if structCBSensor(psStruct) != 0 ||
                          structVTOLCBSensor(psStruct) != 0 {
                iAudioID = ID_SOUND_ASSIGNED_TO_COUNTER_RADAR as libc::c_int
            }
        }
        _ => { }
    }
    if iAudioID != NO_SOUND as libc::c_int {
        audio_QueueTrackMinDelay(iAudioID,
                                 (3 as libc::c_int * 1000 as libc::c_int) as
                                     UDWORD);
    };
}
// retreat positions for the players
/* The base order function */
#[no_mangle]
pub unsafe extern "C" fn orderDroidBase(mut psDroid: *mut DROID,
                                        mut psOrder: *mut DROID_ORDER_DATA) {
    //	UDWORD		actionX,actionY;
    let mut iRepairFacDistSq: UDWORD = 0;
    let mut iStructDistSq: UDWORD = 0;
    let mut iFactoryDistSq: UDWORD = 0;
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psRepairFac: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psFactory: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut iDX: SDWORD = 0;
    let mut iDY: SDWORD = 0;
    let mut state: SECONDARY_STATE = 0 as SECONDARY_STATE;
    let mut droidX: UDWORD = 0;
    let mut droidY: UDWORD = 0;
    //	ASSERT( psDroid->x != 0 && psDroid->y != 0,
//		"orderUnitBase: unit at (0,0)" );
    // deal with a droid receiving a primary order
    if secondaryGotPrimaryOrder(psDroid, (*psOrder).order as DROID_ORDER) != 0
       {
        (*psOrder).order = DORDER_NONE as libc::c_int
    }
    // if this is a command droid - all it's units do the same thing
    if (*psDroid).droidType as libc::c_uint ==
           DROID_COMMAND as libc::c_int as libc::c_uint &&
           !(*psDroid).psGroup.is_null() &&
           (*(*psDroid).psGroup).type_0 as libc::c_int ==
               GT_COMMAND as libc::c_int &&
           (*psOrder).order != DORDER_GUARD as libc::c_int &&
           (*psOrder).order != DORDER_RTR as libc::c_int &&
           (*psOrder).order != DORDER_RECYCLE as libc::c_int &&
           (*psOrder).order != DORDER_MOVE as libc::c_int {
        if (*psOrder).order == DORDER_ATTACK as libc::c_int {
            // change to attacktarget so that the group members
			// guard order does not get canceled
            (*psOrder).order = DORDER_ATTACKTARGET as libc::c_int;
            orderCmdGroupBase((*psDroid).psGroup, psOrder);
            (*psOrder).order = DORDER_ATTACK as libc::c_int
        } else { orderCmdGroupBase((*psDroid).psGroup, psOrder); }
        // the commander doesn't have to pick up artifacts, one
		// of his units will do it for him (if there are any in his group).
        if (*psOrder).order == DORDER_RECOVER as libc::c_int &&
               !(*(*psDroid).psGroup).psList.is_null() {
            (*psOrder).order = DORDER_NONE as libc::c_int
        }
    }
    let mut current_block_270: u64;
    /*	if ( ((psDroid->droidType == DROID_SENSOR) ||
		  (psDroid->droidType == DROID_COMMAND)) &&
		 (psOrder->order == DORDER_MOVE) )
	{
		orderCheckFireSupportPos(psDroid, psOrder);
	}*/
    match (*psOrder).order {
        0 => { }
        1 => {
            // get the droid to stop doing whatever it is doing
            actionDroid(psDroid, DACTION_NONE);
            (*psDroid).order = DORDER_NONE as libc::c_int;
            (*psDroid).psTarget = 0 as *mut _base_object;
            (*psDroid).psTarStats = 0 as *mut _base_stats;
            (*psDroid).orderX = 0 as libc::c_int as UWORD;
            (*psDroid).orderY = 0 as libc::c_int as UWORD;
            (*psDroid).orderX2 = 0 as libc::c_int as UWORD;
            (*psDroid).orderY2 = 0 as libc::c_int as UWORD
        }
        2 | 28 => {
            // can't move vtols to blocking tiles
            if !(vtolDroid(psDroid) != 0 &&
                     (fpathGroundBlockingTile((*psOrder).x as libc::c_int >>
                                                  7 as libc::c_int,
                                              (*psOrder).y as libc::c_int >>
                                                  7 as libc::c_int) != 0 ||
                          (*mapTile(((*psOrder).x as libc::c_int >>
                                         7 as libc::c_int) as UDWORD,
                                    ((*psOrder).y as libc::c_int >>
                                         7 as libc::c_int) as
                                        UDWORD)).tileVisBits as libc::c_int &
                              (1 as libc::c_int) <<
                                  (*psDroid).player as libc::c_int == 0)) {
                //in multiPlayer, cannot move Transporter to blocking tile either
                if game.maxPlayers as libc::c_int > 0 as libc::c_int {
                    if (*psDroid).droidType as libc::c_uint ==
                           DROID_TRANSPORTER as libc::c_int as libc::c_uint &&
                           (fpathGroundBlockingTile((*psOrder).x as
                                                        libc::c_int >>
                                                        7 as libc::c_int,
                                                    (*psOrder).y as
                                                        libc::c_int >>
                                                        7 as libc::c_int) != 0
                                ||
                                (*mapTile(((*psOrder).x as libc::c_int >>
                                               7 as libc::c_int) as UDWORD,
                                          ((*psOrder).y as libc::c_int >>
                                               7 as libc::c_int) as
                                              UDWORD)).tileVisBits as
                                    libc::c_int &
                                    (1 as libc::c_int) <<
                                        (*psDroid).player as libc::c_int == 0)
                       {
                        current_block_270 = 12782750567233516376;
                    } else { current_block_270 = 2569451025026770673; }
                } else { current_block_270 = 2569451025026770673; }
                match current_block_270 {
                    12782750567233516376 => { }
                    _ => {
                        // move a droid to a location
                        (*psDroid).order = (*psOrder).order;
                        (*psDroid).orderX = (*psOrder).x;
                        (*psDroid).orderY = (*psOrder).y;
                        actionDroidLoc(psDroid, DACTION_MOVE,
                                       (*psOrder).x as UDWORD,
                                       (*psOrder).y as UDWORD);
                    }
                }
            }
        }
        31 => {
            (*psDroid).order = (*psOrder).order;
            (*psDroid).orderX = (*psOrder).x;
            (*psDroid).orderY = (*psOrder).y;
            (*psDroid).orderX2 = (*psDroid).x;
            (*psDroid).orderY2 = (*psDroid).y;
            actionDroidLoc(psDroid, DACTION_MOVE, (*psOrder).x as UDWORD,
                           (*psOrder).y as UDWORD);
        }
        35 => {
            (*psDroid).order = DORDER_RECOVER as libc::c_int;
            (*psDroid).psTarget = (*psOrder).psObj;
            actionDroidLoc(psDroid, DACTION_MOVE,
                           (*(*psOrder).psObj).x as UDWORD,
                           (*(*psOrder).psObj).y as UDWORD);
        }
        22 => {
            // tell a (transporter) droid to leave home base for the offworld mission
            (*psDroid).order = DORDER_TRANSPORTOUT as libc::c_int;
            (*psDroid).orderX = (*psOrder).x;
            (*psDroid).orderY = (*psOrder).y;
            actionDroidLoc(psDroid, DACTION_TRANSPORTOUT,
                           (*psOrder).x as UDWORD, (*psOrder).y as UDWORD);
        }
        24 => {
            // tell a (transporter) droid to return after unloading
            (*psDroid).order = DORDER_TRANSPORTRETURN as libc::c_int;
            (*psDroid).orderX = (*psOrder).x;
            (*psDroid).orderY = (*psOrder).y;
            actionDroidLoc(psDroid, DACTION_TRANSPORTOUT,
                           (*psOrder).x as UDWORD, (*psOrder).y as UDWORD);
        }
        23 => {
            // tell a (transporter) droid to fly onworld
            (*psDroid).order = DORDER_TRANSPORTIN as libc::c_int;
            (*psDroid).orderX = (*psOrder).x;
            (*psDroid).orderY = (*psOrder).y;
            actionDroidLoc(psDroid, DACTION_TRANSPORTIN,
                           (*psOrder).x as UDWORD, (*psOrder).y as UDWORD);
        }
        3 | 18 => {
            // If there arn't any weapons stop
		//if (psDroid->numWeaps == 0)
            if !((*psDroid).asWeaps[0 as libc::c_int as usize].nStat ==
                     0 as libc::c_int as libc::c_uint ||
                     (*psDroid).droidType as libc::c_uint ==
                         DROID_TRANSPORTER as libc::c_int as libc::c_uint) {
                //check if vtol there is ammo
		/*if (vtolEmpty(psDroid))
		{
			moveToRearm(psDroid);
			break;
		}*/
/*		if (!proj_Direct(asWeaponStats + psDroid->asWeaps[0].nStat) &&
			!visibleObject((BASE_OBJECT *)psDroid, psOrder->psObj))
		{
			// indirect can only attack things they can see unless attacking
			// through a sensor droid - see DORDER_FIRESUPPORT
			break;
		}*/
                if (*psDroid).order == DORDER_GUARD as libc::c_int &&
                       (*psOrder).order == DORDER_ATTACKTARGET as libc::c_int
                   {
                    // attacking something while guarding, don't change the order
                    actionDroidObj(psDroid, DACTION_ATTACK, (*psOrder).psObj);
                } else if !(game.maxPlayers as libc::c_int > 0 as libc::c_int
                                && electronicDroid(psDroid) != 0 &&
                                (*(*psOrder).psObj).type_0 as libc::c_uint ==
                                    OBJ_DROID as libc::c_int as libc::c_uint
                                &&
                                (*((*psOrder).psObj as *mut DROID)).droidType
                                    as libc::c_uint ==
                                    DROID_TRANSPORTER as libc::c_int as
                                        libc::c_uint) {
                    (*psDroid).psTarget = (*psOrder).psObj;
                    //cannot attack a Transporter with EW in multiPlayer
                    //			psDroid->order = DORDER_ATTACK;
                    (*psDroid).order = (*psOrder).order;
                    if vtolDroid(psDroid) != 0 ||
                           actionInsideMinRange(psDroid, (*psDroid).psTarget)
                               != 0 ||
                           (*psOrder).order ==
                               DORDER_ATTACKTARGET as libc::c_int &&
                               secondaryGetState(psDroid, DSO_HALTTYPE,
                                                 &mut state) != 0 &&
                               state as libc::c_uint ==
                                   DSS_HALT_HOLD as libc::c_int as
                                       libc::c_uint {
                        actionDroidObj(psDroid, DACTION_ATTACK,
                                       (*psDroid).psTarget);
                    } else {
                        actionDroidLoc(psDroid, DACTION_MOVE,
                                       (*(*psOrder).psObj).x as UDWORD,
                                       (*(*psOrder).psObj).y as UDWORD);
                    }
                }
            }
        }
        4 => {
            // build a new structure
		//if (psDroid->droidType != DROID_CONSTRUCT)
            if (*psDroid).droidType as libc::c_uint ==
                   DROID_CONSTRUCT as libc::c_int as libc::c_uint ||
                   (*psDroid).droidType as libc::c_uint ==
                       DROID_CYBORG_CONSTRUCT as libc::c_int as libc::c_uint {
                if 1 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"orderUnitBase: invalid structure stats pointer\x00"
                              as *const u8 as *const libc::c_char);
                };
                if 1 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"order.c\x00" as *const u8 as *const libc::c_char,
                          1616 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 15],
                                                    &[libc::c_char; 15]>(b"orderDroidBase\x00")).as_ptr(),
                          b"PTRVALID(psOrder->psStats, sizeof(STRUCTURE_STATS))\x00"
                              as *const u8 as *const libc::c_char);
                };
                //if (getDroidDestination((STRUCTURE_STATS *)psOrder->psStats,
//		if (getDroidDestination(psOrder->psStats,
//								psOrder->x,psOrder->y, &actionX,&actionY))
                (*psDroid).order = DORDER_BUILD as libc::c_int;
                (*psDroid).orderX = (*psOrder).x;
                (*psDroid).orderY = (*psOrder).y;
                (*psDroid).psTarget = 0 as *mut _base_object;
                (*psDroid).psTarStats = (*psOrder).psStats;
                actionDroidLoc(psDroid, DACTION_BUILD, (*psOrder).x as UDWORD,
                               (*psOrder).y as UDWORD);
            }
        }
        20 => {
            //build a module onto the structure
		//if (psDroid->droidType != DROID_CONSTRUCT)
            if (*psDroid).droidType as libc::c_uint ==
                   DROID_CONSTRUCT as libc::c_int as libc::c_uint ||
                   (*psDroid).droidType as libc::c_uint ==
                       DROID_CYBORG_CONSTRUCT as libc::c_int as libc::c_uint {
                //if (getDroidDestination(((STRUCTURE *)psOrder->psObj)->pStructureType,
//		if (getDroidDestination((BASE_STATS *)((STRUCTURE *)psOrder->psObj)->pStructureType,
//								psOrder->psObj->x,psOrder->psObj->y,
//								&actionX,&actionY))
                (*psDroid).order = DORDER_BUILD as libc::c_int;
                (*psDroid).orderX = (*(*psOrder).psObj).x;
                (*psDroid).orderY = (*(*psOrder).psObj).y;
                (*psDroid).psTarget = 0 as *mut _base_object;
                (*psDroid).psTarStats =
                    getModuleStat((*psOrder).psObj as *mut STRUCTURE) as
                        *mut BASE_STATS;
                if !(*psDroid).psTarStats.is_null() {
                } else {
                    debug(LOG_ERROR,
                          b"orderUnitBase: should have found a module stats\x00"
                              as *const u8 as *const libc::c_char);
                };
                if !(*psDroid).psTarStats.is_null() {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"order.c\x00" as *const u8 as *const libc::c_char,
                          1679 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 15],
                                                    &[libc::c_char; 15]>(b"orderDroidBase\x00")).as_ptr(),
                          b"psDroid->psTarStats != NULL\x00" as *const u8 as
                              *const libc::c_char);
                };
                actionDroidLoc(psDroid, DACTION_BUILD,
                               (*(*psOrder).psObj).x as UDWORD,
                               (*(*psOrder).psObj).y as UDWORD);
            }
        }
        6 => {
            // build a line of structures
		//if (psDroid->droidType != DROID_CONSTRUCT)
            if (*psDroid).droidType as libc::c_uint ==
                   DROID_CONSTRUCT as libc::c_int as libc::c_uint ||
                   (*psDroid).droidType as libc::c_uint ==
                       DROID_CYBORG_CONSTRUCT as libc::c_int as libc::c_uint {
                if 1 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"orderUnitBase: invalid structure stats pointer\x00"
                              as *const u8 as *const libc::c_char);
                };
                if 1 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"order.c\x00" as *const u8 as *const libc::c_char,
                          1692 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 15],
                                                    &[libc::c_char; 15]>(b"orderDroidBase\x00")).as_ptr(),
                          b"PTRVALID(psOrder->psStats, sizeof(STRUCTURE_STATS))\x00"
                              as *const u8 as *const libc::c_char);
                };
                //if (getDroidDestination((STRUCTURE_STATS *)psOrder->psStats,
//		if (getDroidDestination(psOrder->psStats,
//								psOrder->x,psOrder->y, &actionX,&actionY))
                (*psDroid).order = DORDER_LINEBUILD as libc::c_int;
                (*psDroid).orderX = (*psOrder).x;
                (*psDroid).orderY = (*psOrder).y;
                (*psDroid).orderX2 = (*psOrder).x2;
                (*psDroid).orderY2 = (*psOrder).y2;
                (*psDroid).psTarget = 0 as *mut _base_object;
                (*psDroid).psTarStats = (*psOrder).psStats;
                actionDroidLoc(psDroid, DACTION_BUILD, (*psOrder).x as UDWORD,
                               (*psOrder).y as UDWORD);
            }
        }
        5 => {
            // help to build a structure that is starting to be built
		//if (psDroid->droidType != DROID_CONSTRUCT)
            if (*psDroid).droidType as libc::c_uint ==
                   DROID_CONSTRUCT as libc::c_int as libc::c_uint ||
                   (*psDroid).droidType as libc::c_uint ==
                       DROID_CYBORG_CONSTRUCT as libc::c_int as libc::c_uint {
                //if (getDroidDestination(((STRUCTURE *)psOrder->psObj)->pStructureType,
//		if (getDroidDestination((BASE_STATS *)((STRUCTURE *)psOrder->psObj)->pStructureType,
//								psOrder->psObj->x,psOrder->psObj->y,
//								&actionX,&actionY))
                (*psDroid).order = DORDER_HELPBUILD as libc::c_int;
                (*psDroid).orderX = (*(*psOrder).psObj).x;
                (*psDroid).orderY = (*(*psOrder).psObj).y;
                //			psDroid->orderX = (UWORD)(psOrder->psObj->x + ((STRUCTURE *)psOrder->psObj)->
//              pStructureType->baseWidth * TILE_UNITS/2);
//            psDroid->orderY = (UWORD)(psOrder->psObj->y + ((STRUCTURE *)psOrder->psObj)->
//                pStructureType->baseBreadth * TILE_UNITS/2);
                (*psDroid).psTarget = (*psOrder).psObj;
                (*psDroid).psTarStats =
                    (*((*psOrder).psObj as *mut STRUCTURE)).pStructureType as
                        *mut BASE_STATS;
                actionDroidLoc(psDroid, DACTION_BUILD,
                               (*psDroid).orderX as UDWORD,
                               (*psDroid).orderY as UDWORD);
            }
        }
        7 => {
            //if (psDroid->droidType != DROID_CONSTRUCT)
            if (*psDroid).droidType as libc::c_uint ==
                   DROID_CONSTRUCT as libc::c_int as libc::c_uint ||
                   (*psDroid).droidType as libc::c_uint ==
                       DROID_CYBORG_CONSTRUCT as libc::c_int as libc::c_uint {
                (*psDroid).order = DORDER_DEMOLISH as libc::c_int;
                (*psDroid).orderX = (*(*psOrder).psObj).x;
                (*psDroid).orderY = (*(*psOrder).psObj).y;
                (*psDroid).psTarget = (*psOrder).psObj;
                actionDroidObj(psDroid, DACTION_DEMOLISH, (*psOrder).psObj);
            }
        }
        8 => {
            //if (psDroid->droidType != DROID_CONSTRUCT)
            if (*psDroid).droidType as libc::c_uint ==
                   DROID_CONSTRUCT as libc::c_int as libc::c_uint ||
                   (*psDroid).droidType as libc::c_uint ==
                       DROID_CYBORG_CONSTRUCT as libc::c_int as libc::c_uint {
                (*psDroid).order = DORDER_REPAIR as libc::c_int;
                (*psDroid).orderX = (*(*psOrder).psObj).x;
                (*psDroid).orderY = (*(*psOrder).psObj).y;
                (*psDroid).psTarget = (*psOrder).psObj;
                actionDroidObj(psDroid, DACTION_REPAIR, (*psOrder).psObj);
            }
        }
        26 => {
            //if (psDroid->droidType != DROID_REPAIR)
            if (*psDroid).droidType as libc::c_uint ==
                   DROID_REPAIR as libc::c_int as libc::c_uint ||
                   (*psDroid).droidType as libc::c_uint ==
                       DROID_CYBORG_REPAIR as libc::c_int as libc::c_uint {
                (*psDroid).order = DORDER_DROIDREPAIR as libc::c_int;
                (*psDroid).psTarget = (*psOrder).psObj;
                actionDroidObj(psDroid, DACTION_DROIDREPAIR,
                               (*psOrder).psObj);
            }
        }
        9 => {
            // keep an object within sensor view
            (*psDroid).order = DORDER_OBSERVE as libc::c_int;
            (*psDroid).psTarget = (*psOrder).psObj;
            actionDroidObj(psDroid, DACTION_OBSERVE, (*psOrder).psObj);
        }
        10 => {
            //if (psDroid->numWeaps == 0)
            if !((*psDroid).asWeaps[0 as libc::c_int as usize].nStat ==
                     0 as libc::c_int as libc::c_uint) {
                (*psDroid).order = DORDER_FIRESUPPORT as libc::c_int;
                (*psDroid).psTarget = (*psOrder).psObj;
                // let the order update deal with vtol droids
                if vtolDroid(psDroid) == 0 {
                    actionDroidObj(psDroid, DACTION_FIRESUPPORT,
                                   (*psOrder).psObj);
                }
                if (*psDroid).player as libc::c_uint == selectedPlayer {
                    orderPlayFireSupportAudio((*psOrder).psObj);
                }
            }
        }
        11 | 29 | 15 => {
            (*psDroid).order = (*psOrder).order;
            if (*psOrder).order == DORDER_RUN as libc::c_int &&
                   ((*psOrder).x as libc::c_int != 0 as libc::c_int ||
                        (*psOrder).y as libc::c_int != 0 as libc::c_int) {
                (*psDroid).orderX = (*psOrder).x;
                (*psDroid).orderY = (*psOrder).y
            } else {
                (*psDroid).orderX =
                    asRunData[(*psDroid).player as usize].sPos.x as UWORD;
                (*psDroid).orderY =
                    asRunData[(*psDroid).player as usize].sPos.y as UWORD
            }
            actionDroidLoc(psDroid, DACTION_MOVE, (*psDroid).orderX as UDWORD,
                           (*psDroid).orderY as UDWORD);
        }
        12 => {
            (*psDroid).order = DORDER_DESTRUCT as libc::c_int;
            actionDroid(psDroid, DACTION_DESTRUCT);
        }
        13 => {
            // send vtols back to their return pos
            if vtolDroid(psDroid) != 0 && bMultiPlayer == 0 &&
                   (*psDroid).player as libc::c_uint != selectedPlayer {
                iDX = asVTOLReturnPos[(*psDroid).player as usize].x;
                iDY = asVTOLReturnPos[(*psDroid).player as usize].y;
                if iDX != 0 && iDY != 0 {
                    (*psDroid).order = DORDER_LEAVEMAP as libc::c_int;
                    actionDroidLoc(psDroid, DACTION_MOVE, iDX as UDWORD,
                                   iDY as UDWORD);
                    if !(*psDroid).sMove.psFormation.is_null() {
                        formationLeave((*psDroid).sMove.psFormation,
                                       psDroid as *mut BASE_OBJECT);
                        (*psDroid).sMove.psFormation = 0 as *mut _formation
                    }
                    current_block_270 = 12782750567233516376;
                } else { current_block_270 = 9822987968968565122; }
            } else { current_block_270 = 9822987968968565122; }
            match current_block_270 {
                12782750567233516376 => { }
                _ => {
                    psStruct = apsStructLists[(*psDroid).player as usize];
                    while !psStruct.is_null() {
                        if (*(*psStruct).pStructureType).type_0 ==
                               REF_HQ as libc::c_int as libc::c_uint {
                            (*psDroid).order = DORDER_RTB as libc::c_int;
                            droidX = (*psStruct).x as UDWORD;
                            droidY = (*psStruct).y as UDWORD;
                            //find a place to land for vtols
                //And Transporters in a multiPlay game
                            if vtolDroid(psDroid) != 0 ||
                                   game.maxPlayers as libc::c_int >
                                       0 as libc::c_int &&
                                       (*psDroid).droidType as libc::c_uint ==
                                           DROID_TRANSPORTER as libc::c_int as
                                               libc::c_uint {
                                actionVTOLLandingPos(psDroid, &mut droidX,
                                                     &mut droidY);
                            }
                            actionDroidLoc(psDroid, DACTION_MOVE, droidX,
                                           droidY);
                            break ;
                        } else { psStruct = (*psStruct).psNext }
                    }
                    // no HQ go to the landing zone
/*#ifndef PSX
		if (!bMultiPlayer && selectedPlayer == 0)
#else
		if (selectedPlayer == 0)
#endif*/
                    if (*psDroid).order != DORDER_RTB as libc::c_int {
                        //see if the LZ has been set up
                        iDX =
                            getLandingX((*psDroid).player as SDWORD) as
                                SDWORD;
                        iDY =
                            getLandingY((*psDroid).player as SDWORD) as
                                SDWORD;
                        if iDX != 0 && iDY != 0 {
                            (*psDroid).order = DORDER_RTB as libc::c_int;
                            //actionDroidLoc(psDroid, DACTION_MOVE, getLandingX(psDroid->player),
			    //				getLandingY(psDroid->player));
                            actionDroidLoc(psDroid, DACTION_MOVE,
                                           iDX as UDWORD, iDY as UDWORD);
                        } else {
                            //haven't got an LZ set up so don't do anything
                            (*psDroid).order = DORDER_NONE as libc::c_int
                        }
                    }
                }
            }
        }
        14 | 37 => {
            if vtolDroid(psDroid) != 0 {
                moveToRearm(psDroid);
            } else {
                if (*psOrder).psObj.is_null() {
                    psRepairFac = 0 as *mut STRUCTURE;
                    iRepairFacDistSq = 0 as libc::c_int as UDWORD;
                    psStruct = apsStructLists[(*psDroid).player as usize];
                    while !psStruct.is_null() {
                        if (*(*psStruct).pStructureType).type_0 ==
                               REF_REPAIR_FACILITY as libc::c_int as
                                   libc::c_uint ||
                               (*(*psStruct).pStructureType).type_0 ==
                                   REF_HQ as libc::c_int as libc::c_uint &&
                                   psRepairFac.is_null() {
                            /* get droid->facility distance squared */
                            iDX =
                                (*psDroid).x as SDWORD -
                                    (*psStruct).x as SDWORD;
                            iDY =
                                (*psDroid).y as SDWORD -
                                    (*psStruct).y as SDWORD;
                            iStructDistSq = (iDX * iDX + iDY * iDY) as UDWORD;
                            /* choose current structure if first repair facility found or
					 * nearer than previously chosen facility
					 */
                            if psRepairFac.is_null() ||
                                   (*(*psRepairFac).pStructureType).type_0 ==
                                       REF_HQ as libc::c_int as libc::c_uint
                                   || iRepairFacDistSq > iStructDistSq {
                                /* first facility found */
                                psRepairFac = psStruct;
                                iRepairFacDistSq = iStructDistSq
                            }
                        }
                        psStruct = (*psStruct).psNext
                    }
                } else { psRepairFac = (*psOrder).psObj as *mut STRUCTURE }
                // droids doing a DORDER_RTR periodically give themselves a DORDER_RTR so that
		// they always go to the nearest repair facility
		// this stops the unit doing anything more if the same repair facility gets chosen
                if !((*psDroid).order == DORDER_RTR as libc::c_int &&
                         (*psDroid).psTarget ==
                             psRepairFac as *mut BASE_OBJECT) {
                    /* give repair order if repair facility found */
                    if !psRepairFac.is_null() {
                        if (*(*psRepairFac).pStructureType).type_0 ==
                               REF_REPAIR_FACILITY as libc::c_int as
                                   libc::c_uint {
                            /* move to front of structure */
                            (*psDroid).order = (*psOrder).order;
                            (*psDroid).orderX = (*psRepairFac).x;
                            (*psDroid).orderY = (*psRepairFac).y;
                            (*psDroid).psTarget =
                                psRepairFac as *mut BASE_OBJECT;
                            /*if in multiPlayer, and the Transporter has been sent to be
                repaired, need to find a suitable location to drop down*/
                            if game.maxPlayers as libc::c_int >
                                   0 as libc::c_int &&
                                   (*psDroid).droidType as libc::c_uint ==
                                       DROID_TRANSPORTER as libc::c_int as
                                           libc::c_uint {
                                let mut droidX_0: UDWORD = 0;
                                let mut droidY_0: UDWORD = 0;
                                droidX_0 = (*psDroid).orderX as UDWORD;
                                droidY_0 = (*psDroid).orderY as UDWORD;
                                actionVTOLLandingPos(psDroid, &mut droidX_0,
                                                     &mut droidY_0);
                                actionDroidLoc(psDroid, DACTION_MOVE,
                                               droidX_0, droidY_0);
                            } else {
                                actionDroidObjLoc(psDroid, DACTION_MOVE,
                                                  psRepairFac as
                                                      *mut BASE_OBJECT,
                                                  (*psDroid).orderX as UDWORD,
                                                  (*psDroid).orderY as
                                                      UDWORD);
                            }
                        } else { orderDroid(psDroid, DORDER_RTB); }
                    } else if bMultiPlayer == 0 &&
                                  selectedPlayer ==
                                      0 as libc::c_int as libc::c_uint {
                        orderDroid(psDroid, DORDER_RTB);
                        // no repair facility or HQ go to the landing zone
                        /*				orderDroidLoc(psDroid, DORDER_MOVE, getLandingX(psDroid->player),
								getLandingY(psDroid->player));*/
//					(sLandingZone[0].x1 + sLandingZone[0].x2)/2, (sLandingZone[0].y1 + sLandingZone[0].y2)/2);
                    }
                }
            }
        }
        16 => {
            // move the droid to the transporter location
            (*psDroid).order = DORDER_EMBARK as libc::c_int;
            (*psDroid).orderX = (*(*psOrder).psObj).x;
            (*psDroid).orderY = (*(*psOrder).psObj).y;
            (*psDroid).psTarget = (*psOrder).psObj;
            actionDroidLoc(psDroid, DACTION_MOVE,
                           (*(*psOrder).psObj).x as UDWORD,
                           (*(*psOrder).psObj).y as UDWORD);
        }
        17 => {
            //only valid in multiPlayer mode - cannot use the check on bMultiPlayer since it has been
        //set to FALSE before this function call
            if game.maxPlayers as libc::c_int > 0 as libc::c_int {
                //this order can only be given to Transporter droids
                if (*psDroid).droidType as libc::c_uint ==
                       DROID_TRANSPORTER as libc::c_int as libc::c_uint {
                    (*psDroid).order = DORDER_DISEMBARK as libc::c_int;
                    (*psDroid).orderX = (*psOrder).x;
                    (*psDroid).orderY = (*psOrder).y;
                    //move the Transporter to the requested location
                    actionDroidLoc(psDroid, DACTION_MOVE,
                                   (*psOrder).x as UDWORD,
                                   (*psOrder).y as UDWORD);
                    //close the Transporter interface - if up
                    if !widgGetFromID(psWScreen,
                                      9000 as libc::c_int as UDWORD).is_null()
                       {
                        intRemoveTrans();
                    }
                }
            }
        }
        21 => {
            psFactory = 0 as *mut STRUCTURE;
            iFactoryDistSq = 0 as libc::c_int as UDWORD;
            psStruct = apsStructLists[(*psDroid).player as usize];
            while !psStruct.is_null() {
                //look for nearest factory or repair facility
                if (*(*psStruct).pStructureType).type_0 ==
                       REF_FACTORY as libc::c_int as libc::c_uint ||
                       (*(*psStruct).pStructureType).type_0 ==
                           REF_CYBORG_FACTORY as libc::c_int as libc::c_uint
                       ||
                       (*(*psStruct).pStructureType).type_0 ==
                           REF_VTOL_FACTORY as libc::c_int as libc::c_uint ||
                       (*(*psStruct).pStructureType).type_0 ==
                           REF_REPAIR_FACILITY as libc::c_int as libc::c_uint
                   {
                    /* get droid->facility distance squared */
                    iDX = (*psDroid).x as SDWORD - (*psStruct).x as SDWORD;
                    iDY = (*psDroid).y as SDWORD - (*psStruct).y as SDWORD;
                    iStructDistSq = (iDX * iDX + iDY * iDY) as UDWORD;
                    /* choose current structure if first facility found or
				 * nearer than previously chosen facility
				 */
                    if psFactory.is_null() || iFactoryDistSq > iStructDistSq {
                        /* first facility found */
                        psFactory = psStruct;
                        iFactoryDistSq = iStructDistSq
                    }
                }
                psStruct = (*psStruct).psNext
            }
            /* give recycle order if facility found */
            if !psFactory.is_null() {
                /* move to front of structure */
                (*psDroid).order = DORDER_RECYCLE as libc::c_int;
                (*psDroid).orderX = (*psFactory).x;
                (*psDroid).orderY =
                    ((*psFactory).y as
                         libc::c_uint).wrapping_add(((*(*psFactory).pStructureType).baseBreadth
                                                         <<
                                                         7 as
                                                             libc::c_int).wrapping_div(2
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           libc::c_uint)).wrapping_add((128
                                                                                                                            as
                                                                                                                            libc::c_int
                                                                                                                            /
                                                                                                                            2
                                                                                                                                as
                                                                                                                                libc::c_int)
                                                                                                                           as
                                                                                                                           libc::c_uint)
                        as UWORD;
                (*psDroid).psTarget = psFactory as *mut BASE_OBJECT;
                actionDroidObjLoc(psDroid, DACTION_MOVE,
                                  psFactory as *mut BASE_OBJECT,
                                  (*psDroid).orderX as UDWORD,
                                  (*psDroid).orderY as UDWORD);
            }
        }
        25 => {
            (*psDroid).order = DORDER_GUARD as libc::c_int;
            (*psDroid).psTarget = (*psOrder).psObj;
            if !(*psOrder).psObj.is_null() {
                (*psDroid).orderX = (*(*psOrder).psObj).x;
                (*psDroid).orderY = (*(*psOrder).psObj).y
            } else {
                (*psDroid).orderX = (*psOrder).x;
                (*psDroid).orderY = (*psOrder).y
            }
            actionDroid(psDroid, DACTION_NONE);
        }
        27 => {
            if !(electronicDroid(psDroid) == 0) {
                if (*(*psOrder).psObj).type_0 as libc::c_uint !=
                       OBJ_STRUCTURE as libc::c_int as libc::c_uint {
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"orderDroidBase: invalid object type for Restore order\x00"
                                  as *const u8 as *const libc::c_char);
                    };
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                                  *const u8 as *const libc::c_char,
                              b"order.c\x00" as *const u8 as
                                  *const libc::c_char, 2071 as libc::c_int,
                              (*::std::mem::transmute::<&[u8; 15],
                                                        &[libc::c_char; 15]>(b"orderDroidBase\x00")).as_ptr(),
                              b"FALSE\x00" as *const u8 as
                                  *const libc::c_char);
                    };
                } else {
                    (*psDroid).order = DORDER_RESTORE as libc::c_int;
                    (*psDroid).orderX = (*(*psOrder).psObj).x;
                    (*psDroid).orderY = (*(*psOrder).psObj).y;
                    (*psDroid).psTarget = (*psOrder).psObj;
                    actionDroidObj(psDroid, DACTION_RESTORE,
                                   (*psOrder).psObj);
                }
            }
        }
        30 => {
            //if (psDroid->droidType != DROID_CONSTRUCT)
            if (*psDroid).droidType as libc::c_uint ==
                   DROID_CONSTRUCT as libc::c_int as libc::c_uint ||
                   (*psDroid).droidType as libc::c_uint ==
                       DROID_CYBORG_CONSTRUCT as libc::c_int as libc::c_uint {
                (*psDroid).order = DORDER_CLEARWRECK as libc::c_int;
                (*psDroid).orderX = (*(*psOrder).psObj).x;
                (*psDroid).orderY = (*(*psOrder).psObj).y;
                (*psDroid).psTarget = (*psOrder).psObj;
                actionDroidObj(psDroid, DACTION_CLEARWRECK, (*psOrder).psObj);
            }
        }
        32 => {
            // didn't get executed before
            if !(vtolDroid(psDroid) == 0) {
                (*psDroid).order = DORDER_REARM as libc::c_int;
                (*psDroid).psTarget = (*psOrder).psObj;
                actionDroidObj(psDroid, DACTION_MOVETOREARM,
                               (*psOrder).psObj);
                assignVTOLPad(psDroid, (*psOrder).psObj as *mut STRUCTURE);
            }
        }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"orderUnitBase: unknown order\x00" as *const u8 as
                          *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"order.c\x00" as *const u8 as *const libc::c_char,
                      2105 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 15],
                                                &[libc::c_char; 15]>(b"orderDroidBase\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
        }
    };
    //	ASSERT( psDroid->x != 0 && psDroid->y != 0,
//		"orderUnitBase: unit at (0,0)" );
}
/* Give a droid an order */
/* Give a droid an order */
#[no_mangle]
pub unsafe extern "C" fn orderDroid(mut psDroid: *mut DROID,
                                    mut order: DROID_ORDER) {
    let mut sOrder: DROID_ORDER_DATA =
        DROID_ORDER_DATA{order: 0,
                         x: 0,
                         y: 0,
                         x2: 0,
                         y2: 0,
                         psObj: 0 as *mut BASE_OBJECT,
                         psStats: 0 as *mut BASE_STATS,};
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"orderUnit: Invalid unit pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"order.c\x00" as *const u8 as *const libc::c_char,
              2120 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 11],
                                        &[libc::c_char; 11]>(b"orderDroid\x00")).as_ptr(),
              b"PTRVALID(psDroid, sizeof(DROID))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if order as libc::c_uint == DORDER_NONE as libc::c_int as libc::c_uint ||
           order as libc::c_uint ==
               DORDER_RETREAT as libc::c_int as libc::c_uint ||
           order as libc::c_uint ==
               DORDER_DESTRUCT as libc::c_int as libc::c_uint ||
           order as libc::c_uint == DORDER_RTR as libc::c_int as libc::c_uint
           ||
           order as libc::c_uint == DORDER_RTB as libc::c_int as libc::c_uint
           ||
           order as libc::c_uint ==
               DORDER_RECYCLE as libc::c_int as libc::c_uint ||
           order as libc::c_uint == DORDER_RUN as libc::c_int as libc::c_uint
           ||
           order as libc::c_uint ==
               DORDER_RUNBURN as libc::c_int as libc::c_uint ||
           order as libc::c_uint ==
               DORDER_TRANSPORTIN as libc::c_int as libc::c_uint ||
           order as libc::c_uint == DORDER_STOP as libc::c_int as libc::c_uint
       {
    } else {
        debug(LOG_ERROR,
              b"orderUnit: Invalid order\x00" as *const u8 as
                  *const libc::c_char);
    };
    if order as libc::c_uint == DORDER_NONE as libc::c_int as libc::c_uint ||
           order as libc::c_uint ==
               DORDER_RETREAT as libc::c_int as libc::c_uint ||
           order as libc::c_uint ==
               DORDER_DESTRUCT as libc::c_int as libc::c_uint ||
           order as libc::c_uint == DORDER_RTR as libc::c_int as libc::c_uint
           ||
           order as libc::c_uint == DORDER_RTB as libc::c_int as libc::c_uint
           ||
           order as libc::c_uint ==
               DORDER_RECYCLE as libc::c_int as libc::c_uint ||
           order as libc::c_uint == DORDER_RUN as libc::c_int as libc::c_uint
           ||
           order as libc::c_uint ==
               DORDER_RUNBURN as libc::c_int as libc::c_uint ||
           order as libc::c_uint ==
               DORDER_TRANSPORTIN as libc::c_int as libc::c_uint ||
           order as libc::c_uint == DORDER_STOP as libc::c_int as libc::c_uint
       {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"order.c\x00" as *const u8 as *const libc::c_char,
              2131 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 11],
                                        &[libc::c_char; 11]>(b"orderDroid\x00")).as_ptr(),
              b"order == DORDER_NONE || order == DORDER_RETREAT || order == DORDER_DESTRUCT || order == DORDER_RTR || order == DORDER_RTB || order == DORDER_RECYCLE || order == DORDER_RUN || order == DORDER_RUNBURN || order == DORDER_TRANSPORTIN || order == DORDER_STOP\x00"
                  as *const u8 as *const libc::c_char);
    };
    memset(&mut sOrder as *mut DROID_ORDER_DATA as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<DROID_ORDER_DATA>() as libc::c_ulong);
    sOrder.order = order as SDWORD;
    orderDroidBase(psDroid, &mut sOrder);
    if bMultiPlayer != 0 {
        SendDroidInfo(psDroid, order, 0 as libc::c_int as UDWORD,
                      0 as libc::c_int as UDWORD, 0 as *mut BASE_OBJECT);
    };
}
/* Check the order state of a droid */
/* Check the order state of a droid */
#[no_mangle]
pub unsafe extern "C" fn orderState(mut psDroid: *mut DROID,
                                    mut order: DROID_ORDER) -> BOOL {
    if order as libc::c_uint == DORDER_RTR as libc::c_int as libc::c_uint {
        return ((*psDroid).order == DORDER_RTR as libc::c_int ||
                    (*psDroid).order == DORDER_RTR_SPECIFIED as libc::c_int)
                   as libc::c_int
    }
    return ((*psDroid).order as libc::c_uint == order as libc::c_uint) as
               libc::c_int;
}
/* Give a droid an order with a location target */
/* Give a droid an order with a location target */
#[no_mangle]
pub unsafe extern "C" fn orderDroidLoc(mut psDroid: *mut DROID,
                                       mut order: DROID_ORDER, mut x: UDWORD,
                                       mut y: UDWORD) {
    let mut sOrder: DROID_ORDER_DATA =
        DROID_ORDER_DATA{order: 0,
                         x: 0,
                         y: 0,
                         x2: 0,
                         y2: 0,
                         psObj: 0 as *mut BASE_OBJECT,
                         psStats: 0 as *mut BASE_STATS,};
    //#ifndef PSX
//	BOOL bTemp = FALSE;
//#endif
//#warning memory report here !!!!!
//	DBPRINTF(("test printf\n"));
//	DBPRINTF(("droid=%p\n",psDroid);
//	memMemoryReport(NULL);
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"orderUnitLoc: Invalid unit pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"order.c\x00" as *const u8 as *const libc::c_char,
              2173 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"orderDroidLoc\x00")).as_ptr(),
              b"PTRVALID(psDroid, sizeof(DROID))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if order as libc::c_uint == DORDER_NONE as libc::c_int as libc::c_uint ||
           order as libc::c_uint == DORDER_MOVE as libc::c_int as libc::c_uint
           ||
           order as libc::c_uint ==
               DORDER_GUARD as libc::c_int as libc::c_uint ||
           order as libc::c_uint ==
               DORDER_SCOUT as libc::c_int as libc::c_uint ||
           order as libc::c_uint == DORDER_RUN as libc::c_int as libc::c_uint
           ||
           order as libc::c_uint ==
               DORDER_PATROL as libc::c_int as libc::c_uint ||
           order as libc::c_uint ==
               DORDER_TRANSPORTOUT as libc::c_int as libc::c_uint ||
           order as libc::c_uint ==
               DORDER_TRANSPORTIN as libc::c_int as libc::c_uint ||
           order as libc::c_uint ==
               DORDER_TRANSPORTRETURN as libc::c_int as libc::c_uint ||
           order as libc::c_uint ==
               DORDER_DISEMBARK as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"orderUnitLoc: Invalid order for location\x00" as *const u8 as
                  *const libc::c_char);
    };
    if order as libc::c_uint == DORDER_NONE as libc::c_int as libc::c_uint ||
           order as libc::c_uint == DORDER_MOVE as libc::c_int as libc::c_uint
           ||
           order as libc::c_uint ==
               DORDER_GUARD as libc::c_int as libc::c_uint ||
           order as libc::c_uint ==
               DORDER_SCOUT as libc::c_int as libc::c_uint ||
           order as libc::c_uint == DORDER_RUN as libc::c_int as libc::c_uint
           ||
           order as libc::c_uint ==
               DORDER_PATROL as libc::c_int as libc::c_uint ||
           order as libc::c_uint ==
               DORDER_TRANSPORTOUT as libc::c_int as libc::c_uint ||
           order as libc::c_uint ==
               DORDER_TRANSPORTIN as libc::c_int as libc::c_uint ||
           order as libc::c_uint ==
               DORDER_TRANSPORTRETURN as libc::c_int as libc::c_uint ||
           order as libc::c_uint ==
               DORDER_DISEMBARK as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"order.c\x00" as *const u8 as *const libc::c_char,
              2184 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"orderDroidLoc\x00")).as_ptr(),
              b"order == DORDER_NONE || order == DORDER_MOVE || order == DORDER_GUARD || order == DORDER_SCOUT || order == DORDER_RUN || order == DORDER_PATROL || order == DORDER_TRANSPORTOUT || order == DORDER_TRANSPORTIN || order == DORDER_TRANSPORTRETURN || order == DORDER_DISEMBARK\x00"
                  as *const u8 as *const libc::c_char);
    };
    orderClearDroidList(psDroid);
    if bMultiPlayer != 0 {
        //ajl
        SendDroidInfo(psDroid, order, x, y, 0 as *mut BASE_OBJECT);
        turnOffMultiMsg(1 as libc::c_int);
        // msgs off.
    }
    memset(&mut sOrder as *mut DROID_ORDER_DATA as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<DROID_ORDER_DATA>() as libc::c_ulong);
    sOrder.order = order as SDWORD;
    sOrder.x = x as UWORD;
    sOrder.y = y as UWORD;
    orderDroidBase(psDroid, &mut sOrder);
    turnOffMultiMsg(0 as libc::c_int);
    //msgs back on..
}
/* Get the state of a droid order with a location */
/* Get the state of a droid order with it's location */
#[no_mangle]
pub unsafe extern "C" fn orderStateLoc(mut psDroid: *mut DROID,
                                       mut order: DROID_ORDER,
                                       mut pX: *mut UDWORD,
                                       mut pY: *mut UDWORD) -> BOOL {
    if order as libc::c_uint != (*psDroid).order as libc::c_uint {
        return 0 as libc::c_int
    }
    // check the order is one with a location
    match (*psDroid).order {
        2 | 11 => {
            *pX = (*psDroid).orderX as UDWORD;
            *pY = (*psDroid).orderY as UDWORD;
            return 1 as libc::c_int
        }
        _ => { }
    }
    return 0 as libc::c_int;
}
/* Give a droid an order with an object target */
/* Give a droid an order with an object target */
#[no_mangle]
pub unsafe extern "C" fn orderDroidObj(mut psDroid: *mut DROID,
                                       mut order: DROID_ORDER,
                                       mut psObj: *mut BASE_OBJECT) {
    let mut sOrder: DROID_ORDER_DATA =
        DROID_ORDER_DATA{order: 0,
                         x: 0,
                         y: 0,
                         x2: 0,
                         y2: 0,
                         psObj: 0 as *mut BASE_OBJECT,
                         psStats: 0 as *mut BASE_STATS,};
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"orderUnitObj: Invalid unit pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"order.c\x00" as *const u8 as *const libc::c_char,
              2238 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"orderDroidObj\x00")).as_ptr(),
              b"PTRVALID(psDroid, sizeof(DROID))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if order as libc::c_uint == DORDER_NONE as libc::c_int as libc::c_uint ||
           order as libc::c_uint ==
               DORDER_HELPBUILD as libc::c_int as libc::c_uint ||
           order as libc::c_uint ==
               DORDER_DEMOLISH as libc::c_int as libc::c_uint ||
           order as libc::c_uint ==
               DORDER_REPAIR as libc::c_int as libc::c_uint ||
           order as libc::c_uint ==
               DORDER_ATTACK as libc::c_int as libc::c_uint ||
           order as libc::c_uint ==
               DORDER_FIRESUPPORT as libc::c_int as libc::c_uint ||
           order as libc::c_uint ==
               DORDER_OBSERVE as libc::c_int as libc::c_uint ||
           order as libc::c_uint ==
               DORDER_ATTACKTARGET as libc::c_int as libc::c_uint ||
           order as libc::c_uint == DORDER_RTR as libc::c_int as libc::c_uint
           ||
           order as libc::c_uint ==
               DORDER_RTR_SPECIFIED as libc::c_int as libc::c_uint ||
           order as libc::c_uint ==
               DORDER_EMBARK as libc::c_int as libc::c_uint ||
           order as libc::c_uint ==
               DORDER_GUARD as libc::c_int as libc::c_uint ||
           order as libc::c_uint ==
               DORDER_DROIDREPAIR as libc::c_int as libc::c_uint ||
           order as libc::c_uint ==
               DORDER_RESTORE as libc::c_int as libc::c_uint ||
           order as libc::c_uint ==
               DORDER_BUILDMODULE as libc::c_int as libc::c_uint ||
           order as libc::c_uint ==
               DORDER_REARM as libc::c_int as libc::c_uint ||
           order as libc::c_uint ==
               DORDER_CLEARWRECK as libc::c_int as libc::c_uint ||
           order as libc::c_uint ==
               DORDER_RECOVER as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"orderUnitObj: Invalid order for object\x00" as *const u8 as
                  *const libc::c_char);
    };
    if order as libc::c_uint == DORDER_NONE as libc::c_int as libc::c_uint ||
           order as libc::c_uint ==
               DORDER_HELPBUILD as libc::c_int as libc::c_uint ||
           order as libc::c_uint ==
               DORDER_DEMOLISH as libc::c_int as libc::c_uint ||
           order as libc::c_uint ==
               DORDER_REPAIR as libc::c_int as libc::c_uint ||
           order as libc::c_uint ==
               DORDER_ATTACK as libc::c_int as libc::c_uint ||
           order as libc::c_uint ==
               DORDER_FIRESUPPORT as libc::c_int as libc::c_uint ||
           order as libc::c_uint ==
               DORDER_OBSERVE as libc::c_int as libc::c_uint ||
           order as libc::c_uint ==
               DORDER_ATTACKTARGET as libc::c_int as libc::c_uint ||
           order as libc::c_uint == DORDER_RTR as libc::c_int as libc::c_uint
           ||
           order as libc::c_uint ==
               DORDER_RTR_SPECIFIED as libc::c_int as libc::c_uint ||
           order as libc::c_uint ==
               DORDER_EMBARK as libc::c_int as libc::c_uint ||
           order as libc::c_uint ==
               DORDER_GUARD as libc::c_int as libc::c_uint ||
           order as libc::c_uint ==
               DORDER_DROIDREPAIR as libc::c_int as libc::c_uint ||
           order as libc::c_uint ==
               DORDER_RESTORE as libc::c_int as libc::c_uint ||
           order as libc::c_uint ==
               DORDER_BUILDMODULE as libc::c_int as libc::c_uint ||
           order as libc::c_uint ==
               DORDER_REARM as libc::c_int as libc::c_uint ||
           order as libc::c_uint ==
               DORDER_CLEARWRECK as libc::c_int as libc::c_uint ||
           order as libc::c_uint ==
               DORDER_RECOVER as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"order.c\x00" as *const u8 as *const libc::c_char,
              2257 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"orderDroidObj\x00")).as_ptr(),
              b"order == DORDER_NONE || order == DORDER_HELPBUILD || order == DORDER_DEMOLISH || order == DORDER_REPAIR || order == DORDER_ATTACK || order == DORDER_FIRESUPPORT || order == DORDER_OBSERVE || order == DORDER_ATTACKTARGET || order == DORDER_RTR || order == DORDER_RTR_SPECIFIED || order == DORDER_EMBARK || order == DORDER_GUARD || order == DORDER_DROIDREPAIR || order == DORDER_RESTORE || order == DORDER_BUILDMODULE || order == DORDER_REARM || order == DORDER_CLEARWRECK || order == DORDER_RECOVER\x00"
                  as *const u8 as *const libc::c_char);
    };
    orderClearDroidList(psDroid);
    if bMultiPlayer != 0 {
        //ajl
        SendDroidInfo(psDroid, order, 0 as libc::c_int as UDWORD,
                      0 as libc::c_int as UDWORD, psObj);
    }
    memset(&mut sOrder as *mut DROID_ORDER_DATA as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<DROID_ORDER_DATA>() as libc::c_ulong);
    sOrder.order = order as SDWORD;
    sOrder.psObj = psObj;
    orderDroidBase(psDroid, &mut sOrder);
}
/* Get the state of a droid order with an object */
/* Get the state of a droid order with an object */
#[no_mangle]
pub unsafe extern "C" fn orderStateObj(mut psDroid: *mut DROID,
                                       mut order: DROID_ORDER,
                                       mut ppsObj: *mut *mut BASE_OBJECT)
 -> BOOL {
    let mut match_0: BOOL = 0 as libc::c_int;
    match order as libc::c_uint {
        4 | 6 | 5 => {
            if (*psDroid).order == DORDER_BUILD as libc::c_int ||
                   (*psDroid).order == DORDER_HELPBUILD as libc::c_int ||
                   (*psDroid).order == DORDER_LINEBUILD as libc::c_int {
                match_0 = 1 as libc::c_int
            }
        }
        3 | 10 | 9 | 7 | 26 | 32 | 25 => {
            if (*psDroid).order as libc::c_uint == order as libc::c_uint {
                match_0 = 1 as libc::c_int
            }
        }
        14 => {
            if (*psDroid).order == DORDER_RTR as libc::c_int ||
                   (*psDroid).order == DORDER_RTR_SPECIFIED as libc::c_int {
                match_0 = 1 as libc::c_int
            }
        }
        _ => { }
    }
    if match_0 == 0 { return 0 as libc::c_int }
    // check the order is one with an object
    match (*psDroid).order {
        4 | 6 => {
            if (*psDroid).action == DACTION_BUILD as libc::c_int ||
                   (*psDroid).action == DACTION_BUILDWANDER as libc::c_int {
                *ppsObj = (*psDroid).psTarget;
                return 1 as libc::c_int
            }
        }
        5 => {
            if (*psDroid).action == DACTION_BUILD as libc::c_int ||
                   (*psDroid).action == DACTION_BUILDWANDER as libc::c_int ||
                   (*psDroid).action == DACTION_MOVETOBUILD as libc::c_int {
                *ppsObj = (*psDroid).psTarget;
                return 1 as libc::c_int
            }
        }
        3 | 10 | 9 | 7 | 14 | 37 | 26 | 32 | 25 => {
            //case DORDER_HELPBUILD:
            *ppsObj = (*psDroid).psTarget;
            return 1 as libc::c_int
        }
        _ => {
            // not an object order - return false
            return 0 as libc::c_int
        }
    }
    return 0 as libc::c_int;
}
/* Give a droid an order with a location and a stat */
/* Give a droid an order with a location and a stat */
#[no_mangle]
pub unsafe extern "C" fn orderDroidStatsLoc(mut psDroid: *mut DROID,
                                            mut order: DROID_ORDER,
                                            mut psStats: *mut BASE_STATS,
                                            mut x: UDWORD, mut y: UDWORD) {
    let mut sOrder: DROID_ORDER_DATA =
        DROID_ORDER_DATA{order: 0,
                         x: 0,
                         y: 0,
                         x2: 0,
                         y2: 0,
                         psObj: 0 as *mut BASE_OBJECT,
                         psStats: 0 as *mut BASE_STATS,};
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"orderUnitStatsLoc: Invalid unit pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"order.c\x00" as *const u8 as *const libc::c_char,
              2370 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 19],
                                        &[libc::c_char; 19]>(b"orderDroidStatsLoc\x00")).as_ptr(),
              b"PTRVALID(psDroid, sizeof(DROID))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if order as libc::c_uint == DORDER_BUILD as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"orderUnitStatsLoc: Invalid order for location\x00" as
                  *const u8 as *const libc::c_char);
    };
    if order as libc::c_uint == DORDER_BUILD as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"order.c\x00" as *const u8 as *const libc::c_char,
              2372 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 19],
                                        &[libc::c_char; 19]>(b"orderDroidStatsLoc\x00")).as_ptr(),
              b"order == DORDER_BUILD\x00" as *const u8 as
                  *const libc::c_char);
    };
    //	Right, now due to some extra special weirdness, we must enusure that the coordinates
	//	are snapped to tile EDGE coordinates, so we need to mask off the bottom 6 bits
	//  This is because feature coordinates might eb coming in for a build order at tile centre
	//	coordinates, but the build order requires tile EDGE coords.
	// NO MORE, GET YE HENCE - JOHN
//	x = x & (~0x7f);
//	y = y & (~0x7f);
    memset(&mut sOrder as *mut DROID_ORDER_DATA as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<DROID_ORDER_DATA>() as libc::c_ulong);
    sOrder.order = order as SDWORD;
    sOrder.x = x as UWORD;
    sOrder.y = y as UWORD;
    sOrder.psStats = psStats;
    orderDroidBase(psDroid, &mut sOrder);
}
/* add an order with a location and a stat to the droids order list*/
/* add an order with a location and a stat to the droids order list*/
#[no_mangle]
pub unsafe extern "C" fn orderDroidStatsLocAdd(mut psDroid: *mut DROID,
                                               mut order: DROID_ORDER,
                                               mut psStats: *mut BASE_STATS,
                                               mut x: UDWORD, mut y: UDWORD) {
    let mut sOrder: DROID_ORDER_DATA =
        DROID_ORDER_DATA{order: 0,
                         x: 0,
                         y: 0,
                         x2: 0,
                         y2: 0,
                         psObj: 0 as *mut BASE_OBJECT,
                         psStats: 0 as *mut BASE_STATS,};
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"orderUnitStatsLoc: Invalid unit pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"order.c\x00" as *const u8 as *const libc::c_char,
              2398 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 22],
                                        &[libc::c_char; 22]>(b"orderDroidStatsLocAdd\x00")).as_ptr(),
              b"PTRVALID(psDroid, sizeof(DROID))\x00" as *const u8 as
                  *const libc::c_char);
    };
    // can only queue build orders with this function
    if order as libc::c_uint != DORDER_BUILD as libc::c_int as libc::c_uint {
        return
    }
    memset(&mut sOrder as *mut DROID_ORDER_DATA as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<DROID_ORDER_DATA>() as libc::c_ulong);
    sOrder.order = order as SDWORD;
    sOrder.x = x as UWORD;
    sOrder.y = y as UWORD;
    sOrder.psStats = psStats;
    orderDroidAdd(psDroid, &mut sOrder);
}
/* Give a droid an order with a location and a stat */
/* Give a droid an order with a location and a stat */
#[no_mangle]
pub unsafe extern "C" fn orderDroidStatsTwoLoc(mut psDroid: *mut DROID,
                                               mut order: DROID_ORDER,
                                               mut psStats: *mut BASE_STATS,
                                               mut x1: UDWORD, mut y1: UDWORD,
                                               mut x2: UDWORD,
                                               mut y2: UDWORD) {
    let mut sOrder: DROID_ORDER_DATA =
        DROID_ORDER_DATA{order: 0,
                         x: 0,
                         y: 0,
                         x2: 0,
                         y2: 0,
                         psObj: 0 as *mut BASE_OBJECT,
                         psStats: 0 as *mut BASE_STATS,};
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"orderUnitStatsTwoLoc: Invalid unit pointer\x00" as *const u8
                  as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"order.c\x00" as *const u8 as *const libc::c_char,
              2422 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 22],
                                        &[libc::c_char; 22]>(b"orderDroidStatsTwoLoc\x00")).as_ptr(),
              b"PTRVALID(psDroid, sizeof(DROID))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if order as libc::c_uint ==
           DORDER_LINEBUILD as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"orderUnitStatsTwoLoc: Invalid order for location\x00" as
                  *const u8 as *const libc::c_char);
    };
    if order as libc::c_uint ==
           DORDER_LINEBUILD as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"order.c\x00" as *const u8 as *const libc::c_char,
              2424 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 22],
                                        &[libc::c_char; 22]>(b"orderDroidStatsTwoLoc\x00")).as_ptr(),
              b"order == DORDER_LINEBUILD\x00" as *const u8 as
                  *const libc::c_char);
    };
    if x1 == x2 || y1 == y2 {
    } else {
        debug(LOG_ERROR,
              b"orderUnitStatsTwoLoc: Invalid locations for LINEBUILD\x00" as
                  *const u8 as *const libc::c_char);
    };
    if x1 == x2 || y1 == y2 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"order.c\x00" as *const u8 as *const libc::c_char,
              2426 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 22],
                                        &[libc::c_char; 22]>(b"orderDroidStatsTwoLoc\x00")).as_ptr(),
              b"x1 == x2 || y1 == y2\x00" as *const u8 as
                  *const libc::c_char);
    };
    memset(&mut sOrder as *mut DROID_ORDER_DATA as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<DROID_ORDER_DATA>() as libc::c_ulong);
    sOrder.order = order as SDWORD;
    sOrder.x = x1 as UWORD;
    sOrder.y = y1 as UWORD;
    sOrder.x2 = x2 as UWORD;
    sOrder.y2 = y2 as UWORD;
    sOrder.psStats = psStats;
    orderDroidBase(psDroid, &mut sOrder);
}
/* Add an order with a location and a stat */
#[no_mangle]
pub unsafe extern "C" fn orderDroidStatsTwoLocAdd(mut psDroid: *mut DROID,
                                                  mut order: DROID_ORDER,
                                                  mut psStats:
                                                      *mut BASE_STATS,
                                                  mut x1: UDWORD,
                                                  mut y1: UDWORD,
                                                  mut x2: UDWORD,
                                                  mut y2: UDWORD) {
    let mut sOrder: DROID_ORDER_DATA =
        DROID_ORDER_DATA{order: 0,
                         x: 0,
                         y: 0,
                         x2: 0,
                         y2: 0,
                         psObj: 0 as *mut BASE_OBJECT,
                         psStats: 0 as *mut BASE_STATS,};
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"orderUnitStatsTwoLocAdd: Invalid unit pointer\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"order.c\x00" as *const u8 as *const libc::c_char,
              2447 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 25],
                                        &[libc::c_char; 25]>(b"orderDroidStatsTwoLocAdd\x00")).as_ptr(),
              b"PTRVALID(psDroid, sizeof(DROID))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if order as libc::c_uint ==
           DORDER_LINEBUILD as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"orderUnitStatsTwoLocAdd: Invalid order for location\x00" as
                  *const u8 as *const libc::c_char);
    };
    if order as libc::c_uint ==
           DORDER_LINEBUILD as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"order.c\x00" as *const u8 as *const libc::c_char,
              2449 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 25],
                                        &[libc::c_char; 25]>(b"orderDroidStatsTwoLocAdd\x00")).as_ptr(),
              b"order == DORDER_LINEBUILD\x00" as *const u8 as
                  *const libc::c_char);
    };
    if x1 == x2 || y1 == y2 {
    } else {
        debug(LOG_ERROR,
              b"orderUnitStatsTwoLocAdd: Invalid locations for LINEBUILD\x00"
                  as *const u8 as *const libc::c_char);
    };
    if x1 == x2 || y1 == y2 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"order.c\x00" as *const u8 as *const libc::c_char,
              2451 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 25],
                                        &[libc::c_char; 25]>(b"orderDroidStatsTwoLocAdd\x00")).as_ptr(),
              b"x1 == x2 || y1 == y2\x00" as *const u8 as
                  *const libc::c_char);
    };
    memset(&mut sOrder as *mut DROID_ORDER_DATA as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<DROID_ORDER_DATA>() as libc::c_ulong);
    sOrder.order = order as SDWORD;
    sOrder.x = x1 as UWORD;
    sOrder.y = y1 as UWORD;
    sOrder.x2 = x2 as UWORD;
    sOrder.y2 = y2 as UWORD;
    sOrder.psStats = psStats;
    orderDroidAdd(psDroid, &mut sOrder);
}
/* Get the state of a droid order with a location and a stat */
/* Get the state of a droid order with a location and a stat */
#[no_mangle]
pub unsafe extern "C" fn orderStateStatsLoc(mut psDroid: *mut DROID,
                                            mut order: DROID_ORDER,
                                            mut ppsStats:
                                                *mut *mut BASE_STATS,
                                            mut pX: *mut UDWORD,
                                            mut pY: *mut UDWORD) -> BOOL {
    let mut match_0: BOOL = 0 as libc::c_int;
    match order as libc::c_uint {
        4 | 6 => {
            if (*psDroid).order == DORDER_BUILD as libc::c_int ||
                   (*psDroid).order == DORDER_LINEBUILD as libc::c_int {
                match_0 = 1 as libc::c_int
            }
        }
        _ => { }
    }
    if match_0 == 0 { return 0 as libc::c_int }
    // check the order is one with stats and a location
    match (*psDroid).order {
        4 | 6 => {
            if (*psDroid).action == DACTION_MOVETOBUILD as libc::c_int ||
                   (*psDroid).action ==
                       DACTION_BUILD_FOUNDATION as libc::c_int ||
                   (*psDroid).action ==
                       DACTION_FOUNDATION_WANDER as libc::c_int {
                *ppsStats = (*psDroid).psTarStats;
                *pX = (*psDroid).orderX as UDWORD;
                *pY = (*psDroid).orderY as UDWORD;
                return 1 as libc::c_int
            }
        }
        _ => {
            // not a stats/location order - return false
            return 0 as libc::c_int
        }
    }
    return 0 as libc::c_int;
}
// whether an order effect has been displayed
#[no_mangle]
pub static mut bOrderEffectDisplayed: BOOL = 0 as libc::c_int;
// add an order to a droids order list
// add an order to a droids order list
#[no_mangle]
pub unsafe extern "C" fn orderDroidAdd(mut psDroid: *mut DROID,
                                       mut psOrder: *mut DROID_ORDER_DATA) {
    let mut position: iVector = iVector{x: 0, y: 0, z: 0,};
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"orderUnitAdd: invalid unit pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"order.c\x00" as *const u8 as *const libc::c_char,
              2523 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"orderDroidAdd\x00")).as_ptr(),
              b"PTRVALID(psDroid, sizeof(DROID))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (*psDroid).listSize >= 10 as libc::c_int {
        // no room to store the order, quit
        return
    }
    // if not doing anything - do it immediately
    if (*psDroid).listSize == 0 as libc::c_int &&
           ((*psDroid).order == DORDER_NONE as libc::c_int ||
                (*psDroid).order == DORDER_GUARD as libc::c_int) {
        orderDroidBase(psDroid, psOrder);
    } else {
        (*psDroid).asOrderList[(*psDroid).listSize as usize].order =
            (*psOrder).order;
        //psDroid->asOrderList[psDroid->listSize].psObj = psOrder->psObj;
        if (*psOrder).order == DORDER_BUILD as libc::c_int ||
               (*psOrder).order == DORDER_LINEBUILD as libc::c_int {
            (*psDroid).asOrderList[(*psDroid).listSize as usize].psOrderTarget
                = (*psOrder).psStats as *mut libc::c_void
        } else {
            (*psDroid).asOrderList[(*psDroid).listSize as usize].psOrderTarget
                = (*psOrder).psObj as *mut libc::c_void
        }
        (*psDroid).asOrderList[(*psDroid).listSize as usize].x = (*psOrder).x;
        (*psDroid).asOrderList[(*psDroid).listSize as usize].y = (*psOrder).y;
        (*psDroid).asOrderList[(*psDroid).listSize as usize].x2 =
            (*psOrder).x2;
        (*psDroid).asOrderList[(*psDroid).listSize as usize].y2 =
            (*psOrder).y2;
        (*psDroid).listSize += 1 as libc::c_int
    }
    //don't display the arrow-effects with build orders since unnecessary
    if bOrderEffectDisplayed == 0 &&
           ((*psOrder).order != DORDER_BUILD as libc::c_int ||
                (*psOrder).order != DORDER_LINEBUILD as libc::c_int ||
                (*psOrder).order != DORDER_BUILDMODULE as libc::c_int ||
                (*psOrder).order != DORDER_HELPBUILD as libc::c_int) {
        position.x = (*psOrder).x as int32;
        position.z = (*psOrder).y as int32;
        position.y =
            map_Height(position.x as UDWORD, position.z as UDWORD) as
                libc::c_int + 32 as libc::c_int;
        if !(*psOrder).psObj.is_null() &&
               !(*(*psOrder).psObj).sDisplay.imd.is_null() {
            position.y += (*(*(*psOrder).psObj).sDisplay.imd).ymax
        }
        addEffect(&mut position, EFFECT_WAYPOINT, WAYPOINT_TYPE,
                  0 as libc::c_int, 0 as *mut iIMDShape, 0 as libc::c_int);
        bOrderEffectDisplayed = 1 as libc::c_int
    };
}
// do the next order from a droids order list
// do the next order from a droids order list
#[no_mangle]
pub unsafe extern "C" fn orderDroidList(mut psDroid: *mut DROID) -> BOOL {
    let mut sOrder: DROID_ORDER_DATA =
        DROID_ORDER_DATA{order: 0,
                         x: 0,
                         y: 0,
                         x2: 0,
                         y2: 0,
                         psObj: 0 as *mut BASE_OBJECT,
                         psStats: 0 as *mut BASE_STATS,};
    if (*psDroid).listSize > 0 as libc::c_int {
        // there are some orders to give
        memset(&mut sOrder as *mut DROID_ORDER_DATA as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<DROID_ORDER_DATA>() as libc::c_ulong);
        sOrder.order =
            (*psDroid).asOrderList[0 as libc::c_int as usize].order;
        //sOrder.psObj = psDroid->asOrderList[0].psObj;
        match (*psDroid).asOrderList[0 as libc::c_int as usize].order {
            2 => { sOrder.psObj = 0 as *mut BASE_OBJECT }
            3 | 8 | 9 | 26 | 10 | 30 | 7 | 5 | 20 => {
                sOrder.psObj =
                    (*psDroid).asOrderList[0 as libc::c_int as
                                               usize].psOrderTarget as
                        *mut BASE_OBJECT
            }
            4 | 6 => {
                sOrder.psObj = 0 as *mut BASE_OBJECT;
                sOrder.psStats =
                    (*psDroid).asOrderList[0 as libc::c_int as
                                               usize].psOrderTarget as
                        *mut BASE_STATS
            }
            _ => {
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"orderDroidList: Invalid order\x00" as *const u8 as
                              *const libc::c_char);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"order.c\x00" as *const u8 as *const libc::c_char,
                          2611 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 15],
                                                    &[libc::c_char; 15]>(b"orderDroidList\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
                return 0 as libc::c_int
            }
        }
        sOrder.x = (*psDroid).asOrderList[0 as libc::c_int as usize].x;
        sOrder.y = (*psDroid).asOrderList[0 as libc::c_int as usize].y;
        sOrder.x2 = (*psDroid).asOrderList[0 as libc::c_int as usize].x2;
        sOrder.y2 = (*psDroid).asOrderList[0 as libc::c_int as usize].y2;
        (*psDroid).listSize -= 1 as libc::c_int;
        // move the rest of the list down
        memmove((*psDroid).asOrderList.as_mut_ptr() as *mut libc::c_void,
                (*psDroid).asOrderList.as_mut_ptr().offset(1 as libc::c_int as
                                                               isize) as
                    *const libc::c_void,
                ((*psDroid).listSize as
                     libc::c_uint).wrapping_mul(::std::mem::size_of::<ORDER_LIST>()
                                                    as libc::c_ulong));
        memset((*psDroid).asOrderList.as_mut_ptr().offset((*psDroid).listSize
                                                              as isize) as
                   *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<ORDER_LIST>() as libc::c_ulong);
        orderDroidBase(psDroid, &mut sOrder);
        //don't send BUILD orders in multiplayer
        if bMultiPlayer != 0 &&
               !(sOrder.order == DORDER_BUILD as libc::c_int ||
                     sOrder.order == DORDER_LINEBUILD as libc::c_int) {
            SendDroidInfo(psDroid, sOrder.order as DROID_ORDER,
                          sOrder.x as UDWORD, sOrder.y as UDWORD,
                          sOrder.psObj);
        }
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
// clear all the orders from the list
// clear all the orders from the list
#[no_mangle]
pub unsafe extern "C" fn orderClearDroidList(mut psDroid: *mut DROID) {
    // ffs je
    (*psDroid).listSize = 0 as libc::c_int;
    memset((*psDroid).asOrderList.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           (::std::mem::size_of::<ORDER_LIST>() as
                libc::c_ulong).wrapping_mul(10 as libc::c_int as
                                                libc::c_uint));
}
// check all the orders in the list for died objects
// check all the orders in the list for died objects
#[no_mangle]
pub unsafe extern "C" fn orderCheckList(mut psDroid: *mut DROID) {
    let mut i: SDWORD = 0;
    i = 0 as libc::c_int;
    while i < (*psDroid).listSize {
        //if (psDroid->asOrderList[i].psObj &&
	    //	(psDroid->asOrderList[i].psObj)->died)
        //if order requires an object
        if (*psDroid).asOrderList[i as usize].order ==
               DORDER_ATTACK as libc::c_int ||
               (*psDroid).asOrderList[i as usize].order ==
                   DORDER_REPAIR as libc::c_int ||
               (*psDroid).asOrderList[i as usize].order ==
                   DORDER_OBSERVE as libc::c_int ||
               (*psDroid).asOrderList[i as usize].order ==
                   DORDER_DROIDREPAIR as libc::c_int ||
               (*psDroid).asOrderList[i as usize].order ==
                   DORDER_FIRESUPPORT as libc::c_int ||
               (*psDroid).asOrderList[i as usize].order ==
                   DORDER_CLEARWRECK as libc::c_int ||
               (*psDroid).asOrderList[i as usize].order ==
                   DORDER_DEMOLISH as libc::c_int ||
               (*psDroid).asOrderList[i as usize].order ==
                   DORDER_HELPBUILD as libc::c_int ||
               (*psDroid).asOrderList[i as usize].order ==
                   DORDER_BUILDMODULE as libc::c_int {
            if !((*psDroid).asOrderList[i as usize].psOrderTarget as
                     *mut BASE_OBJECT).is_null() &&
                   (*((*psDroid).asOrderList[i as usize].psOrderTarget as
                          *mut BASE_OBJECT)).died != 0 {
                // copy any other orders down the stack
                (*psDroid).listSize -= 1 as libc::c_int;
                memmove((*psDroid).asOrderList.as_mut_ptr().offset(i as isize)
                            as *mut libc::c_void,
                        (*psDroid).asOrderList.as_mut_ptr().offset(i as
                                                                       isize).offset(1
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         isize)
                            as *const libc::c_void,
                        (((*psDroid).listSize - i) as
                             libc::c_uint).wrapping_mul(::std::mem::size_of::<ORDER_LIST>()
                                                            as
                                                            libc::c_ulong));
                memset((*psDroid).asOrderList.as_mut_ptr().offset((*psDroid).listSize
                                                                      as
                                                                      isize)
                           as *mut libc::c_void, 0 as libc::c_int,
                       ::std::mem::size_of::<ORDER_LIST>() as libc::c_ulong);
            } else { i += 1 }
        } else { i += 1 }
    };
}
// add a location order to a droids order list
#[no_mangle]
pub unsafe extern "C" fn orderDroidLocAdd(mut psDroid: *mut DROID,
                                          mut order: DROID_ORDER,
                                          mut x: UDWORD, mut y: UDWORD)
 -> BOOL {
    let mut sOrder: DROID_ORDER_DATA =
        DROID_ORDER_DATA{order: 0,
                         x: 0,
                         y: 0,
                         x2: 0,
                         y2: 0,
                         psObj: 0 as *mut BASE_OBJECT,
                         psStats: 0 as *mut BASE_STATS,};
    // can only queue move orders
    if order as libc::c_uint != DORDER_MOVE as libc::c_int as libc::c_uint {
        return 0 as libc::c_int
    }
    memset(&mut sOrder as *mut DROID_ORDER_DATA as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<DROID_ORDER_DATA>() as libc::c_ulong);
    sOrder.order = order as SDWORD;
    sOrder.x = x as UWORD;
    sOrder.y = y as UWORD;
    orderDroidAdd(psDroid, &mut sOrder);
    return 1 as libc::c_int;
}
// add an object order to a droids order list
#[no_mangle]
pub unsafe extern "C" fn orderDroidObjAdd(mut psDroid: *mut DROID,
                                          mut order: DROID_ORDER,
                                          mut psObj: *mut BASE_OBJECT)
 -> BOOL {
    let mut sOrder: DROID_ORDER_DATA =
        DROID_ORDER_DATA{order: 0,
                         x: 0,
                         y: 0,
                         x2: 0,
                         y2: 0,
                         psObj: 0 as *mut BASE_OBJECT,
                         psStats: 0 as *mut BASE_STATS,};
    // check can queue the order
    if order as libc::c_uint != DORDER_ATTACK as libc::c_int as libc::c_uint
           &&
           order as libc::c_uint !=
               DORDER_REPAIR as libc::c_int as libc::c_uint &&
           order as libc::c_uint !=
               DORDER_OBSERVE as libc::c_int as libc::c_uint &&
           order as libc::c_uint !=
               DORDER_DROIDREPAIR as libc::c_int as libc::c_uint &&
           order as libc::c_uint !=
               DORDER_FIRESUPPORT as libc::c_int as libc::c_uint &&
           order as libc::c_uint !=
               DORDER_CLEARWRECK as libc::c_int as libc::c_uint &&
           order as libc::c_uint !=
               DORDER_DEMOLISH as libc::c_int as libc::c_uint &&
           order as libc::c_uint !=
               DORDER_HELPBUILD as libc::c_int as libc::c_uint &&
           order as libc::c_uint !=
               DORDER_BUILDMODULE as libc::c_int as libc::c_uint {
        return 0 as libc::c_int
    }
    memset(&mut sOrder as *mut DROID_ORDER_DATA as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<DROID_ORDER_DATA>() as libc::c_ulong);
    sOrder.order = order as SDWORD;
    sOrder.psObj = psObj;
    sOrder.x = (*psObj).x;
    sOrder.y = (*psObj).y;
    orderDroidAdd(psDroid, &mut sOrder);
    return 1 as libc::c_int;
}
/* Choose an order for a droid from a location */
#[no_mangle]
pub unsafe extern "C" fn chooseOrderLoc(mut psDroid: *mut DROID,
                                        mut x: UDWORD, mut y: UDWORD)
 -> DROID_ORDER {
    let mut order: DROID_ORDER = DORDER_NONE;
    let mut state: SECONDARY_STATE = 0 as SECONDARY_STATE;
    // default to move
    order = DORDER_MOVE;
    /*if(psDroid->droidType == DROID_TRANSPORTER)
	{
		order = DORDER_NONE;
	}*/
	//VTOL Droids won't move to a location - only to a target object -
	//this cope with Transporters as well - not any more! AB 16/11/98
//	if (vtolDroid(psDroid) OR psDroid->droidType == DROID_TRANSPORTER)
    //and now we want Transporters to fly! - in multiPlayer!!
    if (*psDroid).droidType as libc::c_uint ==
           DROID_TRANSPORTER as libc::c_int as libc::c_uint {
        //if (!bMultiPlayer)
        if game.maxPlayers as libc::c_int == 0 as libc::c_int {
            order = DORDER_NONE
        } else if keyDown(KEY_LALT) != 0 || keyDown(KEY_RALT) != 0 {
            order = DORDER_DISEMBARK
        }
    } else if secondaryGetState(psDroid, DSO_PATROL, &mut state) != 0 &&
                  state as libc::c_uint ==
                      DSS_PATROL_SET as libc::c_int as libc::c_uint {
        order = DORDER_PATROL;
        secondarySetState(psDroid, DSO_PATROL, 0 as SECONDARY_STATE);
    }
    return order;
}
/*in MultiPlayer - if ALT-key is pressed then need to get the Transporter
        to fly to location and all units disembark*/
/* Give selected droids an order from a location target or
   move selected Delivery Point to new location

   If add is true then the order is queued in the droid
*/
#[no_mangle]
pub unsafe extern "C" fn orderSelectedLocAdd(mut player: UDWORD,
                                             mut x: UDWORD, mut y: UDWORD,
                                             mut add: BOOL) {
    let mut psCurr: *mut DROID = 0 as *mut DROID; //, *psPrev;
    let mut order: DROID_ORDER = DORDER_NONE;
    //	FORMATION		*psFormation = NULL;
    //	DBPRINTF(("orderSelectedLoc: player %d -> (%d,%d)\n", player, x,y));
    //if were in build select mode ignore all other clicking
    if intBuildSelectMode() != 0 { return }
    if add == 0 && bMultiPlayer != 0 &&
           SendGroupOrderSelected(player as UBYTE, x, y,
                                  0 as *mut BASE_OBJECT) != 0 {
        // turn off multiplay messages,since we've send a group one instead.
        turnOffMultiMsg(1 as libc::c_int);
    }
    // remove any units from their command group
    psCurr = apsDroidLists[player as usize];
    while !psCurr.is_null() {
        if (*psCurr).selected as libc::c_int != 0 &&
               (*psCurr).droidType as libc::c_uint !=
                   DROID_COMMAND as libc::c_int as libc::c_uint &&
               !(*psCurr).psGroup.is_null() &&
               (*(*psCurr).psGroup).type_0 as libc::c_int ==
                   GT_COMMAND as libc::c_int {
            grpLeave((*psCurr).psGroup, psCurr);
        }
        psCurr = (*psCurr).psNext
    }
    // note that an order list graphic needs to be displayed
    bOrderEffectDisplayed = 0 as libc::c_int;
    //	psPrev = NULL;
    psCurr = apsDroidLists[player as usize];
    while !psCurr.is_null() {
        if (*psCurr).selected != 0 {
            order = chooseOrderLoc(psCurr, x, y);
            // see if the order can be added to the list
            if !(add != 0 && orderDroidLocAdd(psCurr, order, x, y) != 0) {
                // if not just do it straight off
                orderDroidLoc(psCurr, order, x, y);
            }
        }
        psCurr = (*psCurr).psNext
    }
    //might be a Delivery Point of a factory
	//processDeliveryPoint(player,x,y);
    turnOffMultiMsg(0 as libc::c_int);
    // msgs back on...
}
/* Give selected droids an order with a location target */
#[no_mangle]
pub unsafe extern "C" fn orderSelectedLoc(mut player: UDWORD, mut x: UDWORD,
                                          mut y: UDWORD) {
    orderSelectedLocAdd(player, x, y, 0 as libc::c_int);
}
/* Choose an order for a droid from an object */
#[no_mangle]
pub unsafe extern "C" fn chooseOrderObj(mut psDroid: *mut DROID,
                                        mut psObj: *mut BASE_OBJECT)
 -> DROID_ORDER {
    let mut order: DROID_ORDER = DORDER_NONE;
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psFeature: *mut FEATURE = 0 as *mut FEATURE;
    if (*psDroid).droidType as libc::c_uint ==
           DROID_TRANSPORTER as libc::c_int as libc::c_uint {
        //in multiPlayer, need to be able to get Transporter repaired
        if bMultiPlayer != 0 {
            //default to no order
            order = DORDER_NONE;
            if (*psObj).player as libc::c_int ==
                   (*psDroid).player as libc::c_int &&
                   (*psObj).type_0 as libc::c_uint ==
                       OBJ_STRUCTURE as libc::c_int as libc::c_uint {
                psStruct = psObj as *mut STRUCTURE;
                if 1 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"chooseOrderObj: invalid structure pointer\x00" as
                              *const u8 as *const libc::c_char);
                };
                if 1 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"order.c\x00" as *const u8 as *const libc::c_char,
                          2875 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 15],
                                                    &[libc::c_char; 15]>(b"chooseOrderObj\x00")).as_ptr(),
                          b"PTRVALID(psObj, sizeof(STRUCTURE))\x00" as
                              *const u8 as *const libc::c_char);
                };
                if (*(*psStruct).pStructureType).type_0 ==
                       REF_REPAIR_FACILITY as libc::c_int as libc::c_uint &&
                       (*psStruct).status as libc::c_int ==
                           SS_BUILT as libc::c_int {
                    order = DORDER_RTR_SPECIFIED
                }
            }
            return order
        } else { return DORDER_NONE }
    }
    //check for transporters first
    if (*psObj).type_0 as libc::c_uint ==
           OBJ_DROID as libc::c_int as libc::c_uint &&
           (*(psObj as *mut DROID)).droidType as libc::c_uint ==
               DROID_TRANSPORTER as libc::c_int as libc::c_uint &&
           (*psObj).player as libc::c_int == (*psDroid).player as libc::c_int
       {
        order = DORDER_EMBARK
        //Cannot test this here since bMultiPlayer will have been set to FALSE
/*        //in multiPlayer can only put cyborgs onto a Transporter
        if (bMultiPlayer AND psDroid->droidType != DROID_CYBORG)
        {
            order = DORDER_NONE;
        }
*/
    } else if (*psObj).type_0 as libc::c_uint ==
                  OBJ_FEATURE as libc::c_int as libc::c_uint &&
                  ((*(*(psObj as *mut FEATURE)).psStats).subType as
                       libc::c_uint ==
                       FEAT_GEN_ARTE as libc::c_int as libc::c_uint ||
                       (*(*(psObj as *mut FEATURE)).psStats).subType as
                           libc::c_uint ==
                           FEAT_OIL_DRUM as libc::c_int as libc::c_uint) {
        if vtolDroid(psDroid) != 0 {
            order = DORDER_NONE
        } else { order = DORDER_RECOVER }
    } else if (*psDroid).asWeaps[0 as libc::c_int as usize].nStat >
                  0 as libc::c_int as libc::c_uint &&
                  (*psObj).player as libc::c_int !=
                      (*psDroid).player as libc::c_int &&
                  aiCheckAlliances((*psObj).player as UDWORD,
                                   (*psDroid).player as UDWORD) == 0 {
        // go to recover an artifact/oil drum - don't allow VTOL's to get this order
        // else default to attack if the droid has a weapon
	//else if (psDroid->numWeaps > 0
        //check valid weapon/prop combination
        if validTarget(psDroid as *mut BASE_OBJECT, psObj) == 0 {
            order = DORDER_NONE
        } else {
            //check vtol not empty of ammo - handled in DORDER_ATTACK
        //else if (vtolEmpty(psDroid))
		//{
		//	order = DORDER_NONE;
		//}
            order = DORDER_ATTACK
        }
    } else if (*psDroid).droidType as libc::c_uint ==
                  DROID_SENSOR as libc::c_int as libc::c_uint &&
                  (*psObj).player as libc::c_int !=
                      (*psDroid).player as libc::c_int &&
                  aiCheckAlliances((*psObj).player as UDWORD,
                                   (*psDroid).player as UDWORD) == 0 {
        //check for standard sensor or VTOL intercept sensor
        if (*asSensorStats.offset((*psDroid).asBits[COMP_SENSOR as libc::c_int
                                                        as usize].nStat as
                                      isize)).type_0 as libc::c_uint ==
               STANDARD_SENSOR as libc::c_int as libc::c_uint ||
               (*asSensorStats.offset((*psDroid).asBits[COMP_SENSOR as
                                                            libc::c_int as
                                                            usize].nStat as
                                          isize)).type_0 as libc::c_uint ==
                   VTOL_INTERCEPT_SENSOR as libc::c_int as libc::c_uint ||
               (*asSensorStats.offset((*psDroid).asBits[COMP_SENSOR as
                                                            libc::c_int as
                                                            usize].nStat as
                                          isize)).type_0 as libc::c_uint ==
                   SUPER_SENSOR as libc::c_int as libc::c_uint {
            // a sensor droid observing an object
            order = DORDER_OBSERVE
        } else { order = DORDER_NONE }
    } else if droidSensorDroidWeapon(psObj, psDroid) != 0 {
        // got an indirect weapon droid or vtol doing fire support
        order = DORDER_FIRESUPPORT;
        setSensorAssigned();
    } else if (*psObj).player as libc::c_int ==
                  (*psDroid).player as libc::c_int &&
                  (*psObj).type_0 as libc::c_uint ==
                      OBJ_DROID as libc::c_int as libc::c_uint &&
                  (*(psObj as *mut DROID)).droidType as libc::c_uint ==
                      DROID_COMMAND as libc::c_int as libc::c_uint &&
                  (*psDroid).droidType as libc::c_uint !=
                      DROID_COMMAND as libc::c_int as libc::c_uint &&
                  (*psDroid).droidType as libc::c_uint !=
                      DROID_CONSTRUCT as libc::c_int as libc::c_uint &&
                  (*psDroid).droidType as libc::c_uint !=
                      DROID_CYBORG_CONSTRUCT as libc::c_int as libc::c_uint {
        //		if (!vtolDroid(psDroid))
        // get a droid to join a command droids group
        cmdDroidAddDroid(psObj as *mut DROID, psDroid);
        DeSelectDroid(psDroid);
        order = DORDER_NONE
    } else if (*psObj).player as libc::c_int ==
                  (*psDroid).player as libc::c_int &&
                  (*psObj).type_0 as libc::c_uint ==
                      OBJ_DROID as libc::c_int as libc::c_uint &&
                  ((*psDroid).droidType as libc::c_uint ==
                       DROID_REPAIR as libc::c_int as libc::c_uint ||
                       (*psDroid).droidType as libc::c_uint ==
                           DROID_CYBORG_REPAIR as libc::c_int as libc::c_uint)
                  && droidIsDamaged(psObj as *mut DROID) != 0 {
        order = DORDER_DROIDREPAIR
    } else if (*psObj).player as libc::c_int ==
                  (*psDroid).player as libc::c_int &&
                  (*psObj).type_0 as libc::c_uint ==
                      OBJ_DROID as libc::c_int as libc::c_uint &&
                  ((*(psObj as *mut DROID)).droidType as libc::c_uint ==
                       DROID_CONSTRUCT as libc::c_int as libc::c_uint ||
                       (*(psObj as *mut DROID)).droidType as libc::c_uint ==
                           DROID_CYBORG_CONSTRUCT as libc::c_int as
                               libc::c_uint ||
                       (*(psObj as *mut DROID)).droidType as libc::c_uint ==
                           DROID_SENSOR as libc::c_int as libc::c_uint) &&
                  ((*psDroid).droidType as libc::c_uint ==
                       DROID_WEAPON as libc::c_int as libc::c_uint ||
                       (*psDroid).droidType as libc::c_uint ==
                           DROID_CYBORG as libc::c_int as libc::c_uint) &&
                  proj_Direct(asWeaponStats.offset((*psDroid).asWeaps[0 as
                                                                          libc::c_int
                                                                          as
                                                                          usize].nStat
                                                       as isize)) != 0 {
        order = DORDER_GUARD;
        assignSensorTarget(psObj);
        (*psDroid).selected = 0 as libc::c_int as UBYTE
    } else if (*psObj).player as libc::c_int ==
                  (*psDroid).player as libc::c_int &&
                  (*psObj).type_0 as libc::c_uint ==
                      OBJ_STRUCTURE as libc::c_int as libc::c_uint {
        psStruct = psObj as *mut STRUCTURE;
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"chooseOrderObj: invalid structure pointer\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"order.c\x00" as *const u8 as *const libc::c_char,
                  3011 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 15],
                                            &[libc::c_char; 15]>(b"chooseOrderObj\x00")).as_ptr(),
                  b"PTRVALID(psObj, sizeof(STRUCTURE))\x00" as *const u8 as
                      *const libc::c_char);
        };
        /*		else
		{
			order = DORDER_FIRESUPPORT;
		}*/
        //repair droid
        // guarding constructor droids
        /* check whether construction droid */
        order = DORDER_NONE;
        if (*psDroid).droidType as libc::c_uint ==
               DROID_CONSTRUCT as libc::c_int as libc::c_uint ||
               (*psDroid).droidType as libc::c_uint ==
                   DROID_CYBORG_CONSTRUCT as libc::c_int as libc::c_uint {
            //check for demolish first
			//if ((psDroid->psTarget == NULL AND
            //    psDroid->psTarStats == (BASE_STATS *) structGetDemolishStat())
            //Re-written to allow demolish order to be added to the queuing system
            if intDemolishSelectMode() != 0 {
                //check to see if anything is currently trying to build the structure
				//can't build and demolish at the same time!
                if (*psStruct).status as libc::c_int !=
                       SS_BUILT as libc::c_int &&
                       checkDroidsBuilding(psStruct) != 0 {
                    order = DORDER_NONE;
                    (*psDroid).psTarStats = 0 as *mut _base_stats
                } else { order = DORDER_DEMOLISH }
            } else if (*psStruct).status as libc::c_int !=
                          SS_BUILT as libc::c_int {
                //check for non complete structures
                //if something else is demolishing, then help demolish
                if checkDroidsDemolishing(psStruct) != 0 {
                    (*psDroid).psTarStats =
                        structGetDemolishStat() as *mut BASE_STATS;
                    order = DORDER_DEMOLISH
                } else {
                    //else help build
                    order = DORDER_HELPBUILD
                }
            } else if ((*psStruct).body as libc::c_uint) <
                          structureBody(psStruct) {
                order = DORDER_REPAIR
            } else if buildModule(psDroid, psStruct, 1 as libc::c_int) != 0 {
                order = DORDER_BUILDMODULE
            } else { order = DORDER_NONE }
        }
        if order as libc::c_uint == DORDER_NONE as libc::c_int as libc::c_uint
           {
            //check for half built structure
			/*else if ( psStruct->status == SS_BEING_BUILT)
			{
				// got a construction droid building a structure
				order = DORDER_HELPBUILD;
			}*/
			//else if ( psStruct->body < psStruct->baseBodyPoints )
            //check if can build a module
            /* check repair facility and in need of repair */
            if (*(*psStruct).pStructureType).type_0 ==
                   REF_REPAIR_FACILITY as libc::c_int as libc::c_uint &&
                   (*psStruct).status as libc::c_int ==
                       SS_BUILT as libc::c_int {
                //			((SDWORD)(PERCENT(psDroid->body,psDroid->originalBody)) < 100) )
                order = DORDER_RTR_SPECIFIED
            } else if electronicDroid(psDroid) != 0 &&
                          ((*psStruct).resistance as libc::c_int) <
                              structureResistance((*psStruct).pStructureType,
                                                  (*psStruct).player) as
                                  SDWORD {
                order = DORDER_RESTORE
            } else if structSensorDroidWeapon(psStruct, psDroid) != 0 {
                //check for counter battery assignment
                //				secondarySetState(psDroid, DSO_HALTTYPE, DSS_HALT_HOLD);
                order = DORDER_FIRESUPPORT;
                //inform display system
                setSensorAssigned();
                //deselect droid
	//			psDroid->selected = FALSE;
                DeSelectDroid(psDroid);
            } else if vtolDroid(psDroid) != 0 {
                //REARM VTOLS
                //default to no order
                order = DORDER_NONE;
                //check if rearm pad
                if (*(*psStruct).pStructureType).type_0 ==
                       REF_REARM_PAD as libc::c_int as libc::c_uint {
                    //don't bother checking cos we want it to go there if directed
					//check if need to be rearmed/repaired
					//if (!vtolHappy(psDroid))
                    order = DORDER_REARM
                }
            } else { order = DORDER_GUARD }
        }
    } else if ((*psDroid).droidType as libc::c_uint ==
                   DROID_CONSTRUCT as libc::c_int as libc::c_uint ||
                   (*psDroid).droidType as libc::c_uint ==
                       DROID_CYBORG_CONSTRUCT as libc::c_int as libc::c_uint)
                  &&
                  (*psObj).type_0 as libc::c_uint ==
                      OBJ_FEATURE as libc::c_int as libc::c_uint {
        psFeature = psObj as *mut FEATURE;
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"chooseOrderObj: invalid feature pointer\x00" as *const u8
                      as *const libc::c_char);
        };
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"order.c\x00" as *const u8 as *const libc::c_char,
                  3130 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 15],
                                            &[libc::c_char; 15]>(b"chooseOrderObj\x00")).as_ptr(),
                  b"PTRVALID(psObj, sizeof(FEATURE))\x00" as *const u8 as
                      *const libc::c_char);
        };
        if (*(*psFeature).psStats).subType as libc::c_uint ==
               FEAT_BUILD_WRECK as libc::c_int as libc::c_uint {
            order = DORDER_CLEARWRECK
        } else { order = DORDER_NONE }
    } else { order = DORDER_NONE }
    return order;
}
#[no_mangle]
pub unsafe extern "C" fn orderPlayOrderObjAudio(mut player: UDWORD,
                                                mut psObj: *mut BASE_OBJECT) {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    //check for constructor droid clearing up wrecked buildings
    /* loop over selected droids */
    psDroid = apsDroidLists[player as usize];
    while !psDroid.is_null() {
        if (*psDroid).selected != 0 {
            /* currently only looks for VTOL */
            if vtolDroid(psDroid) != 0 {
                match (*psDroid).order {
                    3 => {
                        audio_QueueTrack(ID_SOUND_ON_OUR_WAY2 as libc::c_int);
                    }
                    _ => { }
                }
            }
            break ;
        } else { psDroid = (*psDroid).psNext }
    };
}
/* Give selected droids an order from an object target
 * If add is true the order is queued with the droid
 */
#[no_mangle]
pub unsafe extern "C" fn orderSelectedObjAdd(mut player: UDWORD,
                                             mut psObj: *mut BASE_OBJECT,
                                             mut add: BOOL) {
    let mut psCurr: *mut DROID = 0 as *mut DROID;
    let mut psDemolish: *mut DROID = 0 as *mut DROID;
    let mut order: DROID_ORDER = DORDER_NONE;
    if add == 0 && bMultiPlayer != 0 &&
           SendGroupOrderSelected(player as UBYTE, 0 as libc::c_int as UDWORD,
                                  0 as libc::c_int as UDWORD, psObj) != 0 {
        // turn off multiplay messages,since we've send a group one instead.
        turnOffMultiMsg(1 as libc::c_int);
    }
    // remove any units from their command group
    psCurr = apsDroidLists[player as usize];
    while !psCurr.is_null() {
        if (*psCurr).selected as libc::c_int != 0 &&
               (*psCurr).droidType as libc::c_uint !=
                   DROID_COMMAND as libc::c_int as libc::c_uint &&
               !(*psCurr).psGroup.is_null() &&
               (*(*psCurr).psGroup).type_0 as libc::c_int ==
                   GT_COMMAND as libc::c_int {
            grpLeave((*psCurr).psGroup, psCurr);
        }
        psCurr = (*psCurr).psNext
    }
    // note that an order list graphic needs to be displayed
    bOrderEffectDisplayed = 0 as libc::c_int;
    psDemolish = 0 as *mut DROID;
    psCurr = apsDroidLists[player as usize];
    while !psCurr.is_null() {
        if (*psCurr).selected != 0 {
            order = chooseOrderObj(psCurr, psObj);
            if order as libc::c_uint ==
                   DORDER_DEMOLISH as libc::c_int as libc::c_uint &&
                   player == selectedPlayer {
                psDemolish = psCurr
            }
            // see if the order can be added to the list
            if !(add != 0 && orderDroidObjAdd(psCurr, order, psObj) != 0) {
                // if not just do it straight off
                orderDroidObj(psCurr, order, psObj); //msgs back on.
            }
        }
        psCurr = (*psCurr).psNext
    }
    orderPlayOrderObjAudio(player, psObj);
    turnOffMultiMsg(0 as libc::c_int);
    //This feels like the wrong place but it has to be done once the order has been received...
    //demolish queuing...need to bring the interface back up
    if !psDemolish.is_null() {
        /*this will stop the constructor being able to demolish any other
        buildings until the demolish button is re-selected*/
        intDemolishCancel();
        //turn off the build queue availability until desired release date!
        //re-add the stat (side) interface to allow a new selection
        if ctrlShiftDown() != 0 { intConstructorSelected(psDemolish); }
    };
}
/* Give selected droids a new waypoint to add to move*/
//extern void orderSelectedWaypoint(UDWORD player, UDWORD x, UDWORD y);
//extern BOOL orderAddWayPoint(DROID *psDroid ,UDWORD dX,UDWORD dY);
/* Give selected droids an order with an object target */
#[no_mangle]
pub unsafe extern "C" fn orderSelectedObj(mut player: UDWORD,
                                          mut psObj: *mut BASE_OBJECT) {
    orderSelectedObjAdd(player, psObj, 0 as libc::c_int);
}
/* order all selected droids with a location and a stat */
/* order all selected droids with a location and a stat */
#[no_mangle]
pub unsafe extern "C" fn orderSelectedStatsLoc(mut player: UDWORD,
                                               mut order: DROID_ORDER,
                                               mut psStats: *mut BASE_STATS,
                                               mut x: UDWORD, mut y: UDWORD,
                                               mut add: BOOL) {
    let mut psCurr: *mut DROID = 0 as *mut DROID;
    //turn off the build queue availability until desired release date!
    psCurr = apsDroidLists[player as usize];
    while !psCurr.is_null() {
        if (*psCurr).selected != 0 {
            if add != 0 {
                orderDroidStatsLocAdd(psCurr, order, psStats, x, y);
            } else { orderDroidStatsLoc(psCurr, order, psStats, x, y); }
        }
        psCurr = (*psCurr).psNext
    };
}
/* order all selected droids with two a locations and a stat */
/* order all selected droids with two a locations and a stat */
#[no_mangle]
pub unsafe extern "C" fn orderSelectedStatsTwoLoc(mut player: UDWORD,
                                                  mut order: DROID_ORDER,
                                                  mut psStats:
                                                      *mut BASE_STATS,
                                                  mut x1: UDWORD,
                                                  mut y1: UDWORD,
                                                  mut x2: UDWORD,
                                                  mut y2: UDWORD,
                                                  mut add: BOOL) {
    let mut psCurr: *mut DROID = 0 as *mut DROID;
    //turn off the build queue availability until desired release date!
    psCurr = apsDroidLists[player as usize];
    while !psCurr.is_null() {
        if (*psCurr).selected != 0 {
            if add != 0 {
                orderDroidStatsTwoLocAdd(psCurr, order, psStats, x1, y1, x2,
                                         y2);
            } else {
                orderDroidStatsTwoLoc(psCurr, order, psStats, x1, y1, x2, y2);
            }
        }
        psCurr = (*psCurr).psNext
    };
}
// See if the player has access to a transporter in this map.
//
#[no_mangle]
pub unsafe extern "C" fn FindATransporter() -> *mut DROID {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    psDroid = apsDroidLists[selectedPlayer as usize];
    while !psDroid.is_null() {
        if (*psDroid).droidType as libc::c_uint ==
               DROID_TRANSPORTER as libc::c_int as libc::c_uint {
            return psDroid
        }
        psDroid = (*psDroid).psNext
    }
    return 0 as *mut DROID;
}
// See if the player has access to a factory in this map.
//
#[no_mangle]
pub unsafe extern "C" fn FindAFactory(mut player: UDWORD,
                                      mut factoryType: UDWORD)
 -> *mut STRUCTURE {
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    if player < 8 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"FindAFactory: invalid player number\x00" as *const u8 as
                  *const libc::c_char);
    };
    if player < 8 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"order.c\x00" as *const u8 as *const libc::c_char,
              3332 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 13],
                                        &[libc::c_char; 13]>(b"FindAFactory\x00")).as_ptr(),
              b"player < MAX_PLAYERS\x00" as *const u8 as
                  *const libc::c_char);
    };
    psStruct = apsStructLists[player as usize];
    while !psStruct.is_null() {
        if (*(*psStruct).pStructureType).type_0 == factoryType {
            return psStruct
        }
        psStruct = (*psStruct).psNext
    }
    return 0 as *mut STRUCTURE;
}
// See if the player has access to a repair facility in this map.
//
#[no_mangle]
pub unsafe extern "C" fn FindARepairFacility() -> *mut STRUCTURE {
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    psStruct = apsStructLists[selectedPlayer as usize];
    while !psStruct.is_null() {
        if (*(*psStruct).pStructureType).type_0 ==
               REF_REPAIR_FACILITY as libc::c_int as libc::c_uint {
            return psStruct
        }
        psStruct = (*psStruct).psNext
    }
    return 0 as *mut STRUCTURE;
}
// see if a droid supports a secondary order
// see if a droid supports a secondary order
#[no_mangle]
pub unsafe extern "C" fn secondarySupported(mut psDroid: *mut DROID,
                                            mut sec: SECONDARY_ORDER)
 -> BOOL {
    let mut supported: BOOL = 0; // Default to supported.
    supported = 1 as libc::c_int;
    match sec as libc::c_uint {
        3 | 4 | 11 | 5 | 10 => {
            // remove production from a command droid
            if (*psDroid).droidType as libc::c_uint !=
                   DROID_COMMAND as libc::c_int as libc::c_uint {
                supported = 0 as libc::c_int
            }
            if sec as libc::c_uint ==
                   DSO_ASSIGN_PRODUCTION as libc::c_int as libc::c_uint &&
                   FindAFactory((*psDroid).player as UDWORD,
                                REF_FACTORY as libc::c_int as
                                    UDWORD).is_null() ||
                   sec as libc::c_uint ==
                       DSO_ASSIGN_CYBORG_PRODUCTION as libc::c_int as
                           libc::c_uint &&
                       FindAFactory((*psDroid).player as UDWORD,
                                    REF_CYBORG_FACTORY as libc::c_int as
                                        UDWORD).is_null() ||
                   sec as libc::c_uint ==
                       DSO_ASSIGN_VTOL_PRODUCTION as libc::c_int as
                           libc::c_uint &&
                       FindAFactory((*psDroid).player as UDWORD,
                                    REF_VTOL_FACTORY as libc::c_int as
                                        UDWORD).is_null() {
                supported = 0 as libc::c_int
            }
            //don't allow factories to be assigned to commanders during a Limbo Expand mission
            if (sec as libc::c_uint ==
                    DSO_ASSIGN_PRODUCTION as libc::c_int as libc::c_uint ||
                    sec as libc::c_uint ==
                        DSO_ASSIGN_CYBORG_PRODUCTION as libc::c_int as
                            libc::c_uint ||
                    sec as libc::c_uint ==
                        DSO_ASSIGN_VTOL_PRODUCTION as libc::c_int as
                            libc::c_uint) && missionLimboExpand() != 0 {
                supported = 0 as libc::c_int
            }
        }
        0 | 2 => {
            if (*psDroid).droidType as libc::c_uint ==
                   DROID_REPAIR as libc::c_int as libc::c_uint ||
                   (*psDroid).droidType as libc::c_uint ==
                       DROID_CYBORG_REPAIR as libc::c_int as libc::c_uint {
                supported = 0 as libc::c_int
            }
            if (*psDroid).droidType as libc::c_uint ==
                   DROID_CONSTRUCT as libc::c_int as libc::c_uint ||
                   (*psDroid).droidType as libc::c_uint ==
                       DROID_CYBORG_CONSTRUCT as libc::c_int as libc::c_uint {
                supported = 0 as libc::c_int
            }
        }
        1 | 7 | 8 | 9 => { }
        6 => {
            /*	case DSO_RETURN_TO_REPAIR:	// Only if player has got a repair facility.
		if(FindARepairFacility() == NULL) {
			supported = FALSE;
		}
		break;*/
            // Only if player has got a factory.
            if FindAFactory((*psDroid).player as UDWORD,
                            REF_FACTORY as libc::c_int as UDWORD).is_null() &&
                   FindAFactory((*psDroid).player as UDWORD,
                                REF_CYBORG_FACTORY as libc::c_int as
                                    UDWORD).is_null() &&
                   FindAFactory((*psDroid).player as UDWORD,
                                REF_VTOL_FACTORY as libc::c_int as
                                    UDWORD).is_null() &&
                   FindARepairFacility().is_null() {
                supported = 0 as libc::c_int
            }
        }
        _ => {
            /*	case DSO_EMBARK:			// Only if player has got a transporter.
		if(FindATransporter() == NULL) {
			supported = FALSE;
		}
		break;*/
            supported = 0 as libc::c_int
        }
    }
    return supported;
}
// get the state of a secondary order, return FALSE if unsupported
// get the state of a secondary order, return FALSE if unsupported
#[no_mangle]
pub unsafe extern "C" fn secondaryGetState(mut psDroid: *mut DROID,
                                           mut sec: SECONDARY_ORDER,
                                           mut pState: *mut SECONDARY_STATE)
 -> BOOL {
    let mut state: UDWORD = 0;
    state = (*psDroid).secondaryOrder;
    match sec as libc::c_uint {
        0 => {
            *pState =
                (state & 0x3 as libc::c_int as libc::c_uint) as
                    SECONDARY_STATE
        }
        1 => {
            *pState =
                (state & 0xc as libc::c_int as libc::c_uint) as
                    SECONDARY_STATE
        }
        2 => {
            *pState =
                (state & 0x30 as libc::c_int as libc::c_uint) as
                    SECONDARY_STATE
        }
        3 | 4 | 11 => {
            *pState =
                (state & 0x1f07fe00 as libc::c_int as libc::c_uint) as
                    SECONDARY_STATE
        }
        6 => {
            *pState =
                (state & 0x100 as libc::c_int as libc::c_uint) as
                    SECONDARY_STATE
        }
        7 => {
            *pState =
                (state & 0x400000 as libc::c_int as libc::c_uint) as
                    SECONDARY_STATE
        }
        8 => {
            *pState =
                (state & 0xc0 as libc::c_int as libc::c_uint) as
                    SECONDARY_STATE
        }
        9 => {
            *pState =
                (state & 0x380000 as libc::c_int as libc::c_uint) as
                    SECONDARY_STATE
        }
        10 => {
            //		*pState = state & DSS_FIREDES_MASK;
            if cmdDroidGetDesignator((*psDroid).player as UDWORD) == psDroid {
                *pState = DSS_FIREDES_SET
            } else { *pState = 0 as SECONDARY_STATE }
        }
        _ => { *pState = 0 as SECONDARY_STATE }
    }
    return 1 as libc::c_int;
}
// check the damage level of a droid against it's secondary state
// check the damage level of a droid against it's secondary state
#[no_mangle]
pub unsafe extern "C" fn secondaryCheckDamageLevel(mut psDroid: *mut DROID) {
    let mut State: SECONDARY_STATE = 0 as SECONDARY_STATE;
    if secondaryGetState(psDroid, DSO_REPAIR_LEVEL, &mut State) != 0 {
        if State as libc::c_uint ==
               DSS_REPLEV_LOW as libc::c_int as libc::c_uint {
            State = 75 as SECONDARY_STATE
            //repair often
        } else if State as libc::c_uint ==
                      DSS_REPLEV_HIGH as libc::c_int as libc::c_uint {
            State = 50 as SECONDARY_STATE
            // don't repair often.
        } else {
            State = 0 as SECONDARY_STATE
            //never repair
        }
        //don't bother checking if 'do or die'
        if State as libc::c_uint != 0 &&
               (*psDroid).body.wrapping_mul(100 as libc::c_int as
                                                libc::c_uint).wrapping_div((*psDroid).originalBody)
                   as SDWORD as libc::c_uint <= State as libc::c_uint {
            if (*psDroid).selected != 0 { DeSelectDroid(psDroid); }
            if vtolDroid(psDroid) == 0 {
                (*psDroid).group = 0xff as libc::c_int as UBYTE
            }
            /* set return to repair if not on hold */
            if (*psDroid).order != DORDER_RTR as libc::c_int &&
                   (*psDroid).order != DORDER_RTB as libc::c_int &&
                   vtolRearming(psDroid) == 0 {
                if vtolDroid(psDroid) != 0 {
                    moveToRearm(psDroid);
                } else { orderDroid(psDroid, DORDER_RTR); }
            }
        }
    };
}
// set the state of a secondary order, return FALSE if failed.
// set the state of a secondary order, return FALSE if failed.
#[no_mangle]
pub unsafe extern "C" fn secondarySetState(mut psDroid: *mut DROID,
                                           mut sec: SECONDARY_ORDER,
                                           mut State: SECONDARY_STATE)
 -> BOOL {
    let mut CurrState: UDWORD = 0;
    let mut factType: UDWORD = 0;
    let mut prodType: UDWORD = 0;
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut factoryInc: SDWORD = 0;
    let mut order: SDWORD = 0;
    let mut retVal: BOOL = 0;
    let mut bMultiPlayGame: BOOL = 0 as libc::c_int;
    let mut psTransport: *mut DROID = 0 as *mut DROID;
    let mut psCurr: *mut DROID = 0 as *mut DROID;
    let mut psNext: *mut DROID = 0 as *mut DROID;
    if bMultiPlayer != 0 {
        //store the value before overwriting
        bMultiPlayGame = bMultiPlayer;
        sendDroidSecondary(psDroid, sec, State);
        turnOffMultiMsg(1 as libc::c_int);
        // msgs off.
    }
    // set the state for any droids in the command group
    if sec as libc::c_uint != DSO_RECYCLE as libc::c_int as libc::c_uint &&
           (*psDroid).droidType as libc::c_uint ==
               DROID_COMMAND as libc::c_int as libc::c_uint &&
           !(*psDroid).psGroup.is_null() &&
           (*(*psDroid).psGroup).type_0 as libc::c_int ==
               GT_COMMAND as libc::c_int {
        grpSetSecondary((*psDroid).psGroup, sec, State);
    }
    CurrState = (*psDroid).secondaryOrder;
    retVal = 1 as libc::c_int;
    match sec as libc::c_uint {
        0 => {
            CurrState =
                CurrState & !(0x3 as libc::c_int) as libc::c_uint |
                    State as libc::c_uint
        }
        1 => {
            CurrState =
                CurrState & !(0xc as libc::c_int) as libc::c_uint |
                    State as libc::c_uint;
            (*psDroid).secondaryOrder = CurrState;
            secondaryCheckDamageLevel(psDroid);
        }
        2 => {
            CurrState =
                CurrState & !(0x30 as libc::c_int) as libc::c_uint |
                    State as libc::c_uint;
            if State as libc::c_uint ==
                   DSS_ALEV_NEVER as libc::c_int as libc::c_uint {
                if orderState(psDroid, DORDER_ATTACK) != 0 {
                    // ||
                    //					 orderState(psDroid, DORDER_FIRESUPPORT) )
                    // just kill these orders
                    orderDroid(psDroid, DORDER_STOP);
                    if vtolDroid(psDroid) != 0 { moveToRearm(psDroid); }
                } else if orderState(psDroid, DORDER_GUARD) != 0 &&
                              droidAttacking(psDroid) != 0 {
                    // send the unit back to the guard position
                    actionDroid(psDroid, DACTION_NONE);
                } else if orderState(psDroid, DORDER_PATROL) != 0 {
                    // send the unit back to the patrol
                    actionDroidLoc(psDroid, DACTION_RETURNTOPOS,
                                   (*psDroid).actionX, (*psDroid).actionY);
                }
            }
        }
        3 | 4 | 11 => {
            if sec as libc::c_uint ==
                   DSO_ASSIGN_PRODUCTION as libc::c_int as libc::c_uint {
                prodType = REF_FACTORY as libc::c_int as UDWORD
            } else if sec as libc::c_uint ==
                          DSO_ASSIGN_CYBORG_PRODUCTION as libc::c_int as
                              libc::c_uint {
                prodType = REF_CYBORG_FACTORY as libc::c_int as UDWORD
            } else { prodType = REF_VTOL_FACTORY as libc::c_int as UDWORD }
            if (*psDroid).droidType as libc::c_uint ==
                   DROID_COMMAND as libc::c_int as libc::c_uint {
                // look for the factories
                psStruct = apsStructLists[(*psDroid).player as usize];
                while !psStruct.is_null() {
                    factType = (*(*psStruct).pStructureType).type_0;
                    if factType == REF_FACTORY as libc::c_int as libc::c_uint
                           ||
                           factType ==
                               REF_VTOL_FACTORY as libc::c_int as libc::c_uint
                           ||
                           factType ==
                               REF_CYBORG_FACTORY as libc::c_int as
                                   libc::c_uint {
                        factoryInc =
                            (*(*((*psStruct).pFunctionality as
                                     *mut FACTORY)).psAssemblyPoint).factoryInc
                                as SDWORD;
                        if factType ==
                               REF_FACTORY as libc::c_int as libc::c_uint {
                            factoryInc += 9 as libc::c_int
                        } else if factType ==
                                      REF_CYBORG_FACTORY as libc::c_int as
                                          libc::c_uint {
                            factoryInc += 9 as libc::c_int + 5 as libc::c_int
                        } else { factoryInc += 24 as libc::c_int }
                        if CurrState &
                               ((1 as libc::c_int) << factoryInc) as
                                   libc::c_uint == 0 &&
                               State as libc::c_uint &
                                   ((1 as libc::c_int) << factoryInc) as
                                       libc::c_uint != 0 {
                            assignFactoryCommandDroid(psStruct, psDroid);
                            // assign this factory to the command droid
                        } else if prodType == factType &&
                                      CurrState &
                                          ((1 as libc::c_int) << factoryInc)
                                              as libc::c_uint != 0 &&
                                      State as libc::c_uint &
                                          ((1 as libc::c_int) << factoryInc)
                                              as libc::c_uint == 0 {
                            // remove this factory from the command droid
                            assignFactoryCommandDroid(psStruct,
                                                      0 as *mut _droid);
                        }
                    }
                    psStruct = (*psStruct).psNext
                }
                if prodType == REF_FACTORY as libc::c_int as libc::c_uint {
                    CurrState &= !(0x3e00 as libc::c_int) as libc::c_uint
                } else if prodType ==
                              REF_CYBORG_FACTORY as libc::c_int as
                                  libc::c_uint {
                    CurrState &= !(0x7c000 as libc::c_int) as libc::c_uint
                } else {
                    CurrState &= !(0x1f000000 as libc::c_int) as libc::c_uint
                }
                CurrState |=
                    State as libc::c_uint &
                        0x1f07fe00 as libc::c_int as libc::c_uint
            }
        }
        5 => {
            if (*psDroid).droidType as libc::c_uint ==
                   DROID_COMMAND as libc::c_int as libc::c_uint {
                // simply clear the flag - all the factory stuff is done in assignFactoryCommandDroid
                CurrState &=
                    !(State as libc::c_uint &
                          0x1f07fe00 as libc::c_int as libc::c_uint)
            }
        }
        6 => {
            if State as libc::c_uint & 0x100 as libc::c_int as libc::c_uint !=
                   0 {
                if orderState(psDroid, DORDER_RECYCLE) == 0 {
                    orderDroid(psDroid, DORDER_RECYCLE);
                }
                //				CurrState &= ~(DSS_HOLD_SET|DSS_RTB_SET|DSS_RTR_SET|DSS_RECYCLE_SET);
                CurrState &=
                    !(0x380000 as libc::c_int | 0x100 as libc::c_int |
                          0xc0 as libc::c_int) as libc::c_uint;
                CurrState |=
                    (DSS_RECYCLE_SET as libc::c_int |
                         DSS_HALT_GUARD as libc::c_int) as libc::c_uint;
                (*psDroid).group = 0xff as libc::c_int as UBYTE;
                if !(*psDroid).psGroup.is_null() {
                    if (*psDroid).droidType as libc::c_uint ==
                           DROID_COMMAND as libc::c_int as libc::c_uint {
                        // remove all the units from the commanders group
                        psCurr = (*(*psDroid).psGroup).psList;
                        while !psCurr.is_null() {
                            psNext = (*psCurr).psGrpNext;
                            grpLeave((*psCurr).psGroup, psCurr);
                            orderDroid(psCurr, DORDER_STOP);
                            psCurr = psNext
                        }
                    } else if (*(*psDroid).psGroup).type_0 as libc::c_int ==
                                  GT_COMMAND as libc::c_int {
                        grpLeave((*psDroid).psGroup, psDroid);
                    }
                }
            } else {
                if orderState(psDroid, DORDER_RECYCLE) != 0 {
                    orderDroid(psDroid, DORDER_STOP);
                }
                CurrState &= !(0x100 as libc::c_int) as libc::c_uint
            }
        }
        7 => {
            if State as libc::c_uint &
                   DSS_PATROL_SET as libc::c_int as libc::c_uint != 0 {
                CurrState |= DSS_PATROL_SET as libc::c_int as libc::c_uint
            } else { CurrState &= !(0x400000 as libc::c_int) as libc::c_uint }
        }
        8 => {
            match State as libc::c_uint & 0xc0 as libc::c_int as libc::c_uint
                {
                192 => {
                    CurrState &= !(0xc0 as libc::c_int) as libc::c_uint;
                    CurrState |=
                        DSS_HALT_PERSUE as libc::c_int as libc::c_uint;
                    if orderState(psDroid, DORDER_GUARD) != 0 {
                        orderDroid(psDroid, DORDER_STOP);
                    }
                }
                128 => {
                    CurrState &= !(0xc0 as libc::c_int) as libc::c_uint;
                    CurrState |=
                        DSS_HALT_GUARD as libc::c_int as libc::c_uint;
                    orderDroidLoc(psDroid, DORDER_GUARD,
                                  (*psDroid).x as UDWORD,
                                  (*psDroid).y as UDWORD);
                }
                64 => {
                    CurrState &= !(0xc0 as libc::c_int) as libc::c_uint;
                    CurrState |= DSS_HALT_HOLD as libc::c_int as libc::c_uint;
                    if orderState(psDroid, DORDER_FIRESUPPORT) == 0 {
                        orderDroid(psDroid, DORDER_STOP);
                    }
                }
                _ => { }
            }
        }
        9 => {
            if State as libc::c_uint & 0x380000 as libc::c_int as libc::c_uint
                   == 0 as libc::c_int as libc::c_uint {
                if orderState(psDroid, DORDER_RTR) != 0 ||
                       orderState(psDroid, DORDER_RTB) != 0 ||
                       orderState(psDroid, DORDER_EMBARK) != 0 {
                    orderDroid(psDroid, DORDER_STOP);
                }
                CurrState &= !(0x380000 as libc::c_int) as libc::c_uint
            } else {
                order = DORDER_NONE as libc::c_int;
                CurrState &= !(0x380000 as libc::c_int) as libc::c_uint;
                if CurrState & 0xc0 as libc::c_int as libc::c_uint ==
                       DSS_HALT_HOLD as libc::c_int as libc::c_uint {
                    CurrState &= !(0xc0 as libc::c_int) as libc::c_uint;
                    CurrState |= DSS_HALT_GUARD as libc::c_int as libc::c_uint
                }
                match State as libc::c_uint &
                          0x380000 as libc::c_int as libc::c_uint {
                    524288 => {
                        //					if (FindARepairFacility() != NULL)
                        order = DORDER_RTR as libc::c_int;
                        CurrState |=
                            DSS_RTL_REPAIR as libc::c_int as libc::c_uint
                        // can't clear the selection here cos it breaks
						// the secondary order screen
//						psDroid->selected = FALSE;
//						psDroid->group = UBYTE_MAX;
                    }
                    1048576 => {
                        order = DORDER_RTB as libc::c_int;
                        CurrState |=
                            DSS_RTL_BASE as libc::c_int as libc::c_uint
                    }
                    2097152 => {
                        psTransport = FindATransporter();
                        if !psTransport.is_null() {
                            //in multiPlayer can only put cyborgs onto a Transporter
                            if bMultiPlayGame != 0 &&
                                   cyborgDroid(psDroid) == 0 {
                                retVal = 0 as libc::c_int
                            } else {
                                order = DORDER_EMBARK as libc::c_int;
                                CurrState |=
                                    DSS_RTL_TRANSPORT as libc::c_int as
                                        libc::c_uint;
                                if orderState(psDroid, DORDER_EMBARK) == 0 {
                                    orderDroidObj(psDroid, DORDER_EMBARK,
                                                  psTransport as
                                                      *mut BASE_OBJECT);
                                }
                            }
                        } else { retVal = 0 as libc::c_int }
                    }
                    _ => { order = DORDER_NONE as libc::c_int }
                }
                if orderState(psDroid, order as DROID_ORDER) == 0 {
                    orderDroid(psDroid, order as DROID_ORDER);
                }
            }
        }
        10 => {
            // don't actually set any secondary flags - the cmdDroid array is
			// always used to determine which commander is the designator
            if State as libc::c_uint &
                   DSS_FIREDES_SET as libc::c_int as libc::c_uint ==
                   0 as libc::c_int as libc::c_uint {
                if cmdDroidGetDesignator((*psDroid).player as UDWORD) ==
                       psDroid {
                    cmdDroidClearDesignator((*psDroid).player as UDWORD);
                }
            } else { cmdDroidSetDesignator(psDroid); }
        }
        _ => { }
    }
    (*psDroid).secondaryOrder = CurrState;
    turnOffMultiMsg(0 as libc::c_int);
    return retVal;
}
// deal with a droid receiving a primary order
// deal with a droid receiving a primary order
#[no_mangle]
pub unsafe extern "C" fn secondaryGotPrimaryOrder(mut psDroid: *mut DROID,
                                                  mut order: DROID_ORDER)
 -> BOOL {
    let mut oldState: UDWORD = 0;
    if (*psDroid).droidType as libc::c_uint ==
           DROID_TRANSPORTER as libc::c_int as libc::c_uint {
        return 0 as libc::c_int
    }
    if order as libc::c_uint != DORDER_NONE as libc::c_int as libc::c_uint &&
           order as libc::c_uint != DORDER_STOP as libc::c_int as libc::c_uint
           &&
           order as libc::c_uint !=
               DORDER_DESTRUCT as libc::c_int as libc::c_uint &&
           order as libc::c_uint !=
               DORDER_GUARD as libc::c_int as libc::c_uint &&
           order as libc::c_uint !=
               DORDER_ATTACKTARGET as libc::c_int as libc::c_uint {
        //reset 2ndary order
        oldState = (*psDroid).secondaryOrder;
        (*psDroid).secondaryOrder &=
            !(0x380000 as libc::c_int | 0x100 as libc::c_int |
                  0x400000 as libc::c_int) as libc::c_uint;
        if oldState != (*psDroid).secondaryOrder &&
               (*psDroid).player as libc::c_uint == selectedPlayer {
            intRefreshScreen();
        }
    }
    return 0 as libc::c_int;
}
// set the state of a numeric group
#[no_mangle]
pub unsafe extern "C" fn secondarySetGroupState(mut player: UDWORD,
                                                mut group: UDWORD,
                                                mut sec: SECONDARY_ORDER,
                                                mut state: SECONDARY_STATE) {
    let mut psCurr: *mut DROID = 0 as *mut DROID;
    let mut currState: SECONDARY_STATE = 0 as SECONDARY_STATE;
    psCurr = apsDroidLists[player as usize];
    while !psCurr.is_null() {
        if (*psCurr).group as libc::c_uint == group {
            if secondaryGetState(psCurr, sec, &mut currState) != 0 &&
                   currState as libc::c_uint != state as libc::c_uint {
                secondarySetState(psCurr, sec, state);
            }
        }
        psCurr = (*psCurr).psNext
    };
}
// get the average secondary state of a numeric group
#[no_mangle]
pub unsafe extern "C" fn secondaryGetAverageGroupState(mut player: UDWORD,
                                                       mut group: UDWORD,
                                                       mut mask: UDWORD)
 -> SECONDARY_STATE {
    let mut aStateCount: [C2RustUnnamed_1; 5] =
        [C2RustUnnamed_1{state: 0, num: 0,}; 5];
    let mut i: SDWORD = 0;
    let mut numStates: SDWORD = 0;
    let mut max: SDWORD = 0;
    let mut psCurr: *mut DROID = 0 as *mut DROID;
    // count the number of units for each state
    numStates = 0 as libc::c_int;
    memset(aStateCount.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<[C2RustUnnamed_1; 5]>() as libc::c_ulong);
    psCurr = apsDroidLists[player as usize];
    while !psCurr.is_null() {
        if (*psCurr).group as libc::c_uint == group {
            i = 0 as libc::c_int;
            while i < numStates {
                if aStateCount[i as usize].state ==
                       (*psCurr).secondaryOrder & mask {
                    aStateCount[i as usize].num =
                        (aStateCount[i as usize].num as
                             libc::c_uint).wrapping_add(1 as libc::c_int as
                                                            libc::c_uint) as
                            UDWORD as UDWORD;
                    break ;
                } else { i += 1 }
            }
            if i == numStates {
                aStateCount[numStates as usize].state =
                    (*psCurr).secondaryOrder & mask;
                aStateCount[numStates as usize].num =
                    1 as libc::c_int as UDWORD;
                numStates += 1 as libc::c_int
            }
        }
        psCurr = (*psCurr).psNext
    }
    max = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < numStates {
        if aStateCount[i as usize].num > aStateCount[max as usize].num {
            max = i
        }
        i += 1
    }
    return aStateCount[max as usize].state as SECONDARY_STATE;
}
// make all the members of a numeric group have the same secondary states
// make all the members of a numeric group have the same secondary states
#[no_mangle]
pub unsafe extern "C" fn secondarySetAverageGroupState(mut player: UDWORD,
                                                       mut group: UDWORD) {
    // lookup table for orders and masks
    let mut aOrders: [C2RustUnnamed_0; 4] =
        [{
             let mut init =
                 C2RustUnnamed_0{order:
                                     DSO_ATTACK_RANGE as libc::c_int as
                                         UDWORD,
                                 mask: 0x3 as libc::c_int as UDWORD,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed_0{order:
                                     DSO_REPAIR_LEVEL as libc::c_int as
                                         UDWORD,
                                 mask: 0xc as libc::c_int as UDWORD,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed_0{order:
                                     DSO_ATTACK_LEVEL as libc::c_int as
                                         UDWORD,
                                 mask: 0x30 as libc::c_int as UDWORD,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed_0{order: DSO_HALTTYPE as libc::c_int as UDWORD,
                                 mask: 0xc0 as libc::c_int as UDWORD,};
             init
         }];
    let mut i: SDWORD = 0;
    let mut state: SDWORD = 0;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        state =
            secondaryGetAverageGroupState(player, group,
                                          aOrders[i as usize].mask) as SDWORD;
        secondarySetGroupState(player, group,
                               aOrders[i as usize].order as SECONDARY_ORDER,
                               state as SECONDARY_STATE);
        i += 1
    };
}
// do a moral check for a player
//  /////////////
//  Functions for Waypoint navigation. ajl from here on in...
// here's how waypoints currently work.
// probably want a better way of doing this, but at least it this'll
// help me get my head around the problem..
//
//  shift clicking on a tile adds a waypoint.
//  a waypoint is essentially identical to a move, with one exception.
//  adding a waypoint adds a route from the droids current target, not it's current
//  position. the route is therefore appended to the droid's movelist.
/*
// ajl 18thsep98, pulled from psx version. (no interface and not big enough move list.
#ifndef PSX
// append a new point to a droid's list.
BOOL orderAddWayPoint(DROID *psDroid ,UDWORD dX,UDWORD dY)
{
	static MOVE_CONTROL move;	// static to reduce stack usage	// we manipulate the droid at the move level.

	//UDWORD		 sX,sY;							// current position
	//UDWORD		 mX,mY;							// position now.
	UWORD		 sX,sY;							// current position
	UWORD		 mX,mY;							// position now.
	UDWORD		 oCount,nCount,i;
	UDWORD		 currX,currY,prevX=0,prevY=0;

	if(psDroid->order == DORDER_MOVE)
	{
//		if (psDroid->sMove.psFormation)
//		{
//			formationLeave(psDroid->sMove.psFormation, (BASE_OBJECT *)psDroid);
//			psDroid->sMove.psFormation = NULL;
//		}

		move = psDroid->sMove;					// get the droid's current move list

		sX	= psDroid->x;						// store droids pos.
		sY	= psDroid->y;
		mX	= psDroid->orderX;					// get the droids destination.
		mY	= psDroid->orderY;

		psDroid->x = mX;						// fool findpath to cheat...
		psDroid->y = mY;

		orderDroidLoc(psDroid,DORDER_MOVE,dX,dY);// create a route from droids current dest to target (issue the move order)

		psDroid->x = sX;						// undo our hack now route is found.
		psDroid->y = sY;

//		for(oCount= 0; (move.MovementList[oCount].XCoordinate!= -1)
//					 &&(move.MovementList[oCount].YCoordinate!= -1);oCount++);			// number of old moves.
		oCount = move.numPoints;

//		for(nCount= 0; (psDroid->sMove.MovementList[nCount].XCoordinate!= -1)
//					 &&(psDroid->sMove.MovementList[nCount].YCoordinate!= -1);nCount++);// number of new moves.
		nCount = psDroid->sMove.numPoints;

		if((nCount + oCount) > TRAVELSIZE)		// check number of moves is low enough!
		{
			DBPRINTF(("Waypoint List Exceeded, Point not Added."));
			return FALSE;						// go no further. failed....
		}

		for(i=oCount ; i<(oCount+nCount);i++)	// append the old list and new list together
		{
			prevX = move.asPath[i-1].x;
			prevY = move.asPath[i-1].y;

			currX =	psDroid->sMove.asPath[i-oCount].x;
			currY = psDroid->sMove.asPath[i-oCount].y;

			if (!((prevX == currX) && (prevY == currY)))	// avoid mutliple points(click twice on 1 point)
			{
				move.asPath[i]=psDroid->sMove.asPath[i-oCount];
			}
		}

		move.numPoints = (UBYTE)(oCount + nCount);
//		move.MovementList[i].XCoordinate = -1;		//terminate route.
//		move.MovementList[i].YCoordinate = -1;

		// store the new formation if any
		move.psFormation = psDroid->sMove.psFormation;

//		// clear the formation again!
//		if (psDroid->sMove.psFormation)
//		{
//			formationLeave(psDroid->sMove.psFormation, (BASE_OBJECT *)psDroid);
//			psDroid->sMove.psFormation = NULL;
//		}
//
		// set new move list.
		psDroid->sMove = move;

	}
	else										// not currently moving,just order it..
	{
		if(psDroid->x == dX && psDroid->y == dY)
		{
			return FALSE;
		}
		orderDroidLoc(psDroid,DORDER_MOVE, dX,dY);
	}
	return TRUE;
}


// add a new waypoint to selected droid set. .
void orderSelectedWaypoint(UDWORD player, UDWORD x, UDWORD y)
{
	DROID		*psCurr;//,*psPrev = NULL;
	iVector		position;
	FORMATION	*psFormation = NULL;

#ifndef PSX
	if(bMultiPlayer)
	{
		SendDroidWaypoint((UBYTE)player,x,y);
		turnOffMultiMsg(TRUE);
	}
#endif

	for(psCurr = apsDroidLists[player]; psCurr; psCurr=psCurr->psNext)
	{
		if (psCurr->selected)
		{
			orderAddWayPoint(psCurr,x,y);

//			if (psPrev && !psFormation)
//			{
//				if (formationNew(&psFormation, FT_LINE, (SDWORD)x,(SDWORD)y,
//					(SDWORD)calcDirection((SDWORD)psCurr->x,(SDWORD)psCurr->y, (SDWORD)x,(SDWORD)y)) )
//				{
//					formationJoin(psFormation, (BASE_OBJECT *)psPrev);
//					psPrev->sMove.psFormation = psFormation;
//				}
//				else
//				{
//					psFormation = NULL;
//				}
//			}
//			if (psFormation)
//			{
//				formationJoin(psFormation, (BASE_OBJECT *)psCurr);
//				psCurr->sMove.psFormation = psFormation;
// /			}
// /			psPrev = psCurr;

		}
	}

	if(player == selectedPlayer)		// add waypoint to the display.
	{
		position.x = x;
		position.z = y;
		position.y = map_Height(position.x, position.z) + 32;
		addEffect(&position,EFFECT_WAYPOINT,WAYPOINT_TYPE,FALSE,NULL,0);
	}

	turnOffMultiMsg(FALSE);	//propagate consequences back on
}
#endif
*/
// do a moral check for a player
#[no_mangle]
pub unsafe extern "C" fn orderMoralCheck(mut player: UDWORD) {
    let mut psCurr: *mut DROID = 0 as *mut DROID;
    let mut units: SDWORD = 0;
    let mut numVehicles: SDWORD = 0;
    let mut leadership: SDWORD = 0;
    let mut personLShip: SDWORD = 0;
    let mut check: SDWORD = 0;
    // count the number of vehicles and units on the side
    units = 0 as libc::c_int;
    numVehicles = 0 as libc::c_int;
    psCurr = apsDroidLists[player as usize];
    while !psCurr.is_null() {
        units += 1 as libc::c_int;
        if (*psCurr).droidType as libc::c_uint !=
               DROID_PERSON as libc::c_int as libc::c_uint {
            numVehicles += 1 as libc::c_int
        }
        psCurr = (*psCurr).psNext
    }
    if units > asRunData[player as usize].forceLevel as libc::c_int {
        // too many units, don't run
        return
    }
    // calculate the overall leadership
    leadership =
        asRunData[player as usize].leadership as libc::c_int +
            10 as libc::c_int;
    personLShip =
        asRunData[player as usize].leadership as libc::c_int +
            numVehicles * 3 as libc::c_int;
    // do the moral check for each droid
    psCurr = apsDroidLists[player as usize];
    while !psCurr.is_null() {
        if !(orderState(psCurr, DORDER_RUN) != 0 ||
                 orderState(psCurr, DORDER_RUNBURN) != 0 ||
                 orderState(psCurr, DORDER_RETREAT) != 0 ||
                 orderState(psCurr, DORDER_RTB) != 0 ||
                 orderState(psCurr, DORDER_RTR) != 0 ||
                 orderState(psCurr, DORDER_DESTRUCT) != 0) {
            check = rand() % 100 as libc::c_int;
            if (*psCurr).droidType as libc::c_uint ==
                   DROID_PERSON as libc::c_int as libc::c_uint {
                if check > personLShip { orderDroid(psCurr, DORDER_RUN); }
            } else if check > leadership { orderDroid(psCurr, DORDER_RUN); }
        }
        // already running - ignore
        psCurr = (*psCurr).psNext
    };
}
// do a moral check for a group
// do a moral check for a group
#[no_mangle]
pub unsafe extern "C" fn orderGroupMoralCheck(mut psGroup: *mut DROID_GROUP) {
    let mut psCurr: *mut DROID = 0 as *mut DROID;
    let mut units: SDWORD = 0;
    let mut numVehicles: SDWORD = 0;
    let mut leadership: SDWORD = 0;
    let mut personLShip: SDWORD = 0;
    let mut check: SDWORD = 0;
    let mut psRunData: *mut RUN_DATA = 0 as *mut RUN_DATA;
    // count the number of vehicles and units on the side
    units = 0 as libc::c_int;
    numVehicles = 0 as libc::c_int;
    psCurr = (*psGroup).psList;
    while !psCurr.is_null() {
        units += 1 as libc::c_int;
        if (*psCurr).droidType as libc::c_uint !=
               DROID_PERSON as libc::c_int as libc::c_uint {
            numVehicles += 1 as libc::c_int
        }
        psCurr = (*psCurr).psGrpNext
    }
    psRunData = &mut (*psGroup).sRunData;
    if units > (*psRunData).forceLevel as libc::c_int {
        // too many units, don't run
        return
    }
    // calculate the overall leadership
    leadership = (*psRunData).leadership as libc::c_int + 10 as libc::c_int;
    personLShip =
        (*psRunData).leadership as libc::c_int +
            numVehicles * 3 as libc::c_int;
    // do the moral check for each droid
    psCurr = (*psGroup).psList;
    while !psCurr.is_null() {
        if !(orderState(psCurr, DORDER_RUN) != 0 ||
                 orderState(psCurr, DORDER_RUNBURN) != 0 ||
                 orderState(psCurr, DORDER_RETREAT) != 0 ||
                 orderState(psCurr, DORDER_RTB) != 0 ||
                 orderState(psCurr, DORDER_RTR) != 0 ||
                 orderState(psCurr, DORDER_DESTRUCT) != 0) {
            check = rand() % 100 as libc::c_int;
            if (*psCurr).droidType as libc::c_uint ==
                   DROID_PERSON as libc::c_int as libc::c_uint {
                if check > personLShip {
                    orderDroidLoc(psCurr, DORDER_RUN,
                                  (*psRunData).sPos.x as UDWORD,
                                  (*psRunData).sPos.y as UDWORD);
                }
            } else if check > leadership {
                orderDroidLoc(psCurr, DORDER_RUN,
                              (*psRunData).sPos.x as UDWORD,
                              (*psRunData).sPos.y as UDWORD);
            }
        }
        // already running - ignore
        psCurr = (*psCurr).psGrpNext
    };
}
// do a health check for a droid
// do a health check for a group - do we need this function?
/*void orderGroupHealthCheck(DROID_GROUP *psGroup)
{
	DROID		*psCurr;
	RUN_DATA	*psRunData;
    SBYTE       healthLevel;

	psRunData = &psGroup->sRunData;

    healthLevel = psRunData->healthLevel;
    if (!healthLevel)
    {
        //if not set use player'
        healthLevel = asRunData[psGroup->psList->player].healthLevel;
    }

    //if a health level has not been set - quit!
    if (!healthLevel)
    {
        return;
    }

    // check the health value of all the units in the group
	for(psCurr=psGroup->psList; psCurr; psCurr=psCurr->psGrpNext)
	{
        //check if ANY unit in the group has less than the set level
		if (PERCENT(psCurr->body, psCurr->originalBody) < psRunData->healthLevel)
        {
            break;
        }
    }

	// order each unit in the group to run - unless already doing so
	for(psCurr = psGroup->psList; psCurr; psCurr=psCurr->psGrpNext)
	{
		if (orderState(psCurr, DORDER_RUN) ||
			orderState(psCurr, DORDER_RUNBURN) ||
			orderState(psCurr, DORDER_RETREAT) ||
			orderState(psCurr, DORDER_RTB) ||
			orderState(psCurr, DORDER_RTR) ||
			orderState(psCurr, DORDER_DESTRUCT))
		{
			// already running - ignore
			continue;
		}

		DBP0(("   DORDER_RUN: droid %d\n", psCurr->id));
		orderDroidLoc(psCurr, DORDER_RUN, psRunData->sPos.x, psRunData->sPos.y);
	}
}*/
// do a health check for a droid
#[no_mangle]
pub unsafe extern "C" fn orderHealthCheck(mut psDroid: *mut DROID) {
    let mut psCurr: *mut DROID = 0 as *mut DROID;
    let mut healthLevel: SBYTE = 0 as libc::c_int as SBYTE;
    let mut retreatX: UDWORD = 0 as libc::c_int as UDWORD;
    let mut retreatY: UDWORD = 0 as libc::c_int as UDWORD;
    if (*psDroid).droidType as libc::c_uint ==
           DROID_TRANSPORTER as libc::c_int as libc::c_uint {
        return
    }
    //get the health value to compare with
    if !(*psDroid).psGroup.is_null() {
        healthLevel = (*(*psDroid).psGroup).sRunData.healthLevel as SBYTE;
        retreatX = (*(*psDroid).psGroup).sRunData.sPos.x as UDWORD;
        retreatY = (*(*psDroid).psGroup).sRunData.sPos.y as UDWORD
    }
    //if health has not been set for the group - use players'
    if healthLevel == 0 {
        healthLevel =
            asRunData[(*psDroid).player as usize].healthLevel as SBYTE
    }
    //if not got a health level set then ignore
    if healthLevel == 0 { return }
    //if pos has not been set for the group - use players'
    if retreatX == 0 as libc::c_int as libc::c_uint &&
           retreatY == 0 as libc::c_int as libc::c_uint {
        retreatX = asRunData[(*psDroid).player as usize].sPos.x as UDWORD;
        retreatY = asRunData[(*psDroid).player as usize].sPos.y as UDWORD
    }
    if (*psDroid).body.wrapping_mul(100 as libc::c_int as
                                        libc::c_uint).wrapping_div((*psDroid).originalBody)
           < healthLevel as libc::c_uint {
        //order this droid to turn and run - // if already running - ignore
        if !(orderState(psDroid, DORDER_RUN) != 0 ||
                 orderState(psDroid, DORDER_RUNBURN) != 0 ||
                 orderState(psDroid, DORDER_RETREAT) != 0 ||
                 orderState(psDroid, DORDER_RTB) != 0 ||
                 orderState(psDroid, DORDER_RTR) != 0 ||
                 orderState(psDroid, DORDER_DESTRUCT) != 0) {
            orderDroidLoc(psDroid, DORDER_RUN, retreatX, retreatY);
        }
        // order each unit in the same group to run
        if !(*psDroid).psGroup.is_null() {
            psCurr = (*(*psDroid).psGroup).psList;
            while !psCurr.is_null() {
                if !(orderState(psCurr, DORDER_RUN) != 0 ||
                         orderState(psCurr, DORDER_RUNBURN) != 0 ||
                         orderState(psCurr, DORDER_RETREAT) != 0 ||
                         orderState(psCurr, DORDER_RTB) != 0 ||
                         orderState(psCurr, DORDER_RTR) != 0 ||
                         orderState(psCurr, DORDER_DESTRUCT) != 0) {
                    orderDroidLoc(psCurr, DORDER_RUN, retreatX, retreatY);
                }
                // already running - ignore
                psCurr = (*psCurr).psGrpNext
            }
        }
    };
}
// set the state of a secondary order for a Factory, return FALSE if failed.
// set the state of a secondary order for a Factory, return FALSE if failed.
#[no_mangle]
pub unsafe extern "C" fn setFactoryState(mut psStruct: *mut STRUCTURE,
                                         mut sec: SECONDARY_ORDER,
                                         mut State: SECONDARY_STATE) -> BOOL {
    let mut CurrState: UDWORD = 0;
    let mut retVal: BOOL = 0;
    let mut psFactory: *mut FACTORY = 0 as *mut FACTORY;
    if StructIsFactory(psStruct) == 0 {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"setFactoryState: structure is not a factory\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"order.c\x00" as *const u8 as *const libc::c_char,
                  4467 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 16],
                                            &[libc::c_char; 16]>(b"setFactoryState\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    psFactory = (*psStruct).pFunctionality as *mut FACTORY;
    CurrState = (*psFactory).secondaryOrder;
    retVal = 1 as libc::c_int;
    match sec as libc::c_uint {
        0 => {
            CurrState =
                CurrState & !(0x3 as libc::c_int) as libc::c_uint |
                    State as libc::c_uint
        }
        1 => {
            CurrState =
                CurrState & !(0xc as libc::c_int) as libc::c_uint |
                    State as libc::c_uint
        }
        2 => {
            CurrState =
                CurrState & !(0x30 as libc::c_int) as libc::c_uint |
                    State as libc::c_uint
        }
        7 => {
            if State as libc::c_uint &
                   DSS_PATROL_SET as libc::c_int as libc::c_uint != 0 {
                CurrState |= DSS_PATROL_SET as libc::c_int as libc::c_uint
            } else { CurrState &= !(0x400000 as libc::c_int) as libc::c_uint }
        }
        8 => {
            match State as libc::c_uint & 0xc0 as libc::c_int as libc::c_uint
                {
                192 => {
                    CurrState &= !(0xc0 as libc::c_int) as libc::c_uint;
                    CurrState |=
                        DSS_HALT_PERSUE as libc::c_int as libc::c_uint
                }
                128 => {
                    CurrState &= !(0xc0 as libc::c_int) as libc::c_uint;
                    CurrState |= DSS_HALT_GUARD as libc::c_int as libc::c_uint
                }
                64 => {
                    CurrState &= !(0xc0 as libc::c_int) as libc::c_uint;
                    CurrState |= DSS_HALT_HOLD as libc::c_int as libc::c_uint
                }
                _ => { }
            }
        }
        _ => { }
    }
    (*psFactory).secondaryOrder = CurrState;
    return retVal;
}
// get the state of a secondary order for a Factory, return FALSE if unsupported
// get the state of a secondary order for a Factory, return FALSE if unsupported
#[no_mangle]
pub unsafe extern "C" fn getFactoryState(mut psStruct: *mut STRUCTURE,
                                         mut sec: SECONDARY_ORDER,
                                         mut pState: *mut SECONDARY_STATE)
 -> BOOL {
    let mut state: UDWORD = 0;
    if StructIsFactory(psStruct) == 0 {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"getFactoryState: structure is not a factory\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"order.c\x00" as *const u8 as *const libc::c_char,
                  4532 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 16],
                                            &[libc::c_char; 16]>(b"getFactoryState\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    state = (*((*psStruct).pFunctionality as *mut FACTORY)).secondaryOrder;
    match sec as libc::c_uint {
        0 => {
            *pState =
                (state & 0x3 as libc::c_int as libc::c_uint) as
                    SECONDARY_STATE
        }
        1 => {
            *pState =
                (state & 0xc as libc::c_int as libc::c_uint) as
                    SECONDARY_STATE
        }
        2 => {
            *pState =
                (state & 0x30 as libc::c_int as libc::c_uint) as
                    SECONDARY_STATE
        }
        7 => {
            *pState =
                (state & 0x400000 as libc::c_int as libc::c_uint) as
                    SECONDARY_STATE
        }
        8 => {
            *pState =
                (state & 0xc0 as libc::c_int as libc::c_uint) as
                    SECONDARY_STATE
        }
        _ => { *pState = 0 as SECONDARY_STATE }
    }
    return 1 as libc::c_int;
}
//lasSat structure can select a target
//lasSat structure can select a target
#[no_mangle]
pub unsafe extern "C" fn orderStructureObj(mut player: UDWORD,
                                           mut psObj: *mut BASE_OBJECT) {
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut firePause: UDWORD = 0;
    let mut damLevel: UDWORD = 0;
    psStruct = apsStructLists[player as usize];
    while !psStruct.is_null() {
        if lasSatStructSelected(psStruct) != 0 {
            //there will only be the one!
            firePause =
                weaponFirePause(&mut *asWeaponStats.offset((*(*psStruct).asWeaps.as_mut_ptr().offset(0
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         isize)).nStat
                                                               as isize),
                                player as UBYTE);
            damLevel =
                (((*psStruct).body as libc::c_int * 100 as libc::c_int) as
                     libc::c_uint).wrapping_div(structureBody(psStruct));
            if damLevel < 25 as libc::c_int as libc::c_uint {
                firePause =
                    (firePause as libc::c_uint).wrapping_add(firePause) as
                        UDWORD as UDWORD
            }
            if isHumanPlayer(player) != 0 &&
                   gameTime.wrapping_sub((*psStruct).asWeaps[0 as libc::c_int
                                                                 as
                                                                 usize].lastFired)
                       <= firePause {
                break ;
            }
            //ok to fire - so fire away
            proj_SendProjectile(&mut *(*psStruct).asWeaps.as_mut_ptr().offset(0
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize),
                                0 as *mut BASE_OBJECT, player as SDWORD,
                                (*psObj).x as UDWORD, (*psObj).y as UDWORD,
                                (*psObj).z as UDWORD, psObj,
                                1 as libc::c_int);
            //set up last fires time
            (*psStruct).asWeaps[0 as libc::c_int as usize].lastFired =
                gameTime;
            //play 5 second countdown message
            audio_QueueTrackPos(ID_SOUND_LAS_SAT_COUNTDOWN as libc::c_int,
                                (*psObj).x as SDWORD, (*psObj).y as SDWORD,
                                (*psObj).z as SDWORD);
            // send the weapon fire
            if bMultiPlayer != 0 {
                sendLasSat(player as UBYTE, psStruct, psObj);
            }
            break ;
        } else { psStruct = (*psStruct).psNext }
    };
}
