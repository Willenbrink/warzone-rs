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
    #[no_mangle]
    fn resGetData(pType: *mut STRING, pID: *mut STRING) -> *mut libc::c_void;
    #[no_mangle]
    fn pie_LocalRenderBegin();
    #[no_mangle]
    fn pie_LocalRenderEnd();
    #[no_mangle]
    fn pie_BoxFillIndex(x0: libc::c_int, y0: libc::c_int, x1: libc::c_int,
                        y1: libc::c_int, colour: UBYTE);
    #[no_mangle]
    fn pie_ImageFileID(ImageFile: *mut IMAGEFILE, ID: UWORD, x: libc::c_int,
                       y: libc::c_int);
    #[no_mangle]
    fn pie_ImageFileIDTile(ImageFile: *mut IMAGEFILE, ID: UWORD,
                           x: libc::c_int, y: libc::c_int, x0: libc::c_int,
                           y0: libc::c_int, Width: libc::c_int,
                           Height: libc::c_int);
    #[no_mangle]
    fn pie_TransBoxFill(x0: SDWORD, y0: SDWORD, x1: SDWORD, y1: SDWORD);
    #[no_mangle]
    fn iV_GetImageWidth(ImageFile: *mut IMAGEFILE, ID: UWORD) -> UWORD;
    #[no_mangle]
    fn iV_GetImageHeight(ImageFile: *mut IMAGEFILE, ID: UWORD) -> UWORD;
}
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
pub type uint8 = libc::c_uchar;
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
pub type C2RustUnnamed = libc::c_uint;
// Fill rect drawn relative to bottom of frame.
// Fill rect drawn relative to top of frame.
pub const FR_BOTTOM: C2RustUnnamed = 5;
// Fill rect drawn relative to right of frame.
pub const FR_TOP: C2RustUnnamed = 4;
// Fill rect drawn relative to left of frame.
pub const FR_RIGHT: C2RustUnnamed = 3;
// Fill rect drawn relative to frame.
pub const FR_LEFT: C2RustUnnamed = 2;
// Fill rect is ignored.
pub const FR_FRAME: C2RustUnnamed = 1;
pub const FR_IGNORE: C2RustUnnamed = 0;
pub type C2RustUnnamed_0 = libc::c_uint;
// Bitmap drawn with colour 0 transparent.
// Bitmap drawn solid.
pub const FR_KEYED: C2RustUnnamed_0 = 1;
pub const FR_SOLID: C2RustUnnamed_0 = 0;
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
pub const IMAGE_FRAME_VL: C2RustUnnamed_1 = 50;
pub const IMAGE_FRAME_HB: C2RustUnnamed_1 = 48;
pub const IMAGE_FRAME_VR: C2RustUnnamed_1 = 51;
pub const IMAGE_FRAME_HT: C2RustUnnamed_1 = 47;
pub const IMAGE_FRAME_C2: C2RustUnnamed_1 = 22;
pub const IMAGE_FRAME_C3: C2RustUnnamed_1 = 23;
pub const IMAGE_FRAME_C1: C2RustUnnamed_1 = 21;
pub const IMAGE_FRAME_C0: C2RustUnnamed_1 = 20;
pub const IMAGE_TABSELECTED: C2RustUnnamed_1 = 18;
pub const IMAGE_TABHILIGHT: C2RustUnnamed_1 = 19;
pub const IMAGE_TAB1DOWN: C2RustUnnamed_1 = 14;
pub const IMAGE_TAB1: C2RustUnnamed_1 = 10;
pub const IMAGE_SIDETABSEL: C2RustUnnamed_1 = 246;
pub const IMAGE_SIDETABHI: C2RustUnnamed_1 = 244;
pub const IMAGE_SIDETABDOWN: C2RustUnnamed_1 = 245;
pub const IMAGE_SIDETAB: C2RustUnnamed_1 = 243;
pub const IMAGE_DES_WEAPONSDOWN: C2RustUnnamed_1 = 64;
pub const IMAGE_DES_EXTRAHI: C2RustUnnamed_1 = 89;
pub const IMAGE_DES_WEAPONS: C2RustUnnamed_1 = 63;
pub const IMAGE_TAB1SELECTED_SM: C2RustUnnamed_1 = 469;
pub const IMAGE_TABHILIGHT_SM: C2RustUnnamed_1 = 471;
pub const IMAGE_TAB1DOWN_SM: C2RustUnnamed_1 = 470;
pub const IMAGE_TAB1_SM: C2RustUnnamed_1 = 468;
/* **********************************************/
/* Image ID definition file created by Framer. */
/* **********************************************/
pub type C2RustUnnamed_1 = libc::c_uint;
pub const IMAGE_ASCII64: C2RustUnnamed_1 = 483;
pub const IMAGE_ORD_FIREDES_DOWN: C2RustUnnamed_1 = 482;
pub const IMAGE_ORD_FIREDES_UP: C2RustUnnamed_1 = 481;
pub const IMAGE_TRANSETA_DOWN: C2RustUnnamed_1 = 480;
pub const IMAGE_LAUNCHUP: C2RustUnnamed_1 = 479;
pub const IMAGE_BLUE6: C2RustUnnamed_1 = 478;
pub const IMAGE_BLUE5: C2RustUnnamed_1 = 477;
pub const IMAGE_BLUE4: C2RustUnnamed_1 = 476;
pub const IMAGE_BLUE3: C2RustUnnamed_1 = 475;
pub const IMAGE_BLUE2: C2RustUnnamed_1 = 474;
pub const IMAGE_BLUE1: C2RustUnnamed_1 = 473;
pub const IMAGE_DES_ARMOUR_KINETIC: C2RustUnnamed_1 = 472;
pub const IMAGE_TARGET5: C2RustUnnamed_1 = 467;
pub const IMAGE_TARGET4: C2RustUnnamed_1 = 466;
pub const IMAGE_TARGET3: C2RustUnnamed_1 = 465;
pub const IMAGE_TARGET2: C2RustUnnamed_1 = 464;
pub const IMAGE_TARGET1: C2RustUnnamed_1 = 463;
pub const IMAGE_MISSION_CLOCK_UP: C2RustUnnamed_1 = 462;
pub const IMAGE_RES_GRPDAM: C2RustUnnamed_1 = 461;
pub const IMAGE_RES_GRPACC: C2RustUnnamed_1 = 460;
pub const IMAGE_RES_GRPROF: C2RustUnnamed_1 = 459;
pub const IMAGE_RES_GRPREP: C2RustUnnamed_1 = 458;
pub const IMAGE_RES_GRPUPG: C2RustUnnamed_1 = 457;
pub const IMAGE_ASCII131: C2RustUnnamed_1 = 456;
pub const IMAGE_ASCII161: C2RustUnnamed_1 = 455;
pub const IMAGE_ASCII191: C2RustUnnamed_1 = 454;
pub const IMAGE_ASCII208: C2RustUnnamed_1 = 453;
pub const IMAGE_ASCII207: C2RustUnnamed_1 = 452;
pub const IMAGE_ASCII206: C2RustUnnamed_1 = 451;
pub const IMAGE_ASCII205: C2RustUnnamed_1 = 450;
pub const IMAGE_ASCII204: C2RustUnnamed_1 = 449;
pub const IMAGE_ASCII203: C2RustUnnamed_1 = 448;
pub const IMAGE_ASCII202: C2RustUnnamed_1 = 447;
pub const IMAGE_ASCII201: C2RustUnnamed_1 = 446;
pub const IMAGE_ASCII200: C2RustUnnamed_1 = 445;
pub const IMAGE_ASCII198: C2RustUnnamed_1 = 444;
pub const IMAGE_ASCII197: C2RustUnnamed_1 = 443;
pub const IMAGE_ASCII196: C2RustUnnamed_1 = 442;
pub const IMAGE_ASCII195: C2RustUnnamed_1 = 441;
pub const IMAGE_ASCII194: C2RustUnnamed_1 = 440;
pub const IMAGE_ASCII193: C2RustUnnamed_1 = 439;
pub const IMAGE_ASCII192: C2RustUnnamed_1 = 438;
pub const IMAGE_ASCII210: C2RustUnnamed_1 = 437;
pub const IMAGE_ASCII211: C2RustUnnamed_1 = 436;
pub const IMAGE_ASCII214: C2RustUnnamed_1 = 435;
pub const IMAGE_ASCII213: C2RustUnnamed_1 = 434;
pub const IMAGE_ASCII212: C2RustUnnamed_1 = 433;
pub const IMAGE_ASCII216: C2RustUnnamed_1 = 432;
pub const IMAGE_ASCII220: C2RustUnnamed_1 = 431;
pub const IMAGE_ASCII217: C2RustUnnamed_1 = 430;
pub const IMAGE_ASCII219: C2RustUnnamed_1 = 429;
pub const IMAGE_ASCII218: C2RustUnnamed_1 = 428;
pub const IMAGE_ASCII221: C2RustUnnamed_1 = 427;
pub const IMAGE_ASCII223: C2RustUnnamed_1 = 426;
pub const IMAGE_ASCII248: C2RustUnnamed_1 = 425;
pub const IMAGE_ASCII241: C2RustUnnamed_1 = 424;
pub const IMAGE_ASCII253: C2RustUnnamed_1 = 423;
pub const IMAGE_ASCII252: C2RustUnnamed_1 = 422;
pub const IMAGE_ASCII251: C2RustUnnamed_1 = 421;
pub const IMAGE_ASCII250: C2RustUnnamed_1 = 420;
pub const IMAGE_ASCII249: C2RustUnnamed_1 = 419;
pub const IMAGE_ASCII246: C2RustUnnamed_1 = 418;
pub const IMAGE_ASCII245: C2RustUnnamed_1 = 417;
pub const IMAGE_ASCII244: C2RustUnnamed_1 = 416;
pub const IMAGE_ASCII243: C2RustUnnamed_1 = 415;
pub const IMAGE_ASCII242: C2RustUnnamed_1 = 414;
pub const IMAGE_ASCII239: C2RustUnnamed_1 = 413;
pub const IMAGE_ASCII238: C2RustUnnamed_1 = 412;
pub const IMAGE_ASCII237: C2RustUnnamed_1 = 411;
pub const IMAGE_ASCII236: C2RustUnnamed_1 = 410;
pub const IMAGE_ASCII235: C2RustUnnamed_1 = 409;
pub const IMAGE_ASCII234: C2RustUnnamed_1 = 408;
pub const IMAGE_ASCII233: C2RustUnnamed_1 = 407;
pub const IMAGE_ASCII232: C2RustUnnamed_1 = 406;
pub const IMAGE_ASCII231: C2RustUnnamed_1 = 405;
pub const IMAGE_ASCII230: C2RustUnnamed_1 = 404;
pub const IMAGE_ASCII229: C2RustUnnamed_1 = 403;
pub const IMAGE_ASCII228: C2RustUnnamed_1 = 402;
pub const IMAGE_ASCII227: C2RustUnnamed_1 = 401;
pub const IMAGE_ASCII226: C2RustUnnamed_1 = 400;
pub const IMAGE_ASCII225: C2RustUnnamed_1 = 399;
pub const IMAGE_ASCII224: C2RustUnnamed_1 = 398;
pub const IMAGE_ASCII189: C2RustUnnamed_1 = 397;
pub const IMAGE_ASCII188: C2RustUnnamed_1 = 396;
pub const IMAGE_RES_DEFENCE: C2RustUnnamed_1 = 395;
pub const IMAGE_LEV_7: C2RustUnnamed_1 = 394;
pub const IMAGE_LEV_6: C2RustUnnamed_1 = 393;
pub const IMAGE_LEV_5: C2RustUnnamed_1 = 392;
pub const IMAGE_LEV_4: C2RustUnnamed_1 = 391;
pub const IMAGE_LEV_3: C2RustUnnamed_1 = 390;
pub const IMAGE_LEV_2: C2RustUnnamed_1 = 389;
pub const IMAGE_LEV_1: C2RustUnnamed_1 = 388;
pub const IMAGE_LEV_0: C2RustUnnamed_1 = 387;
pub const IMAGE_CDCHANGE_CANCEL: C2RustUnnamed_1 = 386;
pub const IMAGE_CDCHANGE_OK: C2RustUnnamed_1 = 385;
pub const IMAGE_MULTI_CHAN: C2RustUnnamed_1 = 384;
pub const IMAGE_MULTI_NOCHAN: C2RustUnnamed_1 = 383;
pub const IMAGE_ORD_EMBARKDOWN: C2RustUnnamed_1 = 382;
pub const IMAGE_ORD_EMBARKUP: C2RustUnnamed_1 = 381;
pub const IMAGE_ORD_RTRDOWN: C2RustUnnamed_1 = 380;
pub const IMAGE_ORD_RTRUP: C2RustUnnamed_1 = 379;
pub const IMAGE_ORD_GUARDDOWN: C2RustUnnamed_1 = 378;
pub const IMAGE_ORD_GUARDUP: C2RustUnnamed_1 = 377;
pub const IMAGE_ORD_PERSUEDOWN: C2RustUnnamed_1 = 376;
pub const IMAGE_ORD_PERSUEUP: C2RustUnnamed_1 = 375;
pub const IMAGE_ORD_PATROLDOWN: C2RustUnnamed_1 = 374;
pub const IMAGE_ORD_PATROLUP: C2RustUnnamed_1 = 373;
pub const IMAGE_GN_0: C2RustUnnamed_1 = 372;
pub const IMAGE_GN_1: C2RustUnnamed_1 = 371;
pub const IMAGE_GN_2: C2RustUnnamed_1 = 370;
pub const IMAGE_GN_3: C2RustUnnamed_1 = 369;
pub const IMAGE_GN_4: C2RustUnnamed_1 = 368;
pub const IMAGE_GN_5: C2RustUnnamed_1 = 367;
pub const IMAGE_GN_6: C2RustUnnamed_1 = 366;
pub const IMAGE_GN_7: C2RustUnnamed_1 = 365;
pub const IMAGE_GN_8: C2RustUnnamed_1 = 364;
pub const IMAGE_GN_9: C2RustUnnamed_1 = 363;
pub const IMAGE_GN_STAR: C2RustUnnamed_1 = 362;
pub const IMAGE_VDP_HI: C2RustUnnamed_1 = 361;
pub const IMAGE_VDP_UP: C2RustUnnamed_1 = 360;
pub const IMAGE_VDP_DOWN: C2RustUnnamed_1 = 359;
pub const IMAGE_FDP_HI: C2RustUnnamed_1 = 358;
pub const IMAGE_FDP_UP: C2RustUnnamed_1 = 357;
pub const IMAGE_FDP_DOWN: C2RustUnnamed_1 = 356;
pub const IMAGE_CDP_HI: C2RustUnnamed_1 = 355;
pub const IMAGE_CDP_UP: C2RustUnnamed_1 = 354;
pub const IMAGE_CDP_DOWN: C2RustUnnamed_1 = 353;
pub const IMAGE_LOOP_HI: C2RustUnnamed_1 = 352;
pub const IMAGE_LOOP_DOWN: C2RustUnnamed_1 = 351;
pub const IMAGE_LOOP_UP: C2RustUnnamed_1 = 350;
pub const IMAGE_RES_CYBORGTECH: C2RustUnnamed_1 = 349;
pub const IMAGE_CURSOR_DEST: C2RustUnnamed_1 = 348;
pub const IMAGE_DES_EDITBOXRIGHTH: C2RustUnnamed_1 = 347;
pub const IMAGE_DES_EDITBOXMIDH: C2RustUnnamed_1 = 346;
pub const IMAGE_DES_EDITBOXRIGHT: C2RustUnnamed_1 = 345;
pub const IMAGE_DES_EDITBOXMID: C2RustUnnamed_1 = 344;
pub const IMAGE_DES_POWERBAR_RIGHT: C2RustUnnamed_1 = 343;
pub const IMAGE_DES_POWERBAR_LEFT: C2RustUnnamed_1 = 342;
pub const IMAGE_AUDIO_LASTSAMPLEH: C2RustUnnamed_1 = 341;
pub const IMAGE_AUDIO_LASTSAMPLE: C2RustUnnamed_1 = 340;
pub const IMAGE_DES_BINH: C2RustUnnamed_1 = 339;
pub const IMAGE_DES_BIN: C2RustUnnamed_1 = 338;
pub const IMAGE_INFINITE_HI: C2RustUnnamed_1 = 337;
pub const IMAGE_INFINITE_DOWN: C2RustUnnamed_1 = 336;
pub const IMAGE_INFINITE_UP: C2RustUnnamed_1 = 335;
pub const IMAGE_RETICULE_GREY: C2RustUnnamed_1 = 334;
pub const IMAGE_ORD_FACHILITE: C2RustUnnamed_1 = 333;
pub const IMAGE_ORD_FAC5DOWN: C2RustUnnamed_1 = 332;
pub const IMAGE_ORD_FAC5UP: C2RustUnnamed_1 = 331;
pub const IMAGE_ORD_FAC4DOWN: C2RustUnnamed_1 = 330;
pub const IMAGE_ORD_FAC4UP: C2RustUnnamed_1 = 329;
pub const IMAGE_ORD_FAC3DOWN: C2RustUnnamed_1 = 328;
pub const IMAGE_ORD_FAC3UP: C2RustUnnamed_1 = 327;
pub const IMAGE_ORD_FAC2DOWN: C2RustUnnamed_1 = 326;
pub const IMAGE_ORD_FAC2UP: C2RustUnnamed_1 = 325;
pub const IMAGE_ORD_FAC1DOWN: C2RustUnnamed_1 = 324;
pub const IMAGE_ORD_FAC1UP: C2RustUnnamed_1 = 323;
pub const IMAGE_TRANSETA_UP: C2RustUnnamed_1 = 322;
pub const IMAGE_LAUNCHDOWN: C2RustUnnamed_1 = 321;
pub const IMAGE_MISSION_CLOCK: C2RustUnnamed_1 = 320;
pub const IMAGE_NRUTER: C2RustUnnamed_1 = 319;
pub const IMAGE_CMDDROID_EXP: C2RustUnnamed_1 = 318;
pub const IMAGE_MULTI_VIS_HI: C2RustUnnamed_1 = 317;
pub const IMAGE_MULTI_POW_HI: C2RustUnnamed_1 = 316;
pub const IMAGE_MULTI_DRO_HI: C2RustUnnamed_1 = 315;
pub const IMAGE_MULTI_TEK_HI: C2RustUnnamed_1 = 314;
pub const IMAGE_MULTI_TEK: C2RustUnnamed_1 = 313;
pub const IMAGE_MULTI_DRO: C2RustUnnamed_1 = 312;
pub const IMAGE_MULTI_POW: C2RustUnnamed_1 = 311;
pub const IMAGE_MULTI_VIS: C2RustUnnamed_1 = 310;
pub const IMAGE_MULTI_AL_HI: C2RustUnnamed_1 = 309;
pub const IMAGE_MULTI_OFFAL_HI: C2RustUnnamed_1 = 308;
pub const IMAGE_MULTI_NOAL_HI: C2RustUnnamed_1 = 307;
pub const IMAGE_MULTI_AL: C2RustUnnamed_1 = 306;
pub const IMAGE_MULTI_OFFAL: C2RustUnnamed_1 = 305;
pub const IMAGE_MULTI_NOAL: C2RustUnnamed_1 = 304;
pub const IMAGE_RAD_ART3: C2RustUnnamed_1 = 303;
pub const IMAGE_RAD_ART2: C2RustUnnamed_1 = 302;
pub const IMAGE_RAD_ART1: C2RustUnnamed_1 = 301;
pub const IMAGE_RAD_ARTREAD: C2RustUnnamed_1 = 300;
pub const IMAGE_RAD_RES3: C2RustUnnamed_1 = 299;
pub const IMAGE_RAD_RES2: C2RustUnnamed_1 = 298;
pub const IMAGE_RAD_RES1: C2RustUnnamed_1 = 297;
pub const IMAGE_RAD_RESREAD: C2RustUnnamed_1 = 296;
pub const IMAGE_RAD_ENM3: C2RustUnnamed_1 = 295;
pub const IMAGE_RAD_ENM2: C2RustUnnamed_1 = 294;
pub const IMAGE_RAD_ENM1: C2RustUnnamed_1 = 293;
pub const IMAGE_RAD_ENMREAD: C2RustUnnamed_1 = 292;
pub const IMAGE_SLIDER_INFINITY: C2RustUnnamed_1 = 291;
pub const IMAGE_SLIDER_BIGBUT: C2RustUnnamed_1 = 290;
pub const IMAGE_ORD_DESTRUCT2GREY: C2RustUnnamed_1 = 289;
pub const IMAGE_ORD_DESTRUCT2DOWN: C2RustUnnamed_1 = 288;
pub const IMAGE_ORD_DESTRUCT2UP: C2RustUnnamed_1 = 287;
pub const IMAGE_ORD_GOTOHQDOWN: C2RustUnnamed_1 = 286;
pub const IMAGE_ORD_GOTOHQUP: C2RustUnnamed_1 = 285;
pub const IMAGE_ORD_HALTDOWN: C2RustUnnamed_1 = 284;
pub const IMAGE_ORD_HALTUP: C2RustUnnamed_1 = 283;
pub const IMAGE_ORD_HOLDFIREDOWN: C2RustUnnamed_1 = 282;
pub const IMAGE_ORD_HOLDFIREUP: C2RustUnnamed_1 = 281;
pub const IMAGE_ORD_RETFIREDOWN: C2RustUnnamed_1 = 280;
pub const IMAGE_ORD_RETFIREUP: C2RustUnnamed_1 = 279;
pub const IMAGE_ORD_FATWILLDOWN: C2RustUnnamed_1 = 278;
pub const IMAGE_ORD_FATWILLUP: C2RustUnnamed_1 = 277;
pub const IMAGE_ORD_REPAIR3DOWN: C2RustUnnamed_1 = 276;
pub const IMAGE_ORD_REPAIR3UP: C2RustUnnamed_1 = 275;
pub const IMAGE_ORD_REPAIR1DOWN: C2RustUnnamed_1 = 274;
pub const IMAGE_ORD_REPAIR1UP: C2RustUnnamed_1 = 273;
pub const IMAGE_ORD_REPAIR2DOWN: C2RustUnnamed_1 = 272;
pub const IMAGE_ORD_REPAIR2UP: C2RustUnnamed_1 = 271;
pub const IMAGE_ORD_RANGE3DOWN: C2RustUnnamed_1 = 270;
pub const IMAGE_ORD_RANGE3UP: C2RustUnnamed_1 = 269;
pub const IMAGE_ORD_RANGE2DOWN: C2RustUnnamed_1 = 268;
pub const IMAGE_ORD_RANGE2UP: C2RustUnnamed_1 = 267;
pub const IMAGE_ORD_RANGE1DOWN: C2RustUnnamed_1 = 266;
pub const IMAGE_ORD_RANGE1UP: C2RustUnnamed_1 = 265;
pub const IMAGE_ORD_DESTRUCT1DOWN: C2RustUnnamed_1 = 264;
pub const IMAGE_ORD_DESTRUCT1UP: C2RustUnnamed_1 = 263;
pub const IMAGE_RES_QUESTIONMARK: C2RustUnnamed_1 = 262;
pub const IMAGE_RES_STRUCTURETECH: C2RustUnnamed_1 = 261;
pub const IMAGE_RES_SYSTEMTECH: C2RustUnnamed_1 = 260;
pub const IMAGE_RES_POWERTECH: C2RustUnnamed_1 = 259;
pub const IMAGE_RES_COMPUTERTECH: C2RustUnnamed_1 = 258;
pub const IMAGE_RES_WEAPONTECH: C2RustUnnamed_1 = 257;
pub const IMAGE_RES_DROIDTECH: C2RustUnnamed_1 = 256;
pub const IMAGE_QUESTION_MARK: C2RustUnnamed_1 = 255;
pub const IMAGE_RES_MAJOR_ROCKET: C2RustUnnamed_1 = 254;
pub const IMAGE_RES_MINOR_AUTOWEAPONS: C2RustUnnamed_1 = 253;
pub const IMAGE_TRACKS: C2RustUnnamed_1 = 252;
pub const IMAGE_STAR: C2RustUnnamed_1 = 251;
pub const IMAGE_SLIDER_BIG: C2RustUnnamed_1 = 250;
pub const IMAGE_DES_STATBACKMID: C2RustUnnamed_1 = 249;
pub const IMAGE_DES_STATBACKRIGHT: C2RustUnnamed_1 = 248;
pub const IMAGE_DES_STATBACKLEFT: C2RustUnnamed_1 = 247;
pub const IMAGE_CURSOR_NOTPOS: C2RustUnnamed_1 = 242;
pub const IMAGE_CURSOR_PICKUP: C2RustUnnamed_1 = 241;
pub const IMAGE_CURSOR_REPAIR: C2RustUnnamed_1 = 240;
pub const IMAGE_CURSOR_FIX: C2RustUnnamed_1 = 239;
pub const IMAGE_CURSOR_ATTACH: C2RustUnnamed_1 = 238;
pub const IMAGE_CURSOR_BRIDGE: C2RustUnnamed_1 = 237;
pub const IMAGE_CURSOR_EMBARK: C2RustUnnamed_1 = 236;
pub const IMAGE_CURSOR_GUARD: C2RustUnnamed_1 = 235;
pub const IMAGE_CURSOR_MOVE: C2RustUnnamed_1 = 234;
pub const IMAGE_CURSOR_BUILD: C2RustUnnamed_1 = 233;
pub const IMAGE_CURSOR_DEFAULT: C2RustUnnamed_1 = 232;
pub const IMAGE_CURSOR_ECM: C2RustUnnamed_1 = 231;
pub const IMAGE_CURSOR_LOCKON: C2RustUnnamed_1 = 230;
pub const IMAGE_CURSOR_SELECT: C2RustUnnamed_1 = 229;
pub const IMAGE_CURSOR_ATTACK: C2RustUnnamed_1 = 228;
pub const IMAGE_ASCII126: C2RustUnnamed_1 = 227;
pub const IMAGE_ASCII125: C2RustUnnamed_1 = 226;
pub const IMAGE_ASCII124: C2RustUnnamed_1 = 225;
pub const IMAGE_ASCII123: C2RustUnnamed_1 = 224;
pub const IMAGE_ASCII122: C2RustUnnamed_1 = 223;
pub const IMAGE_ASCII121: C2RustUnnamed_1 = 222;
pub const IMAGE_ASCII120: C2RustUnnamed_1 = 221;
pub const IMAGE_ASCII119: C2RustUnnamed_1 = 220;
pub const IMAGE_ASCII118: C2RustUnnamed_1 = 219;
pub const IMAGE_ASCII117: C2RustUnnamed_1 = 218;
pub const IMAGE_ASCII116: C2RustUnnamed_1 = 217;
pub const IMAGE_ASCII115: C2RustUnnamed_1 = 216;
pub const IMAGE_ASCII114: C2RustUnnamed_1 = 215;
pub const IMAGE_ASCII113: C2RustUnnamed_1 = 214;
pub const IMAGE_ASCII112: C2RustUnnamed_1 = 213;
pub const IMAGE_ASCII111: C2RustUnnamed_1 = 212;
pub const IMAGE_ASCII110: C2RustUnnamed_1 = 211;
pub const IMAGE_ASCII109: C2RustUnnamed_1 = 210;
pub const IMAGE_ASCII108: C2RustUnnamed_1 = 209;
pub const IMAGE_ASCII107: C2RustUnnamed_1 = 208;
pub const IMAGE_ASCII106: C2RustUnnamed_1 = 207;
pub const IMAGE_ASCII105: C2RustUnnamed_1 = 206;
pub const IMAGE_ASCII104: C2RustUnnamed_1 = 205;
pub const IMAGE_ASCII103: C2RustUnnamed_1 = 204;
pub const IMAGE_ASCII102: C2RustUnnamed_1 = 203;
pub const IMAGE_ASCII101: C2RustUnnamed_1 = 202;
pub const IMAGE_ASCII100: C2RustUnnamed_1 = 201;
pub const IMAGE_ASCII99: C2RustUnnamed_1 = 200;
pub const IMAGE_ASCII98: C2RustUnnamed_1 = 199;
pub const IMAGE_ASCII97: C2RustUnnamed_1 = 198;
pub const IMAGE_ASCII96: C2RustUnnamed_1 = 197;
pub const IMAGE_ASCII95: C2RustUnnamed_1 = 196;
pub const IMAGE_ASCII94: C2RustUnnamed_1 = 195;
pub const IMAGE_ASCII93: C2RustUnnamed_1 = 194;
pub const IMAGE_ASCII92: C2RustUnnamed_1 = 193;
pub const IMAGE_ASCII91: C2RustUnnamed_1 = 192;
pub const IMAGE_ASCII90: C2RustUnnamed_1 = 191;
pub const IMAGE_ASCII89: C2RustUnnamed_1 = 190;
pub const IMAGE_ASCII88: C2RustUnnamed_1 = 189;
pub const IMAGE_ASCII87: C2RustUnnamed_1 = 188;
pub const IMAGE_ASCII86: C2RustUnnamed_1 = 187;
pub const IMAGE_ASCII85: C2RustUnnamed_1 = 186;
pub const IMAGE_ASCII84: C2RustUnnamed_1 = 185;
pub const IMAGE_ASCII83: C2RustUnnamed_1 = 184;
pub const IMAGE_ASCII82: C2RustUnnamed_1 = 183;
pub const IMAGE_ASCII81: C2RustUnnamed_1 = 182;
pub const IMAGE_ASCII80: C2RustUnnamed_1 = 181;
pub const IMAGE_ASCII79: C2RustUnnamed_1 = 180;
pub const IMAGE_ASCII78: C2RustUnnamed_1 = 179;
pub const IMAGE_ASCII77: C2RustUnnamed_1 = 178;
pub const IMAGE_ASCII76: C2RustUnnamed_1 = 177;
pub const IMAGE_ASCII75: C2RustUnnamed_1 = 176;
pub const IMAGE_ASCII74: C2RustUnnamed_1 = 175;
pub const IMAGE_ASCII73: C2RustUnnamed_1 = 174;
pub const IMAGE_ASCII72: C2RustUnnamed_1 = 173;
pub const IMAGE_ASCII71: C2RustUnnamed_1 = 172;
pub const IMAGE_ASCII70: C2RustUnnamed_1 = 171;
pub const IMAGE_ASCII69: C2RustUnnamed_1 = 170;
pub const IMAGE_ASCII68: C2RustUnnamed_1 = 169;
pub const IMAGE_ASCII67: C2RustUnnamed_1 = 168;
pub const IMAGE_ASCII66: C2RustUnnamed_1 = 167;
pub const IMAGE_ASCII65: C2RustUnnamed_1 = 166;
pub const IMAGE_ASTERISK: C2RustUnnamed_1 = 165;
pub const IMAGE_ASCII63: C2RustUnnamed_1 = 164;
pub const IMAGE_ASCII62: C2RustUnnamed_1 = 163;
pub const IMAGE_ASCII61: C2RustUnnamed_1 = 162;
pub const IMAGE_ASCII60: C2RustUnnamed_1 = 161;
pub const IMAGE_ASCII59: C2RustUnnamed_1 = 160;
pub const IMAGE_ASCII58: C2RustUnnamed_1 = 159;
pub const IMAGE_ASCII57: C2RustUnnamed_1 = 158;
pub const IMAGE_ASCII56: C2RustUnnamed_1 = 157;
pub const IMAGE_ASCII55: C2RustUnnamed_1 = 156;
pub const IMAGE_ASCII54: C2RustUnnamed_1 = 155;
pub const IMAGE_ASCII53: C2RustUnnamed_1 = 154;
pub const IMAGE_ASCII52: C2RustUnnamed_1 = 153;
pub const IMAGE_ASCII51: C2RustUnnamed_1 = 152;
pub const IMAGE_ASCII50: C2RustUnnamed_1 = 151;
pub const IMAGE_ASCII49: C2RustUnnamed_1 = 150;
pub const IMAGE_ASCII48: C2RustUnnamed_1 = 149;
pub const IMAGE_ASCII47: C2RustUnnamed_1 = 148;
pub const IMAGE_ASCII46: C2RustUnnamed_1 = 147;
pub const IMAGE_ASCII45: C2RustUnnamed_1 = 146;
pub const IMAGE_ASCII44: C2RustUnnamed_1 = 145;
pub const IMAGE_ASCII43: C2RustUnnamed_1 = 144;
pub const IMAGE_ASCII42: C2RustUnnamed_1 = 143;
pub const IMAGE_ASCII41: C2RustUnnamed_1 = 142;
pub const IMAGE_ASCII40: C2RustUnnamed_1 = 141;
pub const IMAGE_ASCII39: C2RustUnnamed_1 = 140;
pub const IMAGE_ASCII38: C2RustUnnamed_1 = 139;
pub const IMAGE_ASCII37: C2RustUnnamed_1 = 138;
pub const IMAGE_ASCII36: C2RustUnnamed_1 = 137;
pub const IMAGE_ASCII35: C2RustUnnamed_1 = 136;
pub const IMAGE_ASCII34: C2RustUnnamed_1 = 135;
pub const IMAGE_ASCII33: C2RustUnnamed_1 = 134;
pub const IMAGE_INTEL_CAMPAIGNDOWN: C2RustUnnamed_1 = 133;
pub const IMAGE_INTEL_CAMPAIGN: C2RustUnnamed_1 = 132;
pub const IMAGE_INTEL_MISSIONDOWN: C2RustUnnamed_1 = 131;
pub const IMAGE_INTEL_MISSION: C2RustUnnamed_1 = 130;
pub const IMAGE_INTEL_RESEARCHDOWN: C2RustUnnamed_1 = 129;
pub const IMAGE_INTEL_RESEARCH: C2RustUnnamed_1 = 128;
pub const IMAGE_DES_BARYELLOW: C2RustUnnamed_1 = 127;
pub const IMAGE_DES_BARRED: C2RustUnnamed_1 = 126;
pub const IMAGE_DES_BARBLUE: C2RustUnnamed_1 = 125;
pub const IMAGE_DES_BARBACK: C2RustUnnamed_1 = 124;
pub const IMAGE_9: C2RustUnnamed_1 = 123;
pub const IMAGE_8: C2RustUnnamed_1 = 122;
pub const IMAGE_7: C2RustUnnamed_1 = 121;
pub const IMAGE_6: C2RustUnnamed_1 = 120;
pub const IMAGE_5: C2RustUnnamed_1 = 119;
pub const IMAGE_4: C2RustUnnamed_1 = 118;
pub const IMAGE_3: C2RustUnnamed_1 = 117;
pub const IMAGE_2: C2RustUnnamed_1 = 116;
pub const IMAGE_1: C2RustUnnamed_1 = 115;
pub const IMAGE_0: C2RustUnnamed_1 = 114;
pub const IMAGE_RES_MAJOR_HOVER: C2RustUnnamed_1 = 113;
pub const IMAGE_RES_MAJOR_HEAVYWEP: C2RustUnnamed_1 = 112;
pub const IMAGE_RES_MAJOR_ELECTRONIC: C2RustUnnamed_1 = 111;
pub const IMAGE_RES_MAJOR_PLASCRETE: C2RustUnnamed_1 = 110;
pub const IMAGE_RES_MINOR_RADAR: C2RustUnnamed_1 = 109;
pub const IMAGE_PLASCRETE: C2RustUnnamed_1 = 108;
pub const IMAGE_ECM: C2RustUnnamed_1 = 107;
pub const IMAGE_RES_MINOR_PLASSTEEL: C2RustUnnamed_1 = 106;
pub const IMAGE_HOVERCRAFT: C2RustUnnamed_1 = 105;
pub const IMAGE_CANNON: C2RustUnnamed_1 = 104;
pub const IMAGE_ROCKET: C2RustUnnamed_1 = 103;
pub const IMAGE_DES_BODYPOINTS: C2RustUnnamed_1 = 102;
pub const IMAGE_DES_FIRERATE: C2RustUnnamed_1 = 101;
pub const IMAGE_DES_HOVER: C2RustUnnamed_1 = 100;
pub const IMAGE_DES_POWER: C2RustUnnamed_1 = 99;
pub const IMAGE_DES_DAMAGE: C2RustUnnamed_1 = 98;
pub const IMAGE_DES_WEIGHT: C2RustUnnamed_1 = 97;
pub const IMAGE_DES_ROAD: C2RustUnnamed_1 = 96;
pub const IMAGE_DES_CROSSCOUNTRY: C2RustUnnamed_1 = 95;
pub const IMAGE_DES_ARMOUR_EXPLOSIVE: C2RustUnnamed_1 = 94;
pub const IMAGE_DES_RANGE: C2RustUnnamed_1 = 93;
pub const IMAGE_DES_BUILDRATE: C2RustUnnamed_1 = 92;
pub const IMAGE_DES_TABWEAPONDOWN: C2RustUnnamed_1 = 91;
pub const IMAGE_DES_TABWEAPON: C2RustUnnamed_1 = 90;
pub const IMAGE_DES_STATSCOMP: C2RustUnnamed_1 = 88;
pub const IMAGE_DES_BACK: C2RustUnnamed_1 = 87;
pub const IMAGE_FRAME_HC3: C2RustUnnamed_1 = 86;
pub const IMAGE_FRAME_HC2: C2RustUnnamed_1 = 85;
pub const IMAGE_FRAME_HC1: C2RustUnnamed_1 = 84;
pub const IMAGE_FRAME_HC0: C2RustUnnamed_1 = 83;
pub const IMAGE_FRAME_VC3: C2RustUnnamed_1 = 82;
pub const IMAGE_FRAME_VC2: C2RustUnnamed_1 = 81;
pub const IMAGE_FRAME_VC1: C2RustUnnamed_1 = 80;
pub const IMAGE_FRAME_VC0: C2RustUnnamed_1 = 79;
pub const IMAGE_DES_EDITBOXLEFTH: C2RustUnnamed_1 = 78;
pub const IMAGE_DES_EDITBOXLEFT: C2RustUnnamed_1 = 77;
pub const IMAGE_DES_POWERBACK: C2RustUnnamed_1 = 76;
pub const IMAGE_DES_STATSCURR: C2RustUnnamed_1 = 75;
pub const IMAGE_DES_STATSBACK: C2RustUnnamed_1 = 74;
pub const IMAGE_DES_POWERCURR: C2RustUnnamed_1 = 73;
pub const IMAGE_FRAME_HBH: C2RustUnnamed_1 = 72;
pub const IMAGE_FRAME_HTH: C2RustUnnamed_1 = 71;
pub const IMAGE_FRAME_HB2: C2RustUnnamed_1 = 70;
pub const IMAGE_FRAME_HT2: C2RustUnnamed_1 = 69;
pub const IMAGE_DES_COMMANDDOWN: C2RustUnnamed_1 = 68;
pub const IMAGE_DES_COMMAND: C2RustUnnamed_1 = 67;
pub const IMAGE_DES_SYSTEMSDOWN: C2RustUnnamed_1 = 66;
pub const IMAGE_DES_SYSTEMS: C2RustUnnamed_1 = 65;
pub const IMAGE_FRAME_VRH: C2RustUnnamed_1 = 62;
pub const IMAGE_FRAME_VLH: C2RustUnnamed_1 = 61;
pub const IMAGE_FRAME_VR2: C2RustUnnamed_1 = 60;
pub const IMAGE_FRAME_VL2: C2RustUnnamed_1 = 59;
pub const IMAGE_DES_HILIGHT: C2RustUnnamed_1 = 58;
pub const IMAGE_DES_PROPULSIONH: C2RustUnnamed_1 = 57;
pub const IMAGE_DES_PROPULSION: C2RustUnnamed_1 = 56;
pub const IMAGE_DES_BODYH: C2RustUnnamed_1 = 55;
pub const IMAGE_DES_BODY: C2RustUnnamed_1 = 54;
pub const IMAGE_DES_TURRETH: C2RustUnnamed_1 = 53;
pub const IMAGE_DES_TURRET: C2RustUnnamed_1 = 52;
pub const IMAGE_SLIDER_BACK: C2RustUnnamed_1 = 49;
pub const IMAGE_SLIDER_BUT: C2RustUnnamed_1 = 46;
pub const IMAGE_CLOSEHILIGHT: C2RustUnnamed_1 = 45;
pub const IMAGE_CLOSEDOWN: C2RustUnnamed_1 = 44;
pub const IMAGE_CLOSE: C2RustUnnamed_1 = 43;
pub const IMAGE_CANCEL_HILIGHT: C2RustUnnamed_1 = 42;
pub const IMAGE_CANCEL_DOWN: C2RustUnnamed_1 = 41;
pub const IMAGE_CANCEL_UP: C2RustUnnamed_1 = 40;
pub const IMAGE_RETICULE_BUTDOWN: C2RustUnnamed_1 = 39;
pub const IMAGE_RETICULE_HILIGHT: C2RustUnnamed_1 = 38;
pub const IMAGE_MANUFACTURE_DOWN: C2RustUnnamed_1 = 37;
pub const IMAGE_MANUFACTURE_UP: C2RustUnnamed_1 = 36;
pub const IMAGE_RESEARCH_DOWN: C2RustUnnamed_1 = 35;
pub const IMAGE_RESEARCH_UP: C2RustUnnamed_1 = 34;
pub const IMAGE_BUILD_DOWN: C2RustUnnamed_1 = 33;
pub const IMAGE_BUILD_UP: C2RustUnnamed_1 = 32;
pub const IMAGE_DESIGN_DOWN: C2RustUnnamed_1 = 31;
pub const IMAGE_DESIGN_UP: C2RustUnnamed_1 = 30;
pub const IMAGE_COMMANDDROID_DOWN: C2RustUnnamed_1 = 29;
pub const IMAGE_COMMANDDROID_UP: C2RustUnnamed_1 = 28;
pub const IMAGE_INTELMAP_DOWN: C2RustUnnamed_1 = 27;
pub const IMAGE_INTELMAP_UP: C2RustUnnamed_1 = 26;
pub const IMAGE_PBAR_BOTTOM: C2RustUnnamed_1 = 25;
pub const IMAGE_PBAR_TOP: C2RustUnnamed_1 = 24;
pub const IMAGE_TAB4DOWN: C2RustUnnamed_1 = 17;
pub const IMAGE_TAB3DOWN: C2RustUnnamed_1 = 16;
pub const IMAGE_TAB2DOWN: C2RustUnnamed_1 = 15;
pub const IMAGE_TAB4: C2RustUnnamed_1 = 13;
pub const IMAGE_TAB3: C2RustUnnamed_1 = 12;
pub const IMAGE_TAB2: C2RustUnnamed_1 = 11;
pub const IMAGE_BUTB_HILITE: C2RustUnnamed_1 = 9;
pub const IMAGE_BUT_HILITE: C2RustUnnamed_1 = 8;
pub const IMAGE_BUTB0_DOWN: C2RustUnnamed_1 = 7;
pub const IMAGE_BUTB0_UP: C2RustUnnamed_1 = 6;
pub const IMAGE_BUT0_DOWN: C2RustUnnamed_1 = 5;
pub const IMAGE_BUT0_UP: C2RustUnnamed_1 = 4;
pub const IMAGE_PBAR_REQUIRED: C2RustUnnamed_1 = 3;
pub const IMAGE_PBAR_USED: C2RustUnnamed_1 = 2;
pub const IMAGE_PBAR_AVAIL: C2RustUnnamed_1 = 1;
pub const IMAGE_PBAR_EMPTY: C2RustUnnamed_1 = 0;
/*
 * Image.c
 *
 * Image definitions and related functions.
 *
 */
