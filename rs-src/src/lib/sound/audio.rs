use ::libc;
extern "C" {
    /* Function to create a heap
 * Takes the size of the objects to be managed by the heap,
 * the initial number of objects to allocate and the number of
 * objects to allocate when the heap is extended.
 * Returns an initialised OBJ_HEAP structure.
 */
    #[no_mangle]
    fn heapCreate(ppsHeap: *mut *mut OBJ_HEAP, size: UDWORD, init: UDWORD,
                  ext: UDWORD) -> BOOL;
    /* Allocate an object from a heap
 * Returns a pointer to the object if successful
 */
    #[no_mangle]
    fn heapAlloc(psHeap: *mut OBJ_HEAP, ppObject: *mut *mut libc::c_void)
     -> BOOL;
    /* Return an object to the heap */
    #[no_mangle]
    fn heapFree(psHeap: *mut OBJ_HEAP, pObject: *mut libc::c_void) -> BOOL;
    /* Destroy a heap and release all the memory associated with it */
    #[no_mangle]
    fn heapDestroy(psHeap: *mut OBJ_HEAP);
    #[no_mangle]
    fn resGetData(pType: *mut STRING, pID: *mut STRING) -> *mut libc::c_void;
    #[no_mangle]
    fn resGetDataFromHash(pType: *mut STRING, HashedID: UDWORD)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_uint)
     -> *mut libc::c_void;
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
    fn abort() -> !;
    /* **************************************************************************/
/*
 * Library-specific sound library functions;
 * these need to be re-written for each library.
 */
/* **************************************************************************/
    /* **************************************************************************/
    /* **************************************************************************/
    /* **************************************************************************/
    #[no_mangle]
    fn sound_GetGameTime() -> UDWORD;
    /* **************************************************************************/
/* functions
 */
    #[no_mangle]
    fn sound_Init(iMaxSameSamples: SDWORD) -> BOOL;
    #[no_mangle]
    fn sound_Shutdown() -> BOOL;
    #[no_mangle]
    fn sound_LoadTrackFromFile(szFileName: *mut libc::c_char) -> BOOL;
    #[no_mangle]
    fn sound_LoadTrackFromBuffer(pBuffer: *mut libc::c_char, udwSize: UDWORD)
     -> *mut libc::c_void;
    #[no_mangle]
    fn sound_SetTrackVals(psTrack: *mut TRACK, bLoop: BOOL, iTrack: SDWORD,
                          iVol: SDWORD, iPriority: SDWORD,
                          iAudibleRadius: SDWORD, VagID: SDWORD) -> BOOL;
    #[no_mangle]
    fn sound_ReleaseTrack(psTrack: *mut TRACK) -> BOOL;
    #[no_mangle]
    fn sound_StopTrack(psSample: *mut AUDIO_SAMPLE);
    #[no_mangle]
    fn sound_CheckAllUnloaded();
    #[no_mangle]
    fn sound_CheckTrack(iTrack: SDWORD) -> BOOL;
    #[no_mangle]
    fn sound_GetTrackAudibleRadius(iTrack: SDWORD) -> SDWORD;
    #[no_mangle]
    fn sound_GetTrackVolume(iTrack: SDWORD) -> SDWORD;
    #[no_mangle]
    fn sound_Play2DTrack(psSample: *mut AUDIO_SAMPLE, bQueued: BOOL) -> BOOL;
    #[no_mangle]
    fn sound_Play3DTrack(psSample: *mut AUDIO_SAMPLE) -> BOOL;
    #[no_mangle]
    fn sound_GetTrackID(psTrack: *mut TRACK) -> SDWORD;
    #[no_mangle]
    fn sound_GetAvailableID() -> SDWORD;
    #[no_mangle]
    fn sound_SetStoppedCallback(pStopTrackCallback: AUDIO_CALLBACK);
    #[no_mangle]
    fn sound_GetTrackTimeLastFinished(iTrack: SDWORD) -> UDWORD;
    #[no_mangle]
    fn sound_SetTrackTimeLastFinished(iTrack: SDWORD, iTime: UDWORD);
    #[no_mangle]
    fn sound_PlayStream(psSample: *mut AUDIO_SAMPLE,
                        szFileName: *mut libc::c_char, iVol: SDWORD) -> BOOL;
    #[no_mangle]
    fn sound_GetMaxVolume() -> libc::c_int;
    #[no_mangle]
    fn sound_QueueSamplePlaying() -> BOOL;
    #[no_mangle]
    fn sound_SetPlayerPos(iX: SDWORD, iY: SDWORD, iZ: SDWORD);
    #[no_mangle]
    fn sound_SetPlayerOrientation(iX: SDWORD, iY: SDWORD, iZ: SDWORD);
    #[no_mangle]
    fn sound_SetObjectPosition(iSample: SDWORD, iX: SDWORD, iY: SDWORD,
                               iZ: SDWORD);
    #[no_mangle]
    fn sound_PauseAll();
    #[no_mangle]
    fn sound_ResumeAll();
    #[no_mangle]
    fn sound_StopAll();
    #[no_mangle]
    fn sound_Update();
    /* **************************************************************************/
/*
 * Aud.h
 *
 * Audio wrapper functions
 *
 * Gareth Jones 16/12/97
 */
