use ::libc;
extern "C" {
    #[no_mangle]
    fn PHYSFS_openRead(filename: *const libc::c_char) -> *mut PHYSFS_File;
    #[no_mangle]
    fn PHYSFS_close(handle: *mut PHYSFS_File) -> libc::c_int;
    #[no_mangle]
    fn PHYSFS_read(handle: *mut PHYSFS_File, buffer: *mut libc::c_void,
                   objSize: PHYSFS_uint32, objCount: PHYSFS_uint32)
     -> PHYSFS_sint64;
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
    fn abort() -> !;
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
    // ////////////////////////////////////////////////////////////////////////
// variables
    #[no_mangle]
    static mut NetPlay: NETPLAY;
    #[no_mangle]
    fn NETlogEntry(str: *mut CHAR, a: UDWORD, b: UDWORD) -> BOOL;
}
pub type PHYSFS_uint32 = libc::c_uint;
pub type PHYSFS_sint64 = libc::c_longlong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PHYSFS_File {
    pub opaque: *mut libc::c_void,
}
pub type UBYTE = libc::c_uchar;
pub type STRING = libc::c_char;
pub type UDWORD = libc::c_uint;
pub type BOOL = libc::c_int;
pub type CHAR = libc::c_char;
pub type UCHAR = libc::c_uchar;
pub type DWORD = libc::c_int;
pub type LONG = libc::c_long;
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
// bytes done per encrypt step.
// ////////////////////////////////////////////////////////////////////////
// Prototypes
// ////////////////////////////////////////////////////////////////////////
// make a hash value from an exe name.
#[no_mangle]
pub unsafe extern "C" fn NEThashFile(mut pFileName: *mut libc::c_char)
 -> UDWORD {
    let mut hashval: UDWORD = 0; // must be multiple of 4 bytes.
    let mut c: UDWORD = 0;
    let mut val: *mut UDWORD = 0 as *mut UDWORD;
    let mut pFileHandle: *mut PHYSFS_File = 0 as *mut PHYSFS_File;
    let mut fileName: [STRING; 255] = [0; 255];
    let mut inBuff: [UBYTE; 2048] = [0; 2048];
    strcpy(fileName.as_mut_ptr(), pFileName);
    hashval = 0 as libc::c_int as UDWORD;
    debug(LOG_WZ,
          b"NEThashFile: Hashing File\n\x00" as *const u8 as
              *const libc::c_char);
    // open the file.
    pFileHandle = PHYSFS_openRead(fileName.as_mut_ptr()); // check file exists
    if pFileHandle.is_null() {
        debug(LOG_WZ,
              b"NEThashFile: Failed\n\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int as UDWORD
        // failed
    }
    // multibyte/buff version
    while PHYSFS_read(pFileHandle,
                      &mut inBuff as *mut [UBYTE; 2048] as *mut libc::c_void,
                      ::std::mem::size_of::<[UBYTE; 2048]>() as libc::c_ulong,
                      1 as libc::c_int as PHYSFS_uint32) ==
              1 as libc::c_int as libc::c_longlong {
        // get number of droids in force
        c = 0 as libc::c_int as UDWORD;
        while c < 2048 as libc::c_int as libc::c_uint {
            val =
                &mut *inBuff.as_mut_ptr().offset(c as isize) as *mut UBYTE as
                    *mut UDWORD;
            hashval = hashval ^ *val;
            c =
                (c as
                     libc::c_uint).wrapping_add(4 as libc::c_int as
                                                    libc::c_uint) as UDWORD as
                    UDWORD
        }
    }
    // 1 byte version
    PHYSFS_close(pFileHandle);
    debug(LOG_WZ,
          b"NEThashFile: Hash Complete :   *****  %u  ***** is todays magic number.\n\x00"
              as *const u8 as *const libc::c_char, hashval);
    //	DBERROR(("%d",hashval));
    return hashval;
}
// ////////////////////////////////////////////////////////////////////////
// return a hash from a data buffer.
#[no_mangle]
pub unsafe extern "C" fn NEThashBuffer(mut pData: *mut libc::c_char,
                                       mut size: UDWORD) -> UDWORD {
    let mut hashval: UDWORD = 0;
    let mut val: *mut UDWORD = 0 as *mut UDWORD;
    let mut pt: UDWORD = 0;
    hashval = 0 as libc::c_int as UDWORD;
    pt = 0 as libc::c_int as UDWORD;
    while pt < size {
        val = pData.offset(pt as isize) as *mut UDWORD;
        hashval = hashval ^ *val;
        pt =
            (pt as
                 libc::c_uint).wrapping_add(4 as libc::c_int as libc::c_uint)
                as UDWORD as UDWORD
    }
    return hashval;
}
// ////////////////////////////////////////////////////////////////////////
// return a ubyte hash from a UDWORD value.
#[no_mangle]
pub unsafe extern "C" fn NEThashVal(mut value: UDWORD) -> UCHAR {
    return (value ^
                13416564 as libc::c_int as
                    libc::c_uint).wrapping_rem(246 as libc::c_int as
                                                   libc::c_uint) as UCHAR;
}
// ////////////////////////////////////////////////////////////////////////
// set the key for the encrypter.
#[no_mangle]
pub unsafe extern "C" fn NETsetKey(mut c1: UDWORD, mut c2: UDWORD,
                                   mut c3: UDWORD, mut c4: UDWORD) -> BOOL {
    if c1 != 0 { NetPlay.cryptKey[0 as libc::c_int as usize] = c1 }
    if c2 != 0 { NetPlay.cryptKey[1 as libc::c_int as usize] = c2 }
    if c3 != 0 { NetPlay.cryptKey[2 as libc::c_int as usize] = c3 }
    if c4 != 0 { NetPlay.cryptKey[3 as libc::c_int as usize] = c4 }
    return 1 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////
// encrypt a byte sequence of nibblelength
unsafe extern "C" fn mangle(mut v: *mut libc::c_long,
                            mut w: *mut libc::c_long) -> BOOL {
    let mut y: libc::c_ulong =
        *v.offset(0 as libc::c_int as isize) as libc::c_ulong;
    let mut z: libc::c_ulong =
        *v.offset(1 as libc::c_int as isize) as libc::c_ulong;
    let mut sum: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut delta: libc::c_ulong =
        0x9e3779b9 as libc::c_uint as libc::c_ulong;
    let mut n: libc::c_ulong = 16 as libc::c_int as libc::c_ulong;
    loop  {
        let fresh0 = n;
        n = n.wrapping_sub(1);
        if !(fresh0 > 0 as libc::c_int as libc::c_ulong) { break ; }
        sum = sum.wrapping_add(delta);
        y =
            y.wrapping_add((z <<
                                4 as
                                    libc::c_int).wrapping_add(NetPlay.cryptKey[0
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   usize]
                                                                  as
                                                                  libc::c_ulong)
                               ^ z.wrapping_add(sum) ^
                               (z >>
                                    5 as
                                        libc::c_int).wrapping_add(NetPlay.cryptKey[1
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       usize]
                                                                      as
                                                                      libc::c_ulong));
        z =
            z.wrapping_add((y <<
                                4 as
                                    libc::c_int).wrapping_add(NetPlay.cryptKey[2
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   usize]
                                                                  as
                                                                  libc::c_ulong)
                               ^ y.wrapping_add(sum) ^
                               (y >>
                                    5 as
                                        libc::c_int).wrapping_add(NetPlay.cryptKey[3
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       usize]
                                                                      as
                                                                      libc::c_ulong))
    }
    *w.offset(0 as libc::c_int as isize) = y as libc::c_long;
    *w.offset(1 as libc::c_int as isize) = z as libc::c_long;
    return 1 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////
// decrypt a byte sequence of nibblelength
unsafe extern "C" fn unmangle(mut v: *mut libc::c_long,
                              mut w: *mut libc::c_long) -> BOOL {
    let mut y: libc::c_ulong =
        *v.offset(0 as libc::c_int as isize) as
            libc::c_ulong; /* (generally sum =delta*n )*/
    let mut z: libc::c_ulong =
        *v.offset(1 as libc::c_int as isize) as libc::c_ulong;
    let mut sum: libc::c_ulong = 0;
    let mut delta: libc::c_ulong =
        0x9e3779b9 as libc::c_uint as libc::c_ulong;
    let mut n: libc::c_ulong = 16 as libc::c_int as libc::c_ulong;
    sum = delta.wrapping_mul(n);
    loop  {
        let fresh1 = n;
        n = n.wrapping_sub(1);
        if !(fresh1 > 0 as libc::c_int as libc::c_ulong) { break ; }
        z =
            z.wrapping_sub((y <<
                                4 as
                                    libc::c_int).wrapping_add(NetPlay.cryptKey[2
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   usize]
                                                                  as
                                                                  libc::c_ulong)
                               ^ y.wrapping_add(sum) ^
                               (y >>
                                    5 as
                                        libc::c_int).wrapping_add(NetPlay.cryptKey[3
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       usize]
                                                                      as
                                                                      libc::c_ulong));
        y =
            y.wrapping_sub((z <<
                                4 as
                                    libc::c_int).wrapping_add(NetPlay.cryptKey[0
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   usize]
                                                                  as
                                                                  libc::c_ulong)
                               ^ z.wrapping_add(sum) ^
                               (z >>
                                    5 as
                                        libc::c_int).wrapping_add(NetPlay.cryptKey[1
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       usize]
                                                                      as
                                                                      libc::c_ulong));
        sum = sum.wrapping_sub(delta)
    }
    *w.offset(0 as libc::c_int as isize) = y as libc::c_long;
    *w.offset(1 as libc::c_int as isize) = z as libc::c_long;
    return 1 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////
// encrypt a netplay packet
#[no_mangle]
pub unsafe extern "C" fn NETmanglePacket(mut msg: *mut NETMSG)
 -> *mut NETMSG {
    let mut result: NETMSG =
        NETMSG{size: 0,
               paddedBytes: 0,
               type_0: 0,
               destination: 0,
               body: [0; 8000],};
    let mut pos: UDWORD = 0 as libc::c_int as UDWORD;
    if (*msg).size as libc::c_int > 8000 as libc::c_int - 8 as libc::c_int {
        debug(LOG_ERROR,
              b"NETmanglePacket: can\'t encrypt huge packets. returning unencrypted packet\x00"
                  as *const u8 as *const libc::c_char);
        abort();
    }
    (*msg).paddedBytes = 0 as libc::c_int as libc::c_uchar;
    while (*msg).size as libc::c_int % 8 as libc::c_int != 0 as libc::c_int {
        //need to pad out msg.
        (*msg).body[(*msg).size as usize] = 0 as libc::c_int as libc::c_char;
        (*msg).size = (*msg).size.wrapping_add(1);
        (*msg).paddedBytes = (*msg).paddedBytes.wrapping_add(1)
    }
    result.type_0 =
        ((*msg).type_0 as libc::c_int + 100 as libc::c_int) as libc::c_uchar;
    result.size = (*msg).size;
    result.paddedBytes = (*msg).paddedBytes;
    result.destination = (*msg).destination;
    while (*msg).size != 0 {
        mangle(&mut *(*msg).body.as_mut_ptr().offset(pos as isize) as
                   *mut libc::c_char as *mut libc::c_long,
               &mut *result.body.as_mut_ptr().offset(pos as isize) as
                   *mut libc::c_char as *mut libc::c_long);
        pos =
            (pos as
                 libc::c_uint).wrapping_add(8 as libc::c_int as libc::c_uint)
                as UDWORD as UDWORD;
        (*msg).size =
            ((*msg).size as libc::c_int - 8 as libc::c_int) as libc::c_ushort
    }
    memcpy(msg as *mut libc::c_void,
           &mut result as *mut NETMSG as *const libc::c_void,
           ::std::mem::size_of::<NETMSG>() as libc::c_ulong);
    return msg;
}
// ////////////////////////////////////////////////////////////////////////
// decrypt a netplay packet
// messages SHOULD be 8byte multiples, not required tho. will return padded out..
#[no_mangle]
pub unsafe extern "C" fn NETunmanglePacket(mut msg: *mut NETMSG) {
    let mut result: NETMSG =
        NETMSG{size: 0,
               paddedBytes: 0,
               type_0: 0,
               destination: 0,
               body: [0; 8000],};
    let mut pos: UDWORD = 0 as libc::c_int as UDWORD;
    if (*msg).size as libc::c_int % 8 as libc::c_int != 0 as libc::c_int {
        debug(LOG_ERROR,
              b"NETunmanglePacket: Incoming msg wrong length\x00" as *const u8
                  as *const libc::c_char);
        abort();
    }
    result.type_0 =
        ((*msg).type_0 as libc::c_int - 100 as libc::c_int) as libc::c_uchar;
    result.size = 0 as libc::c_int as libc::c_ushort;
    result.paddedBytes = (*msg).paddedBytes;
    while (*msg).size != 0 {
        unmangle(&mut *(*msg).body.as_mut_ptr().offset(pos as isize) as
                     *mut libc::c_char as *mut LONG,
                 &mut *result.body.as_mut_ptr().offset(pos as isize) as
                     *mut libc::c_char as *mut libc::c_long);
        pos =
            (pos as
                 libc::c_uint).wrapping_add(8 as libc::c_int as libc::c_uint)
                as UDWORD as UDWORD;
        (*msg).size =
            ((*msg).size as libc::c_int - 8 as libc::c_int) as libc::c_ushort;
        result.size =
            (result.size as libc::c_int + 8 as libc::c_int) as libc::c_ushort
    }
    result.size =
        (result.size as libc::c_int - (*msg).paddedBytes as libc::c_int) as
            libc::c_ushort;
    memcpy(msg as *mut libc::c_void,
           &mut result as *mut NETMSG as *const libc::c_void,
           ::std::mem::size_of::<NETMSG>() as libc::c_ulong);
}
// ////////////////////////////////////////////////////////////////////////
// encrypt any datastream.
#[no_mangle]
pub unsafe extern "C" fn NETmangleData(mut input: *mut libc::c_long,
                                       mut result: *mut libc::c_long,
                                       mut dataSize: UDWORD) -> BOOL {
    let mut offset: libc::c_long = 0;
    offset = 0 as libc::c_int as libc::c_long;
    if dataSize.wrapping_rem(8 as libc::c_int as libc::c_uint) !=
           0 as libc::c_int as libc::c_uint {
        //if message not multiple of 8 bytes,
        debug(LOG_ERROR,
              b"NETmangleData: msg not a multiple of 8 bytes\x00" as *const u8
                  as *const libc::c_char);
        abort();
    }
    //  /4's are long form. since nibblelength is in char form
    while offset !=
              dataSize.wrapping_div(4 as libc::c_int as libc::c_uint) as
                  libc::c_long {
        mangle(input.offset(offset as isize), result.offset(offset as isize));
        offset += (8 as libc::c_int / 4 as libc::c_int) as libc::c_long
    }
    return 1 as libc::c_int;
}
// encryption
// ////////////////////////////////////////////////////////////////////////
// decrypt any datastream.
#[no_mangle]
pub unsafe extern "C" fn NETunmangleData(mut input: *mut libc::c_long,
                                         mut result: *mut libc::c_long,
                                         mut dataSize: UDWORD) -> BOOL {
    let mut offset: libc::c_long = 0;
    memset(result as *mut libc::c_void, 0 as libc::c_int, dataSize);
    offset = 0 as libc::c_int as libc::c_long;
    if dataSize.wrapping_rem(8 as libc::c_int as libc::c_uint) !=
           0 as libc::c_int as libc::c_uint {
        //if message not multiple of 8 bytes,
        debug(LOG_ERROR,
              b"NETunmangleData: msg not a multiple of 8 bytes\x00" as
                  *const u8 as *const libc::c_char);
        abort();
    }
    //  /4's are long form. since nibblelength is in char form
    while offset !=
              dataSize.wrapping_div(4 as libc::c_int as libc::c_uint) as
                  libc::c_long {
        unmangle(input.offset(offset as isize),
                 result.offset(offset as isize));
        offset += (8 as libc::c_int / 4 as libc::c_int) as libc::c_long
    }
    return 1 as libc::c_int;
}
