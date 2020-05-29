use ::libc;
extern "C" {
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_uint)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_uint;
    /*
 * mem.h
 *
 * Interface to the malloc/free replacements
 */
    /* DEBUG_MALLOC == TRUE uses debugging malloc and free
   DEBUG_MALLOC == FALSE uses normal malloc and free */
    /* Function Prototypes */
    /* Initialise the memory system */
    /* Shutdown the memory system */
    /* Set a block heap to use for all memory allocation rather than standard malloc/free */
    /* Get the current block heap */
    /* malloc replacements */
    #[no_mangle]
    fn memMallocRelease(Size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn memFreeRelease(pMemToFree: *mut libc::c_void);
    #[no_mangle]
    fn memGetBlockHeap() -> *mut _block_heap;
    #[no_mangle]
    fn memSetBlockHeap(psHeap: *mut _block_heap);
    #[no_mangle]
    fn abort() -> !;
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
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn loadFile(pFileName: *const libc::c_char,
                ppFileData: *mut *mut libc::c_char, pFileSize: *mut UDWORD)
     -> BOOL;
    #[no_mangle]
    fn HashString(String: *mut libc::c_char) -> UINT;
    #[no_mangle]
    fn HashStringIgnoreCase(String: *mut libc::c_char) -> UINT;
    /* *********************************************************************************/
/*                    function prototypes                                         */
    // initialise the block system
    // shutdown the block system
    // Create a new block heap
    // Release a block heap
    // Allocate some memory from a block heap
    // return a chunk of memory to the block
// this only does anything whith DEBUG_BLOCK defined
    // Reset a block heap
    // Find which block a pointer is from if any
    // check if a pointer is valid in a block
    // check if a pointer is valid in any currently allocated block
    // Note the call position for a blkAlloc or blkFree
    #[no_mangle]
    fn blockUnsuspendUsage();
    #[no_mangle]
    fn blockSuspendUsage();
    /* Maximum number of TEXT items in any one Yacc rule */
    /* The initial resource directory and the current resource directory */
    /* Set the current input buffer for the lexer - used by resLoad */
    #[no_mangle]
    fn resSetInputBuffer(pBuffer: *mut libc::c_char, size: UDWORD);
    /* Call the yacc parser */
    #[no_mangle]
    fn res_parse() -> libc::c_int;
}
pub type size_t = libc::c_uint;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type UBYTE = libc::c_uchar;
pub type STRING = libc::c_char;
pub type UWORD = libc::c_ushort;
/*
 * types.h
 *
 * Simple type definitions.
 *
 */
/* Check the header files have been included from frame.h if they
 * are used outside of the framework library.
 */
// WIN32
/* Compilers that have support for C99 have all of the above defined in stdint.h */
// _MSC_VER
/* Basic numeric types */
pub type UDWORD = libc::c_uint;
pub type SDWORD = libc::c_int;
pub type BOOL = libc::c_int;
pub type UINT = libc::c_uint;
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
 * FrameResources.h
 *
 * Resource file processing functions
 */
/* Maximum number of characters in a resource type */
/* Maximum number of characters in a resource ID */
/* Function pointer for a function that loads from a memory buffer */
pub type RES_BUFFERLOAD
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_char, _: UDWORD,
                                _: *mut *mut libc::c_void) -> BOOL>;
/* Function pointer for a function that loads from a filename */
pub type RES_FILELOAD
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_char,
                                _: *mut *mut libc::c_void) -> BOOL>;
/* Function pointer for releasing a resource loaded by the above functions */
pub type RES_FREE = Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
/* callback type for resload display callback*/
pub type RESLOAD_CALLBACK = Option<unsafe extern "C" fn() -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct res_data {
    pub aID: [STRING; 40],
    pub pData: *mut libc::c_void,
    pub blockID: SDWORD,
    pub HashedID: UDWORD,
    pub psNext: *mut res_data,
    pub usage: UDWORD,
}
pub type RES_DATA = res_data;
// New reduced resource type ... specially for PSX
// These types  are statically defined in data.c
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _res_type {
    pub aType: [STRING; 20],
    pub buffLoad: RES_BUFFERLOAD,
    pub release: RES_FREE,
    pub psRes: *mut RES_DATA,
    pub HashedType: UDWORD,
    pub fileLoad: RES_FILELOAD,
    pub psNext: *mut _res_type,
}
pub type RES_TYPE = _res_type;
pub type RESPRELOAD_CALLBACK
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_char, _: *mut libc::c_char,
                                _: *mut libc::c_char) -> BOOL>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RESOURCEFILE {
    pub pBuffer: *mut libc::c_char,
    pub size: UDWORD,
    pub type_0: UBYTE,
}
pub type BLOCK_HEAP = _block_heap;
/*
 * FrameResource.c
 *
 * Framework Resource file processing functions
 *
 */
