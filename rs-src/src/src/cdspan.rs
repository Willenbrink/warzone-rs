use ::libc;
extern "C" {
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn audio_StopAll();
    #[no_mangle]
    fn audio_PlayStream(szFileName: *mut libc::c_char, iVol: SDWORD,
                        pUserCallback: AUDIO_CALLBACK) -> BOOL;
}
pub type STRING = libc::c_char;
pub type SDWORD = libc::c_int;
pub type BOOL = libc::c_int;
/* * unsigned 32-bit integer */
pub type ALuint = libc::c_uint;
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
/* Handles the two CD issue */
/* Alex McLean */
/* seems to be responsible for playing music in-game,
 * move that to somewhere else
 */
static mut g_szCurDriveName: [STRING; 255] =
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
#[no_mangle]
pub unsafe extern "C" fn cdspan_PlayInGameAudio(mut szFileName: *mut STRING,
                                                mut iVol: SDWORD) {
    let mut szStream: [STRING; 255] = [0; 255]; //	szDrive[MAX_STR] = "",
    let mut bPlaying: BOOL = 0 as libc::c_int;
    audio_StopAll();
    sprintf(szStream.as_mut_ptr(),
            b"%s%s\x00" as *const u8 as *const libc::c_char,
            g_szCurDriveName.as_mut_ptr(), szFileName);
    bPlaying = audio_PlayStream(szStream.as_mut_ptr(), iVol, None);
    /* try playing from hard disk */
    if bPlaying == 0 as libc::c_int {
        sprintf(szStream.as_mut_ptr(),
                b"audio\\%s\x00" as *const u8 as *const libc::c_char,
                szFileName);
        bPlaying = audio_PlayStream(szStream.as_mut_ptr(), iVol, None)
    };
}
