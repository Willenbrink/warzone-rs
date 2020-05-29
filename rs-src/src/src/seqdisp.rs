use ::libc;
extern "C" {
    #[no_mangle]
    fn PHYSFS_exists(fname: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn memFreeRelease(pMemToFree: *mut libc::c_void);
    #[no_mangle]
    fn memMallocRelease(Size: size_t) -> *mut libc::c_void;
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
    #[no_mangle]
    fn loadFileToBufferNoError(pFileName: *mut libc::c_char,
                               pFileBuffer: *mut libc::c_char,
                               bufferSize: UDWORD, pSize: *mut UDWORD)
     -> BOOL;
    #[no_mangle]
    fn GetTickCount() -> DWORD;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_uint;
    #[no_mangle]
    fn strtok(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_uint)
     -> *mut libc::c_void;
    #[no_mangle]
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn pal_GetNearestColour(r: uint8, g: uint8, b: uint8) -> uint8;
    #[no_mangle]
    fn iV_SetFont(FontID: libc::c_int);
    #[no_mangle]
    fn iV_GetTextLineSize() -> libc::c_int;
    #[no_mangle]
    fn iV_GetTextWidth(String: *mut STRING) -> libc::c_int;
    #[no_mangle]
    fn iV_SetTextColour(Index: SWORD);
    #[no_mangle]
    fn pie_DrawTextToSurface(String: *mut STRING, XPos: libc::c_int,
                             YPos: libc::c_int);
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
    fn seq_SetSequenceForBuffer(filename: *mut libc::c_char,
                                startTime: libc::c_int, perfMode_0: PERF_MODE)
     -> BOOL;
    #[no_mangle]
    fn seq_RenderOneFrameToBuffer(lpSF: *mut libc::c_char, skip: libc::c_int,
                                  boxMin: SDWORD, boxMax: SDWORD)
     -> libc::c_int;
    //directX fullscreeen render uses local buffer to store previous frame data
    #[no_mangle]
    fn seq_SetSequence(filename: *mut libc::c_char, startTime: libc::c_int,
                       lpBF: *mut libc::c_char, perfMode_0: PERF_MODE)
     -> BOOL;
    #[no_mangle]
    fn seq_RenderOneFrame(skip: libc::c_int, boxMin: SDWORD, boxMax: SDWORD)
     -> libc::c_int;
    #[no_mangle]
    fn seq_ClearMovie() -> libc::c_int;
    //setup monitoring and control
    #[no_mangle]
    fn seq_RefreshVideoBuffers() -> BOOL;
    #[no_mangle]
    fn seq_ShutDown() -> BOOL;
    #[no_mangle]
    fn seq_GetFrameSize(pWidth: *mut SDWORD, pHeight: *mut SDWORD) -> BOOL;
    #[no_mangle]
    fn seq_GetCurrentFrame() -> libc::c_int;
    #[no_mangle]
    fn seq_GetFrameTimeInClicks() -> libc::c_int;
    #[no_mangle]
    fn loop_SetVideoPlaybackMode();
    #[no_mangle]
    fn loop_ClearVideoPlaybackMode();
    #[no_mangle]
    fn loop_GetVideoMode() -> SDWORD;
    /* **************************************************************************/
/*
 * piefunc.h
 *
 * type defines for extended image library functions.
 *
 */
/* **************************************************************************/
    /* **************************************************************************/
    /* **************************************************************************/
/*
 *	Global Definitions
 */
/* **************************************************************************/
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
    #[no_mangle]
    fn pie_DownLoadBufferToScreen(srcData: *mut libc::c_void, destX: UDWORD,
                                  destY: UDWORD, srcWidth: UDWORD,
                                  srcHeight: UDWORD, srcStride: UDWORD);
    /* **************************************************************************/
    /* **************************************************************************/
    /* **************************************************************************/
    #[no_mangle]
    fn audio_PlayStream(szFileName: *mut libc::c_char, iVol: SDWORD,
                        pUserCallback: AUDIO_CALLBACK) -> BOOL;
    #[no_mangle]
    static mut displayBufferSize: UDWORD;
    #[no_mangle]
    static mut DisplayBuffer: *mut libc::c_char;
    #[no_mangle]
    fn intAddReticule() -> BOOL;
    /* **************************************************************************/
/*
 * pieclip.h
 *
 * clipping for all pumpkin image library functions.
 *
 */
/* **************************************************************************/
    /* **************************************************************************/
    /* **************************************************************************/
/*
 *	Global Definitions
 */
/* **************************************************************************/
    /* **************************************************************************/
/*
 *	Global ProtoTypes
 */
/* **************************************************************************/
    #[no_mangle]
    fn pie_GetVideoBufferHeight() -> UDWORD;
    #[no_mangle]
    fn pie_GetVideoBufferWidth() -> UDWORD;
    #[no_mangle]
    static mut WFont: libc::c_int;
    #[no_mangle]
    fn cdAudio_Pause() -> BOOL;
    #[no_mangle]
    fn war_GetSeqMode() -> SEQ_MODE;
    #[no_mangle]
    fn screen_StopBackDrop();
    // reset the event system
    // Initialise the create/release function array - specify the maximum value type
    // a create function for data stored in an INTERP_VAL
    // a release function for data stored in an INTERP_VAL
    // Add a new value create function
    // Add a new value release function
    // Create a new context for a script
    // Copy a context, including variable values
    // Add a new object to the trigger system
// Time is the application time at which all the triggers are to be started
    // Remove a context from the event system
    // Set a global variable value for a context
    // Get the value pointer for a variable index
    // Process all the currently active triggers
// Time is the application time at which all the triggers are to be processed
    // Activate a callback trigger
    #[no_mangle]
    fn eventFireCallbackTrigger(callback: TRIGGER_TYPE);
    /*resets the pause states */
    #[no_mangle]
    fn resetDesignPauseState();
    #[no_mangle]
    fn displayGameOver(success: BOOL) -> BOOL;
    #[no_mangle]
    fn getScriptWinLoseVideo() -> UBYTE;
}
pub type size_t = libc::c_uint;
pub type UBYTE = libc::c_uchar;
pub type STRING = libc::c_char;
pub type UWORD = libc::c_ushort;
pub type SWORD = libc::c_short;
pub type UDWORD = libc::c_uint;
pub type SDWORD = libc::c_int;
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
pub type int32 = libc::c_int;
pub type uint8 = libc::c_uchar;
pub type uint32 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iClip {
    pub left: libc::c_int,
    pub top: libc::c_int,
    pub right: libc::c_int,
    pub bottom: libc::c_int,
}
/* **************************************************************************/
/*
 * ivisdef.h
 *
 * type defines for all ivis library functions.
 *
 */
/* **************************************************************************/
/* **************************************************************************/
/* **************************************************************************/
/*
 *	Global Definitions
 */
/* **************************************************************************/
// now defined for all versions (optional BSP handled on all formats)
//Not really needed I guess, however, see debug.c comments.  -Qamly
// texture animation defines
/* **************************************************************************/
/*
 *	Global Macros
 */
/* **************************************************************************/
/* **************************************************************************/
/*
 *	Global Type Definitions
 */
/* **************************************************************************/
//*************************************************************************
//
// Basic types (now defined in pieTypes.h)
//
//*************************************************************************
//*************************************************************************
//
// Simple derived types (now defined in pieTypes.h)
//
//*************************************************************************
//*************************************************************************
//
// screen surface structure
//
//*************************************************************************
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iSurface {
    pub usr: libc::c_int,
    pub flags: uint32,
    pub xcentre: libc::c_int,
    pub ycentre: libc::c_int,
    pub xpshift: libc::c_int,
    pub ypshift: libc::c_int,
    pub clip: iClip,
    pub buffer: *mut uint8,
    pub scantable: [int32; 1024],
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub size: int32,
}
pub type CLEAR_MODE = libc::c_uint;
pub const CLEAR_FOG: CLEAR_MODE = 3;
pub const CLEAR_BLACK: CLEAR_MODE = 2;
pub const CLEAR_OFF_AND_NO_BUFFER_DOWNLOAD: CLEAR_MODE = 1;
pub const CLEAR_OFF: CLEAR_MODE = 0;
pub type PERF_MODE = _perf_mode;
pub type _perf_mode = libc::c_uint;
pub const VIDEO_PERF_SKIP_FRAMES: _perf_mode = 2;
pub const VIDEO_PERF_WINDOW: _perf_mode = 1;
pub const VIDEO_PERF_FULLSCREEN: _perf_mode = 0;
// Normal alternate line video
// 320 240 centred
// 320 240 centred and display every 4th frame
/* **************************************************************************/
/*
 * sequence.h
 *
 * video streaming to game surfaces.
 *
 */
/* **************************************************************************/
/* **************************************************************************/
/* **************************************************************************/
/*
 *	Global Definitions
 */
/* **************************************************************************/
pub type VIDEO_MODE = _video_mode;
pub type _video_mode = libc::c_uint;
pub const VIDEO_3DFX_FULLSCREEN: _video_mode = 5;
pub const VIDEO_3DFX_WINDOW: _video_mode = 4;
pub const VIDEO_D3D_FULLSCREEN: _video_mode = 3;
pub const VIDEO_D3D_WINDOW: _video_mode = 2;
pub const VIDEO_SOFT_FULLSCREEN: _video_mode = 1;
pub const VIDEO_SOFT_WINDOW: _video_mode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SEQLIST {
    pub pSeq: *mut libc::c_char,
    pub pAudio: *mut libc::c_char,
    pub bSeqLoop: BOOL,
    pub currentText: SDWORD,
    pub aText: [SEQTEXT; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SEQTEXT {
    pub pText: [libc::c_char; 256],
    pub x: SDWORD,
    pub y: SDWORD,
    pub startFrame: SDWORD,
    pub endFrame: SDWORD,
    pub bSubtitle: BOOL,
}
// video size 16bit rgb 555 mode (convert to 8bit from the buffer)
// directX 640 * 480 16bit rgb mode render through local buffer to back buffer
// video 16bit screen pixel mode
// 640 * 480 screen pixel mode
// video 16bit BGR 565 mode
// 640 * 480 BGR 565 mode
/*
 * Interp.h
 *
 * Script interpreter definitions
 */
/* The possible value types for scripts */
// Basic types
//	VAL_FLOAT,
// events and triggers
/* Type used by the compiler for functions that do not return a value */
// user defined types should start with this id
// flag to specify a variable reference rather than simple value
/* A value consists of its type and value */
// VAL_BOOL
// VAL_INT
//		float		fval;		// VAL_FLOAT
// VAL_STRING
// VAL_OBJECT
// VAL_VOIDPTR
// maximum number of equivalent types for a type
// type equivalences
// the type that the others are equivalent to
// number of equivalent types
// the equivalent types
/* Opcodes for the script interpreter */
// Push value onto stack
// Push a pointer to a variable onto the stack
// Pop value from stack
// Push the value of a global variable onto the stack
// Pop a value from the stack into a global variable
// Push the value of a global array variable onto the stack
// Pop a value from the stack into a global array variable
// Call the 'C' function pointed to by the next value
// Call the variable access 'C' function pointed to by the next value
// Jump to a different location in the script
// Jump if the top stack value is true
// Jump if the top stack value is false
// Call a binary maths/boolean operator
// Call a unary maths/boolean operator
// End the program
// temporarily pause the current event
// The following operations are secondary data to OP_BINARYOP and OP_UNARYOP
// Maths operators
// Boolean operators
//String cancatenation
// Comparison operators
//custom (in-script) function call
//local var
//variable of object type (pointer)
/* How far the opcode is shifted up a UDWORD to allow other data to be
 * stored in the same UDWORD
 */
// maximum sizes for arrays
/* The mask for the number of array elements stored in the data part of an opcode */
/* The type of function called by an OP_CALL */
/* The type of function called to access an object or in-game variable */
/* The possible storage types for a variable */
// Public variable
// Private variable
// A value stored in an objects data space.
// An external value accessed by function call
// A local variable
/* Variable debugging info for a script */
/* Array info for a script */
// the base index of the array values
// the array data type
/* Array debug info for a script */
/* Line debugging information for a script */
// Offset in the compiled script that corresponds to
// this line in the original script.
// the trigger/event that starts at this line
/* Different types of triggers */
pub type TRIGGER_TYPE = _trigger_type;
pub type _trigger_type = libc::c_uint;
pub const TR_CALLBACKSTART: _trigger_type = 5;
pub const TR_PAUSE: _trigger_type = 4;
pub const TR_EVERY: _trigger_type = 3;
pub const TR_WAIT: _trigger_type = 2;
pub const TR_CODE: _trigger_type = 1;
pub const TR_INIT: _trigger_type = 0;
pub const CALL_VIDEO_QUIT: _scr_callback_types = 15;
// Trigger fires when the script is first run
// Trigger uses script code
// Trigger after a time pause
// Trigger at repeated intervals
// Event has paused for an interval and will restart in the middle of it's code
// The user defined callback triggers should start with this id
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
/* **************************************************************************/
/* **************************************************************************/
/* enums */
/* **************************************************************************/
/* forward definitions
 */
/* **************************************************************************/
/* typedefs
 */
pub type AUDIO_CALLBACK
    =
    Option<unsafe extern "C" fn(_: *mut AUDIO_SAMPLE) -> BOOL>;
pub type ALuint = libc::c_uint;
pub const SEQ_FULL: SEQ_MODE = 0;
pub const SEQ_SKIP: SEQ_MODE = 2;
pub const SEQ_SMALL: SEQ_MODE = 1;
pub type SEQ_MODE = libc::c_uint;
pub type _scr_callback_types = libc::c_uint;
pub const CALL_BEACON: _scr_callback_types = 62;
pub const CALL_TRANSPORTER_LANDED_B: _scr_callback_types = 61;
pub const CALL_STRUCTBUILT: _scr_callback_types = 60;
pub const CALL_AI_MSG: _scr_callback_types = 59;
pub const CALL_CONSOLE: _scr_callback_types = 58;
pub const CALL_ALLIANCEOFFER: _scr_callback_types = 57;
pub const CALL_PLAYERLEFT: _scr_callback_types = 56;
pub const CALL_UNITTAKEOVER: _scr_callback_types = 55;
pub const CALL_VTOL_OFF_MAP: _scr_callback_types = 54;
pub const CALL_CLUSTER_EMPTY: _scr_callback_types = 53;
pub const CALL_NO_REINFORCEMENTS_LEFT: _scr_callback_types = 52;
pub const CALL_ALL_ONSCREEN_DROIDS_SELECTED: _scr_callback_types = 51;
pub const CALL_TRANSPORTER_LANDED: _scr_callback_types = 50;
pub const CALL_TRANSPORTER_OFFMAP: _scr_callback_types = 49;
pub const CALL_OBJECTCLOSE: _scr_callback_types = 48;
pub const CALL_OBJECTOPEN: _scr_callback_types = 47;
pub const CALL_FEATURE_DESTROYED: _scr_callback_types = 46;
pub const CALL_DROID_DESTROYED: _scr_callback_types = 45;
pub const CALL_STRUCT_DESTROYED: _scr_callback_types = 44;
pub const CALL_OBJ_DESTROYED: _scr_callback_types = 43;
pub const CALL_OBJ_SEEN: _scr_callback_types = 42;
pub const CALL_FEATURE_SEEN: _scr_callback_types = 41;
pub const CALL_DROID_SEEN: _scr_callback_types = 40;
pub const CALL_STRUCT_SEEN: _scr_callback_types = 39;
pub const CALL_ATTACKED: _scr_callback_types = 38;
pub const CALL_DROID_ATTACKED: _scr_callback_types = 37;
pub const CALL_STRUCT_ATTACKED: _scr_callback_types = 36;
pub const CALL_NEWDROID: _scr_callback_types = 35;
pub const CALL_RESEARCHCOMPLETED: _scr_callback_types = 34;
pub const CALL_DESIGN_PROPULSION: _scr_callback_types = 33;
pub const CALL_DESIGN_BODY: _scr_callback_types = 32;
pub const CALL_DESIGN_COMMAND: _scr_callback_types = 31;
pub const CALL_DESIGN_SYSTEM: _scr_callback_types = 30;
pub const CALL_DESIGN_WEAPON: _scr_callback_types = 29;
pub const CALL_DESIGN_QUIT: _scr_callback_types = 28;
pub const CALL_DROID_SELECTED: _scr_callback_types = 27;
pub const CALL_BUTTON_PRESSED: _scr_callback_types = 26;
pub const CALL_MANULIST: _scr_callback_types = 25;
pub const CALL_MANURUN: _scr_callback_types = 24;
pub const CALL_RESEARCHLIST: _scr_callback_types = 23;
pub const CALL_BUILDGRID: _scr_callback_types = 22;
pub const CALL_BUILDLIST: _scr_callback_types = 21;
pub const CALL_ELECTRONIC_TAKEOVER: _scr_callback_types = 20;
pub const CALL_MISSION_TIME: _scr_callback_types = 19;
pub const CALL_TRANSPORTER_REINFORCE: _scr_callback_types = 18;
pub const CALL_START_NEXT_LEVEL: _scr_callback_types = 17;
pub const CALL_LAUNCH_TRANSPORTER: _scr_callback_types = 16;
pub const CALL_MISSION_END: _scr_callback_types = 14;
pub const CALL_MISSION_START: _scr_callback_types = 13;
pub const CALL_FACTORY_BUILT: _scr_callback_types = 12;
pub const CALL_RESEARCH_BUILT: _scr_callback_types = 11;
pub const CALL_RESEX_BUILT: _scr_callback_types = 10;
pub const CALL_POWERGEN_BUILT: _scr_callback_types = 9;
pub const CALL_DROIDBUILT: _scr_callback_types = 8;
pub const CALL_DROIDDESIGNED: _scr_callback_types = 7;
pub const CALL_DELIVPOINTMOVED: _scr_callback_types = 6;
pub const CALL_GAMEINIT: _scr_callback_types = 5;
/* **************************************************************************/
/*
 *	local Variables
 */
/* **************************************************************************/
static mut bBackDropWasAlreadyUp: BOOL = 0;
#[no_mangle]
pub static mut bSeqInit: BOOL = 0 as libc::c_int;
#[no_mangle]
pub static mut bSeqPlaying: BOOL = 0 as libc::c_int;
#[no_mangle]
pub static mut bAudioPlaying: BOOL = 0 as libc::c_int;
#[no_mangle]
pub static mut bHoldSeqForAudio: BOOL = 0 as libc::c_int;
#[no_mangle]
pub static mut bSeq8Bit: BOOL = 1 as libc::c_int;
#[no_mangle]
pub static mut bCDPath: BOOL = 0 as libc::c_int;
#[no_mangle]
pub static mut bHardPath: BOOL = 0 as libc::c_int;
#[no_mangle]
pub static mut bSeqSubtitles: BOOL = 1 as libc::c_int;
#[no_mangle]
pub static mut aCDPath: [libc::c_char; 256] = [0; 256];
#[no_mangle]
pub static mut aHardPath: [libc::c_char; 256] = [0; 256];
#[no_mangle]
pub static mut aVideoName: [libc::c_char; 256] = [0; 256];
#[no_mangle]
pub static mut aAudioName: [libc::c_char; 256] = [0; 256];
#[no_mangle]
pub static mut aTextName: [libc::c_char; 256] = [0; 256];
#[no_mangle]
pub static mut aSubtitleName: [libc::c_char; 256] = [0; 256];
#[no_mangle]
pub static mut pVideoBuffer: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut p3DFXVideoBuffer: *mut UWORD = 0 as *const UWORD as *mut UWORD;
#[no_mangle]
pub static mut pVideoPalette: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut videoMode: VIDEO_MODE = VIDEO_SOFT_WINDOW;
#[no_mangle]
pub static mut perfMode: PERF_MODE = VIDEO_PERF_FULLSCREEN;
static mut frameSkip: SDWORD = 1 as libc::c_int;
#[no_mangle]
pub static mut aSeqList: [SEQLIST; 6] =
    [SEQLIST{pSeq: 0 as *const libc::c_char as *mut libc::c_char,
             pAudio: 0 as *const libc::c_char as *mut libc::c_char,
             bSeqLoop: 0,
             currentText: 0,
             aText:
                 [SEQTEXT{pText: [0; 256],
                          x: 0,
                          y: 0,
                          startFrame: 0,
                          endFrame: 0,
                          bSubtitle: 0,}; 32],}; 6];
static mut currentSeq: SDWORD = -(1 as libc::c_int);
static mut currentPlaySeq: SDWORD = -(1 as libc::c_int);
static mut frameDuration: SDWORD = 40 as libc::c_int;
static mut g_bResumeInGame: BOOL = 0 as libc::c_int;
static mut videoFrameTime: libc::c_int = 0 as libc::c_int;
static mut frame: SDWORD = 0 as libc::c_int;
//justify if less than 520/600
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
//buffer render
/* **************************************************************************/
/*
 *	Source
 */
/* **************************************************************************/
/* Renders a video sequence specified by filename to a buffer*/
#[no_mangle]
pub unsafe extern "C" fn seq_RenderVideoToBuffer(mut pSurface: *mut iSurface,
                                                 mut sequenceName:
                                                     *mut libc::c_char,
                                                 mut time: libc::c_int,
                                                 mut seqCommand: libc::c_int)
 -> BOOL {
    let mut frameLag: SDWORD = 0;
    let mut videoTime: libc::c_int = 0;
    let mut state: BOOL = 1 as libc::c_int;
    if seqCommand == 3 as libc::c_int {
        //stop the movie
        seq_ShutDown();
        bSeqPlaying = 0 as libc::c_int;
        //		bSeqInit = FALSE;
		//return seq_ReleaseVideoBuffers();
        return 1 as libc::c_int
    }
    (bSeqInit) == 0;
    if bSeqPlaying == 0 {
        //set a valid video path if there is one
        if bCDPath == 0 && bHardPath == 0 { seq_SetVideoPath(); }
        if bHardPath != 0 {
            //jps 18 feb 99		seq_RenderOneFrameToBuffer(pVideoBuffer, 1);//skip frame if behind
            //use this first
            if strlen(sequenceName).wrapping_add(strlen(aHardPath.as_mut_ptr()))
                   < 256 as libc::c_int as libc::c_uint {
            } else {
                debug(LOG_ERROR,
                      b"sequence path+name greater than max string\x00" as
                          *const u8 as *const libc::c_char);
            };
            if strlen(sequenceName).wrapping_add(strlen(aHardPath.as_mut_ptr()))
                   < 256 as libc::c_int as libc::c_uint {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"seqdisp.c\x00" as *const u8 as *const libc::c_char,
                      161 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 24],
                                                &[libc::c_char; 24]>(b"seq_RenderVideoToBuffer\x00")).as_ptr(),
                      b"(strlen(sequenceName) + strlen(aHardPath))<MAX_STR_LENGTH\x00"
                          as *const u8 as *const libc::c_char);
            };
            strcpy(aVideoName.as_mut_ptr(), aHardPath.as_mut_ptr());
            strcat(aVideoName.as_mut_ptr(), sequenceName);
            // check it exists. If not then try CD.
            if PHYSFS_exists(aVideoName.as_mut_ptr()) == 0 && bCDPath != 0 {
                if strlen(sequenceName).wrapping_add(strlen(aCDPath.as_mut_ptr()))
                       < 256 as libc::c_int as libc::c_uint {
                } else {
                    debug(LOG_ERROR,
                          b"sequence path+name greater than max string\x00" as
                              *const u8 as *const libc::c_char);
                };
                if strlen(sequenceName).wrapping_add(strlen(aCDPath.as_mut_ptr()))
                       < 256 as libc::c_int as libc::c_uint {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"seqdisp.c\x00" as *const u8 as
                              *const libc::c_char, 168 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 24],
                                                    &[libc::c_char; 24]>(b"seq_RenderVideoToBuffer\x00")).as_ptr(),
                          b"(strlen(sequenceName) + strlen(aCDPath))<MAX_STR_LENGTH\x00"
                              as *const u8 as *const libc::c_char);
                };
                strcpy(aVideoName.as_mut_ptr(), aCDPath.as_mut_ptr());
                strcat(aVideoName.as_mut_ptr(), sequenceName);
            }
        } else if bCDPath != 0 {
            if strlen(sequenceName).wrapping_add(strlen(aCDPath.as_mut_ptr()))
                   < 256 as libc::c_int as libc::c_uint {
            } else {
                debug(LOG_ERROR,
                      b"sequence path+name greater than max string\x00" as
                          *const u8 as *const libc::c_char);
            };
            if strlen(sequenceName).wrapping_add(strlen(aCDPath.as_mut_ptr()))
                   < 256 as libc::c_int as libc::c_uint {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"seqdisp.c\x00" as *const u8 as *const libc::c_char,
                      175 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 24],
                                                &[libc::c_char; 24]>(b"seq_RenderVideoToBuffer\x00")).as_ptr(),
                      b"(strlen(sequenceName) + strlen(aCDPath))<MAX_STR_LENGTH\x00"
                          as *const u8 as *const libc::c_char);
            };
            strcpy(aVideoName.as_mut_ptr(), aCDPath.as_mut_ptr());
            strcat(aVideoName.as_mut_ptr(), sequenceName);
        } else {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"seq_StartFullScreenVideo: sequence path not found\x00"
                          as *const u8 as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"seqdisp.c\x00" as *const u8 as *const libc::c_char,
                      181 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 24],
                                                &[libc::c_char; 24]>(b"seq_RenderVideoToBuffer\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            return 0 as libc::c_int
        }
        iV_SetFont(WFont);
        iV_SetTextColour(-(1 as libc::c_int) as SWORD);
        videoMode = VIDEO_D3D_WINDOW;
        frame = 0 as libc::c_int;
        videoFrameTime = GetTickCount();
        bSeqPlaying =
            seq_SetSequenceForBuffer(aVideoName.as_mut_ptr(), videoFrameTime,
                                     perfMode);
        if bSeqPlaying == 0 as libc::c_int {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"seq_RenderVideoToBuffer: unable to initialise sequence %s\x00"
                          as *const u8 as *const libc::c_char,
                      aVideoName.as_mut_ptr());
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"seqdisp.c\x00" as *const u8 as *const libc::c_char,
                      198 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 24],
                                                &[libc::c_char; 24]>(b"seq_RenderVideoToBuffer\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            return 0 as libc::c_int
        }
        bSeqPlaying = 1 as libc::c_int;
        frameDuration = seq_GetFrameTimeInClicks()
    }
    if frame != -(1 as libc::c_int) {
        //for new timing
        //new call with timing
		//poll the sequence player while timing the video
        videoTime = GetTickCount();
        while videoTime < videoFrameTime + frameDuration {
            seq_RefreshVideoBuffers();
            videoTime = GetTickCount()
        }
        frameLag = videoTime - videoFrameTime;
        //skip frame if behind
        //old call
//		frame = seq_RenderOneFrameToBuffer(pVideoBuffer, 1);
        frameLag /=
            frameDuration; // if were running slow frame lag will be greater than 1
        videoFrameTime +=
            frameLag *
                frameDuration; //frame Lag should be 1 (most of the time)
        frame =
            seq_RenderOneFrameToBuffer(pVideoBuffer, frameLag,
                                       2 as libc::c_int, 0 as libc::c_int)
    }
    if frame == -(1 as libc::c_int) {
        if seqCommand == 1 as libc::c_int {
            bSeqPlaying = 0 as libc::c_int;
            state = 1 as libc::c_int
        }
        if seqCommand == 4 as libc::c_int {
            //call sequence player to decode a frame
            //wait for call to stop video
            state = 1 as libc::c_int
        } else {
            state = 0 as libc::c_int;
            seq_ShutDown();
            bSeqPlaying = 0 as libc::c_int
        }
    } else if frame < 0 as libc::c_int {
        //an ERROR
        debug(LOG_WZ,
              b"VIDEO FRAME ERROR %d\n\x00" as *const u8 as
                  *const libc::c_char, frame);
        state = 0 as libc::c_int;
        seq_ShutDown();
        bSeqPlaying = 0 as libc::c_int
    }
    return state;
}
#[no_mangle]
pub unsafe extern "C" fn seq_BlitBufferToScreen(mut screen: *mut libc::c_char,
                                                mut screenStride: SDWORD,
                                                mut xOffset: SDWORD,
                                                mut yOffset: SDWORD) -> BOOL {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut c8: libc::c_char = 0;
    let mut d8: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c16: UWORD = 0;
    let mut p16: *mut UWORD = 0 as *mut UWORD;
    let mut width: SDWORD = 0;
    let mut height: SDWORD = 0;
    seq_GetFrameSize(&mut width, &mut height);
    if videoMode as libc::c_uint ==
           VIDEO_SOFT_WINDOW as libc::c_int as libc::c_uint {
        d8 =
            screen.offset(xOffset as
                              isize).offset((yOffset * screenStride) as
                                                isize);
        p16 = pVideoBuffer as *mut UWORD;
        j = 0 as libc::c_int;
        while j < height {
            i = 0 as libc::c_int;
            while i < width {
                c16 = *p16.offset(i as isize);
                c16 = (c16 as libc::c_int & 0x7fff as libc::c_int) as UWORD;
                c8 = *pVideoPalette.offset(c16 as isize);
                *d8.offset(i as isize) = c8;
                i += 1
            }
            d8 = d8.offset(screenStride as isize);
            p16 = p16.offset(width as isize);
            j += 1
        }
    } else {
        pie_DownLoadBufferToScreen(pVideoBuffer as *mut libc::c_void,
                                   xOffset as UDWORD, yOffset as UDWORD,
                                   width as UDWORD, height as UDWORD,
                                   (2 as libc::c_int * width) as UDWORD);
    }
    return 1 as libc::c_int;
}
/* **************************************************************************/
/*
 *	local ProtoTypes
 */
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn clearVideoBuffer(mut surface: *mut iSurface) {
    let mut i: UDWORD = 0;
    let mut toClear: *mut UDWORD = 0 as *mut UDWORD;
    toClear = (*surface).buffer as *mut UDWORD;
    i = 0 as libc::c_int as UDWORD;
    while i < ((*surface).size / 4 as libc::c_int) as UDWORD {
        let fresh0 = toClear;
        toClear = toClear.offset(1);
        *fresh0 = 0xfcfcfcfc as libc::c_uint;
        i = i.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn seq_ReleaseVideoBuffers() -> BOOL {
    memFreeRelease(pVideoBuffer as *mut libc::c_void);
    pVideoBuffer = 0 as *mut libc::c_char;
    memFreeRelease(pVideoPalette as *mut libc::c_void);
    pVideoPalette = 0 as *mut libc::c_char;
    return 1 as libc::c_int;
}
//control
#[no_mangle]
pub unsafe extern "C" fn seq_SetupVideoBuffers() -> BOOL {
    let mut c: SDWORD = 0;
    let mut mallocSize: SDWORD = 0;
    let mut r: UBYTE = 0;
    let mut g: UBYTE = 0;
    let mut b: UBYTE = 0;
    //assume 320 * 240 * 16bit playback surface
    mallocSize =
        640 as libc::c_int * 480 as libc::c_int *
            2 as libc::c_int; //palette only used in 555mode
    pVideoBuffer =
        memMallocRelease(mallocSize as size_t) as *mut libc::c_char;
    if pVideoBuffer.is_null() { return 0 as libc::c_int }
    mallocSize = (1 as libc::c_int) << 15 as libc::c_int;
    pVideoPalette =
        memMallocRelease(mallocSize as size_t) as *mut libc::c_char;
    if pVideoPalette.is_null() { return 0 as libc::c_int }
    //Assume 555 RGB buffer for 8 bit rendering
    c = 0 as libc::c_int;
    r = 0 as libc::c_int as UBYTE;
    while (r as libc::c_int) < 32 as libc::c_int {
        //	loadingScreenCallback();
        g = 0 as libc::c_int as UBYTE;
        while (g as libc::c_int) < 32 as libc::c_int {
            b = 0 as libc::c_int as UBYTE;
            while (b as libc::c_int) < 32 as libc::c_int {
                *pVideoPalette.offset(c as isize) =
                    pal_GetNearestColour(((r as libc::c_int) <<
                                              3 as libc::c_int) as uint8,
                                         ((g as libc::c_int) <<
                                              3 as libc::c_int) as uint8,
                                         ((b as libc::c_int) <<
                                              3 as libc::c_int) as uint8) as
                        libc::c_char;
                c += 1;
                b = b.wrapping_add(1)
            }
            g = g.wrapping_add(1)
        }
        r = r.wrapping_add(1)
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn seq_SetVideoPath() {
    // now set up the hard disc path /
    if bHardPath == 0 {
        strcpy(aHardPath.as_mut_ptr(),
               b"sequences\\\x00" as *const u8 as *const libc::c_char);
        //yes, always true, as it should be on windows ALSO.
        //#endif
        bHardPath = 1 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn SeqEndCallBack(mut psSample: *mut AUDIO_SAMPLE)
 -> BOOL {
    /*#ifdef WIN32
		fileHandle = FindFirstFile("sequences\\*.rpl",&findData);
		if (fileHandle == INVALID_HANDLE_VALUE)
		{
//			bHardPath = FALSE;	//If it fails, then why say true?  Cause we *ALWAYS* need the
			bHardPath=TRUE;		//videos enabled.  Yes, we are playing novideo.rpl, but for
			return;			//windows, if we don't have that directory, then what?
		}				//then we run into a game stopping bug!
		else
		{
			bHardPath = TRUE;
			FindClose(fileHandle);
			return;
		}
#else
*/
    //	psSample;
    bAudioPlaying = 0 as libc::c_int;
    debug(LOG_NEVER,
          b"************* briefing ended **************\x00" as *const u8 as
              *const libc::c_char);
    return 1 as libc::c_int;
}
//full screenvideo functions
#[no_mangle]
pub unsafe extern "C" fn seq_StartFullScreenVideo(mut videoName:
                                                      *mut libc::c_char,
                                                  mut audioName:
                                                      *mut libc::c_char)
 -> BOOL {
    bHoldSeqForAudio = 0 as libc::c_int;
    debug(LOG_VIDEO,
          b"seq_StartFullScreenVideo: Refusing to play video! (Not a bug)\x00"
              as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}
//full screen render
//extern BOOL seq_PlayVideo(char* pSeq, char* pAudio);
//extern BOOL seq_StartFullScreenVideo(char* sequenceFile, char* audioFile);//start videos through seqList
#[no_mangle]
pub unsafe extern "C" fn seq_UpdateFullScreenVideo(mut pbClear:
                                                       *mut CLEAR_MODE)
 -> BOOL {
    let mut i: SDWORD = 0;
    let mut x: SDWORD = 0;
    let mut y: SDWORD = 0;
    let mut w: SDWORD = 0;
    let mut h: SDWORD = 0;
    let mut frame_0: SDWORD = 0;
    let mut frameLag: SDWORD = 0;
    let mut realFrame: SDWORD = 0;
    let mut subMin: SDWORD = 0;
    let mut subMax: SDWORD = 0;
    let mut videoTime: libc::c_int = 0;
    static mut videoFrameTime_0: libc::c_int = 0 as libc::c_int;
    static mut textFrame: libc::c_int = 0 as libc::c_int;
    let mut bMoreThanOneSequenceLine: BOOL = 0 as libc::c_int;
    if seq_GetCurrentFrame() == 0 as libc::c_int {
        //for new timing
        frame_0 = 0 as libc::c_int;
        videoFrameTime_0 = GetTickCount();
        textFrame = 0 as libc::c_int
    }
    seq_GetFrameSize(&mut w, &mut h);
    x =
        pie_GetVideoBufferWidth().wrapping_sub(w as
                                                   libc::c_uint).wrapping_div(2
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  libc::c_uint)
            as SDWORD;
    y =
        pie_GetVideoBufferHeight().wrapping_sub(h as
                                                    libc::c_uint).wrapping_div(2
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   libc::c_uint)
            as SDWORD;
    subMin =
        (480 as libc::c_int as
             libc::c_uint).wrapping_add(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint).wrapping_div(2
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   libc::c_uint))
            as SDWORD;
    subMax =
        (430 as libc::c_int as
             libc::c_uint).wrapping_add(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint).wrapping_div(2
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   libc::c_uint))
            as SDWORD;
    //get any text lines over bottom of the video
    realFrame = textFrame + 1 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if aSeqList[currentPlaySeq as
                        usize].aText[i as
                                         usize].pText[0 as libc::c_int as
                                                          usize] as
               libc::c_int != 0 as libc::c_int {
            if aSeqList[currentPlaySeq as usize].aText[i as usize].bSubtitle
                   == 1 as libc::c_int {
                if realFrame >=
                       aSeqList[currentPlaySeq as
                                    usize].aText[i as usize].startFrame &&
                       realFrame <=
                           aSeqList[currentPlaySeq as
                                        usize].aText[i as usize].endFrame {
                    if subMin >
                           aSeqList[currentPlaySeq as
                                        usize].aText[i as usize].y {
                        if aSeqList[currentPlaySeq as
                                        usize].aText[i as usize].y >
                               430 as libc::c_int {
                            subMin =
                                aSeqList[currentPlaySeq as
                                             usize].aText[i as usize].y
                        }
                    }
                    if subMax <
                           aSeqList[currentPlaySeq as
                                        usize].aText[i as usize].y {
                        subMax =
                            aSeqList[currentPlaySeq as
                                         usize].aText[i as usize].y
                    }
                } else if aSeqList[currentPlaySeq as usize].bSeqLoop != 0 {
                    //if its a looped video always draw the text
                    if subMin >=
                           aSeqList[currentPlaySeq as
                                        usize].aText[i as usize].y {
                        if aSeqList[currentPlaySeq as
                                        usize].aText[i as usize].y >
                               430 as libc::c_int {
                            subMin =
                                aSeqList[currentPlaySeq as
                                             usize].aText[i as usize].y
                        }
                    } //adjust video window here because text is already ofset for big screens
                    if subMax <
                           aSeqList[currentPlaySeq as
                                        usize].aText[i as usize].y {
                        subMax =
                            aSeqList[currentPlaySeq as
                                         usize].aText[i as usize].y
                    }
                }
            }
            if realFrame >=
                   aSeqList[currentPlaySeq as
                                usize].aText[i as usize].endFrame &&
                   realFrame <
                       aSeqList[currentPlaySeq as
                                    usize].aText[i as usize].endFrame +
                           frameSkip {
                if !pbClear.is_null() {
                    if perfMode as libc::c_uint !=
                           VIDEO_PERF_FULLSCREEN as libc::c_int as
                               libc::c_uint {
                        *pbClear = CLEAR_BLACK
                    }
                }
            }
        }
        i += 1
    }
    subMin =
        (subMin as
             libc::c_uint).wrapping_sub(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint).wrapping_div(2
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   libc::c_uint))
            as SDWORD as SDWORD;
    subMax =
        (subMax as
             libc::c_uint).wrapping_sub(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint).wrapping_div(2
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   libc::c_uint))
            as SDWORD as SDWORD;
    if subMin < 430 as libc::c_int { subMin = 430 as libc::c_int }
    if subMax > 480 as libc::c_int { subMax = 480 as libc::c_int }
    if subMax > subMin { bMoreThanOneSequenceLine = 1 as libc::c_int }
    if bHoldSeqForAudio == 0 as libc::c_int {
        if perfMode as libc::c_uint !=
               VIDEO_PERF_SKIP_FRAMES as libc::c_int as libc::c_uint {
            //version 1.00 release code
				//poll the sequence player while timing the video
            videoTime =
                GetTickCount(); // if were running slow frame lag will be greater than 1
            while videoTime < videoFrameTime_0 + frameDuration * frameSkip {
                videoTime =
                    GetTickCount(); //frame Lag should be 1 (most of the time)
                seq_RefreshVideoBuffers();
            }
            frameLag = videoTime - videoFrameTime_0;
            frameLag /= frameDuration;
            videoFrameTime_0 += frameLag * frameDuration;
            //call sequence player to decode a frame
            frame_0 = seq_RenderOneFrame(frameLag, subMin, subMax)
        } else {
            //new version with timeing removed
				//poll the sequence player while timing the video
            videoTime =
                GetTickCount(); //frame Lag should be 1 (most of the time)
            while videoTime < videoFrameTime_0 + frameDuration * frameSkip {
                videoTime = GetTickCount();
                seq_RefreshVideoBuffers();
            }
            videoFrameTime_0 += frameSkip * frameDuration;
            //call sequence player to decode a frame
            frame_0 = seq_RenderOneFrame(frameSkip, subMin, subMax)
        }
    } else {
        //call sequence player to download last frame
        frame_0 =
            seq_RenderOneFrame(0 as libc::c_int, 2 as libc::c_int,
                               0 as libc::c_int)
    }
    //print any text over the video
    realFrame = textFrame + 1 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if aSeqList[currentPlaySeq as
                        usize].aText[i as
                                         usize].pText[0 as libc::c_int as
                                                          usize] as
               libc::c_int != 0 as libc::c_int {
            if realFrame >=
                   aSeqList[currentPlaySeq as
                                usize].aText[i as usize].startFrame &&
                   realFrame <=
                       aSeqList[currentPlaySeq as
                                    usize].aText[i as usize].endFrame {
                if bMoreThanOneSequenceLine != 0 {
                    aSeqList[currentPlaySeq as usize].aText[i as usize].x =
                        (20 as libc::c_int as
                             libc::c_uint).wrapping_add(pie_GetVideoBufferWidth().wrapping_sub(640
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   libc::c_uint).wrapping_div(2
                                                                                                                                  as
                                                                                                                                  libc::c_int
                                                                                                                                  as
                                                                                                                                  libc::c_uint))
                            as SDWORD
                }
                pie_DrawTextToSurface(&mut *(*(*aSeqList.as_mut_ptr().offset(currentPlaySeq
                                                                                 as
                                                                                 isize)).aText.as_mut_ptr().offset(i
                                                                                                                       as
                                                                                                                       isize)).pText.as_mut_ptr().offset(0
                                                                                                                                                             as
                                                                                                                                                             libc::c_int
                                                                                                                                                             as
                                                                                                                                                             isize),
                                      aSeqList[currentPlaySeq as
                                                   usize].aText[i as usize].x,
                                      aSeqList[currentPlaySeq as
                                                   usize].aText[i as
                                                                    usize].y);
            } else if aSeqList[currentPlaySeq as usize].bSeqLoop != 0 {
                //if its a looped video always draw the text
                if bMoreThanOneSequenceLine != 0 {
                    aSeqList[currentPlaySeq as usize].aText[i as usize].x =
                        (20 as libc::c_int as
                             libc::c_uint).wrapping_add(pie_GetVideoBufferWidth().wrapping_sub(640
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   libc::c_uint).wrapping_div(2
                                                                                                                                  as
                                                                                                                                  libc::c_int
                                                                                                                                  as
                                                                                                                                  libc::c_uint))
                            as SDWORD
                }
                pie_DrawTextToSurface(&mut *(*(*aSeqList.as_mut_ptr().offset(currentPlaySeq
                                                                                 as
                                                                                 isize)).aText.as_mut_ptr().offset(i
                                                                                                                       as
                                                                                                                       isize)).pText.as_mut_ptr().offset(0
                                                                                                                                                             as
                                                                                                                                                             libc::c_int
                                                                                                                                                             as
                                                                                                                                                             isize),
                                      aSeqList[currentPlaySeq as
                                                   usize].aText[i as usize].x,
                                      aSeqList[currentPlaySeq as
                                                   usize].aText[i as
                                                                    usize].y);
            }
        }
        i += 1
    }
    textFrame = frame_0 * frameDuration / 40 as libc::c_int;
    if frame_0 == -(1 as libc::c_int) || bHoldSeqForAudio != 0 {
        if bAudioPlaying != 0 {
            if aSeqList[currentPlaySeq as usize].bSeqLoop != 0 {
                seq_ClearMovie();
                if seq_SetSequence(aVideoName.as_mut_ptr(),
                                   GetTickCount() + 0 as libc::c_int,
                                   pVideoBuffer, perfMode) == 0 {
                    bHoldSeqForAudio = 1 as libc::c_int
                }
                frameDuration = seq_GetFrameTimeInClicks()
            } else { bHoldSeqForAudio = 1 as libc::c_int }
            return 1 as libc::c_int
            //should hold the video
        } else {
            return 0 as libc::c_int
            //should terminate the video
        }
    } else {
        if frame_0 < 0 as libc::c_int {
            //an ERROR
            debug(LOG_NEVER,
                  b"VIDEO FRAME ERROR %d\n\x00" as *const u8 as
                      *const libc::c_char, frame_0);
            return 0 as libc::c_int
        }
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn seq_StopFullScreenVideo() -> BOOL {
    if seq_AnySeqLeft() == 0 { loop_ClearVideoPlaybackMode(); }
    seq_ShutDown();
    if seq_AnySeqLeft() == 0 {
        if g_bResumeInGame == 1 as libc::c_int {
            resetDesignPauseState();
            intAddReticule();
            g_bResumeInGame = 0 as libc::c_int
        }
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn seq_GetVideoSize(mut pWidth: *mut SDWORD,
                                          mut pHeight: *mut SDWORD) -> BOOL {
    return seq_GetFrameSize(pWidth, pHeight);
}
//text
// add a string at x,y or add string below last line if x and y are 0
#[no_mangle]
pub unsafe extern "C" fn seq_AddTextForVideo(mut pText: *mut libc::c_char,
                                             mut xOffset: SDWORD,
                                             mut yOffset: SDWORD,
                                             mut startFrame: SDWORD,
                                             mut endFrame: SDWORD,
                                             mut bJustify: SDWORD,
                                             mut PSXSeqNumber: UDWORD)
 -> BOOL {
    let mut sourceLength: SDWORD = 0;
    let mut currentLength: SDWORD = 0;
    let mut currentText: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut justification: SDWORD = 0;
    static mut lastX: SDWORD = 0;
    iV_SetFont(WFont);
    if aSeqList[currentSeq as usize].currentText < 32 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"seq_AddTextForVideo: too many text lines\x00" as *const u8 as
                  *const libc::c_char);
    };
    if aSeqList[currentSeq as usize].currentText < 32 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"seqdisp.c\x00" as *const u8 as *const libc::c_char,
              743 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 20],
                                        &[libc::c_char; 20]>(b"seq_AddTextForVideo\x00")).as_ptr(),
              b"aSeqList[currentSeq].currentText < MAX_TEXT_OVERLAYS\x00" as
                  *const u8 as *const libc::c_char);
    };
    sourceLength = strlen(pText) as SDWORD;
    currentLength = sourceLength;
    currentText =
        &mut *(*(*aSeqList.as_mut_ptr().offset(currentSeq as
                                                   isize)).aText.as_mut_ptr().offset((*aSeqList.as_mut_ptr().offset(currentSeq
                                                                                                                        as
                                                                                                                        isize)).currentText
                                                                                         as
                                                                                         isize)).pText.as_mut_ptr().offset(0
                                                                                                                               as
                                                                                                                               libc::c_int
                                                                                                                               as
                                                                                                                               isize)
            as *mut libc::c_char;
    //if the string is bigger than the buffer get the last end of the last fullword in the buffer
    if currentLength >= 256 as libc::c_int {
        currentLength = 256 as libc::c_int - 1 as libc::c_int;
        //get end of the last word
        while *pText.offset(currentLength as isize) as libc::c_int !=
                  ' ' as i32 && currentLength > 0 as libc::c_int {
            currentLength -= 1
        } //terminate the string what ever
        currentLength -= 1
    }
    memcpy(currentText as *mut libc::c_void, pText as *const libc::c_void,
           currentLength as libc::c_uint);
    *currentText.offset(currentLength as isize) =
        0 as libc::c_int as libc::c_char;
    //check the string is shortenough to print
	//if not take a word of the end and try again
    while iV_GetTextWidth(currentText) > 600 as libc::c_int {
        currentLength -= 1;
        while *pText.offset(currentLength as isize) as libc::c_int !=
                  ' ' as i32 && currentLength > 0 as libc::c_int {
            currentLength -= 1
        }
        *currentText.offset(currentLength as isize) =
            0 as libc::c_int as libc::c_char
        //terminate the string what ever
    } //terminate the string what ever
    *currentText.offset(currentLength as isize) =
        0 as libc::c_int as libc::c_char;
    //check if x and y are 0 and put text on next line
    if xOffset == 0 as libc::c_int && yOffset == 0 as libc::c_int &&
           currentLength > 0 as libc::c_int {
        aSeqList[currentSeq as
                     usize].aText[aSeqList[currentSeq as usize].currentText as
                                      usize].x = lastX;
        //	aSeqList[currentSeq].aText[aSeqList[currentSeq].currentText-1].x;
        aSeqList[currentSeq as
                     usize].aText[aSeqList[currentSeq as usize].currentText as
                                      usize].y =
            aSeqList[currentSeq as
                         usize].aText[(aSeqList[currentSeq as
                                                    usize].currentText -
                                           1 as libc::c_int) as usize].y +
                iV_GetTextLineSize()
    } else {
        aSeqList[currentSeq as
                     usize].aText[aSeqList[currentSeq as usize].currentText as
                                      usize].x =
            (xOffset as
                 libc::c_uint).wrapping_add(pie_GetVideoBufferWidth().wrapping_sub(640
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       libc::c_uint).wrapping_div(2
                                                                                                                      as
                                                                                                                      libc::c_int
                                                                                                                      as
                                                                                                                      libc::c_uint))
                as SDWORD;
        aSeqList[currentSeq as
                     usize].aText[aSeqList[currentSeq as usize].currentText as
                                      usize].y =
            (yOffset as
                 libc::c_uint).wrapping_add(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        libc::c_uint).wrapping_div(2
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       libc::c_uint))
                as SDWORD
    }
    lastX =
        aSeqList[currentSeq as
                     usize].aText[aSeqList[currentSeq as usize].currentText as
                                      usize].x;
    if bJustify != 0 && currentLength == sourceLength {
        //justify this text
        justification = 600 as libc::c_int - iV_GetTextWidth(currentText);
        if bJustify == 2 as libc::c_int && justification > 40 as libc::c_int {
            aSeqList[currentSeq as
                         usize].aText[aSeqList[currentSeq as
                                                   usize].currentText as
                                          usize].x +=
                justification / 2 as libc::c_int
        } else {
            (bJustify == 1 as libc::c_int) &&
                justification > 160 as libc::c_int;
        }
    }
    //set start and finish times for the objects
    aSeqList[currentSeq as
                 usize].aText[aSeqList[currentSeq as usize].currentText as
                                  usize].startFrame = startFrame;
    aSeqList[currentSeq as
                 usize].aText[aSeqList[currentSeq as usize].currentText as
                                  usize].endFrame = endFrame;
    aSeqList[currentSeq as
                 usize].aText[aSeqList[currentSeq as usize].currentText as
                                  usize].bSubtitle = bJustify;
    aSeqList[currentSeq as usize].currentText += 1;
    if aSeqList[currentSeq as usize].currentText >= 32 as libc::c_int {
        aSeqList[currentSeq as usize].currentText = 0 as libc::c_int
    }
    //check text is okay on the screen
    if currentLength < sourceLength {
        //RECURSE x= 0 y = 0 for nextLine
        if bJustify == 2 as libc::c_int { bJustify = 0 as libc::c_int }
        seq_AddTextForVideo(&mut *pText.offset((currentLength +
                                                    1 as libc::c_int) as
                                                   isize), 0 as libc::c_int,
                            0 as libc::c_int, startFrame, endFrame, bJustify,
                            0 as libc::c_int as UDWORD);
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn seq_ClearTextForVideo() -> BOOL {
    let mut i: SDWORD = 0;
    let mut j: SDWORD = 0;
    j = 0 as libc::c_int;
    while j < 6 as libc::c_int {
        i = 0 as libc::c_int;
        while i < 32 as libc::c_int {
            aSeqList[j as
                         usize].aText[i as
                                          usize].pText[0 as libc::c_int as
                                                           usize] =
                0 as libc::c_int as libc::c_char;
            aSeqList[j as usize].aText[i as usize].x = 0 as libc::c_int;
            aSeqList[j as usize].aText[i as usize].y = 0 as libc::c_int;
            aSeqList[j as usize].aText[i as usize].startFrame =
                0 as libc::c_int;
            aSeqList[j as usize].aText[i as usize].endFrame =
                0 as libc::c_int;
            aSeqList[j as usize].aText[i as usize].bSubtitle =
                0 as libc::c_int;
            i += 1
        }
        aSeqList[j as usize].currentText = 0 as libc::c_int;
        j += 1
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn seq_AddTextFromFile(mut pTextName: *mut STRING,
                                             mut bJustify: BOOL) -> BOOL {
    let mut pTextBuffer: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pCurrentLine: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pText: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fileSize: UDWORD = 0;
    //	HANDLE	fileHandle;
//	WIN32_FIND_DATA findData;
//	BOOL endOfFile = FALSE;
    let mut xOffset: SDWORD = 0;
    let mut yOffset: SDWORD = 0;
    let mut startFrame: SDWORD = 0;
    let mut endFrame: SDWORD = 0;
    let mut seps: *mut libc::c_char =
        b"\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    strcpy(aTextName.as_mut_ptr(),
           b"sequenceAudio\\\x00" as *const u8 as *const libc::c_char);
    strcat(aTextName.as_mut_ptr(), pTextName);
    /*
	fileHandle = FindFirstFile(aTextName,&findData);
	if (fileHandle == INVALID_HANDLE_VALUE)
	{
		return FALSE;
	}
	FindClose(fileHandle);
*/
    if loadFileToBufferNoError(aTextName.as_mut_ptr(), DisplayBuffer,
                               displayBufferSize, &mut fileSize) ==
           0 as libc::c_int {
        //Did I mention this is lame? -Q
        return 0 as libc::c_int
    }
    pTextBuffer = DisplayBuffer;
    pCurrentLine = strtok(pTextBuffer, seps);
    while !pCurrentLine.is_null() {
        if *pCurrentLine as libc::c_int != '/' as i32 {
            if sscanf(pCurrentLine,
                      b"%d %d %d %d\x00" as *const u8 as *const libc::c_char,
                      &mut xOffset as *mut SDWORD,
                      &mut yOffset as *mut SDWORD,
                      &mut startFrame as *mut SDWORD,
                      &mut endFrame as *mut SDWORD) == 4 as libc::c_int {
                //get the text
                pText = strrchr(pCurrentLine, '\"' as i32);
                if !pText.is_null() {
                } else {
                    debug(LOG_ERROR,
                          b"seq_AddTextFromFile error parsing text file\x00"
                              as *const u8 as *const libc::c_char);
                };
                if !pText.is_null() {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"seqdisp.c\x00" as *const u8 as
                              *const libc::c_char, 887 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 20],
                                                    &[libc::c_char; 20]>(b"seq_AddTextFromFile\x00")).as_ptr(),
                          b"pText != NULL\x00" as *const u8 as
                              *const libc::c_char);
                };
                if !pText.is_null() {
                    *pText = 0 as libc::c_int as UBYTE as libc::c_char
                }
                pText = strchr(pCurrentLine, '\"' as i32);
                if !pText.is_null() {
                } else {
                    debug(LOG_ERROR,
                          b"seq_AddTextFromFile error parsing text file\x00"
                              as *const u8 as *const libc::c_char);
                };
                if !pText.is_null() {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"seqdisp.c\x00" as *const u8 as
                              *const libc::c_char, 893 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 20],
                                                    &[libc::c_char; 20]>(b"seq_AddTextFromFile\x00")).as_ptr(),
                          b"pText != NULL\x00" as *const u8 as
                              *const libc::c_char);
                };
                if !pText.is_null() {
                    seq_AddTextForVideo(&mut *pText.offset(1 as libc::c_int as
                                                               isize),
                                        xOffset, yOffset, startFrame,
                                        endFrame, bJustify,
                                        0 as libc::c_int as UDWORD);
                }
            }
        }
        //get next line
        pCurrentLine = strtok(0 as *mut libc::c_char, seps)
    }
    return 1 as libc::c_int;
}
//clear the sequence list
//clear the sequence list
#[no_mangle]
pub unsafe extern "C" fn seq_ClearSeqList() {
    let mut i: SDWORD = 0;
    seq_ClearTextForVideo();
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        aSeqList[i as usize].pSeq = 0 as *mut libc::c_char;
        i += 1
    }
    currentSeq = -(1 as libc::c_int);
    currentPlaySeq = -(1 as libc::c_int);
}
//add a sequence to the list to be played
//add a sequence to the list to be played
#[no_mangle]
pub unsafe extern "C" fn seq_AddSeqToList(mut pSeqName: *mut STRING,
                                          mut pAudioName: *mut STRING,
                                          mut pTextName: *mut STRING,
                                          mut bLoop: BOOL,
                                          mut PSXSeqNumber: UDWORD) {
    let mut strLen: SDWORD = 0;
    currentSeq += 1;
    if currentSeq >= 6 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"seq_AddSeqToList: too many sequences\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"seqdisp.c\x00" as *const u8 as *const libc::c_char,
                  931 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 17],
                                            &[libc::c_char; 17]>(b"seq_AddSeqToList\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return
    }
    //OK so add it to the list
    aSeqList[currentSeq as usize].pSeq = pSeqName;
    aSeqList[currentSeq as usize].pAudio = pAudioName;
    aSeqList[currentSeq as usize].bSeqLoop = bLoop;
    if !pTextName.is_null() {
        seq_AddTextFromFile(pTextName, 0 as libc::c_int);
        //SEQ_TEXT_POSITION);//ordinary text not justified
    }
    if bSeqSubtitles != 0 {
        //check for a subtitle file
        strLen = strlen(pSeqName) as SDWORD;
        if strLen < 256 as libc::c_int {
        } else {
            debug(LOG_ERROR,
                  b"seq_AddSeqToList: sequence name error\x00" as *const u8 as
                      *const libc::c_char);
        };
        if strLen < 256 as libc::c_int {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"seqdisp.c\x00" as *const u8 as *const libc::c_char,
                  951 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 17],
                                            &[libc::c_char; 17]>(b"seq_AddSeqToList\x00")).as_ptr(),
                  b"strLen < MAX_STR_LENGTH\x00" as *const u8 as
                      *const libc::c_char);
        };
        strcpy(aSubtitleName.as_mut_ptr(), pSeqName);
        aSubtitleName[(strLen - 4 as libc::c_int) as usize] =
            0 as libc::c_int as libc::c_char;
        strcat(aSubtitleName.as_mut_ptr(),
               b".txt\x00" as *const u8 as *const libc::c_char);
        seq_AddTextFromFile(aSubtitleName.as_mut_ptr(), 1 as libc::c_int);
        //SEQ_TEXT_JUSTIFY);//subtitles centre justified
    };
}
/*checks to see if there are any sequences left in the list to play*/
/*checks to see if there are any sequences left in the list to play*/
#[no_mangle]
pub unsafe extern "C" fn seq_AnySeqLeft() -> BOOL {
    let mut nextSeq: UBYTE = 0;
    nextSeq = (currentPlaySeq + 1 as libc::c_int) as UBYTE;
    //check haven't reached end
    if nextSeq as libc::c_int > 6 as libc::c_int {
        return 0 as libc::c_int
    } else if !aSeqList[nextSeq as usize].pSeq.is_null() {
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn seqDispCDOK() {
    let mut bPlayedOK: BOOL = 0;
    if bBackDropWasAlreadyUp == 0 as libc::c_int { screen_StopBackDrop(); }
    currentPlaySeq += 1;
    if currentPlaySeq >= 6 as libc::c_int {
        bPlayedOK = 0 as libc::c_int
    } else {
        bPlayedOK =
            seq_StartFullScreenVideo(aSeqList[currentPlaySeq as usize].pSeq,
                                     aSeqList[currentPlaySeq as usize].pAudio)
    }
    if bPlayedOK == 0 as libc::c_int {
        //don't do the callback if we're playing the win/lose video
        if getScriptWinLoseVideo() == 0 {
            eventFireCallbackTrigger(CALL_VIDEO_QUIT as libc::c_int as
                                         TRIGGER_TYPE);
        } else {
            displayGameOver((getScriptWinLoseVideo() as libc::c_int ==
                                 1 as libc::c_int) as libc::c_int);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn seqDispCDCancel() { }
/*returns the next sequence in the list to play*/
/*returns the next sequence in the list to play*/
#[no_mangle]
pub unsafe extern "C" fn seq_StartNextFullScreenVideo() { seqDispCDOK(); }
//set and check subtitle mode, TRUE subtitles on
#[no_mangle]
pub unsafe extern "C" fn seq_SetSubtitles(mut bNewState: BOOL) {
    bSeqSubtitles = bNewState;
}
#[no_mangle]
pub unsafe extern "C" fn seq_GetSubtitles() -> BOOL { return bSeqSubtitles; }
/*play a video now and clear all other videos, front end use only*/
/*
BOOL seq_PlayVideo(char* pSeq, char* pAudio)
{
	seq_ClearSeqList();//other wise me might trigger these videos when we finish
	seq_StartFullScreenVideo(pSeq, pAudio);
	while (loop_GetVideoStatus())
	{
		videoLoop();
	}
	return TRUE;
}
*/
