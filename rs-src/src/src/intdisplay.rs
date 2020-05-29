use ::libc;
extern "C" {
    pub type _formation;
    /* Write formatted output to S.  */
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
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
    /* This returns true if the key is currently depressed */
    #[no_mangle]
    fn keyDown(code: KEY_CODE) -> BOOL;
    #[no_mangle]
    fn mouseY() -> SDWORD;
    /* Warps the mouse to the given position */
    #[no_mangle]
    fn SetMousePos(nowt: UDWORD, x: UDWORD, y: UDWORD);
    /* Get the string from an ID number */
    #[no_mangle]
    fn strresGetString(psRes: *mut STR_RES, id: UDWORD) -> *mut STRING;
    #[no_mangle]
    fn memMallocRelease(Size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn memFreeRelease(pMemToFree: *mut libc::c_void);
    //*************************************************************************
    //*************************************************************************
    #[no_mangle]
    static mut colours: [uint8; 0];
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
    fn pie_Set2DClip(x0: libc::c_int, y0: libc::c_int, x1: libc::c_int,
                     y1: libc::c_int);
    #[no_mangle]
    fn pie_GetVideoBufferWidth() -> UDWORD;
    #[no_mangle]
    fn pie_GetVideoBufferHeight() -> UDWORD;
    #[no_mangle]
    fn audio_PlayTrack(iTrack: libc::c_int);
    #[no_mangle]
    fn droidType(psDroid: *mut DROID) -> DROID_TYPE;
    /* get demolish stat */
    #[no_mangle]
    fn structGetDemolishStat() -> *mut STRUCTURE_STATS;
    // Is a structure a factory of somekind?
    #[no_mangle]
    fn StructIsFactory(Struct: *mut STRUCTURE) -> BOOL;
    //returns the quantity of a specific template in the production list
    #[no_mangle]
    fn getProductionQuantity(psStructure: *mut STRUCTURE,
                             psTemplate: *mut DROID_TEMPLATE) -> UDWORD;
    /*returns the quantity of a specific template in the production list that 
have already been built*/
    #[no_mangle]
    fn getProductionBuilt(psStructure: *mut STRUCTURE,
                          psTemplate: *mut DROID_TEMPLATE) -> UDWORD;
    /*returns the power cost to build this structure*/
    #[no_mangle]
    fn structPowerToBuild(psStruct: *mut STRUCTURE) -> UDWORD;
    #[no_mangle]
    static mut apsStructLists: [*mut STRUCTURE; 8];
    //Sets all states
    #[no_mangle]
    fn pie_SetDepthBufferStatus(depthMode: DEPTH_MODE);
    //fog currently on
    #[no_mangle]
    fn pie_SetFogStatus(val: BOOL);
    #[no_mangle]
    fn pie_ImageFileID(ImageFile: *mut IMAGEFILE, ID: UWORD, x: libc::c_int,
                       y: libc::c_int);
    #[no_mangle]
    fn pie_ImageFileIDTile(ImageFile: *mut IMAGEFILE, ID: UWORD,
                           x: libc::c_int, y: libc::c_int, x0: libc::c_int,
                           y0: libc::c_int, Width: libc::c_int,
                           Height: libc::c_int);
    #[no_mangle]
    fn iV_SetFont(FontID: libc::c_int);
    #[no_mangle]
    fn iV_GetTextAboveBase() -> libc::c_int;
    #[no_mangle]
    fn iV_GetTextLineSize() -> libc::c_int;
    #[no_mangle]
    fn iV_GetTextWidth(String: *mut STRING) -> libc::c_int;
    #[no_mangle]
    fn iV_SetTextColour(Index: SWORD);
    #[no_mangle]
    fn pie_DrawText(string: *mut STRING, x: UDWORD, y: UDWORD);
    // vid.c 0.1 10-01-96.22-11-96
    //*************************************************************************
//patch
    //#define	iV_DrawColourTransImage	pie_ImageFileIDColour
    //*************************************************************************
    // Direct3D 640x480x16bit RGB renderer (mmx)
    // Direct3D 640x480x16bit hardware
    // Direct3D 640x480x16bit hardware
    // 3dfx Glide API
    // 16bit software mode for video
    // off-screen surface
    // PlayStation - added by tjc
    // undefined mode
    //*************************************************************************
// polygon flags	b0..b7: col, b24..b31: anim index
    //#define PIE_FLAT			0x00000100
    //#define PIE_WIRE			0x00000400
    //#define PIE_GOURAUD			0x00001000
    //#define PIE_TEXANIM			0x00004000	// PIE_TEX must be set also
    // - use playstation texture allocation method
    // Freshly created by the BSP 
    //*************************************************************************
    //*************************************************************************
    //*************************************************************************
    #[no_mangle]
    fn iV_SurfaceCreate(flags: uint32, width: libc::c_int,
                        height: libc::c_int, xp: libc::c_int, yp: libc::c_int,
                        buffer: *mut uint8) -> *mut iSurface;
    #[no_mangle]
    fn iV_GetImageHeightNoCC(ImageFile: *mut IMAGEFILE, ID: UWORD) -> UWORD;
    #[no_mangle]
    fn iV_SurfaceDestroy(s: *mut iSurface);
    #[no_mangle]
    static mut psRendSurface: *mut iSurface;
    #[no_mangle]
    fn iV_GetImageWidth(ImageFile: *mut IMAGEFILE, ID: UWORD) -> UWORD;
    #[no_mangle]
    fn iV_GetImageHeight(ImageFile: *mut IMAGEFILE, ID: UWORD) -> UWORD;
    #[no_mangle]
    fn iV_GetImageWidthNoCC(ImageFile: *mut IMAGEFILE, ID: UWORD) -> UWORD;
    #[no_mangle]
    fn pie_SetGeometricOffset(x: libc::c_int, y: libc::c_int);
    #[no_mangle]
    fn clipXY(x: SDWORD, y: SDWORD) -> BOOL;
    /*returns the graphic ID for a droid rank*/
    #[no_mangle]
    fn getDroidRankGraphic(psDroid: *mut DROID) -> UDWORD;
    #[no_mangle]
    static mut apsProxDisp: [*mut PROXIMITY_DISPLAY; 8];
    /* The store for the research stats */
    #[no_mangle]
    static mut asResearch: *mut RESEARCH;
    //List of pointers to arrays of PLAYER_RESEARCH[numResearch] for each player
    #[no_mangle]
    static mut asPlayerResList: [*mut PLAYER_RESEARCH; 8];
    // The maximum time for one frame (stops the clock running away when debugging)
// changed to /6 by ajl. if this needs to go back to ticks/10 then tell me. 
    /* The current time in the game world */
    #[no_mangle]
    static mut gameTime: UDWORD;
    /* The current time in the game world */
    #[no_mangle]
    static mut gameTime2: UDWORD;
    // Never stops.
    /* The time for the last frame */
    #[no_mangle]
    static mut frameTime2: UDWORD;
    /* Get widget structure */
    /* Find a widget in a screen from its ID number */
    #[no_mangle]
    fn widgGetFromID(psScreen: *mut W_SCREEN, id: UDWORD) -> *mut WIDGET;
    #[no_mangle]
    fn intRefreshScreen();
    #[no_mangle]
    static mut psWScreen: *mut W_SCREEN;
    #[no_mangle]
    static mut WFont: libc::c_int;
    #[no_mangle]
    static mut selectedPlayer: UDWORD;
    //access function for selected object in the interface
    #[no_mangle]
    fn getCurrentSelected() -> *mut BASE_OBJECT;
    #[no_mangle]
    static mut asPower: [*mut PLAYER_POWER; 8];
    /* Check the order state of a droid */
    #[no_mangle]
    fn orderState(psDroid: *mut DROID, order: DROID_ORDER) -> BOOL;
    /* Get the state of a droid order with an object */
    #[no_mangle]
    fn orderStateObj(psDroid: *mut DROID, order: DROID_ORDER,
                     ppsObj: *mut *mut BASE_OBJECT) -> BOOL;
    /* Get the state of a droid order with a location and a stat */
    #[no_mangle]
    fn orderStateStatsLoc(psDroid: *mut DROID, order: DROID_ORDER,
                          ppsStats: *mut *mut BASE_STATS, pX: *mut UDWORD,
                          pY: *mut UDWORD) -> BOOL;
    // to disable mouse pointer when using keys.
    #[no_mangle]
    static mut bUsingSlider: BOOL;
    #[no_mangle]
    static mut IntImages: *mut IMAGEFILE;
    // All the 2d graphics for the user interface.
    // A few useful defined frames.
    #[no_mangle]
    static mut FrameNormal: IMAGEFRAME;
    #[no_mangle]
    fn imageInitBitmaps() -> BOOL;
    #[no_mangle]
    fn imageDeleteBitmaps();
    #[no_mangle]
    fn DrawBegin();
    #[no_mangle]
    fn DrawEnd();
    #[no_mangle]
    fn RenderWindowFrame(Frame: *mut IMAGEFRAME, x: UDWORD, y: UDWORD,
                         Width: UDWORD, Height: UDWORD);
    #[no_mangle]
    fn getComponentDroidRadius(psDroid: *mut DROID) -> UDWORD;
    #[no_mangle]
    fn getComponentDroidTemplateRadius(psDroid: *mut DROID_TEMPLATE)
     -> UDWORD;
    #[no_mangle]
    fn getComponentRadius(psComponent: *mut BASE_STATS) -> UDWORD;
    #[no_mangle]
    fn getResearchRadius(Stat: *mut BASE_STATS) -> UDWORD;
    #[no_mangle]
    fn getStructureSize(psStructure: *mut STRUCTURE) -> UDWORD;
    #[no_mangle]
    fn getStructureStatSize(Stats: *mut STRUCTURE_STATS) -> UDWORD;
    #[no_mangle]
    fn displayIMDButton(IMDShape: *mut iIMDShape, Rotation: *mut iVector,
                        Position: *mut iVector, RotXYZ: BOOL, scale: SDWORD);
    #[no_mangle]
    fn displayStructureButton(psStructure: *mut STRUCTURE,
                              Rotation: *mut iVector, Position: *mut iVector,
                              RotXYZ: BOOL, scale: SDWORD);
    #[no_mangle]
    fn displayStructureStatButton(Stats: *mut STRUCTURE_STATS, Player: UDWORD,
                                  Rotation: *mut iVector,
                                  Position: *mut iVector, RotXYZ: BOOL,
                                  scale: SDWORD);
    #[no_mangle]
    fn displayComponentButton(Stat: *mut BASE_STATS, Rotation: *mut iVector,
                              Position: *mut iVector, RotXYZ: BOOL,
                              scale: SDWORD);
    #[no_mangle]
    fn displayResearchButton(Stat: *mut BASE_STATS, Rotation: *mut iVector,
                             Position: *mut iVector, RotXYZ: BOOL,
                             scale: SDWORD);
    #[no_mangle]
    fn displayComponentButtonTemplate(psTemplate: *mut DROID_TEMPLATE,
                                      Rotation: *mut iVector,
                                      Position: *mut iVector, RotXYZ: BOOL,
                                      scale: SDWORD);
    #[no_mangle]
    fn displayComponentButtonObject(psDroid: *mut DROID,
                                    Rotation: *mut iVector,
                                    Position: *mut iVector, RotXYZ: BOOL,
                                    scale: SDWORD);
    #[no_mangle]
    fn rescaleButtonObject(radius: SDWORD, baseScale: SDWORD,
                           baseRadius: SDWORD) -> SDWORD;
    // get the experience level of a command droid
    #[no_mangle]
    fn cmdDroidGetLevel(psCommander: *mut DROID) -> SDWORD;
    // get the maximum group size for a command droid
    #[no_mangle]
    fn cmdDroidMaxGroup(psCommander: *mut DROID) -> SDWORD;
    // count the members of a group
    #[no_mangle]
    fn grpNumMembers(psGroup: *mut DROID_GROUP) -> SDWORD;
    /* The string resource object */
    #[no_mangle]
    static mut psStringRes: *mut STR_RES;
    #[no_mangle]
    fn missionForReInforcements() -> BOOL;
}
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
pub type SBYTE = libc::c_schar;
pub type STRING = libc::c_char;
pub type UWORD = libc::c_ushort;
pub type SWORD = libc::c_short;
pub type UDWORD = libc::c_uint;
pub type SDWORD = libc::c_int;
pub type BOOL = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct POINT {
    pub x: libc::c_int,
    pub y: libc::c_int,
}
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
 * Input.h
 *
 * Prototypes for the keyboard and mouse input funcitons.
 */
/* Check the header files have been included from frame.h if they
 * are used outside of the framework library.
 */
/* The defines for all the key codes */
pub type _key_code = libc::c_uint;
pub const KEY_KPENTER: _key_code = 271;
pub const KEY_DELETE: _key_code = 127;
pub const KEY_INSERT: _key_code = 277;
pub const KEY_PAGEDOWN: _key_code = 281;
pub const KEY_DOWNARROW: _key_code = 274;
pub const KEY_END: _key_code = 279;
pub const KEY_RIGHTARROW: _key_code = 275;
pub const KEY_LEFTARROW: _key_code = 276;
pub const KEY_PAGEUP: _key_code = 280;
pub const KEY_UPARROW: _key_code = 273;
pub const KEY_HOME: _key_code = 278;
pub const KEY_RALT: _key_code = 307;
pub const KEY_KP_BACKSLASH: _key_code = 267;
pub const KEY_RCTRL: _key_code = 305;
pub const KEY_F12: _key_code = 293;
pub const KEY_F11: _key_code = 292;
pub const KEY_KP_FULLSTOP: _key_code = 266;
pub const KEY_KP_0: _key_code = 256;
pub const KEY_KP_3: _key_code = 259;
pub const KEY_KP_2: _key_code = 258;
pub const KEY_KP_1: _key_code = 257;
pub const KEY_KP_PLUS: _key_code = 270;
pub const KEY_KP_6: _key_code = 262;
pub const KEY_KP_5: _key_code = 261;
pub const KEY_KP_4: _key_code = 260;
pub const KEY_KP_MINUS: _key_code = 269;
pub const KEY_KP_9: _key_code = 265;
pub const KEY_KP_8: _key_code = 264;
pub const KEY_KP_7: _key_code = 263;
pub const KEY_SCROLLLOCK: _key_code = 302;
pub const KEY_NUMLOCK: _key_code = 300;
pub const KEY_F10: _key_code = 291;
pub const KEY_F9: _key_code = 290;
pub const KEY_F8: _key_code = 289;
pub const KEY_F7: _key_code = 288;
pub const KEY_F6: _key_code = 287;
pub const KEY_F5: _key_code = 286;
pub const KEY_F4: _key_code = 285;
pub const KEY_F3: _key_code = 284;
pub const KEY_F2: _key_code = 283;
pub const KEY_F1: _key_code = 282;
pub const KEY_CAPSLOCK: _key_code = 301;
pub const KEY_SPACE: _key_code = 32;
pub const KEY_LALT: _key_code = 308;
pub const KEY_KP_STAR: _key_code = 268;
pub const KEY_RSHIFT: _key_code = 303;
pub const KEY_FORWARDSLASH: _key_code = 47;
pub const KEY_FULLSTOP: _key_code = 46;
pub const KEY_COMMA: _key_code = 44;
pub const KEY_M: _key_code = 109;
pub const KEY_N: _key_code = 110;
pub const KEY_B: _key_code = 98;
pub const KEY_V: _key_code = 118;
pub const KEY_C: _key_code = 99;
pub const KEY_X: _key_code = 120;
pub const KEY_Z: _key_code = 122;
pub const KEY_BACKSLASH: _key_code = 92;
pub const KEY_LSHIFT: _key_code = 304;
pub const KEY_BACKQUOTE: _key_code = 96;
pub const KEY_QUOTE: _key_code = 39;
pub const KEY_SEMICOLON: _key_code = 59;
pub const KEY_L: _key_code = 108;
pub const KEY_K: _key_code = 107;
pub const KEY_J: _key_code = 106;
pub const KEY_H: _key_code = 104;
pub const KEY_G: _key_code = 103;
pub const KEY_F: _key_code = 102;
pub const KEY_D: _key_code = 100;
pub const KEY_S: _key_code = 115;
pub const KEY_A: _key_code = 97;
pub const KEY_LCTRL: _key_code = 306;
pub const KEY_RETURN: _key_code = 13;
pub const KEY_RBRACE: _key_code = 93;
pub const KEY_LBRACE: _key_code = 91;
pub const KEY_P: _key_code = 112;
pub const KEY_O: _key_code = 111;
pub const KEY_I: _key_code = 105;
pub const KEY_U: _key_code = 117;
pub const KEY_Y: _key_code = 121;
pub const KEY_T: _key_code = 116;
pub const KEY_R: _key_code = 114;
pub const KEY_E: _key_code = 101;
pub const KEY_W: _key_code = 119;
pub const KEY_Q: _key_code = 113;
pub const KEY_TAB: _key_code = 9;
pub const KEY_BACKSPACE: _key_code = 8;
pub const KEY_EQUALS: _key_code = 61;
pub const KEY_MINUS: _key_code = 45;
pub const KEY_0: _key_code = 48;
pub const KEY_9: _key_code = 57;
pub const KEY_8: _key_code = 56;
pub const KEY_7: _key_code = 55;
pub const KEY_6: _key_code = 54;
pub const KEY_5: _key_code = 53;
pub const KEY_4: _key_code = 52;
pub const KEY_3: _key_code = 51;
pub const KEY_2: _key_code = 50;
pub const KEY_1: _key_code = 49;
pub const KEY_ESC: _key_code = 27;
pub type KEY_CODE = _key_code;
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
pub type FRACT = libc::c_float;
// root of the tree
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
pub type int32 = libc::c_int;
pub type uint8 = libc::c_uchar;
pub type uint16 = libc::c_ushort;
pub type uint32 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iClip {
    pub left: libc::c_int,
    pub top: libc::c_int,
    pub right: libc::c_int,
    pub bottom: libc::c_int,
}
// The next free ID
//*************************************************************************
//
// Simple derived types
//
//*************************************************************************
pub type iBitmap = uint8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iSprite {
    pub width: libc::c_uint,
    pub height: libc::c_uint,
    pub bmp: *mut iBitmap,
}
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
/* **************************************************************************/
/* The type of object */
/* ID number of the object */
/* Object's location */
/* Object's direction +ve rotation about y axis*/
/* Object's pitch +ve nose up*/
/* Object's roll +ve left up, right down */
/* screen coordinate details */
/* Which player the object belongs to */
/* Which group selection is the droid currently in? */
/* Whether the object is selected */
/* might want this elsewhere */
/* Which cluster the object is a member of */
/* Whether object is visible to specific player */
/* When an object was destroyed, if 0 still alive */
/*BOOL			    (*damage)(pointerType	*psObject, */
/*UDWORD		damage, */
/* UDWORD		weaponClass); - Damage function */
/*UDWORD				emissionInterval;	how frequently does it puff out damage smoke?*/
/* when did it last puff out smoke? */
/* Data for fire damage */
/* TRUE if the object is in a fire */
/* When the object entered the fire */
/* How much damage has been done since the object */
/* entered the fire */
/* Pointer to next object in list */
/* Pointer to previois object in list */
/* **************************************************************************/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _base_object {
    pub type_0: OBJECT_TYPE,
    pub id: UDWORD,
    pub x: UWORD,
    pub y: UWORD,
    pub z: UWORD,
    pub direction: UWORD,
    pub pitch: SWORD,
    pub roll: SWORD,
    pub psNext: *mut _base_object,
    pub sDisplay: SCREEN_DISP_DATA,
    pub player: UBYTE,
    pub group: UBYTE,
    pub selected: UBYTE,
    pub cluster: UBYTE,
    pub visible: [UBYTE; 8],
    pub died: UDWORD,
    pub lastEmission: UDWORD,
    pub inFire: BOOL,
    pub burnStart: UDWORD,
    pub burnDamage: UDWORD,
}
/*
 * DisplayDef.h
 *
 * Definitions of the display structures
 *
 */
// ffs am
//#define BOUNDARY_X		(DISP_WIDTH/20)	   // proportional to resolution - Alex M
//#define	BOUNDARY_Y		(DISP_WIDTH/16)
//#define	BOUNDARY_X		(24)
//#define	BOUNDARY_Y		(24)
pub type SCREEN_DISP_DATA = _screen_disp_data;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _screen_disp_data {
    pub imd: *mut iIMDShape,
    pub frameNumber: UDWORD,
    pub screenX: UDWORD,
    pub screenY: UDWORD,
    pub screenR: UDWORD,
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
//UDWORD		dummy;
//	BOOL		drawnThisFrame;		// for sorting - have we drawn the imd already?
// last frame it was drawn
//	UDWORD		animFrame;			// anim Frame
//	SDWORD		bucketDepth;
//	BOOL		onScreen;
//	UDWORD		numTiles;
// only needed when generating the tree
/* **************************************************************************/
// These 1st three entries can NOT NOW be cast into a iVectorf *   (iVectorf on PC are doubles)
// these values form the plane equation ax+by+cz=d
// a point on the plane - in normal non-fract format
pub type PSBSPTREENODE = *mut BSPTREENODE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BSPTREENODE {
    pub link: [*mut BSPTREENODE; 2],
    pub Plane: PLANE,
    pub TriSameDir: BSPPOLYID,
    pub TriOppoDir: BSPPOLYID,
}
pub type BSPPOLYID = uint16;
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
pub struct iTexAnim {
    pub nFrames: libc::c_int,
    pub playbackRate: libc::c_int,
    pub textureWidth: libc::c_int,
    pub textureHeight: libc::c_int,
}
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
pub type VERTEXID = libc::c_int;
// points to first polygon in the BSP tree entry ... BSP_NextPoly in the iIMDPoly structure will point to the next entry
// id of the first polygon in the list ... or BSPPOLYID_TERMINATE for none
// id of the first polygon in the list ... or BSPPOLYID_TERMINATE for none
// only needed when generating the tree
/* **************************************************************************/
/*
 * base.h
 *
 * Definitions for the base object type.
 *
 */
/* **************************************************************************/
/* **************************************************************************/
/* **************************************************************************/
pub type OBJECT_TYPE = _object_type;
pub type _object_type = libc::c_uint;
pub const OBJ_TARGET: _object_type = 4;
pub const OBJ_BULLET: _object_type = 3;
pub const OBJ_FEATURE: _object_type = 2;
pub const OBJ_STRUCTURE: _object_type = 1;
pub const OBJ_DROID: _object_type = 0;
// Droids
// All Buildings
// Things like roads, trees, bridges, fires
// Comes out of guns, stupid :-)
// for the camera tracking
/*
 * StatsDef.h
 *
 * Structure definitions for the stats system
 *
 */
/* Elements common to all stats structures */
/* Unique ID of the item */
/* pointer to the text id name (i.e. short language-independant name) */
/* Stats common to all stats structs */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _base_stats {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
}
pub type BASE_STATS = _base_stats;
/*
if any types are added BEFORE 'COMP_BODY' - then Save/Load Game will have to be 
altered since it loops through the components from 1->MAX_COMP
*/
pub type _component_type = libc::c_uint;
pub const COMP_NUMCOMPONENTS: _component_type = 9;
//COMP_PROGRAM,		//this needs to be removed when save games changes
pub const COMP_WEAPON: _component_type = 8;
pub const COMP_CONSTRUCT: _component_type = 7;
pub const COMP_SENSOR: _component_type = 6;
pub const COMP_ECM: _component_type = 5;
pub const COMP_REPAIRUNIT: _component_type = 4;
//COMP_POWERPLANT,
pub const COMP_PROPULSION: _component_type = 3;
pub const COMP_BRAIN: _component_type = 2;
//COMP_ARMOUR,
pub const COMP_BODY: _component_type = 1;
pub const COMP_UNKNOWN: _component_type = 0;
//ALL components and structures and research topics have a tech level to which they belong
pub type _tech_level = libc::c_uint;
pub const MAX_TECH_LEVELS: _tech_level = 6;
pub const TECH_LEVEL_ALL: _tech_level = 5;
pub const TECH_LEVEL_TWO_THREE: _tech_level = 4;
pub const TECH_LEVEL_ONE_TWO: _tech_level = 3;
pub const TECH_LEVEL_THREE: _tech_level = 2;
pub const TECH_LEVEL_TWO: _tech_level = 1;
pub const TECH_LEVEL_ONE: _tech_level = 0;
pub type TECH_LEVEL = _tech_level;
/* **********************************************************************************
 *
 * Stats structures type definitions
 */
/* Stats common to all droid components */
/* Basic stats */
/* technology level of the component */
/* Power required to build the component */
/* Time required to build the component */
/* Component's weight */
/* Component's hit points */
/* Space the component takes in the droid */
/* Component's body points */
/* flag to indicate whether this component can*/
/* be used in the design screen*/
/* The IMD to draw for this component */
/* Stats common to all components */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _comp_base_stats {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
    pub techLevel: TECH_LEVEL,
    pub buildPower: UDWORD,
    pub buildPoints: UDWORD,
    pub weight: UDWORD,
    pub hitPoints: UDWORD,
    pub systemPoints: UDWORD,
    pub body: UDWORD,
    pub design: BOOL,
    pub pIMD: *mut iIMDShape,
}
pub type COMP_BASE_STATS = _comp_base_stats;
//only using KINETIC and HEAT for now
pub type _weapon_class = libc::c_uint;
//laser etc
//WC_MISC,		//others we haven't thought of! - classed as WC_HEAT now to save space in struct _droid AB 25/11/98 
pub const NUM_WEAPON_CLASS: _weapon_class = 2;
//bullets etc
//WC_EXPLOSIVE,	//rockets etc - classed as WC_KINETIC now to save space in struct _droid AB 25/11/98 
pub const WC_HEAT: _weapon_class = 1;
pub const WC_KINETIC: _weapon_class = 0;
pub type WEAPON_CLASS = _weapon_class;
// weapon subclasses used to define which weapons are affected by weapon upgrade functions
pub type _weapon_subclass = libc::c_uint;
pub const NUM_WEAPON_SUBCLASS: _weapon_subclass = 17;
pub const WSC_EMP: _weapon_subclass = 16;
pub const WSC_COMMAND: _weapon_subclass = 15;
pub const WSC_BOMB: _weapon_subclass = 14;
pub const WSC_LAS_SAT: _weapon_subclass = 13;
pub const WSC_SLOWROCKET: _weapon_subclass = 12;
pub const WSC_SLOWMISSILE: _weapon_subclass = 11;
pub const WSC_AAGUN: _weapon_subclass = 10;
pub const WSC_ELECTRONIC: _weapon_subclass = 9;
//WSC_CLOSECOMBAT,
pub const WSC_HOWITZERS: _weapon_subclass = 8;
pub const WSC_FLAME: _weapon_subclass = 7;
pub const WSC_GAUSS: _weapon_subclass = 6;
pub const WSC_ENERGY: _weapon_subclass = 5;
pub const WSC_ROCKET: _weapon_subclass = 4;
pub const WSC_MISSILE: _weapon_subclass = 3;
//WSC_ARTILLARY,
pub const WSC_MORTARS: _weapon_subclass = 2;
pub const WSC_CANNON: _weapon_subclass = 1;
pub const WSC_MGUN: _weapon_subclass = 0;
pub type WEAPON_SUBCLASS = _weapon_subclass;
// used to define which projectile model to use for the weapon
pub type _movement_model = libc::c_uint;
pub const NUM_MOVEMENT_MODEL: _movement_model = 6;
pub const MM_SWEEP: _movement_model = 5;
pub const MM_ERRATICDIRECT: _movement_model = 4;
pub const MM_HOMINGINDIRECT: _movement_model = 3;
pub const MM_HOMINGDIRECT: _movement_model = 2;
pub const MM_INDIRECT: _movement_model = 1;
pub const MM_DIRECT: _movement_model = 0;
pub type MOVEMENT_MODEL = _movement_model;
//used to modify the damage to a propuslion type (or structure) based on weapon
pub type _weapon_effect = libc::c_uint;
pub const WE_NUMEFFECTS: _weapon_effect = 6;
pub const WE_ANTI_AIRCRAFT: _weapon_effect = 5;
pub const WE_FLAMER: _weapon_effect = 4;
pub const WE_ARTILLERY_ROUND: _weapon_effect = 3;
pub const WE_BUNKER_BUSTER: _weapon_effect = 2;
pub const WE_ANTI_TANK: _weapon_effect = 1;
pub const WE_ANTI_PERSONNEL: _weapon_effect = 0;
pub type WEAPON_EFFECT = _weapon_effect;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _brain_stats {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
    pub techLevel: TECH_LEVEL,
    pub buildPower: UDWORD,
    pub buildPoints: UDWORD,
    pub weight: UDWORD,
    pub hitPoints: UDWORD,
    pub systemPoints: UDWORD,
    pub body: UDWORD,
    pub design: BOOL,
    pub pIMD: *mut iIMDShape,
    pub progCap: UDWORD,
    pub psWeaponStat: *mut _weapon_stats,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _weapon_stats {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
    pub techLevel: TECH_LEVEL,
    pub buildPower: UDWORD,
    pub buildPoints: UDWORD,
    pub weight: UDWORD,
    pub hitPoints: UDWORD,
    pub systemPoints: UDWORD,
    pub body: UDWORD,
    pub design: BOOL,
    pub pIMD: *mut iIMDShape,
    pub shortRange: UDWORD,
    pub longRange: UDWORD,
    pub minRange: UDWORD,
    pub shortHit: UDWORD,
    pub longHit: UDWORD,
    pub firePause: UDWORD,
    pub numExplosions: UDWORD,
    pub numRounds: UBYTE,
    pub reloadTime: UDWORD,
    pub damage: UDWORD,
    pub radius: UDWORD,
    pub radiusHit: UDWORD,
    pub radiusDamage: UDWORD,
    pub incenTime: UDWORD,
    pub incenDamage: UDWORD,
    pub incenRadius: UDWORD,
    pub flightSpeed: UDWORD,
    pub indirectHeight: UDWORD,
    pub fireOnMove: FIREONMOVE,
    pub weaponClass: WEAPON_CLASS,
    pub weaponSubClass: WEAPON_SUBCLASS,
    pub movementModel: MOVEMENT_MODEL,
    pub weaponEffect: WEAPON_EFFECT,
    pub recoilValue: UDWORD,
    pub rotate: UBYTE,
    pub maxElevation: UBYTE,
    pub minElevation: SBYTE,
    pub facePlayer: UBYTE,
    pub faceInFlight: UBYTE,
    pub effectSize: UBYTE,
    pub lightWorld: BOOL,
    pub surfaceToAir: UBYTE,
    pub vtolAttackRuns: UBYTE,
    pub directLife: UDWORD,
    pub radiusLife: UDWORD,
    pub pMountGraphic: *mut iIMDShape,
    pub pMuzzleGraphic: *mut iIMDShape,
    pub pInFlightGraphic: *mut iIMDShape,
    pub pTargetHitGraphic: *mut iIMDShape,
    pub pTargetMissGraphic: *mut iIMDShape,
    pub pWaterHitGraphic: *mut iIMDShape,
    pub pTrailGraphic: *mut iIMDShape,
    pub iAudioFireID: SDWORD,
    pub iAudioImpactID: SDWORD,
}
//weapon stats associated with this brain - for Command Droids
//defines the left and right sides for propulsion IMDs
/* Common stats */
// Max speed for the droid
// Type of propulsion used - index 
// into PropulsionTable
//works as all of the above together! - new for updates - added 11/06/99 AB
/* Common stats */
// Sensor range
// Sensor power (put against ecm power)
// specifies whether the Sensor is default or for the Turret
// used for combat
//time delay before associated weapon droids 'know' where the
//attack is from
// The turret mount to use
/* Common stats */
// ECM range
// ECM power (put against sensor power)
// specifies whether the ECM is default or for the Turret
// The turret mount to use
/* Adjustment to armour damage for non-penetrating hit */
/* Adjustment to armour damage for penetrating hit */
/* Common stats */
// How much damage is restored to Body Points 
// and armour each Repair Cycle
// whether armour can be repaired or not
// specifies whether the Repair is default or for the Turret
// time delay for repair cycle
// The turret mount to use
//no longer used 7/8/98
/*typedef struct _program_stats
{
	// Common stats - This structure doesn't actually need all the stats
	COMPONENT_STATS;		
	UDWORD		slots;				// How many brain slots the program takes
	UDWORD		order;				// The order activated by the program if any
	UDWORD		special;			// The special ability that the droid can perform
									// with this program
} PROGRAM_STATS;*/
/*these are defined in Access database - if you change them in there,
  then change them here! (and the rest of the code)
  They are made up values for now - defined when Jim does it!*/
/*typedef enum _program_orders
{
	ORDER_STOP,
	ORDER_SCAVANGE,
	ORDER_ATTACK,
	ORDER_GUARD,
	ORDER_AID,
	ORDER_BUILD,
	ORDER_DEMOLISH,
	ORDER_REPAIR,
} PROGRAM_ORDERS;*/
pub type FIREONMOVE = _fireonmove;
pub type _fireonmove = libc::c_uint;
pub const FOM_YES: _fireonmove = 2;
pub const FOM_PARTIAL: _fireonmove = 1;
pub const FOM_NO: _fireonmove = 0;
pub type BRAIN_STATS = _brain_stats;
pub type _sensor_type = libc::c_uint;
pub const SUPER_SENSOR: _sensor_type = 4;
pub const VTOL_INTERCEPT_SENSOR: _sensor_type = 3;
pub const VTOL_CB_SENSOR: _sensor_type = 2;
pub const INDIRECT_CB_SENSOR: _sensor_type = 1;
pub const STANDARD_SENSOR: _sensor_type = 0;
pub type SENSOR_TYPE = _sensor_type;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _sensor_stats {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
    pub techLevel: TECH_LEVEL,
    pub buildPower: UDWORD,
    pub buildPoints: UDWORD,
    pub weight: UDWORD,
    pub hitPoints: UDWORD,
    pub systemPoints: UDWORD,
    pub body: UDWORD,
    pub design: BOOL,
    pub pIMD: *mut iIMDShape,
    pub range: UDWORD,
    pub power: UDWORD,
    pub location: UDWORD,
    pub type_0: SENSOR_TYPE,
    pub time: UDWORD,
    pub pMountGraphic: *mut iIMDShape,
}
pub type SENSOR_STATS = _sensor_stats;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ecm_stats {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
    pub techLevel: TECH_LEVEL,
    pub buildPower: UDWORD,
    pub buildPoints: UDWORD,
    pub weight: UDWORD,
    pub hitPoints: UDWORD,
    pub systemPoints: UDWORD,
    pub body: UDWORD,
    pub design: BOOL,
    pub pIMD: *mut iIMDShape,
    pub range: UDWORD,
    pub power: UDWORD,
    pub location: UDWORD,
    pub pMountGraphic: *mut iIMDShape,
}
pub type ECM_STATS = _ecm_stats;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _repair_stats {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
    pub techLevel: TECH_LEVEL,
    pub buildPower: UDWORD,
    pub buildPoints: UDWORD,
    pub weight: UDWORD,
    pub hitPoints: UDWORD,
    pub systemPoints: UDWORD,
    pub body: UDWORD,
    pub design: BOOL,
    pub pIMD: *mut iIMDShape,
    pub repairPoints: UDWORD,
    pub repairArmour: BOOL,
    pub location: UDWORD,
    pub time: UDWORD,
    pub pMountGraphic: *mut iIMDShape,
}
pub type REPAIR_STATS = _repair_stats;
pub type WEAPON_STATS = _weapon_stats;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _construct_stats {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
    pub techLevel: TECH_LEVEL,
    pub buildPower: UDWORD,
    pub buildPoints: UDWORD,
    pub weight: UDWORD,
    pub hitPoints: UDWORD,
    pub systemPoints: UDWORD,
    pub body: UDWORD,
    pub design: BOOL,
    pub pIMD: *mut iIMDShape,
    pub constructPoints: UDWORD,
    pub pMountGraphic: *mut iIMDShape,
}
pub type CONSTRUCT_STATS = _construct_stats;
//no capability - droid must stop
//partial capability - droid has 50% chance to hit
//full capability - droid fires normally on move
// The turret mount to use
/*Common stats for all Structure Functions*/
/* Unique ID of the item */
/* Text name of the component */
/* The type of Function */
/*Common struct for all functions*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _function {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
    pub type_0: UBYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _structure_stats {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
    pub type_0: UDWORD,
    pub techLevel: TECH_LEVEL,
    pub strength: STRUCT_STRENGTH,
    pub terrainType: UDWORD,
    pub baseWidth: UDWORD,
    pub baseBreadth: UDWORD,
    pub foundationType: UDWORD,
    pub buildPoints: UDWORD,
    pub height: UDWORD,
    pub armourValue: UDWORD,
    pub bodyPoints: UDWORD,
    pub repairSystem: UDWORD,
    pub powerToBuild: UDWORD,
    pub resistance: UDWORD,
    pub sizeModifier: UDWORD,
    pub pIMD: *mut iIMDShape,
    pub pBaseIMD: *mut iIMDShape,
    pub pECM: *mut _ecm_stats,
    pub pSensor: *mut _sensor_stats,
    pub psWeapStat: *mut _weapon_stats,
    pub numFuncs: UDWORD,
    pub defaultFunc: SDWORD,
    pub asFuncList: *mut *mut _function,
}
/*
 * StructureDef.h
 *
 * Structure definitions for structures
 *
 */
// Used to indicate any kind of building when calling intGotoNextStructureType()
/* Defines for indexing an appropriate IMD object given a buildings purpose. */
//draw as factory 2	
//corner wall - no gun
//control centre for command droids
//the demolish structure type - should only be one stat with this type
//added for updates - AB 8/6/99
//REF_WALLH,     //the following are needed for the demo
//REF_WALLV,		
//REF_CORNER1,
//REF_CORNER2,
//REF_CORNER3,
//REF_CORNER4,
//REF_GATE1,
//REF_GATE2,
//REF_GATE3,
//REF_GATE4,
//REF_TOWER1,
//REF_TOWER2,
//REF_TOWER3,
//REF_TOWER4,
//REF_VANH,
//REF_VANV,
//REF_JEEP,
//REF_TANKERH,
//REF_TANKERV,
//need to keep a count of how many types for IMD loading
//Delivery Points NOT wayPoints
//proximity messages that are data generated
//proximity messages that are in game generated
//SAVE ONLY delivery point for factory currently assigned to commander
/*the type of position obj - FlagPos or ProxDisp*/
/*when the Position was last drawn*/
/*screen coords and radius of Position imd */
/*which player the Position belongs to*/
/*flag to indicate whether the Position 
										is to be highlighted*/
//the world coords of the Position
//UDWORD		frameNumber;					//when the Position was last drawn
	//UDWORD		screenX, screenY, screenR;		//screen coords and radius of Position imd
	//UDWORD		player;							//which player the Position belongs to
	//BOOL		selected;						//flag to indicate whether the 
												//Position is to be highlighted
//indicates whether the first, second etc factory
//indicates whether standard, cyborg or vtol factory
//	UBYTE		factorySub;						//sub value. needed to order production points.
//	UBYTE		primary;
//only allowed one weapon per structure (more memory for Tim) 
pub type STRUCT_STRENGTH = _struct_strength;
pub type _struct_strength = libc::c_uint;
pub const NUM_STRUCT_STRENGTH: _struct_strength = 4;
pub const STRENGTH_BUNKER: _struct_strength = 3;
pub const STRENGTH_HARD: _struct_strength = 2;
pub const STRENGTH_MEDIUM: _struct_strength = 1;
pub const STRENGTH_SOFT: _struct_strength = 0;
/*
 * MoveDef.h
 *
 * Structure definitions for movement structures.
 *
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _path_point {
    pub x: UBYTE,
    pub y: UBYTE,
}
pub type PATH_POINT = _path_point;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _move_control {
    pub Status: UBYTE,
    pub Mask: UBYTE,
    pub Position: UBYTE,
    pub numPoints: UBYTE,
    pub asPath: [PATH_POINT; 100],
    pub DestinationX: SDWORD,
    pub DestinationY: SDWORD,
    pub srcX: SDWORD,
    pub srcY: SDWORD,
    pub targetX: SDWORD,
    pub targetY: SDWORD,
    pub fx: FRACT,
    pub fy: FRACT,
    pub speed: FRACT,
    pub boundX: SWORD,
    pub boundY: SWORD,
    pub dir: SWORD,
    pub bumpDir: SWORD,
    pub bumpTime: UDWORD,
    pub lastBump: UWORD,
    pub pauseTime: UWORD,
    pub bumpX: UWORD,
    pub bumpY: UWORD,
    pub shuffleStart: UDWORD,
    pub psFormation: *mut _formation,
    pub iVertSpeed: SWORD,
    pub iAttackRuns: UWORD,
    pub fz: FRACT,
}
pub type MOVE_CONTROL = _move_control;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct research_stats {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
    pub techCode: UBYTE,
    pub techLevel: TECH_LEVEL,
    pub subGroup: UWORD,
    pub researchPoints: UWORD,
    pub researchPower: UDWORD,
    pub keyTopic: UBYTE,
    pub storeCount: UBYTE,
    pub numPRRequired: UBYTE,
    pub pPRList: *mut UWORD,
    pub numStructures: UBYTE,
    pub pStructList: *mut UWORD,
    pub numFunctions: UBYTE,
    pub pFunctionList: *mut *mut _function,
    pub numRedStructs: UBYTE,
    pub pRedStructs: *mut UWORD,
    pub numRedArtefacts: UBYTE,
    pub pRedArtefacts: *mut *mut COMP_BASE_STATS,
    pub numStructResults: UBYTE,
    pub pStructureResults: *mut UWORD,
    pub numArteResults: UBYTE,
    pub pArtefactResults: *mut *mut COMP_BASE_STATS,
    pub pReplacedArtefacts: *mut *mut COMP_BASE_STATS,
    pub pViewData: *mut _viewdata,
    pub iconID: UWORD,
    pub psStat: *mut BASE_STATS,
    pub pIMD: *mut iIMDShape,
    pub pIMD2: *mut iIMDShape,
}
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
pub type RESEARCH = research_stats;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _player_research {
    pub currentPoints: UDWORD,
    pub ResearchStatus: UBYTE,
}
pub type PLAYER_RESEARCH = _player_research;
/* Only needed for Alex's movement update ? */
//	UDWORD	timeStarted;
//	UDWORD	arrivalTime;
//	UDWORD	pathStarted;
//	BOOL	startedMoving;
//	UDWORD	lastTime;
//	BOOL	speedChange;
/* the 2nd IMD for base plates/turrets*/
// Bit flags   ...  see below
//	UBYTE		possible;				/* Flag to specify whether the research is possible - so
//										   can enable topics vis scripts */
//	UBYTE		researched;				/* Flag to specify whether the research is 
//										   complete	*/
/*
 * Weapons.h
 *
 * Definitions for the weapons
 *
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _weapon {
    pub nStat: UDWORD,
    pub hitPoints: UDWORD,
    pub ammo: UDWORD,
    pub lastFired: UDWORD,
    pub recoilValue: UDWORD,
}
pub type WEAPON = _weapon;
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
pub type BASE_OBJECT = _base_object;
/* **************************************************************************/
/* ensure ANIM2D/3D structs same size */
/* width of container bitmap */
/* ensure ANIM2D/3D structs same size */
/* **************************************************************************/
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
pub struct ANIM3D {
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
    pub psFrames: *mut iIMDShape,
    pub apFrame: *mut *mut iIMDShape,
}
/* **************************************************************************/
/*
 * Animobj.h
 *
 * Animation object types and function headers
 *
 * Gareth Jones 14/11/97
 */
/* **************************************************************************/
/* **************************************************************************/
/* **************************************************************************/
/* **************************************************************************/
/* forward struct declarations */
/* **************************************************************************/
/* typedefs */
/* **************************************************************************/
/* struct member macros */
/* this must be the last entry in this structure */
/* **************************************************************************/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ANIM_OBJECT {
    pub psNext: *mut ANIM_OBJECT,
    pub uwID: UWORD,
    pub psAnim: *mut ANIM3D,
    pub psParent: *mut libc::c_void,
    pub udwStartTime: UDWORD,
    pub udwStartDelay: UDWORD,
    pub uwCycles: UWORD,
    pub bVisible: BOOL,
    pub pDoneFunc: ANIMOBJDONEFUNC,
    pub apComponents: [COMPONENT_OBJECT; 10],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct COMPONENT_OBJECT {
    pub position: VECTOR3D,
    pub orientation: VECTOR3D,
    pub psParent: *mut libc::c_void,
    pub psShape: *mut iIMDShape,
}
pub type ANIMOBJDONEFUNC
    =
    Option<unsafe extern "C" fn(_: *mut ANIM_OBJECT) -> ()>;
/*
 * DroidDef.h
 *
 * Droid structure definitions
 */
/* The number of components in the asParts / asBits arrays */
//(COMP_NUMCOMPONENTS - 2)
/* The maximum number of droid weapons and programs */
//3
//#define DROID_MAXPROGS		3
// This should really be logarithmic
//defines how many times to perform the iteration on looking for a blank location
//used to get a location next to a droid - withinh one tile
/* The different types of droid */
// NOTE, if you add to, or change this list then you'll need
// to update the DroidSelectionWeights lookup table in Display.c
pub type _droid_type = libc::c_uint;
// Any droid, Only used as a parameter for intGotoNextDroidType(droidtype).
//cyborg repair droid - new for update 7/6/99
pub const DROID_ANY: _droid_type = 13;
//cyborg repair droid - new for update 28/5/99
pub const DROID_CYBORG_SUPER: _droid_type = 12;
//cyborg constructor droid - new for update 28/5/99
pub const DROID_CYBORG_REPAIR: _droid_type = 11;
// Default droid
pub const DROID_CYBORG_CONSTRUCT: _droid_type = 10;
// Repair droid
pub const DROID_DEFAULT: _droid_type = 9;
// Command droid
pub const DROID_REPAIR: _droid_type = 8;
// guess what this is!
pub const DROID_COMMAND: _droid_type = 7;
// cyborg-type thang
pub const DROID_TRANSPORTER: _droid_type = 6;
// person
pub const DROID_CYBORG: _droid_type = 5;
// Constructor droid
pub const DROID_PERSON: _droid_type = 4;
// ECM droid
pub const DROID_CONSTRUCT: _droid_type = 3;
// Sensor droid
pub const DROID_ECM: _droid_type = 2;
// Weapon droid
pub const DROID_SENSOR: _droid_type = 1;
pub const DROID_WEAPON: _droid_type = 0;
pub type DROID_TYPE = _droid_type;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _component {
    pub nStat: UBYTE,
}
pub type COMPONENT = _component;
// Allowing a maximum of 255 stats per file
//UDWORD					hitPoints; NOT USED?
/* Possibly a set of function pointers here to be called when
	 * damage is done to a component, power is low ...
	 */
// maximum number of queued orders
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _order_list {
    pub order: SDWORD,
    pub psOrderTarget: *mut libc::c_void,
    pub x: UWORD,
    pub y: UWORD,
    pub x2: UWORD,
    pub y2: UWORD,
}
pub type ORDER_LIST = _order_list;
//line build requires two sets of coords
// maximum number of characters in a droid name
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _droid_template {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
    pub aName: [STRING; 60],
    pub NameVersion: UBYTE,
    pub asParts: [SDWORD; 8],
    pub buildPoints: UDWORD,
    pub powerPoints: UDWORD,
    pub storeCount: UDWORD,
    pub numWeaps: UDWORD,
    pub asWeaps: [UDWORD; 1],
    pub droidType: DROID_TYPE,
    pub multiPlayerID: UDWORD,
    pub psNext: *mut _droid_template,
}
pub type DROID_TEMPLATE = _droid_template;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _droid {
    pub type_0: OBJECT_TYPE,
    pub id: UDWORD,
    pub x: UWORD,
    pub y: UWORD,
    pub z: UWORD,
    pub direction: UWORD,
    pub pitch: SWORD,
    pub roll: SWORD,
    pub psNext: *mut _droid,
    pub sDisplay: SCREEN_DISP_DATA,
    pub player: UBYTE,
    pub group: UBYTE,
    pub selected: UBYTE,
    pub cluster: UBYTE,
    pub visible: [UBYTE; 8],
    pub died: UDWORD,
    pub lastEmission: UDWORD,
    pub inFire: BOOL,
    pub burnStart: UDWORD,
    pub burnDamage: UDWORD,
    pub aName: [STRING; 60],
    pub droidType: DROID_TYPE,
    pub asBits: [COMPONENT; 8],
    pub weight: UDWORD,
    pub baseSpeed: UDWORD,
    pub sensorRange: UDWORD,
    pub sensorPower: UDWORD,
    pub ECMMod: UDWORD,
    pub originalBody: UDWORD,
    pub body: UDWORD,
    pub armour: [UDWORD; 2],
    pub numKills: UWORD,
    pub turretRotation: UWORD,
    pub turretPitch: UWORD,
    pub NameVersion: UBYTE,
    pub currRayAng: UBYTE,
    pub resistance: SWORD,
    pub asWeaps: [WEAPON; 1],
    pub psGroup: *mut _droid_group,
    pub psGrpNext: *mut _droid,
    pub psBaseStruct: *mut _structure,
    pub listSize: SDWORD,
    pub asOrderList: [ORDER_LIST; 10],
    pub order: SDWORD,
    pub orderX: UWORD,
    pub orderY: UWORD,
    pub orderX2: UWORD,
    pub orderY2: UWORD,
    pub lastHitWeapon: UDWORD,
    pub timeLastHit: UDWORD,
    pub bTargetted: BOOL,
    pub psTarget: *mut _base_object,
    pub psTarStats: *mut _base_stats,
    pub secondaryOrder: UDWORD,
    pub lastSync: UDWORD,
    pub action: SDWORD,
    pub actionX: UDWORD,
    pub actionY: UDWORD,
    pub psActionTarget: *mut _base_object,
    pub actionStarted: UDWORD,
    pub actionPoints: UDWORD,
    pub powerAccrued: UWORD,
    pub illumination: UBYTE,
    pub updateFlags: UBYTE,
    pub sMove: MOVE_CONTROL,
    pub psCurAnim: *mut ANIM_OBJECT,
    pub iAudioID: SDWORD,
}
/* Pointer to next template*/
//this structure is used whenever an instance of a building is required in game
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _structure {
    pub type_0: OBJECT_TYPE,
    pub id: UDWORD,
    pub x: UWORD,
    pub y: UWORD,
    pub z: UWORD,
    pub direction: UWORD,
    pub pitch: SWORD,
    pub roll: SWORD,
    pub psNext: *mut _structure,
    pub sDisplay: SCREEN_DISP_DATA,
    pub player: UBYTE,
    pub group: UBYTE,
    pub selected: UBYTE,
    pub cluster: UBYTE,
    pub visible: [UBYTE; 8],
    pub died: UDWORD,
    pub lastEmission: UDWORD,
    pub inFire: BOOL,
    pub burnStart: UDWORD,
    pub burnDamage: UDWORD,
    pub pStructureType: *mut STRUCTURE_STATS,
    pub status: UBYTE,
    pub currentBuildPts: SWORD,
    pub currentPowerAccrued: SWORD,
    pub body: UWORD,
    pub armour: UWORD,
    pub resistance: SWORD,
    pub lastResistance: UDWORD,
    pub sensorRange: UWORD,
    pub sensorPower: UWORD,
    pub turretRotation: UWORD,
    pub turretPitch: UWORD,
    pub timeLastHit: UDWORD,
    pub lastHitWeapon: UDWORD,
    pub radarX: UWORD,
    pub radarY: UWORD,
    pub ecmPower: UWORD,
    pub pFunctionality: *mut FUNCTIONALITY,
    pub targetted: UBYTE,
    pub asWeaps: [WEAPON; 1],
    pub psTarget: *mut BASE_OBJECT,
    pub psCurAnim: *mut ANIM_OBJECT,
}
//this structure is used to hold the permenant stats for each type of building
/* basic stats */
/* the type of structure */
/* technology level of the structure */
/* strength against the weapon effects */
/*The type of terrain the structure has to be 
									  built next to - may be none*/
/*The width of the base in tiles*/
/*The breadth of the base in tiles*/
/*The type of foundation for the structure*/
/*The number of build points required to build
									  the structure*/
/*The height above/below the terrain - negative
									  values denote below the terrain*/
/*The armour value for the structure - can be 
									  upgraded */
/*The structure's body points - A structure goes
									  off-line when 50% of its body points are lost*/
/*The repair system points are added to the body
									  points until fully restored . The points are 
									  then added to the Armour Points*/
/*How much power the structure requires to build*/
//NOT USED ANYMORE - AB 24/01/99
	/*UDWORD		minimumPower;		The minimum power requirement to start building
								      the structure*/
/*The number used to determine whether a 
									  structure can resist an enemy takeover - 
									  0 = cannot be attacked electrically*/
//NOT USED ANYMORE - AB 24/01/99
	/*UDWORD		quantityLimit;		The maximum number that a player can have - 
									  0 = no limit 1 = only 1 allowed etc*/
/*The larger the target, the easier to hit*/
/*The IMD to draw for this structure */
/*The base IMD to draw for this structure */
/*Which ECM is standard for the structure - 
									  if any*/
/*Which Sensor is standard for the structure - 
									  if any*/
//NOT USED ANYMORE - AB 24/01/99
	/*UDWORD		weaponSlots;		/Number of weapons that can be attached to the
									  building/
	UDWORD		numWeaps;			/Number of weapons for default /
	SDWORD		defaultWeap;		/The default weapon/
    
	struct _weapon_stats **asWeapList;		/List of pointers to default weapons/
    */
//can only have one weapon now
/*Number of functions for default*/
/*The default function*/
/*List of pointers to allowable functions - 
									  unalterable*/
//this is sizeof(FACTORY) the largest at present 11-2-99 - increased AB 22-04-99
pub type FUNCTIONALITY = [UBYTE; 40];
pub type STRUCTURE_STATS = _structure_stats;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _droid_group {
    pub type_0: SWORD,
    pub refCount: SWORD,
    pub psList: *mut DROID,
    pub psCommander: *mut DROID,
    pub sRunData: RUN_DATA,
}
//
// orderdef.h 
//
// order releated structures.
// data for barbarians retreating
pub type RUN_DATA = _run_data;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _run_data {
    pub sPos: POINT,
    pub forceLevel: UBYTE,
    pub healthLevel: UBYTE,
    pub leadership: UBYTE,
}
pub type DROID = _droid;
pub type C2RustUnnamed = libc::c_uint;
pub const NUM_DIFF_BUILDINGS: C2RustUnnamed = 22;
pub const REF_SAT_UPLINK: C2RustUnnamed = 21;
pub const REF_MISSILE_SILO: C2RustUnnamed = 20;
pub const REF_REARM_PAD: C2RustUnnamed = 19;
pub const REF_LAB: C2RustUnnamed = 18;
pub const REF_VTOL_FACTORY: C2RustUnnamed = 17;
pub const REF_CYBORG_FACTORY: C2RustUnnamed = 16;
pub const REF_DEMOLISH: C2RustUnnamed = 15;
pub const REF_BRIDGE: C2RustUnnamed = 14;
pub const REF_COMMAND_CONTROL: C2RustUnnamed = 13;
pub const REF_REPAIR_FACILITY: C2RustUnnamed = 12;
pub const REF_RESEARCH_MODULE: C2RustUnnamed = 11;
pub const REF_RESEARCH: C2RustUnnamed = 10;
pub const REF_BLASTDOOR: C2RustUnnamed = 9;
pub const REF_WALLCORNER: C2RustUnnamed = 8;
pub const REF_WALL: C2RustUnnamed = 7;
pub const REF_DEFENSE: C2RustUnnamed = 6;
pub const REF_RESOURCE_EXTRACTOR: C2RustUnnamed = 5;
pub const REF_POWER_MODULE: C2RustUnnamed = 4;
pub const REF_POWER_GEN: C2RustUnnamed = 3;
pub const REF_FACTORY_MODULE: C2RustUnnamed = 2;
pub const REF_FACTORY: C2RustUnnamed = 1;
pub const REF_HQ: C2RustUnnamed = 0;
pub type _position_type = libc::c_uint;
pub const POS_TEMPDELIVERY: _position_type = 3;
pub const POS_PROXOBJ: _position_type = 2;
pub const POS_PROXDATA: _position_type = 1;
pub const POS_DELIVERY: _position_type = 0;
pub type POSITION_TYPE = _position_type;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _flag_position {
    pub type_0: POSITION_TYPE,
    pub frameNumber: UDWORD,
    pub screenX: UDWORD,
    pub screenY: UDWORD,
    pub screenR: UDWORD,
    pub player: UDWORD,
    pub selected: BOOL,
    pub coords: iVector,
    pub factoryInc: UBYTE,
    pub factoryType: UBYTE,
    pub psNext: *mut _flag_position,
}
pub type FLAG_POSITION = _flag_position;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _research_facility {
    pub psSubject: *mut _base_stats,
    pub capacity: UDWORD,
    pub timeStarted: UDWORD,
    pub researchPoints: UDWORD,
    pub timeToResearch: UDWORD,
    pub psBestTopic: *mut _base_stats,
    pub powerAccrued: UDWORD,
    pub timeStartHold: UDWORD,
}
pub type RESEARCH_FACILITY = _research_facility;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _factory {
    pub capacity: UBYTE,
    pub quantity: UBYTE,
    pub loopsPerformed: UBYTE,
    pub productionOutput: UBYTE,
    pub powerAccrued: UDWORD,
    pub psSubject: *mut BASE_STATS,
    pub timeStarted: UDWORD,
    pub timeToBuild: UDWORD,
    pub timeStartHold: UDWORD,
    pub psAssemblyPoint: *mut FLAG_POSITION,
    pub psFormation: *mut _formation,
    pub psCommander: *mut _droid,
    pub secondaryOrder: UDWORD,
}
pub type FACTORY = _factory;
pub type STRUCTURE = _structure;
// position to retreat to
// number of units below which might run
// %health value below which to turn and run - FOR GROUPS ONLY
// basic chance to run
/* The time the research facility was put on hold*/
// secondary order state for all units coming out of the factory
                                            // added AB 22/04/99
//these are no longer required - yipee!
	// The group the droids produced by this factory belong to - used for Missions
	//struct _droid_group		*psGroup;
	//struct _droid			*psGrpNext;
/*
 * FeatureDef.h
 *
 * Structure definitions for features
 *
 */
pub type _feature_type = libc::c_uint;
pub const FEAT_SKYSCRAPER: _feature_type = 12;
pub const FEAT_TREE: _feature_type = 11;
pub const FEAT_OIL_DRUM: _feature_type = 10;
pub const FEAT_LOS_OBJ: _feature_type = 9;
pub const FEAT_DROID: _feature_type = 8;
pub const FEAT_BUILDING: _feature_type = 7;
pub const FEAT_VEHICLE: _feature_type = 6;
pub const FEAT_BOULDER: _feature_type = 5;
pub const FEAT_OIL_RESOURCE: _feature_type = 4;
pub const FEAT_GEN_ARTE: _feature_type = 3;
pub const FEAT_TANK: _feature_type = 2;
pub const FEAT_HOVER: _feature_type = 1;
pub const FEAT_BUILD_WRECK: _feature_type = 0;
pub type FEATURE_TYPE = _feature_type;
//FEAT_MESA,	no longer used
	//FEAT_MESA2,	
	//FEAT_CLIFF,
	//FEAT_STACK,
	//FEAT_BUILD_WRECK1,
	//FEAT_BUILD_WRECK2,
	//FEAT_BUILD_WRECK3,
	//FEAT_BUILD_WRECK4,
	//FEAT_BOULDER1,
	//FEAT_BOULDER2,
	//FEAT_BOULDER3,
	//FEAT_FUTCAR,
	//FEAT_FUTVAN,
/* Stats for a feature */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _feature_stats {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
    pub subType: FEATURE_TYPE,
    pub psImd: *mut iIMDShape,
    pub baseWidth: UWORD,
    pub baseBreadth: UWORD,
    pub tileDraw: BOOL,
    pub allowLOS: BOOL,
    pub visibleAtStart: BOOL,
    pub damageable: BOOL,
    pub body: UDWORD,
    pub armour: UDWORD,
}
pub type FEATURE_STATS = _feature_stats;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _feature {
    pub type_0: OBJECT_TYPE,
    pub id: UDWORD,
    pub x: UWORD,
    pub y: UWORD,
    pub z: UWORD,
    pub direction: UWORD,
    pub pitch: SWORD,
    pub roll: SWORD,
    pub psNext: *mut _feature,
    pub sDisplay: SCREEN_DISP_DATA,
    pub player: UBYTE,
    pub group: UBYTE,
    pub selected: UBYTE,
    pub cluster: UBYTE,
    pub visible: [UBYTE; 8],
    pub died: UDWORD,
    pub lastEmission: UDWORD,
    pub inFire: BOOL,
    pub burnStart: UDWORD,
    pub burnDamage: UDWORD,
    pub psStats: *mut FEATURE_STATS,
    pub startTime: UDWORD,
    pub body: UDWORD,
    pub gfxScaling: UWORD,
    pub timeLastHit: UDWORD,
    pub bTargetted: BOOL,
}
pub type FEATURE = _feature;
pub type DEPTH_MODE = libc::c_uint;
pub const DEPTH_CMP_ALWAYS_WRT_OFF: DEPTH_MODE = 3;
pub const DEPTH_CMP_LEQ_WRT_OFF: DEPTH_MODE = 2;
pub const DEPTH_CMP_ALWAYS_WRT_ON: DEPTH_MODE = 1;
pub const DEPTH_CMP_LEQ_WRT_ON: DEPTH_MODE = 0;
pub type _message_type = libc::c_uint;
pub const MSG_TYPES: _message_type = 4;
pub const MSG_PROXIMITY: _message_type = 3;
pub const MSG_MISSION: _message_type = 2;
pub const MSG_CAMPAIGN: _message_type = 1;
pub const MSG_RESEARCH: _message_type = 0;
pub type MESSAGE_TYPE = _message_type;
pub type _prox_type = libc::c_uint;
pub const PROX_TYPES: _prox_type = 3;
pub const PROX_ARTEFACT: _prox_type = 2;
pub const PROX_RESOURCE: _prox_type = 1;
pub const PROX_ENEMY: _prox_type = 0;
pub type PROX_TYPE = _prox_type;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _view_proximity {
    pub x: UDWORD,
    pub y: UDWORD,
    pub z: UDWORD,
    pub proxType: PROX_TYPE,
    pub audioID: SDWORD,
}
pub type VIEW_PROXIMITY = _view_proximity;
pub type VIEWDATA = _viewdata;
pub type MSG_VIEWDATA = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _message {
    pub type_0: MESSAGE_TYPE,
    pub id: UDWORD,
    pub pViewData: *mut MSG_VIEWDATA,
    pub read: BOOL,
    pub player: UDWORD,
    pub psNext: *mut _message,
}
pub type MESSAGE = _message;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _proximity_display {
    pub type_0: POSITION_TYPE,
    pub frameNumber: UDWORD,
    pub screenX: UDWORD,
    pub screenY: UDWORD,
    pub screenR: UDWORD,
    pub player: UDWORD,
    pub selected: BOOL,
    pub psMessage: *mut MESSAGE,
    pub radarX: UDWORD,
    pub radarY: UDWORD,
    pub timeLastDrawn: UDWORD,
    pub strobe: UDWORD,
    pub buttonID: UDWORD,
    pub psNext: *mut _proximity_display,
}
pub type PROXIMITY_DISPLAY = _proximity_display;
// Feature armour
/*
 * WidgBase.h
 *
 * Definitions for the basic widget types.
 */
/* The different base types of widget */
pub type _widget_type = libc::c_uint;
pub const WIDG_SLIDER: _widget_type = 5;
pub const WIDG_BARGRAPH: _widget_type = 4;
pub const WIDG_EDITBOX: _widget_type = 3;
pub const WIDG_BUTTON: _widget_type = 2;
pub const WIDG_LABEL: _widget_type = 1;
pub const WIDG_FORM: _widget_type = 0;
pub type WIDGET_TYPE = _widget_type;
/* Button colours */
// Colour for button text
// Colour for button background
// Colour for button border
// 2nd border colour
// Hilite colour
/* The display function prototype */
/* The optional user callback function */
/* The common widget data */
/* ID of the widgets base form. */
/* The user set ID number for the widget */
/* This is returned when e.g. a button is pressed */
/* The widget type */
/* The style of the widget */
/* The location of the widget */
/* The size of the widget */
/* Display the widget */
/* User callback (if any) */
/* Pointer to a user data block (if any) */
/* User data (if any) */
/* Pointer to the next widget in the screen list */
/* The base widget data type */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _widget {
    pub formID: UDWORD,
    pub id: UDWORD,
    pub type_0: WIDGET_TYPE,
    pub style: UDWORD,
    pub x: SWORD,
    pub y: SWORD,
    pub width: UWORD,
    pub height: UWORD,
    pub display: WIDGET_DISPLAY,
    pub callback: WIDGET_CALLBACK,
    pub pUserData: *mut libc::c_void,
    pub UserData: UDWORD,
    pub psNext: *mut _widget,
}
pub type WIDGET_CALLBACK
    =
    Option<unsafe extern "C" fn(_: *mut _widget, _: *mut _w_context) -> ()>;
/*
 * WidgInt.h
 *
 * Internal widget library definitions
 */
/* Control whether to use malloc for widgets */
/* Control whether the internal widget string heap should be used */
/* Context information to pass into the widget functions */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_context {
    pub psScreen: *mut W_SCREEN,
    pub psForm: *mut _w_form,
    pub xOffset: SDWORD,
    pub yOffset: SDWORD,
    pub mx: SDWORD,
    pub my: SDWORD,
}
/*
 * Form.h
 *
 * Definitions for the form functions.
 *
 */
/* The widget heaps */
/* The basic form data */
/* The common widget data */
/* Disable all child widgets if TRUE */
/* Working coords for animations. */
/* Animation counter. */
/* Animation start time */
/* Colours for the form and its widgets. signed since aColours -1 means use bitmap. */
/* The last widget to be hilited */
/* This is used to track when the mouse moves */
/* off something */
/* The widgets on the form */
/* The standard form */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_form {
    pub formID: UDWORD,
    pub id: UDWORD,
    pub type_0: WIDGET_TYPE,
    pub style: UDWORD,
    pub x: SWORD,
    pub y: SWORD,
    pub width: UWORD,
    pub height: UWORD,
    pub display: WIDGET_DISPLAY,
    pub callback: WIDGET_CALLBACK,
    pub pUserData: *mut libc::c_void,
    pub UserData: UDWORD,
    pub psNext: *mut _widget,
    pub disableChildren: BOOL,
    pub Ax0: UWORD,
    pub Ay0: UWORD,
    pub Ax1: UWORD,
    pub Ay1: UWORD,
    pub animCount: UDWORD,
    pub startTime: UDWORD,
    pub aColours: [UDWORD; 8],
    pub psLastHiLite: *mut WIDGET,
    pub psWidgets: *mut WIDGET,
}
pub type WIDGET = _widget;
pub type WIDGET_DISPLAY
    =
    Option<unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                _: *mut UDWORD) -> ()>;
/* The screen structure which stores all info for a widget screen */
pub type W_SCREEN = _w_screen;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_screen {
    pub psForm: *mut WIDGET,
    pub psFocus: *mut WIDGET,
    pub TipFontID: libc::c_int,
}
pub type WIDGET_AUDIOCALLBACK
    =
    Option<unsafe extern "C" fn(_: libc::c_int) -> ()>;
pub type TAB_DISPLAY
    =
    Option<unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                _: UDWORD, _: BOOL, _: BOOL, _: UDWORD,
                                _: UDWORD, _: UDWORD, _: UDWORD) -> ()>;
pub type FONT_DISPLAY
    =
    Option<unsafe extern "C" fn(_: UDWORD, _: UDWORD, _: *mut STRING) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _player_power {
    pub currentPower: UDWORD,
    pub extractedPower: UDWORD,
    pub psLastPowered: *mut _base_object,
}
// The root form of the screen
// The widget that has keyboard focus
//	PROP_FONT	*psTipFont;			// The font for tool tips
// ID of the IVIS font to use for tool tips.
/*
 * Power.h
 *
 * Definitions for the Power Functionality.
 *
 */
// free power on collection of oildrum.
//% used to determine the power cost of repairing a droid
                                         //definately DON'T WANT the brackets round 1/2 - it will equate to zero!
//used to multiply all repair calculations by to avaoid rounding errors
//OLD PLAYER_POWER
//typedef struct _player_power
//{
//	UDWORD		availablePower;		/* quantity that can be used from the Generators */
//	UDWORD		extractedPower;		/* quantity being extracted but not converted 
//									   by a Generator */
//	SDWORD		capacity;			/* the spare capacity of the generators */
//	SDWORD		usedPower;			/* quantity currently being used */
//} PLAYER_POWER;
//NEW PLAYER_POWER
pub type PLAYER_POWER = _player_power;
pub type _droid_order = libc::c_uint;
pub const DORDER_RTR_SPECIFIED: _droid_order = 37;
pub const DORDER_LEAVEMAP: _droid_order = 36;
pub const DORDER_RECOVER: _droid_order = 35;
pub const DORDER_SCOUT_ATTACKWALL: _droid_order = 34;
pub const DORDER_MOVE_ATTACKWALL: _droid_order = 33;
pub const DORDER_REARM: _droid_order = 32;
pub const DORDER_PATROL: _droid_order = 31;
pub const DORDER_CLEARWRECK: _droid_order = 30;
pub const DORDER_RUNBURN: _droid_order = 29;
pub const DORDER_SCOUT: _droid_order = 28;
pub const DORDER_RESTORE: _droid_order = 27;
pub const DORDER_DROIDREPAIR: _droid_order = 26;
pub const DORDER_GUARD: _droid_order = 25;
pub const DORDER_TRANSPORTRETURN: _droid_order = 24;
pub const DORDER_TRANSPORTIN: _droid_order = 23;
pub const DORDER_TRANSPORTOUT: _droid_order = 22;
pub const DORDER_RECYCLE: _droid_order = 21;
pub const DORDER_BUILDMODULE: _droid_order = 20;
pub const DORDER_COMMAND: _droid_order = 19;
pub const DORDER_ATTACKTARGET: _droid_order = 18;
pub const DORDER_DISEMBARK: _droid_order = 17;
pub const DORDER_EMBARK: _droid_order = 16;
pub const DORDER_RUN: _droid_order = 15;
pub const DORDER_RTR: _droid_order = 14;
pub const DORDER_RTB: _droid_order = 13;
pub const DORDER_DESTRUCT: _droid_order = 12;
pub const DORDER_RETREAT: _droid_order = 11;
pub const DORDER_FIRESUPPORT: _droid_order = 10;
pub const DORDER_OBSERVE: _droid_order = 9;
pub const DORDER_REPAIR: _droid_order = 8;
pub const DORDER_DEMOLISH: _droid_order = 7;
pub const DORDER_LINEBUILD: _droid_order = 6;
pub const DORDER_HELPBUILD: _droid_order = 5;
pub const DORDER_BUILD: _droid_order = 4;
pub const DORDER_ATTACK: _droid_order = 3;
pub const DORDER_MOVE: _droid_order = 2;
pub const DORDER_STOP: _droid_order = 1;
pub const DORDER_NONE: _droid_order = 0;
//UDWORD		initialPower;	    HAVEN'T FOUND A USE FOR IT YET! AB 26/8/98	
									    /* what the initial power level is - set 
									    in script not sure if will need it, but 
									    keeping for now*/
/* the current amount of power avaialble 
									    to the player*/
/* the power extracted but not converted */
/*the last object that received power 
									    before it ran out*/
/*
 * Order.h
 *
 * Function prototypes for giving droids orders
 *
 */
//turn off the build queue availability until desired release date!
//#define DISABLE_BUILD_QUEUE
// The droid orders
pub type DROID_ORDER = _droid_order;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FRAMERECT {
    pub Type: UWORD,
    pub TLXOffset: SWORD,
    pub TLYOffset: SWORD,
    pub BRXOffset: SWORD,
    pub BRYOffset: SWORD,
    pub ColourIndex: UBYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IMAGEFRAME {
    pub OffsetX0: SWORD,
    pub OffsetY0: SWORD,
    pub OffsetX1: SWORD,
    pub OffsetY1: SWORD,
    pub TopLeft: SWORD,
    pub TopRight: SWORD,
    pub BottomLeft: SWORD,
    pub BottomRight: SWORD,
    pub TopEdge: SWORD,
    pub TopType: SWORD,
    pub RightEdge: SWORD,
    pub RightType: SWORD,
    pub BottomEdge: SWORD,
    pub BottomType: SWORD,
    pub LeftEdge: SWORD,
    pub LeftType: SWORD,
    pub FRect: [FRAMERECT; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TABDEF {
    pub MajorUp: SWORD,
    pub MajorDown: SWORD,
    pub MajorHilight: SWORD,
    pub MajorSelected: SWORD,
    pub MinorUp: SWORD,
    pub MinorDown: SWORD,
    pub MinorHilight: SWORD,
    pub MinorSelected: SWORD,
}
// no order set
// stop the current order
// 2 - move to a location
// attack an enemy
// 4 - build a structure
// help to build a structure
// 6 - build a number of structures in a row (walls + bridges)
// demolish a structure
// 8 - repair a structure
// keep a target in sensor view
// 10 - attack whatever the linked sensor droid attacks
// return to the players retreat position
// 12 - self destruct
// return to base
// 14 - return to repair at any repair facility
// run away after moral failure
// 16 - board a transporter
// get off a transporter
// 18 - a suggestion to attack something
// i.e. the target was chosen because the droid could see it
// a command droid issuing orders to it's group
// 20 - build a module (power, research or factory)
// return to factory to be recycled
// 22 - offworld transporter order
// onworld transporter order
// 24 - transporter return after unloading
// guard a structure
// 26 - repair a droid
// restore resistance points for a structure
// 28 - same as move, but stop if an enemy is seen
// run away on fire
// 30 - constructor droid to clear up building wreckage
// move between two way points
// 32 - order a vtol to rearming pad
// move to a location taking out a blocking wall on the way
// 34 - scout to a location taking out a blocking wall on the way
// pick up an artifact
// 36 - vtol flying off the map
// return to repair at a specified repair center
/* **********************************************/
/* Image ID definition file created by Framer. */
/* **********************************************/
pub type C2RustUnnamed_0 = libc::c_uint;
pub const IMAGE_ASCII64: C2RustUnnamed_0 = 483;
pub const IMAGE_ORD_FIREDES_DOWN: C2RustUnnamed_0 = 482;
pub const IMAGE_ORD_FIREDES_UP: C2RustUnnamed_0 = 481;
pub const IMAGE_TRANSETA_DOWN: C2RustUnnamed_0 = 480;
pub const IMAGE_LAUNCHUP: C2RustUnnamed_0 = 479;
pub const IMAGE_BLUE6: C2RustUnnamed_0 = 478;
pub const IMAGE_BLUE5: C2RustUnnamed_0 = 477;
pub const IMAGE_BLUE4: C2RustUnnamed_0 = 476;
pub const IMAGE_BLUE3: C2RustUnnamed_0 = 475;
pub const IMAGE_BLUE2: C2RustUnnamed_0 = 474;
pub const IMAGE_BLUE1: C2RustUnnamed_0 = 473;
pub const IMAGE_DES_ARMOUR_KINETIC: C2RustUnnamed_0 = 472;
pub const IMAGE_TABHILIGHT_SM: C2RustUnnamed_0 = 471;
pub const IMAGE_TAB1DOWN_SM: C2RustUnnamed_0 = 470;
pub const IMAGE_TAB1SELECTED_SM: C2RustUnnamed_0 = 469;
pub const IMAGE_TAB1_SM: C2RustUnnamed_0 = 468;
pub const IMAGE_TARGET5: C2RustUnnamed_0 = 467;
pub const IMAGE_TARGET4: C2RustUnnamed_0 = 466;
pub const IMAGE_TARGET3: C2RustUnnamed_0 = 465;
pub const IMAGE_TARGET2: C2RustUnnamed_0 = 464;
pub const IMAGE_TARGET1: C2RustUnnamed_0 = 463;
pub const IMAGE_MISSION_CLOCK_UP: C2RustUnnamed_0 = 462;
pub const IMAGE_RES_GRPDAM: C2RustUnnamed_0 = 461;
pub const IMAGE_RES_GRPACC: C2RustUnnamed_0 = 460;
pub const IMAGE_RES_GRPROF: C2RustUnnamed_0 = 459;
pub const IMAGE_RES_GRPREP: C2RustUnnamed_0 = 458;
pub const IMAGE_RES_GRPUPG: C2RustUnnamed_0 = 457;
pub const IMAGE_ASCII131: C2RustUnnamed_0 = 456;
pub const IMAGE_ASCII161: C2RustUnnamed_0 = 455;
pub const IMAGE_ASCII191: C2RustUnnamed_0 = 454;
pub const IMAGE_ASCII208: C2RustUnnamed_0 = 453;
pub const IMAGE_ASCII207: C2RustUnnamed_0 = 452;
pub const IMAGE_ASCII206: C2RustUnnamed_0 = 451;
pub const IMAGE_ASCII205: C2RustUnnamed_0 = 450;
pub const IMAGE_ASCII204: C2RustUnnamed_0 = 449;
pub const IMAGE_ASCII203: C2RustUnnamed_0 = 448;
pub const IMAGE_ASCII202: C2RustUnnamed_0 = 447;
pub const IMAGE_ASCII201: C2RustUnnamed_0 = 446;
pub const IMAGE_ASCII200: C2RustUnnamed_0 = 445;
pub const IMAGE_ASCII198: C2RustUnnamed_0 = 444;
pub const IMAGE_ASCII197: C2RustUnnamed_0 = 443;
pub const IMAGE_ASCII196: C2RustUnnamed_0 = 442;
pub const IMAGE_ASCII195: C2RustUnnamed_0 = 441;
pub const IMAGE_ASCII194: C2RustUnnamed_0 = 440;
pub const IMAGE_ASCII193: C2RustUnnamed_0 = 439;
pub const IMAGE_ASCII192: C2RustUnnamed_0 = 438;
pub const IMAGE_ASCII210: C2RustUnnamed_0 = 437;
pub const IMAGE_ASCII211: C2RustUnnamed_0 = 436;
pub const IMAGE_ASCII214: C2RustUnnamed_0 = 435;
pub const IMAGE_ASCII213: C2RustUnnamed_0 = 434;
pub const IMAGE_ASCII212: C2RustUnnamed_0 = 433;
pub const IMAGE_ASCII216: C2RustUnnamed_0 = 432;
pub const IMAGE_ASCII220: C2RustUnnamed_0 = 431;
pub const IMAGE_ASCII217: C2RustUnnamed_0 = 430;
pub const IMAGE_ASCII219: C2RustUnnamed_0 = 429;
pub const IMAGE_ASCII218: C2RustUnnamed_0 = 428;
pub const IMAGE_ASCII221: C2RustUnnamed_0 = 427;
pub const IMAGE_ASCII223: C2RustUnnamed_0 = 426;
pub const IMAGE_ASCII248: C2RustUnnamed_0 = 425;
pub const IMAGE_ASCII241: C2RustUnnamed_0 = 424;
pub const IMAGE_ASCII253: C2RustUnnamed_0 = 423;
pub const IMAGE_ASCII252: C2RustUnnamed_0 = 422;
pub const IMAGE_ASCII251: C2RustUnnamed_0 = 421;
pub const IMAGE_ASCII250: C2RustUnnamed_0 = 420;
pub const IMAGE_ASCII249: C2RustUnnamed_0 = 419;
pub const IMAGE_ASCII246: C2RustUnnamed_0 = 418;
pub const IMAGE_ASCII245: C2RustUnnamed_0 = 417;
pub const IMAGE_ASCII244: C2RustUnnamed_0 = 416;
pub const IMAGE_ASCII243: C2RustUnnamed_0 = 415;
pub const IMAGE_ASCII242: C2RustUnnamed_0 = 414;
pub const IMAGE_ASCII239: C2RustUnnamed_0 = 413;
pub const IMAGE_ASCII238: C2RustUnnamed_0 = 412;
pub const IMAGE_ASCII237: C2RustUnnamed_0 = 411;
pub const IMAGE_ASCII236: C2RustUnnamed_0 = 410;
pub const IMAGE_ASCII235: C2RustUnnamed_0 = 409;
pub const IMAGE_ASCII234: C2RustUnnamed_0 = 408;
pub const IMAGE_ASCII233: C2RustUnnamed_0 = 407;
pub const IMAGE_ASCII232: C2RustUnnamed_0 = 406;
pub const IMAGE_ASCII231: C2RustUnnamed_0 = 405;
pub const IMAGE_ASCII230: C2RustUnnamed_0 = 404;
pub const IMAGE_ASCII229: C2RustUnnamed_0 = 403;
pub const IMAGE_ASCII228: C2RustUnnamed_0 = 402;
pub const IMAGE_ASCII227: C2RustUnnamed_0 = 401;
pub const IMAGE_ASCII226: C2RustUnnamed_0 = 400;
pub const IMAGE_ASCII225: C2RustUnnamed_0 = 399;
pub const IMAGE_ASCII224: C2RustUnnamed_0 = 398;
pub const IMAGE_ASCII189: C2RustUnnamed_0 = 397;
pub const IMAGE_ASCII188: C2RustUnnamed_0 = 396;
pub const IMAGE_RES_DEFENCE: C2RustUnnamed_0 = 395;
pub const IMAGE_LEV_7: C2RustUnnamed_0 = 394;
pub const IMAGE_LEV_6: C2RustUnnamed_0 = 393;
pub const IMAGE_LEV_5: C2RustUnnamed_0 = 392;
pub const IMAGE_LEV_4: C2RustUnnamed_0 = 391;
pub const IMAGE_LEV_3: C2RustUnnamed_0 = 390;
pub const IMAGE_LEV_2: C2RustUnnamed_0 = 389;
pub const IMAGE_LEV_1: C2RustUnnamed_0 = 388;
pub const IMAGE_LEV_0: C2RustUnnamed_0 = 387;
pub const IMAGE_CDCHANGE_CANCEL: C2RustUnnamed_0 = 386;
pub const IMAGE_CDCHANGE_OK: C2RustUnnamed_0 = 385;
pub const IMAGE_MULTI_CHAN: C2RustUnnamed_0 = 384;
pub const IMAGE_MULTI_NOCHAN: C2RustUnnamed_0 = 383;
pub const IMAGE_ORD_EMBARKDOWN: C2RustUnnamed_0 = 382;
pub const IMAGE_ORD_EMBARKUP: C2RustUnnamed_0 = 381;
pub const IMAGE_ORD_RTRDOWN: C2RustUnnamed_0 = 380;
pub const IMAGE_ORD_RTRUP: C2RustUnnamed_0 = 379;
pub const IMAGE_ORD_GUARDDOWN: C2RustUnnamed_0 = 378;
pub const IMAGE_ORD_GUARDUP: C2RustUnnamed_0 = 377;
pub const IMAGE_ORD_PERSUEDOWN: C2RustUnnamed_0 = 376;
pub const IMAGE_ORD_PERSUEUP: C2RustUnnamed_0 = 375;
pub const IMAGE_ORD_PATROLDOWN: C2RustUnnamed_0 = 374;
pub const IMAGE_ORD_PATROLUP: C2RustUnnamed_0 = 373;
pub const IMAGE_GN_0: C2RustUnnamed_0 = 372;
pub const IMAGE_GN_1: C2RustUnnamed_0 = 371;
pub const IMAGE_GN_2: C2RustUnnamed_0 = 370;
pub const IMAGE_GN_3: C2RustUnnamed_0 = 369;
pub const IMAGE_GN_4: C2RustUnnamed_0 = 368;
pub const IMAGE_GN_5: C2RustUnnamed_0 = 367;
pub const IMAGE_GN_6: C2RustUnnamed_0 = 366;
pub const IMAGE_GN_7: C2RustUnnamed_0 = 365;
pub const IMAGE_GN_8: C2RustUnnamed_0 = 364;
pub const IMAGE_GN_9: C2RustUnnamed_0 = 363;
pub const IMAGE_GN_STAR: C2RustUnnamed_0 = 362;
pub const IMAGE_VDP_HI: C2RustUnnamed_0 = 361;
pub const IMAGE_VDP_UP: C2RustUnnamed_0 = 360;
pub const IMAGE_VDP_DOWN: C2RustUnnamed_0 = 359;
pub const IMAGE_FDP_HI: C2RustUnnamed_0 = 358;
pub const IMAGE_FDP_UP: C2RustUnnamed_0 = 357;
pub const IMAGE_FDP_DOWN: C2RustUnnamed_0 = 356;
pub const IMAGE_CDP_HI: C2RustUnnamed_0 = 355;
pub const IMAGE_CDP_UP: C2RustUnnamed_0 = 354;
pub const IMAGE_CDP_DOWN: C2RustUnnamed_0 = 353;
pub const IMAGE_LOOP_HI: C2RustUnnamed_0 = 352;
pub const IMAGE_LOOP_DOWN: C2RustUnnamed_0 = 351;
pub const IMAGE_LOOP_UP: C2RustUnnamed_0 = 350;
pub const IMAGE_RES_CYBORGTECH: C2RustUnnamed_0 = 349;
pub const IMAGE_CURSOR_DEST: C2RustUnnamed_0 = 348;
pub const IMAGE_DES_EDITBOXRIGHTH: C2RustUnnamed_0 = 347;
pub const IMAGE_DES_EDITBOXMIDH: C2RustUnnamed_0 = 346;
pub const IMAGE_DES_EDITBOXRIGHT: C2RustUnnamed_0 = 345;
pub const IMAGE_DES_EDITBOXMID: C2RustUnnamed_0 = 344;
pub const IMAGE_DES_POWERBAR_RIGHT: C2RustUnnamed_0 = 343;
pub const IMAGE_DES_POWERBAR_LEFT: C2RustUnnamed_0 = 342;
pub const IMAGE_AUDIO_LASTSAMPLEH: C2RustUnnamed_0 = 341;
pub const IMAGE_AUDIO_LASTSAMPLE: C2RustUnnamed_0 = 340;
pub const IMAGE_DES_BINH: C2RustUnnamed_0 = 339;
pub const IMAGE_DES_BIN: C2RustUnnamed_0 = 338;
pub const IMAGE_INFINITE_HI: C2RustUnnamed_0 = 337;
pub const IMAGE_INFINITE_DOWN: C2RustUnnamed_0 = 336;
pub const IMAGE_INFINITE_UP: C2RustUnnamed_0 = 335;
pub const IMAGE_RETICULE_GREY: C2RustUnnamed_0 = 334;
pub const IMAGE_ORD_FACHILITE: C2RustUnnamed_0 = 333;
pub const IMAGE_ORD_FAC5DOWN: C2RustUnnamed_0 = 332;
pub const IMAGE_ORD_FAC5UP: C2RustUnnamed_0 = 331;
pub const IMAGE_ORD_FAC4DOWN: C2RustUnnamed_0 = 330;
pub const IMAGE_ORD_FAC4UP: C2RustUnnamed_0 = 329;
pub const IMAGE_ORD_FAC3DOWN: C2RustUnnamed_0 = 328;
pub const IMAGE_ORD_FAC3UP: C2RustUnnamed_0 = 327;
pub const IMAGE_ORD_FAC2DOWN: C2RustUnnamed_0 = 326;
pub const IMAGE_ORD_FAC2UP: C2RustUnnamed_0 = 325;
pub const IMAGE_ORD_FAC1DOWN: C2RustUnnamed_0 = 324;
pub const IMAGE_ORD_FAC1UP: C2RustUnnamed_0 = 323;
pub const IMAGE_TRANSETA_UP: C2RustUnnamed_0 = 322;
pub const IMAGE_LAUNCHDOWN: C2RustUnnamed_0 = 321;
pub const IMAGE_MISSION_CLOCK: C2RustUnnamed_0 = 320;
pub const IMAGE_NRUTER: C2RustUnnamed_0 = 319;
pub const IMAGE_CMDDROID_EXP: C2RustUnnamed_0 = 318;
pub const IMAGE_MULTI_VIS_HI: C2RustUnnamed_0 = 317;
pub const IMAGE_MULTI_POW_HI: C2RustUnnamed_0 = 316;
pub const IMAGE_MULTI_DRO_HI: C2RustUnnamed_0 = 315;
pub const IMAGE_MULTI_TEK_HI: C2RustUnnamed_0 = 314;
pub const IMAGE_MULTI_TEK: C2RustUnnamed_0 = 313;
pub const IMAGE_MULTI_DRO: C2RustUnnamed_0 = 312;
pub const IMAGE_MULTI_POW: C2RustUnnamed_0 = 311;
pub const IMAGE_MULTI_VIS: C2RustUnnamed_0 = 310;
pub const IMAGE_MULTI_AL_HI: C2RustUnnamed_0 = 309;
pub const IMAGE_MULTI_OFFAL_HI: C2RustUnnamed_0 = 308;
pub const IMAGE_MULTI_NOAL_HI: C2RustUnnamed_0 = 307;
pub const IMAGE_MULTI_AL: C2RustUnnamed_0 = 306;
pub const IMAGE_MULTI_OFFAL: C2RustUnnamed_0 = 305;
pub const IMAGE_MULTI_NOAL: C2RustUnnamed_0 = 304;
pub const IMAGE_RAD_ART3: C2RustUnnamed_0 = 303;
pub const IMAGE_RAD_ART2: C2RustUnnamed_0 = 302;
pub const IMAGE_RAD_ART1: C2RustUnnamed_0 = 301;
pub const IMAGE_RAD_ARTREAD: C2RustUnnamed_0 = 300;
pub const IMAGE_RAD_RES3: C2RustUnnamed_0 = 299;
pub const IMAGE_RAD_RES2: C2RustUnnamed_0 = 298;
pub const IMAGE_RAD_RES1: C2RustUnnamed_0 = 297;
pub const IMAGE_RAD_RESREAD: C2RustUnnamed_0 = 296;
pub const IMAGE_RAD_ENM3: C2RustUnnamed_0 = 295;
pub const IMAGE_RAD_ENM2: C2RustUnnamed_0 = 294;
pub const IMAGE_RAD_ENM1: C2RustUnnamed_0 = 293;
pub const IMAGE_RAD_ENMREAD: C2RustUnnamed_0 = 292;
pub const IMAGE_SLIDER_INFINITY: C2RustUnnamed_0 = 291;
pub const IMAGE_SLIDER_BIGBUT: C2RustUnnamed_0 = 290;
pub const IMAGE_ORD_DESTRUCT2GREY: C2RustUnnamed_0 = 289;
pub const IMAGE_ORD_DESTRUCT2DOWN: C2RustUnnamed_0 = 288;
pub const IMAGE_ORD_DESTRUCT2UP: C2RustUnnamed_0 = 287;
pub const IMAGE_ORD_GOTOHQDOWN: C2RustUnnamed_0 = 286;
pub const IMAGE_ORD_GOTOHQUP: C2RustUnnamed_0 = 285;
pub const IMAGE_ORD_HALTDOWN: C2RustUnnamed_0 = 284;
pub const IMAGE_ORD_HALTUP: C2RustUnnamed_0 = 283;
pub const IMAGE_ORD_HOLDFIREDOWN: C2RustUnnamed_0 = 282;
pub const IMAGE_ORD_HOLDFIREUP: C2RustUnnamed_0 = 281;
pub const IMAGE_ORD_RETFIREDOWN: C2RustUnnamed_0 = 280;
pub const IMAGE_ORD_RETFIREUP: C2RustUnnamed_0 = 279;
pub const IMAGE_ORD_FATWILLDOWN: C2RustUnnamed_0 = 278;
pub const IMAGE_ORD_FATWILLUP: C2RustUnnamed_0 = 277;
pub const IMAGE_ORD_REPAIR3DOWN: C2RustUnnamed_0 = 276;
pub const IMAGE_ORD_REPAIR3UP: C2RustUnnamed_0 = 275;
pub const IMAGE_ORD_REPAIR1DOWN: C2RustUnnamed_0 = 274;
pub const IMAGE_ORD_REPAIR1UP: C2RustUnnamed_0 = 273;
pub const IMAGE_ORD_REPAIR2DOWN: C2RustUnnamed_0 = 272;
pub const IMAGE_ORD_REPAIR2UP: C2RustUnnamed_0 = 271;
pub const IMAGE_ORD_RANGE3DOWN: C2RustUnnamed_0 = 270;
pub const IMAGE_ORD_RANGE3UP: C2RustUnnamed_0 = 269;
pub const IMAGE_ORD_RANGE2DOWN: C2RustUnnamed_0 = 268;
pub const IMAGE_ORD_RANGE2UP: C2RustUnnamed_0 = 267;
pub const IMAGE_ORD_RANGE1DOWN: C2RustUnnamed_0 = 266;
pub const IMAGE_ORD_RANGE1UP: C2RustUnnamed_0 = 265;
pub const IMAGE_ORD_DESTRUCT1DOWN: C2RustUnnamed_0 = 264;
pub const IMAGE_ORD_DESTRUCT1UP: C2RustUnnamed_0 = 263;
pub const IMAGE_RES_QUESTIONMARK: C2RustUnnamed_0 = 262;
pub const IMAGE_RES_STRUCTURETECH: C2RustUnnamed_0 = 261;
pub const IMAGE_RES_SYSTEMTECH: C2RustUnnamed_0 = 260;
pub const IMAGE_RES_POWERTECH: C2RustUnnamed_0 = 259;
pub const IMAGE_RES_COMPUTERTECH: C2RustUnnamed_0 = 258;
pub const IMAGE_RES_WEAPONTECH: C2RustUnnamed_0 = 257;
pub const IMAGE_RES_DROIDTECH: C2RustUnnamed_0 = 256;
pub const IMAGE_QUESTION_MARK: C2RustUnnamed_0 = 255;
pub const IMAGE_RES_MAJOR_ROCKET: C2RustUnnamed_0 = 254;
pub const IMAGE_RES_MINOR_AUTOWEAPONS: C2RustUnnamed_0 = 253;
pub const IMAGE_TRACKS: C2RustUnnamed_0 = 252;
pub const IMAGE_STAR: C2RustUnnamed_0 = 251;
pub const IMAGE_SLIDER_BIG: C2RustUnnamed_0 = 250;
pub const IMAGE_DES_STATBACKMID: C2RustUnnamed_0 = 249;
pub const IMAGE_DES_STATBACKRIGHT: C2RustUnnamed_0 = 248;
pub const IMAGE_DES_STATBACKLEFT: C2RustUnnamed_0 = 247;
pub const IMAGE_SIDETABSEL: C2RustUnnamed_0 = 246;
pub const IMAGE_SIDETABDOWN: C2RustUnnamed_0 = 245;
pub const IMAGE_SIDETABHI: C2RustUnnamed_0 = 244;
pub const IMAGE_SIDETAB: C2RustUnnamed_0 = 243;
pub const IMAGE_CURSOR_NOTPOS: C2RustUnnamed_0 = 242;
pub const IMAGE_CURSOR_PICKUP: C2RustUnnamed_0 = 241;
pub const IMAGE_CURSOR_REPAIR: C2RustUnnamed_0 = 240;
pub const IMAGE_CURSOR_FIX: C2RustUnnamed_0 = 239;
pub const IMAGE_CURSOR_ATTACH: C2RustUnnamed_0 = 238;
pub const IMAGE_CURSOR_BRIDGE: C2RustUnnamed_0 = 237;
pub const IMAGE_CURSOR_EMBARK: C2RustUnnamed_0 = 236;
pub const IMAGE_CURSOR_GUARD: C2RustUnnamed_0 = 235;
pub const IMAGE_CURSOR_MOVE: C2RustUnnamed_0 = 234;
pub const IMAGE_CURSOR_BUILD: C2RustUnnamed_0 = 233;
pub const IMAGE_CURSOR_DEFAULT: C2RustUnnamed_0 = 232;
pub const IMAGE_CURSOR_ECM: C2RustUnnamed_0 = 231;
pub const IMAGE_CURSOR_LOCKON: C2RustUnnamed_0 = 230;
pub const IMAGE_CURSOR_SELECT: C2RustUnnamed_0 = 229;
pub const IMAGE_CURSOR_ATTACK: C2RustUnnamed_0 = 228;
pub const IMAGE_ASCII126: C2RustUnnamed_0 = 227;
pub const IMAGE_ASCII125: C2RustUnnamed_0 = 226;
pub const IMAGE_ASCII124: C2RustUnnamed_0 = 225;
pub const IMAGE_ASCII123: C2RustUnnamed_0 = 224;
pub const IMAGE_ASCII122: C2RustUnnamed_0 = 223;
pub const IMAGE_ASCII121: C2RustUnnamed_0 = 222;
pub const IMAGE_ASCII120: C2RustUnnamed_0 = 221;
pub const IMAGE_ASCII119: C2RustUnnamed_0 = 220;
pub const IMAGE_ASCII118: C2RustUnnamed_0 = 219;
pub const IMAGE_ASCII117: C2RustUnnamed_0 = 218;
pub const IMAGE_ASCII116: C2RustUnnamed_0 = 217;
pub const IMAGE_ASCII115: C2RustUnnamed_0 = 216;
pub const IMAGE_ASCII114: C2RustUnnamed_0 = 215;
pub const IMAGE_ASCII113: C2RustUnnamed_0 = 214;
pub const IMAGE_ASCII112: C2RustUnnamed_0 = 213;
pub const IMAGE_ASCII111: C2RustUnnamed_0 = 212;
pub const IMAGE_ASCII110: C2RustUnnamed_0 = 211;
pub const IMAGE_ASCII109: C2RustUnnamed_0 = 210;
pub const IMAGE_ASCII108: C2RustUnnamed_0 = 209;
pub const IMAGE_ASCII107: C2RustUnnamed_0 = 208;
pub const IMAGE_ASCII106: C2RustUnnamed_0 = 207;
pub const IMAGE_ASCII105: C2RustUnnamed_0 = 206;
pub const IMAGE_ASCII104: C2RustUnnamed_0 = 205;
pub const IMAGE_ASCII103: C2RustUnnamed_0 = 204;
pub const IMAGE_ASCII102: C2RustUnnamed_0 = 203;
pub const IMAGE_ASCII101: C2RustUnnamed_0 = 202;
pub const IMAGE_ASCII100: C2RustUnnamed_0 = 201;
pub const IMAGE_ASCII99: C2RustUnnamed_0 = 200;
pub const IMAGE_ASCII98: C2RustUnnamed_0 = 199;
pub const IMAGE_ASCII97: C2RustUnnamed_0 = 198;
pub const IMAGE_ASCII96: C2RustUnnamed_0 = 197;
pub const IMAGE_ASCII95: C2RustUnnamed_0 = 196;
pub const IMAGE_ASCII94: C2RustUnnamed_0 = 195;
pub const IMAGE_ASCII93: C2RustUnnamed_0 = 194;
pub const IMAGE_ASCII92: C2RustUnnamed_0 = 193;
pub const IMAGE_ASCII91: C2RustUnnamed_0 = 192;
pub const IMAGE_ASCII90: C2RustUnnamed_0 = 191;
pub const IMAGE_ASCII89: C2RustUnnamed_0 = 190;
pub const IMAGE_ASCII88: C2RustUnnamed_0 = 189;
pub const IMAGE_ASCII87: C2RustUnnamed_0 = 188;
pub const IMAGE_ASCII86: C2RustUnnamed_0 = 187;
pub const IMAGE_ASCII85: C2RustUnnamed_0 = 186;
pub const IMAGE_ASCII84: C2RustUnnamed_0 = 185;
pub const IMAGE_ASCII83: C2RustUnnamed_0 = 184;
pub const IMAGE_ASCII82: C2RustUnnamed_0 = 183;
pub const IMAGE_ASCII81: C2RustUnnamed_0 = 182;
pub const IMAGE_ASCII80: C2RustUnnamed_0 = 181;
pub const IMAGE_ASCII79: C2RustUnnamed_0 = 180;
pub const IMAGE_ASCII78: C2RustUnnamed_0 = 179;
pub const IMAGE_ASCII77: C2RustUnnamed_0 = 178;
pub const IMAGE_ASCII76: C2RustUnnamed_0 = 177;
pub const IMAGE_ASCII75: C2RustUnnamed_0 = 176;
pub const IMAGE_ASCII74: C2RustUnnamed_0 = 175;
pub const IMAGE_ASCII73: C2RustUnnamed_0 = 174;
pub const IMAGE_ASCII72: C2RustUnnamed_0 = 173;
pub const IMAGE_ASCII71: C2RustUnnamed_0 = 172;
pub const IMAGE_ASCII70: C2RustUnnamed_0 = 171;
pub const IMAGE_ASCII69: C2RustUnnamed_0 = 170;
pub const IMAGE_ASCII68: C2RustUnnamed_0 = 169;
pub const IMAGE_ASCII67: C2RustUnnamed_0 = 168;
pub const IMAGE_ASCII66: C2RustUnnamed_0 = 167;
pub const IMAGE_ASCII65: C2RustUnnamed_0 = 166;
pub const IMAGE_ASTERISK: C2RustUnnamed_0 = 165;
pub const IMAGE_ASCII63: C2RustUnnamed_0 = 164;
pub const IMAGE_ASCII62: C2RustUnnamed_0 = 163;
pub const IMAGE_ASCII61: C2RustUnnamed_0 = 162;
pub const IMAGE_ASCII60: C2RustUnnamed_0 = 161;
pub const IMAGE_ASCII59: C2RustUnnamed_0 = 160;
pub const IMAGE_ASCII58: C2RustUnnamed_0 = 159;
pub const IMAGE_ASCII57: C2RustUnnamed_0 = 158;
pub const IMAGE_ASCII56: C2RustUnnamed_0 = 157;
pub const IMAGE_ASCII55: C2RustUnnamed_0 = 156;
pub const IMAGE_ASCII54: C2RustUnnamed_0 = 155;
pub const IMAGE_ASCII53: C2RustUnnamed_0 = 154;
pub const IMAGE_ASCII52: C2RustUnnamed_0 = 153;
pub const IMAGE_ASCII51: C2RustUnnamed_0 = 152;
pub const IMAGE_ASCII50: C2RustUnnamed_0 = 151;
pub const IMAGE_ASCII49: C2RustUnnamed_0 = 150;
pub const IMAGE_ASCII48: C2RustUnnamed_0 = 149;
pub const IMAGE_ASCII47: C2RustUnnamed_0 = 148;
pub const IMAGE_ASCII46: C2RustUnnamed_0 = 147;
pub const IMAGE_ASCII45: C2RustUnnamed_0 = 146;
pub const IMAGE_ASCII44: C2RustUnnamed_0 = 145;
pub const IMAGE_ASCII43: C2RustUnnamed_0 = 144;
pub const IMAGE_ASCII42: C2RustUnnamed_0 = 143;
pub const IMAGE_ASCII41: C2RustUnnamed_0 = 142;
pub const IMAGE_ASCII40: C2RustUnnamed_0 = 141;
pub const IMAGE_ASCII39: C2RustUnnamed_0 = 140;
pub const IMAGE_ASCII38: C2RustUnnamed_0 = 139;
pub const IMAGE_ASCII37: C2RustUnnamed_0 = 138;
pub const IMAGE_ASCII36: C2RustUnnamed_0 = 137;
pub const IMAGE_ASCII35: C2RustUnnamed_0 = 136;
pub const IMAGE_ASCII34: C2RustUnnamed_0 = 135;
pub const IMAGE_ASCII33: C2RustUnnamed_0 = 134;
pub const IMAGE_INTEL_CAMPAIGNDOWN: C2RustUnnamed_0 = 133;
pub const IMAGE_INTEL_CAMPAIGN: C2RustUnnamed_0 = 132;
pub const IMAGE_INTEL_MISSIONDOWN: C2RustUnnamed_0 = 131;
pub const IMAGE_INTEL_MISSION: C2RustUnnamed_0 = 130;
pub const IMAGE_INTEL_RESEARCHDOWN: C2RustUnnamed_0 = 129;
pub const IMAGE_INTEL_RESEARCH: C2RustUnnamed_0 = 128;
pub const IMAGE_DES_BARYELLOW: C2RustUnnamed_0 = 127;
pub const IMAGE_DES_BARRED: C2RustUnnamed_0 = 126;
pub const IMAGE_DES_BARBLUE: C2RustUnnamed_0 = 125;
pub const IMAGE_DES_BARBACK: C2RustUnnamed_0 = 124;
pub const IMAGE_9: C2RustUnnamed_0 = 123;
pub const IMAGE_8: C2RustUnnamed_0 = 122;
pub const IMAGE_7: C2RustUnnamed_0 = 121;
pub const IMAGE_6: C2RustUnnamed_0 = 120;
pub const IMAGE_5: C2RustUnnamed_0 = 119;
pub const IMAGE_4: C2RustUnnamed_0 = 118;
pub const IMAGE_3: C2RustUnnamed_0 = 117;
pub const IMAGE_2: C2RustUnnamed_0 = 116;
pub const IMAGE_1: C2RustUnnamed_0 = 115;
pub const IMAGE_0: C2RustUnnamed_0 = 114;
pub const IMAGE_RES_MAJOR_HOVER: C2RustUnnamed_0 = 113;
pub const IMAGE_RES_MAJOR_HEAVYWEP: C2RustUnnamed_0 = 112;
pub const IMAGE_RES_MAJOR_ELECTRONIC: C2RustUnnamed_0 = 111;
pub const IMAGE_RES_MAJOR_PLASCRETE: C2RustUnnamed_0 = 110;
pub const IMAGE_RES_MINOR_RADAR: C2RustUnnamed_0 = 109;
pub const IMAGE_PLASCRETE: C2RustUnnamed_0 = 108;
pub const IMAGE_ECM: C2RustUnnamed_0 = 107;
pub const IMAGE_RES_MINOR_PLASSTEEL: C2RustUnnamed_0 = 106;
pub const IMAGE_HOVERCRAFT: C2RustUnnamed_0 = 105;
pub const IMAGE_CANNON: C2RustUnnamed_0 = 104;
pub const IMAGE_ROCKET: C2RustUnnamed_0 = 103;
pub const IMAGE_DES_BODYPOINTS: C2RustUnnamed_0 = 102;
pub const IMAGE_DES_FIRERATE: C2RustUnnamed_0 = 101;
pub const IMAGE_DES_HOVER: C2RustUnnamed_0 = 100;
pub const IMAGE_DES_POWER: C2RustUnnamed_0 = 99;
pub const IMAGE_DES_DAMAGE: C2RustUnnamed_0 = 98;
pub const IMAGE_DES_WEIGHT: C2RustUnnamed_0 = 97;
pub const IMAGE_DES_ROAD: C2RustUnnamed_0 = 96;
pub const IMAGE_DES_CROSSCOUNTRY: C2RustUnnamed_0 = 95;
pub const IMAGE_DES_ARMOUR_EXPLOSIVE: C2RustUnnamed_0 = 94;
pub const IMAGE_DES_RANGE: C2RustUnnamed_0 = 93;
pub const IMAGE_DES_BUILDRATE: C2RustUnnamed_0 = 92;
pub const IMAGE_DES_TABWEAPONDOWN: C2RustUnnamed_0 = 91;
pub const IMAGE_DES_TABWEAPON: C2RustUnnamed_0 = 90;
pub const IMAGE_DES_EXTRAHI: C2RustUnnamed_0 = 89;
pub const IMAGE_DES_STATSCOMP: C2RustUnnamed_0 = 88;
pub const IMAGE_DES_BACK: C2RustUnnamed_0 = 87;
pub const IMAGE_FRAME_HC3: C2RustUnnamed_0 = 86;
pub const IMAGE_FRAME_HC2: C2RustUnnamed_0 = 85;
pub const IMAGE_FRAME_HC1: C2RustUnnamed_0 = 84;
pub const IMAGE_FRAME_HC0: C2RustUnnamed_0 = 83;
pub const IMAGE_FRAME_VC3: C2RustUnnamed_0 = 82;
pub const IMAGE_FRAME_VC2: C2RustUnnamed_0 = 81;
pub const IMAGE_FRAME_VC1: C2RustUnnamed_0 = 80;
pub const IMAGE_FRAME_VC0: C2RustUnnamed_0 = 79;
pub const IMAGE_DES_EDITBOXLEFTH: C2RustUnnamed_0 = 78;
pub const IMAGE_DES_EDITBOXLEFT: C2RustUnnamed_0 = 77;
pub const IMAGE_DES_POWERBACK: C2RustUnnamed_0 = 76;
pub const IMAGE_DES_STATSCURR: C2RustUnnamed_0 = 75;
pub const IMAGE_DES_STATSBACK: C2RustUnnamed_0 = 74;
pub const IMAGE_DES_POWERCURR: C2RustUnnamed_0 = 73;
pub const IMAGE_FRAME_HBH: C2RustUnnamed_0 = 72;
pub const IMAGE_FRAME_HTH: C2RustUnnamed_0 = 71;
pub const IMAGE_FRAME_HB2: C2RustUnnamed_0 = 70;
pub const IMAGE_FRAME_HT2: C2RustUnnamed_0 = 69;
pub const IMAGE_DES_COMMANDDOWN: C2RustUnnamed_0 = 68;
pub const IMAGE_DES_COMMAND: C2RustUnnamed_0 = 67;
pub const IMAGE_DES_SYSTEMSDOWN: C2RustUnnamed_0 = 66;
pub const IMAGE_DES_SYSTEMS: C2RustUnnamed_0 = 65;
pub const IMAGE_DES_WEAPONSDOWN: C2RustUnnamed_0 = 64;
pub const IMAGE_DES_WEAPONS: C2RustUnnamed_0 = 63;
pub const IMAGE_FRAME_VRH: C2RustUnnamed_0 = 62;
pub const IMAGE_FRAME_VLH: C2RustUnnamed_0 = 61;
pub const IMAGE_FRAME_VR2: C2RustUnnamed_0 = 60;
pub const IMAGE_FRAME_VL2: C2RustUnnamed_0 = 59;
pub const IMAGE_DES_HILIGHT: C2RustUnnamed_0 = 58;
pub const IMAGE_DES_PROPULSIONH: C2RustUnnamed_0 = 57;
pub const IMAGE_DES_PROPULSION: C2RustUnnamed_0 = 56;
pub const IMAGE_DES_BODYH: C2RustUnnamed_0 = 55;
pub const IMAGE_DES_BODY: C2RustUnnamed_0 = 54;
pub const IMAGE_DES_TURRETH: C2RustUnnamed_0 = 53;
pub const IMAGE_DES_TURRET: C2RustUnnamed_0 = 52;
pub const IMAGE_FRAME_VR: C2RustUnnamed_0 = 51;
pub const IMAGE_FRAME_VL: C2RustUnnamed_0 = 50;
pub const IMAGE_SLIDER_BACK: C2RustUnnamed_0 = 49;
pub const IMAGE_FRAME_HB: C2RustUnnamed_0 = 48;
pub const IMAGE_FRAME_HT: C2RustUnnamed_0 = 47;
pub const IMAGE_SLIDER_BUT: C2RustUnnamed_0 = 46;
pub const IMAGE_CLOSEHILIGHT: C2RustUnnamed_0 = 45;
pub const IMAGE_CLOSEDOWN: C2RustUnnamed_0 = 44;
pub const IMAGE_CLOSE: C2RustUnnamed_0 = 43;
pub const IMAGE_CANCEL_HILIGHT: C2RustUnnamed_0 = 42;
pub const IMAGE_CANCEL_DOWN: C2RustUnnamed_0 = 41;
pub const IMAGE_CANCEL_UP: C2RustUnnamed_0 = 40;
pub const IMAGE_RETICULE_BUTDOWN: C2RustUnnamed_0 = 39;
pub const IMAGE_RETICULE_HILIGHT: C2RustUnnamed_0 = 38;
pub const IMAGE_MANUFACTURE_DOWN: C2RustUnnamed_0 = 37;
pub const IMAGE_MANUFACTURE_UP: C2RustUnnamed_0 = 36;
pub const IMAGE_RESEARCH_DOWN: C2RustUnnamed_0 = 35;
pub const IMAGE_RESEARCH_UP: C2RustUnnamed_0 = 34;
pub const IMAGE_BUILD_DOWN: C2RustUnnamed_0 = 33;
pub const IMAGE_BUILD_UP: C2RustUnnamed_0 = 32;
pub const IMAGE_DESIGN_DOWN: C2RustUnnamed_0 = 31;
pub const IMAGE_DESIGN_UP: C2RustUnnamed_0 = 30;
pub const IMAGE_COMMANDDROID_DOWN: C2RustUnnamed_0 = 29;
pub const IMAGE_COMMANDDROID_UP: C2RustUnnamed_0 = 28;
pub const IMAGE_INTELMAP_DOWN: C2RustUnnamed_0 = 27;
pub const IMAGE_INTELMAP_UP: C2RustUnnamed_0 = 26;
pub const IMAGE_PBAR_BOTTOM: C2RustUnnamed_0 = 25;
pub const IMAGE_PBAR_TOP: C2RustUnnamed_0 = 24;
pub const IMAGE_FRAME_C3: C2RustUnnamed_0 = 23;
pub const IMAGE_FRAME_C2: C2RustUnnamed_0 = 22;
pub const IMAGE_FRAME_C1: C2RustUnnamed_0 = 21;
pub const IMAGE_FRAME_C0: C2RustUnnamed_0 = 20;
pub const IMAGE_TABHILIGHT: C2RustUnnamed_0 = 19;
pub const IMAGE_TABSELECTED: C2RustUnnamed_0 = 18;
pub const IMAGE_TAB4DOWN: C2RustUnnamed_0 = 17;
pub const IMAGE_TAB3DOWN: C2RustUnnamed_0 = 16;
pub const IMAGE_TAB2DOWN: C2RustUnnamed_0 = 15;
pub const IMAGE_TAB1DOWN: C2RustUnnamed_0 = 14;
pub const IMAGE_TAB4: C2RustUnnamed_0 = 13;
pub const IMAGE_TAB3: C2RustUnnamed_0 = 12;
pub const IMAGE_TAB2: C2RustUnnamed_0 = 11;
pub const IMAGE_TAB1: C2RustUnnamed_0 = 10;
pub const IMAGE_BUTB_HILITE: C2RustUnnamed_0 = 9;
pub const IMAGE_BUT_HILITE: C2RustUnnamed_0 = 8;
pub const IMAGE_BUTB0_DOWN: C2RustUnnamed_0 = 7;
pub const IMAGE_BUTB0_UP: C2RustUnnamed_0 = 6;
pub const IMAGE_BUT0_DOWN: C2RustUnnamed_0 = 5;
pub const IMAGE_BUT0_UP: C2RustUnnamed_0 = 4;
pub const IMAGE_PBAR_REQUIRED: C2RustUnnamed_0 = 3;
pub const IMAGE_PBAR_USED: C2RustUnnamed_0 = 2;
pub const IMAGE_PBAR_AVAIL: C2RustUnnamed_0 = 1;
pub const IMAGE_PBAR_EMPTY: C2RustUnnamed_0 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_bargraph {
    pub formID: UDWORD,
    pub id: UDWORD,
    pub type_0: WIDGET_TYPE,
    pub style: UDWORD,
    pub x: SWORD,
    pub y: SWORD,
    pub width: UWORD,
    pub height: UWORD,
    pub display: WIDGET_DISPLAY,
    pub callback: WIDGET_CALLBACK,
    pub pUserData: *mut libc::c_void,
    pub UserData: UDWORD,
    pub psNext: *mut _widget,
    pub barPos: UWORD,
    pub majorSize: UWORD,
    pub minorSize: UWORD,
    pub iRange: UWORD,
    pub iValue: UWORD,
    pub majorCol: UBYTE,
    pub minorCol: UBYTE,
    pub pTip: *mut STRING,
}
pub type W_BARGRAPH = _w_bargraph;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_minortab {
    pub psWidgets: *mut WIDGET,
    pub pTip: *mut STRING,
}
pub type W_MINORTAB = _w_minortab;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_majortab {
    pub lastMinor: UWORD,
    pub numMinor: UWORD,
    pub asMinor: [W_MINORTAB; 5],
    pub pTip: *mut STRING,
}
pub type W_MAJORTAB = _w_majortab;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_tabform {
    pub formID: UDWORD,
    pub id: UDWORD,
    pub type_0: WIDGET_TYPE,
    pub style: UDWORD,
    pub x: SWORD,
    pub y: SWORD,
    pub width: UWORD,
    pub height: UWORD,
    pub display: WIDGET_DISPLAY,
    pub callback: WIDGET_CALLBACK,
    pub pUserData: *mut libc::c_void,
    pub UserData: UDWORD,
    pub psNext: *mut _widget,
    pub disableChildren: BOOL,
    pub Ax0: UWORD,
    pub Ay0: UWORD,
    pub Ax1: UWORD,
    pub Ay1: UWORD,
    pub animCount: UDWORD,
    pub startTime: UDWORD,
    pub aColours: [UDWORD; 8],
    pub psLastHiLite: *mut WIDGET,
    pub psWidgets: *mut WIDGET,
    pub majorPos: UWORD,
    pub minorPos: UWORD,
    pub majorSize: UWORD,
    pub minorSize: UWORD,
    pub tabMajorThickness: UWORD,
    pub tabMinorThickness: UWORD,
    pub tabMajorGap: UWORD,
    pub tabMinorGap: UWORD,
    pub tabVertOffset: SWORD,
    pub tabHorzOffset: SWORD,
    pub majorOffset: SWORD,
    pub minorOffset: SWORD,
    pub majorT: UWORD,
    pub minorT: UWORD,
    pub state: UWORD,
    pub tabHiLite: UWORD,
    pub numMajor: UWORD,
    pub asMajor: [W_MAJORTAB; 9],
    pub pTabDisplay: TAB_DISPLAY,
    pub pFormDisplay: WIDGET_DISPLAY,
}
pub type W_TABFORM = _w_tabform;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_clickform {
    pub formID: UDWORD,
    pub id: UDWORD,
    pub type_0: WIDGET_TYPE,
    pub style: UDWORD,
    pub x: SWORD,
    pub y: SWORD,
    pub width: UWORD,
    pub height: UWORD,
    pub display: WIDGET_DISPLAY,
    pub callback: WIDGET_CALLBACK,
    pub pUserData: *mut libc::c_void,
    pub UserData: UDWORD,
    pub psNext: *mut _widget,
    pub disableChildren: BOOL,
    pub Ax0: UWORD,
    pub Ay0: UWORD,
    pub Ax1: UWORD,
    pub Ay1: UWORD,
    pub animCount: UDWORD,
    pub startTime: UDWORD,
    pub aColours: [UDWORD; 8],
    pub psLastHiLite: *mut WIDGET,
    pub psWidgets: *mut WIDGET,
    pub state: UDWORD,
    pub pTip: *mut STRING,
    pub HilightAudioID: SWORD,
    pub ClickedAudioID: SWORD,
    pub AudioCallback: WIDGET_AUDIOCALLBACK,
}
pub type W_CLICKFORM = _w_clickform;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_label {
    pub formID: UDWORD,
    pub id: UDWORD,
    pub type_0: WIDGET_TYPE,
    pub style: UDWORD,
    pub x: SWORD,
    pub y: SWORD,
    pub width: UWORD,
    pub height: UWORD,
    pub display: WIDGET_DISPLAY,
    pub callback: WIDGET_CALLBACK,
    pub pUserData: *mut libc::c_void,
    pub UserData: UDWORD,
    pub psNext: *mut _widget,
    pub state: UDWORD,
    pub aText: [STRING; 80],
    pub FontID: libc::c_int,
    pub pTip: *mut STRING,
}
pub type W_LABEL = _w_label;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_button {
    pub formID: UDWORD,
    pub id: UDWORD,
    pub type_0: WIDGET_TYPE,
    pub style: UDWORD,
    pub x: SWORD,
    pub y: SWORD,
    pub width: UWORD,
    pub height: UWORD,
    pub display: WIDGET_DISPLAY,
    pub callback: WIDGET_CALLBACK,
    pub pUserData: *mut libc::c_void,
    pub UserData: UDWORD,
    pub psNext: *mut _widget,
    pub state: UDWORD,
    pub pText: *mut STRING,
    pub pTip: *mut STRING,
    pub HilightAudioID: SWORD,
    pub ClickedAudioID: SWORD,
    pub AudioCallback: WIDGET_AUDIOCALLBACK,
    pub FontID: libc::c_int,
}
pub type W_BUTTON = _w_button;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_editbox {
    pub formID: UDWORD,
    pub id: UDWORD,
    pub type_0: WIDGET_TYPE,
    pub style: UDWORD,
    pub x: SWORD,
    pub y: SWORD,
    pub width: UWORD,
    pub height: UWORD,
    pub display: WIDGET_DISPLAY,
    pub callback: WIDGET_CALLBACK,
    pub pUserData: *mut libc::c_void,
    pub UserData: UDWORD,
    pub psNext: *mut _widget,
    pub state: UDWORD,
    pub aText: [STRING; 80],
    pub FontID: libc::c_int,
    pub insPos: UWORD,
    pub printStart: UWORD,
    pub printChars: UWORD,
    pub printWidth: UWORD,
    pub pBoxDisplay: WIDGET_DISPLAY,
    pub pFontDisplay: FONT_DISPLAY,
    pub HilightAudioID: SWORD,
    pub ClickedAudioID: SWORD,
    pub AudioCallback: WIDGET_AUDIOCALLBACK,
}
// The tool tip for the graph
//
// disable button from selection
pub type W_EDITBOX = _w_editbox;
/* The common widget data */
// The current edit box state
// The text in the edit box
//	PROP_FONT	*psFont;					// The font for the edit box
// The insertion point in the buffer
// Where in the string appears at the far left of the box
// The number of characters appearing in the box
// The pixel width of the characters in the box
// Optional callback to display the edit box background.
// Optional callback to display a string.
// Audio ID for form clicked sound
// Audio ID for form hilighted sound
// Pointer to audio callback function
/* Slider state */
// Slider is being dragged
// Slider is hilited
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_slider {
    pub formID: UDWORD,
    pub id: UDWORD,
    pub type_0: WIDGET_TYPE,
    pub style: UDWORD,
    pub x: SWORD,
    pub y: SWORD,
    pub width: UWORD,
    pub height: UWORD,
    pub display: WIDGET_DISPLAY,
    pub callback: WIDGET_CALLBACK,
    pub pUserData: *mut libc::c_void,
    pub UserData: UDWORD,
    pub psNext: *mut _widget,
    pub orientation: UWORD,
    pub numStops: UWORD,
    pub barSize: UWORD,
    pub pos: UWORD,
    pub state: UWORD,
    pub pTip: *mut STRING,
}
pub type W_SLIDER = _w_slider;
// Tool tip
/* Power levels are divided by this for power bar display. The extra factor has
been included so that the levels appear the same for the power bar as for the
power values in the buttons */
// Speed to rotate objects rendered in
// buttons ( degrees per second )
//the two types of button used in the object display (bottom bar)
pub type C2RustUnnamed_1 = libc::c_uint;
pub const IMDTYPE_STRUCTURESTAT: C2RustUnnamed_1 = 6;
pub const IMDTYPE_RESEARCH: C2RustUnnamed_1 = 5;
pub const IMDTYPE_STRUCTURE: C2RustUnnamed_1 = 4;
pub const IMDTYPE_COMPONENT: C2RustUnnamed_1 = 3;
pub const IMDTYPE_DROIDTEMPLATE: C2RustUnnamed_1 = 2;
pub const IMDTYPE_DROID: C2RustUnnamed_1 = 1;
pub const IMDTYPE_NONE: C2RustUnnamed_1 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BUTTON_SURFACE {
    pub Buffer: *mut uint8,
    pub Surface: *mut iSurface,
}
// I tried to get the PC code working with the above PSX structure but it was having none of it
//  ... sorry about that ... TC
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RENDERED_BUTTON {
    pub InUse: BOOL,
    pub Initialised: BOOL,
    pub ImdRotation: SDWORD,
    pub State: UDWORD,
    pub Data: *mut libc::c_void,
    pub Data2: *mut libc::c_void,
    pub ButSurf: *mut BUTTON_SURFACE,
}
pub type DROID_GROUP = _droid_group;
// list of droids in the group
// the command droid of a command group
// where the group should retreat to
// Other interface strings
pub const STR_INT_BLDPROGRESS: _fixed_str_id = 43;
pub const STR_INT_POWERACCRUED: _fixed_str_id = 54;
/*
 * Text.h (was Strings.h)
 *
 * String management functions
 *
 */
//the two defines below are MUTUALLY EXCLUSIVE! don't have both defined...
//#define RESOURCE_NAMES
/* ID numbers for all the fixed strings */
pub type _fixed_str_id = libc::c_uint;
pub const STR_MAX_ID: _fixed_str_id = 442;
pub const STR_SEQ_MINIMAL: _fixed_str_id = 441;
pub const STR_SEQ_WINDOW: _fixed_str_id = 440;
pub const STR_SEQ_FULL: _fixed_str_id = 439;
pub const STR_SEQ_PLAYBACK: _fixed_str_id = 438;
pub const STR_GAM_FORMATION_OFF: _fixed_str_id = 437;
pub const STR_GAM_FORMATION_ON: _fixed_str_id = 436;
pub const STR_GAM_BUILD_NO_REOPEN: _fixed_str_id = 435;
pub const STR_GAM_BUILD_REOPEN: _fixed_str_id = 434;
pub const STR_BIND_REOPEN_BUILD: _fixed_str_id = 433;
pub const STR_FE_SUBTITLES: _fixed_str_id = 432;
pub const STR_DORD_VTOL_FACTORY: _fixed_str_id = 431;
pub const STR_DORD_FIREDES: _fixed_str_id = 430;
pub const STR_BIND_RADJUMP: _fixed_str_id = 429;
pub const STR_BIND_RELOAD: _fixed_str_id = 428;
pub const STR_GAM_NORMAL_SPEED: _fixed_str_id = 427;
pub const STR_GAM_SLOW_DOWN: _fixed_str_id = 426;
pub const STR_GAM_SPEED_UP: _fixed_str_id = 425;
pub const STR_BIND_NORMAL_SPEED: _fixed_str_id = 424;
pub const STR_BIND_SLOW_DOWN: _fixed_str_id = 423;
pub const STR_BIND_SPEED_UP: _fixed_str_id = 422;
pub const STR_BIND_LEFT: _fixed_str_id = 421;
pub const STR_BIND_RIGHT: _fixed_str_id = 420;
pub const STR_BIND_DOWN: _fixed_str_id = 419;
pub const STR_BIND_UP: _fixed_str_id = 418;
pub const STR_BIND_CONSOLE: _fixed_str_id = 417;
pub const STR_BIND_SELCYBORG: _fixed_str_id = 416;
pub const STR_BIND_SELPOWER: _fixed_str_id = 415;
pub const STR_BIND_SELRESEARCH: _fixed_str_id = 414;
pub const STR_BIND_SELFACTORY: _fixed_str_id = 413;
pub const STR_BIND_CMD9: _fixed_str_id = 412;
pub const STR_BIND_CMD8: _fixed_str_id = 411;
pub const STR_BIND_CMD7: _fixed_str_id = 410;
pub const STR_BIND_CMD6: _fixed_str_id = 409;
pub const STR_BIND_CMD5: _fixed_str_id = 408;
pub const STR_BIND_CMD4: _fixed_str_id = 407;
pub const STR_BIND_CMD3: _fixed_str_id = 406;
pub const STR_BIND_CMD2: _fixed_str_id = 405;
pub const STR_BIND_CMD1: _fixed_str_id = 404;
pub const STR_GAM_COMNOTFOUND: _fixed_str_id = 403;
pub const STR_GAM_SENNOTFOUND: _fixed_str_id = 402;
pub const STR_GAM_CONNOTFOUND: _fixed_str_id = 401;
pub const STR_BIND_COMJ: _fixed_str_id = 400;
pub const STR_BIND_SENJ: _fixed_str_id = 399;
// number of string ID's
//added by alex 26-01-99
pub const STR_BIND_CONJ: _fixed_str_id = 398;
pub const STR_FE_CYAN: _fixed_str_id = 397;
pub const STR_FE_PINK: _fixed_str_id = 396;
pub const STR_FE_BLUE: _fixed_str_id = 395;
pub const STR_FE_RED: _fixed_str_id = 394;
pub const STR_FE_BLACK: _fixed_str_id = 393;
pub const STR_FE_GREY: _fixed_str_id = 392;
pub const STR_FE_ORANGE: _fixed_str_id = 391;
pub const STR_FE_GREEN: _fixed_str_id = 390;
pub const STR_FE_OFF: _fixed_str_id = 389;
pub const STR_FE_ON: _fixed_str_id = 388;
pub const STR_FE_MFLIP: _fixed_str_id = 387;
pub const STR_FE_SSHAKE: _fixed_str_id = 386;
pub const STR_SEL_NO_STRUCTURE: _fixed_str_id = 385;
// something else.
pub const STR_GAM_DERRICK_BURNING: _fixed_str_id = 384;
pub const STR_BIND_ASIMIL: _fixed_str_id = 383;
pub const STR_BIND_AWHE: _fixed_str_id = 382;
pub const STR_BIND_AVTOL: _fixed_str_id = 381;
pub const STR_BIND_ALL: _fixed_str_id = 380;
pub const STR_BIND_ATR: _fixed_str_id = 379;
pub const STR_BIND_ASCR: _fixed_str_id = 378;
pub const STR_BIND_RECY: _fixed_str_id = 377;
pub const STR_BIND_AHOV: _fixed_str_id = 376;
pub const STR_BIND_AHTR: _fixed_str_id = 375;
pub const STR_BIND_ABDU: _fixed_str_id = 374;
pub const STR_BIND_ACU: _fixed_str_id = 373;
pub const STR_BIND_NDAM: _fixed_str_id = 372;
pub const STR_BIND_HDAM: _fixed_str_id = 371;
pub const STR_BIND_LDAM: _fixed_str_id = 370;
pub const STR_BIND_SCAT: _fixed_str_id = 369;
pub const STR_BIND_LONGR: _fixed_str_id = 368;
pub const STR_BIND_SENDT: _fixed_str_id = 367;
pub const STR_BIND_DSTOP: _fixed_str_id = 366;
pub const STR_BIND_REPA: _fixed_str_id = 365;
pub const STR_BIND_PATR: _fixed_str_id = 364;
pub const STR_BIND_PURS: _fixed_str_id = 363;
pub const STR_BIND_SHOR: _fixed_str_id = 362;
pub const STR_BIND_SPLIM: _fixed_str_id = 361;
pub const STR_BIND_DEFR: _fixed_str_id = 360;
pub const STR_BIND_RTB: _fixed_str_id = 359;
pub const STR_BIND_FAW: _fixed_str_id = 358;
pub const STR_BIND_ENGAG: _fixed_str_id = 357;
pub const STR_BIND_UNITJ: _fixed_str_id = 356;
pub const STR_BIND_CEASE: _fixed_str_id = 355;
pub const STR_BIND_CENTV: _fixed_str_id = 354;
pub const STR_BIND_OVERL: _fixed_str_id = 353;
pub const STR_BIND_REPJ: _fixed_str_id = 352;
pub const STR_BIND_RESJ: _fixed_str_id = 351;
pub const STR_BIND_ORD: _fixed_str_id = 350;
pub const STR_BIND_PB: _fixed_str_id = 349;
pub const STR_BIND_RR: _fixed_str_id = 348;
pub const STR_BIND_RP: _fixed_str_id = 347;
pub const STR_BIND_RL: _fixed_str_id = 346;
pub const STR_BIND_PF: _fixed_str_id = 345;
pub const STR_BIND_ZOUT: _fixed_str_id = 344;
pub const STR_BIND_ZIN: _fixed_str_id = 343;
pub const STR_BIND_ROUT: _fixed_str_id = 342;
pub const STR_BIND_RIN: _fixed_str_id = 341;
pub const STR_BIND_OPT: _fixed_str_id = 340;
pub const STR_BIND_TRACK: _fixed_str_id = 339;
pub const STR_BIND_NORTH: _fixed_str_id = 338;
pub const STR_BIND_AUDOFF: _fixed_str_id = 337;
pub const STR_BIND_AUDON: _fixed_str_id = 336;
pub const STR_BIND_MULOP: _fixed_str_id = 335;
pub const STR_BIND_GR9: _fixed_str_id = 334;
pub const STR_BIND_GR8: _fixed_str_id = 333;
pub const STR_BIND_GR7: _fixed_str_id = 332;
pub const STR_BIND_GR6: _fixed_str_id = 331;
pub const STR_BIND_GR5: _fixed_str_id = 330;
pub const STR_BIND_GR4: _fixed_str_id = 329;
pub const STR_BIND_GR3: _fixed_str_id = 328;
pub const STR_BIND_GR2: _fixed_str_id = 327;
pub const STR_BIND_GR1: _fixed_str_id = 326;
pub const STR_BIND_AS9: _fixed_str_id = 325;
pub const STR_BIND_AS8: _fixed_str_id = 324;
pub const STR_BIND_AS7: _fixed_str_id = 323;
pub const STR_BIND_AS6: _fixed_str_id = 322;
pub const STR_BIND_AS5: _fixed_str_id = 321;
pub const STR_BIND_AS4: _fixed_str_id = 320;
pub const STR_BIND_AS3: _fixed_str_id = 319;
pub const STR_BIND_AS2: _fixed_str_id = 318;
pub const STR_BIND_AS1: _fixed_str_id = 317;
pub const STR_BIND_PREV: _fixed_str_id = 316;
pub const STR_BIND_PAUSE: _fixed_str_id = 315;
pub const STR_BIND_SHOT: _fixed_str_id = 314;
pub const STR_BIND_BARS: _fixed_str_id = 313;
pub const STR_BIND_TOGCON: _fixed_str_id = 312;
pub const STR_BIND_TOGRAD: _fixed_str_id = 311;
pub const STR_BIND_CHOCOM: _fixed_str_id = 310;
pub const STR_BIND_CHOINT: _fixed_str_id = 309;
pub const STR_BIND_CHODES: _fixed_str_id = 308;
pub const STR_BIND_CHOBUI: _fixed_str_id = 307;
pub const STR_BIND_CHORES: _fixed_str_id = 306;
pub const STR_BIND_CHOMAN: _fixed_str_id = 305;
pub const STR_KM_KEYMAP_SIDE: _fixed_str_id = 304;
// keymap editor.
pub const STR_KM_KEYMAP: _fixed_str_id = 303;
pub const STR_GAM_MAXUNITSREACHED: _fixed_str_id = 302;
pub const STR_GAME_RESTART: _fixed_str_id = 301;
pub const STR_DORD_CYBORG_FACTORY: _fixed_str_id = 300;
pub const STR_GAME_LOADED: _fixed_str_id = 299;
pub const STR_GAME_SAVED: _fixed_str_id = 298;
pub const STR_GAME_REPLAY: _fixed_str_id = 297;
pub const STR_CD_CHANGE_1OR2: _fixed_str_id = 296;
pub const STR_CD_CHANGE: _fixed_str_id = 295;
pub const STR_DL_LEVEL_ACE: _fixed_str_id = 294;
pub const STR_DL_LEVEL_SPECIAL: _fixed_str_id = 293;
pub const STR_DL_LEVEL_ELITE: _fixed_str_id = 292;
pub const STR_DL_LEVEL_CRACK: _fixed_str_id = 291;
pub const STR_DL_LEVEL_VETERAN: _fixed_str_id = 290;
pub const STR_DL_LEVEL_PROF: _fixed_str_id = 289;
pub const STR_DL_LEVEL_REGULAR: _fixed_str_id = 288;
pub const STR_DL_LEVEL_TRAINED: _fixed_str_id = 287;
pub const STR_DL_LEVEL_GREEN: _fixed_str_id = 286;
pub const STR_DL_LEVEL_ROOKIE: _fixed_str_id = 285;
pub const STR_MR_LEVEL_ACE: _fixed_str_id = 284;
pub const STR_MR_LEVEL_SPECIAL: _fixed_str_id = 283;
pub const STR_MR_LEVEL_ELITE: _fixed_str_id = 282;
pub const STR_MR_LEVEL_CRACK: _fixed_str_id = 281;
pub const STR_MR_LEVEL_VETERAN: _fixed_str_id = 280;
pub const STR_MR_LEVEL_PROF: _fixed_str_id = 279;
pub const STR_MR_LEVEL_REGULAR: _fixed_str_id = 278;
pub const STR_MR_LEVEL_TRAINED: _fixed_str_id = 277;
pub const STR_MR_LEVEL_GREEN: _fixed_str_id = 276;
pub const STR_MR_LEVEL_ROOKIE: _fixed_str_id = 275;
pub const STR_MR_BABAS_RUN_OVER: _fixed_str_id = 274;
pub const STR_MR_SHOTS_OFF_TARGET: _fixed_str_id = 273;
pub const STR_MR_SHOTS_ON_TARGET: _fixed_str_id = 272;
pub const STR_MR_GAME_TIME: _fixed_str_id = 271;
pub const STR_MR_MISSION_TIME: _fixed_str_id = 270;
pub const STR_MR_ARTEFACTS_FOUND: _fixed_str_id = 269;
pub const STR_MR_STR_NOW: _fixed_str_id = 268;
pub const STR_MR_STR_LOST: _fixed_str_id = 267;
pub const STR_MR_STR_BLOWN_UP: _fixed_str_id = 266;
pub const STR_MR_STR_BUILT: _fixed_str_id = 265;
pub const STR_MR_AV_UNIT_EL: _fixed_str_id = 264;
pub const STR_MR_UNITS_NOW: _fixed_str_id = 263;
pub const STR_MR_UNITS_LOST: _fixed_str_id = 262;
pub const STR_MR_UNITS_KILLED: _fixed_str_id = 261;
pub const STR_MR_UNITS_BUILT: _fixed_str_id = 260;
pub const STR_MR_RANKINGS: _fixed_str_id = 259;
pub const STR_MR_FORCE_INFO: _fixed_str_id = 258;
pub const STR_MR_STRUCTURE_LOSSES: _fixed_str_id = 257;
pub const STR_MR_UNIT_LOSSES: _fixed_str_id = 256;
pub const STR_MR_CONTINUE: _fixed_str_id = 255;
pub const STR_MR_QUIT_TO_MAIN: _fixed_str_id = 254;
pub const STR_MR_LOAD_GAME: _fixed_str_id = 253;
pub const STR_MR_SAVE_GAME: _fixed_str_id = 252;
pub const STR_MR_OBJECTIVE_FAILED: _fixed_str_id = 251;
pub const STR_MR_OBJECTIVE_ACHIEVED: _fixed_str_id = 250;
pub const STR_GP_ALLIGN: _fixed_str_id = 249;
pub const STR_GP_CENTERED: _fixed_str_id = 248;
pub const STR_GP_ASSIGNED: _fixed_str_id = 247;
pub const STR_GP_SELECTED: _fixed_str_id = 246;
pub const STR_GAM_REPNOTFOUND: _fixed_str_id = 245;
pub const STR_GAM_RESNOTFOUND: _fixed_str_id = 244;
pub const STR_GAM_REWREPN: _fixed_str_id = 243;
pub const STR_GAM_REWREPA: _fixed_str_id = 242;
pub const STR_GAM_REWNOWT: _fixed_str_id = 241;
pub const STR_GAM_REWWEAP: _fixed_str_id = 240;
pub const STR_GAM_REWBODY: _fixed_str_id = 239;
pub const STR_GAM_REWPROP: _fixed_str_id = 238;
pub const STR_GAM_REWELEC: _fixed_str_id = 237;
pub const STR_GAM_JOINING: _fixed_str_id = 236;
pub const STR_GAM_GAMEOVER: _fixed_str_id = 235;
pub const STR_GAM_REINF: _fixed_str_id = 234;
pub const STR_GAM_UNITSEL: _fixed_str_id = 233;
pub const STR_GAM_RESREWARD: _fixed_str_id = 232;
pub const STR_GAM_NOHQ: _fixed_str_id = 231;
pub const STR_GAM_GOHQ: _fixed_str_id = 230;
pub const STR_GAM_STRREST: _fixed_str_id = 229;
pub const STR_GAM_ELECDAM: _fixed_str_id = 228;
pub const STR_GAM_STRDAM: _fixed_str_id = 227;
pub const STR_GAM_POWCON: _fixed_str_id = 226;
pub const STR_GAM_RESREM: _fixed_str_id = 225;
pub const STR_GAM_ENERGY: _fixed_str_id = 224;
pub const STR_GAM_NORTH: _fixed_str_id = 223;
pub const STR_GAM_UNITLOST: _fixed_str_id = 222;
pub const STR_GAM_DROIDSTATE: _fixed_str_id = 221;
pub const STR_GAM_DERRICK: _fixed_str_id = 220;
pub const STR_DORD_FACTORY: _fixed_str_id = 219;
pub const STR_DORD_RECYCLE: _fixed_str_id = 218;
pub const STR_DORD_ARMRECYCLE: _fixed_str_id = 217;
pub const STR_DORD_EMBARK: _fixed_str_id = 216;
pub const STR_DORD_RETBASE: _fixed_str_id = 215;
pub const STR_DORD_RETREPAIR: _fixed_str_id = 214;
pub const STR_DORD_HOLDPOS: _fixed_str_id = 213;
pub const STR_DORD_GUARD: _fixed_str_id = 212;
pub const STR_DORD_PERSUE: _fixed_str_id = 211;
pub const STR_DORD_PATROL: _fixed_str_id = 210;
pub const STR_DORD_FIRE3: _fixed_str_id = 209;
pub const STR_DORD_FIRE2: _fixed_str_id = 208;
pub const STR_DORD_FIRE1: _fixed_str_id = 207;
pub const STR_DORD_REPAIR3: _fixed_str_id = 206;
pub const STR_DORD_REPAIR2: _fixed_str_id = 205;
pub const STR_DORD_REPAIR1: _fixed_str_id = 204;
pub const STR_DORD_RANGE3: _fixed_str_id = 203;
pub const STR_DORD_RANGE2: _fixed_str_id = 202;
pub const STR_DORD_RANGE1: _fixed_str_id = 201;
pub const STR_MUL_RESPOND: _fixed_str_id = 200;
pub const STR_MUL_JOINING: _fixed_str_id = 199;
pub const STR_MUL_LEAVE: _fixed_str_id = 198;
pub const STR_HARD: _fixed_str_id = 197;
pub const STR_NORMAL: _fixed_str_id = 196;
pub const STR_EASY: _fixed_str_id = 195;
pub const STR_FE_DIFFICULTY: _fixed_str_id = 194;
pub const STR_FE_TRANSPARENCY: _fixed_str_id = 193;
pub const STR_FE_FOG: _fixed_str_id = 192;
pub const STR_FE_EFFECTS: _fixed_str_id = 191;
pub const STR_FE_TEXTURE: _fixed_str_id = 190;
pub const STR_FE_AUDIO: _fixed_str_id = 189;
pub const STR_FE_GRAPHICS: _fixed_str_id = 188;
pub const STR_FE_GAME: _fixed_str_id = 187;
pub const STR_FE_GLIDE: _fixed_str_id = 186;
pub const STR_FE_OPENGL: _fixed_str_id = 185;
pub const STR_FE_DIRECTX: _fixed_str_id = 184;
pub const STR_FE_SOFTWARE: _fixed_str_id = 183;
pub const STR_FE_VIDEO: _fixed_str_id = 182;
pub const STR_FE_GOODFOG: _fixed_str_id = 181;
pub const STR_FE_CRAPFOG: _fixed_str_id = 180;
pub const STR_FE_CLAN: _fixed_str_id = 179;
pub const STR_FE_MUSIC: _fixed_str_id = 178;
pub const STR_FE_3D_FX: _fixed_str_id = 177;
pub const STR_FE_FX: _fixed_str_id = 176;
pub const STR_FE_GAMMA: _fixed_str_id = 175;
pub const STR_FE_SCROLL: _fixed_str_id = 174;
pub const STR_FE_MOUSE: _fixed_str_id = 173;
pub const STR_FE_SIDEOPTIONS: _fixed_str_id = 172;
pub const STR_FE_SKIRMISH: _fixed_str_id = 171;
pub const STR_FE_FORCEEDIT: _fixed_str_id = 170;
pub const STR_FE_JOIN: _fixed_str_id = 169;
pub const STR_FE_HOST: _fixed_str_id = 168;
pub const STR_FE_SIDEMULTI: _fixed_str_id = 167;
pub const STR_FE_RETURN: _fixed_str_id = 166;
pub const STR_FE_FASTPLAY: _fixed_str_id = 165;
pub const STR_FE_TUT: _fixed_str_id = 164;
pub const STR_FE_LOAD: _fixed_str_id = 163;
pub const STR_FE_NEW: _fixed_str_id = 162;
pub const STR_FE_SIDETUT: _fixed_str_id = 161;
pub const STR_FE_SIDESINGLE2: _fixed_str_id = 160;
pub const STR_FE_SIDESINGLE1: _fixed_str_id = 159;
pub const STR_FE_QUIT: _fixed_str_id = 158;
pub const STR_FE_INTRO: _fixed_str_id = 157;
pub const STR_FE_OPTIONS: _fixed_str_id = 156;
pub const STR_FE_MULTI: _fixed_str_id = 155;
pub const STR_FE_SINGLE: _fixed_str_id = 154;
// FrontEnd STrings
pub const STR_FE_SIDEMAIN: _fixed_str_id = 153;
pub const STR_GAME_RESUME: _fixed_str_id = 152;
//ingame ops.
pub const STR_GAME_QUIT: _fixed_str_id = 151;
pub const STR_GAME_NAME: _fixed_str_id = 150;
pub const STR_PLAYER_NAME: _fixed_str_id = 149;
pub const STR_COMPATIBLE: _fixed_str_id = 148;
pub const STR_MUL_ARTIF: _fixed_str_id = 147;
pub const STR_GIFT_POW: _fixed_str_id = 146;
pub const STR_GIFT_TEC: _fixed_str_id = 145;
pub const STR_GIFT_DRO: _fixed_str_id = 144;
pub const STR_GIFT_VIS: _fixed_str_id = 143;
pub const STR_ALLI_FRM: _fixed_str_id = 142;
pub const STR_ALLI_BRK: _fixed_str_id = 141;
pub const STR_ALLI_OFF: _fixed_str_id = 140;
pub const STR_ALLI_REQ: _fixed_str_id = 139;
pub const STR_ALLI_POW: _fixed_str_id = 138;
pub const STR_ALLI_DRO: _fixed_str_id = 137;
pub const STR_ALLI_TEC: _fixed_str_id = 136;
pub const STR_ALLI_VIS: _fixed_str_id = 135;
pub const STR_ALLI_STATE: _fixed_str_id = 134;
pub const STR_MUL_FOG_OFF: _fixed_str_id = 133;
pub const STR_MUL_FOG_ON: _fixed_str_id = 132;
pub const STR_MUL_TEAMPLAY: _fixed_str_id = 131;
pub const STR_MUL_SKIRMISH: _fixed_str_id = 130;
pub const STR_MUL_STRLIM: _fixed_str_id = 129;
pub const STR_MUL_COMP_N: _fixed_str_id = 128;
pub const STR_MUL_COMP_Y: _fixed_str_id = 127;
pub const STR_MUL_COMP: _fixed_str_id = 126;
pub const STR_MUL_PLAY: _fixed_str_id = 125;
pub const STR_MUL_PLAYERS: _fixed_str_id = 124;
pub const STR_LABEL_FOG: _fixed_str_id = 123;
pub const STR_LABEL_LIMIT: _fixed_str_id = 122;
pub const STR_LABEL_BASE: _fixed_str_id = 121;
pub const STR_LABEL_TEC: _fixed_str_id = 120;
pub const STR_LABEL_ALLI: _fixed_str_id = 119;
pub const STR_LABEL_TYPE: _fixed_str_id = 118;
pub const STR_GAMES_GAMES: _fixed_str_id = 117;
pub const STR_CON_MORE: _fixed_str_id = 116;
pub const STR_CON_CABLE: _fixed_str_id = 115;
pub const STR_CON_LAN: _fixed_str_id = 114;
pub const STR_CON_INTERNET: _fixed_str_id = 113;
pub const STR_CON_MODEM: _fixed_str_id = 112;
pub const STR_MUL_FRAGLIM: _fixed_str_id = 111;
pub const STR_MUL_TIMELIM: _fixed_str_id = 110;
pub const STR_MUL_NOLIM: _fixed_str_id = 109;
pub const STR_MUL_ALLIANCEY: _fixed_str_id = 108;
pub const STR_MUL_ALLIANCEN: _fixed_str_id = 107;
pub const STR_MUL_GAMEIC: _fixed_str_id = 106;
pub const STR_MUL_MAPIC: _fixed_str_id = 105;
pub const STR_MUL_FORCEIC: _fixed_str_id = 104;
pub const STR_MUL_PLAYERIC: _fixed_str_id = 103;
pub const STR_MUL_CAMPBASE: _fixed_str_id = 102;
pub const STR_MUL_CAMPDEFENCE: _fixed_str_id = 101;
pub const STR_MUL_CAMPCLEAN: _fixed_str_id = 100;
pub const STR_MUL_TECH3: _fixed_str_id = 99;
pub const STR_MUL_TECH2: _fixed_str_id = 98;
pub const STR_MUL_TECH1: _fixed_str_id = 97;
pub const STR_MUL_POWHI: _fixed_str_id = 96;
pub const STR_MUL_POWMED: _fixed_str_id = 95;
pub const STR_MUL_POWLO: _fixed_str_id = 94;
pub const STR_MUL_PING: _fixed_str_id = 93;
pub const STR_MUL_KILLS: _fixed_str_id = 92;
pub const STR_MUL_SCORE: _fixed_str_id = 91;
pub const STR_MUL_ALLIANCES: _fixed_str_id = 90;
pub const STR_MUL_STARTING: _fixed_str_id = 89;
pub const STR_MUL_CHAT: _fixed_str_id = 88;
pub const STR_MUL_SIDEINFO: _fixed_str_id = 87;
pub const STR_MUL_SIDETEMPLATES: _fixed_str_id = 86;
pub const STR_MUL_SIDEFORCE: _fixed_str_id = 85;
pub const STR_MUL_SIDEPLAYERS: _fixed_str_id = 84;
pub const STR_MUL_SIDEGAMES: _fixed_str_id = 83;
pub const STR_MUL_SIDEOPTIONS: _fixed_str_id = 82;
pub const STR_MUL_SIDECONNECTION: _fixed_str_id = 81;
pub const STR_MUL_115200: _fixed_str_id = 80;
pub const STR_MUL_56000: _fixed_str_id = 79;
pub const STR_MUL_19200: _fixed_str_id = 78;
pub const STR_MUL_14400: _fixed_str_id = 77;
pub const STR_MUL_SEARCHING: _fixed_str_id = 76;
pub const STR_MUL_DESIGN: _fixed_str_id = 75;
pub const STR_MUL_SAVE: _fixed_str_id = 74;
pub const STR_MUL_LOAD: _fixed_str_id = 73;
pub const STR_MUL_DEFAULT: _fixed_str_id = 72;
pub const STR_MUL_CLEAR: _fixed_str_id = 71;
pub const STR_MUL_HOST: _fixed_str_id = 70;
//	STR_MUL_RESLO,	
//	STR_MUL_RESMED,		
//	STR_MUL_RESHI,		
//	STR_MUL_HELPON,	
//	STR_MUL_HELPOFF,		
pub const STR_MUL_REFRESH: _fixed_str_id = 69;
pub const STR_MUL_CAMPAIGN: _fixed_str_id = 68;
pub const STR_MUL_ARENA: _fixed_str_id = 67;
pub const STR_MUL_MAXPLAY: _fixed_str_id = 66;
pub const STR_MUL_GAME: _fixed_str_id = 65;
pub const STR_MUL_PLAYER: _fixed_str_id = 64;
pub const STR_MUL_OK: _fixed_str_id = 63;
pub const STR_MUL_CANCEL: _fixed_str_id = 62;
pub const STR_MUL_COM4: _fixed_str_id = 61;
pub const STR_MUL_COM3: _fixed_str_id = 60;
pub const STR_MUL_COM2: _fixed_str_id = 59;
pub const STR_MUL_COM1: _fixed_str_id = 58;
pub const STR_MUL_IPADDR: _fixed_str_id = 57;
// multiplayer strings
pub const STR_MUL_PHONENO: _fixed_str_id = 56;
pub const STR_INT_POWER: _fixed_str_id = 55;
pub const STR_INT_LOOP: _fixed_str_id = 53;
pub const STR_INT_DPOINT: _fixed_str_id = 52;
pub const STR_INT_TRANSLAUNCH: _fixed_str_id = 51;
pub const STR_INT_TRANSPORTER: _fixed_str_id = 50;
pub const STR_INT_RESCOMPLETED: _fixed_str_id = 49;
pub const STR_INT_MISMESSAGE: _fixed_str_id = 48;
pub const STR_INT_GENMESSAGE: _fixed_str_id = 47;
pub const STR_INT_RESMESSAGE: _fixed_str_id = 46;
pub const STR_INT_PWRUSAGE: _fixed_str_id = 45;
pub const STR_INT_BLDSPEED: _fixed_str_id = 44;
pub const STR_DES_TEMPBODY: _fixed_str_id = 42;
pub const STR_DES_TEMPPOWER: _fixed_str_id = 41;
pub const STR_DES_TURRET: _fixed_str_id = 40;
pub const STR_DES_PROPULSION: _fixed_str_id = 39;
pub const STR_DES_BODY: _fixed_str_id = 38;
pub const STR_DES_COMMAND: _fixed_str_id = 37;
pub const STR_DES_OTHER: _fixed_str_id = 36;
pub const STR_DES_WEAPONS: _fixed_str_id = 35;
pub const STR_DES_WATER: _fixed_str_id = 34;
pub const STR_DES_OFFROAD: _fixed_str_id = 33;
pub const STR_DES_ROAD: _fixed_str_id = 32;
pub const STR_DES_AIR: _fixed_str_id = 31;
pub const STR_DES_ROF: _fixed_str_id = 30;
pub const STR_DES_DAMAGE: _fixed_str_id = 29;
pub const STR_DES_RANGE: _fixed_str_id = 28;
pub const STR_DES_BUILD_POINTS: _fixed_str_id = 27;
pub const STR_DES_ECM_POWER: _fixed_str_id = 26;
pub const STR_DES_SENSOR_POWER: _fixed_str_id = 25;
pub const STR_DES_SENSOR_RANGE: _fixed_str_id = 24;
pub const STR_DES_POWERUSE: _fixed_str_id = 23;
pub const STR_DES_WEIGHT: _fixed_str_id = 22;
pub const STR_DES_POWER: _fixed_str_id = 21;
pub const STR_DES_ARMOUR_HEAT: _fixed_str_id = 20;
pub const STR_DES_ARMOUR_KIN: _fixed_str_id = 19;
pub const STR_DES_NEW: _fixed_str_id = 18;
pub const STR_DES_DEL: _fixed_str_id = 17;
pub const STR_DES_CANCEL: _fixed_str_id = 16;
pub const STR_DES_STORE: _fixed_str_id = 15;
// Design screen strings
pub const STR_DES_NEWVEH: _fixed_str_id = 14;
pub const STR_MISC_PAUSED: _fixed_str_id = 13;
pub const STR_MISC_QUIT: _fixed_str_id = 12;
pub const STR_MISC_SAVEGAME: _fixed_str_id = 11;
pub const STR_MISC_LOADGAME: _fixed_str_id = 10;
// Miscellaneous strings
pub const STR_MISC_CLOSE: _fixed_str_id = 9;
pub const STR_RET_COMMAND: _fixed_str_id = 8;
pub const STR_RET_CLOSE: _fixed_str_id = 7;
pub const STR_RET_BUILD: _fixed_str_id = 6;
pub const STR_RET_RESEARCH: _fixed_str_id = 5;
pub const STR_RET_DESIGN: _fixed_str_id = 4;
pub const STR_RET_MANUFACTURE: _fixed_str_id = 3;
pub const STR_RET_INTELLIGENCE: _fixed_str_id = 2;
// Reticule strings
pub const STR_RET_OPTIONS: _fixed_str_id = 1;
// The default (id == 0) string
pub const STR_DEFAULT: _fixed_str_id = 0;
static mut FormOpenAudioID: libc::c_int = 0;
// ID of sfx to play when form opens.
static mut FormCloseAudioID: libc::c_int = 0;
// ID of sfx to play when form closes.
static mut FormOpenCount: libc::c_int = 0;
// Count used to ensure only one sfx played when two forms opening.
static mut FormCloseCount: libc::c_int = 0;
// Count used to ensure only one sfx played when two forms closeing.
#[no_mangle]
pub static mut CurrentStatsTemplate: *mut BASE_STATS =
    0 as *const BASE_STATS as *mut BASE_STATS;
// Token look up table for matching IMD's to droid components.
//
/*TOKENID CompIMDIDs[]={
//COMP_BODY:
	{"Viper Body",IMD_BD_VIPER},
	{"Cobra Body",IMD_BD_COBRA},

//COMP_WEAPON:
	{"Single Rocket",IMD_TR_SINGROCK},
	{"Rocket Pod",IMD_TR_ROCKPOD},
	{"Light Machine Gun",IMD_TR_LGUN},
	{"Light Cannon",IMD_TR_LCAN},
	{"Heavy Cannon",IMD_TR_HCAN},

//COMP_PROPULSION:
	{"Wheeled Propulsion",IMD_PR_WHEELS},
	{"Tracked Propulsion",IMD_PR_TRACKS},
	{"Hover Propulsion",IMD_PR_HOVER},

//COMP_CONSTRUCT:
	{"Building Constructor",IMD_TR_BUILDER},

//COMP_REPAIRUNIT:
	{"Light Repair #1",IMD_TR_LGUN},

//COMP_ECM:
	{"Light ECM #1",IMD_TR_ECM},
	{"Heavy ECM #1",IMD_TR_ECM},

//COMP_SENSOR:
	{"EM Sensor",IMD_TR_SENS},
	{"Default Sensor",IMD_TR_SENS},

	{NULL,-1},
};*/
// Token look up table for matching Images and IMD's to research projects.
//
/*RESEARCHICON ResearchIMAGEIDs[]={
	{"Tracks",			IMAGE_RES_MINOR_TRACKS,		IMD_PR_TRACKS},
	{"Hovercraft",		IMAGE_RES_MINOR_HOVER,		IMD_PR_HOVER},
	{"Light Cannon",	IMAGE_RES_MINOR_HEAVYWEP,	IMD_TR_LCAN},
	{"Heavy Cannon",	IMAGE_RES_MINOR_HEAVYWEP,	IMD_TR_HCAN},
	{"Rocket Launcher",	IMAGE_RES_MINOR_ROCKET,		IMD_TR_SINGROCK},
	{"ECM PickUp",		IMAGE_RES_MINOR_ELECTRONIC,	IMD_TR_ECM},
	{"PlasCrete",		IMAGE_RES_MINOR_PLASCRETE,	IMD_PLASCRETE},
	{"EM Sensor",		IMAGE_RES_MINOR_ELECTRONIC,	IMD_TR_SENS},
	{" Rocket Pod",		IMAGE_RES_MINOR_ROCKET,		IMD_TR_ROCKPOD},

	{NULL,-1},
};*/
#[no_mangle]
pub static mut ManuPower: UDWORD = 0 as libc::c_int as UDWORD;
// Power required to manufacture the current item.
// Display surfaces for rendered buttons.
#[no_mangle]
pub static mut TopicSurfaces: [BUTTON_SURFACE; 5] =
    [BUTTON_SURFACE{Buffer: 0 as *const uint8 as *mut uint8,
                    Surface: 0 as *const iSurface as *mut iSurface,}; 5];
#[no_mangle]
pub static mut ObjectSurfaces: [BUTTON_SURFACE; 10] =
    [BUTTON_SURFACE{Buffer: 0 as *const uint8 as *mut uint8,
                    Surface: 0 as *const iSurface as *mut iSurface,}; 10];
#[no_mangle]
pub static mut StatSurfaces: [BUTTON_SURFACE; 20] =
    [BUTTON_SURFACE{Buffer: 0 as *const uint8 as *mut uint8,
                    Surface: 0 as *const iSurface as *mut iSurface,}; 20];
#[no_mangle]
pub static mut System0Surfaces: [BUTTON_SURFACE; 10] =
    [BUTTON_SURFACE{Buffer: 0 as *const uint8 as *mut uint8,
                    Surface: 0 as *const iSurface as *mut iSurface,}; 10];
// Working buffers for rendered buttons.
#[no_mangle]
pub static mut System0Buffers: [RENDERED_BUTTON; 80] =
    [RENDERED_BUTTON{InUse: 0,
                     Initialised: 0,
                     ImdRotation: 0,
                     State: 0,
                     Data: 0 as *const libc::c_void as *mut libc::c_void,
                     Data2: 0 as *const libc::c_void as *mut libc::c_void,
                     ButSurf:
                         0 as *const BUTTON_SURFACE as *mut BUTTON_SURFACE,};
        80];
// References ObjectSurfaces.
//RENDERED_BUTTON System1Buffers[NUM_OBJECTBUFFERS];	// References ObjectSurfaces.
//RENDERED_BUTTON System2Buffers[NUM_OBJECTBUFFERS];	// References ObjectSurfaces.
#[no_mangle]
pub static mut ObjectBuffers: [RENDERED_BUTTON; 40] =
    [RENDERED_BUTTON{InUse: 0,
                     Initialised: 0,
                     ImdRotation: 0,
                     State: 0,
                     Data: 0 as *const libc::c_void as *mut libc::c_void,
                     Data2: 0 as *const libc::c_void as *mut libc::c_void,
                     ButSurf:
                         0 as *const BUTTON_SURFACE as *mut BUTTON_SURFACE,};
        40];
// References ObjectSurfaces.
#[no_mangle]
pub static mut TopicBuffers: [RENDERED_BUTTON; 20] =
    [RENDERED_BUTTON{InUse: 0,
                     Initialised: 0,
                     ImdRotation: 0,
                     State: 0,
                     Data: 0 as *const libc::c_void as *mut libc::c_void,
                     Data2: 0 as *const libc::c_void as *mut libc::c_void,
                     ButSurf:
                         0 as *const BUTTON_SURFACE as *mut BUTTON_SURFACE,};
        20];
// References TopicSurfaces.
#[no_mangle]
pub static mut StatBuffers: [RENDERED_BUTTON; 80] =
    [RENDERED_BUTTON{InUse: 0,
                     Initialised: 0,
                     ImdRotation: 0,
                     State: 0,
                     Data: 0 as *const libc::c_void as *mut libc::c_void,
                     Data2: 0 as *const libc::c_void as *mut libc::c_void,
                     ButSurf:
                         0 as *const BUTTON_SURFACE as *mut BUTTON_SURFACE,};
        80];
static mut ButtonDrawXOffset: SDWORD = 0;
static mut ButtonDrawYOffset: SDWORD = 0;
//static SDWORD ActualQuantity = -1;
// Set audio IDs for form opening/closing anims.
// Use -1 to dissable audio.
//
#[no_mangle]
pub unsafe extern "C" fn SetFormAudioIDs(mut OpenID: libc::c_int,
                                         mut CloseID: libc::c_int) {
    FormOpenAudioID = OpenID;
    FormCloseAudioID = CloseID;
    FormOpenCount = 0 as libc::c_int;
    FormCloseCount = 0 as libc::c_int;
}
// Widget callback to update the progress bar in the object stats screen.
//
#[no_mangle]
pub unsafe extern "C" fn intUpdateProgressBar(mut psWidget: *mut _widget,
                                              mut psContext:
                                                  *mut _w_context) {
    let mut psObj: *mut BASE_OBJECT =
        0 as *mut BASE_OBJECT; // Get the object associated with this widget.
    let mut Droid: *mut DROID = 0 as *mut DROID;
    let mut Structure: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut Manufacture: *mut FACTORY = 0 as *mut FACTORY;
    let mut Research: *mut RESEARCH_FACILITY = 0 as *mut RESEARCH_FACILITY;
    let mut pPlayerRes: *mut PLAYER_RESEARCH = 0 as *mut PLAYER_RESEARCH;
    let mut BuildPoints: UDWORD = 0;
    let mut Range: UDWORD = 0;
    let mut BuildPower: UDWORD = 0;
    let mut BarGraph: *mut W_BARGRAPH = psWidget as *mut W_BARGRAPH;
    psObj = (*BarGraph).pUserData as *mut BASE_OBJECT;
    if psObj.is_null() {
        (*BarGraph).style |= 0x8000 as libc::c_int as libc::c_uint;
        return
    }
    //	ASSERT( !psObj->died,"intUpdateProgressBar: object is dead" );
    if (*psObj).died != 0 && (*psObj).died != 1 as libc::c_int as libc::c_uint
       {
        return
    }
    match (*psObj).type_0 as libc::c_uint {
        0 => {
            // If it's a droid and...
            Droid = psObj as *mut DROID;
            if DroidIsBuilding(Droid) != 0 {
                // Is it building.
                if (*Droid).asBits[COMP_CONSTRUCT as libc::c_int as
                                       usize].nStat as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"intUpdateProgressBar: invalid droid type\x00" as
                              *const u8 as
                              *const libc::c_char); // Get the structure it's building.
                };
                if (*Droid).asBits[COMP_CONSTRUCT as libc::c_int as
                                       usize].nStat as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"intdisplay.c\x00" as *const u8 as
                              *const libc::c_char, 231 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 21],
                                                    &[libc::c_char; 21]>(b"intUpdateProgressBar\x00")).as_ptr(),
                          b"Droid->asBits[COMP_CONSTRUCT].nStat\x00" as
                              *const u8 as *const libc::c_char);
                };
                Structure = DroidGetBuildStructure(Droid);
                //				ASSERT( Structure != NULL,"intUpdateProgressBar : NULL Structure pointer." );
                if !Structure.is_null() {
                    //check if have all the power to build yet
                    BuildPower = structPowerToBuild(Structure);
                    //if (Structure->currentPowerAccrued < (SWORD)Structure->pStructureType->powerToBuild)
                    if ((*Structure).currentPowerAccrued as libc::c_int) <
                           BuildPower as SWORD as libc::c_int {
                        //if not started building show how much power accrued
					    //Range = Structure->pStructureType->powerToBuild;
                        Range = BuildPower;
                        BuildPoints =
                            (*Structure).currentPowerAccrued as UDWORD;
                        //set the colour of the bar to green
                        (*BarGraph).majorCol =
                            *colours.as_mut_ptr().offset(10 as libc::c_int as
                                                             isize);
                        //and change the tool tip
                        (*BarGraph).pTip =
                            strresGetString(psStringRes,
                                            STR_INT_POWERACCRUED as
                                                libc::c_int as UDWORD)
                    } else {
                        //show progress of build
                        Range =
                            (*(*Structure).pStructureType).buildPoints; // And how long it takes to build.
                        BuildPoints =
                            (*Structure).currentBuildPts as
                                UDWORD; // How near to completion.
                        //set the colour of the bar to yellow
                        (*BarGraph).majorCol =
                            *colours.as_mut_ptr().offset(14 as libc::c_int as
                                                             isize);
                        //and change the tool tip
                        (*BarGraph).pTip =
                            strresGetString(psStringRes,
                                            STR_INT_BLDPROGRESS as libc::c_int
                                                as UDWORD)
                    }
                    if BuildPoints > Range { BuildPoints = Range }
                    (*BarGraph).majorSize =
                        BuildPoints.wrapping_mul(100 as libc::c_int as
                                                     libc::c_uint).wrapping_div(Range)
                            as UWORD;
                    (*BarGraph).style &=
                        !(0x8000 as libc::c_int) as libc::c_uint
                } else {
                    (*BarGraph).majorSize = 0 as libc::c_int as UWORD;
                    (*BarGraph).style |= 0x8000 as libc::c_int as libc::c_uint
                }
            } else {
                (*BarGraph).majorSize = 0 as libc::c_int as UWORD;
                (*BarGraph).style |= 0x8000 as libc::c_int as libc::c_uint
            }
        }
        1 => {
            // If it's a structure and...
            Structure = psObj as *mut STRUCTURE;
            if StructureIsManufacturing(Structure) != 0 {
                // Is it manufacturing.
                Manufacture = StructureGetFactory(Structure);
                //check started to build
                if (*Manufacture).timeStarted ==
                       0 as libc::c_int as libc::c_uint {
                    //BuildPoints = 0;
					//if not started building show how much power accrued
                    Range =
                        (*((*Manufacture).psSubject as
                               *mut DROID_TEMPLATE)).powerPoints;
                    BuildPoints = (*Manufacture).powerAccrued;
                    //set the colour of the bar to green
                    (*BarGraph).majorCol =
                        *colours.as_mut_ptr().offset(10 as libc::c_int as
                                                         isize);
                    //and change the tool tip
                    (*BarGraph).pTip =
                        strresGetString(psStringRes,
                                        STR_INT_POWERACCRUED as libc::c_int as
                                            UDWORD)
                } else {
                    Range = (*Manufacture).timeToBuild;
                    //set the colour of the bar to yellow
                    (*BarGraph).majorCol =
                        *colours.as_mut_ptr().offset(14 as libc::c_int as
                                                         isize);
                    //and change the tool tip
                    (*BarGraph).pTip =
                        strresGetString(psStringRes,
                                        STR_INT_BLDPROGRESS as libc::c_int as
                                            UDWORD);
                    //if on hold need to take it into account
                    if (*Manufacture).timeStartHold != 0 {
                        BuildPoints =
                            gameTime.wrapping_sub((*Manufacture).timeStarted.wrapping_add(gameTime.wrapping_sub((*Manufacture).timeStartHold))).wrapping_div(1000
                                                                                                                                                                 as
                                                                                                                                                                 libc::c_int
                                                                                                                                                                 as
                                                                                                                                                                 libc::c_uint)
                    } else {
                        BuildPoints =
                            gameTime.wrapping_sub((*Manufacture).timeStarted).wrapping_div(1000
                                                                                               as
                                                                                               libc::c_int
                                                                                               as
                                                                                               libc::c_uint)
                    }
                }
                if BuildPoints > Range { BuildPoints = Range }
                (*BarGraph).majorSize =
                    BuildPoints.wrapping_mul(100 as libc::c_int as
                                                 libc::c_uint).wrapping_div(Range)
                        as UWORD;
                (*BarGraph).style &= !(0x8000 as libc::c_int) as libc::c_uint
            } else if StructureIsResearching(Structure) != 0 {
                // Is it researching.
                Research = StructureGetResearch(Structure);
                pPlayerRes =
                    asPlayerResList[selectedPlayer as
                                        usize].offset(((*Research).psSubject
                                                           as
                                                           *mut RESEARCH).wrapping_offset_from(asResearch)
                                                          as libc::c_int as
                                                          isize);
                //this is no good if you change which lab is researching the topic and one lab is faster
				//Range = Research->timeToResearch;
                Range =
                    (*((*((*Structure).pFunctionality as
                              *mut RESEARCH_FACILITY)).psSubject as
                           *mut RESEARCH)).researchPoints as UDWORD;
                //check started to research
                if (*Research).timeStarted == 0 as libc::c_int as libc::c_uint
                   {
                    //BuildPoints = 0;
					//if not started building show how much power accrued
                    Range =
                        (*((*Research).psSubject as
                               *mut RESEARCH)).researchPower;
                    BuildPoints = (*Research).powerAccrued;
                    //set the colour of the bar to green
                    (*BarGraph).majorCol =
                        *colours.as_mut_ptr().offset(10 as libc::c_int as
                                                         isize);
                    //and change the tool tip
                    (*BarGraph).pTip =
                        strresGetString(psStringRes,
                                        STR_INT_POWERACCRUED as libc::c_int as
                                            UDWORD)
                } else {
                    //set the colour of the bar to yellow
                    (*BarGraph).majorCol =
                        *colours.as_mut_ptr().offset(14 as libc::c_int as
                                                         isize);
                    //and change the tool tip
                    (*BarGraph).pTip =
                        strresGetString(psStringRes,
                                        STR_INT_BLDPROGRESS as libc::c_int as
                                            UDWORD);
                    //if on hold need to take it into account
                    if (*Research).timeStartHold != 0 {
                        BuildPoints =
                            (*((*Structure).pFunctionality as
                                   *mut RESEARCH_FACILITY)).researchPoints.wrapping_mul(gameTime.wrapping_sub((*Research).timeStarted.wrapping_add(gameTime.wrapping_sub((*Research).timeStartHold)))).wrapping_div(1000
                                                                                                                                                                                                                        as
                                                                                                                                                                                                                        libc::c_int
                                                                                                                                                                                                                        as
                                                                                                                                                                                                                        libc::c_uint); // Get the object associated with this widget.
                        BuildPoints =
                            (BuildPoints as
                                 libc::c_uint).wrapping_add((*pPlayerRes).currentPoints)
                                as UDWORD as UDWORD
                    } else {
                        BuildPoints =
                            (*((*Structure).pFunctionality as
                                   *mut RESEARCH_FACILITY)).researchPoints.wrapping_mul(gameTime.wrapping_sub((*Research).timeStarted)).wrapping_div(1000
                                                                                                                                                         as
                                                                                                                                                         libc::c_int
                                                                                                                                                         as
                                                                                                                                                         libc::c_uint);
                        BuildPoints =
                            (BuildPoints as
                                 libc::c_uint).wrapping_add((*pPlayerRes).currentPoints)
                                as UDWORD as UDWORD
                    }
                }
                if BuildPoints > Range { BuildPoints = Range }
                (*BarGraph).majorSize =
                    BuildPoints.wrapping_mul(100 as libc::c_int as
                                                 libc::c_uint).wrapping_div(Range)
                        as UWORD;
                (*BarGraph).style &= !(0x8000 as libc::c_int) as libc::c_uint
            } else {
                (*BarGraph).majorSize = 0 as libc::c_int as UWORD;
                (*BarGraph).style |= 0x8000 as libc::c_int as libc::c_uint
            }
        }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"intUpdateProgressBar: invalid object type\x00" as
                          *const u8 as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"intdisplay.c\x00" as *const u8 as *const libc::c_char,
                      386 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 21],
                                                &[libc::c_char; 21]>(b"intUpdateProgressBar\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn intUpdateQuantity(mut psWidget: *mut _widget,
                                           mut psContext: *mut _w_context) {
    let mut psObj: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut Structure: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psTemplate: *mut DROID_TEMPLATE = 0 as *mut DROID_TEMPLATE;
    let mut Label: *mut W_LABEL = psWidget as *mut W_LABEL;
    let mut Quantity: UDWORD = 0;
    let mut Remaining: UDWORD = 0;
    psObj = (*Label).pUserData as *mut BASE_OBJECT;
    Structure = psObj as *mut STRUCTURE;
    if !psObj.is_null() &&
           (*psObj).type_0 as libc::c_uint ==
               OBJ_STRUCTURE as libc::c_int as libc::c_uint &&
           StructureIsManufacturing(Structure) != 0 {
        if (*psObj).died == 0 {
        } else {
            debug(LOG_ERROR,
                  b"intUpdateQuantity: object is dead\x00" as *const u8 as
                      *const libc::c_char);
        };
        if (*psObj).died == 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"intdisplay.c\x00" as *const u8 as *const libc::c_char,
                  405 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 18],
                                            &[libc::c_char; 18]>(b"intUpdateQuantity\x00")).as_ptr(),
                  b"!psObj->died\x00" as *const u8 as *const libc::c_char);
        };
        /*Quantity = StructureGetFactory(Structure)->quantity;
		if (Quantity == NON_STOP_PRODUCTION)
		{
			Label->aText[0] = (UBYTE)('*');
			Label->aText[1] = (UBYTE)('\0');
		}
		else
		{
			Label->aText[0] = (UBYTE)('0'+Quantity / 10);
			Label->aText[1] = (UBYTE)('0'+Quantity % 10);
		}*/
        psTemplate =
            (*StructureGetFactory(Structure)).psSubject as
                *mut DROID_TEMPLATE;
        //Quantity = getProductionQuantity(Structure, psTemplate) -
		//					getProductionBuilt(Structure, psTemplate);
        Quantity = getProductionQuantity(Structure, psTemplate);
        Remaining = getProductionBuilt(Structure, psTemplate);
        if Quantity > Remaining {
            Quantity =
                (Quantity as libc::c_uint).wrapping_sub(Remaining) as UDWORD
                    as UDWORD
        } else { Quantity = 0 as libc::c_int as UDWORD }
        if Quantity != 0 {
            (*Label).aText[0 as libc::c_int as usize] =
                ('0' as i32 as
                     libc::c_uint).wrapping_add(Quantity.wrapping_div(10 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_uint))
                    as UBYTE as STRING;
            (*Label).aText[1 as libc::c_int as usize] =
                ('0' as i32 as
                     libc::c_uint).wrapping_add(Quantity.wrapping_rem(10 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_uint))
                    as UBYTE as STRING
        }
        (*Label).style &= !(0x8000 as libc::c_int) as libc::c_uint
    } else { (*Label).style |= 0x8000 as libc::c_int as libc::c_uint };
}
//callback to display the factory number
#[no_mangle]
pub unsafe extern "C" fn intAddFactoryInc(mut psWidget: *mut _widget,
                                          mut psContext: *mut _w_context) {
    let mut psObj: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut Structure: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut Label: *mut W_LABEL = psWidget as *mut W_LABEL;
    // Get the object associated with this widget.
    psObj = (*Label).pUserData as *mut BASE_OBJECT;
    if !psObj.is_null() {
        if 1 as libc::c_int != 0 &&
               (*psObj).type_0 as libc::c_uint ==
                   OBJ_STRUCTURE as libc::c_int as libc::c_uint {
        } else {
            debug(LOG_ERROR,
                  b"intAddFactoryInc: invalid structure pointer\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 1 as libc::c_int != 0 &&
               (*psObj).type_0 as libc::c_uint ==
                   OBJ_STRUCTURE as libc::c_int as libc::c_uint {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"intdisplay.c\x00" as *const u8 as *const libc::c_char,
                  457 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 17],
                                            &[libc::c_char; 17]>(b"intAddFactoryInc\x00")).as_ptr(),
                  b"PTRVALID(psObj, sizeof(STRUCTURE)) && psObj->type == OBJ_STRUCTURE\x00"
                      as *const u8 as *const libc::c_char);
        };
        if (*psObj).died == 0 {
        } else {
            debug(LOG_ERROR,
                  b"intAddFactoryInc: object is dead\x00" as *const u8 as
                      *const libc::c_char);
        };
        if (*psObj).died == 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"intdisplay.c\x00" as *const u8 as *const libc::c_char,
                  459 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 17],
                                            &[libc::c_char; 17]>(b"intAddFactoryInc\x00")).as_ptr(),
                  b"!psObj->died\x00" as *const u8 as *const libc::c_char);
        };
        Structure = psObj as *mut STRUCTURE;
        if (*(*Structure).pStructureType).type_0 ==
               REF_FACTORY as libc::c_int as libc::c_uint ||
               (*(*Structure).pStructureType).type_0 ==
                   REF_CYBORG_FACTORY as libc::c_int as libc::c_uint ||
               (*(*Structure).pStructureType).type_0 ==
                   REF_VTOL_FACTORY as libc::c_int as libc::c_uint {
        } else {
            debug(LOG_ERROR,
                  b"intAddFactoryInc: structure is not a factory\x00" as
                      *const u8 as *const libc::c_char);
        };
        if (*(*Structure).pStructureType).type_0 ==
               REF_FACTORY as libc::c_int as libc::c_uint ||
               (*(*Structure).pStructureType).type_0 ==
                   REF_CYBORG_FACTORY as libc::c_int as libc::c_uint ||
               (*(*Structure).pStructureType).type_0 ==
                   REF_VTOL_FACTORY as libc::c_int as libc::c_uint {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"intdisplay.c\x00" as *const u8 as *const libc::c_char,
                  466 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 17],
                                            &[libc::c_char; 17]>(b"intAddFactoryInc\x00")).as_ptr(),
                  b"(Structure->pStructureType->type == REF_FACTORY OR Structure->pStructureType->type == REF_CYBORG_FACTORY OR Structure->pStructureType->type == REF_VTOL_FACTORY)\x00"
                      as *const u8 as *const libc::c_char);
        };
        (*Label).aText[0 as libc::c_int as usize] =
            ('0' as i32 +
                 ((*(*((*Structure).pFunctionality as
                           *mut FACTORY)).psAssemblyPoint).factoryInc as
                      libc::c_int + 1 as libc::c_int)) as UBYTE as STRING;
        (*Label).aText[1 as libc::c_int as usize] =
            '\u{0}' as i32 as UBYTE as STRING;
        (*Label).style &= !(0x8000 as libc::c_int) as libc::c_uint
    } else {
        (*Label).aText[0 as libc::c_int as usize] =
            0 as libc::c_int as UBYTE as STRING;
        (*Label).style |= 0x8000 as libc::c_int as libc::c_uint
    };
}
//callback to display the production quantity number for a template
#[no_mangle]
pub unsafe extern "C" fn intAddProdQuantity(mut psWidget: *mut _widget,
                                            mut psContext: *mut _w_context) {
    let mut psStat: *mut BASE_STATS = 0 as *mut BASE_STATS;
    let mut psTemplate: *mut DROID_TEMPLATE = 0 as *mut DROID_TEMPLATE;
    let mut psStructure: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psObj: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut Label: *mut W_LABEL = psWidget as *mut W_LABEL;
    let mut quantity: UDWORD = 0 as libc::c_int as UDWORD;
    // Get the object associated with this widget.
    psStat = (*Label).pUserData as *mut BASE_STATS;
    if !psStat.is_null() {
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"intAddProdQuantity: invalid template pointer\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"intdisplay.c\x00" as *const u8 as *const libc::c_char,
                  495 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 19],
                                            &[libc::c_char; 19]>(b"intAddProdQuantity\x00")).as_ptr(),
                  b"PTRVALID(psStat, sizeof(DROID_TEMPLATE))\x00" as *const u8
                      as *const libc::c_char);
        };
        psTemplate = psStat as *mut DROID_TEMPLATE;
        psObj = getCurrentSelected();
        if !psObj.is_null() &&
               (*psObj).type_0 as libc::c_uint ==
                   OBJ_STRUCTURE as libc::c_int as libc::c_uint {
            psStructure = psObj as *mut STRUCTURE
        }
        if !psStructure.is_null() && StructIsFactory(psStructure) != 0 {
            quantity = getProductionQuantity(psStructure, psTemplate)
        }
        if quantity != 0 as libc::c_int as libc::c_uint {
            (*Label).aText[0 as libc::c_int as usize] =
                ('0' as i32 as libc::c_uint).wrapping_add(quantity) as UBYTE
                    as STRING;
            (*Label).aText[1 as libc::c_int as usize] =
                '\u{0}' as i32 as UBYTE as STRING;
            (*Label).style &= !(0x8000 as libc::c_int) as libc::c_uint
        } else {
            (*Label).aText[0 as libc::c_int as usize] =
                0 as libc::c_int as UBYTE as STRING;
            (*Label).style |= 0x8000 as libc::c_int as libc::c_uint
        }
    };
}
//callback to display the production loop quantity number for a factory
#[no_mangle]
pub unsafe extern "C" fn intAddLoopQuantity(mut psWidget: *mut _widget,
                                            mut psContext: *mut _w_context) {
    let mut psFactory: *mut FACTORY = 0 as *mut FACTORY;
    let mut Label: *mut W_LABEL = psWidget as *mut W_LABEL;
    //loop depends on the factory
    if !(*Label).pUserData.is_null() {
        psFactory =
            (*((*Label).pUserData as *mut STRUCTURE)).pFunctionality as
                *mut FACTORY;
        if (*psFactory).quantity != 0 {
            if (*psFactory).quantity as libc::c_int == 9 as libc::c_int {
                (*Label).aText[0 as libc::c_int as usize] =
                    7 as libc::c_int as UBYTE as STRING;
                (*Label).aText[1 as libc::c_int as usize] =
                    '\u{0}' as i32 as UBYTE as STRING
            } else {
                (*Label).aText[0 as libc::c_int as usize] =
                    ('0' as i32 +
                         (*psFactory).quantity as libc::c_int /
                             10 as libc::c_int) as UBYTE as STRING;
                (*Label).aText[1 as libc::c_int as usize] =
                    ('0' as i32 +
                         ((*psFactory).quantity as libc::c_int +
                              1 as libc::c_int) % 10 as libc::c_int) as UBYTE
                        as STRING;
                (*Label).aText[2 as libc::c_int as usize] =
                    '\u{0}' as i32 as UBYTE as STRING
            }
        } else {
            //set to default loop quantity
            (*Label).aText[0 as libc::c_int as usize] =
                '0' as i32 as UBYTE as STRING;
            (*Label).aText[1 as libc::c_int as usize] =
                ('0' as i32 + 1 as libc::c_int) as UBYTE as STRING;
            (*Label).aText[2 as libc::c_int as usize] =
                '\u{0}' as i32 as UBYTE as STRING
        }
        (*Label).style &= !(0x8000 as libc::c_int) as libc::c_uint
    } else {
        //hide the label if no factory
        (*Label).aText[0 as libc::c_int as usize] =
            0 as libc::c_int as UBYTE as STRING;
        (*Label).style |= 0x8000 as libc::c_int as libc::c_uint
    };
}
// callback to update the command droid size label
#[no_mangle]
pub unsafe extern "C" fn intUpdateCommandSize(mut psWidget: *mut _widget,
                                              mut psContext:
                                                  *mut _w_context) {
    let mut psObj: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut Label: *mut W_LABEL = psWidget as *mut W_LABEL;
    // Get the object associated with this widget.
    psObj = (*Label).pUserData as *mut BASE_OBJECT;
    if !psObj.is_null() {
        if 1 as libc::c_int != 0 &&
               (*psObj).type_0 as libc::c_uint ==
                   OBJ_DROID as libc::c_int as libc::c_uint {
        } else {
            debug(LOG_ERROR,
                  b"intUpdateCommandSize: invalid droid pointer\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 1 as libc::c_int != 0 &&
               (*psObj).type_0 as libc::c_uint ==
                   OBJ_DROID as libc::c_int as libc::c_uint {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"intdisplay.c\x00" as *const u8 as *const libc::c_char,
                  578 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 21],
                                            &[libc::c_char; 21]>(b"intUpdateCommandSize\x00")).as_ptr(),
                  b"PTRVALID(psObj, sizeof(DROID)) && psObj->type == OBJ_DROID\x00"
                      as *const u8 as *const libc::c_char);
        };
        if (*psObj).died == 0 {
        } else {
            debug(LOG_ERROR,
                  b"intUpdateCommandSize: droid has died\x00" as *const u8 as
                      *const libc::c_char);
        };
        if (*psObj).died == 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"intdisplay.c\x00" as *const u8 as *const libc::c_char,
                  580 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 21],
                                            &[libc::c_char; 21]>(b"intUpdateCommandSize\x00")).as_ptr(),
                  b"!psObj->died\x00" as *const u8 as *const libc::c_char);
        };
        psDroid = psObj as *mut DROID;
        if (*psDroid).droidType as libc::c_uint ==
               DROID_COMMAND as libc::c_int as libc::c_uint {
        } else {
            debug(LOG_ERROR,
                  b"intUpdateCommandSize: droid is not a command droid\x00" as
                      *const u8 as *const libc::c_char);
        };
        if (*psDroid).droidType as libc::c_uint ==
               DROID_COMMAND as libc::c_int as libc::c_uint {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"intdisplay.c\x00" as *const u8 as *const libc::c_char,
                  585 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 21],
                                            &[libc::c_char; 21]>(b"intUpdateCommandSize\x00")).as_ptr(),
                  b"psDroid->droidType == DROID_COMMAND\x00" as *const u8 as
                      *const libc::c_char);
        };
        sprintf((*Label).aText.as_mut_ptr(),
                b"%d/%d\x00" as *const u8 as *const libc::c_char,
                if !(*psDroid).psGroup.is_null() {
                    grpNumMembers((*psDroid).psGroup)
                } else { 0 as libc::c_int }, cmdDroidMaxGroup(psDroid));
        (*Label).style &= !(0x8000 as libc::c_int) as libc::c_uint
    } else {
        (*Label).aText[0 as libc::c_int as usize] =
            0 as libc::c_int as UBYTE as STRING;
        (*Label).style |= 0x8000 as libc::c_int as libc::c_uint
    };
}
// callback to update the command droid experience
#[no_mangle]
pub unsafe extern "C" fn intUpdateCommandExp(mut psWidget: *mut _widget,
                                             mut psContext: *mut _w_context) {
    let mut psObj: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut Label: *mut W_LABEL = psWidget as *mut W_LABEL;
    let mut i: SDWORD = 0;
    let mut numStars: SDWORD = 0;
    // Get the object associated with this widget.
    psObj = (*Label).pUserData as *mut BASE_OBJECT;
    if !psObj.is_null() {
        if 1 as libc::c_int != 0 &&
               (*psObj).type_0 as libc::c_uint ==
                   OBJ_DROID as libc::c_int as libc::c_uint {
        } else {
            debug(LOG_ERROR,
                  b"intUpdateCommandSize: invalid droid pointer\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 1 as libc::c_int != 0 &&
               (*psObj).type_0 as libc::c_uint ==
                   OBJ_DROID as libc::c_int as libc::c_uint {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"intdisplay.c\x00" as *const u8 as *const libc::c_char,
                  610 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 20],
                                            &[libc::c_char; 20]>(b"intUpdateCommandExp\x00")).as_ptr(),
                  b"PTRVALID(psObj, sizeof(DROID)) && psObj->type == OBJ_DROID\x00"
                      as *const u8 as *const libc::c_char);
        };
        if (*psObj).died == 0 {
        } else {
            debug(LOG_ERROR,
                  b"intUpdateCommandSize: droid has died\x00" as *const u8 as
                      *const libc::c_char);
        };
        if (*psObj).died == 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"intdisplay.c\x00" as *const u8 as *const libc::c_char,
                  612 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 20],
                                            &[libc::c_char; 20]>(b"intUpdateCommandExp\x00")).as_ptr(),
                  b"!psObj->died\x00" as *const u8 as *const libc::c_char);
        };
        psDroid = psObj as *mut DROID;
        if (*psDroid).droidType as libc::c_uint ==
               DROID_COMMAND as libc::c_int as libc::c_uint {
        } else {
            debug(LOG_ERROR,
                  b"intUpdateCommandSize: droid is not a command droid\x00" as
                      *const u8 as *const libc::c_char);
        };
        if (*psDroid).droidType as libc::c_uint ==
               DROID_COMMAND as libc::c_int as libc::c_uint {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"intdisplay.c\x00" as *const u8 as *const libc::c_char,
                  617 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 20],
                                            &[libc::c_char; 20]>(b"intUpdateCommandExp\x00")).as_ptr(),
                  b"psDroid->droidType == DROID_COMMAND\x00" as *const u8 as
                      *const libc::c_char);
        };
        numStars = cmdDroidGetLevel(psDroid);
        numStars =
            if numStars >= 1 as libc::c_int {
                (numStars) - 1 as libc::c_int
            } else { 0 as libc::c_int };
        i = 0 as libc::c_int;
        while i < numStars {
            (*Label).aText[i as usize] = '*' as i32 as STRING;
            i += 1
        }
        (*Label).aText[i as usize] = '\u{0}' as i32 as STRING;
        (*Label).style &= !(0x8000 as libc::c_int) as libc::c_uint
    } else {
        (*Label).aText[0 as libc::c_int as usize] =
            0 as libc::c_int as UBYTE as STRING;
        (*Label).style |= 0x8000 as libc::c_int as libc::c_uint
    };
}
// callback to update the command droid factories
#[no_mangle]
pub unsafe extern "C" fn intUpdateCommandFact(mut psWidget: *mut _widget,
                                              mut psContext:
                                                  *mut _w_context) {
    let mut psObj: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut Label: *mut W_LABEL = psWidget as *mut W_LABEL;
    let mut i: SDWORD = 0;
    let mut cIndex: SDWORD = 0;
    let mut start: SDWORD = 0;
    // Get the object associated with this widget.
    psObj = (*Label).pUserData as *mut BASE_OBJECT;
    if !psObj.is_null() {
        if 1 as libc::c_int != 0 &&
               (*psObj).type_0 as libc::c_uint ==
                   OBJ_DROID as libc::c_int as libc::c_uint {
        } else {
            debug(LOG_ERROR,
                  b"intUpdateCommandSize: invalid droid pointer\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 1 as libc::c_int != 0 &&
               (*psObj).type_0 as libc::c_uint ==
                   OBJ_DROID as libc::c_int as libc::c_uint {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"intdisplay.c\x00" as *const u8 as *const libc::c_char,
                  648 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 21],
                                            &[libc::c_char; 21]>(b"intUpdateCommandFact\x00")).as_ptr(),
                  b"PTRVALID(psObj, sizeof(DROID)) && psObj->type == OBJ_DROID\x00"
                      as *const u8 as *const libc::c_char);
        };
        if (*psObj).died == 0 {
        } else {
            debug(LOG_ERROR,
                  b"intUpdateCommandSize: droid has died\x00" as *const u8 as
                      *const libc::c_char);
        };
        if (*psObj).died == 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"intdisplay.c\x00" as *const u8 as *const libc::c_char,
                  650 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 21],
                                            &[libc::c_char; 21]>(b"intUpdateCommandFact\x00")).as_ptr(),
                  b"!psObj->died\x00" as *const u8 as *const libc::c_char);
        };
        psDroid = psObj as *mut DROID;
        if (*psDroid).droidType as libc::c_uint ==
               DROID_COMMAND as libc::c_int as libc::c_uint {
        } else {
            debug(LOG_ERROR,
                  b"intUpdateCommandSize: droid is not a command droid\x00" as
                      *const u8 as *const libc::c_char);
        };
        if (*psDroid).droidType as libc::c_uint ==
               DROID_COMMAND as libc::c_int as libc::c_uint {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"intdisplay.c\x00" as *const u8 as *const libc::c_char,
                  655 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 21],
                                            &[libc::c_char; 21]>(b"intUpdateCommandFact\x00")).as_ptr(),
                  b"psDroid->droidType == DROID_COMMAND\x00" as *const u8 as
                      *const libc::c_char);
        };
        // see which type of factory this is for
        if (*Label).id >= 3400 as libc::c_int as libc::c_uint &&
               (*Label).id < 3499 as libc::c_int as libc::c_uint {
            start = 9 as libc::c_int
        } else if (*Label).id >= 3750 as libc::c_int as libc::c_uint &&
                      (*Label).id < 3799 as libc::c_int as libc::c_uint {
            start = 9 as libc::c_int + 5 as libc::c_int
        } else { start = 24 as libc::c_int }
        cIndex = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < 5 as libc::c_int {
            if (*psDroid).secondaryOrder &
                   ((1 as libc::c_int) << i + start) as libc::c_uint != 0 {
                (*Label).aText[cIndex as usize] =
                    ('0' as i32 + i + 1 as libc::c_int) as STRING;
                cIndex += 1 as libc::c_int
            }
            i += 1
        }
        (*Label).aText[cIndex as usize] = '\u{0}' as i32 as STRING;
        (*Label).style &= !(0x8000 as libc::c_int) as libc::c_uint
    } else {
        (*Label).aText[0 as libc::c_int as usize] =
            0 as libc::c_int as UBYTE as STRING;
        (*Label).style |= 0x8000 as libc::c_int as libc::c_uint
    };
}
//#ifndef PSX
//#endif
// Widget callback to update and display the power bar.
// !!!!!!!!!!!!!!!!!!!!!!ONLY WORKS ON A SIDEWAYS POWERBAR!!!!!!!!!!!!!!!!!
#[no_mangle]
pub unsafe extern "C" fn intDisplayPowerBar(mut psWidget: *mut _widget,
                                            mut xOffset: UDWORD,
                                            mut yOffset: UDWORD,
                                            mut pColours: *mut UDWORD) {
    let mut BarGraph: *mut W_BARGRAPH = psWidget as *mut W_BARGRAPH;
    let mut x0: SDWORD = 0;
    let mut y0: SDWORD = 0;
    let mut Avail: SDWORD = 0;
    let mut ManPow: SDWORD = 0;
    let mut realPower: SDWORD = 0;
    let mut Empty: SDWORD = 0;
    let mut BarWidth: SDWORD = 0;
    let mut textWidth: SDWORD = 0 as libc::c_int;
    let mut iX: SDWORD = 0;
    let mut iY: SDWORD = 0;
    static mut szVal: [libc::c_char; 8] = [0; 8];
    //SDWORD Used,Avail,ManPow;
    //	asPower[selectedPlayer]->availablePower+=32;	// temp to test.
    ManPow =
        ManuPower.wrapping_div((5 as libc::c_int * 100 as libc::c_int /
                                    (60 as libc::c_int - 8 as libc::c_int)) as
                                   libc::c_uint) as SDWORD;
    Avail =
        (*asPower[selectedPlayer as
                      usize]).currentPower.wrapping_div((5 as libc::c_int *
                                                             100 as
                                                                 libc::c_int /
                                                             (60 as
                                                                  libc::c_int
                                                                  -
                                                                  8 as
                                                                      libc::c_int))
                                                            as libc::c_uint)
            as SDWORD;
    realPower =
        (*asPower[selectedPlayer as
                      usize]).currentPower.wrapping_sub(ManuPower) as SDWORD;
    BarWidth = (*BarGraph).width as SDWORD;
    iV_SetFont(WFont);
    sprintf(szVal.as_mut_ptr(), b"%d\x00" as *const u8 as *const libc::c_char,
            realPower);
    textWidth = iV_GetTextWidth(szVal.as_mut_ptr());
    BarWidth -= textWidth;
    /*Avail = asPower[selectedPlayer]->availablePower / POWERBAR_SCALE;
	Used = asPower[selectedPlayer]->usedPower / POWERBAR_SCALE;*/
    /*if (Used < 0)
	{
		Used = 0;
	}

	Total = Avail + Used;*/
    /*if(ManPow > Avail) {
		ManPow = Avail;
	}*/
    //Empty = BarGraph->width - Total;
    if ManPow > Avail {
        Empty = BarWidth - ManPow
    } else { Empty = BarWidth - Avail }
    //if(Total > BarGraph->width) {				// If total size greater than bar size then scale values.
    if Avail > BarWidth {
        //Used = PERNUM(BarGraph->width,Used,Total);
		//ManPow = PERNUM(BarGraph->width,ManPow,Total);
		//Avail = BarGraph->width - Used;
        ManPow = ManPow * BarWidth / Avail;
        Avail = BarWidth;
        Empty = 0 as libc::c_int
    }
    if ManPow > BarWidth {
        ManPow = BarWidth;
        Avail = 0 as libc::c_int;
        Empty = 0 as libc::c_int
    }
    x0 = xOffset.wrapping_add((*BarGraph).x as libc::c_uint) as SDWORD;
    y0 = yOffset.wrapping_add((*BarGraph).y as libc::c_uint) as SDWORD;
    //	pie_SetDepthBufferStatus(DEPTH_CMP_ALWAYS_WRT_OFF);
    pie_SetDepthBufferStatus(DEPTH_CMP_ALWAYS_WRT_ON);
    pie_SetFogStatus(0 as libc::c_int);
    pie_ImageFileID(IntImages, IMAGE_PBAR_TOP as libc::c_int as UWORD, x0,
                    y0);
    iX = x0 + 3 as libc::c_int;
    iY = y0 + 9 as libc::c_int;
    x0 +=
        iV_GetImageWidthNoCC(IntImages,
                             IMAGE_PBAR_TOP as libc::c_int as UWORD) as
            libc::c_int;
    /* indent to allow text value */
	//draw used section
	/*iV_DrawImageRect(IntImages,IMAGE_PBAR_USED,
						x0,y0,
						0,0,
						Used, iV_GetImageHeight(IntImages,IMAGE_PBAR_USED));
	x0 += Used;*/
    //fill in the empty section behind text
    if textWidth > 0 as libc::c_int {
        pie_ImageFileIDTile(IntImages,
                            IMAGE_PBAR_EMPTY as libc::c_int as UWORD, x0, y0,
                            0 as libc::c_int, 0 as libc::c_int, textWidth,
                            iV_GetImageHeightNoCC(IntImages,
                                                  IMAGE_PBAR_EMPTY as
                                                      libc::c_int as UWORD) as
                                libc::c_int);
        x0 += textWidth
    }
    //draw required section
    if ManPow > Avail {
        //draw the required in red
        pie_ImageFileIDTile(IntImages,
                            IMAGE_PBAR_USED as libc::c_int as UWORD, x0, y0,
                            0 as libc::c_int, 0 as libc::c_int, ManPow,
                            iV_GetImageHeightNoCC(IntImages,
                                                  IMAGE_PBAR_USED as
                                                      libc::c_int as UWORD) as
                                libc::c_int);
    } else {
        pie_ImageFileIDTile(IntImages,
                            IMAGE_PBAR_REQUIRED as libc::c_int as UWORD, x0,
                            y0, 0 as libc::c_int, 0 as libc::c_int, ManPow,
                            iV_GetImageHeightNoCC(IntImages,
                                                  IMAGE_PBAR_REQUIRED as
                                                      libc::c_int as UWORD) as
                                libc::c_int);
    }
    x0 += ManPow;
    //draw the available section if any!
    if Avail - ManPow > 0 as libc::c_int {
        pie_ImageFileIDTile(IntImages,
                            IMAGE_PBAR_AVAIL as libc::c_int as UWORD, x0, y0,
                            0 as libc::c_int, 0 as libc::c_int,
                            Avail - ManPow,
                            iV_GetImageHeightNoCC(IntImages,
                                                  IMAGE_PBAR_AVAIL as
                                                      libc::c_int as UWORD) as
                                libc::c_int);
        x0 += Avail - ManPow
    }
    //fill in the rest with empty section
    if Empty > 0 as libc::c_int {
        pie_ImageFileIDTile(IntImages,
                            IMAGE_PBAR_EMPTY as libc::c_int as UWORD, x0, y0,
                            0 as libc::c_int, 0 as libc::c_int, Empty,
                            iV_GetImageHeightNoCC(IntImages,
                                                  IMAGE_PBAR_EMPTY as
                                                      libc::c_int as UWORD) as
                                libc::c_int);
        x0 += Empty
    }
    pie_ImageFileID(IntImages, IMAGE_PBAR_BOTTOM as libc::c_int as UWORD, x0,
                    y0);
    /* draw text value */
    iV_SetTextColour(-(1 as libc::c_int) as SWORD);
    pie_DrawText(szVal.as_mut_ptr(), iX as UDWORD, iY as UDWORD);
}
// Widget callback to display a rendered status button, ie the progress of a manufacturing or
// building task.
//
#[no_mangle]
pub unsafe extern "C" fn intDisplayStatusButton(mut psWidget: *mut _widget,
                                                mut xOffset: UDWORD,
                                                mut yOffset: UDWORD,
                                                mut pColours: *mut UDWORD) {
    let mut Form: *mut W_CLICKFORM =
        psWidget as *mut W_CLICKFORM; // changed by AJL for multiplayer.
    let mut psObj: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut Structure: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut Droid: *mut DROID = 0 as *mut DROID;
    let mut Down: BOOL = 0;
    let mut Image: SDWORD = 0;
    let mut Hilight: BOOL = 0 as libc::c_int;
    let mut Stats: *mut BASE_STATS = 0 as *mut BASE_STATS;
    let mut psResGraphic: *mut BASE_STATS = 0 as *mut BASE_STATS;
    let mut Buffer: *mut RENDERED_BUTTON =
        (*Form).pUserData as *mut RENDERED_BUTTON;
    let mut IMDType: UDWORD = 0 as libc::c_int as UDWORD;
    let mut compID: UDWORD = 0;
    let mut Player: UDWORD = selectedPlayer;
    let mut Object: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut bOnHold: BOOL = 0 as libc::c_int;
    OpenButtonRender(xOffset.wrapping_add((*Form).x as libc::c_uint) as UWORD,
                     yOffset.wrapping_add((*Form).y as libc::c_uint) as UWORD,
                     (*Form).width, (*Form).height);
    Down =
        ((*Form).state &
             (0x1 as libc::c_int | 0x8 as libc::c_int | 0x10 as libc::c_int)
                 as libc::c_uint) as BOOL;
    Hilight = ((*Form).state & 0x4 as libc::c_int as libc::c_uint) as BOOL;
    if Hilight != 0 {
        (*Buffer).ImdRotation +=
            (90 as libc::c_int as
                 libc::c_uint).wrapping_mul(frameTime2).wrapping_div(1000 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_uint)
                as UWORD as libc::c_int
    }
    //						RENDERBUTTON_INITIALISED(Buffer);
    Hilight =
        ((*Form).state & 0x4 as libc::c_int as libc::c_uint) as
            BOOL; // Hilited or flashing.
    (*Buffer).State = (*Form).state;
    Object = 0 as *mut libc::c_void;
    Image = -(1 as libc::c_int);
    psObj = (*Buffer).Data as *mut BASE_OBJECT;
    if !psObj.is_null() && (*psObj).died != 0 &&
           (*psObj).died != 1 as libc::c_int as libc::c_uint {
        //		Down = Form->state & (WCLICK_DOWN | WCLICK_LOCKED | WCLICK_CLICKLOCK);
        // Get the object associated with this widget.
        // this may catch this horrible crash bug we've been having,
			// who knows?.... Shipping tomorrow, la de da :-)
        psObj = 0 as *mut BASE_OBJECT;
        (*Buffer).Data = 0 as *mut libc::c_void;
        intRefreshScreen();
    }
    if !psObj.is_null() {
        //			screenTextOut(64,48,"psObj: %p",psObj);
        match (*psObj).type_0 as libc::c_uint {
            0 => {
                // If it's a droid...
                Droid = psObj as *mut DROID;
                if DroidIsBuilding(Droid) != 0 {
                    Structure = DroidGetBuildStructure(Droid);
                    //						DBPRINTF(("%p : %p",Droid,Structure));
                    if !Structure.is_null() {
                        Object =
                            Structure as
                                *mut libc::c_void; //(void*)StructureGetIMD(Structure);
                        IMDType =
                            IMDTYPE_STRUCTURE as libc::c_int as
                                UDWORD; //StatGetStructureIMD(Stats,selectedPlayer);
                        (*Buffer).Initialised = 1 as libc::c_int
                    }
                } else if DroidGoingToBuild(Droid) != 0 {
                    Stats = DroidGetBuildStats(Droid);
                    if !Stats.is_null() {
                    } else {
                        debug(LOG_ERROR,
                              b"intDisplayStatusButton : NULL Stats pointer.\x00"
                                  as *const u8 as *const libc::c_char);
                    };
                    if !Stats.is_null() {
                    } else {
                        debug(LOG_ERROR,
                              b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                                  *const u8 as *const libc::c_char,
                              b"intdisplay.c\x00" as *const u8 as
                                  *const libc::c_char, 936 as libc::c_int,
                              (*::std::mem::transmute::<&[u8; 23],
                                                        &[libc::c_char; 23]>(b"intDisplayStatusButton\x00")).as_ptr(),
                              b"Stats!=NULL\x00" as *const u8 as
                                  *const libc::c_char);
                    };
                    Object = Stats as *mut libc::c_void;
                    Player = selectedPlayer;
                    IMDType = IMDTYPE_STRUCTURESTAT as libc::c_int as UDWORD;
                    (*Buffer).Initialised = 1 as libc::c_int
                } else if orderState(Droid, DORDER_DEMOLISH) != 0 {
                    Stats = structGetDemolishStat() as *mut BASE_STATS;
                    if !Stats.is_null() {
                    } else {
                        debug(LOG_ERROR,
                              b"intDisplayStatusButton : NULL Stats pointer.\x00"
                                  as *const u8 as *const libc::c_char);
                    };
                    if !Stats.is_null() {
                    } else {
                        debug(LOG_ERROR,
                              b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                                  *const u8 as *const libc::c_char,
                              b"intdisplay.c\x00" as *const u8 as
                                  *const libc::c_char, 944 as libc::c_int,
                              (*::std::mem::transmute::<&[u8; 23],
                                                        &[libc::c_char; 23]>(b"intDisplayStatusButton\x00")).as_ptr(),
                              b"Stats!=NULL\x00" as *const u8 as
                                  *const libc::c_char);
                    };
                    Object = Stats as *mut libc::c_void;
                    Player = selectedPlayer;
                    IMDType = IMDTYPE_STRUCTURESTAT as libc::c_int as UDWORD;
                    (*Buffer).Initialised = 1 as libc::c_int
                } else if (*Droid).droidType as libc::c_uint ==
                              DROID_COMMAND as libc::c_int as libc::c_uint {
                    Structure = droidGetCommandFactory(Droid);
                    if !Structure.is_null() {
                        Object = Structure as *mut libc::c_void;
                        IMDType = IMDTYPE_STRUCTURE as libc::c_int as UDWORD;
                        (*Buffer).Initialised = 1 as libc::c_int
                    }
                }
            }
            1 => {
                // If it's a structure...
                Structure = psObj as *mut STRUCTURE;
                match (*(*Structure).pStructureType).type_0 {
                    1 | 16 | 17 => {
                        if StructureIsManufacturing(Structure) != 0 {
                            IMDType =
                                IMDTYPE_DROIDTEMPLATE as libc::c_int as
                                    UDWORD;
                            Object =
                                FactoryGetTemplate(StructureGetFactory(Structure))
                                    as *mut libc::c_void;
                            (*Buffer).Initialised = 1 as libc::c_int;
                            if (*StructureGetFactory(Structure)).timeStartHold
                                   != 0 {
                                bOnHold = 1 as libc::c_int
                            }
                        }
                    }
                    10 => {
                        if StructureIsResearching(Structure) != 0 {
                            Stats = (*Buffer).Data2 as *mut BASE_STATS;
                            if !Stats.is_null() {
                                //								Image = ResearchGetImage((RESEARCH_FACILITY*)Structure);
                                /*StatGetResearchImage(Stats,&Image,(iIMDShape**)&Object,FALSE);
									//if Object != NULL the there must be a IMD so set the object to
									//equal the Research stat
									if (Object != NULL)
									{
										Object = (void*)Stats;
									}
    								IMDType = IMDTYPE_RESEARCH;*/
                                if (*((*Structure).pFunctionality as
                                          *mut RESEARCH_FACILITY)).timeStartHold
                                       != 0 {
                                    bOnHold = 1 as libc::c_int
                                }
                                StatGetResearchImage(Stats, &mut Image,
                                                     &mut Object as
                                                         *mut *mut libc::c_void
                                                         as
                                                         *mut *mut iIMDShape,
                                                     &mut psResGraphic,
                                                     0 as libc::c_int);
                                if !psResGraphic.is_null() {
                                    //we have a Stat associated with this research topic
                                    if StatIsStructure(psResGraphic) != 0 {
                                        //overwrite the Object pointer
                                        Object =
                                            psResGraphic as *mut libc::c_void;
                                        Player = selectedPlayer;
                                        //this defines how the button is drawn
                                        IMDType =
                                            IMDTYPE_STRUCTURESTAT as
                                                libc::c_int as UDWORD
                                    } else {
                                        compID =
                                            StatIsComponent(psResGraphic) as
                                                UDWORD;
                                        if compID !=
                                               COMP_UNKNOWN as libc::c_int as
                                                   libc::c_uint {
                                            //this defines how the button is drawn
                                            IMDType =
                                                IMDTYPE_COMPONENT as
                                                    libc::c_int as UDWORD;
                                            //overwrite the Object pointer
                                            Object =
                                                psResGraphic as
                                                    *mut libc::c_void
                                        } else {
                                            if 0 as libc::c_int != 0 {
                                            } else {
                                                debug(LOG_ERROR,
                                                      b"intDisplayStatsButton:Invalid Stat for research button\x00"
                                                          as *const u8 as
                                                          *const libc::c_char);
                                            };
                                            if 0 as libc::c_int != 0 {
                                            } else {
                                                debug(LOG_ERROR,
                                                      b"Assert in Warzone: %s:%d : %s (%s)\x00"
                                                          as *const u8 as
                                                          *const libc::c_char,
                                                      b"intdisplay.c\x00" as
                                                          *const u8 as
                                                          *const libc::c_char,
                                                      1022 as libc::c_int,
                                                      (*::std::mem::transmute::<&[u8; 23],
                                                                                &[libc::c_char; 23]>(b"intDisplayStatusButton\x00")).as_ptr(),
                                                      b"FALSE\x00" as
                                                          *const u8 as
                                                          *const libc::c_char);
                                            };
                                            Object = 0 as *mut libc::c_void;
                                            IMDType =
                                                IMDTYPE_RESEARCH as
                                                    libc::c_int as UDWORD
                                        }
                                    }
                                } else if !Object.is_null() {
                                    Object = Stats as *mut libc::c_void;
                                    IMDType =
                                        IMDTYPE_RESEARCH as libc::c_int as
                                            UDWORD
                                }
                                (*Buffer).Initialised = 1 as libc::c_int
                            }
                        }
                    }
                    _ => { }
                }
            }
            _ => {
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"intDisplayObjectButton: invalid structure type\x00"
                              as *const u8 as *const libc::c_char);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"intdisplay.c\x00" as *const u8 as
                              *const libc::c_char, 1048 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 23],
                                                    &[libc::c_char; 23]>(b"intDisplayStatusButton\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
            }
        }
    } else { (*Buffer).Initialised = 1 as libc::c_int }
    ButtonDrawYOffset = 0 as libc::c_int;
    ButtonDrawXOffset = ButtonDrawYOffset;
    if !Object.is_null() {
        if Image >= 0 as libc::c_int {
            RenderToButton(IntImages, Image as UWORD, Object, Player, Buffer,
                           Down, IMDType, 0 as libc::c_int as UDWORD);
        } else {
            RenderToButton(0 as *mut IMAGEFILE, 0 as libc::c_int as UWORD,
                           Object, Player, Buffer, Down, IMDType,
                           0 as libc::c_int as UDWORD);
        }
    } else if Image >= 0 as libc::c_int {
        RenderImageToButton(IntImages, Image as UWORD, Buffer, Down,
                            0 as libc::c_int as UDWORD);
    } else { RenderBlankToButton(Buffer, Down, 0 as libc::c_int as UDWORD); }
    CloseButtonRender();
    //no Stat for this research topic so just use the graphic provided
                                        //if Object != NULL the there must be a IMD so set the object to
                                        //equal the Research stat
    // Render the object into the button.
    //need to flash the button if a factory is on hold production
    if bOnHold != 0 {
        if gameTime2.wrapping_div(250 as libc::c_int as
                                      libc::c_uint).wrapping_rem(2 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint)
               == 0 as libc::c_int as libc::c_uint {
            pie_ImageFileID(IntImages,
                            IMAGE_BUT0_DOWN as libc::c_int as UWORD,
                            xOffset.wrapping_add((*Form).x as libc::c_uint) as
                                libc::c_int,
                            yOffset.wrapping_add((*Form).y as libc::c_uint) as
                                libc::c_int);
        } else {
            pie_ImageFileID(IntImages,
                            IMAGE_BUT_HILITE as libc::c_int as UWORD,
                            xOffset.wrapping_add((*Form).x as libc::c_uint) as
                                libc::c_int,
                            yOffset.wrapping_add((*Form).y as libc::c_uint) as
                                libc::c_int);
        }
    } else if Hilight != 0 {
        pie_ImageFileID(IntImages, IMAGE_BUT_HILITE as libc::c_int as UWORD,
                        xOffset.wrapping_add((*Form).x as libc::c_uint) as
                            libc::c_int,
                        yOffset.wrapping_add((*Form).y as libc::c_uint) as
                            libc::c_int);
    };
}
// Widget callback to display a rendered object button.
//
#[no_mangle]
pub unsafe extern "C" fn intDisplayObjectButton(mut psWidget: *mut _widget,
                                                mut xOffset: UDWORD,
                                                mut yOffset: UDWORD,
                                                mut pColours: *mut UDWORD) {
    let mut Form: *mut W_CLICKFORM =
        psWidget as *mut W_CLICKFORM; // Hilited or flashing.
    let mut psObj: *mut BASE_OBJECT =
        0 as *mut BASE_OBJECT; // Get the object associated with this widget.
    let mut Down: BOOL = 0;
    let mut Hilight: BOOL = 0 as libc::c_int;
    let mut Buffer: *mut RENDERED_BUTTON =
        (*Form).pUserData as *mut RENDERED_BUTTON;
    let mut IMDType: UDWORD = 0 as libc::c_int as UDWORD;
    let mut Object: *mut libc::c_void = 0 as *mut libc::c_void;
    OpenButtonRender(xOffset.wrapping_add((*Form).x as libc::c_uint) as UWORD,
                     yOffset.wrapping_add((*Form).y as libc::c_uint) as UWORD,
                     (*Form).width,
                     ((*Form).height as libc::c_int + 9 as libc::c_int) as
                         UWORD);
    Down =
        ((*Form).state &
             (0x1 as libc::c_int | 0x8 as libc::c_int | 0x10 as libc::c_int)
                 as libc::c_uint) as BOOL;
    Hilight = ((*Form).state & 0x4 as libc::c_int as libc::c_uint) as BOOL;
    if Hilight != 0 {
        (*Buffer).ImdRotation +=
            (90 as libc::c_int as
                 libc::c_uint).wrapping_mul(frameTime2).wrapping_div(1000 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_uint)
                as UWORD as libc::c_int
    }
    Hilight = ((*Form).state & 0x4 as libc::c_int as libc::c_uint) as BOOL;
    (*Buffer).State = (*Form).state;
    Object = 0 as *mut libc::c_void;
    psObj = (*Buffer).Data as *mut BASE_OBJECT;
    if !psObj.is_null() && (*psObj).died != 0 &&
           (*psObj).died != 1 as libc::c_int as libc::c_uint {
        // this may catch this horrible crash bug we've been having,
			// who knows?.... Shipping tomorrow, la de da :-)
        psObj = 0 as *mut BASE_OBJECT;
        (*Buffer).Data = 0 as *mut libc::c_void;
        intRefreshScreen();
    }
    if !psObj.is_null() {
        match (*psObj).type_0 as libc::c_uint {
            0 => {
                // If it's a droid...
                IMDType = IMDTYPE_DROID as libc::c_int as UDWORD;
                Object = psObj as *mut libc::c_void
            }
            1 => {
                // If it's a structure...
                IMDType = IMDTYPE_STRUCTURE as libc::c_int as UDWORD;
                //					Object = (void*)StructureGetIMD((STRUCTURE*)psObj);
                Object = psObj as *mut libc::c_void
            }
            _ => {
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"intDisplayStatusButton: invalid structure type\x00"
                              as *const u8 as *const libc::c_char);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"intdisplay.c\x00" as *const u8 as
                              *const libc::c_char, 1155 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 23],
                                                    &[libc::c_char; 23]>(b"intDisplayObjectButton\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
            }
        }
    }
    ButtonDrawYOffset = 0 as libc::c_int;
    ButtonDrawXOffset = ButtonDrawYOffset;
    if !Object.is_null() {
        RenderToButton(0 as *mut IMAGEFILE, 0 as libc::c_int as UWORD, Object,
                       selectedPlayer, Buffer, Down, IMDType,
                       1 as libc::c_int as UDWORD);
        // ajl, changed from 0 to selectedPlayer
    } else { RenderBlankToButton(Buffer, Down, 1 as libc::c_int as UDWORD); }
    (*Buffer).Initialised = 1 as libc::c_int;
    CloseButtonRender();
    if Hilight != 0 {
        pie_ImageFileID(IntImages, IMAGE_BUTB_HILITE as libc::c_int as UWORD,
                        xOffset.wrapping_add((*Form).x as libc::c_uint) as
                            libc::c_int,
                        yOffset.wrapping_add((*Form).y as libc::c_uint) as
                            libc::c_int);
    };
}
// Widget callback to display a rendered stats button, ie the job selection window buttons.
//
#[no_mangle]
pub unsafe extern "C" fn intDisplayStatsButton(mut psWidget: *mut _widget,
                                               mut xOffset: UDWORD,
                                               mut yOffset: UDWORD,
                                               mut pColours: *mut UDWORD) {
    let mut Form: *mut W_CLICKFORM = psWidget as *mut W_CLICKFORM;
    let mut Stat: *mut BASE_STATS = 0 as *mut BASE_STATS;
    let mut psResGraphic: *mut BASE_STATS = 0 as *mut BASE_STATS;
    let mut Down: BOOL = 0;
    let mut Image: SDWORD = 0;
    let mut compID: SDWORD = 0;
    let mut Hilight: BOOL = 0 as libc::c_int;
    let mut Buffer: *mut RENDERED_BUTTON =
        (*Form).pUserData as *mut RENDERED_BUTTON;
    let mut IMDType: UDWORD = 0 as libc::c_int as UDWORD;
    //	UDWORD          IMDIndex = 0;
    let mut Player: UDWORD =
        selectedPlayer; // ajl, changed for multiplayer (from 0)
    let mut Object: *mut libc::c_void = 0 as *mut libc::c_void;
    OpenButtonRender(xOffset.wrapping_add((*Form).x as libc::c_uint) as UWORD,
                     yOffset.wrapping_add((*Form).y as libc::c_uint) as UWORD,
                     (*Form).width, (*Form).height);
    Down =
        ((*Form).state &
             (0x1 as libc::c_int | 0x8 as libc::c_int | 0x10 as libc::c_int)
                 as libc::c_uint) as BOOL;
    Hilight = ((*Form).state & 0x4 as libc::c_int as libc::c_uint) as BOOL;
    if Hilight != 0 {
        (*Buffer).ImdRotation +=
            (90 as libc::c_int as
                 libc::c_uint).wrapping_mul(frameTime2).wrapping_div(1000 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_uint)
                as UWORD as libc::c_int
    }
    Hilight = ((*Form).state & 0x4 as libc::c_int as libc::c_uint) as BOOL;
    (*Buffer).State = (*Form).state;
    Object = 0 as *mut libc::c_void;
    Image = -(1 as libc::c_int);
    Stat = (*Buffer).Data as *mut BASE_STATS;
    ButtonDrawYOffset = 0 as libc::c_int;
    ButtonDrawXOffset = ButtonDrawYOffset;
    if !Stat.is_null() {
        if StatIsStructure(Stat) != 0 {
            //				IMDType = IMDTYPE_STRUCTURE;
//				Object = (void*)StatGetStructureIMD(Stat,selectedPlayer);
            Object = Stat as *mut libc::c_void;
            Player = selectedPlayer;
            IMDType = IMDTYPE_STRUCTURESTAT as libc::c_int as UDWORD
        } else if StatIsTemplate(Stat) != 0 {
            IMDType = IMDTYPE_DROIDTEMPLATE as libc::c_int as UDWORD;
            Object = Stat as *mut libc::c_void
        } else {
            //if(StatIsComponent(Stat))
				//{
				//	IMDType = IMDTYPE_COMPONENT;
				//	Shape = StatGetComponentIMD(Stat);
				//}
            compID = StatIsComponent(Stat); // This failes for viper body.
            if compID != COMP_UNKNOWN as libc::c_int {
                IMDType = IMDTYPE_COMPONENT as libc::c_int as UDWORD;
                Object = Stat as *mut libc::c_void
                //StatGetComponentIMD(Stat, compID);
            } else if StatIsResearch(Stat) != 0 {
                /*IMDType = IMDTYPE_RESEARCH;
					StatGetResearchImage(Stat,&Image,(iIMDShape**)&Object,TRUE);
					//if Object != NULL the there must be a IMD so set the object to
					//equal the Research stat
					if (Object != NULL)
					{
						Object = (void*)Stat;
					}*/
                StatGetResearchImage(Stat, &mut Image,
                                     &mut Object as *mut *mut libc::c_void as
                                         *mut *mut iIMDShape,
                                     &mut psResGraphic, 1 as libc::c_int);
                if !psResGraphic.is_null() {
                    //we have a Stat associated with this research topic
                    if StatIsStructure(psResGraphic) != 0 {
                        //overwrite the Object pointer
                        Object = psResGraphic as *mut libc::c_void;
                        Player = selectedPlayer;
                        //this defines how the button is drawn
                        IMDType =
                            IMDTYPE_STRUCTURESTAT as libc::c_int as UDWORD
                    } else {
                        compID = StatIsComponent(psResGraphic);
                        if compID != COMP_UNKNOWN as libc::c_int {
                            //this defines how the button is drawn
                            IMDType =
                                IMDTYPE_COMPONENT as libc::c_int as UDWORD;
                            //overwrite the Object pointer
                            Object = psResGraphic as *mut libc::c_void
                        } else {
                            if 0 as libc::c_int != 0 {
                            } else {
                                debug(LOG_ERROR,
                                      b"intDisplayStatsButton:Invalid Stat for research button\x00"
                                          as *const u8 as
                                          *const libc::c_char);
                            };
                            if 0 as libc::c_int != 0 {
                            } else {
                                debug(LOG_ERROR,
                                      b"Assert in Warzone: %s:%d : %s (%s)\x00"
                                          as *const u8 as *const libc::c_char,
                                      b"intdisplay.c\x00" as *const u8 as
                                          *const libc::c_char,
                                      1283 as libc::c_int,
                                      (*::std::mem::transmute::<&[u8; 22],
                                                                &[libc::c_char; 22]>(b"intDisplayStatsButton\x00")).as_ptr(),
                                      b"FALSE\x00" as *const u8 as
                                          *const libc::c_char);
                            };
                            Object = 0 as *mut libc::c_void;
                            IMDType =
                                IMDTYPE_RESEARCH as libc::c_int as UDWORD
                        }
                    }
                } else if !Object.is_null() {
                    Object = Stat as *mut libc::c_void;
                    IMDType = IMDTYPE_RESEARCH as libc::c_int as UDWORD
                }
            }
        }
        if Down != 0 {
            CurrentStatsTemplate = Stat
            //no Stat for this research topic so just use the graphic provided
                        //if Object != NULL the there must be a IMD so set the object to
                        //equal the Research stat
            //				CurrentStatsShape = Object;
//				CurrentStatsIndex = (SWORD)IMDIndex;
        }
    } else {
        IMDType = IMDTYPE_COMPONENT as libc::c_int as UDWORD;
        //			CurrentStatsShape = NULL;
//			CurrentStatsIndex = -1;
        Object = 0 as *mut libc::c_void;
        CurrentStatsTemplate = 0 as *mut BASE_STATS
    }
    if !Object.is_null() {
        if Image >= 0 as libc::c_int {
            RenderToButton(IntImages, Image as UWORD, Object, Player, Buffer,
                           Down, IMDType, 0 as libc::c_int as UDWORD);
        } else {
            RenderToButton(0 as *mut IMAGEFILE, 0 as libc::c_int as UWORD,
                           Object, Player, Buffer, Down, IMDType,
                           0 as libc::c_int as UDWORD);
        }
    } else if Image >= 0 as libc::c_int {
        RenderImageToButton(IntImages, Image as UWORD, Buffer, Down,
                            0 as libc::c_int as UDWORD);
    } else { RenderBlankToButton(Buffer, Down, 0 as libc::c_int as UDWORD); }
    (*Buffer).Initialised = 1 as libc::c_int;
    CloseButtonRender();
    if Hilight != 0 {
        pie_ImageFileID(IntImages, IMAGE_BUT_HILITE as libc::c_int as UWORD,
                        xOffset.wrapping_add((*Form).x as libc::c_uint) as
                            libc::c_int,
                        yOffset.wrapping_add((*Form).y as libc::c_uint) as
                            libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn RenderToButton(mut ImageFile: *mut IMAGEFILE,
                                        mut ImageID: UWORD,
                                        mut Object: *mut libc::c_void,
                                        mut Player: UDWORD,
                                        mut Buffer: *mut RENDERED_BUTTON,
                                        mut Down: BOOL, mut IMDType: UDWORD,
                                        mut buttonType: UDWORD) {
    CreateIMDButton(ImageFile, ImageID, Object, Player, Buffer, Down, IMDType,
                    buttonType);
}
#[no_mangle]
pub unsafe extern "C" fn RenderImageToButton(mut ImageFile: *mut IMAGEFILE,
                                             mut ImageID: UWORD,
                                             mut Buffer: *mut RENDERED_BUTTON,
                                             mut Down: BOOL,
                                             mut buttonType: UDWORD) {
    CreateImageButton(ImageFile, ImageID, Buffer, Down, buttonType);
}
#[no_mangle]
pub unsafe extern "C" fn RenderBlankToButton(mut Buffer: *mut RENDERED_BUTTON,
                                             mut Down: BOOL,
                                             mut buttonType: UDWORD) {
    CreateBlankButton(Buffer, Down, buttonType);
}
#[no_mangle]
pub unsafe extern "C" fn AdjustTabFormSize(mut Form: *mut W_TABFORM,
                                           mut x0: *mut UDWORD,
                                           mut y0: *mut UDWORD,
                                           mut x1: *mut UDWORD,
                                           mut y1: *mut UDWORD) {
    //BLANK button for now - AB 9/1/98
    /* Adjust for where the tabs are */
    if (*Form).majorPos as libc::c_int == 2 as libc::c_int {
        *x0 =
            (*x0 as
                 libc::c_uint).wrapping_add(((*Form).tabMajorThickness as
                                                 libc::c_int -
                                                 (*Form).tabHorzOffset as
                                                     libc::c_int) as
                                                libc::c_uint) as UDWORD as
                UDWORD
    } else if (*Form).minorPos as libc::c_int == 2 as libc::c_int {
        *x0 =
            (*x0 as
                 libc::c_uint).wrapping_add(((*Form).tabMinorThickness as
                                                 libc::c_int -
                                                 (*Form).tabHorzOffset as
                                                     libc::c_int) as
                                                libc::c_uint) as UDWORD as
                UDWORD
    }
    if (*Form).majorPos as libc::c_int == 3 as libc::c_int {
        *x1 =
            (*x1 as
                 libc::c_uint).wrapping_sub(((*Form).tabMajorThickness as
                                                 libc::c_int -
                                                 (*Form).tabHorzOffset as
                                                     libc::c_int) as
                                                libc::c_uint) as UDWORD as
                UDWORD
    } else if (*Form).minorPos as libc::c_int == 3 as libc::c_int {
        *x1 =
            (*x1 as
                 libc::c_uint).wrapping_sub(((*Form).tabMinorThickness as
                                                 libc::c_int -
                                                 (*Form).tabHorzOffset as
                                                     libc::c_int) as
                                                libc::c_uint) as UDWORD as
                UDWORD
    }
    if (*Form).majorPos as libc::c_int == 1 as libc::c_int {
        *y0 =
            (*y0 as
                 libc::c_uint).wrapping_add(((*Form).tabMajorThickness as
                                                 libc::c_int -
                                                 (*Form).tabVertOffset as
                                                     libc::c_int) as
                                                libc::c_uint) as UDWORD as
                UDWORD
    } else if (*Form).minorPos as libc::c_int == 1 as libc::c_int {
        *y0 =
            (*y0 as
                 libc::c_uint).wrapping_add(((*Form).tabMinorThickness as
                                                 libc::c_int -
                                                 (*Form).tabVertOffset as
                                                     libc::c_int) as
                                                libc::c_uint) as UDWORD as
                UDWORD
    }
    if (*Form).majorPos as libc::c_int == 4 as libc::c_int {
        *y1 =
            (*y1 as
                 libc::c_uint).wrapping_sub(((*Form).tabMajorThickness as
                                                 libc::c_int -
                                                 (*Form).tabVertOffset as
                                                     libc::c_int) as
                                                libc::c_uint) as UDWORD as
                UDWORD
    } else if (*Form).minorPos as libc::c_int == 4 as libc::c_int {
        *y1 =
            (*y1 as
                 libc::c_uint).wrapping_sub(((*Form).tabMinorThickness as
                                                 libc::c_int -
                                                 (*Form).tabVertOffset as
                                                     libc::c_int) as
                                                libc::c_uint) as UDWORD as
                UDWORD
    };
}
#[no_mangle]
pub unsafe extern "C" fn intDisplayObjectForm(mut psWidget: *mut _widget,
                                              mut xOffset: UDWORD,
                                              mut yOffset: UDWORD,
                                              mut pColours: *mut UDWORD) {
    //	W_TABFORM *Form = (W_TABFORM*)psWidget;
//	UDWORD x0,y0,x1,y1;
//
//	x0 = xOffset+Form->x;
//	y0 = yOffset+Form->y;
//	x1 = x0 + Form->width;
//	y1 = y0 + Form->height;
//
//	AdjustTabFormSize(Form,&x0,&y0,&x1,&y1);
//
//	RenderWindowFrame(&FrameObject,x0,y0,x1-x0,y1-y0);
}
// Widget callback function to do the open form animation. Doesn't just open Plain Forms!!
//
#[no_mangle]
pub unsafe extern "C" fn intOpenPlainForm(mut psWidget: *mut _widget,
                                          mut xOffset: UDWORD,
                                          mut yOffset: UDWORD,
                                          mut pColours: *mut UDWORD) {
    let mut Form: *mut W_TABFORM = psWidget as *mut W_TABFORM;
    let mut Tx0: UDWORD = 0;
    let mut Ty0: UDWORD = 0;
    let mut Tx1: UDWORD = 0;
    let mut Ty1: UDWORD = 0;
    let mut Range: UDWORD = 0;
    let mut Duration: UDWORD = 0;
    let mut APos: UDWORD = 0;
    let mut Ay0: SDWORD = 0;
    let mut Ay1: SDWORD = 0;
    Tx0 = xOffset.wrapping_add((*Form).x as libc::c_uint);
    Ty0 = yOffset.wrapping_add((*Form).y as libc::c_uint);
    Tx1 = Tx0.wrapping_add((*Form).width as libc::c_uint);
    Ty1 = Ty0.wrapping_add((*Form).height as libc::c_uint);
    if (*Form).animCount == 0 as libc::c_int as libc::c_uint {
        if FormOpenAudioID >= 0 as libc::c_int &&
               FormOpenCount == 0 as libc::c_int {
            audio_PlayTrack(FormOpenAudioID);
            FormOpenCount += 1
        }
        (*Form).Ax0 = Tx0 as UWORD;
        (*Form).Ax1 = Tx1 as UWORD;
        (*Form).Ay0 =
            Ty0.wrapping_add(((*Form).height as libc::c_int /
                                  2 as libc::c_int) as
                                 libc::c_uint).wrapping_sub(4 as libc::c_int
                                                                as
                                                                libc::c_uint)
                as UWORD;
        (*Form).Ay1 =
            Ty0.wrapping_add(((*Form).height as libc::c_int /
                                  2 as libc::c_int) as
                                 libc::c_uint).wrapping_add(4 as libc::c_int
                                                                as
                                                                libc::c_uint)
                as UWORD;
        (*Form).startTime = gameTime2
    } else { FormOpenCount = 0 as libc::c_int }
    RenderWindowFrame(&mut FrameNormal, (*Form).Ax0 as UDWORD,
                      (*Form).Ay0 as UDWORD,
                      ((*Form).Ax1 as libc::c_int -
                           (*Form).Ax0 as libc::c_int) as UDWORD,
                      ((*Form).Ay1 as libc::c_int -
                           (*Form).Ay0 as libc::c_int) as UDWORD);
    (*Form).animCount = (*Form).animCount.wrapping_add(1);
    Range =
        ((*Form).height as libc::c_int / 2 as libc::c_int - 4 as libc::c_int)
            as UDWORD;
    Duration = gameTime2.wrapping_sub((*Form).startTime) << 16 as libc::c_int;
    APos =
        Range.wrapping_mul(Duration.wrapping_div((1000 as libc::c_int /
                                                      6 as libc::c_int) as
                                                     libc::c_uint)) >>
            16 as libc::c_int;
    Ay0 =
        Ty0.wrapping_add(((*Form).height as libc::c_int / 2 as libc::c_int) as
                             libc::c_uint).wrapping_sub(4 as libc::c_int as
                                                            libc::c_uint).wrapping_sub(APos)
            as SDWORD;
    Ay1 =
        Ty0.wrapping_add(((*Form).height as libc::c_int / 2 as libc::c_int) as
                             libc::c_uint).wrapping_add(4 as libc::c_int as
                                                            libc::c_uint).wrapping_add(APos)
            as SDWORD;
    if Ay0 <= Ty0 as SDWORD { Ay0 = Ty0 as SDWORD }
    if Ay1 >= Ty1 as SDWORD { Ay1 = Ty1 as SDWORD }
    (*Form).Ay0 = Ay0 as UWORD;
    (*Form).Ay1 = Ay1 as UWORD;
    if (*Form).Ay0 as libc::c_uint == Ty0 &&
           (*Form).Ay1 as libc::c_uint == Ty1 {
        if !(*Form).pUserData.is_null() {
            (*Form).display =
                ::std::mem::transmute::<libc::intptr_t,
                                        WIDGET_DISPLAY>((*Form).pUserData as
                                                            SDWORD as
                                                            libc::intptr_t)
        } else {
            //default to display
            (*Form).display =
                Some(intDisplayPlainForm as
                         unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                              _: UDWORD, _: *mut UDWORD)
                             -> ())
        }
        (*Form).disableChildren = 0 as libc::c_int;
        (*Form).animCount = 0 as libc::c_int as UDWORD
    };
}
// Widget callback function to do the close form animation.
//
#[no_mangle]
pub unsafe extern "C" fn intClosePlainForm(mut psWidget: *mut _widget,
                                           mut xOffset: UDWORD,
                                           mut yOffset: UDWORD,
                                           mut pColours: *mut UDWORD) {
    let mut Form: *mut W_TABFORM = psWidget as *mut W_TABFORM;
    let mut Tx0: UDWORD = 0;
    let mut Ty0: UDWORD = 0;
    let mut Tx1: UDWORD = 0;
    let mut Ty1: UDWORD = 0;
    let mut Range: UDWORD = 0;
    let mut Duration: UDWORD = 0;
    let mut APos: UDWORD = 0;
    Tx0 = xOffset.wrapping_add((*Form).x as libc::c_uint);
    Tx1 = Tx0.wrapping_add((*Form).width as libc::c_uint);
    Ty0 =
        yOffset.wrapping_add((*Form).y as
                                 libc::c_uint).wrapping_add(((*Form).height as
                                                                 libc::c_int /
                                                                 2 as
                                                                     libc::c_int)
                                                                as
                                                                libc::c_uint).wrapping_sub(4
                                                                                               as
                                                                                               libc::c_int
                                                                                               as
                                                                                               libc::c_uint);
    Ty1 =
        yOffset.wrapping_add((*Form).y as
                                 libc::c_uint).wrapping_add(((*Form).height as
                                                                 libc::c_int /
                                                                 2 as
                                                                     libc::c_int)
                                                                as
                                                                libc::c_uint).wrapping_add(4
                                                                                               as
                                                                                               libc::c_int
                                                                                               as
                                                                                               libc::c_uint);
    if (*Form).animCount == 0 as libc::c_int as libc::c_uint {
        if FormCloseAudioID >= 0 as libc::c_int &&
               FormCloseCount == 0 as libc::c_int {
            audio_PlayTrack(FormCloseAudioID);
            FormCloseCount += 1
        }
        (*Form).Ax0 =
            xOffset.wrapping_add((*Form).x as libc::c_uint) as UWORD;
        (*Form).Ay0 =
            yOffset.wrapping_add((*Form).y as libc::c_uint) as UWORD;
        (*Form).Ax1 =
            ((*Form).Ax0 as libc::c_int + (*Form).width as libc::c_int) as
                UWORD;
        (*Form).Ay1 =
            ((*Form).Ay0 as libc::c_int + (*Form).height as libc::c_int) as
                UWORD;
        (*Form).startTime = gameTime2
    } else { FormCloseCount = 0 as libc::c_int }
    RenderWindowFrame(&mut FrameNormal, (*Form).Ax0 as UDWORD,
                      (*Form).Ay0 as UDWORD,
                      ((*Form).Ax1 as libc::c_int -
                           (*Form).Ax0 as libc::c_int) as UDWORD,
                      ((*Form).Ay1 as libc::c_int -
                           (*Form).Ay0 as libc::c_int) as UDWORD);
    (*Form).animCount = (*Form).animCount.wrapping_add(1);
    Range =
        ((*Form).height as libc::c_int / 2 as libc::c_int - 4 as libc::c_int)
            as UDWORD;
    Duration = gameTime2.wrapping_sub((*Form).startTime) << 16 as libc::c_int;
    APos =
        Range.wrapping_mul(Duration.wrapping_div((1000 as libc::c_int /
                                                      6 as libc::c_int) as
                                                     libc::c_uint)) >>
            16 as libc::c_int;
    (*Form).Ay0 =
        yOffset.wrapping_add((*Form).y as libc::c_uint).wrapping_add(APos) as
            UWORD;
    (*Form).Ay1 =
        yOffset.wrapping_add((*Form).y as
                                 libc::c_uint).wrapping_add((*Form).height as
                                                                libc::c_uint).wrapping_sub(APos)
            as UWORD;
    if (*Form).Ay0 as libc::c_uint >= Ty0 { (*Form).Ay0 = Ty0 as UWORD }
    if (*Form).Ay1 as libc::c_uint <= Ty1 { (*Form).Ay1 = Ty1 as UWORD }
    if (*Form).Ay0 as libc::c_uint == Ty0 &&
           (*Form).Ay1 as libc::c_uint == Ty1 {
        (*Form).pUserData = 1 as libc::c_int as *mut libc::c_void;
        (*Form).animCount = 0 as libc::c_int as UDWORD
    };
}
#[no_mangle]
pub unsafe extern "C" fn intDisplayPlainForm(mut psWidget: *mut _widget,
                                             mut xOffset: UDWORD,
                                             mut yOffset: UDWORD,
                                             mut pColours: *mut UDWORD) {
    let mut Form: *mut W_TABFORM = psWidget as *mut W_TABFORM;
    let mut x0: UDWORD = 0;
    let mut y0: UDWORD = 0;
    let mut x1: UDWORD = 0;
    let mut y1: UDWORD = 0;
    x0 = xOffset.wrapping_add((*Form).x as libc::c_uint);
    y0 = yOffset.wrapping_add((*Form).y as libc::c_uint);
    x1 = x0.wrapping_add((*Form).width as libc::c_uint);
    y1 = y0.wrapping_add((*Form).height as libc::c_uint);
    RenderWindowFrame(&mut FrameNormal, x0, y0, x1.wrapping_sub(x0),
                      y1.wrapping_sub(y0));
}
#[no_mangle]
pub unsafe extern "C" fn intDisplayStatsForm(mut psWidget: *mut _widget,
                                             mut xOffset: UDWORD,
                                             mut yOffset: UDWORD,
                                             mut pColours: *mut UDWORD) {
    let mut Form: *mut W_TABFORM = psWidget as *mut W_TABFORM;
    let mut x0: UDWORD = 0;
    let mut y0: UDWORD = 0;
    let mut x1: UDWORD = 0;
    let mut y1: UDWORD = 0;
    x0 = xOffset.wrapping_add((*Form).x as libc::c_uint);
    y0 = yOffset.wrapping_add((*Form).y as libc::c_uint);
    x1 = x0.wrapping_add((*Form).width as libc::c_uint);
    y1 = y0.wrapping_add((*Form).height as libc::c_uint);
    AdjustTabFormSize(Form, &mut x0, &mut y0, &mut x1, &mut y1);
    RenderWindowFrame(&mut FrameNormal, x0, y0, x1.wrapping_sub(x0),
                      y1.wrapping_sub(y0));
}
// Display an image for a widget.
//
#[no_mangle]
pub unsafe extern "C" fn intDisplayImage(mut psWidget: *mut _widget,
                                         mut xOffset: UDWORD,
                                         mut yOffset: UDWORD,
                                         mut pColours: *mut UDWORD) {
    let mut x: UDWORD = xOffset.wrapping_add((*psWidget).x as libc::c_uint);
    let mut y: UDWORD = yOffset.wrapping_add((*psWidget).y as libc::c_uint);
    pie_ImageFileID(IntImages, (*psWidget).pUserData as UDWORD as UWORD,
                    x as libc::c_int, y as libc::c_int);
}
//draws the mission clock - flashes when below a predefined time
#[no_mangle]
pub unsafe extern "C" fn intDisplayMissionClock(mut psWidget: *mut _widget,
                                                mut xOffset: UDWORD,
                                                mut yOffset: UDWORD,
                                                mut pColours: *mut UDWORD) {
    let mut x: UDWORD = xOffset.wrapping_add((*psWidget).x as libc::c_uint);
    let mut y: UDWORD = yOffset.wrapping_add((*psWidget).y as libc::c_uint);
    let mut flash: UDWORD = 0;
    //draw the background image
    pie_ImageFileID(IntImages,
                    ((*psWidget).pUserData as UDWORD >> 10 as libc::c_int &
                         0x3ff as libc::c_int as libc::c_uint) as UWORD,
                    x as libc::c_int, y as libc::c_int);
    //need to flash the timer when < 5 minutes remaining, but > 4 minutes
    flash =
        (*psWidget).pUserData as UDWORD >> 20 as libc::c_int &
            0x3ff as libc::c_int as libc::c_uint;
    if flash != 0 &&
           gameTime2.wrapping_div(250 as libc::c_int as
                                      libc::c_uint).wrapping_rem(2 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint)
               == 0 as libc::c_int as libc::c_uint {
        pie_ImageFileID(IntImages,
                        ((*psWidget).pUserData as UDWORD &
                             0x3ff as libc::c_int as libc::c_uint) as UWORD,
                        x as libc::c_int, y as libc::c_int);
    };
}
// Display one of two images depending on if the widget is hilighted by the mouse.
//
#[no_mangle]
pub unsafe extern "C" fn intDisplayImageHilight(mut psWidget: *mut _widget,
                                                mut xOffset: UDWORD,
                                                mut yOffset: UDWORD,
                                                mut pColours: *mut UDWORD) {
    let mut x: UDWORD = xOffset.wrapping_add((*psWidget).x as libc::c_uint);
    let mut y: UDWORD = yOffset.wrapping_add((*psWidget).y as libc::c_uint);
    let mut flash: UDWORD = 0;
    let mut ImageID: UWORD = 0;
    let mut Hilight: BOOL = 0 as libc::c_int;
    match (*psWidget).type_0 as libc::c_uint {
        0 => {
            Hilight =
                ((*(psWidget as *mut W_CLICKFORM)).state &
                     0x4 as libc::c_int as libc::c_uint) as BOOL
        }
        2 => {
            Hilight =
                ((*(psWidget as *mut W_BUTTON)).state &
                     0x4 as libc::c_int as libc::c_uint) as BOOL
        }
        3 => {
            if (*(psWidget as *mut W_EDITBOX)).state &
                   0x10 as libc::c_int as libc::c_uint != 0 {
                Hilight = 1 as libc::c_int
            }
        }
        5 => {
            if (*(psWidget as *mut W_SLIDER)).state as libc::c_int &
                   0x2 as libc::c_int != 0 {
                Hilight = 1 as libc::c_int
            }
        }
        _ => { Hilight = 0 as libc::c_int }
    }
    ImageID =
        ((*psWidget).pUserData as UDWORD &
             0x3ff as libc::c_int as libc::c_uint) as UWORD;
    //need to flash the button if Full Transporter
    flash =
        (*psWidget).pUserData as UDWORD >> 20 as libc::c_int &
            0x3ff as libc::c_int as libc::c_uint;
    if flash != 0 && (*psWidget).id == 9010 as libc::c_int as libc::c_uint {
        if gameTime2.wrapping_div(250 as libc::c_int as
                                      libc::c_uint).wrapping_rem(2 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint)
               == 0 as libc::c_int as libc::c_uint {
            pie_ImageFileID(IntImages,
                            ((*psWidget).pUserData as UDWORD >>
                                 10 as libc::c_int &
                                 0x3ff as libc::c_int as libc::c_uint) as
                                UWORD, x as libc::c_int, y as libc::c_int);
        } else {
            pie_ImageFileID(IntImages, ImageID, x as libc::c_int,
                            y as libc::c_int);
        }
    } else {
        pie_ImageFileID(IntImages, ImageID, x as libc::c_int,
                        y as libc::c_int);
        if Hilight != 0 {
            pie_ImageFileID(IntImages,
                            ((*psWidget).pUserData as UDWORD >>
                                 10 as libc::c_int &
                                 0x3ff as libc::c_int as libc::c_uint) as
                                UWORD, x as libc::c_int, y as libc::c_int);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn GetButtonState(mut psWidget: *mut _widget,
                                        mut Hilight: *mut BOOL,
                                        mut Down: *mut UDWORD,
                                        mut Grey: *mut BOOL) {
    match (*psWidget).type_0 as libc::c_uint {
        0 => {
            *Hilight =
                ((*(psWidget as *mut W_CLICKFORM)).state &
                     0x4 as libc::c_int as libc::c_uint) as BOOL;
            //			if( ((W_CLICKFORM*)psWidget)->state & WCLICK_HILITE) {
//				Hilight = TRUE;
//			}
            if (*(psWidget as *mut W_CLICKFORM)).state &
                   (0x1 as libc::c_int | 0x8 as libc::c_int |
                        0x10 as libc::c_int) as libc::c_uint != 0 {
                *Down = 1 as libc::c_int as UDWORD
            }
            if (*(psWidget as *mut W_CLICKFORM)).state &
                   0x2 as libc::c_int as libc::c_uint != 0 {
                *Grey = 1 as libc::c_int
            }
        }
        2 => {
            *Hilight =
                ((*(psWidget as *mut W_BUTTON)).state &
                     0x4 as libc::c_int as libc::c_uint) as BOOL;
            //			if( ((W_BUTTON*)psWidget)->state & WBUTS_HILITE) {
//				*Hilight = TRUE;
//			}
            if (*(psWidget as *mut W_BUTTON)).state &
                   (0x1 as libc::c_int | 0x8 as libc::c_int |
                        0x10 as libc::c_int) as libc::c_uint != 0 {
                *Down = 1 as libc::c_int as UDWORD
            }
            if (*(psWidget as *mut W_BUTTON)).state &
                   0x2 as libc::c_int as libc::c_uint != 0 {
                *Grey = 1 as libc::c_int
            }
        }
        3 => {
            if (*(psWidget as *mut W_EDITBOX)).state &
                   0x10 as libc::c_int as libc::c_uint != 0 {
                *Hilight = 1 as libc::c_int
            }
        }
        5 => {
            if (*(psWidget as *mut W_SLIDER)).state as libc::c_int &
                   0x2 as libc::c_int != 0 {
                *Hilight = 1 as libc::c_int
            }
            if (*(psWidget as *mut W_SLIDER)).state as libc::c_int &
                   (0x1 as libc::c_int | 0x8 as libc::c_int |
                        0x10 as libc::c_int) != 0 {
                *Down = 1 as libc::c_int as UDWORD
            }
        }
        _ => { *Hilight = 0 as libc::c_int }
    };
}
// Display one of two images depending on if the widget is hilighted by the mouse.
//
#[no_mangle]
pub unsafe extern "C" fn intDisplayButtonHilight(mut psWidget: *mut _widget,
                                                 mut xOffset: UDWORD,
                                                 mut yOffset: UDWORD,
                                                 mut pColours: *mut UDWORD) {
    let mut x: UDWORD = xOffset.wrapping_add((*psWidget).x as libc::c_uint);
    let mut y: UDWORD = yOffset.wrapping_add((*psWidget).y as libc::c_uint);
    let mut Hilight: BOOL = 0 as libc::c_int;
    let mut Grey: BOOL = 0 as libc::c_int;
    let mut Down: UDWORD = 0 as libc::c_int as UDWORD;
    let mut ImageID: UWORD = 0;
    GetButtonState(psWidget, &mut Hilight, &mut Down, &mut Grey);
    //	switch(psWidget->type) {
//		case WIDG_FORM:
//			if( ((W_CLICKFORM*)psWidget)->state & WCLICK_HILITE) {
//				Hilight = TRUE;
//			}
//			if( ((W_CLICKFORM*)psWidget)->state & (WCLICK_DOWN | WCLICK_LOCKED | WCLICK_CLICKLOCK)) {
//				Down = 1;
//			}
//			if( ((W_CLICKFORM*)psWidget)->state & WCLICK_GREY) {
//				Grey = 1;
//			}
//			break;
//
//		case WIDG_BUTTON:
//			if( ((W_BUTTON*)psWidget)->state & WBUTS_HILITE) {
//				Hilight = TRUE;
//			}
//			if( ((W_BUTTON*)psWidget)->state & (WBUTS_DOWN | WBUTS_LOCKED | WBUTS_CLICKLOCK)) {
//				Down = 1;
//			}
//			if( ((W_BUTTON*)psWidget)->state & WBUTS_GREY) {
//				Grey = 1;
//			}
//			break;
//
//		case WIDG_EDITBOX:
//			if( ((W_EDITBOX*)psWidget)->state & WEDBS_HILITE) {
//				Hilight = TRUE;
//			}
//			break;
//
//		case WIDG_SLIDER:
//			if( ((W_SLIDER*)psWidget)->state & SLD_HILITE) {
//				Hilight = TRUE;
//			}
//			if( ((W_SLIDER*)psWidget)->state & (WCLICK_DOWN | WCLICK_LOCKED | WCLICK_CLICKLOCK)) {
//				Down = 1;
//			}
//			break;
//
//		default:
//			Hilight = FALSE;
//	}
    if Grey != 0 {
        ImageID =
            ((*psWidget).pUserData as UDWORD >> 20 as libc::c_int &
                 0x3ff as libc::c_int as libc::c_uint) as UWORD;
        Hilight = 0 as libc::c_int
    } else {
        ImageID =
            ((*psWidget).pUserData as UDWORD &
                 0x3ff as libc::c_int as libc::c_uint).wrapping_add(Down) as
                UWORD
    }
    pie_ImageFileID(IntImages, ImageID, x as libc::c_int, y as libc::c_int);
    if Hilight != 0 {
        pie_ImageFileID(IntImages,
                        ((*psWidget).pUserData as UDWORD >> 10 as libc::c_int
                             & 0x3ff as libc::c_int as libc::c_uint) as UWORD,
                        x as libc::c_int, y as libc::c_int);
    };
}
// Display one of two images depending on if the widget is hilighted by the mouse.
//
#[no_mangle]
pub unsafe extern "C" fn intDisplayAltButtonHilight(mut psWidget:
                                                        *mut _widget,
                                                    mut xOffset: UDWORD,
                                                    mut yOffset: UDWORD,
                                                    mut pColours:
                                                        *mut UDWORD) {
    let mut x: UDWORD = xOffset.wrapping_add((*psWidget).x as libc::c_uint);
    let mut y: UDWORD = yOffset.wrapping_add((*psWidget).y as libc::c_uint);
    let mut Hilight: BOOL = 0 as libc::c_int;
    let mut Grey: BOOL = 0 as libc::c_int;
    let mut Down: UDWORD = 0 as libc::c_int as UDWORD;
    let mut ImageID: UWORD = 0;
    GetButtonState(psWidget, &mut Hilight, &mut Down, &mut Grey);
    if Grey != 0 {
        ImageID =
            ((*psWidget).pUserData as UDWORD >> 20 as libc::c_int &
                 0x3ff as libc::c_int as libc::c_uint) as UWORD;
        Hilight = 0 as libc::c_int
    } else {
        ImageID =
            ((*psWidget).pUserData as UDWORD &
                 0x3ff as libc::c_int as libc::c_uint).wrapping_add(Down) as
                UWORD
    }
    pie_ImageFileID(IntImages, ImageID, x as libc::c_int, y as libc::c_int);
    if Hilight != 0 {
        pie_ImageFileID(IntImages,
                        ((*psWidget).pUserData as UDWORD >> 10 as libc::c_int
                             & 0x3ff as libc::c_int as libc::c_uint) as UWORD,
                        x as libc::c_int, y as libc::c_int);
    };
}
// Flash one of two images depending on if the widget is hilighted by the mouse.
//
#[no_mangle]
pub unsafe extern "C" fn intDisplayButtonFlash(mut psWidget: *mut _widget,
                                               mut xOffset: UDWORD,
                                               mut yOffset: UDWORD,
                                               mut pColours: *mut UDWORD) {
    let mut x: UDWORD = xOffset.wrapping_add((*psWidget).x as libc::c_uint);
    let mut y: UDWORD = yOffset.wrapping_add((*psWidget).y as libc::c_uint);
    let mut Hilight: BOOL = 0 as libc::c_int;
    let mut Down: UDWORD = 0 as libc::c_int as UDWORD;
    let mut ImageID: UWORD = 0;
    if (*psWidget).type_0 as libc::c_uint ==
           WIDG_BUTTON as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"intDisplayButtonFlash : Not a button\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (*psWidget).type_0 as libc::c_uint ==
           WIDG_BUTTON as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"intdisplay.c\x00" as *const u8 as *const libc::c_char,
              1829 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 22],
                                        &[libc::c_char; 22]>(b"intDisplayButtonFlash\x00")).as_ptr(),
              b"psWidget->type == WIDG_BUTTON\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (*(psWidget as *mut W_BUTTON)).state &
           0x4 as libc::c_int as libc::c_uint != 0 {
        Hilight = 1 as libc::c_int
    }
    if (*(psWidget as *mut W_BUTTON)).state &
           (0x1 as libc::c_int | 0x8 as libc::c_int | 0x10 as libc::c_int) as
               libc::c_uint != 0 {
        Down = 1 as libc::c_int as UDWORD
    }
    if Down != 0 &&
           gameTime2.wrapping_div(250 as libc::c_int as
                                      libc::c_uint).wrapping_rem(2 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint)
               == 0 as libc::c_int as libc::c_uint {
        ImageID =
            ((*psWidget).pUserData as UDWORD >> 10 as libc::c_int &
                 0x3ff as libc::c_int as libc::c_uint) as UWORD;
        Hilight = 0 as libc::c_int
    } else {
        ImageID =
            ((*psWidget).pUserData as UDWORD &
                 0x3ff as libc::c_int as libc::c_uint) as UWORD
    }
    pie_ImageFileID(IntImages, ImageID, x as libc::c_int, y as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn intDisplayReticuleButton(mut psWidget: *mut _widget,
                                                  mut xOffset: UDWORD,
                                                  mut yOffset: UDWORD,
                                                  mut pColours: *mut UDWORD) {
    let mut x: UDWORD = xOffset.wrapping_add((*psWidget).x as libc::c_uint);
    let mut y: UDWORD = yOffset.wrapping_add((*psWidget).y as libc::c_uint);
    let mut Hilight: BOOL = 0 as libc::c_int;
    let mut Down: BOOL = 0 as libc::c_int;
    let mut DownTime: UBYTE =
        ((*psWidget).pUserData as UDWORD >> 8 as libc::c_int &
             0xff as libc::c_int as libc::c_uint) as UBYTE;
    let mut Index: UBYTE =
        ((*psWidget).pUserData as UDWORD &
             0xff as libc::c_int as libc::c_uint) as UBYTE;
    let mut flashing: UBYTE =
        ((*psWidget).pUserData as UDWORD >> 24 as libc::c_int &
             0xff as libc::c_int as libc::c_uint) as UBYTE;
    let mut flashTime: UBYTE =
        ((*psWidget).pUserData as UDWORD >> 16 as libc::c_int &
             0xff as libc::c_int as libc::c_uint) as UBYTE;
    let mut ImageID: UWORD = 0;
    if (*psWidget).type_0 as libc::c_uint ==
           WIDG_BUTTON as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"intDisplayReticuleButton : Not a button\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (*psWidget).type_0 as libc::c_uint ==
           WIDG_BUTTON as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"intdisplay.c\x00" as *const u8 as *const libc::c_char,
              1869 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 25],
                                        &[libc::c_char; 25]>(b"intDisplayReticuleButton\x00")).as_ptr(),
              b"psWidget->type == WIDG_BUTTON\x00" as *const u8 as
                  *const libc::c_char);
    };
    //	iV_DrawTransImage(IntImages,ImageID,x,y);
    if (*(psWidget as *mut W_BUTTON)).state &
           0x2 as libc::c_int as libc::c_uint != 0 {
        pie_ImageFileID(IntImages,
                        IMAGE_RETICULE_GREY as libc::c_int as UWORD,
                        x as libc::c_int, y as libc::c_int);
        return
    }
    Down =
        ((*(psWidget as *mut W_BUTTON)).state &
             (0x1 as libc::c_int | 0x10 as libc::c_int) as libc::c_uint) as
            BOOL;
    //	Hilight = ((W_BUTTON*)psWidget)->state & WBUTS_HILITE;
    Hilight =
        ((*(psWidget as *mut W_BUTTON)).state &
             0x4 as libc::c_int as libc::c_uint) as BOOL;
    if Down != 0 {
        if (DownTime as libc::c_int) < 1 as libc::c_int &&
               Index as libc::c_int != IMAGE_CANCEL_UP as libc::c_int {
            ImageID = IMAGE_RETICULE_BUTDOWN as libc::c_int as UWORD
            // Do the button flash.
        } else {
            ImageID = (Index as libc::c_int + 1 as libc::c_int) as UWORD
            // It's down.
        }
        DownTime = DownTime.wrapping_add(1);
        //stop the reticule from flashing if it was
        flashing = 0 as libc::c_int as UBYTE
    } else if flashing != 0 {
        //flashing button?
        //			if (flashTime < 2)
        if gameTime.wrapping_div(250 as libc::c_int as
                                     libc::c_uint).wrapping_rem(2 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uint)
               == 0 as libc::c_int as libc::c_uint {
            ImageID = Index as UWORD
            //IMAGE_RETICULE_BUTDOWN;//a step in the right direction JPS 27-4-98
        } else {
            ImageID = (Index as libc::c_int + 1 as libc::c_int) as UWORD;
            flashTime = 0 as libc::c_int as UBYTE
        }
        flashTime = flashTime.wrapping_add(1)
    } else {
        DownTime = 0 as libc::c_int as UBYTE;
        ImageID = Index as UWORD
        // It's up.
    }
    pie_ImageFileID(IntImages, ImageID, x as libc::c_int, y as libc::c_int);
    if Hilight != 0 {
        if Index as libc::c_int == IMAGE_CANCEL_UP as libc::c_int {
            pie_ImageFileID(IntImages,
                            IMAGE_CANCEL_HILIGHT as libc::c_int as UWORD,
                            x as libc::c_int, y as libc::c_int);
        } else {
            pie_ImageFileID(IntImages,
                            IMAGE_RETICULE_HILIGHT as libc::c_int as UWORD,
                            x as libc::c_int, y as libc::c_int);
        }
    }
    (*psWidget).pUserData =
        ((flashTime as libc::c_int & 0xff as libc::c_int) << 24 as libc::c_int
             |
             (flashing as libc::c_int & 0xff as libc::c_int) <<
                 16 as libc::c_int |
             (DownTime as libc::c_int & 0xff as libc::c_int) <<
                 8 as libc::c_int |
             Index as libc::c_int & 0xff as libc::c_int) as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn intDisplayTab(mut psWidget: *mut _widget,
                                       mut TabType: UDWORD,
                                       mut Position: UDWORD,
                                       mut Number: UDWORD, mut Selected: BOOL,
                                       mut Hilight: BOOL, mut x: UDWORD,
                                       mut y: UDWORD, mut Width: UDWORD,
                                       mut Height: UDWORD) {
    let mut Tab: *mut TABDEF = (*psWidget).pUserData as *mut TABDEF;
    //	ASSERT( Number < 4,"intDisplayTab : Too many tabs." );
    //Number represents which tab we are on but not interested since they all look the same now - AB 25/01/99
	/*if(Number > 3) {
		Number = 3;
	}*/
    if TabType == 1 as libc::c_int as libc::c_uint {
        //iV_DrawTransImage(IntImages,(UWORD)(Tab->MajorUp+Number),x,y);
        pie_ImageFileID(IntImages, (*Tab).MajorUp as UWORD, x as libc::c_int,
                        y as libc::c_int);
        if Hilight != 0 {
            pie_ImageFileID(IntImages, (*Tab).MajorHilight as UWORD,
                            x as libc::c_int, y as libc::c_int);
        } else if Selected != 0 {
            pie_ImageFileID(IntImages, (*Tab).MajorSelected as UWORD,
                            x as libc::c_int, y as libc::c_int);
        }
    } else {
        //iV_DrawTransImage(IntImages,(UWORD)(Tab->MinorUp+Number),x,y);
        pie_ImageFileID(IntImages, (*Tab).MinorUp as UWORD, x as libc::c_int,
                        y as libc::c_int);
        if Hilight != 0 {
            pie_ImageFileID(IntImages, (*Tab).MinorHilight as UWORD,
                            x as libc::c_int, y as libc::c_int);
        } else if Selected != 0 {
            pie_ImageFileID(IntImages, (*Tab).MinorSelected as UWORD,
                            x as libc::c_int, y as libc::c_int);
        }
    };
}
//void intDisplaySystemTab(struct _widget *psWidget,UDWORD TabType, UDWORD Position,
//				   UDWORD Number,BOOL Selected,BOOL Hilight,UDWORD x,UDWORD y,UDWORD Width,UDWORD Height)
//{
//	TABDEF *Tab = (TABDEF*)psWidget->pUserData;
//#ifdef PSX
//	UWORD ImageID;
//#endif
//
// //	ASSERT( Number < 4,"intDisplaySystemTab : Too many tabs." );
//
//	Number = Number%4;	// Make sure number never gets bigger than 3.
//
//#ifndef PSX
//	if(TabType == TAB_MAJOR)
//	{
//		iV_DrawTransImage(IntImages,(UWORD)(Tab->MajorUp+Number),x,y);
//
//		if(Hilight)
//		{
//			iV_DrawTransImage(IntImages,Tab->MajorHilight,x,y);
//		}
//		else if(Selected)
//		{
//			iV_DrawTransImage(IntImages,(UWORD)(Tab->MajorSelected+Number),x,y);
//		}
//	}
//	else
//	{
//		//ASSERT( FALSE,"intDisplaySystemTab : NOT CATERED FOR!!!" );
//		iV_DrawTransImage(IntImages,(UWORD)(Tab->MinorUp),x,y);
//
//		if(Hilight)
//		{
//			iV_DrawTransImage(IntImages,Tab->MinorHilight,x,y);
//		}
//		else if(Selected)
//		{
//			iV_DrawTransImage(IntImages,Tab->MinorSelected,x,y);
//		}
//	}
//#else
//	if(TabType == TAB_MAJOR)
//	{
//		if(Hilight)
//		{
//			iV_DrawTransImage(IntImages,Tab->MajorHilight,x,y);
//		}
//		else if(Selected)
//		{
//			iV_DrawTransImage(IntImages,(UWORD)(Tab->MajorSelected+Number),x,y);
//		}
//
//		ImageID = (UWORD)(Tab->MajorUp+Number);
//		iV_DrawTransImage(IntImages,ImageID,x,y);
//	}
//	else
//	{
//		if(Hilight)
//		{
//			iV_DrawTransImage(IntImages,Tab->MinorHilight,x,y);
//		}
//		else if(Selected)
//		{
//			iV_DrawTransImage(IntImages,Tab->MinorSelected,x,y);
//		}
//
//		ImageID = (UWORD)(Tab->MinorUp);
//		iV_DrawTransImage(IntImages,ImageID,x,y);
//	}
//
//	AddCursorSnap(&InterfaceSnap,
//					x+(iV_GetImageXOffset(IntImages,ImageID))+iV_GetImageWidth(IntImages,ImageID)/2,
//					y+(iV_GetImageYOffset(IntImages,ImageID))+iV_GetImageHeight(IntImages,ImageID)/2,
//					psWidget->formID,psWidget->id,NULL);
//#endif
//}
//static void intUpdateSliderCount(struct _widget *psWidget, struct _w_context *psContext)
//{
//	W_SLIDER *Slider = (W_SLIDER*)psWidget;
//	UDWORD Quantity = Slider->pos + 1;
//
//	W_LABEL *Label = (W_LABEL*)widgGetFromID(psWScreen,IDSTAT_SLIDERCOUNT);
//	Label->pUserData = (void*)Quantity;
//}
// Display one of three images depending on if the widget is currently depressed (ah!).
//
#[no_mangle]
pub unsafe extern "C" fn intDisplayButtonPressed(mut psWidget: *mut _widget,
                                                 mut xOffset: UDWORD,
                                                 mut yOffset: UDWORD,
                                                 mut pColours: *mut UDWORD) {
    let mut psButton: *mut W_BUTTON = psWidget as *mut W_BUTTON;
    let mut x: UDWORD = xOffset.wrapping_add((*psButton).x as libc::c_uint);
    let mut y: UDWORD = yOffset.wrapping_add((*psButton).y as libc::c_uint);
    let mut Hilight: UBYTE = 0 as libc::c_int as UBYTE;
    let mut ImageID: UWORD = 0;
    if (*psButton).state &
           (0x1 as libc::c_int | 0x8 as libc::c_int | 0x10 as libc::c_int) as
               libc::c_uint != 0 {
        ImageID =
            ((*psWidget).pUserData as UDWORD >> 20 as libc::c_int &
                 0x3ff as libc::c_int as libc::c_uint) as UWORD
    } else {
        ImageID =
            ((*psWidget).pUserData as UDWORD &
                 0x3ff as libc::c_int as libc::c_uint) as UWORD
    }
    Hilight =
        ((*psButton).state & 0x4 as libc::c_int as libc::c_uint) as UBYTE;
    //	if (psButton->state & WBUTS_HILITE)
//	{
//		Hilight = 1;
//	}
    pie_ImageFileID(IntImages, ImageID, x as libc::c_int, y as libc::c_int);
    if Hilight != 0 {
        pie_ImageFileID(IntImages,
                        ((*psWidget).pUserData as UDWORD >> 10 as libc::c_int
                             & 0x3ff as libc::c_int as libc::c_uint) as UWORD,
                        x as libc::c_int, y as libc::c_int);
    };
}
// Display DP images depending on factory and if the widget is currently depressed
#[no_mangle]
pub unsafe extern "C" fn intDisplayDPButton(mut psWidget: *mut _widget,
                                            mut xOffset: UDWORD,
                                            mut yOffset: UDWORD,
                                            mut pColours: *mut UDWORD) {
    let mut psButton: *mut W_BUTTON = psWidget as *mut W_BUTTON;
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut x: UDWORD = xOffset.wrapping_add((*psButton).x as libc::c_uint);
    let mut y: UDWORD = yOffset.wrapping_add((*psButton).y as libc::c_uint);
    let mut hilight: UBYTE = 0 as libc::c_int as UBYTE;
    let mut down: UBYTE = 0 as libc::c_int as UBYTE;
    let mut imageID: UWORD = 0;
    psStruct = (*psButton).pUserData as *mut STRUCTURE;
    if !psStruct.is_null() {
        if StructIsFactory(psStruct) != 0 {
        } else {
            debug(LOG_ERROR,
                  b"intDisplayDPButton: structure is not a factory\x00" as
                      *const u8 as *const libc::c_char);
        };
        if StructIsFactory(psStruct) != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"intdisplay.c\x00" as *const u8 as *const libc::c_char,
                  2115 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 19],
                                            &[libc::c_char; 19]>(b"intDisplayDPButton\x00")).as_ptr(),
                  b"StructIsFactory(psStruct)\x00" as *const u8 as
                      *const libc::c_char);
        };
        if (*psButton).state &
               (0x1 as libc::c_int | 0x8 as libc::c_int | 0x10 as libc::c_int)
                   as libc::c_uint != 0 {
            down = 1 as libc::c_int as UBYTE
        }
        hilight =
            ((*psButton).state & 0x4 as libc::c_int as libc::c_uint) as UBYTE;
        //		if (psButton->state & WBUTS_HILITE)
//		{
//			hilight = TRUE;
//		}
        match (*(*psStruct).pStructureType).type_0 {
            1 => { imageID = IMAGE_FDP_UP as libc::c_int as UWORD }
            16 => { imageID = IMAGE_CDP_UP as libc::c_int as UWORD }
            17 => { imageID = IMAGE_VDP_UP as libc::c_int as UWORD }
            _ => { return }
        }
        pie_ImageFileID(IntImages, imageID, x as libc::c_int,
                        y as libc::c_int);
        if hilight != 0 {
            imageID = imageID.wrapping_add(1);
            pie_ImageFileID(IntImages, imageID, x as libc::c_int,
                            y as libc::c_int);
        } else if down != 0 {
            imageID = imageID.wrapping_sub(1);
            pie_ImageFileID(IntImages, imageID, x as libc::c_int,
                            y as libc::c_int);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn intDisplaySlider(mut psWidget: *mut _widget,
                                          mut xOffset: UDWORD,
                                          mut yOffset: UDWORD,
                                          mut pColours: *mut UDWORD) {
    let mut Slider: *mut W_SLIDER = psWidget as *mut W_SLIDER;
    let mut x: UDWORD = xOffset.wrapping_add((*psWidget).x as libc::c_uint);
    let mut y: UDWORD = yOffset.wrapping_add((*psWidget).y as libc::c_uint);
    let mut sx: SWORD = 0;
    //SWORD x0,y0, x1;
    pie_ImageFileID(IntImages, IMAGE_SLIDER_BACK as libc::c_int as UWORD,
                    x.wrapping_add(0 as libc::c_int as libc::c_uint) as
                        libc::c_int,
                    y.wrapping_add(0 as libc::c_int as libc::c_uint) as
                        libc::c_int);
    sx =
        (((*Slider).width as libc::c_int - (*Slider).barSize as libc::c_int) *
             (*Slider).pos as libc::c_int / (*Slider).numStops as libc::c_int)
            as SWORD;
    pie_ImageFileID(IntImages, IMAGE_SLIDER_BUT as libc::c_int as UWORD,
                    x.wrapping_add(sx as libc::c_uint) as libc::c_int,
                    y.wrapping_sub(2 as libc::c_int as libc::c_uint) as
                        libc::c_int);
}
/* display highlighted edit box from left, middle and end edit box graphics */
#[no_mangle]
pub unsafe extern "C" fn intDisplayEditBox(mut psWidget: *mut _widget,
                                           mut xOffset: UDWORD,
                                           mut yOffset: UDWORD,
                                           mut pColours: *mut UDWORD) {
    let mut psEditBox: *mut W_EDITBOX = psWidget as *mut W_EDITBOX;
    let mut iImageIDLeft: UWORD = 0;
    let mut iImageIDMid: UWORD = 0;
    let mut iImageIDRight: UWORD = 0;
    let mut iX: UDWORD = 0;
    let mut iY: UDWORD = 0;
    let mut iDX: UDWORD = 0;
    let mut iXRight: UDWORD = 0;
    let mut iXLeft: UDWORD =
        xOffset.wrapping_add((*psWidget).x as libc::c_uint);
    let mut iYLeft: UDWORD =
        yOffset.wrapping_add((*psWidget).y as libc::c_uint);
    if (*psEditBox).state & 0x10 as libc::c_int as libc::c_uint != 0 {
        iImageIDLeft = IMAGE_DES_EDITBOXLEFTH as libc::c_int as UWORD;
        iImageIDMid = IMAGE_DES_EDITBOXMIDH as libc::c_int as UWORD;
        iImageIDRight = IMAGE_DES_EDITBOXRIGHTH as libc::c_int as UWORD
    } else {
        iImageIDLeft = IMAGE_DES_EDITBOXLEFT as libc::c_int as UWORD;
        iImageIDMid = IMAGE_DES_EDITBOXMID as libc::c_int as UWORD;
        iImageIDRight = IMAGE_DES_EDITBOXRIGHT as libc::c_int as UWORD
    }
    /* draw left side of bar */
    iX = iXLeft;
    iY = iYLeft;
    pie_ImageFileID(IntImages, iImageIDLeft, iX as libc::c_int,
                    iY as libc::c_int);
    /* draw middle of bar */
    iX =
        (iX as
             libc::c_uint).wrapping_add(iV_GetImageWidth(IntImages,
                                                         iImageIDLeft) as
                                            libc::c_uint) as UDWORD as UDWORD;
    iDX = iV_GetImageWidth(IntImages, iImageIDMid) as UDWORD;
    iXRight =
        xOffset.wrapping_add((*psWidget).width as
                                 libc::c_uint).wrapping_sub(iV_GetImageWidth(IntImages,
                                                                             iImageIDRight)
                                                                as
                                                                libc::c_uint);
    while iX < iXRight {
        pie_ImageFileID(IntImages, iImageIDMid, iX as libc::c_int,
                        iY as libc::c_int);
        iX = (iX as libc::c_uint).wrapping_add(iDX) as UDWORD as UDWORD
    }
    /* draw right side of bar */
    pie_ImageFileID(IntImages, iImageIDRight, iXRight as libc::c_int,
                    iY as libc::c_int); // = (UDWORD)Label->pUserData;
}
#[no_mangle]
pub unsafe extern "C" fn intDisplayNumber(mut psWidget: *mut _widget,
                                          mut xOffset: UDWORD,
                                          mut yOffset: UDWORD,
                                          mut pColours: *mut UDWORD) {
    let mut Label: *mut W_LABEL = psWidget as *mut W_LABEL;
    let mut i: UDWORD = 0 as libc::c_int as UDWORD;
    let mut x: UDWORD = ((*Label).x as libc::c_uint).wrapping_add(xOffset);
    let mut y: UDWORD = ((*Label).y as libc::c_uint).wrapping_add(yOffset);
    let mut Quantity: UDWORD = 0;
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psFactory: *mut FACTORY = 0 as *mut FACTORY;
    //Quantity depends on the factory
    Quantity = 1 as libc::c_int as UDWORD;
    if !(*Label).pUserData.is_null() {
        psStruct = (*Label).pUserData as *mut STRUCTURE;
        psFactory = (*psStruct).pFunctionality as *mut FACTORY;
        /*else
		{
			Quantity = 1;
		}*/
        Quantity = (*psFactory).quantity as UDWORD
    }
    if Quantity >= 10 as libc::c_int as libc::c_uint {
        pie_ImageFileID(IntImages,
                        IMAGE_SLIDER_INFINITY as libc::c_int as UWORD,
                        x.wrapping_add(4 as libc::c_int as libc::c_uint) as
                            libc::c_int, y as libc::c_int);
    } else {
        (*Label).aText[0 as libc::c_int as usize] =
            ('0' as i32 as
                 libc::c_uint).wrapping_add(Quantity.wrapping_div(10 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint))
                as UBYTE as STRING;
        (*Label).aText[1 as libc::c_int as usize] =
            ('0' as i32 as
                 libc::c_uint).wrapping_add(Quantity.wrapping_rem(10 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint))
                as UBYTE as STRING;
        (*Label).aText[2 as libc::c_int as usize] =
            0 as libc::c_int as STRING;
        while (*Label).aText[i as usize] != 0 {
            pie_ImageFileID(IntImages,
                            (IMAGE_0 as libc::c_int +
                                 ((*Label).aText[i as usize] as libc::c_int -
                                      '0' as i32)) as UWORD, x as libc::c_int,
                            y as libc::c_int);
            x =
                (x as
                     libc::c_uint).wrapping_add((iV_GetImageWidth(IntImages,
                                                                  (IMAGE_0 as
                                                                       libc::c_int
                                                                       +
                                                                       ((*Label).aText[i
                                                                                           as
                                                                                           usize]
                                                                            as
                                                                            libc::c_int
                                                                            -
                                                                            '0'
                                                                                as
                                                                                i32))
                                                                      as
                                                                      UWORD)
                                                     as libc::c_int +
                                                     1 as libc::c_int) as
                                                    libc::c_uint) as UDWORD as
                    UDWORD;
            i = i.wrapping_add(1)
        }
    };
}
//psFactory = (FACTORY *)((STRUCTURE *)Label->pUserData)->pFunctionality;
		//if (psFactory->psSubject)
// Initialise all the surfaces,graphics etc. used by the interface.
//
#[no_mangle]
pub unsafe extern "C" fn intInitialiseGraphics() {
    // Initialise any bitmaps used by the interface.
    imageInitBitmaps();
    // Initialise button surfaces.
    InitialiseButtonData();
}
// Free up all surfaces,graphics etc. used by the interface.
//
#[no_mangle]
pub unsafe extern "C" fn intDeleteGraphics() {
    DeleteButtonData();
    imageDeleteBitmaps();
}
//#ifdef PSX
// // This sets up a test button for rendering on the playstation
//void InitialiseTestButton(UDWORD Width,UDWORD Height)
//{
//	TestButtonBuffer.InUse=FALSE;
//  	TestButtonBuffer.Surface = iV_SurfaceCreate(REND_SURFACE_USR,Width,Height,0,0,NULL);	// This allocates the surface in psx VRAM
//	ASSERT( TestButtonBuffer.Surface!=NULL,"intInitialise : Failed to create TestButton surface" );
//}
//
//#endif
// Initialise data for interface buttons.
//
#[no_mangle]
pub unsafe extern "C" fn InitialiseButtonData() {
    // Allocate surfaces for rendered buttons.
    let mut Width: UDWORD =
        (iV_GetImageWidth(IntImages, IMAGE_BUT0_UP as libc::c_int as UWORD) as
             libc::c_int + 3 as libc::c_int) as libc::c_uint &
            0xfffffffc as
                libc::c_uint; // Ensure width is whole number of dwords.
    let mut Height: UDWORD =
        iV_GetImageHeight(IntImages, IMAGE_BUT0_UP as libc::c_int as UWORD) as
            UDWORD; // Ensure width is whole number of dwords.
    let mut WidthTopic: UDWORD =
        (iV_GetImageWidth(IntImages, IMAGE_BUTB0_UP as libc::c_int as UWORD)
             as libc::c_int + 3 as libc::c_int) as libc::c_uint &
            0xfffffffc as libc::c_uint; //  what have I done
    let mut HeightTopic: UDWORD =
        iV_GetImageHeight(IntImages, IMAGE_BUTB0_UP as libc::c_int as UWORD)
            as UDWORD; //  what have I done
    let mut i: UDWORD = 0; //  what have I done
    i = 0 as libc::c_int as UDWORD;
    while i < 10 as libc::c_int as libc::c_uint {
        ObjectSurfaces[i as usize].Buffer =
            memMallocRelease(Width.wrapping_mul(Height)) as *mut uint8;
        if !ObjectSurfaces[i as usize].Buffer.is_null() {
        } else {
            debug(LOG_ERROR,
                  b"intInitialise : Failed to allocate Object surface\x00" as
                      *const u8 as *const libc::c_char);
        };
        if !ObjectSurfaces[i as usize].Buffer.is_null() {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"intdisplay.c\x00" as *const u8 as *const libc::c_char,
                  2323 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 21],
                                            &[libc::c_char; 21]>(b"InitialiseButtonData\x00")).as_ptr(),
                  b"ObjectSurfaces[i].Buffer!=NULL\x00" as *const u8 as
                      *const libc::c_char);
        };
        ObjectSurfaces[i as usize].Surface =
            iV_SurfaceCreate(2 as libc::c_int as uint32, Width as libc::c_int,
                             Height as libc::c_int, 10 as libc::c_int,
                             10 as libc::c_int,
                             ObjectSurfaces[i as usize].Buffer);
        if !ObjectSurfaces[i as usize].Surface.is_null() {
        } else {
            debug(LOG_ERROR,
                  b"intInitialise : Failed to create Object surface\x00" as
                      *const u8 as *const libc::c_char);
        };
        if !ObjectSurfaces[i as usize].Surface.is_null() {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"intdisplay.c\x00" as *const u8 as *const libc::c_char,
                  2325 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 21],
                                            &[libc::c_char; 21]>(b"InitialiseButtonData\x00")).as_ptr(),
                  b"ObjectSurfaces[i].Surface!=NULL\x00" as *const u8 as
                      *const libc::c_char);
        };
        i = i.wrapping_add(1)
    }
    i = 0 as libc::c_int as UDWORD;
    while i < (10 as libc::c_int * 4 as libc::c_int) as libc::c_uint {
        ObjectBuffers[i as usize].InUse = 0 as libc::c_int;
        ObjectBuffers[i as usize].ButSurf =
            &mut *ObjectSurfaces.as_mut_ptr().offset(i.wrapping_rem(10 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_uint)
                                                         as isize) as
                *mut BUTTON_SURFACE;
        i = i.wrapping_add(1)
    }
    i = 0 as libc::c_int as UDWORD;
    while i < 10 as libc::c_int as libc::c_uint {
        System0Surfaces[i as usize].Buffer =
            memMallocRelease(Width.wrapping_mul(Height)) as *mut uint8;
        if !System0Surfaces[i as usize].Buffer.is_null() {
        } else {
            debug(LOG_ERROR,
                  b"intInitialise : Failed to allocate System0 surface\x00" as
                      *const u8 as *const libc::c_char);
        };
        if !System0Surfaces[i as usize].Buffer.is_null() {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"intdisplay.c\x00" as *const u8 as *const libc::c_char,
                  2335 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 21],
                                            &[libc::c_char; 21]>(b"InitialiseButtonData\x00")).as_ptr(),
                  b"System0Surfaces[i].Buffer!=NULL\x00" as *const u8 as
                      *const libc::c_char);
        };
        System0Surfaces[i as usize].Surface =
            iV_SurfaceCreate(2 as libc::c_int as uint32, Width as libc::c_int,
                             Height as libc::c_int, 10 as libc::c_int,
                             10 as libc::c_int,
                             System0Surfaces[i as usize].Buffer);
        if !System0Surfaces[i as usize].Surface.is_null() {
        } else {
            debug(LOG_ERROR,
                  b"intInitialise : Failed to create System0 surface\x00" as
                      *const u8 as *const libc::c_char);
        };
        if !System0Surfaces[i as usize].Surface.is_null() {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"intdisplay.c\x00" as *const u8 as *const libc::c_char,
                  2337 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 21],
                                            &[libc::c_char; 21]>(b"InitialiseButtonData\x00")).as_ptr(),
                  b"System0Surfaces[i].Surface!=NULL\x00" as *const u8 as
                      *const libc::c_char);
        };
        i = i.wrapping_add(1)
    }
    i = 0 as libc::c_int as UDWORD;
    while i < (10 as libc::c_int * 8 as libc::c_int) as libc::c_uint {
        System0Buffers[i as usize].InUse = 0 as libc::c_int;
        System0Buffers[i as usize].ButSurf =
            &mut *System0Surfaces.as_mut_ptr().offset(i.wrapping_rem(10 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_uint)
                                                          as isize) as
                *mut BUTTON_SURFACE;
        i = i.wrapping_add(1)
    }
    i = 0 as libc::c_int as UDWORD;
    while i < 5 as libc::c_int as libc::c_uint {
        TopicSurfaces[i as usize].Buffer =
            memMallocRelease(WidthTopic.wrapping_mul(HeightTopic)) as
                *mut uint8;
        if !TopicSurfaces[i as usize].Buffer.is_null() {
        } else {
            debug(LOG_ERROR,
                  b"intInitialise : Failed to allocate Topic surface\x00" as
                      *const u8 as *const libc::c_char);
        };
        if !TopicSurfaces[i as usize].Buffer.is_null() {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"intdisplay.c\x00" as *const u8 as *const libc::c_char,
                  2347 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 21],
                                            &[libc::c_char; 21]>(b"InitialiseButtonData\x00")).as_ptr(),
                  b"TopicSurfaces[i].Buffer!=NULL\x00" as *const u8 as
                      *const libc::c_char);
        };
        TopicSurfaces[i as usize].Surface =
            iV_SurfaceCreate(2 as libc::c_int as uint32,
                             WidthTopic as libc::c_int,
                             HeightTopic as libc::c_int, 10 as libc::c_int,
                             10 as libc::c_int,
                             TopicSurfaces[i as usize].Buffer);
        if !TopicSurfaces[i as usize].Surface.is_null() {
        } else {
            debug(LOG_ERROR,
                  b"intInitialise : Failed to create Topic surface\x00" as
                      *const u8 as *const libc::c_char);
        };
        if !TopicSurfaces[i as usize].Surface.is_null() {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"intdisplay.c\x00" as *const u8 as *const libc::c_char,
                  2349 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 21],
                                            &[libc::c_char; 21]>(b"InitialiseButtonData\x00")).as_ptr(),
                  b"TopicSurfaces[i].Surface!=NULL\x00" as *const u8 as
                      *const libc::c_char);
        };
        i = i.wrapping_add(1)
    }
    i = 0 as libc::c_int as UDWORD;
    while i < (5 as libc::c_int * 4 as libc::c_int) as libc::c_uint {
        TopicBuffers[i as usize].InUse = 0 as libc::c_int;
        TopicBuffers[i as usize].ButSurf =
            &mut *TopicSurfaces.as_mut_ptr().offset(i.wrapping_rem(5 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint)
                                                        as isize) as
                *mut BUTTON_SURFACE;
        i = i.wrapping_add(1)
    }
    i = 0 as libc::c_int as UDWORD;
    while i < 20 as libc::c_int as libc::c_uint {
        StatSurfaces[i as usize].Buffer =
            memMallocRelease(Width.wrapping_mul(Height)) as *mut uint8;
        if !StatSurfaces[i as usize].Buffer.is_null() {
        } else {
            debug(LOG_ERROR,
                  b"intInitialise : Failed to allocate Stats surface\x00" as
                      *const u8 as *const libc::c_char);
        };
        if !StatSurfaces[i as usize].Buffer.is_null() {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"intdisplay.c\x00" as *const u8 as *const libc::c_char,
                  2359 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 21],
                                            &[libc::c_char; 21]>(b"InitialiseButtonData\x00")).as_ptr(),
                  b"StatSurfaces[i].Buffer!=NULL\x00" as *const u8 as
                      *const libc::c_char);
        };
        StatSurfaces[i as usize].Surface =
            iV_SurfaceCreate(2 as libc::c_int as uint32, Width as libc::c_int,
                             Height as libc::c_int, 10 as libc::c_int,
                             10 as libc::c_int,
                             StatSurfaces[i as usize].Buffer);
        if !StatSurfaces[i as usize].Surface.is_null() {
        } else {
            debug(LOG_ERROR,
                  b"intInitialise : Failed to create Stat surface\x00" as
                      *const u8 as *const libc::c_char);
        };
        if !StatSurfaces[i as usize].Surface.is_null() {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"intdisplay.c\x00" as *const u8 as *const libc::c_char,
                  2361 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 21],
                                            &[libc::c_char; 21]>(b"InitialiseButtonData\x00")).as_ptr(),
                  b"StatSurfaces[i].Surface!=NULL\x00" as *const u8 as
                      *const libc::c_char);
        };
        i = i.wrapping_add(1)
    }
    i = 0 as libc::c_int as UDWORD;
    while i < (20 as libc::c_int * 4 as libc::c_int) as libc::c_uint {
        StatBuffers[i as usize].InUse = 0 as libc::c_int;
        StatBuffers[i as usize].ButSurf =
            &mut *StatSurfaces.as_mut_ptr().offset(i.wrapping_rem(20 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint)
                                                       as isize) as
                *mut BUTTON_SURFACE;
        i = i.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn RefreshObjectButtons() {
    let mut i: UDWORD = 0;
    i = 0 as libc::c_int as UDWORD;
    while i < (10 as libc::c_int * 4 as libc::c_int) as libc::c_uint {
        ObjectBuffers[i as usize].Initialised = 0 as libc::c_int;
        i = i.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn RefreshSystem0Buttons() {
    let mut i: UDWORD = 0;
    i = 0 as libc::c_int as UDWORD;
    while i < (10 as libc::c_int * 8 as libc::c_int) as libc::c_uint {
        System0Buffers[i as usize].Initialised = 0 as libc::c_int;
        i = i.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn RefreshTopicButtons() {
    let mut i: UDWORD = 0;
    i = 0 as libc::c_int as UDWORD;
    while i < (5 as libc::c_int * 4 as libc::c_int) as libc::c_uint {
        TopicBuffers[i as usize].Initialised = 0 as libc::c_int;
        i = i.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn RefreshStatsButtons() {
    let mut i: UDWORD = 0;
    i = 0 as libc::c_int as UDWORD;
    while i < (20 as libc::c_int * 4 as libc::c_int) as libc::c_uint {
        StatBuffers[i as usize].Initialised = 0 as libc::c_int;
        i = i.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn ClearObjectBuffers() {
    let mut i: UDWORD = 0;
    i = 0 as libc::c_int as UDWORD;
    while i < (10 as libc::c_int * 4 as libc::c_int) as libc::c_uint {
        ClearObjectButtonBuffer(i as SDWORD);
        i = i.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn ClearTopicBuffers() {
    let mut i: UDWORD = 0;
    i = 0 as libc::c_int as UDWORD;
    while i < (5 as libc::c_int * 4 as libc::c_int) as libc::c_uint {
        ClearTopicButtonBuffer(i as SDWORD);
        i = i.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn ClearObjectButtonBuffer(mut BufferID: SDWORD) {
    ObjectBuffers[BufferID as usize].Initialised = 0 as libc::c_int;
    ObjectBuffers[BufferID as usize].InUse = 0 as libc::c_int;
    ObjectBuffers[BufferID as usize].Data = 0 as *mut libc::c_void;
    ObjectBuffers[BufferID as usize].Data2 = 0 as *mut libc::c_void;
    ObjectBuffers[BufferID as usize].ImdRotation = 45 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ClearTopicButtonBuffer(mut BufferID: SDWORD) {
    TopicBuffers[BufferID as usize].Initialised = 0 as libc::c_int;
    TopicBuffers[BufferID as usize].InUse = 0 as libc::c_int;
    TopicBuffers[BufferID as usize].Data = 0 as *mut libc::c_void;
    TopicBuffers[BufferID as usize].Data2 = 0 as *mut libc::c_void;
    TopicBuffers[BufferID as usize].ImdRotation = 45 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn GetObjectBuffer() -> SDWORD {
    let mut i: SDWORD = 0;
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int * 4 as libc::c_int {
        if ObjectBuffers[i as usize].InUse == 0 as libc::c_int { return i }
        i += 1
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn GetTopicBuffer() -> SDWORD {
    let mut i: SDWORD = 0;
    i = 0 as libc::c_int;
    while i < 5 as libc::c_int * 4 as libc::c_int {
        if TopicBuffers[i as usize].InUse == 0 as libc::c_int { return i }
        i += 1
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn ClearStatBuffers() {
    let mut i: UDWORD = 0;
    i = 0 as libc::c_int as UDWORD;
    while i < (20 as libc::c_int * 4 as libc::c_int) as libc::c_uint {
        StatBuffers[i as usize].Initialised = 0 as libc::c_int;
        StatBuffers[i as usize].InUse = 0 as libc::c_int;
        StatBuffers[i as usize].Data = 0 as *mut libc::c_void;
        StatBuffers[i as usize].ImdRotation = 45 as libc::c_int;
        i = i.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn GetStatBuffer() -> SDWORD {
    let mut i: SDWORD = 0;
    i = 0 as libc::c_int;
    while i < 20 as libc::c_int * 4 as libc::c_int {
        if StatBuffers[i as usize].InUse == 0 as libc::c_int { return i }
        i += 1
    }
    return -(1 as libc::c_int);
}
/*these have been set up for the Transporter - the design screen DOESN'T use them
NB On the PC there are 80!!!!!*/
#[no_mangle]
pub unsafe extern "C" fn ClearSystem0Buffers() {
    let mut i: UDWORD = 0; //  what have I done
    i = 0 as libc::c_int as UDWORD;
    while i < (10 as libc::c_int * 8 as libc::c_int) as libc::c_uint {
        ClearSystem0ButtonBuffer(i as SDWORD);
        i = i.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn ClearSystem0ButtonBuffer(mut BufferID: SDWORD) {
    System0Buffers[BufferID as usize].Initialised = 0 as libc::c_int;
    System0Buffers[BufferID as usize].InUse = 0 as libc::c_int;
    System0Buffers[BufferID as usize].Data = 0 as *mut libc::c_void;
    System0Buffers[BufferID as usize].Data2 = 0 as *mut libc::c_void;
    System0Buffers[BufferID as usize].ImdRotation = 45 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn GetSystem0Buffer() -> SDWORD {
    let mut i: SDWORD = 0;
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int * 8 as libc::c_int {
        if System0Buffers[i as usize].InUse == 0 as libc::c_int { return i }
        i += 1
    }
    return -(1 as libc::c_int);
}
// Free up data for interface buttons.
//
#[no_mangle]
pub unsafe extern "C" fn DeleteButtonData() {
    let mut i: UDWORD = 0;
    i = 0 as libc::c_int as UDWORD;
    while i < 10 as libc::c_int as libc::c_uint {
        memFreeRelease(ObjectSurfaces[i as usize].Buffer as
                           *mut libc::c_void);
        ObjectSurfaces[i as usize].Buffer = 0 as *mut uint8;
        iV_SurfaceDestroy(ObjectSurfaces[i as usize].Surface);
        i = i.wrapping_add(1)
    }
    i = 0 as libc::c_int as UDWORD;
    while i < 5 as libc::c_int as libc::c_uint {
        memFreeRelease(TopicSurfaces[i as usize].Buffer as *mut libc::c_void);
        TopicSurfaces[i as usize].Buffer = 0 as *mut uint8;
        iV_SurfaceDestroy(TopicSurfaces[i as usize].Surface);
        i = i.wrapping_add(1)
    }
    i = 0 as libc::c_int as UDWORD;
    while i < 20 as libc::c_int as libc::c_uint {
        memFreeRelease(StatSurfaces[i as usize].Buffer as *mut libc::c_void);
        StatSurfaces[i as usize].Buffer = 0 as *mut uint8;
        iV_SurfaceDestroy(StatSurfaces[i as usize].Surface);
        i = i.wrapping_add(1)
    }
    i = 0 as libc::c_int as UDWORD;
    while i < 10 as libc::c_int as libc::c_uint {
        memFreeRelease(System0Surfaces[i as usize].Buffer as
                           *mut libc::c_void);
        System0Surfaces[i as usize].Buffer = 0 as *mut uint8;
        iV_SurfaceDestroy(System0Surfaces[i as usize].Surface);
        i = i.wrapping_add(1)
    };
}
#[no_mangle]
pub static mut ButXPos: UWORD = 0 as libc::c_int as UWORD;
#[no_mangle]
pub static mut ButYPos: UWORD = 0 as libc::c_int as UWORD;
#[no_mangle]
pub static mut ButWidth: UWORD = 0;
#[no_mangle]
pub static mut ButHeight: UWORD = 0;
#[no_mangle]
pub unsafe extern "C" fn OpenButtonRender(mut XPos: UWORD, mut YPos: UWORD,
                                          mut Width: UWORD,
                                          mut Height: UWORD) {
    ButXPos = XPos;
    ButYPos = YPos;
    ButWidth = Width;
    ButHeight = Height;
    pie_Set2DClip(XPos as libc::c_int, YPos as libc::c_int,
                  (XPos as libc::c_int + Width as libc::c_int) as UWORD as
                      libc::c_int,
                  (YPos as libc::c_int + Height as libc::c_int) as UWORD as
                      libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn CloseButtonRender() {
    pie_Set2DClip(0 as libc::c_int, 0 as libc::c_int,
                  (*psRendSurface).width - 0 as libc::c_int,
                  (*psRendSurface).height - 0 as libc::c_int);
}
// Clear a button bitmap. ( copy the button background ).
//
#[no_mangle]
pub unsafe extern "C" fn ClearButton(mut Down: BOOL, mut Size: UDWORD,
                                     mut buttonType: UDWORD) {
    if Down != 0 {
        //		pie_ImageFileID(IntImages,(UWORD)(IMAGE_BUT0_DOWN+(Size*2)+(buttonType*6)),ButXPos,ButYPos);
        pie_ImageFileID(IntImages,
                        (IMAGE_BUT0_DOWN as libc::c_int as
                             libc::c_uint).wrapping_add(buttonType.wrapping_mul(2
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint))
                            as UWORD, ButXPos as libc::c_int,
                        ButYPos as libc::c_int);
    } else {
        //		pie_ImageFileID(IntImages,(UWORD)(IMAGE_BUT0_UP+(Size*2)+(buttonType*6)),ButXPos,ButYPos);
        pie_ImageFileID(IntImages,
                        (IMAGE_BUT0_UP as libc::c_int as
                             libc::c_uint).wrapping_add(buttonType.wrapping_mul(2
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint))
                            as UWORD, ButXPos as libc::c_int,
                        ButYPos as libc::c_int);
    };
}
// Create a button by rendering an IMD object into it.
//
#[no_mangle]
pub unsafe extern "C" fn CreateIMDButton(mut ImageFile: *mut IMAGEFILE,
                                         mut ImageID: UWORD,
                                         mut Object: *mut libc::c_void,
                                         mut Player: UDWORD,
                                         mut Buffer: *mut RENDERED_BUTTON,
                                         mut Down: BOOL, mut IMDType: UDWORD,
                                         mut buttonType: UDWORD) {
    let mut Size: UDWORD = 0;
    let mut Rotation: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut Position: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut NullVector: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut ox: UDWORD = 0;
    let mut oy: UDWORD = 0;
    let mut ButSurf: *mut BUTTON_SURFACE = 0 as *mut BUTTON_SURFACE;
    let mut Radius: UDWORD = 0;
    let mut basePlateSize: UDWORD = 0;
    let mut scale: SDWORD = 0;
    ButSurf = (*Buffer).ButSurf;
    if Down != 0 {
        oy = 2 as libc::c_int as UDWORD;
        ox = oy
    } else { oy = 0 as libc::c_int as UDWORD; ox = oy }
    if IMDType == IMDTYPE_DROID as libc::c_int as libc::c_uint ||
           IMDType == IMDTYPE_DROIDTEMPLATE as libc::c_int as libc::c_uint {
        // The case where we have to render a composite droid.
        if Down != 0 {
            //the top button is smaller than the bottom button
            if buttonType == 0 as libc::c_int as libc::c_uint {
                pie_SetGeometricOffset(ButXPos as libc::c_int +
                                           iV_GetImageWidth(IntImages,
                                                            IMAGE_BUT0_DOWN as
                                                                libc::c_int as
                                                                UWORD) as
                                               libc::c_int / 2 as libc::c_int
                                           + ButtonDrawXOffset +
                                           2 as libc::c_int,
                                       ButYPos as libc::c_int +
                                           iV_GetImageHeight(IntImages,
                                                             IMAGE_BUT0_DOWN
                                                                 as
                                                                 libc::c_int
                                                                 as UWORD) as
                                               libc::c_int / 2 as libc::c_int
                                           + 2 as libc::c_int +
                                           8 as libc::c_int +
                                           ButtonDrawYOffset);
            } else {
                pie_SetGeometricOffset(ButXPos as libc::c_int +
                                           iV_GetImageWidth(IntImages,
                                                            IMAGE_BUTB0_DOWN
                                                                as libc::c_int
                                                                as UWORD) as
                                               libc::c_int / 2 as libc::c_int
                                           + ButtonDrawXOffset +
                                           2 as libc::c_int,
                                       ButYPos as libc::c_int +
                                           iV_GetImageHeight(IntImages,
                                                             IMAGE_BUTB0_DOWN
                                                                 as
                                                                 libc::c_int
                                                                 as UWORD) as
                                               libc::c_int / 2 as libc::c_int
                                           + 2 as libc::c_int +
                                           12 as libc::c_int +
                                           ButtonDrawYOffset);
            }
        } else if buttonType == 0 as libc::c_int as libc::c_uint {
            pie_SetGeometricOffset(ButXPos as libc::c_int +
                                       iV_GetImageWidth(IntImages,
                                                        IMAGE_BUT0_UP as
                                                            libc::c_int as
                                                            UWORD) as
                                           libc::c_int / 2 as libc::c_int +
                                       ButtonDrawXOffset,
                                   ButYPos as libc::c_int +
                                       iV_GetImageHeight(IntImages,
                                                         IMAGE_BUT0_UP as
                                                             libc::c_int as
                                                             UWORD) as
                                           libc::c_int / 2 as libc::c_int +
                                       8 as libc::c_int + ButtonDrawYOffset);
        } else {
            pie_SetGeometricOffset(ButXPos as libc::c_int +
                                       iV_GetImageWidth(IntImages,
                                                        IMAGE_BUT0_UP as
                                                            libc::c_int as
                                                            UWORD) as
                                           libc::c_int / 2 as libc::c_int +
                                       ButtonDrawXOffset,
                                   ButYPos as libc::c_int +
                                       iV_GetImageHeight(IntImages,
                                                         IMAGE_BUTB0_UP as
                                                             libc::c_int as
                                                             UWORD) as
                                           libc::c_int / 2 as libc::c_int +
                                       12 as libc::c_int + ButtonDrawYOffset);
        }
        if IMDType == IMDTYPE_DROID as libc::c_int as libc::c_uint {
            Radius = getComponentDroidRadius(Object as *mut DROID)
        } else {
            Radius =
                getComponentDroidTemplateRadius(Object as *mut DROID_TEMPLATE)
        }
        Size = 2 as libc::c_int as UDWORD;
        scale = 72 as libc::c_int;
        if Radius <= 128 as libc::c_int as libc::c_uint {
        } else {
            debug(LOG_ERROR,
                  b"create PIE button big component found\x00" as *const u8 as
                      *const libc::c_char);
        };
        if Radius <= 128 as libc::c_int as libc::c_uint {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"intdisplay.c\x00" as *const u8 as *const libc::c_char,
                  2668 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 16],
                                            &[libc::c_char; 16]>(b"CreateIMDButton\x00")).as_ptr(),
                  b"Radius <= 128\x00" as *const u8 as *const libc::c_char);
        };
        ClearButton(Down, Size, buttonType);
        Rotation.x = -(30 as libc::c_int);
        Rotation.y = (*Buffer).ImdRotation as UDWORD as int32;
        Rotation.z = 0 as libc::c_int;
        NullVector.x = 0 as libc::c_int;
        NullVector.y = 0 as libc::c_int;
        NullVector.z = 0 as libc::c_int;
        if IMDType == IMDTYPE_DROID as libc::c_int as libc::c_uint {
            if (*(Object as *mut DROID)).droidType as libc::c_uint ==
                   DROID_TRANSPORTER as libc::c_int as libc::c_uint {
                Position.x = 0 as libc::c_int;
                //the top button is smaller than the bottom button
                Position.y = 0 as libc::c_int; //BUT_TRANSPORTER_ALT;
                Position.z = 2000 as libc::c_int;
                scale = 72 as libc::c_int / 2 as libc::c_int
            } else {
                Position.y = 0 as libc::c_int;
                Position.x = Position.y;
                Position.z = 2000 as libc::c_int
            }
        } else if (*(Object as *mut DROID_TEMPLATE)).droidType as libc::c_uint
                      == DROID_TRANSPORTER as libc::c_int as libc::c_uint {
            Position.x = 0 as libc::c_int;
            //(IMDType == IMDTYPE_DROIDTEMPLATE)
            Position.y = 0 as libc::c_int; //BUT_TRANSPORTER_ALT;
            Position.z = 2000 as libc::c_int;
            scale = 72 as libc::c_int / 2 as libc::c_int
        } else {
            Position.y = 0 as libc::c_int;
            Position.x = Position.y;
            Position.z = 2000 as libc::c_int
        }
        //lefthand display droid buttons
        if IMDType == IMDTYPE_DROID as libc::c_int as libc::c_uint {
            displayComponentButtonObject(Object as *mut DROID, &mut Rotation,
                                         &mut Position, 1 as libc::c_int,
                                         scale);
        } else {
            displayComponentButtonTemplate(Object as *mut DROID_TEMPLATE,
                                           &mut Rotation, &mut Position,
                                           1 as libc::c_int, scale);
        }
    } else {
        // Just drawing a single IMD.
        if Down != 0 {
            if buttonType == 0 as libc::c_int as libc::c_uint {
                pie_SetGeometricOffset(ButXPos as libc::c_int +
                                           iV_GetImageWidth(IntImages,
                                                            IMAGE_BUT0_DOWN as
                                                                libc::c_int as
                                                                UWORD) as
                                               libc::c_int / 2 as libc::c_int
                                           + ButtonDrawXOffset +
                                           2 as libc::c_int,
                                       ButYPos as libc::c_int +
                                           iV_GetImageHeight(IntImages,
                                                             IMAGE_BUT0_DOWN
                                                                 as
                                                                 libc::c_int
                                                                 as UWORD) as
                                               libc::c_int / 2 as libc::c_int
                                           + 2 as libc::c_int +
                                           8 as libc::c_int +
                                           ButtonDrawYOffset);
            } else {
                pie_SetGeometricOffset(ButXPos as libc::c_int +
                                           iV_GetImageWidth(IntImages,
                                                            IMAGE_BUTB0_DOWN
                                                                as libc::c_int
                                                                as UWORD) as
                                               libc::c_int / 2 as libc::c_int
                                           + ButtonDrawXOffset +
                                           2 as libc::c_int,
                                       ButYPos as libc::c_int +
                                           iV_GetImageHeight(IntImages,
                                                             IMAGE_BUTB0_DOWN
                                                                 as
                                                                 libc::c_int
                                                                 as UWORD) as
                                               libc::c_int / 2 as libc::c_int
                                           + 2 as libc::c_int +
                                           12 as libc::c_int +
                                           ButtonDrawYOffset);
            }
        } else if buttonType == 0 as libc::c_int as libc::c_uint {
            pie_SetGeometricOffset(ButXPos as libc::c_int +
                                       iV_GetImageWidth(IntImages,
                                                        IMAGE_BUT0_UP as
                                                            libc::c_int as
                                                            UWORD) as
                                           libc::c_int / 2 as libc::c_int +
                                       ButtonDrawXOffset,
                                   ButYPos as libc::c_int +
                                       iV_GetImageHeight(IntImages,
                                                         IMAGE_BUT0_UP as
                                                             libc::c_int as
                                                             UWORD) as
                                           libc::c_int / 2 as libc::c_int +
                                       8 as libc::c_int + ButtonDrawYOffset);
        } else {
            pie_SetGeometricOffset(ButXPos as libc::c_int +
                                       iV_GetImageWidth(IntImages,
                                                        IMAGE_BUTB0_UP as
                                                            libc::c_int as
                                                            UWORD) as
                                           libc::c_int / 2 as libc::c_int +
                                       ButtonDrawXOffset,
                                   ButYPos as libc::c_int +
                                       iV_GetImageHeight(IntImages,
                                                         IMAGE_BUTB0_UP as
                                                             libc::c_int as
                                                             UWORD) as
                                           libc::c_int / 2 as libc::c_int +
                                       12 as libc::c_int + ButtonDrawYOffset);
        }
        // Decide which button grid size to use.
        if IMDType == IMDTYPE_COMPONENT as libc::c_int as libc::c_uint {
            Radius = getComponentRadius(Object as *mut BASE_STATS);
            //scale = COMP_BUT_SCALE;
			//ASSERT( Radius <= OBJECT_RADIUS,"Object too big for button - %s",
			//		((BASE_STATS*)Object)->pName );
            Size = 2 as libc::c_int as UDWORD; //small structure
            scale =
                rescaleButtonObject(Radius as SDWORD, 100 as libc::c_int,
                                    64 as libc::c_int)
        } else if IMDType == IMDTYPE_RESEARCH as libc::c_int as libc::c_uint {
            Radius =
                getResearchRadius(Object as
                                      *mut BASE_STATS); //small structure
            if Radius <= 100 as libc::c_int as libc::c_uint {
                Size = 2 as libc::c_int as UDWORD;
                scale =
                    rescaleButtonObject(Radius as SDWORD, 100 as libc::c_int,
                                        64 as libc::c_int)
                //scale = COMP_BUT_SCALE;
            } else if Radius <= 128 as libc::c_int as libc::c_uint {
                Size = 2 as libc::c_int as UDWORD; //small structure
                scale = 55 as libc::c_int
            } else if Radius <= 256 as libc::c_int as libc::c_uint {
                Size = 1 as libc::c_int as UDWORD; //med structure
                scale = 25 as libc::c_int
            } else {
                Size = 0 as libc::c_int as UDWORD; //small structure
                scale = 25 as libc::c_int
            }
        } else if IMDType == IMDTYPE_STRUCTURE as libc::c_int as libc::c_uint
         {
            basePlateSize =
                getStructureSize(Object as *mut STRUCTURE); //med structure
            if basePlateSize == 1 as libc::c_int as libc::c_uint {
                Size = 2 as libc::c_int as UDWORD; //small structure
                scale = 55 as libc::c_int
            } else if basePlateSize == 2 as libc::c_int as libc::c_uint {
                Size = 1 as libc::c_int as UDWORD; //med structure
                scale = 25 as libc::c_int
            } else {
                Size = 0 as libc::c_int as UDWORD; //small structure
                scale = 25 as libc::c_int
            }
        } else if IMDType ==
                      IMDTYPE_STRUCTURESTAT as libc::c_int as libc::c_uint {
            basePlateSize =
                getStructureStatSize(Object as
                                         *mut STRUCTURE_STATS); //med structure
            if basePlateSize == 1 as libc::c_int as libc::c_uint {
                Size =
                    2 as libc::c_int as
                        UDWORD; //was 		Position.z = Radius*30;
                scale = 55 as libc::c_int
            } else if basePlateSize == 2 as libc::c_int as libc::c_uint {
                Size = 1 as libc::c_int as UDWORD;
                scale = 25 as libc::c_int
            } else {
                Size = 0 as libc::c_int as UDWORD;
                scale = 25 as libc::c_int
            }
        } else {
            Radius = (*(Object as *mut iIMDShape)).sradius as UDWORD;
            if Radius <= 128 as libc::c_int as libc::c_uint {
                Size = 2 as libc::c_int as UDWORD;
                scale = 55 as libc::c_int
            } else if Radius <= 256 as libc::c_int as libc::c_uint {
                Size = 1 as libc::c_int as UDWORD;
                scale = 25 as libc::c_int
            } else {
                Size = 0 as libc::c_int as UDWORD;
                scale = 25 as libc::c_int
            }
        }
        ClearButton(Down, Size, buttonType);
        Rotation.x = -(30 as libc::c_int);
        Rotation.y = (*Buffer).ImdRotation as UWORD as int32;
        Rotation.z = 0 as libc::c_int;
        NullVector.x = 0 as libc::c_int;
        NullVector.y = 0 as libc::c_int;
        NullVector.z = 0 as libc::c_int;
        Position.x = 0 as libc::c_int;
        Position.y = 0 as libc::c_int;
        Position.z = 2000 as libc::c_int;
        if !ImageFile.is_null() {
            pie_ImageFileID(ImageFile, ImageID,
                            (ButXPos as libc::c_uint).wrapping_add(ox) as
                                libc::c_int,
                            (ButYPos as libc::c_uint).wrapping_add(oy) as
                                libc::c_int);
            //there may be an extra icon for research buttons now - AB 9/1/99
            /*if (IMDType == IMDTYPE_RESEARCH)
            {
                if (((RESEARCH *)Object)->subGroup != NO_RESEARCH_ICON)
                {
                    iV_DrawTransImage(ImageFile,((RESEARCH *)Object)->subGroup,ButXPos+ox + 40,ButYPos+oy);
                }
            }*/
        }
        pie_SetDepthBufferStatus(DEPTH_CMP_LEQ_WRT_ON);
        /* all non droid buttons */
        if IMDType == IMDTYPE_COMPONENT as libc::c_int as libc::c_uint {
            displayComponentButton(Object as *mut BASE_STATS, &mut Rotation,
                                   &mut Position, 1 as libc::c_int, scale);
        } else if IMDType == IMDTYPE_RESEARCH as libc::c_int as libc::c_uint {
            displayResearchButton(Object as *mut BASE_STATS, &mut Rotation,
                                  &mut Position, 1 as libc::c_int, scale);
        } else if IMDType == IMDTYPE_STRUCTURE as libc::c_int as libc::c_uint
         {
            displayStructureButton(Object as *mut STRUCTURE, &mut Rotation,
                                   &mut Position, 1 as libc::c_int, scale);
        } else if IMDType ==
                      IMDTYPE_STRUCTURESTAT as libc::c_int as libc::c_uint {
            displayStructureStatButton(Object as *mut STRUCTURE_STATS, Player,
                                       &mut Rotation, &mut Position,
                                       1 as libc::c_int, scale);
        } else {
            displayIMDButton(Object as *mut iIMDShape, &mut Rotation,
                             &mut Position, 1 as libc::c_int, scale);
        }
        pie_SetDepthBufferStatus(DEPTH_CMP_ALWAYS_WRT_ON);
    };
}
// Create a button by rendering an image into it.
//
#[no_mangle]
pub unsafe extern "C" fn CreateImageButton(mut ImageFile: *mut IMAGEFILE,
                                           mut ImageID: UWORD,
                                           mut Buffer: *mut RENDERED_BUTTON,
                                           mut Down: BOOL,
                                           mut buttonType: UDWORD) {
    let mut ox: UDWORD = 0;
    let mut oy: UDWORD = 0;
    oy = 0 as libc::c_int as UDWORD;
    ox = oy;
    /*if(Down)
	{
		ox = oy = 2;
	} */
    ClearButton(Down, 0 as libc::c_int as UDWORD, buttonType);
    pie_ImageFileID(ImageFile, ImageID,
                    (ButXPos as libc::c_uint).wrapping_add(ox) as libc::c_int,
                    (ButYPos as libc::c_uint).wrapping_add(oy) as
                        libc::c_int);
    //	DrawTransImageSR(Image,ox,oy);
}
// Create a blank button.
//
#[no_mangle]
pub unsafe extern "C" fn CreateBlankButton(mut Buffer: *mut RENDERED_BUTTON,
                                           mut Down: BOOL,
                                           mut buttonType: UDWORD) {
    let mut ox: UDWORD = 0;
    let mut oy: UDWORD = 0;
    if Down != 0 {
        oy = 1 as libc::c_int as UDWORD;
        ox = oy
    } else { oy = 0 as libc::c_int as UDWORD; ox = oy }
    ClearButton(Down, 0 as libc::c_int as UDWORD, buttonType);
    // Draw a question mark, bit of quick hack this.
    pie_ImageFileID(IntImages, IMAGE_QUESTION_MARK as libc::c_int as UWORD,
                    (ButXPos as
                         libc::c_uint).wrapping_add(ox).wrapping_add(10 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_uint)
                        as libc::c_int,
                    (ButYPos as
                         libc::c_uint).wrapping_add(oy).wrapping_add(3 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_uint)
                        as libc::c_int);
}
// Returns TRUE if the droid is currently demolishing something or moving to demolish something.
//
#[no_mangle]
pub unsafe extern "C" fn DroidIsDemolishing(mut Droid: *mut DROID) -> BOOL {
    let mut Stats: *mut BASE_STATS = 0 as *mut BASE_STATS;
    let mut Structure: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut x: UDWORD = 0;
    let mut y: UDWORD = 0;
    //if(droidType(Droid) != DROID_CONSTRUCT) return FALSE;
    if !(droidType(Droid) as libc::c_uint ==
             DROID_CONSTRUCT as libc::c_int as libc::c_uint ||
             droidType(Droid) as libc::c_uint ==
                 DROID_CYBORG_CONSTRUCT as libc::c_int as libc::c_uint) {
        return 0 as libc::c_int
    }
    if orderStateStatsLoc(Droid, DORDER_DEMOLISH, &mut Stats, &mut x, &mut y)
           != 0 {
        // Moving to demolish location?
        return 1 as libc::c_int
    } else {
        if orderStateObj(Droid, DORDER_DEMOLISH,
                         &mut Structure as *mut *mut STRUCTURE as
                             *mut *mut BASE_OBJECT) != 0 {
            // Is demolishing?
            return 1 as libc::c_int
        }
    }
    return 0 as libc::c_int;
}
// Returns TRUE if the droid is currently repairing another droid.
#[no_mangle]
pub unsafe extern "C" fn DroidIsRepairing(mut Droid: *mut DROID) -> BOOL {
    let mut psObject: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    //if(droidType(Droid) != DROID_REPAIR)
    if !(droidType(Droid) as libc::c_uint ==
             DROID_REPAIR as libc::c_int as libc::c_uint ||
             droidType(Droid) as libc::c_uint ==
                 DROID_CYBORG_REPAIR as libc::c_int as libc::c_uint) {
        return 0 as libc::c_int
    }
    if orderStateObj(Droid, DORDER_DROIDREPAIR, &mut psObject) != 0 {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
// Returns TRUE if the droid is currently building something.
//
#[no_mangle]
pub unsafe extern "C" fn DroidIsBuilding(mut Droid: *mut DROID) -> BOOL {
    let mut Stats: *mut BASE_STATS = 0 as *mut BASE_STATS;
    let mut Structure: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut x: UDWORD = 0;
    let mut y: UDWORD = 0;
    //if(droidType(Droid) != DROID_CONSTRUCT) return FALSE;
    if !(droidType(Droid) as libc::c_uint ==
             DROID_CONSTRUCT as libc::c_int as libc::c_uint ||
             droidType(Droid) as libc::c_uint ==
                 DROID_CYBORG_CONSTRUCT as libc::c_int as libc::c_uint) {
        return 0 as libc::c_int
    }
    if orderStateStatsLoc(Droid, DORDER_BUILD, &mut Stats, &mut x, &mut y) !=
           0 {
        // Moving to build location?
        return 0 as libc::c_int
    } else {
        if orderStateObj(Droid, DORDER_BUILD,
                         &mut Structure as *mut *mut STRUCTURE as
                             *mut *mut BASE_OBJECT) != 0 ||
               orderStateObj(Droid, DORDER_HELPBUILD,
                             &mut Structure as *mut *mut STRUCTURE as
                                 *mut *mut BASE_OBJECT) != 0 {
            //		DBPRINTF(("%p : %d %d\n",Droid,orderStateObj(Droid, DORDER_BUILD,(BASE_OBJECT**)&Structure),
//						orderStateObj(Droid, DORDER_HELPBUILD,(BASE_OBJECT**)&Structure)));
            return 1 as libc::c_int
        }
    }
    return 0 as libc::c_int;
}
// Returns TRUE if the droid has been ordered build something ( but has'nt started yet )
//
#[no_mangle]
pub unsafe extern "C" fn DroidGoingToBuild(mut Droid: *mut DROID) -> BOOL {
    let mut Stats: *mut BASE_STATS = 0 as *mut BASE_STATS;
    let mut x: UDWORD = 0;
    let mut y: UDWORD = 0;
    //if(droidType(Droid) != DROID_CONSTRUCT) return FALSE;
    if !(droidType(Droid) as libc::c_uint ==
             DROID_CONSTRUCT as libc::c_int as libc::c_uint ||
             droidType(Droid) as libc::c_uint ==
                 DROID_CYBORG_CONSTRUCT as libc::c_int as libc::c_uint) {
        return 0 as libc::c_int
    }
    if orderStateStatsLoc(Droid, DORDER_BUILD, &mut Stats, &mut x, &mut y) !=
           0 {
        // Moving to build location?
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
// Get the structure for a structure which a droid is currently building.
//
#[no_mangle]
pub unsafe extern "C" fn DroidGetBuildStructure(mut Droid: *mut DROID)
 -> *mut STRUCTURE {
    let mut Structure: *mut STRUCTURE = 0 as *mut STRUCTURE;
    if orderStateObj(Droid, DORDER_BUILD,
                     &mut Structure as *mut *mut STRUCTURE as
                         *mut *mut BASE_OBJECT) == 0 {
        orderStateObj(Droid, DORDER_HELPBUILD,
                      &mut Structure as *mut *mut STRUCTURE as
                          *mut *mut BASE_OBJECT);
    }
    return Structure;
}
// References StatSurfaces.
// Get the first factory assigned to a command droid
// Get the first factory assigned to a command droid
#[no_mangle]
pub unsafe extern "C" fn droidGetCommandFactory(mut psDroid: *mut DROID)
 -> *mut STRUCTURE {
    let mut inc: SDWORD = 0;
    let mut psCurr: *mut STRUCTURE = 0 as *mut STRUCTURE;
    inc = 0 as libc::c_int;
    while inc < 5 as libc::c_int {
        if (*psDroid).secondaryOrder &
               ((1 as libc::c_int) << inc + 9 as libc::c_int) as libc::c_uint
               != 0 {
            // found an assigned factory - look for it in the lists
            psCurr = apsStructLists[(*psDroid).player as usize];
            while !psCurr.is_null() {
                if (*(*psCurr).pStructureType).type_0 ==
                       REF_FACTORY as libc::c_int as libc::c_uint &&
                       (*(*((*psCurr).pFunctionality as
                                *mut FACTORY)).psAssemblyPoint).factoryInc as
                           libc::c_int == inc {
                    return psCurr
                }
                psCurr = (*psCurr).psNext
            }
        }
        if (*psDroid).secondaryOrder &
               ((1 as libc::c_int) <<
                    inc + (9 as libc::c_int + 5 as libc::c_int)) as
                   libc::c_uint != 0 {
            // found an assigned factory - look for it in the lists
            psCurr = apsStructLists[(*psDroid).player as usize];
            while !psCurr.is_null() {
                if (*(*psCurr).pStructureType).type_0 ==
                       REF_CYBORG_FACTORY as libc::c_int as libc::c_uint &&
                       (*(*((*psCurr).pFunctionality as
                                *mut FACTORY)).psAssemblyPoint).factoryInc as
                           libc::c_int == inc {
                    return psCurr
                }
                psCurr = (*psCurr).psNext
            }
        }
        if (*psDroid).secondaryOrder &
               ((1 as libc::c_int) << inc + 24 as libc::c_int) as libc::c_uint
               != 0 {
            // found an assigned factory - look for it in the lists
            psCurr = apsStructLists[(*psDroid).player as usize];
            while !psCurr.is_null() {
                if (*(*psCurr).pStructureType).type_0 ==
                       REF_VTOL_FACTORY as libc::c_int as libc::c_uint &&
                       (*(*((*psCurr).pFunctionality as
                                *mut FACTORY)).psAssemblyPoint).factoryInc as
                           libc::c_int == inc {
                    return psCurr
                }
                psCurr = (*psCurr).psNext
            }
        }
        inc += 1
    }
    return 0 as *mut STRUCTURE;
}
// Get the stats for a structure which a droid is going to ( but not yet ) building.
//
#[no_mangle]
pub unsafe extern "C" fn DroidGetBuildStats(mut Droid: *mut DROID)
 -> *mut BASE_STATS {
    let mut Stats: *mut BASE_STATS = 0 as *mut BASE_STATS;
    let mut x: UDWORD = 0;
    let mut y: UDWORD = 0;
    if orderStateStatsLoc(Droid, DORDER_BUILD, &mut Stats, &mut x, &mut y) !=
           0 {
        // Moving to build location?
        return Stats
    }
    return 0 as *mut BASE_STATS;
}
#[no_mangle]
pub unsafe extern "C" fn DroidGetIMD(mut Droid: *mut DROID)
 -> *mut iIMDShape {
    return (*Droid).sDisplay.imd;
}
/*UDWORD DroidGetIMDIndex(DROID *Droid)
{
	return Droid->imdNum;
}*/
#[no_mangle]
pub unsafe extern "C" fn StructureIsManufacturing(mut Structure:
                                                      *mut STRUCTURE)
 -> BOOL {
    return (((*(*Structure).pStructureType).type_0 ==
                 REF_FACTORY as libc::c_int as libc::c_uint ||
                 (*(*Structure).pStructureType).type_0 ==
                     REF_CYBORG_FACTORY as libc::c_int as libc::c_uint ||
                 (*(*Structure).pStructureType).type_0 ==
                     REF_VTOL_FACTORY as libc::c_int as libc::c_uint) &&
                !(*((*Structure).pFunctionality as
                        *mut FACTORY)).psSubject.is_null()) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn StructureGetFactory(mut Structure: *mut STRUCTURE)
 -> *mut FACTORY {
    return (*Structure).pFunctionality as *mut FACTORY;
}
#[no_mangle]
pub unsafe extern "C" fn StructureIsResearching(mut Structure: *mut STRUCTURE)
 -> BOOL {
    return ((*(*Structure).pStructureType).type_0 ==
                REF_RESEARCH as libc::c_int as libc::c_uint &&
                !(*((*Structure).pFunctionality as
                        *mut RESEARCH_FACILITY)).psSubject.is_null()) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn StructureGetResearch(mut Structure: *mut STRUCTURE)
 -> *mut RESEARCH_FACILITY {
    return (*Structure).pFunctionality as *mut RESEARCH_FACILITY;
}
#[no_mangle]
pub unsafe extern "C" fn StructureGetIMD(mut Structure: *mut STRUCTURE)
 -> *mut iIMDShape {
    //	return buildingIMDs[aBuildingIMDs[Structure->player][Structure->pStructureType->type]];
    return (*(*Structure).pStructureType).pIMD;
}
#[no_mangle]
pub unsafe extern "C" fn FactoryGetTemplate(mut Factory: *mut FACTORY)
 -> *mut DROID_TEMPLATE {
    return (*Factory).psSubject as *mut DROID_TEMPLATE;
}
//iIMDShape *TemplateGetIMD(DROID_TEMPLATE *Template,UDWORD Player)
//{
// //	return droidIMDs[GetIMDFromTemplate(Template,Player)];
//	return NULL;
//}
//
// /*UDWORD TemplateGetIMDIndex(DROID_TEMPLATE *Template,UDWORD Player)
//{
//	return GetIMDFromTemplate(Template,Player);
//}*/
//
//SDWORD ResearchGetImage(RESEARCH_FACILITY *Research)
//{
//	return 0;	//IMAGE_RESITEM;
//}
#[no_mangle]
pub unsafe extern "C" fn StatIsStructure(mut Stat: *mut BASE_STATS) -> BOOL {
    return ((*Stat).ref_0 >= 0xd0000 as libc::c_int as libc::c_uint &&
                (*Stat).ref_0 <
                    (0xd0000 as libc::c_int + 0x10000 as libc::c_int) as
                        libc::c_uint) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn StatIsFeature(mut Stat: *mut BASE_STATS) -> BOOL {
    return ((*Stat).ref_0 >= 0x100000 as libc::c_int as libc::c_uint &&
                (*Stat).ref_0 <
                    (0x100000 as libc::c_int + 0x10000 as libc::c_int) as
                        libc::c_uint) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn StatGetStructureIMD(mut Stat: *mut BASE_STATS,
                                             mut Player: UDWORD)
 -> *mut iIMDShape {
    //return buildingIMDs[aBuildingIMDs[Player][((STRUCTURE_STATS*)Stat)->type]];
    return (*(Stat as *mut STRUCTURE_STATS)).pIMD;
}
#[no_mangle]
pub unsafe extern "C" fn StatIsTemplate(mut Stat: *mut BASE_STATS) -> BOOL {
    return ((*Stat).ref_0 >= 0xc0000 as libc::c_int as libc::c_uint &&
                (*Stat).ref_0 <
                    (0xc0000 as libc::c_int + 0x10000 as libc::c_int) as
                        libc::c_uint) as libc::c_int;
}
//iIMDShape *StatGetTemplateIMD(BASE_STATS *Stat,UDWORD Player)
//{
//	return TemplateGetIMD((DROID_TEMPLATE*)Stat,Player);
//}
//
// /*UDWORD StatGetTemplateIMDIndex(BASE_STATS *Stat,UDWORD Player)
//{
//	return TemplateGetIMDIndex((DROID_TEMPLATE*)Stat,Player);
//}*/
#[no_mangle]
pub unsafe extern "C" fn StatIsComponent(mut Stat: *mut BASE_STATS)
 -> SDWORD {
    if (*Stat).ref_0 >= 0x10000 as libc::c_int as libc::c_uint &&
           (*Stat).ref_0 <
               (0x10000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
        //return TRUE;
        return COMP_BODY as libc::c_int
    }
    if (*Stat).ref_0 >= 0x20000 as libc::c_int as libc::c_uint &&
           (*Stat).ref_0 <
               (0x20000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
        //return TRUE;
        return COMP_BRAIN as libc::c_int
    }
    if (*Stat).ref_0 >= 0x40000 as libc::c_int as libc::c_uint &&
           (*Stat).ref_0 <
               (0x40000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
        //return TRUE;
        return COMP_PROPULSION as libc::c_int
    }
    if (*Stat).ref_0 >= 0xa0000 as libc::c_int as libc::c_uint &&
           (*Stat).ref_0 <
               (0xa0000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
        //return TRUE;
        return COMP_WEAPON as libc::c_int
    }
    if (*Stat).ref_0 >= 0x50000 as libc::c_int as libc::c_uint &&
           (*Stat).ref_0 <
               (0x50000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
        //return TRUE;
        return COMP_SENSOR as libc::c_int
    }
    if (*Stat).ref_0 >= 0x60000 as libc::c_int as libc::c_uint &&
           (*Stat).ref_0 <
               (0x60000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
        //return TRUE;
        return COMP_ECM as libc::c_int
    }
    if (*Stat).ref_0 >= 0xf0000 as libc::c_int as libc::c_uint &&
           (*Stat).ref_0 <
               (0xf0000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
        //return TRUE;
        return COMP_CONSTRUCT as libc::c_int
    }
    if (*Stat).ref_0 >= 0x80000 as libc::c_int as libc::c_uint &&
           (*Stat).ref_0 <
               (0x80000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
        //return TRUE;
        return COMP_REPAIRUNIT as libc::c_int
    }
    //return FALSE;
    return COMP_UNKNOWN as libc::c_int;
}
//iIMDShape *StatGetComponentIMD(BASE_STATS *Stat)
//iIMDShape *StatGetComponentIMD(BASE_STATS *Stat, SDWORD compID)
#[no_mangle]
pub unsafe extern "C" fn StatGetComponentIMD(mut Stat: *mut BASE_STATS,
                                             mut compID: SDWORD,
                                             mut CompIMD: *mut *mut iIMDShape,
                                             mut MountIMD:
                                                 *mut *mut iIMDShape)
 -> BOOL {
    let mut psWStat: *mut WEAPON_STATS = 0 as *mut WEAPON_STATS;
    /*SWORD ID;

	ID = GetTokenID(CompIMDIDs,Stat->pName);
	if(ID >= 0) {
		return componentIMDs[ID];
	}

	ASSERT( 0,"StatGetComponent : Unknown component" );*/
    //	COMP_BASE_STATS *CompStat = (COMP_BASE_STATS *)Stat;
//	DBPRINTF(("%s\n",Stat->pName));
    *CompIMD = 0 as *mut iIMDShape;
    *MountIMD = 0 as *mut iIMDShape;
    match compID {
        1 => {
            *CompIMD = (*(Stat as *mut COMP_BASE_STATS)).pIMD;
            return 1 as libc::c_int
        }
        2 => {
            //		return ((COMP_BASE_STATS *)Stat)->pIMD;
            //		ASSERT( ((UBYTE*)Stat >= (UBYTE*)asCommandDroids) &&
//				 ((UBYTE*)Stat < (UBYTE*)asCommandDroids + sizeof(asCommandDroids)),
//				 "StatGetComponentIMD: This 'BRAIN_STATS' is actually meant to be a 'COMMAND_DROID'" );
            //		psWStat = asWeaponStats + ((COMMAND_DROID *)Stat)->nWeapStat;
            psWStat = (*(Stat as *mut BRAIN_STATS)).psWeaponStat;
            *MountIMD = (*psWStat).pMountGraphic;
            *CompIMD = (*psWStat).pIMD;
            return 1 as libc::c_int
        }
        8 => {
            *MountIMD = (*(Stat as *mut WEAPON_STATS)).pMountGraphic;
            *CompIMD = (*(Stat as *mut COMP_BASE_STATS)).pIMD;
            return 1 as libc::c_int
        }
        6 => {
            *MountIMD = (*(Stat as *mut SENSOR_STATS)).pMountGraphic;
            *CompIMD = (*(Stat as *mut COMP_BASE_STATS)).pIMD;
            return 1 as libc::c_int
        }
        5 => {
            *MountIMD = (*(Stat as *mut ECM_STATS)).pMountGraphic;
            *CompIMD = (*(Stat as *mut COMP_BASE_STATS)).pIMD;
            return 1 as libc::c_int
        }
        7 => {
            *MountIMD = (*(Stat as *mut CONSTRUCT_STATS)).pMountGraphic;
            *CompIMD = (*(Stat as *mut COMP_BASE_STATS)).pIMD;
            return 1 as libc::c_int
        }
        3 => {
            *CompIMD = (*(Stat as *mut COMP_BASE_STATS)).pIMD;
            return 1 as libc::c_int
        }
        4 => {
            *MountIMD = (*(Stat as *mut REPAIR_STATS)).pMountGraphic;
            *CompIMD = (*(Stat as *mut COMP_BASE_STATS)).pIMD;
            return 1 as libc::c_int
        }
        _ => {
            //COMP_UNKNOWN should be an error
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"StatGetComponent : Unknown component\x00" as *const u8
                          as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"intdisplay.c\x00" as *const u8 as *const libc::c_char,
                      3333 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 20],
                                                &[libc::c_char; 20]>(b"StatGetComponentIMD\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn StatIsResearch(mut Stat: *mut BASE_STATS) -> BOOL {
    return ((*Stat).ref_0 >= 0xb0000 as libc::c_int as libc::c_uint &&
                (*Stat).ref_0 <
                    (0xb0000 as libc::c_int + 0x10000 as libc::c_int) as
                        libc::c_uint) as libc::c_int;
}
//void StatGetResearchImage(BASE_STATS *psStat, SDWORD *Image,iIMDShape **Shape, BOOL drawTechIcon)
#[no_mangle]
pub unsafe extern "C" fn StatGetResearchImage(mut psStat: *mut BASE_STATS,
                                              mut Image: *mut SDWORD,
                                              mut Shape: *mut *mut iIMDShape,
                                              mut ppGraphicData:
                                                  *mut *mut BASE_STATS,
                                              mut drawTechIcon: BOOL) {
    *Image = -(1 as libc::c_int);
    if drawTechIcon != 0 {
        if (*(psStat as *mut RESEARCH)).iconID as libc::c_int !=
               0 as libc::c_int {
            *Image = (*(psStat as *mut RESEARCH)).iconID as SDWORD
        }
    }
    //if the research has a Stat associated with it - use this as display in the button
    if !(*(psStat as *mut RESEARCH)).psStat.is_null() {
        *ppGraphicData = (*(psStat as *mut RESEARCH)).psStat;
        //make sure the IMDShape is initialised
        *Shape = 0 as *mut iIMDShape
    } else {
        //no stat so just just the IMD associated with the research
        *Shape = (*(psStat as *mut RESEARCH)).pIMD;
        //make sure the stat is initialised
        *ppGraphicData = 0 as *mut BASE_STATS
    };
}
// Find a token in the specified token list and return it's ID.
//
/*SWORD GetTokenID(TOKENID *Tok,STRING *Token)
{
	while(Tok->Token!=NULL) {
		if(strcmp(Tok->Token,Token) == 0) {
			return Tok->ID;
		}
		Tok++;
	}

	//test for all - AB
//	return IMD_DEFAULT;
	return -1;
}*/
// Find a token in the specified token list and return it's Index.
//
/*SWORD FindTokenID(TOKENID *Tok,STRING *Token)
{
	SWORD Index = 0;
	while(Tok->Token!=NULL) {
		if(strcmp(Tok->Token,Token) == 0) {
			return Index;
		}
		Index++;
		Tok++;
	}

	return -1;
}*/
//void intDisplayBorderForm(struct _widget *psWidget, UDWORD xOffset, UDWORD yOffset, UDWORD *pColours)
//{
//	W_TABFORM *Form = (W_TABFORM*)psWidget;
//	UDWORD x0,y0,x1,y1;
//
//	x0 = xOffset+Form->x;
//	y0 = yOffset+Form->y;
//	x1 = x0 + Form->width;
//	y1 = y0 + Form->height;
//
//	AdjustTabFormSize(Form,&x0,&y0,&x1,&y1);
//
//	RenderWindowFrame(&FrameNormal,x0,y0,x1-x0,y1-y0);
//}
/* Draws a stats bar for the design screen */
#[no_mangle]
pub unsafe extern "C" fn intDisplayStatsBar(mut psWidget: *mut _widget,
                                            mut xOffset: UDWORD,
                                            mut yOffset: UDWORD,
                                            mut pColours: *mut UDWORD) {
    let mut BarGraph: *mut W_BARGRAPH = psWidget as *mut W_BARGRAPH;
    let mut x0: SDWORD = 0;
    let mut y0: SDWORD = 0;
    let mut iX: SDWORD = 0;
    let mut iY: SDWORD = 0;
    static mut szVal: [libc::c_char; 6] = [0; 6];
    static mut szCheckWidth: [libc::c_char; 6] = [48, 48, 48, 48, 48, 0];
    x0 = xOffset.wrapping_add((*BarGraph).x as libc::c_uint) as SDWORD;
    y0 = yOffset.wrapping_add((*BarGraph).y as libc::c_uint) as SDWORD;
    //	//draw the background image
//	iV_DrawTransImage(IntImages,IMAGE_DES_STATSBACK,x0,y0);
    //increment for the position of the level indicator
    x0 += 3 as libc::c_int;
    y0 += 3 as libc::c_int;
    /* indent to allow text value */
    iX = x0 + iV_GetTextWidth(szCheckWidth.as_mut_ptr());
    iY =
        y0 +
            (iV_GetImageHeight(IntImages,
                               IMAGE_DES_STATSCURR as libc::c_int as UWORD) as
                 libc::c_int - iV_GetTextLineSize()) / 2 as libc::c_int -
            iV_GetTextAboveBase();
    //draw current value section
    pie_ImageFileIDTile(IntImages,
                        IMAGE_DES_STATSCURR as libc::c_int as UWORD, iX, y0,
                        0 as libc::c_int, 0 as libc::c_int,
                        (*BarGraph).majorSize as libc::c_int,
                        iV_GetImageHeight(IntImages,
                                          IMAGE_DES_STATSCURR as libc::c_int
                                              as UWORD) as libc::c_int);
    /* draw text value */
    sprintf(szVal.as_mut_ptr(), b"%d\x00" as *const u8 as *const libc::c_char,
            (*BarGraph).iValue as libc::c_int);
    iV_SetTextColour(-(1 as libc::c_int) as SWORD);
    pie_DrawText(szVal.as_mut_ptr(), x0 as UDWORD, iY as UDWORD);
    //draw the comparison value - only if not zero
    if (*BarGraph).minorSize as libc::c_int != 0 as libc::c_int {
        y0 -= 1 as libc::c_int;
        pie_ImageFileID(IntImages,
                        IMAGE_DES_STATSCOMP as libc::c_int as UWORD,
                        iX + (*BarGraph).minorSize as libc::c_int, y0);
    };
}
/* Draws a Template Power Bar for the Design Screen */
#[no_mangle]
pub unsafe extern "C" fn intDisplayDesignPowerBar(mut psWidget: *mut _widget,
                                                  mut xOffset: UDWORD,
                                                  mut yOffset: UDWORD,
                                                  mut pColours: *mut UDWORD) {
    let mut BarGraph: *mut W_BARGRAPH = psWidget as *mut W_BARGRAPH;
    let mut x0: SDWORD = 0;
    let mut y0: SDWORD = 0;
    let mut iX: SDWORD = 0;
    let mut iY: SDWORD = 0;
    let mut width: UDWORD = 0;
    let mut barWidth: UDWORD = 0;
    static mut szVal: [libc::c_char; 6] = [0; 6];
    static mut szCheckWidth: [libc::c_char; 6] = [48, 48, 48, 48, 48, 0];
    let mut arbitaryOffset: UBYTE = 0;
    x0 = xOffset.wrapping_add((*BarGraph).x as libc::c_uint) as SDWORD;
    y0 = yOffset.wrapping_add((*BarGraph).y as libc::c_uint) as SDWORD;
    //this is a % so need to work out how much of the bar to draw
    /*
	// If power required is greater than Design Power bar then set to max
	if (BarGraph->majorSize > BarGraph->width)
	{
		BarGraph->majorSize = BarGraph->width;
	}*/
    DrawBegin();
    //draw the background image
    pie_ImageFileID(IntImages,
                    IMAGE_DES_POWERBAR_LEFT as libc::c_int as UWORD, x0, y0);
    pie_ImageFileID(IntImages,
                    IMAGE_DES_POWERBAR_RIGHT as libc::c_int as UWORD,
                    x0 + (*psWidget).width as libc::c_int -
                        iV_GetImageWidth(IntImages,
                                         IMAGE_DES_POWERBAR_RIGHT as
                                             libc::c_int as UWORD) as
                            libc::c_int, y0);
    //increment for the position of the bars within the background image
    arbitaryOffset = 3 as libc::c_int as UBYTE;
    x0 += arbitaryOffset as libc::c_int;
    y0 += arbitaryOffset as libc::c_int;
    /* indent to allow text value */
    iX = x0 + iV_GetTextWidth(szCheckWidth.as_mut_ptr());
    iY =
        y0 +
            (iV_GetImageHeight(IntImages,
                               IMAGE_DES_STATSCURR as libc::c_int as UWORD) as
                 libc::c_int - iV_GetTextLineSize()) / 2 as libc::c_int -
            iV_GetTextAboveBase();
    //adjust the width based on the text drawn
    barWidth =
        ((*BarGraph).width as libc::c_int -
             (iX - x0 + arbitaryOffset as libc::c_int)) as UDWORD;
    width =
        ((*BarGraph).majorSize as
             libc::c_uint).wrapping_mul(barWidth).wrapping_div(100 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uint);
    //quick check that don't go over the end - ensure % is not > 100
    if width > barWidth { width = barWidth }
    //draw current value section
    pie_ImageFileIDTile(IntImages,
                        IMAGE_DES_STATSCURR as libc::c_int as UWORD, iX, y0,
                        0 as libc::c_int, 0 as libc::c_int,
                        width as libc::c_int,
                        iV_GetImageHeight(IntImages,
                                          IMAGE_DES_STATSCURR as libc::c_int
                                              as UWORD) as libc::c_int);
    /* draw text value */
    sprintf(szVal.as_mut_ptr(), b"%d\x00" as *const u8 as *const libc::c_char,
            (*BarGraph).iValue as libc::c_int);
    iV_SetTextColour(-(1 as libc::c_int) as SWORD);
    pie_DrawText(szVal.as_mut_ptr(), x0 as UDWORD, iY as UDWORD);
    //draw the comparison value - only if not zero
    if (*BarGraph).minorSize as libc::c_int != 0 as libc::c_int {
        y0 -= 1 as libc::c_int;
        width =
            ((*BarGraph).minorSize as
                 libc::c_uint).wrapping_mul(barWidth).wrapping_div(100 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint);
        if width > barWidth { width = barWidth }
        //iV_DrawTransImage(IntImages,IMAGE_DES_STATSCOMP,x0+BarGraph->minorSize ,y0);
        pie_ImageFileID(IntImages,
                        IMAGE_DES_STATSCOMP as libc::c_int as UWORD,
                        (iX as libc::c_uint).wrapping_add(width) as
                            libc::c_int, y0);
    }
    DrawEnd();
}
// 200 milliseconds between each beep please
#[no_mangle]
pub unsafe extern "C" fn WidgetAudioCallback(mut AudioID: libc::c_int) {
    static mut LastTimeAudio: SDWORD = 0;
    if AudioID >= 0 as libc::c_int {
        //		DBPRINTF(("%d\n",AudioID));
        let mut TimeSinceLastWidgetBeep: SDWORD = 0;
        // Don't allow a widget beep if one was made in the last WIDGETBEEPGAP milliseconds
		// This stops double beeps happening (which seems to happen all the time)
        TimeSinceLastWidgetBeep =
            gameTime2.wrapping_sub(LastTimeAudio as libc::c_uint) as SDWORD;
        if TimeSinceLastWidgetBeep < 0 as libc::c_int ||
               TimeSinceLastWidgetBeep > 200 as libc::c_int {
            LastTimeAudio = gameTime2 as SDWORD;
            audio_PlayTrack(AudioID);
            //			DBPRINTF(("AudioID %d\n",AudioID));
        }
    };
}
// Widget callback to display a contents button for the Transporter
#[no_mangle]
pub unsafe extern "C" fn intDisplayTransportButton(mut psWidget: *mut _widget,
                                                   mut xOffset: UDWORD,
                                                   mut yOffset: UDWORD,
                                                   mut pColours:
                                                       *mut UDWORD) {
    let mut Form: *mut W_CLICKFORM = psWidget as *mut W_CLICKFORM;
    let mut Down: BOOL = 0;
    let mut Hilight: BOOL = 0 as libc::c_int;
    let mut Buffer: *mut RENDERED_BUTTON =
        (*Form).pUserData as *mut RENDERED_BUTTON;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut gfxId: UDWORD = 0;
    OpenButtonRender(xOffset.wrapping_add((*Form).x as libc::c_uint) as UWORD,
                     yOffset.wrapping_add((*Form).y as libc::c_uint) as UWORD,
                     (*Form).width, (*Form).height);
    Down =
        ((*Form).state &
             (0x1 as libc::c_int | 0x8 as libc::c_int | 0x10 as libc::c_int)
                 as libc::c_uint) as BOOL;
    //allocate this outside of the if so the rank icons are always draw
    psDroid = (*Buffer).Data as *mut DROID;
    //there should always be a droid associated with the button
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"intDisplayTransportButton: invalid droid pointer\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"intdisplay.c\x00" as *const u8 as *const libc::c_char,
              3606 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 26],
                                        &[libc::c_char; 26]>(b"intDisplayTransportButton\x00")).as_ptr(),
              b"PTRVALID(psDroid, sizeof(DROID))\x00" as *const u8 as
                  *const libc::c_char);
    };
    Hilight = ((*Form).state & 0x4 as libc::c_int as libc::c_uint) as BOOL;
    if Hilight != 0 {
        (*Buffer).ImdRotation +=
            (90 as libc::c_int as
                 libc::c_uint).wrapping_mul(frameTime2).wrapping_div(1000 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_uint)
                as UWORD as libc::c_int
    }
    Hilight = ((*Form).state & 0x4 as libc::c_int as libc::c_uint) as BOOL;
    (*Buffer).State = (*Form).state;
    //psDroid = (DROID*)Buffer->Data;
    //there should always be a droid associated with the button
		//ASSERT( PTRVALID(psDroid, sizeof(DROID)),
		//	"intDisplayTransportButton: invalid droid pointer" );
    if !psDroid.is_null() {
        RenderToButton(0 as *mut IMAGEFILE, 0 as libc::c_int as UWORD,
                       psDroid as *mut libc::c_void,
                       (*psDroid).player as UDWORD, Buffer, Down,
                       IMDTYPE_DROID as libc::c_int as UDWORD,
                       0 as libc::c_int as UDWORD);
    } else { RenderBlankToButton(Buffer, Down, 0 as libc::c_int as UDWORD); }
    (*Buffer).Initialised = 1 as libc::c_int;
    CloseButtonRender();
    if Hilight != 0 {
        pie_ImageFileID(IntImages, IMAGE_BUT_HILITE as libc::c_int as UWORD,
                        xOffset.wrapping_add((*Form).x as libc::c_uint) as
                            libc::c_int,
                        yOffset.wrapping_add((*Form).y as libc::c_uint) as
                            libc::c_int);
    }
    //if (psDroid AND missionIsOffworld()) Want this on all reInforcement missions
    if !psDroid.is_null() && missionForReInforcements() != 0 {
        //add the experience level for each droid
        gfxId = getDroidRankGraphic(psDroid);
        if gfxId != 0xffffffff as libc::c_uint {
            /* Render the rank graphic at the correct location */
		    /* Render the rank graphic at the correct location */
            pie_ImageFileID(IntImages, gfxId as UWORD,
                            xOffset.wrapping_add((*Form).x as
                                                     libc::c_uint).wrapping_add(50
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint)
                                as libc::c_int,
                            yOffset.wrapping_add((*Form).y as
                                                     libc::c_uint).wrapping_add(30
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint)
                                as libc::c_int);
        }
    };
}
/*draws blips on radar to represent Proximity Display and damaged structures*/
#[no_mangle]
pub unsafe extern "C" fn drawRadarBlips() {
    let mut psProxDisp: *mut PROXIMITY_DISPLAY = 0 as *mut PROXIMITY_DISPLAY;
    //	STRUCTURE			*psBuilding;
    let mut psFeature: *mut FEATURE = 0 as *mut FEATURE;
    //VIEW_PROXIMITY		*pViewProximity;
	//SDWORD				x, y;
    let mut imageID: UWORD = 0;
    let mut VisWidth: UDWORD = 0;
    let mut VisHeight: UDWORD = 0;
    let mut delay: UDWORD = 150 as libc::c_int as UDWORD;
    let mut proxType: PROX_TYPE = PROX_ENEMY;
    /*#ifndef PSX
	SDWORD				radarX,radarY;		// for multiplayer blips
	//FEATURE				*psFeature;			// ditto. Needed always now!
#endif*/
    VisWidth = 128 as libc::c_int as UDWORD;
    VisHeight = 128 as libc::c_int as UDWORD;
    /* Go through all the proximity Displays*/
    psProxDisp = apsProxDisp[selectedPlayer as usize];
    while !psProxDisp.is_null() {
        //check it is within the radar coords
        if (*psProxDisp).radarX > 0 as libc::c_int as libc::c_uint &&
               (*psProxDisp).radarX < VisWidth &&
               (*psProxDisp).radarY > 0 as libc::c_int as libc::c_uint &&
               (*psProxDisp).radarY < VisHeight {
            //pViewProximity = (VIEW_PROXIMITY*)psProxDisp->psMessage->
			//	pViewData->pData;
            if (*psProxDisp).type_0 as libc::c_uint ==
                   POS_PROXDATA as libc::c_int as libc::c_uint {
                proxType =
                    (*((*((*(*psProxDisp).psMessage).pViewData as
                              *mut VIEWDATA)).pData as
                           *mut VIEW_PROXIMITY)).proxType
            } else {
                psFeature =
                    (*(*psProxDisp).psMessage).pViewData as *mut FEATURE;
                if !psFeature.is_null() &&
                       (*(*psFeature).psStats).subType as libc::c_uint ==
                           FEAT_OIL_RESOURCE as libc::c_int as libc::c_uint {
                    proxType = PROX_RESOURCE
                } else { proxType = PROX_ARTEFACT }
            }
            //draw the 'blips' on the radar - use same timings as radar blips
			//if the message is read - don't animate
            if (*(*psProxDisp).psMessage).read != 0 {
                //imageID = (UWORD)(IMAGE_RAD_ENM3 + (pViewProximity->
				//	proxType * (NUM_PULSES + 1)));
                imageID =
                    (IMAGE_RAD_ENM3 as libc::c_int as
                         libc::c_uint).wrapping_add((proxType as
                                                         libc::c_uint).wrapping_mul((3
                                                                                         as
                                                                                         libc::c_int
                                                                                         +
                                                                                         1
                                                                                             as
                                                                                             libc::c_int)
                                                                                        as
                                                                                        libc::c_uint))
                        as UWORD
            } else {
                //draw animated
                if gameTime2.wrapping_sub((*psProxDisp).timeLastDrawn) > delay
                   {
                    (*psProxDisp).strobe =
                        (*psProxDisp).strobe.wrapping_add(1);
                    if (*psProxDisp).strobe >
                           (3 as libc::c_int - 1 as libc::c_int) as
                               libc::c_uint {
                        (*psProxDisp).strobe = 0 as libc::c_int as UDWORD
                    }
                    (*psProxDisp).timeLastDrawn = gameTime2
                }
                //imageID = (UWORD)(IMAGE_RAD_ENM1 + psProxDisp->strobe + (
				//	pViewProximity->proxType * (NUM_PULSES + 1)));
                imageID =
                    (IMAGE_RAD_ENM1 as libc::c_int as
                         libc::c_uint).wrapping_add((*psProxDisp).strobe).wrapping_add((proxType
                                                                                            as
                                                                                            libc::c_uint).wrapping_mul((3
                                                                                                                            as
                                                                                                                            libc::c_int
                                                                                                                            +
                                                                                                                            1
                                                                                                                                as
                                                                                                                                libc::c_int)
                                                                                                                           as
                                                                                                                           libc::c_uint))
                        as UWORD
            }
            //draw the 'blip'
            pie_ImageFileID(IntImages, imageID,
                            (*psProxDisp).radarX.wrapping_add(((23 as
                                                                    libc::c_int
                                                                    +
                                                                    132 as
                                                                        libc::c_int
                                                                    +
                                                                    6 as
                                                                        libc::c_int)
                                                                   as
                                                                   libc::c_uint).wrapping_add(pie_GetVideoBufferWidth().wrapping_sub(640
                                                                                                                                         as
                                                                                                                                         libc::c_int
                                                                                                                                         as
                                                                                                                                         libc::c_uint).wrapping_div(2
                                                                                                                                                                        as
                                                                                                                                                                        libc::c_int
                                                                                                                                                                        as
                                                                                                                                                                        libc::c_uint)).wrapping_add(320
                                                                                                                                                                                                        as
                                                                                                                                                                                                        libc::c_int
                                                                                                                                                                                                        as
                                                                                                                                                                                                        libc::c_uint).wrapping_add(6
                                                                                                                                                                                                                                       as
                                                                                                                                                                                                                                       libc::c_int
                                                                                                                                                                                                                                       as
                                                                                                                                                                                                                                       libc::c_uint).wrapping_add(1
                                                                                                                                                                                                                                                                      as
                                                                                                                                                                                                                                                                      libc::c_int
                                                                                                                                                                                                                                                                      as
                                                                                                                                                                                                                                                                      libc::c_uint).wrapping_add(pie_GetVideoBufferWidth().wrapping_sub(640
                                                                                                                                                                                                                                                                                                                                            as
                                                                                                                                                                                                                                                                                                                                            libc::c_int
                                                                                                                                                                                                                                                                                                                                            as
                                                                                                                                                                                                                                                                                                                                            libc::c_uint).wrapping_div(2
                                                                                                                                                                                                                                                                                                                                                                           as
                                                                                                                                                                                                                                                                                                                                                                           libc::c_int
                                                                                                                                                                                                                                                                                                                                                                           as
                                                                                                                                                                                                                                                                                                                                                                           libc::c_uint)))
                                as libc::c_int,
                            (*psProxDisp).radarY.wrapping_add((324 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uint).wrapping_add(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                                                                          as
                                                                                                                                          libc::c_int
                                                                                                                                          as
                                                                                                                                          libc::c_uint)).wrapping_add(1
                                                                                                                                                                          as
                                                                                                                                                                          libc::c_int
                                                                                                                                                                          as
                                                                                                                                                                          libc::c_uint))
                                as libc::c_int);
        }
        psProxDisp = (*psProxDisp).psNext
    };
    /*
	for (psBuilding = apsStructLists[selectedPlayer]; psBuilding != NULL;
		psBuilding = psBuilding->psNext)
	{
		//check it is within the radar coords
		if (psBuilding->radarX > 0 AND psBuilding->radarX < VisWidth AND
			psBuilding->radarY > 0 AND psBuilding->radarY < VisHeight)
		{
			//check if recently damaged
			if (psBuilding->timeLastHit)
			{
				if (((gameTime/250) % 2) == 0)
				{
					imageID = IMAGE_RAD_ENMREAD;
				}
				else
				{
					imageID = 0;//IMAGE_RAD_ENM1;
				}
				//turn blips off if been on for long enough
				if (psBuilding->timeLastHit + ATTACK_CB_PAUSE < gameTime)
				{
					psBuilding->timeLastHit = 0;
				}
				//draw the 'blip'
				if (imageID)
				{
#ifndef PSX
					iV_DrawTransImage(IntImages,imageID, psBuilding->radarX + RADTLX,
						psBuilding->radarY + RADTLY);
#else
					iV_DrawTransImage(IntImages,imageID, psBuilding->radarX*2 + RADTLX,
									psBuilding->radarY*2 + RADTLY);
#endif
				}
			}
		}
	}
	*/
    // deathmatch code
//#ifndef PSX
//	if(bMultiPlayer && (game.type == DMATCH))
//	{
//		for (psFeature = apsFeatureLists[0]; psFeature != NULL; psFeature =
//			psFeature->psNext)
//		{
//			if( psFeature->psStats->subType == FEAT_GEN_ARTE)	// it's an artifact.
//			{
//				worldPosToRadarPos(psFeature->x >> TILE_SHIFT ,
//								   psFeature->y >> TILE_SHIFT,  &radarX,&radarY);
//				if (radarX > 0 && radarX < (SDWORD)VisWidth &&			// it's visable.
//					radarY > 0 && radarY < (SDWORD)VisHeight)
//				{
//					iV_DrawTransImage(IntImages,(UWORD)(IMAGE_RAD_ENM3),radarX + RADTLX, radarY + RADTLY);
//				}
//			}
//		}
//	}
//#endif
}
/*draws blips on world to represent Proximity Messages - no longer the Green Arrow!*/
/*void drawProximityBlips()
{
	PROXIMITY_DISPLAY	*psProxDisp;
	VIEW_PROXIMITY		*pViewProximity;
	UWORD				imageID;
	UDWORD				VisWidth, VisHeight, delay = 150;

	// Go through all the proximity Displays
	for (psProxDisp = apsProxDisp[selectedPlayer]; psProxDisp != NULL;
		psProxDisp = psProxDisp->psNext)
	{
		pViewProximity = (VIEW_PROXIMITY*)psProxDisp->psMessage->pViewData->pData;
		// Is the Message worth rendering?
		if(clipXY(pViewProximity->x,pViewProximity->y))
		{
			//if the message is read - don't draw
			if (!psProxDisp->psMessage->read)
			{
				//draw animated - use same timings as radar blips
				if ((gameTime2 - psProxDisp->timeLastDrawn) > delay)
				{
					psProxDisp->strobe++;
					if (psProxDisp->strobe > (NUM_PULSES-1))
					{
						psProxDisp->strobe = 0;
					}
					psProxDisp->timeLastDrawn = gameTime2;
				}
				imageID = (UWORD)(IMAGE_GAM_ENM1 + psProxDisp->strobe +
					(pViewProximity->proxType * (NUM_PULSES + 1)));
			}
			//draw the 'blip'
			iV_DrawTransImage(IntImages,imageID, psProxDisp->screenX, psProxDisp->screenY);
		}
	}
}*/
/*Displays the proximity messages blips over the world*/
#[no_mangle]
pub unsafe extern "C" fn intDisplayProximityBlips(mut psWidget: *mut _widget,
                                                  mut xOffset: UDWORD,
                                                  mut yOffset: UDWORD,
                                                  mut pColours: *mut UDWORD) {
    let mut psButton: *mut W_CLICKFORM = psWidget as *mut W_CLICKFORM;
    let mut psProxDisp: *mut PROXIMITY_DISPLAY =
        (*psButton).pUserData as *mut PROXIMITY_DISPLAY;
    let mut psMsg: *mut MESSAGE = (*psProxDisp).psMessage;
    //BOOL				Hilight = FALSE;
//	UWORD				imageID;
//	UDWORD				delay = 100;
	//VIEW_PROXIMITY		*pViewProximity;
    let mut x: SDWORD = 0 as libc::c_int;
    let mut y: SDWORD = 0 as libc::c_int;
    if (*psMsg).type_0 as libc::c_uint ==
           MSG_PROXIMITY as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Invalid message type\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (*psMsg).type_0 as libc::c_uint ==
           MSG_PROXIMITY as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"intdisplay.c\x00" as *const u8 as *const libc::c_char,
              3859 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 25],
                                        &[libc::c_char; 25]>(b"intDisplayProximityBlips\x00")).as_ptr(),
              b"psMsg->type == MSG_PROXIMITY\x00" as *const u8 as
                  *const libc::c_char);
    };
    //if no data - ignore message
    if (*psMsg).pViewData.is_null() { return }
    //pViewProximity = (VIEW_PROXIMITY*)psProxDisp->psMessage->pViewData->pData;
    if (*psProxDisp).type_0 as libc::c_uint ==
           POS_PROXDATA as libc::c_int as libc::c_uint {
        x =
            (*((*((*(*psProxDisp).psMessage).pViewData as
                      *mut VIEWDATA)).pData as *mut VIEW_PROXIMITY)).x as
                SDWORD;
        y =
            (*((*((*(*psProxDisp).psMessage).pViewData as
                      *mut VIEWDATA)).pData as *mut VIEW_PROXIMITY)).y as
                SDWORD
    } else if (*psProxDisp).type_0 as libc::c_uint ==
                  POS_PROXOBJ as libc::c_int as libc::c_uint {
        x =
            (*((*(*psProxDisp).psMessage).pViewData as *mut BASE_OBJECT)).x as
                SDWORD;
        y =
            (*((*(*psProxDisp).psMessage).pViewData as *mut BASE_OBJECT)).y as
                SDWORD
    }
    //if not within view ignore message
	//if (!clipXY(pViewProximity->x, pViewProximity->y))
    if clipXY(x, y) == 0 { return }
    /*Hilight = psButton->state & WBUTS_HILITE;

	//if hilighted
	if (Hilight)
	{
		imageID = IMAGE_DES_ROAD;
		//set the button's x/y so that can be clicked on
		psButton->x = (SWORD)psProxDisp->screenX;
		psButton->y = (SWORD)psProxDisp->screenY;

		//draw the 'button'
		iV_DrawTransImage(IntImages,imageID, psButton->x, psButton->y);
		return;
	}*/
    //if the message is read - don't draw
    if (*psMsg).read == 0 {
        //draw animated
		/*
		if ((gameTime2 - psProxDisp->timeLastDrawn) > delay)
		{
			psProxDisp->strobe++;
			if (psProxDisp->strobe > (NUM_PULSES-1))
			{
				psProxDisp->strobe = 0;
			}
			psProxDisp->timeLastDrawn = gameTime2;
		}
		imageID = (UWORD)(IMAGE_GAM_ENM1 + psProxDisp->strobe +
			(pViewProximity->proxType * (NUM_PULSES + 1)));
		*/
		//set the button's x/y so that can be clicked on
        (*psButton).x =
            (*psProxDisp).screenX.wrapping_sub(((*psButton).width as
                                                    libc::c_int /
                                                    2 as libc::c_int) as
                                                   libc::c_uint) as SWORD;
        (*psButton).y =
            (*psProxDisp).screenY.wrapping_sub(((*psButton).height as
                                                    libc::c_int /
                                                    2 as libc::c_int) as
                                                   libc::c_uint) as SWORD
        /*
		//draw the 'button'
		iV_DrawTransImage(IntImages,imageID, psProxDisp->screenX,
			psProxDisp->screenY);
			*/
    };
}
unsafe extern "C" fn sliderMousePos(mut Slider: *mut W_SLIDER) -> UDWORD {
    return ((*widgGetFromID(psWScreen, (*Slider).formID)).x as libc::c_int +
                (*Slider).x as libc::c_int +
                (*Slider).pos as libc::c_int * (*Slider).width as libc::c_int
                    / (*Slider).numStops as libc::c_int) as UDWORD;
}
unsafe extern "C" fn sliderMouseUnit(mut Slider: *mut W_SLIDER) -> UWORD {
    let mut posStops: UWORD =
        ((*Slider).numStops as libc::c_int / 20 as libc::c_int) as UWORD;
    if posStops as libc::c_int == 0 as libc::c_int ||
           (*Slider).pos as libc::c_int == 0 as libc::c_int ||
           (*Slider).pos as libc::c_int == (*Slider).numStops as libc::c_int {
        return 1 as libc::c_int as UWORD
    }
    if ((*Slider).pos as libc::c_int) < posStops as libc::c_int {
        return (*Slider).pos
    }
    if (*Slider).pos as libc::c_int >
           (*Slider).numStops as libc::c_int - posStops as libc::c_int {
        return ((*Slider).numStops as libc::c_int -
                    (*Slider).pos as libc::c_int) as UWORD
    }
    return posStops;
}
#[no_mangle]
pub unsafe extern "C" fn intUpdateQuantitySlider(mut psWidget: *mut _widget,
                                                 mut psContext:
                                                     *mut _w_context) {
    let mut Slider: *mut W_SLIDER = psWidget as *mut W_SLIDER;
    if (*Slider).state as libc::c_int & 0x2 as libc::c_int != 0 {
        if keyDown(KEY_LEFTARROW) != 0 {
            if (*Slider).pos as libc::c_int > 0 as libc::c_int {
                (*Slider).pos =
                    ((*Slider).pos as libc::c_int -
                         sliderMouseUnit(Slider) as libc::c_int) as UWORD;
                bUsingSlider = 1 as libc::c_int;
                SetMousePos(0 as libc::c_int as UDWORD,
                            sliderMousePos(Slider), mouseY() as UDWORD);
                // move mouse
            }
        } else if keyDown(KEY_RIGHTARROW) != 0 {
            if ((*Slider).pos as libc::c_int) <
                   (*Slider).numStops as libc::c_int {
                (*Slider).pos =
                    ((*Slider).pos as libc::c_int +
                         sliderMouseUnit(Slider) as libc::c_int) as UWORD;
                bUsingSlider = 1 as libc::c_int;
                SetMousePos(0 as libc::c_int as UDWORD,
                            sliderMousePos(Slider), mouseY() as UDWORD);
                // move mouse
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn intUpdateOptionText(mut psWidget: *mut _widget,
                                             mut psContext: *mut _w_context) {
}
#[no_mangle]
pub unsafe extern "C" fn intDisplayResSubGroup(mut psWidget: *mut _widget,
                                               mut xOffset: UDWORD,
                                               mut yOffset: UDWORD,
                                               mut pColours: *mut UDWORD) {
    let mut Label: *mut W_LABEL = psWidget as *mut W_LABEL;
    let mut x: UDWORD = ((*Label).x as libc::c_uint).wrapping_add(xOffset);
    let mut y: UDWORD = ((*Label).y as libc::c_uint).wrapping_add(yOffset);
    let mut psResearch: *mut RESEARCH = (*Label).pUserData as *mut RESEARCH;
    if (*psResearch).subGroup as libc::c_int != 0 as libc::c_int {
        pie_ImageFileID(IntImages, (*psResearch).subGroup, x as libc::c_int,
                        y as libc::c_int);
    };
}
//extern RENDERED_BUTTON System1Buffers[NUM_OBJECTBUFFERS];
//extern RENDERED_BUTTON System2Buffers[NUM_OBJECTBUFFERS];
// Power required to manufacture the current item.
// Set audio IDs for form opening/closing anims.
// Initialise interface graphics.
// Free up interface graphics.
// Intialise button surfaces.
// Free up button surfaces.
// Get a free RENDERED_BUTTON structure for an object window button.
// Clear ( make unused ) all RENDERED_BUTTON structures for the object window.
// Clear ( make unused ) all RENDERED_BUTTON structures for the topic window.
// Clear ( make unused ) a RENDERED_BUTTON structure.
// Clear ( make unused ) a RENDERED_BUTTON structure.
// Get a free RENDERED_BUTTON structure for a stat window button.
// Clear ( make unused ) all RENDERED_BUTTON structures for the stat window.
/*these have been set up for the Transporter - the design screen DOESN'T use them*/
// Clear ( make unused ) *all* RENDERED_BUTTON structures.
// Clear ( make unused ) a RENDERED_BUTTON structure.
// Get a free RENDERED_BUTTON structure.
// callback to update the command droid size label
// callback to update the command droid experience
// callback to update the command droid factories
//callback to display the factory number
//callback to display the production quantity number for a template
//void RenderCompositeDroid(UDWORD Index,iVector *Rotation,iVector *Position);
//iIMDShape *TemplateGetIMD(DROID_TEMPLATE *DroidTemp,UDWORD Player);
//UDWORD TemplateGetIMDIndex(DROID_TEMPLATE *Template,UDWORD Player);
//SDWORD ResearchGetImage(RESEARCH_FACILITY *Research);
//iIMDShape *StatGetTemplateIMD(BASE_STATS *Stat,UDWORD Player);
//UDWORD StatGetTemplateIMDIndex(BASE_STATS *Stat,UDWORD Player);
//iIMDShape *StatGetComponentIMD(BASE_STATS *Stat);
//iIMDShape *StatGetComponentIMD(BASE_STATS *Stat, SDWORD compID);
//void StatGetResearchImage(BASE_STATS *Stat,SDWORD *Image,iIMDShape **Shape, BOOL drawTechIcon);
//SWORD GetTokenID(TOKENID *Tok,STRING *Token);
//SWORD FindTokenID(TOKENID *Tok,STRING *Token);
//displays a border for a form
/* Draws a stats bar for the design screen */
/* Draws a Template Power Bar for the Design Screen */
// Widget callback function to play an audio track.
// Widget callback to display a contents button for the Transporter
/*draws blips on radar to represent Proximity Display*/
/*Displays the proximity messages blips over the world*/
#[no_mangle]
pub unsafe extern "C" fn intDisplayAllyIcon(mut psWidget: *mut _widget,
                                            mut xOffset: UDWORD,
                                            mut yOffset: UDWORD,
                                            mut pColours: *mut UDWORD) {
    let mut Label: *mut W_LABEL = psWidget as *mut W_LABEL;
    //	UDWORD		i = Label->pUserData;
    let mut x: UDWORD = ((*Label).x as libc::c_uint).wrapping_add(xOffset);
    let mut y: UDWORD = ((*Label).y as libc::c_uint).wrapping_add(yOffset);
    //	char		str[2];
    pie_ImageFileID(IntImages, IMAGE_DES_BODYPOINTS as libc::c_int as UWORD,
                    x as libc::c_int, y as libc::c_int);
    //	iV_SetTextColour(-1);
//	sprintf(&str,"%d",i);
//	pie_DrawText(&str, x+6, y-1 );
}