/* **************************************************************************/
    /* **************************************************************************/
    /* **************************************************************************/
    #[no_mangle]
    fn audio_Get3DPlayerRotAboutVerticalAxis(piA: *mut SDWORD);
    #[no_mangle]
    fn audio_GetObjectPos(psObj: *mut libc::c_void, piX: *mut SDWORD,
                          piY: *mut SDWORD, piZ: *mut SDWORD);
    #[no_mangle]
    fn audio_GetStaticPos(iWorldX: SDWORD, iWorldY: SDWORD, piX: *mut SDWORD,
                          piY: *mut SDWORD, piZ: *mut SDWORD);
    #[no_mangle]
    fn audio_ObjectDead(psObj: *mut libc::c_void) -> BOOL;
    #[no_mangle]
    fn audio_Display3D() -> BOOL;
    #[no_mangle]
    fn audio_GetIDFromStr(pWavStr: *mut STRING, piID: *mut SDWORD) -> BOOL;
    #[no_mangle]
    fn audio_Get2DPlayerPos(piX: *mut SDWORD, piY: *mut SDWORD,
                            piZ: *mut SDWORD);
    #[no_mangle]
    fn audio_Get3DPlayerPos(piX: *mut SDWORD, piY: *mut SDWORD,
                            piZ: *mut SDWORD);
}
pub type UBYTE = libc::c_uchar;
pub type STRING = libc::c_char;
pub type UDWORD = libc::c_uint;
pub type SDWORD = libc::c_int;
pub type BOOL = libc::c_int;
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
pub struct _block_heap {
    pub init: SDWORD,
    pub ext: SDWORD,
    pub psBlocks: *mut BLOCK_HEAP_MEM,
    pub psNext: *mut _block_heap,
}
pub type BLOCK_HEAP_MEM = _block_heap_mem;
/*
 * Block.h
 *
 * Routines to allocate memory from one large block.
 * Any memory allocated is only available to be reallocated after
 * the whole block has been reset.
 */
// control whether the debugging block malloc is used
/* *********************************************************************************/
/*                    type definitions                                            */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _block_heap_mem {
    pub size: SDWORD,
    pub pFree: *mut UBYTE,
    pub pMem: *mut UBYTE,
    pub pLastAllocated: *mut UBYTE,
    pub psNext: *mut _block_heap_mem,
}
/*
 * Heap.h
 *
 * Interface to the heap memory routines.
 *
 * Overhead of using the heap is :
 *			24 bytes for the initial block
 *           4 bytes for the extension blocks
 *
 */
/* Include Mem.h to get the DEBUG_MALLOC #define - this controls whether
  * normal or debugging memory management is used.
  */
/* structure used to store the list of free heap objects */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _free_object {
    pub psNext: *mut _free_object,
}
pub type FREE_OBJECT = _free_object;
/* structure used to store the extra space allocated for the heap */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _heap_extension {
    pub pMemory: *mut UBYTE,
    pub psNext: *mut _heap_extension,
}
pub type HEAP_EXTENSION = _heap_extension;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _obj_heap {
    pub objSize: UDWORD,
    pub initAlloc: UDWORD,
    pub extAlloc: UDWORD,
    pub psBlkHeap: *mut _block_heap,
    pub psFree: *mut FREE_OBJECT,
    pub pMemory: *mut UBYTE,
    pub psExt: *mut HEAP_EXTENSION,
}
pub type OBJ_HEAP = _obj_heap;
// Extension memory for the heap
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDWVEC3D {
    pub x: SDWORD,
    pub y: SDWORD,
    pub z: SDWORD,
}
//*
//
// externs
//*
//
// static functions
//*
//
// global variables
static mut g_psSampleHeap: *mut OBJ_HEAP =
    0 as *const OBJ_HEAP as *mut OBJ_HEAP;
static mut g_psSampleList: *mut AUDIO_SAMPLE =
    0 as *const AUDIO_SAMPLE as *mut AUDIO_SAMPLE;
static mut g_psSampleQueue: *mut AUDIO_SAMPLE =
    0 as *const AUDIO_SAMPLE as *mut AUDIO_SAMPLE;
static mut g_bAudioEnabled: BOOL = 0 as libc::c_int;
static mut g_bAudioPaused: BOOL = 0 as libc::c_int;
static mut g_bStopAll: BOOL = 0 as libc::c_int;
static mut g_sPreviousSample: AUDIO_SAMPLE =
    {
        let mut init =
            AUDIO_SAMPLE{iTrack: -(2 as libc::c_int),
                         iSample: -(5 as libc::c_int) as ALuint,
                         x: -(5 as libc::c_int),
                         y: -(5 as libc::c_int),
                         z: 0,
                         iLoops: 0,
                         bRemove: 0,
                         pCallback: None,
                         psObj: 0 as *const libc::c_void as *mut libc::c_void,
                         psPrev:
                             0 as *const AUDIO_SAMPLE as *mut AUDIO_SAMPLE,
                         psNext:
                             0 as *const AUDIO_SAMPLE as *mut AUDIO_SAMPLE,};
        init
    };
