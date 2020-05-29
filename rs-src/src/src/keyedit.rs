use ::libc;
extern "C" {
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_uint)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn abort() -> !;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_uint) -> libc::c_int;
    #[no_mangle]
    fn PHYSFS_getLastError() -> *const libc::c_char;
    #[no_mangle]
    fn PHYSFS_exists(fname: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn PHYSFS_openWrite(filename: *const libc::c_char) -> *mut PHYSFS_File;
    #[no_mangle]
    fn PHYSFS_openRead(filename: *const libc::c_char) -> *mut PHYSFS_File;
    #[no_mangle]
    fn PHYSFS_close(handle: *mut PHYSFS_File) -> libc::c_int;
    #[no_mangle]
    fn PHYSFS_read(handle: *mut PHYSFS_File, buffer: *mut libc::c_void,
                   objSize: PHYSFS_uint32, objCount: PHYSFS_uint32)
     -> PHYSFS_sint64;
    #[no_mangle]
    fn PHYSFS_write(handle: *mut PHYSFS_File, buffer: *const libc::c_void,
                    objSize: PHYSFS_uint32, objCount: PHYSFS_uint32)
     -> PHYSFS_sint64;
    #[no_mangle]
    fn PHYSFS_fileLength(handle: *mut PHYSFS_File) -> PHYSFS_sint64;
    #[no_mangle]
    fn keyPressed(code: KEY_CODE) -> BOOL;
    #[no_mangle]
    fn keyDown(code: KEY_CODE) -> BOOL;
    #[no_mangle]
    fn keyScanToString(code: KEY_CODE, ascii: *mut STRING,
                       maxStringSize: UDWORD);
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
    /* Get the string from an ID number */
    #[no_mangle]
    fn strresGetString(psRes: *mut STR_RES, id: UDWORD) -> *mut STRING;
    /* Add a form to the widget screen */
    #[no_mangle]
    fn widgAddForm(psScreen: *mut W_SCREEN, psInit: *mut W_FORMINIT) -> BOOL;
    /* Add a button to a form */
    #[no_mangle]
    fn widgAddButton(psScreen: *mut W_SCREEN, psInit: *mut W_BUTINIT) -> BOOL;
    /* Delete a widget from the screen */
    #[no_mangle]
    fn widgDelete(psScreen: *mut W_SCREEN, id: UDWORD);
    /* Get widget structure */
    /* Find a widget in a screen from its ID number */
    #[no_mangle]
    fn widgGetFromID(psScreen: *mut W_SCREEN, id: UDWORD) -> *mut WIDGET;
    /* Execute a set of widgets for one cycle.
 * Return the id of the widget that was activated, or 0 for none.
 */
    #[no_mangle]
    fn widgRunScreen(psScreen: *mut W_SCREEN) -> UDWORD;
    /* Display the screen's widgets in their current state
 * (Call after calling widgRunScreen, this allows the input
 *  processing to be seperated from the display of the widgets).
 */
    #[no_mangle]
    fn widgDisplayScreen(psScreen: *mut W_SCREEN);
    #[no_mangle]
    fn changeTitleMode(mode: tMode);
    #[no_mangle]
    fn addBackdrop();
    #[no_mangle]
    fn addSideText(id: UDWORD, PosX: UDWORD, PosY: UDWORD, txt: *mut STRING);
    /* The string resource object */
    #[no_mangle]
    static mut psStringRes: *mut STR_RES;
    #[no_mangle]
    fn iV_SetFont(FontID: libc::c_int);
    #[no_mangle]
    fn iV_SetTextColour(Index: SWORD);
    #[no_mangle]
    fn pie_DrawText(string: *mut STRING, x: UDWORD, y: UDWORD);
    //*************************************************************************
    //*************************************************************************
    #[no_mangle]
    static mut colours: [uint8; 0];
    #[no_mangle]
    fn audio_PlayTrack(iTrack: libc::c_int);
    /* The widget screen */
    #[no_mangle]
    static mut psWScreen: *mut W_SCREEN;
    /* the widget font */
    #[no_mangle]
    static mut WFont: libc::c_int;
    #[no_mangle]
    fn numForms(total: UDWORD, perForm: UDWORD) -> UWORD;
    /* **************************************************************************/
/*
 *	Global ProtoTypes
 */
/* **************************************************************************/
    #[no_mangle]
    fn drawBlueBox(x: UDWORD, y: UDWORD, w: UDWORD, h: UDWORD);
    //extern BOOL	keyAddMapping			( UDWORD functionId, KEY_CODE metaCode, KEY_CODE subCode );
//extern BOOL	keyAddMapping			( KEY_CODE metaCode, KEY_CODE subcode, KEY_ACTION action, void *function, STRING *name );
    #[no_mangle]
    fn keyAddMapping(status: KEY_STATUS, metaCode: KEY_CODE,
                     subcode: KEY_CODE, action: KEY_ACTION,
                     pKeyMapFunc: Option<unsafe extern "C" fn() -> ()>,
                     name: *mut STRING) -> *mut KEY_MAPPING;
    #[no_mangle]
    fn keyGetMappingFromFunction(function: *mut libc::c_void)
     -> *mut KEY_MAPPING;
    #[no_mangle]
    fn keyFindMapping(metaCode: KEY_CODE, subCode: KEY_CODE)
     -> *mut KEY_MAPPING;
    #[no_mangle]
    fn keyClearMappings();
    #[no_mangle]
    fn keyInitMappings(bForceDefaults: BOOL);
    #[no_mangle]
    fn keyReAssignMapping(origMetaCode: KEY_CODE, origSubCode: KEY_CODE,
                          newMetaCode: KEY_CODE, newSubCode: KEY_CODE)
     -> BOOL;
    #[no_mangle]
    static mut keyMapSaveTable: [_keymapsave; 0];
    #[no_mangle]
    static mut keyMappings: *mut KEY_MAPPING;
    #[no_mangle]
    fn StartCursorSnap(SnapBuffer: *mut CURSORSNAP);
    #[no_mangle]
    static mut StandardTab: TABDEF;
    // Begin a rendering lock.
    #[no_mangle]
    fn DrawBegin();
    // End a rendering lock.
    #[no_mangle]
    fn DrawEnd();
    #[no_mangle]
    fn iV_GetImageWidth(ImageFile: *mut IMAGEFILE, ID: UWORD) -> UWORD;
    #[no_mangle]
    fn iV_GetImageHeight(ImageFile: *mut IMAGEFILE, ID: UWORD) -> UWORD;
    #[no_mangle]
    fn intDisplayObjectForm(psWidget: *mut _widget, xOffset: UDWORD,
                            yOffset: UDWORD, pColours: *mut UDWORD);
    #[no_mangle]
    fn intDisplayPlainForm(psWidget: *mut _widget, xOffset: UDWORD,
                           yOffset: UDWORD, pColours: *mut UDWORD);
    #[no_mangle]
    fn intDisplayTab(psWidget: *mut _widget, TabType: UDWORD,
                     Position: UDWORD, Number: UDWORD, Selected: BOOL,
                     Hilight: BOOL, x: UDWORD, y: UDWORD, Width: UDWORD,
                     Height: UDWORD);
    #[no_mangle]
    fn pie_BoxFillIndex(x0: libc::c_int, y0: libc::c_int, x1: libc::c_int,
                        y1: libc::c_int, colour: UBYTE);
    #[no_mangle]
    fn addMultiBut(screen: *mut W_SCREEN, formid: UDWORD, id: UDWORD,
                   x: UDWORD, y: UDWORD, width: UDWORD, height: UDWORD,
                   tipres: UDWORD, norm: UDWORD, hi: UDWORD,
                   showmouseover: BOOL) -> BOOL;
    // ////////////////////////////////////////////////////////////////////////////
// variables
    #[no_mangle]
    static mut FrontImages: *mut IMAGEFILE;
    #[no_mangle]
    static mut InterfaceSnap: CURSORSNAP;
    #[no_mangle]
    static mut KeyMapPath: [libc::c_char; 0];
}
pub type C2RustUnnamed = libc::c_uint;
pub const SDLK_LAST: C2RustUnnamed = 323;
pub const SDLK_UNDO: C2RustUnnamed = 322;
pub const SDLK_EURO: C2RustUnnamed = 321;
pub const SDLK_POWER: C2RustUnnamed = 320;
pub const SDLK_MENU: C2RustUnnamed = 319;
pub const SDLK_BREAK: C2RustUnnamed = 318;
pub const SDLK_SYSREQ: C2RustUnnamed = 317;
pub const SDLK_PRINT: C2RustUnnamed = 316;
pub const SDLK_HELP: C2RustUnnamed = 315;
pub const SDLK_COMPOSE: C2RustUnnamed = 314;
pub const SDLK_MODE: C2RustUnnamed = 313;
pub const SDLK_RSUPER: C2RustUnnamed = 312;
pub const SDLK_LSUPER: C2RustUnnamed = 311;
pub const SDLK_LMETA: C2RustUnnamed = 310;
pub const SDLK_RMETA: C2RustUnnamed = 309;
pub const SDLK_LALT: C2RustUnnamed = 308;
pub const SDLK_RALT: C2RustUnnamed = 307;
pub const SDLK_LCTRL: C2RustUnnamed = 306;
pub const SDLK_RCTRL: C2RustUnnamed = 305;
pub const SDLK_LSHIFT: C2RustUnnamed = 304;
pub const SDLK_RSHIFT: C2RustUnnamed = 303;
pub const SDLK_SCROLLOCK: C2RustUnnamed = 302;
pub const SDLK_CAPSLOCK: C2RustUnnamed = 301;
pub const SDLK_NUMLOCK: C2RustUnnamed = 300;
pub const SDLK_F15: C2RustUnnamed = 296;
pub const SDLK_F14: C2RustUnnamed = 295;
pub const SDLK_F13: C2RustUnnamed = 294;
pub const SDLK_F12: C2RustUnnamed = 293;
pub const SDLK_F11: C2RustUnnamed = 292;
pub const SDLK_F10: C2RustUnnamed = 291;
pub const SDLK_F9: C2RustUnnamed = 290;
pub const SDLK_F8: C2RustUnnamed = 289;
pub const SDLK_F7: C2RustUnnamed = 288;
pub const SDLK_F6: C2RustUnnamed = 287;
pub const SDLK_F5: C2RustUnnamed = 286;
pub const SDLK_F4: C2RustUnnamed = 285;
pub const SDLK_F3: C2RustUnnamed = 284;
pub const SDLK_F2: C2RustUnnamed = 283;
pub const SDLK_F1: C2RustUnnamed = 282;
pub const SDLK_PAGEDOWN: C2RustUnnamed = 281;
pub const SDLK_PAGEUP: C2RustUnnamed = 280;
pub const SDLK_END: C2RustUnnamed = 279;
pub const SDLK_HOME: C2RustUnnamed = 278;
pub const SDLK_INSERT: C2RustUnnamed = 277;
pub const SDLK_LEFT: C2RustUnnamed = 276;
pub const SDLK_RIGHT: C2RustUnnamed = 275;
pub const SDLK_DOWN: C2RustUnnamed = 274;
pub const SDLK_UP: C2RustUnnamed = 273;
pub const SDLK_KP_EQUALS: C2RustUnnamed = 272;
pub const SDLK_KP_ENTER: C2RustUnnamed = 271;
pub const SDLK_KP_PLUS: C2RustUnnamed = 270;
pub const SDLK_KP_MINUS: C2RustUnnamed = 269;
pub const SDLK_KP_MULTIPLY: C2RustUnnamed = 268;
pub const SDLK_KP_DIVIDE: C2RustUnnamed = 267;
pub const SDLK_KP_PERIOD: C2RustUnnamed = 266;
pub const SDLK_KP9: C2RustUnnamed = 265;
pub const SDLK_KP8: C2RustUnnamed = 264;
pub const SDLK_KP7: C2RustUnnamed = 263;
pub const SDLK_KP6: C2RustUnnamed = 262;
pub const SDLK_KP5: C2RustUnnamed = 261;
pub const SDLK_KP4: C2RustUnnamed = 260;
pub const SDLK_KP3: C2RustUnnamed = 259;
pub const SDLK_KP2: C2RustUnnamed = 258;
pub const SDLK_KP1: C2RustUnnamed = 257;
pub const SDLK_KP0: C2RustUnnamed = 256;
pub const SDLK_WORLD_95: C2RustUnnamed = 255;
pub const SDLK_WORLD_94: C2RustUnnamed = 254;
pub const SDLK_WORLD_93: C2RustUnnamed = 253;
pub const SDLK_WORLD_92: C2RustUnnamed = 252;
pub const SDLK_WORLD_91: C2RustUnnamed = 251;
pub const SDLK_WORLD_90: C2RustUnnamed = 250;
pub const SDLK_WORLD_89: C2RustUnnamed = 249;
pub const SDLK_WORLD_88: C2RustUnnamed = 248;
pub const SDLK_WORLD_87: C2RustUnnamed = 247;
pub const SDLK_WORLD_86: C2RustUnnamed = 246;
pub const SDLK_WORLD_85: C2RustUnnamed = 245;
pub const SDLK_WORLD_84: C2RustUnnamed = 244;
pub const SDLK_WORLD_83: C2RustUnnamed = 243;
pub const SDLK_WORLD_82: C2RustUnnamed = 242;
pub const SDLK_WORLD_81: C2RustUnnamed = 241;
pub const SDLK_WORLD_80: C2RustUnnamed = 240;
pub const SDLK_WORLD_79: C2RustUnnamed = 239;
pub const SDLK_WORLD_78: C2RustUnnamed = 238;
pub const SDLK_WORLD_77: C2RustUnnamed = 237;
pub const SDLK_WORLD_76: C2RustUnnamed = 236;
pub const SDLK_WORLD_75: C2RustUnnamed = 235;
pub const SDLK_WORLD_74: C2RustUnnamed = 234;
pub const SDLK_WORLD_73: C2RustUnnamed = 233;
pub const SDLK_WORLD_72: C2RustUnnamed = 232;
pub const SDLK_WORLD_71: C2RustUnnamed = 231;
pub const SDLK_WORLD_70: C2RustUnnamed = 230;
pub const SDLK_WORLD_69: C2RustUnnamed = 229;
pub const SDLK_WORLD_68: C2RustUnnamed = 228;
pub const SDLK_WORLD_67: C2RustUnnamed = 227;
pub const SDLK_WORLD_66: C2RustUnnamed = 226;
pub const SDLK_WORLD_65: C2RustUnnamed = 225;
pub const SDLK_WORLD_64: C2RustUnnamed = 224;
pub const SDLK_WORLD_63: C2RustUnnamed = 223;
pub const SDLK_WORLD_62: C2RustUnnamed = 222;
pub const SDLK_WORLD_61: C2RustUnnamed = 221;
pub const SDLK_WORLD_60: C2RustUnnamed = 220;
pub const SDLK_WORLD_59: C2RustUnnamed = 219;
pub const SDLK_WORLD_58: C2RustUnnamed = 218;
pub const SDLK_WORLD_57: C2RustUnnamed = 217;
pub const SDLK_WORLD_56: C2RustUnnamed = 216;
pub const SDLK_WORLD_55: C2RustUnnamed = 215;
pub const SDLK_WORLD_54: C2RustUnnamed = 214;
pub const SDLK_WORLD_53: C2RustUnnamed = 213;
pub const SDLK_WORLD_52: C2RustUnnamed = 212;
pub const SDLK_WORLD_51: C2RustUnnamed = 211;
pub const SDLK_WORLD_50: C2RustUnnamed = 210;
pub const SDLK_WORLD_49: C2RustUnnamed = 209;
pub const SDLK_WORLD_48: C2RustUnnamed = 208;
pub const SDLK_WORLD_47: C2RustUnnamed = 207;
pub const SDLK_WORLD_46: C2RustUnnamed = 206;
pub const SDLK_WORLD_45: C2RustUnnamed = 205;
pub const SDLK_WORLD_44: C2RustUnnamed = 204;
pub const SDLK_WORLD_43: C2RustUnnamed = 203;
pub const SDLK_WORLD_42: C2RustUnnamed = 202;
pub const SDLK_WORLD_41: C2RustUnnamed = 201;
pub const SDLK_WORLD_40: C2RustUnnamed = 200;
pub const SDLK_WORLD_39: C2RustUnnamed = 199;
pub const SDLK_WORLD_38: C2RustUnnamed = 198;
pub const SDLK_WORLD_37: C2RustUnnamed = 197;
pub const SDLK_WORLD_36: C2RustUnnamed = 196;
pub const SDLK_WORLD_35: C2RustUnnamed = 195;
pub const SDLK_WORLD_34: C2RustUnnamed = 194;
pub const SDLK_WORLD_33: C2RustUnnamed = 193;
pub const SDLK_WORLD_32: C2RustUnnamed = 192;
pub const SDLK_WORLD_31: C2RustUnnamed = 191;
pub const SDLK_WORLD_30: C2RustUnnamed = 190;
pub const SDLK_WORLD_29: C2RustUnnamed = 189;
pub const SDLK_WORLD_28: C2RustUnnamed = 188;
pub const SDLK_WORLD_27: C2RustUnnamed = 187;
pub const SDLK_WORLD_26: C2RustUnnamed = 186;
pub const SDLK_WORLD_25: C2RustUnnamed = 185;
pub const SDLK_WORLD_24: C2RustUnnamed = 184;
pub const SDLK_WORLD_23: C2RustUnnamed = 183;
pub const SDLK_WORLD_22: C2RustUnnamed = 182;
pub const SDLK_WORLD_21: C2RustUnnamed = 181;
pub const SDLK_WORLD_20: C2RustUnnamed = 180;
pub const SDLK_WORLD_19: C2RustUnnamed = 179;
pub const SDLK_WORLD_18: C2RustUnnamed = 178;
pub const SDLK_WORLD_17: C2RustUnnamed = 177;
pub const SDLK_WORLD_16: C2RustUnnamed = 176;
pub const SDLK_WORLD_15: C2RustUnnamed = 175;
pub const SDLK_WORLD_14: C2RustUnnamed = 174;
pub const SDLK_WORLD_13: C2RustUnnamed = 173;
pub const SDLK_WORLD_12: C2RustUnnamed = 172;
pub const SDLK_WORLD_11: C2RustUnnamed = 171;
pub const SDLK_WORLD_10: C2RustUnnamed = 170;
pub const SDLK_WORLD_9: C2RustUnnamed = 169;
pub const SDLK_WORLD_8: C2RustUnnamed = 168;
pub const SDLK_WORLD_7: C2RustUnnamed = 167;
pub const SDLK_WORLD_6: C2RustUnnamed = 166;
pub const SDLK_WORLD_5: C2RustUnnamed = 165;
pub const SDLK_WORLD_4: C2RustUnnamed = 164;
pub const SDLK_WORLD_3: C2RustUnnamed = 163;
pub const SDLK_WORLD_2: C2RustUnnamed = 162;
pub const SDLK_WORLD_1: C2RustUnnamed = 161;
pub const SDLK_WORLD_0: C2RustUnnamed = 160;
pub const SDLK_DELETE: C2RustUnnamed = 127;
pub const SDLK_z: C2RustUnnamed = 122;
pub const SDLK_y: C2RustUnnamed = 121;
pub const SDLK_x: C2RustUnnamed = 120;
pub const SDLK_w: C2RustUnnamed = 119;
pub const SDLK_v: C2RustUnnamed = 118;
pub const SDLK_u: C2RustUnnamed = 117;
pub const SDLK_t: C2RustUnnamed = 116;
pub const SDLK_s: C2RustUnnamed = 115;
pub const SDLK_r: C2RustUnnamed = 114;
pub const SDLK_q: C2RustUnnamed = 113;
pub const SDLK_p: C2RustUnnamed = 112;
pub const SDLK_o: C2RustUnnamed = 111;
pub const SDLK_n: C2RustUnnamed = 110;
pub const SDLK_m: C2RustUnnamed = 109;
pub const SDLK_l: C2RustUnnamed = 108;
pub const SDLK_k: C2RustUnnamed = 107;
pub const SDLK_j: C2RustUnnamed = 106;
pub const SDLK_i: C2RustUnnamed = 105;
pub const SDLK_h: C2RustUnnamed = 104;
pub const SDLK_g: C2RustUnnamed = 103;
pub const SDLK_f: C2RustUnnamed = 102;
pub const SDLK_e: C2RustUnnamed = 101;
pub const SDLK_d: C2RustUnnamed = 100;
pub const SDLK_c: C2RustUnnamed = 99;
pub const SDLK_b: C2RustUnnamed = 98;
pub const SDLK_a: C2RustUnnamed = 97;
pub const SDLK_BACKQUOTE: C2RustUnnamed = 96;
pub const SDLK_UNDERSCORE: C2RustUnnamed = 95;
pub const SDLK_CARET: C2RustUnnamed = 94;
pub const SDLK_RIGHTBRACKET: C2RustUnnamed = 93;
pub const SDLK_BACKSLASH: C2RustUnnamed = 92;
pub const SDLK_LEFTBRACKET: C2RustUnnamed = 91;
pub const SDLK_AT: C2RustUnnamed = 64;
pub const SDLK_QUESTION: C2RustUnnamed = 63;
pub const SDLK_GREATER: C2RustUnnamed = 62;
pub const SDLK_EQUALS: C2RustUnnamed = 61;
pub const SDLK_LESS: C2RustUnnamed = 60;
pub const SDLK_SEMICOLON: C2RustUnnamed = 59;
pub const SDLK_COLON: C2RustUnnamed = 58;
pub const SDLK_9: C2RustUnnamed = 57;
pub const SDLK_8: C2RustUnnamed = 56;
pub const SDLK_7: C2RustUnnamed = 55;
pub const SDLK_6: C2RustUnnamed = 54;
pub const SDLK_5: C2RustUnnamed = 53;
pub const SDLK_4: C2RustUnnamed = 52;
pub const SDLK_3: C2RustUnnamed = 51;
pub const SDLK_2: C2RustUnnamed = 50;
pub const SDLK_1: C2RustUnnamed = 49;
pub const SDLK_0: C2RustUnnamed = 48;
pub const SDLK_SLASH: C2RustUnnamed = 47;
pub const SDLK_PERIOD: C2RustUnnamed = 46;
pub const SDLK_MINUS: C2RustUnnamed = 45;
pub const SDLK_COMMA: C2RustUnnamed = 44;
pub const SDLK_PLUS: C2RustUnnamed = 43;
pub const SDLK_ASTERISK: C2RustUnnamed = 42;
pub const SDLK_RIGHTPAREN: C2RustUnnamed = 41;
pub const SDLK_LEFTPAREN: C2RustUnnamed = 40;
pub const SDLK_QUOTE: C2RustUnnamed = 39;
pub const SDLK_AMPERSAND: C2RustUnnamed = 38;
pub const SDLK_DOLLAR: C2RustUnnamed = 36;
pub const SDLK_HASH: C2RustUnnamed = 35;
pub const SDLK_QUOTEDBL: C2RustUnnamed = 34;
pub const SDLK_EXCLAIM: C2RustUnnamed = 33;
pub const SDLK_SPACE: C2RustUnnamed = 32;
pub const SDLK_ESCAPE: C2RustUnnamed = 27;
pub const SDLK_PAUSE: C2RustUnnamed = 19;
pub const SDLK_RETURN: C2RustUnnamed = 13;
pub const SDLK_CLEAR: C2RustUnnamed = 12;
pub const SDLK_TAB: C2RustUnnamed = 9;
pub const SDLK_BACKSPACE: C2RustUnnamed = 8;
pub const SDLK_FIRST: C2RustUnnamed = 0;
pub const SDLK_UNKNOWN: C2RustUnnamed = 0;
pub type PHYSFS_uint32 = libc::c_uint;
pub type PHYSFS_sint64 = libc::c_longlong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PHYSFS_File {
    pub opaque: *mut libc::c_void,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _free_object {
    pub psNext: *mut _free_object,
}
pub type FREE_OBJECT = _free_object;
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
pub type TREAP_CMP
    =
    Option<unsafe extern "C" fn(_: UDWORD, _: UDWORD) -> SDWORD>;
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
/* Treap data structure */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _treap {
    pub cmp: TREAP_CMP,
    pub psNodes: *mut OBJ_HEAP,
    pub psRoot: *mut TREAP_NODE,
}
pub type TREAP = _treap;
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
// The next free ID
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
/* The common widget data */
/* The screen structure which stores all info for a widget screen */
pub type W_SCREEN = _w_screen;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_screen {
    pub psForm: *mut WIDGET,
    pub psFocus: *mut WIDGET,
    pub TipFontID: libc::c_int,
}
// The root form of the screen
// The widget that has keyboard focus
//	PROP_FONT	*psTipFont;			// The font for tool tips
// ID of the IVIS font to use for tool tips.
/* Upper limits for major and minor tabs on a tab form.
 * Not the best way to do it I know, but it keeps the memory
 * management MUCH simpler.
 */
//15		// Maximum number of major tabs on a tab form
//15		// Maximum number of minor tabs off a major
// Tab types passed into tab display callbacks.
pub type TAB_DISPLAY
    =
    Option<unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                _: UDWORD, _: BOOL, _: BOOL, _: UDWORD,
                                _: UDWORD, _: UDWORD, _: UDWORD) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_forminit {
    pub formID: UDWORD,
    pub majorID: UWORD,
    pub minorID: UWORD,
    pub id: UDWORD,
    pub style: UDWORD,
    pub x: SWORD,
    pub y: SWORD,
    pub width: UWORD,
    pub height: UWORD,
    pub pDisplay: WIDGET_DISPLAY,
    pub pCallback: WIDGET_CALLBACK,
    pub pUserData: *mut libc::c_void,
    pub UserData: UDWORD,
    pub disableChildren: BOOL,
    pub majorPos: UWORD,
    pub minorPos: UWORD,
    pub majorSize: UWORD,
    pub minorSize: UWORD,
    pub majorOffset: SWORD,
    pub minorOffset: SWORD,
    pub tabVertOffset: SWORD,
    pub tabHorzOffset: SWORD,
    pub tabMajorThickness: UWORD,
    pub tabMinorThickness: UWORD,
    pub tabMajorGap: UWORD,
    pub tabMinorGap: UWORD,
    pub numMajor: UWORD,
    pub aNumMinors: [UWORD; 9],
    pub pTip: *mut STRING,
    pub apMajorTips: [*mut STRING; 9],
    pub apMinorTips: [[*mut STRING; 5]; 9],
    pub pTabDisplay: TAB_DISPLAY,
    pub pFormDisplay: WIDGET_DISPLAY,
}
/* Form initialisation structure */
pub type W_FORMINIT = _w_forminit;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_butinit {
    pub formID: UDWORD,
    pub majorID: UWORD,
    pub minorID: UWORD,
    pub id: UDWORD,
    pub style: UDWORD,
    pub x: SWORD,
    pub y: SWORD,
    pub width: UWORD,
    pub height: UWORD,
    pub pDisplay: WIDGET_DISPLAY,
    pub pCallback: WIDGET_CALLBACK,
    pub pUserData: *mut libc::c_void,
    pub UserData: UDWORD,
    pub pText: *mut STRING,
    pub pTip: *mut STRING,
    pub FontID: libc::c_int,
}
/* The basic init entries */
/* Data for a tabbed form */
// Position of the tabs on the form
// Size of the tabs (in pixels)
// Tab start offset.
// Tab form overlap offset.
// Tab form overlap offset.
// The thickness of the tabs
// The thickness of the tabs
// The space between tabs
// The space between tabs
// Number of major tabs
// Number of minor tabs for each major
// Tool tip for the form itself
// Tool tips for the major tabs
// Tool tips for the minor tabs
// Optional callback for displaying a tab.
// Optional callback to display the form.
/* Button initialisation structure */
pub type W_BUTINIT = _w_butinit;
pub type _title_mode = libc::c_uint;
pub const GAME3: _title_mode = 18;
pub const GAME2: _title_mode = 17;
pub const KEYMAP: _title_mode = 16;
pub const LOADSAVEGAME: _title_mode = 15;
pub const QUIT: _title_mode = 14;
pub const SHOWINTRO: _title_mode = 13;
pub const STARTGAME: _title_mode = 12;
pub const MULTILIMIT: _title_mode = 11;
pub const GAMEFIND: _title_mode = 10;
pub const FORCESELECT: _title_mode = 9;
pub const MULTIOPTION: _title_mode = 8;
pub const PROTOCOL: _title_mode = 7;
pub const CREDITS: _title_mode = 6;
pub const TUTORIAL: _title_mode = 5;
pub const GAME: _title_mode = 4;
pub const OPTIONS: _title_mode = 3;
pub const MULTI: _title_mode = 2;
pub const SINGLE: _title_mode = 1;
pub const TITLE: _title_mode = 0;
/* The basic init entries */
// button text
// Tool tip text
//	PROP_FONT	*psFont;		// button font
// ID of the IVIS font to use for this widget.
/* 
 * Frontend.h
 */
