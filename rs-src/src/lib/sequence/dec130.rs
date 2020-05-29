use ::libc;
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
// FIXME Stubfile!
// Replace this with a function that actually decodes the video.
#[no_mangle]
pub unsafe extern "C" fn dec130_decode(mut rpl: *mut RPL,
                                       mut _in: *mut libc::c_char,
                                       mut in_size: libc::c_uint,
                                       mut out: *mut libc::c_char)
 -> libc::c_uint 
 // FIXME Remove if unused
 {
    return 0 as libc::c_int as libc::c_uint;
}
