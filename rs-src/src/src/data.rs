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
    /* Free a block allocated by `malloc', `realloc' or `calloc'.  */
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    /* Abort execution and generate a core-dump.  */
    #[no_mangle]
    fn abort() -> !;
    /* Set a block heap to use for all memory allocation rather than standard malloc/free */
    #[no_mangle]
    fn memSetBlockHeap(psHeap: *mut _block_heap);
    /* Get the current block heap */
    #[no_mangle]
    fn memGetBlockHeap() -> *mut _block_heap;
    #[no_mangle]
    fn memMallocRelease(Size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn memFreeRelease(pMemToFree: *mut libc::c_void);
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_uint;
    #[no_mangle]
    fn strstr(_: *const libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_uint) -> libc::c_int;
    #[no_mangle]
    fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_uint)
     -> *mut libc::c_char;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    /* Add a buffer load and release function for a file type */
    #[no_mangle]
    fn resAddBufferLoad(pType: *mut STRING, buffLoad: RES_BUFFERLOAD,
                        release: RES_FREE) -> BOOL;
    /* Add a file name load and release function for a file type */
    #[no_mangle]
    fn resAddFileLoad(pType: *mut STRING, fileLoad: RES_FILELOAD,
                      release: RES_FREE) -> BOOL;
    #[no_mangle]
    fn resPresent(pType: *mut STRING, pID: *mut STRING) -> BOOL;
    #[no_mangle]
    fn resToLower(pStr: *mut STRING);
    //return last imd resource
    #[no_mangle]
    fn GetLastResourceFilename() -> *mut libc::c_char;
    // Set the resource name of the last resource file loaded
    #[no_mangle]
    fn SetLastResourceFilename(pName: *mut libc::c_char);
    /* Release a string resource object */
    #[no_mangle]
    fn strresDestroy(psRes: *mut STR_RES);
    /* Load a string resource file */
    #[no_mangle]
    fn strresLoad(psRes: *mut STR_RES, pData: *mut libc::c_char, size: UDWORD)
     -> BOOL;
    #[no_mangle]
    fn pie_PNGLoadMem(pngimage: *mut libc::c_char, s: *mut iSprite,
                      pal: *mut iColour) -> iBool;
    #[no_mangle]
    fn iV_LoadImageFile(FileData: *mut libc::c_char, FileSize: UDWORD)
     -> *mut IMAGEFILE;
    #[no_mangle]
    fn iV_FreeImageFile(ImageFile: *mut IMAGEFILE);
    #[no_mangle]
    static mut tilesPCX: iSprite;
    #[no_mangle]
    fn makeTileTexturePages(srcWidth: UDWORD, srcHeight: UDWORD,
                            tileWidth: UDWORD, tileHeight: UDWORD,
                            src: *mut libc::c_uchar);
    #[no_mangle]
    fn remakeTileTexturePages(srcWidth: UDWORD, srcHeight: UDWORD,
                              tileWidth: UDWORD, tileHeight: UDWORD,
                              src: *mut libc::c_uchar);
    #[no_mangle]
    fn getTileRadarColours() -> libc::c_int;
    #[no_mangle]
    fn freeTileTextures();
    #[no_mangle]
    fn war_GetAdditive() -> BOOL;
    #[no_mangle]
    fn pie_ReloadTexPage(filename: *mut STRING, pBuffer: *mut libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn pie_AddBMPtoTexPages(s: *mut iSprite, filename: *mut STRING,
                            type_0: libc::c_int, bColourKeyed: iBool,
                            bResource: iBool) -> libc::c_int;
    //*************************************************************************
    #[no_mangle]
    fn pie_TexShutDown();
    #[no_mangle]
    fn iV_ProcessIMD(ppFileData: *mut *mut STRING, FileDataEnd: *mut STRING,
                     IMDpath: *mut STRING, PCXpath: *mut STRING,
                     palkeep: iBool) -> *mut iIMDShape;
    #[no_mangle]
    fn iV_IMDRelease(s: *mut iIMDShape);
    #[no_mangle]
    fn anim_LoadFromBuffer(pBuffer: *mut libc::c_char, size: UDWORD)
     -> *mut BASEANIM;
    #[no_mangle]
    fn anim_ReleaseAnim(psAnim: *mut BASEANIM);
    #[no_mangle]
    fn audio_Disabled() -> BOOL;
    #[no_mangle]
    fn audio_LoadTrackFromBuffer(pBuffer: *mut libc::c_char, udwSize: UDWORD)
     -> *mut libc::c_void;
    #[no_mangle]
    fn audio_ReleaseTrack(psTrack: *mut TRACK);
    #[no_mangle]
    static mut numBodyStats: UDWORD;
    #[no_mangle]
    static mut numBrainStats: UDWORD;
    #[no_mangle]
    static mut numPropulsionStats: UDWORD;
    #[no_mangle]
    static mut numSensorStats: UDWORD;
    #[no_mangle]
    static mut numECMStats: UDWORD;
    #[no_mangle]
    static mut numRepairStats: UDWORD;
    #[no_mangle]
    static mut numWeaponStats: UDWORD;
    #[no_mangle]
    static mut numConstructStats: UDWORD;
    #[no_mangle]
    fn loadWeaponStats(pWeaponData: *mut libc::c_char, bufferSize: UDWORD)
     -> BOOL;
    #[no_mangle]
    fn loadBodyStats(pBodyData: *mut libc::c_char, bufferSize: UDWORD)
     -> BOOL;
    #[no_mangle]
    fn loadBrainStats(pBrainData: *mut libc::c_char, bufferSize: UDWORD)
     -> BOOL;
    #[no_mangle]
    fn loadPropulsionStats(pPropulsionData: *mut libc::c_char,
                           bufferSize: UDWORD) -> BOOL;
    #[no_mangle]
    fn loadSensorStats(pSensorData: *mut libc::c_char, bufferSize: UDWORD)
     -> BOOL;
    #[no_mangle]
    fn loadECMStats(pECMData: *mut libc::c_char, bufferSize: UDWORD) -> BOOL;
    #[no_mangle]
    fn loadRepairStats(pRepairData: *mut libc::c_char, bufferSize: UDWORD)
     -> BOOL;
    #[no_mangle]
    fn loadConstructStats(pConstructData: *mut libc::c_char,
                          bufferSize: UDWORD) -> BOOL;
    #[no_mangle]
    fn loadPropulsionTypes(pPropTypeData: *mut libc::c_char,
                           bufferSize: UDWORD) -> BOOL;
    #[no_mangle]
    fn loadPropulsionSounds(pSoundData: *mut libc::c_char, bufferSize: UDWORD)
     -> BOOL;
    #[no_mangle]
    fn loadTerrainTable(pTerrainTableData: *mut libc::c_char,
                        bufferSize: UDWORD) -> BOOL;
    #[no_mangle]
    fn loadSpecialAbility(pSAbilityData: *mut libc::c_char,
                          bufferSize: UDWORD) -> BOOL;
    #[no_mangle]
    fn loadBodyPropulsionIMDs(pData: *mut libc::c_char, bufferSize: UDWORD)
     -> BOOL;
    #[no_mangle]
    fn loadWeaponSounds(pSoundData: *mut libc::c_char, bufferSize: UDWORD)
     -> BOOL;
    #[no_mangle]
    fn loadWeaponModifiers(pWeapModData: *mut libc::c_char,
                           bufferSize: UDWORD) -> BOOL;
    #[no_mangle]
    fn statsShutDown() -> BOOL;
    #[no_mangle]
    fn adjustMaxDesignStats();
    #[no_mangle]
    fn loadStructureStats(pStructData: *mut libc::c_char, bufferSize: UDWORD)
     -> BOOL;
    #[no_mangle]
    fn loadStructureWeapons(pWeaponData: *mut libc::c_char,
                            bufferSize: UDWORD) -> BOOL;
    #[no_mangle]
    fn loadStructureFunctions(pFunctionData: *mut libc::c_char,
                              bufferSize: UDWORD) -> BOOL;
    /*Load the Structure Strength Modifiers from the file exported from Access*/
    #[no_mangle]
    fn loadStructureStrengthModifiers(pStrengthModData: *mut libc::c_char,
                                      bufferSize: UDWORD) -> BOOL;
    #[no_mangle]
    fn structureStatsShutDown() -> BOOL;
    /* Load the feature stats */
    #[no_mangle]
    fn loadFeatureStats(pFeatureData: *mut libc::c_char, bufferSize: UDWORD)
     -> BOOL;
    /* Release the feature stats memory */
    #[no_mangle]
    fn featureStatsShutDown();
    #[no_mangle]
    static mut numResearch: UDWORD;
    //extern BOOL loadResearch(void);
    #[no_mangle]
    fn loadResearch(pResearchData: *mut libc::c_char, bufferSize: UDWORD)
     -> BOOL;
    //Load the pre-requisites for a research list
    #[no_mangle]
    fn loadResearchPR(pPRData: *mut libc::c_char, bufferSize: UDWORD) -> BOOL;
    //Load the artefacts for a research list
    #[no_mangle]
    fn loadResearchArtefacts(pArteData: *mut libc::c_char, bufferSize: UDWORD,
                             listNumber: UDWORD) -> BOOL;
    //Load the pre-requisites for a research list
    #[no_mangle]
    fn loadResearchFunctions(pFunctionData: *mut libc::c_char,
                             bufferSize: UDWORD) -> BOOL;
    //Load the Structures for a research list
    #[no_mangle]
    fn loadResearchStructures(pStructData: *mut libc::c_char,
                              bufferSize: UDWORD, listNumber: UDWORD) -> BOOL;
    //this just inits all the research arrays
    #[no_mangle]
    fn ResearchShutDown() -> BOOL;
    /* The string resource object */
    #[no_mangle]
    static mut psStringRes: *mut STR_RES;
    /* Initialise the string system */
    #[no_mangle]
    fn stringsInitialise() -> BOOL;
    #[no_mangle]
    fn loadDroidTemplates(pDroidData: *mut libc::c_char, bufferSize: UDWORD)
     -> BOOL;
    #[no_mangle]
    fn loadDroidWeapons(pWeaponData: *mut libc::c_char, bufferSize: UDWORD)
     -> BOOL;
    //free the storage for the droid templates
    #[no_mangle]
    fn droidTemplateShutDown() -> BOOL;
    //lists the current Upgrade level that can be applied to a structure through research
//extern FUNCTION_UPGRADE		*apProductionUpgrades[MAX_PLAYERS];
//extern UDWORD		numProductionUpgrades;
//extern FUNCTION_UPGRADE		*apResearchUpgrades[MAX_PLAYERS];
//extern UDWORD		numResearchUpgrades;
//extern FUNCTION_UPGRADE		*apArmourUpgrades[MAX_PLAYERS];
//extern UDWORD		numArmourUpgrades;
//extern FUNCTION_UPGRADE		*apBodyUpgrades[MAX_PLAYERS];
//extern UDWORD		numBodyUpgrades;
//extern FUNCTION_UPGRADE		*apRepairUpgrades[MAX_PLAYERS];
//extern UDWORD		numRepairUpgrades;
//extern FUNCTION_UPGRADE		*apResistanceUpgrades[MAX_PLAYERS];
//extern UDWORD		numResistanceUpgrades;
//extern FUNCTION_UPGRADE		*apWeaponUpgrades[MAX_PLAYERS];
//extern UDWORD		numWeaponUpgrades;
    #[no_mangle]
    fn loadFunctionStats(pFunctionData: *mut libc::c_char, bufferSize: UDWORD)
     -> BOOL;
    //extern void armourUpgrade(FUNCTION *pFunction, STRUCTURE *psBuilding);
//extern void repairUpgrade(FUNCTION *pFunction, STRUCTURE *psBuilding);
//extern void bodyUpgrade(FUNCTION *pFunction, STRUCTURE *psBuilding);
//extern void resistanceUpgrade(FUNCTION *pFunction, STRUCTURE *psBuilding);
    #[no_mangle]
    fn FunctionShutDown() -> BOOL;
    /*load the view data for the messages from the file exported from the world editor*/
    #[no_mangle]
    fn loadViewData(pViewMsgData: *mut libc::c_char, bufferSize: UDWORD)
     -> *mut VIEWDATA;
    /* Release the viewdata memory */
    #[no_mangle]
    fn viewDataShutDown(psViewData: *mut VIEWDATA);
    #[no_mangle]
    fn scriptCompile(pData: *mut libc::c_char, fileSize: UDWORD,
                     ppsProg: *mut *mut SCRIPT_CODE, debugType: SCR_DEBUGTYPE)
     -> BOOL;
    #[no_mangle]
    fn cpPrintProgram(psProg: *mut SCRIPT_CODE);
    #[no_mangle]
    fn scriptFreeCode(psCode: *mut SCRIPT_CODE);
    // Load a script value file
    #[no_mangle]
    fn scrvLoad(pData: *mut libc::c_char, size: UDWORD) -> BOOL;
    /* **************************************************************************/
/*
 *	Global ProtoTypes
 */
/* **************************************************************************/
    #[no_mangle]
    fn loadGame(pGameToLoad: *mut STRING, keepObjects: BOOL, freeMem: BOOL,
                UserSaveGame: BOOL) -> BOOL;
    // UserSaveGame is TRUE when the save game is not a new level (User Save Game)
    /*This just loads up the .gam file to determine which level data to set up - split up
so can be called in levLoadData when starting a game from a load save game*/
    #[no_mangle]
    fn loadGameInit(pGameToLoad: *mut STRING) -> BOOL;
    #[no_mangle]
    fn ParseResourceFile(pData: *mut libc::c_char, fileSize: UDWORD) -> BOOL;
    /*
 * Mechanics.h
 *
 * Game world mechanics.
 *
 */
    /* Initialise the mechanics system */
//extern BOOL mechInitialise(void);
    /* Shutdown the mechanics system */
    // Allocate the list for a component
    // release all the component lists
    //allocate the space for the Players' structure lists
    // release the structure lists
    #[no_mangle]
    fn freeStructureLists();
    #[no_mangle]
    fn allocStructLists() -> BOOL;
    #[no_mangle]
    fn allocComponentList(type_0: COMPONENT_TYPE, number: SDWORD) -> BOOL;
    #[no_mangle]
    fn freeComponentLists();
    #[no_mangle]
    fn stageTwoInitialise() -> BOOL;
    #[no_mangle]
    fn newMapInitialise() -> BOOL;
    #[no_mangle]
    static mut bMultiPlayer: BOOL;
    #[no_mangle]
    fn NEThashBuffer(pData: *mut libc::c_char, size: UDWORD) -> UDWORD;
    #[no_mangle]
    static mut scr_lineno: libc::c_int;
}
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
pub type size_t = libc::c_uint;
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
pub type UBYTE = libc::c_uchar;
pub type STRING = libc::c_char;
pub type UWORD = libc::c_ushort;
pub type SWORD = libc::c_short;
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
// size of block
// pointer to the start of the free memory section
// pointer to the base of the memory block
// The start of the last allocated block (so that it can be freed by blkSpecialFree
/* Parse the res file */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _block_heap {
    pub init: SDWORD,
    pub ext: SDWORD,
    pub psBlocks: *mut BLOCK_HEAP_MEM,
    pub psNext: *mut _block_heap,
}
pub type BLOCK_HEAP_MEM = _block_heap_mem;
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
/*
 * Treap.h
 *
 * A balanced binary tree implementation
 *
 * Overhead for using the treap is -
 *		Overhead for the heap used by the treap :
 *                  24 bytes + 4 bytes per extension
 *      12 bytes for the root
 *      20 bytes per node
 */
