use ::libc;
extern "C" {
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
    /* special: sets all to on */
    /* special; on by default */
    /* if too verbose for anything but dedicated debugging... */
    /* _must_ be last! */
    // / Function which does the output
    // / Setup function
    // / Cleaning function
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
}
pub type BOOL = libc::c_int;
pub type DWORD = libc::c_int;
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
pub struct NETMSG {
    pub size: libc::c_ushort,
    pub paddedBytes: libc::c_uchar,
    pub type_0: libc::c_uchar,
    pub destination: libc::c_uchar,
    pub body: [libc::c_char; 8000],
}
/*
 * NetAudio.c
 *
 * Internet audio. Team communication using microphone etc.. Currently
 * just an unimplemented skeleton.
 */
// ////////////////////////////////////////////////////////////////////////
#[no_mangle]
pub unsafe extern "C" fn NETinitAudioCapture() -> BOOL {
    debug(LOG_SOUND,
          b"NETinitAudioCapture\x00" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////
#[no_mangle]
pub unsafe extern "C" fn NETshutdownAudioCapture() -> BOOL {
    debug(LOG_SOUND,
          b"NETshutdownAudioCapture\x00" as *const u8 as *const libc::c_char);
    return 1 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////
#[no_mangle]
pub unsafe extern "C" fn NETstartAudioCapture() -> BOOL {
    debug(LOG_SOUND,
          b"NETstartAudioCapture\x00" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}
//capture
// ////////////////////////////////////////////////////////////////////////
#[no_mangle]
pub unsafe extern "C" fn NETstopAudioCapture() -> BOOL {
    debug(LOG_SOUND,
          b"NETstopAudioCapture\x00" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}
// check for spectator status.
// from net audio.
// ////////////////////////////////////////////////////////////////////////
// update the pointers and process the buffer accordingly.
#[no_mangle]
pub unsafe extern "C" fn NETprocessAudioCapture() -> BOOL {
    debug(LOG_SOUND,
          b"NETprocessAudioCapture\x00" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////
#[no_mangle]
pub unsafe extern "C" fn NETinitPlaybackBuffer(mut pSoundBuffer:
                                                   *mut libc::c_void)
 -> BOOL {
    debug(LOG_SOUND,
          b"NETinitPlaybackBuffer\x00" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////
// handle the playback buffer.
#[no_mangle]
pub unsafe extern "C" fn NETqueueIncomingAudio(mut pSoundData:
                                                   *mut libc::c_void,
                                               mut soundBytes: DWORD,
                                               mut bStream: BOOL) -> BOOL {
    debug(LOG_SOUND,
          b"NETqueueIncomingAudio\x00" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}
// playback
// ////////////////////////////////////////////////////////////////////////
// Handle a incoming message that needs to be played
#[no_mangle]
pub unsafe extern "C" fn NETplayIncomingAudio(mut pMsg: *mut NETMSG) {
    debug(LOG_SOUND,
          b"NETplayIncomingAudio\x00" as *const u8 as *const libc::c_char);
}
// ////////////////////////////////////////////////////////////////////////
// close it all down
#[no_mangle]
pub unsafe extern "C" fn NETshutdownAudioPlayback() -> BOOL {
    debug(LOG_SOUND,
          b"NETshutdownAudioPlayback\x00" as *const u8 as
              *const libc::c_char);
    return 1 as libc::c_int;
}
