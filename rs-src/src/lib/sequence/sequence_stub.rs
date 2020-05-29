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
    fn rpl_open(filename: *mut libc::c_char) -> *mut RPL;
    #[no_mangle]
    fn rpl_decode_sound(rpl: *mut RPL, buffer: *mut libc::c_short,
                        buffer_size: libc::c_uint) -> libc::c_uint;
    #[no_mangle]
    fn rpl_decode_next_image(rpl: *mut RPL, buffer: *mut libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn rpl_close(rpl: *mut RPL);
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
    fn alSourcePlay(source: ALuint);
    #[no_mangle]
    fn alSourceStop(source: ALuint);
    #[no_mangle]
    fn alSourceQueueBuffers(source: ALuint, nb: ALsizei,
                            buffers: *const ALuint);
    #[no_mangle]
    fn alGenBuffers(n: ALsizei, buffers: *mut ALuint);
    #[no_mangle]
    fn alDeleteBuffers(n: ALsizei, buffers: *const ALuint);
    #[no_mangle]
    fn alBufferData(buffer: ALuint, format: ALenum, data: *const libc::c_void,
                    size: ALsizei, freq: ALsizei);
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PHYSFS_File {
    pub opaque: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RPL_chunk_info_t {
    pub offset: libc::c_int,
    pub video_size: libc::c_int,
    pub audio_size: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RPL {
    pub f: *mut PHYSFS_File,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub bpp: libc::c_int,
    pub fps: libc::c_float,
    pub video_decoder: Option<unsafe extern "C" fn(_: *mut RPL,
                                                   _: *mut libc::c_char,
                                                   _: libc::c_uint,
                                                   _: *mut libc::c_char)
                                  -> libc::c_uint>,
    pub current_video_frame: libc::c_uint,
    pub samples: libc::c_int,
    pub channels: libc::c_int,
    pub bps: libc::c_int,
    pub fpc: libc::c_int,
    pub sound_decoder: Option<unsafe extern "C" fn(_: *mut RPL,
                                                   _: *mut libc::c_short,
                                                   _: libc::c_uint)
                                  -> libc::c_uint>,
    pub current_sound_frame: libc::c_uint,
    pub nb_chunks: libc::c_int,
    pub chunks: *mut RPL_chunk_info_t,
    pub ocs: libc::c_int,
    pub ecs: libc::c_int,
    pub otcc: libc::c_int,
    pub ots: libc::c_int,
    pub sprite_size: libc::c_int,
    pub otkf: libc::c_int,
}
pub type _perf_mode = libc::c_uint;
pub const VIDEO_PERF_SKIP_FRAMES: _perf_mode = 2;
pub const VIDEO_PERF_WINDOW: _perf_mode = 1;
pub const VIDEO_PERF_FULLSCREEN: _perf_mode = 0;
pub type PERF_MODE = _perf_mode;
pub type ALuint = libc::c_uint;
pub type ALsizei = libc::c_int;
pub type ALint = libc::c_int;
pub type ALenum = libc::c_int;
pub type ALfloat = libc::c_float;
pub type ALvoid = ();
// Normal alternate line video
// 320 240 centred
// 320 240 centred and display every 4th frame
/* **************************************************************************/
/*
 * Sequence.c
 *
 * Sequence setup and video control
 *
 * based on eidos example code
 *
 *
 */
/* **************************************************************************/
// Standard include file
#[no_mangle]
pub static mut current_sequence: *mut RPL = 0 as *const RPL as *mut RPL;
#[no_mangle]
pub static mut current_frame: libc::c_uint = 0 as libc::c_int as libc::c_uint;
#[no_mangle]
pub static mut sound_buffer: [libc::c_short; 1048576] = [0; 1048576];
#[no_mangle]
pub static mut seq_sound: BOOL = 0 as libc::c_int;
/* **************************************************************************/
/*
 *	Source
 */
/* **************************************************************************/
#[no_mangle]
pub static mut seq_buffer: ALuint = 0;
#[no_mangle]
pub static mut seq_source: ALuint = 0;
#[no_mangle]
pub unsafe extern "C" fn seq_start_sound(mut s: *mut RPL) {
    let mut buffer_size: libc::c_uint =
        rpl_decode_sound(s, sound_buffer.as_mut_ptr(),
                         (1024 as libc::c_int * 1024 as libc::c_int) as
                             libc::c_uint);
    if buffer_size != 0 as libc::c_int as libc::c_uint {
        seq_sound = 1 as libc::c_int;
        alGenBuffers(1 as libc::c_int, &mut seq_buffer);
        alBufferData(seq_buffer, 0x1101 as libc::c_int,
                     sound_buffer.as_mut_ptr() as *const libc::c_void,
                     buffer_size.wrapping_mul(::std::mem::size_of::<libc::c_short>()
                                                  as libc::c_ulong) as
                         ALsizei, 22050 as libc::c_int);
        alGenSources(1 as libc::c_int, &mut seq_source);
        alSourcef(seq_source, 0x100a as libc::c_int,
                  1 as libc::c_int as ALfloat);
        alSource3f(seq_source, 0x1004 as libc::c_int, 0.0f64 as ALfloat,
                   0.0f64 as ALfloat, 0.0f64 as ALfloat);
        alSource3f(seq_source, 0x1006 as libc::c_int, 0.0f64 as ALfloat,
                   0.0f64 as ALfloat, 0.0f64 as ALfloat);
        alSourcef(seq_source, 0x1021 as libc::c_int, 0.0f64 as ALfloat);
        alSourcei(seq_source, 0x202 as libc::c_int, 1 as libc::c_int);
        alSourceQueueBuffers(seq_source, 1 as libc::c_int, &mut seq_buffer);
        alSourcePlay(seq_source);
    };
}
/* **************************************************************************/
/*
 *	Global Variables
 */
/* **************************************************************************/
/* **************************************************************************/
/*
 *	Global ProtoTypes
 */
/* **************************************************************************/
//buffer render for software_window 3DFX_window and 3DFX_fullscreen modes
#[no_mangle]
pub unsafe extern "C" fn seq_SetSequenceForBuffer(mut filename:
                                                      *mut libc::c_char,
                                                  mut startTime: libc::c_int,
                                                  mut perfMode: PERF_MODE)
 -> BOOL {
    debug(LOG_VIDEO,
          b"seq_SetSequenceForBuffer %s -> novideo.rpl\n\x00" as *const u8 as
              *const libc::c_char, filename);
    filename =
        b"novideo.rpl\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char;
    if !current_sequence.is_null() { rpl_close(current_sequence); }
    current_sequence = rpl_open(filename);
    if current_sequence.is_null() { return 0 as libc::c_int }
    current_frame = 0 as libc::c_int as libc::c_uint;
    seq_start_sound(current_sequence);
    return 1 as libc::c_int;
}
//directX fullscreeen render uses local buffer to store previous frame data
/*
 * directX fullscreeen render uses local buffer to store previous frame data
 * directX 640 * 480 16bit rgb mode render through local buffer to back buffer
 */
#[no_mangle]
pub unsafe extern "C" fn seq_SetSequence(mut filename: *mut libc::c_char,
                                         mut startTime: libc::c_int,
                                         mut lpBF: *mut libc::c_char,
                                         mut perfMode: PERF_MODE) -> BOOL {
    debug(LOG_VIDEO,
          b"seq_SetSequence %s -> novideo.rpl\n\x00" as *const u8 as
              *const libc::c_char, filename);
    filename =
        b"novideo.rpl\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char;
    if !current_sequence.is_null() { rpl_close(current_sequence); }
    current_sequence = rpl_open(filename);
    if current_sequence.is_null() { return 0 as libc::c_int }
    current_frame = 0 as libc::c_int as libc::c_uint;
    seq_start_sound(current_sequence);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn seq_ClearMovie() -> libc::c_int {
    debug(LOG_VIDEO,
          b"seq_ClearMovie\n\x00" as *const u8 as *const libc::c_char);
    if !current_sequence.is_null() {
        rpl_close(current_sequence);
        current_sequence = 0 as *mut RPL
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn seq_RenderOneFrameToBuffer(mut lpSF:
                                                        *mut libc::c_char,
                                                    mut skip: libc::c_int,
                                                    mut subMin: SDWORD,
                                                    mut subMax: SDWORD)
 -> libc::c_int {
    //printf("seq_RenderOneFrameToBuffer %i\n", skip);
    if current_sequence.is_null() {
        return -(3 as libc::c_int)
    } else {
        rpl_decode_next_image(current_sequence, lpSF);
        current_frame = current_frame.wrapping_add(1);
        if current_frame >= (*current_sequence).nb_chunks as libc::c_uint {
            return -(1 as libc::c_int)
        }
    }
    return current_frame as libc::c_int;
}
/*
 * render one frame to a direct draw surface (normally the back buffer)
 * directX 640 * 480 16bit rgb mode render through local buffer to back buffer
 */
#[no_mangle]
pub unsafe extern "C" fn seq_RenderOneFrame(mut skip: libc::c_int,
                                            mut subMin: SDWORD,
                                            mut subMax: SDWORD)
 -> libc::c_int {
    //printf("seq_RenderOneFrame %i\n", skip);
    if current_sequence.is_null() { return -(3 as libc::c_int) }
    current_frame = current_frame.wrapping_add(1);
    if current_frame >= 1000 as libc::c_int as libc::c_uint {
        return -(1 as libc::c_int)
    }
    return current_frame as libc::c_int;
}
//setup monitoring and control
#[no_mangle]
pub unsafe extern "C" fn seq_RefreshVideoBuffers() -> BOOL {
    //printf("seq_RefreshVideoBuffers\n");
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn seq_ShutDown() -> BOOL {
    debug(LOG_VIDEO,
          b"seq_ShutDown\n\x00" as *const u8 as *const libc::c_char);
    if !current_sequence.is_null() {
        if seq_sound == 1 as libc::c_int {
            alSourceStop(seq_source);
            alDeleteSources(1 as libc::c_int, &mut seq_source);
            alDeleteBuffers(1 as libc::c_int, &mut seq_buffer);
        }
        rpl_close(current_sequence);
        current_sequence = 0 as *mut RPL
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn seq_GetFrameSize(mut pWidth: *mut SDWORD,
                                          mut pHeight: *mut SDWORD) -> BOOL {
    //printf("seq_GetFrameSize\n");
    if !current_sequence.is_null() {
        *pWidth = (*current_sequence).width;
        *pHeight = (*current_sequence).height;
        return 1 as libc::c_int
    } else {
        *pWidth = 0 as libc::c_int;
        *pHeight = 0 as libc::c_int;
        return 0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn seq_GetCurrentFrame() -> libc::c_int {
    //printf("seq_GetCurrentFrame\n");
    if !current_sequence.is_null() {
        return current_frame as libc::c_int
    } else { return -(1 as libc::c_int) };
}
#[no_mangle]
pub unsafe extern "C" fn seq_GetFrameTimeInClicks() -> libc::c_int {
    //printf("seq_GetFrameTimeInClicks\n");
    if !current_sequence.is_null() {
        return (1000.0f64 / (*current_sequence).fps as libc::c_double) as
                   libc::c_int
    } else { return 0 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn seq_GetTotalFrames() -> libc::c_int {
    //printf("seq_GetTotalFrames\n");
    if !current_sequence.is_null() {
        return (*current_sequence).nb_chunks
    } else { return -(1 as libc::c_int) };
}