/* Includes direct access to render library */
// FIXME Direct iVis implementation include!
static mut EnableLocks: BOOL = 1 as libc::c_int;
static mut LockRefs: SDWORD = 0 as libc::c_int;
#[no_mangle]
pub static mut IntImages: *mut IMAGEFILE =
    0 as *const IMAGEFILE as *mut IMAGEFILE;
// All the 2d graphics for the user interface.
// Form frame definitions.
#[no_mangle]
pub static mut FrameNormal: IMAGEFRAME =
    {
        let mut init =
            IMAGEFRAME{OffsetX0: 0 as libc::c_int as SWORD,
                       OffsetY0: 0 as libc::c_int as SWORD,
                       OffsetX1: 0 as libc::c_int as SWORD,
                       OffsetY1: 0 as libc::c_int as SWORD,
                       TopLeft: IMAGE_FRAME_C0 as libc::c_int as SWORD,
                       TopRight: IMAGE_FRAME_C1 as libc::c_int as SWORD,
                       BottomLeft: IMAGE_FRAME_C3 as libc::c_int as SWORD,
                       BottomRight: IMAGE_FRAME_C2 as libc::c_int as SWORD,
                       TopEdge: IMAGE_FRAME_HT as libc::c_int as SWORD,
                       TopType: FR_SOLID as libc::c_int as SWORD,
                       RightEdge: IMAGE_FRAME_VR as libc::c_int as SWORD,
                       RightType: FR_SOLID as libc::c_int as SWORD,
                       BottomEdge: IMAGE_FRAME_HB as libc::c_int as SWORD,
                       BottomType: FR_SOLID as libc::c_int as SWORD,
                       LeftEdge: IMAGE_FRAME_VL as libc::c_int as SWORD,
                       LeftType: FR_SOLID as libc::c_int as SWORD,
                       FRect:
                           [{
                                let mut init =
                                    FRAMERECT{Type:
                                                  FR_FRAME as libc::c_int as
                                                      UWORD,
                                              TLXOffset:
                                                  0 as libc::c_int as SWORD,
                                              TLYOffset:
                                                  1 as libc::c_int as SWORD,
                                              BRXOffset:
                                                  0 as libc::c_int as SWORD,
                                              BRYOffset:
                                                  -(1 as libc::c_int) as
                                                      SWORD,
                                              ColourIndex:
                                                  190 as libc::c_int as
                                                      UBYTE,};
                                init
                            },
                            {
                                let mut init =
                                    FRAMERECT{Type:
                                                  FR_IGNORE as libc::c_int as
                                                      UWORD,
                                              TLXOffset:
                                                  0 as libc::c_int as SWORD,
                                              TLYOffset:
                                                  0 as libc::c_int as SWORD,
                                              BRXOffset:
                                                  0 as libc::c_int as SWORD,
                                              BRYOffset:
                                                  0 as libc::c_int as SWORD,
                                              ColourIndex:
                                                  0 as libc::c_int as UBYTE,};
                                init
                            },
                            {
                                let mut init =
                                    FRAMERECT{Type:
                                                  FR_IGNORE as libc::c_int as
                                                      UWORD,
                                              TLXOffset:
                                                  0 as libc::c_int as SWORD,
                                              TLYOffset:
                                                  0 as libc::c_int as SWORD,
                                              BRXOffset:
                                                  0 as libc::c_int as SWORD,
                                              BRYOffset:
                                                  0 as libc::c_int as SWORD,
                                              ColourIndex:
                                                  0 as libc::c_int as UBYTE,};
                                init
                            },
                            {
                                let mut init =
                                    FRAMERECT{Type:
                                                  FR_IGNORE as libc::c_int as
                                                      UWORD,
                                              TLXOffset:
                                                  0 as libc::c_int as SWORD,
                                              TLYOffset:
                                                  0 as libc::c_int as SWORD,
                                              BRXOffset:
                                                  0 as libc::c_int as SWORD,
                                              BRYOffset:
                                                  0 as libc::c_int as SWORD,
                                              ColourIndex:
                                                  0 as libc::c_int as UBYTE,};
                                init
                            },
                            {
                                let mut init =
                                    FRAMERECT{Type:
                                                  FR_IGNORE as libc::c_int as
                                                      UWORD,
                                              TLXOffset:
                                                  0 as libc::c_int as SWORD,
                                              TLYOffset:
                                                  0 as libc::c_int as SWORD,
                                              BRXOffset:
                                                  0 as libc::c_int as SWORD,
                                              BRYOffset:
                                                  0 as libc::c_int as SWORD,
                                              ColourIndex:
                                                  0 as libc::c_int as UBYTE,};
                                init
                            }],};
        init
    };