static mut g_i3DVolume: SDWORD = 100 as libc::c_long as SDWORD;
// ifdef WIN32 //Not needed ?--Qamly
//
// static CRITICAL_SECTION critSecAudio;
//
// endif
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn audio_Disabled() -> BOOL {
    return (g_bAudioEnabled == 0) as libc::c_int;
}
/* **************************************************************************/
/* **************************************************************************/
/* **************************************************************************/
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn audio_Init(mut pStopTrackCallback: AUDIO_CALLBACK)
 -> BOOL {
    // init audio system
    g_bAudioEnabled = sound_Init(2 as libc::c_int);
    if g_bAudioEnabled != 0 {
        // allocate sample heap
        if heapCreate(&mut g_psSampleHeap, 1000 as libc::c_int as UDWORD,
                      10 as libc::c_int as UDWORD,
                      ::std::mem::size_of::<AUDIO_SAMPLE>() as libc::c_ulong)
               == 0 {
            debug(LOG_ERROR,
                  b"audio_Init: couldn\'t create sample queue\n\x00" as
                      *const u8 as *const libc::c_char);
            abort();
        }
        sound_SetStoppedCallback(pStopTrackCallback);
    }
    return g_bAudioEnabled;
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn audio_Shutdown() -> BOOL {
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    let mut psSample: *mut AUDIO_SAMPLE = 0 as *mut AUDIO_SAMPLE;
    let mut psSampleTemp: *mut AUDIO_SAMPLE = 0 as *mut AUDIO_SAMPLE;
    let mut bOK: BOOL = 0;
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // if audio not enabled return TRUE to carry on game without audio
    if g_bAudioEnabled == 0 as libc::c_int { return 1 as libc::c_int }
    sound_StopAll();
    bOK = sound_Shutdown();
    // empty sample heap
	// ifdef WIN32 //Not needed ?--Qamly
	//
	// EnterCriticalSection( &critSecAudio );
	//
	// endif
	// empty sample list
    psSample = g_psSampleList;
    while !psSample.is_null() {
        psSampleTemp = (*psSample).psNext;
        heapFree(g_psSampleHeap, psSample as *mut libc::c_void);
        psSample = psSampleTemp
    }
    // empty sample queue
    psSample = g_psSampleQueue;
    while !psSample.is_null() {
        psSampleTemp = (*psSample).psNext;
        heapFree(g_psSampleHeap, psSample as *mut libc::c_void);
        psSample = psSampleTemp
    }
    // ifdef WIN32 //Not needed ?--Qamly
	//
	// LeaveCriticalSection( &critSecAudio );
	//
	// endif
	// free sample heap
    heapDestroy(g_psSampleHeap);
    g_psSampleHeap = 0 as *mut OBJ_HEAP;
    g_psSampleList = 0 as *mut AUDIO_SAMPLE;
    g_psSampleQueue = 0 as *mut AUDIO_SAMPLE;
    // ifdef WIN32 //Not needed ?--Qamly
	//
	// DeleteCriticalSection( &critSecAudio );
	//
	// endif
    return bOK;
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn audio_PlayPreviousQueueTrack() {
    if g_sPreviousSample.iTrack != -(2 as libc::c_int) {
        audio_PlayTrack(g_sPreviousSample.iTrack);
    };
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn audio_GetPreviousQueueTrackPos(mut iX: *mut SDWORD,
                                                        mut iY: *mut SDWORD,
                                                        mut iZ: *mut SDWORD)
 -> BOOL {
    if g_sPreviousSample.x != -(5 as libc::c_int) &&
           g_sPreviousSample.y != -(5 as libc::c_int) &&
           g_sPreviousSample.z != -(5 as libc::c_int) {
        *iX = g_sPreviousSample.x;
        *iY = g_sPreviousSample.y;
        *iZ = g_sPreviousSample.z;
        return 1 as libc::c_int
    } else { *iY = *iZ; *iX = *iY; return 0 as libc::c_int };
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
unsafe extern "C" fn audio_AddSampleToHead(mut ppsSampleList:
                                               *mut *mut AUDIO_SAMPLE,
                                           mut psSample: *mut AUDIO_SAMPLE) {
    // ifdef WIN32 //Not needed ?--Qamly
	//
	// EnterCriticalSection( &critSecAudio );
	//
	// endif
    (*psSample).psNext = *ppsSampleList;
    (*psSample).psPrev = 0 as *mut AUDIO_SAMPLE;
    if !(*ppsSampleList).is_null() { (**ppsSampleList).psPrev = psSample }
    *ppsSampleList = psSample;
    // ifdef WIN32 //Not needed ?--Qamly
	//
	// LeaveCriticalSection( &critSecAudio );
	//
	// endif
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
unsafe extern "C" fn audio_AddSampleToTail(mut ppsSampleList:
                                               *mut *mut AUDIO_SAMPLE,
                                           mut psSample: *mut AUDIO_SAMPLE) {
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    let mut psSampleTail: *mut AUDIO_SAMPLE = 0 as *mut AUDIO_SAMPLE;
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // ifdef WIN32 //Not needed ?--Qamly
	//
	// EnterCriticalSection( &critSecAudio );
	//
	// endif
    if (*ppsSampleList).is_null() {
        *ppsSampleList = psSample
    } else {
        psSampleTail = *ppsSampleList;
        while !(*psSampleTail).psNext.is_null() {
            psSampleTail = (*psSampleTail).psNext
        }
        (*psSampleTail).psNext = psSample;
        (*psSample).psPrev = psSampleTail;
        (*psSample).psNext = 0 as *mut AUDIO_SAMPLE
    };
    // ifdef WIN32 //Not needed ?--Qamly
	//
	// LeaveCriticalSection( &critSecAudio );
	//
	// endif
}
//*
//
// audio_RemoveSample Removes sample from list but doesn't free its memory
//*
// =======================================================================================================================
// =======================================================================================================================
//
unsafe extern "C" fn audio_RemoveSample(mut ppsSampleList:
                                            *mut *mut AUDIO_SAMPLE,
                                        mut psSample: *mut AUDIO_SAMPLE) {
    if psSample.is_null() { return }
    // ifdef WIN32 //Not needed ?--Qamly
	//
	// EnterCriticalSection( &critSecAudio );
	//
	// endif
    if psSample == *ppsSampleList {
        // first sample in list
        *ppsSampleList = (*psSample).psNext
    } else {
        if !(*psSample).psPrev.is_null() {
            (*(*psSample).psPrev).psNext = (*psSample).psNext
        }
        if !(*psSample).psNext.is_null() {
            (*(*psSample).psNext).psPrev = (*psSample).psPrev
        }
    }
    // set sample pointers NULL for safety
    (*psSample).psPrev = 0 as *mut AUDIO_SAMPLE;
    (*psSample).psNext = 0 as *mut AUDIO_SAMPLE;
    // ifdef WIN32 //Not needed ?--Qamly
	//
	// LeaveCriticalSection( &critSecAudio );
	//
	// endif
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
unsafe extern "C" fn audio_CheckSameQueueTracksPlaying(mut iTrack: SDWORD)
 -> BOOL {
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    let mut iCount: SDWORD = 0;
    let mut psSample: *mut AUDIO_SAMPLE = 0 as *mut AUDIO_SAMPLE;
    let mut bOK: BOOL = 1 as libc::c_int;
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // return if audio not enabled
    if g_bAudioEnabled == 0 as libc::c_int ||
           g_bAudioPaused == 1 as libc::c_int {
        return 1 as libc::c_int
    }
    iCount = 0 as libc::c_int;
    // loop through queue sounds and check whether too many already in it
    psSample = g_psSampleQueue;
    while !psSample.is_null() {
        if (*psSample).iTrack == iTrack { iCount += 1 }
        if iCount > 2 as libc::c_int {
            bOK = 0 as libc::c_int;
            break ;
        } else { psSample = (*psSample).psNext }
    }
    return bOK;
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn audio_QueueSample(mut iTrack: SDWORD)
 -> *mut AUDIO_SAMPLE {
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    let mut psSample: *mut AUDIO_SAMPLE = 0 as *mut AUDIO_SAMPLE;
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    //
	// SDWORD iSameSamples = 0;
	//
//	debug(LOG_SOUND, "audio_queuesample called - track=%d", iTrack );
    // return if audio not enabled
    if g_bAudioEnabled == 0 as libc::c_int ||
           g_bAudioPaused == 1 as libc::c_int ||
           g_bStopAll == 1 as libc::c_int {
        return 0 as *mut AUDIO_SAMPLE
    }
    if sound_CheckTrack(iTrack) == 1 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"audio_QueueSample: track %i outside limits\n\x00" as *const u8
                  as *const libc::c_char, iTrack);
    };
    if sound_CheckTrack(iTrack) == 1 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"audio.c\x00" as *const u8 as *const libc::c_char,
              365 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 18],
                                        &[libc::c_char; 18]>(b"audio_QueueSample\x00")).as_ptr(),
              b"sound_CheckTrack(iTrack) == TRUE\x00" as *const u8 as
                  *const libc::c_char);
    };
    // reject track if too many of same ID already in queue
    if audio_CheckSameQueueTracksPlaying(iTrack) == 0 as libc::c_int {
        return 0 as *mut AUDIO_SAMPLE
    }
    //	debug(LOG_SOUND, "audio_queuetrack called1" );
    heapAlloc(g_psSampleHeap,
              &mut psSample as *mut *mut AUDIO_SAMPLE as *mut libc::c_void as
                  *mut *mut libc::c_void); //[check] -Q
    if !psSample.is_null() {
        memset(psSample as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<AUDIO_SAMPLE>() as libc::c_ulong);
        (*psSample).iTrack = iTrack;
        (*psSample).x = -(5 as libc::c_int);
        (*psSample).y = -(5 as libc::c_int);
        (*psSample).z = -(5 as libc::c_int);
        (*psSample).bRemove = 0 as libc::c_int;
        // add to queue
        audio_AddSampleToTail(&mut g_psSampleQueue, psSample);
    }
    return psSample;
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn audio_QueueTrack(mut iTrack: SDWORD) {
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    let mut psSample: *mut AUDIO_SAMPLE = 0 as *mut AUDIO_SAMPLE;
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // return if audio not enabled
    if g_bAudioEnabled == 0 as libc::c_int ||
           g_bAudioPaused == 1 as libc::c_int ||
           g_bStopAll == 1 as libc::c_int {
        return
    }
    psSample = audio_QueueSample(iTrack);
}
//*
//
//
// * audio_QueueTrackMinDelay Will only play track if iMinDelay has elapsed since
// * track last finished
//
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn audio_QueueTrackMinDelay(mut iTrack: SDWORD,
                                                  mut iMinDelay: UDWORD) {
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    let mut psSample: *mut AUDIO_SAMPLE = 0 as *mut AUDIO_SAMPLE;
    let mut iDelay: UDWORD = 0;
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // return if audio not enabled
    if g_bAudioEnabled == 0 as libc::c_int ||
           g_bAudioPaused == 1 as libc::c_int {
        return
    }
    iDelay =
        sound_GetGameTime().wrapping_sub(sound_GetTrackTimeLastFinished(iTrack));
    if iDelay > iMinDelay {
        psSample = audio_QueueSample(iTrack);
        if !psSample.is_null() {
            sound_SetTrackTimeLastFinished(iTrack, sound_GetGameTime());
        }
    };
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn audio_QueueTrackMinDelayPos(mut iTrack: SDWORD,
                                                     mut iMinDelay: UDWORD,
                                                     mut iX: SDWORD,
                                                     mut iY: SDWORD,
                                                     mut iZ: SDWORD) {
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    let mut psSample: *mut AUDIO_SAMPLE = 0 as *mut AUDIO_SAMPLE;
    let mut iDelay: UDWORD = 0;
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // return if audio not enabled
    if g_bAudioEnabled == 0 as libc::c_int ||
           g_bAudioPaused == 1 as libc::c_int {
        return
    }
    iDelay =
        sound_GetGameTime().wrapping_sub(sound_GetTrackTimeLastFinished(iTrack));
    if iDelay > iMinDelay {
        psSample = audio_QueueSample(iTrack);
        if !psSample.is_null() {
            sound_SetTrackTimeLastFinished(iTrack, sound_GetGameTime());
            (*psSample).x = iX;
            (*psSample).y = iY;
            (*psSample).z = iZ
        }
    };
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn audio_QueueTrackPos(mut iTrack: SDWORD,
                                             mut iX: SDWORD, mut iY: SDWORD,
                                             mut iZ: SDWORD) {
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    let mut psSample: *mut AUDIO_SAMPLE = 0 as *mut AUDIO_SAMPLE;
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // return if audio not enabled
    if g_bAudioEnabled == 0 as libc::c_int ||
           g_bAudioPaused == 1 as libc::c_int {
        return
    }
    psSample = audio_QueueSample(iTrack);
    if !psSample.is_null() {
        (*psSample).x = iX;
        (*psSample).y = iY;
        (*psSample).z = iZ
    };
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn audio_UpdateQueue() {
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    let mut psSample: *mut AUDIO_SAMPLE = 0 as *mut AUDIO_SAMPLE;
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // return if audio not enabled
    if g_bAudioEnabled == 0 as libc::c_int ||
           g_bAudioPaused == 1 as libc::c_int {
        return
    }
    if sound_QueueSamplePlaying() == 1 as libc::c_int {
        // lower volume whilst playing queue audio
        audio_Set3DVolume((100 as libc::c_long /
                               4 as libc::c_int as libc::c_long) as SDWORD);
    } else {
        // set full global volume
        audio_Set3DVolume(100 as libc::c_long as SDWORD);
        // check queue for members
        if !g_psSampleQueue.is_null() {
            // remove queue head
            psSample = g_psSampleQueue;
            audio_RemoveSample(&mut g_psSampleQueue, psSample);
            // add sample to list if able to play
            if sound_Play2DTrack(psSample, 1 as libc::c_int) ==
                   1 as libc::c_int {
                audio_AddSampleToHead(&mut g_psSampleList, psSample);
                // update last queue sound coords
                if (*psSample).x != -(5 as libc::c_int) &&
                       (*psSample).y != -(5 as libc::c_int) &&
                       (*psSample).z != -(5 as libc::c_int) {
                    g_sPreviousSample.x = (*psSample).x;
                    g_sPreviousSample.y = (*psSample).y;
                    g_sPreviousSample.z = (*psSample).z
                }
            } else {
                debug(LOG_NEVER,
                      b"audio_UpdateQueue: couldn\'t play sample\n\x00" as
                          *const u8 as *const libc::c_char);
                heapFree(g_psSampleHeap, psSample as *mut libc::c_void);
            }
        }
    };
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn audio_Update() -> BOOL {
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    let mut vecPlayer: SDWVEC3D = SDWVEC3D{x: 0, y: 0, z: 0,};
    let mut iA: SDWORD = 0;
    let mut psSample: *mut AUDIO_SAMPLE = 0 as *mut AUDIO_SAMPLE;
    let mut psSampleTemp: *mut AUDIO_SAMPLE = 0 as *mut AUDIO_SAMPLE;
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // if audio not enabled return TRUE to carry on game without audio
    if g_bAudioEnabled == 0 as libc::c_int { return 1 as libc::c_int }
    audio_UpdateQueue();
    // get player position
    if audio_Display3D() == 1 as libc::c_int {
        audio_Get3DPlayerPos(&mut vecPlayer.x, &mut vecPlayer.y,
                             &mut vecPlayer.z);
    } else {
        audio_Get2DPlayerPos(&mut vecPlayer.x, &mut vecPlayer.y,
                             &mut vecPlayer.z);
    }
    sound_SetPlayerPos(vecPlayer.x, vecPlayer.y, vecPlayer.z);
    audio_Get3DPlayerRotAboutVerticalAxis(&mut iA);
    sound_SetPlayerOrientation(0 as libc::c_int, 0 as libc::c_int, iA);
    // loop through 3D sounds and remove if finished or update position
    psSample = g_psSampleList;
    while !psSample.is_null() {
        // remove finished samples from list
        if (*psSample).bRemove == 1 as libc::c_int {
            audio_RemoveSample(&mut g_psSampleList, psSample);
            psSampleTemp = (*psSample).psNext;
            heapFree(g_psSampleHeap, psSample as *mut libc::c_void);
            psSample = psSampleTemp
        } else {
            // check looping sound callbacks for finished condition
            if !(*psSample).psObj.is_null() {
                if audio_ObjectDead((*psSample).psObj) != 0 ||
                       (*psSample).pCallback.is_some() &&
                           (*psSample).pCallback.expect("non-null function pointer")(psSample)
                               == 0 as libc::c_int {
                    sound_StopTrack(psSample);
                    (*psSample).psObj = 0 as *mut libc::c_void
                } else {
                    // update sample position
                    audio_GetObjectPos((*psSample).psObj, &mut (*psSample).x,
                                       &mut (*psSample).y,
                                       &mut (*psSample).z);
                    sound_SetObjectPosition((*psSample).iSample as SDWORD,
                                            (*psSample).x, (*psSample).y,
                                            (*psSample).z);
                }
            }
            // next sample
            psSample = (*psSample).psNext
        }
    }
    sound_Update();
    return 1 as libc::c_int;
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn audio_LoadTrackFromFile(mut szFileName:
                                                     *mut libc::c_char)
 -> BOOL {
    // if audio not enabled return TRUE to carry on game without audio
    if g_bAudioEnabled == 0 as libc::c_int { return 1 as libc::c_int }
    return sound_LoadTrackFromFile(szFileName);
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn audio_LoadTrackFromBuffer(mut pBuffer:
                                                       *mut libc::c_char,
                                                   mut udwSize: UDWORD)
 -> *mut libc::c_void {
    // if audio not enabled return TRUE to carry on game without audio
    if g_bAudioEnabled == 0 as libc::c_int { return 0 as *mut libc::c_void }
    return sound_LoadTrackFromBuffer(pBuffer, udwSize);
}
//*
//
// Routine to convert wav filename into a track number
// ... This is really not going to be practical on the PSX is it?
//
// What is the point of all the scripts storing .WAV file names like: "Incoming
// Intelligence Report-22hz Mon.wav"
//
//
// when all that is required is a single byte saying which sound effect is
// required.
//
// A typical example of using 50 times the amount of memory that is needed.
//
// =======================================================================================================================
// *  ... bloody PC programmers they're spoiled !
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn audio_SetTrackVals(mut szFileName: *mut libc::c_char,
                                            mut bLoop: BOOL,
                                            mut piID: *mut libc::c_int,
                                            mut iVol: libc::c_int,
                                            mut iPriority: libc::c_int,
                                            mut iAudibleRadius: libc::c_int,
                                            mut VagID: libc::c_int) -> BOOL {
    //~~~~~~~~~~~~~
    let mut psTrack: *mut TRACK = 0 as *mut TRACK;
    //~~~~~~~~~~~~~
    // if audio not enabled return TRUE to carry on game without audio
    if g_bAudioEnabled == 0 as libc::c_int { return 1 as libc::c_int }
    // get track pointer from resource
    psTrack =
        resGetData(b"WAV\x00" as *const u8 as *const libc::c_char as
                       *mut STRING, szFileName) as
            *mut TRACK; //at this point we have 4 valid entries, and 8 invalid -Q
    if psTrack.is_null() {
        debug(LOG_NEVER,
              b"audio_SetTrackVals: track %s resource not found\n\x00" as
                  *const u8 as *const libc::c_char, szFileName);
        return 0 as libc::c_int
    } else {
        // get current ID or spare one
        if audio_GetIDFromStr(szFileName, piID) == 0 as libc::c_int {
            *piID = sound_GetAvailableID()
        }
        if *piID == -(1 as libc::c_int) {
            debug(LOG_NEVER,
                  b"audio_SetTrackVals: couldn\'t get spare track ID\n\x00" as
                      *const u8 as *const libc::c_char);
            return 0 as libc::c_int
        } else {
            return sound_SetTrackVals(psTrack, bLoop, *piID, iVol, iPriority,
                                      iAudibleRadius, VagID)
            //now psTrack should be fully set. -Q
        }
    };
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn audio_SetTrackValsHashName(mut hash: UDWORD,
                                                    mut bLoop: BOOL,
                                                    mut iTrack: libc::c_int,
                                                    mut iVol: libc::c_int,
                                                    mut iPriority:
                                                        libc::c_int,
                                                    mut iAudibleRadius:
                                                        libc::c_int,
                                                    mut VagID: libc::c_int)
 -> BOOL {
    //~~~~~~~~~~~~~
    let mut psTrack: *mut TRACK = 0 as *mut TRACK;
    //~~~~~~~~~~~~~
    // if audio not enabled return TRUE to carry on game without audio
    if g_bAudioEnabled == 0 as libc::c_int { return 1 as libc::c_int }
    // get track pointer from resource
    psTrack =
        resGetDataFromHash(b"WAV\x00" as *const u8 as *const libc::c_char as
                               *mut STRING, hash) as *mut TRACK;
    if psTrack.is_null() {
        return 0 as libc::c_int
    } else {
        return sound_SetTrackVals(psTrack, bLoop, iTrack, iVol, iPriority,
                                  iAudibleRadius, VagID)
    };
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn audio_ReleaseTrack(mut psTrack: *mut TRACK) {
    // return if audio not enabled
    if g_bAudioEnabled == 0 as libc::c_int { return }
    sound_ReleaseTrack(psTrack);
}
//*
//
//
// * audio_CheckSame3DTracksPlaying Reject samples if too many already playing in
// * same area
//
//*
// =======================================================================================================================
// =======================================================================================================================
//
unsafe extern "C" fn audio_CheckSame3DTracksPlaying(mut iTrack: SDWORD,
                                                    mut iX: SDWORD,
                                                    mut iY: SDWORD,
                                                    mut iZ: SDWORD) -> BOOL {
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    let mut iCount: SDWORD = 0;
    let mut iDx: SDWORD = 0;
    let mut iDy: SDWORD = 0;
    let mut iDz: SDWORD = 0;
    let mut iDistSq: SDWORD = 0;
    let mut iMaxDistSq: SDWORD = 0;
    let mut iRad: SDWORD = 0;
    let mut psSample: *mut AUDIO_SAMPLE = 0 as *mut AUDIO_SAMPLE;
    let mut bOK: BOOL = 1 as libc::c_int;
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // return if audio not enabled
    if g_bAudioEnabled == 0 as libc::c_int ||
           g_bAudioPaused == 1 as libc::c_int {
        return 1 as libc::c_int
    }
    iCount = 0 as libc::c_int;
    // loop through 3D sounds and check whether too many already in earshot
    psSample = g_psSampleList;
    while !psSample.is_null() {
        if (*psSample).iTrack == iTrack {
            iDx = iX - (*psSample).x;
            iDy = iY - (*psSample).y;
            iDz = iZ - (*psSample).z;
            iDistSq = iDx * iDx + iDy * iDy + iDz * iDz;
            iRad = sound_GetTrackAudibleRadius(iTrack);
            iMaxDistSq = iRad * iRad;
            if iDistSq < iMaxDistSq { iCount += 1 }
            if iCount > 2 as libc::c_int { bOK = 0 as libc::c_int; break ; }
        }
        psSample = (*psSample).psNext
    }
    return bOK;
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
unsafe extern "C" fn audio_Play3DTrack(mut iX: SDWORD, mut iY: SDWORD,
                                       mut iZ: SDWORD,
                                       mut iTrack: libc::c_int,
                                       mut psObj: *mut libc::c_void,
                                       mut pUserCallback: AUDIO_CALLBACK)
 -> BOOL {
    //~~~~~~~~~~~~~~~~~~~~~~
    let mut psSample: *mut AUDIO_SAMPLE = 0 as *mut AUDIO_SAMPLE;
    //~~~~~~~~~~~~~~~~~~~~~~
    // if audio not enabled return TRUE to carry on game without audio
    if g_bAudioEnabled == 0 as libc::c_int ||
           g_bAudioPaused == 1 as libc::c_int ||
           g_bStopAll == 1 as libc::c_int {
        return 0 as libc::c_int
    }
    if audio_CheckSame3DTracksPlaying(iTrack, iX, iY, iZ) == 0 as libc::c_int
       {
        return 0 as libc::c_int
    }
    heapAlloc(g_psSampleHeap,
              &mut psSample as *mut *mut AUDIO_SAMPLE as *mut libc::c_void as
                  *mut *mut libc::c_void);
    if psSample.is_null() {
        return 0 as libc::c_int
    } else {
        // setup sample
        memset(psSample as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<AUDIO_SAMPLE>() as
                   libc::c_ulong); // [check] -Q
        (*psSample).iTrack = iTrack;
        (*psSample).x = iX;
        (*psSample).y = iY;
        (*psSample).z = iZ;
        (*psSample).bRemove = 0 as libc::c_int;
        (*psSample).psObj = psObj;
        (*psSample).pCallback = pUserCallback;
        // add sample to list if able to play
        if sound_Play3DTrack(psSample) == 1 as libc::c_int {
            audio_AddSampleToHead(&mut g_psSampleList, psSample);
            return 1 as libc::c_int
        } else {
            debug(LOG_NEVER,
                  b"audio_Play3DTrack: couldn\'t play sample\n\x00" as
                      *const u8 as *const libc::c_char);
            heapFree(g_psSampleHeap, psSample as *mut libc::c_void);
            return 0 as libc::c_int
        }
    };
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn audio_PlayStaticTrack(mut iMapX: SDWORD,
                                               mut iMapY: SDWORD,
                                               mut iTrack: libc::c_int)
 -> BOOL {
    //~~~~~~~~~~~~~~~
    let mut iX: SDWORD = 0;
    let mut iY: SDWORD = 0;
    let mut iZ: SDWORD = 0;
    //~~~~~~~~~~~~~~~
    // if audio not enabled return TRUE to carry on game without audio
    if g_bAudioEnabled == 0 as libc::c_int { return 0 as libc::c_int }
    audio_GetStaticPos(iMapX, iMapY, &mut iX, &mut iY, &mut iZ);
    return audio_Play3DTrack(iX, iY, iZ, iTrack, 0 as *mut libc::c_void,
                             None);
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn audio_PlayObjStaticTrack(mut psObj:
                                                      *mut libc::c_void,
                                                  mut iTrack: libc::c_int)
 -> BOOL {
    //~~~~~~~~~~~~~~~
    let mut iX: SDWORD = 0;
    let mut iY: SDWORD = 0;
    let mut iZ: SDWORD = 0;
    //~~~~~~~~~~~~~~~
    // if audio not enabled return TRUE to carry on game without audio
    if g_bAudioEnabled == 0 as libc::c_int { return 0 as libc::c_int }
    audio_GetObjectPos(psObj, &mut iX, &mut iY, &mut iZ);
    return audio_Play3DTrack(iX, iY, iZ, iTrack, psObj, None);
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn audio_PlayObjStaticTrackCallback(mut psObj:
                                                              *mut libc::c_void,
                                                          mut iTrack:
                                                              libc::c_int,
                                                          mut pUserCallback:
                                                              AUDIO_CALLBACK)
 -> BOOL {
    //~~~~~~~~~~~~~~~
    let mut iX: SDWORD = 0;
    let mut iY: SDWORD = 0;
    let mut iZ: SDWORD = 0;
    //~~~~~~~~~~~~~~~
    // if audio not enabled return TRUE to carry on game without audio
    if g_bAudioEnabled == 0 as libc::c_int { return 0 as libc::c_int }
    audio_GetObjectPos(psObj, &mut iX, &mut iY, &mut iZ);
    return audio_Play3DTrack(iX, iY, iZ, iTrack, psObj, pUserCallback);
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn audio_PlayObjDynamicTrack(mut psObj:
                                                       *mut libc::c_void,
                                                   mut iTrack: libc::c_int,
                                                   mut pUserCallback:
                                                       AUDIO_CALLBACK)
 -> BOOL {
    //~~~~~~~~~~~~~~~
    let mut iX: SDWORD = 0;
    let mut iY: SDWORD = 0;
    let mut iZ: SDWORD = 0;
    //~~~~~~~~~~~~~~~
    // if audio not enabled return TRUE to carry on game without audio
    if g_bAudioEnabled == 0 as libc::c_int { return 0 as libc::c_int }
    audio_GetObjectPos(psObj, &mut iX, &mut iY, &mut iZ);
    return audio_Play3DTrack(iX, iY, iZ, iTrack, psObj, pUserCallback);
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn audio_PlayStream(mut szFileName: *mut libc::c_char,
                                          mut iVol: SDWORD,
                                          mut pUserCallback: AUDIO_CALLBACK)
 -> BOOL {
    //~~~~~~~~~~~~~~~~~~~~~~
    let mut psSample: *mut AUDIO_SAMPLE = 0 as *mut AUDIO_SAMPLE;
    //~~~~~~~~~~~~~~~~~~~~~~
    // if audio not enabled return TRUE to carry on game without audio
    if g_bAudioEnabled == 0 as libc::c_int { return 0 as libc::c_int }
    heapAlloc(g_psSampleHeap,
              &mut psSample as *mut *mut AUDIO_SAMPLE as *mut libc::c_void as
                  *mut *mut libc::c_void);
    if !psSample.is_null() {
        memset(psSample as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<AUDIO_SAMPLE>() as libc::c_ulong);
        (*psSample).pCallback = pUserCallback;
        (*psSample).bRemove = 0 as libc::c_int;
        audio_Set3DVolume(100 as libc::c_long as SDWORD);
        if sound_PlayStream(psSample, szFileName, iVol) == 1 as libc::c_int {
            return 1 as libc::c_int
        }
    }
    return 0 as libc::c_int;
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn audio_StopObjTrack(mut psObj: *mut libc::c_void,
                                            mut iTrack: libc::c_int) {
    //~~~~~~~~~~~~~~~~~~~~~~
    let mut psSample: *mut AUDIO_SAMPLE = 0 as *mut AUDIO_SAMPLE;
    //~~~~~~~~~~~~~~~~~~~~~~
    // return if audio not enabled
    if g_bAudioEnabled == 0 as libc::c_int || g_bStopAll == 1 as libc::c_int {
        return
    }
    // find sample
    psSample = g_psSampleList;
    while !psSample.is_null() {
        if (*psSample).psObj == psObj && (*psSample).iTrack == iTrack {
            break ;
        }
        // get next sample from hash table
        psSample = (*psSample).psNext
    }
    if !psSample.is_null() { sound_StopTrack(psSample); };
}
//*
//
// audio_PlayTrack Play immediate 2D FX track
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn audio_PlayTrack(mut iTrack: libc::c_int) {
    //~~~~~~~~~~~~~~~~~~~~~~
    let mut psSample: *mut AUDIO_SAMPLE = 0 as *mut AUDIO_SAMPLE;
    //~~~~~~~~~~~~~~~~~~~~~~
    // return if audio not enabled
    if g_bAudioEnabled == 0 as libc::c_int ||
           g_bAudioPaused == 1 as libc::c_int ||
           g_bStopAll == 1 as libc::c_int {
        return
    }
    heapAlloc(g_psSampleHeap,
              &mut psSample as *mut *mut AUDIO_SAMPLE as *mut libc::c_void as
                  *mut *mut libc::c_void);
    if !psSample.is_null() {
        // setup sample
        memset(psSample as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<AUDIO_SAMPLE>() as
                   libc::c_ulong); // Set everything to 0  (looks good) -Q
        (*psSample).iTrack = iTrack;
        (*psSample).bRemove = 0 as libc::c_int;
        // add sample to list if able to play
        if sound_Play2DTrack(psSample, 0 as libc::c_int) == 1 as libc::c_int {
            audio_AddSampleToHead(&mut g_psSampleList, psSample);
        } else {
            debug(LOG_NEVER,
                  b"audio_PlayTrack: couldn\'t play sample\n\x00" as *const u8
                      as *const libc::c_char);
            heapFree(g_psSampleHeap, psSample as *mut libc::c_void);
        }
    };
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn audio_StopTrack(mut iTrack: libc::c_int) {
    // return if audio not enabled
    if g_bAudioEnabled == 0 as libc::c_int { return };
    //
	// iTrack;
	//
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn audio_SetTrackPan(mut iTrack: libc::c_int,
                                           mut iPan: libc::c_int) {
    // return if audio not enabled
    if g_bAudioEnabled == 0 as libc::c_int { return };
    //
	// iTrack;
	//
	//
	// iPan;
	//
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn audio_SetTrackVol(mut iTrack: libc::c_int,
                                           mut iVol: libc::c_int) {
    // return if audio not enabled
    if g_bAudioEnabled == 0 as libc::c_int { return };
    //
	// iTrack;
	//
	//
	// iVol;
	//
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn audio_SetTrackFreq(mut iTrack: libc::c_int,
                                            mut iFreq: libc::c_int) {
    // return if audio not enabled
    if g_bAudioEnabled == 0 as libc::c_int { return };
    //
	// iTrack;
	//
	//
	// iFreq;
	//
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn audio_PauseAll() {
    // return if audio not enabled
    if g_bAudioEnabled == 0 as libc::c_int { return }
    g_bAudioPaused = 1 as libc::c_int;
    sound_PauseAll();
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn audio_ResumeAll() {
    // return if audio not enabled
    if g_bAudioEnabled == 0 as libc::c_int { return }
    g_bAudioPaused = 0 as libc::c_int;
    sound_ResumeAll();
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn audio_StopAll() {
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    let mut psSample: *mut AUDIO_SAMPLE = 0 as *mut AUDIO_SAMPLE;
    let mut psSampleTemp: *mut AUDIO_SAMPLE = 0 as *mut AUDIO_SAMPLE;
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // return if audio not enabled
    if g_bAudioEnabled == 0 as libc::c_int { return }
    debug(LOG_NEVER,
          b"audio_StopAll called\n\x00" as *const u8 as *const libc::c_char);
    g_bStopAll = 1 as libc::c_int;
    //
	// * empty list - audio_Update will free samples because callbacks have to come in
	// * first
	//
    psSample = g_psSampleList;
    while !psSample.is_null() {
        sound_StopTrack(psSample);
        psSample = (*psSample).psNext
    }
    // empty sample queue
    psSample = g_psSampleQueue;
    while !psSample.is_null() {
        psSampleTemp = (*psSample).psNext;
        heapFree(g_psSampleHeap, psSample as *mut libc::c_void);
        psSample = psSampleTemp
    }
    g_psSampleQueue = 0 as *mut AUDIO_SAMPLE;
    g_bStopAll = 0 as libc::c_int;
    debug(LOG_NEVER,
          b"audio_StopAll done\n\x00" as *const u8 as *const libc::c_char);
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn audio_CheckAllUnloaded() {
    sound_CheckAllUnloaded();
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn audio_GetTrackID(mut szFileName: *mut libc::c_char)
 -> SDWORD {
    //~~~~~~~~~~~~~
    let mut psTrack: *mut TRACK = 0 as *mut TRACK;
    let mut iID: SDWORD = 0;
    //~~~~~~~~~~~~~
    // return if audio not enabled
    if g_bAudioEnabled == 0 as libc::c_int {
        return -(3 as libc::c_int)
    } else {
        psTrack =
            resGetData(b"WAV\x00" as *const u8 as *const libc::c_char as
                           *mut STRING, szFileName) as *mut TRACK;
        if psTrack.is_null() {
            return -(3 as libc::c_int)
        } else { iID = sound_GetTrackID(psTrack); return iID }
    };
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn audio_GetTrackIDFromHash(mut hash: UDWORD)
 -> SDWORD {
    //~~~~~~~~~~~~~
    let mut psTrack: *mut TRACK = 0 as *mut TRACK;
    let mut iID: SDWORD = 0;
    //~~~~~~~~~~~~~
    // return if audio not enabled
    if g_bAudioEnabled == 0 as libc::c_int {
        return -(3 as libc::c_int)
    } else {
        psTrack =
            resGetDataFromHash(b"WAV\x00" as *const u8 as *const libc::c_char
                                   as *mut STRING, hash) as *mut TRACK;
        if psTrack.is_null() {
            return -(3 as libc::c_int)
        } else { iID = sound_GetTrackID(psTrack); return iID }
    };
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn audio_GetAvailableID() -> SDWORD {
    // return if audio not enabled
    if g_bAudioEnabled == 0 as libc::c_int {
        return 0 as libc::c_int
    } else { return sound_GetAvailableID() };
}
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn audio_Get3DVolume() -> SDWORD { return g_i3DVolume; }
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn audio_Set3DVolume(mut iVol: SDWORD) {
    g_i3DVolume = iVol;
}
//*
//
// audio_GetMixVol iVol and audio_Get3DVolume need to be scaled by AUDIO_VOL_RANGE
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn audio_GetMixVol(mut iVol: SDWORD) -> SDWORD {
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    let mut iMixVol: SDWORD =
        ((iVol * sound_GetMaxVolume() * audio_Get3DVolume()) as libc::c_long /
             ((100 as libc::c_long - 0 as libc::c_long) *
                  (100 as libc::c_long - 0 as libc::c_long))) as SDWORD;
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    return iMixVol;
}
//*
//
//
// * audio_GetSampleMixVol iVol, audio_Get3DVolume and sound_GetTrackVolume all need
// * to be scaled by AUDIO_VOL_RANGE
//
//*
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn audio_GetSampleMixVol(mut psSample:
                                                   *mut AUDIO_SAMPLE,
                                               mut iVol: SDWORD,
                                               mut bScale3D: BOOL) -> SDWORD {
    //~~~~~~~~~~~~
    let mut iMixVol: SDWORD = 0;
    //~~~~~~~~~~~~
    iMixVol =
        ((iVol * sound_GetMaxVolume()) as libc::c_long /
             (100 as libc::c_long - 0 as libc::c_long)) as SDWORD;
    iMixVol =
        ((iMixVol * sound_GetTrackVolume((*psSample).iTrack)) as libc::c_long
             / (100 as libc::c_long - 0 as libc::c_long)) as SDWORD;
    if bScale3D != 0 {
        iMixVol =
            ((iMixVol * audio_Get3DVolume()) as libc::c_long /
                 (100 as libc::c_long - 0 as libc::c_long)) as SDWORD
    }
    return iMixVol;
}
//*
//
