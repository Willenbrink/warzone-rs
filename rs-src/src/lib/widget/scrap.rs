use ::libc;
extern "C" {
    pub type _XDisplay;
    pub type _XGC;
    pub type _XPrivate;
    pub type _XrmHashBucketRec;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_uint) -> *mut libc::c_void;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_uint) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_uint)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_uint;
    #[no_mangle]
    fn SDL_SetError(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn SDL_EventState(type_0: Uint8, state: libc::c_int) -> Uint8;
    #[no_mangle]
    fn SDL_SetEventFilter(filter: SDL_EventFilter);
    #[no_mangle]
    fn SDL_WaitEvent(event: *mut SDL_Event) -> libc::c_int;
    #[no_mangle]
    fn XInternAtom(_: *mut Display, _: *const libc::c_char, _: libc::c_int)
     -> Atom;
    #[no_mangle]
    fn XGetSelectionOwner(_: *mut Display, _: Atom) -> Window;
    #[no_mangle]
    fn XChangeProperty(_: *mut Display, _: Window, _: Atom, _: Atom,
                       _: libc::c_int, _: libc::c_int,
                       _: *const libc::c_uchar, _: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn XConvertSelection(_: *mut Display, _: Atom, _: Atom, _: Atom,
                         _: Window, _: Time) -> libc::c_int;
    #[no_mangle]
    fn XFree(_: *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn XGetWindowProperty(_: *mut Display, _: Window, _: Atom,
                          _: libc::c_long, _: libc::c_long, _: libc::c_int,
                          _: Atom, _: *mut Atom, _: *mut libc::c_int,
                          _: *mut libc::c_ulong, _: *mut libc::c_ulong,
                          _: *mut *mut libc::c_uchar) -> libc::c_int;
    #[no_mangle]
    fn XSendEvent(_: *mut Display, _: Window, _: libc::c_int, _: libc::c_long,
                  _: *mut XEvent) -> libc::c_int;
    #[no_mangle]
    fn XSetSelectionOwner(_: *mut Display, _: Atom, _: Window, _: Time)
     -> libc::c_int;
    #[no_mangle]
    fn XSync(_: *mut Display, _: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn SDL_GetWMInfo(info: *mut SDL_SysWMinfo) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type int16_t = __int16_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type Uint8 = uint8_t;
pub type Sint16 = int16_t;
pub type Uint16 = uint16_t;
pub type SDLKey = libc::c_uint;
pub const SDLK_LAST: SDLKey = 323;
pub const SDLK_UNDO: SDLKey = 322;
pub const SDLK_EURO: SDLKey = 321;
pub const SDLK_POWER: SDLKey = 320;
pub const SDLK_MENU: SDLKey = 319;
pub const SDLK_BREAK: SDLKey = 318;
pub const SDLK_SYSREQ: SDLKey = 317;
pub const SDLK_PRINT: SDLKey = 316;
pub const SDLK_HELP: SDLKey = 315;
pub const SDLK_COMPOSE: SDLKey = 314;
pub const SDLK_MODE: SDLKey = 313;
pub const SDLK_RSUPER: SDLKey = 312;
pub const SDLK_LSUPER: SDLKey = 311;
pub const SDLK_LMETA: SDLKey = 310;
pub const SDLK_RMETA: SDLKey = 309;
pub const SDLK_LALT: SDLKey = 308;
pub const SDLK_RALT: SDLKey = 307;
pub const SDLK_LCTRL: SDLKey = 306;
pub const SDLK_RCTRL: SDLKey = 305;
pub const SDLK_LSHIFT: SDLKey = 304;
pub const SDLK_RSHIFT: SDLKey = 303;
pub const SDLK_SCROLLOCK: SDLKey = 302;
pub const SDLK_CAPSLOCK: SDLKey = 301;
pub const SDLK_NUMLOCK: SDLKey = 300;
pub const SDLK_F15: SDLKey = 296;
pub const SDLK_F14: SDLKey = 295;
pub const SDLK_F13: SDLKey = 294;
pub const SDLK_F12: SDLKey = 293;
pub const SDLK_F11: SDLKey = 292;
pub const SDLK_F10: SDLKey = 291;
pub const SDLK_F9: SDLKey = 290;
pub const SDLK_F8: SDLKey = 289;
pub const SDLK_F7: SDLKey = 288;
pub const SDLK_F6: SDLKey = 287;
pub const SDLK_F5: SDLKey = 286;
pub const SDLK_F4: SDLKey = 285;
pub const SDLK_F3: SDLKey = 284;
pub const SDLK_F2: SDLKey = 283;
pub const SDLK_F1: SDLKey = 282;
pub const SDLK_PAGEDOWN: SDLKey = 281;
pub const SDLK_PAGEUP: SDLKey = 280;
pub const SDLK_END: SDLKey = 279;
pub const SDLK_HOME: SDLKey = 278;
pub const SDLK_INSERT: SDLKey = 277;
pub const SDLK_LEFT: SDLKey = 276;
pub const SDLK_RIGHT: SDLKey = 275;
pub const SDLK_DOWN: SDLKey = 274;
pub const SDLK_UP: SDLKey = 273;
pub const SDLK_KP_EQUALS: SDLKey = 272;
pub const SDLK_KP_ENTER: SDLKey = 271;
pub const SDLK_KP_PLUS: SDLKey = 270;
pub const SDLK_KP_MINUS: SDLKey = 269;
pub const SDLK_KP_MULTIPLY: SDLKey = 268;
pub const SDLK_KP_DIVIDE: SDLKey = 267;
pub const SDLK_KP_PERIOD: SDLKey = 266;
pub const SDLK_KP9: SDLKey = 265;
pub const SDLK_KP8: SDLKey = 264;
pub const SDLK_KP7: SDLKey = 263;
pub const SDLK_KP6: SDLKey = 262;
pub const SDLK_KP5: SDLKey = 261;
pub const SDLK_KP4: SDLKey = 260;
pub const SDLK_KP3: SDLKey = 259;
pub const SDLK_KP2: SDLKey = 258;
pub const SDLK_KP1: SDLKey = 257;
pub const SDLK_KP0: SDLKey = 256;
pub const SDLK_WORLD_95: SDLKey = 255;
pub const SDLK_WORLD_94: SDLKey = 254;
pub const SDLK_WORLD_93: SDLKey = 253;
pub const SDLK_WORLD_92: SDLKey = 252;
pub const SDLK_WORLD_91: SDLKey = 251;
pub const SDLK_WORLD_90: SDLKey = 250;
pub const SDLK_WORLD_89: SDLKey = 249;
pub const SDLK_WORLD_88: SDLKey = 248;
pub const SDLK_WORLD_87: SDLKey = 247;
pub const SDLK_WORLD_86: SDLKey = 246;
pub const SDLK_WORLD_85: SDLKey = 245;
pub const SDLK_WORLD_84: SDLKey = 244;
pub const SDLK_WORLD_83: SDLKey = 243;
pub const SDLK_WORLD_82: SDLKey = 242;
pub const SDLK_WORLD_81: SDLKey = 241;
pub const SDLK_WORLD_80: SDLKey = 240;
pub const SDLK_WORLD_79: SDLKey = 239;
pub const SDLK_WORLD_78: SDLKey = 238;
pub const SDLK_WORLD_77: SDLKey = 237;
pub const SDLK_WORLD_76: SDLKey = 236;
pub const SDLK_WORLD_75: SDLKey = 235;
pub const SDLK_WORLD_74: SDLKey = 234;
pub const SDLK_WORLD_73: SDLKey = 233;
pub const SDLK_WORLD_72: SDLKey = 232;
pub const SDLK_WORLD_71: SDLKey = 231;
pub const SDLK_WORLD_70: SDLKey = 230;
pub const SDLK_WORLD_69: SDLKey = 229;
pub const SDLK_WORLD_68: SDLKey = 228;
pub const SDLK_WORLD_67: SDLKey = 227;
pub const SDLK_WORLD_66: SDLKey = 226;
pub const SDLK_WORLD_65: SDLKey = 225;
pub const SDLK_WORLD_64: SDLKey = 224;
pub const SDLK_WORLD_63: SDLKey = 223;
pub const SDLK_WORLD_62: SDLKey = 222;
pub const SDLK_WORLD_61: SDLKey = 221;
pub const SDLK_WORLD_60: SDLKey = 220;
pub const SDLK_WORLD_59: SDLKey = 219;
pub const SDLK_WORLD_58: SDLKey = 218;
pub const SDLK_WORLD_57: SDLKey = 217;
pub const SDLK_WORLD_56: SDLKey = 216;
pub const SDLK_WORLD_55: SDLKey = 215;
pub const SDLK_WORLD_54: SDLKey = 214;
pub const SDLK_WORLD_53: SDLKey = 213;
pub const SDLK_WORLD_52: SDLKey = 212;
pub const SDLK_WORLD_51: SDLKey = 211;
pub const SDLK_WORLD_50: SDLKey = 210;
pub const SDLK_WORLD_49: SDLKey = 209;
pub const SDLK_WORLD_48: SDLKey = 208;
pub const SDLK_WORLD_47: SDLKey = 207;
pub const SDLK_WORLD_46: SDLKey = 206;
pub const SDLK_WORLD_45: SDLKey = 205;
pub const SDLK_WORLD_44: SDLKey = 204;
pub const SDLK_WORLD_43: SDLKey = 203;
pub const SDLK_WORLD_42: SDLKey = 202;
pub const SDLK_WORLD_41: SDLKey = 201;
pub const SDLK_WORLD_40: SDLKey = 200;
pub const SDLK_WORLD_39: SDLKey = 199;
pub const SDLK_WORLD_38: SDLKey = 198;
pub const SDLK_WORLD_37: SDLKey = 197;
pub const SDLK_WORLD_36: SDLKey = 196;
pub const SDLK_WORLD_35: SDLKey = 195;
pub const SDLK_WORLD_34: SDLKey = 194;
pub const SDLK_WORLD_33: SDLKey = 193;
pub const SDLK_WORLD_32: SDLKey = 192;
pub const SDLK_WORLD_31: SDLKey = 191;
pub const SDLK_WORLD_30: SDLKey = 190;
pub const SDLK_WORLD_29: SDLKey = 189;
pub const SDLK_WORLD_28: SDLKey = 188;
pub const SDLK_WORLD_27: SDLKey = 187;
pub const SDLK_WORLD_26: SDLKey = 186;
pub const SDLK_WORLD_25: SDLKey = 185;
pub const SDLK_WORLD_24: SDLKey = 184;
pub const SDLK_WORLD_23: SDLKey = 183;
pub const SDLK_WORLD_22: SDLKey = 182;
pub const SDLK_WORLD_21: SDLKey = 181;
pub const SDLK_WORLD_20: SDLKey = 180;
pub const SDLK_WORLD_19: SDLKey = 179;
pub const SDLK_WORLD_18: SDLKey = 178;
pub const SDLK_WORLD_17: SDLKey = 177;
pub const SDLK_WORLD_16: SDLKey = 176;
pub const SDLK_WORLD_15: SDLKey = 175;
pub const SDLK_WORLD_14: SDLKey = 174;
pub const SDLK_WORLD_13: SDLKey = 173;
pub const SDLK_WORLD_12: SDLKey = 172;
pub const SDLK_WORLD_11: SDLKey = 171;
pub const SDLK_WORLD_10: SDLKey = 170;
pub const SDLK_WORLD_9: SDLKey = 169;
pub const SDLK_WORLD_8: SDLKey = 168;
pub const SDLK_WORLD_7: SDLKey = 167;
pub const SDLK_WORLD_6: SDLKey = 166;
pub const SDLK_WORLD_5: SDLKey = 165;
pub const SDLK_WORLD_4: SDLKey = 164;
pub const SDLK_WORLD_3: SDLKey = 163;
pub const SDLK_WORLD_2: SDLKey = 162;
pub const SDLK_WORLD_1: SDLKey = 161;
pub const SDLK_WORLD_0: SDLKey = 160;
pub const SDLK_DELETE: SDLKey = 127;
pub const SDLK_z: SDLKey = 122;
pub const SDLK_y: SDLKey = 121;
pub const SDLK_x: SDLKey = 120;
pub const SDLK_w: SDLKey = 119;
pub const SDLK_v: SDLKey = 118;
pub const SDLK_u: SDLKey = 117;
pub const SDLK_t: SDLKey = 116;
pub const SDLK_s: SDLKey = 115;
pub const SDLK_r: SDLKey = 114;
pub const SDLK_q: SDLKey = 113;
pub const SDLK_p: SDLKey = 112;
pub const SDLK_o: SDLKey = 111;
pub const SDLK_n: SDLKey = 110;
pub const SDLK_m: SDLKey = 109;
pub const SDLK_l: SDLKey = 108;
pub const SDLK_k: SDLKey = 107;
pub const SDLK_j: SDLKey = 106;
pub const SDLK_i: SDLKey = 105;
pub const SDLK_h: SDLKey = 104;
pub const SDLK_g: SDLKey = 103;
pub const SDLK_f: SDLKey = 102;
pub const SDLK_e: SDLKey = 101;
pub const SDLK_d: SDLKey = 100;
pub const SDLK_c: SDLKey = 99;
pub const SDLK_b: SDLKey = 98;
pub const SDLK_a: SDLKey = 97;
pub const SDLK_BACKQUOTE: SDLKey = 96;
pub const SDLK_UNDERSCORE: SDLKey = 95;
pub const SDLK_CARET: SDLKey = 94;
pub const SDLK_RIGHTBRACKET: SDLKey = 93;
pub const SDLK_BACKSLASH: SDLKey = 92;
pub const SDLK_LEFTBRACKET: SDLKey = 91;
pub const SDLK_AT: SDLKey = 64;
pub const SDLK_QUESTION: SDLKey = 63;
pub const SDLK_GREATER: SDLKey = 62;
pub const SDLK_EQUALS: SDLKey = 61;
pub const SDLK_LESS: SDLKey = 60;
pub const SDLK_SEMICOLON: SDLKey = 59;
pub const SDLK_COLON: SDLKey = 58;
pub const SDLK_9: SDLKey = 57;
pub const SDLK_8: SDLKey = 56;
pub const SDLK_7: SDLKey = 55;
pub const SDLK_6: SDLKey = 54;
pub const SDLK_5: SDLKey = 53;
pub const SDLK_4: SDLKey = 52;
pub const SDLK_3: SDLKey = 51;
pub const SDLK_2: SDLKey = 50;
pub const SDLK_1: SDLKey = 49;
pub const SDLK_0: SDLKey = 48;
pub const SDLK_SLASH: SDLKey = 47;
pub const SDLK_PERIOD: SDLKey = 46;
pub const SDLK_MINUS: SDLKey = 45;
pub const SDLK_COMMA: SDLKey = 44;
pub const SDLK_PLUS: SDLKey = 43;
pub const SDLK_ASTERISK: SDLKey = 42;
pub const SDLK_RIGHTPAREN: SDLKey = 41;
pub const SDLK_LEFTPAREN: SDLKey = 40;
pub const SDLK_QUOTE: SDLKey = 39;
pub const SDLK_AMPERSAND: SDLKey = 38;
pub const SDLK_DOLLAR: SDLKey = 36;
pub const SDLK_HASH: SDLKey = 35;
pub const SDLK_QUOTEDBL: SDLKey = 34;
pub const SDLK_EXCLAIM: SDLKey = 33;
pub const SDLK_SPACE: SDLKey = 32;
pub const SDLK_ESCAPE: SDLKey = 27;
pub const SDLK_PAUSE: SDLKey = 19;
pub const SDLK_RETURN: SDLKey = 13;
pub const SDLK_CLEAR: SDLKey = 12;
pub const SDLK_TAB: SDLKey = 9;
pub const SDLK_BACKSPACE: SDLKey = 8;
pub const SDLK_FIRST: SDLKey = 0;
pub const SDLK_UNKNOWN: SDLKey = 0;
pub type SDLMod = libc::c_uint;
pub const KMOD_RESERVED: SDLMod = 32768;
pub const KMOD_MODE: SDLMod = 16384;
pub const KMOD_CAPS: SDLMod = 8192;
pub const KMOD_NUM: SDLMod = 4096;
pub const KMOD_RMETA: SDLMod = 2048;
pub const KMOD_LMETA: SDLMod = 1024;
pub const KMOD_RALT: SDLMod = 512;
pub const KMOD_LALT: SDLMod = 256;
pub const KMOD_RCTRL: SDLMod = 128;
pub const KMOD_LCTRL: SDLMod = 64;
pub const KMOD_RSHIFT: SDLMod = 2;
pub const KMOD_LSHIFT: SDLMod = 1;
pub const KMOD_NONE: SDLMod = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_keysym {
    pub scancode: Uint8,
    pub sym: SDLKey,
    pub mod_0: SDLMod,
    pub unicode: Uint16,
}
pub type C2RustUnnamed = libc::c_uint;
pub const SDL_NUMEVENTS: C2RustUnnamed = 32;
pub const SDL_USEREVENT: C2RustUnnamed = 24;
pub const SDL_EVENT_RESERVED7: C2RustUnnamed = 23;
pub const SDL_EVENT_RESERVED6: C2RustUnnamed = 22;
pub const SDL_EVENT_RESERVED5: C2RustUnnamed = 21;
pub const SDL_EVENT_RESERVED4: C2RustUnnamed = 20;
pub const SDL_EVENT_RESERVED3: C2RustUnnamed = 19;
pub const SDL_EVENT_RESERVED2: C2RustUnnamed = 18;
pub const SDL_VIDEOEXPOSE: C2RustUnnamed = 17;
pub const SDL_VIDEORESIZE: C2RustUnnamed = 16;
pub const SDL_EVENT_RESERVEDB: C2RustUnnamed = 15;
pub const SDL_EVENT_RESERVEDA: C2RustUnnamed = 14;
pub const SDL_SYSWMEVENT: C2RustUnnamed = 13;
pub const SDL_QUIT: C2RustUnnamed = 12;
pub const SDL_JOYBUTTONUP: C2RustUnnamed = 11;
pub const SDL_JOYBUTTONDOWN: C2RustUnnamed = 10;
pub const SDL_JOYHATMOTION: C2RustUnnamed = 9;
pub const SDL_JOYBALLMOTION: C2RustUnnamed = 8;
pub const SDL_JOYAXISMOTION: C2RustUnnamed = 7;
pub const SDL_MOUSEBUTTONUP: C2RustUnnamed = 6;
pub const SDL_MOUSEBUTTONDOWN: C2RustUnnamed = 5;
pub const SDL_MOUSEMOTION: C2RustUnnamed = 4;
pub const SDL_KEYUP: C2RustUnnamed = 3;
pub const SDL_KEYDOWN: C2RustUnnamed = 2;
pub const SDL_ACTIVEEVENT: C2RustUnnamed = 1;
pub const SDL_NOEVENT: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_ActiveEvent {
    pub type_0: Uint8,
    pub gain: Uint8,
    pub state: Uint8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_KeyboardEvent {
    pub type_0: Uint8,
    pub which: Uint8,
    pub state: Uint8,
    pub keysym: SDL_keysym,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_MouseMotionEvent {
    pub type_0: Uint8,
    pub which: Uint8,
    pub state: Uint8,
    pub x: Uint16,
    pub y: Uint16,
    pub xrel: Sint16,
    pub yrel: Sint16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_MouseButtonEvent {
    pub type_0: Uint8,
    pub which: Uint8,
    pub button: Uint8,
    pub state: Uint8,
    pub x: Uint16,
    pub y: Uint16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_JoyAxisEvent {
    pub type_0: Uint8,
    pub which: Uint8,
    pub axis: Uint8,
    pub value: Sint16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_JoyBallEvent {
    pub type_0: Uint8,
    pub which: Uint8,
    pub ball: Uint8,
    pub xrel: Sint16,
    pub yrel: Sint16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_JoyHatEvent {
    pub type_0: Uint8,
    pub which: Uint8,
    pub hat: Uint8,
    pub value: Uint8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_JoyButtonEvent {
    pub type_0: Uint8,
    pub which: Uint8,
    pub button: Uint8,
    pub state: Uint8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_ResizeEvent {
    pub type_0: Uint8,
    pub w: libc::c_int,
    pub h: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_ExposeEvent {
    pub type_0: Uint8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_QuitEvent {
    pub type_0: Uint8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_UserEvent {
    pub type_0: Uint8,
    pub code: libc::c_int,
    pub data1: *mut libc::c_void,
    pub data2: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_SysWMmsg {
    pub version: SDL_version,
    pub subsystem: SDL_SYSWM_TYPE,
    pub event: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub xevent: XEvent,
}
pub type XEvent = _XEvent;
#[derive(Copy, Clone)]
#[repr(C)]
pub union _XEvent {
    pub type_0: libc::c_int,
    pub xany: XAnyEvent,
    pub xkey: XKeyEvent,
    pub xbutton: XButtonEvent,
    pub xmotion: XMotionEvent,
    pub xcrossing: XCrossingEvent,
    pub xfocus: XFocusChangeEvent,
    pub xexpose: XExposeEvent,
    pub xgraphicsexpose: XGraphicsExposeEvent,
    pub xnoexpose: XNoExposeEvent,
    pub xvisibility: XVisibilityEvent,
    pub xcreatewindow: XCreateWindowEvent,
    pub xdestroywindow: XDestroyWindowEvent,
    pub xunmap: XUnmapEvent,
    pub xmap: XMapEvent,
    pub xmaprequest: XMapRequestEvent,
    pub xreparent: XReparentEvent,
    pub xconfigure: XConfigureEvent,
    pub xgravity: XGravityEvent,
    pub xresizerequest: XResizeRequestEvent,
    pub xconfigurerequest: XConfigureRequestEvent,
    pub xcirculate: XCirculateEvent,
    pub xcirculaterequest: XCirculateRequestEvent,
    pub xproperty: XPropertyEvent,
    pub xselectionclear: XSelectionClearEvent,
    pub xselectionrequest: XSelectionRequestEvent,
    pub xselection: XSelectionEvent,
    pub xcolormap: XColormapEvent,
    pub xclient: XClientMessageEvent,
    pub xmapping: XMappingEvent,
    pub xerror: XErrorEvent,
    pub xkeymap: XKeymapEvent,
    pub xgeneric: XGenericEvent,
    pub xcookie: XGenericEventCookie,
    pub pad: [libc::c_long; 24],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGenericEventCookie {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub extension: libc::c_int,
    pub evtype: libc::c_int,
    pub cookie: libc::c_uint,
    pub data: *mut libc::c_void,
}
pub type Display = _XDisplay;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGenericEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub extension: libc::c_int,
    pub evtype: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XKeymapEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub key_vector: [libc::c_char; 32],
}
pub type Window = XID;
pub type XID = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XErrorEvent {
    pub type_0: libc::c_int,
    pub display: *mut Display,
    pub resourceid: XID,
    pub serial: libc::c_ulong,
    pub error_code: libc::c_uchar,
    pub request_code: libc::c_uchar,
    pub minor_code: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMappingEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub request: libc::c_int,
    pub first_keycode: libc::c_int,
    pub count: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XClientMessageEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub message_type: Atom,
    pub format: libc::c_int,
    pub data: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub b: [libc::c_char; 20],
    pub s: [libc::c_short; 10],
    pub l: [libc::c_long; 5],
}
pub type Atom = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XColormapEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub colormap: Colormap,
    pub new: libc::c_int,
    pub state: libc::c_int,
}
pub type Colormap = XID;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSelectionEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub requestor: Window,
    pub selection: Atom,
    pub target: Atom,
    pub property: Atom,
    pub time: Time,
}
pub type Time = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSelectionRequestEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub owner: Window,
    pub requestor: Window,
    pub selection: Atom,
    pub target: Atom,
    pub property: Atom,
    pub time: Time,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSelectionClearEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub selection: Atom,
    pub time: Time,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XPropertyEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub atom: Atom,
    pub time: Time,
    pub state: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCirculateRequestEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub place: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCirculateEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub place: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XConfigureRequestEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub border_width: libc::c_int,
    pub above: Window,
    pub detail: libc::c_int,
    pub value_mask: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XResizeRequestEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub width: libc::c_int,
    pub height: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGravityEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XConfigureEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub border_width: libc::c_int,
    pub above: Window,
    pub override_redirect: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XReparentEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub parent: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub override_redirect: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMapRequestEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMapEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub override_redirect: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XUnmapEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub from_configure: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XDestroyWindowEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCreateWindowEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub border_width: libc::c_int,
    pub override_redirect: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XVisibilityEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub state: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XNoExposeEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub drawable: Drawable,
    pub major_code: libc::c_int,
    pub minor_code: libc::c_int,
}
pub type Drawable = XID;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGraphicsExposeEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub drawable: Drawable,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub count: libc::c_int,
    pub major_code: libc::c_int,
    pub minor_code: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XExposeEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub count: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XFocusChangeEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub mode: libc::c_int,
    pub detail: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCrossingEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub x_root: libc::c_int,
    pub y_root: libc::c_int,
    pub mode: libc::c_int,
    pub detail: libc::c_int,
    pub same_screen: libc::c_int,
    pub focus: libc::c_int,
    pub state: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMotionEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub x_root: libc::c_int,
    pub y_root: libc::c_int,
    pub state: libc::c_uint,
    pub is_hint: libc::c_char,
    pub same_screen: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XButtonEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub x_root: libc::c_int,
    pub y_root: libc::c_int,
    pub state: libc::c_uint,
    pub button: libc::c_uint,
    pub same_screen: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XKeyEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub x_root: libc::c_int,
    pub y_root: libc::c_int,
    pub state: libc::c_uint,
    pub keycode: libc::c_uint,
    pub same_screen: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XAnyEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
}
pub type SDL_SYSWM_TYPE = libc::c_uint;
pub const SDL_SYSWM_X11: SDL_SYSWM_TYPE = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_version {
    pub major: Uint8,
    pub minor: Uint8,
    pub patch: Uint8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_SysWMEvent {
    pub type_0: Uint8,
    pub msg: *mut SDL_SysWMmsg,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union SDL_Event {
    pub type_0: Uint8,
    pub active: SDL_ActiveEvent,
    pub key: SDL_KeyboardEvent,
    pub motion: SDL_MouseMotionEvent,
    pub button: SDL_MouseButtonEvent,
    pub jaxis: SDL_JoyAxisEvent,
    pub jball: SDL_JoyBallEvent,
    pub jhat: SDL_JoyHatEvent,
    pub jbutton: SDL_JoyButtonEvent,
    pub resize: SDL_ResizeEvent,
    pub expose: SDL_ExposeEvent,
    pub quit: SDL_QuitEvent,
    pub user: SDL_UserEvent,
    pub syswm: SDL_SysWMEvent,
}
pub type SDL_EventFilter
    =
    Option<unsafe extern "C" fn(_: *const SDL_Event) -> libc::c_int>;
pub type VisualID = libc::c_ulong;
pub type XPointer = *mut libc::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XExtData {
    pub number: libc::c_int,
    pub next: *mut _XExtData,
    pub free_private: Option<unsafe extern "C" fn(_: *mut _XExtData)
                                 -> libc::c_int>,
    pub private_data: XPointer,
}
pub type XExtData = _XExtData;
pub type GC = *mut _XGC;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Visual {
    pub ext_data: *mut XExtData,
    pub visualid: VisualID,
    pub class: libc::c_int,
    pub red_mask: libc::c_ulong,
    pub green_mask: libc::c_ulong,
    pub blue_mask: libc::c_ulong,
    pub bits_per_rgb: libc::c_int,
    pub map_entries: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Depth {
    pub depth: libc::c_int,
    pub nvisuals: libc::c_int,
    pub visuals: *mut Visual,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Screen {
    pub ext_data: *mut XExtData,
    pub display: *mut _XDisplay,
    pub root: Window,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub mwidth: libc::c_int,
    pub mheight: libc::c_int,
    pub ndepths: libc::c_int,
    pub depths: *mut Depth,
    pub root_depth: libc::c_int,
    pub root_visual: *mut Visual,
    pub default_gc: GC,
    pub cmap: Colormap,
    pub white_pixel: libc::c_ulong,
    pub black_pixel: libc::c_ulong,
    pub max_maps: libc::c_int,
    pub min_maps: libc::c_int,
    pub backing_store: libc::c_int,
    pub save_unders: libc::c_int,
    pub root_input_mask: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ScreenFormat {
    pub ext_data: *mut XExtData,
    pub depth: libc::c_int,
    pub bits_per_pixel: libc::c_int,
    pub scanline_pad: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub ext_data: *mut XExtData,
    pub private1: *mut _XPrivate,
    pub fd: libc::c_int,
    pub private2: libc::c_int,
    pub proto_major_version: libc::c_int,
    pub proto_minor_version: libc::c_int,
    pub vendor: *mut libc::c_char,
    pub private3: XID,
    pub private4: XID,
    pub private5: XID,
    pub private6: libc::c_int,
    pub resource_alloc: Option<unsafe extern "C" fn(_: *mut _XDisplay)
                                   -> XID>,
    pub byte_order: libc::c_int,
    pub bitmap_unit: libc::c_int,
    pub bitmap_pad: libc::c_int,
    pub bitmap_bit_order: libc::c_int,
    pub nformats: libc::c_int,
    pub pixmap_format: *mut ScreenFormat,
    pub private8: libc::c_int,
    pub release: libc::c_int,
    pub private9: *mut _XPrivate,
    pub private10: *mut _XPrivate,
    pub qlen: libc::c_int,
    pub last_request_read: libc::c_ulong,
    pub request: libc::c_ulong,
    pub private11: XPointer,
    pub private12: XPointer,
    pub private13: XPointer,
    pub private14: XPointer,
    pub max_request_size: libc::c_uint,
    pub db: *mut _XrmHashBucketRec,
    pub private15: Option<unsafe extern "C" fn(_: *mut _XDisplay)
                              -> libc::c_int>,
    pub display_name: *mut libc::c_char,
    pub default_screen: libc::c_int,
    pub nscreens: libc::c_int,
    pub screens: *mut Screen,
    pub motion_buffer: libc::c_ulong,
    pub private16: libc::c_ulong,
    pub min_keycode: libc::c_int,
    pub max_keycode: libc::c_int,
    pub private17: XPointer,
    pub private18: XPointer,
    pub private19: libc::c_int,
    pub xdefaults: *mut libc::c_char,
}
pub type _XPrivDisplay = *mut C2RustUnnamed_2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_SysWMinfo {
    pub version: SDL_version,
    pub subsystem: SDL_SYSWM_TYPE,
    pub info: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub x11: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub display: *mut Display,
    pub window: Window,
    pub lock_func: Option<unsafe extern "C" fn() -> ()>,
    pub unlock_func: Option<unsafe extern "C" fn() -> ()>,
    pub fswindow: Window,
    pub wmwindow: Window,
    pub gfxdisplay: *mut Display,
}
/* Handle clipboard text and data in arbitrary formats */
/* Miscellaneous defines */
/* Determine what type of clipboard we are using */
/* scrap type */
/* System dependent data types */
/* * */
pub type scrap_type = Atom;
/* scrap type */
/* System dependent variables */
/* * */
static mut SDL_Display: *mut Display = 0 as *const Display as *mut Display;
static mut SDL_Window: Window = 0;
static mut Lock_Display: Option<unsafe extern "C" fn() -> ()> = None;
static mut Unlock_Display: Option<unsafe extern "C" fn() -> ()> = None;
unsafe extern "C" fn convert_format(mut type_0: libc::c_int) -> scrap_type {
    match type_0 {
        1413830740 => {
            /* * */
            return 31 as libc::c_int as Atom
        }
        _ => {
            /* scrap type */
            let mut format: [libc::c_char; 22] = [0; 22];
            sprintf(format.as_mut_ptr(),
                    b"%s%08lx\x00" as *const u8 as *const libc::c_char,
                    b"SDL_scrap_0x\x00" as *const u8 as *const libc::c_char,
                    type_0 as libc::c_ulong);
            /* scrap type */
            return XInternAtom(SDL_Display, format.as_mut_ptr(),
                               0 as libc::c_int)
        }
    };
}
/* * */
/* Convert internal data to scrap format */
unsafe extern "C" fn convert_data(mut type_0: libc::c_int,
                                  mut dst: *mut libc::c_char,
                                  mut src: *mut libc::c_char,
                                  mut srclen: libc::c_int) -> libc::c_int {
    let mut dstlen: libc::c_int = 0;
    dstlen = 0 as libc::c_int;
    match type_0 {
        1413830740 => {
            if !dst.is_null() {
                loop  {
                    srclen -= 1;
                    if !(srclen >= 0 as libc::c_int) { break ; }
                    if *src as libc::c_int == '\r' as i32 {
                        let fresh0 = dst;
                        dst = dst.offset(1);
                        *fresh0 = '\n' as i32 as libc::c_char;
                        dstlen += 1
                    } else {
                        let fresh1 = dst;
                        dst = dst.offset(1);
                        *fresh1 = *src;
                        dstlen += 1
                    }
                    src = src.offset(1)
                }
                *dst = '\u{0}' as i32 as libc::c_char;
                dstlen += 1
            } else {
                loop  {
                    srclen -= 1;
                    if !(srclen >= 0 as libc::c_int) { break ; }
                    if *src as libc::c_int == '\r' as i32 {
                        dstlen += 1
                    } else { dstlen += 1 }
                    src = src.offset(1)
                }
                dstlen += 1
            }
        }
        _ => {
            if !dst.is_null() {
                *(dst as *mut libc::c_int) = srclen;
                dst =
                    dst.offset(::std::mem::size_of::<libc::c_int>() as
                                   libc::c_ulong as isize);
                memcpy(dst as *mut libc::c_void, src as *const libc::c_void,
                       srclen as libc::c_uint);
            }
            dstlen =
                (::std::mem::size_of::<libc::c_int>() as
                     libc::c_ulong).wrapping_add(srclen as libc::c_uint) as
                    libc::c_int
        }
    }
    return dstlen;
}
/* Convert scrap data to internal format */
unsafe extern "C" fn convert_scrap(mut type_0: libc::c_int,
                                   mut dst: *mut libc::c_char,
                                   mut src: *mut libc::c_char,
                                   mut srclen: libc::c_int) -> libc::c_int {
    let mut dstlen: libc::c_int = 0;
    dstlen = 0 as libc::c_int;
    match type_0 {
        1413830740 => {
            if srclen == 0 as libc::c_int {
                srclen = strlen(src) as libc::c_int
            }
            if !dst.is_null() {
                loop  {
                    srclen -= 1;
                    if !(srclen >= 0 as libc::c_int) { break ; }
                    if *src as libc::c_int == '\n' as i32 {
                        let fresh2 = dst;
                        dst = dst.offset(1);
                        *fresh2 = '\r' as i32 as libc::c_char;
                        dstlen += 1
                    } else {
                        let fresh3 = dst;
                        dst = dst.offset(1);
                        *fresh3 = *src;
                        dstlen += 1
                    }
                    src = src.offset(1)
                }
                *dst = '\u{0}' as i32 as libc::c_char;
                dstlen += 1
            } else {
                loop  {
                    srclen -= 1;
                    if !(srclen >= 0 as libc::c_int) { break ; }
                    dstlen += 1;
                    src = src.offset(1)
                }
                dstlen += 1
            }
        }
        _ => {
            dstlen = *(src as *mut libc::c_int);
            if !dst.is_null() {
                if srclen == 0 as libc::c_int {
                    memcpy(dst as *mut libc::c_void,
                           src.offset(::std::mem::size_of::<libc::c_int>() as
                                          libc::c_ulong as isize) as
                               *const libc::c_void, dstlen as libc::c_uint);
                } else {
                    memcpy(dst as *mut libc::c_void,
                           src.offset(::std::mem::size_of::<libc::c_int>() as
                                          libc::c_ulong as isize) as
                               *const libc::c_void,
                           (srclen as
                                libc::c_uint).wrapping_sub(::std::mem::size_of::<libc::c_int>()
                                                               as
                                                               libc::c_ulong));
                }
            }
        }
    }
    return dstlen;
}
#[no_mangle]
pub unsafe extern "C" fn init_scrap() -> libc::c_int {
    let mut info: SDL_SysWMinfo =
        SDL_SysWMinfo{version: SDL_version{major: 0, minor: 0, patch: 0,},
                      subsystem: SDL_SYSWM_X11,
                      info:
                          C2RustUnnamed_3{x11:
                                              C2RustUnnamed_4{display:
                                                                  0 as
                                                                      *mut Display,
                                                              window: 0,
                                                              lock_func: None,
                                                              unlock_func:
                                                                  None,
                                                              fswindow: 0,
                                                              wmwindow: 0,
                                                              gfxdisplay:
                                                                  0 as
                                                                      *mut Display,},},};
    let mut retval: libc::c_int = 0;
    /* Grab the window manager specific information */
    retval = -(1 as libc::c_int);
    SDL_SetError(b"SDL is not running on known window manager\x00" as
                     *const u8 as *const libc::c_char);
    info.version.major = 1 as libc::c_int as Uint8;
    info.version.minor = 2 as libc::c_int as Uint8;
    info.version.patch = 15 as libc::c_int as Uint8;
    if SDL_GetWMInfo(&mut info) != 0 {
        /* Save the information for later use */
        /* * */
        if info.subsystem as libc::c_uint ==
               SDL_SYSWM_X11 as libc::c_int as libc::c_uint {
            SDL_Display = info.info.x11.display;
            SDL_Window = info.info.x11.window;
            Lock_Display = info.info.x11.lock_func;
            Unlock_Display = info.info.x11.unlock_func;
            /* Enable the special window hook events */
            SDL_EventState(SDL_SYSWMEVENT as libc::c_int as Uint8,
                           1 as libc::c_int);
            SDL_SetEventFilter(Some(clipboard_filter as
                                        unsafe extern "C" fn(_:
                                                                 *const SDL_Event)
                                            -> libc::c_int));
            retval = 0 as libc::c_int
        } else {
            SDL_SetError(b"SDL is not running on X11\x00" as *const u8 as
                             *const libc::c_char);
        }
        /* scrap type */
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn lost_scrap() -> libc::c_int {
    let mut retval: libc::c_int = 0;
    /* * */
    Lock_Display.expect("non-null function pointer")();
    retval =
        (XGetSelectionOwner(SDL_Display, 1 as libc::c_int as Atom) !=
             SDL_Window) as libc::c_int;
    Unlock_Display.expect("non-null function pointer")();
    /* scrap type */
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn put_scrap(mut type_0: libc::c_int,
                                   mut srclen: libc::c_int,
                                   mut src: *mut libc::c_char) {
    let mut format: scrap_type = 0;
    let mut dstlen: libc::c_int = 0;
    let mut dst: *mut libc::c_char = 0 as *mut libc::c_char;
    format = convert_format(type_0);
    dstlen = convert_data(type_0, 0 as *mut libc::c_char, src, srclen);
    /* * */
    dst = malloc(dstlen as libc::c_uint) as *mut libc::c_char;
    if !dst.is_null() {
        Lock_Display.expect("non-null function pointer")();
        convert_data(type_0, dst, src, srclen);
        XChangeProperty(SDL_Display,
                        (*(*(SDL_Display as
                                 _XPrivDisplay)).screens.offset((*(SDL_Display
                                                                       as
                                                                       _XPrivDisplay)).default_screen
                                                                    as
                                                                    isize)).root,
                        9 as libc::c_int as Atom, format, 8 as libc::c_int,
                        0 as libc::c_int, dst as *const libc::c_uchar,
                        dstlen);
        free(dst as *mut libc::c_void);
        if lost_scrap() != 0 {
            XSetSelectionOwner(SDL_Display, 1 as libc::c_int as Atom,
                               SDL_Window, 0 as libc::c_long as Time);
        }
        Unlock_Display.expect("non-null function pointer")();
    };
    /* scrap type */
}
#[no_mangle]
pub unsafe extern "C" fn get_scrap(mut type_0: libc::c_int,
                                   mut dstlen: *mut libc::c_int,
                                   mut dst: *mut *mut libc::c_char) {
    let mut format: scrap_type = 0;
    *dstlen = 0 as libc::c_int;
    format = convert_format(type_0);
    /* * */
    let mut owner: Window = 0;
    let mut selection: Atom = 0;
    let mut seln_type: Atom = 0;
    let mut seln_format: libc::c_int = 0;
    let mut nbytes: libc::c_ulong = 0;
    let mut overflow: libc::c_ulong = 0;
    let mut src: *mut libc::c_char = 0 as *mut libc::c_char;
    Lock_Display.expect("non-null function pointer")();
    owner = XGetSelectionOwner(SDL_Display, 1 as libc::c_int as Atom);
    Unlock_Display.expect("non-null function pointer")();
    if owner == 0 as libc::c_long as libc::c_ulong || owner == SDL_Window {
        owner =
            (*(*(SDL_Display as
                     _XPrivDisplay)).screens.offset((*(SDL_Display as
                                                           _XPrivDisplay)).default_screen
                                                        as isize)).root;
        selection = 9 as libc::c_int as Atom
    } else {
        let mut selection_response: libc::c_int = 0 as libc::c_int;
        let mut event: SDL_Event = SDL_Event{type_0: 0,};
        owner = SDL_Window;
        Lock_Display.expect("non-null function pointer")();
        selection =
            XInternAtom(SDL_Display,
                        b"SDL_SELECTION\x00" as *const u8 as
                            *const libc::c_char, 0 as libc::c_int);
        XConvertSelection(SDL_Display, 1 as libc::c_int as Atom, format,
                          selection, owner, 0 as libc::c_long as Time);
        Unlock_Display.expect("non-null function pointer")();
        while selection_response == 0 {
            SDL_WaitEvent(&mut event);
            if event.type_0 as libc::c_int == SDL_SYSWMEVENT as libc::c_int {
                let mut xevent: XEvent = (*event.syswm.msg).event.xevent;
                if xevent.type_0 == 31 as libc::c_int &&
                       xevent.xselection.requestor == owner {
                    selection_response = 1 as libc::c_int
                }
            }
        }
    }
    Lock_Display.expect("non-null function pointer")();
    if XGetWindowProperty(SDL_Display, owner, selection,
                          0 as libc::c_int as libc::c_long,
                          (2147483647 as libc::c_int / 4 as libc::c_int) as
                              libc::c_long, 0 as libc::c_int, format,
                          &mut seln_type, &mut seln_format, &mut nbytes,
                          &mut overflow,
                          &mut src as *mut *mut libc::c_char as
                              *mut *mut libc::c_uchar) == 0 as libc::c_int {
        if seln_type == format {
            *dstlen =
                convert_scrap(type_0, 0 as *mut libc::c_char, src,
                              nbytes as libc::c_int);
            *dst =
                realloc(*dst as *mut libc::c_void, *dstlen as libc::c_uint) as
                    *mut libc::c_char;
            if (*dst).is_null() {
                *dstlen = 0 as libc::c_int
            } else {
                convert_scrap(type_0, *dst, src, nbytes as libc::c_int);
            }
        }
        XFree(src as *mut libc::c_void);
    }
    Unlock_Display.expect("non-null function pointer")();
    /* scrap type */
}
/* The system message filter function -- handle clipboard messages */
unsafe extern "C" fn clipboard_filter(mut event: *const SDL_Event)
 -> libc::c_int {
    /* Post all non-window manager specific events */
    if (*event).type_0 as libc::c_int != SDL_SYSWMEVENT as libc::c_int {
        return 1 as libc::c_int
    }
    /* Handle window-manager specific clipboard events */
    match (*(*event).syswm.msg).event.xevent.type_0 {
        30 => {
            /* Copy the selection from XA_CUT_BUFFER0 to the requested property */
            let mut req: *mut XSelectionRequestEvent =
                0 as *mut XSelectionRequestEvent;
            let mut sevent: XEvent = _XEvent{type_0: 0,};
            let mut seln_format: libc::c_int = 0;
            let mut nbytes: libc::c_ulong = 0;
            let mut overflow: libc::c_ulong = 0;
            let mut seln_data: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
            req = &mut (*(*event).syswm.msg).event.xevent.xselectionrequest;
            sevent.xselection.type_0 = 31 as libc::c_int;
            sevent.xselection.display = (*req).display;
            sevent.xselection.selection = (*req).selection;
            sevent.xselection.target = 0 as libc::c_long as Atom;
            sevent.xselection.property = 0 as libc::c_long as Atom;
            sevent.xselection.requestor = (*req).requestor;
            sevent.xselection.time = (*req).time;
            if XGetWindowProperty(SDL_Display,
                                  (*(*(SDL_Display as
                                           _XPrivDisplay)).screens.offset((*(SDL_Display
                                                                                 as
                                                                                 _XPrivDisplay)).default_screen
                                                                              as
                                                                              isize)).root,
                                  9 as libc::c_int as Atom,
                                  0 as libc::c_int as libc::c_long,
                                  (2147483647 as libc::c_int /
                                       4 as libc::c_int) as libc::c_long,
                                  0 as libc::c_int, (*req).target,
                                  &mut sevent.xselection.target,
                                  &mut seln_format, &mut nbytes,
                                  &mut overflow, &mut seln_data) ==
                   0 as libc::c_int {
                if sevent.xselection.target == (*req).target {
                    if sevent.xselection.target == 31 as libc::c_int as Atom {
                        if *seln_data.offset(nbytes.wrapping_sub(1 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_ulong)
                                                 as isize) as libc::c_int ==
                               '\u{0}' as i32 {
                            nbytes = nbytes.wrapping_sub(1)
                        }
                    }
                    XChangeProperty(SDL_Display, (*req).requestor,
                                    (*req).property, sevent.xselection.target,
                                    seln_format, 0 as libc::c_int, seln_data,
                                    nbytes as libc::c_int);
                    sevent.xselection.property = (*req).property
                }
                XFree(seln_data as *mut libc::c_void);
            }
            XSendEvent(SDL_Display, (*req).requestor, 0 as libc::c_int,
                       0 as libc::c_int as libc::c_long, &mut sevent);
            XSync(SDL_Display, 0 as libc::c_int);
        }
        _ => { }
    }
    /* Post the event for X11 clipboard reading above */
    return 1 as libc::c_int;
}
/* X11_SCRAP */
