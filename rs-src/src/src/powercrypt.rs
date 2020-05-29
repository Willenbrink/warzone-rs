use ::libc;
extern "C" {
    #[no_mangle]
    fn rand() -> libc::c_int;
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
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_uint)
     -> *mut libc::c_void;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
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
    // the game description.
    #[no_mangle]
    static mut bMultiPlayer: BOOL;
    // true when more than 1 player.
    #[no_mangle]
    static mut selectedPlayer: UDWORD;
    #[no_mangle]
    fn getPlayerName(player: UDWORD) -> *mut STRING;
    // send a destruct feature message.      
    #[no_mangle]
    fn sendTextMessage(pStr: *mut libc::c_char, cast: BOOL) -> BOOL;
    // ////////////////////////////////////////////////////////////////////////
// variables
    #[no_mangle]
    static mut NetPlay: NETPLAY;
}
pub type UBYTE = libc::c_uchar;
pub type STRING = libc::c_char;
pub type UWORD = libc::c_ushort;
pub type UDWORD = libc::c_uint;
pub type SDWORD = libc::c_int;
pub type BOOL = libc::c_int;
pub type DWORD = libc::c_int;
/* !WIN32 */
pub type DPID = libc::c_int;
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
// store copies of the power value encrypted to foil cheaters
pub type PWRC_STORE = _pwrc_store;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _pwrc_store {
    pub key: UDWORD,
    pub pad1: UWORD,
    pub xorHigh: UWORD,
    pub xorLow: UWORD,
    pub pad2: UBYTE,
    pub aShuffle: [UBYTE; 4],
    pub valid: BOOL,
    pub lastSent: UDWORD,
}
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
#[no_mangle]
pub static mut asPCrypt: [PWRC_STORE; 8] =
    [PWRC_STORE{key: 0,
                pad1: 0,
                xorHigh: 0,
                xorLow: 0,
                pad2: 0,
                aShuffle: [0; 4],
                valid: 0,
                lastSent: 0,}; 8];
#[no_mangle]
pub static mut invalidPower: [BOOL; 8] = [0; 8];
// xor key
// get of a DWORD boundary
// get even more off a DWORD boundary
/*
 * PowerCrypt.h
 *
 * Set up a seperate encrypted copy of each players power.
 *
 */