#[no_mangle]
pub static mut FrameRadar: IMAGEFRAME =
    {
        let mut init =
            IMAGEFRAME{OffsetX0: 0 as libc::c_int as SWORD,
                       OffsetY0: 0 as libc::c_int as SWORD,
                       OffsetX1: 0 as libc::c_int as SWORD,
                       OffsetY1: 0 as libc::c_int as SWORD,
                       TopLeft: IMAGE_FRAME_C0 as libc::c_int as SWORD,
                       TopRight: IMAGE_FRAME_C1 as libc::c_int as SWORD,
                       BottomLeft: IMAGE_FRAME_C3 as libc::c_int as SWORD,
                       BottomRight: IMAGE_FRAME_C2 as libc::c_int as SWORD,
                       TopEdge: IMAGE_FRAME_HT as libc::c_int as SWORD,
                       TopType: FR_SOLID as libc::c_int as SWORD,
                       RightEdge: IMAGE_FRAME_VR as libc::c_int as SWORD,
                       RightType: FR_SOLID as libc::c_int as SWORD,
                       BottomEdge: IMAGE_FRAME_HB as libc::c_int as SWORD,
                       BottomType: FR_SOLID as libc::c_int as SWORD,
                       LeftEdge: IMAGE_FRAME_VL as libc::c_int as SWORD,
                       LeftType: FR_SOLID as libc::c_int as SWORD,
                       FRect:
                           [{
                                let mut init =
                                    FRAMERECT{Type:
                                                  FR_IGNORE as libc::c_int as
                                                      UWORD,
                                              TLXOffset:
                                                  0 as libc::c_int as SWORD,
                                              TLYOffset:
                                                  0 as libc::c_int as SWORD,
                                              BRXOffset:
                                                  0 as libc::c_int as SWORD,
                                              BRYOffset:
                                                  0 as libc::c_int as SWORD,
                                              ColourIndex:
                                                  0 as libc::c_int as UBYTE,};
                                init
                            },
                            {
                                let mut init =
                                    FRAMERECT{Type:
                                                  FR_IGNORE as libc::c_int as
                                                      UWORD,
                                              TLXOffset:
                                                  0 as libc::c_int as SWORD,
                                              TLYOffset:
                                                  0 as libc::c_int as SWORD,
                                              BRXOffset:
                                                  0 as libc::c_int as SWORD,
                                              BRYOffset:
                                                  0 as libc::c_int as SWORD,
                                              ColourIndex:
                                                  0 as libc::c_int as UBYTE,};
                                init
                            },
                            {
                                let mut init =
                                    FRAMERECT{Type:
                                                  FR_IGNORE as libc::c_int as
                                                      UWORD,
                                              TLXOffset:
                                                  0 as libc::c_int as SWORD,
                                              TLYOffset:
                                                  0 as libc::c_int as SWORD,
                                              BRXOffset:
                                                  0 as libc::c_int as SWORD,
                                              BRYOffset:
                                                  0 as libc::c_int as SWORD,
                                              ColourIndex:
                                                  0 as libc::c_int as UBYTE,};
                                init
                            },
                            {
                                let mut init =
                                    FRAMERECT{Type:
                                                  FR_IGNORE as libc::c_int as
                                                      UWORD,
                                              TLXOffset:
                                                  0 as libc::c_int as SWORD,
                                              TLYOffset:
                                                  0 as libc::c_int as SWORD,
                                              BRXOffset:
                                                  0 as libc::c_int as SWORD,
                                              BRYOffset:
                                                  0 as libc::c_int as SWORD,
                                              ColourIndex:
                                                  0 as libc::c_int as UBYTE,};
                                init
                            },
                            {
                                let mut init =
                                    FRAMERECT{Type:
                                                  FR_IGNORE as libc::c_int as
                                                      UWORD,
                                              TLXOffset:
                                                  0 as libc::c_int as SWORD,
                                              TLYOffset:
                                                  0 as libc::c_int as SWORD,
                                              BRXOffset:
                                                  0 as libc::c_int as SWORD,
                                              BRYOffset:
                                                  0 as libc::c_int as SWORD,
                                              ColourIndex:
                                                  0 as libc::c_int as UBYTE,};
                                init
                            }],};
        init
    };
