use ::libc;
extern "C" {
    pub type _TCPsocket;
    pub type _SDLNet_SocketSet;
    #[no_mangle]
    fn clock() -> clock_t;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn SDL_GetError() -> *mut libc::c_char;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_uint)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_uint)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_uint;
    #[no_mangle]
    fn malloc(_: libc::c_uint) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn SDLNet_FreeSocketSet(set: SDLNet_SocketSet);
    #[no_mangle]
    fn SDLNet_Init() -> libc::c_int;
    #[no_mangle]
    fn SDLNet_Quit();
    #[no_mangle]
    fn SDLNet_ResolveHost(address: *mut IPaddress, host: *const libc::c_char,
                          port: Uint16) -> libc::c_int;
    #[no_mangle]
    fn SDLNet_TCP_Open(ip: *mut IPaddress) -> TCPsocket;
    #[no_mangle]
    fn SDLNet_TCP_Accept(server: TCPsocket) -> TCPsocket;
    #[no_mangle]
    fn SDLNet_TCP_Send(sock: TCPsocket, data: *const libc::c_void,
                       len: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn SDLNet_TCP_Recv(sock: TCPsocket, data: *mut libc::c_void,
                       maxlen: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn SDLNet_TCP_Close(sock: TCPsocket);
    #[no_mangle]
    fn SDLNet_AllocSocketSet(maxsockets: libc::c_int) -> SDLNet_SocketSet;
    #[no_mangle]
    fn SDLNet_AddSocket(set: SDLNet_SocketSet, sock: SDLNet_GenericSocket)
     -> libc::c_int;
    #[no_mangle]
    fn SDLNet_DelSocket(set: SDLNet_SocketSet, sock: SDLNet_GenericSocket)
     -> libc::c_int;
    #[no_mangle]
    fn SDLNet_CheckSockets(set: SDLNet_SocketSet, timeout: Uint32)
     -> libc::c_int;
    #[no_mangle]
    fn PHYSFS_openWrite(filename: *const libc::c_char) -> *mut PHYSFS_File;
    #[no_mangle]
    fn PHYSFS_openRead(filename: *const libc::c_char) -> *mut PHYSFS_File;
    #[no_mangle]
    fn PHYSFS_close(handle: *mut PHYSFS_File) -> libc::c_int;
    #[no_mangle]
    fn PHYSFS_read(handle: *mut PHYSFS_File, buffer: *mut libc::c_void,
                   objSize: PHYSFS_uint32, objCount: PHYSFS_uint32)
     -> PHYSFS_sint64;
    #[no_mangle]
    fn PHYSFS_write(handle: *mut PHYSFS_File, buffer: *const libc::c_void,
                    objSize: PHYSFS_uint32, objCount: PHYSFS_uint32)
     -> PHYSFS_sint64;
    #[no_mangle]
    fn PHYSFS_seek(handle: *mut PHYSFS_File, pos: PHYSFS_uint64)
     -> libc::c_int;
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
    //from netusers.c
    #[no_mangle]
    fn NETuseNetwork(val: BOOL) -> BOOL;
    // encryption
    #[no_mangle]
    fn NETsetKey(c1: UDWORD, c2: UDWORD, c3: UDWORD, c4: UDWORD) -> BOOL;
    #[no_mangle]
    fn NETmanglePacket(msg: *mut NETMSG) -> *mut NETMSG;
    #[no_mangle]
    fn NETunmanglePacket(msg: *mut NETMSG);
    #[no_mangle]
    fn NETstartLogging() -> BOOL;
    #[no_mangle]
    fn NETstopLogging() -> BOOL;
    #[no_mangle]
    fn MultiPlayerJoin(dpid: DPID) -> BOOL;
    #[no_mangle]
    fn MultiPlayerLeave(dpid: DPID) -> BOOL;
}
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __clock_t = libc::c_long;
pub type clock_t = __clock_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type Uint16 = uint16_t;
pub type Uint32 = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IPaddress {
    pub host: Uint32,
    pub port: Uint16,
}
pub type TCPsocket = *mut _TCPsocket;
pub type SDLNet_SocketSet = *mut _SDLNet_SocketSet;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _SDLNet_GenericSocket {
    pub ready: libc::c_int,
}
pub type SDLNet_GenericSocket = *mut _SDLNet_GenericSocket;
pub type PHYSFS_uint32 = libc::c_uint;
pub type PHYSFS_uint64 = libc::c_ulonglong;
pub type PHYSFS_sint64 = libc::c_longlong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PHYSFS_File {
    pub opaque: *mut libc::c_void,
}
pub type UBYTE = libc::c_uchar;
pub type UDWORD = libc::c_uint;
pub type HRESULT = libc::c_int;
pub type BOOL = libc::c_int;
pub type CHAR = libc::c_char;
pub type UCHAR = libc::c_uchar;
pub type DWORD = libc::c_int;
pub type LPSTR = *mut libc::c_char;
pub type LPVOID = *mut libc::c_void;
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
pub struct NETMSG {
    pub size: libc::c_ushort,
    pub paddedBytes: libc::c_uchar,
    pub type_0: libc::c_uchar,
    pub destination: libc::c_uchar,
    pub body: [libc::c_char; 8000],
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NETSTATS {
    pub bytesRecvd: UDWORD,
    pub bytesSent: UDWORD,
    pub packetsSent: UDWORD,
    pub packetsRecvd: UDWORD,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NETBUFSOCKET {
    pub socket: TCPsocket,
    pub buffer: *mut libc::c_char,
    pub buffer_start: libc::c_uint,
    pub bytes: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NET_PLAYER {
    pub allocated: BOOL,
    pub id: libc::c_uint,
    pub name: [libc::c_char; 64],
    pub flags: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NET_PLAYER_DATA {
    pub size: libc::c_uint,
    pub data: *mut libc::c_void,
    pub buffer_size: libc::c_uint,
}
/*
 * Netplay.c
 *
 * Alex Lee sep97-> june98
 */
// ////////////////////////////////////////////////////////////////////////
// includes
// for stats
#[no_mangle]
pub static mut master_server: *mut libc::c_char =
    b"warzone2100.kicks-ass.org\x00" as *const u8 as *const libc::c_char as
        *mut libc::c_char;
//#define NET_DEBUG
static mut allow_joining: BOOL = 0 as libc::c_int;
// ////////////////////////////////////////////////////////////////////////
// Variables
#[no_mangle]
pub static mut NetPlay: NETPLAY =
    NETPLAY{games:
                [GAMESTRUCT{name: [0; 64],
                            desc:
                                SESSIONDESC{dwSize: 0,
                                            dwFlags: 0,
                                            host: [0; 16],
                                            dwMaxPlayers: 0,
                                            dwCurrentPlayers: 0,
                                            dwUser1: 0,
                                            dwUser2: 0,
                                            dwUser3: 0,
                                            dwUser4: 0,},}; 12],
            players:
                [PLAYER{dpid: 0, name: [0; 64], bHost: 0, bSpectator: 0,}; 8],
            playercount: 0,
            dpidPlayer: 0,
            bComms: 0,
            bHost: 0,
            bLobbyLaunched: 0,
            bSpectator: 0,
            bEncryptAllPackets: 0,
            cryptKey: [0; 4],
            bCaptureInUse: 0,
            bAllowCaptureRecord: 0,
            bAllowCapturePlay: 0,};
static mut game: GAMESTRUCT =
    GAMESTRUCT{name: [0; 64],
               desc:
                   SESSIONDESC{dwSize: 0,
                               dwFlags: 0,
                               host: [0; 16],
                               dwMaxPlayers: 0,
                               dwCurrentPlayers: 0,
                               dwUser1: 0,
                               dwUser2: 0,
                               dwUser3: 0,
                               dwUser4: 0,},};
#[no_mangle]
pub unsafe extern "C" fn NETsetMessageSize(mut pMsg: *mut NETMSG,
                                           mut size: libc::c_uint) {
    let mut tmp: libc::c_uint =
        (8 as libc::c_int as
             libc::c_uint).wrapping_sub(size.wrapping_rem(8 as libc::c_int as
                                                              libc::c_uint));
    if tmp == 8 as libc::c_int as libc::c_uint {
        (*pMsg).size = size as libc::c_ushort;
        (*pMsg).paddedBytes = 0 as libc::c_int as libc::c_uchar
    } else {
        (*pMsg).size = size.wrapping_add(tmp) as libc::c_ushort;
        (*pMsg).paddedBytes = tmp as libc::c_uchar
    };
}
#[no_mangle]
pub unsafe extern "C" fn NET_createBufferedSocket() -> *mut NETBUFSOCKET {
    let mut bs: *mut NETBUFSOCKET =
        malloc(::std::mem::size_of::<NETBUFSOCKET>() as libc::c_ulong) as
            *mut NETBUFSOCKET;
    (*bs).socket = 0 as TCPsocket;
    (*bs).buffer = 0 as *mut libc::c_char;
    (*bs).buffer_start = 0 as libc::c_int as libc::c_uint;
    (*bs).bytes = 0 as libc::c_int as libc::c_uint;
    return bs;
}
#[no_mangle]
pub unsafe extern "C" fn NET_destroyBufferedSocket(mut bs:
                                                       *mut NETBUFSOCKET) {
    free((*bs).buffer as *mut libc::c_void);
    free(bs as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn NET_initBufferedSocket(mut bs: *mut NETBUFSOCKET,
                                                mut s: TCPsocket) {
    (*bs).socket = s;
    if (*bs).buffer.is_null() {
        (*bs).buffer =
            malloc(1024 as libc::c_int as libc::c_uint) as *mut libc::c_char
    }
    (*bs).buffer_start = 0 as libc::c_int as libc::c_uint;
    (*bs).bytes = 0 as libc::c_int as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn NET_fillBuffer(mut bs: *mut NETBUFSOCKET,
                                        mut socket_set_0: SDLNet_SocketSet)
 -> BOOL {
    let mut size: libc::c_int = 0;
    let mut bufstart: *mut libc::c_char =
        (*bs).buffer.offset((*bs).buffer_start as
                                isize).offset((*bs).bytes as isize);
    let bufsize: libc::c_int =
        (1024 as libc::c_int as
             libc::c_uint).wrapping_sub((*bs).buffer_start).wrapping_sub((*bs).bytes)
            as libc::c_int;
    if (*bs).buffer_start != 0 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int
    }
    if (!(*bs).socket.is_null() &&
            (*((*bs).socket as SDLNet_GenericSocket)).ready != 0) as
           libc::c_int <= 0 as libc::c_int {
        return 0 as libc::c_int
    }
    size =
        SDLNet_TCP_Recv((*bs).socket, bufstart as *mut libc::c_void, bufsize);
    if size > 0 as libc::c_int {
        (*bs).bytes = (*bs).bytes.wrapping_add(size as libc::c_uint);
        return 1 as libc::c_int
    } else {
        if !socket_set_0.is_null() {
            SDLNet_DelSocket(socket_set_0,
                             (*bs).socket as SDLNet_GenericSocket);
        }
        SDLNet_TCP_Close((*bs).socket);
        (*bs).socket = 0 as TCPsocket
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn NET_recvMessage(mut bs: *mut NETBUFSOCKET,
                                         mut pMsg: *mut NETMSG) -> BOOL {
    let mut size: libc::c_uint = 0;
    let mut message_0: *const NETMSG =
        (*bs).buffer.offset((*bs).buffer_start as isize) as *mut NETMSG;
    let headersize: libc::c_uint =
        (::std::mem::size_of::<libc::c_ushort>() as
             libc::c_ulong).wrapping_add(::std::mem::size_of::<libc::c_uchar>()
                                             as
                                             libc::c_ulong).wrapping_add(::std::mem::size_of::<libc::c_uchar>()
                                                                             as
                                                                             libc::c_ulong).wrapping_add(::std::mem::size_of::<libc::c_uchar>()
                                                                                                             as
                                                                                                             libc::c_ulong);
    if !(headersize > (*bs).bytes) {
        size = ((*message_0).size as libc::c_uint).wrapping_add(headersize);
        if !(size > (*bs).bytes) {
            debug(LOG_NET,
                  b"NETrecvMessage: received message of type %i and size %i.\n\x00"
                      as *const u8 as *const libc::c_char,
                  (*message_0).type_0 as libc::c_int,
                  (*message_0).size as libc::c_int);
            memcpy(pMsg as *mut libc::c_void,
                   message_0 as *const libc::c_void, size);
            (*bs).buffer_start = (*bs).buffer_start.wrapping_add(size);
            (*bs).bytes = (*bs).bytes.wrapping_sub(size);
            return 1 as libc::c_int
        }
    }
    if (*bs).buffer_start != 0 as libc::c_int as libc::c_uint {
        static mut tmp_buffer: *mut libc::c_char =
            0 as *const libc::c_char as *mut libc::c_char;
        let mut buffer_start: *mut libc::c_char =
            (*bs).buffer.offset((*bs).buffer_start as isize);
        let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
        //printf("Moving data in buffer\n");
        // Create tmp buffer if necessary
        if tmp_buffer.is_null() {
            //printf("Creating tmp buffer\n");
            tmp_buffer =
                malloc(1024 as libc::c_int as libc::c_uint) as
                    *mut libc::c_char
        }
        // Move remaining contents into tmp buffer
        memcpy(tmp_buffer as *mut libc::c_void,
               buffer_start as *const libc::c_void, (*bs).bytes);
        // swap tmp buffer with buffer
        tmp = (*bs).buffer;
        (*bs).buffer = tmp_buffer;
        tmp_buffer = tmp;
        // Now data is in the beginning of the buffer
        (*bs).buffer_start = 0 as libc::c_int as libc::c_uint
    }
    return 0 as libc::c_int;
}
static mut tcp_socket: TCPsocket = 0 as *const _TCPsocket as TCPsocket;
static mut bsocket: *mut NETBUFSOCKET =
    0 as *const NETBUFSOCKET as *mut NETBUFSOCKET;
static mut connected_bsocket: [*mut NETBUFSOCKET; 8] =
    [0 as *const NETBUFSOCKET as *mut NETBUFSOCKET,
     0 as *const NETBUFSOCKET as *mut NETBUFSOCKET,
     0 as *const NETBUFSOCKET as *mut NETBUFSOCKET,
     0 as *const NETBUFSOCKET as *mut NETBUFSOCKET,
     0 as *const NETBUFSOCKET as *mut NETBUFSOCKET,
     0 as *const NETBUFSOCKET as *mut NETBUFSOCKET,
     0 as *const NETBUFSOCKET as *mut NETBUFSOCKET,
     0 as *const NETBUFSOCKET as *mut NETBUFSOCKET];
static mut socket_set: SDLNet_SocketSet =
    0 as *const _SDLNet_SocketSet as SDLNet_SocketSet;
static mut is_server: BOOL = 0 as libc::c_int;
static mut tmp_socket: [TCPsocket; 16] =
    [0 as *const _TCPsocket as TCPsocket,
     0 as *const _TCPsocket as *mut _TCPsocket,
     0 as *const _TCPsocket as *mut _TCPsocket,
     0 as *const _TCPsocket as *mut _TCPsocket,
     0 as *const _TCPsocket as *mut _TCPsocket,
     0 as *const _TCPsocket as *mut _TCPsocket,
     0 as *const _TCPsocket as *mut _TCPsocket,
     0 as *const _TCPsocket as *mut _TCPsocket,
     0 as *const _TCPsocket as *mut _TCPsocket,
     0 as *const _TCPsocket as *mut _TCPsocket,
     0 as *const _TCPsocket as *mut _TCPsocket,
     0 as *const _TCPsocket as *mut _TCPsocket,
     0 as *const _TCPsocket as *mut _TCPsocket,
     0 as *const _TCPsocket as *mut _TCPsocket,
     0 as *const _TCPsocket as *mut _TCPsocket,
     0 as *const _TCPsocket as *mut _TCPsocket];
static mut tmp_socket_set: SDLNet_SocketSet =
    0 as *const _SDLNet_SocketSet as SDLNet_SocketSet;
static mut message: NETMSG =
    NETMSG{size: 0,
           paddedBytes: 0,
           type_0: 0,
           destination: 0,
           body: [0; 8000],};
static mut hostname: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
static mut nStats: NETSTATS =
    {
        let mut init =
            NETSTATS{bytesRecvd: 0 as libc::c_int as UDWORD,
                     bytesSent: 0,
                     packetsSent: 0,
                     packetsRecvd: 0,};
        init
    };
static mut players: [NET_PLAYER; 8] =
    [NET_PLAYER{allocated: 0, id: 0, name: [0; 64], flags: 0,}; 8];
#[no_mangle]
pub unsafe extern "C" fn NET_InitPlayers() {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < 8 as libc::c_int as libc::c_uint {
        players[i as usize].allocated = 0 as libc::c_int;
        players[i as usize].id = i;
        i = i.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn NETBroadcastPlayerInfo(mut dpid: libc::c_int) {
    message.type_0 = 92 as libc::c_int as libc::c_uchar;
    NETsetMessageSize(&mut message,
                      ::std::mem::size_of::<NET_PLAYER>() as libc::c_ulong);
    memcpy(message.body.as_mut_ptr() as *mut libc::c_void,
           &mut *players.as_mut_ptr().offset(dpid as isize) as *mut NET_PLAYER
               as *const libc::c_void,
           ::std::mem::size_of::<NET_PLAYER>() as libc::c_ulong);
    NETbcast(&mut message, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn NET_CreatePlayer(mut name: *const libc::c_char,
                                          mut flags: libc::c_uint)
 -> libc::c_int {
    let mut i: libc::c_uint = 0;
    i = 1 as libc::c_int as libc::c_uint;
    while i < 8 as libc::c_int as libc::c_uint {
        if players[i as usize].allocated == 0 as libc::c_int {
            players[i as usize].allocated = 1 as libc::c_int;
            strcpy(players[i as usize].name.as_mut_ptr(), name);
            players[i as usize].flags = flags;
            NETBroadcastPlayerInfo(i as libc::c_int);
            return i as libc::c_int
        }
        i = i.wrapping_add(1)
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn NET_DestroyPlayer(mut id: libc::c_uint) {
    players[id as usize].allocated = 0 as libc::c_int;
}
// TURN on/off networking.
// ////////////////////////////////////////////////////////////////////////
// count players. call with null to enumerate the game already joined.
#[no_mangle]
pub unsafe extern "C" fn NETplayerInfo() -> UDWORD {
    let mut i: libc::c_uint = 0; // reset player counter
    NetPlay.playercount = 0 as libc::c_int as UDWORD; // reset player info
    if NetPlay.bComms == 0 {
        NetPlay.playercount = 1 as libc::c_int as UDWORD;
        NetPlay.players[0 as libc::c_int as usize].bHost = 1 as libc::c_int;
        NetPlay.players[0 as libc::c_int as usize].bSpectator =
            0 as libc::c_int;
        NetPlay.players[0 as libc::c_int as usize].dpid = 1 as libc::c_int;
        return 1 as libc::c_int as UDWORD
    }
    memset(NetPlay.players.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           (::std::mem::size_of::<PLAYER>() as
                libc::c_ulong).wrapping_mul(8 as libc::c_int as
                                                libc::c_uint));
    i = 0 as libc::c_int as libc::c_uint;
    while i < 8 as libc::c_int as libc::c_uint {
        if players[i as usize].allocated == 1 as libc::c_int {
            NetPlay.players[NetPlay.playercount as usize].dpid = i as DPID;
            strcpy(NetPlay.players[NetPlay.playercount as
                                       usize].name.as_mut_ptr(),
                   players[i as usize].name.as_mut_ptr());
            if players[i as usize].flags & 1 as libc::c_int as libc::c_uint !=
                   0 {
                NetPlay.players[NetPlay.playercount as usize].bHost =
                    1 as libc::c_int
            } else {
                NetPlay.players[NetPlay.playercount as usize].bHost =
                    0 as libc::c_int
            }
            if players[i as usize].flags & 2 as libc::c_int as libc::c_uint !=
                   0 {
                NetPlay.players[NetPlay.playercount as usize].bSpectator =
                    1 as libc::c_int
            } else {
                NetPlay.players[NetPlay.playercount as usize].bSpectator =
                    0 as libc::c_int
            }
            NetPlay.playercount = NetPlay.playercount.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    }
    return NetPlay.playercount;
}
// count players in this game.
// ////////////////////////////////////////////////////////////////////////
// rename the local player
// dont call this a lot, since it uses a guaranteed msg.
#[no_mangle]
pub unsafe extern "C" fn NETchangePlayerName(mut dpid: UDWORD,
                                             mut newName: *mut libc::c_char)
 -> BOOL {
    if NetPlay.bComms == 0 {
        strcpy(NetPlay.players[0 as libc::c_int as usize].name.as_mut_ptr(),
               newName);
        return 1 as libc::c_int
    }
    strcpy(players[dpid as usize].name.as_mut_ptr(), newName);
    NETBroadcastPlayerInfo(dpid as libc::c_int);
    return 1 as libc::c_int;
}
static mut local_player_data: [NET_PLAYER_DATA; 8] =
    [{
         let mut init =
             NET_PLAYER_DATA{size: 0 as libc::c_int as libc::c_uint,
                             data:
                                 0 as *const libc::c_void as
                                     *mut libc::c_void,
                             buffer_size: 0,};
         init
     },
     NET_PLAYER_DATA{size: 0,
                     data: 0 as *const libc::c_void as *mut libc::c_void,
                     buffer_size: 0,},
     NET_PLAYER_DATA{size: 0,
                     data: 0 as *const libc::c_void as *mut libc::c_void,
                     buffer_size: 0,},
     NET_PLAYER_DATA{size: 0,
                     data: 0 as *const libc::c_void as *mut libc::c_void,
                     buffer_size: 0,},
     NET_PLAYER_DATA{size: 0,
                     data: 0 as *const libc::c_void as *mut libc::c_void,
                     buffer_size: 0,},
     NET_PLAYER_DATA{size: 0,
                     data: 0 as *const libc::c_void as *mut libc::c_void,
                     buffer_size: 0,},
     NET_PLAYER_DATA{size: 0,
                     data: 0 as *const libc::c_void as *mut libc::c_void,
                     buffer_size: 0,},
     NET_PLAYER_DATA{size: 0,
                     data: 0 as *const libc::c_void as *mut libc::c_void,
                     buffer_size: 0,}];
static mut global_player_data: [NET_PLAYER_DATA; 8] =
    [{
         let mut init =
             NET_PLAYER_DATA{size: 0 as libc::c_int as libc::c_uint,
                             data:
                                 0 as *const libc::c_void as
                                     *mut libc::c_void,
                             buffer_size: 0,};
         init
     },
     NET_PLAYER_DATA{size: 0,
                     data: 0 as *const libc::c_void as *mut libc::c_void,
                     buffer_size: 0,},
     NET_PLAYER_DATA{size: 0,
                     data: 0 as *const libc::c_void as *mut libc::c_void,
                     buffer_size: 0,},
     NET_PLAYER_DATA{size: 0,
                     data: 0 as *const libc::c_void as *mut libc::c_void,
                     buffer_size: 0,},
     NET_PLAYER_DATA{size: 0,
                     data: 0 as *const libc::c_void as *mut libc::c_void,
                     buffer_size: 0,},
     NET_PLAYER_DATA{size: 0,
                     data: 0 as *const libc::c_void as *mut libc::c_void,
                     buffer_size: 0,},
     NET_PLAYER_DATA{size: 0,
                     data: 0 as *const libc::c_void as *mut libc::c_void,
                     buffer_size: 0,},
     NET_PLAYER_DATA{size: 0,
                     data: 0 as *const libc::c_void as *mut libc::c_void,
                     buffer_size: 0,}];
#[no_mangle]
pub unsafe extern "C" fn resize_local_player_data(mut i: libc::c_uint,
                                                  mut size: libc::c_uint) {
    if local_player_data[i as usize].buffer_size < size {
        if !local_player_data[i as usize].data.is_null() {
            free(local_player_data[i as usize].data);
        }
        local_player_data[i as usize].data = malloc(size);
        local_player_data[i as usize].buffer_size = size
    };
}
#[no_mangle]
pub unsafe extern "C" fn resize_global_player_data(mut i: libc::c_uint,
                                                   mut size: libc::c_uint) {
    if global_player_data[i as usize].buffer_size < size {
        if !global_player_data[i as usize].data.is_null() {
            free(global_player_data[i as usize].data);
        }
        global_player_data[i as usize].data = malloc(size);
        global_player_data[i as usize].buffer_size = size
    };
}
// change a players name.
// ////////////////////////////////////////////////////////////////////////
#[no_mangle]
pub unsafe extern "C" fn NETgetLocalPlayerData(mut dpid: DPID,
                                               mut pData: *mut libc::c_void,
                                               mut pSize: *mut DWORD)
 -> BOOL {
    memcpy(pData, local_player_data[dpid as usize].data,
           local_player_data[dpid as usize].size);
    return 1 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////
#[no_mangle]
pub unsafe extern "C" fn NETgetGlobalPlayerData(mut dpid: DPID,
                                                mut pData: *mut libc::c_void,
                                                mut pSize: *mut DWORD)
 -> BOOL {
    if NetPlay.bComms == 0 {
        memcpy(pData, local_player_data[dpid as usize].data,
               local_player_data[dpid as usize].size);
        return 1 as libc::c_int
    }
    memcpy(pData, global_player_data[dpid as usize].data,
           global_player_data[dpid as usize].size);
    return 1 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////
#[no_mangle]
pub unsafe extern "C" fn NETsetLocalPlayerData(mut dpid: DPID,
                                               mut pData: *mut libc::c_void,
                                               mut size: DWORD) -> BOOL {
    local_player_data[dpid as usize].size = size as libc::c_uint;
    resize_local_player_data(dpid as libc::c_uint, size as libc::c_uint);
    memcpy(local_player_data[dpid as usize].data, pData,
           size as libc::c_uint);
    return 1 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////
#[no_mangle]
pub unsafe extern "C" fn NETsetGlobalPlayerData(mut dpid: DPID,
                                                mut pData: *mut libc::c_void,
                                                mut size: DWORD) -> BOOL {
    if NetPlay.bComms == 0 {
        local_player_data[dpid as usize].size = size as libc::c_uint;
        resize_local_player_data(dpid as libc::c_uint, size as libc::c_uint);
        memcpy(local_player_data[dpid as usize].data, pData,
               size as libc::c_uint);
        return 1 as libc::c_int
    }
    global_player_data[dpid as usize].size = size as libc::c_uint;
    resize_global_player_data(dpid as libc::c_uint, size as libc::c_uint);
    memcpy(global_player_data[dpid as usize].data, pData,
           size as libc::c_uint);
    // broadcast player data
    let mut p_dpid: *mut libc::c_uint =
        message.body.as_mut_ptr() as *mut libc::c_uint;
    message.type_0 = 93 as libc::c_int as libc::c_uchar;
    NETsetMessageSize(&mut message,
                      (::std::mem::size_of::<libc::c_uint>() as
                           libc::c_ulong).wrapping_add(size as libc::c_uint));
    *p_dpid = dpid as libc::c_uint;
    memcpy(message.body.as_mut_ptr().offset(::std::mem::size_of::<libc::c_uint>()
                                                as libc::c_ulong as isize) as
               *mut libc::c_void, pData, size as libc::c_uint);
    NETBroadcastPlayerInfo(dpid);
    return 1 as libc::c_int;
}
// ///////////////////////////////////////////////////////////////////////
// Game flags stuff...
#[no_mangle]
pub static mut NetGameFlags: [DWORD; 4] = [0; 4];
// from netjoin.c
// ////////////////////////////////////////////////////////////////////////
// return one of the four user flags in the current sessiondescription.
#[no_mangle]
pub unsafe extern "C" fn NETgetGameFlags(mut flag: UDWORD) -> DWORD {
    if flag < 1 as libc::c_int as libc::c_uint ||
           flag > 4 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int
    } else {
        return NetGameFlags[flag.wrapping_sub(1 as libc::c_int as
                                                  libc::c_uint) as usize]
    };
}
// return one of the four flags(dword) about the game.
// ////////////////////////////////////////////////////////////////////////
// Set a game flag
#[no_mangle]
pub unsafe extern "C" fn NETsetGameFlags(mut flag: UDWORD, mut value: DWORD)
 -> BOOL {
    if NetPlay.bComms == 0 { return 1 as libc::c_int }
    if flag < 1 as libc::c_int as libc::c_uint ||
           flag > 4 as libc::c_int as libc::c_uint {
        NetGameFlags[flag.wrapping_sub(1 as libc::c_int as libc::c_uint) as
                         usize] = value;
        return NetGameFlags[flag.wrapping_sub(1 as libc::c_int as
                                                  libc::c_uint) as usize]
    }
    message.type_0 = 96 as libc::c_int as libc::c_uchar;
    NETsetMessageSize(&mut message,
                      ::std::mem::size_of::<[DWORD; 4]>() as libc::c_ulong);
    memcpy(message.body.as_mut_ptr() as *mut libc::c_void,
           NetGameFlags.as_mut_ptr() as *const libc::c_void,
           ::std::mem::size_of::<[DWORD; 4]>() as libc::c_ulong);
    NETbcast(&mut message, 1 as libc::c_int);
    return 1 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////
// functions available to you.
// ////////////////////////////////////////////////////////////////////////
// setup stuff
#[no_mangle]
pub unsafe extern "C" fn NETinit(mut bFirstCall: BOOL) -> BOOL {
    let mut i: UDWORD = 0;
    debug(LOG_NET, b"NETinit\x00" as *const u8 as *const libc::c_char);
    //	NEThashFile("warzonedebug.exe");
    if bFirstCall != 0 {
        debug(LOG_NEVER,
              b"NETPLAY: Init called, MORNIN\'\n\x00" as *const u8 as
                  *const libc::c_char); // clean up
        NetPlay.bLobbyLaunched =
            0 as libc::c_int; // j-random key to get us started
        NetPlay.dpidPlayer = 0 as libc::c_int;
        NetPlay.bHost = 0 as libc::c_int;
        NetPlay.bComms = 1 as libc::c_int;
        NetPlay.bEncryptAllPackets = 0 as libc::c_int;
        NETsetKey(0x2fe8f810 as libc::c_int as UDWORD,
                  0xb72a5 as libc::c_int as UDWORD,
                  0x114d0 as libc::c_int as UDWORD,
                  0x2a7 as libc::c_int as UDWORD);
        NetPlay.bAllowCaptureRecord = 0 as libc::c_int;
        NetPlay.bAllowCapturePlay = 0 as libc::c_int;
        NetPlay.bCaptureInUse = 0 as libc::c_int;
        i = 0 as libc::c_int as UDWORD;
        while i < 8 as libc::c_int as libc::c_uint {
            memset(&mut *NetPlay.players.as_mut_ptr().offset(i as isize) as
                       *mut PLAYER as *mut libc::c_void, 0 as libc::c_int,
                   ::std::mem::size_of::<PLAYER>() as libc::c_ulong);
            memset(&mut *NetPlay.games.as_mut_ptr().offset(i as isize) as
                       *mut GAMESTRUCT as *mut libc::c_void, 0 as libc::c_int,
                   ::std::mem::size_of::<GAMESTRUCT>() as libc::c_ulong);
            i = i.wrapping_add(1)
        }
        //GAME_GUID = g;
        NETuseNetwork(1 as libc::c_int);
        NETstartLogging();
    }
    if SDLNet_Init() == -(1 as libc::c_int) {
        debug(LOG_ERROR,
              b"SDLNet_Init: %s\n\x00" as *const u8 as *const libc::c_char,
              SDL_GetError());
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
// close current game
// ////////////////////////////////////////////////////////////////////////
// SHUTDOWN THE CONNECTION.
#[no_mangle]
pub unsafe extern "C" fn NETshutdown() -> HRESULT {
    let mut i: libc::c_uint = 0;
    debug(LOG_NET, b"NETshutdown\x00" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int as libc::c_uint;
    while i < 8 as libc::c_int as libc::c_uint {
        if !local_player_data[i as usize].data.is_null() {
            free(local_player_data[i as usize].data);
        }
        i = i.wrapping_add(1)
    }
    NETstopLogging();
    SDLNet_Quit();
    return 0 as libc::c_int;
}
// recv file chunk
// ////////////////////////////////////////////////////////////////////////
//close the open game..
#[no_mangle]
pub unsafe extern "C" fn NETclose() -> HRESULT {
    let mut i: libc::c_uint = 0;
    debug(LOG_NET, b"NETclose\n\x00" as *const u8 as *const libc::c_char);
    NEThaltJoining();
    is_server = 0 as libc::c_int;
    if !bsocket.is_null() {
        NET_destroyBufferedSocket(bsocket);
        bsocket = 0 as *mut NETBUFSOCKET
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < 8 as libc::c_int as libc::c_uint {
        if !connected_bsocket[i as usize].is_null() {
            NET_destroyBufferedSocket(connected_bsocket[i as usize]);
            connected_bsocket[i as usize] = 0 as *mut NETBUFSOCKET
        }
        NET_DestroyPlayer(i);
        i = i.wrapping_add(1)
    }
    if !tmp_socket_set.is_null() {
        SDLNet_FreeSocketSet(tmp_socket_set);
        tmp_socket_set = 0 as SDLNet_SocketSet
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < 16 as libc::c_int as libc::c_uint {
        if !tmp_socket[i as usize].is_null() {
            SDLNet_TCP_Close(tmp_socket[i as usize]);
            tmp_socket[i as usize] = 0 as TCPsocket
        }
        i = i.wrapping_add(1)
    }
    if !socket_set.is_null() {
        SDLNet_FreeSocketSet(socket_set);
        socket_set = 0 as SDLNet_SocketSet
    }
    if !tcp_socket.is_null() {
        SDLNet_TCP_Close(tcp_socket);
        tcp_socket = 0 as TCPsocket
    }
    return 0 as libc::c_int;
}
// leave the game in play.
// ////////////////////////////////////////////////////////////////////////
// ////////////////////////////////////////////////////////////////////////
// Send and Recv functions
// ////////////////////////////////////////////////////////////////////////
// return bytes of data sent recently.
#[no_mangle]
pub unsafe extern "C" fn NETgetBytesSent() -> UDWORD {
    static mut lastsec: UDWORD = 0 as libc::c_int as UDWORD;
    static mut timy: UDWORD = 0 as libc::c_int as UDWORD;
    if clock() as UDWORD as libc::c_ulong >
           (timy as
                libc::c_ulong).wrapping_add(1000000 as libc::c_int as
                                                __clock_t as libc::c_ulong) {
        timy = clock() as UDWORD;
        lastsec = nStats.bytesSent;
        nStats.bytesSent = 0 as libc::c_int as UDWORD
    }
    return lastsec;
}
// return packets sent/recv.  call regularly for good results
#[no_mangle]
pub unsafe extern "C" fn NETgetRecentBytesSent() -> UDWORD {
    return nStats.bytesSent;
}
// return packets sent/recv.  call regularly for good results
#[no_mangle]
pub unsafe extern "C" fn NETgetBytesRecvd() -> UDWORD {
    static mut lastsec: UDWORD = 0 as libc::c_int as UDWORD;
    static mut timy: UDWORD = 0 as libc::c_int as UDWORD;
    if clock() as UDWORD as libc::c_ulong >
           (timy as
                libc::c_ulong).wrapping_add(1000000 as libc::c_int as
                                                __clock_t as libc::c_ulong) {
        timy = clock() as UDWORD;
        lastsec = nStats.bytesRecvd;
        nStats.bytesRecvd = 0 as libc::c_int as UDWORD
    }
    return lastsec;
}
#[no_mangle]
pub unsafe extern "C" fn NETgetRecentBytesRecvd() -> UDWORD {
    return nStats.bytesRecvd;
}
// return bytes sent/recv.  call regularly for good results
//return number of packets sent last sec.
#[no_mangle]
pub unsafe extern "C" fn NETgetPacketsSent() -> UDWORD {
    static mut lastsec: UDWORD = 0 as libc::c_int as UDWORD;
    static mut timy: UDWORD = 0 as libc::c_int as UDWORD;
    if clock() as UDWORD as libc::c_ulong >
           (timy as
                libc::c_ulong).wrapping_add(1000000 as libc::c_int as
                                                __clock_t as libc::c_ulong) {
        timy = clock() as UDWORD;
        lastsec = nStats.packetsSent;
        nStats.packetsSent = 0 as libc::c_int as UDWORD
    }
    return lastsec;
}
// more immediate functions.
#[no_mangle]
pub unsafe extern "C" fn NETgetRecentPacketsSent() -> UDWORD {
    return nStats.packetsSent;
}
// return bytes sent/recv.  call regularly for good results
#[no_mangle]
pub unsafe extern "C" fn NETgetPacketsRecvd() -> UDWORD {
    static mut lastsec: UDWORD = 0 as libc::c_int as UDWORD;
    static mut timy: UDWORD = 0 as libc::c_int as UDWORD;
    if clock() as UDWORD as libc::c_ulong >
           (timy as
                libc::c_ulong).wrapping_add(1000000 as libc::c_int as
                                                __clock_t as libc::c_ulong) {
        timy = clock() as UDWORD;
        lastsec = nStats.packetsRecvd;
        nStats.packetsRecvd = 0 as libc::c_int as UDWORD
    }
    return lastsec;
}
#[no_mangle]
pub unsafe extern "C" fn NETgetRecentPacketsRecvd() -> UDWORD {
    return nStats.packetsRecvd;
}
//choose one. 
// ////////////////////////////////////////////////////////////////////////
// Send a message to a player, option to guarantee message
#[no_mangle]
pub unsafe extern "C" fn NETsend(mut msg: *mut NETMSG, mut player: DPID,
                                 mut guarantee: BOOL) -> BOOL {
    let mut size: libc::c_uint = 0;
    debug(LOG_NET, b"NETsend\n\x00" as *const u8 as *const libc::c_char);
    if NetPlay.bComms == 0 { return 1 as libc::c_int }
    if player >= 8 as libc::c_int { return 0 as libc::c_int }
    //printf("Sending message %i to %i\n", msg->type, msg->destination);
    (*msg).destination = player as libc::c_uchar;
    if NetPlay.bEncryptAllPackets != 0 &&
           (*msg).type_0 as libc::c_int != 255 as libc::c_int &&
           (*msg).type_0 as libc::c_int != 254 as libc::c_int {
        NETmanglePacket(msg);
    }
    //printf("NETsend %i\n", msg->type);
    size =
        ((*msg).size as
             libc::c_uint).wrapping_add(::std::mem::size_of::<libc::c_ushort>()
                                            as
                                            libc::c_ulong).wrapping_add(::std::mem::size_of::<libc::c_uchar>()
                                                                            as
                                                                            libc::c_ulong).wrapping_add(::std::mem::size_of::<libc::c_uchar>()
                                                                                                            as
                                                                                                            libc::c_ulong).wrapping_add(::std::mem::size_of::<libc::c_uchar>()
                                                                                                                                            as
                                                                                                                                            libc::c_ulong);
    if is_server != 0 {
        if player < 8 as libc::c_int &&
               !connected_bsocket[player as usize].is_null() &&
               !(*connected_bsocket[player as usize]).socket.is_null() &&
               SDLNet_TCP_Send((*connected_bsocket[player as usize]).socket,
                               msg as *const libc::c_void,
                               size as libc::c_int) as libc::c_uint == size {
            nStats.bytesSent =
                (nStats.bytesSent as libc::c_uint).wrapping_add(size) as
                    UDWORD as UDWORD;
            nStats.packetsSent =
                (nStats.packetsSent as
                     libc::c_uint).wrapping_add(1 as libc::c_int as
                                                    libc::c_uint) as UDWORD as
                    UDWORD;
            return 1 as libc::c_int
        }
    } else if !tcp_socket.is_null() &&
                  SDLNet_TCP_Send(tcp_socket, msg as *const libc::c_void,
                                  size as libc::c_int) as libc::c_uint == size
     {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
// send to player, possibly guaranteed
// ////////////////////////////////////////////////////////////////////////
// broadcast a message to all players.
#[no_mangle]
pub unsafe extern "C" fn NETbcast(mut msg: *mut NETMSG, mut guarantee: BOOL)
 -> BOOL {
    let mut size: libc::c_uint = 0;
    debug(LOG_NET, b"NETbcast\n\x00" as *const u8 as *const libc::c_char);
    if NetPlay.bComms == 0 { return 1 as libc::c_int }
    (*msg).destination = 255 as libc::c_int as libc::c_uchar;
    if NetPlay.bEncryptAllPackets != 0 &&
           (*msg).type_0 as libc::c_int != 255 as libc::c_int &&
           (*msg).type_0 as libc::c_int != 254 as libc::c_int {
        NETmanglePacket(msg);
    }
    size =
        ((*msg).size as
             libc::c_uint).wrapping_add(::std::mem::size_of::<libc::c_ushort>()
                                            as
                                            libc::c_ulong).wrapping_add(::std::mem::size_of::<libc::c_uchar>()
                                                                            as
                                                                            libc::c_ulong).wrapping_add(::std::mem::size_of::<libc::c_uchar>()
                                                                                                            as
                                                                                                            libc::c_ulong).wrapping_add(::std::mem::size_of::<libc::c_uchar>()
                                                                                                                                            as
                                                                                                                                            libc::c_ulong);
    //printf("NETbcast %i\n", msg->type);
    if is_server != 0 {
        let mut i: libc::c_uint = 0;
        i = 0 as libc::c_int as libc::c_uint;
        while i < 8 as libc::c_int as libc::c_uint {
            if !connected_bsocket[i as usize].is_null() &&
                   !(*connected_bsocket[i as usize]).socket.is_null() {
                SDLNet_TCP_Send((*connected_bsocket[i as usize]).socket,
                                msg as *const libc::c_void,
                                size as libc::c_int);
            }
            i = i.wrapping_add(1)
        }
    } else if tcp_socket.is_null() ||
                  (SDLNet_TCP_Send(tcp_socket, msg as *const libc::c_void,
                                   size as libc::c_int) as libc::c_uint) <
                      size {
        return 0 as libc::c_int
    }
    nStats.bytesSent =
        (nStats.bytesSent as libc::c_uint).wrapping_add(size) as UDWORD as
            UDWORD;
    nStats.packetsSent =
        (nStats.packetsSent as
             libc::c_uint).wrapping_add(1 as libc::c_int as libc::c_uint) as
            UDWORD as UDWORD;
    return 1 as libc::c_int;
}
// /////////////////////////////////////////////////////////////////////////
// Check if a message is a system message
#[no_mangle]
pub unsafe extern "C" fn NETprocessSystemMessage(mut pMsg: *mut NETMSG)
 -> BOOL {
    debug(LOG_NET,
          b"NETprocessSystemMessage\n\x00" as *const u8 as
              *const libc::c_char);
    match (*pMsg).type_0 as libc::c_int {
        92 => {
            let mut pi: *mut NET_PLAYER =
                (*pMsg).body.as_mut_ptr() as *mut NET_PLAYER;
            let mut dpid: libc::c_int = (*pi).id as libc::c_int;
            //printf("Just received and processed MSG_PLAYER_INFO\n");
            memcpy(&mut *players.as_mut_ptr().offset(dpid as isize) as
                       *mut NET_PLAYER as *mut libc::c_void,
                   pi as *const libc::c_void,
                   ::std::mem::size_of::<NET_PLAYER>() as libc::c_ulong);
            NETplayerInfo();
            if is_server != 0 { NETBroadcastPlayerInfo(dpid); }
        }
        93 => {
            let mut pdpid: *mut libc::c_uint =
                (*pMsg).body.as_mut_ptr() as *mut libc::c_uint;
            let mut dpid_0: libc::c_int = *pdpid as libc::c_int;
            let mut size: libc::c_uint =
                ((*pMsg).size as
                     libc::c_uint).wrapping_sub(::std::mem::size_of::<libc::c_uint>()
                                                    as libc::c_ulong);
            //printf("Receiving info for player %i\n", dpid);
            //printf("Just received and processed MSG_PLAYER_DATA\n");
            global_player_data[dpid_0 as usize].size = size;
            resize_global_player_data(dpid_0 as libc::c_uint, size);
            memcpy(global_player_data[dpid_0 as usize].data,
                   (*pMsg).body.as_mut_ptr().offset(::std::mem::size_of::<libc::c_uint>()
                                                        as libc::c_ulong as
                                                        isize) as
                       *const libc::c_void, size);
            if is_server != 0 { NETbcast(pMsg, 1 as libc::c_int); }
        }
        94 => {
            let mut pdpid_0: *mut libc::c_uint =
                (*pMsg).body.as_mut_ptr() as *mut libc::c_uint;
            let mut dpid_1: libc::c_int = *pdpid_0 as libc::c_int;
            MultiPlayerJoin(dpid_1);
        }
        95 => {
            let mut pdpid_1: *mut libc::c_uint =
                (*pMsg).body.as_mut_ptr() as *mut libc::c_uint;
            let mut dpid_2: libc::c_int = *pdpid_1 as libc::c_int;
            NET_DestroyPlayer(dpid_2 as libc::c_uint);
            MultiPlayerLeave(dpid_2);
        }
        96 => {
            //printf("Receiving DATA for player %i\n", dpid);
            //printf("Receiving game flags\n");
            memcpy(NetGameFlags.as_mut_ptr() as *mut libc::c_void,
                   (*pMsg).body.as_mut_ptr() as *const libc::c_void,
                   ::std::mem::size_of::<[DWORD; 4]>() as libc::c_ulong);
            if is_server != 0 { NETbcast(pMsg, 1 as libc::c_int); }
            //printf("Just received and processed MSG_GAME_FLAGS\n");
        }
        _ => { return 0 as libc::c_int }
    }
    return 1 as libc::c_int;
}
// broadcast to everyone, possibly guaranteed
// ////////////////////////////////////////////////////////////////////////
// receive a message over the current connection
#[no_mangle]
pub unsafe extern "C" fn NETrecv(mut pMsg: *mut NETMSG) -> BOOL {
    static mut current: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut received: BOOL = 0;
    let mut size: libc::c_int = 0;
    if NetPlay.bComms == 0 { return 0 as libc::c_int }
    if is_server != 0 { NETallowJoining(); }
    loop  {
        loop  {
            received = 0 as libc::c_int;
            //printf("Receiving something\n");
            if is_server != 0 {
                if connected_bsocket[current as usize].is_null() {
                    return 0 as libc::c_int
                }
                received =
                    NET_recvMessage(connected_bsocket[current as usize],
                                    pMsg);
                if received == 0 as libc::c_int {
                    let mut i: libc::c_uint =
                        current.wrapping_add(1 as libc::c_int as
                                                 libc::c_uint);
                    if socket_set.is_null() ||
                           SDLNet_CheckSockets(socket_set,
                                               0 as libc::c_int as Uint32) <=
                               0 as libc::c_int {
                        return 0 as libc::c_int
                    }
                    loop  {
                        if !(*connected_bsocket[i as usize]).socket.is_null()
                           {
                            if NET_fillBuffer(connected_bsocket[i as usize],
                                              socket_set) != 0 {
                                received =
                                    NET_recvMessage(connected_bsocket[i as
                                                                          usize],
                                                    pMsg);
                                current = i;
                                break ;
                            } else if (*connected_bsocket[i as
                                                              usize]).socket.is_null()
                             {
                                let mut message_dpid: *mut libc::c_uint =
                                    message.body.as_mut_ptr() as
                                        *mut libc::c_uint;
                                game.desc.dwCurrentPlayers -= 1;
                                message.type_0 =
                                    95 as libc::c_int as libc::c_uchar;
                                NETsetMessageSize(&mut message,
                                                  4 as libc::c_int as
                                                      libc::c_uint);
                                *message_dpid = i;
                                NETbcast(&mut message, 1 as libc::c_int);
                                NET_DestroyPlayer(i);
                                MultiPlayerLeave(i as DPID);
                            }
                        }
                        i = i.wrapping_add(1);
                        if i == 8 as libc::c_int as libc::c_uint {
                            i = 0 as libc::c_int as libc::c_uint
                        }
                        if i ==
                               current.wrapping_add(1 as libc::c_int as
                                                        libc::c_uint) {
                            return 0 as libc::c_int
                        }
                    }
                }
            } else if bsocket.is_null() {
                return 0 as libc::c_int
            } else {
                received = NET_recvMessage(bsocket, pMsg);
                if received == 0 as libc::c_int {
                    if !socket_set.is_null() &&
                           SDLNet_CheckSockets(socket_set,
                                               0 as libc::c_int as Uint32) >
                               0 as libc::c_int &&
                           NET_fillBuffer(bsocket, socket_set) != 0 {
                        received = NET_recvMessage(bsocket, pMsg)
                    }
                }
            }
            if received == 0 as libc::c_int {
                return 0 as libc::c_int
            } else {
                size =
                    ((*pMsg).size as
                         libc::c_uint).wrapping_add(::std::mem::size_of::<libc::c_ushort>()
                                                        as
                                                        libc::c_ulong).wrapping_add(::std::mem::size_of::<libc::c_uchar>()
                                                                                        as
                                                                                        libc::c_ulong).wrapping_add(::std::mem::size_of::<libc::c_uchar>()
                                                                                                                        as
                                                                                                                        libc::c_ulong).wrapping_add(::std::mem::size_of::<libc::c_uchar>()
                                                                                                                                                        as
                                                                                                                                                        libc::c_ulong)
                        as libc::c_int;
                if is_server == 0 as libc::c_int { break ; }
                if (*pMsg).destination as libc::c_int == 255 as libc::c_int {
                    let mut j: libc::c_uint = 0;
                    j = 0 as libc::c_int as libc::c_uint;
                    while j < 8 as libc::c_int as libc::c_uint {
                        if j != current &&
                               !connected_bsocket[j as usize].is_null() &&
                               !(*connected_bsocket[j as
                                                        usize]).socket.is_null()
                           {
                            SDLNet_TCP_Send((*connected_bsocket[j as
                                                                    usize]).socket,
                                            pMsg as *const libc::c_void,
                                            size);
                        }
                        j = j.wrapping_add(1)
                    }
                    break ;
                } else {
                    if !((*pMsg).destination as libc::c_int !=
                             NetPlay.dpidPlayer) {
                        break ;
                    }
                    if ((*pMsg).destination as libc::c_int) < 8 as libc::c_int
                           &&
                           !connected_bsocket[(*pMsg).destination as
                                                  usize].is_null() &&
                           !(*connected_bsocket[(*pMsg).destination as
                                                    usize]).socket.is_null() {
                        //printf("Reflecting message to DPID %i\n", pMsg->destination);
                        SDLNet_TCP_Send((*connected_bsocket[(*pMsg).destination
                                                                as
                                                                usize]).socket,
                                        pMsg as *const libc::c_void, size);
                    }
                }
            }
        }
        if (*pMsg).type_0 as libc::c_int >= 100 as libc::c_int &&
               (*pMsg).type_0 as libc::c_int != 255 as libc::c_int &&
               (*pMsg).type_0 as libc::c_int != 254 as libc::c_int {
            NETunmanglePacket(pMsg);
        }
        //printf("Received message : type = %i, size = %i, size = %i.\n", pMsg->type, pMsg->size, size);
        nStats.bytesRecvd =
            (nStats.bytesRecvd as
                 libc::c_uint).wrapping_add(size as libc::c_uint) as UDWORD as
                UDWORD;
        nStats.packetsRecvd =
            (nStats.packetsRecvd as
                 libc::c_uint).wrapping_add(1 as libc::c_int as libc::c_uint)
                as UDWORD as UDWORD;
        if !(NETprocessSystemMessage(pMsg) == 1 as libc::c_int) { break ; }
    }
    return 1 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////
// ////////////////////////////////////////////////////////////////////////
// Protocol functions
#[no_mangle]
pub unsafe extern "C" fn NETsetupTCPIP(mut addr: *mut LPVOID,
                                       mut machine: *mut libc::c_char)
 -> BOOL {
    debug(LOG_NET,
          b"NETsetupTCPIP\n\x00" as *const u8 as *const libc::c_char);
    if !hostname.is_null() && hostname != master_server {
        free(hostname as *mut libc::c_void);
    }
    if !machine.is_null() &&
           *machine.offset(0 as libc::c_int as isize) as libc::c_int !=
               '\u{0}' as i32 {
        hostname = strdup(machine)
    } else { hostname = master_server }
    return 1 as libc::c_int;
}
//put connections in Protocols[] (Lobbies optional)
// select a current protocol. Call with a connection returned by findprotocol.
#[no_mangle]
pub unsafe extern "C" fn NETselectProtocol(mut lpConnection: LPVOID) -> BOOL {
    return 1 as libc::c_int;
}
//init(guid can be NULL)		
// ////////////////////////////////////////////////////////////////////////
// call with true to enumerate available protocols.
#[no_mangle]
pub unsafe extern "C" fn NETfindProtocol(mut Lob: BOOL) -> BOOL {
    return 1 as libc::c_int;
}
// recv a message if possible
// ////////////////////////////////////////////////////////////////////////
// File Transfer programs.
// uses guaranteed messages to send files between clients.
// send file. it returns % of file sent. when 100 it's complete. call until it returns 100.
#[no_mangle]
pub unsafe extern "C" fn NETsendFile(mut newFile: BOOL,
                                     mut fileName: *mut CHAR,
                                     mut player: DPID) -> UCHAR {
    static mut fileSize: PHYSFS_sint64 = 0;
    static mut currPos: PHYSFS_sint64 = 0;
    static mut pFileHandle: *mut PHYSFS_File =
        0 as *const PHYSFS_File as *mut PHYSFS_File;
    let mut bytesRead: PHYSFS_sint64 = 0;
    let mut inBuff: [UBYTE; 2048] = [0; 2048];
    let mut msg: NETMSG =
        NETMSG{size: 0,
               paddedBytes: 0,
               type_0: 0,
               destination: 0,
               body: [0; 8000],};
    if newFile != 0 {
        //force it true for now -Q
        // open the file.
        pFileHandle = PHYSFS_openRead(fileName); // check file exists
        if pFileHandle.is_null() {
            debug(LOG_WZ,
                  b"NETsendFile: Failed\n\x00" as *const u8 as
                      *const libc::c_char);
            return 0 as libc::c_int as UCHAR
            // failed
        }
        fileSize = 0 as libc::c_int as PHYSFS_sint64;
        currPos = 0 as libc::c_int as PHYSFS_sint64;
        loop  {
            bytesRead =
                PHYSFS_read(pFileHandle,
                            &mut inBuff as *mut [UBYTE; 2048] as
                                *mut libc::c_void,
                            1 as libc::c_int as PHYSFS_uint32,
                            ::std::mem::size_of::<[UBYTE; 2048]>() as
                                libc::c_ulong);
            fileSize += bytesRead;
            if !(bytesRead != 0 as libc::c_int as libc::c_longlong) {
                break ;
            }
        }
        PHYSFS_seek(pFileHandle, 0 as libc::c_int as PHYSFS_uint64);
    }
    // get the file's size.
    // read some bytes.
    bytesRead =
        PHYSFS_read(pFileHandle,
                    &mut inBuff as *mut [UBYTE; 2048] as *mut libc::c_void,
                    ::std::mem::size_of::<[UBYTE; 2048]>() as libc::c_ulong,
                    1 as libc::c_int as PHYSFS_uint32);
    // form a message
    memcpy(&mut *msg.body.as_mut_ptr().offset(0 as libc::c_int as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut fileSize as *mut PHYSFS_sint64 as *const libc::c_void,
           ::std::mem::size_of::<PHYSFS_sint64>() as
               libc::c_ulong); // total bytes in this file.
    memcpy(&mut *msg.body.as_mut_ptr().offset(4 as libc::c_int as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut bytesRead as *mut PHYSFS_sint64 as *const libc::c_void,
           ::std::mem::size_of::<PHYSFS_sint64>() as
               libc::c_ulong); // bytes in this packet
    memcpy(&mut *msg.body.as_mut_ptr().offset(8 as libc::c_int as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut currPos as *mut PHYSFS_sint64 as *const libc::c_void,
           ::std::mem::size_of::<PHYSFS_sint64>() as
               libc::c_ulong); // start byte
    msg.body[12 as libc::c_int as usize] = strlen(fileName) as libc::c_char;
    msg.size = 13 as libc::c_int as libc::c_ushort;
    strcpy(&mut *msg.body.as_mut_ptr().offset(msg.size as isize), fileName);
    msg.size =
        (msg.size as libc::c_uint).wrapping_add(strlen(fileName)) as
            libc::c_ushort as libc::c_ushort;
    memcpy(&mut *msg.body.as_mut_ptr().offset(msg.size as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut inBuff as *mut [UBYTE; 2048] as *const libc::c_void,
           bytesRead as libc::c_uint);
    msg.size = (msg.size as libc::c_longlong + bytesRead) as libc::c_ushort;
    msg.type_0 = 254 as libc::c_int as libc::c_uchar;
    msg.paddedBytes = 0 as libc::c_int as libc::c_uchar;
    if player == 0 as libc::c_int {
        NETbcast(&mut msg, 1 as libc::c_int);
        // send it.
    } else {
        NETsend(&mut msg, player, 1 as libc::c_int); // update position!
    }
    currPos += bytesRead;
    if currPos == fileSize { PHYSFS_close(pFileHandle); }
    return (currPos * 100 as libc::c_int as libc::c_longlong / fileSize) as
               UCHAR;
}
// send file chunk.
// recv file. it returns % of the file so far recvd.
#[no_mangle]
pub unsafe extern "C" fn NETrecvFile(mut pMsg: *mut NETMSG) -> UCHAR {
    let mut pos: UDWORD = 0;
    let mut fileSize: UDWORD = 0;
    let mut currPos: UDWORD = 0;
    let mut bytesRead: UDWORD = 0;
    let mut fileName: [libc::c_char; 256] = [0; 256];
    let mut len: libc::c_uint = 0;
    static mut pFileHandle: *mut PHYSFS_File =
        0 as *const PHYSFS_File as *mut PHYSFS_File;
    //read incoming bytes.
    memcpy(&mut fileSize as *mut UDWORD as *mut libc::c_void,
           &mut *(*pMsg).body.as_mut_ptr().offset(0 as libc::c_int as isize)
               as *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    memcpy(&mut bytesRead as *mut UDWORD as *mut libc::c_void,
           &mut *(*pMsg).body.as_mut_ptr().offset(4 as libc::c_int as isize)
               as *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    memcpy(&mut currPos as *mut UDWORD as *mut libc::c_void,
           &mut *(*pMsg).body.as_mut_ptr().offset(8 as libc::c_int as isize)
               as *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    // read filename
    len =
        (*pMsg).body[12 as libc::c_int as usize] as
            libc::c_uint; // terminate string.
    memcpy(fileName.as_mut_ptr() as *mut libc::c_void,
           &mut *(*pMsg).body.as_mut_ptr().offset(13 as libc::c_int as isize)
               as *mut libc::c_char as *const libc::c_void, len);
    fileName[len as usize] = '\u{0}' as i32 as libc::c_char;
    pos = (13 as libc::c_int as libc::c_uint).wrapping_add(len);
    //printf("NETrecvFile: Creating new file %s\n", fileName);
    if currPos == 0 as libc::c_int as libc::c_uint {
        // first packet!
        //printf("NETrecvFile: Creating new file %s\n", fileName);
        pFileHandle = PHYSFS_openWrite(fileName.as_mut_ptr())
    }
    //write packet to the file.
    PHYSFS_write(pFileHandle,
                 &mut *(*pMsg).body.as_mut_ptr().offset(pos as isize) as
                     *mut libc::c_char as *const libc::c_void, bytesRead,
                 1 as libc::c_int as PHYSFS_uint32);
    if currPos.wrapping_add(bytesRead) == fileSize {
        // last packet
        PHYSFS_close(pFileHandle);
    }
    //return the percent count.
    return currPos.wrapping_add(bytesRead).wrapping_mul(100 as libc::c_int as
                                                            libc::c_uint).wrapping_div(fileSize)
               as UCHAR;
}
#[no_mangle]
pub unsafe extern "C" fn NETregisterServer(mut state: libc::c_int) {
    static mut rs_socket: TCPsocket = 0 as *const _TCPsocket as TCPsocket;
    static mut registered: libc::c_int = 0 as libc::c_int;
    static mut server_not_there: libc::c_int = 0 as libc::c_int;
    let mut ip: IPaddress = IPaddress{host: 0, port: 0,};
    if server_not_there != 0 { return }
    if state != registered {
        match state {
            1 => {
                if SDLNet_ResolveHost(&mut ip, master_server,
                                      9998 as libc::c_int as Uint16) ==
                       -(1 as libc::c_int) {
                    debug(LOG_ERROR,
                          b"NETregisterServer: couldn\'t resolve master server (%s): %s\n\x00"
                              as *const u8 as *const libc::c_char,
                          master_server, SDL_GetError());
                    server_not_there = 1 as libc::c_int;
                    return
                }
                if rs_socket.is_null() {
                    rs_socket = SDLNet_TCP_Open(&mut ip)
                }
                if rs_socket.is_null() {
                    debug(LOG_ERROR,
                          b"NETregisterServer: Cannot connect to master server (%s): %s\n\x00"
                              as *const u8 as *const libc::c_char,
                          master_server, SDL_GetError());
                    server_not_there = 1 as libc::c_int;
                    return
                }
                SDLNet_TCP_Send(rs_socket,
                                b"addg\x00" as *const u8 as
                                    *const libc::c_char as
                                    *const libc::c_void, 5 as libc::c_int);
                SDLNet_TCP_Send(rs_socket,
                                &mut game as *mut GAMESTRUCT as
                                    *const libc::c_void,
                                ::std::mem::size_of::<GAMESTRUCT>() as
                                    libc::c_ulong as libc::c_int);
            }
            0 => { SDLNet_TCP_Close(rs_socket); rs_socket = 0 as TCPsocket }
            _ => { }
        }
        registered = state
    };
}
// ////////////////////////////////////////////////////////////////////////
// Host a game with a given name and player name. & 4 user game flags
#[no_mangle]
pub unsafe extern "C" fn NETallowJoining() {
    let mut i: libc::c_uint = 0; // always 1 on normal server
    let mut numgames: UDWORD = 1 as libc::c_int as UDWORD;
    let mut buffer: [libc::c_char; 5] = [0; 5];
    if allow_joining == 0 as libc::c_int { return }
    NETregisterServer(1 as libc::c_int);
    if tmp_socket_set.is_null() {
        tmp_socket_set =
            SDLNet_AllocSocketSet(16 as libc::c_int + 1 as libc::c_int);
        if tmp_socket_set.is_null() {
            debug(LOG_ERROR,
                  b"Couldn\'t create socket set: %s\n\x00" as *const u8 as
                      *const libc::c_char, SDL_GetError());
            return
        }
        SDLNet_AddSocket(tmp_socket_set, tcp_socket as SDLNet_GenericSocket);
    }
    if SDLNet_CheckSockets(tmp_socket_set, 0 as libc::c_int as Uint32) >
           0 as libc::c_int {
        if !tcp_socket.is_null() &&
               (*(tcp_socket as SDLNet_GenericSocket)).ready != 0 {
            i = 0 as libc::c_int as libc::c_uint;
            while i < 16 as libc::c_int as libc::c_uint {
                if tmp_socket[i as usize].is_null() { break ; }
                i = i.wrapping_add(1)
            }
            tmp_socket[i as usize] = SDLNet_TCP_Accept(tcp_socket);
            SDLNet_AddSocket(tmp_socket_set,
                             tmp_socket[i as usize] as SDLNet_GenericSocket);
            if SDLNet_CheckSockets(tmp_socket_set,
                                   1000 as libc::c_int as Uint32) >
                   0 as libc::c_int &&
                   (!tmp_socket.as_mut_ptr().is_null() &&
                        (*(tmp_socket.as_mut_ptr() as
                               SDLNet_GenericSocket)).ready != 0) &&
                   SDLNet_TCP_Recv(tmp_socket[i as usize],
                                   buffer.as_mut_ptr() as *mut libc::c_void,
                                   5 as libc::c_int) != 0 {
                if strcmp(buffer.as_mut_ptr(),
                          b"list\x00" as *const u8 as *const libc::c_char) ==
                       0 as libc::c_int {
                    SDLNet_TCP_Send(tmp_socket[i as usize],
                                    &mut numgames as *mut UDWORD as
                                        *const libc::c_void,
                                    ::std::mem::size_of::<UDWORD>() as
                                        libc::c_ulong as libc::c_int);
                    SDLNet_TCP_Send(tmp_socket[i as usize],
                                    &mut game as *mut GAMESTRUCT as
                                        *const libc::c_void,
                                    ::std::mem::size_of::<GAMESTRUCT>() as
                                        libc::c_ulong as libc::c_int);
                } else if strcmp(buffer.as_mut_ptr(),
                                 b"join\x00" as *const u8 as
                                     *const libc::c_char) == 0 as libc::c_int
                 {
                    SDLNet_TCP_Send(tmp_socket[i as usize],
                                    &mut game as *mut GAMESTRUCT as
                                        *const libc::c_void,
                                    ::std::mem::size_of::<GAMESTRUCT>() as
                                        libc::c_ulong as libc::c_int);
                }
            } else { return }
        }
        i = 0 as libc::c_int as libc::c_uint;
        while i < 16 as libc::c_int as libc::c_uint {
            if !tmp_socket[i as usize].is_null() &&
                   (!tmp_socket[i as usize].is_null() &&
                        (*(tmp_socket[i as usize] as
                               SDLNet_GenericSocket)).ready != 0) as
                       libc::c_int > 0 as libc::c_int {
                let mut size: libc::c_int =
                    SDLNet_TCP_Recv(tmp_socket[i as usize],
                                    &mut message as *mut NETMSG as
                                        *mut libc::c_void,
                                    ::std::mem::size_of::<NETMSG>() as
                                        libc::c_ulong as libc::c_int);
                if size <= 0 as libc::c_int {
                    // socket probably disconnected.
                    SDLNet_DelSocket(tmp_socket_set,
                                     tmp_socket[i as usize] as
                                         SDLNet_GenericSocket);
                    SDLNet_TCP_Close(tmp_socket[i as usize]);
                    tmp_socket[i as usize] = 0 as TCPsocket
                } else if message.type_0 as libc::c_int == 90 as libc::c_int {
                    let mut j: libc::c_int = 0;
                    let mut name: *mut libc::c_char =
                        message.body.as_mut_ptr();
                    let mut dpid: libc::c_int =
                        NET_CreatePlayer(name,
                                         0 as libc::c_int as libc::c_uint);
                    let mut message_dpid: *mut libc::c_uint =
                        message.body.as_mut_ptr() as *mut libc::c_uint;
                    SDLNet_DelSocket(tmp_socket_set,
                                     tmp_socket[i as usize] as
                                         SDLNet_GenericSocket);
                    NET_initBufferedSocket(connected_bsocket[dpid as usize],
                                           tmp_socket[i as usize]);
                    SDLNet_AddSocket(socket_set,
                                     (*connected_bsocket[dpid as
                                                             usize]).socket as
                                         SDLNet_GenericSocket);
                    tmp_socket[i as usize] = 0 as TCPsocket;
                    //printf("Creating player %i\n", dpid);
                    game.desc.dwCurrentPlayers += 1;
                    message.type_0 = 91 as libc::c_int as libc::c_uchar;
                    NETsetMessageSize(&mut message,
                                      4 as libc::c_int as libc::c_uint);
                    *message_dpid = dpid as libc::c_uint;
                    NETsend(&mut message, dpid, 1 as libc::c_int);
                    MultiPlayerJoin(dpid);
                    message.type_0 = 94 as libc::c_int as libc::c_uchar;
                    NETsetMessageSize(&mut message,
                                      4 as libc::c_int as libc::c_uint);
                    // Send info about players to newcomer.
                    j = 0 as libc::c_int;
                    while j < 8 as libc::c_int {
                        if players[j as usize].allocated != 0 &&
                               dpid as libc::c_uint != players[j as usize].id
                           {
                            *message_dpid = players[j as usize].id;
                            NETsend(&mut message, dpid, 1 as libc::c_int);
                        }
                        j += 1
                    }
                    // Send info about newcomer to all players.
                    *message_dpid = dpid as libc::c_uint;
                    NETbcast(&mut message, 1 as libc::c_int);
                    j = 0 as libc::c_int;
                    while j < 8 as libc::c_int {
                        NETBroadcastPlayerInfo(j);
                        j += 1
                    }
                }
            }
            i = i.wrapping_add(1)
        }
    };
}
// join game given with playername
#[no_mangle]
pub unsafe extern "C" fn NEThostGame(mut SessionName: LPSTR,
                                     mut PlayerName: LPSTR, mut one: DWORD,
                                     mut two: DWORD, mut three: DWORD,
                                     mut four: DWORD, mut plyrs: UDWORD)
 -> BOOL 
 // # of players.
 {
    let mut ip: IPaddress = IPaddress{host: 0, port: 0,};
    let mut i: libc::c_uint = 0;
    debug(LOG_NET, b"NEThostGame\n\x00" as *const u8 as *const libc::c_char);
    if NetPlay.bComms == 0 {
        NetPlay.dpidPlayer = 1 as libc::c_int;
        NetPlay.bHost = 1 as libc::c_int;
        return 1 as libc::c_int
    }
    if SDLNet_ResolveHost(&mut ip, 0 as *const libc::c_char,
                          9999 as libc::c_int as Uint16) ==
           -(1 as libc::c_int) {
        debug(LOG_ERROR,
              b"NEThostGame: couldn\'t resolve master self: %s\n\x00" as
                  *const u8 as *const libc::c_char, SDL_GetError());
        return 0 as libc::c_int
    }
    if tcp_socket.is_null() { tcp_socket = SDLNet_TCP_Open(&mut ip) }
    if tcp_socket.is_null() {
        printf(b"SDLNet_TCP_Open: %s\n\x00" as *const u8 as
                   *const libc::c_char, SDL_GetError());
        return 0 as libc::c_int
    }
    if socket_set.is_null() {
        socket_set = SDLNet_AllocSocketSet(8 as libc::c_int)
    }
    if socket_set.is_null() {
        debug(LOG_ERROR,
              b"Couldn\'t create socket set: %s\n\x00" as *const u8 as
                  *const libc::c_char, SDL_GetError());
        return 0 as libc::c_int
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < 8 as libc::c_int as libc::c_uint {
        connected_bsocket[i as usize] = NET_createBufferedSocket();
        i = i.wrapping_add(1)
    }
    is_server = 1 as libc::c_int;
    strcpy(game.name.as_mut_ptr(), SessionName as *const libc::c_char);
    memset(&mut game.desc as *mut SESSIONDESC as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<SESSIONDESC>() as libc::c_ulong);
    game.desc.dwSize =
        ::std::mem::size_of::<SESSIONDESC>() as libc::c_ulong as DWORD;
    //game.desc.guidApplication = GAME_GUID;
    strcpy(game.desc.host.as_mut_ptr(),
           b"\x00" as *const u8 as *const libc::c_char);
    game.desc.dwCurrentPlayers = 1 as libc::c_int;
    game.desc.dwMaxPlayers = plyrs as DWORD;
    game.desc.dwFlags = 0 as libc::c_int;
    game.desc.dwUser1 = one;
    game.desc.dwUser2 = two;
    game.desc.dwUser3 = three;
    game.desc.dwUser4 = four;
    NET_InitPlayers();
    NetPlay.dpidPlayer =
        NET_CreatePlayer(PlayerName as *const libc::c_char,
                         1 as libc::c_int as libc::c_uint);
    NetPlay.bHost = 1 as libc::c_int;
    NetPlay.bSpectator = 0 as libc::c_int;
    MultiPlayerJoin(NetPlay.dpidPlayer);
    allow_joining = 1 as libc::c_int;
    NETregisterServer(0 as libc::c_int);
    return 1 as libc::c_int;
}
// set game flag(1-4) to value.		
// ////////////////////////////////////////////////////////////////////////
// Stop the dplay interface from accepting more players.
#[no_mangle]
pub unsafe extern "C" fn NEThaltJoining() -> BOOL {
    debug(LOG_NET,
          b"NEThaltJoining\n\x00" as *const u8 as *const libc::c_char);
    allow_joining = 0 as libc::c_int;
    return 1 as libc::c_int;
}
// stop new players joining this game
// ////////////////////////////////////////////////////////////////////////
// find games on open connection, option to do this asynchronously
// since it can sometimes take a while.
#[no_mangle]
pub unsafe extern "C" fn NETfindGame(mut async_0: BOOL) -> BOOL 
 // / may (not) want to use async here...
 {
    static mut gamecount: UDWORD = 0 as libc::c_int as UDWORD;
    static mut gamesavailable: UDWORD = 0;
    let mut ip: IPaddress = IPaddress{host: 0, port: 0,};
    let mut buffer: [libc::c_char; 224] = [0; 224];
    let mut tmpgame: *mut GAMESTRUCT = buffer.as_mut_ptr() as *mut GAMESTRUCT;
    debug(LOG_NET, b"NETfindGame\n\x00" as *const u8 as *const libc::c_char);
    gamecount = 0 as libc::c_int as UDWORD;
    NetPlay.games[0 as libc::c_int as usize].desc.dwSize = 0 as libc::c_int;
    NetPlay.games[0 as libc::c_int as usize].desc.dwCurrentPlayers =
        0 as libc::c_int;
    NetPlay.games[0 as libc::c_int as usize].desc.dwMaxPlayers =
        0 as libc::c_int;
    if NetPlay.bComms == 0 {
        NetPlay.dpidPlayer = 1 as libc::c_int;
        NetPlay.bHost = 1 as libc::c_int;
        return 1 as libc::c_int
    }
    if SDLNet_ResolveHost(&mut ip, hostname,
                          (if hostname == master_server {
                               9998 as libc::c_int
                           } else { 9999 as libc::c_int }) as Uint16) ==
           -(1 as libc::c_int) {
        debug(LOG_ERROR,
              b"NETfindGame: couldn\'t resolve hostname (%s): %s\n\x00" as
                  *const u8 as *const libc::c_char, hostname, SDL_GetError());
        return 0 as libc::c_int
    }
    if !tcp_socket.is_null() { SDLNet_TCP_Close(tcp_socket); }
    tcp_socket = SDLNet_TCP_Open(&mut ip);
    if tcp_socket.is_null() {
        debug(LOG_ERROR,
              b"SDLNet_TCP_Open: %s\n\x00" as *const u8 as
                  *const libc::c_char, SDL_GetError());
        return 0 as libc::c_int
    }
    socket_set = SDLNet_AllocSocketSet(1 as libc::c_int);
    if socket_set.is_null() {
        debug(LOG_ERROR,
              b"Couldn\'t create socket set: %s\n\x00" as *const u8 as
                  *const libc::c_char, SDL_GetError());
        return 0 as libc::c_int
    }
    SDLNet_AddSocket(socket_set, tcp_socket as SDLNet_GenericSocket);
    SDLNet_TCP_Send(tcp_socket,
                    b"list\x00" as *const u8 as *const libc::c_char as
                        *const libc::c_void, 5 as libc::c_int);
    if SDLNet_CheckSockets(socket_set, 1000 as libc::c_int as Uint32) >
           0 as libc::c_int &&
           (!tcp_socket.is_null() &&
                (*(tcp_socket as SDLNet_GenericSocket)).ready != 0) &&
           SDLNet_TCP_Recv(tcp_socket,
                           buffer.as_mut_ptr() as *mut libc::c_void,
                           ::std::mem::size_of::<libc::c_int>() as
                               libc::c_ulong as libc::c_int) != 0 {
        gamesavailable = buffer[0 as libc::c_int as usize] as UDWORD
    }
    debug(LOG_NET,
          b"receiving info of %d game(s)\n\x00" as *const u8 as
              *const libc::c_char, gamesavailable);
    loop  {
        if SDLNet_CheckSockets(socket_set, 1000 as libc::c_int as Uint32) >
               0 as libc::c_int &&
               (!tcp_socket.is_null() &&
                    (*(tcp_socket as SDLNet_GenericSocket)).ready != 0) &&
               SDLNet_TCP_Recv(tcp_socket,
                               buffer.as_mut_ptr() as *mut libc::c_void,
                               ::std::mem::size_of::<GAMESTRUCT>() as
                                   libc::c_ulong as libc::c_int) as
                   libc::c_uint ==
                   ::std::mem::size_of::<GAMESTRUCT>() as libc::c_ulong &&
               (*tmpgame).desc.dwSize as libc::c_uint ==
                   ::std::mem::size_of::<SESSIONDESC>() as libc::c_ulong {
            strcpy(NetPlay.games[gamecount as usize].name.as_mut_ptr(),
                   (*tmpgame).name.as_mut_ptr());
            NetPlay.games[gamecount as usize].desc.dwSize =
                (*tmpgame).desc.dwSize;
            NetPlay.games[gamecount as usize].desc.dwCurrentPlayers =
                (*tmpgame).desc.dwCurrentPlayers;
            NetPlay.games[gamecount as usize].desc.dwMaxPlayers =
                (*tmpgame).desc.dwMaxPlayers;
            if (*tmpgame).desc.host[0 as libc::c_int as usize] as libc::c_int
                   == '\u{0}' as i32 {
                let mut address: *mut libc::c_uchar =
                    &mut ip.host as *mut Uint32 as *mut libc::c_uchar;
                sprintf(NetPlay.games[gamecount as
                                          usize].desc.host.as_mut_ptr(),
                        b"%i.%i.%i.%i\x00" as *const u8 as
                            *const libc::c_char,
                        *address.offset(0 as libc::c_int as isize) as
                            libc::c_int,
                        *address.offset(1 as libc::c_int as isize) as
                            libc::c_int,
                        *address.offset(2 as libc::c_int as isize) as
                            libc::c_int,
                        *address.offset(3 as libc::c_int as isize) as
                            libc::c_int);
            } else {
                strcpy(NetPlay.games[gamecount as
                                         usize].desc.host.as_mut_ptr(),
                       (*tmpgame).desc.host.as_mut_ptr());
            }
            //printf("Received info for host %s\n", NetPlay.games[gamecount].desc.host);
            gamecount = gamecount.wrapping_add(1)
        }
        if !(gamecount < gamesavailable) { break ; }
    }
    return 1 as libc::c_int;
}
// find games being played(uses GAME_GUID);
// ////////////////////////////////////////////////////////////////////////
// ////////////////////////////////////////////////////////////////////////
// Functions used to setup and join games.
#[no_mangle]
pub unsafe extern "C" fn NETjoinGame(mut gameNumber: UDWORD,
                                     mut playername: LPSTR) -> BOOL {
    let mut name: *mut libc::c_char =
        0 as *mut libc::c_char; // just to be sure :)
    let mut ip: IPaddress = IPaddress{host: 0, port: 0,};
    let mut buffer: [libc::c_char; 224] = [0; 224];
    let mut tmpgame: *mut GAMESTRUCT = buffer.as_mut_ptr() as *mut GAMESTRUCT;
    debug(LOG_NET,
          b"NETjoinGame gameNumber=%d\n\x00" as *const u8 as
              *const libc::c_char, gameNumber);
    NETclose();
    if hostname != master_server { free(hostname as *mut libc::c_void); }
    hostname =
        strdup(NetPlay.games[gameNumber as usize].desc.host.as_mut_ptr());
    if SDLNet_ResolveHost(&mut ip, hostname, 9999 as libc::c_int as Uint16) ==
           -(1 as libc::c_int) {
        debug(LOG_ERROR,
              b"NETjoinGame: couldn\'t resolve hostname (%s): %s\n\x00" as
                  *const u8 as *const libc::c_char, hostname, SDL_GetError());
        return 0 as libc::c_int
    }
    if !tcp_socket.is_null() { SDLNet_TCP_Close(tcp_socket); }
    tcp_socket = SDLNet_TCP_Open(&mut ip);
    if tcp_socket.is_null() {
        debug(LOG_ERROR,
              b"SDLNet_TCP_Open: %s\n\x00" as *const u8 as
                  *const libc::c_char, SDL_GetError());
        return 0 as libc::c_int
    }
    socket_set = SDLNet_AllocSocketSet(1 as libc::c_int);
    if socket_set.is_null() {
        debug(LOG_ERROR,
              b"Couldn\'t create socket set: %s\n\x00" as *const u8 as
                  *const libc::c_char, SDL_GetError());
        return 0 as libc::c_int
    }
    SDLNet_AddSocket(socket_set, tcp_socket as SDLNet_GenericSocket);
    SDLNet_TCP_Send(tcp_socket,
                    b"join\x00" as *const u8 as *const libc::c_char as
                        *const libc::c_void, 5 as libc::c_int);
    if SDLNet_CheckSockets(socket_set, 1000 as libc::c_int as Uint32) >
           0 as libc::c_int &&
           (!tcp_socket.is_null() &&
                (*(tcp_socket as SDLNet_GenericSocket)).ready != 0) &&
           SDLNet_TCP_Recv(tcp_socket,
                           buffer.as_mut_ptr() as *mut libc::c_void,
                           (::std::mem::size_of::<GAMESTRUCT>() as
                                libc::c_ulong).wrapping_mul(2 as libc::c_int
                                                                as
                                                                libc::c_uint)
                               as libc::c_int) as libc::c_uint ==
               ::std::mem::size_of::<GAMESTRUCT>() as libc::c_ulong &&
           (*tmpgame).desc.dwSize as libc::c_uint ==
               ::std::mem::size_of::<SESSIONDESC>() as libc::c_ulong {
        strcpy(NetPlay.games[gameNumber as usize].name.as_mut_ptr(),
               (*tmpgame).name.as_mut_ptr());
        NetPlay.games[gameNumber as usize].desc.dwSize =
            (*tmpgame).desc.dwSize;
        NetPlay.games[gameNumber as usize].desc.dwCurrentPlayers =
            (*tmpgame).desc.dwCurrentPlayers;
        NetPlay.games[gameNumber as usize].desc.dwMaxPlayers =
            (*tmpgame).desc.dwMaxPlayers;
        strcpy(NetPlay.games[gameNumber as usize].desc.host.as_mut_ptr(),
               (*tmpgame).desc.host.as_mut_ptr());
        if (*tmpgame).desc.host[0 as libc::c_int as usize] as libc::c_int ==
               '\u{0}' as i32 {
            let mut address: *mut libc::c_uchar =
                &mut ip.host as *mut Uint32 as *mut libc::c_uchar;
            sprintf(NetPlay.games[gameNumber as usize].desc.host.as_mut_ptr(),
                    b"%i.%i.%i.%i\x00" as *const u8 as *const libc::c_char,
                    *address.offset(0 as libc::c_int as isize) as libc::c_int,
                    *address.offset(1 as libc::c_int as isize) as libc::c_int,
                    *address.offset(2 as libc::c_int as isize) as libc::c_int,
                    *address.offset(3 as libc::c_int as isize) as
                        libc::c_int);
        } else {
            strcpy(NetPlay.games[gameNumber as usize].desc.host.as_mut_ptr(),
                   (*tmpgame).desc.host.as_mut_ptr());
        }
        //printf("JoinGame: Received info for host %s\n", NetPlay.games[gameNumber].desc.host);
    }
    bsocket = NET_createBufferedSocket();
    NET_initBufferedSocket(bsocket, tcp_socket);
    message.type_0 = 90 as libc::c_int as libc::c_uchar;
    NETsetMessageSize(&mut message, 64 as libc::c_int as libc::c_uint);
    name = message.body.as_mut_ptr();
    strcpy(name, playername as *const libc::c_char);
    NETsend(&mut message, 1 as libc::c_int, 1 as libc::c_int);
    loop  {
        NETrecv(&mut message);
        if message.type_0 as libc::c_int == 91 as libc::c_int {
            let mut message_dpid: *mut libc::c_uint =
                message.body.as_mut_ptr() as *mut libc::c_uint;
            NetPlay.dpidPlayer = *message_dpid as DPID;
            //printf("I'm player %i\n", NetPlay.dpidPlayer);
            NetPlay.bHost = 0 as libc::c_int;
            NetPlay.bSpectator = 0 as libc::c_int;
            players[NetPlay.dpidPlayer as usize].allocated = 1 as libc::c_int;
            players[NetPlay.dpidPlayer as usize].id =
                NetPlay.dpidPlayer as libc::c_uint;
            strcpy(players[NetPlay.dpidPlayer as usize].name.as_mut_ptr(),
                   playername as *const libc::c_char);
            players[NetPlay.dpidPlayer as usize].flags =
                0 as libc::c_int as libc::c_uint;
            return 1 as libc::c_int
        }
    };
}
