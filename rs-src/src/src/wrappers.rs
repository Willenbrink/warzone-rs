use ::libc;
extern "C" {
    /* set the function to call when loading files with resloadfile*/
    #[no_mangle]
    fn resSetLoadCallback(funcToCall: RESLOAD_CALLBACK);
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
    /* Use misc.  */
    /* Use extended X/Open || misc. */
    /* Return a random integer between 0 and RAND_MAX inclusive.  */
    #[no_mangle]
    fn rand() -> libc::c_int;
    /* Abort execution and generate a core-dump.  */
    #[no_mangle]
    fn abort() -> !;
    /* * Wait a specified number of milliseconds before returning */
    #[no_mangle]
    fn SDL_Delay(ms: Uint32);
    /* This returns true if the key is currently depressed */
    #[no_mangle]
    fn keyDown(code: KEY_CODE) -> BOOL;
    /* This returns true if the key went from being up to being down this frame */
    #[no_mangle]
    fn keyPressed(code: KEY_CODE) -> BOOL;
    /* This returns true if the key went from being down to being up this frame */
    #[no_mangle]
    fn keyReleased(code: KEY_CODE) -> BOOL;
    /* This returns true if the mouse key went from being down to being up this frame */
    #[no_mangle]
    fn mouseReleased(code: MOUSE_KEY_CODE) -> BOOL;
    #[no_mangle]
    fn frameSetCursorFromRes(resID: WORD);
    #[no_mangle]
    fn GetTickCount() -> DWORD;
    //Sets all states
    #[no_mangle]
    fn pie_SetDepthBufferStatus(depthMode: DEPTH_MODE);
    #[no_mangle]
    fn pie_SetFogStatus(val: BOOL);
    #[no_mangle]
    fn screen_StopBackDrop();
    #[no_mangle]
    fn screen_RestartBackDrop();
    /* Toggle the display between full screen or windowed */
    #[no_mangle]
    fn screenToggleMode();
    #[no_mangle]
    fn pie_ScreenFlip(ClearMode: CLEAR_MODE);
    #[no_mangle]
    fn pie_GlobalRenderBegin();
    #[no_mangle]
    fn pie_GlobalRenderEnd(bForceClearToBlack: BOOL);
    #[no_mangle]
    fn pie_GetVideoBufferWidth() -> UDWORD;
    #[no_mangle]
    fn pie_GetVideoBufferHeight() -> UDWORD;
    #[no_mangle]
    fn audio_Update() -> BOOL;
    // This dos'nt compile on the PSX.
//typedef enum _titlemode tMode;	// define the type
    #[no_mangle]
    static mut titleMode: tMode;
    #[no_mangle]
    fn changeTitleMode(mode: tMode);
    #[no_mangle]
    fn startTitleMenu() -> BOOL;
    #[no_mangle]
    fn runTitleMenu() -> BOOL;
    #[no_mangle]
    fn runSinglePlayerMenu() -> BOOL;
    #[no_mangle]
    fn runMultiPlayerMenu() -> BOOL;
    //extern BOOL runVideoOptionsMenu		(VOID);
//extern BOOL runGraphicsOptionsMenu	(VOID);
    #[no_mangle]
    fn runGameOptionsMenu() -> BOOL;
    #[no_mangle]
    fn runOptionsMenu() -> BOOL;
    #[no_mangle]
    fn runTutorialMenu() -> BOOL;
    #[no_mangle]
    fn runGameOptions2Menu() -> BOOL;
    #[no_mangle]
    fn runGameOptions3Menu() -> BOOL;
    // Begin a rendering lock.
    #[no_mangle]
    fn DrawBegin();
    // End a rendering lock.
    #[no_mangle]
    fn DrawEnd();
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
    #[no_mangle]
    fn intAddMissionResult(result: BOOL, bPlaySuccess: BOOL) -> BOOL;
    #[no_mangle]
    fn clearMissionWidgets();
    /*
 * keyedit.h
 */
    #[no_mangle]
    fn runKeyMapEditor() -> BOOL;
    #[no_mangle]
    fn pie_UniTransBoxFill(x0: SDWORD, y0: SDWORD, x1: SDWORD, y1: SDWORD,
                           rgb: UDWORD, transparency: UDWORD);
    #[no_mangle]
    fn pie_ResetBackDrop();
    #[no_mangle]
    fn pie_LoadBackDrop(screenType: SCREENTYPE, b3DFX: BOOL);
    // ////////////////////////////////////////////////////////////////////////
// variables
    #[no_mangle]
    static mut NetPlay: NETPLAY;
    #[no_mangle]
    static mut gameSpy: GAMESPY;
    // the game description.
    #[no_mangle]
    static mut ingame: MULTIPLAYERINGAME;
    // the game description.
    #[no_mangle]
    static mut bMultiPlayer: BOOL;
    #[no_mangle]
    fn multiplayerWinSequence(firstCall: BOOL) -> BOOL;
    /*
 *multiint.h
 * Interface defines/externs for warzone frontend.
 * Alex Lee, pumpkin Studios.
 */
    #[no_mangle]
    fn runConnectionScreen();
    #[no_mangle]
    fn runGameFind();
    #[no_mangle]
    fn runMultiOptions();
    #[no_mangle]
    fn runForceSelect();
    #[no_mangle]
    fn updateMultiStatsWins();
    #[no_mangle]
    fn updateMultiStatsLoses();
    #[no_mangle]
    fn runLimitScreen();
}
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
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
pub type UWORD = libc::c_ushort;
pub type UDWORD = libc::c_uint;
pub type SDWORD = libc::c_int;
pub type BOOL = libc::c_int;
pub type CHAR = libc::c_char;
pub type WORD = libc::c_short;
pub type DWORD = libc::c_int;
/* !WIN32 */
pub type DPID = libc::c_int;
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
pub type Uint32 = uint32_t;
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
pub type _mouse_key_code = libc::c_uint;
pub const MOUSE_WDN: _mouse_key_code = 5;
pub const MOUSE_WUP: _mouse_key_code = 4;
pub const MOUSE_RMB: _mouse_key_code = 3;
pub const MOUSE_MMB: _mouse_key_code = 2;
pub const MOUSE_LMB: _mouse_key_code = 1;
pub type MOUSE_KEY_CODE = _mouse_key_code;
/* callback type for resload display callback*/
pub type RESLOAD_CALLBACK = Option<unsafe extern "C" fn() -> ()>;
pub type DEPTH_MODE = libc::c_uint;
pub const DEPTH_CMP_ALWAYS_WRT_OFF: DEPTH_MODE = 3;
pub const DEPTH_CMP_LEQ_WRT_OFF: DEPTH_MODE = 2;
pub const DEPTH_CMP_ALWAYS_WRT_ON: DEPTH_MODE = 1;
pub const DEPTH_CMP_LEQ_WRT_ON: DEPTH_MODE = 0;
pub type CLEAR_MODE = libc::c_uint;
pub const CLEAR_FOG: CLEAR_MODE = 3;
pub const CLEAR_BLACK: CLEAR_MODE = 2;
pub const CLEAR_OFF_AND_NO_BUFFER_DOWNLOAD: CLEAR_MODE = 1;
pub const CLEAR_OFF: CLEAR_MODE = 0;
pub type TITLECODE = libc::c_uint;
pub const TITLECODE_SAVEGAMELOAD: TITLECODE = 4;
pub const TITLECODE_SHOWINTRO: TITLECODE = 3;
pub const TITLECODE_QUITGAME: TITLECODE = 2;
pub const TITLECODE_STARTGAME: TITLECODE = 1;
pub const TITLECODE_CONTINUE: TITLECODE = 0;
/*
 * Wrappers.c
 * frontend loop & also loading screen & game over screen.
 * AlexL. Pumpkin Studios, EIDOS Interactive, 1997
 */
