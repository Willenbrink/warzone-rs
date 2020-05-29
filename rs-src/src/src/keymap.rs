use ::libc;
extern "C" {
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
    fn abort() -> !;
    /* Set a block heap to use for all memory allocation rather than standard malloc/free */
    #[no_mangle]
    fn memSetBlockHeap(psHeap: *mut _block_heap);
    /* Get the current block heap */
    #[no_mangle]
    fn memGetBlockHeap() -> *mut _block_heap;
    #[no_mangle]
    fn memMallocRelease(Size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn memFreeRelease(pMemToFree: *mut libc::c_void);
    /* Converts the key code into an ascii string */
    #[no_mangle]
    fn keyScanToString(code: KEY_CODE, ascii: *mut STRING,
                       maxStringSize: UDWORD);
    /* This returns true if the key is currently depressed */
    #[no_mangle]
    fn keyDown(code: KEY_CODE) -> BOOL;
    /* This returns true if the key went from being up to being down this frame */
    #[no_mangle]
    fn keyPressed(code: KEY_CODE) -> BOOL;
    /* This returns true if the key went from being down to being up this frame */
    #[no_mangle]
    fn keyReleased(code: KEY_CODE) -> BOOL;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_uint;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    /* Get the string from an ID number */
    #[no_mangle]
    fn strresGetString(psRes: *mut STR_RES, id: UDWORD) -> *mut STRING;
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
    /* The string resource object */
    #[no_mangle]
    static mut psStringRes: *mut STR_RES;
    #[no_mangle]
    static mut ConsoleString: [libc::c_char; 255];
    #[no_mangle]
    fn addConsoleMessage(messageText: *mut STRING,
                         jusType: CONSOLE_TEXT_JUSTIFICATION) -> BOOL;
    #[no_mangle]
    fn kf_DebugDroidInfo();
    #[no_mangle]
    fn kf_FrameRate();
    #[no_mangle]
    fn kf_ToggleRadar();
    #[no_mangle]
    fn kf_TogglePower();
    #[no_mangle]
    fn kf_RecalcLighting();
    #[no_mangle]
    fn kf_ScreenDump();
    #[no_mangle]
    fn kf_AllAvailable();
    #[no_mangle]
    fn kf_ToggleMistFog();
    #[no_mangle]
    fn kf_ToggleFogColour();
    #[no_mangle]
    fn kf_ToggleFog();
    #[no_mangle]
    fn kf_ToggleShadows();
    #[no_mangle]
    fn kf_ToggleCamera();
    #[no_mangle]
    fn kf_LowerTile();
    #[no_mangle]
    fn kf_SystemClose();
    #[no_mangle]
    fn kf_ZoomOut();
    #[no_mangle]
    fn kf_ZoomIn();
    #[no_mangle]
    fn kf_RotateLeft();
    #[no_mangle]
    fn kf_RotateRight();
    #[no_mangle]
    fn kf_PitchBack();
    #[no_mangle]
    fn kf_PitchForward();
    #[no_mangle]
    fn kf_ResetPitch();
    #[no_mangle]
    fn kf_ShowMappings();
    #[no_mangle]
    fn kf_SelectGrouping_1();
    #[no_mangle]
    fn kf_SelectGrouping_2();
    #[no_mangle]
    fn kf_SelectGrouping_3();
    #[no_mangle]
    fn kf_SelectGrouping_4();
    #[no_mangle]
    fn kf_SelectGrouping_5();
    #[no_mangle]
    fn kf_SelectGrouping_6();
    #[no_mangle]
    fn kf_SelectGrouping_7();
    #[no_mangle]
    fn kf_SelectGrouping_8();
    #[no_mangle]
    fn kf_SelectGrouping_9();
    #[no_mangle]
    fn kf_AssignGrouping_1();
    #[no_mangle]
    fn kf_AssignGrouping_2();
    #[no_mangle]
    fn kf_AssignGrouping_3();
    #[no_mangle]
    fn kf_AssignGrouping_4();
    #[no_mangle]
    fn kf_AssignGrouping_5();
    #[no_mangle]
    fn kf_AssignGrouping_6();
    #[no_mangle]
    fn kf_AssignGrouping_7();
    #[no_mangle]
    fn kf_AssignGrouping_8();
    #[no_mangle]
    fn kf_AssignGrouping_9();
    #[no_mangle]
    fn kf_addInGameOptions();
    #[no_mangle]
    fn kf_AddMissionOffWorld();
    #[no_mangle]
    fn kf_EndMissionOffWorld();
    #[no_mangle]
    fn kf_NewPlayerPower();
    #[no_mangle]
    fn kf_addMultiMenu();
    #[no_mangle]
    fn kf_multiAudioStart();
    #[no_mangle]
    fn kf_multiAudioStop();
    #[no_mangle]
    fn kf_JumpToMapMarker();
    #[no_mangle]
    fn kf_ToggleDebugMappings();
    #[no_mangle]
    fn kf_ToggleGodMode();
    #[no_mangle]
    fn kf_SeekNorth();
    #[no_mangle]
    fn kf_MaxScrollLimits();
    #[no_mangle]
    fn kf_TogglePauseMode();
    #[no_mangle]
    fn kf_ToggleDemoMode();
    #[no_mangle]
    fn kf_ToggleEnergyBars();
    #[no_mangle]
    fn kf_ToggleReloadBars();
    #[no_mangle]
    fn kf_FinishResearch();
    #[no_mangle]
    fn kf_ToggleOverlays();
    #[no_mangle]
    fn kf_ChooseOptions();
    #[no_mangle]
    fn kf_ChooseCommand();
    #[no_mangle]
    fn kf_ChooseManufacture();
    #[no_mangle]
    fn kf_ChooseResearch();
    #[no_mangle]
    fn kf_ChooseBuild();
    #[no_mangle]
    fn kf_ChooseDesign();
    #[no_mangle]
    fn kf_ChooseIntelligence();
    #[no_mangle]
    fn kf_ToggleWeather();
    #[no_mangle]
    fn kf_KillSelected();
    #[no_mangle]
    fn kf_GiveTemplateSet();
    #[no_mangle]
    fn kf_SendTextMessage();
    #[no_mangle]
    fn kf_SelectPlayer();
    #[no_mangle]
    fn kf_ToggleConsole();
    #[no_mangle]
    fn kf_SelectAllOnScreenUnits();
    #[no_mangle]
    fn kf_SelectAllUnits();
    #[no_mangle]
    fn kf_SelectAllVTOLs();
    #[no_mangle]
    fn kf_SelectAllHovers();
    #[no_mangle]
    fn kf_SelectAllWheeled();
    #[no_mangle]
    fn kf_SelectAllTracked();
    #[no_mangle]
    fn kf_SelectAllHalfTracked();
    #[no_mangle]
    fn kf_SelectAllCombatUnits();
    #[no_mangle]
    fn kf_SelectAllSameType();
    #[no_mangle]
    fn kf_SetDroidRangeShort();
    #[no_mangle]
    fn kf_SetDroidRangeDefault();
    #[no_mangle]
    fn kf_SetDroidRangeLong();
    #[no_mangle]
    fn kf_SetDroidRetreatMedium();
    #[no_mangle]
    fn kf_SetDroidRetreatHeavy();
    #[no_mangle]
    fn kf_SetDroidRetreatNever();
    #[no_mangle]
    fn kf_SetDroidAttackAtWill();
    #[no_mangle]
    fn kf_SetDroidAttackReturn();
    #[no_mangle]
    fn kf_SetDroidAttackCease();
    #[no_mangle]
    fn kf_SetDroidMoveHold();
    #[no_mangle]
    fn kf_SetDroidMovePursue();
    //not there?
    #[no_mangle]
    fn kf_SetDroidMovePatrol();
    // not there?
    #[no_mangle]
    fn kf_SetDroidReturnToBase();
    #[no_mangle]
    fn kf_SetDroidGoForRepair();
    #[no_mangle]
    fn kf_SetDroidRecycle();
    #[no_mangle]
    fn kf_ScatterDroids();
    #[no_mangle]
    fn kf_CentreOnBase();
    #[no_mangle]
    fn kf_MoveToLastMessagePos();
    #[no_mangle]
    fn kf_SelectAllDamaged();
    #[no_mangle]
    fn kf_RightOrderMenu();
    #[no_mangle]
    fn kf_ToggleFormationSpeedLimiting();
    #[no_mangle]
    fn kf_ToggleSensorDisplay();
    //Was commented out.  Re-enabled --Q 5/10/05
    #[no_mangle]
    fn kf_SensorDisplayOn();
    #[no_mangle]
    fn kf_SensorDisplayOff();
    #[no_mangle]
    fn kf_JumpToResourceExtractor();
    #[no_mangle]
    fn kf_JumpToRepairUnits();
    #[no_mangle]
    fn kf_JumpToConstructorUnits();
    #[no_mangle]
    fn kf_JumpToCommandUnits();
    #[no_mangle]
    fn kf_JumpToSensorUnits();
    #[no_mangle]
    fn kf_JumpToUnassignedUnits();
    #[no_mangle]
    fn kf_ToggleVisibility();
    #[no_mangle]
    fn kf_RadarZoomIn();
    #[no_mangle]
    fn kf_RadarZoomOut();
    #[no_mangle]
    fn kf_SelectNextFactory();
    #[no_mangle]
    fn kf_SelectNextCyborgFactory();
    #[no_mangle]
    fn kf_SelectNextPowerStation();
    #[no_mangle]
    fn kf_SelectNextResearch();
    #[no_mangle]
    fn kf_ToggleConsoleDrop();
    #[no_mangle]
    fn kf_SelectCommander_1();
    #[no_mangle]
    fn kf_SelectCommander_2();
    #[no_mangle]
    fn kf_SelectCommander_3();
    #[no_mangle]
    fn kf_SelectCommander_4();
    #[no_mangle]
    fn kf_SelectCommander_5();
    #[no_mangle]
    fn kf_SelectCommander_6();
    #[no_mangle]
    fn kf_SelectCommander_7();
    #[no_mangle]
    fn kf_SelectCommander_8();
    #[no_mangle]
    fn kf_SelectCommander_9();
    #[no_mangle]
    fn kf_ToggleReopenBuildMenu();
    #[no_mangle]
    fn kf_FaceNorth();
    #[no_mangle]
    fn kf_FaceSouth();
    #[no_mangle]
    fn kf_FaceEast();
    #[no_mangle]
    fn kf_FaceWest();
    #[no_mangle]
    fn kf_ToggleRadarJump();
    #[no_mangle]
    fn kf_MovePause();
    #[no_mangle]
    fn kf_SpeedUp();
    #[no_mangle]
    fn kf_SlowDown();
    #[no_mangle]
    fn kf_NormalSpeed();
    #[no_mangle]
    fn kf_ToggleRadarTerrain();
    //radar terrain
    #[no_mangle]
    fn kf_ToggleRadarAllyEnemy();
    /* Display3D.h */
    // Added to get sensor/gun range on screen.  -Q 5-10-05
    //fast version - optimised
    #[no_mangle]
    static mut player: iView;
    #[no_mangle]
    fn saveKeyMap() -> BOOL;
    #[no_mangle]
    fn loadKeyMap() -> BOOL;
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
pub type STRING = libc::c_char;
pub type UDWORD = libc::c_uint;
pub type SDWORD = libc::c_int;
pub type BOOL = libc::c_int;
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
pub type C2RustUnnamed = libc::c_uint;
pub const SDLK_LAST: C2RustUnnamed = 323;
pub const SDLK_UNDO: C2RustUnnamed = 322;
pub const SDLK_EURO: C2RustUnnamed = 321;
pub const SDLK_POWER: C2RustUnnamed = 320;
pub const SDLK_MENU: C2RustUnnamed = 319;
pub const SDLK_BREAK: C2RustUnnamed = 318;
pub const SDLK_SYSREQ: C2RustUnnamed = 317;
pub const SDLK_PRINT: C2RustUnnamed = 316;
pub const SDLK_HELP: C2RustUnnamed = 315;
pub const SDLK_COMPOSE: C2RustUnnamed = 314;
pub const SDLK_MODE: C2RustUnnamed = 313;
pub const SDLK_RSUPER: C2RustUnnamed = 312;
pub const SDLK_LSUPER: C2RustUnnamed = 311;
pub const SDLK_LMETA: C2RustUnnamed = 310;
pub const SDLK_RMETA: C2RustUnnamed = 309;
pub const SDLK_LALT: C2RustUnnamed = 308;
pub const SDLK_RALT: C2RustUnnamed = 307;
pub const SDLK_LCTRL: C2RustUnnamed = 306;
pub const SDLK_RCTRL: C2RustUnnamed = 305;
pub const SDLK_LSHIFT: C2RustUnnamed = 304;
pub const SDLK_RSHIFT: C2RustUnnamed = 303;
pub const SDLK_SCROLLOCK: C2RustUnnamed = 302;
pub const SDLK_CAPSLOCK: C2RustUnnamed = 301;
pub const SDLK_NUMLOCK: C2RustUnnamed = 300;
pub const SDLK_F15: C2RustUnnamed = 296;
pub const SDLK_F14: C2RustUnnamed = 295;
pub const SDLK_F13: C2RustUnnamed = 294;
pub const SDLK_F12: C2RustUnnamed = 293;
pub const SDLK_F11: C2RustUnnamed = 292;
pub const SDLK_F10: C2RustUnnamed = 291;
pub const SDLK_F9: C2RustUnnamed = 290;
pub const SDLK_F8: C2RustUnnamed = 289;
pub const SDLK_F7: C2RustUnnamed = 288;
pub const SDLK_F6: C2RustUnnamed = 287;
pub const SDLK_F5: C2RustUnnamed = 286;
pub const SDLK_F4: C2RustUnnamed = 285;
pub const SDLK_F3: C2RustUnnamed = 284;
pub const SDLK_F2: C2RustUnnamed = 283;
pub const SDLK_F1: C2RustUnnamed = 282;
pub const SDLK_PAGEDOWN: C2RustUnnamed = 281;
pub const SDLK_PAGEUP: C2RustUnnamed = 280;
pub const SDLK_END: C2RustUnnamed = 279;
pub const SDLK_HOME: C2RustUnnamed = 278;
pub const SDLK_INSERT: C2RustUnnamed = 277;
pub const SDLK_LEFT: C2RustUnnamed = 276;
pub const SDLK_RIGHT: C2RustUnnamed = 275;
pub const SDLK_DOWN: C2RustUnnamed = 274;
pub const SDLK_UP: C2RustUnnamed = 273;
pub const SDLK_KP_EQUALS: C2RustUnnamed = 272;
pub const SDLK_KP_ENTER: C2RustUnnamed = 271;
pub const SDLK_KP_PLUS: C2RustUnnamed = 270;
pub const SDLK_KP_MINUS: C2RustUnnamed = 269;
pub const SDLK_KP_MULTIPLY: C2RustUnnamed = 268;
pub const SDLK_KP_DIVIDE: C2RustUnnamed = 267;
pub const SDLK_KP_PERIOD: C2RustUnnamed = 266;
pub const SDLK_KP9: C2RustUnnamed = 265;
pub const SDLK_KP8: C2RustUnnamed = 264;
pub const SDLK_KP7: C2RustUnnamed = 263;
pub const SDLK_KP6: C2RustUnnamed = 262;
pub const SDLK_KP5: C2RustUnnamed = 261;
pub const SDLK_KP4: C2RustUnnamed = 260;
pub const SDLK_KP3: C2RustUnnamed = 259;
pub const SDLK_KP2: C2RustUnnamed = 258;
pub const SDLK_KP1: C2RustUnnamed = 257;
pub const SDLK_KP0: C2RustUnnamed = 256;
pub const SDLK_WORLD_95: C2RustUnnamed = 255;
pub const SDLK_WORLD_94: C2RustUnnamed = 254;
pub const SDLK_WORLD_93: C2RustUnnamed = 253;
pub const SDLK_WORLD_92: C2RustUnnamed = 252;
pub const SDLK_WORLD_91: C2RustUnnamed = 251;
pub const SDLK_WORLD_90: C2RustUnnamed = 250;
pub const SDLK_WORLD_89: C2RustUnnamed = 249;
pub const SDLK_WORLD_88: C2RustUnnamed = 248;
pub const SDLK_WORLD_87: C2RustUnnamed = 247;
pub const SDLK_WORLD_86: C2RustUnnamed = 246;
pub const SDLK_WORLD_85: C2RustUnnamed = 245;
pub const SDLK_WORLD_84: C2RustUnnamed = 244;
pub const SDLK_WORLD_83: C2RustUnnamed = 243;
pub const SDLK_WORLD_82: C2RustUnnamed = 242;
pub const SDLK_WORLD_81: C2RustUnnamed = 241;
pub const SDLK_WORLD_80: C2RustUnnamed = 240;
pub const SDLK_WORLD_79: C2RustUnnamed = 239;
pub const SDLK_WORLD_78: C2RustUnnamed = 238;
pub const SDLK_WORLD_77: C2RustUnnamed = 237;
pub const SDLK_WORLD_76: C2RustUnnamed = 236;
pub const SDLK_WORLD_75: C2RustUnnamed = 235;
pub const SDLK_WORLD_74: C2RustUnnamed = 234;
pub const SDLK_WORLD_73: C2RustUnnamed = 233;
pub const SDLK_WORLD_72: C2RustUnnamed = 232;
pub const SDLK_WORLD_71: C2RustUnnamed = 231;
pub const SDLK_WORLD_70: C2RustUnnamed = 230;
pub const SDLK_WORLD_69: C2RustUnnamed = 229;
pub const SDLK_WORLD_68: C2RustUnnamed = 228;
pub const SDLK_WORLD_67: C2RustUnnamed = 227;
pub const SDLK_WORLD_66: C2RustUnnamed = 226;
pub const SDLK_WORLD_65: C2RustUnnamed = 225;
pub const SDLK_WORLD_64: C2RustUnnamed = 224;
pub const SDLK_WORLD_63: C2RustUnnamed = 223;
pub const SDLK_WORLD_62: C2RustUnnamed = 222;
pub const SDLK_WORLD_61: C2RustUnnamed = 221;
pub const SDLK_WORLD_60: C2RustUnnamed = 220;
pub const SDLK_WORLD_59: C2RustUnnamed = 219;
pub const SDLK_WORLD_58: C2RustUnnamed = 218;
pub const SDLK_WORLD_57: C2RustUnnamed = 217;
pub const SDLK_WORLD_56: C2RustUnnamed = 216;
pub const SDLK_WORLD_55: C2RustUnnamed = 215;
pub const SDLK_WORLD_54: C2RustUnnamed = 214;
pub const SDLK_WORLD_53: C2RustUnnamed = 213;
pub const SDLK_WORLD_52: C2RustUnnamed = 212;
pub const SDLK_WORLD_51: C2RustUnnamed = 211;
pub const SDLK_WORLD_50: C2RustUnnamed = 210;
pub const SDLK_WORLD_49: C2RustUnnamed = 209;
pub const SDLK_WORLD_48: C2RustUnnamed = 208;
pub const SDLK_WORLD_47: C2RustUnnamed = 207;
pub const SDLK_WORLD_46: C2RustUnnamed = 206;
pub const SDLK_WORLD_45: C2RustUnnamed = 205;
pub const SDLK_WORLD_44: C2RustUnnamed = 204;
pub const SDLK_WORLD_43: C2RustUnnamed = 203;
pub const SDLK_WORLD_42: C2RustUnnamed = 202;
pub const SDLK_WORLD_41: C2RustUnnamed = 201;
pub const SDLK_WORLD_40: C2RustUnnamed = 200;
pub const SDLK_WORLD_39: C2RustUnnamed = 199;
pub const SDLK_WORLD_38: C2RustUnnamed = 198;
pub const SDLK_WORLD_37: C2RustUnnamed = 197;
pub const SDLK_WORLD_36: C2RustUnnamed = 196;
pub const SDLK_WORLD_35: C2RustUnnamed = 195;
pub const SDLK_WORLD_34: C2RustUnnamed = 194;
pub const SDLK_WORLD_33: C2RustUnnamed = 193;
pub const SDLK_WORLD_32: C2RustUnnamed = 192;
pub const SDLK_WORLD_31: C2RustUnnamed = 191;
pub const SDLK_WORLD_30: C2RustUnnamed = 190;
pub const SDLK_WORLD_29: C2RustUnnamed = 189;
pub const SDLK_WORLD_28: C2RustUnnamed = 188;
pub const SDLK_WORLD_27: C2RustUnnamed = 187;
pub const SDLK_WORLD_26: C2RustUnnamed = 186;
pub const SDLK_WORLD_25: C2RustUnnamed = 185;
pub const SDLK_WORLD_24: C2RustUnnamed = 184;
pub const SDLK_WORLD_23: C2RustUnnamed = 183;
pub const SDLK_WORLD_22: C2RustUnnamed = 182;
pub const SDLK_WORLD_21: C2RustUnnamed = 181;
pub const SDLK_WORLD_20: C2RustUnnamed = 180;
pub const SDLK_WORLD_19: C2RustUnnamed = 179;
pub const SDLK_WORLD_18: C2RustUnnamed = 178;
pub const SDLK_WORLD_17: C2RustUnnamed = 177;
pub const SDLK_WORLD_16: C2RustUnnamed = 176;
pub const SDLK_WORLD_15: C2RustUnnamed = 175;
pub const SDLK_WORLD_14: C2RustUnnamed = 174;
pub const SDLK_WORLD_13: C2RustUnnamed = 173;
pub const SDLK_WORLD_12: C2RustUnnamed = 172;
pub const SDLK_WORLD_11: C2RustUnnamed = 171;
pub const SDLK_WORLD_10: C2RustUnnamed = 170;
pub const SDLK_WORLD_9: C2RustUnnamed = 169;
pub const SDLK_WORLD_8: C2RustUnnamed = 168;
pub const SDLK_WORLD_7: C2RustUnnamed = 167;
pub const SDLK_WORLD_6: C2RustUnnamed = 166;
pub const SDLK_WORLD_5: C2RustUnnamed = 165;
pub const SDLK_WORLD_4: C2RustUnnamed = 164;
pub const SDLK_WORLD_3: C2RustUnnamed = 163;
pub const SDLK_WORLD_2: C2RustUnnamed = 162;
pub const SDLK_WORLD_1: C2RustUnnamed = 161;
pub const SDLK_WORLD_0: C2RustUnnamed = 160;
pub const SDLK_DELETE: C2RustUnnamed = 127;
pub const SDLK_z: C2RustUnnamed = 122;
pub const SDLK_y: C2RustUnnamed = 121;
pub const SDLK_x: C2RustUnnamed = 120;
pub const SDLK_w: C2RustUnnamed = 119;
pub const SDLK_v: C2RustUnnamed = 118;
pub const SDLK_u: C2RustUnnamed = 117;
pub const SDLK_t: C2RustUnnamed = 116;
pub const SDLK_s: C2RustUnnamed = 115;
pub const SDLK_r: C2RustUnnamed = 114;
pub const SDLK_q: C2RustUnnamed = 113;
pub const SDLK_p: C2RustUnnamed = 112;
pub const SDLK_o: C2RustUnnamed = 111;
pub const SDLK_n: C2RustUnnamed = 110;
pub const SDLK_m: C2RustUnnamed = 109;
pub const SDLK_l: C2RustUnnamed = 108;
pub const SDLK_k: C2RustUnnamed = 107;
pub const SDLK_j: C2RustUnnamed = 106;
pub const SDLK_i: C2RustUnnamed = 105;
pub const SDLK_h: C2RustUnnamed = 104;
pub const SDLK_g: C2RustUnnamed = 103;
pub const SDLK_f: C2RustUnnamed = 102;
pub const SDLK_e: C2RustUnnamed = 101;
pub const SDLK_d: C2RustUnnamed = 100;
pub const SDLK_c: C2RustUnnamed = 99;
pub const SDLK_b: C2RustUnnamed = 98;
pub const SDLK_a: C2RustUnnamed = 97;
pub const SDLK_BACKQUOTE: C2RustUnnamed = 96;
pub const SDLK_UNDERSCORE: C2RustUnnamed = 95;
pub const SDLK_CARET: C2RustUnnamed = 94;
pub const SDLK_RIGHTBRACKET: C2RustUnnamed = 93;
pub const SDLK_BACKSLASH: C2RustUnnamed = 92;
pub const SDLK_LEFTBRACKET: C2RustUnnamed = 91;
pub const SDLK_AT: C2RustUnnamed = 64;
pub const SDLK_QUESTION: C2RustUnnamed = 63;
pub const SDLK_GREATER: C2RustUnnamed = 62;
pub const SDLK_EQUALS: C2RustUnnamed = 61;
pub const SDLK_LESS: C2RustUnnamed = 60;
pub const SDLK_SEMICOLON: C2RustUnnamed = 59;
pub const SDLK_COLON: C2RustUnnamed = 58;
pub const SDLK_9: C2RustUnnamed = 57;
pub const SDLK_8: C2RustUnnamed = 56;
pub const SDLK_7: C2RustUnnamed = 55;
pub const SDLK_6: C2RustUnnamed = 54;
pub const SDLK_5: C2RustUnnamed = 53;
pub const SDLK_4: C2RustUnnamed = 52;
pub const SDLK_3: C2RustUnnamed = 51;
pub const SDLK_2: C2RustUnnamed = 50;
pub const SDLK_1: C2RustUnnamed = 49;
pub const SDLK_0: C2RustUnnamed = 48;
pub const SDLK_SLASH: C2RustUnnamed = 47;
pub const SDLK_PERIOD: C2RustUnnamed = 46;
pub const SDLK_MINUS: C2RustUnnamed = 45;
pub const SDLK_COMMA: C2RustUnnamed = 44;
pub const SDLK_PLUS: C2RustUnnamed = 43;
pub const SDLK_ASTERISK: C2RustUnnamed = 42;
pub const SDLK_RIGHTPAREN: C2RustUnnamed = 41;
pub const SDLK_LEFTPAREN: C2RustUnnamed = 40;
pub const SDLK_QUOTE: C2RustUnnamed = 39;
pub const SDLK_AMPERSAND: C2RustUnnamed = 38;
pub const SDLK_DOLLAR: C2RustUnnamed = 36;
pub const SDLK_HASH: C2RustUnnamed = 35;
pub const SDLK_QUOTEDBL: C2RustUnnamed = 34;
pub const SDLK_EXCLAIM: C2RustUnnamed = 33;
pub const SDLK_SPACE: C2RustUnnamed = 32;
pub const SDLK_ESCAPE: C2RustUnnamed = 27;
pub const SDLK_PAUSE: C2RustUnnamed = 19;
pub const SDLK_RETURN: C2RustUnnamed = 13;
pub const SDLK_CLEAR: C2RustUnnamed = 12;
pub const SDLK_TAB: C2RustUnnamed = 9;
pub const SDLK_BACKSPACE: C2RustUnnamed = 8;
pub const SDLK_FIRST: C2RustUnnamed = 0;
pub const SDLK_UNKNOWN: C2RustUnnamed = 0;
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
pub type BLOCK_HEAP = _block_heap;
// The next free ID
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
pub type KEY_ACTION = libc::c_uint;
pub const KEYMAP_RELEASED: KEY_ACTION = 2;
pub const KEYMAP_PRESSED: KEY_ACTION = 1;
pub const KEYMAP_DOWN: KEY_ACTION = 0;
pub type KEY_STATUS = libc::c_uint;
pub const KEYMAP___HIDE: KEY_STATUS = 4;
pub const KEYMAP_ALWAYS_PROCESS: KEY_STATUS = 3;
pub const KEYMAP_ASSIGNABLE: KEY_STATUS = 2;
pub const KEYMAP_ALWAYS: KEY_STATUS = 1;
pub const KEYMAP__DEBUG: KEY_STATUS = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _keyMapping {
    pub function: Option<unsafe extern "C" fn() -> ()>,
    pub active: BOOL,
    pub status: KEY_STATUS,
    pub lastCalled: UDWORD,
    pub metaKeyCode: KEY_CODE,
    pub altMetaKeyCode: KEY_CODE,
    pub subKeyCode: KEY_CODE,
    pub action: KEY_ACTION,
    pub pName: *mut STRING,
    pub psNext: *mut _keyMapping,
}
pub type KEY_MAPPING = _keyMapping;
pub type CONSOLE_TEXT_JUSTIFICATION = libc::c_uint;
pub const DEFAULT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 3;
pub const CENTRE_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 2;
pub const RIGHT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 1;
pub const LEFT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 0;
/* **************************************************************************/
/*
 * pieTypes.h
 *
 * type defines for simple pies.
 *
 */
/* **************************************************************************/
/* **************************************************************************/
/* **************************************************************************/
/*
 *	Global Definitions
 */
/* **************************************************************************/
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
pub type int32 = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iVector {
    pub x: int32,
    pub y: int32,
    pub z: int32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iView {
    pub p: iVector,
    pub r: iVector,
}
pub type KEYMAP_MARKER = _keymap_Marker;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _keymap_Marker {
    pub psMapping: *mut KEY_MAPPING,
    pub xPos: UDWORD,
    pub yPos: UDWORD,
    pub spin: SDWORD,
}
// for keymap editor.
pub type _keymapsave = Option<unsafe extern "C" fn() -> ()>;
// ----------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn keyGetMappingFromFunction(mut function:
                                                       *mut libc::c_void)
 -> *mut KEY_MAPPING {
    let mut psMapping: *mut KEY_MAPPING = 0 as *mut KEY_MAPPING;
    let mut psReturn: *mut KEY_MAPPING = 0 as *mut KEY_MAPPING;
    psMapping = keyMappings;
    psReturn = 0 as *mut KEY_MAPPING;
    while !psMapping.is_null() && psReturn.is_null() {
        if (*psMapping).function ==
               ::std::mem::transmute::<*mut libc::c_void,
                                       Option<unsafe extern "C" fn()
                                                  -> ()>>(function) {
            psReturn = psMapping
        }
        psMapping = (*psMapping).psNext
    }
    return psReturn;
}
static mut qwertyKeyMappings: [KEYMAP_MARKER; 26] =
    [KEYMAP_MARKER{psMapping: 0 as *const KEY_MAPPING as *mut KEY_MAPPING,
                   xPos: 0,
                   yPos: 0,
                   spin: 0,}; 26];
static mut bDoingDebugMappings: BOOL = 0 as libc::c_int;
// PSX needs this too...
// ----------------------------------------------------------------------------------
// ----------------------------------------------------------------------------------
/* The linked list of present key mappings */
#[no_mangle]
pub static mut keyMappings: *mut KEY_MAPPING =
    0 as *const KEY_MAPPING as *mut KEY_MAPPING;
/* Holds number of active mappings */
#[no_mangle]
pub static mut numActiveMappings: UDWORD = 0;
/* Last meta and sub key that were recorded */
static mut lastMetaKey: KEY_CODE = 0 as KEY_CODE;
static mut lastSubKey: KEY_CODE = 0 as KEY_CODE;
static mut bKeyProcessing: BOOL = 1 as libc::c_int;
// ----------------------------------------------------------------------------------
// Adding a mapped function ? add a save pointer! Thank AlexL.
// don't bugger around with the order either. new ones go at the end! DEBUG in debug section..
//typedef void (*_keymapsave)(void);
#[no_mangle]
pub static mut keyMapSaveTable: [_keymapsave; 140] =
    unsafe {
        [Some(kf_ChooseManufacture as unsafe extern "C" fn() -> ()),
         Some(kf_ChooseResearch as unsafe extern "C" fn() -> ()),
         Some(kf_ChooseBuild as unsafe extern "C" fn() -> ()),
         Some(kf_ChooseDesign as unsafe extern "C" fn() -> ()),
         Some(kf_ChooseIntelligence as unsafe extern "C" fn() -> ()),
         Some(kf_ChooseCommand as unsafe extern "C" fn() -> ()),
         Some(kf_ToggleRadar as unsafe extern "C" fn() -> ()),
         Some(kf_ToggleConsole as unsafe extern "C" fn() -> ()),
         Some(kf_ToggleEnergyBars as unsafe extern "C" fn() -> ()),
         Some(kf_ToggleReloadBars as unsafe extern "C" fn() -> ()),
         Some(kf_ScreenDump as unsafe extern "C" fn() -> ()),
         Some(kf_MoveToLastMessagePos as unsafe extern "C" fn() -> ()),
         Some(kf_AssignGrouping_1 as unsafe extern "C" fn() -> ()),
         Some(kf_AssignGrouping_2 as unsafe extern "C" fn() -> ()),
         Some(kf_AssignGrouping_3 as unsafe extern "C" fn() -> ()),
         Some(kf_AssignGrouping_4 as unsafe extern "C" fn() -> ()),
         Some(kf_AssignGrouping_5 as unsafe extern "C" fn() -> ()),
         Some(kf_AssignGrouping_6 as unsafe extern "C" fn() -> ()),
         Some(kf_AssignGrouping_7 as unsafe extern "C" fn() -> ()),
         Some(kf_AssignGrouping_8 as unsafe extern "C" fn() -> ()),
         Some(kf_AssignGrouping_9 as unsafe extern "C" fn() -> ()),
         Some(kf_SelectGrouping_1 as unsafe extern "C" fn() -> ()),
         Some(kf_SelectGrouping_2 as unsafe extern "C" fn() -> ()),
         Some(kf_SelectGrouping_3 as unsafe extern "C" fn() -> ()),
         Some(kf_SelectGrouping_4 as unsafe extern "C" fn() -> ()),
         Some(kf_SelectGrouping_5 as unsafe extern "C" fn() -> ()),
         Some(kf_SelectGrouping_6 as unsafe extern "C" fn() -> ()),
         Some(kf_SelectGrouping_7 as unsafe extern "C" fn() -> ()),
         Some(kf_SelectGrouping_8 as unsafe extern "C" fn() -> ()),
         Some(kf_SelectGrouping_9 as unsafe extern "C" fn() -> ()),
         Some(kf_addMultiMenu as unsafe extern "C" fn() -> ()),
         Some(kf_multiAudioStart as unsafe extern "C" fn() -> ()),
         Some(kf_multiAudioStop as unsafe extern "C" fn() -> ()),
         Some(kf_SeekNorth as unsafe extern "C" fn() -> ()),
         Some(kf_ToggleCamera as unsafe extern "C" fn() -> ()),
         Some(kf_addInGameOptions as unsafe extern "C" fn() -> ()),
         Some(kf_RadarZoomOut as unsafe extern "C" fn() -> ()),
         Some(kf_RadarZoomIn as unsafe extern "C" fn() -> ()),
         Some(kf_ZoomOut as unsafe extern "C" fn() -> ()),
         Some(kf_ZoomIn as unsafe extern "C" fn() -> ()),
         Some(kf_PitchForward as unsafe extern "C" fn() -> ()),
         Some(kf_RotateLeft as unsafe extern "C" fn() -> ()),
         Some(kf_ResetPitch as unsafe extern "C" fn() -> ()),
         Some(kf_RotateRight as unsafe extern "C" fn() -> ()),
         Some(kf_PitchBack as unsafe extern "C" fn() -> ()),
         Some(kf_RightOrderMenu as unsafe extern "C" fn() -> ()),
         Some(kf_JumpToResourceExtractor as unsafe extern "C" fn() -> ()),
         Some(kf_JumpToRepairUnits as unsafe extern "C" fn() -> ()),
         Some(kf_JumpToConstructorUnits as unsafe extern "C" fn() -> ()),
         Some(kf_JumpToSensorUnits as unsafe extern "C" fn() -> ()),
         Some(kf_JumpToCommandUnits as unsafe extern "C" fn() -> ()),
         Some(kf_ToggleOverlays as unsafe extern "C" fn() -> ()),
         Some(kf_CentreOnBase as unsafe extern "C" fn() -> ()),
         Some(kf_SetDroidAttackCease as unsafe extern "C" fn() -> ()),
         Some(kf_JumpToUnassignedUnits as unsafe extern "C" fn() -> ()),
         Some(kf_SetDroidAttackReturn as unsafe extern "C" fn() -> ()),
         Some(kf_SetDroidAttackAtWill as unsafe extern "C" fn() -> ()),
         Some(kf_SetDroidReturnToBase as unsafe extern "C" fn() -> ()),
         Some(kf_SetDroidRangeDefault as unsafe extern "C" fn() -> ()),
         Some(kf_ToggleFormationSpeedLimiting as
                  unsafe extern "C" fn() -> ()),
         Some(kf_SetDroidRangeShort as unsafe extern "C" fn() -> ()),
         Some(kf_SetDroidMovePursue as unsafe extern "C" fn() -> ()),
         Some(kf_SetDroidMovePatrol as unsafe extern "C" fn() -> ()),
         Some(kf_SetDroidGoForRepair as unsafe extern "C" fn() -> ()),
         Some(kf_SetDroidMoveHold as unsafe extern "C" fn() -> ()),
         Some(kf_SendTextMessage as unsafe extern "C" fn() -> ()),
         Some(kf_SetDroidRangeLong as unsafe extern "C" fn() -> ()),
         Some(kf_ScatterDroids as unsafe extern "C" fn() -> ()),
         Some(kf_SetDroidRetreatMedium as unsafe extern "C" fn() -> ()),
         Some(kf_SetDroidRetreatHeavy as unsafe extern "C" fn() -> ()),
         Some(kf_SetDroidRetreatNever as unsafe extern "C" fn() -> ()),
         Some(kf_SelectAllCombatUnits as unsafe extern "C" fn() -> ()),
         Some(kf_SelectAllDamaged as unsafe extern "C" fn() -> ()),
         Some(kf_SelectAllHalfTracked as unsafe extern "C" fn() -> ()),
         Some(kf_SelectAllHovers as unsafe extern "C" fn() -> ()),
         Some(kf_SetDroidRecycle as unsafe extern "C" fn() -> ()),
         Some(kf_SelectAllOnScreenUnits as unsafe extern "C" fn() -> ()),
         Some(kf_SelectAllTracked as unsafe extern "C" fn() -> ()),
         Some(kf_SelectAllUnits as unsafe extern "C" fn() -> ()),
         Some(kf_SelectAllVTOLs as unsafe extern "C" fn() -> ()),
         Some(kf_SelectAllWheeled as unsafe extern "C" fn() -> ()),
         Some(kf_FinishResearch as unsafe extern "C" fn() -> ()),
         Some(kf_FrameRate as unsafe extern "C" fn() -> ()),
         Some(kf_SelectAllSameType as unsafe extern "C" fn() -> ()),
         Some(kf_SelectNextFactory as unsafe extern "C" fn() -> ()),
         Some(kf_SelectNextResearch as unsafe extern "C" fn() -> ()),
         Some(kf_SelectNextPowerStation as unsafe extern "C" fn() -> ()),
         Some(kf_SelectNextCyborgFactory as unsafe extern "C" fn() -> ()),
         Some(kf_ToggleConsoleDrop as unsafe extern "C" fn() -> ()),
         Some(kf_SelectCommander_1 as unsafe extern "C" fn() -> ()),
         Some(kf_SelectCommander_2 as unsafe extern "C" fn() -> ()),
         Some(kf_SelectCommander_3 as unsafe extern "C" fn() -> ()),
         Some(kf_SelectCommander_4 as unsafe extern "C" fn() -> ()),
         Some(kf_SelectCommander_5 as unsafe extern "C" fn() -> ()),
         Some(kf_SelectCommander_6 as unsafe extern "C" fn() -> ()),
         Some(kf_SelectCommander_7 as unsafe extern "C" fn() -> ()),
         Some(kf_SelectCommander_8 as unsafe extern "C" fn() -> ()),
         Some(kf_SelectCommander_9 as unsafe extern "C" fn() -> ()),
         Some(kf_FaceNorth as unsafe extern "C" fn() -> ()),
         Some(kf_FaceSouth as unsafe extern "C" fn() -> ()),
         Some(kf_FaceWest as unsafe extern "C" fn() -> ()),
         Some(kf_FaceEast as unsafe extern "C" fn() -> ()),
         Some(kf_SpeedUp as unsafe extern "C" fn() -> ()),
         Some(kf_SlowDown as unsafe extern "C" fn() -> ()),
         Some(kf_NormalSpeed as unsafe extern "C" fn() -> ()),
         Some(kf_ToggleRadarJump as unsafe extern "C" fn() -> ()),
         Some(kf_MovePause as unsafe extern "C" fn() -> ()),
         Some(kf_ToggleReopenBuildMenu as unsafe extern "C" fn() -> ()),
         Some(kf_SensorDisplayOn as unsafe extern "C" fn() -> ()),
         Some(kf_SensorDisplayOff as unsafe extern "C" fn() -> ()),
         Some(kf_ToggleRadarTerrain as unsafe extern "C" fn() -> ()),
         Some(kf_ToggleRadarAllyEnemy as unsafe extern "C" fn() -> ()),
         Some(kf_ToggleSensorDisplay as unsafe extern "C" fn() -> ()),
         Some(kf_AllAvailable as unsafe extern "C" fn() -> ()),
         Some(kf_ToggleDebugMappings as unsafe extern "C" fn() -> ()),
         Some(kf_NewPlayerPower as unsafe extern "C" fn() -> ()),
         Some(kf_TogglePauseMode as unsafe extern "C" fn() -> ()),
         Some(kf_MaxScrollLimits as unsafe extern "C" fn() -> ()),
         Some(kf_DebugDroidInfo as unsafe extern "C" fn() -> ()),
         Some(kf_RecalcLighting as unsafe extern "C" fn() -> ()),
         Some(kf_ToggleFog as unsafe extern "C" fn() -> ()),
         Some(kf_ChooseOptions as unsafe extern "C" fn() -> ()),
         Some(kf_TogglePower as unsafe extern "C" fn() -> ()),
         Some(kf_ToggleWeather as unsafe extern "C" fn() -> ()),
         Some(kf_SelectPlayer as unsafe extern "C" fn() -> ()),
         Some(kf_ToggleMistFog as unsafe extern "C" fn() -> ()),
         Some(kf_ToggleFogColour as unsafe extern "C" fn() -> ()),
         Some(kf_AddMissionOffWorld as unsafe extern "C" fn() -> ()),
         Some(kf_KillSelected as unsafe extern "C" fn() -> ()),
         Some(kf_ShowMappings as unsafe extern "C" fn() -> ()),
         Some(kf_GiveTemplateSet as unsafe extern "C" fn() -> ()),
         Some(kf_ToggleVisibility as unsafe extern "C" fn() -> ()),
         Some(kf_FinishResearch as unsafe extern "C" fn() -> ()),
         Some(kf_LowerTile as unsafe extern "C" fn() -> ()),
         Some(kf_ToggleDemoMode as unsafe extern "C" fn() -> ()),
         Some(kf_ToggleGodMode as unsafe extern "C" fn() -> ()),
         Some(kf_EndMissionOffWorld as unsafe extern "C" fn() -> ()),
         Some(kf_SystemClose as unsafe extern "C" fn() -> ()),
         Some(kf_ToggleShadows as unsafe extern "C" fn() -> ()), None]
    };
// ----------------------------------------------------------------------------------
/*
	Here is where we assign functions to keys and to combinations of keys.
	these will be read in from a .cfg file customisable by the player from
	an in-game menu
*/
#[no_mangle]
pub unsafe extern "C" fn keyInitMappings(mut bForceDefaults: BOOL) {
    let mut i: UDWORD = 0;
    keyMappings = 0 as *mut KEY_MAPPING;
    numActiveMappings = 0 as libc::c_int as UDWORD;
    bKeyProcessing = 1 as libc::c_int;
    processDebugMappings(0 as libc::c_int);
    i = 0 as libc::c_int as UDWORD;
    while i < 26 as libc::c_int as libc::c_uint {
        qwertyKeyMappings[i as usize].psMapping = 0 as *mut KEY_MAPPING;
        i = i.wrapping_add(1)
    }
    // load the mappings.
    if bForceDefaults == 0 && loadKeyMap() == 1 as libc::c_int { return }
    // mappings failed to load, so set the defaults.
    // ********************************* ALL THE MAPPINGS ARE NOW IN ORDER, PLEASE ****
	// ********************************* DO NOT REORDER THEM!!!!!! ********************
	/* ALL OF THIS NEEDS TO COME IN OFF A USER CUSTOMISABLE TEXT FILE */
	//                                **********************************
	//                                **********************************
	//									FUNCTION KEY MAPPINGS F1 to F12
    keyAddMapping(KEYMAP_ALWAYS_PROCESS, 5190 as KEY_CODE, KEY_F1,
                  KEYMAP_PRESSED,
                  Some(kf_ChooseManufacture as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_RET_MANUFACTURE as libc::c_int as
                                      UDWORD)); //Which key should we use? --Re enabled see below! -Q 5-10-05
    keyAddMapping(KEYMAP_ALWAYS_PROCESS, 5190 as KEY_CODE, KEY_F2,
                  KEYMAP_PRESSED,
                  Some(kf_ChooseResearch as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_RET_RESEARCH as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ALWAYS_PROCESS, 5190 as KEY_CODE, KEY_F3,
                  KEYMAP_PRESSED,
                  Some(kf_ChooseBuild as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_RET_BUILD as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ALWAYS_PROCESS, 5190 as KEY_CODE, KEY_F4,
                  KEYMAP_PRESSED,
                  Some(kf_ChooseDesign as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_RET_DESIGN as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ALWAYS_PROCESS, 5190 as KEY_CODE, KEY_F5,
                  KEYMAP_PRESSED,
                  Some(kf_ChooseIntelligence as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_RET_INTELLIGENCE as libc::c_int as
                                      UDWORD));
    keyAddMapping(KEYMAP_ALWAYS_PROCESS, 5190 as KEY_CODE, KEY_F6,
                  KEYMAP_PRESSED,
                  Some(kf_ChooseCommand as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_RET_COMMAND as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, 5190 as KEY_CODE, KEY_F7, KEYMAP_PRESSED,
                  Some(kf_ToggleRadar as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_TOGRAD as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, 5190 as KEY_CODE, KEY_F8, KEYMAP_PRESSED,
                  Some(kf_ToggleConsole as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_TOGCON as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, 5190 as KEY_CODE, KEY_F9, KEYMAP_PRESSED,
                  Some(kf_ToggleEnergyBars as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_BARS as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, 5190 as KEY_CODE, KEY_F10,
                  KEYMAP_PRESSED,
                  Some(kf_ScreenDump as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_SHOT as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, 5190 as KEY_CODE, KEY_F11,
                  KEYMAP_PRESSED,
                  Some(kf_ToggleFormationSpeedLimiting as
                           unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_SPLIM as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, 5190 as KEY_CODE, KEY_F12,
                  KEYMAP_PRESSED,
                  Some(kf_MoveToLastMessagePos as
                           unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_PREV as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, KEY_LSHIFT, KEY_F12, KEYMAP_PRESSED,
                  Some(kf_ToggleSensorDisplay as
                           unsafe extern "C" fn() -> ()),
                  b"Toggle Sensor display\x00" as *const u8 as
                      *const libc::c_char as *mut STRING);
    //                                **********************************
	//                                **********************************
	//										ASSIGN GROUPS
    keyAddMapping(KEYMAP_ASSIGNABLE, KEY_LCTRL, KEY_1, KEYMAP_PRESSED,
                  Some(kf_AssignGrouping_1 as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_AS1 as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, KEY_LCTRL, KEY_2, KEYMAP_PRESSED,
                  Some(kf_AssignGrouping_2 as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_AS2 as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, KEY_LCTRL, KEY_3, KEYMAP_PRESSED,
                  Some(kf_AssignGrouping_3 as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_AS3 as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, KEY_LCTRL, KEY_4, KEYMAP_PRESSED,
                  Some(kf_AssignGrouping_4 as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_AS4 as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, KEY_LCTRL, KEY_5, KEYMAP_PRESSED,
                  Some(kf_AssignGrouping_5 as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_AS5 as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, KEY_LCTRL, KEY_6, KEYMAP_PRESSED,
                  Some(kf_AssignGrouping_6 as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_AS6 as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, KEY_LCTRL, KEY_7, KEYMAP_PRESSED,
                  Some(kf_AssignGrouping_7 as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_AS7 as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, KEY_LCTRL, KEY_8, KEYMAP_PRESSED,
                  Some(kf_AssignGrouping_8 as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_AS8 as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, KEY_LCTRL, KEY_9, KEYMAP_PRESSED,
                  Some(kf_AssignGrouping_9 as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_AS9 as libc::c_int as UDWORD));
    //                                **********************************
	//                                **********************************
	//	SELECT GROUPS - Will jump to the group as well as select if group is ALREADY selected
    keyAddMapping(KEYMAP_ASSIGNABLE, 5190 as KEY_CODE, KEY_1, KEYMAP_PRESSED,
                  Some(kf_SelectGrouping_1 as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_GR1 as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, 5190 as KEY_CODE, KEY_2, KEYMAP_PRESSED,
                  Some(kf_SelectGrouping_2 as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_GR2 as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, 5190 as KEY_CODE, KEY_3, KEYMAP_PRESSED,
                  Some(kf_SelectGrouping_3 as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_GR3 as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, 5190 as KEY_CODE, KEY_4, KEYMAP_PRESSED,
                  Some(kf_SelectGrouping_4 as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_GR4 as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, 5190 as KEY_CODE, KEY_5, KEYMAP_PRESSED,
                  Some(kf_SelectGrouping_5 as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_GR5 as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, 5190 as KEY_CODE, KEY_6, KEYMAP_PRESSED,
                  Some(kf_SelectGrouping_6 as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_GR6 as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, 5190 as KEY_CODE, KEY_7, KEYMAP_PRESSED,
                  Some(kf_SelectGrouping_7 as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_GR7 as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, 5190 as KEY_CODE, KEY_8, KEYMAP_PRESSED,
                  Some(kf_SelectGrouping_8 as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_GR8 as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, 5190 as KEY_CODE, KEY_9, KEYMAP_PRESSED,
                  Some(kf_SelectGrouping_9 as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_GR9 as libc::c_int as UDWORD));
    //                                **********************************
	//                                **********************************
	//	SELECT COMMANDER - Will jump to the group as well as select if group is ALREADY selected
    keyAddMapping(KEYMAP_ASSIGNABLE, KEY_LALT, KEY_1, KEYMAP_PRESSED,
                  Some(kf_SelectCommander_1 as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_CMD1 as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, KEY_LALT, KEY_2, KEYMAP_PRESSED,
                  Some(kf_SelectCommander_2 as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_CMD2 as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, KEY_LALT, KEY_3, KEYMAP_PRESSED,
                  Some(kf_SelectCommander_3 as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_CMD3 as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, KEY_LALT, KEY_4, KEYMAP_PRESSED,
                  Some(kf_SelectCommander_4 as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_CMD4 as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, KEY_LALT, KEY_5, KEYMAP_PRESSED,
                  Some(kf_SelectCommander_5 as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_CMD5 as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, KEY_LALT, KEY_6, KEYMAP_PRESSED,
                  Some(kf_SelectCommander_6 as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_CMD6 as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, KEY_LALT, KEY_7, KEYMAP_PRESSED,
                  Some(kf_SelectCommander_7 as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_CMD7 as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, KEY_LALT, KEY_8, KEYMAP_PRESSED,
                  Some(kf_SelectCommander_8 as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_CMD8 as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, KEY_LALT, KEY_9, KEYMAP_PRESSED,
                  Some(kf_SelectCommander_9 as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_CMD9 as libc::c_int as UDWORD));
    //                                **********************************
	//                                **********************************
	//	MULTIPLAYER
    keyAddMapping(KEYMAP_ASSIGNABLE, 5190 as KEY_CODE, KEY_KPENTER,
                  KEYMAP_PRESSED,
                  Some(kf_addMultiMenu as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_MULOP as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ALWAYS, 5190 as KEY_CODE, KEY_KP_FULLSTOP,
                  KEYMAP_PRESSED,
                  Some(kf_multiAudioStart as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_AUDON as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ALWAYS, 5190 as KEY_CODE, KEY_KP_FULLSTOP,
                  KEYMAP_RELEASED,
                  Some(kf_multiAudioStop as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_AUDOFF as libc::c_int as UDWORD));
    //
	//	GAME CONTROLS - Moving around, zooming in, rotating etc
    keyAddMapping(KEYMAP_ASSIGNABLE, 5190 as KEY_CODE, KEY_BACKSPACE,
                  KEYMAP_PRESSED,
                  Some(kf_SeekNorth as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_NORTH as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, 5190 as KEY_CODE, KEY_SPACE,
                  KEYMAP_PRESSED,
                  Some(kf_ToggleCamera as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_TRACK as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ALWAYS, 5190 as KEY_CODE, KEY_ESC, KEYMAP_PRESSED,
                  Some(kf_addInGameOptions as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_OPT as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, 5190 as KEY_CODE, KEY_MINUS,
                  KEYMAP_PRESSED,
                  Some(kf_RadarZoomOut as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_RIN as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, 5190 as KEY_CODE, KEY_EQUALS,
                  KEYMAP_PRESSED,
                  Some(kf_RadarZoomIn as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_ROUT as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, 5190 as KEY_CODE, KEY_KP_MINUS,
                  KEYMAP_DOWN,
                  Some(kf_ZoomOut as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_ZOUT as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, 5190 as KEY_CODE, KEY_KP_PLUS,
                  KEYMAP_DOWN,
                  Some(kf_ZoomIn as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_ZIN as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, 5190 as KEY_CODE, KEY_KP_2, KEYMAP_DOWN,
                  Some(kf_PitchForward as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_PF as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, 5190 as KEY_CODE, KEY_KP_4, KEYMAP_DOWN,
                  Some(kf_RotateLeft as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_RL as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, 5190 as KEY_CODE, KEY_KP_5, KEYMAP_DOWN,
                  Some(kf_ResetPitch as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_RP as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, 5190 as KEY_CODE, KEY_KP_6, KEYMAP_DOWN,
                  Some(kf_RotateRight as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_RR as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, 5190 as KEY_CODE, KEY_KP_8, KEYMAP_DOWN,
                  Some(kf_PitchBack as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_PB as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, 5190 as KEY_CODE, KEY_KP_0,
                  KEYMAP_PRESSED,
                  Some(kf_RightOrderMenu as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_ORD as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, KEY_LCTRL, KEY_MINUS, KEYMAP_PRESSED,
                  Some(kf_SlowDown as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_SLOW_DOWN as libc::c_int as
                                      UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, KEY_LCTRL, KEY_EQUALS, KEYMAP_PRESSED,
                  Some(kf_SpeedUp as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_SPEED_UP as libc::c_int as
                                      UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, KEY_LCTRL, KEY_BACKSPACE, KEYMAP_PRESSED,
                  Some(kf_NormalSpeed as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_NORMAL_SPEED as libc::c_int as
                                      UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, KEY_LCTRL, KEY_UPARROW, KEYMAP_PRESSED,
                  Some(kf_FaceNorth as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_UP as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, KEY_LCTRL, KEY_DOWNARROW, KEYMAP_PRESSED,
                  Some(kf_FaceSouth as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_DOWN as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, KEY_LCTRL, KEY_LEFTARROW, KEYMAP_PRESSED,
                  Some(kf_FaceEast as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_RIGHT as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, KEY_LCTRL, KEY_RIGHTARROW,
                  KEYMAP_PRESSED,
                  Some(kf_FaceWest as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_LEFT as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, 5190 as KEY_CODE, KEY_KP_STAR,
                  KEYMAP_PRESSED,
                  Some(kf_JumpToResourceExtractor as
                           unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_RESJ as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, 5190 as KEY_CODE,
                  SDLK_LAST as libc::c_int as KEY_CODE, KEYMAP_PRESSED,
                  Some(kf_JumpToRepairUnits as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_REPJ as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, 5190 as KEY_CODE,
                  SDLK_LAST as libc::c_int as KEY_CODE, KEYMAP_PRESSED,
                  Some(kf_JumpToConstructorUnits as
                           unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_CONJ as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, 5190 as KEY_CODE,
                  SDLK_LAST as libc::c_int as KEY_CODE, KEYMAP_PRESSED,
                  Some(kf_JumpToSensorUnits as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_SENJ as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, 5190 as KEY_CODE,
                  SDLK_LAST as libc::c_int as KEY_CODE, KEYMAP_PRESSED,
                  Some(kf_JumpToCommandUnits as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_COMJ as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, 5190 as KEY_CODE, KEY_TAB,
                  KEYMAP_PRESSED,
                  Some(kf_ToggleOverlays as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_OVERL as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ALWAYS, 5190 as KEY_CODE, KEY_BACKQUOTE,
                  KEYMAP_PRESSED,
                  Some(kf_ToggleConsoleDrop as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_CONSOLE as libc::c_int as UDWORD));
    //                                **********************************
	// IN GAME MAPPINGS - Single key presses - ALL __DEBUG keymappings will be removed for master
    keyAddMapping(KEYMAP_ASSIGNABLE, 5190 as KEY_CODE, KEY_B, KEYMAP_PRESSED,
                  Some(kf_CentreOnBase as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_CENTV as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, 5190 as KEY_CODE, KEY_C, KEYMAP_PRESSED,
                  Some(kf_SetDroidAttackCease as
                           unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_CEASE as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, 5190 as KEY_CODE, KEY_D, KEYMAP_PRESSED,
                  Some(kf_JumpToUnassignedUnits as
                           unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_UNITJ as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, 5190 as KEY_CODE, KEY_E, KEYMAP_PRESSED,
                  Some(kf_SetDroidAttackReturn as
                           unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_ENGAG as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, 5190 as KEY_CODE, KEY_F, KEYMAP_PRESSED,
                  Some(kf_SetDroidAttackAtWill as
                           unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_FAW as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, 5190 as KEY_CODE, KEY_H, KEYMAP_PRESSED,
                  Some(kf_SetDroidReturnToBase as
                           unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_RTB as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, 5190 as KEY_CODE, KEY_I, KEYMAP_PRESSED,
                  Some(kf_SetDroidRangeDefault as
                           unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_DEFR as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, 5190 as KEY_CODE, KEY_O, KEYMAP_PRESSED,
                  Some(kf_SetDroidRangeShort as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_SHOR as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, 5190 as KEY_CODE, KEY_P, KEYMAP_PRESSED,
                  Some(kf_SetDroidMovePursue as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_PURS as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, 5190 as KEY_CODE, KEY_Q, KEYMAP_PRESSED,
                  Some(kf_SetDroidMovePatrol as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_PATR as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, 5190 as KEY_CODE, KEY_R, KEYMAP_PRESSED,
                  Some(kf_SetDroidGoForRepair as
                           unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_REPA as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, 5190 as KEY_CODE, KEY_S, KEYMAP_PRESSED,
                  Some(kf_SetDroidMoveHold as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_DSTOP as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, 5190 as KEY_CODE, KEY_T, KEYMAP_PRESSED,
                  Some(kf_SendTextMessage as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_SENDT as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, 5190 as KEY_CODE, KEY_U, KEYMAP_PRESSED,
                  Some(kf_SetDroidRangeLong as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_LONGR as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ALWAYS, 5190 as KEY_CODE, KEY_Z, KEYMAP_PRESSED,
                  Some(kf_SensorDisplayOn as unsafe extern "C" fn() -> ()),
                  b"Sensor display On\x00" as *const u8 as *const libc::c_char
                      as *mut STRING);
    keyAddMapping(KEYMAP_ALWAYS, 5190 as KEY_CODE, KEY_Z, KEYMAP_RELEASED,
                  Some(kf_SensorDisplayOff as unsafe extern "C" fn() -> ()),
                  b"Sensor display Off\x00" as *const u8 as
                      *const libc::c_char as *mut STRING);
    keyAddMapping(KEYMAP_ASSIGNABLE, KEY_LALT, KEY_S, KEYMAP_PRESSED,
                  Some(kf_ToggleShadows as unsafe extern "C" fn() -> ()),
                  b"Toggles shadows\x00" as *const u8 as *const libc::c_char
                      as *mut STRING);
    keyAddMapping(KEYMAP_ASSIGNABLE, KEY_LCTRL, KEY_TAB, KEYMAP_PRESSED,
                  Some(kf_ToggleRadarTerrain as unsafe extern "C" fn() -> ()),
                  b"Toggle radar terrain\x00" as *const u8 as
                      *const libc::c_char as *mut STRING);
    keyAddMapping(KEYMAP_ASSIGNABLE, KEY_LSHIFT, KEY_TAB, KEYMAP_PRESSED,
                  Some(kf_ToggleRadarAllyEnemy as
                           unsafe extern "C" fn() -> ()),
                  b"Toggle ally-enemy radar view\x00" as *const u8 as
                      *const libc::c_char as *mut STRING);
    // Some extra non QWERTY mappings but functioning in same way
    keyAddMapping(KEYMAP_ASSIGNABLE, 5190 as KEY_CODE, KEY_COMMA,
                  KEYMAP_PRESSED,
                  Some(kf_SetDroidRetreatMedium as
                           unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_LDAM as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, 5190 as KEY_CODE, KEY_FULLSTOP,
                  KEYMAP_PRESSED,
                  Some(kf_SetDroidRetreatHeavy as
                           unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_HDAM as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, 5190 as KEY_CODE, KEY_FORWARDSLASH,
                  KEYMAP_PRESSED,
                  Some(kf_SetDroidRetreatNever as
                           unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_NDAM as libc::c_int as UDWORD));
    //                                **********************************
	//                                **********************************
	//								In game mappings - COMBO (CTRL + LETTER) presses.
    keyAddMapping(KEYMAP_ASSIGNABLE, KEY_LCTRL, KEY_A, KEYMAP_PRESSED,
                  Some(kf_SelectAllCombatUnits as
                           unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_ACU as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, KEY_LCTRL, KEY_D, KEYMAP_PRESSED,
                  Some(kf_SelectAllDamaged as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_ABDU as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, KEY_LCTRL, KEY_F, KEYMAP_PRESSED,
                  Some(kf_SelectAllHalfTracked as
                           unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_AHTR as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, KEY_LCTRL, KEY_H, KEYMAP_PRESSED,
                  Some(kf_SelectAllHovers as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_AHOV as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, KEY_LCTRL, KEY_R, KEYMAP_PRESSED,
                  Some(kf_SetDroidRecycle as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_RECY as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, KEY_LCTRL, KEY_S, KEYMAP_PRESSED,
                  Some(kf_SelectAllOnScreenUnits as
                           unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_ASCR as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, KEY_LCTRL, KEY_T, KEYMAP_PRESSED,
                  Some(kf_SelectAllTracked as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_ATR as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, KEY_LCTRL, KEY_U, KEYMAP_PRESSED,
                  Some(kf_SelectAllUnits as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_ALL as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, KEY_LCTRL, KEY_V, KEYMAP_PRESSED,
                  Some(kf_SelectAllVTOLs as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_AVTOL as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, KEY_LCTRL, KEY_W, KEYMAP_PRESSED,
                  Some(kf_SelectAllWheeled as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_AWHE as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP__DEBUG, KEY_LCTRL, KEY_Y, KEYMAP_PRESSED,
                  Some(kf_FrameRate as unsafe extern "C" fn() -> ()),
                  b"Show frame rate\x00" as *const u8 as *const libc::c_char
                      as *mut STRING);
    keyAddMapping(KEYMAP_ASSIGNABLE, KEY_LCTRL, KEY_Z, KEYMAP_PRESSED,
                  Some(kf_SelectAllSameType as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_ASIMIL as libc::c_int as UDWORD));
    //                                **********************************
	//                                **********************************
	//									SELECT PLAYERS - DEBUG ONLY
    keyAddMapping(KEYMAP_ASSIGNABLE, 5190 as KEY_CODE,
                  SDLK_LAST as libc::c_int as KEY_CODE, KEYMAP_PRESSED,
                  Some(kf_SelectNextFactory as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_SELFACTORY as libc::c_int as
                                      UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, 5190 as KEY_CODE,
                  SDLK_LAST as libc::c_int as KEY_CODE, KEYMAP_PRESSED,
                  Some(kf_SelectNextResearch as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_SELRESEARCH as libc::c_int as
                                      UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, 5190 as KEY_CODE,
                  SDLK_LAST as libc::c_int as KEY_CODE, KEYMAP_PRESSED,
                  Some(kf_SelectNextPowerStation as
                           unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_SELPOWER as libc::c_int as
                                      UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, 5190 as KEY_CODE,
                  SDLK_LAST as libc::c_int as KEY_CODE, KEYMAP_PRESSED,
                  Some(kf_SelectNextCyborgFactory as
                           unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_SELCYBORG as libc::c_int as
                                      UDWORD));
    keyAddMapping(KEYMAP_ASSIGNABLE, 5190 as KEY_CODE,
                  SDLK_LAST as libc::c_int as KEY_CODE, KEYMAP_PRESSED,
                  Some(kf_ToggleReopenBuildMenu as
                           unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_REOPEN_BUILD as libc::c_int as
                                      UDWORD));
    // NOTE THIS!!!!!!!
    keyAddMapping(KEYMAP__DEBUG, 5190 as KEY_CODE, KEY_A, KEYMAP_PRESSED,
                  Some(kf_AllAvailable as unsafe extern "C" fn() -> ()),
                  b"Make all items available\x00" as *const u8 as
                      *const libc::c_char as *mut STRING);
    keyAddMapping(KEYMAP___HIDE, KEY_LSHIFT, KEY_BACKSPACE, KEYMAP_PRESSED,
                  Some(kf_ToggleDebugMappings as
                           unsafe extern "C" fn() -> ()),
                  b"TOGGLE Debug Mappings\x00" as *const u8 as
                      *const libc::c_char as *mut STRING);
    keyAddMapping(KEYMAP__DEBUG, KEY_LCTRL, KEY_X, KEYMAP_PRESSED,
                  Some(kf_FinishResearch as unsafe extern "C" fn() -> ()),
                  b"Complete current research\x00" as *const u8 as
                      *const libc::c_char as *mut STRING);
    keyAddMapping(KEYMAP__DEBUG, 5190 as KEY_CODE, KEY_SCROLLLOCK,
                  KEYMAP_PRESSED,
                  Some(kf_TogglePauseMode as unsafe extern "C" fn() -> ()),
                  strresGetString(psStringRes,
                                  STR_BIND_PAUSE as libc::c_int as UDWORD));
    keyAddMapping(KEYMAP__DEBUG, 5190 as KEY_CODE, KEY_J, KEYMAP_PRESSED,
                  Some(kf_MaxScrollLimits as unsafe extern "C" fn() -> ()),
                  b"Maximum scroll limits\x00" as *const u8 as
                      *const libc::c_char as *mut STRING);
    keyAddMapping(KEYMAP__DEBUG, 5190 as KEY_CODE, KEY_K, KEYMAP_PRESSED,
                  Some(kf_KillSelected as unsafe extern "C" fn() -> ()),
                  b"Kill Selected Unit(s)\x00" as *const u8 as
                      *const libc::c_char as *mut STRING);
    keyAddMapping(KEYMAP__DEBUG, 5190 as KEY_CODE, KEY_M, KEYMAP_PRESSED,
                  Some(kf_ShowMappings as unsafe extern "C" fn() -> ()),
                  b"Show all keyboard mappings - use pause!\x00" as *const u8
                      as *const libc::c_char as *mut STRING);
    keyAddMapping(KEYMAP__DEBUG, 5190 as KEY_CODE, KEY_N, KEYMAP_PRESSED,
                  Some(kf_GiveTemplateSet as unsafe extern "C" fn() -> ()),
                  b"Give template set(s) to player 0 \x00" as *const u8 as
                      *const libc::c_char as *mut STRING);
    keyAddMapping(KEYMAP__DEBUG, 5190 as KEY_CODE, KEY_V, KEYMAP_PRESSED,
                  Some(kf_ToggleVisibility as unsafe extern "C" fn() -> ()),
                  b"Toggle visibility\x00" as *const u8 as *const libc::c_char
                      as *mut STRING);
    keyAddMapping(KEYMAP__DEBUG, 5190 as KEY_CODE, KEY_W, KEYMAP_DOWN,
                  Some(kf_LowerTile as unsafe extern "C" fn() -> ()),
                  b"Lower tile height\x00" as *const u8 as *const libc::c_char
                      as *mut STRING);
    keyAddMapping(KEYMAP__DEBUG, 5190 as KEY_CODE, KEY_Y, KEYMAP_PRESSED,
                  Some(kf_ToggleDemoMode as unsafe extern "C" fn() -> ()),
                  b"Toggles on/off DEMO Mode\x00" as *const u8 as
                      *const libc::c_char as *mut STRING);
    //	keyAddMapping(KEYMAP__DEBUG,KEY_IGNORE,KEY_Z,KEYMAP_PRESSED,kf_ToggleSensorDisplay,			"Toggle Sensor display");
    keyAddMapping(KEYMAP__DEBUG, KEY_LCTRL, KEY_B, KEYMAP_PRESSED,
                  Some(kf_EndMissionOffWorld as unsafe extern "C" fn() -> ()),
                  b"End Mission\x00" as *const u8 as *const libc::c_char as
                      *mut STRING);
    keyAddMapping(KEYMAP__DEBUG, KEY_LCTRL, KEY_KP_MINUS, KEYMAP_PRESSED,
                  Some(kf_SystemClose as unsafe extern "C" fn() -> ()),
                  b"System Close (EXIT)\x00" as *const u8 as
                      *const libc::c_char as *mut STRING);
    keyAddMapping(KEYMAP__DEBUG, KEY_LCTRL, KEY_E, KEYMAP_PRESSED,
                  Some(kf_DebugDroidInfo as unsafe extern "C" fn() -> ()),
                  b"Show unit info\x00" as *const u8 as *const libc::c_char as
                      *mut STRING);
    keyAddMapping(KEYMAP__DEBUG, KEY_LCTRL, KEY_G, KEYMAP_PRESSED,
                  Some(kf_ToggleGodMode as unsafe extern "C" fn() -> ()),
                  b"Toggle god Mode Status\x00" as *const u8 as
                      *const libc::c_char as *mut STRING);
    keyAddMapping(KEYMAP__DEBUG, KEY_LCTRL, KEY_I, KEYMAP_PRESSED,
                  Some(kf_RecalcLighting as unsafe extern "C" fn() -> ()),
                  b"Recalculate lighting\x00" as *const u8 as
                      *const libc::c_char as *mut STRING);
    keyAddMapping(KEYMAP__DEBUG, KEY_LCTRL, KEY_J, KEYMAP_PRESSED,
                  Some(kf_ToggleFog as unsafe extern "C" fn() -> ()),
                  b"Toggles All fog\x00" as *const u8 as *const libc::c_char
                      as *mut STRING);
    keyAddMapping(KEYMAP__DEBUG, KEY_LCTRL, KEY_K, KEYMAP_PRESSED,
                  Some(kf_ToggleMistFog as unsafe extern "C" fn() -> ()),
                  b"Toggle Mist Fog\x00" as *const u8 as *const libc::c_char
                      as *mut STRING);
    keyAddMapping(KEYMAP__DEBUG, KEY_LCTRL, KEY_L, KEYMAP_PRESSED,
                  Some(kf_ToggleFogColour as unsafe extern "C" fn() -> ()),
                  b"Toggle Fog Colour Fog\x00" as *const u8 as
                      *const libc::c_char as *mut STRING);
    keyAddMapping(KEYMAP__DEBUG, KEY_LCTRL, KEY_M, KEYMAP_PRESSED,
                  Some(kf_AddMissionOffWorld as unsafe extern "C" fn() -> ()),
                  b"Add Mission - Keep\x00" as *const u8 as
                      *const libc::c_char as *mut STRING);
    keyAddMapping(KEYMAP__DEBUG, KEY_LCTRL, KEY_N, KEYMAP_PRESSED,
                  Some(kf_NewPlayerPower as unsafe extern "C" fn() -> ()),
                  b"New game player power\x00" as *const u8 as
                      *const libc::c_char as *mut STRING);
    keyAddMapping(KEYMAP__DEBUG, KEY_LCTRL, KEY_O, KEYMAP_PRESSED,
                  Some(kf_ChooseOptions as unsafe extern "C" fn() -> ()),
                  b"Display Options Screen\x00" as *const u8 as
                      *const libc::c_char as *mut STRING);
    keyAddMapping(KEYMAP__DEBUG, KEY_LCTRL, KEY_P, KEYMAP_PRESSED,
                  Some(kf_TogglePower as unsafe extern "C" fn() -> ()),
                  b"Infinite power\x00" as *const u8 as *const libc::c_char as
                      *mut STRING);
    keyAddMapping(KEYMAP__DEBUG, KEY_LCTRL, KEY_Q, KEYMAP_PRESSED,
                  Some(kf_ToggleWeather as unsafe extern "C" fn() -> ()),
                  b"Trigger some weather\x00" as *const u8 as
                      *const libc::c_char as *mut STRING);
    keyAddMapping(KEYMAP__DEBUG, KEY_LALT, KEY_F1, KEYMAP_PRESSED,
                  Some(kf_SelectPlayer as unsafe extern "C" fn() -> ()),
                  b"Select player  0\x00" as *const u8 as *const libc::c_char
                      as *mut STRING);
    keyAddMapping(KEYMAP__DEBUG, KEY_LALT, KEY_F2, KEYMAP_PRESSED,
                  Some(kf_SelectPlayer as unsafe extern "C" fn() -> ()),
                  b"Select player  1\x00" as *const u8 as *const libc::c_char
                      as *mut STRING);
    keyAddMapping(KEYMAP__DEBUG, KEY_LALT, KEY_F3, KEYMAP_PRESSED,
                  Some(kf_SelectPlayer as unsafe extern "C" fn() -> ()),
                  b"Select player  2\x00" as *const u8 as *const libc::c_char
                      as *mut STRING);
    keyAddMapping(KEYMAP__DEBUG, KEY_LALT, KEY_F4, KEYMAP_PRESSED,
                  Some(kf_SelectPlayer as unsafe extern "C" fn() -> ()),
                  b"Select player  3\x00" as *const u8 as *const libc::c_char
                      as *mut STRING);
    keyAddMapping(KEYMAP__DEBUG, KEY_LALT, KEY_F5, KEYMAP_PRESSED,
                  Some(kf_SelectPlayer as unsafe extern "C" fn() -> ()),
                  b"Select player  4\x00" as *const u8 as *const libc::c_char
                      as *mut STRING);
    keyAddMapping(KEYMAP__DEBUG, KEY_LALT, KEY_F6, KEYMAP_PRESSED,
                  Some(kf_SelectPlayer as unsafe extern "C" fn() -> ()),
                  b"Select player  5\x00" as *const u8 as *const libc::c_char
                      as *mut STRING);
    keyAddMapping(KEYMAP__DEBUG, KEY_LALT, KEY_F7, KEYMAP_PRESSED,
                  Some(kf_SelectPlayer as unsafe extern "C" fn() -> ()),
                  b"Select player  6\x00" as *const u8 as *const libc::c_char
                      as *mut STRING);
    keyAddMapping(KEYMAP__DEBUG, KEY_LALT, KEY_F8, KEYMAP_PRESSED,
                  Some(kf_SelectPlayer as unsafe extern "C" fn() -> ()),
                  b"Select player  7\x00" as *const u8 as *const libc::c_char
                      as *mut STRING);
    saveKeyMap();
    // save out the default key mappings.
    //  ------------------------ OLD STUFF - Store here!
	/*
	keyAddMapping(KEYMAP__DEBUG,KEY_IGNORE,KEY_F6,KEYMAP_DOWN,kf_UpGeoOffset,"Raise the geometric offset");
	keyAddMapping(KEYMAP__DEBUG,KEY_IGNORE,KEY_F7,KEYMAP_DOWN,kf_DownGeoOffset,"Lower the geometric offset");
	keyAddMapping(KEYMAP__DEBUG,KEY_IGNORE,KEY_F8,KEYMAP_DOWN,kf_UpDroidScale,"Increase droid Scaling");
	keyAddMapping(KEYMAP__DEBUG,KEY_IGNORE,KEY_F9,KEYMAP_DOWN,kf_DownDroidScale,"Decrease droid Scaling");
	keyAddMapping(KEYMAP__DEBUG,KEY_IGNORE,KEY_A,KEYMAP_PRESSED,kf_AllAvailable,"Make all avilable");
	keyAddMapping(KEYMAP__DEBUG,KEY_IGNORE,KEY_B,KEYMAP_PRESSED,kf_MaxScrollLimits,"Allows full area map viewing");
	keyAddMapping(KEYMAP__DEBUG,KEY_IGNORE,KEY_C,KEYMAP_PRESSED,kf_SimCloseDown,"Simulate Screen Close Down");
	keyAddMapping(KEYMAP_ALWAYS,KEY_IGNORE,KEY_D,KEYMAP_PRESSED,kf_ToggleDrivingMode,"Toggle Driving Mode");
	keyAddMapping(KEYMAP__DEBUG,KEY_IGNORE,KEY_E,KEYMAP_PRESSED,kf_ToggleDroidInfo,"Display droid info whilst tracking");
	keyAddMapping(KEYMAP__DEBUG,KEY_IGNORE,KEY_F,KEYMAP_PRESSED,kf_TriFlip,"Flip terrain triangle");
	keyAddMapping(KEYMAP__DEBUG,KEY_IGNORE,KEY_I,KEYMAP_PRESSED,kf_ToggleWidgets,"Toggle Widgets");
	keyAddMapping(KEYMAP__DEBUG,KEY_IGNORE,KEY_J,KEYMAP_PRESSED,kf_ToggleRadarAllign,"Toggles Radar allignment");
	keyAddMapping(KEYMAP__DEBUG,KEY_IGNORE,KEY_K,KEYMAP_PRESSED,kf_KillSelected,"Kill Selected Droid");
	keyAddMapping(KEYMAP__DEBUG,KEY_IGNORE,KEY_L,KEYMAP_PRESSED,kf_RecalcLighting,"Recalculate Lighting");
	keyAddMapping(KEYMAP__DEBUG,KEY_IGNORE,KEY_M,KEYMAP_PRESSED,kf_ShowMappings,"Show Keyboard mappings");
	keyAddMapping(KEYMAP__DEBUG,KEY_IGNORE,KEY_N,KEYMAP_PRESSED,kf_GiveTemplateSet,"Give Template Set");
	keyAddMapping(KEYMAP__DEBUG,KEY_IGNORE,KEY_O,KEYMAP_PRESSED,kf_ToggleOutline,"Tile Outline");
	keyAddMapping(KEYMAP__DEBUG,KEY_IGNORE,KEY_P,KEYMAP_PRESSED,kf_TogglePower,"Infinite power");
	keyAddMapping(KEYMAP__DEBUG,KEY_IGNORE,KEY_Q,KEYMAP_DOWN,kf_RaiseTile,"Raise tile height");
	keyAddMapping(KEYMAP__DEBUG,KEY_IGNORE,KEY_R,KEYMAP_PRESSED,kf_ShowNumObjects,"Show number of Objects");
	keyAddMapping(KEYMAP__DEBUG,KEY_IGNORE,KEY_S,KEYMAP_PRESSED,kf_FrameRate,"Show Frame Rate");
	keyAddMapping(KEYMAP_ALWAYS,KEY_IGNORE,KEY_T,KEYMAP_PRESSED,kf_SendTextMessage,"Send Text Message");
	keyAddMapping(KEYMAP_ALWAYS,KEY_IGNORE,KEY_U,KEYMAP_PRESSED,kf_ToggleBackgroundFog,"Toggle Background Fog");
	keyAddMapping(KEYMAP__DEBUG,KEY_IGNORE,KEY_V,KEYMAP_PRESSED,kf_BuildInfo,"Build date and time");
	keyAddMapping(KEYMAP__DEBUG,KEY_IGNORE,KEY_W,KEYMAP_DOWN,kf_LowerTile,"Lower tile height");
	keyAddMapping(KEYMAP__DEBUG,KEY_IGNORE,KEY_X,KEYMAP_PRESSED,kf_DebugDroidInfo,"Droid Debug Info");
	keyAddMapping(KEYMAP__DEBUG,KEY_IGNORE,KEY_Y,KEYMAP_PRESSED,kf_ToggleDemoMode,"Toggles on/off DEMO Mode");
	keyAddMapping(KEYMAP__DEBUG,KEY_IGNORE,KEY_Z,KEYMAP_PRESSED,kf_ShowGridInfo,"DBPRINTF map grid coverage");
	*/
    //  ------------------------ OLD STUFF - Store here!
}
//extern BOOL	keyAddMapping			( UDWORD functionId, KEY_CODE metaCode, KEY_CODE subCode );
//extern BOOL	keyAddMapping			( KEY_CODE metaCode, KEY_CODE subcode, KEY_ACTION action, void *function, STRING *name );
/*
	KeyMap.c
	Alex McLean
	Pumpkin Studios, EIDOS Interactive.
	Internal Use Only
	-----------------

	Handles the assignment of functions to keys.
*/
// ----------------------------------------------------------------------------------
/* Function Prototypes */
// ----------------------------------------------------------------------------------
/* Adds a new mapping to the list */
//BOOL	keyAddMapping(KEY_CODE metaCode, KEY_CODE subCode, KEY_ACTION action,void *function, STRING *name)
#[no_mangle]
pub unsafe extern "C" fn keyAddMapping(mut status: KEY_STATUS,
                                       mut metaCode: KEY_CODE,
                                       mut subCode: KEY_CODE,
                                       mut action: KEY_ACTION,
                                       mut pKeyMapFunc:
                                           Option<unsafe extern "C" fn()
                                                      -> ()>,
                                       mut name: *mut STRING)
 -> *mut KEY_MAPPING {
    let mut newMapping: *mut KEY_MAPPING = 0 as *mut KEY_MAPPING;
    let mut psHeap: *mut BLOCK_HEAP = 0 as *mut BLOCK_HEAP;
    psHeap = memGetBlockHeap();
    memSetBlockHeap(0 as *mut _block_heap);
    /* Get some memory for our binding */
    newMapping =
        memMallocRelease(::std::mem::size_of::<KEY_MAPPING>() as
                             libc::c_ulong) as *mut KEY_MAPPING;
    if newMapping as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Couldn\'t allocate memory for a key mapping\x00" as *const u8
                  as *const libc::c_char);
    };
    if newMapping as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"keymap.c\x00" as *const u8 as *const libc::c_char,
              515 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"keyAddMapping\x00")).as_ptr(),
              b"(int)newMapping\x00" as *const u8 as *const libc::c_char);
    };
    /* Plus one for the terminator */
    (*newMapping).pName =
        memMallocRelease(strlen(name).wrapping_add(1 as libc::c_int as
                                                       libc::c_uint)) as
            *mut STRING;
    if (*newMapping).pName as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Couldn\'t allocate the memory for the string in a mapping\x00"
                  as *const u8 as *const libc::c_char);
    };
    if (*newMapping).pName as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"keymap.c\x00" as *const u8 as *const libc::c_char,
              522 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"keyAddMapping\x00")).as_ptr(),
              b"(int)newMapping->pName\x00" as *const u8 as
                  *const libc::c_char);
    };
    memSetBlockHeap(psHeap);
    /* Copy over the name */
    strcpy((*newMapping).pName, name);
    /* Fill up our entries, first the ones that activate it */
    (*newMapping).metaKeyCode = metaCode;
    (*newMapping).subKeyCode = subCode;
    (*newMapping).status = status;
    /* When it was last called - needed? */
    (*newMapping).lastCalled = gameTime;
    /* And what gets called when it's activated */
	//newMapping->function	= function;
    (*newMapping).function = pKeyMapFunc;
    /* Is it functional on the key being down or just pressed */
    (*newMapping).action = action;
    (*newMapping).altMetaKeyCode = 5190 as KEY_CODE;
    /* We always request only the left hand one */
    if metaCode as libc::c_uint == KEY_LCTRL as libc::c_int as libc::c_uint {
        (*newMapping).altMetaKeyCode = KEY_RCTRL
    } else if metaCode as libc::c_uint ==
                  KEY_LALT as libc::c_int as libc::c_uint {
        (*newMapping).altMetaKeyCode = KEY_RALT
    } else if metaCode as libc::c_uint ==
                  KEY_LSHIFT as libc::c_int as libc::c_uint {
        (*newMapping).altMetaKeyCode = KEY_RSHIFT
    }
    /* Set it to be active */
    (*newMapping).active = 1 as libc::c_int;
    /* Add it to the start of the list */
    (*newMapping).psNext = keyMappings;
    keyMappings = newMapping;
    numActiveMappings = numActiveMappings.wrapping_add(1);
    return newMapping;
}
// ----------------------------------------------------------------------------------
/* Removes a mapping from the list specified by the key codes */
#[no_mangle]
pub unsafe extern "C" fn keyRemoveMapping(mut metaCode: KEY_CODE,
                                          mut subCode: KEY_CODE) -> BOOL {
    let mut mapping: *mut KEY_MAPPING = 0 as *mut KEY_MAPPING;
    mapping = keyFindMapping(metaCode, subCode);
    return keyRemoveMappingPt(mapping);
}
// ----------------------------------------------------------------------------------
/* Returns a pointer to a mapping if it exists - NULL otherwise */
#[no_mangle]
pub unsafe extern "C" fn keyFindMapping(mut metaCode: KEY_CODE,
                                        mut subCode: KEY_CODE)
 -> *mut KEY_MAPPING {
    let mut psCurr: *mut KEY_MAPPING = 0 as *mut KEY_MAPPING;
    /* See if we can find it */
    psCurr = keyMappings;
    while !psCurr.is_null() {
        if (*psCurr).metaKeyCode as libc::c_uint == metaCode as libc::c_uint
               &&
               (*psCurr).subKeyCode as libc::c_uint == subCode as libc::c_uint
           {
            return psCurr
        }
        psCurr = (*psCurr).psNext
    }
    return 0 as *mut KEY_MAPPING;
}
// ----------------------------------------------------------------------------------
/* clears the mappings list and frees the memory */
#[no_mangle]
pub unsafe extern "C" fn keyClearMappings() {
    while !keyMappings.is_null() { keyRemoveMappingPt(keyMappings); };
}
// ----------------------------------------------------------------------------------
/* Removes a mapping specified by a pointer */
#[no_mangle]
pub unsafe extern "C" fn keyRemoveMappingPt(mut psToRemove: *mut KEY_MAPPING)
 -> BOOL {
    let mut psPrev: *mut KEY_MAPPING = 0 as *mut KEY_MAPPING; // ffs
    let mut psCurr: *mut KEY_MAPPING = 0 as *mut KEY_MAPPING;
    if psToRemove.is_null() { return 0 as libc::c_int }
    if psToRemove == keyMappings && (*keyMappings).psNext.is_null() {
        if !(*keyMappings).pName.is_null() {
            memFreeRelease((*keyMappings).pName as *mut libc::c_void);
            (*keyMappings).pName = 0 as *mut STRING
        }
        memFreeRelease(keyMappings as *mut libc::c_void);
        keyMappings = 0 as *mut KEY_MAPPING;
        keyMappings = 0 as *mut KEY_MAPPING;
        numActiveMappings = 0 as libc::c_int as UDWORD;
        return 1 as libc::c_int
    }
    /* See if we can find it */
    psPrev = 0 as *mut KEY_MAPPING;
    psCurr = keyMappings;
    while !psCurr.is_null() && psCurr != psToRemove {
        /*NOP*/
        psPrev = psCurr;
        psCurr = (*psCurr).psNext
    }
    /* If it was found... */
    if psCurr == psToRemove {
        /* See if it was the first element */
        if !psPrev.is_null() {
            /* It wasn't */
            (*psPrev).psNext = (*psCurr).psNext
        } else {
            /* It was */
            keyMappings = (*psCurr).psNext
        }
        /* Free up the memory, first for the string  */
        if !(*psCurr).pName.is_null() {
            memFreeRelease((*psCurr).pName as
                               *mut libc::c_void); // only free it if it was allocated in the first place (ffs)
            (*psCurr).pName = 0 as *mut STRING
        }
        /* and then for the mapping itself */
        memFreeRelease(psCurr as *mut libc::c_void);
        psCurr = 0 as *mut KEY_MAPPING;
        numActiveMappings = numActiveMappings.wrapping_sub(1);
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
// ----------------------------------------------------------------------------------
/* Just returns how many are active */
#[no_mangle]
pub unsafe extern "C" fn getNumMappings() -> UDWORD {
    return numActiveMappings;
}
// ----------------------------------------------------------------------------------
/* Manages update of all the active function mappings */
#[no_mangle]
pub unsafe extern "C" fn keyProcessMappings(mut bExclude: BOOL) {
    let mut keyToProcess: *mut KEY_MAPPING = 0 as *mut KEY_MAPPING;
    let mut bMetaKeyDown: BOOL = 0;
    let mut bKeyProcessed: BOOL = 0;
    /* Bomb out if there are none */
    if keyMappings.is_null() || numActiveMappings == 0 || bKeyProcessing == 0
       {
        return
    }
    /* Jump out if we've got a new mapping */
    checkQwertyKeys();
    /* Check for the meta keys */
    if keyDown(KEY_LCTRL) != 0 || keyDown(KEY_RCTRL) != 0 ||
           keyDown(KEY_LALT) != 0 || keyDown(KEY_RALT) != 0 ||
           keyDown(KEY_LSHIFT) != 0 || keyDown(KEY_RSHIFT) != 0 {
        bMetaKeyDown = 1 as libc::c_int
    } else { bMetaKeyDown = 0 as libc::c_int }
    /* Run through all our mappings */
    keyToProcess = keyMappings;
    while !keyToProcess.is_null() {
        /* We haven't acted upon it */
        bKeyProcessed = 0 as libc::c_int;
        if (*keyToProcess).active == 0 { break ; }
        /* Skip innappropriate ones when necessary */
        if bExclude != 0 &&
               (*keyToProcess).status as libc::c_uint !=
                   KEYMAP_ALWAYS_PROCESS as libc::c_int as libc::c_uint {
            break ;
        }
        if !((*keyToProcess).subKeyCode as libc::c_uint ==
                 SDLK_LAST as libc::c_int as libc::c_uint) {
            if (*keyToProcess).metaKeyCode as libc::c_uint ==
                   5190 as libc::c_int as libc::c_uint && bMetaKeyDown == 0 &&
                   !((*keyToProcess).status as libc::c_uint ==
                         KEYMAP__DEBUG as libc::c_int as libc::c_uint &&
                         bDoingDebugMappings == 0 as libc::c_int) {
                match (*keyToProcess).action as libc::c_uint {
                    1 => {
                        /* Were the right keys pressed? */
                        if keyPressed((*keyToProcess).subKeyCode) != 0 {
                            lastSubKey = (*keyToProcess).subKeyCode;
                            /* Jump to the associated function call */
                            (*keyToProcess).function.expect("non-null function pointer")();
                            bKeyProcessed = 1 as libc::c_int
                        }
                    }
                    0 => {
                        /* Is the key Down? */
                        if keyDown((*keyToProcess).subKeyCode) != 0 {
                            lastSubKey = (*keyToProcess).subKeyCode;
                            /* Jump to the associated function call */
                            (*keyToProcess).function.expect("non-null function pointer")();
                            bKeyProcessed = 1 as libc::c_int
                        }
                    }
                    2 => {
                        /* Has the key been released? */
                        if keyReleased((*keyToProcess).subKeyCode) != 0 {
                            lastSubKey = (*keyToProcess).subKeyCode;
                            /* Jump to the associated function call */
                            (*keyToProcess).function.expect("non-null function pointer")();
                            bKeyProcessed = 1 as libc::c_int
                        }
                    }
                    _ => {
                        debug(LOG_ERROR,
                              b"Weirdy action on keymap processing\x00" as
                                  *const u8 as *const libc::c_char);
                        abort();
                    }
                }
            }
            /* Process the combi ones */
            if (*keyToProcess).metaKeyCode as libc::c_uint !=
                   5190 as libc::c_int as libc::c_uint && bMetaKeyDown != 0 &&
                   !((*keyToProcess).status as libc::c_uint ==
                         KEYMAP__DEBUG as libc::c_int as libc::c_uint &&
                         bDoingDebugMappings == 0 as libc::c_int) {
                /* It's a combo keypress - one held down and the other pressed */
                if keyDown((*keyToProcess).metaKeyCode) != 0 &&
                       keyPressed((*keyToProcess).subKeyCode) != 0 {
                    lastMetaKey = (*keyToProcess).metaKeyCode;
                    lastSubKey = (*keyToProcess).subKeyCode;
                    (*keyToProcess).function.expect("non-null function pointer")();
                    bKeyProcessed = 1 as libc::c_int
                } else if (*keyToProcess).altMetaKeyCode as libc::c_uint !=
                              5190 as libc::c_int as libc::c_uint {
                    if keyDown((*keyToProcess).altMetaKeyCode) != 0 &&
                           keyPressed((*keyToProcess).subKeyCode) != 0 {
                        lastMetaKey = (*keyToProcess).metaKeyCode;
                        lastSubKey = (*keyToProcess).subKeyCode;
                        (*keyToProcess).function.expect("non-null function pointer")();
                        bKeyProcessed = 1 as libc::c_int
                    }
                }
            }
            if bKeyProcessed != 0 {
                if (*keyToProcess).status as libc::c_uint ==
                       KEYMAP__DEBUG as libc::c_int as libc::c_uint &&
                       bDoingDebugMappings != 0 {
                    sprintf(ConsoleString.as_mut_ptr(),
                            b"DEBUG MAPPING : %s\x00" as *const u8 as
                                *const libc::c_char, (*keyToProcess).pName);
                    addConsoleMessage(ConsoleString.as_mut_ptr(),
                                      DEFAULT_JUSTIFY);
                }
            }
        }
        keyToProcess = (*keyToProcess).psNext
    };
}
// ----------------------------------------------------------------------------------
/* WIN 32 specific */
// ----------------------------------------------------------------------------------
/* Allows _new_ mappings to be made at runtime */
#[no_mangle]
pub unsafe extern "C" fn checkQwertyKeys() -> BOOL {
    let mut qKey: KEY_CODE = 0 as KEY_CODE;
    let mut tableEntry: UDWORD = 0;
    let mut aquired: BOOL = 0;
    aquired = 0 as libc::c_int;
    /* Are we trying to make a new map marker? */
    if keyDown(KEY_LALT) != 0 {
        /* Did we press a key */
        qKey = getQwertyKey();
        if qKey as u64 != 0 {
            tableEntry = asciiKeyCodeToTable(qKey);
            /* We're assigning something to the key */
            if !qwertyKeyMappings[tableEntry as usize].psMapping.is_null() {
                /* Get rid of the old mapping on this key if there was one */
                keyRemoveMappingPt(qwertyKeyMappings[tableEntry as
                                                         usize].psMapping);
            }
            /* Now add the new one for this location */
            qwertyKeyMappings[tableEntry as usize].psMapping =
                keyAddMapping(KEYMAP_ALWAYS, KEY_LSHIFT, qKey, KEYMAP_PRESSED,
                              Some(kf_JumpToMapMarker as
                                       unsafe extern "C" fn() -> ()),
                              b"Jump to new map marker\x00" as *const u8 as
                                  *const libc::c_char as *mut STRING);
            aquired = 1 as libc::c_int;
            /* Store away the position and view angle */
            qwertyKeyMappings[tableEntry as usize].xPos =
                player.p.x as UDWORD;
            qwertyKeyMappings[tableEntry as usize].yPos =
                player.p.z as UDWORD;
            qwertyKeyMappings[tableEntry as usize].spin = player.r.y
        }
    }
    return aquired;
}
//remove this one below
// ----------------------------------------------------------------------------------
// this function isn't really module static - should be removed - debug only
#[no_mangle]
pub unsafe extern "C" fn keyShowMappings() {
    let mut psMapping: *mut KEY_MAPPING = 0 as *mut KEY_MAPPING;
    psMapping = keyMappings;
    while !psMapping.is_null() {
        keyShowMapping(psMapping);
        psMapping = (*psMapping).psNext
    };
}
// ----------------------------------------------------------------------------------
/* Sends a particular key mapping to the console */
#[no_mangle]
pub unsafe extern "C" fn keyShowMapping(mut psMapping: *mut KEY_MAPPING) {
    let mut asciiSub: [STRING; 20] = [0; 20];
    let mut asciiMeta: [STRING; 20] = [0; 20];
    let mut onlySub: BOOL = 0;
    onlySub = 1 as libc::c_int;
    if (*psMapping).metaKeyCode as libc::c_uint !=
           5190 as libc::c_int as libc::c_uint {
        keyScanToString((*psMapping).metaKeyCode,
                        &mut asciiMeta as *mut [STRING; 20] as *mut STRING,
                        20 as libc::c_int as UDWORD);
        onlySub = 0 as libc::c_int
    }
    keyScanToString((*psMapping).subKeyCode,
                    &mut asciiSub as *mut [STRING; 20] as *mut STRING,
                    20 as libc::c_int as UDWORD);
    if onlySub != 0 {
        sprintf(ConsoleString.as_mut_ptr(),
                b"%s - %s\x00" as *const u8 as *const libc::c_char,
                asciiSub.as_mut_ptr(), (*psMapping).pName);
        addConsoleMessage(ConsoleString.as_mut_ptr(), DEFAULT_JUSTIFY);
    } else {
        sprintf(ConsoleString.as_mut_ptr(),
                b"%s and %s - %s\x00" as *const u8 as *const libc::c_char,
                asciiMeta.as_mut_ptr(), asciiSub.as_mut_ptr(),
                (*psMapping).pName);
        addConsoleMessage(ConsoleString.as_mut_ptr(), DEFAULT_JUSTIFY);
    };
}
// ----------------------------------------------------------------------------------
/* Returns the key code of the last sub key pressed - allows called functions to have a simple stack */
#[no_mangle]
pub unsafe extern "C" fn getLastSubKey() -> KEY_CODE { return lastSubKey; }
// ----------------------------------------------------------------------------------
/* Returns the key code of the last meta key pressed - allows called functions to have a simple stack */
#[no_mangle]
pub unsafe extern "C" fn getLastMetaKey() -> KEY_CODE { return lastMetaKey; }
// ----------------------------------------------------------------------------------
/* Allows us to enable/disable the whole mapping system */
#[no_mangle]
pub unsafe extern "C" fn keyEnableProcessing(mut val: BOOL) {
    bKeyProcessing = val;
}
// ----------------------------------------------------------------------------------
/* Sets all mappings to be inactive */
#[no_mangle]
pub unsafe extern "C" fn keyAllMappingsInactive() {
    let mut psMapping: *mut KEY_MAPPING = 0 as *mut KEY_MAPPING;
    psMapping = keyMappings;
    while !psMapping.is_null() {
        (*psMapping).active = 0 as libc::c_int;
        psMapping = (*psMapping).psNext
    };
}
// ----------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn keyAllMappingsActive() {
    let mut psMapping: *mut KEY_MAPPING = 0 as *mut KEY_MAPPING;
    psMapping = keyMappings;
    while !psMapping.is_null() {
        (*psMapping).active = 1 as libc::c_int;
        psMapping = (*psMapping).psNext
    };
}
// ----------------------------------------------------------------------------------
/* Allows us to make active/inactive specific mappings */
#[no_mangle]
pub unsafe extern "C" fn keySetMappingStatus(mut psMapping: *mut KEY_MAPPING,
                                             mut state: BOOL) {
    (*psMapping).active = state;
}
/* Returns the key code of the first ascii key that its finds has been PRESSED */
#[no_mangle]
pub unsafe extern "C" fn getQwertyKey() -> KEY_CODE {
    let mut i: UDWORD = 0;
    i = KEY_Q as libc::c_int as UDWORD;
    while i <= KEY_P as libc::c_int as libc::c_uint {
        if keyPressed(i as KEY_CODE) != 0 {
            return i as KEY_CODE
            // top row key pressed
        }
        i = i.wrapping_add(1)
    }
    i = KEY_A as libc::c_int as UDWORD;
    while i <= KEY_L as libc::c_int as libc::c_uint {
        if keyPressed(i as KEY_CODE) != 0 {
            return i as KEY_CODE
            // middle row key pressed
        }
        i = i.wrapping_add(1)
    }
    i = KEY_Z as libc::c_int as UDWORD;
    while i <= KEY_M as libc::c_int as libc::c_uint {
        if keyPressed(i as KEY_CODE) != 0 {
            return i as KEY_CODE
            // bottomw row key pressed
        }
        i = i.wrapping_add(1)
    }
    return 0 as KEY_CODE;
    // no ascii key pressed
}
// ----------------------------------------------------------------------------------
/*	Returns the number (0 to 26) of a key on the keyboard
	from it's keycode. Q is zero, through to M being 25
*/
#[no_mangle]
pub unsafe extern "C" fn asciiKeyCodeToTable(mut code: KEY_CODE) -> UDWORD {
    if code as libc::c_uint <= KEY_P as libc::c_int as libc::c_uint {
        code =
            (code as
                 libc::c_uint).wrapping_sub(KEY_Q as libc::c_int as
                                                libc::c_uint) as KEY_CODE
        // q is the first of the ascii scan codes
    } else if code as libc::c_uint <= KEY_L as libc::c_int as libc::c_uint {
        code =
            (code as
                 libc::c_uint).wrapping_sub(KEY_A as libc::c_int as
                                                libc::c_uint).wrapping_add(10
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_uint)
                as KEY_CODE
        // ten keys from q to p
    } else if code as libc::c_uint <= KEY_M as libc::c_int as libc::c_uint {
        code =
            (code as
                 libc::c_uint).wrapping_sub(KEY_Z as libc::c_int as
                                                libc::c_uint).wrapping_add(19
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_uint)
                as KEY_CODE
        // 19 keys before, the 10 from q..p and the 9 from a..l
    }
    return code as UDWORD;
}
// ----------------------------------------------------------------------------------
/* Returns the map X position associated with the passed in keycode */
#[no_mangle]
pub unsafe extern "C" fn getMarkerX(mut code: KEY_CODE) -> UDWORD {
    let mut entry: UDWORD = 0;
    entry = asciiKeyCodeToTable(code);
    return qwertyKeyMappings[entry as usize].xPos;
}
// ----------------------------------------------------------------------------------
/* Returns the map Y position associated with the passed in keycode */
#[no_mangle]
pub unsafe extern "C" fn getMarkerY(mut code: KEY_CODE) -> UDWORD {
    let mut entry: UDWORD = 0;
    entry = asciiKeyCodeToTable(code);
    return qwertyKeyMappings[entry as usize].yPos;
}
// ----------------------------------------------------------------------------------
/* Returns the map Y rotation associated with the passed in keycode */
#[no_mangle]
pub unsafe extern "C" fn getMarkerSpin(mut code: KEY_CODE) -> SDWORD {
    let mut entry: UDWORD = 0;
    entry = asciiKeyCodeToTable(code);
    return qwertyKeyMappings[entry as usize].spin;
}
// ----------------------------------------------------------------------------------
/* Defines whether we process debug key mapping stuff */
#[no_mangle]
pub unsafe extern "C" fn processDebugMappings(mut val: BOOL) {
    bDoingDebugMappings = val;
}
// ----------------------------------------------------------------------------------
/* Returns present status of debug mapping processing */
#[no_mangle]
pub unsafe extern "C" fn getDebugMappingStatus() -> BOOL {
    return bDoingDebugMappings;
}
// ----------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn keyReAssignMapping(mut origMetaCode: KEY_CODE,
                                            mut origSubCode: KEY_CODE,
                                            mut newMetaCode: KEY_CODE,
                                            mut newSubCode: KEY_CODE)
 -> BOOL {
    let mut psMapping: *mut KEY_MAPPING = 0 as *mut KEY_MAPPING;
    let mut bFound: BOOL = 0;
    psMapping = keyMappings;
    bFound = 0 as libc::c_int;
    while !psMapping.is_null() && bFound == 0 {
        /* Find the original */
        if (*psMapping).metaKeyCode as libc::c_uint ==
               origMetaCode as libc::c_uint &&
               (*psMapping).subKeyCode as libc::c_uint ==
                   origSubCode as libc::c_uint {
            /* Not all can be remapped */
            if (*psMapping).status as libc::c_uint !=
                   KEYMAP_ALWAYS as libc::c_int as libc::c_uint ||
                   (*psMapping).status as libc::c_uint ==
                       KEYMAP_ALWAYS_PROCESS as libc::c_int as libc::c_uint {
                (*psMapping).metaKeyCode = newMetaCode;
                (*psMapping).subKeyCode = newSubCode;
                bFound = 1 as libc::c_int
            }
        }
        psMapping = (*psMapping).psNext
    }
    return bFound;
}
/*
BOOL	keyReAssignMappingName(STRING *pName, KEY_CODE newMetaCode, KEY_CODE newSubCode)
							   )
{
KEY_MAPPING	*psMapping;
KEY_CODE	origMetaCode,origSubCode;
BOOL	bReplaced;

  	for(psMapping = keyMappings,bReplaced = FALSE; psMapping AND !bReplaced;
		psMapping = psMapping->psNext)
	{
		if(strcmp(psMapping->pName,pName) == FALSE)	//negative
		{
			if(psMapping->status==KEYMAP_ASSIGNABLE)
			{
				(void)keyAddMapping(psMapping->status,newMetaCode,
					newSubCode, psMapping->action,psMapping->function,psMapping->pName);
				bReplaced = TRUE;
				origMetaCode = psMapping->metaKeyCode;
				origSubCode = psMapping->subKeyCode;
			}
		}
	}

	if(bReplaced)
	{
		keyRemoveMapping(origMetaCode, origSubCode);
	}
	return(bReplaced);
}
*/
// ----------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn getKeyMapFromName(mut pName: *mut STRING)
 -> *mut KEY_MAPPING {
    let mut psMapping: *mut KEY_MAPPING = 0 as *mut KEY_MAPPING;
    psMapping = keyMappings;
    while !psMapping.is_null() {
        if strcmp(pName, (*psMapping).pName) == 0 as libc::c_int {
            return psMapping
        }
        psMapping = (*psMapping).psNext
    }
    return 0 as *mut KEY_MAPPING;
}
// ----------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn keyReAssignMappingName(mut pName: *mut STRING,
                                                mut newMetaCode: KEY_CODE,
                                                mut newSubCode: KEY_CODE)
 -> BOOL {
    let mut psMapping: *mut KEY_MAPPING = 0 as *mut KEY_MAPPING;
    psMapping = getKeyMapFromName(pName);
    if !psMapping.is_null() {
        if (*psMapping).status as libc::c_uint ==
               KEYMAP_ASSIGNABLE as libc::c_int as libc::c_uint {
            keyAddMapping((*psMapping).status, newMetaCode, newSubCode,
                          (*psMapping).action, (*psMapping).function,
                          (*psMapping).pName);
            keyRemoveMappingPt(psMapping);
            return 1 as libc::c_int
        }
    }
    return 0 as libc::c_int;
}
// ----------------------------------------------------------------------------------