//IMAGEFRAME FrameObject = {
//	0,0, 0,0,
//	-1,
//	-1,
//	-1,
//	-1,
//	-1, FR_SOLID,
//	-1, FR_SOLID,
//	-1, FR_SOLID,
//	-1, FR_SOLID,
//	{{FR_IGNORE, 0,0, 0,0 ,0},
//	{FR_IGNORE, 0,0, 0,0 ,0},
//	{FR_IGNORE, 0,0, 0,0 ,0},
//	{FR_IGNORE, 0,0, 0,0 ,0},
//	{FR_IGNORE, 0,0, 0,0 ,0}},
//};
//
//IMAGEFRAME FrameStats = {
//	0,0, 0,0,
//	-1,
//	-1,
//	-1,
//	-1,
//	IMAGE_FRAME_HT, FR_SOLID,
//	IMAGE_FRAME_VR, FR_SOLID,
//	IMAGE_FRAME_HB, FR_SOLID,
//	IMAGE_FRAME_VL, FR_SOLID,
//	{{FR_FRAME, 8,3, -6,-5 ,190},
//	{FR_IGNORE, 0,0, 0,0 ,0},
//	{FR_IGNORE, 0,0, 0,0 ,0},
//	{FR_IGNORE, 0,0, 0,0 ,0},
//	{FR_IGNORE, 0,0, 0,0 ,0}},
//};
//
//IMAGEFRAME FrameDesignView = {
//	0,0, 0,0,
//	IMAGE_FRAME_VC0,
//	IMAGE_FRAME_VC1,
//	IMAGE_FRAME_VC2,
//	IMAGE_FRAME_VC3,
//	IMAGE_FRAME_HT2, FR_SOLID,
//	IMAGE_FRAME_VR2, FR_SOLID,
//	IMAGE_FRAME_HB2, FR_SOLID,
//	IMAGE_FRAME_VL2, FR_SOLID,
//	{{FR_FRAME, 0,0, 0,0, 1},
//	{FR_FRAME, 0,0, 0,0 ,1},
//	{FR_FRAME, 0,0, 0,0 ,1},
//	{FR_FRAME, 0,0, 0,0 ,1},
//	{FR_FRAME, 0,0, 0,0 ,1}},
//};
//
//IMAGEFRAME FrameDesignHilight = {
//	0,0, 0,0,
//	IMAGE_FRAME_HC0,
//	IMAGE_FRAME_HC1,
//	IMAGE_FRAME_HC2,
//	IMAGE_FRAME_HC3,
//	IMAGE_FRAME_HTH, FR_SOLID,
//	IMAGE_FRAME_VRH, FR_SOLID,
//	IMAGE_FRAME_HBH, FR_SOLID,
//	IMAGE_FRAME_VLH, FR_SOLID,
//	{{FR_FRAME, 0,0, 0,0, 1},
//	{FR_FRAME, 0,0, 0,0 ,1},
//	{FR_FRAME, 0,0, 0,0 ,1},
//	{FR_FRAME, 0,0, 0,0 ,1},
//	{FR_FRAME, 0,0, 0,0 ,1}},
//};
//
//IMAGEFRAME FrameText = {
//	0,0, 0,0,
//	-1,
//	-1,
//	IMAGE_FRAME_C3,
//	IMAGE_FRAME_C2,
//	-1, FR_SOLID,
//	IMAGE_FRAME_VR, FR_SOLID,
//	IMAGE_FRAME_HB, FR_SOLID,
//	IMAGE_FRAME_VL, FR_SOLID,
//	{{FR_FRAME,	0,1, 0,-1 ,224},
//	{FR_IGNORE, 0,0, 0,0 ,0},
//	{FR_IGNORE, 0,0, 0,0 ,0},
//	{FR_IGNORE, 0,0, 0,0 ,0},
//	{FR_IGNORE, 0,0, 0,0 ,0}},
//};
// Tab definitions, defines graphics to use for major and minor tabs.
#[no_mangle]
pub static mut StandardTab: TABDEF =
    {
        let mut init =
            TABDEF{MajorUp: IMAGE_TAB1 as libc::c_int as SWORD,
                   MajorDown: IMAGE_TAB1DOWN as libc::c_int as SWORD,
                   MajorHilight: IMAGE_TABHILIGHT as libc::c_int as SWORD,
                   MajorSelected: IMAGE_TABSELECTED as libc::c_int as SWORD,
                   MinorUp: IMAGE_TAB1 as libc::c_int as SWORD,
                   MinorDown: IMAGE_TAB1DOWN as libc::c_int as SWORD,
                   MinorHilight: IMAGE_TABHILIGHT as libc::c_int as SWORD,
                   MinorSelected: IMAGE_TABSELECTED as libc::c_int as SWORD,};
        init
    };
