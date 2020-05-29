use ::libc;
extern "C" {
    /* Misc || X/Open.  */
    /* Set N bytes of S to C.  */
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_uint)
     -> *mut libc::c_void;
    /* Copy SRC to DEST.  */
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    /* Return the length of S.  */
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_uint;
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
    /* Abort execution and generate a core-dump.  */
    #[no_mangle]
    fn abort() -> !;
    #[no_mangle]
    fn memMallocRelease(Size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn memFreeRelease(pMemToFree: *mut libc::c_void);
    //return last imd resource
    #[no_mangle]
    fn GetLastResourceFilename() -> *mut libc::c_char;
    // Returns the filename of the last resource file loaded
    #[no_mangle]
    fn GetLastHashName() -> UDWORD;
    #[no_mangle]
    fn HashStringIgnoreCase(String: *mut libc::c_char) -> UINT;
    #[no_mangle]
    fn sound_InitLibrary() -> BOOL;
    #[no_mangle]
    fn sound_ShutdownLibrary();
    #[no_mangle]
    fn sound_ReadTrackFromFile(psTrack: *mut TRACK,
                               szFileName: *mut libc::c_char) -> BOOL;
    #[no_mangle]
    fn sound_ReadTrackFromBuffer(psTrack: *mut TRACK,
                                 pBuffer: *mut libc::c_void, udwSize: UDWORD)
     -> BOOL;
    #[no_mangle]
    fn sound_FreeTrack(psTrack: *mut TRACK);
    #[no_mangle]
    fn sound_StopSample(iSample: UDWORD);
    #[no_mangle]
    fn sound_Play2DSample(psTrack: *mut TRACK, psSample: *mut AUDIO_SAMPLE,
                          bQueued: BOOL) -> BOOL;
    #[no_mangle]
    fn sound_Play3DSample(psTrack: *mut TRACK, psSample: *mut AUDIO_SAMPLE)
     -> BOOL;
    #[no_mangle]
    fn sound_GetGameTime() -> UDWORD;
}
pub type size_t = libc::c_uint;
pub type STRING = libc::c_char;
pub type UDWORD = libc::c_uint;
pub type SDWORD = libc::c_int;
pub type BOOL = libc::c_int;
pub type UINT = libc::c_uint;
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
/* * unsigned 32-bit integer */
pub type ALuint = libc::c_uint;
/* **************************************************************************/
/* **************************************************************************/
/* defines */
/* **************************************************************************/
/* **************************************************************************/
/* enums */
/* **************************************************************************/
/* forward definitions
 */
/* **************************************************************************/
/* typedefs
 */
/* **************************************************************************/
/* structs */
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
//*
//
// static global variables
// array of pointers to sound effects
static mut g_apTrack: *mut *mut TRACK =
    0 as *const *mut TRACK as *mut *mut TRACK;
