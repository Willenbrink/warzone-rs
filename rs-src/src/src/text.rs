use ::libc;
extern "C" {
    /* Create a string resource object */
    #[no_mangle]
    fn strresCreate(ppsRes: *mut *mut STR_RES, init: UDWORD, ext: UDWORD)
     -> BOOL;
    /* Release a string resource object */
    #[no_mangle]
    fn strresDestroy(psRes: *mut STR_RES);
    /* Load a list of string ID's from a memory buffer
 * id == 0 should be a default string which is returned if the
 * requested string is not found.
 */
    #[no_mangle]
    fn strresLoadFixedID(psRes: *mut STR_RES, psID: *mut STR_ID,
                         numID: UDWORD) -> BOOL;
}
pub type UBYTE = libc::c_uchar;
pub type STRING = libc::c_char;
pub type UDWORD = libc::c_uint;
pub type SDWORD = libc::c_int;
pub type BOOL = libc::c_int;
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
/* An ID entry */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _str_id {
    pub id: UDWORD,
    pub pIDStr: *mut STRING,
}
pub type STR_ID = _str_id;
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
// The next free ID
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
pub const STR_INT_POWERACCRUED: _fixed_str_id = 54;
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
// Other interface strings
pub const STR_INT_BLDPROGRESS: _fixed_str_id = 43;
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
/* The fixed id buffer for the string resource system
 * This just tells it what the keywords are for the fixed strings
 */