#[no_mangle]
pub static mut SystemTab: TABDEF =
    {
        let mut init =
            TABDEF{MajorUp: IMAGE_DES_WEAPONS as libc::c_int as SWORD,
                   MajorDown: IMAGE_DES_WEAPONSDOWN as libc::c_int as SWORD,
                   MajorHilight: IMAGE_DES_EXTRAHI as libc::c_int as SWORD,
                   MajorSelected:
                       IMAGE_DES_WEAPONSDOWN as libc::c_int as SWORD,
                   MinorUp: IMAGE_SIDETAB as libc::c_int as SWORD,
                   MinorDown: IMAGE_SIDETABDOWN as libc::c_int as SWORD,
                   MinorHilight: IMAGE_SIDETABHI as libc::c_int as SWORD,
                   MinorSelected: IMAGE_SIDETABSEL as libc::c_int as SWORD,};
        init
    };
#[no_mangle]
pub static mut SmallTab: TABDEF =
    {
        let mut init =
            TABDEF{MajorUp: IMAGE_TAB1_SM as libc::c_int as SWORD,
                   MajorDown: IMAGE_TAB1DOWN_SM as libc::c_int as SWORD,
                   MajorHilight: IMAGE_TABHILIGHT_SM as libc::c_int as SWORD,
                   MajorSelected:
                       IMAGE_TAB1SELECTED_SM as libc::c_int as SWORD,
                   MinorUp: IMAGE_TAB1_SM as libc::c_int as SWORD,
                   MinorDown: IMAGE_TAB1DOWN_SM as libc::c_int as SWORD,
                   MinorHilight: IMAGE_TABHILIGHT_SM as libc::c_int as SWORD,
                   MinorSelected:
                       IMAGE_TAB1SELECTED_SM as libc::c_int as SWORD,};
        init
    };
