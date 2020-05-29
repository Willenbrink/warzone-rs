use ::libc;
extern "C" {
    /* Read formatted input from S.  */
    #[no_mangle]
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
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
    fn memFreeRelease(pMemToFree: *mut libc::c_void);
    #[no_mangle]
    fn memMallocRelease(Size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn abort() -> !;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_uint;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_uint)
     -> *mut libc::c_void;
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
    fn resGetData(pType: *mut STRING, pID: *mut STRING) -> *mut libc::c_void;
    /* Return the ID number for an ID string */
    #[no_mangle]
    fn strresGetIDNum(psRes: *mut STR_RES, pIDStr: *mut STRING,
                      pIDNum: *mut UDWORD) -> BOOL;
    /* Get the string from an ID number */
    #[no_mangle]
    fn strresGetString(psRes: *mut STR_RES, id: UDWORD) -> *mut STRING;
    #[no_mangle]
    fn audio_QueueTrackPos(iTrack: SDWORD, iX: SDWORD, iY: SDWORD,
                           iZ: SDWORD);
    /* ******************************************************************************
*		Load stats functions
*******************************************************************************/
/* Return the number of newlines in a file buffer */
    #[no_mangle]
    fn numCR(pFileBuffer: *mut libc::c_char, fileSize: UDWORD) -> UDWORD;
    /* The string resource object */
    #[no_mangle]
    static mut psStringRes: *mut STR_RES;
    #[no_mangle]
    fn addConsoleMessage(messageText: *mut STRING,
                         jusType: CONSOLE_TEXT_JUSTIFICATION) -> BOOL;
    /* **************************************************************************/
    #[no_mangle]
    fn audioID_GetIDFromStr(pWavStr: *mut STRING, piID: *mut SDWORD) -> BOOL;
    #[no_mangle]
    static mut selectedPlayer: UDWORD;
    #[no_mangle]
    fn intRemoveProximityButton(psProxDisp: *mut PROXIMITY_DISPLAY);
    #[no_mangle]
    fn intAddProximityButton(psProxDisp: *mut PROXIMITY_DISPLAY, inc: UDWORD)
     -> BOOL;
    #[no_mangle]
    static mut apsFeatureLists: [*mut FEATURE; 8];
    /* The maximum map size */
    /* The size and contents of the map */
    #[no_mangle]
    static mut mapWidth: UDWORD;
    #[no_mangle]
    static mut mapHeight: UDWORD;
    #[no_mangle]
    static mut psMapTiles: *mut MAPTILE;
}
pub type size_t = libc::c_uint;
pub type UBYTE = libc::c_uchar;
pub type STRING = libc::c_char;
pub type UWORD = libc::c_ushort;
pub type SWORD = libc::c_short;
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
// The next free ID
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _viewdata {
    pub pName: *mut STRING,
    pub type_0: VIEW_TYPE,
    pub numText: UBYTE,
    pub ppTextMsg: *mut *mut STRING,
    pub pData: *mut libc::c_void,
}
// Droids
// All Buildings
// Things like roads, trees, bridges, fires
// Comes out of guns, stupid :-)
// for the camera tracking
/*
 * messageDef.h
 *
 * Message structure definitions
 */
// Research message
// Campaign message
// Mission Report messages
// Proximity message
pub type VIEW_TYPE = _view_type;
pub type _view_type = libc::c_uint;
pub const VIEW_TYPES: _view_type = 4;
pub const VIEW_RPLX: _view_type = 3;
pub const VIEW_PROX: _view_type = 2;
pub const VIEW_RPL: _view_type = 1;
pub const VIEW_RES: _view_type = 0;
pub type BASE_OBJECT = _base_object;
pub type _position_type = libc::c_uint;
pub const POS_TEMPDELIVERY: _position_type = 3;
pub const POS_PROXOBJ: _position_type = 2;
pub const POS_PROXDATA: _position_type = 1;
pub const POS_DELIVERY: _position_type = 0;
pub type POSITION_TYPE = _position_type;
// research view
// full screen view sequence - flic
// proximity view - no view really!
// full screen view sequence - flic.	extended format
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
pub type _message_type = libc::c_uint;
pub const MSG_TYPES: _message_type = 4;
pub const MSG_PROXIMITY: _message_type = 3;
pub const MSG_MISSION: _message_type = 2;
pub const MSG_CAMPAIGN: _message_type = 1;
pub const MSG_RESEARCH: _message_type = 0;
pub type MESSAGE_TYPE = _message_type;
pub type _prox_type = libc::c_uint;
// Feature armour
//artefact proximity message
pub const PROX_TYPES: _prox_type = 3;
//resource proximity message
pub const PROX_ARTEFACT: _prox_type = 2;
//enemy proximity message
pub const PROX_RESOURCE: _prox_type = 1;
pub const PROX_ENEMY: _prox_type = 0;
pub type PROX_TYPE = _prox_type;
// info required to view an object in Intelligence screen
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _view_research {
    pub pIMD: *mut iIMDShape,
    pub pIMD2: *mut iIMDShape,
    pub sequenceName: [STRING; 256],
    pub pAudio: *mut STRING,
    pub numFrames: UWORD,
}
pub type VIEW_RESEARCH = _view_research;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _seq_display {
    pub sequenceName: [STRING; 256],
    pub flag: UBYTE,
    pub numText: UBYTE,
    pub ppTextMsg: *mut *mut STRING,
    pub pAudio: *mut STRING,
    pub numFrames: UWORD,
}
pub type SEQ_DISPLAY = _seq_display;
/* On PSX if type is VIEW_RPL then
											this is used as a number_of_frames_in_the_stream
											count - NOT used on PC*/
/* On PSX if type is VIEW_RPL then
								this is used as a number_of_frames_in_the_stream
								count - NOT used on PC*/
//info required to view a flic in Intelligence Screen
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _view_replay {
    pub numSeq: UBYTE,
    pub pSeqList: *mut SEQ_DISPLAY,
}
pub type VIEW_REPLAY = _view_replay;
//STRING		**ppSeqName;
	//UBYTE		numText;	//the number of textmessages associated with this sequence
	//STRING		**ppTextMsg;	//Pointer to text messages - if any
// info required to view a proximity message
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
/*ID of the audio track to play - if any */
/*the data required to view - either a
							  VIEW_RESEARCH, VIEW_PROXIMITY or VIEW_REPLAY*/