/* Turn on and off the treap debugging */
/* Function type for the object compare
 * return -1 for less
 *         1 for more
 *         0 for equal
 */
pub type TREAP_CMP
    =
    Option<unsafe extern "C" fn(_: UDWORD, _: UDWORD) -> SDWORD>;
/* The basic elements in the treap node.
 * These are done as macros so that the memory system
 * can use parts of the treap system.
 */
/* The key to sort the node on */
/* Treap priority */
/* The object stored in the treap */
/* The sub trees */
/* The debug info */
/* file the node was created in */
/* line the node was created at */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _treap_node {
    pub key: UDWORD,
    pub priority: UDWORD,
    pub pObj: *mut libc::c_void,
    pub psLeft: *mut _treap_node,
    pub psRight: *mut _treap_node,
}
pub type TREAP_NODE = _treap_node;
/* Treap data structure */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _treap {
    pub cmp: TREAP_CMP,
    pub psNodes: *mut OBJ_HEAP,
    pub psRoot: *mut TREAP_NODE,
}
pub type TREAP = _treap;
// root of the tree
// Routines to provide simple maths functions that work on both PSX & PC
// Use the type "FRACT" instead of FLOAT
//  - This is defined as a float on PC and a 20.12 fixed point number on PSX
//
//  Use:-
//		MAKEFRACT(int);  to convert from a SDWORD to a FRACT
//		MAKEINT(fract);	to convert the other way
//		FRACTmul(fract,fract); to multiply two fract numbers
//		FRACTdiv(fract,fract); to divide two numbers
//		SQRT(fract);		to get square root of a fract (returns a fract)
//      iSQRT(int);			to get a square root of an integer (returns an UDWORD)
//      FRACTCONST(constA,constB);	; Generates a constant of (constA/constB)
//                         e.g. to define 0.5 use FRACTCONST(1,2)
//                              to define 0.114 use FRACTCONT(114,1000)
//
// Also PERCENT(int,int);	// returns a int value 0->100 of the percentage of the first param over the second
//
// This file used to be in the deliverance src directory. But Jeremy quite correctly
// pointed out to me that it should be library based not deliverance based, and hence
// has now been moved to the lib\framework directory
//
// If you are reading this file from the deliverance source directory, please delete it now
// To multiply a FRACT by a integer just use the normal operator 
//   e.g.   FractValue2=FractValue*Interger;
//
// save is true of divide
/* Check the header files have been included from frame.h if they
 * are used outside of the framework library.
 */
pub type FRACT = libc::c_float;
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
/*
 * StrRes.h
 *
 * String resource interface functions
 *
 */
