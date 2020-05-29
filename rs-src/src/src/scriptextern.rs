use ::libc;
extern "C" {
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
    static mut mapWidth: UDWORD;
    #[no_mangle]
    static mut mapHeight: UDWORD;
    /* Pop a value off the stack */
    #[no_mangle]
    fn stackPop(psVal: *mut INTERP_VAL) -> BOOL;
    #[no_mangle]
    fn stackPushResult(type_0: INTERP_TYPE, data: SDWORD) -> BOOL;
    #[no_mangle]
    fn iV_GetMouseFrame() -> UDWORD;
    #[no_mangle]
    fn getTargetType() -> UDWORD;
    #[no_mangle]
    static mut selectedPlayer: UDWORD;
    #[no_mangle]
    static mut game: MULTIPLAYERGAME;
    /* WinMain.h */
    //flag to indicate when initialisation is complete
    #[no_mangle]
    static mut gameInitialised: BOOL;
    #[no_mangle]
    static mut intMode: INTMODE;
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
}
pub type UBYTE = libc::c_uchar;
pub type STRING = libc::c_char;
pub type UWORD = libc::c_ushort;
pub type UDWORD = libc::c_uint;
pub type SDWORD = libc::c_int;
pub type BOOL = libc::c_int;
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
/*
 * ScriptExtern.h
 *
 * All game variable access functions for the scripts
 *
 */
// current game level
// whether the tutorial is active
// whether any additional special case victory/failure conditions have been met
// ID numbers for external variables
pub type _externids = libc::c_uint;
pub const EXTID_ISPSX: _externids = 16;
pub const EXTID_TRACKTRANSPORTER: _externids = 15;
pub const EXTID_EXTRAFAILFLAG: _externids = 14;
// IHATESCRIPTSANDEVERYTHINGTHEYSTANDFOR(ESPECIALLYONSUNDAYS)
pub const EXTID_EXTRAVICTORYFLAG: _externids = 13;
pub const EXTID_TARGETTYPE: _externids = 12;
pub const EXTID_INTMODE: _externids = 11;
pub const EXTID_CURSOR: _externids = 10;
pub const EXTID_MULTIGAMEBASETYPE: _externids = 9;
pub const EXTID_MULTIGAMEHUMANMAX: _externids = 8;
pub const EXTID_MULTIGAMETYPE: _externids = 7;
pub const EXTID_TUTORIAL: _externids = 6;
pub const EXTID_GAMETIME: _externids = 5;
pub const EXTID_GAMELEVEL: _externids = 4;
pub const EXTID_SELECTEDPLAYER: _externids = 3;
pub const EXTID_GAMEINIT: _externids = 2;
pub const EXTID_MAPHEIGHT: _externids = 1;
pub const EXTID_MAPWIDTH: _externids = 0;
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
/*
 * ScriptExtern.c
 *
 * All game variable access functions for the scripts
 *
 */