// FIXME Direct iVis implementation include!
// FIXME Direct iVis implementation include!
// FIXME Direct iVis implementation include!
pub type STAR = _star;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _star {
    pub xPos: UWORD,
    pub speed: SDWORD,
}
/* 
 * Frontend.h
 */
// determines which option screen to use. when in GS_TITLE_SCREEN mode.
pub type tMode = _title_mode;
pub type _title_mode = libc::c_uint;
pub const GAME3: _title_mode = 18;
pub const GAME2: _title_mode = 17;
pub const KEYMAP: _title_mode = 16;
pub const LOADSAVEGAME: _title_mode = 15;
pub const QUIT: _title_mode = 14;
pub const SHOWINTRO: _title_mode = 13;
pub const STARTGAME: _title_mode = 12;
pub const MULTILIMIT: _title_mode = 11;
pub const GAMEFIND: _title_mode = 10;
pub const FORCESELECT: _title_mode = 9;
pub const MULTIOPTION: _title_mode = 8;
pub const PROTOCOL: _title_mode = 7;
pub const CREDITS: _title_mode = 6;
pub const TUTORIAL: _title_mode = 5;
pub const GAME: _title_mode = 4;
pub const OPTIONS: _title_mode = 3;
pub const MULTI: _title_mode = 2;
pub const SINGLE: _title_mode = 1;
pub const TITLE: _title_mode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NETPLAY {
    pub games: [GAMESTRUCT; 12],
    pub players: [PLAYER; 8],
    pub playercount: UDWORD,
    pub dpidPlayer: DPID,
    pub bComms: BOOL,
    pub bHost: BOOL,
    pub bLobbyLaunched: BOOL,
    pub bSpectator: BOOL,
    pub bEncryptAllPackets: BOOL,
    pub cryptKey: [UDWORD; 4],
    pub bCaptureInUse: BOOL,
    pub bAllowCaptureRecord: BOOL,
    pub bAllowCapturePlay: BOOL,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PLAYER {
    pub dpid: DPID,
    pub name: [libc::c_char; 64],
    pub bHost: BOOL,
    pub bSpectator: BOOL,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GAMESTRUCT {
    pub name: [libc::c_char; 64],
    pub desc: SESSIONDESC,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SESSIONDESC {
    pub dwSize: DWORD,
    pub dwFlags: DWORD,
    pub host: [libc::c_char; 16],
    pub dwMaxPlayers: DWORD,
    pub dwCurrentPlayers: DWORD,
    pub dwUser1: DWORD,
    pub dwUser2: DWORD,
    pub dwUser3: DWORD,
    pub dwUser4: DWORD,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MULTIPLAYERINGAME {
    pub PingTimes: [UDWORD; 8],
    pub localOptionsReceived: BOOL,
    pub localJoiningInProgress: BOOL,
    pub JoiningInProgress: [BOOL; 8],
    pub bHostSetup: BOOL,
    pub startTime: UDWORD,
    pub modem: UDWORD,
    pub numStructureLimits: UDWORD,
    pub pStructureLimits: *mut UBYTE,
    pub skScores: [[UDWORD; 2]; 8],
    pub phrases: [[CHAR; 255]; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GAMESPY {
    pub bGameSpy: BOOL,
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
#[no_mangle]
pub static mut stars: [STAR; 30] = [STAR{xPos: 0, speed: 0,}; 30];
static mut firstcall: BOOL = 0;
static mut loadScreenCallNo: UDWORD = 0 as libc::c_int as UDWORD;
static mut bPlayerHasLost: BOOL = 0 as libc::c_int;
static mut bPlayerHasWon: BOOL = 0 as libc::c_int;
static mut scriptWinLoseVideo: UBYTE = 0 as libc::c_int as UBYTE;
#[no_mangle]
pub static mut lastTick: UDWORD = 0 as libc::c_int as UDWORD;
#[no_mangle]
pub unsafe extern "C" fn initStars() {
    let mut i: UDWORD = 0;
    i = 0 as libc::c_int as UDWORD;
    while i < 20 as libc::c_int as libc::c_uint {
        // 0 intro mode
        // 1 single player menu
        // 2 multiplayer menu
        // 3 options menu	
        // 4
        // 5  tutorial/fastplay	
        // 6  credits
        // 7  MULTIPLAYER, select proto
        // 8 MULTIPLAYER, select game options
        // 9 MULTIPLAYER, Force design screen
        // 10 MULTIPLAYER, gamefinder.
        // 11 MULTIPLAYER, Limit the multistuff.
        // 12 Fire up the game
        // 13 reshow the intro
        // 14 leaving game
        // 15 loading a save game
        // 16 keymap editor
        // 17 second options menu.
        // 18 third options menu.
        //		GRAPHICS,					// 5
//		VIDEO,
//	DEMOMODE,					// demo mode. remove for release?
        stars[i as usize].xPos = (rand() % 598 as libc::c_int) as UWORD;
        stars[i as usize].speed =
            rand() % 10 as libc::c_int + 2 as libc::c_int;
        i = i.wrapping_add(1) // scatter them
        // always move
    };
}
// //////////////////////////////////////////////////////////////////
// Initialise frontend globals and statics.
//
#[no_mangle]
pub unsafe extern "C" fn frontendInitVars() -> BOOL {
    firstcall = 1 as libc::c_int;
    initStars();
    return 1 as libc::c_int;
}
// ///////////////// /////////////////////////////////////////////////
// Main Front end game loop.
#[no_mangle]
pub unsafe extern "C" fn titleLoop() -> TITLECODE {
    let mut RetCode: TITLECODE = TITLECODE_CONTINUE; // reset cursor	(sw)
    pie_GlobalRenderBegin();
    pie_SetDepthBufferStatus(DEPTH_CMP_ALWAYS_WRT_ON);
    pie_SetFogStatus(0 as libc::c_int);
    screen_RestartBackDrop();
    if firstcall != 0 {
        startTitleMenu();
        titleMode = TITLE;
        firstcall = 0 as libc::c_int;
        frameSetCursorFromRes(108 as libc::c_int as WORD);
        if NetPlay.bLobbyLaunched != 0 {
            // lobbies skip title screens & go into the game
            if NetPlay.bHost != 0 {
                ingame.bHostSetup = 1 as libc::c_int
            } else { ingame.bHostSetup = 0 as libc::c_int }
            changeTitleMode(QUIT);
        } else if gameSpy.bGameSpy != 0 {
            // set host
            if NetPlay.bHost != 0 {
                ingame.bHostSetup = 1 as libc::c_int
            } else { ingame.bHostSetup = 0 as libc::c_int }
            // set protocol
			// set address
			// if host goto options.
			// if client goto game find.
            if NetPlay.bHost != 0 {
                changeTitleMode(MULTIOPTION);
            } else { changeTitleMode(GAMEFIND); }
        }
    }
    match titleMode as libc::c_uint {
        7 => {
            // run relevant title screen code.
            runConnectionScreen();
        }
        8 => { runMultiOptions(); }
        10 => { runGameFind(); }
        9 => { runForceSelect(); }
        2 => { runMultiPlayerMenu(); }
        11 => { runLimitScreen(); }
        16 => { runKeyMapEditor(); }
        0 => { runTitleMenu(); }
        1 => { runSinglePlayerMenu(); }
        5 => { runTutorialMenu(); }
        6 => {
            //		case GRAPHICS:
//			runGraphicsOptionsMenu();
//			break;
            runCreditsScreen();
        }
        3 => {
            //		case DEMOMODE:
//			runDemoMenu();
//			break;
//	case VIDEO:
//			runVideoOptionsMenu();
//			break;
            runOptionsMenu(); //render active
        }
        4 => {
            runGameOptionsMenu(); //force to black
        }
        17 => {
            runGameOptions2Menu(); // don't flip!
        }
        18 => {
            runGameOptions3Menu(); //flip to clear screen but not here//reshow intro video.
        }
        14 => { RetCode = TITLECODE_QUITGAME }
        12 | 15 => {
            initLoadingScreen(1 as libc::c_int,
                              1 as
                                  libc::c_int); //flip to clear screen but not here
            if titleMode as libc::c_uint ==
                   LOADSAVEGAME as libc::c_int as libc::c_uint {
                RetCode = TITLECODE_SAVEGAMELOAD
            } else {
                RetCode = TITLECODE_STARTGAME
            } //force to black  //[movie]  --We got no videos yet, so better to display the backdrop... -Q
            pie_GlobalRenderEnd(1 as libc::c_int); //title loop
            return RetCode
        }
        13 => {
            pie_SetFogStatus(0 as
                                 libc::c_int); //To fix ALL menus to be less CPU hogging. -Q 5-14-05
            pie_ScreenFlip(CLEAR_BLACK);
            pie_ScreenFlip(CLEAR_BLACK);
            changeTitleMode(TITLE);
            RetCode = TITLECODE_SHOWINTRO
        }
        _ => {
            debug(LOG_ERROR,
                  b"unknown title screen mode\x00" as *const u8 as
                      *const libc::c_char);
            abort();
        }
    }
    audio_Update();
    pie_GlobalRenderEnd(0 as libc::c_int);
    pie_SetFogStatus(0 as libc::c_int);
    pie_ScreenFlip(CLEAR_BLACK);
    if (keyDown(KEY_LALT) != 0 || keyDown(KEY_RALT) != 0) &&
           keyPressed(KEY_RETURN) != 0 {
        screenToggleMode();
    }
    SDL_Delay(30 as libc::c_int as Uint32);
    return RetCode;
}
// //////////////////////////////////////////////////////////////////////////////
// //////////////////////////////////////////////////////////////////////////////
// Loading Screen.
//loadbar update
#[no_mangle]
pub unsafe extern "C" fn loadingScreenCallback() {
    let mut i: UDWORD = 0;
    let mut topX: UDWORD = 0;
    let mut topY: UDWORD = 0;
    let mut botX: UDWORD = 0;
    let mut botY: UDWORD = 0;
    let mut currTick: UDWORD = 0;
    if (GetTickCount() as libc::c_uint).wrapping_sub(lastTick) <
           16 as libc::c_int as libc::c_uint {
        return
    }
    currTick = GetTickCount() as UDWORD;
    if currTick.wrapping_sub(lastTick) > 500 as libc::c_int as libc::c_uint {
        currTick =
            (currTick as libc::c_uint).wrapping_sub(lastTick) as UDWORD as
                UDWORD;
        debug(LOG_NEVER,
              b"loadingScreenCallback: pause %d\n\x00" as *const u8 as
                  *const libc::c_char, currTick);
    }
    lastTick = GetTickCount() as UDWORD;
    pie_GlobalRenderBegin();
    DrawBegin();
    pie_UniTransBoxFill(1 as libc::c_int, 1 as libc::c_int, 2 as libc::c_int,
                        2 as libc::c_int, 0x10101 as libc::c_int as UDWORD,
                        32 as libc::c_int as UDWORD);
    /* Draw the black rectangle at the bottom */
    topX =
        (10 as libc::c_int as
             libc::c_uint).wrapping_add(pie_GetVideoBufferWidth().wrapping_sub(640
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   libc::c_uint).wrapping_div(2
                                                                                                                  as
                                                                                                                  libc::c_int
                                                                                                                  as
                                                                                                                  libc::c_uint));
    topY =
        (450 as libc::c_int as
             libc::c_uint).wrapping_add(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint).wrapping_div(2
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   libc::c_uint)).wrapping_sub(1
                                                                                                                                                   as
                                                                                                                                                   libc::c_int
                                                                                                                                                   as
                                                                                                                                                   libc::c_uint);
    botX =
        (630 as libc::c_int as
             libc::c_uint).wrapping_add(pie_GetVideoBufferWidth().wrapping_sub(640
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   libc::c_uint).wrapping_div(2
                                                                                                                  as
                                                                                                                  libc::c_int
                                                                                                                  as
                                                                                                                  libc::c_uint));
    botY =
        (470 as libc::c_int as
             libc::c_uint).wrapping_add(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint).wrapping_div(2
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   libc::c_uint)).wrapping_add(1
                                                                                                                                                   as
                                                                                                                                                   libc::c_int
                                                                                                                                                   as
                                                                                                                                                   libc::c_uint);
    //	pie_BoxFillIndex(10+D_W,450+D_H-1,630+D_W,470+D_H+1,COL_BLACK);
    pie_UniTransBoxFill(topX as SDWORD, topY as SDWORD, botX as SDWORD,
                        botY as SDWORD, 0x10101 as libc::c_int as UDWORD,
                        24 as libc::c_int as UDWORD); //force to black
    i = 1 as libc::c_int as UDWORD; //loading callback		// dont clear.
    while i < 19 as libc::c_int as libc::c_uint {
        if stars[i as usize].xPos as libc::c_int + stars[i as usize].speed >=
               598 as libc::c_int {
            stars[i as usize].xPos = 1 as libc::c_int as UWORD
        } else {
            stars[i as usize].xPos =
                (stars[i as usize].xPos as libc::c_int +
                     stars[i as usize].speed) as UWORD
        }
        pie_UniTransBoxFill(((10 as libc::c_int +
                                  stars[i as usize].xPos as libc::c_int) as
                                 libc::c_uint).wrapping_add(pie_GetVideoBufferWidth().wrapping_sub(640
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_uint).wrapping_div(2
                                                                                                                                      as
                                                                                                                                      libc::c_int
                                                                                                                                      as
                                                                                                                                      libc::c_uint))
                                as SDWORD,
                            (450 as libc::c_int as
                                 libc::c_uint).wrapping_add(i).wrapping_add(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                                                        as
                                                                                                                        libc::c_int
                                                                                                                        as
                                                                                                                        libc::c_uint).wrapping_div(2
                                                                                                                                                       as
                                                                                                                                                       libc::c_int
                                                                                                                                                       as
                                                                                                                                                       libc::c_uint))
                                as SDWORD,
                            ((10 as libc::c_int +
                                  stars[i as usize].xPos as libc::c_int +
                                  2 as libc::c_int * stars[i as usize].speed)
                                 as
                                 libc::c_uint).wrapping_add(pie_GetVideoBufferWidth().wrapping_sub(640
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_uint).wrapping_div(2
                                                                                                                                      as
                                                                                                                                      libc::c_int
                                                                                                                                      as
                                                                                                                                      libc::c_uint))
                                as SDWORD,
                            (450 as libc::c_int as
                                 libc::c_uint).wrapping_add(i).wrapping_add(2
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_uint).wrapping_add(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                                                                                       as
                                                                                                                                                       libc::c_int
                                                                                                                                                       as
                                                                                                                                                       libc::c_uint).wrapping_div(2
                                                                                                                                                                                      as
                                                                                                                                                                                      libc::c_int
                                                                                                                                                                                      as
                                                                                                                                                                                      libc::c_uint))
                                as SDWORD, 0xffffff as libc::c_int as UDWORD,
                            255 as libc::c_int as UDWORD);
        i = i.wrapping_add(1)
    }
    DrawEnd();
    pie_GlobalRenderEnd(1 as libc::c_int);
    pie_ScreenFlip(CLEAR_OFF_AND_NO_BUFFER_DOWNLOAD);
    audio_Update();
}
// fill buffers with the static screen
#[no_mangle]
pub unsafe extern "C" fn initLoadingScreen(mut drawbdrop: BOOL,
                                           mut bRenderActive: BOOL) {
    if drawbdrop == 0 {
        // fill buffers
        //just init the load bar with the current screen
		// setup the callback....
        pie_SetFogStatus(0 as libc::c_int); //init loading
        pie_ScreenFlip(CLEAR_BLACK); //init loading
        pie_ScreenFlip(CLEAR_BLACK);
        resSetLoadCallback(Some(loadingScreenCallback as
                                    unsafe extern "C" fn() -> ()));
        loadScreenCallNo = 0 as libc::c_int as UDWORD;
        return
    }
    if bRenderActive != 0 {
        pie_GlobalRenderEnd(1 as libc::c_int);
        //force to black
    } //init loading
    pie_ResetBackDrop(); //init loading
    pie_SetFogStatus(0 as libc::c_int);
    pie_ScreenFlip(CLEAR_BLACK);
    pie_ScreenFlip(CLEAR_BLACK);
    // setup the callback....
    resSetLoadCallback(Some(loadingScreenCallback as
                                unsafe extern "C" fn() -> ()));
    loadScreenCallNo = 0 as libc::c_int as UDWORD;
    screen_StopBackDrop();
    if bRenderActive != 0 { pie_GlobalRenderBegin(); };
}
#[no_mangle]
pub static mut lastChange: UDWORD = 0 as libc::c_int as UDWORD;
// fill buffers with the static screen
#[no_mangle]
pub unsafe extern "C" fn startCreditsScreen(mut bRenderActive: BOOL) {
    let mut screen: SCREENTYPE = SCREEN_CREDITS;
    lastChange = gameTime;
    // fill buffers
    pie_LoadBackDrop(screen, 0 as libc::c_int);
    if bRenderActive != 0 {
        pie_GlobalRenderEnd(1 as libc::c_int);
        //force to black
    } //flip to set back buffer
    pie_SetFogStatus(0 as libc::c_int); //init loading
    pie_ScreenFlip(CLEAR_BLACK);
    pie_ScreenFlip(CLEAR_BLACK);
    if bRenderActive != 0 { pie_GlobalRenderBegin(); };
}
/* This function does nothing - since it's already been drawn */
#[no_mangle]
pub unsafe extern "C" fn runCreditsScreen() {
    // Check for key presses now.
    if keyReleased(KEY_ESC) != 0 || keyReleased(KEY_SPACE) != 0 ||
           mouseReleased(MOUSE_LMB) != 0 ||
           gameTime.wrapping_sub(lastChange) >
               4000 as libc::c_int as libc::c_uint {
        lastChange = gameTime;
        changeTitleMode(QUIT);
    };
}
// shut down the loading screen
#[no_mangle]
pub unsafe extern "C" fn closeLoadingScreen() {
    resSetLoadCallback(None);
    loadScreenCallNo = 0 as libc::c_int as UDWORD;
}
// //////////////////////////////////////////////////////////////////////////////
// //////////////////////////////////////////////////////////////////////////////
// Gameover screen.
#[no_mangle]
pub unsafe extern "C" fn displayGameOver(mut bDidit: BOOL) -> BOOL {
    // AlexL says take this out......
//	setConsolePermanence(TRUE,TRUE);
//	flushConsoleMessages( );
    //	addConsoleMessage(" ", CENTRE_JUSTIFY );
//	addConsoleMessage(strresGetString(psStringRes,STR_GAM_GAMEOVER), CENTRE_JUSTIFY );
//	addConsoleMessage(" ", CENTRE_JUSTIFY );
    //set this for debug mode too - what gets displayed then depends on whether we
    //have won or lost and if we are in debug mode
    // this bit decides whether to auto quit to front end.
	//if(!getDebugMappingStatus())
    if bDidit != 0 {
        setPlayerHasWon(1 as libc::c_int);
        // quit to frontend..
    } else { setPlayerHasLost(1 as libc::c_int); }
    if bMultiPlayer != 0 {
        if bDidit != 0 {
            updateMultiStatsWins();
            multiplayerWinSequence(1 as libc::c_int);
        } else { updateMultiStatsLoses(); }
    } else {
        //	if(getDebugMappingStatus())
//	{
//		intAddInGameOptions();
//	}
        //clear out any mission widgets - timers etc that may be on the screen
        clearMissionWidgets();
        intAddMissionResult(bDidit, 1 as libc::c_int);
    }
    return 1 as libc::c_int;
}
// //////////////////////////////////////////////////////////////////////////////
#[no_mangle]
pub unsafe extern "C" fn testPlayerHasLost() -> BOOL {
    return bPlayerHasLost;
}
#[no_mangle]
pub unsafe extern "C" fn setPlayerHasLost(mut val: BOOL) {
    bPlayerHasLost = val;
}
// //////////////////////////////////////////////////////////////////////////////
#[no_mangle]
pub unsafe extern "C" fn testPlayerHasWon() -> BOOL { return bPlayerHasWon; }
#[no_mangle]
pub unsafe extern "C" fn setPlayerHasWon(mut val: BOOL) {
    bPlayerHasWon = val;
}
/*access functions for scriptWinLoseVideo - used to indicate when the script is playing the win/lose video*/
#[no_mangle]
pub unsafe extern "C" fn setScriptWinLoseVideo(mut val: UBYTE) {
    scriptWinLoseVideo = val;
}
#[no_mangle]
pub unsafe extern "C" fn getScriptWinLoseVideo() -> UBYTE {
    return scriptWinLoseVideo;
}