//base structure for each message
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
//pointer to the next in the list
//used to display the proximity messages
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _viewData_list {
    pub psViewData: *mut VIEWDATA,
    pub numViewData: UBYTE,
    pub psNext: *mut _viewData_list,
}
pub type VIEWDATA_LIST = _viewData_list;
pub const NO_SOUND: C2RustUnnamed = -1;
pub const ID_MAX_SOUND: C2RustUnnamed = 354;
pub const ID_SOUND_ARTIFACT: C2RustUnnamed = 28;
pub const ID_SOUND_RESOURCE_HERE: C2RustUnnamed = 21;
pub type CONSOLE_TEXT_JUSTIFICATION = libc::c_uint;
pub const DEFAULT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 3;
pub const CENTRE_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 2;
pub const RIGHT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 1;
pub const LEFT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 0;
//pointer to the next in the list
//next array of data
/* Information stored with each tile */
// The name is now changed to MAPTILE to allow correct compilation on the PlayStation
pub type MAPTILE = _maptile;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _maptile {
    pub tileInfoBits: UBYTE,
    pub tileVisBits: UBYTE,
    pub height: UBYTE,
    pub illumination: UBYTE,
    pub texture: UWORD,
    pub bMaxed: UBYTE,
    pub level: UBYTE,
    pub inRange: UBYTE,
}
pub type C2RustUnnamed = libc::c_int;
pub const ID_SOUND_LAS_SAT_COUNTDOWN: C2RustUnnamed = 353;
pub const ID_SOUND_UPLINK: C2RustUnnamed = 352;
pub const ID_SOUND_PLASMA_FLAMER: C2RustUnnamed = 351;
pub const ID_SOUND_LASER_HEAVY: C2RustUnnamed = 350;
pub const ID_SOUND_EMP: C2RustUnnamed = 349;
pub const ID_SOUND_CYBORG_HEAVY: C2RustUnnamed = 348;
pub const ID_SOUND_CYBORG_GROUND: C2RustUnnamed = 347;
pub const ID_SOUND_NEXUS_UNIT_NEUTRALISED: C2RustUnnamed = 346;
pub const ID_SOUND_NEXUS_UNIT_ABSORBED: C2RustUnnamed = 345;
pub const ID_SOUND_NEXUS_SYNAPTIC_LINK: C2RustUnnamed = 344;
pub const ID_SOUND_NEXUS_STRUCTURE_NEUTRALISED: C2RustUnnamed = 343;
pub const ID_SOUND_NEXUS_STRUCTURE_ABSORBED: C2RustUnnamed = 342;
pub const ID_SOUND_NEXUS_RESEARCH_ABSORBED: C2RustUnnamed = 341;
pub const ID_SOUND_NEXUS_PRODUCTION_COMPLETED: C2RustUnnamed = 340;
pub const ID_SOUND_NEXUS_LAUGH3: C2RustUnnamed = 339;
pub const ID_SOUND_NEXUS_LAUGH2: C2RustUnnamed = 338;
pub const ID_SOUND_NEXUS_LAUGH1: C2RustUnnamed = 337;
pub const ID_SOUND_NEXUS_DEFENCES_NEUTRALISED: C2RustUnnamed = 336;
pub const ID_SOUND_NEXUS_DEFENCES_ABSORBED: C2RustUnnamed = 335;
pub const ID_SOUND_INCOMING_LASER_SAT_STRIKE: C2RustUnnamed = 334;
pub const ID_SOUND_LASER_SATELLITE_FIRING: C2RustUnnamed = 333;
pub const ID_SOUND_TEAM_GAMMA_RESCUED: C2RustUnnamed = 332;
pub const ID_SOUND_TEAM_BETA_RESCUED: C2RustUnnamed = 331;
pub const ID_SOUND_TEAM_ALPHA_RESCUED: C2RustUnnamed = 330;
pub const ID_SOUND_TEAM_GAMMA_ERADICATED: C2RustUnnamed = 329;
pub const ID_SOUND_TEAM_BETA_ERADICATED: C2RustUnnamed = 328;
pub const ID_SOUND_TEAM_ALPHA_ERADICATED: C2RustUnnamed = 327;
pub const ID_SOUND_ENEMY_TRANSPORT_LANDING: C2RustUnnamed = 326;
pub const ID_SOUND_ENEMY_ESCAPING: C2RustUnnamed = 325;
pub const ID_SOUND_ENEMY_ESCAPED: C2RustUnnamed = 324;
pub const ID_SOUND_OUT_OF_TIME: C2RustUnnamed = 323;
pub const ID_SOUND_GROUP_INFECTED: C2RustUnnamed = 322;
pub const ID_SOUND_STRUCTURE_INFECTED: C2RustUnnamed = 321;
pub const ID_SOUND_OBJECTIVE_DESTROYED: C2RustUnnamed = 320;
pub const ID_SOUND_OBJECTIVE_CAPTURED: C2RustUnnamed = 319;
pub const ID_SOUND_GROUP_CAPTURED: C2RustUnnamed = 318;
pub const ID_SOUND_GROUP_RESCUED: C2RustUnnamed = 317;
pub const ID_SOUND_UNITS_RESCUED: C2RustUnnamed = 316;
pub const ID_SOUND_CIVILIANS_RESCUED: C2RustUnnamed = 315;
pub const ID_SOUND_CIVILIAN_RESCUED: C2RustUnnamed = 314;
pub const ID_SOUND_STRUCTURE_CAPTURED: C2RustUnnamed = 313;
pub const ID_SOUND_MISSILE_NME_DETECTED: C2RustUnnamed = 312;
pub const ID_SOUND_MISSILE_SILO: C2RustUnnamed = 311;
pub const ID_SOUND_SAM_SITE: C2RustUnnamed = 310;
pub const ID_SOUND_NUCLEAR_REACTOR: C2RustUnnamed = 309;
pub const ID_SOUND_NASDA_CENTRAL: C2RustUnnamed = 308;
pub const ID_SOUND_SATELLITE_UPLINK: C2RustUnnamed = 307;
pub const ID_SOUND_LANDING_ZONE: C2RustUnnamed = 306;
pub const ID_SOUND_NOFAULTS: C2RustUnnamed = 305;
pub const ID_SOUND_OF_SILENCE: C2RustUnnamed = 304;
pub const ID_SOUND_BARB_SCREAM3: C2RustUnnamed = 303;
pub const ID_SOUND_BARB_SCREAM2: C2RustUnnamed = 302;
pub const ID_SOUND_BARB_SCREAM: C2RustUnnamed = 301;
pub const ID_SOUND_HELP: C2RustUnnamed = 300;
pub const ID_SOUND_FIRE_ROAR: C2RustUnnamed = 299;
pub const ID_SOUND_ECM_TOWER: C2RustUnnamed = 298;
pub const ID_SOUND_STEAM: C2RustUnnamed = 297;
pub const ID_SOUND_POWER_SPARK: C2RustUnnamed = 296;
pub const ID_SOUND_POWER_HUM: C2RustUnnamed = 295;
pub const ID_SOUND_OIL_PUMP_2: C2RustUnnamed = 294;
pub const ID_SOUND_CYBORG_MOVE: C2RustUnnamed = 293;
pub const ID_SOUND_HOVER_STOP: C2RustUnnamed = 292;
pub const ID_SOUND_HOVER_START: C2RustUnnamed = 291;
pub const ID_SOUND_HOVER_MOVE: C2RustUnnamed = 290;
pub const ID_SOUND_TREAD: C2RustUnnamed = 289;
pub const ID_SOUND_VTOL_MOVE: C2RustUnnamed = 288;
pub const ID_SOUND_VTOL_OFF: C2RustUnnamed = 287;
pub const ID_SOUND_VTOL_LAND: C2RustUnnamed = 286;
pub const ID_SOUND_BLIMP_TAKE_OFF: C2RustUnnamed = 285;
pub const ID_SOUND_BLIMP_LAND: C2RustUnnamed = 284;
pub const ID_SOUND_BLIMP_IDLE: C2RustUnnamed = 283;
pub const ID_SOUND_BLIMP_FLIGHT: C2RustUnnamed = 282;
pub const ID_SOUND_CONSTRUCTOR_SHUTDOWN: C2RustUnnamed = 281;
pub const ID_SOUND_CONSTRUCTOR_MOVE: C2RustUnnamed = 280;
pub const ID_SOUND_CONSTRUCTOR_MOVEOFF: C2RustUnnamed = 279;
pub const ID_SOUND_NEXUS_EXPLOSION: C2RustUnnamed = 278;
pub const ID_SOUND_BUILDING_FALL: C2RustUnnamed = 277;
pub const ID_SOUND_BARB_SQUISH: C2RustUnnamed = 276;
pub const ID_SOUND_RICOCHET_3: C2RustUnnamed = 275;
pub const ID_SOUND_RICOCHET_2: C2RustUnnamed = 274;
pub const ID_SOUND_RICOCHET_1: C2RustUnnamed = 273;
pub const ID_SOUND_EXPLOSION_ANTITANK: C2RustUnnamed = 272;
pub const ID_SOUND_EXPLOSION: C2RustUnnamed = 271;
pub const ID_SOUND_EXPLOSION_LASER: C2RustUnnamed = 270;
pub const ID_SOUND_EXPLOSION_SMALL: C2RustUnnamed = 269;
pub const ID_SOUND_CONSTRUCTION_4: C2RustUnnamed = 268;
pub const ID_SOUND_CONSTRUCTION_3: C2RustUnnamed = 267;
pub const ID_SOUND_CONSTRUCTION_2: C2RustUnnamed = 266;
pub const ID_SOUND_CONSTRUCTION_1: C2RustUnnamed = 265;
pub const ID_SOUND_CONSTRUCTION_LOOP: C2RustUnnamed = 264;
pub const ID_SOUND_CONSTRUCTION_START: C2RustUnnamed = 263;
pub const ID_SOUND_WELD_2: C2RustUnnamed = 262;
pub const ID_SOUND_WELD_1: C2RustUnnamed = 261;
pub const ID_SOUND_NEXUS_TOWER: C2RustUnnamed = 260;
pub const ID_SOUND_HIVEL_CANNON: C2RustUnnamed = 259;
pub const ID_SOUND_RAPID_CANNON: C2RustUnnamed = 258;
pub const ID_SOUND_ASSAULT_MG: C2RustUnnamed = 257;
pub const ID_SOUND_SPLASH: C2RustUnnamed = 256;
pub const ID_SOUND_BABA_MG_TOWER: C2RustUnnamed = 255;
pub const ID_SOUND_BABA_MG_HEAVY: C2RustUnnamed = 254;
pub const ID_SOUND_BABA_MG_3: C2RustUnnamed = 253;
pub const ID_SOUND_BABA_MG_2: C2RustUnnamed = 252;
pub const ID_SOUND_BABA_MG_1: C2RustUnnamed = 251;
pub const ID_SOUND_HOWITZ_FLIGHT: C2RustUnnamed = 250;
pub const ID_SOUND_MORTAR: C2RustUnnamed = 249;
pub const ID_SOUND_BEAM_LASER: C2RustUnnamed = 248;
pub const ID_SOUND_PULSE_LASER: C2RustUnnamed = 247;
pub const ID_SOUND_FLAME_THROWER: C2RustUnnamed = 246;
pub const ID_SOUND_MEDIUM_CANNON: C2RustUnnamed = 245;
pub const ID_SOUND_SMALL_CANNON: C2RustUnnamed = 244;
pub const ID_SOUND_LARGE_CANNON: C2RustUnnamed = 243;
pub const ID_SOUND_GAUSSGUN: C2RustUnnamed = 242;
pub const ID_SOUND_ROTARY_LASER: C2RustUnnamed = 241;
pub const ID_SOUND_ROCKET: C2RustUnnamed = 240;
pub const ID_SOUND_COLL_ENEMY_DESTROYED: C2RustUnnamed = 239;
pub const ID_SOUND_COLL_INTERCEPT_AND_DESTROY: C2RustUnnamed = 238;
pub const ID_SOUND_COLL_DIE: C2RustUnnamed = 237;
pub const ID_SOUND_COLL_STARTING_ATTACK_RUN: C2RustUnnamed = 236;
pub const ID_SOUND_COLL_ENGAGING: C2RustUnnamed = 235;
pub const ID_SOUND_COLL_ENEMY_DETECTED: C2RustUnnamed = 234;
pub const ID_SOUND_COLL_FIRE: C2RustUnnamed = 233;
pub const ID_SOUND_COLL_ATTACK: C2RustUnnamed = 232;
pub const ID_SOUND_COLL_DESTROYING_BIOLOGICALS: C2RustUnnamed = 231;
pub const ID_SOUND_COLL_CLEANSE_AND_DESTROY: C2RustUnnamed = 230;
pub const ID_SOUND_ABORTING_ATTACK_RUN3: C2RustUnnamed = 229;
pub const ID_SOUND_COMMENCING_ATTACK_RUN3: C2RustUnnamed = 228;
pub const ID_SOUND_LOCKED_ON3: C2RustUnnamed = 227;
pub const ID_SOUND_RETURNING_TO_BASE3: C2RustUnnamed = 226;
pub const ID_SOUND_ON_OUR_WAY3: C2RustUnnamed = 225;
pub const ID_SOUND_ENEMY_LOCATED3: C2RustUnnamed = 224;
pub const ID_SOUND_ABORTING_ATTACK_RUN2: C2RustUnnamed = 223;
pub const ID_SOUND_COMMENCING_ATTACK_RUN2: C2RustUnnamed = 222;
pub const ID_SOUND_LOCKED_ON2: C2RustUnnamed = 221;
pub const ID_SOUND_RETURNING_TO_BASE2: C2RustUnnamed = 220;
pub const ID_SOUND_ON_OUR_WAY2: C2RustUnnamed = 219;
pub const ID_SOUND_ENEMY_LOCATED2: C2RustUnnamed = 218;
pub const ID_SOUND_ABORTING_ATTACK_RUN1: C2RustUnnamed = 217;
pub const ID_SOUND_COMMENCING_ATTACK_RUN1: C2RustUnnamed = 216;
pub const ID_SOUND_LOCKED_ON1: C2RustUnnamed = 215;
pub const ID_SOUND_RETURNING_TO_BASE1: C2RustUnnamed = 214;
pub const ID_SOUND_ON_OUR_WAY1: C2RustUnnamed = 213;
pub const ID_SOUND_ENEMY_LOCATED1: C2RustUnnamed = 212;
pub const ID_SOUND_PREPARE_FOR_DUST_OFF: C2RustUnnamed = 211;
pub const ID_SOUND_GO_GO_GO: C2RustUnnamed = 210;
pub const ID_SOUND_GREEN_LIGHT_IN_2: C2RustUnnamed = 209;
pub const ID_SOUND_GREEN_LIGHT_IN_3: C2RustUnnamed = 208;
pub const ID_SOUND_GREEN_LIGHT_IN_4: C2RustUnnamed = 207;
pub const ID_SOUND_GREEN_LIGHT_IN_5: C2RustUnnamed = 206;
pub const ID_SOUND_ALRIGHT_BOYS: C2RustUnnamed = 205;
pub const ID_SOUND_APPROACHING_LZ: C2RustUnnamed = 204;
pub const ID_SOUND_RADIOCLICK_6: C2RustUnnamed = 203;
pub const ID_SOUND_RADIOCLICK_5: C2RustUnnamed = 202;
pub const ID_SOUND_RADIOCLICK_4: C2RustUnnamed = 201;
pub const ID_SOUND_RADIOCLICK_3: C2RustUnnamed = 200;
pub const ID_SOUND_RADIOCLICK_2: C2RustUnnamed = 199;
pub const ID_SOUND_RADIOCLICK_1: C2RustUnnamed = 198;
pub const ID_SOUND_COM_HEADING_FOR_RALLY_POINT: C2RustUnnamed = 197;
pub const ID_SOUND_COM_RETURNING_FOR_REPAIR: C2RustUnnamed = 196;
pub const ID_SOUND_COM_UNABLE_TO_COMPLY: C2RustUnnamed = 195;
pub const ID_SOUND_COM_NO_ROUTE_AVAILABLE: C2RustUnnamed = 194;
pub const ID_SOUND_COM_ROUTE_OBSTRUCTED: C2RustUnnamed = 193;
pub const ID_SOUND_COM_ENEMY_VTOLS_DETECTED: C2RustUnnamed = 192;
pub const ID_SOUND_COM_ENEMY_BATTERY_DETECTED: C2RustUnnamed = 191;
pub const ID_SOUND_COM_NEXUS_DETECTED: C2RustUnnamed = 190;
pub const ID_SOUND_COM_NEXUS_TURRET_DETECTED: C2RustUnnamed = 189;
pub const ID_SOUND_COM_NEXUS_TOWER_DETECTED: C2RustUnnamed = 188;
pub const ID_SOUND_COM_FRIENDLY_LZ_DETECTED: C2RustUnnamed = 187;
pub const ID_SOUND_COM_ENEMY_LZ_DETECTED: C2RustUnnamed = 186;
pub const ID_SOUND_COM_ENEMY_TRANSPORT_DETECTED: C2RustUnnamed = 185;
pub const ID_SOUND_COM_ALLY_DETECTED: C2RustUnnamed = 184;
pub const ID_SOUND_COM_ENEMY_BASE_DETECTED: C2RustUnnamed = 183;
pub const ID_SOUND_COM_ENEMY_DETECTED: C2RustUnnamed = 182;
pub const ID_SOUND_COM_ARTEFACT_DETECTED: C2RustUnnamed = 181;
pub const ID_SOUND_COM_RESOURCE_DETECTED: C2RustUnnamed = 180;
pub const ID_SOUND_COM_SCAV_OUTPOST_DETECTED: C2RustUnnamed = 179;
pub const ID_SOUND_COM_SCAV_BASE_DETECTED: C2RustUnnamed = 178;
pub const ID_SOUND_COM_SCAVS_DETECTED: C2RustUnnamed = 177;
pub const ID_SOUND_COMMANDER: C2RustUnnamed = 176;
pub const ID_SOUND_REPORTING: C2RustUnnamed = 175;
pub const ID_SOUND_GROUP_9: C2RustUnnamed = 174;
pub const ID_SOUND_GROUP_8: C2RustUnnamed = 173;
pub const ID_SOUND_GROUP_7: C2RustUnnamed = 172;
pub const ID_SOUND_GROUP_6: C2RustUnnamed = 171;
pub const ID_SOUND_GROUP_5: C2RustUnnamed = 170;
pub const ID_SOUND_GROUP_4: C2RustUnnamed = 169;
pub const ID_SOUND_GROUP_3: C2RustUnnamed = 168;
pub const ID_SOUND_GROUP_2: C2RustUnnamed = 167;
pub const ID_SOUND_GROUP_1: C2RustUnnamed = 166;
pub const ID_SOUND_GROUP_0: C2RustUnnamed = 165;
pub const ID_SOUND_GROUP: C2RustUnnamed = 164;
pub const ID_UNITS_TRANSFER: C2RustUnnamed = 163;
pub const ID_TECHNOLOGY_TRANSFER: C2RustUnnamed = 162;
pub const ID_SENSOR_DOWNLOAD: C2RustUnnamed = 161;
pub const ID_POWER_TRANSMIT: C2RustUnnamed = 160;
pub const ID_GIFT: C2RustUnnamed = 159;
pub const ID_CLAN_EXIT: C2RustUnnamed = 158;
pub const ID_CLAN_ENTER: C2RustUnnamed = 157;
pub const ID_ALLIANCE_OFF: C2RustUnnamed = 156;
pub const ID_ALLIANCE_BRO: C2RustUnnamed = 155;
pub const ID_ALLIANCE_ACC: C2RustUnnamed = 154;
pub const ID_SOUND_ENTERING_WARZONE: C2RustUnnamed = 153;
pub const ID_SOUND_MISSILE_CODES_CRACKED: C2RustUnnamed = 152;
pub const ID_SOUND_3RD_MISSILE_CODES_DECIPHERED: C2RustUnnamed = 151;
pub const ID_SOUND_2ND_MISSILE_CODES_DECIPHERED: C2RustUnnamed = 150;
pub const ID_SOUND_1ST_MISSILE_CODES_DECIPHERED: C2RustUnnamed = 149;
pub const ID_SOUND_MISSILE_CODES_DECIPHERED: C2RustUnnamed = 148;
pub const ID_SOUND_YOU_ARE_DEFEATED: C2RustUnnamed = 147;
pub const ID_SOUND_SYSTEM_FAILURE_IMMINENT: C2RustUnnamed = 146;
pub const ID_SOUND_UNIT_CAPTURED: C2RustUnnamed = 145;
pub const ID_SOUND_1_MINUTE_REMAINING: C2RustUnnamed = 144;
pub const ID_SOUND_2_MINUTES_REMAINING: C2RustUnnamed = 143;
pub const ID_SOUND_3_MINUTES_REMAINING: C2RustUnnamed = 142;
pub const ID_SOUND_5_MINUTES_REMAINING: C2RustUnnamed = 141;
pub const ID_SOUND_10_MINUTES_REMAINING: C2RustUnnamed = 140;
pub const ID_SOUND_MISSION_TIMER_ACTIVATED: C2RustUnnamed = 139;
pub const ID_SOUND_OBJECTIVE_FAILED: C2RustUnnamed = 138;
pub const ID_SOUND_OBJECTIVE_ACCOMPLISHED: C2RustUnnamed = 137;
pub const ID_SOUND_MISSION_SUCCESSFUL: C2RustUnnamed = 136;
pub const ID_SOUND_MISSION_FAILED: C2RustUnnamed = 135;
pub const ID_SOUND_INCOMING_INTELLIGENCE_REPORT: C2RustUnnamed = 134;
pub const ID_SOUND_INCOMING_TRANSMISSION: C2RustUnnamed = 133;
pub const ID_SOUND_TECHNOLOGY_TAKEN: C2RustUnnamed = 132;
pub const ID_SOUND_RESEARCH_STOLEN: C2RustUnnamed = 131;
pub const ID_SOUND_MISSION_RESULTS: C2RustUnnamed = 130;
pub const ID_SOUND_WARZONE_ACTIVE: C2RustUnnamed = 129;
pub const ID_SOUND_WARZONE_PAUSED: C2RustUnnamed = 128;
pub const ID_SOUND_MISSION_UPDATE: C2RustUnnamed = 127;
pub const ID_SOUND_MISSION_OBJECTIVE: C2RustUnnamed = 126;
pub const ID_SOUND_TRANSPORT_UNABLE_TO_LAND: C2RustUnnamed = 125;
pub const ID_SOUND_TRANSPORT_RETURNING_TO_BASE: C2RustUnnamed = 124;
pub const ID_SOUND_LZ_CLEAR: C2RustUnnamed = 123;
pub const ID_SOUND_LZ_COMPROMISED: C2RustUnnamed = 122;
pub const ID_SOUND_TRANSPORT_REPAIRING: C2RustUnnamed = 121;
pub const ID_SOUND_TRANSPORT_UNDER_ATTACK: C2RustUnnamed = 120;
pub const ID_SOUND_TRANSPORT_LANDING: C2RustUnnamed = 119;
pub const ID_SOUND_REINFORCEMENTS_IN_TRANSIT: C2RustUnnamed = 118;
pub const ID_SOUND_REINFORCEMENTS_AVAILABLE: C2RustUnnamed = 117;
pub const ID_SOUND_NO_ROUTE_AVAILABLE: C2RustUnnamed = 116;
pub const ID_SOUND_ROUTE_OBSTRUCTED: C2RustUnnamed = 115;
pub const ID_SOUND_COMMANDER_REPORTING: C2RustUnnamed = 114;
pub const ID_SOUND_GROUP_REPORTING: C2RustUnnamed = 113;
pub const ID_SOUND_ASSIGNED_TO_COMMANDER: C2RustUnnamed = 112;
pub const ID_SOUND_EXCELLENT: C2RustUnnamed = 111;
pub const ID_SOUND_WELL_DONE: C2RustUnnamed = 110;
pub const ID_SOUND_THAT_IS_INCORRECT: C2RustUnnamed = 109;
pub const ID_SOUND_NO: C2RustUnnamed = 108;
pub const ID_SOUND_NOT_POSSIBLE_TRY_AGAIN: C2RustUnnamed = 107;
pub const ID_SOUND_SCATTER: C2RustUnnamed = 106;
pub const ID_SOUND_RECYCLING: C2RustUnnamed = 105;
pub const ID_SOUND_RETURN_TO_LZ: C2RustUnnamed = 104;
pub const ID_SOUND_PATROL: C2RustUnnamed = 103;
pub const ID_SOUND_PURSUE: C2RustUnnamed = 102;
pub const ID_SOUND_GUARD: C2RustUnnamed = 101;
pub const ID_SOUND_HOLD_POSITION: C2RustUnnamed = 100;
pub const ID_SOUND_CEASEFIRE: C2RustUnnamed = 99;
pub const ID_SOUND_RETURN_FIRE: C2RustUnnamed = 98;
pub const ID_SOUND_FIRE_AT_WILL: C2RustUnnamed = 97;
pub const ID_SOUND_NO_RETREAT: C2RustUnnamed = 96;
pub const ID_SOUND_RETREAT_AT_HEAVY_DAMAGE: C2RustUnnamed = 95;
pub const ID_SOUND_RETREAT_AT_MEDIUM_DAMAGE: C2RustUnnamed = 94;
pub const ID_SOUND_OPTIMUM_RANGE: C2RustUnnamed = 93;
pub const ID_SOUND_LONG_RANGE: C2RustUnnamed = 92;
pub const ID_SOUND_SHORT_RANGE: C2RustUnnamed = 91;
pub const ID_SOUND_COMMAND_CONSOLE_ACTIVATED: C2RustUnnamed = 90;
pub const ID_SOUND_INTERCEPTORS_ASSIGNED: C2RustUnnamed = 89;
pub const ID_SOUND_ASSIGNED: C2RustUnnamed = 88;
pub const ID_SOUND_VTOLS_ENGAGING: C2RustUnnamed = 87;
pub const ID_SOUND_REARMING: C2RustUnnamed = 86;
pub const ID_SOUND_INTERCEPTORS_LAUNCHED: C2RustUnnamed = 85;
pub const ID_SOUND_BATTERY_FIRING_COUNTER_ATTACK: C2RustUnnamed = 84;
pub const ID_SOUND_ENEMY_BATTERY_LOCATED: C2RustUnnamed = 83;
pub const ID_SOUND_ASSIGNED_TO_COUNTER_RADAR: C2RustUnnamed = 82;
pub const ID_SOUND_SENSOR_LOCKED_ON: C2RustUnnamed = 81;
pub const ID_SOUND_ASSIGNED_TO_SENSOR: C2RustUnnamed = 80;
pub const ID_SOUND_UNIT_RETURNING_FOR_REPAIR: C2RustUnnamed = 79;
pub const ID_SOUND_UNIT_RETREATING: C2RustUnnamed = 78;
pub const ID_SOUND_UNIT_DESTROYED: C2RustUnnamed = 77;
pub const ID_SOUND_UNIT_UNDER_ATTACK: C2RustUnnamed = 76;
pub const ID_SOUND_LZ2: C2RustUnnamed = 75;
pub const ID_SOUND_LZ1: C2RustUnnamed = 74;
pub const ID_SOUND_ENEMY_LZ: C2RustUnnamed = 73;
pub const ID_SOUND_INCOMING_ENEMY_TRANSPORT: C2RustUnnamed = 72;
pub const ID_SOUND_ENEMY_BASE_ERADICATED: C2RustUnnamed = 71;
pub const ID_SOUND_ENEMY_BASE: C2RustUnnamed = 70;
pub const ID_SOUND_SCAVENGER_BASE_ERADICATED: C2RustUnnamed = 69;
pub const ID_SOUND_SCAVENGER_OUTPOST_ERADICATED: C2RustUnnamed = 68;
pub const ID_SOUND_SCAVENGER_OUTPOST: C2RustUnnamed = 67;
pub const ID_SOUND_SCAVENGER_BASE: C2RustUnnamed = 66;
pub const ID_SOUND_ENEMY_VTOLS_DETECTED: C2RustUnnamed = 65;
pub const ID_SOUND_ENEMY_BATTERY_DETECTED: C2RustUnnamed = 64;
pub const ID_SOUND_NEXUS_UNIT_DETECTED: C2RustUnnamed = 63;
pub const ID_SOUND_NEXUS_TURRET_DETECTED: C2RustUnnamed = 62;
pub const ID_SOUND_NEXUS_TOWER_DETECTED: C2RustUnnamed = 61;
pub const ID_SOUND_FRIENDLY_LZ_DETECTED: C2RustUnnamed = 60;
pub const ID_SOUND_ENEMY_LZ_DETECTED: C2RustUnnamed = 59;
pub const ID_SOUND_ENEMY_TRANSPORT_DETECTED: C2RustUnnamed = 58;
pub const ID_SOUND_ALLY_DETECTED: C2RustUnnamed = 57;
pub const ID_SOUND_ENEMY_BASE_DETECTED: C2RustUnnamed = 56;
pub const ID_SOUND_ENEMY_UNIT_DETECTED: C2RustUnnamed = 55;
pub const ID_SOUND_ARTEFACT_DISC: C2RustUnnamed = 54;
pub const ID_SOUND_POWER_RESOURCE: C2RustUnnamed = 53;
pub const ID_SOUND_SCAVENGER_OUTPOST_DETECTED: C2RustUnnamed = 52;
pub const ID_SOUND_SCAVENGER_BASE_DETECTED: C2RustUnnamed = 51;
pub const ID_SOUND_SCAVENGERS_DETECTED: C2RustUnnamed = 50;
pub const ID_SOUND_UNIT_REPAIRED: C2RustUnnamed = 49;
pub const ID_SOUND_DELIVERY_POINT_ASSIGNED_TO: C2RustUnnamed = 48;
pub const ID_SOUND_DELIVERY_POINT_ASSIGNED: C2RustUnnamed = 47;
pub const ID_SOUND_PRODUCTION_CANCELLED: C2RustUnnamed = 46;
pub const ID_SOUND_PRODUCTION_PAUSED: C2RustUnnamed = 45;
pub const ID_SOUND_DROID_COMPLETED: C2RustUnnamed = 44;
pub const ID_SOUND_PRODUCTION_STARTED: C2RustUnnamed = 43;
pub const ID_SOUND_CYBORG_RESEARCH_COMPLETED: C2RustUnnamed = 42;
pub const ID_SOUND_WEAPON_RESEARCH_COMPLETED: C2RustUnnamed = 41;
pub const ID_SOUND_SYSTEMS_RESEARCH_COMPLETED: C2RustUnnamed = 40;
pub const ID_SOUND_VEHICLE_RESEARCH_COMPLETED: C2RustUnnamed = 39;
pub const ID_SOUND_COMPUTER_RESEARCH_COMPLETED: C2RustUnnamed = 38;
pub const ID_SOUND_POWER_RESEARCH_COMPLETED: C2RustUnnamed = 37;
pub const ID_SOUND_STRUCTURE_RESEARCH_COMPLETED: C2RustUnnamed = 36;
pub const ID_SOUND_MAJOR_RESEARCH: C2RustUnnamed = 35;
pub const ID_SOUND_RESEARCH_COMPLETED: C2RustUnnamed = 34;
pub const ID_SOUND_NEW_CYBORG_AVAILABLE: C2RustUnnamed = 33;
pub const ID_SOUND_NEW_COMPONENT_AVAILABLE: C2RustUnnamed = 32;
pub const ID_SOUND_NEW_STRUCTURE_AVAILABLE: C2RustUnnamed = 31;
pub const ID_SOUND_NEW_RESEARCH_PROJ_AVAILABLE: C2RustUnnamed = 30;
pub const ID_SOUND_ARTIFACT_RECOVERED: C2RustUnnamed = 29;
pub const ID_SOUND_RESEARCH_FACILITY_REQUIRED: C2RustUnnamed = 27;
pub const ID_SOUND_POWER_GENERATOR_REQUIRED: C2RustUnnamed = 26;
pub const ID_SOUND_POWER_TRANSFER_IN_PROGRESS: C2RustUnnamed = 25;
pub const ID_SOUND_RESOURCE_DEPLETED: C2RustUnnamed = 24;
pub const ID_SOUND_DERRICK_DESTROYED: C2RustUnnamed = 23;
pub const ID_SOUND_DERRICK_UNDER_ATTACK: C2RustUnnamed = 22;
pub const ID_SOUND_POWER_LOW: C2RustUnnamed = 20;
pub const ID_SOUND_POWER_GENERATOR_DESTROYED: C2RustUnnamed = 19;
pub const ID_SOUND_POWER_GENERATOR_UNDER_ATTACK: C2RustUnnamed = 18;
pub const ID_SOUND_STRUCTURE_DEMOLISHED: C2RustUnnamed = 17;
pub const ID_SOUND_STRUCTURE_REPAIR_IN_PROGRESS: C2RustUnnamed = 16;
pub const ID_SOUND_STRUCTURE_UNDER_ATTACK: C2RustUnnamed = 15;
pub const ID_SOUND_STRUCTURE_COMPLETED: C2RustUnnamed = 14;
pub const ID_SOUND_CONSTRUCTION_STARTED: C2RustUnnamed = 13;
pub const ID_SOUND_DESIGN_COMPLETED: C2RustUnnamed = 12;
pub const ID_SOUND_PROPULSION_SELECTED: C2RustUnnamed = 11;
pub const ID_SOUND_BODY_SELECTED: C2RustUnnamed = 10;
pub const ID_SOUND_TURRET_SELECTED: C2RustUnnamed = 9;
pub const ID_SOUND_GAME_SHUTDOWN: C2RustUnnamed = 8;
pub const ID_SOUND_MESSAGEEND: C2RustUnnamed = 7;
pub const ID_SOUND_BUILD_FAIL: C2RustUnnamed = 6;
pub const ID_SOUND_ZOOM_ON_RADAR: C2RustUnnamed = 5;
pub const FE_AUDIO_MESSAGEEND: C2RustUnnamed = 4;
pub const ID_SOUND_BUTTON_CLICK_5: C2RustUnnamed = 3;
pub const ID_SOUND_SELECT: C2RustUnnamed = 2;
pub const ID_SOUND_WINDOWOPEN: C2RustUnnamed = 1;
pub const ID_SOUND_WINDOWCLOSE: C2RustUnnamed = 0;
/*#ifndef PSX
	UBYTE			tileExtraBits;	// We've got more than you... We've got more than you..;-)
#endif*/
// COMPRESSED - bit per player
/*#ifndef PSX
	UBYTE			tileDoorBits;   // same thing - bit per player
#endif*/
// The height at the top left of the tile
// How bright is this tile?
// Which graphics texture is on this tile
// sensor range display.
// This is also used to store the tile flip flags
//  What's been removed - 46 bytes per tile so far
//	BASE_OBJECT		*psObject;		// Any object sitting on the location (e.g. building)
//	UBYTE			onFire;			// Is tile on fire?
//	UBYTE			rippleIndex;	// Current value in ripple table?
//	BOOL			tileVisible[MAX_PLAYERS]; // Which players can see the tile?
//	BOOL			triangleFlip;	// Is the triangle flipped?
//	TYPE_OF_TERRAIN	type;			// The terrain type for the tile
/* The shift on the y coord when calculating into the map */
/* The number of units accross a tile */
/* The shift on a coordinate to get the tile coordinate */
/* The mask to get internal tile coords from a full coordinate */
/* Shutdown the map module */
/* Create a new map of a specified size */
/* Load the map data */
/* Save the map data */
/* Load map texture info */
/* Save the current map texture info */
/* A post process for the water tiles in map to ensure height integrity */
/* Return a pointer to the tile structure at x,y */
#[inline]
unsafe extern "C" fn mapTile(mut x: UDWORD, mut y: UDWORD) -> *mut MAPTILE {
    if x < mapWidth {
    } else {
        debug(LOG_ERROR,
              b"mapTile: x coordinate bigger than map width\x00" as *const u8
                  as *const libc::c_char);
    };
    if x < mapWidth {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"./map.h\x00" as *const u8 as *const libc::c_char,
              285 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 8],
                                        &[libc::c_char; 8]>(b"mapTile\x00")).as_ptr(),
              b"x < mapWidth\x00" as *const u8 as *const libc::c_char);
    };
    if y < mapHeight {
    } else {
        debug(LOG_ERROR,
              b"mapTile: y coordinate bigger than map height\x00" as *const u8
                  as *const libc::c_char);
    };
    if y < mapHeight {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"./map.h\x00" as *const u8 as *const libc::c_char,
              287 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 8],
                                        &[libc::c_char; 8]>(b"mapTile\x00")).as_ptr(),
              b"y < mapHeight\x00" as *const u8 as *const libc::c_char);
    };
    //return psMapTiles + x + (y << mapShift); //width no longer a power of 2
    return psMapTiles.offset(x as
                                 isize).offset(y.wrapping_mul(mapWidth) as
                                                   isize);
}
//VIEWDATA			*asViewData;
//UDWORD			numViewData;
//array of pointers for the view data
#[no_mangle]
pub static mut apsViewData: *mut VIEWDATA_LIST =
    0 as *const VIEWDATA_LIST as *mut VIEWDATA_LIST;