// Local prototypes
static mut psResTypes: *mut RES_TYPE = 0 as *const RES_TYPE as *mut RES_TYPE;
/* The initial resource directory and the current resource directory */
#[no_mangle]
pub static mut aResDir: [STRING; 255] = [0; 255];
#[no_mangle]
pub static mut aCurrResDir: [STRING; 255] = [0; 255];
// the current resource block ID
static mut resBlockID: SDWORD = 0;
// buffer to load file data into
static mut pFileBuffer: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
static mut fileBufferSize: SDWORD = 0 as libc::c_int;
// callback to resload screen.
static mut resLoadCallback: RESLOAD_CALLBACK = None;
static mut resPreLoadCallback: RESPRELOAD_CALLBACK = None;
#[no_mangle]
pub unsafe extern "C" fn resSetPreLoadCallback(mut funcToCall:
                                                   RESPRELOAD_CALLBACK) {
    resPreLoadCallback = funcToCall;
}
/* set the callback function for the res loader*/
#[no_mangle]
pub unsafe extern "C" fn resSetLoadCallback(mut funcToCall:
                                                RESLOAD_CALLBACK) {
    resLoadCallback = funcToCall;
}
/* do the callback for the resload display function */
#[no_mangle]
pub unsafe extern "C" fn resDoResLoadCallback() {
    if resLoadCallback.is_some() {
        resLoadCallback.expect("non-null function pointer")();
    };
}
/* Initialise the resource module */
#[no_mangle]
pub unsafe extern "C" fn resInitialise() -> BOOL {
    if psResTypes.is_null() {
    } else {
        debug(LOG_ERROR,
              b"resInitialise: resource module hasn\'t been shut down??\x00"
                  as *const u8 as *const libc::c_char);
    };
    if psResTypes.is_null() {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"frameresource.c\x00" as *const u8 as *const libc::c_char,
              70 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"resInitialise\x00")).as_ptr(),
              b"psResTypes == NULL\x00" as *const u8 as *const libc::c_char);
    };
    psResTypes = 0 as *mut RES_TYPE;
    resBlockID = 0 as libc::c_int;
    resLoadCallback = None;
    resPreLoadCallback = None;
    ResetResourceFile();
    return 1 as libc::c_int;
}
/* Shutdown the resource module */
#[no_mangle]
pub unsafe extern "C" fn resShutDown() {
    if !psResTypes.is_null() {
        debug(LOG_WZ,
              b"resShutDown: warning resources still allocated\x00" as
                  *const u8 as *const libc::c_char);
        resReleaseAll();
    };
}
// set the base resource directory
#[no_mangle]
pub unsafe extern "C" fn resSetBaseDir(mut pResDir: *mut STRING) {
    strncpy(aResDir.as_mut_ptr(), pResDir,
            (255 as libc::c_int - 1 as libc::c_int) as libc::c_uint);
}
/* Parse the res file */
#[no_mangle]
pub unsafe extern "C" fn resLoad(mut pResFile: *mut STRING,
                                 mut blockID: SDWORD,
                                 mut pLoadBuffer: *mut libc::c_char,
                                 mut bufferSize: SDWORD,
                                 mut psMemHeap: *mut BLOCK_HEAP) -> BOOL {
    let mut pBuffer: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut size: UDWORD = 0;
    let mut psOldHeap: *mut BLOCK_HEAP = 0 as *mut BLOCK_HEAP;
    strcpy(aCurrResDir.as_mut_ptr(), aResDir.as_mut_ptr());
    // Note the buffer for file data
    pFileBuffer = pLoadBuffer;
    fileBufferSize = bufferSize;
    // Note the block id number
    resBlockID = blockID;
    debug(LOG_WZ,
          b"resLoad: loading %s\x00" as *const u8 as *const libc::c_char,
          pResFile);
    // make sure the WRF doesn't get loaded into a block heap
    psOldHeap = memGetBlockHeap();
    memSetBlockHeap(0 as *mut _block_heap);
    // Load the RES file; allocate memory for a wrf, and load it
    if loadFile(pResFile, &mut pBuffer, &mut size) == 0 {
        debug(LOG_ERROR,
              b"resLoad: failed to load %s\x00" as *const u8 as
                  *const libc::c_char, pResFile);
        return 0 as libc::c_int
    }
    // now set the memory system to use the block heap
    memSetBlockHeap(psMemHeap);
    // and parse it
    resSetInputBuffer(pBuffer, size);
    if res_parse() != 0 as libc::c_int {
        debug(LOG_ERROR,
              b"resLoad: failed to parse %s\x00" as *const u8 as
                  *const libc::c_char, pResFile);
        return 0 as libc::c_int
    }
    // reset the memory system
    memSetBlockHeap(psOldHeap);
    memFreeRelease(pBuffer as *mut libc::c_void);
    pBuffer = 0 as *mut libc::c_char;
    return 1 as libc::c_int;
}
/* Allocate a RES_TYPE structure */
unsafe extern "C" fn resAlloc(mut pType: *mut STRING,
                              mut ppsFunc: *mut *mut RES_TYPE) -> BOOL {
    let mut psT: *mut RES_TYPE = 0 as *mut RES_TYPE;
    // Allocate the memory
    psT =
        memMallocRelease(::std::mem::size_of::<RES_TYPE>() as libc::c_ulong)
            as *mut RES_TYPE;
    if psT.is_null() {
        debug(LOG_ERROR,
              b"resAlloc: Out of memory\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    // setup the structure
    strncpy((*psT).aType.as_mut_ptr(), pType,
            (20 as libc::c_int - 1 as libc::c_int) as
                libc::c_uint); // store a hased version for super speed !
    (*psT).aType[(20 as libc::c_int - 1 as libc::c_int) as usize] =
        0 as libc::c_int as STRING;
    (*psT).HashedType = HashString((*psT).aType.as_mut_ptr());
    (*psT).psRes = 0 as *mut RES_DATA;
    *ppsFunc = psT;
    return 1 as libc::c_int;
}
/* Add a buffer load function for a file type */
#[no_mangle]
pub unsafe extern "C" fn resAddBufferLoad(mut pType: *mut STRING,
                                          mut buffLoad: RES_BUFFERLOAD,
                                          mut release: RES_FREE) -> BOOL {
    let mut psT: *mut RES_TYPE = 0 as *mut RES_TYPE;
    if resAlloc(pType, &mut psT) == 0 { return 0 as libc::c_int }
    (*psT).buffLoad = buffLoad;
    (*psT).fileLoad = None;
    (*psT).release = release;
    (*psT).psNext = psResTypes;
    psResTypes = psT;
    return 1 as libc::c_int;
}
/* Add a file name load function for a file type */
#[no_mangle]
pub unsafe extern "C" fn resAddFileLoad(mut pType: *mut STRING,
                                        mut fileLoad: RES_FILELOAD,
                                        mut release: RES_FREE) -> BOOL {
    let mut psT: *mut RES_TYPE = 0 as *mut RES_TYPE;
    if resAlloc(pType, &mut psT) == 0 { return 0 as libc::c_int }
    (*psT).buffLoad = None;
    (*psT).fileLoad = fileLoad;
    (*psT).release = release;
    (*psT).psNext = psResTypes;
    psResTypes = psT;
    return 1 as libc::c_int;
}
// Make a string lower case
#[no_mangle]
pub unsafe extern "C" fn resToLower(mut pStr: *mut STRING) {
    while *pStr as libc::c_int != 0 as libc::c_int {
        if *(*__ctype_b_loc()).offset(*pStr as libc::c_int as isize) as
               libc::c_int &
               _ISupper as libc::c_int as libc::c_ushort as libc::c_int != 0 {
            *pStr =
                (*pStr as libc::c_int -
                     ('A' as i32 - 'a' as i32) as STRING as libc::c_int) as
                    STRING
        }
        pStr = pStr.offset(1 as libc::c_int as isize)
    };
}
static mut LastResourceFilename: [libc::c_char; 255] = [0; 255];
// Returns the filename of the last resource file loaded
#[no_mangle]
pub unsafe extern "C" fn GetLastResourceFilename() -> *mut libc::c_char {
    return LastResourceFilename.as_mut_ptr();
}
// Set the resource name of the last resource file loaded
#[no_mangle]
pub unsafe extern "C" fn SetLastResourceFilename(mut pName:
                                                     *mut libc::c_char) {
    strncpy(LastResourceFilename.as_mut_ptr(), pName,
            (255 as libc::c_int - 1 as libc::c_int) as libc::c_uint);
    LastResourceFilename[(255 as libc::c_int - 1 as libc::c_int) as usize] =
        0 as libc::c_int as libc::c_char;
}
static mut LastHashName: UDWORD = 0;
// Returns the filename of the last resource file loaded
#[no_mangle]
pub unsafe extern "C" fn GetLastHashName() -> UDWORD { return LastHashName; }
//return last imd resource
// Set the resource name of the last resource file loaded
// Returns the filename of the last resource file loaded
// Set the resource name of the last resource file loaded
// Set the resource name of the last resource file loaded
#[no_mangle]
pub unsafe extern "C" fn SetLastHashName(mut HashName: UDWORD) {
    LastHashName = HashName;
}
static mut LoadedResourceFiles: [RESOURCEFILE; 6] =
    [RESOURCEFILE{pBuffer: 0 as *const libc::c_char as *mut libc::c_char,
                  size: 0,
                  type_0: 0,}; 6];
// prototypes
// Clear out the resource list ... needs to be called during init.
unsafe extern "C" fn ResetResourceFile() {
    let mut i: UWORD = 0;
    i = 0 as libc::c_int as UWORD;
    while (i as libc::c_int) < 6 as libc::c_int {
        LoadedResourceFiles[i as usize].type_0 = 0 as libc::c_int as UBYTE;
        i = i.wrapping_add(1)
    };
}
// Returns an empty resource entry or -1 if none exsist
#[no_mangle]
pub unsafe extern "C" fn FindEmptyResourceFile() -> SDWORD {
    let mut i: UWORD = 0;
    i = 0 as libc::c_int as UWORD;
    while (i as libc::c_int) < 6 as libc::c_int {
        if LoadedResourceFiles[i as usize].type_0 as libc::c_int ==
               0 as libc::c_int {
            return i as SDWORD
        }
        i = i.wrapping_add(1)
    }
    return -(1 as libc::c_int);
    // ERROR
}
// Get a resource data file ... either loads it or just returns a pointer
#[no_mangle]
pub unsafe extern "C" fn RetreiveResourceFile(mut ResourceName:
                                                  *mut libc::c_char,
                                              mut NewResource:
                                                  *mut *mut RESOURCEFILE)
 -> BOOL {
    let mut ResID: SDWORD = 0; // all resource files are full
    let mut ResData: *mut RESOURCEFILE = 0 as *mut RESOURCEFILE;
    let mut size: UDWORD = 0;
    let mut pBuffer: *mut libc::c_char = 0 as *mut libc::c_char;
    ResID = FindEmptyResourceFile();
    if ResID == -(1 as libc::c_int) { return 0 as libc::c_int }
    ResData =
        &mut *LoadedResourceFiles.as_mut_ptr().offset(ResID as isize) as
            *mut RESOURCEFILE;
    *NewResource = ResData;
    if !pFileBuffer.is_null() &&
           loadFile(ResourceName, &mut pBuffer, &mut size) != 0 {
        (*ResData).type_0 = 1 as libc::c_int as UBYTE;
        (*ResData).size = size;
        (*ResData).pBuffer = pBuffer;
        return 1 as libc::c_int
    }
    blockSuspendUsage();
    // This is needed for files that do not fit in the WDG cache ... (VAB file for example)
    if loadFile(ResourceName, &mut pBuffer, &mut size) == 0 {
        return 0 as libc::c_int
    }
    blockUnsuspendUsage();
    (*ResData).type_0 = 2 as libc::c_int as UBYTE;
    (*ResData).size = size;
    (*ResData).pBuffer = pBuffer;
    return 1 as libc::c_int;
}
// Free up the file depending on what type it is
#[no_mangle]
pub unsafe extern "C" fn FreeResourceFile(mut OldResource:
                                              *mut RESOURCEFILE) {
    match (*OldResource).type_0 as libc::c_int {
        2 => {
            memFreeRelease((*OldResource).pBuffer as *mut libc::c_void);
            (*OldResource).pBuffer = 0 as *mut libc::c_char
        }
        _ => { }
    }
    // Remove from the list
    (*OldResource).type_0 = 0 as libc::c_int as UBYTE;
}
#[no_mangle]
pub unsafe extern "C" fn resDataInit(mut psRes: *mut RES_DATA,
                                     mut DebugName: *mut STRING,
                                     mut DataIDHash: UDWORD,
                                     mut pData: *mut libc::c_void,
                                     mut BlockID: UDWORD) {
    (*psRes).pData = pData;
    (*psRes).blockID = resBlockID;
    (*psRes).HashedID = DataIDHash;
    strcpy((*psRes).aID.as_mut_ptr(), DebugName);
    (*psRes).usage = 0 as libc::c_int as UDWORD;
}
/* Call the load function for a file */
#[no_mangle]
pub unsafe extern "C" fn resLoadFile(mut pType: *mut STRING,
                                     mut pFile: *mut STRING) -> BOOL {
    let mut psT: *mut RES_TYPE = 0 as *mut RES_TYPE;
    let mut pData: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut psRes: *mut RES_DATA = 0 as *mut RES_DATA;
    let mut aFileName: [STRING; 255] = [0; 255];
    let mut loadresource: BOOL = 0;
    let mut HashedName: UDWORD = 0;
    loadresource = 1 as libc::c_int;
    if resPreLoadCallback.is_some() {
        loadresource =
            resPreLoadCallback.expect("non-null function pointer")(pType,
                                                                   pFile,
                                                                   aCurrResDir.as_mut_ptr())
    }
    if loadresource == 1 as libc::c_int {
        let mut HashedType: UDWORD = HashString(pType);
        psT = psResTypes;
        while !psT.is_null() {
            if (*psT).HashedType == HashedType { break ; }
            psT = (*psT).psNext
        }
        if psT.is_null() {
            debug(LOG_WZ,
                  b"resLoadFile: Unknown type: %s\x00" as *const u8 as
                      *const libc::c_char, pType);
            return 0 as libc::c_int
        }
        HashedName = HashStringIgnoreCase(pFile);
        psRes = (*psT).psRes;
        while !psRes.is_null() {
            if (*psRes).HashedID == HashedName {
                debug(LOG_WZ,
                      b"resLoadFile: Duplicate file name: %s (hash %x) for type %s\x00"
                          as *const u8 as *const libc::c_char, pFile,
                      HashedName, (*psT).aType.as_mut_ptr());
                // assume that they are actually both the same and silently fail
				// lovely little hack to allow some files to be loaded from disk (believe it or not!).
                return 1 as libc::c_int
            }
            psRes = (*psRes).psNext
        }
        // Create the file name
        if strlen(aCurrResDir.as_mut_ptr()).wrapping_add(strlen(pFile)).wrapping_add(1
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_uint)
               >= 255 as libc::c_int as libc::c_uint {
            debug(LOG_ERROR,
                  b"resLoadFile: Filename too long!! %s%s\x00" as *const u8 as
                      *const libc::c_char, aCurrResDir.as_mut_ptr(),
                  pFile); // Save the filename in case any routines need it
            return 0 as libc::c_int
        }
        strcpy(aFileName.as_mut_ptr(), aCurrResDir.as_mut_ptr());
        strcat(aFileName.as_mut_ptr(), pFile);
        strcpy(LastResourceFilename.as_mut_ptr(), pFile);
        resToLower(LastResourceFilename.as_mut_ptr());
        SetLastHashName(HashStringIgnoreCase(LastResourceFilename.as_mut_ptr()));
        // load the resource
        if (*psT).buffLoad.is_some() {
            let mut Resource: *mut RESOURCEFILE = 0 as *mut RESOURCEFILE;
            let mut Result: BOOL = 0;
            Result =
                RetreiveResourceFile(aFileName.as_mut_ptr(), &mut Resource);
            if Result == 0 as libc::c_int {
                debug(LOG_ERROR,
                      b"resLoadFile: Unable to retreive resource - %s\x00" as
                          *const u8 as *const libc::c_char,
                      aFileName.as_mut_ptr());
                return 0 as libc::c_int
            }
            // do callback.
            if (*psT).buffLoad.expect("non-null function pointer")((*Resource).pBuffer,
                                                                   (*Resource).size,
                                                                   &mut pData)
                   == 0 {
                FreeResourceFile(Resource);
                (*psT).release.expect("non-null function pointer")(pData);
                return 0 as libc::c_int
            }
            FreeResourceFile(Resource);
            resDoResLoadCallback();
        } else {
            debug(LOG_ERROR,
                  b"resLoadFile:  No load functions for this type (%s)\x00" as
                      *const u8 as *const libc::c_char, pType);
            return 0 as libc::c_int
        }
        // Now process the buffer data
        // Set up the resource structure if there is something to store
        if !pData.is_null() {
            psRes =
                memMallocRelease(::std::mem::size_of::<RES_DATA>() as
                                     libc::c_ulong) as *mut RES_DATA;
            if psRes.is_null() {
                debug(LOG_ERROR,
                      b"resLoadFile: Out of memory\x00" as *const u8 as
                          *const libc::c_char);
                (*psT).release.expect("non-null function pointer")(pData);
                return 0 as libc::c_int
            }
            // LastResourceFilename may have been changed (e.g. by TEXPAGE loading)
            resDataInit(psRes, LastResourceFilename.as_mut_ptr(),
                        HashStringIgnoreCase(LastResourceFilename.as_mut_ptr()),
                        pData, resBlockID as UDWORD);
            // Add the resource to the list
            (*psRes).psNext = (*psT).psRes;
            (*psT).psRes = psRes
        }
    }
    return 1 as libc::c_int;
}
/* Return the resource for a type and hashedname */
#[no_mangle]
pub unsafe extern "C" fn resGetDataFromHash(mut pType: *mut STRING,
                                            mut HashedID: UDWORD)
 -> *mut libc::c_void {
    let mut psT: *mut RES_TYPE = 0 as *mut RES_TYPE;
    let mut psRes: *mut RES_DATA = 0 as *mut RES_DATA;
    let mut HashedType: UDWORD = 0;
    // Find the correct type
    HashedType = HashString(pType); // la da la
    psT = psResTypes;
    while !psT.is_null() {
        if (*psT).HashedType == HashedType { break ; }
        psT = (*psT).psNext
    }
    if psT.is_null() {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"resGetData: Unknown type: %s\x00" as *const u8 as
                      *const libc::c_char, pType);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"frameresource.c\x00" as *const u8 as *const libc::c_char,
                  507 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 19],
                                            &[libc::c_char; 19]>(b"resGetDataFromHash\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as *mut libc::c_void
    }
    //		UDWORD HashedID=HashStringIgnoreCase(pID);
    psRes = (*psT).psRes;
    while !psRes.is_null() {
        if (*psRes).HashedID == HashedID { break ; }
        psRes = (*psRes).psNext
    }
    if psRes.is_null() {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"resGetDataFromHash: Unknown ID:\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"frameresource.c\x00" as *const u8 as *const libc::c_char,
                  525 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 19],
                                            &[libc::c_char; 19]>(b"resGetDataFromHash\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as *mut libc::c_void
    }
    (*psRes).usage =
        ((*psRes).usage as
             libc::c_uint).wrapping_add(1 as libc::c_int as libc::c_uint) as
            UDWORD as UDWORD;
    return (*psRes).pData;
}
/* Return the resource for a type and ID */
#[no_mangle]
pub unsafe extern "C" fn resGetData(mut pType: *mut STRING,
                                    mut pID: *mut STRING)
 -> *mut libc::c_void {
    let mut psT: *mut RES_TYPE = 0 as *mut RES_TYPE;
    let mut psRes: *mut RES_DATA = 0 as *mut RES_DATA;
    let mut HashedType: UDWORD = 0;
    // Find the correct type
    HashedType = HashString(pType); // la da la
    //printf("[resGetData] entering with %s / %s  = %0x\n",pID,pType,HashedType);
    psT = psResTypes;
    while !psT.is_null() {
        if (*psT).HashedType == HashedType { break ; }
        psT = (*psT).psNext
    }
    if psT.is_null() {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"resGetData: Unknown type: %s\x00" as *const u8 as
                      *const libc::c_char, pType);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"frameresource.c\x00" as *const u8 as *const libc::c_char,
                  555 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 11],
                                            &[libc::c_char; 11]>(b"resGetData\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as *mut libc::c_void
    }
    let mut HashedID: UDWORD = HashStringIgnoreCase(pID);
    psRes = (*psT).psRes;
    while !psRes.is_null() {
        if (*psRes).HashedID == HashedID { break ; }
        psRes = (*psRes).psNext
    }
    if psRes.is_null() {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"resGetData: Unknown ID: %s\x00" as *const u8 as
                      *const libc::c_char, pID);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"frameresource.c\x00" as *const u8 as *const libc::c_char,
                  574 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 11],
                                            &[libc::c_char; 11]>(b"resGetData\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        //		resLoadFile(pType,pID);
//		resGetData(pType,pID);
//		return NULL;
    }
    (*psRes).usage =
        ((*psRes).usage as
             libc::c_uint).wrapping_add(1 as libc::c_int as libc::c_uint) as
            UDWORD as UDWORD;
    return (*psRes).pData;
}
// return the HashedID string for a piece of data
#[no_mangle]
pub unsafe extern "C" fn resGetHashfromData(mut pType: *mut STRING,
                                            mut pData: *mut libc::c_void,
                                            mut pHash: *mut UDWORD) -> BOOL {
    let mut psT: *mut RES_TYPE = 0 as *mut RES_TYPE;
    let mut psRes: *mut RES_DATA = 0 as *mut RES_DATA;
    // Find the correct type
    let mut HashedType: UDWORD = HashString(pType);
    psT = psResTypes;
    while !psT.is_null() {
        if (*psT).HashedType == HashedType { break ; }
        psT = (*psT).psNext
    }
    if psT.is_null() {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"resGetHashfromData: Unknown type: %x\x00" as *const u8 as
                      *const libc::c_char, HashedType);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"frameresource.c\x00" as *const u8 as *const libc::c_char,
                  604 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 19],
                                            &[libc::c_char; 19]>(b"resGetHashfromData\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    // Find the resource
    psRes = (*psT).psRes;
    while !psRes.is_null() {
        if (*psRes).pData == pData { break ; }
        psRes = (*psRes).psNext
    }
    if psRes.is_null() {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"resGetHashfromData:: couldn\'t find data for type %x\n\x00"
                      as *const u8 as *const libc::c_char, HashedType);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"frameresource.c\x00" as *const u8 as *const libc::c_char,
                  620 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 19],
                                            &[libc::c_char; 19]>(b"resGetHashfromData\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    *pHash = (*psRes).HashedID;
    return 1 as libc::c_int;
}
/* Add a buffer load and release function for a file type */
/* Add a file name load and release function for a file type */
/* Call the load function for a file */
// Add data to the resource system
/* Return the resource for a type and ID */
/* Simply returns true if a resource is present */
#[no_mangle]
pub unsafe extern "C" fn resPresent(mut pType: *mut STRING,
                                    mut pID: *mut STRING) -> BOOL {
    let mut psT: *mut RES_TYPE = 0 as *mut RES_TYPE;
    let mut psRes: *mut RES_DATA = 0 as *mut RES_DATA;
    // Find the correct type
    let mut HashedType: UDWORD = HashString(pType);
    psT = psResTypes;
    while !psT.is_null() {
        if (*psT).HashedType == HashedType { break ; }
        psT = (*psT).psNext
    }
    /* Bow out if unrecognised type */
    if psT.is_null() {
        //		ASSERT( FALSE, "resPresent: Unknown type" );
        return 0 as libc::c_int
    }
    let mut HashedID: UDWORD = HashStringIgnoreCase(pID);
    //		DBPRINTF(("%x - %d\n",HashedID,pID));
    psRes = (*psT).psRes;
    while !psRes.is_null() {
        //	DBPRINTF(("!= %x\n",psRes->HashedID));
        if (*psRes).HashedID == HashedID { break ; }
        psRes = (*psRes).psNext
    }
    /* Did we find it? */
    if !psRes.is_null() { return 1 as libc::c_int }
    return 0 as libc::c_int;
}
/* Release all the resources currently loaded and the resource load functions */
#[no_mangle]
pub unsafe extern "C" fn resReleaseAll() {
    let mut psT: *mut RES_TYPE = 0 as *mut RES_TYPE;
    let mut psNT: *mut RES_TYPE = 0 as *mut RES_TYPE;
    let mut psRes: *mut RES_DATA = 0 as *mut RES_DATA;
    let mut psNRes: *mut RES_DATA = 0 as *mut RES_DATA;
    psT = psResTypes;
    while !psT.is_null() {
        psRes = (*psT).psRes;
        while !psRes.is_null() {
            if (*psRes).usage == 0 as libc::c_int as libc::c_uint {
                debug(LOG_WZ,
                      b"%s resource: %s(%04x) not used\x00" as *const u8 as
                          *const libc::c_char, (*psT).aType.as_mut_ptr(),
                      (*psRes).aID.as_mut_ptr(), (*psRes).HashedID);
            }
            if (*psT).release.is_some() {
                (*psT).release.expect("non-null function pointer")((*psRes).pData);
            } else {
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"resReleaseAll: NULL release function\x00" as
                              *const u8 as *const libc::c_char);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"frameresource.c\x00" as *const u8 as
                              *const libc::c_char, 694 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 14],
                                                    &[libc::c_char; 14]>(b"resReleaseAll\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
            }
            psNRes = (*psRes).psNext;
            memFreeRelease(psRes as *mut libc::c_void);
            psRes = 0 as *mut RES_DATA;
            psRes = psNRes
        }
        psNT = (*psT).psNext;
        memFreeRelease(psT as *mut libc::c_void);
        psT = 0 as *mut RES_TYPE;
        psT = psNT
    }
    psResTypes = 0 as *mut RES_TYPE;
}
// release the data for a particular block ID
#[no_mangle]
pub unsafe extern "C" fn resReleaseBlockData(mut blockID: SDWORD) {
    let mut psT: *mut RES_TYPE = 0 as *mut RES_TYPE;
    let mut psNT: *mut RES_TYPE = 0 as *mut RES_TYPE;
    let mut psPRes: *mut RES_DATA = 0 as *mut RES_DATA;
    let mut psRes: *mut RES_DATA = 0 as *mut RES_DATA;
    let mut psNRes: *mut RES_DATA = 0 as *mut RES_DATA;
    psT = psResTypes;
    while !psT.is_null() {
        psPRes = 0 as *mut RES_DATA;
        psRes = (*psT).psRes;
        while !psRes.is_null() {
            if !psRes.is_null() {
            } else {
                debug(LOG_ERROR,
                      b"resReleaseBlockData: null pointer passed into loop\x00"
                          as *const u8 as *const libc::c_char);
            };
            if !psRes.is_null() {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"frameresource.c\x00" as *const u8 as
                          *const libc::c_char, 719 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 20],
                                                &[libc::c_char; 20]>(b"resReleaseBlockData\x00")).as_ptr(),
                      b"psRes != NULL\x00" as *const u8 as
                          *const libc::c_char);
            };
            if (*psRes).blockID == blockID {
                if (*psRes).usage == 0 as libc::c_int as libc::c_uint {
                    debug(LOG_WZ,
                          b"%s resource: %s(%04x) not used\x00" as *const u8
                              as *const libc::c_char,
                          (*psT).aType.as_mut_ptr(),
                          (*psRes).aID.as_mut_ptr(), (*psRes).HashedID);
                }
                if (*psT).release.is_some() {
                    (*psT).release.expect("non-null function pointer")((*psRes).pData);
                } else {
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"resReleaseAllData: NULL release function\x00"
                                  as *const u8 as *const libc::c_char);
                    };
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                                  *const u8 as *const libc::c_char,
                              b"frameresource.c\x00" as *const u8 as
                                  *const libc::c_char, 732 as libc::c_int,
                              (*::std::mem::transmute::<&[u8; 20],
                                                        &[libc::c_char; 20]>(b"resReleaseBlockData\x00")).as_ptr(),
                              b"FALSE\x00" as *const u8 as
                                  *const libc::c_char);
                    };
                }
                psNRes = (*psRes).psNext;
                memFreeRelease(psRes as *mut libc::c_void);
                psRes = 0 as *mut RES_DATA;
                if psPRes.is_null() {
                    (*psT).psRes = psNRes
                } else { (*psPRes).psNext = psNRes }
            } else { psPRes = psRes; psNRes = (*psRes).psNext }
            if psNRes != 0xdddddddd as libc::c_uint as *mut RES_DATA {
            } else {
                debug(LOG_ERROR,
                      b"resReleaseBlockData: next data (next pointer) already freed\x00"
                          as *const u8 as *const libc::c_char);
            };
            if psNRes != 0xdddddddd as libc::c_uint as *mut RES_DATA {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"frameresource.c\x00" as *const u8 as
                          *const libc::c_char, 752 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 20],
                                                &[libc::c_char; 20]>(b"resReleaseBlockData\x00")).as_ptr(),
                      b"psNRes != (RES_DATA *)0xdddddddd\x00" as *const u8 as
                          *const libc::c_char);
            };
            psRes = psNRes
        }
        psNT = (*psT).psNext;
        if psNT != 0xdddddddd as libc::c_uint as *mut RES_TYPE {
        } else {
            debug(LOG_ERROR,
                  b"resReleaseBlockData: next data (next pointer) already freed\x00"
                      as *const u8 as *const libc::c_char);
        };
        if psNT != 0xdddddddd as libc::c_uint as *mut RES_TYPE {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"frameresource.c\x00" as *const u8 as *const libc::c_char,
                  755 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 20],
                                            &[libc::c_char; 20]>(b"resReleaseBlockData\x00")).as_ptr(),
                  b"psNT != (RES_TYPE *)0xdddddddd\x00" as *const u8 as
                      *const libc::c_char);
        };
        psT = psNT
    };
}
/* set the function to call when loading files with resloadfile*/
/* callback type for res pre-load callback*/
/* set the function to call when loading files with resloadfile*/
/* Initialise the resource module */
/* Shutdown the resource module */
// set the base resource directory
/* Parse the res file */
/* Release all the resources currently loaded and the resource load functions */
// release the data for a particular block ID
/* Release all the resources currently loaded but keep the resource load functions */
/* Release all the resources currently loaded but keep the resource load functions */
#[no_mangle]
pub unsafe extern "C" fn resReleaseAllData() {
    let mut psT: *mut RES_TYPE = 0 as *mut RES_TYPE;
    let mut psNT: *mut RES_TYPE = 0 as *mut RES_TYPE;
    let mut psRes: *mut RES_DATA = 0 as *mut RES_DATA;
    let mut psNRes: *mut RES_DATA = 0 as *mut RES_DATA;
    psT = psResTypes;
    while !psT.is_null() {
        psRes = (*psT).psRes;
        while !psRes.is_null() {
            if (*psRes).usage == 0 as libc::c_int as libc::c_uint {
                debug(LOG_WZ,
                      b"%s resource: %s(%04x) not used\x00" as *const u8 as
                          *const libc::c_char, (*psT).aType.as_mut_ptr(),
                      (*psRes).aID.as_mut_ptr(), (*psRes).HashedID);
            }
            if (*psT).release.is_some() {
                (*psT).release.expect("non-null function pointer")((*psRes).pData);
            } else {
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"resReleaseAllData: NULL release function\x00" as
                              *const u8 as *const libc::c_char);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"frameresource.c\x00" as *const u8 as
                              *const libc::c_char, 775 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 18],
                                                    &[libc::c_char; 18]>(b"resReleaseAllData\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
            }
            psNRes = (*psRes).psNext;
            memFreeRelease(psRes as *mut libc::c_void);
            psRes = 0 as *mut RES_DATA;
            psRes = psNRes
        }
        (*psT).psRes = 0 as *mut RES_DATA;
        psNT = (*psT).psNext;
        psT = psNT
    };
}
