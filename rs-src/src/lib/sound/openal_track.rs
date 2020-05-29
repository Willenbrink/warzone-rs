use ::libc;
extern "C" {
    pub type ALCdevice_struct;
    pub type ALCcontext_struct;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_uint)
     -> *mut libc::c_void;
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
    fn malloc(_: libc::c_uint) -> *mut libc::c_void;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_uint) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn cos(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn sin(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn sqrt(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn alDistanceModel(distanceModel: ALenum);
    #[no_mangle]
    fn alGetString(param: ALenum) -> *const ALchar;
    #[no_mangle]
    fn alGetError() -> ALenum;
    #[no_mangle]
    fn alListenerfv(param: ALenum, values: *const ALfloat);
    #[no_mangle]
    fn alGenSources(n: ALsizei, sources: *mut ALuint);
    #[no_mangle]
    fn alDeleteSources(n: ALsizei, sources: *const ALuint);
    #[no_mangle]
    fn alSourcef(source: ALuint, param: ALenum, value: ALfloat);
    #[no_mangle]
    fn alSourcefv(source: ALuint, param: ALenum, values: *const ALfloat);
    #[no_mangle]
    fn alSourcei(source: ALuint, param: ALenum, value: ALint);
    #[no_mangle]
    fn alGetSourcei(source: ALuint, param: ALenum, value: *mut ALint);
    #[no_mangle]
    fn alSourcePlay(source: ALuint);
    #[no_mangle]
    fn alSourceStop(source: ALuint);
    #[no_mangle]
    fn alSourcePause(source: ALuint);
    #[no_mangle]
    fn alGenBuffers(n: ALsizei, buffers: *mut ALuint);
    #[no_mangle]
    fn alDeleteBuffers(n: ALsizei, buffers: *const ALuint);
    #[no_mangle]
    fn alBufferData(buffer: ALuint, format: ALenum,
                    data_0: *const libc::c_void, size: ALsizei,
                    freq: ALsizei);
    #[no_mangle]
    fn alcCreateContext(device_0: *mut ALCdevice, attrlist: *const ALCint)
     -> *mut ALCcontext;
    #[no_mangle]
    fn alcMakeContextCurrent(context_0: *mut ALCcontext) -> ALCboolean;
    #[no_mangle]
    fn alcProcessContext(context_0: *mut ALCcontext);
    #[no_mangle]
    fn alcDestroyContext(context_0: *mut ALCcontext);
    #[no_mangle]
    fn alcOpenDevice(devicename: *const ALCchar) -> *mut ALCdevice;
    #[no_mangle]
    fn alcCloseDevice(device_0: *mut ALCdevice) -> ALCboolean;
    #[no_mangle]
    fn alcGetError(device_0: *mut ALCdevice) -> ALCenum;
    #[no_mangle]
    fn alcGetString(device_0: *mut ALCdevice, param: ALCenum)
     -> *const ALCchar;
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
    fn PHYSFS_openRead(filename: *const libc::c_char) -> *mut PHYSFS_File;
    #[no_mangle]
    fn PHYSFS_read(handle: *mut PHYSFS_File, buffer: *mut libc::c_void,
                   objSize: PHYSFS_uint32, objCount: PHYSFS_uint32)
     -> PHYSFS_sint64;
    #[no_mangle]
    fn PHYSFS_tell(handle: *mut PHYSFS_File) -> PHYSFS_sint64;
    #[no_mangle]
    fn PHYSFS_seek(handle: *mut PHYSFS_File, pos: PHYSFS_uint64)
     -> libc::c_int;
    #[no_mangle]
    fn sound_TrackLooped(iTrack: SDWORD) -> BOOL;
    #[no_mangle]
    fn sound_FinishedCallback(psSample: *mut AUDIO_SAMPLE);
    #[no_mangle]
    fn cdAudio_Update() -> BOOL;
}
pub type __int64_t = libc::c_longlong;
pub type size_t = libc::c_uint;
pub type int64_t = __int64_t;
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
pub type ALchar = libc::c_char;
pub type ALint = libc::c_int;
pub type ALuint = libc::c_uint;
pub type ALsizei = libc::c_int;
pub type ALenum = libc::c_int;
pub type ALfloat = libc::c_float;
pub type ALvoid = ();
pub type ALCdevice = ALCdevice_struct;
pub type ALCcontext = ALCcontext_struct;
pub type ALCboolean = libc::c_char;
pub type ALCchar = libc::c_char;
pub type ALCint = libc::c_int;
pub type ALCenum = libc::c_int;
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
pub type PHYSFS_uint32 = libc::c_uint;
pub type PHYSFS_uint64 = libc::c_ulonglong;
pub type PHYSFS_sint64 = libc::c_longlong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PHYSFS_File {
    pub opaque: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AUDIO_SAMPLE {
    pub iTrack: SDWORD,
    pub iSample: ALuint,
    pub x: SDWORD,
    pub y: SDWORD,
    pub z: SDWORD,
    pub iLoops: SDWORD,
    pub bRemove: BOOL,
    pub pCallback: AUDIO_CALLBACK,
    pub psObj: *mut libc::c_void,
    pub psPrev: *mut AUDIO_SAMPLE,
    pub psNext: *mut AUDIO_SAMPLE,
}
pub type AUDIO_CALLBACK
    =
    Option<unsafe extern "C" fn(_: *mut AUDIO_SAMPLE) -> BOOL>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TRACK {
    pub bLoop: BOOL,
    pub iVol: SDWORD,
    pub iPriority: SDWORD,
    pub iAudibleRadius: SDWORD,
    pub iTime: SDWORD,
    pub iTimeLastFinished: UDWORD,
    pub iNumPlaying: UDWORD,
    pub bMemBuffer: BOOL,
    pub bCompressed: BOOL,
    pub pMem: *mut libc::c_void,
    pub pName: *mut STRING,
    pub resID: UDWORD,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SAMPLE_LIST {
    pub curr: *mut AUDIO_SAMPLE,
    pub next: *mut SAMPLE_LIST,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ov_buffer_t {
    pub buffer: *mut libc::c_void,
    pub size: libc::c_uint,
    pub pos: libc::c_uint,
}
#[no_mangle]
pub static mut current_queue_sample: ALuint = -(1 as libc::c_int) as ALuint;
static mut active_samples: *mut SAMPLE_LIST =
    0 as *const SAMPLE_LIST as *mut SAMPLE_LIST;
static mut listenerPos: [ALfloat; 3] =
    [0.0f64 as ALfloat, 0.0f64 as ALfloat, 0.0f64 as ALfloat];
static mut sfx_volume: ALfloat = 1.0f64 as ALfloat;
static mut sfx3d_volume: ALfloat = 1.0f64 as ALfloat;
static mut device: *mut ALCdevice = 0 as *const ALCdevice as *mut ALCdevice;
static mut context: *mut ALCcontext =
    0 as *const ALCcontext as *mut ALCcontext;
static mut data: *mut libc::c_void =
    0 as *const libc::c_void as *mut libc::c_void;
// Needed for ReadTrackFromBuffer, must be global, so it can be free'd on shutdown
#[no_mangle]
pub static mut openal_initialized: BOOL = 0 as libc::c_int;
unsafe extern "C" fn PrintOpenALVersion() {
    debug(LOG_ERROR,
          b"OpenAL Vendor: %s\nOpenAL Version: %s\nOpenAL Renderer: %s\nOpenAL Extensions: %s\n\x00"
              as *const u8 as *const libc::c_char,
          alGetString(0xb001 as libc::c_int),
          alGetString(0xb002 as libc::c_int),
          alGetString(0xb003 as libc::c_int),
          alGetString(0xb004 as libc::c_int));
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn sound_InitLibrary() -> BOOL {
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    let mut err: libc::c_int = 0 as libc::c_int;
    let mut listenerVel: [ALfloat; 3] =
        [0.0f64 as ALfloat, 0.0f64 as ALfloat, 0.0f64 as ALfloat];
    let mut listenerOri: [ALfloat; 6] =
        [0.0f64 as ALfloat, 0.0f64 as ALfloat, 1.0f64 as ALfloat,
         0.0f64 as ALfloat, 1.0f64 as ALfloat, 0.0f64 as ALfloat];
    //	int contextAttributes[] = { 0 };
	//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    device = alcOpenDevice(0 as *const ALCchar); //NULL was contextAttributes
    if device.is_null() {
        PrintOpenALVersion();
        debug(LOG_ERROR,
              b"Couldn\'t open audio device.\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    context = alcCreateContext(device, 0 as *const ALCint);
    alcMakeContextCurrent(context);
    err = alcGetError(device);
    if err != 0 as libc::c_int {
        PrintOpenALVersion();
        debug(LOG_ERROR,
              b"Couldn\'t initialize audio context: %s\x00" as *const u8 as
                  *const libc::c_char, alcGetString(device, err));
        return 0 as libc::c_int
    }
    err = alGetError();
    if err != 0 as libc::c_int {
        PrintOpenALVersion();
        debug(LOG_ERROR,
              b"Audio error after init: %s\x00" as *const u8 as
                  *const libc::c_char, alGetString(err));
        return 0 as libc::c_int
    }
    openal_initialized = 1 as libc::c_int;
    // Clear Error Codes
    alGetError();
    alcGetError(device);
    // Check what version of Open AL we are using
    debug(LOG_SOUND,
          b"OpenAL Version : %s\x00" as *const u8 as *const libc::c_char,
          alGetString(0xb002 as libc::c_int));
    debug(LOG_SOUND,
          b"OpenAL Renderer : %s\x00" as *const u8 as *const libc::c_char,
          alGetString(0xb003 as libc::c_int));
    debug(LOG_SOUND,
          b"OpenAL Extensions : %s\x00" as *const u8 as *const libc::c_char,
          alGetString(0xb004 as libc::c_int));
    alListenerfv(0x1004 as libc::c_int, listenerPos.as_mut_ptr());
    alListenerfv(0x1006 as libc::c_int, listenerVel.as_mut_ptr());
    alListenerfv(0x100f as libc::c_int, listenerOri.as_mut_ptr());
    alDistanceModel(0 as libc::c_int);
    return 1 as libc::c_int;
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn sound_ShutdownLibrary() {
    let mut aSample: *mut SAMPLE_LIST =
        active_samples; // this gives a long delay on some impl.
    let mut tmpSample: *mut SAMPLE_LIST = 0 as *mut SAMPLE_LIST;
    debug(LOG_SOUND,
          b"sound_ShutdownLibrary: starting shutdown\x00" as *const u8 as
              *const libc::c_char);
    if !context.is_null() {
        debug(LOG_SOUND,
              b"sound_ShutdownLibrary: destroy previous context\x00" as
                  *const u8 as *const libc::c_char);
        alcDestroyContext(context);
        context = 0 as *mut ALCcontext
    }
    debug(LOG_SOUND,
          b"sound_ShutdownLibrary: close device\x00" as *const u8 as
              *const libc::c_char);
    if !device.is_null() {
        alcCloseDevice(device);
        device = 0 as *mut ALCdevice
    }
    free(data);
    while !aSample.is_null() {
        tmpSample = (*aSample).next;
        free(aSample as *mut libc::c_void);
        aSample = tmpSample
    };
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn sound_Update() {
    let mut err: libc::c_int = 0 as libc::c_int;
    //	{			//  <=== whats up with this ??
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    let mut tmp: *mut *mut SAMPLE_LIST = &mut active_samples;
    let mut i: *mut SAMPLE_LIST = 0 as *mut SAMPLE_LIST;
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    tmp = &mut active_samples;
    i = *tmp;
    while !i.is_null() {
        //~~~~~~~~~~
        let mut state: ALenum = 0x1014 as libc::c_int;
        //~~~~~~~~~~
        alGetSourcei((*(*i).curr).iSample, 0x1010 as libc::c_int, &mut state);
        match state {
            4114 | 4115 => {
                //
				// * sound_SetObjectPosition(i->curr->iSample, i->curr->x, i->curr->y, i->curr->z);
				//
                tmp = &mut (*i).next
            }
            _ => {
                sound_FinishedCallback((*i).curr);
                if (*(*i).curr).iSample != -(1 as libc::c_int) as libc::c_uint
                   {
                    alDeleteSources(1 as libc::c_int,
                                    &mut (*(*i).curr).iSample);
                    (*(*i).curr).iSample = -(1 as libc::c_int) as ALuint
                }
                *tmp = (*i).next;
                free(i as *mut libc::c_void);
            }
        }
        i = *tmp
    }
    //	}//  <=== whats up with this You trying to make those local only ??
    cdAudio_Update();
    alcProcessContext(context);
    err = alcGetError(device);
    if err != 0 as libc::c_int {
        debug(LOG_ERROR,
              b"Error while processing audio context: %s\x00" as *const u8 as
                  *const libc::c_char, alGetString(err));
    };
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn sound_QueueSamplePlaying() -> BOOL {
    if current_queue_sample == -(1 as libc::c_int) as libc::c_uint {
        return 0 as libc::c_int
    } else {
        //~~~~~~~~~~
        let mut state: ALenum = 0;
        //~~~~~~~~~~
        alGetSourcei(current_queue_sample, 0x1010 as libc::c_int, &mut state);
        if state == 0x1012 as libc::c_int {
            return 1 as libc::c_int
        } else {
            if current_queue_sample != -(1 as libc::c_int) as libc::c_uint {
                alDeleteSources(1 as libc::c_int, &mut current_queue_sample);
                current_queue_sample = -(1 as libc::c_int) as ALuint
            }
            current_queue_sample = -(1 as libc::c_int) as ALuint;
            return 0 as libc::c_int
        }
    };
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
unsafe extern "C" fn sound_SaveTrackData(mut psTrack: *mut TRACK,
                                         mut buffer: ALuint) {
    // save data pointer in track
    (*psTrack).pMem = buffer as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn ovbuf_read(mut ptr: *mut libc::c_void,
                                    mut size: size_t, mut nmemb: size_t,
                                    mut datasource: *mut libc::c_void)
 -> size_t {
    let mut ovbuf: *mut ov_buffer_t = datasource as *mut ov_buffer_t;
    let mut read_size: libc::c_uint = size.wrapping_mul(nmemb);
    if (*ovbuf).pos.wrapping_add(read_size) > (*ovbuf).size {
        read_size = (*ovbuf).size.wrapping_sub((*ovbuf).pos)
    }
    memcpy(ptr as *mut libc::c_char as *mut libc::c_void,
           ((*ovbuf).buffer as
                *mut libc::c_char).offset((*ovbuf).pos as isize) as
               *const libc::c_void, read_size);
    (*ovbuf).pos = (*ovbuf).pos.wrapping_add(read_size);
    return read_size;
}
#[no_mangle]
pub unsafe extern "C" fn ovbuf_seek(mut datasource: *mut libc::c_void,
                                    mut offset: ogg_int64_t,
                                    mut whence: libc::c_int) -> libc::c_int {
    let mut ovbuf: *mut ov_buffer_t = datasource as *mut ov_buffer_t;
    let mut new_pos: libc::c_int = 0 as libc::c_int;
    match whence {
        0 => { new_pos = offset as libc::c_int }
        1 => {
            new_pos =
                ((*ovbuf).pos as libc::c_longlong + offset) as libc::c_int
        }
        2 => {
            new_pos =
                ((*ovbuf).size as libc::c_longlong - offset -
                     1 as libc::c_int as libc::c_longlong) as libc::c_int
        }
        _ => { }
    }
    if new_pos >= 0 as libc::c_int &&
           (new_pos as libc::c_uint) < (*ovbuf).size {
        (*ovbuf).pos = new_pos as libc::c_uint;
        return new_pos
    } else { return -(1 as libc::c_int) };
}
#[no_mangle]
pub unsafe extern "C" fn ovbuf_close(mut datasource: *mut libc::c_void)
 -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ovbuf_tell(mut datasource: *mut libc::c_void)
 -> libc::c_long {
    let mut ovbuf: *mut ov_buffer_t = datasource as *mut ov_buffer_t;
    return (*ovbuf).pos as libc::c_long;
}
static mut ovbuf_callbacks: ov_callbacks =
    unsafe {
        {
            let mut init =
                ov_callbacks{read_func:
                                 Some(ovbuf_read as
                                          unsafe extern "C" fn(_:
                                                                   *mut libc::c_void,
                                                               _: size_t,
                                                               _: size_t,
                                                               _:
                                                                   *mut libc::c_void)
                                              -> size_t),
                             seek_func:
                                 Some(ovbuf_seek as
                                          unsafe extern "C" fn(_:
                                                                   *mut libc::c_void,
                                                               _: ogg_int64_t,
                                                               _: libc::c_int)
                                              -> libc::c_int),
                             close_func:
                                 Some(ovbuf_close as
                                          unsafe extern "C" fn(_:
                                                                   *mut libc::c_void)
                                              -> libc::c_int),
                             tell_func:
                                 Some(ovbuf_tell as
                                          unsafe extern "C" fn(_:
                                                                   *mut libc::c_void)
                                              -> libc::c_long),};
            init
        }
    };
#[no_mangle]
pub unsafe extern "C" fn sound_ReadTrackFromBuffer(mut psTrack: *mut TRACK,
                                                   mut pBuffer:
                                                       *mut libc::c_void,
                                                   mut udwSize: UDWORD)
 -> BOOL {
    let mut ovbuf: ov_buffer_t =
        ov_buffer_t{buffer: 0 as *mut libc::c_void, size: 0, pos: 0,};
    let mut ogg_stream: OggVorbis_File =
        OggVorbis_File{datasource: 0 as *mut libc::c_void,
                       seekable: 0,
                       offset: 0,
                       end: 0,
                       oy:
                           ogg_sync_state{data: 0 as *mut libc::c_uchar,
                                          storage: 0,
                                          fill: 0,
                                          returned: 0,
                                          unsynced: 0,
                                          headerbytes: 0,
                                          bodybytes: 0,},
                       links: 0,
                       offsets: 0 as *mut ogg_int64_t,
                       dataoffsets: 0 as *mut ogg_int64_t,
                       serialnos: 0 as *mut libc::c_long,
                       pcmlengths: 0 as *mut ogg_int64_t,
                       vi: 0 as *mut vorbis_info,
                       vc: 0 as *mut vorbis_comment,
                       pcm_offset: 0,
                       ready_state: 0,
                       current_serialno: 0,
                       current_link: 0,
                       bittrack: 0.,
                       samptrack: 0.,
                       os:
                           ogg_stream_state{body_data:
                                                0 as *mut libc::c_uchar,
                                            body_storage: 0,
                                            body_fill: 0,
                                            body_returned: 0,
                                            lacing_vals:
                                                0 as *mut libc::c_int,
                                            granule_vals:
                                                0 as *mut ogg_int64_t,
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
                                            vi: 0 as *mut vorbis_info,
                                            pcm: 0 as *mut *mut libc::c_float,
                                            pcmret:
                                                0 as *mut *mut libc::c_float,
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
                                                0 as *mut libc::c_void,},
                       vb:
                           vorbis_block{pcm: 0 as *mut *mut libc::c_float,
                                        opb:
                                            oggpack_buffer{endbyte: 0,
                                                           endbit: 0,
                                                           buffer:
                                                               0 as
                                                                   *mut libc::c_uchar,
                                                           ptr:
                                                               0 as
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
                                        vd: 0 as *mut vorbis_dsp_state,
                                        localstore: 0 as *mut libc::c_void,
                                        localtop: 0,
                                        localalloc: 0,
                                        totaluse: 0,
                                        reap: 0 as *mut alloc_chain,
                                        glue_bits: 0,
                                        time_bits: 0,
                                        floor_bits: 0,
                                        res_bits: 0,
                                        internal: 0 as *mut libc::c_void,},
                       callbacks:
                           ov_callbacks{read_func: None,
                                        seek_func: None,
                                        close_func: None,
                                        tell_func: None,},};
    let mut ogg_info: *mut vorbis_info = 0 as *mut vorbis_info;
    let mut buffer: ALuint = 0;
    let mut size: ALsizei = 0 as libc::c_int;
    let mut freq: ALsizei = 0;
    let mut format: ALenum = 0;
    static mut data_size: ALuint = 0;
    let mut result: libc::c_int = 0;
    let mut section: libc::c_int = 0;
    ovbuf.buffer = pBuffer;
    ovbuf.size = udwSize;
    ovbuf.pos = 0 as libc::c_int as libc::c_uint;
    if ov_open_callbacks(&mut ovbuf as *mut ov_buffer_t as *mut libc::c_void,
                         &mut ogg_stream, 0 as *const libc::c_char,
                         0 as libc::c_int as libc::c_long, ovbuf_callbacks) <
           0 as libc::c_int {
        return 0 as libc::c_int
    }
    ogg_info = ov_info(&mut ogg_stream, -(1 as libc::c_int));
    if (*ogg_info).channels == 1 as libc::c_int {
        format = 0x1101 as libc::c_int
    } else { format = 0x1103 as libc::c_int }
    freq = (*ogg_info).rate as ALsizei;
    if data.is_null() {
        data_size = 8192 as libc::c_int as ALuint;
        data = malloc(data_size)
    }
    result =
        ov_read(&mut ogg_stream,
                (data as *mut libc::c_char).offset(size as isize),
                data_size.wrapping_sub(size as libc::c_uint) as libc::c_int,
                0 as libc::c_int, 2 as libc::c_int, 1 as libc::c_int,
                &mut section) as libc::c_int;
    while result != 0 as libc::c_int {
        size += result;
        if size as libc::c_uint == data_size {
            data_size =
                (data_size as
                     libc::c_uint).wrapping_mul(2 as libc::c_int as
                                                    libc::c_uint) as ALuint as
                    ALuint;
            data = realloc(data, data_size)
        }
        result =
            ov_read(&mut ogg_stream,
                    (data as *mut libc::c_char).offset(size as isize),
                    data_size.wrapping_sub(size as libc::c_uint) as
                        libc::c_int, 0 as libc::c_int, 2 as libc::c_int,
                    1 as libc::c_int, &mut section) as libc::c_int
    }
    ov_clear(&mut ogg_stream);
    alGenBuffers(1 as libc::c_int, &mut buffer);
    alBufferData(buffer, format, data, size, freq);
    sound_SaveTrackData(psTrack, buffer);
    return 1 as libc::c_int;
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn sound_ReadTrackFromFile(mut psTrack: *mut TRACK,
                                                 mut szFileName:
                                                     *mut libc::c_char)
 -> BOOL {
    let mut f: *mut PHYSFS_File =
        PHYSFS_openRead(szFileName as *const libc::c_char);
    static mut buffer: *mut libc::c_char =
        0 as *const libc::c_char as *mut libc::c_char;
    static mut buffer_size: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut size: libc::c_uint = 0;
    let mut seekbuf: [libc::c_char; 1] = [0; 1];
    if f.is_null() { return 0 as libc::c_int }
    // FIXME Ugly hack because PhysFS doesn't support reporting the filesize, nor seeking to the end
    while PHYSFS_read(f, seekbuf.as_mut_ptr() as *mut libc::c_void,
                      1 as libc::c_int as PHYSFS_uint32,
                      1 as libc::c_int as PHYSFS_uint32) != 0 {
    }
    size = PHYSFS_tell(f) as libc::c_uint;
    PHYSFS_seek(f, 0 as libc::c_int as PHYSFS_uint64);
    if size > buffer_size {
        if !buffer.is_null() { free(buffer as *mut libc::c_void); }
        buffer_size = size.wrapping_mul(2 as libc::c_int as libc::c_uint);
        buffer = malloc(buffer_size) as *mut libc::c_char
    }
    PHYSFS_read(f, buffer as *mut libc::c_void,
                1 as libc::c_int as PHYSFS_uint32, size);
    return sound_ReadTrackFromBuffer(psTrack, buffer as *mut libc::c_void,
                                     size);
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn sound_FreeTrack(mut psTrack: *mut TRACK) {
    let mut buffer: ALuint = (*psTrack).pMem as ALuint;
    alDeleteBuffers(1 as libc::c_int, &mut buffer);
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn sound_GetMaxVolume() -> libc::c_int {
    return 32767 as libc::c_int;
    // Why this value? -Q
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn sound_AddActiveSample(mut psSample:
                                                   *mut AUDIO_SAMPLE) {
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    let mut tmp: *mut SAMPLE_LIST =
        malloc(::std::mem::size_of::<SAMPLE_LIST>() as libc::c_ulong) as
            *mut SAMPLE_LIST;
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    (*tmp).curr = psSample;
    (*tmp).next = active_samples;
    active_samples = tmp;
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
unsafe extern "C" fn sound_SetupChannel(mut psSample: *mut AUDIO_SAMPLE,
                                        mut piLoops: *mut SDWORD) {
    if sound_TrackLooped((*psSample).iTrack) == 1 as libc::c_int {
        *piLoops = -(1 as libc::c_int)
    } else { *piLoops = 0 as libc::c_int }
    sound_AddActiveSample(psSample);
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn sound_Play2DSample(mut psTrack: *mut TRACK,
                                            mut psSample: *mut AUDIO_SAMPLE,
                                            mut bQueued: BOOL) -> BOOL {
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    let mut iLoops: SDWORD = 0;
    let mut zero: [ALfloat; 3] =
        [0.0f64 as ALfloat, 0.0f64 as ALfloat, 0.0f64 as ALfloat];
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    if sfx_volume as libc::c_double == 0.0f64 { return 0 as libc::c_int }
    alGenSources(1 as libc::c_int, &mut (*psSample).iSample);
    alSourcef((*psSample).iSample, 0x1003 as libc::c_int, 1.0f32);
    alSourcef((*psSample).iSample, 0x100a as libc::c_int, sfx_volume);
    alSourcefv((*psSample).iSample, 0x1004 as libc::c_int, zero.as_mut_ptr());
    alSourcefv((*psSample).iSample, 0x1006 as libc::c_int, zero.as_mut_ptr());
    alSourcei((*psSample).iSample, 0x1009 as libc::c_int,
              (*psTrack).pMem as ALuint as ALint);
    alSourcei((*psSample).iSample, 0x202 as libc::c_int, 1 as libc::c_int);
    sound_SetupChannel(psSample, &mut iLoops);
    alSourcei((*psSample).iSample, 0x1007 as libc::c_int,
              if iLoops != 0 { 1 as libc::c_int } else { 0 as libc::c_int });
    alSourcePlay((*psSample).iSample);
    if bQueued != 0 {
        current_queue_sample = (*psSample).iSample
    } else if current_queue_sample == (*psSample).iSample {
        current_queue_sample = -(1 as libc::c_int) as ALuint
    }
    return 1 as libc::c_int;
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn sound_Play3DSample(mut psTrack: *mut TRACK,
                                            mut psSample: *mut AUDIO_SAMPLE)
 -> BOOL {
    let mut iLoops: SDWORD = 0;
    let mut zero: [ALfloat; 3] =
        [0.0f64 as ALfloat, 0.0f64 as ALfloat, 0.0f64 as ALfloat];
    if sfx3d_volume as libc::c_double == 0.0f64 { return 0 as libc::c_int }
    alGenSources(1 as libc::c_int, &mut (*psSample).iSample);
    alSourcef((*psSample).iSample, 0x1003 as libc::c_int, 1.0f64 as ALfloat);
    alSourcef((*psSample).iSample, 0x100a as libc::c_int, sfx3d_volume);
    sound_SetObjectPosition((*psSample).iSample as SDWORD, (*psSample).x,
                            (*psSample).y, (*psSample).z);
    alSourcefv((*psSample).iSample, 0x1006 as libc::c_int, zero.as_mut_ptr());
    alSourcei((*psSample).iSample, 0x1009 as libc::c_int,
              (*psTrack).pMem as ALuint as ALint);
    sound_SetupChannel(psSample, &mut iLoops);
    alSourcei((*psSample).iSample, 0x1007 as libc::c_int,
              if iLoops != 0 { 1 as libc::c_int } else { 0 as libc::c_int });
    alSourcePlay((*psSample).iSample);
    return 1 as libc::c_int;
}
/* **************************************************************************/
/*
 * Library-specific sound library functions;
 * these need to be re-written for each library.
 */
/* **************************************************************************/
/* **************************************************************************/
/* **************************************************************************/
/* **************************************************************************/
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn sound_PlayStream(mut psSample: *mut AUDIO_SAMPLE,
                                          mut szFileName: *mut libc::c_char,
                                          mut iVol: SDWORD) -> BOOL {
    return 1 as libc::c_int;
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn sound_StopSample(mut iSample: UDWORD) {
    alSourceStop(iSample);
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn sound_SetSamplePan(mut psSample: *mut AUDIO_SAMPLE,
                                            mut iPan: SDWORD) {
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn sound_SetSampleVolAll(mut iVol: libc::c_int) { }
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn sound_SetPlayerPos(mut iX: SDWORD, mut iY: SDWORD,
                                            mut iZ: SDWORD) {
    listenerPos[0 as libc::c_int as usize] = iX as ALfloat;
    listenerPos[1 as libc::c_int as usize] = iY as ALfloat;
    listenerPos[2 as libc::c_int as usize] = iZ as ALfloat;
    alListenerfv(0x1004 as libc::c_int, listenerPos.as_mut_ptr());
}
//
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn sound_SetPlayerOrientation(mut iX: SDWORD,
                                                    mut iY: SDWORD,
                                                    mut iZ: SDWORD) {
    //~~~~~~~~~~~
    let mut ori: [libc::c_float; 6] = [0.; 6];
    //~~~~~~~~~~~
    ori[0 as libc::c_int as usize] =
        -sin(iZ as libc::c_float as libc::c_double * 3.14159265358979323846f64
                 / 180.0f32 as libc::c_double) as libc::c_float;
    ori[1 as libc::c_int as usize] =
        cos(iZ as libc::c_float as libc::c_double * 3.14159265358979323846f64
                / 180.0f32 as libc::c_double) as libc::c_float;
    ori[2 as libc::c_int as usize] = 0 as libc::c_int as libc::c_float;
    ori[3 as libc::c_int as usize] = 0 as libc::c_int as libc::c_float;
    ori[4 as libc::c_int as usize] = 0 as libc::c_int as libc::c_float;
    ori[5 as libc::c_int as usize] = 1 as libc::c_int as libc::c_float;
    alListenerfv(0x100f as libc::c_int, ori.as_mut_ptr());
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn sound_SetObjectPosition(mut iSample: SDWORD,
                                                 mut iX: SDWORD,
                                                 mut iY: SDWORD,
                                                 mut iZ: SDWORD) {
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    let mut pos: [libc::c_float; 3] = [0.; 3];
    let mut vx: libc::c_float =
        iX as libc::c_float - listenerPos[0 as libc::c_int as usize];
    let mut vy: libc::c_float =
        iY as libc::c_float - listenerPos[1 as libc::c_int as usize];
    let mut vz: libc::c_float =
        iZ as libc::c_float - listenerPos[2 as libc::c_int as usize];
    let mut l2: libc::c_float = vx * vx + vy * vy + vz * vz;
    let mut gain: libc::c_float =
        (1 as libc::c_int as libc::c_double -
             sqrt(l2 as libc::c_double) * 0.0003f64) as libc::c_float;
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    if (gain as libc::c_double) < 0.0f64 { gain = 0.0f64 as libc::c_float }
    alSourcef(iSample as ALuint, 0x100a as libc::c_int, gain * sfx3d_volume);
    pos[0 as libc::c_int as usize] = iX as libc::c_float;
    pos[1 as libc::c_int as usize] = iY as libc::c_float;
    pos[2 as libc::c_int as usize] = iZ as libc::c_float;
    alSourcefv(iSample as ALuint, 0x1004 as libc::c_int, pos.as_mut_ptr());
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn sound_PauseSample(mut psSample: *mut AUDIO_SAMPLE) {
    alSourcePause((*psSample).iSample);
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn sound_ResumeSample(mut psSample: *mut AUDIO_SAMPLE) {
    alSourcePlay((*psSample).iSample);
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn sound_PauseAll() { }
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn sound_ResumeAll() { }
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn sound_StopAll() { }
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn sound_SampleIsFinished(mut psSample:
                                                    *mut AUDIO_SAMPLE)
 -> BOOL {
    //~~~~~~~~~~
    let mut state: ALenum = 0;
    //~~~~~~~~~~
    alGetSourcei((*psSample).iSample, 0x1010 as libc::c_int, &mut state);
    if state == 0x1012 as libc::c_int {
        return 0 as libc::c_int
    } else {
        if (*psSample).iSample != -(1 as libc::c_int) as libc::c_uint {
            alDeleteSources(1 as libc::c_int, &mut (*psSample).iSample);
            (*psSample).iSample = -(1 as libc::c_int) as ALuint
        }
        return 1 as libc::c_int
    };
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn mixer_GetWavVolume() -> SDWORD {
    return (100 as libc::c_int as libc::c_float * sfx_volume) as SDWORD;
}
#[no_mangle]
pub unsafe extern "C" fn mixer_SetWavVolume(mut iVol: SDWORD) {
    sfx_volume = (iVol as libc::c_double * 0.01f64) as ALfloat;
    if (sfx_volume as libc::c_double) < 0.0f64 {
        sfx_volume = 0.0f64 as ALfloat
    } else if sfx_volume as libc::c_double > 1.0f64 {
        sfx_volume = 1.0f64 as ALfloat
    };
}
#[no_mangle]
pub unsafe extern "C" fn mixer_Get3dWavVolume() -> SDWORD {
    return (100 as libc::c_int as libc::c_float * sfx3d_volume) as SDWORD;
}
#[no_mangle]
pub unsafe extern "C" fn mixer_Set3dWavVolume(mut iVol: SDWORD) {
    sfx3d_volume = (iVol as libc::c_double * 0.01f64) as ALfloat;
    if (sfx3d_volume as libc::c_double) < 0.0f64 {
        sfx3d_volume = 0.0f64 as ALfloat
    } else if sfx3d_volume as libc::c_double > 1.0f64 {
        sfx3d_volume = 1.0f64 as ALfloat
    };
}