/* A string block */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _str_block {
    pub apStrings: *mut *mut STRING,
    pub idStart: UDWORD,
    pub idEnd: UDWORD,
    pub psNext: *mut _str_block,
}
pub type STR_BLOCK = _str_block;
/* A String Resource */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _str_res {
    pub psIDTreap: *mut TREAP,
    pub psStrings: *mut STR_BLOCK,
    pub init: UDWORD,
    pub ext: UDWORD,
    pub nextID: UDWORD,
}
pub type STR_RES = _str_res;
pub type BLOCK_HEAP = _block_heap;
pub type int32 = libc::c_int;
pub type uint8 = libc::c_uchar;
pub type uint16 = libc::c_ushort;
pub type uint32 = libc::c_uint;
pub type iBitmap = uint8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iColour {
    pub r: uint8,
    pub g: uint8,
    pub b: uint8,
}
pub type iBool = libc::c_int;
// The next free ID
/* **************************************************************************/
/*
 * pieTypes.h
 *
 * type defines for simple pies.
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
// Simple derived types
//
//*************************************************************************
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iSprite {
    pub width: libc::c_uint,
    pub height: libc::c_uint,
    pub bmp: *mut iBitmap,
}
pub type iPalette = [iColour; 256];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iVector {
    pub x: int32,
    pub y: int32,
    pub z: int32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iVertex {
    pub x: int32,
    pub y: int32,
    pub z: int32,
    pub u: int32,
    pub v: int32,
    pub g: uint8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iTexAnim {
    pub nFrames: libc::c_int,
    pub playbackRate: libc::c_int,
    pub textureWidth: libc::c_int,
    pub textureHeight: libc::c_int,
}
//*************************************************************************
//
// imd structures
//
//*************************************************************************
pub type BSPPOLYID = uint16;
/* **************************************************************************/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PLANE {
    pub a: FRACT,
    pub b: FRACT,
    pub c: FRACT,
    pub d: FRACT,
    pub vP: iVector,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BSPTREENODE {
    pub link: [*mut BSPTREENODE; 2],
    pub Plane: PLANE,
    pub TriSameDir: BSPPOLYID,
    pub TriOppoDir: BSPPOLYID,
}
pub type PSBSPTREENODE = *mut BSPTREENODE;
// lets hope this can work as a byte ... that will limit it to 255 polygons in 1 imd
pub type VERTEXID = libc::c_int;
// Size of the entry for vertex id in the imd polygon structure (32bits on pc 16bits on PSX)
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iIMDPoly {
    pub flags: uint32,
    pub zcentre: int32,
    pub npnts: libc::c_int,
    pub normal: iVector,
    pub pindex: *mut VERTEXID,
    pub vrt: *mut iVertex,
    pub pTexAnim: *mut iTexAnim,
    pub BSP_NextPoly: BSPPOLYID,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iIMDShape {
    pub flags: uint32,
    pub texpage: int32,
    pub oradius: int32,
    pub sradius: int32,
    pub radius: int32,
    pub visRadius: int32,
    pub xmin: int32,
    pub xmax: int32,
    pub ymin: int32,
    pub ymax: int32,
    pub zmin: int32,
    pub zmax: int32,
    pub ocen: iVector,
    pub numFrames: UWORD,
    pub animInterval: UWORD,
    pub npoints: libc::c_int,
    pub npolys: libc::c_int,
    pub nconnectors: libc::c_int,
    pub points: *mut iVector,
    pub polys: *mut iIMDPoly,
    pub connectors: *mut iVector,
    pub ntexanims: libc::c_int,
    pub texanims: *mut *mut iTexAnim,
    pub next: *mut iIMDShape,
    pub BSPNode: PSBSPTREENODE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IMAGEHEADER {
    pub Type: [UBYTE; 4],
    pub Version: UWORD,
    pub NumImages: UWORD,
    pub BitDepth: UWORD,
    pub NumTPages: UWORD,
    pub TPageFiles: [[UBYTE; 16]; 16],
    pub PalFile: [UBYTE; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IMAGEDEF {
    pub TPageID: UWORD,
    pub PalID: UWORD,
    pub Tu: UWORD,
    pub Tv: UWORD,
    pub Width: UWORD,
    pub Height: UWORD,
    pub XOffset: SWORD,
    pub YOffset: SWORD,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IMAGEFILE {
    pub Header: IMAGEHEADER,
    pub TexturePages: *mut iSprite,
    pub NumCluts: UWORD,
    pub TPageIDs: [UWORD; 16],
    pub ClutIDs: [UWORD; 48],
    pub ImageDefs: *mut IMAGEDEF,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TEXTUREPAGE {
    pub Texture: *mut iSprite,
    pub Palette: *mut iPalette,
}
pub type _component_type = libc::c_uint;
pub const COMP_NUMCOMPONENTS: _component_type = 9;
pub const COMP_WEAPON: _component_type = 8;
pub const COMP_CONSTRUCT: _component_type = 7;
pub const COMP_SENSOR: _component_type = 6;
pub const COMP_ECM: _component_type = 5;
pub const COMP_REPAIRUNIT: _component_type = 4;
pub const COMP_PROPULSION: _component_type = 3;
pub const COMP_BRAIN: _component_type = 2;
pub const COMP_BODY: _component_type = 1;
pub const COMP_UNKNOWN: _component_type = 0;
pub type COMPONENT_TYPE = _component_type;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _viewdata {
    pub pName: *mut STRING,
    pub type_0: VIEW_TYPE,
    pub numText: UBYTE,
    pub ppTextMsg: *mut *mut STRING,
    pub pData: *mut libc::c_void,
}
pub type VIEW_TYPE = _view_type;
pub type _view_type = libc::c_uint;
pub const VIEW_TYPES: _view_type = 4;
pub const VIEW_RPLX: _view_type = 3;
pub const VIEW_PROX: _view_type = 2;
pub const VIEW_RPL: _view_type = 1;
pub const VIEW_RES: _view_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ANIM_STATE {
    pub uwFrame: UWORD,
    pub vecPos: VECTOR3D,
    pub vecAngle: VECTOR3D,
    pub vecScale: VECTOR3D,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VECTOR3D {
    pub x: SDWORD,
    pub y: SDWORD,
    pub z: SDWORD,
}
// research view
// full screen view sequence - flic
// proximity view - no view really!
// full screen view sequence - flic.	extended format
/* **************************************************************************/
/*
 * Anim.h
 *
 * Animation types and function headers
 *
 * Gareth Jones 11/7/97
 */
/* **************************************************************************/
/* **************************************************************************/
/* **************************************************************************/
/* **************************************************************************/
/* **************************************************************************/
/* ensure ANIM2D/3D structs same size */
/* width of container bitmap */
/* ensure ANIM2D/3D structs same size */
/* **************************************************************************/
/* frame to play           */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BASEANIM {
    pub szFileName: [libc::c_char; 256],
    pub animType: libc::c_char,
    pub uwID: UWORD,
    pub uwFrameRate: UWORD,
    pub uwStates: UWORD,
    pub uwObj: UWORD,
    pub uwAnimTime: UWORD,
    pub ubType: UBYTE,
    pub psStates: *mut ANIM_STATE,
    pub psNext: *mut BASEANIM,
}
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
/*
 * Research.h
 *
 * structures required for research stats
 *
 */
//used for loading in the research stats into the appropriate list
pub type C2RustUnnamed_0 = libc::c_uint;
pub const RES_LIST: C2RustUnnamed_0 = 2;
pub const RED_LIST: C2RustUnnamed_0 = 1;
pub const REQ_LIST: C2RustUnnamed_0 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RES_TYPE_MIN {
    pub aType: *mut STRING,
    pub buffLoad: RES_BUFFERLOAD,
    pub release: RES_FREE,
    pub ResourceData: *mut libc::c_void,
    pub HashedType: UDWORD,
}
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
// Trigger fires when the script is first run
// Trigger uses script code
// Trigger after a time pause
// Trigger at repeated intervals
// Event has paused for an interval and will restart in the middle of it's code
// The user defined callback triggers should start with this id
/* Description of a trigger for the SCRIPT_CODE */
// Type of trigger
// BOOL - is there code with this trigger
// How often to check the trigger
/* A compiled script and its associated data */
pub type SCRIPT_CODE = _script_code;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _script_code {
    pub size: UDWORD,
    pub pCode: *mut UDWORD,
    pub numTriggers: UWORD,
    pub numEvents: UWORD,
    pub pTriggerTab: *mut UWORD,
    pub psTriggerData: *mut TRIGGER_DATA,
    pub pEventTab: *mut UWORD,
    pub pEventLinks: *mut SWORD,
    pub numGlobals: UWORD,
    pub numArrays: UWORD,
    pub arraySize: UDWORD,
    pub pGlobals: *mut INTERP_TYPE,
    pub ppsLocalVars: *mut *mut INTERP_TYPE,
    pub numLocalVars: *mut UDWORD,
    pub ppsLocalVarVal: *mut *mut INTERP_VAL,
    pub numParams: *mut UDWORD,
    pub psVarDebug: *mut VAR_DEBUG,
    pub psArrayInfo: *mut ARRAY_DATA,
    pub psArrayDebug: *mut ARRAY_DEBUG,
    pub debugEntries: UWORD,
    pub psDebug: *mut SCRIPT_DEBUG,
}
pub type SCRIPT_DEBUG = _script_debug;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _script_debug {
    pub offset: UDWORD,
    pub line: UDWORD,
    pub pLabel: *mut STRING,
}
pub type ARRAY_DEBUG = _array_debug;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _array_debug {
    pub pIdent: *mut STRING,
    pub storage: UBYTE,
}
pub type ARRAY_DATA = _array_data;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _array_data {
    pub base: UDWORD,
    pub type_0: UBYTE,
    pub dimensions: UBYTE,
    pub elements: [UBYTE; 4],
}
pub type VAR_DEBUG = _var_debug;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _var_debug {
    pub pIdent: *mut STRING,
    pub storage: STORAGE_TYPE,
}
pub type STORAGE_TYPE = UBYTE;
pub type INTERP_VAL = _interp_val;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _interp_val {
    pub type_0: INTERP_TYPE,
    pub v: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub bval: BOOL,
    pub ival: SDWORD,
    pub sval: *mut STRING,
    pub oval: *mut libc::c_void,
    pub pVoid: *mut libc::c_void,
}
pub type INTERP_TYPE = _interp_type;
pub type _interp_type = libc::c_uint;
pub const VAL_USERTYPESTART: _interp_type = 6;
pub const VAL_VOID: _interp_type = 5;
pub const VAL_EVENT: _interp_type = 4;
pub const VAL_TRIGGER: _interp_type = 3;
pub const VAL_STRING: _interp_type = 2;
pub const VAL_INT: _interp_type = 1;
pub const VAL_BOOL: _interp_type = 0;
pub type TRIGGER_DATA = _trigger_data;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _trigger_data {
    pub type_0: UWORD,
    pub code: UWORD,
    pub time: UDWORD,
}
pub type SCR_DEBUGTYPE = _scr_debugtype;
pub type _scr_debugtype = libc::c_uint;
pub const SCR_NODEBUG: _scr_debugtype = 1;
pub const SCR_DEBUGINFO: _scr_debugtype = 0;
pub type VIEWDATA = _viewdata;
// The size (in bytes) of the compiled code
// Pointer to the compiled code
// The number of triggers
// The number of events
// The table of trigger offsets
// The extra info for each trigger
// The table of event offsets
// The original trigger/event linkage
// -1 for no link
// The number of global variables
// the number of arrays in the program
// the number of elements in all the defined arrays
// Types of the global variables
//storage for local vars (type)
//number of local vars each event has
//Values of the local vars used during interpreting process
//number of arguments this event has
// The names and storage types of variables
// The sizes of the program arrays
// Debug info for the arrays
// Number of entries in psDebug
// Debugging info for the script
//name ID of the message - used for loading in and identifying
//the type of view
//the number of textmessages associated with this data
//Pointer to text messages - if any
/*the data required to view - either a
							  VIEW_RESEARCH, VIEW_PROXIMITY or VIEW_REPLAY*/
/*
 * Data.c
 *
 * Data loading functions used by the framework resource module
 *
 */
//render library
/* *********************************************************
 *
 * Local Variables
 *
 *********************************************************/
#[no_mangle]
pub static mut bTilesPCXLoaded: BOOL = 0 as libc::c_int;
// whether a save game is currently being loaded
#[no_mangle]
pub static mut saveFlag: BOOL = 0 as libc::c_int;
// Arse
#[no_mangle]
pub static mut cheatHash: [UDWORD; 29] = [0; 29];
/* *********************************************************
 *
 * Source
 *
 *********************************************************/
#[no_mangle]
pub unsafe extern "C" fn calcCheatHash(mut pBuffer: *mut libc::c_char,
                                       mut size: UDWORD, mut cheat: UDWORD) {
    if bMultiPlayer == 0 { return }
    // create the hash for that data block.
    cheatHash[cheat as usize] =
        cheatHash[cheat as usize] ^ NEThashBuffer(pBuffer, size);
}
#[no_mangle]
pub unsafe extern "C" fn resetCheatHash() {
    let mut i: UDWORD = 0;
    i = 0 as libc::c_int as UDWORD;
    while i < 29 as libc::c_int as libc::c_uint {
        cheatHash[i as usize] = 0 as libc::c_int as UDWORD;
        i = i.wrapping_add(1)
    };
}
/* *********************************************************/
#[no_mangle]
pub unsafe extern "C" fn dataSetSaveFlag() { saveFlag = 1 as libc::c_int; }
#[no_mangle]
pub unsafe extern "C" fn dataClearSaveFlag() { saveFlag = 0 as libc::c_int; }
/* Load the body stats */
#[no_mangle]
pub unsafe extern "C" fn bufferSBODYLoad(mut pBuffer: *mut libc::c_char,
                                         mut size: UDWORD,
                                         mut ppData: *mut *mut libc::c_void)
 -> BOOL {
    calcCheatHash(pBuffer, size, 1 as libc::c_int as UDWORD);
    if loadBodyStats(pBuffer, size) == 0 { return 0 as libc::c_int }
    if allocComponentList(COMP_BODY, numBodyStats as SDWORD) == 0 {
        return 0 as libc::c_int
    }
    // set a dummy value so the release function gets called
    *ppData = 1 as libc::c_int as *mut libc::c_void;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dataReleaseStats(mut pData: *mut libc::c_void) {
    freeComponentLists();
    statsShutDown();
}
/* Load the weapon stats */
#[no_mangle]
pub unsafe extern "C" fn bufferSWEAPONLoad(mut pBuffer: *mut libc::c_char,
                                           mut size: UDWORD,
                                           mut ppData: *mut *mut libc::c_void)
 -> BOOL {
    calcCheatHash(pBuffer, size, 0 as libc::c_int as UDWORD);
    if loadWeaponStats(pBuffer, size) == 0 { return 0 as libc::c_int }
    if allocComponentList(COMP_WEAPON, numWeaponStats as SDWORD) == 0 {
        return 0 as libc::c_int
    }
    //not interested in this value
    *ppData = 0 as *mut libc::c_void;
    return 1 as libc::c_int;
}
/* Load the constructor stats */
#[no_mangle]
pub unsafe extern "C" fn bufferSCONSTRLoad(mut pBuffer: *mut libc::c_char,
                                           mut size: UDWORD,
                                           mut ppData: *mut *mut libc::c_void)
 -> BOOL {
    calcCheatHash(pBuffer, size, 2 as libc::c_int as UDWORD);
    if loadConstructStats(pBuffer, size) == 0 { return 0 as libc::c_int }
    if allocComponentList(COMP_CONSTRUCT, numConstructStats as SDWORD) == 0 {
        return 0 as libc::c_int
    }
    //not interested in this value
    *ppData = 0 as *mut libc::c_void;
    return 1 as libc::c_int;
}
/* Load the ECM stats */
#[no_mangle]
pub unsafe extern "C" fn bufferSECMLoad(mut pBuffer: *mut libc::c_char,
                                        mut size: UDWORD,
                                        mut ppData: *mut *mut libc::c_void)
 -> BOOL {
    calcCheatHash(pBuffer, size, 3 as libc::c_int as UDWORD);
    if loadECMStats(pBuffer, size) == 0 { return 0 as libc::c_int }
    if allocComponentList(COMP_ECM, numECMStats as SDWORD) == 0 {
        return 0 as libc::c_int
    }
    //not interested in this value
    *ppData = 0 as *mut libc::c_void;
    return 1 as libc::c_int;
}
/* Load the Propulsion stats */
#[no_mangle]
pub unsafe extern "C" fn bufferSPROPLoad(mut pBuffer: *mut libc::c_char,
                                         mut size: UDWORD,
                                         mut ppData: *mut *mut libc::c_void)
 -> BOOL {
    calcCheatHash(pBuffer, size, 4 as libc::c_int as UDWORD);
    if loadPropulsionStats(pBuffer, size) == 0 { return 0 as libc::c_int }
    if allocComponentList(COMP_PROPULSION, numPropulsionStats as SDWORD) == 0
       {
        return 0 as libc::c_int
    }
    //not interested in this value
    *ppData = 0 as *mut libc::c_void;
    return 1 as libc::c_int;
}
/* Load the Sensor stats */
#[no_mangle]
pub unsafe extern "C" fn bufferSSENSORLoad(mut pBuffer: *mut libc::c_char,
                                           mut size: UDWORD,
                                           mut ppData: *mut *mut libc::c_void)
 -> BOOL {
    calcCheatHash(pBuffer, size, 5 as libc::c_int as UDWORD);
    if loadSensorStats(pBuffer, size) == 0 { return 0 as libc::c_int }
    if allocComponentList(COMP_SENSOR, numSensorStats as SDWORD) == 0 {
        return 0 as libc::c_int
    }
    //not interested in this value
    *ppData = 0 as *mut libc::c_void;
    return 1 as libc::c_int;
}
/* Load the Repair stats */
#[no_mangle]
pub unsafe extern "C" fn bufferSREPAIRLoad(mut pBuffer: *mut libc::c_char,
                                           mut size: UDWORD,
                                           mut ppData: *mut *mut libc::c_void)
 -> BOOL {
    calcCheatHash(pBuffer, size, 6 as libc::c_int as UDWORD);
    if loadRepairStats(pBuffer, size) == 0 { return 0 as libc::c_int }
    if allocComponentList(COMP_REPAIRUNIT, numRepairStats as SDWORD) == 0 {
        return 0 as libc::c_int
    }
    //not interested in this value
    *ppData = 0 as *mut libc::c_void;
    return 1 as libc::c_int;
}
/* Load the Brain stats */
#[no_mangle]
pub unsafe extern "C" fn bufferSBRAINLoad(mut pBuffer: *mut libc::c_char,
                                          mut size: UDWORD,
                                          mut ppData: *mut *mut libc::c_void)
 -> BOOL {
    calcCheatHash(pBuffer, size, 7 as libc::c_int as UDWORD);
    if loadBrainStats(pBuffer, size) == 0 { return 0 as libc::c_int }
    if allocComponentList(COMP_BRAIN, numBrainStats as SDWORD) == 0 {
        return 0 as libc::c_int
    }
    //not interested in this value
    *ppData = 0 as *mut libc::c_void;
    return 1 as libc::c_int;
}
/* Load the PropulsionType stats */
#[no_mangle]
pub unsafe extern "C" fn bufferSPROPTYPESLoad(mut pBuffer: *mut libc::c_char,
                                              mut size: UDWORD,
                                              mut ppData:
                                                  *mut *mut libc::c_void)
 -> BOOL {
    calcCheatHash(pBuffer, size, 8 as libc::c_int as UDWORD);
    if loadPropulsionTypes(pBuffer, size) == 0 { return 0 as libc::c_int }
    //not interested in this value
    *ppData = 0 as *mut libc::c_void;
    return 1 as libc::c_int;
}
/* Load the propulsion type sound stats */
#[no_mangle]
pub unsafe extern "C" fn bufferSPROPSNDLoad(mut pBuffer: *mut libc::c_char,
                                            mut size: UDWORD,
                                            mut ppData:
                                                *mut *mut libc::c_void)
 -> BOOL {
    if loadPropulsionSounds(pBuffer, size) == 0 { return 0 as libc::c_int }
    //not interested in this value
    *ppData = 0 as *mut libc::c_void;
    return 1 as libc::c_int;
}
/* Load the SSPECABIL stats */
#[no_mangle]
pub unsafe extern "C" fn bufferSSPECABILLoad(mut pBuffer: *mut libc::c_char,
                                             mut size: UDWORD,
                                             mut ppData:
                                                 *mut *mut libc::c_void)
 -> BOOL {
    if loadSpecialAbility(pBuffer, size) == 0 { return 0 as libc::c_int }
    //not interested in this value
    *ppData = 0 as *mut libc::c_void;
    return 1 as libc::c_int;
}
/* Load the STERRTABLE stats */
#[no_mangle]
pub unsafe extern "C" fn bufferSTERRTABLELoad(mut pBuffer: *mut libc::c_char,
                                              mut size: UDWORD,
                                              mut ppData:
                                                  *mut *mut libc::c_void)
 -> BOOL {
    calcCheatHash(pBuffer, size, 9 as libc::c_int as UDWORD);
    if loadTerrainTable(pBuffer, size) == 0 { return 0 as libc::c_int }
    //not interested in this value
    *ppData = 0 as *mut libc::c_void;
    return 1 as libc::c_int;
}
/* Load the body/propulsion IMDs stats */
#[no_mangle]
pub unsafe extern "C" fn bufferSBPIMDLoad(mut pBuffer: *mut libc::c_char,
                                          mut size: UDWORD,
                                          mut ppData: *mut *mut libc::c_void)
 -> BOOL {
    if loadBodyPropulsionIMDs(pBuffer, size) == 0 { return 0 as libc::c_int }
    //not interested in this value
    *ppData = 0 as *mut libc::c_void;
    return 1 as libc::c_int;
}
/* Load the weapon sound stats */
#[no_mangle]
pub unsafe extern "C" fn bufferSWEAPSNDLoad(mut pBuffer: *mut libc::c_char,
                                            mut size: UDWORD,
                                            mut ppData:
                                                *mut *mut libc::c_void)
 -> BOOL {
    if loadWeaponSounds(pBuffer, size) == 0 { return 0 as libc::c_int }
    //not interested in this value
    *ppData = 0 as *mut libc::c_void;
    return 1 as libc::c_int;
}
/* Load the Weapon Effect modifier stats */
#[no_mangle]
pub unsafe extern "C" fn bufferSWEAPMODLoad(mut pBuffer: *mut libc::c_char,
                                            mut size: UDWORD,
                                            mut ppData:
                                                *mut *mut libc::c_void)
 -> BOOL {
    calcCheatHash(pBuffer, size, 10 as libc::c_int as UDWORD);
    if loadWeaponModifiers(pBuffer, size) == 0 { return 0 as libc::c_int }
    //not interested in this value
    *ppData = 0 as *mut libc::c_void;
    return 1 as libc::c_int;
}
/* Load the Template stats */
#[no_mangle]
pub unsafe extern "C" fn bufferSTEMPLLoad(mut pBuffer: *mut libc::c_char,
                                          mut size: UDWORD,
                                          mut ppData: *mut *mut libc::c_void)
 -> BOOL {
    calcCheatHash(pBuffer, size, 11 as libc::c_int as UDWORD);
    if loadDroidTemplates(pBuffer, size) == 0 { return 0 as libc::c_int }
    // set a dummy value so the release function gets called
    *ppData = 1 as libc::c_int as *mut libc::c_void;
    return 1 as libc::c_int;
}
// release the templates
#[no_mangle]
pub unsafe extern "C" fn dataSTEMPLRelease(mut pData: *mut libc::c_void) {
    //free the storage allocated to the droid templates
    droidTemplateShutDown();
}
/* Load the Template weapons stats */
#[no_mangle]
pub unsafe extern "C" fn bufferSTEMPWEAPLoad(mut pBuffer: *mut libc::c_char,
                                             mut size: UDWORD,
                                             mut ppData:
                                                 *mut *mut libc::c_void)
 -> BOOL {
    calcCheatHash(pBuffer, size, 12 as libc::c_int as UDWORD);
    if loadDroidWeapons(pBuffer, size) == 0 { return 0 as libc::c_int }
    //not interested in this value
    *ppData = 0 as *mut libc::c_void;
    return 1 as libc::c_int;
}
/* Load the Structure stats */
#[no_mangle]
pub unsafe extern "C" fn bufferSSTRUCTLoad(mut pBuffer: *mut libc::c_char,
                                           mut size: UDWORD,
                                           mut ppData: *mut *mut libc::c_void)
 -> BOOL {
    calcCheatHash(pBuffer, size, 13 as libc::c_int as UDWORD);
    if loadStructureStats(pBuffer, size) == 0 { return 0 as libc::c_int }
    if allocStructLists() == 0 { return 0 as libc::c_int }
    // set a dummy value so the release function gets called
    *ppData = 1 as libc::c_int as *mut libc::c_void;
    return 1 as libc::c_int;
}
// release the structure stats
#[no_mangle]
pub unsafe extern "C" fn dataSSTRUCTRelease(mut pData: *mut libc::c_void) {
    freeStructureLists();
    structureStatsShutDown();
}
/* Load the Structure Weapons stats */
#[no_mangle]
pub unsafe extern "C" fn bufferSSTRWEAPLoad(mut pBuffer: *mut libc::c_char,
                                            mut size: UDWORD,
                                            mut ppData:
                                                *mut *mut libc::c_void)
 -> BOOL {
    calcCheatHash(pBuffer, size, 14 as libc::c_int as UDWORD);
    if loadStructureWeapons(pBuffer, size) == 0 { return 0 as libc::c_int }
    //not interested in this value
    *ppData = 0 as *mut libc::c_void;
    return 1 as libc::c_int;
}
/* Load the Structure Functions stats */
#[no_mangle]
pub unsafe extern "C" fn bufferSSTRFUNCLoad(mut pBuffer: *mut libc::c_char,
                                            mut size: UDWORD,
                                            mut ppData:
                                                *mut *mut libc::c_void)
 -> BOOL {
    calcCheatHash(pBuffer, size, 15 as libc::c_int as UDWORD);
    if loadStructureFunctions(pBuffer, size) == 0 { return 0 as libc::c_int }
    //not interested in this value
    *ppData = 0 as *mut libc::c_void;
    return 1 as libc::c_int;
}
/* Load the Structure strength modifier stats */
#[no_mangle]
pub unsafe extern "C" fn bufferSSTRMODLoad(mut pBuffer: *mut libc::c_char,
                                           mut size: UDWORD,
                                           mut ppData: *mut *mut libc::c_void)
 -> BOOL {
    calcCheatHash(pBuffer, size, 16 as libc::c_int as UDWORD);
    if loadStructureStrengthModifiers(pBuffer, size) == 0 {
        return 0 as libc::c_int
    }
    //not interested in this value
    *ppData = 0 as *mut libc::c_void;
    return 1 as libc::c_int;
}
/* Load the Feature stats */
#[no_mangle]
pub unsafe extern "C" fn bufferSFEATLoad(mut pBuffer: *mut libc::c_char,
                                         mut size: UDWORD,
                                         mut ppData: *mut *mut libc::c_void)
 -> BOOL {
    calcCheatHash(pBuffer, size, 17 as libc::c_int as UDWORD);
    if loadFeatureStats(pBuffer, size) == 0 { return 0 as libc::c_int }
    // set a dummy value so the release function gets called
    *ppData = 1 as libc::c_int as *mut libc::c_void;
    return 1 as libc::c_int;
}
// free the feature stats
#[no_mangle]
pub unsafe extern "C" fn dataSFEATRelease(mut pData: *mut libc::c_void) {
    featureStatsShutDown();
}
/* Load the Functions stats */
#[no_mangle]
pub unsafe extern "C" fn bufferSFUNCLoad(mut pBuffer: *mut libc::c_char,
                                         mut size: UDWORD,
                                         mut ppData: *mut *mut libc::c_void)
 -> BOOL {
    calcCheatHash(pBuffer, size, 18 as libc::c_int as UDWORD);
    if loadFunctionStats(pBuffer, size) == 0 { return 0 as libc::c_int }
    //adjust max values of stats used in the design screen due to any possible upgrades
    adjustMaxDesignStats();
    // set a dummy value so the release function gets called
    *ppData = 1 as libc::c_int as *mut libc::c_void;
    return 1 as libc::c_int;
}
// release the function stats
#[no_mangle]
pub unsafe extern "C" fn dataSFUNCRelease(mut pData: *mut libc::c_void) {
    FunctionShutDown();
}
// release the research stats
#[no_mangle]
pub unsafe extern "C" fn dataRESCHRelease(mut pData: *mut libc::c_void) {
    //free the storage allocated to the stats
    ResearchShutDown();
}
/* Load the Research stats */
#[no_mangle]
pub unsafe extern "C" fn bufferRESCHLoad(mut pBuffer: *mut libc::c_char,
                                         mut size: UDWORD,
                                         mut ppData: *mut *mut libc::c_void)
 -> BOOL {
    calcCheatHash(pBuffer, size, 19 as libc::c_int as UDWORD);
    //check to see if already loaded
    if numResearch > 0 as libc::c_int as libc::c_uint {
        //release previous data before loading in the new
        dataRESCHRelease(0 as *mut libc::c_void);
    }
    if loadResearch(pBuffer, size) == 0 { return 0 as libc::c_int }
    /* set a dummy value so the release function gets called - the Release
    function is now called when load up the next set
	// *ppData = (void *)1;
    pass back NULL so that can load the same name file for the next campaign*/
    *ppData = 0 as *mut libc::c_void;
    return 1 as libc::c_int;
}
/* Load the research pre-requisites */
#[no_mangle]
pub unsafe extern "C" fn bufferRPREREQLoad(mut pBuffer: *mut libc::c_char,
                                           mut size: UDWORD,
                                           mut ppData: *mut *mut libc::c_void)
 -> BOOL {
    calcCheatHash(pBuffer, size, 20 as libc::c_int as UDWORD);
    if loadResearchPR(pBuffer, size) == 0 { return 0 as libc::c_int }
    //not interested in this value
    *ppData = 0 as *mut libc::c_void;
    return 1 as libc::c_int;
}
/* Load the research components made redundant */
#[no_mangle]
pub unsafe extern "C" fn bufferRCOMPREDLoad(mut pBuffer: *mut libc::c_char,
                                            mut size: UDWORD,
                                            mut ppData:
                                                *mut *mut libc::c_void)
 -> BOOL {
    calcCheatHash(pBuffer, size, 21 as libc::c_int as UDWORD);
    if loadResearchArtefacts(pBuffer, size, RED_LIST as libc::c_int as UDWORD)
           == 0 {
        return 0 as libc::c_int
    }
    //not interested in this value
    *ppData = 0 as *mut libc::c_void;
    return 1 as libc::c_int;
}
/* Load the research component results */
#[no_mangle]
pub unsafe extern "C" fn bufferRCOMPRESLoad(mut pBuffer: *mut libc::c_char,
                                            mut size: UDWORD,
                                            mut ppData:
                                                *mut *mut libc::c_void)
 -> BOOL {
    calcCheatHash(pBuffer, size, 23 as libc::c_int as UDWORD);
    if loadResearchArtefacts(pBuffer, size, RES_LIST as libc::c_int as UDWORD)
           == 0 {
        return 0 as libc::c_int
    }
    //not interested in this value
    *ppData = 0 as *mut libc::c_void;
    return 1 as libc::c_int;
}
/* Load the research structures required */
#[no_mangle]
pub unsafe extern "C" fn bufferRSTRREQLoad(mut pBuffer: *mut libc::c_char,
                                           mut size: UDWORD,
                                           mut ppData: *mut *mut libc::c_void)
 -> BOOL {
    calcCheatHash(pBuffer, size, 22 as libc::c_int as UDWORD);
    if loadResearchStructures(pBuffer, size,
                              REQ_LIST as libc::c_int as UDWORD) == 0 {
        return 0 as libc::c_int
    }
    //not interested in this value
    *ppData = 0 as *mut libc::c_void;
    return 1 as libc::c_int;
}
/* Load the research structures made redundant */
#[no_mangle]
pub unsafe extern "C" fn bufferRSTRREDLoad(mut pBuffer: *mut libc::c_char,
                                           mut size: UDWORD,
                                           mut ppData: *mut *mut libc::c_void)
 -> BOOL {
    calcCheatHash(pBuffer, size, 24 as libc::c_int as UDWORD);
    if loadResearchStructures(pBuffer, size,
                              RED_LIST as libc::c_int as UDWORD) == 0 {
        return 0 as libc::c_int
    }
    //not interested in this value
    *ppData = 0 as *mut libc::c_void;
    return 1 as libc::c_int;
}
/* Load the research structure results */
#[no_mangle]
pub unsafe extern "C" fn bufferRSTRRESLoad(mut pBuffer: *mut libc::c_char,
                                           mut size: UDWORD,
                                           mut ppData: *mut *mut libc::c_void)
 -> BOOL {
    calcCheatHash(pBuffer, size, 25 as libc::c_int as UDWORD);
    if loadResearchStructures(pBuffer, size,
                              RES_LIST as libc::c_int as UDWORD) == 0 {
        return 0 as libc::c_int
    }
    //not interested in this value
    *ppData = 0 as *mut libc::c_void;
    return 1 as libc::c_int;
}
/* Load the research functions */
#[no_mangle]
pub unsafe extern "C" fn bufferRFUNCLoad(mut pBuffer: *mut libc::c_char,
                                         mut size: UDWORD,
                                         mut ppData: *mut *mut libc::c_void)
 -> BOOL {
    calcCheatHash(pBuffer, size, 26 as libc::c_int as UDWORD);
    if loadResearchFunctions(pBuffer, size) == 0 { return 0 as libc::c_int }
    //not interested in this value
    *ppData = 0 as *mut libc::c_void;
    return 1 as libc::c_int;
}
/* Load the message viewdata */
#[no_mangle]
pub unsafe extern "C" fn bufferSMSGLoad(mut pBuffer: *mut libc::c_char,
                                        mut size: UDWORD,
                                        mut ppData: *mut *mut libc::c_void)
 -> BOOL {
    let mut pViewData: *mut VIEWDATA = 0 as *mut VIEWDATA;
    pViewData = loadViewData(pBuffer, size);
    if pViewData.is_null() { return 0 as libc::c_int }
    // set the pointer so the release function gets called with it
    *ppData = pViewData as *mut libc::c_void;
    return 1 as libc::c_int;
}
// release the message viewdata
#[no_mangle]
pub unsafe extern "C" fn dataSMSGRelease(mut pData: *mut libc::c_void) {
    viewDataShutDown(pData as *mut VIEWDATA);
}
/* Load an imd */
#[no_mangle]
pub unsafe extern "C" fn dataIMDBufferLoad(mut pBuffer: *mut libc::c_char,
                                           mut size: UDWORD,
                                           mut ppData: *mut *mut libc::c_void)
 -> BOOL {
    let mut psIMD: *mut iIMDShape = 0 as *mut iIMDShape;
    let mut pBufferPosition: *mut libc::c_char = pBuffer;
    psIMD =
        iV_ProcessIMD(&mut pBufferPosition,
                      pBufferPosition.offset(size as isize),
                      b"\x00" as *const u8 as *const libc::c_char as
                          *mut STRING,
                      b"\x00" as *const u8 as *const libc::c_char as
                          *mut STRING, 0 as libc::c_int);
    if psIMD.is_null() {
        debug(LOG_ERROR,
              b"IMD load failed - %s\x00" as *const u8 as *const libc::c_char,
              GetLastResourceFilename());
        abort();
    }
    *ppData = psIMD as *mut libc::c_void;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dataIMGPAGELoad(mut pBuffer: *mut libc::c_char,
                                         mut size: UDWORD,
                                         mut ppData: *mut *mut libc::c_void)
 -> BOOL {
    let mut psSprite: *mut iSprite =
        memMallocRelease(::std::mem::size_of::<iSprite>() as libc::c_ulong) as
            *mut iSprite;
    if psSprite.is_null() { return 0 as libc::c_int }
    if pie_PNGLoadMem(pBuffer, psSprite, 0 as *mut iColour) == 0 {
        debug(LOG_ERROR,
              b"IMGPAGE load failed\x00" as *const u8 as *const libc::c_char);
        return 0 as libc::c_int
    }
    *ppData = psSprite as *mut libc::c_void;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dataIMGPAGERelease(mut pData: *mut libc::c_void) {
    let mut psSprite: *mut iSprite = pData as *mut iSprite;
    dataISpriteRelease(psSprite as *mut libc::c_void);
}
// Tertiles loader. This version for hardware renderer.
#[no_mangle]
pub unsafe extern "C" fn dataHWTERTILESLoad(mut pBuffer: *mut libc::c_char,
                                            mut size: UDWORD,
                                            mut ppData:
                                                *mut *mut libc::c_void)
 -> BOOL {
    // tile loader.
    if bTilesPCXLoaded != 0 {
        debug(LOG_TEXTURE,
              b"Reloading terrain tiles\n\x00" as *const u8 as
                  *const libc::c_char);
        if pie_PNGLoadMem(pBuffer, &mut tilesPCX, 0 as *mut iColour) == 0 {
            debug(LOG_ERROR,
                  b"HWTERTILES reload failed\x00" as *const u8 as
                      *const libc::c_char);
            return 0 as libc::c_int
        }
    } else {
        debug(LOG_TEXTURE,
              b"Loading terrain tiles\n\x00" as *const u8 as
                  *const libc::c_char);
        if pie_PNGLoadMem(pBuffer, &mut tilesPCX, 0 as *mut iColour) == 0 {
            debug(LOG_ERROR,
                  b"HWTERTILES load failed\x00" as *const u8 as
                      *const libc::c_char);
            return 0 as libc::c_int
        }
    }
    getTileRadarColours();
    // make several 256 * 256 pages
    if bTilesPCXLoaded != 0 {
        remakeTileTexturePages(tilesPCX.width, tilesPCX.height,
                               128 as libc::c_int as UDWORD,
                               128 as libc::c_int as UDWORD, tilesPCX.bmp);
    } else {
        makeTileTexturePages(tilesPCX.width, tilesPCX.height,
                             128 as libc::c_int as UDWORD,
                             128 as libc::c_int as UDWORD, tilesPCX.bmp);
    }
    if bTilesPCXLoaded != 0 {
        *ppData = 0 as *mut libc::c_void
    } else {
        bTilesPCXLoaded = 1 as libc::c_int;
        *ppData = &mut tilesPCX as *mut iSprite as *mut libc::c_void
    }
    debug(LOG_TEXTURE,
          b"HW Tiles loaded\n\x00" as *const u8 as *const libc::c_char);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dataHWTERTILESRelease(mut pData: *mut libc::c_void) {
    let mut psSprite: *mut iSprite = pData as *mut iSprite;
    freeTileTextures();
    if !(*psSprite).bmp.is_null() {
        free((*psSprite).bmp as *mut libc::c_void);
        (*psSprite).bmp = 0 as *mut iBitmap
    }
    // We are not allowed to free psSprite also, this would give an error on Windows: HEAP[Warzone.exe]: Invalid Address specified to RtlFreeHeap( xxx, xxx )
    bTilesPCXLoaded = 0 as libc::c_int;
    pie_TexShutDown();
}
#[no_mangle]
pub unsafe extern "C" fn dataIMGLoad(mut pBuffer: *mut libc::c_char,
                                     mut size: UDWORD,
                                     mut ppData: *mut *mut libc::c_void)
 -> BOOL {
    let mut ImageFile: *mut IMAGEFILE = 0 as *mut IMAGEFILE;
    ImageFile = iV_LoadImageFile(pBuffer, size);
    if ImageFile.is_null() { return 0 as libc::c_int }
    *ppData = ImageFile as *mut libc::c_void;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dataIMGRelease(mut pData: *mut libc::c_void) {
    iV_FreeImageFile(pData as *mut IMAGEFILE);
}
/* Load a texturepage into memory */
#[no_mangle]
pub unsafe extern "C" fn bufferTexPageLoad(mut pBuffer: *mut libc::c_char,
                                           mut size: UDWORD,
                                           mut ppData: *mut *mut libc::c_void)
 -> BOOL {
    let mut NewTexturePage: *mut TEXTUREPAGE = 0 as *mut TEXTUREPAGE;
    let mut psPal: *mut iPalette = 0 as *mut iPalette;
    let mut psSprite: *mut iSprite = 0 as *mut iSprite;
    let mut texfile: [STRING; 255] = [0; 255];
    let mut i: SDWORD = 0;
    let mut id: SDWORD = 0;
    //	BOOL		bFound = FALSE;
    // generate a texture page name in "page-xx" format
    strncpy(texfile.as_mut_ptr(), GetLastResourceFilename(),
            254 as libc::c_int as libc::c_uint);
    texfile[254 as libc::c_int as usize] = 0 as libc::c_int as STRING;
    resToLower(texfile.as_mut_ptr());
    debug(LOG_TEXTURE,
          b"bufferTexPageLoad: %s texturepage ...\x00" as *const u8 as
              *const libc::c_char, texfile.as_mut_ptr());
    if war_GetAdditive() != 0 {
        //(war_GetTranslucent())
        //hardware
        if !strstr(texfile.as_mut_ptr(),
                   b"soft\x00" as *const u8 as *const libc::c_char).is_null()
           {
            //and this is a software textpage
            //so dont load it
            *ppData = 0 as *mut libc::c_void;
            return 1 as libc::c_int
        }
    } else if !strstr(texfile.as_mut_ptr(),
                      b"hard\x00" as *const u8 as
                          *const libc::c_char).is_null() {
        //software or old d3d card
        //and this is a hardware textpage
        //so dont load it
        *ppData = 0 as *mut libc::c_void;
        return 1 as libc::c_int
    }
    if strncmp(texfile.as_mut_ptr(),
               b"page-\x00" as *const u8 as *const libc::c_char,
               5 as libc::c_int as libc::c_uint) == 0 as libc::c_int {
        i = 5 as libc::c_int;
        while i < strlen(texfile.as_mut_ptr()) as SDWORD {
            if *(*__ctype_b_loc()).offset(texfile[i as usize] as libc::c_int
                                              as isize) as libc::c_int &
                   _ISdigit as libc::c_int as libc::c_ushort as libc::c_int ==
                   0 {
                break ;
            }
            i += 1
        }
        texfile[i as usize] = 0 as libc::c_int as STRING
    }
    SetLastResourceFilename(texfile.as_mut_ptr());
    debug(LOG_TEXTURE,
          b"bufferTexPageLoad: %s texturepage added (length=%d)\x00" as
              *const u8 as *const libc::c_char, texfile.as_mut_ptr(),
          strlen(texfile.as_mut_ptr()));
    // see if this texture page has already been loaded
    if resPresent(b"TEXPAGE\x00" as *const u8 as *const libc::c_char as
                      *mut STRING, texfile.as_mut_ptr()) != 0 {
        // replace the old texture page with the new one
        debug(LOG_TEXTURE,
              b"bufferTexPageLoad: replacing old\x00" as *const u8 as
                  *const libc::c_char);
        id = pie_ReloadTexPage(texfile.as_mut_ptr(), pBuffer);
        if id >= 0 as libc::c_int {
        } else {
            debug(LOG_ERROR,
                  b"pie_ReloadTexPage failed\x00" as *const u8 as
                      *const libc::c_char);
        };
        if id >= 0 as libc::c_int {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"data.c\x00" as *const u8 as *const libc::c_char,
                  898 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 18],
                                            &[libc::c_char; 18]>(b"bufferTexPageLoad\x00")).as_ptr(),
                  b"id >=0\x00" as *const u8 as *const libc::c_char);
        };
        *ppData = 0 as *mut libc::c_void
    } else {
        NewTexturePage =
            memMallocRelease(::std::mem::size_of::<TEXTUREPAGE>() as
                                 libc::c_ulong) as *mut TEXTUREPAGE;
        if NewTexturePage.is_null() { return 0 as libc::c_int }
        (*NewTexturePage).Texture = 0 as *mut iSprite;
        (*NewTexturePage).Palette = 0 as *mut iPalette;
        psPal =
            memMallocRelease(::std::mem::size_of::<iPalette>() as
                                 libc::c_ulong) as *mut iPalette;
        if psPal.is_null() { return 0 as libc::c_int }
        psSprite =
            memMallocRelease(::std::mem::size_of::<iSprite>() as
                                 libc::c_ulong) as *mut iSprite;
        if psSprite.is_null() { return 0 as libc::c_int }
        if pie_PNGLoadMem(pBuffer, psSprite, 0 as *mut iColour) == 0 {
            return 0 as libc::c_int
        }
        (*NewTexturePage).Texture = psSprite;
        (*NewTexturePage).Palette = psPal;
        //Hack mar8 to load	textures in order
/*	for(i=0;i<_TEX_INDEX;i++)
	{
		if (stricmp(texfile,_TEX_PAGE[i].name) != 0)
		{
			bFound = TRUE;
			break;
		}
	}
	if (!bFound)
*/
        pie_AddBMPtoTexPages(psSprite, texfile.as_mut_ptr(), 1 as libc::c_int,
                             0 as libc::c_int, 1 as libc::c_int);
        //Hack end
        *ppData = NewTexturePage as *mut libc::c_void
    }
    return 1 as libc::c_int;
}
/* Release an iSprite */
#[no_mangle]
pub unsafe extern "C" fn dataISpriteRelease(mut pData: *mut libc::c_void) {
    let mut psSprite: *mut iSprite = pData as *mut iSprite;
    if !psSprite.is_null() {
        if !(*psSprite).bmp.is_null() {
            free((*psSprite).bmp as *mut libc::c_void);
            (*psSprite).bmp = 0 as *mut iBitmap
        }
        memFreeRelease(psSprite as *mut libc::c_void);
        psSprite = 0 as *mut iSprite;
        psSprite = 0 as *mut iSprite
    };
}
/* Release a texPage */
#[no_mangle]
pub unsafe extern "C" fn dataTexPageRelease(mut pData: *mut libc::c_void) {
    let mut Tpage: *mut TEXTUREPAGE = pData as *mut TEXTUREPAGE;
    // We need to handle null texpage data
    if Tpage.is_null() { return }
    if !(*Tpage).Texture.is_null() {
        dataISpriteRelease((*Tpage).Texture as *mut libc::c_void);
    }
    if !(*Tpage).Palette.is_null() {
        memFreeRelease((*Tpage).Palette as *mut libc::c_void);
        (*Tpage).Palette = 0 as *mut iPalette
    }
    memFreeRelease(Tpage as *mut libc::c_void);
    Tpage = 0 as *mut TEXTUREPAGE;
}
/* Load an audio file */
#[no_mangle]
pub unsafe extern "C" fn dataAudioLoad(mut pBuffer: *mut libc::c_char,
                                       mut size: UDWORD,
                                       mut ppData: *mut *mut libc::c_void)
 -> BOOL {
    let mut psTrack: *mut TRACK = 0 as *mut TRACK;
    if audio_Disabled() == 1 as libc::c_int {
        *ppData = 0 as *mut libc::c_void;
        return 1 as libc::c_int
    } else {
        psTrack = audio_LoadTrackFromBuffer(pBuffer, size) as *mut TRACK;
        if psTrack.is_null() { return 0 as libc::c_int }
    }
    /* save track data */
    *ppData = psTrack as *mut libc::c_void;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dataAudioRelease(mut pData: *mut libc::c_void) {
    if audio_Disabled() == 0 as libc::c_int {
        let mut psTrack: *mut TRACK = pData as *mut TRACK;
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"dataAudioRelease: invalid track pointer\x00" as *const u8
                      as *const libc::c_char);
        };
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"data.c\x00" as *const u8 as *const libc::c_char,
                  1015 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 17],
                                            &[libc::c_char; 17]>(b"dataAudioRelease\x00")).as_ptr(),
                  b"PTRVALID(psTrack, sizeof(TRACK))\x00" as *const u8 as
                      *const libc::c_char);
        };
        audio_ReleaseTrack(psTrack);
        memFreeRelease(psTrack as *mut libc::c_void);
        psTrack = 0 as *mut TRACK
    };
}
/* Load an audio file */
#[no_mangle]
pub unsafe extern "C" fn dataAudioCfgLoad(mut pBuffer: *mut libc::c_char,
                                          mut size: UDWORD,
                                          mut ppData: *mut *mut libc::c_void)
 -> BOOL {
    *ppData = 0 as *mut libc::c_void;
    if audio_Disabled() == 0 as libc::c_int &&
           ParseResourceFile(pBuffer, size) == 0 as libc::c_int {
        return 0 as libc::c_int
    } else { return 1 as libc::c_int };
}
/* Load an anim file */
#[no_mangle]
pub unsafe extern "C" fn dataAnimLoad(mut pBuffer: *mut libc::c_char,
                                      mut size: UDWORD,
                                      mut ppData: *mut *mut libc::c_void)
 -> BOOL {
    let mut psAnim: *mut BASEANIM = 0 as *mut BASEANIM;
    psAnim = anim_LoadFromBuffer(pBuffer, size);
    if psAnim.is_null() { return 0 as libc::c_int }
    /* copy anim for return */
    *ppData = psAnim as *mut libc::c_void;
    return 1 as libc::c_int;
}
/* Load an audio config file */
#[no_mangle]
pub unsafe extern "C" fn dataAnimCfgLoad(mut pBuffer: *mut libc::c_char,
                                         mut size: UDWORD,
                                         mut ppData: *mut *mut libc::c_void)
 -> BOOL {
    *ppData = 0 as *mut libc::c_void;
    if ParseResourceFile(pBuffer, size) == 0 as libc::c_int {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dataAnimRelease(mut pData: *mut libc::c_void) {
    anim_ReleaseAnim(pData as *mut BASEANIM);
}
/* Load a string resource file */
#[no_mangle]
pub unsafe extern "C" fn dataStrResLoad(mut pBuffer: *mut libc::c_char,
                                        mut size: UDWORD,
                                        mut ppData: *mut *mut libc::c_void)
 -> BOOL {
    // recreate the string resource if it was freed by a WRF release
    if psStringRes.is_null() {
        if stringsInitialise() == 0 { return 0 as libc::c_int }
    }
    if strresLoad(psStringRes, pBuffer, size) == 0 { return 0 as libc::c_int }
    *ppData = psStringRes as *mut libc::c_void;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dataStrResRelease(mut pData: *mut libc::c_void) {
    if !psStringRes.is_null() {
        strresDestroy(psStringRes);
        psStringRes = 0 as *mut STR_RES
    };
}
/* Load a script file */
// All scripts, binary or otherwise are now passed through this routine
#[no_mangle]
pub unsafe extern "C" fn dataScriptLoad(mut pBuffer: *mut libc::c_char,
                                        mut size: UDWORD,
                                        mut ppData: *mut *mut libc::c_void)
 -> BOOL {
    let mut psProg: *mut SCRIPT_CODE = 0 as *mut SCRIPT_CODE;
    let mut psHeap: *mut BLOCK_HEAP = 0 as *mut BLOCK_HEAP;
    let mut printHack: BOOL = 0 as libc::c_int;
    calcCheatHash(pBuffer, size, 27 as libc::c_int as UDWORD);
    debug(LOG_WZ,
          b"COMPILING SCRIPT ...%s\x00" as *const u8 as *const libc::c_char,
          GetLastResourceFilename());
    // make sure the memory system uses normal malloc for a compile
    psHeap = memGetBlockHeap();
    memSetBlockHeap(0 as *mut _block_heap);
    scr_lineno = 1 as libc::c_int;
    if scriptCompile(pBuffer, size, &mut psProg, SCR_DEBUGINFO) == 0 {
        // see script.h
        debug(LOG_ERROR,
              b"Script %s did not compile\x00" as *const u8 as
                  *const libc::c_char, GetLastResourceFilename());
        return 0 as libc::c_int
    }
    memSetBlockHeap(psHeap);
    if printHack != 0 { cpPrintProgram(psProg); }
    *ppData = psProg as *mut libc::c_void;
    return 1 as libc::c_int;
}
// Load a script variable values file
#[no_mangle]
pub unsafe extern "C" fn dataScriptLoadVals(mut pBuffer: *mut libc::c_char,
                                            mut size: UDWORD,
                                            mut ppData:
                                                *mut *mut libc::c_void)
 -> BOOL {
    *ppData = 0 as *mut libc::c_void;
    calcCheatHash(pBuffer, size, 28 as libc::c_int as UDWORD);
    // don't load anything if a saved game is being loaded
    if saveFlag != 0 { return 1 as libc::c_int }
    debug(LOG_WZ,
          b"Loading script data %s\x00" as *const u8 as *const libc::c_char,
          GetLastResourceFilename());
    if scrvLoad(pBuffer, size) == 0 {
        debug(LOG_ERROR,
              b"Script %s did not compile\x00" as *const u8 as
                  *const libc::c_char, GetLastResourceFilename());
        return 0 as libc::c_int
    }
    *ppData = 0 as *mut libc::c_void;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dataSaveGameLoad(mut pFile: *mut libc::c_char,
                                          mut ppData: *mut *mut libc::c_void)
 -> BOOL {
    if stageTwoInitialise() == 0 { return 0 as libc::c_int }
    if loadGameInit(pFile) == 0 { return 0 as libc::c_int }
    if loadGame(pFile, (1 as libc::c_int == 0) as libc::c_int,
                1 as libc::c_int, 1 as libc::c_int) == 0 {
        return 0 as libc::c_int
    }
    if newMapInitialise() == 0 { return 0 as libc::c_int }
    //not interested in this value
    *ppData = 0 as *mut libc::c_void;
    return 1 as libc::c_int;
}
static mut ResourceTypes: [RES_TYPE_MIN; 46] =
    unsafe {
        [{
             let mut init =
                 RES_TYPE_MIN{aType:
                                  b"SWEAPON\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              buffLoad:
                                  Some(bufferSWEAPONLoad as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut *mut libc::c_void)
                                               -> BOOL),
                              release: None,
                              ResourceData:
                                  0 as *const libc::c_void as
                                      *mut libc::c_void,
                              HashedType: 0,};
             init
         },
         {
             let mut init =
                 RES_TYPE_MIN{aType:
                                  b"SBODY\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              buffLoad:
                                  Some(bufferSBODYLoad as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut *mut libc::c_void)
                                               -> BOOL),
                              release:
                                  Some(dataReleaseStats as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_void)
                                               -> ()),
                              ResourceData:
                                  0 as *const libc::c_void as
                                      *mut libc::c_void,
                              HashedType: 0,};
             init
         },
         {
             let mut init =
                 RES_TYPE_MIN{aType:
                                  b"SBRAIN\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              buffLoad:
                                  Some(bufferSBRAINLoad as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut *mut libc::c_void)
                                               -> BOOL),
                              release: None,
                              ResourceData:
                                  0 as *const libc::c_void as
                                      *mut libc::c_void,
                              HashedType: 0,};
             init
         },
         {
             let mut init =
                 RES_TYPE_MIN{aType:
                                  b"SPROP\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              buffLoad:
                                  Some(bufferSPROPLoad as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut *mut libc::c_void)
                                               -> BOOL),
                              release: None,
                              ResourceData:
                                  0 as *const libc::c_void as
                                      *mut libc::c_void,
                              HashedType: 0,};
             init
         },
         {
             let mut init =
                 RES_TYPE_MIN{aType:
                                  b"SSENSOR\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              buffLoad:
                                  Some(bufferSSENSORLoad as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut *mut libc::c_void)
                                               -> BOOL),
                              release: None,
                              ResourceData:
                                  0 as *const libc::c_void as
                                      *mut libc::c_void,
                              HashedType: 0,};
             init
         },
         {
             let mut init =
                 RES_TYPE_MIN{aType:
                                  b"SECM\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              buffLoad:
                                  Some(bufferSECMLoad as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut *mut libc::c_void)
                                               -> BOOL),
                              release: None,
                              ResourceData:
                                  0 as *const libc::c_void as
                                      *mut libc::c_void,
                              HashedType: 0,};
             init
         },
         {
             let mut init =
                 RES_TYPE_MIN{aType:
                                  b"SREPAIR\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              buffLoad:
                                  Some(bufferSREPAIRLoad as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut *mut libc::c_void)
                                               -> BOOL),
                              release: None,
                              ResourceData:
                                  0 as *const libc::c_void as
                                      *mut libc::c_void,
                              HashedType: 0,};
             init
         },
         {
             let mut init =
                 RES_TYPE_MIN{aType:
                                  b"SCONSTR\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              buffLoad:
                                  Some(bufferSCONSTRLoad as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut *mut libc::c_void)
                                               -> BOOL),
                              release: None,
                              ResourceData:
                                  0 as *const libc::c_void as
                                      *mut libc::c_void,
                              HashedType: 0,};
             init
         },
         {
             let mut init =
                 RES_TYPE_MIN{aType:
                                  b"SPROPTYPES\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              buffLoad:
                                  Some(bufferSPROPTYPESLoad as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut *mut libc::c_void)
                                               -> BOOL),
                              release: None,
                              ResourceData:
                                  0 as *const libc::c_void as
                                      *mut libc::c_void,
                              HashedType: 0,};
             init
         },
         {
             let mut init =
                 RES_TYPE_MIN{aType:
                                  b"SPROPSND\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              buffLoad:
                                  Some(bufferSPROPSNDLoad as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut *mut libc::c_void)
                                               -> BOOL),
                              release: None,
                              ResourceData:
                                  0 as *const libc::c_void as
                                      *mut libc::c_void,
                              HashedType: 0,};
             init
         },
         {
             let mut init =
                 RES_TYPE_MIN{aType:
                                  b"STERRTABLE\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              buffLoad:
                                  Some(bufferSTERRTABLELoad as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut *mut libc::c_void)
                                               -> BOOL),
                              release: None,
                              ResourceData:
                                  0 as *const libc::c_void as
                                      *mut libc::c_void,
                              HashedType: 0,};
             init
         },
         {
             let mut init =
                 RES_TYPE_MIN{aType:
                                  b"SSPECABIL\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              buffLoad:
                                  Some(bufferSSPECABILLoad as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut *mut libc::c_void)
                                               -> BOOL),
                              release: None,
                              ResourceData:
                                  0 as *const libc::c_void as
                                      *mut libc::c_void,
                              HashedType: 0,};
             init
         },
         {
             let mut init =
                 RES_TYPE_MIN{aType:
                                  b"SBPIMD\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              buffLoad:
                                  Some(bufferSBPIMDLoad as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut *mut libc::c_void)
                                               -> BOOL),
                              release: None,
                              ResourceData:
                                  0 as *const libc::c_void as
                                      *mut libc::c_void,
                              HashedType: 0,};
             init
         },
         {
             let mut init =
                 RES_TYPE_MIN{aType:
                                  b"SWEAPSND\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              buffLoad:
                                  Some(bufferSWEAPSNDLoad as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut *mut libc::c_void)
                                               -> BOOL),
                              release: None,
                              ResourceData:
                                  0 as *const libc::c_void as
                                      *mut libc::c_void,
                              HashedType: 0,};
             init
         },
         {
             let mut init =
                 RES_TYPE_MIN{aType:
                                  b"SWEAPMOD\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              buffLoad:
                                  Some(bufferSWEAPMODLoad as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut *mut libc::c_void)
                                               -> BOOL),
                              release: None,
                              ResourceData:
                                  0 as *const libc::c_void as
                                      *mut libc::c_void,
                              HashedType: 0,};
             init
         },
         {
             let mut init =
                 RES_TYPE_MIN{aType:
                                  b"STEMPL\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              buffLoad:
                                  Some(bufferSTEMPLLoad as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut *mut libc::c_void)
                                               -> BOOL),
                              release:
                                  Some(dataSTEMPLRelease as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_void)
                                               -> ()),
                              ResourceData:
                                  0 as *const libc::c_void as
                                      *mut libc::c_void,
                              HashedType: 0,};
             init
         },
         {
             let mut init =
                 RES_TYPE_MIN{aType:
                                  b"STEMPWEAP\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              buffLoad:
                                  Some(bufferSTEMPWEAPLoad as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut *mut libc::c_void)
                                               -> BOOL),
                              release: None,
                              ResourceData:
                                  0 as *const libc::c_void as
                                      *mut libc::c_void,
                              HashedType: 0,};
             init
         },
         {
             let mut init =
                 RES_TYPE_MIN{aType:
                                  b"SSTRUCT\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              buffLoad:
                                  Some(bufferSSTRUCTLoad as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut *mut libc::c_void)
                                               -> BOOL),
                              release:
                                  Some(dataSSTRUCTRelease as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_void)
                                               -> ()),
                              ResourceData:
                                  0 as *const libc::c_void as
                                      *mut libc::c_void,
                              HashedType: 0,};
             init
         },
         {
             let mut init =
                 RES_TYPE_MIN{aType:
                                  b"SSTRFUNC\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              buffLoad:
                                  Some(bufferSSTRFUNCLoad as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut *mut libc::c_void)
                                               -> BOOL),
                              release: None,
                              ResourceData:
                                  0 as *const libc::c_void as
                                      *mut libc::c_void,
                              HashedType: 0,};
             init
         },
         {
             let mut init =
                 RES_TYPE_MIN{aType:
                                  b"SSTRWEAP\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              buffLoad:
                                  Some(bufferSSTRWEAPLoad as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut *mut libc::c_void)
                                               -> BOOL),
                              release: None,
                              ResourceData:
                                  0 as *const libc::c_void as
                                      *mut libc::c_void,
                              HashedType: 0,};
             init
         },
         {
             let mut init =
                 RES_TYPE_MIN{aType:
                                  b"SSTRMOD\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              buffLoad:
                                  Some(bufferSSTRMODLoad as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut *mut libc::c_void)
                                               -> BOOL),
                              release: None,
                              ResourceData:
                                  0 as *const libc::c_void as
                                      *mut libc::c_void,
                              HashedType: 0,};
             init
         },
         {
             let mut init =
                 RES_TYPE_MIN{aType:
                                  b"SFEAT\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              buffLoad:
                                  Some(bufferSFEATLoad as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut *mut libc::c_void)
                                               -> BOOL),
                              release:
                                  Some(dataSFEATRelease as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_void)
                                               -> ()),
                              ResourceData:
                                  0 as *const libc::c_void as
                                      *mut libc::c_void,
                              HashedType: 0,};
             init
         },
         {
             let mut init =
                 RES_TYPE_MIN{aType:
                                  b"SFUNC\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              buffLoad:
                                  Some(bufferSFUNCLoad as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut *mut libc::c_void)
                                               -> BOOL),
                              release:
                                  Some(dataSFUNCRelease as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_void)
                                               -> ()),
                              ResourceData:
                                  0 as *const libc::c_void as
                                      *mut libc::c_void,
                              HashedType: 0,};
             init
         },
         {
             let mut init =
                 RES_TYPE_MIN{aType:
                                  b"RESCH\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              buffLoad:
                                  Some(bufferRESCHLoad as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut *mut libc::c_void)
                                               -> BOOL),
                              release:
                                  Some(dataRESCHRelease as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_void)
                                               -> ()),
                              ResourceData:
                                  0 as *const libc::c_void as
                                      *mut libc::c_void,
                              HashedType: 0,};
             init
         },
         {
             let mut init =
                 RES_TYPE_MIN{aType:
                                  b"RPREREQ\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              buffLoad:
                                  Some(bufferRPREREQLoad as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut *mut libc::c_void)
                                               -> BOOL),
                              release: None,
                              ResourceData:
                                  0 as *const libc::c_void as
                                      *mut libc::c_void,
                              HashedType: 0,};
             init
         },
         {
             let mut init =
                 RES_TYPE_MIN{aType:
                                  b"RCOMPRED\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              buffLoad:
                                  Some(bufferRCOMPREDLoad as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut *mut libc::c_void)
                                               -> BOOL),
                              release: None,
                              ResourceData:
                                  0 as *const libc::c_void as
                                      *mut libc::c_void,
                              HashedType: 0,};
             init
         },
         {
             let mut init =
                 RES_TYPE_MIN{aType:
                                  b"RCOMPRES\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              buffLoad:
                                  Some(bufferRCOMPRESLoad as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut *mut libc::c_void)
                                               -> BOOL),
                              release: None,
                              ResourceData:
                                  0 as *const libc::c_void as
                                      *mut libc::c_void,
                              HashedType: 0,};
             init
         },
         {
             let mut init =
                 RES_TYPE_MIN{aType:
                                  b"RSTRREQ\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              buffLoad:
                                  Some(bufferRSTRREQLoad as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut *mut libc::c_void)
                                               -> BOOL),
                              release: None,
                              ResourceData:
                                  0 as *const libc::c_void as
                                      *mut libc::c_void,
                              HashedType: 0,};
             init
         },
         {
             let mut init =
                 RES_TYPE_MIN{aType:
                                  b"RSTRRED\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              buffLoad:
                                  Some(bufferRSTRREDLoad as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut *mut libc::c_void)
                                               -> BOOL),
                              release: None,
                              ResourceData:
                                  0 as *const libc::c_void as
                                      *mut libc::c_void,
                              HashedType: 0,};
             init
         },
         {
             let mut init =
                 RES_TYPE_MIN{aType:
                                  b"RSTRRES\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              buffLoad:
                                  Some(bufferRSTRRESLoad as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut *mut libc::c_void)
                                               -> BOOL),
                              release: None,
                              ResourceData:
                                  0 as *const libc::c_void as
                                      *mut libc::c_void,
                              HashedType: 0,};
             init
         },
         {
             let mut init =
                 RES_TYPE_MIN{aType:
                                  b"RFUNC\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              buffLoad:
                                  Some(bufferRFUNCLoad as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut *mut libc::c_void)
                                               -> BOOL),
                              release: None,
                              ResourceData:
                                  0 as *const libc::c_void as
                                      *mut libc::c_void,
                              HashedType: 0,};
             init
         },
         {
             let mut init =
                 RES_TYPE_MIN{aType:
                                  b"SMSG\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              buffLoad:
                                  Some(bufferSMSGLoad as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut *mut libc::c_void)
                                               -> BOOL),
                              release:
                                  Some(dataSMSGRelease as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_void)
                                               -> ()),
                              ResourceData:
                                  0 as *const libc::c_void as
                                      *mut libc::c_void,
                              HashedType: 0,};
             init
         },
         {
             let mut init =
                 RES_TYPE_MIN{aType:
                                  b"SCRIPT\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              buffLoad:
                                  Some(dataScriptLoad as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut *mut libc::c_void)
                                               -> BOOL),
                              release:
                                  ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                          *mut SCRIPT_CODE)
                                                                     -> ()>,
                                                          RES_FREE>(Some(scriptFreeCode
                                                                             as
                                                                             unsafe extern "C" fn(_:
                                                                                                      *mut SCRIPT_CODE)
                                                                                 ->
                                                                                     ())),
                              ResourceData:
                                  0 as *const libc::c_void as
                                      *mut libc::c_void,
                              HashedType: 0,};
             init
         },
         {
             let mut init =
                 RES_TYPE_MIN{aType:
                                  b"SCRIPTVAL\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              buffLoad:
                                  Some(dataScriptLoadVals as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut *mut libc::c_void)
                                               -> BOOL),
                              release: None,
                              ResourceData:
                                  0 as *const libc::c_void as
                                      *mut libc::c_void,
                              HashedType: 0,};
             init
         },
         {
             let mut init =
                 RES_TYPE_MIN{aType:
                                  b"STR_RES\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              buffLoad:
                                  Some(dataStrResLoad as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut *mut libc::c_void)
                                               -> BOOL),
                              release:
                                  Some(dataStrResRelease as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_void)
                                               -> ()),
                              ResourceData:
                                  0 as *const libc::c_void as
                                      *mut libc::c_void,
                              HashedType: 0,};
             init
         },
         {
             let mut init =
                 RES_TYPE_MIN{aType:
                                  b"IMGPAGE\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              buffLoad:
                                  Some(dataIMGPAGELoad as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut *mut libc::c_void)
                                               -> BOOL),
                              release:
                                  Some(dataIMGPAGERelease as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_void)
                                               -> ()),
                              ResourceData:
                                  0 as *const libc::c_void as
                                      *mut libc::c_void,
                              HashedType: 0,};
             init
         },
         {
             let mut init =
                 RES_TYPE_MIN{aType:
                                  b"TERTILES\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              buffLoad: None,
                              release: None,
                              ResourceData:
                                  0 as *const libc::c_void as
                                      *mut libc::c_void,
                              HashedType: 0,};
             init
         },
         {
             let mut init =
                 RES_TYPE_MIN{aType:
                                  b"HWTERTILES\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              buffLoad:
                                  Some(dataHWTERTILESLoad as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut *mut libc::c_void)
                                               -> BOOL),
                              release:
                                  Some(dataHWTERTILESRelease as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_void)
                                               -> ()),
                              ResourceData:
                                  0 as *const libc::c_void as
                                      *mut libc::c_void,
                              HashedType: 0,};
             init
         },
         {
             let mut init =
                 RES_TYPE_MIN{aType:
                                  b"AUDIOCFG\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              buffLoad:
                                  Some(dataAudioCfgLoad as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut *mut libc::c_void)
                                               -> BOOL),
                              release: None,
                              ResourceData:
                                  0 as *const libc::c_void as
                                      *mut libc::c_void,
                              HashedType: 0,};
             init
         },
         {
             let mut init =
                 RES_TYPE_MIN{aType:
                                  b"WAV\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              buffLoad:
                                  Some(dataAudioLoad as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut *mut libc::c_void)
                                               -> BOOL),
                              release:
                                  Some(dataAudioRelease as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_void)
                                               -> ()),
                              ResourceData:
                                  0 as *const libc::c_void as
                                      *mut libc::c_void,
                              HashedType: 0,};
             init
         },
         {
             let mut init =
                 RES_TYPE_MIN{aType:
                                  b"ANI\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              buffLoad:
                                  Some(dataAnimLoad as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut *mut libc::c_void)
                                               -> BOOL),
                              release:
                                  Some(dataAnimRelease as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_void)
                                               -> ()),
                              ResourceData:
                                  0 as *const libc::c_void as
                                      *mut libc::c_void,
                              HashedType: 0,};
             init
         },
         {
             let mut init =
                 RES_TYPE_MIN{aType:
                                  b"ANIMCFG\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              buffLoad:
                                  Some(dataAnimCfgLoad as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut *mut libc::c_void)
                                               -> BOOL),
                              release: None,
                              ResourceData:
                                  0 as *const libc::c_void as
                                      *mut libc::c_void,
                              HashedType: 0,};
             init
         },
         {
             let mut init =
                 RES_TYPE_MIN{aType:
                                  b"IMG\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              buffLoad:
                                  Some(dataIMGLoad as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut *mut libc::c_void)
                                               -> BOOL),
                              release:
                                  Some(dataIMGRelease as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_void)
                                               -> ()),
                              ResourceData:
                                  0 as *const libc::c_void as
                                      *mut libc::c_void,
                              HashedType: 0,};
             init
         },
         {
             let mut init =
                 RES_TYPE_MIN{aType:
                                  b"TEXPAGE\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              buffLoad:
                                  Some(bufferTexPageLoad as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut *mut libc::c_void)
                                               -> BOOL),
                              release:
                                  Some(dataTexPageRelease as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_void)
                                               -> ()),
                              ResourceData:
                                  0 as *const libc::c_void as
                                      *mut libc::c_void,
                              HashedType: 0,};
             init
         },
         {
             let mut init =
                 RES_TYPE_MIN{aType:
                                  b"IMD\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              buffLoad:
                                  Some(dataIMDBufferLoad as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_char,
                                                                _: UDWORD,
                                                                _:
                                                                    *mut *mut libc::c_void)
                                               -> BOOL),
                              release:
                                  ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                          *mut iIMDShape)
                                                                     -> ()>,
                                                          RES_FREE>(Some(iV_IMDRelease
                                                                             as
                                                                             unsafe extern "C" fn(_:
                                                                                                      *mut iIMDShape)
                                                                                 ->
                                                                                     ())),
                              ResourceData:
                                  0 as *const libc::c_void as
                                      *mut libc::c_void,
                              HashedType: 0,};
             init
         },
         {
             let mut init =
                 RES_TYPE_MIN{aType: 0 as *const STRING as *mut STRING,
                              buffLoad: None,
                              release: None,
                              ResourceData:
                                  0 as *const libc::c_void as
                                      *mut libc::c_void,
                              HashedType: 0,};
             init
         }]
    };
