use ::libc;
extern "C" {
    #[no_mangle]
    fn PHYSFS_getLastError() -> *const libc::c_char;
    #[no_mangle]
    fn PHYSFS_openRead(filename: *const libc::c_char) -> *mut PHYSFS_File;
    #[no_mangle]
    fn PHYSFS_close(handle: *mut PHYSFS_File) -> libc::c_int;
    #[no_mangle]
    fn PHYSFS_read(handle: *mut PHYSFS_File, buffer: *mut libc::c_void,
                   objSize: PHYSFS_uint32, objCount: PHYSFS_uint32)
     -> PHYSFS_sint64;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_uint;
    #[no_mangle]
    fn strncasecmp(_: *const libc::c_char, _: *const libc::c_char,
                   _: libc::c_uint) -> libc::c_int;
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
    fn alGenSources(n: ALsizei, sources: *mut ALuint);
    #[no_mangle]
    fn alDeleteSources(n: ALsizei, sources: *const ALuint);
    #[no_mangle]
    fn alSourcef(source: ALuint, param: ALenum, value: ALfloat);
    #[no_mangle]
    fn alSource3f(source: ALuint, param: ALenum, value1: ALfloat,
                  value2: ALfloat, value3: ALfloat);
    #[no_mangle]
    fn alSourcei(source: ALuint, param: ALenum, value: ALint);
    #[no_mangle]
    fn alGetSourcei(source: ALuint, param: ALenum, value: *mut ALint);
    #[no_mangle]
    fn alSourcePlay(source: ALuint);
    #[no_mangle]
    fn alSourceStop(source: ALuint);
    #[no_mangle]
    fn alSourceQueueBuffers(source: ALuint, nb: ALsizei,
                            buffers: *const ALuint);
    #[no_mangle]
    fn alSourceUnqueueBuffers(source: ALuint, nb: ALsizei,
                              buffers: *mut ALuint);
    #[no_mangle]
    fn alGenBuffers(n: ALsizei, buffers: *mut ALuint);
    #[no_mangle]
    fn alDeleteBuffers(n: ALsizei, buffers: *const ALuint);
    #[no_mangle]
    fn alBufferData(buffer: ALuint, format: ALenum, data: *const libc::c_void,
                    size: ALsizei, freq: ALsizei);
    #[no_mangle]
    fn ov_clear(vf: *mut OggVorbis_File) -> libc::c_int;
    #[no_mangle]
    fn ov_open_callbacks(datasource: *mut libc::c_void,
                         vf: *mut OggVorbis_File,
                         initial: *const libc::c_char, ibytes: libc::c_long,
                         callbacks: ov_callbacks) -> libc::c_int;
    #[no_mangle]
    fn ov_info(vf: *mut OggVorbis_File, link: libc::c_int)
     -> *mut vorbis_info;
    #[no_mangle]
    fn ov_read(vf: *mut OggVorbis_File, buffer: *mut libc::c_char,
               length: libc::c_int, bigendianp: libc::c_int,
               word: libc::c_int, sgned: libc::c_int,
               bitstream: *mut libc::c_int) -> libc::c_long;
    #[no_mangle]
    static mut openal_initialized: BOOL;
    #[no_mangle]
    fn PlayList_Init();
    #[no_mangle]
    fn PlayList_Quit();
    #[no_mangle]
    fn PlayList_Read(path: *const libc::c_char) -> libc::c_char;
    #[no_mangle]
    fn PlayList_SetTrack(t: libc::c_uint);
    #[no_mangle]
    fn PlayList_CurrentSong() -> *mut libc::c_char;
    #[no_mangle]
    fn PlayList_NextSong() -> *mut libc::c_char;
}
pub type PHYSFS_uint32 = libc::c_uint;
pub type PHYSFS_sint64 = libc::c_longlong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PHYSFS_File {
    pub opaque: *mut libc::c_void,
}
pub type __int64_t = libc::c_longlong;
pub type size_t = libc::c_uint;
pub type int64_t = __int64_t;
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
pub type ALint = libc::c_int;
pub type ALuint = libc::c_uint;
pub type ALsizei = libc::c_int;
pub type ALenum = libc::c_int;
pub type ALfloat = libc::c_float;
pub type ALvoid = ();
pub type ogg_int64_t = int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct oggpack_buffer {
    pub endbyte: libc::c_long,
    pub endbit: libc::c_int,
    pub buffer: *mut libc::c_uchar,
    pub ptr: *mut libc::c_uchar,
    pub storage: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ogg_stream_state {
    pub body_data: *mut libc::c_uchar,
    pub body_storage: libc::c_long,
    pub body_fill: libc::c_long,
    pub body_returned: libc::c_long,
    pub lacing_vals: *mut libc::c_int,
    pub granule_vals: *mut ogg_int64_t,
    pub lacing_storage: libc::c_long,
    pub lacing_fill: libc::c_long,
    pub lacing_packet: libc::c_long,
    pub lacing_returned: libc::c_long,
    pub header: [libc::c_uchar; 282],
    pub header_fill: libc::c_int,
    pub e_o_s: libc::c_int,
    pub b_o_s: libc::c_int,
    pub serialno: libc::c_long,
    pub pageno: libc::c_long,
    pub packetno: ogg_int64_t,
    pub granulepos: ogg_int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ogg_sync_state {
    pub data: *mut libc::c_uchar,
    pub storage: libc::c_int,
    pub fill: libc::c_int,
    pub returned: libc::c_int,
    pub unsynced: libc::c_int,
    pub headerbytes: libc::c_int,
    pub bodybytes: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vorbis_info {
    pub version: libc::c_int,
    pub channels: libc::c_int,
    pub rate: libc::c_long,
    pub bitrate_upper: libc::c_long,
    pub bitrate_nominal: libc::c_long,
    pub bitrate_lower: libc::c_long,
    pub bitrate_window: libc::c_long,
    pub codec_setup: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vorbis_dsp_state {
    pub analysisp: libc::c_int,
    pub vi: *mut vorbis_info,
    pub pcm: *mut *mut libc::c_float,
    pub pcmret: *mut *mut libc::c_float,
    pub pcm_storage: libc::c_int,
    pub pcm_current: libc::c_int,
    pub pcm_returned: libc::c_int,
    pub preextrapolate: libc::c_int,
    pub eofflag: libc::c_int,
    pub lW: libc::c_long,
    pub W: libc::c_long,
    pub nW: libc::c_long,
    pub centerW: libc::c_long,
    pub granulepos: ogg_int64_t,
    pub sequence: ogg_int64_t,
    pub glue_bits: ogg_int64_t,
    pub time_bits: ogg_int64_t,
    pub floor_bits: ogg_int64_t,
    pub res_bits: ogg_int64_t,
    pub backend_state: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vorbis_block {
    pub pcm: *mut *mut libc::c_float,
    pub opb: oggpack_buffer,
    pub lW: libc::c_long,
    pub W: libc::c_long,
    pub nW: libc::c_long,
    pub pcmend: libc::c_int,
    pub mode: libc::c_int,
    pub eofflag: libc::c_int,
    pub granulepos: ogg_int64_t,
    pub sequence: ogg_int64_t,
    pub vd: *mut vorbis_dsp_state,
    pub localstore: *mut libc::c_void,
    pub localtop: libc::c_long,
    pub localalloc: libc::c_long,
    pub totaluse: libc::c_long,
    pub reap: *mut alloc_chain,
    pub glue_bits: libc::c_long,
    pub time_bits: libc::c_long,
    pub floor_bits: libc::c_long,
    pub res_bits: libc::c_long,
    pub internal: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct alloc_chain {
    pub ptr: *mut libc::c_void,
    pub next: *mut alloc_chain,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vorbis_comment {
    pub user_comments: *mut *mut libc::c_char,
    pub comment_lengths: *mut libc::c_int,
    pub comments: libc::c_int,
    pub vendor: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ov_callbacks {
    pub read_func: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                               _: size_t, _: size_t,
                                               _: *mut libc::c_void)
                              -> size_t>,
    pub seek_func: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                               _: ogg_int64_t, _: libc::c_int)
                              -> libc::c_int>,
    pub close_func: Option<unsafe extern "C" fn(_: *mut libc::c_void)
                               -> libc::c_int>,
    pub tell_func: Option<unsafe extern "C" fn(_: *mut libc::c_void)
                              -> libc::c_long>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OggVorbis_File {
    pub datasource: *mut libc::c_void,
    pub seekable: libc::c_int,
    pub offset: ogg_int64_t,
    pub end: ogg_int64_t,
    pub oy: ogg_sync_state,
    pub links: libc::c_int,
    pub offsets: *mut ogg_int64_t,
    pub dataoffsets: *mut ogg_int64_t,
    pub serialnos: *mut libc::c_long,
    pub pcmlengths: *mut ogg_int64_t,
    pub vi: *mut vorbis_info,
    pub vc: *mut vorbis_comment,
    pub pcm_offset: ogg_int64_t,
    pub ready_state: libc::c_int,
    pub current_serialno: libc::c_long,
    pub current_link: libc::c_int,
    pub bittrack: libc::c_double,
    pub samptrack: libc::c_double,
    pub os: ogg_stream_state,
    pub vd: vorbis_dsp_state,
    pub vb: vorbis_block,
    pub callbacks: ov_callbacks,
}
pub type C2RustUnnamed = libc::c_uint;
pub const WZ_OGG: C2RustUnnamed = 2;
pub const WZ_MP3: C2RustUnnamed = 1;
pub const WZ_NONE: C2RustUnnamed = 0;
static mut music_initialized: BOOL = 0;
static mut music_file: *mut PHYSFS_File =
    0 as *const PHYSFS_File as *mut PHYSFS_File;
#[no_mangle]
pub static mut music_file_format: C2RustUnnamed = WZ_NONE;
static mut ogg_stream: OggVorbis_File =
    OggVorbis_File{datasource: 0 as *const libc::c_void as *mut libc::c_void,
                   seekable: 0,
                   offset: 0,
                   end: 0,
                   oy:
                       ogg_sync_state{data:
                                          0 as *const libc::c_uchar as
                                              *mut libc::c_uchar,
                                      storage: 0,
                                      fill: 0,
                                      returned: 0,
                                      unsynced: 0,
                                      headerbytes: 0,
                                      bodybytes: 0,},
                   links: 0,
                   offsets: 0 as *const ogg_int64_t as *mut ogg_int64_t,
                   dataoffsets: 0 as *const ogg_int64_t as *mut ogg_int64_t,
                   serialnos: 0 as *const libc::c_long as *mut libc::c_long,
                   pcmlengths: 0 as *const ogg_int64_t as *mut ogg_int64_t,
                   vi: 0 as *const vorbis_info as *mut vorbis_info,
                   vc: 0 as *const vorbis_comment as *mut vorbis_comment,
                   pcm_offset: 0,
                   ready_state: 0,
                   current_serialno: 0,
                   current_link: 0,
                   bittrack: 0.,
                   samptrack: 0.,
                   os:
                       ogg_stream_state{body_data:
                                            0 as *const libc::c_uchar as
                                                *mut libc::c_uchar,
                                        body_storage: 0,
                                        body_fill: 0,
                                        body_returned: 0,
                                        lacing_vals:
                                            0 as *const libc::c_int as
                                                *mut libc::c_int,
                                        granule_vals:
                                            0 as *const ogg_int64_t as
                                                *mut ogg_int64_t,
                                        lacing_storage: 0,
                                        lacing_fill: 0,
                                        lacing_packet: 0,
                                        lacing_returned: 0,
                                        header: [0; 282],
                                        header_fill: 0,
                                        e_o_s: 0,
                                        b_o_s: 0,
                                        serialno: 0,
                                        pageno: 0,
                                        packetno: 0,
                                        granulepos: 0,},
                   vd:
                       vorbis_dsp_state{analysisp: 0,
                                        vi:
                                            0 as *const vorbis_info as
                                                *mut vorbis_info,
                                        pcm:
                                            0 as *const *mut libc::c_float as
                                                *mut *mut libc::c_float,
                                        pcmret:
                                            0 as *const *mut libc::c_float as
                                                *mut *mut libc::c_float,
                                        pcm_storage: 0,
                                        pcm_current: 0,
                                        pcm_returned: 0,
                                        preextrapolate: 0,
                                        eofflag: 0,
                                        lW: 0,
                                        W: 0,
                                        nW: 0,
                                        centerW: 0,
                                        granulepos: 0,
                                        sequence: 0,
                                        glue_bits: 0,
                                        time_bits: 0,
                                        floor_bits: 0,
                                        res_bits: 0,
                                        backend_state:
                                            0 as *const libc::c_void as
                                                *mut libc::c_void,},
                   vb:
                       vorbis_block{pcm:
                                        0 as *const *mut libc::c_float as
                                            *mut *mut libc::c_float,
                                    opb:
                                        oggpack_buffer{endbyte: 0,
                                                       endbit: 0,
                                                       buffer:
                                                           0 as
                                                               *const libc::c_uchar
                                                               as
                                                               *mut libc::c_uchar,
                                                       ptr:
                                                           0 as
                                                               *const libc::c_uchar
                                                               as
                                                               *mut libc::c_uchar,
                                                       storage: 0,},
                                    lW: 0,
                                    W: 0,
                                    nW: 0,
                                    pcmend: 0,
                                    mode: 0,
                                    eofflag: 0,
                                    granulepos: 0,
                                    sequence: 0,
                                    vd:
                                        0 as *const vorbis_dsp_state as
                                            *mut vorbis_dsp_state,
                                    localstore:
                                        0 as *const libc::c_void as
                                            *mut libc::c_void,
                                    localtop: 0,
                                    localalloc: 0,
                                    totaluse: 0,
                                    reap:
                                        0 as *const alloc_chain as
                                            *mut alloc_chain,
                                    glue_bits: 0,
                                    time_bits: 0,
                                    floor_bits: 0,
                                    res_bits: 0,
                                    internal:
                                        0 as *const libc::c_void as
                                            *mut libc::c_void,},
                   callbacks:
                       ov_callbacks{read_func: None,
                                    seek_func: None,
                                    close_func: None,
                                    tell_func: None,},};
static mut ogg_info: *mut vorbis_info =
    0 as *const vorbis_info as *mut vorbis_info;
static mut wz_ogg_callbacks: ov_callbacks =
    unsafe {
        {
            let mut init =
                ov_callbacks{read_func:
                                 Some(wz_ogg_read as
                                          unsafe extern "C" fn(_:
                                                                   *mut libc::c_void,
                                                               _: size_t,
                                                               _: size_t,
                                                               _:
                                                                   *mut libc::c_void)
                                              -> size_t),
                             seek_func:
                                 Some(wz_ogg_seek as
                                          unsafe extern "C" fn(_:
                                                                   *mut libc::c_void,
                                                               _: ogg_int64_t,
                                                               _: libc::c_int)
                                              -> libc::c_int),
                             close_func:
                                 Some(wz_ogg_close as
                                          unsafe extern "C" fn(_:
                                                                   *mut libc::c_void)
                                              -> libc::c_int),
                             tell_func: None,};
            init
        }
    };
static mut music_volume: ALfloat = 0.5f64 as ALfloat;
static mut music_data: [libc::c_char; 16384] = [0; 16384];
static mut music_track: libc::c_uint = 0 as libc::c_int as libc::c_uint;
static mut music_buffers: [ALuint; 16] = [0; 16];
static mut music_source: ALuint = 0;
static mut music_format: ALenum = 0;
static mut music_rate: libc::c_uint = 0;
//*
//
// cdAudio Subclass procedure
unsafe extern "C" fn wz_ogg_read(mut ptr: *mut libc::c_void, mut size: size_t,
                                 mut nmemb: size_t,
                                 mut datasource: *mut libc::c_void)
 -> size_t {
    return PHYSFS_read(datasource as *mut PHYSFS_File, ptr, size, nmemb) as
               size_t;
}
unsafe extern "C" fn wz_ogg_seek(mut datasource: *mut libc::c_void,
                                 mut offset: ogg_int64_t,
                                 mut whence: libc::c_int) -> libc::c_int {
    return -(1 as libc::c_int);
}
unsafe extern "C" fn wz_ogg_close(mut datasource: *mut libc::c_void)
 -> libc::c_int {
    return PHYSFS_close(datasource as *mut PHYSFS_File);
}
/* **************************************************************************/
/* **************************************************************************/
//*
// ======================================================================
// ======================================================================
//
#[no_mangle]
pub unsafe extern "C" fn cdAudio_Open(mut user_musicdir: *mut libc::c_char)
 -> BOOL {
    if openal_initialized == 0 { return 0 as libc::c_int }
    alGenBuffers(16 as libc::c_int, music_buffers.as_mut_ptr());
    alGenSources(1 as libc::c_int, &mut music_source);
    alSourcef(music_source, 0x100a as libc::c_int, music_volume);
    alSource3f(music_source, 0x1004 as libc::c_int, 0.0f64 as ALfloat,
               0.0f64 as ALfloat, 0.0f64 as ALfloat);
    alSource3f(music_source, 0x1006 as libc::c_int, 0.0f64 as ALfloat,
               0.0f64 as ALfloat, 0.0f64 as ALfloat);
    alSourcef(music_source, 0x1021 as libc::c_int, 0.0f64 as ALfloat);
    alSourcei(music_source, 0x202 as libc::c_int, 1 as libc::c_int);
    PlayList_Init();
    if (user_musicdir.is_null() ||
            PlayList_Read(user_musicdir) as libc::c_int != 0) &&
           PlayList_Read(b"music\x00" as *const u8 as *const libc::c_char) as
               libc::c_int != 0 {
        return 0 as libc::c_int
    }
    music_initialized = 1 as libc::c_int;
    return 1 as libc::c_int;
}
//*
// ======================================================================
// ======================================================================
//
#[no_mangle]
pub unsafe extern "C" fn cdAudio_Close() -> BOOL {
    alDeleteBuffers(16 as libc::c_int, music_buffers.as_mut_ptr());
    alDeleteSources(1 as libc::c_int, &mut music_source);
    PlayList_Quit();
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cdAudio_OpenTrack(mut filename: *mut libc::c_char)
 -> BOOL {
    if music_initialized == 0 { return 0 as libc::c_int }
    if !music_file.is_null() { PHYSFS_close(music_file); }
    music_file_format = WZ_NONE;
    if strncasecmp(filename.offset(strlen(filename) as
                                       isize).offset(-(4 as libc::c_int as
                                                           isize)),
                   b".ogg\x00" as *const u8 as *const libc::c_char,
                   4 as libc::c_int as libc::c_uint) == 0 as libc::c_int {
        music_file = PHYSFS_openRead(filename);
        if music_file.is_null() {
            debug(LOG_SOUND,
                  b"Failed opening %s: %s\n\x00" as *const u8 as
                      *const libc::c_char, filename, PHYSFS_getLastError());
            return 0 as libc::c_int
        }
        if ov_open_callbacks(music_file as *mut libc::c_void, &mut ogg_stream,
                             0 as *const libc::c_char,
                             0 as libc::c_int as libc::c_long,
                             wz_ogg_callbacks) < 0 as libc::c_int {
            PHYSFS_close(music_file);
            music_file = 0 as *mut PHYSFS_File;
            return 0 as libc::c_int
        }
        ogg_info = ov_info(&mut ogg_stream, -(1 as libc::c_int));
        if (*ogg_info).channels == 1 as libc::c_int {
            music_format = 0x1101 as libc::c_int
        } else { music_format = 0x1103 as libc::c_int }
        music_file_format = WZ_OGG;
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
    // unhandled
}
#[no_mangle]
pub unsafe extern "C" fn cdAudio_CloseTrack() -> BOOL {
    if music_track != 0 as libc::c_int as libc::c_uint {
        let mut queued: libc::c_int = 0;
        let mut processed: libc::c_int = 0;
        let mut all: libc::c_int = 0;
        alSourceStop(music_source);
        alGetSourcei(music_source, 0x1015 as libc::c_int, &mut queued);
        alGetSourcei(music_source, 0x1016 as libc::c_int, &mut processed);
        all = queued + processed;
        while all > 0 as libc::c_int {
            let mut buffer: ALuint = 0;
            alSourceUnqueueBuffers(music_source, 1 as libc::c_int,
                                   &mut buffer);
            all -= 1
        }
        ov_clear(&mut ogg_stream);
        // WZ_NOOGG
        PHYSFS_close(music_file);
        music_file = 0 as *mut PHYSFS_File;
        music_track = 0 as libc::c_int as libc::c_uint
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cdAudio_FillBuffer(mut b: ALuint) -> BOOL {
    let mut size: libc::c_int = 0 as libc::c_int;
    let mut section: libc::c_int = 0;
    let mut result: libc::c_int = 0 as libc::c_int;
    while size < 16384 as libc::c_int {
        if music_file_format as libc::c_uint ==
               WZ_OGG as libc::c_int as libc::c_uint {
            result =
                ov_read(&mut ogg_stream,
                        music_data.as_mut_ptr().offset(size as isize),
                        16384 as libc::c_int - size, 0 as libc::c_int,
                        2 as libc::c_int, 1 as libc::c_int, &mut section) as
                    libc::c_int;
            music_rate = (*ogg_info).rate as libc::c_uint
        }
        if result > 0 as libc::c_int {
            size += result
        } else {
            let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
            filename = PlayList_NextSong();
            if filename.is_null() {
                music_track = 0 as libc::c_int as libc::c_uint
            } else if cdAudio_OpenTrack(filename) != 0 {
                debug(LOG_SOUND,
                      b"Now playing %s\n\x00" as *const u8 as
                          *const libc::c_char, filename);
            } else {
                return 0 as libc::c_int
                // break out to avoid infinite loops
            }
        }
    }
    if size == 0 as libc::c_int { return 0 as libc::c_int }
    alBufferData(b, music_format,
                 music_data.as_mut_ptr() as *const libc::c_void, size,
                 music_rate as ALsizei);
    return 1 as libc::c_int;
}
//*
// ======================================================================
// ======================================================================
//
#[no_mangle]
pub unsafe extern "C" fn cdAudio_PlayTrack(mut iTrack: SDWORD) -> BOOL {
    let mut i: libc::c_uint = 0;
    cdAudio_CloseTrack();
    PlayList_SetTrack(iTrack as libc::c_uint);
    let mut filename: *mut libc::c_char = PlayList_CurrentSong();
    if filename.is_null() {
        music_track = 0 as libc::c_int as libc::c_uint;
        return 0 as libc::c_int
    }
    if cdAudio_OpenTrack(filename) != 0 {
        music_track = iTrack as libc::c_uint
    } else {
        return 0 as libc::c_int
        // break out to avoid infinite loops
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < 16 as libc::c_int as libc::c_uint {
        if cdAudio_FillBuffer(music_buffers[i as usize]) == 0 {
            return 0 as libc::c_int
        }
        i = i.wrapping_add(1)
    }
    alSourceQueueBuffers(music_source, 16 as libc::c_int,
                         music_buffers.as_mut_ptr());
    alSourcePlay(music_source);
    return 1 as libc::c_int;
}
//*
// ======================================================================
// ======================================================================
//
#[no_mangle]
pub unsafe extern "C" fn cdAudio_Stop() -> BOOL {
    cdAudio_CloseTrack();
    return 1 as libc::c_int;
}
//*
// ======================================================================
// ======================================================================
//
#[no_mangle]
pub unsafe extern "C" fn cdAudio_Pause() -> BOOL { return 1 as libc::c_int; }
//*
// ======================================================================
// ======================================================================
//
#[no_mangle]
pub unsafe extern "C" fn cdAudio_Resume() -> BOOL { return 1 as libc::c_int; }
//*
// ======================================================================
// ======================================================================
//
#[no_mangle]
pub unsafe extern "C" fn cdAudio_SetVolume(mut iVol: SDWORD) {
    //alSourcef(music_source, AL_GAIN, 0.01*iVol);
}
//*
// ======================================================================
// ======================================================================
//
#[no_mangle]
pub unsafe extern "C" fn cdAudio_Update() {
    if music_track != 0 as libc::c_int as libc::c_uint &&
           music_volume as libc::c_double != 0.0f64 {
        let mut processed: libc::c_int = 0 as libc::c_int;
        alGetSourcei(music_source, 0x1016 as libc::c_int, &mut processed);
        while processed > 0 as libc::c_int {
            let mut buffer: ALuint = 0;
            alSourceUnqueueBuffers(music_source, 1 as libc::c_int,
                                   &mut buffer);
            cdAudio_FillBuffer(buffer);
            alSourceQueueBuffers(music_source, 1 as libc::c_int, &mut buffer);
            processed -= 1
        }
        let mut state: ALenum = 0;
        alGetSourcei(music_source, 0x1010 as libc::c_int, &mut state);
        if state != 0x1012 as libc::c_int { alSourcePlay(music_source); }
    };
}
//*
// ======================================================================
// ======================================================================
//
#[no_mangle]
pub unsafe extern "C" fn mixer_GetCDVolume() -> SDWORD {
    return (100 as libc::c_int as libc::c_float * music_volume) as SDWORD;
}
//*
// ======================================================================
// ======================================================================
//
#[no_mangle]
pub unsafe extern "C" fn mixer_SetCDVolume(mut iVol: SDWORD) {
    music_volume = (0.01f64 * iVol as libc::c_double) as ALfloat;
    if (music_volume as libc::c_double) < 0.0f64 {
        music_volume = 0.0f64 as ALfloat
    } else if music_volume as libc::c_double > 1.0f64 {
        music_volume = 1.0f64 as ALfloat
    }
    alSourcef(music_source, 0x100a as libc::c_int, music_volume);
}