/* The memory heaps for the messages and viewData*/
#[no_mangle]
pub static mut psMsgHeap: *mut OBJ_HEAP =
    0 as *const OBJ_HEAP as *mut OBJ_HEAP;
#[no_mangle]
pub static mut psViewDataHeap: *mut OBJ_HEAP =
    0 as *const OBJ_HEAP as *mut OBJ_HEAP;
/* The memory heap for the proximity displays */
#[no_mangle]
pub static mut psProxDispHeap: *mut OBJ_HEAP =
    0 as *const OBJ_HEAP as *mut OBJ_HEAP;
/* The id number for the next message allocated
 * Each message will have a unique id number irrespective of type
 */
#[no_mangle]
pub static mut msgID: UDWORD = 0 as libc::c_int as UDWORD;
static mut currentNumProxDisplays: libc::c_int = 0;
/* The list of messages allocated */
#[no_mangle]
pub static mut apsMessages: [*mut MESSAGE; 8] =
    [0 as *const MESSAGE as *mut MESSAGE; 8];
/* The list of proximity displays allocated */
#[no_mangle]
pub static mut apsProxDisp: [*mut PROXIMITY_DISPLAY; 8] =
    [0 as *const PROXIMITY_DISPLAY as *mut PROXIMITY_DISPLAY; 8];