// set the current power value
// set the current power value
#[no_mangle]
pub unsafe extern "C" fn pwrcSetPlayerCryptPower(mut player: UDWORD,
                                                 mut power: UDWORD) {
    let mut pPower: *mut UBYTE = 0 as *mut UBYTE;
    if player < 8 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"pwrcSetPlayerCryptPower: invalid player number\x00" as
                  *const u8 as *const libc::c_char);
    };
    if player < 8 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"powercrypt.c\x00" as *const u8 as *const libc::c_char,
              44 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 24],
                                        &[libc::c_char; 24]>(b"pwrcSetPlayerCryptPower\x00")).as_ptr(),
              b"player < MAX_PLAYERS\x00" as *const u8 as
                  *const libc::c_char);
    };
    asPCrypt[player as usize].pad1 = rand() as UWORD;
    asPCrypt[player as usize].key = rand() as UDWORD;
    asPCrypt[player as usize].xorHigh =
        (power >> 16 as libc::c_int ^
             asPCrypt[player as usize].key >> 16 as libc::c_int) as UWORD;
    asPCrypt[player as usize].xorLow =
        (power ^ asPCrypt[player as usize].key) as UWORD;
    asPCrypt[player as usize].pad2 = rand() as UBYTE;
    pPower = &mut power as *mut UDWORD as *mut UBYTE;
    asPCrypt[player as usize].aShuffle[0 as libc::c_int as usize] =
        *pPower.offset(3 as libc::c_int as isize);
    asPCrypt[player as usize].aShuffle[1 as libc::c_int as usize] =
        *pPower.offset(0 as libc::c_int as isize);
    asPCrypt[player as usize].aShuffle[2 as libc::c_int as usize] =
        *pPower.offset(2 as libc::c_int as isize);
    asPCrypt[player as usize].aShuffle[3 as libc::c_int as usize] =
        *pPower.offset(1 as libc::c_int as isize);
    // the encrypted power is definately valid
    asPCrypt[player as usize].valid = 1 as libc::c_int;
    asPCrypt[player as usize].lastSent = 0 as libc::c_int as UDWORD;
}
// get the current power value
#[no_mangle]
pub unsafe extern "C" fn pwrcGetPlayerCryptPower(mut player: UDWORD)
 -> UDWORD {
    if player < 8 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"pwrcGetPlayerCryptPower: invalid player number\x00" as
                  *const u8 as *const libc::c_char);
    };
    if player < 8 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"powercrypt.c\x00" as *const u8 as *const libc::c_char,
              69 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 24],
                                        &[libc::c_char; 24]>(b"pwrcGetPlayerCryptPower\x00")).as_ptr(),
              b"player < MAX_PLAYERS\x00" as *const u8 as
                  *const libc::c_char);
    };
    return 0 as libc::c_int as UDWORD;
}
// check the current power value
// check the current power value
#[no_mangle]
pub unsafe extern "C" fn pwrcCheckPlayerCryptPower(mut player: UDWORD,
                                                   mut power: UDWORD)
 -> BOOL {
    let mut aPower: [UBYTE; 4] = [0; 4];
    let mut match_0: BOOL = 0;
    let mut res: UWORD = 0;
    if player < 8 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"pwrcCheckPlayerCryptPower: invalid player number\x00" as
                  *const u8 as *const libc::c_char);
    };
    if player < 8 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"powercrypt.c\x00" as *const u8 as *const libc::c_char,
              83 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 26],
                                        &[libc::c_char; 26]>(b"pwrcCheckPlayerCryptPower\x00")).as_ptr(),
              b"player < MAX_PLAYERS\x00" as *const u8 as
                  *const libc::c_char);
    };
    if bMultiPlayer == 0 || NetPlay.bComms == 0 || player != selectedPlayer {
        return 1 as libc::c_int
    }
    match_0 = 1 as libc::c_int;
    res =
        (power >> 16 as libc::c_int ^
             asPCrypt[player as usize].key >> 16 as libc::c_int) as UWORD;
    if res as libc::c_int != asPCrypt[player as usize].xorHigh as libc::c_int
       {
        match_0 = 0 as libc::c_int
    }
    res = (power ^ asPCrypt[player as usize].key) as UWORD;
    if asPCrypt[player as usize].xorLow as libc::c_int != res as libc::c_int {
        match_0 = 0 as libc::c_int
    }
    memcpy(aPower.as_mut_ptr() as *mut libc::c_void,
           &mut power as *mut UDWORD as *const libc::c_void,
           4 as libc::c_int as libc::c_uint);
    match_0 =
        (match_0 != 0 &&
             asPCrypt[player as usize].aShuffle[0 as libc::c_int as usize] as
                 libc::c_int ==
                 aPower[3 as libc::c_int as usize] as libc::c_int) as
            libc::c_int;
    match_0 =
        (match_0 != 0 &&
             asPCrypt[player as usize].aShuffle[1 as libc::c_int as usize] as
                 libc::c_int ==
                 aPower[0 as libc::c_int as usize] as libc::c_int) as
            libc::c_int;
    match_0 =
        (match_0 != 0 &&
             asPCrypt[player as usize].aShuffle[2 as libc::c_int as usize] as
                 libc::c_int ==
                 aPower[2 as libc::c_int as usize] as libc::c_int) as
            libc::c_int;
    match_0 =
        (match_0 != 0 &&
             asPCrypt[player as usize].aShuffle[3 as libc::c_int as usize] as
                 libc::c_int ==
                 aPower[1 as libc::c_int as usize] as libc::c_int) as
            libc::c_int;
    // if the power didn't match set the flag so that messages can be sent
    if match_0 == 0 { asPCrypt[player as usize].valid = 0 as libc::c_int }
    return match_0;
}
// check the valid flags and scream if the power isn't valid
// check the valid flags and scream if the power isn't valid
#[no_mangle]
pub unsafe extern "C" fn pwrcUpdate() {
    let mut i: SDWORD = 0;
    let mut aBuff: [STRING; 1024] = [0; 1024];
    if bMultiPlayer == 0 || NetPlay.bComms == 0 { return }
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        if i == selectedPlayer as SDWORD && asPCrypt[i as usize].valid == 0 &&
               asPCrypt[i as
                            usize].lastSent.wrapping_add(10000 as libc::c_int
                                                             as libc::c_uint)
                   < gameTime {
            sprintf(aBuff.as_mut_ptr(),
                    b"WARNING %s IS CHEATING (Power Changed)\x00" as *const u8
                        as *const libc::c_char, getPlayerName(i as UDWORD));
            sendTextMessage(aBuff.as_mut_ptr(), 1 as libc::c_int);
            asPCrypt[i as usize].lastSent = gameTime
        }
        i += 1
    };
}