// current game level
#[no_mangle]
pub static mut scrGameLevel: SDWORD = 0 as libc::c_int;
// whether the tutorial is active
#[no_mangle]
pub static mut bInTutorial: BOOL = 0 as libc::c_int;
// whether any additional special case victory/failure conditions have been met
#[no_mangle]
pub static mut bExtraVictoryFlag: BOOL = 0 as libc::c_int;
#[no_mangle]
pub static mut bExtraFailFlag: BOOL = 0 as libc::c_int;
// whether or not to track the player's transporter as it comes
// into an offworld mission.
#[no_mangle]
pub static mut bTrackTransporter: BOOL = 0 as libc::c_int;
// whether or not we're running on the PSX.
#[no_mangle]
pub static mut bIsPSX: BOOL = 0 as libc::c_int;
// reset the script externals for a new level
// reset the script externals for a new level
#[no_mangle]
pub unsafe extern "C" fn scrExternReset() {
    scrGameLevel = 0 as libc::c_int;
    bInTutorial = 0 as libc::c_int;
    bExtraVictoryFlag = 0 as libc::c_int;
    bExtraFailFlag = 0 as libc::c_int;
}
// General function to get some basic game values
// General function to get some basic game values
#[no_mangle]
pub unsafe extern "C" fn scrGenExternGet(mut index: UDWORD) -> BOOL {
    let mut type_0: INTERP_TYPE = VAL_BOOL; // from  rendfunc.c
    let mut val: SDWORD = 0;
    match index {
        15 => { type_0 = VAL_BOOL; val = bTrackTransporter }
        16 => { type_0 = VAL_BOOL; val = bIsPSX }
        0 => { type_0 = VAL_INT; val = mapWidth as SDWORD }
        1 => { type_0 = VAL_INT; val = mapHeight as SDWORD }
        2 => { type_0 = VAL_BOOL; val = gameInitialised }
        3 => { type_0 = VAL_INT; val = selectedPlayer as SDWORD }
        4 => { type_0 = VAL_INT; val = scrGameLevel }
        5 => {
            type_0 = VAL_INT;
            val =
                gameTime.wrapping_div(100 as libc::c_int as libc::c_uint) as
                    SDWORD
        }
        6 => { type_0 = VAL_BOOL; val = bInTutorial }
        10 => { type_0 = VAL_INT; val = iV_GetMouseFrame() as SDWORD }
        11 => { type_0 = VAL_INT; val = intMode as SDWORD }
        12 => { type_0 = VAL_INT; val = getTargetType() as SDWORD }
        13 => { type_0 = VAL_BOOL; val = bExtraVictoryFlag }
        14 => { type_0 = VAL_BOOL; val = bExtraFailFlag }
        7 => {
            // multiplayer variable..
            type_0 = VAL_INT;
            val = game.type_0 as SDWORD
        }
        8 => {
            // multiplayer variable..
            type_0 = VAL_INT;
            val = game.maxPlayers as SDWORD
        }
        9 => { type_0 = VAL_INT; val = game.base as SDWORD }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"scrGenExternGet: unknown variable index\x00" as
                          *const u8 as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"scriptextern.c\x00" as *const u8 as
                          *const libc::c_char, 140 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 16],
                                                &[libc::c_char; 16]>(b"scrGenExternGet\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            return 0 as libc::c_int
        }
    }
    if stackPushResult(type_0, val) == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
// General function to set some basic game values
// General function to set some basic game values
#[no_mangle]
pub unsafe extern "C" fn scrGenExternSet(mut index: UDWORD) -> BOOL {
    let mut sVal: INTERP_VAL =
        INTERP_VAL{type_0: VAL_BOOL, v: C2RustUnnamed{bval: 0,},};
    let mut type_0: INTERP_TYPE = VAL_BOOL;
    let mut val: SDWORD = 0;
    // Get the value and store it in type,val
    if stackPop(&mut sVal) == 0 { return 0 as libc::c_int }
    type_0 = sVal.type_0;
    val = sVal.v.ival;
    match index {
        4 => {
            if type_0 as libc::c_uint !=
                   VAL_INT as libc::c_int as libc::c_uint {
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"invalid type for gameLevel\x00" as *const u8 as
                              *const libc::c_char);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"scriptextern.c\x00" as *const u8 as
                              *const libc::c_char, 174 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 16],
                                                    &[libc::c_char; 16]>(b"scrGenExternSet\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
                return 0 as libc::c_int
            }
            scrGameLevel = val
        }
        6 => {
            if type_0 as libc::c_uint !=
                   VAL_BOOL as libc::c_int as libc::c_uint {
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"invalid type for inTutorial\x00" as *const u8 as
                              *const libc::c_char);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"scriptextern.c\x00" as *const u8 as
                              *const libc::c_char, 182 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 16],
                                                    &[libc::c_char; 16]>(b"scrGenExternSet\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
                return 0 as libc::c_int
            }
            bInTutorial = val
        }
        13 => {
            if type_0 as libc::c_uint !=
                   VAL_BOOL as libc::c_int as libc::c_uint {
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"invalid type for extraVictoryFlag\x00" as
                              *const u8 as *const libc::c_char);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"scriptextern.c\x00" as *const u8 as
                              *const libc::c_char, 190 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 16],
                                                    &[libc::c_char; 16]>(b"scrGenExternSet\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
                return 0 as libc::c_int
            }
            bExtraVictoryFlag = val
        }
        14 => {
            if type_0 as libc::c_uint !=
                   VAL_BOOL as libc::c_int as libc::c_uint {
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"invalid type for extraFailFlag\x00" as *const u8
                              as *const libc::c_char);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"scriptextern.c\x00" as *const u8 as
                              *const libc::c_char, 198 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 16],
                                                    &[libc::c_char; 16]>(b"scrGenExternSet\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
                return 0 as libc::c_int
            }
            bExtraFailFlag = val
        }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"scrGenExternSet: unknown variable index\x00" as
                          *const u8 as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"scriptextern.c\x00" as *const u8 as
                          *const libc::c_char, 204 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 16],
                                                &[libc::c_char; 16]>(b"scrGenExternSet\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            return 0 as libc::c_int
        }
    }
    return 1 as libc::c_int;
}