// A few useful defined tabs.
// Read bitmaps used by the interface.
//
#[no_mangle]
pub unsafe extern "C" fn imageInitBitmaps() -> BOOL {
    IntImages =
        resGetData(b"IMG\x00" as *const u8 as *const libc::c_char as
                       *mut STRING,
                   b"intfac.img\x00" as *const u8 as *const libc::c_char as
                       *mut STRING) as *mut IMAGEFILE;
    //	IntImages = iV_LoadImageFile("intpc.img");
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn imageDeleteBitmaps() {
    //	iV_FreeImageFile(IntImages);
}
#[no_mangle]
pub unsafe extern "C" fn DrawEnableLocks(mut Enable: BOOL) {
    EnableLocks = Enable;
}
#[no_mangle]
pub unsafe extern "C" fn DrawBegin() {
    if EnableLocks != 0 {
        if LockRefs == 0 as libc::c_int { pie_LocalRenderBegin(); }
        LockRefs += 1
    };
}
// Draw an image to a surface.
//extern void DrawImageSR(IMAGE *Image,UDWORD x,UDWORD y);
// Draw an image to a display memory.
//extern void DrawImage4101(IMAGE *Image,UDWORD x,UDWORD y);
//extern void DrawImageRect4101(IMAGE *Image,UDWORD x,UDWORD y,UDWORD x0,UDWORD y0,UDWORD Width,UDWORD Height);
//extern void DrawTransImageRect4101(IMAGE *Image,UDWORD x,UDWORD y,UDWORD x0,UDWORD y0,UDWORD Width,UDWORD Height);
// Draw an image to a surface with colour 0 transparent.
//extern void DrawTransImageSR(IMAGE *Image,UDWORD x,UDWORD y);
// Draw an image to a display memory with colour 0 transparent.
//extern void DrawTransImage4101(IMAGE *Image,UDWORD x,UDWORD y);
// Enable / Dissable locks.
// Begin a rendering lock.
// End a rendering lock.
#[no_mangle]
pub unsafe extern "C" fn DrawEnd() {
    if EnableLocks != 0 {
        LockRefs -= 1;
        if LockRefs >= 0 as libc::c_int {
        } else {
            debug(LOG_ERROR,
                  b"Inbalanced DrawEnd()\x00" as *const u8 as
                      *const libc::c_char);
        };
        if LockRefs >= 0 as libc::c_int {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"intimage.c\x00" as *const u8 as *const libc::c_char,
                  272 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 8],
                                            &[libc::c_char; 8]>(b"DrawEnd\x00")).as_ptr(),
                  b"LockRefs >= 0\x00" as *const u8 as *const libc::c_char);
        };
        if LockRefs == 0 as libc::c_int { pie_LocalRenderEnd(); }
    };
}
#[no_mangle]
pub unsafe extern "C" fn RenderWindowFrame(mut Frame: *mut IMAGEFRAME,
                                           mut x: UDWORD, mut y: UDWORD,
                                           mut Width: UDWORD,
                                           mut Height: UDWORD) {
    RenderWindow(Frame, x, y, Width, Height, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn RenderOpaqueWindow(mut Frame: *mut IMAGEFRAME,
                                            mut x: UDWORD, mut y: UDWORD,
                                            mut Width: UDWORD,
                                            mut Height: UDWORD) {
    RenderWindow(Frame, x, y, Width, Height, 1 as libc::c_int);
}
// Draws a transparent window
// Draws a solid window
// Called by RenderWindowFrame and RenderOpaqueWindow but you can call it yourself if you want.
// Render a window frame.
//
#[no_mangle]
pub unsafe extern "C" fn RenderWindow(mut Frame: *mut IMAGEFRAME,
                                      mut x: UDWORD, mut y: UDWORD,
                                      mut Width: UDWORD, mut Height: UDWORD,
                                      mut Opaque: BOOL) {
    let mut WTopRight: SWORD =
        0 as libc::c_int as
            SWORD; // Software transboxfill needs to be a multiple of 4 pixels.
    let mut WTopLeft: SWORD =
        0 as libc::c_int as
            SWORD; // Software transboxfill needs to be a multiple of 4 pixels.
    let mut WBottomRight: SWORD =
        0 as libc::c_int as
            SWORD; // Software transboxfill needs to be a multiple of 4 pixels.
    let mut WBottomLeft: SWORD =
        0 as libc::c_int as
            SWORD; // Software transboxfill needs to be a multiple of 4 pixels.
    let mut HTopRight: SWORD =
        0 as libc::c_int as
            SWORD; // Software transboxfill needs to be a multiple of 4 pixels.
    let mut HTopLeft: SWORD = 0 as libc::c_int as SWORD;
    let mut HBottomRight: SWORD = 0 as libc::c_int as SWORD;
    let mut HBottomLeft: SWORD = 0 as libc::c_int as SWORD;
    let mut RectI: UWORD = 0;
    let mut Rect: *mut FRAMERECT = 0 as *mut FRAMERECT;
    let mut Masked: BOOL = 0 as libc::c_int;
    x =
        (x as libc::c_uint).wrapping_add((*Frame).OffsetX0 as libc::c_uint) as
            UDWORD as UDWORD;
    y =
        (y as libc::c_uint).wrapping_add((*Frame).OffsetY0 as libc::c_uint) as
            UDWORD as UDWORD;
    Width =
        (Width as
             libc::c_uint).wrapping_sub(((*Frame).OffsetX1 as libc::c_int +
                                             (*Frame).OffsetX0 as libc::c_int)
                                            as libc::c_uint) as UDWORD as
            UDWORD;
    Height =
        (Height as
             libc::c_uint).wrapping_sub(((*Frame).OffsetY1 as libc::c_int +
                                             (*Frame).OffsetY0 as libc::c_int)
                                            as libc::c_uint) as UDWORD as
            UDWORD;
    RectI = 0 as libc::c_int as UWORD;
    while (RectI as libc::c_int) < 5 as libc::c_int {
        Rect =
            &mut *(*Frame).FRect.as_mut_ptr().offset(RectI as isize) as
                *mut FRAMERECT;
        match (*Rect).Type as libc::c_int {
            1 => {
                if Opaque == 0 as libc::c_int {
                    if Masked == 0 as libc::c_int {
                        Width &= 0xfffc as libc::c_int as libc::c_uint;
                        Masked = 1 as libc::c_int
                    }
                    pie_TransBoxFill(x.wrapping_add((*Rect).TLXOffset as
                                                        libc::c_uint) as
                                         SDWORD,
                                     y.wrapping_add((*Rect).TLYOffset as
                                                        libc::c_uint) as
                                         SDWORD,
                                     x.wrapping_add(Width).wrapping_sub(0 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint).wrapping_add((*Rect).BRXOffset
                                                                                                           as
                                                                                                           libc::c_uint)
                                         as SDWORD,
                                     y.wrapping_add(Height).wrapping_sub(0 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint).wrapping_add((*Rect).BRYOffset
                                                                                                            as
                                                                                                            libc::c_uint)
                                         as SDWORD);
                } else {
                    pie_BoxFillIndex(x.wrapping_add((*Rect).TLXOffset as
                                                        libc::c_uint) as
                                         libc::c_int,
                                     y.wrapping_add((*Rect).TLYOffset as
                                                        libc::c_uint) as
                                         libc::c_int,
                                     x.wrapping_add(Width).wrapping_sub(0 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint).wrapping_add((*Rect).BRXOffset
                                                                                                           as
                                                                                                           libc::c_uint)
                                         as libc::c_int,
                                     y.wrapping_add(Height).wrapping_sub(0 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint).wrapping_add((*Rect).BRYOffset
                                                                                                            as
                                                                                                            libc::c_uint)
                                         as libc::c_int, (*Rect).ColourIndex);
                }
            }
            2 => {
                if Opaque == 0 as libc::c_int {
                    if Masked == 0 as libc::c_int {
                        Width &= 0xfffc as libc::c_int as libc::c_uint;
                        Masked = 1 as libc::c_int
                    }
                    pie_TransBoxFill(x.wrapping_add((*Rect).TLXOffset as
                                                        libc::c_uint) as
                                         SDWORD,
                                     y.wrapping_add((*Rect).TLYOffset as
                                                        libc::c_uint) as
                                         SDWORD,
                                     x.wrapping_add((*Rect).BRXOffset as
                                                        libc::c_uint) as
                                         SDWORD,
                                     y.wrapping_add(Height).wrapping_sub(0 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint).wrapping_add((*Rect).BRYOffset
                                                                                                            as
                                                                                                            libc::c_uint)
                                         as SDWORD);
                } else {
                    pie_BoxFillIndex(x.wrapping_add((*Rect).TLXOffset as
                                                        libc::c_uint) as
                                         libc::c_int,
                                     y.wrapping_add((*Rect).TLYOffset as
                                                        libc::c_uint) as
                                         libc::c_int,
                                     x.wrapping_add((*Rect).BRXOffset as
                                                        libc::c_uint) as
                                         libc::c_int,
                                     y.wrapping_add(Height).wrapping_sub(0 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint).wrapping_add((*Rect).BRYOffset
                                                                                                            as
                                                                                                            libc::c_uint)
                                         as libc::c_int, (*Rect).ColourIndex);
                }
            }
            3 => {
                if Opaque == 0 as libc::c_int {
                    if Masked == 0 as libc::c_int {
                        Width &= 0xfffc as libc::c_int as libc::c_uint;
                        Masked = 1 as libc::c_int
                    }
                    pie_TransBoxFill(x.wrapping_add(Width).wrapping_sub(0 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint).wrapping_add((*Rect).TLXOffset
                                                                                                           as
                                                                                                           libc::c_uint)
                                         as SDWORD,
                                     y.wrapping_add((*Rect).TLYOffset as
                                                        libc::c_uint) as
                                         SDWORD,
                                     x.wrapping_add(Width).wrapping_sub(0 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint).wrapping_add((*Rect).BRXOffset
                                                                                                           as
                                                                                                           libc::c_uint)
                                         as SDWORD,
                                     y.wrapping_add(Height).wrapping_sub(0 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint).wrapping_add((*Rect).BRYOffset
                                                                                                            as
                                                                                                            libc::c_uint)
                                         as SDWORD);
                } else {
                    pie_BoxFillIndex(x.wrapping_add(Width).wrapping_sub(0 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint).wrapping_add((*Rect).TLXOffset
                                                                                                           as
                                                                                                           libc::c_uint)
                                         as libc::c_int,
                                     y.wrapping_add((*Rect).TLYOffset as
                                                        libc::c_uint) as
                                         libc::c_int,
                                     x.wrapping_add(Width).wrapping_sub(0 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint).wrapping_add((*Rect).BRXOffset
                                                                                                           as
                                                                                                           libc::c_uint)
                                         as libc::c_int,
                                     y.wrapping_add(Height).wrapping_sub(0 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint).wrapping_add((*Rect).BRYOffset
                                                                                                            as
                                                                                                            libc::c_uint)
                                         as libc::c_int, (*Rect).ColourIndex);
                }
            }
            4 => {
                if Opaque == 0 as libc::c_int {
                    if Masked == 0 as libc::c_int {
                        Width &= 0xfffc as libc::c_int as libc::c_uint;
                        Masked = 1 as libc::c_int
                    }
                    pie_TransBoxFill(x.wrapping_add((*Rect).TLXOffset as
                                                        libc::c_uint) as
                                         SDWORD,
                                     y.wrapping_add((*Rect).TLYOffset as
                                                        libc::c_uint) as
                                         SDWORD,
                                     x.wrapping_add(Width).wrapping_sub(0 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint).wrapping_add((*Rect).BRXOffset
                                                                                                           as
                                                                                                           libc::c_uint)
                                         as SDWORD,
                                     y.wrapping_add((*Rect).BRYOffset as
                                                        libc::c_uint) as
                                         SDWORD);
                } else {
                    pie_BoxFillIndex(x.wrapping_add((*Rect).TLXOffset as
                                                        libc::c_uint) as
                                         libc::c_int,
                                     y.wrapping_add((*Rect).TLYOffset as
                                                        libc::c_uint) as
                                         libc::c_int,
                                     x.wrapping_add(Width).wrapping_sub(0 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint).wrapping_add((*Rect).BRXOffset
                                                                                                           as
                                                                                                           libc::c_uint)
                                         as libc::c_int,
                                     y.wrapping_add((*Rect).BRYOffset as
                                                        libc::c_uint) as
                                         libc::c_int, (*Rect).ColourIndex);
                }
            }
            5 => {
                if Opaque == 0 as libc::c_int {
                    if Masked == 0 as libc::c_int {
                        Width &= 0xfffc as libc::c_int as libc::c_uint;
                        Masked = 1 as libc::c_int
                    }
                    pie_TransBoxFill(x.wrapping_add((*Rect).TLXOffset as
                                                        libc::c_uint) as
                                         SDWORD,
                                     y.wrapping_add(Height).wrapping_sub(0 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint).wrapping_add((*Rect).TLYOffset
                                                                                                            as
                                                                                                            libc::c_uint)
                                         as SDWORD,
                                     x.wrapping_add(Width).wrapping_sub(0 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint).wrapping_add((*Rect).BRXOffset
                                                                                                           as
                                                                                                           libc::c_uint)
                                         as SDWORD,
                                     y.wrapping_add(Height).wrapping_sub(0 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint).wrapping_add((*Rect).BRYOffset
                                                                                                            as
                                                                                                            libc::c_uint)
                                         as SDWORD);
                } else {
                    pie_BoxFillIndex(x.wrapping_add((*Rect).TLXOffset as
                                                        libc::c_uint) as
                                         libc::c_int,
                                     y.wrapping_add(Height).wrapping_sub(0 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint).wrapping_add((*Rect).TLYOffset
                                                                                                            as
                                                                                                            libc::c_uint)
                                         as libc::c_int,
                                     x.wrapping_add(Width).wrapping_sub(0 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint).wrapping_add((*Rect).BRXOffset
                                                                                                           as
                                                                                                           libc::c_uint)
                                         as libc::c_int,
                                     y.wrapping_add(Height).wrapping_sub(0 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint).wrapping_add((*Rect).BRYOffset
                                                                                                            as
                                                                                                            libc::c_uint)
                                         as libc::c_int, (*Rect).ColourIndex);
                }
            }
            _ => { }
        }
        RectI = RectI.wrapping_add(1)
    }
    DrawBegin();
    if (*Frame).TopLeft as libc::c_int >= 0 as libc::c_int {
        WTopLeft =
            iV_GetImageWidth(IntImages, (*Frame).TopLeft as UWORD) as SWORD;
        HTopLeft =
            iV_GetImageHeight(IntImages, (*Frame).TopLeft as UWORD) as SWORD;
        pie_ImageFileID(IntImages, (*Frame).TopLeft as UWORD,
                        x as libc::c_int, y as libc::c_int);
    }
    if (*Frame).TopRight as libc::c_int >= 0 as libc::c_int {
        WTopRight =
            iV_GetImageWidth(IntImages, (*Frame).TopRight as UWORD) as SWORD;
        HTopRight =
            iV_GetImageHeight(IntImages, (*Frame).TopRight as UWORD) as SWORD;
        pie_ImageFileID(IntImages, (*Frame).TopRight as UWORD,
                        x.wrapping_add(Width).wrapping_sub(WTopRight as
                                                               libc::c_uint)
                            as libc::c_int, y as libc::c_int);
    }
    if (*Frame).BottomRight as libc::c_int >= 0 as libc::c_int {
        WBottomRight =
            iV_GetImageWidth(IntImages, (*Frame).BottomRight as UWORD) as
                SWORD;
        HBottomRight =
            iV_GetImageHeight(IntImages, (*Frame).BottomRight as UWORD) as
                SWORD;
        pie_ImageFileID(IntImages, (*Frame).BottomRight as UWORD,
                        x.wrapping_add(Width).wrapping_sub(WBottomRight as
                                                               libc::c_uint)
                            as libc::c_int,
                        y.wrapping_add(Height).wrapping_sub(HBottomRight as
                                                                libc::c_uint)
                            as libc::c_int);
    }
    if (*Frame).BottomLeft as libc::c_int >= 0 as libc::c_int {
        WBottomLeft =
            iV_GetImageWidth(IntImages, (*Frame).BottomLeft as UWORD) as
                SWORD;
        HBottomLeft =
            iV_GetImageHeight(IntImages, (*Frame).BottomLeft as UWORD) as
                SWORD;
        pie_ImageFileID(IntImages, (*Frame).BottomLeft as UWORD,
                        x as libc::c_int,
                        y.wrapping_add(Height).wrapping_sub(HBottomLeft as
                                                                libc::c_uint)
                            as libc::c_int);
    }
    if (*Frame).TopEdge as libc::c_int >= 0 as libc::c_int {
        if (*Frame).TopType as libc::c_int == FR_SOLID as libc::c_int {
            pie_ImageFileIDTile(IntImages, (*Frame).TopEdge as UWORD,
                                x.wrapping_add(iV_GetImageWidth(IntImages,
                                                                (*Frame).TopLeft
                                                                    as UWORD)
                                                   as libc::c_uint) as
                                    libc::c_int, y as libc::c_int,
                                0 as libc::c_int, 0 as libc::c_int,
                                Width.wrapping_sub(WTopLeft as
                                                       libc::c_uint).wrapping_sub(WTopRight
                                                                                      as
                                                                                      libc::c_uint)
                                    as libc::c_int,
                                iV_GetImageHeight(IntImages,
                                                  (*Frame).TopEdge as UWORD)
                                    as libc::c_int);
        } else {
            pie_ImageFileIDTile(IntImages, (*Frame).TopEdge as UWORD,
                                x.wrapping_add(iV_GetImageWidth(IntImages,
                                                                (*Frame).TopLeft
                                                                    as UWORD)
                                                   as libc::c_uint) as
                                    libc::c_int, y as libc::c_int,
                                0 as libc::c_int, 0 as libc::c_int,
                                Width.wrapping_sub(WTopLeft as
                                                       libc::c_uint).wrapping_sub(WTopRight
                                                                                      as
                                                                                      libc::c_uint)
                                    as libc::c_int,
                                iV_GetImageHeight(IntImages,
                                                  (*Frame).TopEdge as UWORD)
                                    as libc::c_int);
        }
    }
    if (*Frame).BottomEdge as libc::c_int >= 0 as libc::c_int {
        if (*Frame).BottomType as libc::c_int == FR_SOLID as libc::c_int {
            pie_ImageFileIDTile(IntImages, (*Frame).BottomEdge as UWORD,
                                x.wrapping_add(WBottomLeft as libc::c_uint) as
                                    libc::c_int,
                                y.wrapping_add(Height).wrapping_sub(iV_GetImageHeight(IntImages,
                                                                                      (*Frame).BottomEdge
                                                                                          as
                                                                                          UWORD)
                                                                        as
                                                                        libc::c_uint)
                                    as libc::c_int, 0 as libc::c_int,
                                0 as libc::c_int,
                                Width.wrapping_sub(WBottomLeft as
                                                       libc::c_uint).wrapping_sub(WBottomRight
                                                                                      as
                                                                                      libc::c_uint)
                                    as libc::c_int,
                                iV_GetImageHeight(IntImages,
                                                  (*Frame).BottomEdge as
                                                      UWORD) as libc::c_int);
        } else {
            pie_ImageFileIDTile(IntImages, (*Frame).BottomEdge as UWORD,
                                x.wrapping_add(WBottomLeft as libc::c_uint) as
                                    libc::c_int,
                                y.wrapping_add(Height).wrapping_sub(iV_GetImageHeight(IntImages,
                                                                                      (*Frame).BottomEdge
                                                                                          as
                                                                                          UWORD)
                                                                        as
                                                                        libc::c_uint)
                                    as libc::c_int, 0 as libc::c_int,
                                0 as libc::c_int,
                                Width.wrapping_sub(WBottomLeft as
                                                       libc::c_uint).wrapping_sub(WBottomRight
                                                                                      as
                                                                                      libc::c_uint)
                                    as libc::c_int,
                                iV_GetImageHeight(IntImages,
                                                  (*Frame).BottomEdge as
                                                      UWORD) as libc::c_int);
        }
    }
    if (*Frame).LeftEdge as libc::c_int >= 0 as libc::c_int {
        if (*Frame).LeftType as libc::c_int == FR_SOLID as libc::c_int {
            pie_ImageFileIDTile(IntImages, (*Frame).LeftEdge as UWORD,
                                x as libc::c_int,
                                y.wrapping_add(HTopLeft as libc::c_uint) as
                                    libc::c_int, 0 as libc::c_int,
                                0 as libc::c_int,
                                iV_GetImageWidth(IntImages,
                                                 (*Frame).LeftEdge as UWORD)
                                    as libc::c_int,
                                Height.wrapping_sub(HTopLeft as
                                                        libc::c_uint).wrapping_sub(HBottomLeft
                                                                                       as
                                                                                       libc::c_uint)
                                    as libc::c_int);
        } else {
            pie_ImageFileIDTile(IntImages, (*Frame).LeftEdge as UWORD,
                                x as libc::c_int,
                                y.wrapping_add(HTopLeft as libc::c_uint) as
                                    libc::c_int, 0 as libc::c_int,
                                0 as libc::c_int,
                                iV_GetImageWidth(IntImages,
                                                 (*Frame).LeftEdge as UWORD)
                                    as libc::c_int,
                                Height.wrapping_sub(HTopLeft as
                                                        libc::c_uint).wrapping_sub(HBottomLeft
                                                                                       as
                                                                                       libc::c_uint)
                                    as libc::c_int);
        }
    }
    if (*Frame).RightEdge as libc::c_int >= 0 as libc::c_int {
        if (*Frame).RightType as libc::c_int == FR_SOLID as libc::c_int {
            pie_ImageFileIDTile(IntImages, (*Frame).RightEdge as UWORD,
                                x.wrapping_add(Width).wrapping_sub(iV_GetImageWidth(IntImages,
                                                                                    (*Frame).RightEdge
                                                                                        as
                                                                                        UWORD)
                                                                       as
                                                                       libc::c_uint)
                                    as libc::c_int,
                                y.wrapping_add(HTopRight as libc::c_uint) as
                                    libc::c_int, 0 as libc::c_int,
                                0 as libc::c_int,
                                iV_GetImageWidth(IntImages,
                                                 (*Frame).RightEdge as UWORD)
                                    as libc::c_int,
                                Height.wrapping_sub(HTopRight as
                                                        libc::c_uint).wrapping_sub(HBottomRight
                                                                                       as
                                                                                       libc::c_uint)
                                    as libc::c_int);
        } else {
            pie_ImageFileIDTile(IntImages, (*Frame).RightEdge as UWORD,
                                x.wrapping_add(Width).wrapping_sub(iV_GetImageWidth(IntImages,
                                                                                    (*Frame).RightEdge
                                                                                        as
                                                                                        UWORD)
                                                                       as
                                                                       libc::c_uint)
                                    as libc::c_int,
                                y.wrapping_add(HTopRight as libc::c_uint) as
                                    libc::c_int, 0 as libc::c_int,
                                0 as libc::c_int,
                                iV_GetImageWidth(IntImages,
                                                 (*Frame).RightEdge as UWORD)
                                    as libc::c_int,
                                Height.wrapping_sub(HTopRight as
                                                        libc::c_uint).wrapping_sub(HBottomRight
                                                                                       as
                                                                                       libc::c_uint)
                                    as libc::c_int);
        }
    }
    DrawEnd();
}