#[no_mangle]
pub static mut asFixedID: [STR_ID; 442] =
    [{
         let mut init =
             _str_id{id: STR_DEFAULT as libc::c_int as UDWORD,
                     pIDStr:
                         b"DEFAULT\x00" as *const u8 as *const libc::c_char as
                             *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_RET_OPTIONS as libc::c_int as UDWORD,
                     pIDStr:
                         b"RET_OPTIONS\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_RET_INTELLIGENCE as libc::c_int as UDWORD,
                     pIDStr:
                         b"RET_INTELLIGENCE\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_RET_MANUFACTURE as libc::c_int as UDWORD,
                     pIDStr:
                         b"RET_MANUFACTURE\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_RET_DESIGN as libc::c_int as UDWORD,
                     pIDStr:
                         b"RET_DESIGN\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_RET_RESEARCH as libc::c_int as UDWORD,
                     pIDStr:
                         b"RET_RESEARCH\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_RET_BUILD as libc::c_int as UDWORD,
                     pIDStr:
                         b"RET_BUILD\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_RET_CLOSE as libc::c_int as UDWORD,
                     pIDStr:
                         b"RET_CLOSE\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_RET_COMMAND as libc::c_int as UDWORD,
                     pIDStr:
                         b"RET_COMMAND\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MISC_CLOSE as libc::c_int as UDWORD,
                     pIDStr:
                         b"MISC_CLOSE\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MISC_LOADGAME as libc::c_int as UDWORD,
                     pIDStr:
                         b"MISC_LOADGAME\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MISC_SAVEGAME as libc::c_int as UDWORD,
                     pIDStr:
                         b"MISC_SAVEGAME\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MISC_QUIT as libc::c_int as UDWORD,
                     pIDStr:
                         b"MISC_QUIT\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MISC_PAUSED as libc::c_int as UDWORD,
                     pIDStr:
                         b"MISC_PAUSED\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_DES_NEWVEH as libc::c_int as UDWORD,
                     pIDStr:
                         b"DES_NEWVEH\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_DES_STORE as libc::c_int as UDWORD,
                     pIDStr:
                         b"DES_STORE\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_DES_CANCEL as libc::c_int as UDWORD,
                     pIDStr:
                         b"DES_CANCEL\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_DES_DEL as libc::c_int as UDWORD,
                     pIDStr:
                         b"DES_DEL\x00" as *const u8 as *const libc::c_char as
                             *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_DES_NEW as libc::c_int as UDWORD,
                     pIDStr:
                         b"DES_NEW\x00" as *const u8 as *const libc::c_char as
                             *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_DES_ARMOUR_KIN as libc::c_int as UDWORD,
                     pIDStr:
                         b"DES_ARMOUR_KIN\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_DES_ARMOUR_HEAT as libc::c_int as UDWORD,
                     pIDStr:
                         b"DES_ARMOUR_HEAT\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_DES_POWER as libc::c_int as UDWORD,
                     pIDStr:
                         b"DES_POWER\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_DES_WEIGHT as libc::c_int as UDWORD,
                     pIDStr:
                         b"DES_WEIGHT\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_DES_POWERUSE as libc::c_int as UDWORD,
                     pIDStr:
                         b"DES_POWERUSE\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_DES_SENSOR_RANGE as libc::c_int as UDWORD,
                     pIDStr:
                         b"DES_SENSOR_RANGE\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_DES_SENSOR_POWER as libc::c_int as UDWORD,
                     pIDStr:
                         b"DES_SENSOR_POWER\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_DES_ECM_POWER as libc::c_int as UDWORD,
                     pIDStr:
                         b"DES_ECM_POWER\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_DES_BUILD_POINTS as libc::c_int as UDWORD,
                     pIDStr:
                         b"DES_BUILD_POINTS\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_DES_RANGE as libc::c_int as UDWORD,
                     pIDStr:
                         b"DES_RANGE\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_DES_DAMAGE as libc::c_int as UDWORD,
                     pIDStr:
                         b"DES_DAMAGE\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_DES_ROF as libc::c_int as UDWORD,
                     pIDStr:
                         b"DES_ROF\x00" as *const u8 as *const libc::c_char as
                             *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_DES_AIR as libc::c_int as UDWORD,
                     pIDStr:
                         b"DES_AIR\x00" as *const u8 as *const libc::c_char as
                             *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_DES_ROAD as libc::c_int as UDWORD,
                     pIDStr:
                         b"DES_ROAD\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_DES_OFFROAD as libc::c_int as UDWORD,
                     pIDStr:
                         b"DES_OFFROAD\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_DES_WATER as libc::c_int as UDWORD,
                     pIDStr:
                         b"DES_WATER\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_DES_WEAPONS as libc::c_int as UDWORD,
                     pIDStr:
                         b"DES_WEAPONS\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_DES_OTHER as libc::c_int as UDWORD,
                     pIDStr:
                         b"DES_OTHER\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_DES_COMMAND as libc::c_int as UDWORD,
                     pIDStr:
                         b"DES_COMMAND\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_DES_BODY as libc::c_int as UDWORD,
                     pIDStr:
                         b"DES_BODY\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_DES_PROPULSION as libc::c_int as UDWORD,
                     pIDStr:
                         b"DES_PROPULSION\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_DES_TURRET as libc::c_int as UDWORD,
                     pIDStr:
                         b"DES_TURRET\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_DES_TEMPPOWER as libc::c_int as UDWORD,
                     pIDStr:
                         b"DES_TEMPPOWER\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_DES_TEMPBODY as libc::c_int as UDWORD,
                     pIDStr:
                         b"DES_TEMPBODY\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_INT_BLDPROGRESS as libc::c_int as UDWORD,
                     pIDStr:
                         b"INT_BLDPROGRESS\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_INT_BLDSPEED as libc::c_int as UDWORD,
                     pIDStr:
                         b"INT_BLDSPEED\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_INT_PWRUSAGE as libc::c_int as UDWORD,
                     pIDStr:
                         b"INT_PWRUSAGE\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_INT_RESMESSAGE as libc::c_int as UDWORD,
                     pIDStr:
                         b"INT_RESMESSAGE\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_INT_GENMESSAGE as libc::c_int as UDWORD,
                     pIDStr:
                         b"INT_GENMESSAGE\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_INT_MISMESSAGE as libc::c_int as UDWORD,
                     pIDStr:
                         b"INT_MISMESSAGE\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_INT_RESCOMPLETED as libc::c_int as UDWORD,
                     pIDStr:
                         b"INT_RESCOMPLETED\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_INT_TRANSPORTER as libc::c_int as UDWORD,
                     pIDStr:
                         b"INT_TRANSPORTER\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_INT_TRANSLAUNCH as libc::c_int as UDWORD,
                     pIDStr:
                         b"INT_TRANSLAUNCH\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_INT_DPOINT as libc::c_int as UDWORD,
                     pIDStr:
                         b"INT_DPOINT\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_INT_LOOP as libc::c_int as UDWORD,
                     pIDStr:
                         b"INT_LOOP\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_INT_POWERACCRUED as libc::c_int as UDWORD,
                     pIDStr:
                         b"INT_POWERACCRUED\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_INT_POWER as libc::c_int as UDWORD,
                     pIDStr:
                         b"INT_POWER\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_PHONENO as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_PHONENO\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_IPADDR as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_IPADDR\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_COM1 as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_COM1\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_COM2 as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_COM2\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_COM3 as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_COM3\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_COM4 as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_COM4\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_CANCEL as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_CANCEL\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_OK as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_OK\x00" as *const u8 as *const libc::c_char as
                             *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_PLAYER as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_PLAYER\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_GAME as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_GAME\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_MAXPLAY as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_MAXPLAY\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_ARENA as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_ARENA\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_CAMPAIGN as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_CAMPAIGN\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_REFRESH as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_REFRESH\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_HOST as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_HOST\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_CLEAR as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_CLEAR\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_DEFAULT as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_DEFAULT\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_LOAD as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_LOAD\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_SAVE as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_SAVE\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_DESIGN as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_DESIGN\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_SEARCHING as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_SEARCHING\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_14400 as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_14400\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_19200 as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_19200\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_56000 as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_56000\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_115200 as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_115200\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_SIDECONNECTION as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_SIDECONNECTION\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_SIDEOPTIONS as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_SIDEOPTIONS\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_SIDEGAMES as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_SIDEGAMES\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_SIDEPLAYERS as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_SIDEPLAYERS\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_SIDEFORCE as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_SIDEFORCE\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_SIDETEMPLATES as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_SIDETEMPLATES\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_SIDEINFO as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_SIDEINFO\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_CHAT as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_CHAT\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_STARTING as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_STARTING\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_ALLIANCES as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_ALLIANCES\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_SCORE as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_SCORE\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_KILLS as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_KILLS\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_PING as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_PING\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_POWLO as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_POWLO\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_POWMED as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_POWMED\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_POWHI as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_POWHI\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_TECH1 as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_TECH1\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_TECH2 as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_TECH2\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_TECH3 as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_TECH3\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_CAMPCLEAN as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_CAMPCLEAN\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_CAMPDEFENCE as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_CAMPDEFENCE\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_CAMPBASE as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_CAMPBASE\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_PLAYERIC as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_PLAYERIC\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_FORCEIC as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_FORCEIC\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_MAPIC as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_MAPIC\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_GAMEIC as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_GAMEIC\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_ALLIANCEN as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_ALLIANCEN\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_ALLIANCEY as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_ALLIANCEY\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_NOLIM as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_NOLIM\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_TIMELIM as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_TIMELIM\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_FRAGLIM as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_FRAGLIM\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_CON_MODEM as libc::c_int as UDWORD,
                     pIDStr:
                         b"CON_MODEM\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_CON_INTERNET as libc::c_int as UDWORD,
                     pIDStr:
                         b"CON_INTERNET\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_CON_LAN as libc::c_int as UDWORD,
                     pIDStr:
                         b"CON_LAN\x00" as *const u8 as *const libc::c_char as
                             *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_CON_CABLE as libc::c_int as UDWORD,
                     pIDStr:
                         b"CON_CABLE\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_CON_MORE as libc::c_int as UDWORD,
                     pIDStr:
                         b"CON_MORE\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_GAMES_GAMES as libc::c_int as UDWORD,
                     pIDStr:
                         b"GAMES_GAMES\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_LABEL_TYPE as libc::c_int as UDWORD,
                     pIDStr:
                         b"LABEL_TYPE\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_LABEL_ALLI as libc::c_int as UDWORD,
                     pIDStr:
                         b"LABEL_ALLI\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_LABEL_TEC as libc::c_int as UDWORD,
                     pIDStr:
                         b"LABEL_TEC\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_LABEL_BASE as libc::c_int as UDWORD,
                     pIDStr:
                         b"LABEL_BASE\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_LABEL_LIMIT as libc::c_int as UDWORD,
                     pIDStr:
                         b"LABEL_LIMIT\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_LABEL_FOG as libc::c_int as UDWORD,
                     pIDStr:
                         b"LABEL_FOG\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_PLAYERS as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_PLAYERS\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_PLAY as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_PLAY\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_COMP as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_COMP\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_COMP_Y as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_COMP_Y\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_COMP_N as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_COMP_N\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_STRLIM as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_STRLIM\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_SKIRMISH as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_SKIRMISH\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_TEAMPLAY as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_TEAMPLAY\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_FOG_ON as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_FOG_ON\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_FOG_OFF as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_FOG_OFF\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_ALLI_STATE as libc::c_int as UDWORD,
                     pIDStr:
                         b"ALLI_STATE\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_ALLI_VIS as libc::c_int as UDWORD,
                     pIDStr:
                         b"ALLI_VIS\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_ALLI_TEC as libc::c_int as UDWORD,
                     pIDStr:
                         b"ALLI_TEC\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_ALLI_DRO as libc::c_int as UDWORD,
                     pIDStr:
                         b"ALLI_DRO\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_ALLI_POW as libc::c_int as UDWORD,
                     pIDStr:
                         b"ALLI_POW\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_ALLI_REQ as libc::c_int as UDWORD,
                     pIDStr:
                         b"ALLI_REQ\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_ALLI_OFF as libc::c_int as UDWORD,
                     pIDStr:
                         b"ALLI_OFF\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_ALLI_BRK as libc::c_int as UDWORD,
                     pIDStr:
                         b"ALLI_BRK\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_ALLI_FRM as libc::c_int as UDWORD,
                     pIDStr:
                         b"ALLI_FRM\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_GIFT_VIS as libc::c_int as UDWORD,
                     pIDStr:
                         b"GIFT_VIS\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_GIFT_DRO as libc::c_int as UDWORD,
                     pIDStr:
                         b"GIFT_DRO\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_GIFT_TEC as libc::c_int as UDWORD,
                     pIDStr:
                         b"GIFT_TEC\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_GIFT_POW as libc::c_int as UDWORD,
                     pIDStr:
                         b"GIFT_POW\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_ARTIF as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_ARTIF\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_COMPATIBLE as libc::c_int as UDWORD,
                     pIDStr:
                         b"COMPATIBLE\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_PLAYER_NAME as libc::c_int as UDWORD,
                     pIDStr:
                         b"PLAYER_NAME\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_GAME_NAME as libc::c_int as UDWORD,
                     pIDStr:
                         b"GAME_NAME\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_GAME_QUIT as libc::c_int as UDWORD,
                     pIDStr:
                         b"GAME_QUIT\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_GAME_RESUME as libc::c_int as UDWORD,
                     pIDStr:
                         b"GAME_RESUME\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_FE_SIDEMAIN as libc::c_int as UDWORD,
                     pIDStr:
                         b"FE_SIDEMAIN\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_FE_SINGLE as libc::c_int as UDWORD,
                     pIDStr:
                         b"FE_SINGLE\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_FE_MULTI as libc::c_int as UDWORD,
                     pIDStr:
                         b"FE_MULTI\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_FE_OPTIONS as libc::c_int as UDWORD,
                     pIDStr:
                         b"FE_OPTIONS\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_FE_INTRO as libc::c_int as UDWORD,
                     pIDStr:
                         b"FE_INTRO\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_FE_QUIT as libc::c_int as UDWORD,
                     pIDStr:
                         b"FE_QUIT\x00" as *const u8 as *const libc::c_char as
                             *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_FE_SIDESINGLE1 as libc::c_int as UDWORD,
                     pIDStr:
                         b"FE_SIDESINGLE1\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_FE_SIDESINGLE2 as libc::c_int as UDWORD,
                     pIDStr:
                         b"FE_SIDESINGLE2\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_FE_SIDETUT as libc::c_int as UDWORD,
                     pIDStr:
                         b"FE_SIDETUT\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_FE_NEW as libc::c_int as UDWORD,
                     pIDStr:
                         b"FE_NEW\x00" as *const u8 as *const libc::c_char as
                             *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_FE_LOAD as libc::c_int as UDWORD,
                     pIDStr:
                         b"FE_LOAD\x00" as *const u8 as *const libc::c_char as
                             *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_FE_TUT as libc::c_int as UDWORD,
                     pIDStr:
                         b"FE_TUT\x00" as *const u8 as *const libc::c_char as
                             *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_FE_FASTPLAY as libc::c_int as UDWORD,
                     pIDStr:
                         b"FE_FASTPLAY\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_FE_RETURN as libc::c_int as UDWORD,
                     pIDStr:
                         b"FE_RETURN\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_FE_SIDEMULTI as libc::c_int as UDWORD,
                     pIDStr:
                         b"FE_SIDEMULTI\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_FE_HOST as libc::c_int as UDWORD,
                     pIDStr:
                         b"FE_HOST\x00" as *const u8 as *const libc::c_char as
                             *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_FE_JOIN as libc::c_int as UDWORD,
                     pIDStr:
                         b"FE_JOIN\x00" as *const u8 as *const libc::c_char as
                             *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_FE_FORCEEDIT as libc::c_int as UDWORD,
                     pIDStr:
                         b"FE_FORCEEDIT\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_FE_SKIRMISH as libc::c_int as UDWORD,
                     pIDStr:
                         b"FE_SKIRMISH\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_FE_SIDEOPTIONS as libc::c_int as UDWORD,
                     pIDStr:
                         b"FE_SIDEOPTIONS\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_FE_MOUSE as libc::c_int as UDWORD,
                     pIDStr:
                         b"FE_MOUSE\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_FE_SCROLL as libc::c_int as UDWORD,
                     pIDStr:
                         b"FE_SCROLL\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_FE_GAMMA as libc::c_int as UDWORD,
                     pIDStr:
                         b"FE_GAMMA\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_FE_FX as libc::c_int as UDWORD,
                     pIDStr:
                         b"FE_FX\x00" as *const u8 as *const libc::c_char as
                             *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_FE_3D_FX as libc::c_int as UDWORD,
                     pIDStr:
                         b"FE_3D_FX\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_FE_MUSIC as libc::c_int as UDWORD,
                     pIDStr:
                         b"FE_MUSIC\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_FE_CLAN as libc::c_int as UDWORD,
                     pIDStr:
                         b"FE_CLAN\x00" as *const u8 as *const libc::c_char as
                             *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_FE_CRAPFOG as libc::c_int as UDWORD,
                     pIDStr:
                         b"FE_CRAPFOG\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_FE_GOODFOG as libc::c_int as UDWORD,
                     pIDStr:
                         b"FE_GOODFOG\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_FE_VIDEO as libc::c_int as UDWORD,
                     pIDStr:
                         b"FE_VIDEO\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_FE_SOFTWARE as libc::c_int as UDWORD,
                     pIDStr:
                         b"FE_SOFTWARE\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_FE_DIRECTX as libc::c_int as UDWORD,
                     pIDStr:
                         b"FE_DIRECTX\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_FE_OPENGL as libc::c_int as UDWORD,
                     pIDStr:
                         b"FE_OPENGL\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_FE_GLIDE as libc::c_int as UDWORD,
                     pIDStr:
                         b"FE_GLIDE\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_FE_GAME as libc::c_int as UDWORD,
                     pIDStr:
                         b"FE_GAME\x00" as *const u8 as *const libc::c_char as
                             *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_FE_GRAPHICS as libc::c_int as UDWORD,
                     pIDStr:
                         b"FE_GRAPHICS\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_FE_AUDIO as libc::c_int as UDWORD,
                     pIDStr:
                         b"FE_AUDIO\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_FE_TEXTURE as libc::c_int as UDWORD,
                     pIDStr:
                         b"FE_TEXTURE\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_FE_EFFECTS as libc::c_int as UDWORD,
                     pIDStr:
                         b"FE_EFFECTS\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_FE_FOG as libc::c_int as UDWORD,
                     pIDStr:
                         b"FE_FOG\x00" as *const u8 as *const libc::c_char as
                             *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_FE_TRANSPARENCY as libc::c_int as UDWORD,
                     pIDStr:
                         b"FE_TRANSPARENCY\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_FE_DIFFICULTY as libc::c_int as UDWORD,
                     pIDStr:
                         b"FE_DIFFICULTY\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_EASY as libc::c_int as UDWORD,
                     pIDStr:
                         b"EASY\x00" as *const u8 as *const libc::c_char as
                             *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_NORMAL as libc::c_int as UDWORD,
                     pIDStr:
                         b"NORMAL\x00" as *const u8 as *const libc::c_char as
                             *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_HARD as libc::c_int as UDWORD,
                     pIDStr:
                         b"HARD\x00" as *const u8 as *const libc::c_char as
                             *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_LEAVE as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_LEAVE\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_JOINING as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_JOINING\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MUL_RESPOND as libc::c_int as UDWORD,
                     pIDStr:
                         b"MUL_RESPOND\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_DORD_RANGE1 as libc::c_int as UDWORD,
                     pIDStr:
                         b"DORD_RANGE1\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_DORD_RANGE2 as libc::c_int as UDWORD,
                     pIDStr:
                         b"DORD_RANGE2\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_DORD_RANGE3 as libc::c_int as UDWORD,
                     pIDStr:
                         b"DORD_RANGE3\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_DORD_REPAIR1 as libc::c_int as UDWORD,
                     pIDStr:
                         b"DORD_REPAIR1\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_DORD_REPAIR2 as libc::c_int as UDWORD,
                     pIDStr:
                         b"DORD_REPAIR2\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_DORD_REPAIR3 as libc::c_int as UDWORD,
                     pIDStr:
                         b"DORD_REPAIR3\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_DORD_FIRE1 as libc::c_int as UDWORD,
                     pIDStr:
                         b"DORD_FIRE1\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_DORD_FIRE2 as libc::c_int as UDWORD,
                     pIDStr:
                         b"DORD_FIRE2\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_DORD_FIRE3 as libc::c_int as UDWORD,
                     pIDStr:
                         b"DORD_FIRE3\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_DORD_PATROL as libc::c_int as UDWORD,
                     pIDStr:
                         b"DORD_PATROL\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_DORD_PERSUE as libc::c_int as UDWORD,
                     pIDStr:
                         b"DORD_PERSUE\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_DORD_GUARD as libc::c_int as UDWORD,
                     pIDStr:
                         b"DORD_GUARD\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_DORD_HOLDPOS as libc::c_int as UDWORD,
                     pIDStr:
                         b"DORD_HOLDPOS\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_DORD_RETREPAIR as libc::c_int as UDWORD,
                     pIDStr:
                         b"DORD_RETREPAIR\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_DORD_RETBASE as libc::c_int as UDWORD,
                     pIDStr:
                         b"DORD_RETBASE\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_DORD_EMBARK as libc::c_int as UDWORD,
                     pIDStr:
                         b"DORD_EMBARK\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_DORD_ARMRECYCLE as libc::c_int as UDWORD,
                     pIDStr:
                         b"DORD_ARMRECYCLE\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_DORD_RECYCLE as libc::c_int as UDWORD,
                     pIDStr:
                         b"DORD_RECYCLE\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_DORD_FACTORY as libc::c_int as UDWORD,
                     pIDStr:
                         b"DORD_FACTORY\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_GAM_DERRICK as libc::c_int as UDWORD,
                     pIDStr:
                         b"GAM_DERRICK\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_GAM_DROIDSTATE as libc::c_int as UDWORD,
                     pIDStr:
                         b"GAM_DROIDSTATE\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_GAM_UNITLOST as libc::c_int as UDWORD,
                     pIDStr:
                         b"GAM_UNITLOST\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_GAM_NORTH as libc::c_int as UDWORD,
                     pIDStr:
                         b"GAM_NORTH\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_GAM_ENERGY as libc::c_int as UDWORD,
                     pIDStr:
                         b"GAM_ENERGY\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_GAM_RESREM as libc::c_int as UDWORD,
                     pIDStr:
                         b"GAM_RESREM\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_GAM_POWCON as libc::c_int as UDWORD,
                     pIDStr:
                         b"GAM_POWCON\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_GAM_STRDAM as libc::c_int as UDWORD,
                     pIDStr:
                         b"GAM_STRDAM\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_GAM_ELECDAM as libc::c_int as UDWORD,
                     pIDStr:
                         b"GAM_ELECDAM\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_GAM_STRREST as libc::c_int as UDWORD,
                     pIDStr:
                         b"GAM_STRREST\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_GAM_GOHQ as libc::c_int as UDWORD,
                     pIDStr:
                         b"GAM_GOHQ\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_GAM_NOHQ as libc::c_int as UDWORD,
                     pIDStr:
                         b"GAM_NOHQ\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_GAM_RESREWARD as libc::c_int as UDWORD,
                     pIDStr:
                         b"GAM_RESREWARD\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_GAM_UNITSEL as libc::c_int as UDWORD,
                     pIDStr:
                         b"GAM_UNITSEL\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_GAM_REINF as libc::c_int as UDWORD,
                     pIDStr:
                         b"GAM_REINF\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_GAM_GAMEOVER as libc::c_int as UDWORD,
                     pIDStr:
                         b"GAM_GAMEOVER\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_GAM_JOINING as libc::c_int as UDWORD,
                     pIDStr:
                         b"GAM_JOINING\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_GAM_REWELEC as libc::c_int as UDWORD,
                     pIDStr:
                         b"GAM_REWELEC\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_GAM_REWPROP as libc::c_int as UDWORD,
                     pIDStr:
                         b"GAM_REWPROP\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_GAM_REWBODY as libc::c_int as UDWORD,
                     pIDStr:
                         b"GAM_REWBODY\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_GAM_REWWEAP as libc::c_int as UDWORD,
                     pIDStr:
                         b"GAM_REWWEAP\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_GAM_REWNOWT as libc::c_int as UDWORD,
                     pIDStr:
                         b"GAM_REWNOWT\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_GAM_REWREPA as libc::c_int as UDWORD,
                     pIDStr:
                         b"GAM_REWREPA\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_GAM_REWREPN as libc::c_int as UDWORD,
                     pIDStr:
                         b"GAM_REWREPN\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_GAM_RESNOTFOUND as libc::c_int as UDWORD,
                     pIDStr:
                         b"GAM_RESNOTFOUND\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_GAM_REPNOTFOUND as libc::c_int as UDWORD,
                     pIDStr:
                         b"GAM_REPNOTFOUND\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_GP_SELECTED as libc::c_int as UDWORD,
                     pIDStr:
                         b"GP_SELECTED\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_GP_ASSIGNED as libc::c_int as UDWORD,
                     pIDStr:
                         b"GP_ASSIGNED\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_GP_CENTERED as libc::c_int as UDWORD,
                     pIDStr:
                         b"GP_CENTERED\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_GP_ALLIGN as libc::c_int as UDWORD,
                     pIDStr:
                         b"GP_ALLIGN\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MR_OBJECTIVE_ACHIEVED as libc::c_int as UDWORD,
                     pIDStr:
                         b"MR_OBJECTIVE_ACHIEVED\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MR_OBJECTIVE_FAILED as libc::c_int as UDWORD,
                     pIDStr:
                         b"MR_OBJECTIVE_FAILED\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MR_SAVE_GAME as libc::c_int as UDWORD,
                     pIDStr:
                         b"MR_SAVE_GAME\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MR_LOAD_GAME as libc::c_int as UDWORD,
                     pIDStr:
                         b"MR_LOAD_GAME\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MR_QUIT_TO_MAIN as libc::c_int as UDWORD,
                     pIDStr:
                         b"MR_QUIT_TO_MAIN\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MR_CONTINUE as libc::c_int as UDWORD,
                     pIDStr:
                         b"MR_CONTINUE\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MR_UNIT_LOSSES as libc::c_int as UDWORD,
                     pIDStr:
                         b"MR_UNIT_LOSSES\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MR_STRUCTURE_LOSSES as libc::c_int as UDWORD,
                     pIDStr:
                         b"MR_STRUCTURE_LOSSES\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MR_FORCE_INFO as libc::c_int as UDWORD,
                     pIDStr:
                         b"MR_FORCE_INFO\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MR_RANKINGS as libc::c_int as UDWORD,
                     pIDStr:
                         b"MR_RANKINGS\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MR_UNITS_BUILT as libc::c_int as UDWORD,
                     pIDStr:
                         b"MR_UNITS_BUILT\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MR_UNITS_KILLED as libc::c_int as UDWORD,
                     pIDStr:
                         b"MR_UNITS_KILLED\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MR_UNITS_LOST as libc::c_int as UDWORD,
                     pIDStr:
                         b"MR_UNITS_LOST\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MR_UNITS_NOW as libc::c_int as UDWORD,
                     pIDStr:
                         b"MR_UNITS_NOW\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MR_AV_UNIT_EL as libc::c_int as UDWORD,
                     pIDStr:
                         b"MR_AV_UNIT_EL\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MR_STR_BUILT as libc::c_int as UDWORD,
                     pIDStr:
                         b"MR_STR_BUILT\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MR_STR_BLOWN_UP as libc::c_int as UDWORD,
                     pIDStr:
                         b"MR_STR_BLOWN_UP\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MR_STR_LOST as libc::c_int as UDWORD,
                     pIDStr:
                         b"MR_STR_LOST\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MR_STR_NOW as libc::c_int as UDWORD,
                     pIDStr:
                         b"MR_STR_NOW\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MR_ARTEFACTS_FOUND as libc::c_int as UDWORD,
                     pIDStr:
                         b"MR_ARTEFACTS_FOUND\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MR_MISSION_TIME as libc::c_int as UDWORD,
                     pIDStr:
                         b"MR_MISSION_TIME\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MR_GAME_TIME as libc::c_int as UDWORD,
                     pIDStr:
                         b"MR_GAME_TIME\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MR_SHOTS_ON_TARGET as libc::c_int as UDWORD,
                     pIDStr:
                         b"MR_SHOTS_ON_TARGET\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MR_SHOTS_OFF_TARGET as libc::c_int as UDWORD,
                     pIDStr:
                         b"MR_SHOTS_OFF_TARGET\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MR_BABAS_RUN_OVER as libc::c_int as UDWORD,
                     pIDStr:
                         b"MR_BABAS_RUN_OVER\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MR_LEVEL_ROOKIE as libc::c_int as UDWORD,
                     pIDStr:
                         b"MR_LEVEL_ROOKIE\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MR_LEVEL_GREEN as libc::c_int as UDWORD,
                     pIDStr:
                         b"MR_LEVEL_GREEN\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MR_LEVEL_TRAINED as libc::c_int as UDWORD,
                     pIDStr:
                         b"MR_LEVEL_TRAINED\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MR_LEVEL_REGULAR as libc::c_int as UDWORD,
                     pIDStr:
                         b"MR_LEVEL_REGULAR\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MR_LEVEL_PROF as libc::c_int as UDWORD,
                     pIDStr:
                         b"MR_LEVEL_PROF\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MR_LEVEL_VETERAN as libc::c_int as UDWORD,
                     pIDStr:
                         b"MR_LEVEL_VETERAN\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MR_LEVEL_CRACK as libc::c_int as UDWORD,
                     pIDStr:
                         b"MR_LEVEL_CRACK\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MR_LEVEL_ELITE as libc::c_int as UDWORD,
                     pIDStr:
                         b"MR_LEVEL_ELITE\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MR_LEVEL_SPECIAL as libc::c_int as UDWORD,
                     pIDStr:
                         b"MR_LEVEL_SPECIAL\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_MR_LEVEL_ACE as libc::c_int as UDWORD,
                     pIDStr:
                         b"MR_LEVEL_ACE\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_DL_LEVEL_ROOKIE as libc::c_int as UDWORD,
                     pIDStr:
                         b"DL_LEVEL_ROOKIE\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_DL_LEVEL_GREEN as libc::c_int as UDWORD,
                     pIDStr:
                         b"DL_LEVEL_GREEN\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_DL_LEVEL_TRAINED as libc::c_int as UDWORD,
                     pIDStr:
                         b"DL_LEVEL_TRAINED\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_DL_LEVEL_REGULAR as libc::c_int as UDWORD,
                     pIDStr:
                         b"DL_LEVEL_REGULAR\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_DL_LEVEL_PROF as libc::c_int as UDWORD,
                     pIDStr:
                         b"DL_LEVEL_PROF\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_DL_LEVEL_VETERAN as libc::c_int as UDWORD,
                     pIDStr:
                         b"DL_LEVEL_VETERAN\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_DL_LEVEL_CRACK as libc::c_int as UDWORD,
                     pIDStr:
                         b"DL_LEVEL_CRACK\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_DL_LEVEL_ELITE as libc::c_int as UDWORD,
                     pIDStr:
                         b"DL_LEVEL_ELITE\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_DL_LEVEL_SPECIAL as libc::c_int as UDWORD,
                     pIDStr:
                         b"DL_LEVEL_SPECIAL\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_DL_LEVEL_ACE as libc::c_int as UDWORD,
                     pIDStr:
                         b"DL_LEVEL_ACE\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_CD_CHANGE as libc::c_int as UDWORD,
                     pIDStr:
                         b"CD_CHANGE\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_CD_CHANGE_1OR2 as libc::c_int as UDWORD,
                     pIDStr:
                         b"CD_CHANGE_1OR2\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_GAME_REPLAY as libc::c_int as UDWORD,
                     pIDStr:
                         b"GAME_REPLAY\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_GAME_SAVED as libc::c_int as UDWORD,
                     pIDStr:
                         b"GAME_SAVED\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_GAME_LOADED as libc::c_int as UDWORD,
                     pIDStr:
                         b"GAME_LOADED\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_DORD_CYBORG_FACTORY as libc::c_int as UDWORD,
                     pIDStr:
                         b"DORD_CYBORG_FACTORY\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_GAME_RESTART as libc::c_int as UDWORD,
                     pIDStr:
                         b"GAME_RESTART\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_GAM_MAXUNITSREACHED as libc::c_int as UDWORD,
                     pIDStr:
                         b"GAM_MAXUNITSREACHED\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_KM_KEYMAP as libc::c_int as UDWORD,
                     pIDStr:
                         b"KM_KEYMAP\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_KM_KEYMAP_SIDE as libc::c_int as UDWORD,
                     pIDStr:
                         b"KM_KEYMAP_SIDE\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_CHOMAN as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_CHOMAN\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_CHORES as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_CHORES\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_CHOBUI as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_CHOBUI\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_CHODES as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_CHODES\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_CHOINT as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_CHOINT\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_CHOCOM as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_CHOCOM\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_TOGRAD as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_TOGRAD\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_TOGCON as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_TOGCON\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_BARS as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_BARS\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_SHOT as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_SHOT\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_PAUSE as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_PAUSE\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_PREV as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_PREV\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_AS1 as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_AS1\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_AS2 as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_AS2\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_AS3 as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_AS3\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_AS4 as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_AS4\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_AS5 as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_AS5\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_AS6 as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_AS6\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_AS7 as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_AS7\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_AS8 as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_AS8\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_AS9 as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_AS9\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_GR1 as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_GR1\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_GR2 as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_GR2\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_GR3 as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_GR3\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_GR4 as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_GR4\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_GR5 as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_GR5\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_GR6 as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_GR6\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_GR7 as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_GR7\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_GR8 as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_GR8\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_GR9 as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_GR9\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_MULOP as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_MULOP\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_AUDON as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_AUDON\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_AUDOFF as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_AUDOFF\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_NORTH as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_NORTH\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_TRACK as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_TRACK\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_OPT as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_OPT\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_RIN as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_RIN\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_ROUT as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_ROUT\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_ZIN as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_ZIN\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_ZOUT as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_ZOUT\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_PF as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_PF\x00" as *const u8 as *const libc::c_char as
                             *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_RL as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_RL\x00" as *const u8 as *const libc::c_char as
                             *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_RP as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_RP\x00" as *const u8 as *const libc::c_char as
                             *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_RR as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_RR\x00" as *const u8 as *const libc::c_char as
                             *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_PB as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_PB\x00" as *const u8 as *const libc::c_char as
                             *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_ORD as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_ORD\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_RESJ as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_RESJ\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_REPJ as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_REPJ\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_OVERL as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_OVERL\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_CENTV as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_CENTV\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_CEASE as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_CEASE\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_UNITJ as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_UNITJ\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_ENGAG as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_ENGAG\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_FAW as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_FAW\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_RTB as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_RTB\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_DEFR as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_DEFR\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_SPLIM as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_SPLIM\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_SHOR as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_SHOR\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_PURS as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_PURS\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_PATR as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_PATR\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_REPA as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_REPA\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_DSTOP as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_DSTOP\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_SENDT as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_SENDT\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_LONGR as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_LONGR\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_SCAT as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_SCAT\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_LDAM as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_LDAM\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_HDAM as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_HDAM\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_NDAM as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_NDAM\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_ACU as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_ACU\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_ABDU as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_ABDU\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_AHTR as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_AHTR\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_AHOV as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_AHOV\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_RECY as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_RECY\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_ASCR as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_ASCR\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_ATR as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_ATR\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_ALL as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_ALL\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_AVTOL as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_AVTOL\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_AWHE as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_AWHE\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_ASIMIL as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_ASIMIL\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_GAM_DERRICK_BURNING as libc::c_int as UDWORD,
                     pIDStr:
                         b"GAM_DERRICK_BURNING\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_SEL_NO_STRUCTURE as libc::c_int as UDWORD,
                     pIDStr:
                         b"SEL_NO_STRUCTURE\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_FE_SSHAKE as libc::c_int as UDWORD,
                     pIDStr:
                         b"FE_SSHAKE\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_FE_MFLIP as libc::c_int as UDWORD,
                     pIDStr:
                         b"FE_MFLIP\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_FE_ON as libc::c_int as UDWORD,
                     pIDStr:
                         b"FE_ON\x00" as *const u8 as *const libc::c_char as
                             *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_FE_OFF as libc::c_int as UDWORD,
                     pIDStr:
                         b"FE_OFF\x00" as *const u8 as *const libc::c_char as
                             *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_FE_GREEN as libc::c_int as UDWORD,
                     pIDStr:
                         b"FE_GREEN\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_FE_ORANGE as libc::c_int as UDWORD,
                     pIDStr:
                         b"FE_ORANGE\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_FE_GREY as libc::c_int as UDWORD,
                     pIDStr:
                         b"FE_GREY\x00" as *const u8 as *const libc::c_char as
                             *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_FE_BLACK as libc::c_int as UDWORD,
                     pIDStr:
                         b"FE_BLACK\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_FE_RED as libc::c_int as UDWORD,
                     pIDStr:
                         b"FE_RED\x00" as *const u8 as *const libc::c_char as
                             *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_FE_BLUE as libc::c_int as UDWORD,
                     pIDStr:
                         b"FE_BLUE\x00" as *const u8 as *const libc::c_char as
                             *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_FE_PINK as libc::c_int as UDWORD,
                     pIDStr:
                         b"FE_PINK\x00" as *const u8 as *const libc::c_char as
                             *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_FE_CYAN as libc::c_int as UDWORD,
                     pIDStr:
                         b"FE_CYAN\x00" as *const u8 as *const libc::c_char as
                             *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_CONJ as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_CONJ\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_SENJ as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_SENJ\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_COMJ as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_COMJ\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_GAM_CONNOTFOUND as libc::c_int as UDWORD,
                     pIDStr:
                         b"GAM_CONNOTFOUND\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_GAM_SENNOTFOUND as libc::c_int as UDWORD,
                     pIDStr:
                         b"GAM_SENNOTFOUND\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_GAM_COMNOTFOUND as libc::c_int as UDWORD,
                     pIDStr:
                         b"GAM_COMNOTFOUND\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_CMD1 as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_CMD1\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_CMD2 as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_CMD2\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_CMD3 as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_CMD3\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_CMD4 as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_CMD4\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_CMD5 as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_CMD5\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_CMD6 as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_CMD6\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_CMD7 as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_CMD7\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_CMD8 as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_CMD8\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_CMD9 as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_CMD9\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_SELFACTORY as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_SELFACTORY\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_SELRESEARCH as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_SELRESEARCH\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_SELPOWER as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_SELPOWER\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_SELCYBORG as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_SELCYBORG\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_CONSOLE as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_CONSOLE\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_UP as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_UP\x00" as *const u8 as *const libc::c_char as
                             *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_DOWN as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_DOWN\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_RIGHT as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_RIGHT\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_LEFT as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_LEFT\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_SPEED_UP as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_SPEED_UP\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_SLOW_DOWN as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_SLOW_DOWN\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_NORMAL_SPEED as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_NORMAL_SPEED\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_GAM_SPEED_UP as libc::c_int as UDWORD,
                     pIDStr:
                         b"GAM_SPEED_UP\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_GAM_SLOW_DOWN as libc::c_int as UDWORD,
                     pIDStr:
                         b"GAM_SLOW_DOWN\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_GAM_NORMAL_SPEED as libc::c_int as UDWORD,
                     pIDStr:
                         b"GAM_NORMAL_SPEED\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_RELOAD as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_RELOAD\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_RADJUMP as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_RADJUMP\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_DORD_FIREDES as libc::c_int as UDWORD,
                     pIDStr:
                         b"DORD_FIREDES\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_DORD_VTOL_FACTORY as libc::c_int as UDWORD,
                     pIDStr:
                         b"DORD_VTOL_FACTORY\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_FE_SUBTITLES as libc::c_int as UDWORD,
                     pIDStr:
                         b"FE_SUBTITLES\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_BIND_REOPEN_BUILD as libc::c_int as UDWORD,
                     pIDStr:
                         b"BIND_REOPEN_BUILD\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_GAM_BUILD_REOPEN as libc::c_int as UDWORD,
                     pIDStr:
                         b"GAM_BUILD_REOPEN\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_GAM_BUILD_NO_REOPEN as libc::c_int as UDWORD,
                     pIDStr:
                         b"GAM_BUILD_NO_REOPEN\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_GAM_FORMATION_ON as libc::c_int as UDWORD,
                     pIDStr:
                         b"GAM_FORMATION_ON\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_GAM_FORMATION_OFF as libc::c_int as UDWORD,
                     pIDStr:
                         b"GAM_FORMATION_OFF\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_SEQ_PLAYBACK as libc::c_int as UDWORD,
                     pIDStr:
                         b"SEQ_PLAYBACK\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_SEQ_FULL as libc::c_int as UDWORD,
                     pIDStr:
                         b"SEQ_FULL\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_SEQ_WINDOW as libc::c_int as UDWORD,
                     pIDStr:
                         b"SEQ_WINDOW\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,};
         init
     },
     {
         let mut init =
             _str_id{id: STR_SEQ_MINIMAL as libc::c_int as UDWORD,
                     pIDStr:
                         b"SEQ_MINIMAL\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,};
         init
     }];
/* The string resource object */
#[no_mangle]
pub static mut psStringRes: *mut STR_RES =
    0 as *const STR_RES as *mut STR_RES;
/* The string resource object */
/* Initialise the string system */
/* Initialise the string system */
#[no_mangle]
pub unsafe extern "C" fn stringsInitialise() -> BOOL {
    if strresCreate(&mut psStringRes, 20 as libc::c_int as UDWORD,
                    5 as libc::c_int as UDWORD) == 0 {
        return 0 as libc::c_int
    }
    if strresLoadFixedID(psStringRes, asFixedID.as_mut_ptr(),
                         STR_MAX_ID as libc::c_int as UDWORD) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
/* Shut down the string system */
/* Shut down the string system */
#[no_mangle]
pub unsafe extern "C" fn stringsShutDown() { strresDestroy(psStringRes); }