/* The current tutorial message - there is only ever one at a time. They are displayed
when called by the script. They are not to be re-displayed*/
//MESSAGE		tutorialMessage;
/* The IMD to use for the proximity messages */
#[no_mangle]
pub static mut pProximityMsgIMD: *mut iIMDShape =
    0 as *const iIMDShape as *mut iIMDShape;
/* Creating a new message
 * new is a pointer to a pointer to the new message
 * type is the type of the message
 */
// ajl modified for netgames
/* Add the message to the BOTTOM of the list
 * list is a pointer to the message list
 * Order is now CAMPAIGN, MISSION, RESEARCH/PROXIMITY
 */
/*add to bottom of the list*/
/*add it before the first campaign message */
/*add it before the first mission message */
#[no_mangle]
pub unsafe extern "C" fn add_msg(mut list: *mut *mut MESSAGE,
                                 mut msg: *mut MESSAGE, mut player: UDWORD) {
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"addMessage: Invalid message pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"message.c\x00" as *const u8 as *const libc::c_char,
              144 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 8],
                                        &[libc::c_char; 8]>(b"add_msg\x00")).as_ptr(),
              b"PTRVALID((msg), sizeof(MESSAGE))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (*list.offset(player as isize)).is_null() {
        let ref mut fresh0 = *list.offset(player as isize);
        *fresh0 = msg;
        (*msg).psNext = 0 as *mut _message
    } else {
        let mut psCurr: *mut MESSAGE = 0 as *mut MESSAGE;
        let mut psPrev: *mut MESSAGE = 0 as *mut MESSAGE;
        psPrev = 0 as *mut MESSAGE;
        psCurr = psPrev;
        match (*msg).type_0 as libc::c_uint {
            1 => {
                /*add it before the first mission/research/prox message */
                psCurr = *list.offset(player as isize);
                while !psCurr.is_null() {
                    if (*psCurr).type_0 as libc::c_uint ==
                           MSG_MISSION as libc::c_int as libc::c_uint ||
                           (*psCurr).type_0 as libc::c_uint ==
                               MSG_RESEARCH as libc::c_int as libc::c_uint ||
                           (*psCurr).type_0 as libc::c_uint ==
                               MSG_PROXIMITY as libc::c_int as libc::c_uint {
                        break ;
                    }
                    psPrev = psCurr;
                    psCurr = (*psCurr).psNext
                }
                if !psPrev.is_null() {
                    (*psPrev).psNext = msg;
                    (*msg).psNext = psCurr
                } else {
                    //must be top of list
                    psPrev = *list.offset(player as isize);
                    let ref mut fresh1 = *list.offset(player as isize);
                    *fresh1 = msg;
                    (*msg).psNext = psPrev
                }
            }
            2 => {
                /*add it before the first research/prox message */
                psCurr = *list.offset(player as isize);
                while !psCurr.is_null() {
                    if (*psCurr).type_0 as libc::c_uint ==
                           MSG_RESEARCH as libc::c_int as libc::c_uint ||
                           (*psCurr).type_0 as libc::c_uint ==
                               MSG_PROXIMITY as libc::c_int as libc::c_uint {
                        break ;
                    }
                    psPrev = psCurr;
                    psCurr = (*psCurr).psNext
                }
                if !psPrev.is_null() {
                    (*psPrev).psNext = msg;
                    (*msg).psNext = psCurr
                } else {
                    //must be top of list
                    psPrev = *list.offset(player as isize);
                    let ref mut fresh2 = *list.offset(player as isize);
                    *fresh2 = msg;
                    (*msg).psNext = psPrev
                }
            }
            0 | 3 => {
                /*add it to the bottom of the list */
                psCurr = *list.offset(player as isize);
                while !(*psCurr).psNext.is_null() {
                    psCurr = (*psCurr).psNext
                }
                (*psCurr).psNext = msg;
                (*msg).psNext = 0 as *mut _message
            }
            _ => { }
        }
    };
}
/* Remove a message from the list
 * list is a pointer to the message list
 * del is a pointer to the message to remove
*/
#[no_mangle]
pub unsafe extern "C" fn messageInitVars() -> BOOL {
    let mut i: libc::c_int = 0;
    psMsgHeap = 0 as *mut OBJ_HEAP;
    psProxDispHeap = 0 as *mut OBJ_HEAP;
    psViewDataHeap = 0 as *mut OBJ_HEAP;
    msgID = 0 as libc::c_int as UDWORD;
    currentNumProxDisplays = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        apsMessages[i as usize] = 0 as *mut MESSAGE;
        apsProxDisp[i as usize] = 0 as *mut PROXIMITY_DISPLAY;
        i += 1
    }
    pProximityMsgIMD = 0 as *mut iIMDShape;
    return 1 as libc::c_int;
}
//allocates the viewdata heap
#[no_mangle]
pub unsafe extern "C" fn initViewData() -> BOOL {
    //initialise the viewData heap - needs to be done before the data is loaded
    if heapCreate(&mut psViewDataHeap,
                  ::std::mem::size_of::<VIEWDATA>() as libc::c_ulong,
                  5 as libc::c_int as UDWORD, 1 as libc::c_int as UDWORD) == 0
       {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
//destroys the viewdata heap
#[no_mangle]
pub unsafe extern "C" fn viewDataHeapShutDown() {
    heapDestroy(psViewDataHeap);
}
/*Add a message to the list */
#[no_mangle]
pub unsafe extern "C" fn addMessage(mut msgType: UDWORD, mut proxPos: BOOL,
                                    mut player: UDWORD) -> *mut MESSAGE {
    let mut psMsgToAdd: *mut MESSAGE = 0 as *mut MESSAGE;
    //first create a message of the required type
    if heapAlloc(psMsgHeap,
                 &mut psMsgToAdd as *mut *mut MESSAGE as *mut libc::c_void as
                     *mut *mut libc::c_void) != 0 {
        (*psMsgToAdd).type_0 = msgType as MESSAGE_TYPE;
        (*psMsgToAdd).id = msgID << 3 as libc::c_int | selectedPlayer;
        msgID = msgID.wrapping_add(1)
    }
    if psMsgToAdd.is_null() { return 0 as *mut MESSAGE }
    //then add to the players' list
	 //ADD_MSG(apsMessages, psMsgToAdd, player);
    add_msg(apsMessages.as_mut_ptr(), psMsgToAdd, player);
    //initialise the message data
    (*psMsgToAdd).player = player;
    (*psMsgToAdd).pViewData = 0 as *mut MSG_VIEWDATA;
    //psMsgToAdd->frameNumber = 0;
    (*psMsgToAdd).read = 0 as libc::c_int;
    //add a proximity display
    if msgType == MSG_PROXIMITY as libc::c_int as libc::c_uint {
        addProximityDisplay(psMsgToAdd, proxPos, player);
    }
    //	 else
//	 {
//		 //make the reticule button flash as long as not prox msg or multiplayer game.
//#ifndef PSX
//		if (player == selectedPlayer && !bMultiPlayer)
//#else
//		if (player == selectedPlayer )
//#endif
//		{
//			flashReticuleButton(IDRET_INTEL_MAP);
//		}
//	 }
    return psMsgToAdd;
}
//function declarations
/* adds a proximity display - holds varaibles that enable the message to be
 displayed in the Intelligence Screen*/
unsafe extern "C" fn addProximityDisplay(mut psMessage: *mut MESSAGE,
                                         mut proxPos: BOOL,
                                         mut player: UDWORD) {
    let mut psToAdd: *mut PROXIMITY_DISPLAY = 0 as *mut PROXIMITY_DISPLAY;
    //create the proximity display
    if heapAlloc(psProxDispHeap,
                 &mut psToAdd as *mut *mut PROXIMITY_DISPLAY as
                     *mut libc::c_void as *mut *mut libc::c_void) != 0 {
        if proxPos != 0 {
            (*psToAdd).type_0 = POS_PROXOBJ
        } else { (*psToAdd).type_0 = POS_PROXDATA }
        (*psToAdd).psMessage = psMessage;
        (*psToAdd).screenX = 0 as libc::c_int as UDWORD;
        (*psToAdd).screenY = 0 as libc::c_int as UDWORD;
        (*psToAdd).screenR = 0 as libc::c_int as UDWORD;
        (*psToAdd).player = player;
        (*psToAdd).timeLastDrawn = 0 as libc::c_int as UDWORD;
        (*psToAdd).frameNumber = 0 as libc::c_int as UDWORD;
        (*psToAdd).selected = 0 as libc::c_int;
        (*psToAdd).strobe = 0 as libc::c_int as UDWORD
    }
    //now add it to the top of the list
    (*psToAdd).psNext = apsProxDisp[player as usize];
    apsProxDisp[player as usize] = psToAdd;
    //add a button to the interface
    intAddProximityButton(psToAdd, currentNumProxDisplays as UDWORD);
    currentNumProxDisplays += 1;
}
/*remove a message */
#[no_mangle]
pub unsafe extern "C" fn removeMessage(mut psDel: *mut MESSAGE,
                                       mut player: UDWORD) {
    if (*psDel).type_0 as libc::c_uint ==
           MSG_PROXIMITY as libc::c_int as libc::c_uint {
        removeProxDisp(psDel, player);
    }
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"removeMessage: Invalid message pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"message.c\x00" as *const u8 as *const libc::c_char,
              392 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"removeMessage\x00")).as_ptr(),
              b"PTRVALID(psDel, sizeof(MESSAGE))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if apsMessages[player as usize] == psDel {
        apsMessages[player as usize] = (*apsMessages[player as usize]).psNext;
        heapFree(psMsgHeap, psDel as *mut libc::c_void);
    } else {
        let mut psPrev: *mut MESSAGE = 0 as *mut MESSAGE;
        let mut psCurr: *mut MESSAGE = 0 as *mut MESSAGE;
        psCurr = apsMessages[player as usize];
        while psCurr != psDel && !psCurr.is_null() {
            psPrev = psCurr;
            psCurr = (*psCurr).psNext
        }
        if !psCurr.is_null() {
        } else {
            debug(LOG_ERROR,
                  b"removeMessage: message not found\x00" as *const u8 as
                      *const libc::c_char);
        };
        if !psCurr.is_null() {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"message.c\x00" as *const u8 as *const libc::c_char,
                  392 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 14],
                                            &[libc::c_char; 14]>(b"removeMessage\x00")).as_ptr(),
                  b"psCurr != NULL\x00" as *const u8 as *const libc::c_char);
        };
        if !psCurr.is_null() {
            (*psPrev).psNext = (*psCurr).psNext;
            heapFree(psMsgHeap, psDel as *mut libc::c_void);
        }
    };
}
/* remove a proximity display */
unsafe extern "C" fn removeProxDisp(mut psMessage: *mut MESSAGE,
                                    mut player: UDWORD) {
    let mut psCurr: *mut PROXIMITY_DISPLAY = 0 as *mut PROXIMITY_DISPLAY;
    let mut psPrev: *mut PROXIMITY_DISPLAY = 0 as *mut PROXIMITY_DISPLAY;
    //find the proximity display for this message
    if (*apsProxDisp[player as usize]).psMessage == psMessage {
        psCurr = apsProxDisp[player as usize];
        apsProxDisp[player as usize] = (*apsProxDisp[player as usize]).psNext;
        intRemoveProximityButton(psCurr);
        heapFree(psProxDispHeap, psCurr as *mut libc::c_void);
    } else {
        psPrev = apsProxDisp[player as usize];
        psCurr = apsProxDisp[player as usize];
        while !psCurr.is_null() {
            //compare the pointers
            if (*psCurr).psMessage == psMessage {
                (*psPrev).psNext = (*psCurr).psNext;
                intRemoveProximityButton(psCurr);
                heapFree(psProxDispHeap, psCurr as *mut libc::c_void);
                break ;
            } else { psPrev = psCurr; psCurr = (*psCurr).psNext }
        }
    };
}
/* Remove all Messages*/
#[no_mangle]
pub unsafe extern "C" fn freeMessages() {
    releaseAllProxDisp();
    let mut i: UDWORD = 0;
    let mut psCurr: *mut MESSAGE = 0 as *mut MESSAGE;
    let mut psNext: *mut MESSAGE = 0 as *mut MESSAGE;
    i = 0 as libc::c_int as UDWORD;
    while i < 8 as libc::c_int as libc::c_uint {
        psCurr = apsMessages[i as usize];
        while !psCurr.is_null() {
            psNext = (*psCurr).psNext;
            heapFree(psMsgHeap, psCurr as *mut libc::c_void);
            psCurr = psNext
        }
        apsMessages[i as usize] = 0 as *mut MESSAGE;
        i = i.wrapping_add(1)
    };
}
/* removes all the proximity displays */
#[no_mangle]
pub unsafe extern "C" fn releaseAllProxDisp() {
    let mut player: UDWORD = 0;
    let mut psCurr: *mut PROXIMITY_DISPLAY = 0 as *mut PROXIMITY_DISPLAY;
    let mut psNext: *mut PROXIMITY_DISPLAY = 0 as *mut PROXIMITY_DISPLAY;
    player = 0 as libc::c_int as UDWORD;
    while player < 8 as libc::c_int as libc::c_uint {
        psCurr = apsProxDisp[player as usize];
        while !psCurr.is_null() {
            psNext = (*psCurr).psNext;
            //HEAP_FREE(psProxDispHeap, psCurr);
            removeMessage((*psCurr).psMessage, player);
            psCurr = psNext
        }
        apsProxDisp[player as usize] = 0 as *mut PROXIMITY_DISPLAY;
        player = player.wrapping_add(1)
    }
    //remove message associated with this display
    //re-initialise variables
    currentNumProxDisplays = 0 as libc::c_int;
}
/* Initialise the message heaps */
#[no_mangle]
pub unsafe extern "C" fn initMessage() -> BOOL {
    //set up the imd used for proximity messages
    pProximityMsgIMD =
        resGetData(b"IMD\x00" as *const u8 as *const libc::c_char as
                       *mut STRING,
                   b"arrow.pie\x00" as *const u8 as *const libc::c_char as
                       *mut STRING) as *mut iIMDShape;
    if pProximityMsgIMD.is_null() {
        debug(LOG_ERROR,
              b"Unable to load Proximity Message PIE\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    //initialise the tutorial message - only used by scripts
	/*tutorialMessage.id = msgID;
	tutorialMessage.type = MSG_TUTORIAL;
	tutorialMessage.pViewData = NULL;
	tutorialMessage.read = FALSE;
	tutorialMessage.player = MAX_PLAYERS + 1;
	tutorialMessage.psNext = NULL;*/
    if heapCreate(&mut psMsgHeap,
                  ::std::mem::size_of::<MESSAGE>() as libc::c_ulong,
                  20 as libc::c_int as UDWORD, 5 as libc::c_int as UDWORD) ==
           0 {
        return 0 as libc::c_int
    }
    //initialise the proximity display heap
    if heapCreate(&mut psProxDispHeap,
                  ::std::mem::size_of::<PROXIMITY_DISPLAY>() as libc::c_ulong,
                  10 as libc::c_int as UDWORD, 5 as libc::c_int as UDWORD) ==
           0 {
        return 0 as libc::c_int
    }
    //JPS add message to get on screen video
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn addToViewDataList(mut psViewData: *mut VIEWDATA,
                                           mut numData: UBYTE) -> BOOL {
    let mut psAdd: *mut VIEWDATA_LIST = 0 as *mut VIEWDATA_LIST;
    if heapAlloc(psViewDataHeap,
                 &mut psAdd as *mut *mut VIEWDATA_LIST as
                     *mut *mut libc::c_void) != 0 {
        (*psAdd).psViewData = psViewData;
        (*psAdd).numViewData = numData;
        //add to top of list
        (*psAdd).psNext = apsViewData;
        apsViewData = psAdd;
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
/*load the view data for the messages from the file */
#[no_mangle]
pub unsafe extern "C" fn loadViewData(mut pViewMsgData: *mut libc::c_char,
                                      mut bufferSize: UDWORD)
 -> *mut VIEWDATA {
    let mut i: UDWORD = 0;
    let mut id: UDWORD = 0;
    let mut dataInc: UDWORD = 0;
    let mut seqInc: UDWORD = 0;
    let mut numFrames: UDWORD = 0;
    let mut numData: UDWORD = 0;
    let mut count: UDWORD = 0;
    let mut count2: UDWORD = 0;
    let mut psViewData: *mut VIEWDATA = 0 as *mut VIEWDATA;
    let mut pData: *mut VIEWDATA = 0 as *mut VIEWDATA;
    let mut psViewRes: *mut VIEW_RESEARCH = 0 as *mut VIEW_RESEARCH;
    let mut psViewReplay: *mut VIEW_REPLAY = 0 as *mut VIEW_REPLAY;
    let mut name: [STRING; 256] = [0; 256];
    let mut imdName: [STRING; 60] = [0; 60];
    let mut string: [STRING; 256] = [0; 256];
    let mut imdName2: [STRING; 60] = [0; 60];
    let mut audioName: [STRING; 256] = [0; 256];
    let mut LocX: SDWORD = 0;
    let mut LocY: SDWORD = 0;
    let mut LocZ: SDWORD = 0;
    let mut proxType: SDWORD = 0;
    let mut audioID: SDWORD = 0;
    let mut cnt: libc::c_int = 0;
    //keep the start so we release it at the end
	//pData = pViewMsgData;
    numData = numCR(pViewMsgData, bufferSize);
    if numData > 0xff as libc::c_int as libc::c_uint {
        debug(LOG_ERROR,
              b"loadViewData: Didn\'t expect 256 viewData messages!\x00" as
                  *const u8 as *const libc::c_char);
        abort();
    }
    //allocate space for the data
    psViewData =
        memMallocRelease(numData.wrapping_mul(::std::mem::size_of::<VIEWDATA>()
                                                  as libc::c_ulong)) as
            *mut VIEWDATA;
    if psViewData.is_null() {
        debug(LOG_ERROR,
              b"Unable to allocate memory for viewdata\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    //add to array list
    addToViewDataList(psViewData, numData as UBYTE);
    //psViewData = asViewData;
	//save so can pass the value back
    pData = psViewData;
    i = 0 as libc::c_int as UDWORD;
    while i < numData {
        let mut numText: UDWORD = 0;
        memset(psViewData as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<VIEWDATA>() as libc::c_ulong);
        name[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        //read the data into the storage - the data is delimeted using comma's
        sscanf(pViewMsgData,
               b"%[^\',\'],%d%n\x00" as *const u8 as *const libc::c_char,
               name.as_mut_ptr(), &mut numText as *mut UDWORD,
               &mut cnt as *mut libc::c_int);
        pViewMsgData = pViewMsgData.offset(cnt as isize);
        //check not loading up too many text strings
        if numText > 4 as libc::c_int as libc::c_uint {
            debug(LOG_ERROR,
                  b"loadViewData: too many text strings for %s\x00" as
                      *const u8 as *const libc::c_char, (*psViewData).pName);
            abort();
        }
        (*psViewData).numText = numText as UBYTE;
        //allocate storage for the name
        (*psViewData).pName =
            memMallocRelease(strlen(name.as_mut_ptr()).wrapping_add(1 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_uint))
                as *mut STRING;
        if (*psViewData).pName.is_null() {
            debug(LOG_ERROR,
                  b"ViewData Name - Out of memory\x00" as *const u8 as
                      *const libc::c_char);
            abort();
        }
        strcpy((*psViewData).pName, name.as_mut_ptr());
        //allocate space for text strings
        if (*psViewData).numText != 0 {
            (*psViewData).ppTextMsg =
                memMallocRelease(((*psViewData).numText as
                                      libc::c_uint).wrapping_mul(::std::mem::size_of::<*mut STRING>()
                                                                     as
                                                                     libc::c_ulong))
                    as *mut *mut STRING
        }
        //read in the data for the text strings
        dataInc = 0 as libc::c_int as UDWORD;
        while dataInc < (*psViewData).numText as libc::c_uint {
            name[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
            //sscanf(pViewMsgData,"%[^',']", &name);
            sscanf(pViewMsgData,
                   b",%[^\',\']%n\x00" as *const u8 as *const libc::c_char,
                   name.as_mut_ptr(), &mut cnt as *mut libc::c_int);
            pViewMsgData = pViewMsgData.offset(cnt as isize);
            //get the ID for the string
            if strresGetIDNum(psStringRes, name.as_mut_ptr(), &mut id) == 0 {
                debug(LOG_ERROR,
                      b"Cannot find the view data string id %s \x00" as
                          *const u8 as *const libc::c_char,
                      name.as_mut_ptr());
                abort();
            }
            //get the string from the id
            let ref mut fresh3 =
                *(*psViewData).ppTextMsg.offset(dataInc as isize);
            *fresh3 = strresGetString(psStringRes, id);
            dataInc = dataInc.wrapping_add(1)
        }
        //sscanf(pViewMsgData,"%d", &psViewData->type);
        sscanf(pViewMsgData, b",%d%n\x00" as *const u8 as *const libc::c_char,
               &mut (*psViewData).type_0 as *mut VIEW_TYPE as
                   *mut libc::c_int, &mut cnt as *mut libc::c_int);
        pViewMsgData = pViewMsgData.offset(cnt as isize);
        //allocate data according to type
        match (*psViewData).type_0 as libc::c_uint {
            0 => {
                (*psViewData).pData =
                    memMallocRelease(::std::mem::size_of::<VIEW_RESEARCH>() as
                                         libc::c_ulong) as *mut VIEW_RESEARCH
                        as *mut libc::c_void;
                if (*psViewData).pData.is_null() {
                    debug(LOG_ERROR,
                          b"Unable to allocate memory\x00" as *const u8 as
                              *const libc::c_char);
                    abort();
                }
                imdName[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
                imdName2[0 as libc::c_int as usize] =
                    '\u{0}' as i32 as STRING;
                string[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
                audioName[0 as libc::c_int as usize] =
                    '\u{0}' as i32 as STRING;
                //sscanf(pViewMsgData, "%[^','],%[^','],%[^','],%[^','],%d",
			//	&imdName, &imdName2, &string, &audioName, &numFrames);
                sscanf(pViewMsgData,
                       b",%[^\',\'],%[^\',\'],%[^\',\'],%[^\',\'],%d%n\x00" as
                           *const u8 as *const libc::c_char,
                       imdName.as_mut_ptr(), imdName2.as_mut_ptr(),
                       string.as_mut_ptr(), audioName.as_mut_ptr(),
                       &mut numFrames as *mut UDWORD,
                       &mut cnt as *mut libc::c_int);
                pViewMsgData = pViewMsgData.offset(cnt as isize);
                psViewRes = (*psViewData).pData as *mut VIEW_RESEARCH;
                (*psViewRes).pIMD =
                    resGetData(b"IMD\x00" as *const u8 as *const libc::c_char
                                   as *mut STRING, imdName.as_mut_ptr()) as
                        *mut iIMDShape;
                if (*psViewRes).pIMD.is_null() {
                    debug(LOG_ERROR,
                          b"Cannot find the PIE for message %s\x00" as
                              *const u8 as *const libc::c_char,
                          name.as_mut_ptr());
                    abort();
                }
                if strcmp(imdName2.as_mut_ptr(),
                          b"0\x00" as *const u8 as *const libc::c_char) != 0 {
                    (*psViewRes).pIMD2 =
                        resGetData(b"IMD\x00" as *const u8 as
                                       *const libc::c_char as *mut STRING,
                                   imdName2.as_mut_ptr()) as *mut iIMDShape;
                    if (*psViewRes).pIMD2.is_null() {
                        debug(LOG_ERROR,
                              b"Cannot find the 2nd PIE for message %s\x00" as
                                  *const u8 as *const libc::c_char,
                              name.as_mut_ptr());
                        abort();
                    }
                } else { (*psViewRes).pIMD2 = 0 as *mut iIMDShape }
                strcpy((*psViewRes).sequenceName.as_mut_ptr(),
                       string.as_mut_ptr());
                //get the audio text string
                if strcmp(audioName.as_mut_ptr(),
                          b"0\x00" as *const u8 as *const libc::c_char) != 0 {
                    //allocate space
                    (*psViewRes).pAudio =
                        memMallocRelease(strlen(audioName.as_mut_ptr()).wrapping_add(1
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_uint))
                            as *mut STRING;
                    if (*psViewRes).pAudio.is_null() {
                        debug(LOG_ERROR,
                              b"loadViewData - Out of memory\x00" as *const u8
                                  as *const libc::c_char);
                        abort();
                    }
                    strcpy((*psViewRes).pAudio, audioName.as_mut_ptr());
                } else { (*psViewRes).pAudio = 0 as *mut STRING }
                //this is for the PSX only
                (*psViewRes).numFrames = numFrames as UWORD
            }
            1 | 3 => {
                // This is now also used for the stream playing on the PSX
			// NOTE: on the psx the last entry (audioID) is used as the number of frames in the stream
                (*psViewData).pData =
                    memMallocRelease(::std::mem::size_of::<VIEW_REPLAY>() as
                                         libc::c_ulong) as *mut VIEW_REPLAY as
                        *mut libc::c_void;
                if (*psViewData).pData.is_null() {
                    debug(LOG_ERROR,
                          b"Unable to allocate memory\x00" as *const u8 as
                              *const libc::c_char);
                    abort();
                }
                psViewReplay = (*psViewData).pData as *mut VIEW_REPLAY;
                //read in number of sequences for this message
			//sscanf(pViewMsgData, "%d", &psViewReplay->numSeq);
                sscanf(pViewMsgData,
                       b",%d%n\x00" as *const u8 as *const libc::c_char,
                       &mut count as *mut UDWORD,
                       &mut cnt as *mut libc::c_int);
                pViewMsgData = pViewMsgData.offset(cnt as isize);
                if count > 4 as libc::c_int as libc::c_uint {
                    debug(LOG_ERROR,
                          b"loadViewData: too many sequence for %s\x00" as
                              *const u8 as *const libc::c_char,
                          (*psViewData).pName);
                    abort();
                }
                (*psViewReplay).numSeq = count as UBYTE;
                //allocate space for the sequences
                (*psViewReplay).pSeqList =
                    memMallocRelease(((*psViewReplay).numSeq as
                                          libc::c_uint).wrapping_mul(::std::mem::size_of::<SEQ_DISPLAY>()
                                                                         as
                                                                         libc::c_ulong))
                        as *mut SEQ_DISPLAY;
                //read in the data for the sequences
                dataInc = 0 as libc::c_int as UDWORD;
                while dataInc < (*psViewReplay).numSeq as libc::c_uint {
                    name[0 as libc::c_int as usize] =
                        '\u{0}' as i32 as STRING;
                    //load extradat for extended type only
                    if (*psViewData).type_0 as libc::c_uint ==
                           VIEW_RPL as libc::c_int as libc::c_uint {
                        sscanf(pViewMsgData,
                               b",%[^\',\'],%d%n\x00" as *const u8 as
                                   *const libc::c_char, name.as_mut_ptr(),
                               &mut count as *mut UDWORD,
                               &mut cnt as *mut libc::c_int);
                        pViewMsgData = pViewMsgData.offset(cnt as isize);
                        if count > 4 as libc::c_int as libc::c_uint {
                            debug(LOG_ERROR,
                                  b"loadViewData: too many strings for %s\x00"
                                      as *const u8 as *const libc::c_char,
                                  (*psViewData).pName);
                            abort();
                        }
                        (*(*psViewReplay).pSeqList.offset(dataInc as
                                                              isize)).numText
                            = count as UBYTE;
                        //set the flag to default
                        (*(*psViewReplay).pSeqList.offset(dataInc as
                                                              isize)).flag =
                            0 as libc::c_int as UBYTE
                    } else {
                        //extended type
                        sscanf(pViewMsgData,
                               b",%[^\',\'],%d,%d%n\x00" as *const u8 as
                                   *const libc::c_char, name.as_mut_ptr(),
                               &mut count as *mut UDWORD,
                               &mut count2 as *mut UDWORD,
                               &mut cnt as *mut libc::c_int);
                        pViewMsgData = pViewMsgData.offset(cnt as isize);
                        if count > 4 as libc::c_int as libc::c_uint {
                            debug(LOG_ERROR,
                                  b"loadViewData: invalid video playback flag %s\x00"
                                      as *const u8 as *const libc::c_char,
                                  (*psViewData).pName);
                            abort();
                        }
                        (*(*psViewReplay).pSeqList.offset(dataInc as
                                                              isize)).flag =
                            count as UBYTE;
                        //check not loading up too many text strings
                        if count2 > 4 as libc::c_int as libc::c_uint {
                            debug(LOG_ERROR,
                                  b"loadViewData: too many text strings for seq for %s\x00"
                                      as *const u8 as *const libc::c_char,
                                  (*psViewData).pName);
                            abort();
                        }
                        (*(*psViewReplay).pSeqList.offset(dataInc as
                                                              isize)).numText
                            = count2 as UBYTE
                    }
                    strcpy((*(*psViewReplay).pSeqList.offset(dataInc as
                                                                 isize)).sequenceName.as_mut_ptr(),
                           name.as_mut_ptr());
                    //get the text strings for this sequence - if any
				//allocate space for text strings
                    if (*(*psViewReplay).pSeqList.offset(dataInc as
                                                             isize)).numText
                           != 0 {
                        let ref mut fresh4 =
                            (*(*psViewReplay).pSeqList.offset(dataInc as
                                                                  isize)).ppTextMsg;
                        *fresh4 =
                            memMallocRelease(((*(*psViewReplay).pSeqList.offset(dataInc
                                                                                    as
                                                                                    isize)).numText
                                                  as
                                                  libc::c_uint).wrapping_mul(::std::mem::size_of::<*mut STRING>()
                                                                                 as
                                                                                 libc::c_ulong))
                                as *mut *mut STRING
                    }
                    //read in the data for the text strings
                    seqInc = 0 as libc::c_int as UDWORD;
                    while seqInc <
                              (*(*psViewReplay).pSeqList.offset(dataInc as
                                                                    isize)).numText
                                  as libc::c_uint {
                        name[0 as libc::c_int as usize] =
                            '\u{0}' as i32 as STRING;
                        sscanf(pViewMsgData,
                               b",%[^\',\']%n\x00" as *const u8 as
                                   *const libc::c_char, name.as_mut_ptr(),
                               &mut cnt as *mut libc::c_int);
                        pViewMsgData = pViewMsgData.offset(cnt as isize);
                        //get the ID for the string
                        if strresGetIDNum(psStringRes, name.as_mut_ptr(),
                                          &mut id) == 0 {
                            debug(LOG_ERROR,
                                  b"Cannot find the view data string id %s \x00"
                                      as *const u8 as *const libc::c_char,
                                  name.as_mut_ptr());
                            abort();
                        }
                        //get the string from the id
                        let ref mut fresh5 =
                            *(*(*psViewReplay).pSeqList.offset(dataInc as
                                                                   isize)).ppTextMsg.offset(seqInc
                                                                                                as
                                                                                                isize);
                        *fresh5 = strresGetString(psStringRes, id);
                        seqInc = seqInc.wrapping_add(1)
                    }
                    //get the audio text string
                    sscanf(pViewMsgData,
                           b",%[^\',\'],%d%n\x00" as *const u8 as
                               *const libc::c_char, audioName.as_mut_ptr(),
                           &mut count as *mut UDWORD,
                           &mut cnt as *mut libc::c_int);
                    pViewMsgData = pViewMsgData.offset(cnt as isize);
                    if count < 0xffff as libc::c_int as libc::c_uint {
                    } else {
                        debug(LOG_ERROR,
                              b"loadViewData: numFrames too high for %s\x00"
                                  as *const u8 as *const libc::c_char,
                              name.as_mut_ptr());
                    };
                    if count < 0xffff as libc::c_int as libc::c_uint {
                    } else {
                        debug(LOG_ERROR,
                              b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                                  *const u8 as *const libc::c_char,
                              b"message.c\x00" as *const u8 as
                                  *const libc::c_char, 796 as libc::c_int,
                              (*::std::mem::transmute::<&[u8; 13],
                                                        &[libc::c_char; 13]>(b"loadViewData\x00")).as_ptr(),
                              b"count < UWORD_MAX\x00" as *const u8 as
                                  *const libc::c_char);
                    };
                    (*(*psViewReplay).pSeqList.offset(dataInc as
                                                          isize)).numFrames =
                        count as UWORD;
                    if strcmp(audioName.as_mut_ptr(),
                              b"0\x00" as *const u8 as *const libc::c_char) !=
                           0 {
                        //allocate space
                        let ref mut fresh6 =
                            (*(*psViewReplay).pSeqList.offset(dataInc as
                                                                  isize)).pAudio; //no longer need to know if it is extended type
                        *fresh6 =
                            memMallocRelease(strlen(audioName.as_mut_ptr()).wrapping_add(1
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_uint))
                                as *mut STRING;
                        if (*(*psViewReplay).pSeqList.offset(dataInc as
                                                                 isize)).pAudio.is_null()
                           {
                            debug(LOG_ERROR,
                                  b"loadViewData - Out of memory\x00" as
                                      *const u8 as *const libc::c_char);
                            abort();
                        }
                        strcpy((*(*psViewReplay).pSeqList.offset(dataInc as
                                                                     isize)).pAudio,
                               audioName.as_mut_ptr());
                    } else {
                        let ref mut fresh7 =
                            (*(*psViewReplay).pSeqList.offset(dataInc as
                                                                  isize)).pAudio;
                        *fresh7 = 0 as *mut STRING
                    }
                    dataInc = dataInc.wrapping_add(1)
                }
                (*psViewData).type_0 = VIEW_RPL
            }
            2 => {
                (*psViewData).pData =
                    memMallocRelease(::std::mem::size_of::<VIEW_PROXIMITY>()
                                         as libc::c_ulong) as
                        *mut VIEW_PROXIMITY as *mut libc::c_void;
                if (*psViewData).pData.is_null() {
                    debug(LOG_ERROR,
                          b"Unable to allocate memory\x00" as *const u8 as
                              *const libc::c_char);
                    abort();
                }
                audioName[0 as libc::c_int as usize] =
                    '\u{0}' as i32 as STRING;
                sscanf(pViewMsgData,
                       b",%d,%d,%d,%[^\',\'],%d%n\x00" as *const u8 as
                           *const libc::c_char, &mut LocX as *mut SDWORD,
                       &mut LocY as *mut SDWORD, &mut LocZ as *mut SDWORD,
                       audioName.as_mut_ptr(), &mut proxType as *mut SDWORD,
                       &mut cnt as *mut libc::c_int);
                pViewMsgData = pViewMsgData.offset(cnt as isize);
                //allocate audioID
                if strcmp(audioName.as_mut_ptr(),
                          b"0\x00" as *const u8 as *const libc::c_char) ==
                       0 as libc::c_int {
                    audioID = NO_SOUND as libc::c_int
                } else {
                    if audioID_GetIDFromStr(audioName.as_mut_ptr(),
                                            &mut audioID) == 0 as libc::c_int
                       {
                        debug(LOG_ERROR,
                              b"loadViewData: couldn\'t get ID %d for weapon sound %s\x00"
                                  as *const u8 as *const libc::c_char,
                              audioID, audioName.as_mut_ptr());
                        abort();
                    }
                    if (audioID < 0 as libc::c_int ||
                            audioID >= ID_MAX_SOUND as libc::c_int) &&
                           audioID != NO_SOUND as libc::c_int {
                        debug(LOG_ERROR,
                              b"Invalid Weapon Sound ID - %d for weapon %s\x00"
                                  as *const u8 as *const libc::c_char,
                              audioID, audioName.as_mut_ptr());
                        abort();
                    }
                }
                (*((*psViewData).pData as *mut VIEW_PROXIMITY)).audioID =
                    audioID;
                if LocX < 0 as libc::c_int {
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"loadViewData: Negative X coord for prox message - %s\x00"
                                  as *const u8 as *const libc::c_char,
                              name.as_mut_ptr());
                    };
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                                  *const u8 as *const libc::c_char,
                              b"message.c\x00" as *const u8 as
                                  *const libc::c_char, 865 as libc::c_int,
                              (*::std::mem::transmute::<&[u8; 13],
                                                        &[libc::c_char; 13]>(b"loadViewData\x00")).as_ptr(),
                              b"FALSE\x00" as *const u8 as
                                  *const libc::c_char);
                    };
                    return 0 as *mut VIEWDATA
                }
                (*((*psViewData).pData as *mut VIEW_PROXIMITY)).x =
                    LocX as UDWORD;
                if LocY < 0 as libc::c_int {
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"loadViewData: Negative Y coord for prox message - %s\x00"
                                  as *const u8 as *const libc::c_char,
                              name.as_mut_ptr());
                    };
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                                  *const u8 as *const libc::c_char,
                              b"message.c\x00" as *const u8 as
                                  *const libc::c_char, 872 as libc::c_int,
                              (*::std::mem::transmute::<&[u8; 13],
                                                        &[libc::c_char; 13]>(b"loadViewData\x00")).as_ptr(),
                              b"FALSE\x00" as *const u8 as
                                  *const libc::c_char);
                    };
                    return 0 as *mut VIEWDATA
                }
                (*((*psViewData).pData as *mut VIEW_PROXIMITY)).y =
                    LocY as UDWORD;
                if LocZ < 0 as libc::c_int {
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"loadViewData: Negative Z coord for prox message - %s\x00"
                                  as *const u8 as *const libc::c_char,
                              name.as_mut_ptr());
                    };
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                                  *const u8 as *const libc::c_char,
                              b"message.c\x00" as *const u8 as
                                  *const libc::c_char, 879 as libc::c_int,
                              (*::std::mem::transmute::<&[u8; 13],
                                                        &[libc::c_char; 13]>(b"loadViewData\x00")).as_ptr(),
                              b"FALSE\x00" as *const u8 as
                                  *const libc::c_char);
                    };
                    return 0 as *mut VIEWDATA
                }
                (*((*psViewData).pData as *mut VIEW_PROXIMITY)).z =
                    LocZ as UDWORD;
                if proxType > PROX_TYPES as libc::c_int {
                    //printf("proxType %d > %d\n",proxType,PROX_TYPES);
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"Invalid proximity message sub type - %s\x00"
                                  as *const u8 as *const libc::c_char,
                              name.as_mut_ptr());
                    };
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                                  *const u8 as *const libc::c_char,
                              b"message.c\x00" as *const u8 as
                                  *const libc::c_char, 887 as libc::c_int,
                              (*::std::mem::transmute::<&[u8; 13],
                                                        &[libc::c_char; 13]>(b"loadViewData\x00")).as_ptr(),
                              b"FALSE\x00" as *const u8 as
                                  *const libc::c_char);
                    };
                    return 0 as *mut VIEWDATA
                }
                (*((*psViewData).pData as *mut VIEW_PROXIMITY)).proxType =
                    proxType as PROX_TYPE
            }
            _ => {
                debug(LOG_ERROR,
                      b"Unknown ViewData type\x00" as *const u8 as
                          *const libc::c_char);
                abort();
            }
        }
        //increment the pointer to the start of the next record
        pViewMsgData =
            strchr(pViewMsgData,
                   '\n' as i32).offset(1 as libc::c_int as isize);
        //increment the list to the start of the next storage block
        psViewData = psViewData.offset(1);
        i = i.wrapping_add(1)
    }
    //	FREE(pData);
    //return TRUE;
    return pData;
}
/*get the view data identified by the name */
#[no_mangle]
pub unsafe extern "C" fn getViewData(mut pName: *mut STRING)
 -> *mut VIEWDATA {
    //VIEWDATA		*psViewData;// = asViewData;
    let mut psList: *mut VIEWDATA_LIST = 0 as *mut VIEWDATA_LIST;
    //UDWORD			i;
    let mut i: UBYTE = 0;
    if strlen(pName) < 20 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"getViewData: verbose message name\x00" as *const u8 as
                  *const libc::c_char);
    };
    if strlen(pName) < 20 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"message.c\x00" as *const u8 as *const libc::c_char,
              916 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 12],
                                        &[libc::c_char; 12]>(b"getViewData\x00")).as_ptr(),
              b"strlen(pName)< MAX_STR_SIZE\x00" as *const u8 as
                  *const libc::c_char);
    };
    psList = apsViewData;
    while !psList.is_null() {
        i = 0 as libc::c_int as UBYTE;
        while (i as libc::c_int) < (*psList).numViewData as libc::c_int {
            //compare the strings
            if strcmp((*(*psList).psViewData.offset(i as isize)).pName, pName)
                   == 0 {
                //return psViewData;
                return &mut *(*psList).psViewData.offset(i as isize) as
                           *mut VIEWDATA
            }
            i = i.wrapping_add(1)
        }
        psList = (*psList).psNext
    }
    debug(LOG_ERROR,
          b"Unable to find viewdata for message %s\x00" as *const u8 as
              *const libc::c_char, pName);
    abort();
}
/* Release the message heaps */
#[no_mangle]
pub unsafe extern "C" fn messageShutdown() -> BOOL {
    freeMessages();
    heapDestroy(psMsgHeap);
    heapDestroy(psProxDispHeap);
    return 1 as libc::c_int;
}
/* Release the viewdata memory */
#[no_mangle]
pub unsafe extern "C" fn viewDataShutDown(mut psViewData: *mut VIEWDATA) {
    let mut psList: *mut VIEWDATA_LIST = 0 as *mut VIEWDATA_LIST;
    let mut psPrev: *mut VIEWDATA_LIST = 0 as *mut VIEWDATA_LIST;
    //VIEWDATA		*psData;// = asViewData;
	//UDWORD		inc, numData;
    let mut seqInc: UDWORD = 0;
    let mut psViewReplay: *mut VIEW_REPLAY = 0 as *mut VIEW_REPLAY;
    let mut psViewRes: *mut VIEW_RESEARCH = 0 as *mut VIEW_RESEARCH;
    let mut i: UBYTE = 0;
    psPrev = apsViewData;
    psList = apsViewData;
    while !psList.is_null() {
        if (*psList).psViewData == psViewData {
            i = 0 as libc::c_int as UBYTE;
            while (i as libc::c_int) < (*psList).numViewData as libc::c_int {
                psViewData =
                    &mut *(*psList).psViewData.offset(i as isize) as
                        *mut VIEWDATA;
                //check for any messages using this viewdata
                checkMessages(psViewData as *mut MSG_VIEWDATA);
                memFreeRelease((*psViewData).pName as *mut libc::c_void);
                (*psViewData).pName = 0 as *mut STRING;
                //free the space allocated for the text messages
                if (*psViewData).numText != 0 {
                    memFreeRelease((*psViewData).ppTextMsg as
                                       *mut libc::c_void);
                    (*psViewData).ppTextMsg = 0 as *mut *mut STRING
                }
                //free the space allocated for multiple sequences
                if (*psViewData).type_0 as libc::c_uint ==
                       VIEW_RPL as libc::c_int as libc::c_uint {
                    psViewReplay = (*psViewData).pData as *mut VIEW_REPLAY;
                    if (*psViewReplay).numSeq != 0 {
                        seqInc = 0 as libc::c_int as UDWORD;
                        while seqInc < (*psViewReplay).numSeq as libc::c_uint
                              {
                            //free the space allocated for the text messages
                            if (*(*psViewReplay).pSeqList.offset(seqInc as
                                                                     isize)).numText
                                   != 0 {
                                memFreeRelease((*(*psViewReplay).pSeqList.offset(seqInc
                                                                                     as
                                                                                     isize)).ppTextMsg
                                                   as *mut libc::c_void);
                                let ref mut fresh8 =
                                    (*(*psViewReplay).pSeqList.offset(seqInc
                                                                          as
                                                                          isize)).ppTextMsg;
                                *fresh8 = 0 as *mut *mut STRING
                            }
                            if !(*(*psViewReplay).pSeqList.offset(seqInc as
                                                                      isize)).pAudio.is_null()
                               {
                                memFreeRelease((*(*psViewReplay).pSeqList.offset(seqInc
                                                                                     as
                                                                                     isize)).pAudio
                                                   as *mut libc::c_void);
                                let ref mut fresh9 =
                                    (*(*psViewReplay).pSeqList.offset(seqInc
                                                                          as
                                                                          isize)).pAudio;
                                *fresh9 = 0 as *mut STRING
                            }
                            seqInc = seqInc.wrapping_add(1)
                        }
                        memFreeRelease((*psViewReplay).pSeqList as
                                           *mut libc::c_void);
                        (*psViewReplay).pSeqList = 0 as *mut SEQ_DISPLAY
                    }
                } else if (*psViewData).type_0 as libc::c_uint ==
                              VIEW_RES as libc::c_int as libc::c_uint {
                    psViewRes = (*psViewData).pData as *mut VIEW_RESEARCH;
                    if !(*psViewRes).pAudio.is_null() {
                        memFreeRelease((*psViewRes).pAudio as
                                           *mut libc::c_void);
                        (*psViewRes).pAudio = 0 as *mut STRING
                    }
                }
                memFreeRelease((*psViewData).pData);
                (*psViewData).pData = 0 as *mut libc::c_void;
                i = i.wrapping_add(1)
            }
            memFreeRelease((*psList).psViewData as *mut libc::c_void);
            (*psList).psViewData = 0 as *mut VIEWDATA;
            //remove viewData list from the heap
            if psList == apsViewData {
                apsViewData = (*psList).psNext;
                heapFree(psViewDataHeap, psList as *mut libc::c_void);
            } else {
                (*psPrev).psNext = (*psList).psNext;
                heapFree(psViewDataHeap, psList as *mut libc::c_void);
            }
            break ;
        } else { psList = (*psList).psNext }
    }
    psPrev = psList;
}
/* Looks through the players list of messages to find one with the same viewData
pointer and which is the same type of message - used in scriptFuncs */
#[no_mangle]
pub unsafe extern "C" fn findMessage(mut pViewData: *mut MSG_VIEWDATA,
                                     mut type_0: MESSAGE_TYPE,
                                     mut player: UDWORD) -> *mut MESSAGE {
    let mut psCurr: *mut MESSAGE = 0 as *mut MESSAGE;
    psCurr = apsMessages[player as usize];
    while !psCurr.is_null() {
        if (*psCurr).type_0 as libc::c_uint == type_0 as libc::c_uint &&
               (*psCurr).pViewData == pViewData {
            return psCurr
        }
        psCurr = (*psCurr).psNext
    }
    //not found the message so return NULL
    return 0 as *mut MESSAGE;
}
/* 'displays' a proximity display*/
#[no_mangle]
pub unsafe extern "C" fn displayProximityMessage(mut psProxDisp:
                                                     *mut PROXIMITY_DISPLAY) {
    let mut psFeature: *mut FEATURE = 0 as *mut FEATURE;
    let mut psViewData: *mut VIEWDATA = 0 as *mut VIEWDATA;
    let mut psViewProx: *mut VIEW_PROXIMITY = 0 as *mut VIEW_PROXIMITY;
    if (*psProxDisp).type_0 as libc::c_uint ==
           POS_PROXDATA as libc::c_int as libc::c_uint {
        psViewData = (*(*psProxDisp).psMessage).pViewData as *mut VIEWDATA;
        //display text - if any
        if !(*psViewData).ppTextMsg.is_null() {
            addConsoleMessage(*(*psViewData).ppTextMsg.offset(0 as libc::c_int
                                                                  as isize),
                              DEFAULT_JUSTIFY);
        }
        //play message - if any
        psViewProx = (*psViewData).pData as *mut VIEW_PROXIMITY;
        if (*psViewProx).audioID != -(1 as libc::c_int) {
            audio_QueueTrackPos((*psViewProx).audioID,
                                (*psViewProx).x as SDWORD,
                                (*psViewProx).y as SDWORD,
                                (*psViewProx).z as SDWORD);
        }
    } else if (*psProxDisp).type_0 as libc::c_uint ==
                  POS_PROXOBJ as libc::c_int as libc::c_uint {
        if (*((*(*psProxDisp).psMessage).pViewData as
                  *mut BASE_OBJECT)).type_0 as libc::c_uint ==
               OBJ_FEATURE as libc::c_int as libc::c_uint {
        } else {
            debug(LOG_ERROR,
                  b"displayProximityMessage: invalid feature\x00" as *const u8
                      as *const libc::c_char);
        };
        if (*((*(*psProxDisp).psMessage).pViewData as
                  *mut BASE_OBJECT)).type_0 as libc::c_uint ==
               OBJ_FEATURE as libc::c_int as libc::c_uint {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"message.c\x00" as *const u8 as *const libc::c_char,
                  1072 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 24],
                                            &[libc::c_char; 24]>(b"displayProximityMessage\x00")).as_ptr(),
                  b"((BASE_OBJECT *)psProxDisp->psMessage->pViewData)->type == OBJ_FEATURE\x00"
                      as *const u8 as *const libc::c_char);
        };
        psFeature = (*(*psProxDisp).psMessage).pViewData as *mut FEATURE;
        if (*(*psFeature).psStats).subType as libc::c_uint ==
               FEAT_OIL_RESOURCE as libc::c_int as libc::c_uint {
            //play default audio message for oil resource
            audio_QueueTrackPos(ID_SOUND_RESOURCE_HERE as libc::c_int,
                                (*psFeature).x as SDWORD,
                                (*psFeature).y as SDWORD,
                                (*psFeature).z as SDWORD);
        } else if (*(*psFeature).psStats).subType as libc::c_uint ==
                      FEAT_GEN_ARTE as libc::c_int as libc::c_uint {
            //play default audio message for artefact
            audio_QueueTrackPos(ID_SOUND_ARTIFACT as libc::c_int,
                                (*psFeature).x as SDWORD,
                                (*psFeature).y as SDWORD,
                                (*psFeature).z as SDWORD);
        }
    }
    //set the read flag
    (*(*psProxDisp).psMessage).read = 1 as libc::c_int;
}
/*void storeProximityScreenCoords(MESSAGE *psMessage, SDWORD x, SDWORD y)
{
	PROXIMITY_DISPLAY		*psProxDisp = NULL;

	psProxDisp = getProximityDisplay(psMessage);
	if (psProxDisp)
	{
		psProxDisp->screenX = x;
		psProxDisp->screenY = y;
	}
	else
	{
		ASSERT( FALSE, "Unable to find proximity display" );
	}
}*/
#[no_mangle]
pub unsafe extern "C" fn getProximityDisplay(mut psMessage: *mut MESSAGE)
 -> *mut PROXIMITY_DISPLAY {
    let mut psCurr: *mut PROXIMITY_DISPLAY = 0 as *mut PROXIMITY_DISPLAY;
    if (*apsProxDisp[(*psMessage).player as usize]).psMessage == psMessage {
        return apsProxDisp[(*psMessage).player as usize]
    } else {
        psCurr = apsProxDisp[(*psMessage).player as usize];
        while !psCurr.is_null() {
            if (*psCurr).psMessage == psMessage { return psCurr }
            psCurr = (*psCurr).psNext
        }
    }
    return 0 as *mut PROXIMITY_DISPLAY;
}
//static void checkMessages(VIEWDATA *psViewData);
//check for any messages using this viewdata and remove them
//void checkMessages(VIEWDATA *psViewData)
unsafe extern "C" fn checkMessages(mut psViewData: *mut MSG_VIEWDATA) {
    let mut psCurr: *mut MESSAGE = 0 as *mut MESSAGE;
    let mut psNext: *mut MESSAGE = 0 as *mut MESSAGE;
    let mut i: UDWORD = 0;
    i = 0 as libc::c_int as UDWORD;
    while i < 8 as libc::c_int as libc::c_uint {
        psCurr = apsMessages[i as usize];
        while !psCurr.is_null() {
            psNext = (*psCurr).psNext;
            if (*psCurr).pViewData == psViewData { removeMessage(psCurr, i); }
            psCurr = psNext
        }
        i = i.wrapping_add(1)
    };
}
/*
 * Message.h
 *
 * Functions for the messages shown in the Intelligence Map View
 */