// determines which option screen to use. when in GS_TITLE_SCREEN mode.
pub type tMode = _title_mode;
// 0 intro mode
// 1 single player menu
// 2 multiplayer menu
// 3 options menu	
// 4
// 5  tutorial/fastplay	
// 6  credits
// 7  MULTIPLAYER, select proto
// 8 MULTIPLAYER, select game options
// 9 MULTIPLAYER, Force design screen
// 10 MULTIPLAYER, gamefinder.
// 11 MULTIPLAYER, Limit the multistuff.
// 12 Fire up the game
// 13 reshow the intro
// 14 leaving game
// 15 loading a save game
// 16 keymap editor
// 17 second options menu.
// 18 third options menu.
//		GRAPHICS,					// 5
//		VIDEO,
//	DEMOMODE,					// demo mode. remove for release?
/* **********************************************/
/* Image ID definition file created by Framer. */
/* **********************************************/
pub type C2RustUnnamed_0 = libc::c_uint;
pub const IMAGE_NOJOIN: C2RustUnnamed_0 = 281;
pub const IMAGE_TFONT_45: C2RustUnnamed_0 = 280;
pub const IMAGE_TFONT_63: C2RustUnnamed_0 = 279;
pub const IMAGE_TFONT_205: C2RustUnnamed_0 = 278;
pub const IMAGE_TFONT_204: C2RustUnnamed_0 = 277;
pub const IMAGE_TFONT_207: C2RustUnnamed_0 = 276;
pub const IMAGE_TFONT_206: C2RustUnnamed_0 = 275;
pub const IMAGE_TFONT_211: C2RustUnnamed_0 = 274;
pub const IMAGE_TFONT_202: C2RustUnnamed_0 = 273;
pub const IMAGE_TFONT_170: C2RustUnnamed_0 = 272;
pub const IMAGE_TFONT_163: C2RustUnnamed_0 = 271;
pub const IMAGE_TFONT_223: C2RustUnnamed_0 = 270;
pub const IMAGE_MULTIRANK3: C2RustUnnamed_0 = 269;
pub const IMAGE_KEYMAP_DEFAULT: C2RustUnnamed_0 = 268;
pub const IMAGE_NOPENCIL: C2RustUnnamed_0 = 267;
pub const IMAGE_PENCIL: C2RustUnnamed_0 = 266;
pub const IMAGE_TFONT_245: C2RustUnnamed_0 = 265;
pub const IMAGE_TFONT_221: C2RustUnnamed_0 = 264;
pub const IMAGE_TFONT_217: C2RustUnnamed_0 = 263;
pub const IMAGE_TFONT_219: C2RustUnnamed_0 = 262;
pub const IMAGE_TFONT_218: C2RustUnnamed_0 = 261;
pub const IMAGE_TFONT_213: C2RustUnnamed_0 = 260;
pub const IMAGE_TFONT_210: C2RustUnnamed_0 = 259;
pub const IMAGE_TFONT_212: C2RustUnnamed_0 = 258;
pub const IMAGE_TFONT_200: C2RustUnnamed_0 = 257;
pub const IMAGE_TFONT_203: C2RustUnnamed_0 = 256;
pub const IMAGE_TFONT_208: C2RustUnnamed_0 = 255;
pub const IMAGE_TFONT_240: C2RustUnnamed_0 = 254;
pub const IMAGE_TFONT_195: C2RustUnnamed_0 = 253;
pub const IMAGE_TFONT_192: C2RustUnnamed_0 = 252;
pub const IMAGE_TFONT_194: C2RustUnnamed_0 = 251;
pub const IMAGE_TFONT_193: C2RustUnnamed_0 = 250;
pub const IMAGE_TFONT_187: C2RustUnnamed_0 = 249;
pub const IMAGE_TFONT_171: C2RustUnnamed_0 = 248;
pub const IMAGE_TFONT_172: C2RustUnnamed_0 = 247;
pub const IMAGE_TFONT_174: C2RustUnnamed_0 = 246;
pub const IMAGE_TFONT_161: C2RustUnnamed_0 = 245;
pub const IMAGE_TFONT_191: C2RustUnnamed_0 = 244;
pub const IMAGE_TFONT_176: C2RustUnnamed_0 = 243;
pub const IMAGE_TFONT_131: C2RustUnnamed_0 = 242;
pub const IMAGE_TFONT_215: C2RustUnnamed_0 = 241;
pub const IMAGE_TFONT_216: C2RustUnnamed_0 = 240;
pub const IMAGE_TFONT_220: C2RustUnnamed_0 = 239;
pub const IMAGE_TFONT_214: C2RustUnnamed_0 = 238;
pub const IMAGE_TFONT_201: C2RustUnnamed_0 = 237;
pub const IMAGE_TFONT_197: C2RustUnnamed_0 = 236;
pub const IMAGE_TFONT_196: C2RustUnnamed_0 = 235;
pub const IMAGE_TFONT_198: C2RustUnnamed_0 = 234;
pub const IMAGE_TFONT_128: C2RustUnnamed_0 = 233;
pub const IMAGE_TFONT_253: C2RustUnnamed_0 = 232;
pub const IMAGE_TFONT_252: C2RustUnnamed_0 = 231;
pub const IMAGE_TFONT_251: C2RustUnnamed_0 = 230;
pub const IMAGE_TFONT_250: C2RustUnnamed_0 = 229;
pub const IMAGE_TFONT_249: C2RustUnnamed_0 = 228;
pub const IMAGE_TFONT_248: C2RustUnnamed_0 = 227;
pub const IMAGE_TFONT_246: C2RustUnnamed_0 = 226;
pub const IMAGE_TFONT_244: C2RustUnnamed_0 = 225;
pub const IMAGE_TFONT_243: C2RustUnnamed_0 = 224;
pub const IMAGE_TFONT_242: C2RustUnnamed_0 = 223;
pub const IMAGE_TFONT_209: C2RustUnnamed_0 = 222;
pub const IMAGE_TFONT_241: C2RustUnnamed_0 = 221;
pub const IMAGE_TFONT_239: C2RustUnnamed_0 = 220;
pub const IMAGE_TFONT_238: C2RustUnnamed_0 = 219;
pub const IMAGE_TFONT_237: C2RustUnnamed_0 = 218;
pub const IMAGE_TFONT_236: C2RustUnnamed_0 = 217;
pub const IMAGE_TFONT_235: C2RustUnnamed_0 = 216;
pub const IMAGE_TFONT_234: C2RustUnnamed_0 = 215;
pub const IMAGE_TFONT_233: C2RustUnnamed_0 = 214;
pub const IMAGE_TFONT_232: C2RustUnnamed_0 = 213;
pub const IMAGE_TFONT_199: C2RustUnnamed_0 = 212;
pub const IMAGE_TFONT_230: C2RustUnnamed_0 = 211;
pub const IMAGE_TFONT_229: C2RustUnnamed_0 = 210;
pub const IMAGE_TFONT_228: C2RustUnnamed_0 = 209;
pub const IMAGE_TFONT_227: C2RustUnnamed_0 = 208;
pub const IMAGE_TFONT_226: C2RustUnnamed_0 = 207;
pub const IMAGE_TFONT_188: C2RustUnnamed_0 = 206;
pub const IMAGE_TFONT_224: C2RustUnnamed_0 = 205;
pub const IMAGE_TFONT_225: C2RustUnnamed_0 = 204;
pub const IMAGE_TFONT_189: C2RustUnnamed_0 = 203;
pub const IMAGE_WEE_GUY: C2RustUnnamed_0 = 202;
pub const IMAGE_FOG_ON_HI: C2RustUnnamed_0 = 201;
pub const IMAGE_FOG_OFF_HI: C2RustUnnamed_0 = 200;
pub const IMAGE_FOG_ON: C2RustUnnamed_0 = 199;
pub const IMAGE_FOG_OFF: C2RustUnnamed_0 = 198;
pub const IMAGE_PLAYERX: C2RustUnnamed_0 = 197;
pub const IMAGE_MEDAL_DUMMY: C2RustUnnamed_0 = 196;
pub const IMAGE_MULTIRANK2: C2RustUnnamed_0 = 195;
pub const IMAGE_PLAYER_PC: C2RustUnnamed_0 = 194;
pub const IMAGE_TEAM_HI: C2RustUnnamed_0 = 193;
pub const IMAGE_SKIRMISH_HI: C2RustUnnamed_0 = 192;
pub const IMAGE_TEAM: C2RustUnnamed_0 = 191;
pub const IMAGE_SKIRMISH: C2RustUnnamed_0 = 190;
pub const IMAGE_TEAM_OVER: C2RustUnnamed_0 = 189;
pub const IMAGE_SKIRMISH_OVER: C2RustUnnamed_0 = 188;
pub const IMAGE_LAMP_GREEN: C2RustUnnamed_0 = 187;
pub const IMAGE_LAMP_AMBER: C2RustUnnamed_0 = 186;
pub const IMAGE_LAMP_RED: C2RustUnnamed_0 = 185;
pub const IMAGE_COMPUTER_Y_HI: C2RustUnnamed_0 = 184;
pub const IMAGE_COMPUTER_Y: C2RustUnnamed_0 = 183;
pub const IMAGE_COMPUTER_N: C2RustUnnamed_0 = 182;
pub const IMAGE_COMPUTER_N_HI: C2RustUnnamed_0 = 181;
pub const IMAGE_HI56: C2RustUnnamed_0 = 180;
pub const IMAGE_DEFAULTFORCE: C2RustUnnamed_0 = 179;
pub const IMAGE_CLEARFORCE: C2RustUnnamed_0 = 178;
pub const IMAGE_SAVEFORCE: C2RustUnnamed_0 = 177;
pub const IMAGE_LOADFORCE: C2RustUnnamed_0 = 176;
pub const IMAGE_SLIM_HI: C2RustUnnamed_0 = 175;
pub const IMAGE_SLIM: C2RustUnnamed_0 = 174;
pub const IMAGE_RETURN_HI: C2RustUnnamed_0 = 173;
pub const IMAGE_FRAGLIMIT_HI: C2RustUnnamed_0 = 172;
pub const IMAGE_TIMELIMIT_HI: C2RustUnnamed_0 = 171;
pub const IMAGE_NOLIMIT_HI: C2RustUnnamed_0 = 170;
pub const IMAGE_FRAGLIMIT: C2RustUnnamed_0 = 169;
pub const IMAGE_TIMELIMIT: C2RustUnnamed_0 = 168;
pub const IMAGE_NOLIMIT: C2RustUnnamed_0 = 167;
pub const IMAGE_LBASE_HI: C2RustUnnamed_0 = 166;
pub const IMAGE_SBASE_HI: C2RustUnnamed_0 = 165;
pub const IMAGE_NOBASE_HI: C2RustUnnamed_0 = 164;
pub const IMAGE_LBASE: C2RustUnnamed_0 = 163;
pub const IMAGE_SBASE: C2RustUnnamed_0 = 162;
pub const IMAGE_NOBASE: C2RustUnnamed_0 = 161;
pub const IMAGE_TECHHI_HI: C2RustUnnamed_0 = 160;
pub const IMAGE_TECHMED_HI: C2RustUnnamed_0 = 159;
pub const IMAGE_TECHLO_HI: C2RustUnnamed_0 = 158;
pub const IMAGE_TECHHI: C2RustUnnamed_0 = 157;
pub const IMAGE_TECHMED: C2RustUnnamed_0 = 156;
pub const IMAGE_TECHLO: C2RustUnnamed_0 = 155;
pub const IMAGE_BIGOK: C2RustUnnamed_0 = 154;
pub const IMAGE_EDIT_GAME: C2RustUnnamed_0 = 153;
pub const IMAGE_EDIT_MAP: C2RustUnnamed_0 = 152;
pub const IMAGE_EDIT_FORCE: C2RustUnnamed_0 = 151;
pub const IMAGE_EDIT_PLAYER: C2RustUnnamed_0 = 150;
pub const IMAGE_RETURN: C2RustUnnamed_0 = 149;
pub const IMAGE_MULTIRANK1: C2RustUnnamed_0 = 148;
pub const IMAGE_POWLO: C2RustUnnamed_0 = 147;
pub const IMAGE_MEDAL_BRONZE: C2RustUnnamed_0 = 146;
pub const IMAGE_MEDAL_SILVER: C2RustUnnamed_0 = 145;
pub const IMAGE_MEDAL_GOLD: C2RustUnnamed_0 = 144;
pub const IMAGE_CAMPAIGN_OVER: C2RustUnnamed_0 = 143;
pub const IMAGE_ARENA_OVER: C2RustUnnamed_0 = 142;
pub const IMAGE_HI64: C2RustUnnamed_0 = 141;
pub const IMAGE_HI41: C2RustUnnamed_0 = 140;
pub const IMAGE_HI39: C2RustUnnamed_0 = 139;
pub const IMAGE_HI23: C2RustUnnamed_0 = 138;
pub const IMAGE_HI31: C2RustUnnamed_0 = 137;
pub const IMAGE_HI34: C2RustUnnamed_0 = 136;
pub const IMAGE_COM4_HI: C2RustUnnamed_0 = 135;
pub const IMAGE_COM3_HI: C2RustUnnamed_0 = 134;
pub const IMAGE_ALLI_HI: C2RustUnnamed_0 = 133;
pub const IMAGE_OFFALLI_HI: C2RustUnnamed_0 = 132;
pub const IMAGE_NOALLI_HI: C2RustUnnamed_0 = 131;
pub const IMAGE_ALLI: C2RustUnnamed_0 = 130;
pub const IMAGE_OFFALLI: C2RustUnnamed_0 = 129;
pub const IMAGE_NOALLI: C2RustUnnamed_0 = 128;
pub const IMAGE_POWHI_HI: C2RustUnnamed_0 = 127;
pub const IMAGE_POWMED_HI: C2RustUnnamed_0 = 126;
pub const IMAGE_POWLO_HI: C2RustUnnamed_0 = 125;
pub const IMAGE_POWHI: C2RustUnnamed_0 = 124;
pub const IMAGE_POWMED: C2RustUnnamed_0 = 123;
pub const IMAGE_OK: C2RustUnnamed_0 = 122;
pub const IMAGE_NO: C2RustUnnamed_0 = 121;
pub const IMAGE_HOST: C2RustUnnamed_0 = 120;
pub const IMAGE_PLAYER7: C2RustUnnamed_0 = 119;
pub const IMAGE_PLAYER6: C2RustUnnamed_0 = 118;
pub const IMAGE_PLAYER5: C2RustUnnamed_0 = 117;
pub const IMAGE_PLAYER4: C2RustUnnamed_0 = 116;
pub const IMAGE_PLAYER3: C2RustUnnamed_0 = 115;
pub const IMAGE_PLAYER2: C2RustUnnamed_0 = 114;
pub const IMAGE_PLAYER1: C2RustUnnamed_0 = 113;
pub const IMAGE_PLAYER0: C2RustUnnamed_0 = 112;
pub const IMAGE_REFRESH: C2RustUnnamed_0 = 111;
pub const IMAGE_CAMPAIGN: C2RustUnnamed_0 = 110;
pub const IMAGE_ARENA: C2RustUnnamed_0 = 109;
pub const IMAGE_115200: C2RustUnnamed_0 = 108;
pub const IMAGE_56000: C2RustUnnamed_0 = 107;
pub const IMAGE_19200: C2RustUnnamed_0 = 106;
pub const IMAGE_14400: C2RustUnnamed_0 = 105;
pub const IMAGE_CAMPAIGN_HI: C2RustUnnamed_0 = 104;
pub const IMAGE_ARENA_HI: C2RustUnnamed_0 = 103;
pub const IMAGE_115200_HI: C2RustUnnamed_0 = 102;
pub const IMAGE_56000_HI: C2RustUnnamed_0 = 101;
pub const IMAGE_19200_HI: C2RustUnnamed_0 = 100;
pub const IMAGE_14400_HI: C2RustUnnamed_0 = 99;
pub const IMAGE_COM2_HI: C2RustUnnamed_0 = 98;
pub const IMAGE_COM1_HI: C2RustUnnamed_0 = 97;
pub const IMAGE_COM4: C2RustUnnamed_0 = 96;
pub const IMAGE_COM3: C2RustUnnamed_0 = 95;
pub const IMAGE_COM2: C2RustUnnamed_0 = 94;
pub const IMAGE_COM1: C2RustUnnamed_0 = 93;
pub const IMAGE_TFONT_126: C2RustUnnamed_0 = 92;
pub const IMAGE_TFONT_125: C2RustUnnamed_0 = 91;
pub const IMAGE_TFONT_124: C2RustUnnamed_0 = 90;
pub const IMAGE_TFONT_123: C2RustUnnamed_0 = 89;
pub const IMAGE_TFONT_122: C2RustUnnamed_0 = 88;
pub const IMAGE_TFONT_121: C2RustUnnamed_0 = 87;
pub const IMAGE_TFONT_120: C2RustUnnamed_0 = 86;
pub const IMAGE_TFONT_119: C2RustUnnamed_0 = 85;
pub const IMAGE_TFONT_118: C2RustUnnamed_0 = 84;
pub const IMAGE_TFONT_117: C2RustUnnamed_0 = 83;
pub const IMAGE_TFONT_116: C2RustUnnamed_0 = 82;
pub const IMAGE_TFONT_115: C2RustUnnamed_0 = 81;
pub const IMAGE_TFONT_114: C2RustUnnamed_0 = 80;
pub const IMAGE_TFONT_113: C2RustUnnamed_0 = 79;
pub const IMAGE_TFONT_112: C2RustUnnamed_0 = 78;
pub const IMAGE_TFONT_111: C2RustUnnamed_0 = 77;
pub const IMAGE_TFONT_110: C2RustUnnamed_0 = 76;
pub const IMAGE_TFONT_109: C2RustUnnamed_0 = 75;
pub const IMAGE_TFONT_108: C2RustUnnamed_0 = 74;
pub const IMAGE_TFONT_107: C2RustUnnamed_0 = 73;
pub const IMAGE_TFONT_106: C2RustUnnamed_0 = 72;
pub const IMAGE_TFONT_105: C2RustUnnamed_0 = 71;
pub const IMAGE_TFONT_104: C2RustUnnamed_0 = 70;
pub const IMAGE_TFONT_103: C2RustUnnamed_0 = 69;
pub const IMAGE_TFONT_102: C2RustUnnamed_0 = 68;
pub const IMAGE_TFONT_101: C2RustUnnamed_0 = 67;
pub const IMAGE_TFONT_100: C2RustUnnamed_0 = 66;
pub const IMAGE_TFONT_99: C2RustUnnamed_0 = 65;
pub const IMAGE_TFONT_98: C2RustUnnamed_0 = 64;
pub const IMAGE_TFONT_97: C2RustUnnamed_0 = 63;
pub const IMAGE_TFONT_96: C2RustUnnamed_0 = 62;
pub const IMAGE_TFONT_95: C2RustUnnamed_0 = 61;
pub const IMAGE_TFONT_94: C2RustUnnamed_0 = 60;
pub const IMAGE_TFONT_93: C2RustUnnamed_0 = 59;
pub const IMAGE_TFONT_92: C2RustUnnamed_0 = 58;
pub const IMAGE_TFONT_91: C2RustUnnamed_0 = 57;
pub const IMAGE_TFONT_90: C2RustUnnamed_0 = 56;
pub const IMAGE_TFONT_89: C2RustUnnamed_0 = 55;
pub const IMAGE_TFONT_88: C2RustUnnamed_0 = 54;
pub const IMAGE_TFONT_87: C2RustUnnamed_0 = 53;
pub const IMAGE_TFONT_86: C2RustUnnamed_0 = 52;
pub const IMAGE_TFONT_85: C2RustUnnamed_0 = 51;
pub const IMAGE_TFONT_84: C2RustUnnamed_0 = 50;
pub const IMAGE_TFONT_83: C2RustUnnamed_0 = 49;
pub const IMAGE_TFONT_82: C2RustUnnamed_0 = 48;
pub const IMAGE_TFONT_81: C2RustUnnamed_0 = 47;
pub const IMAGE_TFONT_80: C2RustUnnamed_0 = 46;
pub const IMAGE_TFONT_79: C2RustUnnamed_0 = 45;
pub const IMAGE_TFONT_78: C2RustUnnamed_0 = 44;
pub const IMAGE_TFONT_77: C2RustUnnamed_0 = 43;
pub const IMAGE_TFONT_76: C2RustUnnamed_0 = 42;
pub const IMAGE_TFONT_75: C2RustUnnamed_0 = 41;
pub const IMAGE_TFONT_74: C2RustUnnamed_0 = 40;
pub const IMAGE_TFONT_73: C2RustUnnamed_0 = 39;
pub const IMAGE_TFONT_72: C2RustUnnamed_0 = 38;
pub const IMAGE_TFONT_71: C2RustUnnamed_0 = 37;
pub const IMAGE_TFONT_70: C2RustUnnamed_0 = 36;
pub const IMAGE_TFONT_69: C2RustUnnamed_0 = 35;
pub const IMAGE_TFONT_68: C2RustUnnamed_0 = 34;
pub const IMAGE_TFONT_67: C2RustUnnamed_0 = 33;
pub const IMAGE_TFONT_66: C2RustUnnamed_0 = 32;
pub const IMAGE_TFONT_65: C2RustUnnamed_0 = 31;
pub const IMAGE_TFONT_64: C2RustUnnamed_0 = 30;
pub const IMAGE_TFONT_62: C2RustUnnamed_0 = 29;
pub const IMAGE_TFONT_61: C2RustUnnamed_0 = 28;
pub const IMAGE_TFONT_60: C2RustUnnamed_0 = 27;
pub const IMAGE_TFONT_59: C2RustUnnamed_0 = 26;
pub const IMAGE_TFONT_58: C2RustUnnamed_0 = 25;
pub const IMAGE_TFONT_57: C2RustUnnamed_0 = 24;
pub const IMAGE_TFONT_56: C2RustUnnamed_0 = 23;
pub const IMAGE_TFONT_55: C2RustUnnamed_0 = 22;
pub const IMAGE_TFONT_54: C2RustUnnamed_0 = 21;
pub const IMAGE_TFONT_53: C2RustUnnamed_0 = 20;
pub const IMAGE_TFONT_52: C2RustUnnamed_0 = 19;
pub const IMAGE_TFONT_51: C2RustUnnamed_0 = 18;
pub const IMAGE_TFONT_50: C2RustUnnamed_0 = 17;
pub const IMAGE_TFONT_49: C2RustUnnamed_0 = 16;
pub const IMAGE_TFONT_48: C2RustUnnamed_0 = 15;
pub const IMAGE_TFONT_47: C2RustUnnamed_0 = 14;
pub const IMAGE_TFONT_46: C2RustUnnamed_0 = 13;
pub const IMAGE_TFONT_44: C2RustUnnamed_0 = 12;
pub const IMAGE_TFONT_43: C2RustUnnamed_0 = 11;
pub const IMAGE_TFONT_42: C2RustUnnamed_0 = 10;
pub const IMAGE_TFONT_41: C2RustUnnamed_0 = 9;
pub const IMAGE_TFONT_40: C2RustUnnamed_0 = 8;
pub const IMAGE_TFONT_39: C2RustUnnamed_0 = 7;
pub const IMAGE_TFONT_38: C2RustUnnamed_0 = 6;
pub const IMAGE_TFONT_37: C2RustUnnamed_0 = 5;
pub const IMAGE_TFONT_36: C2RustUnnamed_0 = 4;
pub const IMAGE_TFONT_35: C2RustUnnamed_0 = 3;
pub const IMAGE_TFONT_34: C2RustUnnamed_0 = 2;
pub const IMAGE_TFONT_33: C2RustUnnamed_0 = 1;
pub const IMAGE_FE_LOGO: C2RustUnnamed_0 = 0;
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
pub type KEY_ACTION = libc::c_uint;
pub const KEYMAP_RELEASED: KEY_ACTION = 2;
pub const KEYMAP_PRESSED: KEY_ACTION = 1;
pub const KEYMAP_DOWN: KEY_ACTION = 0;
pub type KEY_STATUS = libc::c_uint;
pub const KEYMAP___HIDE: KEY_STATUS = 4;
pub const KEYMAP_ALWAYS_PROCESS: KEY_STATUS = 3;
pub const KEYMAP_ASSIGNABLE: KEY_STATUS = 2;
pub const KEYMAP_ALWAYS: KEY_STATUS = 1;
pub const KEYMAP__DEBUG: KEY_STATUS = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _keyMapping {
    pub function: Option<unsafe extern "C" fn() -> ()>,
    pub active: BOOL,
    pub status: KEY_STATUS,
    pub lastCalled: UDWORD,
    pub metaKeyCode: KEY_CODE,
    pub altMetaKeyCode: KEY_CODE,
    pub subKeyCode: KEY_CODE,
    pub action: KEY_ACTION,
    pub pName: *mut STRING,
    pub psNext: *mut _keyMapping,
}
pub type KEY_MAPPING = _keyMapping;
// for keymap editor.
pub type _keymapsave = Option<unsafe extern "C" fn() -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SNAPBIAS {
    pub NDxBias: SWORD,
    pub NDyBias: SWORD,
    pub UDxBias: SWORD,
    pub UDyBias: SWORD,
    pub RDxBias: SWORD,
    pub RDyBias: SWORD,
    pub DDxBias: SWORD,
    pub DDyBias: SWORD,
    pub LDxBias: SWORD,
    pub LDyBias: SWORD,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SNAPCOORD {
    pub FormID: UDWORD,
    pub ID: UDWORD,
    pub SnapX: SWORD,
    pub SnapY: SWORD,
    pub Bias: *mut SNAPBIAS,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CURSORSNAP {
    pub MaxSnaps: UWORD,
    pub NumSnaps: SWORD,
    pub CurrentSnap: SWORD,
    pub NewCurrentFormID: UDWORD,
    pub NewCurrentID: UDWORD,
    pub SnapCoords: *mut SNAPCOORD,
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
pub type C2RustUnnamed_1 = libc::c_int;
pub const ID_MAX_SOUND: C2RustUnnamed_1 = 354;
pub const ID_SOUND_LAS_SAT_COUNTDOWN: C2RustUnnamed_1 = 353;
pub const ID_SOUND_UPLINK: C2RustUnnamed_1 = 352;
pub const ID_SOUND_PLASMA_FLAMER: C2RustUnnamed_1 = 351;
pub const ID_SOUND_LASER_HEAVY: C2RustUnnamed_1 = 350;
pub const ID_SOUND_EMP: C2RustUnnamed_1 = 349;
pub const ID_SOUND_CYBORG_HEAVY: C2RustUnnamed_1 = 348;
pub const ID_SOUND_CYBORG_GROUND: C2RustUnnamed_1 = 347;
pub const ID_SOUND_NEXUS_UNIT_NEUTRALISED: C2RustUnnamed_1 = 346;
pub const ID_SOUND_NEXUS_UNIT_ABSORBED: C2RustUnnamed_1 = 345;
pub const ID_SOUND_NEXUS_SYNAPTIC_LINK: C2RustUnnamed_1 = 344;
pub const ID_SOUND_NEXUS_STRUCTURE_NEUTRALISED: C2RustUnnamed_1 = 343;
pub const ID_SOUND_NEXUS_STRUCTURE_ABSORBED: C2RustUnnamed_1 = 342;
pub const ID_SOUND_NEXUS_RESEARCH_ABSORBED: C2RustUnnamed_1 = 341;
pub const ID_SOUND_NEXUS_PRODUCTION_COMPLETED: C2RustUnnamed_1 = 340;
pub const ID_SOUND_NEXUS_LAUGH3: C2RustUnnamed_1 = 339;
pub const ID_SOUND_NEXUS_LAUGH2: C2RustUnnamed_1 = 338;
pub const ID_SOUND_NEXUS_LAUGH1: C2RustUnnamed_1 = 337;
pub const ID_SOUND_NEXUS_DEFENCES_NEUTRALISED: C2RustUnnamed_1 = 336;
pub const ID_SOUND_NEXUS_DEFENCES_ABSORBED: C2RustUnnamed_1 = 335;
pub const ID_SOUND_INCOMING_LASER_SAT_STRIKE: C2RustUnnamed_1 = 334;
pub const ID_SOUND_LASER_SATELLITE_FIRING: C2RustUnnamed_1 = 333;
pub const ID_SOUND_TEAM_GAMMA_RESCUED: C2RustUnnamed_1 = 332;
pub const ID_SOUND_TEAM_BETA_RESCUED: C2RustUnnamed_1 = 331;
pub const ID_SOUND_TEAM_ALPHA_RESCUED: C2RustUnnamed_1 = 330;
pub const ID_SOUND_TEAM_GAMMA_ERADICATED: C2RustUnnamed_1 = 329;
pub const ID_SOUND_TEAM_BETA_ERADICATED: C2RustUnnamed_1 = 328;
pub const ID_SOUND_TEAM_ALPHA_ERADICATED: C2RustUnnamed_1 = 327;
pub const ID_SOUND_ENEMY_TRANSPORT_LANDING: C2RustUnnamed_1 = 326;
pub const ID_SOUND_ENEMY_ESCAPING: C2RustUnnamed_1 = 325;
pub const ID_SOUND_ENEMY_ESCAPED: C2RustUnnamed_1 = 324;
pub const ID_SOUND_OUT_OF_TIME: C2RustUnnamed_1 = 323;
pub const ID_SOUND_GROUP_INFECTED: C2RustUnnamed_1 = 322;
pub const ID_SOUND_STRUCTURE_INFECTED: C2RustUnnamed_1 = 321;
pub const ID_SOUND_OBJECTIVE_DESTROYED: C2RustUnnamed_1 = 320;
pub const ID_SOUND_OBJECTIVE_CAPTURED: C2RustUnnamed_1 = 319;
pub const ID_SOUND_GROUP_CAPTURED: C2RustUnnamed_1 = 318;
pub const ID_SOUND_GROUP_RESCUED: C2RustUnnamed_1 = 317;
pub const ID_SOUND_UNITS_RESCUED: C2RustUnnamed_1 = 316;
pub const ID_SOUND_CIVILIANS_RESCUED: C2RustUnnamed_1 = 315;
pub const ID_SOUND_CIVILIAN_RESCUED: C2RustUnnamed_1 = 314;
pub const ID_SOUND_STRUCTURE_CAPTURED: C2RustUnnamed_1 = 313;
pub const ID_SOUND_MISSILE_NME_DETECTED: C2RustUnnamed_1 = 312;
pub const ID_SOUND_MISSILE_SILO: C2RustUnnamed_1 = 311;
pub const ID_SOUND_SAM_SITE: C2RustUnnamed_1 = 310;
pub const ID_SOUND_NUCLEAR_REACTOR: C2RustUnnamed_1 = 309;
pub const ID_SOUND_NASDA_CENTRAL: C2RustUnnamed_1 = 308;
pub const ID_SOUND_SATELLITE_UPLINK: C2RustUnnamed_1 = 307;
pub const ID_SOUND_LANDING_ZONE: C2RustUnnamed_1 = 306;
pub const ID_SOUND_NOFAULTS: C2RustUnnamed_1 = 305;
pub const ID_SOUND_OF_SILENCE: C2RustUnnamed_1 = 304;
pub const ID_SOUND_BARB_SCREAM3: C2RustUnnamed_1 = 303;
pub const ID_SOUND_BARB_SCREAM2: C2RustUnnamed_1 = 302;
pub const ID_SOUND_BARB_SCREAM: C2RustUnnamed_1 = 301;
pub const ID_SOUND_HELP: C2RustUnnamed_1 = 300;
pub const ID_SOUND_FIRE_ROAR: C2RustUnnamed_1 = 299;
pub const ID_SOUND_ECM_TOWER: C2RustUnnamed_1 = 298;
pub const ID_SOUND_STEAM: C2RustUnnamed_1 = 297;
pub const ID_SOUND_POWER_SPARK: C2RustUnnamed_1 = 296;
pub const ID_SOUND_POWER_HUM: C2RustUnnamed_1 = 295;
pub const ID_SOUND_OIL_PUMP_2: C2RustUnnamed_1 = 294;
pub const ID_SOUND_CYBORG_MOVE: C2RustUnnamed_1 = 293;
pub const ID_SOUND_HOVER_STOP: C2RustUnnamed_1 = 292;
pub const ID_SOUND_HOVER_START: C2RustUnnamed_1 = 291;
pub const ID_SOUND_HOVER_MOVE: C2RustUnnamed_1 = 290;
pub const ID_SOUND_TREAD: C2RustUnnamed_1 = 289;
pub const ID_SOUND_VTOL_MOVE: C2RustUnnamed_1 = 288;
pub const ID_SOUND_VTOL_OFF: C2RustUnnamed_1 = 287;
pub const ID_SOUND_VTOL_LAND: C2RustUnnamed_1 = 286;
pub const ID_SOUND_BLIMP_TAKE_OFF: C2RustUnnamed_1 = 285;
pub const ID_SOUND_BLIMP_LAND: C2RustUnnamed_1 = 284;
pub const ID_SOUND_BLIMP_IDLE: C2RustUnnamed_1 = 283;
pub const ID_SOUND_BLIMP_FLIGHT: C2RustUnnamed_1 = 282;
pub const ID_SOUND_CONSTRUCTOR_SHUTDOWN: C2RustUnnamed_1 = 281;
pub const ID_SOUND_CONSTRUCTOR_MOVE: C2RustUnnamed_1 = 280;
pub const ID_SOUND_CONSTRUCTOR_MOVEOFF: C2RustUnnamed_1 = 279;
pub const ID_SOUND_NEXUS_EXPLOSION: C2RustUnnamed_1 = 278;
pub const ID_SOUND_BUILDING_FALL: C2RustUnnamed_1 = 277;
pub const ID_SOUND_BARB_SQUISH: C2RustUnnamed_1 = 276;
pub const ID_SOUND_RICOCHET_3: C2RustUnnamed_1 = 275;
pub const ID_SOUND_RICOCHET_2: C2RustUnnamed_1 = 274;
pub const ID_SOUND_RICOCHET_1: C2RustUnnamed_1 = 273;
pub const ID_SOUND_EXPLOSION_ANTITANK: C2RustUnnamed_1 = 272;
pub const ID_SOUND_EXPLOSION: C2RustUnnamed_1 = 271;
pub const ID_SOUND_EXPLOSION_LASER: C2RustUnnamed_1 = 270;
pub const ID_SOUND_EXPLOSION_SMALL: C2RustUnnamed_1 = 269;
pub const ID_SOUND_CONSTRUCTION_4: C2RustUnnamed_1 = 268;
pub const ID_SOUND_CONSTRUCTION_3: C2RustUnnamed_1 = 267;
pub const ID_SOUND_CONSTRUCTION_2: C2RustUnnamed_1 = 266;
pub const ID_SOUND_CONSTRUCTION_1: C2RustUnnamed_1 = 265;
pub const ID_SOUND_CONSTRUCTION_LOOP: C2RustUnnamed_1 = 264;
pub const ID_SOUND_CONSTRUCTION_START: C2RustUnnamed_1 = 263;
pub const ID_SOUND_WELD_2: C2RustUnnamed_1 = 262;
pub const ID_SOUND_WELD_1: C2RustUnnamed_1 = 261;
pub const ID_SOUND_NEXUS_TOWER: C2RustUnnamed_1 = 260;
pub const ID_SOUND_HIVEL_CANNON: C2RustUnnamed_1 = 259;
pub const ID_SOUND_RAPID_CANNON: C2RustUnnamed_1 = 258;
pub const ID_SOUND_ASSAULT_MG: C2RustUnnamed_1 = 257;
pub const ID_SOUND_SPLASH: C2RustUnnamed_1 = 256;
pub const ID_SOUND_BABA_MG_TOWER: C2RustUnnamed_1 = 255;
pub const ID_SOUND_BABA_MG_HEAVY: C2RustUnnamed_1 = 254;
pub const ID_SOUND_BABA_MG_3: C2RustUnnamed_1 = 253;
pub const ID_SOUND_BABA_MG_2: C2RustUnnamed_1 = 252;
pub const ID_SOUND_BABA_MG_1: C2RustUnnamed_1 = 251;
pub const ID_SOUND_HOWITZ_FLIGHT: C2RustUnnamed_1 = 250;
pub const ID_SOUND_MORTAR: C2RustUnnamed_1 = 249;
pub const ID_SOUND_BEAM_LASER: C2RustUnnamed_1 = 248;
pub const ID_SOUND_PULSE_LASER: C2RustUnnamed_1 = 247;
pub const ID_SOUND_FLAME_THROWER: C2RustUnnamed_1 = 246;
pub const ID_SOUND_MEDIUM_CANNON: C2RustUnnamed_1 = 245;
pub const ID_SOUND_SMALL_CANNON: C2RustUnnamed_1 = 244;
pub const ID_SOUND_LARGE_CANNON: C2RustUnnamed_1 = 243;
pub const ID_SOUND_GAUSSGUN: C2RustUnnamed_1 = 242;
pub const ID_SOUND_ROTARY_LASER: C2RustUnnamed_1 = 241;
pub const ID_SOUND_ROCKET: C2RustUnnamed_1 = 240;
pub const ID_SOUND_COLL_ENEMY_DESTROYED: C2RustUnnamed_1 = 239;
pub const ID_SOUND_COLL_INTERCEPT_AND_DESTROY: C2RustUnnamed_1 = 238;
pub const ID_SOUND_COLL_DIE: C2RustUnnamed_1 = 237;
pub const ID_SOUND_COLL_STARTING_ATTACK_RUN: C2RustUnnamed_1 = 236;
pub const ID_SOUND_COLL_ENGAGING: C2RustUnnamed_1 = 235;
pub const ID_SOUND_COLL_ENEMY_DETECTED: C2RustUnnamed_1 = 234;
pub const ID_SOUND_COLL_FIRE: C2RustUnnamed_1 = 233;
pub const ID_SOUND_COLL_ATTACK: C2RustUnnamed_1 = 232;
pub const ID_SOUND_COLL_DESTROYING_BIOLOGICALS: C2RustUnnamed_1 = 231;
pub const ID_SOUND_COLL_CLEANSE_AND_DESTROY: C2RustUnnamed_1 = 230;
pub const ID_SOUND_ABORTING_ATTACK_RUN3: C2RustUnnamed_1 = 229;
pub const ID_SOUND_COMMENCING_ATTACK_RUN3: C2RustUnnamed_1 = 228;
pub const ID_SOUND_LOCKED_ON3: C2RustUnnamed_1 = 227;
pub const ID_SOUND_RETURNING_TO_BASE3: C2RustUnnamed_1 = 226;
pub const ID_SOUND_ON_OUR_WAY3: C2RustUnnamed_1 = 225;
pub const ID_SOUND_ENEMY_LOCATED3: C2RustUnnamed_1 = 224;
pub const ID_SOUND_ABORTING_ATTACK_RUN2: C2RustUnnamed_1 = 223;
pub const ID_SOUND_COMMENCING_ATTACK_RUN2: C2RustUnnamed_1 = 222;
pub const ID_SOUND_LOCKED_ON2: C2RustUnnamed_1 = 221;
pub const ID_SOUND_RETURNING_TO_BASE2: C2RustUnnamed_1 = 220;
pub const ID_SOUND_ON_OUR_WAY2: C2RustUnnamed_1 = 219;
pub const ID_SOUND_ENEMY_LOCATED2: C2RustUnnamed_1 = 218;
pub const ID_SOUND_ABORTING_ATTACK_RUN1: C2RustUnnamed_1 = 217;
pub const ID_SOUND_COMMENCING_ATTACK_RUN1: C2RustUnnamed_1 = 216;
pub const ID_SOUND_LOCKED_ON1: C2RustUnnamed_1 = 215;
pub const ID_SOUND_RETURNING_TO_BASE1: C2RustUnnamed_1 = 214;
pub const ID_SOUND_ON_OUR_WAY1: C2RustUnnamed_1 = 213;
pub const ID_SOUND_ENEMY_LOCATED1: C2RustUnnamed_1 = 212;
pub const ID_SOUND_PREPARE_FOR_DUST_OFF: C2RustUnnamed_1 = 211;
pub const ID_SOUND_GO_GO_GO: C2RustUnnamed_1 = 210;
pub const ID_SOUND_GREEN_LIGHT_IN_2: C2RustUnnamed_1 = 209;
pub const ID_SOUND_GREEN_LIGHT_IN_3: C2RustUnnamed_1 = 208;
pub const ID_SOUND_GREEN_LIGHT_IN_4: C2RustUnnamed_1 = 207;
pub const ID_SOUND_GREEN_LIGHT_IN_5: C2RustUnnamed_1 = 206;
pub const ID_SOUND_ALRIGHT_BOYS: C2RustUnnamed_1 = 205;
pub const ID_SOUND_APPROACHING_LZ: C2RustUnnamed_1 = 204;
pub const ID_SOUND_RADIOCLICK_6: C2RustUnnamed_1 = 203;
pub const ID_SOUND_RADIOCLICK_5: C2RustUnnamed_1 = 202;
pub const ID_SOUND_RADIOCLICK_4: C2RustUnnamed_1 = 201;
pub const ID_SOUND_RADIOCLICK_3: C2RustUnnamed_1 = 200;
pub const ID_SOUND_RADIOCLICK_2: C2RustUnnamed_1 = 199;
pub const ID_SOUND_RADIOCLICK_1: C2RustUnnamed_1 = 198;
pub const ID_SOUND_COM_HEADING_FOR_RALLY_POINT: C2RustUnnamed_1 = 197;
pub const ID_SOUND_COM_RETURNING_FOR_REPAIR: C2RustUnnamed_1 = 196;
pub const ID_SOUND_COM_UNABLE_TO_COMPLY: C2RustUnnamed_1 = 195;
pub const ID_SOUND_COM_NO_ROUTE_AVAILABLE: C2RustUnnamed_1 = 194;
pub const ID_SOUND_COM_ROUTE_OBSTRUCTED: C2RustUnnamed_1 = 193;
pub const ID_SOUND_COM_ENEMY_VTOLS_DETECTED: C2RustUnnamed_1 = 192;
pub const ID_SOUND_COM_ENEMY_BATTERY_DETECTED: C2RustUnnamed_1 = 191;
pub const ID_SOUND_COM_NEXUS_DETECTED: C2RustUnnamed_1 = 190;
pub const ID_SOUND_COM_NEXUS_TURRET_DETECTED: C2RustUnnamed_1 = 189;
pub const ID_SOUND_COM_NEXUS_TOWER_DETECTED: C2RustUnnamed_1 = 188;
pub const ID_SOUND_COM_FRIENDLY_LZ_DETECTED: C2RustUnnamed_1 = 187;
pub const ID_SOUND_COM_ENEMY_LZ_DETECTED: C2RustUnnamed_1 = 186;
pub const ID_SOUND_COM_ENEMY_TRANSPORT_DETECTED: C2RustUnnamed_1 = 185;
pub const ID_SOUND_COM_ALLY_DETECTED: C2RustUnnamed_1 = 184;
pub const ID_SOUND_COM_ENEMY_BASE_DETECTED: C2RustUnnamed_1 = 183;
pub const ID_SOUND_COM_ENEMY_DETECTED: C2RustUnnamed_1 = 182;
pub const ID_SOUND_COM_ARTEFACT_DETECTED: C2RustUnnamed_1 = 181;
pub const ID_SOUND_COM_RESOURCE_DETECTED: C2RustUnnamed_1 = 180;
pub const ID_SOUND_COM_SCAV_OUTPOST_DETECTED: C2RustUnnamed_1 = 179;
pub const ID_SOUND_COM_SCAV_BASE_DETECTED: C2RustUnnamed_1 = 178;
pub const ID_SOUND_COM_SCAVS_DETECTED: C2RustUnnamed_1 = 177;
pub const ID_SOUND_COMMANDER: C2RustUnnamed_1 = 176;
pub const ID_SOUND_REPORTING: C2RustUnnamed_1 = 175;
pub const ID_SOUND_GROUP_9: C2RustUnnamed_1 = 174;
pub const ID_SOUND_GROUP_8: C2RustUnnamed_1 = 173;
pub const ID_SOUND_GROUP_7: C2RustUnnamed_1 = 172;
pub const ID_SOUND_GROUP_6: C2RustUnnamed_1 = 171;
pub const ID_SOUND_GROUP_5: C2RustUnnamed_1 = 170;
pub const ID_SOUND_GROUP_4: C2RustUnnamed_1 = 169;
pub const ID_SOUND_GROUP_3: C2RustUnnamed_1 = 168;
pub const ID_SOUND_GROUP_2: C2RustUnnamed_1 = 167;
pub const ID_SOUND_GROUP_1: C2RustUnnamed_1 = 166;
pub const ID_SOUND_GROUP_0: C2RustUnnamed_1 = 165;
pub const ID_SOUND_GROUP: C2RustUnnamed_1 = 164;
pub const ID_UNITS_TRANSFER: C2RustUnnamed_1 = 163;
pub const ID_TECHNOLOGY_TRANSFER: C2RustUnnamed_1 = 162;
pub const ID_SENSOR_DOWNLOAD: C2RustUnnamed_1 = 161;
pub const ID_POWER_TRANSMIT: C2RustUnnamed_1 = 160;
pub const ID_GIFT: C2RustUnnamed_1 = 159;
pub const ID_CLAN_EXIT: C2RustUnnamed_1 = 158;
pub const ID_CLAN_ENTER: C2RustUnnamed_1 = 157;
pub const ID_ALLIANCE_OFF: C2RustUnnamed_1 = 156;
pub const ID_ALLIANCE_BRO: C2RustUnnamed_1 = 155;
pub const ID_ALLIANCE_ACC: C2RustUnnamed_1 = 154;
pub const ID_SOUND_ENTERING_WARZONE: C2RustUnnamed_1 = 153;
pub const ID_SOUND_MISSILE_CODES_CRACKED: C2RustUnnamed_1 = 152;
pub const ID_SOUND_3RD_MISSILE_CODES_DECIPHERED: C2RustUnnamed_1 = 151;
pub const ID_SOUND_2ND_MISSILE_CODES_DECIPHERED: C2RustUnnamed_1 = 150;
pub const ID_SOUND_1ST_MISSILE_CODES_DECIPHERED: C2RustUnnamed_1 = 149;
pub const ID_SOUND_MISSILE_CODES_DECIPHERED: C2RustUnnamed_1 = 148;
pub const ID_SOUND_YOU_ARE_DEFEATED: C2RustUnnamed_1 = 147;
pub const ID_SOUND_SYSTEM_FAILURE_IMMINENT: C2RustUnnamed_1 = 146;
pub const ID_SOUND_UNIT_CAPTURED: C2RustUnnamed_1 = 145;
pub const ID_SOUND_1_MINUTE_REMAINING: C2RustUnnamed_1 = 144;
pub const ID_SOUND_2_MINUTES_REMAINING: C2RustUnnamed_1 = 143;
pub const ID_SOUND_3_MINUTES_REMAINING: C2RustUnnamed_1 = 142;
pub const ID_SOUND_5_MINUTES_REMAINING: C2RustUnnamed_1 = 141;
pub const ID_SOUND_10_MINUTES_REMAINING: C2RustUnnamed_1 = 140;
pub const ID_SOUND_MISSION_TIMER_ACTIVATED: C2RustUnnamed_1 = 139;
pub const ID_SOUND_OBJECTIVE_FAILED: C2RustUnnamed_1 = 138;
pub const ID_SOUND_OBJECTIVE_ACCOMPLISHED: C2RustUnnamed_1 = 137;
pub const ID_SOUND_MISSION_SUCCESSFUL: C2RustUnnamed_1 = 136;
pub const ID_SOUND_MISSION_FAILED: C2RustUnnamed_1 = 135;
pub const ID_SOUND_INCOMING_INTELLIGENCE_REPORT: C2RustUnnamed_1 = 134;
pub const ID_SOUND_INCOMING_TRANSMISSION: C2RustUnnamed_1 = 133;
pub const ID_SOUND_TECHNOLOGY_TAKEN: C2RustUnnamed_1 = 132;
pub const ID_SOUND_RESEARCH_STOLEN: C2RustUnnamed_1 = 131;
pub const ID_SOUND_MISSION_RESULTS: C2RustUnnamed_1 = 130;
pub const ID_SOUND_WARZONE_ACTIVE: C2RustUnnamed_1 = 129;
pub const ID_SOUND_WARZONE_PAUSED: C2RustUnnamed_1 = 128;
pub const ID_SOUND_MISSION_UPDATE: C2RustUnnamed_1 = 127;
pub const ID_SOUND_MISSION_OBJECTIVE: C2RustUnnamed_1 = 126;
pub const ID_SOUND_TRANSPORT_UNABLE_TO_LAND: C2RustUnnamed_1 = 125;
pub const ID_SOUND_TRANSPORT_RETURNING_TO_BASE: C2RustUnnamed_1 = 124;
pub const ID_SOUND_LZ_CLEAR: C2RustUnnamed_1 = 123;
pub const ID_SOUND_LZ_COMPROMISED: C2RustUnnamed_1 = 122;
pub const ID_SOUND_TRANSPORT_REPAIRING: C2RustUnnamed_1 = 121;
pub const ID_SOUND_TRANSPORT_UNDER_ATTACK: C2RustUnnamed_1 = 120;
pub const ID_SOUND_TRANSPORT_LANDING: C2RustUnnamed_1 = 119;
pub const ID_SOUND_REINFORCEMENTS_IN_TRANSIT: C2RustUnnamed_1 = 118;
pub const ID_SOUND_REINFORCEMENTS_AVAILABLE: C2RustUnnamed_1 = 117;
pub const ID_SOUND_NO_ROUTE_AVAILABLE: C2RustUnnamed_1 = 116;
pub const ID_SOUND_ROUTE_OBSTRUCTED: C2RustUnnamed_1 = 115;
pub const ID_SOUND_COMMANDER_REPORTING: C2RustUnnamed_1 = 114;
pub const ID_SOUND_GROUP_REPORTING: C2RustUnnamed_1 = 113;
pub const ID_SOUND_ASSIGNED_TO_COMMANDER: C2RustUnnamed_1 = 112;
pub const ID_SOUND_EXCELLENT: C2RustUnnamed_1 = 111;
pub const ID_SOUND_WELL_DONE: C2RustUnnamed_1 = 110;
pub const ID_SOUND_THAT_IS_INCORRECT: C2RustUnnamed_1 = 109;
pub const ID_SOUND_NO: C2RustUnnamed_1 = 108;
pub const ID_SOUND_NOT_POSSIBLE_TRY_AGAIN: C2RustUnnamed_1 = 107;
pub const ID_SOUND_SCATTER: C2RustUnnamed_1 = 106;
pub const ID_SOUND_RECYCLING: C2RustUnnamed_1 = 105;
pub const ID_SOUND_RETURN_TO_LZ: C2RustUnnamed_1 = 104;
pub const ID_SOUND_PATROL: C2RustUnnamed_1 = 103;
pub const ID_SOUND_PURSUE: C2RustUnnamed_1 = 102;
pub const ID_SOUND_GUARD: C2RustUnnamed_1 = 101;
pub const ID_SOUND_HOLD_POSITION: C2RustUnnamed_1 = 100;
pub const ID_SOUND_CEASEFIRE: C2RustUnnamed_1 = 99;
pub const ID_SOUND_RETURN_FIRE: C2RustUnnamed_1 = 98;
pub const ID_SOUND_FIRE_AT_WILL: C2RustUnnamed_1 = 97;
pub const ID_SOUND_NO_RETREAT: C2RustUnnamed_1 = 96;
pub const ID_SOUND_RETREAT_AT_HEAVY_DAMAGE: C2RustUnnamed_1 = 95;
pub const ID_SOUND_RETREAT_AT_MEDIUM_DAMAGE: C2RustUnnamed_1 = 94;
pub const ID_SOUND_OPTIMUM_RANGE: C2RustUnnamed_1 = 93;
pub const ID_SOUND_LONG_RANGE: C2RustUnnamed_1 = 92;
pub const ID_SOUND_SHORT_RANGE: C2RustUnnamed_1 = 91;
pub const ID_SOUND_COMMAND_CONSOLE_ACTIVATED: C2RustUnnamed_1 = 90;
pub const ID_SOUND_INTERCEPTORS_ASSIGNED: C2RustUnnamed_1 = 89;
pub const ID_SOUND_ASSIGNED: C2RustUnnamed_1 = 88;
pub const ID_SOUND_VTOLS_ENGAGING: C2RustUnnamed_1 = 87;
pub const ID_SOUND_REARMING: C2RustUnnamed_1 = 86;
pub const ID_SOUND_INTERCEPTORS_LAUNCHED: C2RustUnnamed_1 = 85;
pub const ID_SOUND_BATTERY_FIRING_COUNTER_ATTACK: C2RustUnnamed_1 = 84;
pub const ID_SOUND_ENEMY_BATTERY_LOCATED: C2RustUnnamed_1 = 83;
pub const ID_SOUND_ASSIGNED_TO_COUNTER_RADAR: C2RustUnnamed_1 = 82;
pub const ID_SOUND_SENSOR_LOCKED_ON: C2RustUnnamed_1 = 81;
pub const ID_SOUND_ASSIGNED_TO_SENSOR: C2RustUnnamed_1 = 80;
pub const ID_SOUND_UNIT_RETURNING_FOR_REPAIR: C2RustUnnamed_1 = 79;
pub const ID_SOUND_UNIT_RETREATING: C2RustUnnamed_1 = 78;
pub const ID_SOUND_UNIT_DESTROYED: C2RustUnnamed_1 = 77;
pub const ID_SOUND_UNIT_UNDER_ATTACK: C2RustUnnamed_1 = 76;
pub const ID_SOUND_LZ2: C2RustUnnamed_1 = 75;
pub const ID_SOUND_LZ1: C2RustUnnamed_1 = 74;
pub const ID_SOUND_ENEMY_LZ: C2RustUnnamed_1 = 73;
pub const ID_SOUND_INCOMING_ENEMY_TRANSPORT: C2RustUnnamed_1 = 72;
pub const ID_SOUND_ENEMY_BASE_ERADICATED: C2RustUnnamed_1 = 71;
pub const ID_SOUND_ENEMY_BASE: C2RustUnnamed_1 = 70;
pub const ID_SOUND_SCAVENGER_BASE_ERADICATED: C2RustUnnamed_1 = 69;
pub const ID_SOUND_SCAVENGER_OUTPOST_ERADICATED: C2RustUnnamed_1 = 68;
pub const ID_SOUND_SCAVENGER_OUTPOST: C2RustUnnamed_1 = 67;
pub const ID_SOUND_SCAVENGER_BASE: C2RustUnnamed_1 = 66;
pub const ID_SOUND_ENEMY_VTOLS_DETECTED: C2RustUnnamed_1 = 65;
pub const ID_SOUND_ENEMY_BATTERY_DETECTED: C2RustUnnamed_1 = 64;
pub const ID_SOUND_NEXUS_UNIT_DETECTED: C2RustUnnamed_1 = 63;
pub const ID_SOUND_NEXUS_TURRET_DETECTED: C2RustUnnamed_1 = 62;
pub const ID_SOUND_NEXUS_TOWER_DETECTED: C2RustUnnamed_1 = 61;
pub const ID_SOUND_FRIENDLY_LZ_DETECTED: C2RustUnnamed_1 = 60;
pub const ID_SOUND_ENEMY_LZ_DETECTED: C2RustUnnamed_1 = 59;
pub const ID_SOUND_ENEMY_TRANSPORT_DETECTED: C2RustUnnamed_1 = 58;
pub const ID_SOUND_ALLY_DETECTED: C2RustUnnamed_1 = 57;
pub const ID_SOUND_ENEMY_BASE_DETECTED: C2RustUnnamed_1 = 56;
pub const ID_SOUND_ENEMY_UNIT_DETECTED: C2RustUnnamed_1 = 55;
pub const ID_SOUND_ARTEFACT_DISC: C2RustUnnamed_1 = 54;
pub const ID_SOUND_POWER_RESOURCE: C2RustUnnamed_1 = 53;
pub const ID_SOUND_SCAVENGER_OUTPOST_DETECTED: C2RustUnnamed_1 = 52;
pub const ID_SOUND_SCAVENGER_BASE_DETECTED: C2RustUnnamed_1 = 51;
pub const ID_SOUND_SCAVENGERS_DETECTED: C2RustUnnamed_1 = 50;
pub const ID_SOUND_UNIT_REPAIRED: C2RustUnnamed_1 = 49;
pub const ID_SOUND_DELIVERY_POINT_ASSIGNED_TO: C2RustUnnamed_1 = 48;
pub const ID_SOUND_DELIVERY_POINT_ASSIGNED: C2RustUnnamed_1 = 47;
pub const ID_SOUND_PRODUCTION_CANCELLED: C2RustUnnamed_1 = 46;
pub const ID_SOUND_PRODUCTION_PAUSED: C2RustUnnamed_1 = 45;
pub const ID_SOUND_DROID_COMPLETED: C2RustUnnamed_1 = 44;
pub const ID_SOUND_PRODUCTION_STARTED: C2RustUnnamed_1 = 43;
pub const ID_SOUND_CYBORG_RESEARCH_COMPLETED: C2RustUnnamed_1 = 42;
pub const ID_SOUND_WEAPON_RESEARCH_COMPLETED: C2RustUnnamed_1 = 41;
pub const ID_SOUND_SYSTEMS_RESEARCH_COMPLETED: C2RustUnnamed_1 = 40;
pub const ID_SOUND_VEHICLE_RESEARCH_COMPLETED: C2RustUnnamed_1 = 39;
pub const ID_SOUND_COMPUTER_RESEARCH_COMPLETED: C2RustUnnamed_1 = 38;
pub const ID_SOUND_POWER_RESEARCH_COMPLETED: C2RustUnnamed_1 = 37;
pub const ID_SOUND_STRUCTURE_RESEARCH_COMPLETED: C2RustUnnamed_1 = 36;
pub const ID_SOUND_MAJOR_RESEARCH: C2RustUnnamed_1 = 35;
pub const ID_SOUND_RESEARCH_COMPLETED: C2RustUnnamed_1 = 34;
pub const ID_SOUND_NEW_CYBORG_AVAILABLE: C2RustUnnamed_1 = 33;
pub const ID_SOUND_NEW_COMPONENT_AVAILABLE: C2RustUnnamed_1 = 32;
pub const ID_SOUND_NEW_STRUCTURE_AVAILABLE: C2RustUnnamed_1 = 31;
pub const ID_SOUND_NEW_RESEARCH_PROJ_AVAILABLE: C2RustUnnamed_1 = 30;
pub const ID_SOUND_ARTIFACT_RECOVERED: C2RustUnnamed_1 = 29;
pub const ID_SOUND_ARTIFACT: C2RustUnnamed_1 = 28;
pub const ID_SOUND_RESEARCH_FACILITY_REQUIRED: C2RustUnnamed_1 = 27;
pub const ID_SOUND_POWER_GENERATOR_REQUIRED: C2RustUnnamed_1 = 26;
pub const ID_SOUND_POWER_TRANSFER_IN_PROGRESS: C2RustUnnamed_1 = 25;
pub const ID_SOUND_RESOURCE_DEPLETED: C2RustUnnamed_1 = 24;
pub const ID_SOUND_DERRICK_DESTROYED: C2RustUnnamed_1 = 23;
pub const ID_SOUND_DERRICK_UNDER_ATTACK: C2RustUnnamed_1 = 22;
pub const ID_SOUND_RESOURCE_HERE: C2RustUnnamed_1 = 21;
pub const ID_SOUND_POWER_LOW: C2RustUnnamed_1 = 20;
pub const ID_SOUND_POWER_GENERATOR_DESTROYED: C2RustUnnamed_1 = 19;
pub const ID_SOUND_POWER_GENERATOR_UNDER_ATTACK: C2RustUnnamed_1 = 18;
pub const ID_SOUND_STRUCTURE_DEMOLISHED: C2RustUnnamed_1 = 17;
pub const ID_SOUND_STRUCTURE_REPAIR_IN_PROGRESS: C2RustUnnamed_1 = 16;
pub const ID_SOUND_STRUCTURE_UNDER_ATTACK: C2RustUnnamed_1 = 15;
pub const ID_SOUND_STRUCTURE_COMPLETED: C2RustUnnamed_1 = 14;
pub const ID_SOUND_CONSTRUCTION_STARTED: C2RustUnnamed_1 = 13;
pub const ID_SOUND_DESIGN_COMPLETED: C2RustUnnamed_1 = 12;
pub const ID_SOUND_PROPULSION_SELECTED: C2RustUnnamed_1 = 11;
pub const ID_SOUND_BODY_SELECTED: C2RustUnnamed_1 = 10;
pub const ID_SOUND_TURRET_SELECTED: C2RustUnnamed_1 = 9;
pub const ID_SOUND_GAME_SHUTDOWN: C2RustUnnamed_1 = 8;
pub const ID_SOUND_MESSAGEEND: C2RustUnnamed_1 = 7;
pub const ID_SOUND_BUILD_FAIL: C2RustUnnamed_1 = 6;
pub const ID_SOUND_ZOOM_ON_RADAR: C2RustUnnamed_1 = 5;
pub const FE_AUDIO_MESSAGEEND: C2RustUnnamed_1 = 4;
pub const ID_SOUND_BUTTON_CLICK_5: C2RustUnnamed_1 = 3;
pub const ID_SOUND_SELECT: C2RustUnnamed_1 = 2;
pub const ID_SOUND_WINDOWOPEN: C2RustUnnamed_1 = 1;
pub const ID_SOUND_WINDOWCLOSE: C2RustUnnamed_1 = 0;
pub const NO_SOUND: C2RustUnnamed_1 = -1;
static mut selectedKeyMap: *mut KEY_MAPPING =
    0 as *const KEY_MAPPING as *mut KEY_MAPPING;
#[no_mangle]
pub static mut keymapVersion: [libc::c_char; 8] =
    [75, 77, 95, 48, 48, 48, 50, 0];
// ////////////////////////////////////////////////////////////////////////////
// funcs
unsafe extern "C" fn pushedKeyMap(mut key: UDWORD) -> BOOL {
    //	UDWORD count =0;
//	id-KM_START
//	for(selectedKeyMap = keyMappings;
//		selectedKeyMap->status != KEYMAP_ASSIGNABLE;
//		(selectedKeyMap->status= KEYMAP__DEBUG) && (selectedKeyMap->status==KEYMAP___HIDE);
//
//		selectedKeyMap = selectedKeyMap->psNext);
//
//	while(count!=key)
//	{
//		selectedKeyMap = selectedKeyMap->psNext;
//		if((selectedKeyMap->status!=KEYMAP__DEBUG)&&(selectedKeyMap->status!=KEYMAP___HIDE))		// if it's not a debug mapping..
//		if(selectedKeyMap->status == KEYMAP_ASSIGNABLE)
//		{
//			count++;
//		}
//	}
    selectedKeyMap =
        (*widgGetFromID(psWScreen, key)).pUserData as *mut KEY_MAPPING;
    if !selectedKeyMap.is_null() &&
           (*selectedKeyMap).status as libc::c_uint !=
               KEYMAP_ASSIGNABLE as libc::c_int as libc::c_uint {
        selectedKeyMap = 0 as *mut KEY_MAPPING;
        audio_PlayTrack(ID_SOUND_BUILD_FAIL as libc::c_int);
    }
    return 1 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////////
unsafe extern "C" fn pushedKeyCombo(mut subkey: UDWORD) -> BOOL {
    let mut metakey: KEY_CODE = 5190 as KEY_CODE;
    let mut pExist: *mut KEY_MAPPING = 0 as *mut KEY_MAPPING;
    let mut psMapping: *mut KEY_MAPPING = 0 as *mut KEY_MAPPING;
    let mut alt: KEY_CODE = 0 as KEY_CODE;
    //	void (*function)(void);
   //	KEY_ACTION	action;
   //	KEY_STATUS	status;
   //	STRING	name[255];
    // check for
	// alt
    alt = 0 as KEY_CODE;
    if keyDown(KEY_RALT) != 0 || keyDown(KEY_LALT) != 0 {
        metakey = KEY_LALT;
        alt = KEY_RALT
    } else if keyDown(KEY_RCTRL) != 0 || keyDown(KEY_LCTRL) != 0 {
        metakey = KEY_LCTRL;
        alt = KEY_RCTRL
    } else if keyDown(KEY_RSHIFT) != 0 || keyDown(KEY_LSHIFT) != 0 {
        metakey = KEY_LSHIFT;
        alt = KEY_RSHIFT
    }
    // ctrl
    // shift
    // check if bound to a fixed combo.
    pExist =
        keyFindMapping(metakey, subkey as KEY_CODE); // unhighlght selected.
    if !pExist.is_null() &&
           ((*pExist).status as libc::c_uint ==
                KEYMAP_ALWAYS as libc::c_int as libc::c_uint ||
                (*pExist).status as libc::c_uint ==
                    KEYMAP_ALWAYS_PROCESS as libc::c_int as libc::c_uint) {
        selectedKeyMap = 0 as *mut KEY_MAPPING;
        return 0 as libc::c_int
    }
    /* Clear down mappings using these keys... But only if it isn't unassigned */
    keyReAssignMapping(metakey, subkey as KEY_CODE, 5190 as KEY_CODE,
                       SDLK_LAST as libc::c_int as KEY_CODE);
    /* Try and see if its there already - damn well should be! */
    psMapping =
        keyGetMappingFromFunction(::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                     -> ()>,
                                                          *mut libc::c_void>((*selectedKeyMap).function));
    /* Cough if it's not there */
    if !psMapping.is_null() {
    } else {
        debug(LOG_ERROR,
              b"Trying to patch a non-existant function mapping - whoop whoop!!!\x00"
                  as *const u8 as *const libc::c_char);
    };
    if !psMapping.is_null() {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"keyedit.c\x00" as *const u8 as *const libc::c_char,
              158 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 15],
                                        &[libc::c_char; 15]>(b"pushedKeyCombo\x00")).as_ptr(),
              b"psMapping!=NULL\x00" as *const u8 as *const libc::c_char);
    };
    /* Now alter it to the new values */
    (*psMapping).metaKeyCode = metakey;
    (*psMapping).subKeyCode = subkey as KEY_CODE;
    // was "=="
    (*psMapping).status = KEYMAP_ASSIGNABLE; //must be
    if alt as u64 != 0 { (*psMapping).altMetaKeyCode = alt }
    /*



	// unbind old mapping with this combo.
//	function = selectedKeyMap->function;
//	action = selectedKeyMap->action;
//	status = selectedKeyMap->status;
//	strcpy(name,selectedKeyMap->pName);
//	keyRemoveMappingPt(selectedKeyMap);

	keyAddMapping(status,metakey,subkey,action,function,name);

	// add new binding.
//	keyReAssignMapping( selectedKeyMap->metaKeyCode, selectedKeyMap->subKeyCode, metakey, subkey);
  //	keyAddMapping(

	selectedKeyMap->metaKeyCode = metakey;
	selectedKeyMap->subKeyCode = subkey;

// readd the widgets.
//	widgDelete(psWScreen,FRONTEND_BACKDROP);
//	startKeyMapEditor(FALSE);

	*/
    selectedKeyMap = 0 as *mut KEY_MAPPING; // unhighlght selected .
    return 1 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////////
unsafe extern "C" fn scanKeyBoardForBinding() -> UDWORD {
    let mut i: UDWORD = 0;
    i = 0 as libc::c_int as UDWORD;
    while i <= SDLK_LAST as libc::c_int as libc::c_uint {
        if keyPressed(i as KEY_CODE) != 0 {
            if i != KEY_RALT as libc::c_int as libc::c_uint &&
                   i != KEY_LALT as libc::c_int as libc::c_uint &&
                   i != KEY_RCTRL as libc::c_int as libc::c_uint &&
                   i != KEY_LCTRL as libc::c_int as libc::c_uint &&
                   i != KEY_RSHIFT as libc::c_int as libc::c_uint &&
                   i != KEY_LSHIFT as libc::c_int as libc::c_uint {
                return i
                // top row key pressed
            }
        }
        i = i.wrapping_add(1)
    }
    return 0 as libc::c_int as UDWORD;
}
// ////////////////////////////////////////////////////////////////////////////
// protos
// ////////////////////////////////////////////////////////////////////////////
#[no_mangle]
pub unsafe extern "C" fn runKeyMapEditor() -> BOOL {
    let mut id: UDWORD = 0; // Run the current set of widgets
    id = widgRunScreen(psWScreen);
    if id == 10202 as libc::c_int as libc::c_uint {
        // return
        saveKeyMap(); // readd the widgets
        changeTitleMode(TITLE); // show the widgets currently running
    }
    if id == 10203 as libc::c_int as libc::c_uint {
        keyClearMappings();
        keyInitMappings(1 as libc::c_int);
        widgDelete(psWScreen, 20000 as libc::c_int as UDWORD);
        startKeyMapEditor(0 as libc::c_int);
    } else if id >= 10204 as libc::c_int as libc::c_uint &&
                  id <= 10399 as libc::c_int as libc::c_uint {
        pushedKeyMap(id);
    }
    if !selectedKeyMap.is_null() {
        id = scanKeyBoardForBinding();
        if id != 0 { pushedKeyCombo(id); }
    }
    DrawBegin();
    StartCursorSnap(&mut InterfaceSnap);
    widgDisplayScreen(psWScreen);
    DrawEnd();
    return 1 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////////
// returns key to press given a mapping.
unsafe extern "C" fn keyMapToString(mut pStr: *mut STRING,
                                    mut psMapping: *mut KEY_MAPPING) -> BOOL {
    let mut onlySub: BOOL = 1 as libc::c_int;
    let mut asciiSub: [STRING; 20] = [0; 20];
    let mut asciiMeta: [STRING; 20] = [0; 20];
    if (*psMapping).metaKeyCode as libc::c_uint !=
           5190 as libc::c_int as libc::c_uint {
        keyScanToString((*psMapping).metaKeyCode,
                        &mut asciiMeta as *mut [STRING; 20] as *mut STRING,
                        20 as libc::c_int as UDWORD);
        onlySub = 0 as libc::c_int
    }
    keyScanToString((*psMapping).subKeyCode,
                    &mut asciiSub as *mut [STRING; 20] as *mut STRING,
                    20 as libc::c_int as UDWORD);
    if onlySub != 0 {
        sprintf(pStr, b"%s\x00" as *const u8 as *const libc::c_char,
                asciiSub.as_mut_ptr());
    } else {
        sprintf(pStr, b"%s - %s\x00" as *const u8 as *const libc::c_char,
                asciiMeta.as_mut_ptr(), asciiSub.as_mut_ptr());
    }
    return 1 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////////
// display a keymap on the interface.
#[no_mangle]
pub unsafe extern "C" fn displayKeyMap(mut psWidget: *mut _widget,
                                       mut xOffset: UDWORD,
                                       mut yOffset: UDWORD,
                                       mut pColours: *mut UDWORD) {
    let mut x: UDWORD =
        xOffset.wrapping_add((*psWidget).x as libc::c_uint); // was just 40
    let mut y: UDWORD = yOffset.wrapping_add((*psWidget).y as libc::c_uint);
    let mut w: UDWORD = (*psWidget).width as UDWORD;
    let mut h: UDWORD = (*psWidget).height as UDWORD;
    let mut psMapping: *mut KEY_MAPPING =
        (*psWidget).pUserData as *mut KEY_MAPPING;
    let mut sKey: [STRING; 60] = [0; 60];
    if psMapping == selectedKeyMap {
        pie_BoxFillIndex(x as libc::c_int, y as libc::c_int,
                         x.wrapping_add(w) as libc::c_int,
                         y.wrapping_add(h) as libc::c_int,
                         *colours.as_mut_ptr().offset(2 as libc::c_int as
                                                          isize));
    } else if (*psMapping).status as libc::c_uint ==
                  KEYMAP_ALWAYS as libc::c_int as libc::c_uint ||
                  (*psMapping).status as libc::c_uint ==
                      KEYMAP_ALWAYS_PROCESS as libc::c_int as libc::c_uint {
        pie_BoxFillIndex(x as libc::c_int, y as libc::c_int,
                         x.wrapping_add(w) as libc::c_int,
                         y.wrapping_add(h) as libc::c_int,
                         *colours.as_mut_ptr().offset(4 as libc::c_int as
                                                          isize));
    } else { drawBlueBox(x, y, w, h); }
    // draw name
    iV_SetFont(WFont); // font
    iV_SetTextColour(-(1 as libc::c_int) as SWORD); //colour
    pie_DrawText((*psMapping).pName,
                 x.wrapping_add(2 as libc::c_int as libc::c_uint),
                 y.wrapping_add(((*psWidget).height as libc::c_int /
                                     2 as libc::c_int) as
                                    libc::c_uint).wrapping_add(3 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uint));
    // draw binding
    keyMapToString(sKey.as_mut_ptr(), psMapping);
    pie_DrawText(sKey.as_mut_ptr(),
                 x.wrapping_add(370 as libc::c_int as libc::c_uint),
                 y.wrapping_add(((*psWidget).height as libc::c_int /
                                     2 as libc::c_int) as
                                    libc::c_uint).wrapping_add(3 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uint));
}
// ////////////////////////////////////////////////////////////////////////////
#[no_mangle]
pub unsafe extern "C" fn startKeyMapEditor(mut first: BOOL) -> BOOL {
    let mut sButInit: W_BUTINIT =
        W_BUTINIT{formID: 0,
                  majorID: 0,
                  minorID: 0,
                  id: 0,
                  style: 0,
                  x: 0,
                  y: 0,
                  width: 0,
                  height: 0,
                  pDisplay: None,
                  pCallback: None,
                  pUserData: 0 as *mut libc::c_void,
                  UserData: 0,
                  pText: 0 as *mut STRING,
                  pTip: 0 as *mut STRING,
                  FontID: 0,};
    let mut sFormInit: W_FORMINIT =
        W_FORMINIT{formID: 0,
                   majorID: 0,
                   minorID: 0,
                   id: 0,
                   style: 0,
                   x: 0,
                   y: 0,
                   width: 0,
                   height: 0,
                   pDisplay: None,
                   pCallback: None,
                   pUserData: 0 as *mut libc::c_void,
                   UserData: 0,
                   disableChildren: 0,
                   majorPos: 0,
                   minorPos: 0,
                   majorSize: 0,
                   minorSize: 0,
                   majorOffset: 0,
                   minorOffset: 0,
                   tabVertOffset: 0,
                   tabHorzOffset: 0,
                   tabMajorThickness: 0,
                   tabMinorThickness: 0,
                   tabMajorGap: 0,
                   tabMinorGap: 0,
                   numMajor: 0,
                   aNumMinors: [0; 9],
                   pTip: 0 as *mut STRING,
                   apMajorTips: [0 as *mut STRING; 9],
                   apMinorTips: [[0 as *mut STRING; 5]; 9],
                   pTabDisplay: None,
                   pFormDisplay: None,};
    let mut psMapping: *mut KEY_MAPPING = 0 as *mut KEY_MAPPING;
    let mut i: UDWORD = 0;
    let mut mapcount: UDWORD = 0 as libc::c_int as UDWORD;
    let mut bubbleCount: UDWORD = 0;
    let mut bAtEnd: BOOL = 0;
    let mut bGotOne: BOOL = 0;
    let mut psPresent: *mut KEY_MAPPING = 0 as *mut KEY_MAPPING;
    let mut psNext: *mut KEY_MAPPING = 0 as *mut KEY_MAPPING;
    let mut test: [libc::c_char; 255] = [0; 255];
    addBackdrop();
    addSideText(20026 as libc::c_int as UDWORD,
                (30 as libc::c_int - 2 as libc::c_int) as UDWORD,
                20 as libc::c_int as UDWORD,
                strresGetString(psStringRes,
                                STR_KM_KEYMAP_SIDE as libc::c_int as UDWORD));
    if first != 0 {
        loadKeyMap();
        // get the current mappings.
    } // draw blue form...
    memset(&mut sFormInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_FORMINIT>() as libc::c_ulong); // default.
    sFormInit.formID = 20000 as libc::c_int as UDWORD;
    sFormInit.id = 10200 as libc::c_int as UDWORD;
    sFormInit.style = 0 as libc::c_int as UDWORD;
    sFormInit.x = 30 as libc::c_int as SWORD;
    sFormInit.y = 20 as libc::c_int as SWORD;
    sFormInit.width = 580 as libc::c_int as UWORD;
    sFormInit.height = 440 as libc::c_int as UWORD;
    sFormInit.pDisplay =
        Some(intDisplayPlainForm as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    widgAddForm(psWScreen, &mut sFormInit);
    addMultiBut(psWScreen, 10200 as libc::c_int as UDWORD,
                10202 as libc::c_int as UDWORD, 8 as libc::c_int as UDWORD,
                5 as libc::c_int as UDWORD,
                iV_GetImageWidth(FrontImages,
                                 IMAGE_RETURN as libc::c_int as UWORD) as
                    UDWORD,
                iV_GetImageHeight(FrontImages,
                                  IMAGE_RETURN as libc::c_int as UWORD) as
                    UDWORD, STR_MUL_CANCEL as libc::c_int as UDWORD,
                IMAGE_RETURN as libc::c_int as UDWORD,
                IMAGE_RETURN_HI as libc::c_int as UDWORD, 1 as libc::c_int);
    addMultiBut(psWScreen, 10200 as libc::c_int as UDWORD,
                10203 as libc::c_int as UDWORD, 11 as libc::c_int as UDWORD,
                45 as libc::c_int as UDWORD, 56 as libc::c_int as UDWORD,
                38 as libc::c_int as UDWORD,
                STR_MUL_DEFAULT as libc::c_int as UDWORD,
                IMAGE_KEYMAP_DEFAULT as libc::c_int as UDWORD,
                IMAGE_KEYMAP_DEFAULT as libc::c_int as UDWORD,
                1 as libc::c_int);
    /* Better be none that come after this...! */
    strcpy(test.as_mut_ptr(),
           b"zzzzzzzzzzzzzzzzzzzzz\x00" as *const u8 as *const libc::c_char);
    psMapping = 0 as *mut KEY_MAPPING;
    //count mappings required.
    psMapping = keyMappings;
    while !psMapping.is_null() {
        if (*psMapping).status as libc::c_uint !=
               KEYMAP__DEBUG as libc::c_int as libc::c_uint &&
               (*psMapping).status as libc::c_uint !=
                   KEYMAP___HIDE as libc::c_int as libc::c_uint {
            // if it's not a debug mapping..
            mapcount = mapcount.wrapping_add(1);
            if strcmp((*psMapping).pName, test.as_mut_ptr()) <
                   0 as libc::c_int {
                /* Best one found so far */
                strcpy(test.as_mut_ptr(), (*psMapping).pName);
                psPresent = psMapping
            }
        }
        psMapping = (*psMapping).psNext
    }
    // add tab form..
    memset(&mut sFormInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_FORMINIT>() as libc::c_ulong);
    sFormInit.formID = 10200 as libc::c_int as UDWORD;
    sFormInit.id = 10201 as libc::c_int as UDWORD;
    sFormInit.style = 1 as libc::c_int as UDWORD;
    sFormInit.x = 50 as libc::c_int as SWORD;
    sFormInit.y = 10 as libc::c_int as SWORD;
    sFormInit.width = (580 as libc::c_int - 100 as libc::c_int) as UWORD;
    sFormInit.height = (440 as libc::c_int - 4 as libc::c_int) as UWORD;
    sFormInit.numMajor = numForms(mapcount, 20 as libc::c_int as UDWORD);
    sFormInit.majorPos = 1 as libc::c_int as UWORD;
    sFormInit.minorPos = 0 as libc::c_int as UWORD;
    sFormInit.majorSize = (26 as libc::c_int + 3 as libc::c_int) as UWORD;
    sFormInit.majorOffset = 2 as libc::c_int as SWORD;
    sFormInit.tabVertOffset = (11 as libc::c_int / 2 as libc::c_int) as SWORD;
    sFormInit.tabMajorThickness = 11 as libc::c_int as UWORD;
    sFormInit.pFormDisplay =
        Some(intDisplayObjectForm as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    sFormInit.pUserData =
        &mut StandardTab as *mut TABDEF as *mut libc::c_void;
    sFormInit.pTabDisplay =
        Some(intDisplayTab as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: UDWORD, _: BOOL, _: BOOL, _: UDWORD,
                                      _: UDWORD, _: UDWORD, _: UDWORD) -> ());
    i = 0 as libc::c_int as UDWORD;
    while i < sFormInit.numMajor as libc::c_uint {
        sFormInit.aNumMinors[i as usize] = 1 as libc::c_int as UWORD;
        i = i.wrapping_add(1)
    }
    widgAddForm(psWScreen, &mut sFormInit);
    //Put the buttons on it
    memset(&mut sButInit as *mut W_BUTINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_BUTINIT>() as libc::c_ulong);
    sButInit.formID = 10201 as libc::c_int as UDWORD;
    sButInit.style = 0 as libc::c_int as UDWORD;
    sButInit.width = 480 as libc::c_int as UWORD;
    sButInit.height =
        ((440 as libc::c_int - 50 as libc::c_int) / 20 as libc::c_int -
             3 as libc::c_int) as UWORD;
    sButInit.pDisplay =
        Some(displayKeyMap as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    sButInit.x = 2 as libc::c_int as SWORD;
    sButInit.y = 16 as libc::c_int as SWORD;
    sButInit.id = 10204 as libc::c_int as UDWORD;
    /* Add our first mapping to the form */
    sButInit.pUserData = psPresent as *mut libc::c_void;
    widgAddButton(psWScreen, &mut sButInit);
    sButInit.id = sButInit.id.wrapping_add(1);
    sButInit.y =
        (sButInit.y as libc::c_int +
             ((440 as libc::c_int - 50 as libc::c_int) / 20 as libc::c_int -
                  3 as libc::c_int + 3 as libc::c_int)) as SWORD;
    /* Now add the others... */
    bubbleCount = 0 as libc::c_int as UDWORD;
    bAtEnd = 0 as libc::c_int;
    /* Stop when the right number or when aphabetically last - not sure...! */
    while bubbleCount <
              mapcount.wrapping_sub(1 as libc::c_int as libc::c_uint) &&
              bAtEnd == 0 {
        /* Same test as before for upper limit */
        strcpy(test.as_mut_ptr(),
               b"zzzzzzzzzzzzzzzzzzzzz\x00" as *const u8 as
                   *const libc::c_char);
        psMapping = keyMappings;
        psNext = 0 as *mut KEY_MAPPING;
        bGotOne = 0 as libc::c_int;
        while !psMapping.is_null() {
            /* Only certain mappings get displayed */
            if (*psMapping).status as libc::c_uint !=
                   KEYMAP__DEBUG as libc::c_int as libc::c_uint &&
                   (*psMapping).status as libc::c_uint !=
                       KEYMAP___HIDE as libc::c_int as libc::c_uint {
                // if it's not a debug mapping..
                /* If it's alphabetically good but better then next one */
                if strcmp((*psMapping).pName, test.as_mut_ptr()) <
                       0 as libc::c_int &&
                       strcmp((*psMapping).pName, (*psPresent).pName) >
                           0 as libc::c_int {
                    /* Keep a record of it */
                    strcpy(test.as_mut_ptr(), (*psMapping).pName);
                    psNext = psMapping;
                    bGotOne = 1 as libc::c_int
                }
            }
            psMapping = (*psMapping).psNext
        }
        /* We found one matching criteria */
        if bGotOne != 0 {
            psPresent = psNext;
            bubbleCount = bubbleCount.wrapping_add(1);
            sButInit.pUserData = psNext as *mut libc::c_void;
            widgAddButton(psWScreen, &mut sButInit);
            // move on..
            sButInit.id = sButInit.id.wrapping_add(1);
            /* Might be no more room on the page */
            if sButInit.y as libc::c_int +
                   ((440 as libc::c_int - 50 as libc::c_int) /
                        20 as libc::c_int - 3 as libc::c_int) +
                   5 as libc::c_int >
                   3 as libc::c_int +
                       20 as libc::c_int *
                           ((440 as libc::c_int - 50 as libc::c_int) /
                                20 as libc::c_int - 3 as libc::c_int +
                                3 as libc::c_int) {
                sButInit.y = 16 as libc::c_int as SWORD;
                sButInit.majorID =
                    (sButInit.majorID as libc::c_int + 1 as libc::c_int) as
                        UWORD
            } else {
                sButInit.y =
                    (sButInit.y as libc::c_int +
                         ((440 as libc::c_int - 50 as libc::c_int) /
                              20 as libc::c_int - 3 as libc::c_int +
                              3 as libc::c_int)) as SWORD
            }
        } else {
            /* The previous one we found was alphabetically last - time to stop */
            bAtEnd = 1 as libc::c_int
        }
    }
    /* Go home... */
    return 1 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////////
// save current keymaps to registry
// FIXME: Use the endian-safe physfs functions.
#[no_mangle]
pub unsafe extern "C" fn saveKeyMap() -> BOOL {
    let mut psMapping: *mut KEY_MAPPING = 0 as *mut KEY_MAPPING;
    let mut count: SDWORD = 0;
    let mut name: [STRING; 128] = [0; 128];
    let mut pfile: *mut PHYSFS_File = 0 as *mut PHYSFS_File;
    // KeyMapPath
    debug(LOG_WZ,
          b"We are to write %s for keymap info\x00" as *const u8 as
              *const libc::c_char, KeyMapPath.as_mut_ptr());
    pfile = PHYSFS_openWrite(KeyMapPath.as_mut_ptr());
    if pfile.is_null() {
        debug(LOG_ERROR,
              b"saveKeyMap: %s could not be created: %s\x00" as *const u8 as
                  *const libc::c_char, KeyMapPath.as_mut_ptr(),
              PHYSFS_getLastError());
        return 0 as libc::c_int
    }
    // write out number of entries.
    count = 0 as libc::c_int;
    psMapping = keyMappings;
    while !psMapping.is_null() { count += 1; psMapping = (*psMapping).psNext }
    if PHYSFS_write(pfile, &mut count as *mut SDWORD as *const libc::c_void,
                    1 as libc::c_int as PHYSFS_uint32,
                    ::std::mem::size_of::<SDWORD>() as libc::c_ulong) !=
           ::std::mem::size_of::<SDWORD>() as libc::c_ulong as
               libc::c_longlong {
        debug(LOG_ERROR,
              b"saveKeyMap: could not write to %s %d bytes: %s\x00" as
                  *const u8 as *const libc::c_char, KeyMapPath.as_mut_ptr(),
              ::std::mem::size_of::<SDWORD>() as libc::c_ulong,
              PHYSFS_getLastError());
        return 0 as libc::c_int
    }
    if PHYSFS_write(pfile,
                    &mut keymapVersion as *mut [libc::c_char; 8] as
                        *const libc::c_void,
                    1 as libc::c_int as PHYSFS_uint32,
                    8 as libc::c_int as PHYSFS_uint32) !=
           8 as libc::c_int as libc::c_longlong {
        debug(LOG_ERROR,
              b"saveKeyMap: could not write to %s %d bytes: %s\x00" as
                  *const u8 as *const libc::c_char, KeyMapPath.as_mut_ptr(),
              8 as libc::c_int, PHYSFS_getLastError());
        return 0 as libc::c_int
    }
    psMapping = keyMappings;
    while !psMapping.is_null() {
        // save this map.
		// name
        strcpy(name.as_mut_ptr(), (*psMapping).pName);
        if PHYSFS_write(pfile,
                        &mut name as *mut [STRING; 128] as
                            *const libc::c_void,
                        1 as libc::c_int as PHYSFS_uint32,
                        128 as libc::c_int as PHYSFS_uint32) !=
               128 as libc::c_int as libc::c_longlong {
            debug(LOG_ERROR,
                  b"saveKeyMap: could not write to %s %d bytes: %s\x00" as
                      *const u8 as *const libc::c_char,
                  KeyMapPath.as_mut_ptr(), 128 as libc::c_int,
                  PHYSFS_getLastError());
            return 0 as libc::c_int
        }
        if PHYSFS_write(pfile,
                        &mut (*psMapping).status as *mut KEY_STATUS as
                            *const libc::c_void,
                        1 as libc::c_int as PHYSFS_uint32,
                        ::std::mem::size_of::<KEY_STATUS>() as libc::c_ulong)
               !=
               ::std::mem::size_of::<KEY_STATUS>() as libc::c_ulong as
                   libc::c_longlong {
            debug(LOG_ERROR,
                  b"saveKeyMap: could not write to %s %d bytes: %s\x00" as
                      *const u8 as *const libc::c_char,
                  KeyMapPath.as_mut_ptr(),
                  ::std::mem::size_of::<KEY_STATUS>() as libc::c_ulong,
                  PHYSFS_getLastError());
            return 0 as libc::c_int
        }
        // status
        if PHYSFS_write(pfile,
                        &mut (*psMapping).metaKeyCode as *mut KEY_CODE as
                            *const libc::c_void,
                        1 as libc::c_int as PHYSFS_uint32,
                        ::std::mem::size_of::<KEY_CODE>() as libc::c_ulong) !=
               ::std::mem::size_of::<KEY_CODE>() as libc::c_ulong as
                   libc::c_longlong {
            debug(LOG_ERROR,
                  b"saveKeyMap: could not write to %s %d bytes: %s\x00" as
                      *const u8 as *const libc::c_char,
                  KeyMapPath.as_mut_ptr(),
                  ::std::mem::size_of::<KEY_CODE>() as libc::c_ulong,
                  PHYSFS_getLastError());
            return 0 as libc::c_int
        }
        // metakey
        if PHYSFS_write(pfile,
                        &mut (*psMapping).subKeyCode as *mut KEY_CODE as
                            *const libc::c_void,
                        1 as libc::c_int as PHYSFS_uint32,
                        ::std::mem::size_of::<KEY_CODE>() as libc::c_ulong) !=
               ::std::mem::size_of::<KEY_CODE>() as libc::c_ulong as
                   libc::c_longlong {
            debug(LOG_ERROR,
                  b"saveKeyMap: could not write to %s %d bytes: %s\x00" as
                      *const u8 as *const libc::c_char,
                  KeyMapPath.as_mut_ptr(),
                  ::std::mem::size_of::<KEY_CODE>() as libc::c_ulong,
                  PHYSFS_getLastError());
            return 0 as libc::c_int
        }
        // subkey
        if PHYSFS_write(pfile,
                        &mut (*psMapping).action as *mut KEY_ACTION as
                            *const libc::c_void,
                        1 as libc::c_int as PHYSFS_uint32,
                        ::std::mem::size_of::<KEY_ACTION>() as libc::c_ulong)
               !=
               ::std::mem::size_of::<KEY_ACTION>() as libc::c_ulong as
                   libc::c_longlong {
            debug(LOG_ERROR,
                  b"saveKeyMap: could not write to %s %d bytes: %s\x00" as
                      *const u8 as *const libc::c_char,
                  KeyMapPath.as_mut_ptr(),
                  ::std::mem::size_of::<KEY_ACTION>() as libc::c_ulong,
                  PHYSFS_getLastError());
            return 0 as libc::c_int
        }
        // action
        // function to map to!
        count = 0 as libc::c_int;
        while (*keyMapSaveTable.as_mut_ptr().offset(count as isize)).is_some()
                  &&
                  *keyMapSaveTable.as_mut_ptr().offset(count as isize) !=
                      (*psMapping).function {
            count += 1
        }
        if (*keyMapSaveTable.as_mut_ptr().offset(count as isize)).is_none() {
            debug(LOG_ERROR,
                  b"can\'t find keymapped function in the keymap save table!!\x00"
                      as *const u8 as *const libc::c_char);
            abort();
        }
        if PHYSFS_write(pfile,
                        &mut count as *mut SDWORD as *const libc::c_void,
                        1 as libc::c_int as PHYSFS_uint32,
                        ::std::mem::size_of::<SDWORD>() as libc::c_ulong) !=
               ::std::mem::size_of::<SDWORD>() as libc::c_ulong as
                   libc::c_longlong {
            debug(LOG_ERROR,
                  b"saveKeyMap: could not write to %s %d bytes: %s\x00" as
                      *const u8 as *const libc::c_char,
                  KeyMapPath.as_mut_ptr(),
                  ::std::mem::size_of::<SDWORD>() as libc::c_ulong,
                  PHYSFS_getLastError());
            return 0 as libc::c_int
        }
        psMapping = (*psMapping).psNext
    }
    if PHYSFS_close(pfile) == 0 {
        debug(LOG_ERROR,
              b"saveKeyMap: Error closing %s: %s\x00" as *const u8 as
                  *const libc::c_char, KeyMapPath.as_mut_ptr(),
              PHYSFS_getLastError());
        return 0 as libc::c_int
    }
    debug(LOG_WZ,
          b"Keymap written ok to %s.\x00" as *const u8 as *const libc::c_char,
          KeyMapPath.as_mut_ptr());
    return 1 as libc::c_int;
    // saved ok.
}
// ////////////////////////////////////////////////////////////////////////////
// load keymaps from registry.
#[no_mangle]
pub unsafe extern "C" fn loadKeyMap() -> BOOL {
    let mut status: KEY_STATUS = KEYMAP__DEBUG;
    let mut metaCode: KEY_CODE = 0 as KEY_CODE;
    let mut subCode: KEY_CODE = 0 as KEY_CODE;
    let mut action: KEY_ACTION = KEYMAP_DOWN;
    let mut name: [STRING; 128] = [0; 128];
    let mut count: SDWORD = 0;
    let mut funcmap: UDWORD = 0;
    let mut ver: [libc::c_char; 8] = [0; 8];
    let mut pfile: *mut PHYSFS_File = 0 as *mut PHYSFS_File;
    let mut filesize: PHYSFS_sint64 = 0;
    let mut countsize: PHYSFS_sint64 = 0 as libc::c_int as PHYSFS_sint64;
    let mut length_read: PHYSFS_sint64 = 0;
    // throw away any keymaps!!
    keyClearMappings();
    if PHYSFS_exists(KeyMapPath.as_mut_ptr()) == 0 {
        debug(LOG_WZ,
              b"loadKeyMap: %s not found\x00" as *const u8 as
                  *const libc::c_char, KeyMapPath.as_mut_ptr());
        return 0 as libc::c_int
    }
    pfile = PHYSFS_openRead(KeyMapPath.as_mut_ptr());
    if pfile.is_null() {
        debug(LOG_ERROR,
              b"loadKeyMap: %s could not be opened: %s\x00" as *const u8 as
                  *const libc::c_char, KeyMapPath.as_mut_ptr(),
              PHYSFS_getLastError());
        return 0 as libc::c_int
    }
    filesize = PHYSFS_fileLength(pfile);
    length_read =
        PHYSFS_read(pfile, &mut count as *mut SDWORD as *mut libc::c_void,
                    1 as libc::c_int as PHYSFS_uint32,
                    ::std::mem::size_of::<SDWORD>() as libc::c_ulong);
    countsize += length_read;
    if length_read !=
           ::std::mem::size_of::<SDWORD>() as libc::c_ulong as
               libc::c_longlong {
        debug(LOG_ERROR,
              b"loadKeyMap: Reading %s short: %s\x00" as *const u8 as
                  *const libc::c_char, KeyMapPath.as_mut_ptr(),
              PHYSFS_getLastError());
        PHYSFS_close(pfile);
        return 0 as libc::c_int
    }
    length_read =
        PHYSFS_read(pfile,
                    &mut ver as *mut [libc::c_char; 8] as *mut libc::c_void,
                    1 as libc::c_int as PHYSFS_uint32,
                    8 as libc::c_int as PHYSFS_uint32);
    countsize += length_read;
    if length_read != 8 as libc::c_int as libc::c_longlong {
        debug(LOG_ERROR,
              b"loadKeyMap: Reading %s short: %s\x00" as *const u8 as
                  *const libc::c_char, KeyMapPath.as_mut_ptr(),
              PHYSFS_getLastError());
        PHYSFS_close(pfile);
        return 0 as libc::c_int
    }
    // get version number.
    if strncmp(ver.as_mut_ptr(), keymapVersion.as_mut_ptr(),
               8 as libc::c_int as libc::c_uint) != 0 as libc::c_int {
        /* If wrong version, create a new one instead. */
        PHYSFS_close(pfile);
        return 0 as libc::c_int
    }
    while count > 0 as libc::c_int {
        length_read =
            PHYSFS_read(pfile,
                        &mut name as *mut [STRING; 128] as *mut libc::c_void,
                        1 as libc::c_int as PHYSFS_uint32,
                        128 as libc::c_int as PHYSFS_uint32);
        countsize += length_read;
        if length_read != 128 as libc::c_int as libc::c_longlong {
            debug(LOG_ERROR,
                  b"loadKeyMap: Reading %s short: %s\x00" as *const u8 as
                      *const libc::c_char, KeyMapPath.as_mut_ptr(),
                  PHYSFS_getLastError());
            PHYSFS_close(pfile);
            return 0 as libc::c_int
        }
        // name
        length_read =
            PHYSFS_read(pfile,
                        &mut status as *mut KEY_STATUS as *mut libc::c_void,
                        1 as libc::c_int as PHYSFS_uint32,
                        ::std::mem::size_of::<KEY_STATUS>() as libc::c_ulong);
        countsize += length_read;
        if length_read !=
               ::std::mem::size_of::<KEY_STATUS>() as libc::c_ulong as
                   libc::c_longlong {
            debug(LOG_ERROR,
                  b"loadKeyMap: Reading %s short: %s\x00" as *const u8 as
                      *const libc::c_char, KeyMapPath.as_mut_ptr(),
                  PHYSFS_getLastError());
            PHYSFS_close(pfile);
            return 0 as libc::c_int
        }
        // status
        length_read =
            PHYSFS_read(pfile,
                        &mut metaCode as *mut KEY_CODE as *mut libc::c_void,
                        1 as libc::c_int as PHYSFS_uint32,
                        ::std::mem::size_of::<KEY_CODE>() as libc::c_ulong);
        countsize += length_read;
        if length_read !=
               ::std::mem::size_of::<KEY_CODE>() as libc::c_ulong as
                   libc::c_longlong {
            debug(LOG_ERROR,
                  b"loadKeyMap: Reading %s short: %s\x00" as *const u8 as
                      *const libc::c_char, KeyMapPath.as_mut_ptr(),
                  PHYSFS_getLastError());
            PHYSFS_close(pfile);
            return 0 as libc::c_int
        }
        // metakey
        length_read =
            PHYSFS_read(pfile,
                        &mut subCode as *mut KEY_CODE as *mut libc::c_void,
                        1 as libc::c_int as PHYSFS_uint32,
                        ::std::mem::size_of::<KEY_CODE>() as libc::c_ulong);
        countsize += length_read;
        if length_read !=
               ::std::mem::size_of::<KEY_CODE>() as libc::c_ulong as
                   libc::c_longlong {
            debug(LOG_ERROR,
                  b"loadKeyMap: Reading %s short: %s\x00" as *const u8 as
                      *const libc::c_char, KeyMapPath.as_mut_ptr(),
                  PHYSFS_getLastError());
            PHYSFS_close(pfile);
            return 0 as libc::c_int
        }
        // subkey
        length_read =
            PHYSFS_read(pfile,
                        &mut action as *mut KEY_ACTION as *mut libc::c_void,
                        1 as libc::c_int as PHYSFS_uint32,
                        ::std::mem::size_of::<KEY_ACTION>() as libc::c_ulong);
        countsize += length_read;
        if length_read !=
               ::std::mem::size_of::<KEY_ACTION>() as libc::c_ulong as
                   libc::c_longlong {
            debug(LOG_ERROR,
                  b"loadKeyMap: Reading %s short: %s\x00" as *const u8 as
                      *const libc::c_char, KeyMapPath.as_mut_ptr(),
                  PHYSFS_getLastError());
            PHYSFS_close(pfile);
            return 0 as libc::c_int
        }
        // action
        length_read =
            PHYSFS_read(pfile,
                        &mut funcmap as *mut UDWORD as *mut libc::c_void,
                        1 as libc::c_int as PHYSFS_uint32,
                        ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
        countsize += length_read;
        if length_read !=
               ::std::mem::size_of::<UDWORD>() as libc::c_ulong as
                   libc::c_longlong {
            debug(LOG_ERROR,
                  b"loadKeyMap: Reading %s short: %s\x00" as *const u8 as
                      *const libc::c_char, KeyMapPath.as_mut_ptr(),
                  PHYSFS_getLastError());
            PHYSFS_close(pfile);
            return 0 as libc::c_int
        }
        // function
        // add mapping
        keyAddMapping(status, metaCode, subCode, action,
                      *keyMapSaveTable.as_mut_ptr().offset(funcmap as isize),
                      &mut name as *mut [STRING; 128] as *mut libc::c_char);
        count -= 1
    }
    if PHYSFS_close(pfile) == 0 {
        debug(LOG_ERROR,
              b"loadKeyMap: Error closing %s: %s\x00" as *const u8 as
                  *const libc::c_char, KeyMapPath.as_mut_ptr(),
              PHYSFS_getLastError());
        return 0 as libc::c_int
    }
    if countsize != filesize {
        debug(LOG_ERROR,
              b"loadKeyMap: File size different from bytes read!\x00" as
                  *const u8 as *const libc::c_char);
    }
    return 1 as libc::c_int;
}
