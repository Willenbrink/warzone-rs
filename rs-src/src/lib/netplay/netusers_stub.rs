use ::libc;
extern "C" {
    // ////////////////////////////////////////////////////////////////////////
// variables
    #[no_mangle]
    static mut NetPlay: NETPLAY;
}
pub type UBYTE = libc::c_uchar;
pub type UDWORD = libc::c_uint;
pub type GUID = libc::c_int;
pub type BOOL = libc::c_int;
pub type DWORD = libc::c_int;
/* !WIN32 */
pub type DPID = libc::c_int;
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
pub struct GAMESTRUCT {
    pub name: [libc::c_char; 64],
    pub desc: SESSIONDESC,
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
//from netusers.c
/*
 * netusers.c
 * functions regarding specific players
 */
// call to disable/enable ALL comms. Absolute arse of a thing, be very careful!
#[no_mangle]
pub unsafe extern "C" fn NETuseNetwork(mut val: BOOL) -> BOOL {
    NetPlay.bComms = val;
    return 1 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////
// Functions for spectators.
#[no_mangle]
pub unsafe extern "C" fn NETspectate(mut guidSessionInstance: GUID) -> BOOL 
 // FIXME Remove if unused
 {
    return 0 as libc::c_int;
}
// create a spectator
// ////////////////////////////////////////////////////////////////////////
#[no_mangle]
pub unsafe extern "C" fn NETisSpectator(mut dpid: DPID) -> BOOL 
 // FIXME Remove if unused
 {
    let mut i: UBYTE = 0;
    // was "dpid = NetPlay.dpidPlayer == dpid", but that didn't make sense.
    dpid = NetPlay.dpidPlayer;
    if dpid == dpid {
        // checking ourselves
        return NetPlay.bSpectator
    }
    // could enumerate the spectators and check if he's there!
	// bugger it, just check that dpid isn't a player instead!
    i = 0 as libc::c_int as UBYTE;
    while (i as libc::c_int) < 8 as libc::c_int {
        if NetPlay.players[i as usize].dpid == dpid {
            return 0 as libc::c_int
        }
        i = i.wrapping_add(1)
    }
    return 1 as libc::c_int;
}