// number of tracks loaded
static mut g_iCurTracks: SDWORD = 0 as libc::c_int;
static mut g_iSamples: SDWORD = 0 as libc::c_int;
//
// static SDWORD g_iMaxSamples;
//
static mut g_iMaxSameSamples: SDWORD = 0;
// flag set when system is active (for callbacks etc)
static mut g_bSystemActive: BOOL = 0 as libc::c_int;
static mut g_bDevVolume: BOOL = 0 as libc::c_int;
static mut g_pStopTrackCallback: AUDIO_CALLBACK = None;
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn sound_CheckDevice() -> BOOL {
    //
	// * // Bah, not needed! --Qamly. #ifdef WIN32MM WAVEOUTCAPS waveCaps;
	// * MMRESULT mmRes;
	// * // check wave out device(s) present if ( waveOutGetNumDevs() == 0 ) { DBPRINTF(
	// * ("sound_CheckDevice: error in waveOutGetNumDevs\n") );
	// * return FALSE;
	// * } // default to using first device: check volume control caps mmRes =
	// * waveOutGetDevCaps( 0, (LPWAVEOUTCAPS) &waveCaps, sizeof(WAVEOUTCAPS) );
	// * if ( mmRes != MMSYSERR_NOERROR ) { DBPRINTF( ("sound_CheckDevice: error in
	// * waveOutGetDevCaps\n") );
	// * return FALSE;
	// * } // verify device supports volume changes // if ( waveCaps.dwSupport &
	// * WAVECAPS_VOLUME ) { return TRUE;
	// * } else { DBPRINTF( ("sound_CheckDevice: wave out device doesn't support volume
	// * changes\n") );
	// * return FALSE;
	// * } #endif Checking if needed
	//
    return 1 as libc::c_int;
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn sound_Init(mut iMaxSameSamples: SDWORD) -> BOOL {
    //~~~~~~~~~~~~~
    let mut i: SDWORD = 0;
    //~~~~~~~~~~~~~
    //
	// hWnd;
	//
    g_iMaxSameSamples = iMaxSameSamples;
    g_iCurTracks = 0 as libc::c_int;
    g_bDevVolume = sound_CheckDevice();
    if sound_InitLibrary() == 0 as libc::c_int {
        debug(LOG_NEVER,
              b"Cannot init sound library\n\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    // init audio array
    g_apTrack =
        memMallocRelease((::std::mem::size_of::<*mut TRACK>() as
                              libc::c_ulong).wrapping_mul(600 as libc::c_int
                                                              as
                                                              libc::c_uint))
            as *mut *mut TRACK;
    i = 0 as libc::c_int;
    while i < 600 as libc::c_int {
        let ref mut fresh0 = *g_apTrack.offset(i as isize);
        *fresh0 = 0 as *mut TRACK;
        i += 1
    }
    // set system active flag for callbacks
    g_bSystemActive = 1 as libc::c_int;
    return 1 as libc::c_int;
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn sound_Shutdown() -> BOOL {
    memFreeRelease(g_apTrack as *mut libc::c_void);
    g_apTrack = 0 as *mut *mut TRACK;
    // set inactive flag to prevent callbacks coming after shutdown
    g_bSystemActive = 0 as libc::c_int;
    sound_ShutdownLibrary();
    return 1 as libc::c_int;
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn sound_GetSystemActive() -> BOOL {
    return g_bSystemActive;
}
//*
//
// Vag ID is just used on PSX szFileName just used on PC
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn sound_SetTrackVals(mut psTrack: *mut TRACK,
                                            mut bLoop: BOOL,
                                            mut iTrack: SDWORD,
                                            mut iVol: SDWORD,
                                            mut iPriority: SDWORD,
                                            mut iAudibleRadius: SDWORD,
                                            mut VagID: SDWORD) -> BOOL {
    if iPriority >= 0 as libc::c_int && iPriority <= 2 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"sound_CreateTrack: priority %i out of bounds\n\x00" as
                  *const u8 as *const libc::c_char, iPriority);
    };
    if iPriority >= 0 as libc::c_int && iPriority <= 2 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"track.c\x00" as *const u8 as *const libc::c_char,
              172 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 19],
                                        &[libc::c_char; 19]>(b"sound_SetTrackVals\x00")).as_ptr(),
              b"iPriority >= LOW_PRIORITY && iPriority <= HIGH_PRIORITY\x00"
                  as *const u8 as *const libc::c_char);
    };
    // add to sound array
    if iTrack < 600 as libc::c_int {
        if !(*g_apTrack.offset(iTrack as isize)).is_null() {
            debug(LOG_ERROR,
                  b"sound_SetTrackVals: track %i already set\n\x00" as
                      *const u8 as *const libc::c_char, iTrack);
            abort();
        }
        // set track members
        (*psTrack).bLoop =
            bLoop; //added, since they really should init all the values. -Q
        (*psTrack).iVol =
            iVol; //added  this was the bugger that caused grief for .net.  It was never defined. -Q
        (*psTrack).iPriority = iPriority;
        (*psTrack).iAudibleRadius = iAudibleRadius;
        (*psTrack).iTime = 0 as libc::c_int;
        (*psTrack).iTimeLastFinished = 0 as libc::c_int as UDWORD;
        (*psTrack).iNumPlaying = 0 as libc::c_int as UDWORD;
        (*psTrack).bCompressed = 0 as libc::c_int;
        // I didn't comment the below value out, so I guess NOT needed. -Q
		//
		// VagID;
		//
		// set global
        let ref mut fresh1 = *g_apTrack.offset(iTrack as isize);
        *fresh1 = psTrack;
        // increment current sound
        g_iCurTracks += 1;
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn sound_AddTrack(mut pTrack: *mut TRACK) -> BOOL {
    // add to sound array
    if g_iCurTracks < 600 as libc::c_int {
        // set pointer in table
        let ref mut fresh2 = *g_apTrack.offset(g_iCurTracks as isize);
        *fresh2 = pTrack;
        // increment current sound
        g_iCurTracks += 1;
        return 1 as libc::c_int
    } else {
        debug(LOG_ERROR,
              b"sound_AddTrack: all tracks used: increase MAX_TRACKS\n\x00" as
                  *const u8 as *const libc::c_char);
        abort();
    };
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn sound_LoadTrackFromBuffer(mut pBuffer:
                                                       *mut libc::c_char,
                                                   mut udwSize: UDWORD)
 -> *mut libc::c_void {
    //~~~~~~~~~~~~
    let mut pTrack: *mut TRACK = 0 as *mut TRACK;
    //~~~~~~~~~~~~
    // allocate track
    pTrack =
        memMallocRelease(::std::mem::size_of::<TRACK>() as libc::c_ulong) as
            *mut TRACK;
    memset(pTrack as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<TRACK>() as libc::c_ulong);
    if pTrack.is_null() {
        debug(LOG_ERROR,
              b"sound_LoadTrackFromBuffer: couldn\'t allocate memory\n\x00" as
                  *const u8 as *const libc::c_char);
        abort();
    } else {
        (*pTrack).bMemBuffer = 1 as libc::c_int;
        (*pTrack).pName =
            memMallocRelease(strlen(GetLastResourceFilename()).wrapping_add(1
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_uint))
                as *mut STRING;
        if (*pTrack).pName.is_null() {
            debug(LOG_ERROR,
                  b"sound_LoadTrackFromBuffer: couldn\'t allocate memory\n\x00"
                      as *const u8 as *const libc::c_char);
            abort();
        }
        strcpy((*pTrack).pName, GetLastResourceFilename());
        (*pTrack).resID = GetLastHashName();
        if sound_ReadTrackFromBuffer(pTrack, pBuffer as *mut libc::c_void,
                                     udwSize) == 0 as libc::c_int {
            return 0 as *mut libc::c_void
        } else { return pTrack as *mut libc::c_void }
    };
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn sound_LoadTrackFromFile(mut szFileName:
                                                     *mut libc::c_char)
 -> BOOL {
    //~~~~~~~~~~~~
    let mut pTrack: *mut TRACK = 0 as *mut TRACK;
    //~~~~~~~~~~~~
    // allocate track
    pTrack =
        memMallocRelease(::std::mem::size_of::<TRACK>() as libc::c_ulong) as
            *mut TRACK;
    if !pTrack.is_null() {
        (*pTrack).bMemBuffer = 0 as libc::c_int;
        (*pTrack).pName =
            memMallocRelease(strlen(szFileName).wrapping_add(1 as libc::c_int
                                                                 as
                                                                 libc::c_uint))
                as *mut STRING;
        if (*pTrack).pName.is_null() {
            debug(LOG_ERROR,
                  b"sound_LoadTrackFromFile: Out of memory\x00" as *const u8
                      as *const libc::c_char);
            abort();
        }
        strcpy((*pTrack).pName, szFileName);
        (*pTrack).resID = HashStringIgnoreCase(szFileName);
        if sound_ReadTrackFromFile(pTrack, szFileName) == 0 as libc::c_int {
            return 0 as libc::c_int
        }
        return sound_AddTrack(pTrack)
    }
    return 0 as libc::c_int;
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn sound_ReleaseTrack(mut psTrack: *mut TRACK) -> BOOL {
    //~~~~~~~~~~~
    let mut iTrack: SDWORD = 0;
    //~~~~~~~~~~~
    if !(*psTrack).pName.is_null() {
        memFreeRelease((*psTrack).pName as *mut libc::c_void);
        (*psTrack).pName = 0 as *mut STRING
    }
    iTrack = 0 as libc::c_int;
    while iTrack < g_iCurTracks {
        if *g_apTrack.offset(iTrack as isize) == psTrack {
            let ref mut fresh3 = *g_apTrack.offset(iTrack as isize);
            *fresh3 = 0 as *mut TRACK
        }
        iTrack += 1
    }
    sound_FreeTrack(psTrack);
    return 1 as libc::c_int;
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn sound_CheckAllUnloaded() {
    //~~~~~~~~~~~
    let mut iTrack: SDWORD = 0;
    //~~~~~~~~~~~
    iTrack = 0 as libc::c_int;
    while iTrack < 600 as libc::c_int {
        if (*g_apTrack.offset(iTrack as isize)).is_null() {
        } else {
            debug(LOG_ERROR,
                  b"sound_CheckAllUnloaded: check audio.cfg for duplicate IDs\n\x00"
                      as *const u8 as *const libc::c_char);
        };
        if (*g_apTrack.offset(iTrack as isize)).is_null() {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"track.c\x00" as *const u8 as *const libc::c_char,
                  359 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 23],
                                            &[libc::c_char; 23]>(b"sound_CheckAllUnloaded\x00")).as_ptr(),
                  b"g_apTrack[iTrack] == NULL\x00" as *const u8 as
                      *const libc::c_char);
        };
        iTrack += 1
    };
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn sound_TrackLooped(mut iTrack: SDWORD) -> BOOL {
    sound_CheckTrack(iTrack);
    return (**g_apTrack.offset(iTrack as isize)).bLoop;
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn sound_TrackAudibleRadius(mut iTrack: SDWORD)
 -> SDWORD {
    sound_CheckTrack(iTrack);
    return (**g_apTrack.offset(iTrack as isize)).iAudibleRadius;
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn sound_GetNumPlaying(mut iTrack: SDWORD) -> SDWORD {
    sound_CheckTrack(iTrack);
    return (**g_apTrack.offset(iTrack as isize)).iNumPlaying as SDWORD;
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn sound_CheckSample(mut psSample: *mut AUDIO_SAMPLE) {
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"sound_CheckSample: sample pointer invalid\n\x00" as *const u8
                  as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"track.c\x00" as *const u8 as *const libc::c_char,
              399 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 18],
                                        &[libc::c_char; 18]>(b"sound_CheckSample\x00")).as_ptr(),
              b"PTRVALID(psSample, sizeof(AUDIO_SAMPLE))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (*psSample).iSample >= 0 as libc::c_int as libc::c_uint ||
           (*psSample).iSample == -(1 as libc::c_int) as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"sound_CheckSample: sample %i out of range\n\x00" as *const u8
                  as *const libc::c_char, (*psSample).iSample);
    };
    if (*psSample).iSample >= 0 as libc::c_int as libc::c_uint ||
           (*psSample).iSample == -(1 as libc::c_int) as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"track.c\x00" as *const u8 as *const libc::c_char,
              400 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 18],
                                        &[libc::c_char; 18]>(b"sound_CheckSample\x00")).as_ptr(),
              b"psSample->iSample >= 0 || psSample->iSample == SAMPLE_NOT_ALLOCATED\x00"
                  as *const u8 as *const libc::c_char);
    };
    //
	// psSample;
	//
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn sound_CheckTrack(mut iTrack: SDWORD) -> BOOL {
    if iTrack < 0 as libc::c_int || iTrack > g_iCurTracks - 1 as libc::c_int {
        debug(LOG_NEVER,
              b"sound_CheckTrack: track number %i outside max %i\n\x00" as
                  *const u8 as *const libc::c_char, iTrack, g_iCurTracks);
        return 0 as libc::c_int
    }
    if (*g_apTrack.offset(iTrack as isize)).is_null() {
        debug(LOG_NEVER,
              b"sound_CheckTrack: track %i NULL\n\x00" as *const u8 as
                  *const libc::c_char, iTrack);
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn sound_GetTrackTime(mut iTrack: SDWORD) -> SDWORD {
    sound_CheckTrack(iTrack);
    return (**g_apTrack.offset(iTrack as isize)).iTime;
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn sound_GetTrackPriority(mut iTrack: SDWORD)
 -> SDWORD {
    sound_CheckTrack(iTrack);
    return (**g_apTrack.offset(iTrack as isize)).iPriority;
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn sound_GetTrackVolume(mut iTrack: SDWORD) -> SDWORD {
    sound_CheckTrack(iTrack);
    return (**g_apTrack.offset(iTrack as isize)).iVol;
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn sound_GetTrackAudibleRadius(mut iTrack: SDWORD)
 -> SDWORD {
    sound_CheckTrack(iTrack);
    return (**g_apTrack.offset(iTrack as isize)).iAudibleRadius;
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn sound_GetTrackName(mut iTrack: SDWORD)
 -> *mut libc::c_char {
    if iTrack == -(3 as libc::c_int) { return 0 as *mut libc::c_char }
    if !(*g_apTrack.offset(iTrack as isize)).is_null() {
    } else {
        debug(LOG_ERROR,
              b"sound_GetTrackName: unallocated track\x00" as *const u8 as
                  *const libc::c_char);
    };
    if !(*g_apTrack.offset(iTrack as isize)).is_null() {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"track.c\x00" as *const u8 as *const libc::c_char,
              475 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 19],
                                        &[libc::c_char; 19]>(b"sound_GetTrackName\x00")).as_ptr(),
              b"g_apTrack[iTrack] != NULL\x00" as *const u8 as
                  *const libc::c_char);
    };
    return if !(*g_apTrack.offset(iTrack as isize)).is_null() {
               (**g_apTrack.offset(iTrack as isize)).pName
           } else { b"unallocated\x00" as *const u8 as *const libc::c_char }
               as *mut libc::c_char;
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn sound_GetTrackHashName(mut iTrack: SDWORD)
 -> UDWORD {
    if iTrack == 0 as libc::c_int || iTrack == -(3 as libc::c_int) {
        return 0 as libc::c_int as UDWORD
    }
    if !(*g_apTrack.offset(iTrack as isize)).is_null() {
    } else {
        debug(LOG_ERROR,
              b"sound_GetTrackName: unallocated track\x00" as *const u8 as
                  *const libc::c_char);
    };
    if !(*g_apTrack.offset(iTrack as isize)).is_null() {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"track.c\x00" as *const u8 as *const libc::c_char,
              487 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 23],
                                        &[libc::c_char; 23]>(b"sound_GetTrackHashName\x00")).as_ptr(),
              b"g_apTrack[iTrack] != NULL\x00" as *const u8 as
                  *const libc::c_char);
    };
    return if !(*g_apTrack.offset(iTrack as isize)).is_null() {
               (**g_apTrack.offset(iTrack as isize)).resID
           } else { 0 as libc::c_int as libc::c_uint };
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn sound_Play2DTrack(mut psSample: *mut AUDIO_SAMPLE,
                                           mut bQueued: BOOL) -> BOOL {
    //~~~~~~~~~~~~~
    let mut psTrack: *mut TRACK = 0 as *mut TRACK;
    //~~~~~~~~~~~~~
    psTrack = *g_apTrack.offset((*psSample).iTrack as isize);
    if psTrack.is_null() { return 0 as libc::c_int }
    /*	// check only playing compressed audio on queue channel
#ifdef USE_COMPRESSED_SPEECH
	if ( bQueued && !psTrack->bCompressed )
	{
		DBPRINTF( ("sound_PlayTrack: trying to play uncompressed speech %s!\n", psTrack->pName) );
		return FALSE;
	}

	if ( !bQueued && psTrack->bCompressed )
	{
		DBPRINTF( ("sound_PlayTrack: trying to play compressed audio %s!\n", psTrack->pName) );
		return FALSE;
	}

#else
	if ( psTrack->bCompressed )
	{
		DBPRINTF( ("sound_PlayTrack: trying to play compressed speech %s!\n", psTrack->pName) );
		return FALSE;
	}
#endif
*/
    return sound_Play2DSample(psTrack, psSample, bQueued);
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn sound_Play3DTrack(mut psSample: *mut AUDIO_SAMPLE)
 -> BOOL {
    //~~~~~~~~~~~~~
    let mut psTrack: *mut TRACK = 0 as *mut TRACK;
    //~~~~~~~~~~~~~
    psTrack = *g_apTrack.offset((*psSample).iTrack as isize);
    /*	if ( psTrack->bCompressed )
	{
		DBPRINTF( ("sound_PlayTrack: trying to play compressed audio %s!\n", psTrack->pName) );
		return FALSE;
	}
*/
    return sound_Play3DSample(psTrack, psSample);
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn sound_StopTrack(mut psSample: *mut AUDIO_SAMPLE) {
    sound_CheckSample(psSample);
    if (*psSample).iSample != -(1 as libc::c_int) as libc::c_uint {
        sound_StopSample((*psSample).iSample);
    }
    // do stopped callback
    if g_pStopTrackCallback.is_some() && !(*psSample).psObj.is_null() {
        g_pStopTrackCallback.expect("non-null function pointer")(psSample);
    }
    // update number of samples playing
    g_iSamples -= 1;
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn sound_PauseTrack(mut psSample: *mut AUDIO_SAMPLE) {
    if (*psSample).iSample != -(1 as libc::c_int) as libc::c_uint {
        sound_StopSample((*psSample).iSample);
    };
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn sound_FinishedCallback(mut psSample:
                                                    *mut AUDIO_SAMPLE) {
    sound_CheckSample(psSample);
    if !(*g_apTrack.offset((*psSample).iTrack as isize)).is_null() {
        (**g_apTrack.offset((*psSample).iTrack as isize)).iTimeLastFinished =
            sound_GetGameTime()
    }
    // call user callback if specified
    if (*psSample).pCallback.is_some() {
        (*psSample).pCallback.expect("non-null function pointer")(psSample);
    }
    // set remove flag
    (*psSample).bRemove = 1 as libc::c_int;
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn sound_GetTrackID(mut psTrack: *mut TRACK) -> SDWORD {
    //~~~~~~~~~~
    let mut i: SDWORD = 0 as libc::c_int;
    //~~~~~~~~~~
    // find matching track
    i = 0 as libc::c_int;
    while i < 600 as libc::c_int {
        if !(*g_apTrack.offset(i as isize)).is_null() &&
               *g_apTrack.offset(i as isize) == psTrack {
            break ;
        }
        i += 1
    }
    // if matching track found return it else find empty track
    if i < 600 as libc::c_int {
        return i
    } else { return -(3 as libc::c_int) };
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn sound_GetAvailableID() -> SDWORD {
    //~~~~~~
    let mut i: SDWORD = 0;
    //~~~~~~
    i = 0 as libc::c_int;
    while i < 600 as libc::c_int {
        if (*g_apTrack.offset(i as isize)).is_null() { break ; }
        i += 1
    }
    if i < 600 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"sound_GetTrackID: unused track not found!\n\x00" as *const u8
                  as *const libc::c_char);
    };
    if i < 600 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"track.c\x00" as *const u8 as *const libc::c_char,
              654 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 21],
                                        &[libc::c_char; 21]>(b"sound_GetAvailableID\x00")).as_ptr(),
              b"i < MAX_TRACKS\x00" as *const u8 as *const libc::c_char);
    };
    if i < 600 as libc::c_int {
        return i
    } else { return -(1 as libc::c_int) };
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn sound_SetStoppedCallback(mut pStopTrackCallback:
                                                      AUDIO_CALLBACK) {
    g_pStopTrackCallback = pStopTrackCallback;
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn sound_GetTrackTimeLastFinished(mut iTrack: SDWORD)
 -> UDWORD {
    sound_CheckTrack(iTrack);
    return (**g_apTrack.offset(iTrack as isize)).iTimeLastFinished;
}
/* **************************************************************************/
/* functions
 */
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn sound_SetTrackTimeLastFinished(mut iTrack: SDWORD,
                                                        mut iTime: UDWORD) {
    sound_CheckTrack(iTrack);
    (**g_apTrack.offset(iTrack as isize)).iTimeLastFinished = iTime;
}
//*
//