/*
 * Data.h
 *
 * Data loading functions
 */
/* Pass all the data loading functions to the framework library */
/* Pass all the data loading functions to the framework library */
#[no_mangle]
pub unsafe extern "C" fn dataInitLoadFuncs() -> BOOL {
    let mut CurrentType: *mut RES_TYPE_MIN = 0 as *mut RES_TYPE_MIN;
    //	UDWORD	i;
    // init the cheat system;
    resetCheatHash(); // point to the first entry
    CurrentType = ResourceTypes.as_mut_ptr();
    // While there are still some entries in the list
    while !(*CurrentType).aType.is_null() {
        //		printf(" ==>%s\n",CurrentType->aType);	//TESTING -Q
        if resAddBufferLoad((*CurrentType).aType, (*CurrentType).buffLoad,
                            (*CurrentType).release) == 0 {
            return 0 as libc::c_int
            // error whilst adding a buffer load
        }
        CurrentType = CurrentType.offset(1)
    }
    // Now add the only file load left!
    if resAddFileLoad(b"SAVEGAME\x00" as *const u8 as *const libc::c_char as
                          *mut STRING,
                      Some(dataSaveGameLoad as
                               unsafe extern "C" fn(_: *mut libc::c_char,
                                                    _: *mut *mut libc::c_void)
                                   -> BOOL), None) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