/* The lists of messages allocated */
/* The current tutorial message - there is only ever one at a time. They are displayed 
when called by the script. They are not to be re-displayed*/
//extern MESSAGE		tutorialMessage;
/* The IMD to use for the proximity messages */
/* The list of proximity displays allocated */
//allocates the viewdata heap
/* Initialise the message heaps */
/* Release the message heaps */
//destroys the viewdata heap
/*Add a messgae to the list */
/*remove a message */
/* Remove all Messages*/
/* removes all the proximity displays */
/*load the view data for the messages from the file exported from the world editor*/
/*get the view data that contains the text message pointer passed in */
/* Release the viewdata memory */
//extern void storeProximityScreenCoords(MESSAGE *psMessage, SDWORD x, SDWORD y);
/* Looks through the players list of messages to find one with the same viewData 
pointer and which is the same type of message - used in scriptFuncs */
/*'displays' a proximity display*/
//add proximity messages for all untapped VISIBLE oil resources
//add proximity messages for all untapped VISIBLE oil resources
#[no_mangle]
pub unsafe extern "C" fn addOilResourceProximities() {
    let mut psFeat: *mut FEATURE = 0 as *mut FEATURE;
    let mut psMessage: *mut MESSAGE = 0 as *mut MESSAGE;
    //look thru the features to find oil resources
    psFeat = apsFeatureLists[0 as libc::c_int as usize];
    while !psFeat.is_null() {
        if (*(*psFeat).psStats).subType as libc::c_uint ==
               FEAT_OIL_RESOURCE as libc::c_int as libc::c_uint {
            //check to see if the feature is visible to the selected player
            if (*psFeat).visible[selectedPlayer as usize] != 0 {
                //if there isn't an oil derrick built on it
                if (*mapTile(((*psFeat).x as libc::c_int >> 7 as libc::c_int)
                                 as UDWORD,
                             ((*psFeat).y as libc::c_int >> 7 as libc::c_int)
                                 as UDWORD)).tileInfoBits as libc::c_int &
                       0x1 as libc::c_int == 0 {
                    //add a proximity message
                    psMessage =
                        addMessage(MSG_PROXIMITY as libc::c_int as UDWORD,
                                   1 as libc::c_int, selectedPlayer);
                    if !psMessage.is_null() {
                        (*psMessage).pViewData = psFeat as *mut MSG_VIEWDATA
                    }
                }
            }
        }
        psFeat = (*psFeat).psNext
    };
}
