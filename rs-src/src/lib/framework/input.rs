use ::libc;
extern "C" {
    /* * If you want to use this event, you should include SDL_syswm.h */
    pub type SDL_SysWMmsg;
    /* Maximum chars of output to write in MAXLEN.  */
    #[no_mangle]
    fn snprintf(_: *mut libc::c_char, _: libc::c_uint, _: *const libc::c_char,
                _: ...) -> libc::c_int;
    /* Copy SRC to DEST.  */
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    /* * This is the mask which refers to all hotkey bindings */
    /* Function prototypes */
/* *
 * Enable/Disable UNICODE translation of keyboard input.
 *
 * This translation has some overhead, so translation defaults off.
 *
 * @param[in] enable
 * If 'enable' is 1, translation is enabled.
 * If 'enable' is 0, translation is disabled.
 * If 'enable' is -1, the translation state is not changed.
 *
 * @return It returns the previous state of keyboard translation.
 */
    #[no_mangle]
    fn SDL_EnableUNICODE(enable: libc::c_int) -> libc::c_int;
    /* *
 * Get the name of an SDL virtual keysym
 */
    #[no_mangle]
    fn SDL_GetKeyName(key: SDLKey) -> *mut libc::c_char;
    /* *
 * Set the position of the mouse cursor (generates a mouse motion event)
 */
    #[no_mangle]
    fn SDL_WarpMouse(x: Uint16, y: Uint16);
    /* * Add an event to the event queue.
 *  This function returns 0 on success, or -1 if the event queue was full
 *  or there was some other error.
 */
    #[no_mangle]
    fn SDL_PushEvent(event: *mut SDL_Event) -> libc::c_int;
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
    /* The Current screen size and bit depth */
    #[no_mangle]
    static mut screenWidth: UDWORD;
    #[no_mangle]
    static mut screenHeight: UDWORD;
    /*
 * config.h
 * load and save favourites to the registry.
 */
    #[no_mangle]
    fn getWarzoneKeyNumeric(pName: *mut STRING, val: *mut DWORD) -> BOOL;
}
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type int16_t = __int16_t;
/* Define uintN_t types.
   Copyright (C) 2017-2019 Free Software Foundation, Inc.
   This file is part of the GNU C Library.

   The GNU C Library is free software; you can redistribute it and/or
   modify it under the terms of the GNU Lesser General Public
   License as published by the Free Software Foundation; either
   version 2.1 of the License, or (at your option) any later version.

   The GNU C Library is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public
   License along with the GNU C Library; if not, see
   <http://www.gnu.org/licenses/>.  */
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type Uint8 = uint8_t;
pub type Sint16 = int16_t;
pub type Uint16 = uint16_t;
/*
    SDL - Simple DirectMedia Layer
    Copyright (C) 1997-2012 Sam Lantinga

    This library is free software; you can redistribute it and/or
    modify it under the terms of the GNU Lesser General Public
    License as published by the Free Software Foundation; either
    version 2.1 of the License, or (at your option) any later version.

    This library is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
    Lesser General Public License for more details.

    You should have received a copy of the GNU Lesser General Public
    License along with this library; if not, write to the Free Software
    Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA

    Sam Lantinga
    slouken@libsdl.org
*/
/* * What we really want is a mapping of every raw key on the keyboard.
 *  To support international keyboards, we use the range 0xA1 - 0xFF
 *  as international virtual keycodes.  We'll follow in the footsteps of X11...
 *  @brief The names of the keys
 */
pub type SDLKey = libc::c_uint;
/* *< Atari keyboard has Undo */
/*@}*/
/* Add any other keys here */
pub const SDLK_LAST: SDLKey = 323;
/* *< Some european keyboards */
pub const SDLK_UNDO: SDLKey = 322;
/* *< Power Macintosh power key */
pub const SDLK_EURO: SDLKey = 321;
pub const SDLK_POWER: SDLKey = 320;
pub const SDLK_MENU: SDLKey = 319;
pub const SDLK_BREAK: SDLKey = 318;
pub const SDLK_SYSREQ: SDLKey = 317;
pub const SDLK_PRINT: SDLKey = 316;
/* *< Multi-key compose key */
/*@}*/
/* * @name Miscellaneous function keys */
        /*@{*/
pub const SDLK_HELP: SDLKey = 315;
/* *< "Alt Gr" key */
pub const SDLK_COMPOSE: SDLKey = 314;
/* *< Right "Windows" key */
pub const SDLK_MODE: SDLKey = 313;
/* *< Left "Windows" key */
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
/*@}*/
/* * @name Key state modifier keys */
        /*@{*/
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
/*@}*/
/* * @name Function keys */
        /*@{*/
pub const SDLK_F1: SDLKey = 282;
pub const SDLK_PAGEDOWN: SDLKey = 281;
pub const SDLK_PAGEUP: SDLKey = 280;
pub const SDLK_END: SDLKey = 279;
pub const SDLK_HOME: SDLKey = 278;
pub const SDLK_INSERT: SDLKey = 277;
pub const SDLK_LEFT: SDLKey = 276;
pub const SDLK_RIGHT: SDLKey = 275;
pub const SDLK_DOWN: SDLKey = 274;
/*@}*/
/* * @name Arrows + Home/End pad */
        /*@{*/
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
/* 0xFF */
/*@}*/
/* * @name Numeric keypad */
        /*@{*/
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
/* 0xA0 */
pub const SDLK_WORLD_1: SDLKey = 161;
/* End of ASCII mapped keysyms */
        /*@}*/
/* * @name International keyboard syms */
        /*@{*/
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
/* 
	   Skip uppercase letters
	 */
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
/* * @name ASCII mapped keysyms
         *  The keyboard syms have been cleverly chosen to map to ASCII
         */
        /*@{*/
pub const SDLK_UNKNOWN: SDLKey = 0;
/* * Enumeration of valid key mods (possibly OR'd together) */
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
/*
    SDL - Simple DirectMedia Layer
    Copyright (C) 1997-2012 Sam Lantinga

    This library is free software; you can redistribute it and/or
    modify it under the terms of the GNU Lesser General Public
    License as published by the Free Software Foundation; either
    version 2.1 of the License, or (at your option) any later version.

    This library is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
    Lesser General Public License for more details.

    You should have received a copy of the GNU Lesser General Public
    License along with this library; if not, write to the Free Software
    Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA

    Sam Lantinga
    slouken@libsdl.org
*/
/* * @file SDL_keyboard.h
 *  Include file for SDL keyboard event handling
 */
/* Set up for C function definitions, even when using C++ */
/* * Keysym structure
 *
 *  - The scancode is hardware dependent, and should not be used by general
 *    applications.  If no hardware scancode is available, it will be 0.
 *
 *  - The 'unicode' translated character is only available when character
 *    translation is enabled by the SDL_EnableUNICODE() API.  If non-zero,
 *    this is a UNICODE character corresponding to the keypress.  If the
 *    high 9 bits of the character are 0, then this maps to the equivalent
 *    ASCII character:
 *      @code
 *	char ch;
 *	if ( (keysym.unicode & 0xFF80) == 0 ) {
 *		ch = keysym.unicode & 0x7F;
 *	} else {
 *		An international character..
 *	}
 *      @endcode
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_keysym {
    pub scancode: Uint8,
    pub sym: SDLKey,
    pub mod_0: SDLMod,
    pub unicode: Uint16,
}
/*@}*/
/* * Event enumerations */
pub type C2RustUnnamed = libc::c_uint;
/* * This last event is only for bounding internal arrays
	*  It is the number of bits in the event mask datatype -- Uint32
        */
pub const SDL_NUMEVENTS: C2RustUnnamed = 32;
/* *< Reserved for future use.. */
/* * Events SDL_USEREVENT through SDL_MAXEVENTS-1 are for your use */
pub const SDL_USEREVENT: C2RustUnnamed = 24;
/* *< Reserved for future use.. */
pub const SDL_EVENT_RESERVED7: C2RustUnnamed = 23;
/* *< Reserved for future use.. */
pub const SDL_EVENT_RESERVED6: C2RustUnnamed = 22;
/* *< Reserved for future use.. */
pub const SDL_EVENT_RESERVED5: C2RustUnnamed = 21;
/* *< Reserved for future use.. */
pub const SDL_EVENT_RESERVED4: C2RustUnnamed = 20;
/* *< Reserved for future use.. */
pub const SDL_EVENT_RESERVED3: C2RustUnnamed = 19;
/* *< Screen needs to be redrawn */
pub const SDL_EVENT_RESERVED2: C2RustUnnamed = 18;
/* *< User resized video mode */
pub const SDL_VIDEOEXPOSE: C2RustUnnamed = 17;
/* *< Reserved for future use.. */
pub const SDL_VIDEORESIZE: C2RustUnnamed = 16;
/* *< Reserved for future use.. */
pub const SDL_EVENT_RESERVEDB: C2RustUnnamed = 15;
/* *< System specific event */
pub const SDL_EVENT_RESERVEDA: C2RustUnnamed = 14;
/* *< User-requested quit */
pub const SDL_SYSWMEVENT: C2RustUnnamed = 13;
/* *< Joystick button released */
pub const SDL_QUIT: C2RustUnnamed = 12;
/* *< Joystick button pressed */
pub const SDL_JOYBUTTONUP: C2RustUnnamed = 11;
/* *< Joystick hat position change */
pub const SDL_JOYBUTTONDOWN: C2RustUnnamed = 10;
/* *< Joystick trackball motion */
pub const SDL_JOYHATMOTION: C2RustUnnamed = 9;
/* *< Joystick axis motion */
pub const SDL_JOYBALLMOTION: C2RustUnnamed = 8;
/* *< Mouse button released */
pub const SDL_JOYAXISMOTION: C2RustUnnamed = 7;
/* *< Mouse button pressed */
pub const SDL_MOUSEBUTTONUP: C2RustUnnamed = 6;
/* *< Mouse moved */
pub const SDL_MOUSEBUTTONDOWN: C2RustUnnamed = 5;
/* *< Keys released */
pub const SDL_MOUSEMOTION: C2RustUnnamed = 4;
/* *< Keys pressed */
pub const SDL_KEYUP: C2RustUnnamed = 3;
/* *< Application loses/gains visibility */
pub const SDL_KEYDOWN: C2RustUnnamed = 2;
/* *< Unused (do not remove) */
pub const SDL_ACTIVEEVENT: C2RustUnnamed = 1;
pub const SDL_NOEVENT: C2RustUnnamed = 0;
/*@}*/
/* * Application visibility event structure */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_ActiveEvent {
    pub type_0: Uint8,
    pub gain: Uint8,
    pub state: Uint8,
}
/* * Keyboard event structure */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_KeyboardEvent {
    pub type_0: Uint8,
    pub which: Uint8,
    pub state: Uint8,
    pub keysym: SDL_keysym,
}
/* * Mouse motion event structure */
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
/* * Mouse button event structure */
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
/* * Joystick axis motion event structure */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_JoyAxisEvent {
    pub type_0: Uint8,
    pub which: Uint8,
    pub axis: Uint8,
    pub value: Sint16,
}
/* * Joystick trackball motion event structure */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_JoyBallEvent {
    pub type_0: Uint8,
    pub which: Uint8,
    pub ball: Uint8,
    pub xrel: Sint16,
    pub yrel: Sint16,
}
/* * Joystick hat position change event structure */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_JoyHatEvent {
    pub type_0: Uint8,
    pub which: Uint8,
    pub hat: Uint8,
    pub value: Uint8,
}
/* * Joystick button event structure */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_JoyButtonEvent {
    pub type_0: Uint8,
    pub which: Uint8,
    pub button: Uint8,
    pub state: Uint8,
}
/* * The "window resized" event
 *  When you get this event, you are responsible for setting a new video
 *  mode with the new width and height.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_ResizeEvent {
    pub type_0: Uint8,
    pub w: libc::c_int,
    pub h: libc::c_int,
}
/* * The "screen redraw" event */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_ExposeEvent {
    pub type_0: Uint8,
}
/* * The "quit requested" event */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_QuitEvent {
    pub type_0: Uint8,
}
/* * A user-defined event type */
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
pub struct SDL_SysWMEvent {
    pub type_0: Uint8,
    pub msg: *mut SDL_SysWMmsg,
}
/* * General event structure */
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
pub type STRING = libc::c_char;
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
/*
 * Input.h
 *
 * Prototypes for the keyboard and mouse input funcitons.
 */
/* Check the header files have been included from frame.h if they
 * are used outside of the framework library.
 */
/* The defines for all the key codes */
pub type KEY_CODE = _key_code;
/*
 * Input.c
 *
 * Processes all keyboard and mouse input.
 *
 */
/* Allow frame header files to be singly included */
/* The input buffer printf's */
//#define DEBUG_GROUP1
/* The possible states for keys */
pub type KEY_STATE = _key_state;
pub type _key_state = libc::c_uint;
pub const KEY_DRAG: _key_state = 6;
pub const KEY_DOUBLECLICK: _key_state = 5;
pub const KEY_PRESSRELEASE: _key_state = 4;
pub const KEY_RELEASED: _key_state = 3;
pub const KEY_DOWN: _key_state = 2;
pub const KEY_PRESSED: _key_state = 1;
pub const KEY_UP: _key_state = 0;
pub type MOUSE_KEY_CODE = _mouse_key_code;
pub type _mouse_key_code = libc::c_uint;
pub const MOUSE_WDN: _mouse_key_code = 5;
pub const MOUSE_WUP: _mouse_key_code = 4;
pub const MOUSE_RMB: _mouse_key_code = 3;
pub const MOUSE_MMB: _mouse_key_code = 2;
pub const MOUSE_LMB: _mouse_key_code = 1;
// When a key goes up and down in a frame
// Only used by mouse keys
// Only used by mouse keys
/* The current state of the keyboard */
static mut aKeyState: [KEY_STATE; 323] = [KEY_UP; 323];
/* The current location of the mouse */
static mut mouseXPos: SDWORD = 0;
static mut mouseYPos: SDWORD = 0;
/* Which button is being used for a drag */
static mut dragKey: MOUSE_KEY_CODE = 0 as MOUSE_KEY_CODE;
/* The start of a possible drag by the mouse */
static mut dragX: SDWORD = 0;
static mut dragY: SDWORD = 0;
/* The current mouse button state */
static mut aMouseState: [KEY_STATE; 6] = [KEY_UP; 6];
/* The input string buffer */
static mut pInputBuffer: [UDWORD; 512] = [0; 512];
static mut pStartBuffer: *mut UDWORD = 0 as *const UDWORD as *mut UDWORD;
static mut pEndBuffer: *mut UDWORD = 0 as *const UDWORD as *mut UDWORD;
static mut pCharInputBuffer: [libc::c_char; 512] = [0; 512];
static mut pCharStartBuffer: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
static mut pCharEndBuffer: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
static mut currentChar: libc::c_char = 0;
unsafe extern "C" fn sdlKeyToKeyCode(mut key: SDLKey) -> KEY_CODE {
    return key as KEY_CODE;
}
unsafe extern "C" fn keyCodeToSDLKey(mut code: KEY_CODE) -> SDLKey {
    return code as SDLKey;
}
/* Converts the key code into an ascii string */
#[no_mangle]
pub unsafe extern "C" fn keyScanToString(mut code: KEY_CODE,
                                         mut ascii: *mut STRING,
                                         mut maxStringSize: UDWORD) {
    if keyCodeToSDLKey(code) as libc::c_uint ==
           SDLK_LAST as libc::c_int as libc::c_uint {
        strcpy(ascii, b"???\x00" as *const u8 as *const libc::c_char);
        return
    }
    if code as libc::c_uint >= 0 as libc::c_int as libc::c_uint &&
           code as libc::c_uint <= SDLK_LAST as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Invalid key code: %d\x00" as *const u8 as *const libc::c_char,
              code as libc::c_uint);
    };
    if code as libc::c_uint >= 0 as libc::c_int as libc::c_uint &&
           code as libc::c_uint <= SDLK_LAST as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"input.c\x00" as *const u8 as *const libc::c_char,
              86 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 16],
                                        &[libc::c_char; 16]>(b"keyScanToString\x00")).as_ptr(),
              b"(code >= 0) && (code <= KEY_MAXSCAN)\x00" as *const u8 as
                  *const libc::c_char);
    };
    snprintf(ascii, maxStringSize,
             b"%s\x00" as *const u8 as *const libc::c_char,
             SDL_GetKeyName(keyCodeToSDLKey(code)));
    //use _snprintf() in _MSC_VER	  --Q
}
/* Initialise the input module */
/* Initialise the input module */
#[no_mangle]
pub unsafe extern "C" fn inputInitialise() {
    let mut i: UDWORD = 0;
    i = 0 as libc::c_int as UDWORD;
    while i < SDLK_LAST as libc::c_int as libc::c_uint {
        aKeyState[i as usize] = KEY_UP;
        i = i.wrapping_add(1)
    }
    i = 0 as libc::c_int as UDWORD;
    while i < 6 as libc::c_int as libc::c_uint {
        aMouseState[i as usize] = KEY_UP;
        i = i.wrapping_add(1)
    }
    pStartBuffer = pInputBuffer.as_mut_ptr();
    pEndBuffer = pInputBuffer.as_mut_ptr();
    pCharStartBuffer = pCharInputBuffer.as_mut_ptr();
    pCharEndBuffer = pCharInputBuffer.as_mut_ptr();
    mouseXPos =
        screenWidth.wrapping_div(2 as libc::c_int as libc::c_uint) as SDWORD;
    dragX = mouseXPos;
    mouseYPos =
        screenHeight.wrapping_div(2 as libc::c_int as libc::c_uint) as SDWORD;
    dragY = mouseYPos;
    dragKey = MOUSE_LMB;
    SDL_EnableUNICODE(1 as libc::c_int);
}
/* Add a key press to the key buffer */
/* add count copies of the characater code to the input buffer */
#[no_mangle]
pub unsafe extern "C" fn inputAddBuffer(mut code: UDWORD,
                                        mut char_code: libc::c_char,
                                        mut count: UDWORD) {
    let mut pNext: *mut UDWORD = 0 as *mut UDWORD;
    let mut pCharNext: *mut libc::c_char = 0 as *mut libc::c_char;
    /* Calculate what pEndBuffer will be set to next */
    pNext = pEndBuffer.offset(1 as libc::c_int as isize);
    pCharNext = pCharEndBuffer.offset(1 as libc::c_int as isize);
    if pNext >= pInputBuffer.as_mut_ptr().offset(512 as libc::c_int as isize)
       {
        pNext = pInputBuffer.as_mut_ptr();
        pCharNext = pCharInputBuffer.as_mut_ptr()
    }
    while pNext != pStartBuffer && count > 0 as libc::c_int as libc::c_uint {
        /* Store the character */
        *pEndBuffer = code;
        *pCharEndBuffer = char_code;
        pEndBuffer = pNext;
        pCharEndBuffer = pCharNext;
        count =
            (count as
                 libc::c_uint).wrapping_sub(1 as libc::c_int as libc::c_uint)
                as UDWORD as UDWORD;
        /* Calculate what pEndBuffer will be set to next */
        pNext = pEndBuffer.offset(1 as libc::c_int as isize);
        pCharNext = pCharEndBuffer.offset(1 as libc::c_int as isize);
        if pNext >=
               pInputBuffer.as_mut_ptr().offset(512 as libc::c_int as isize) {
            pNext = pInputBuffer.as_mut_ptr();
            pCharNext = pCharInputBuffer.as_mut_ptr()
        }
    };
}
/* Clear the input buffer */
/* Clear the input buffer */
#[no_mangle]
pub unsafe extern "C" fn inputClearBuffer() {
    pStartBuffer = pInputBuffer.as_mut_ptr();
    pEndBuffer = pInputBuffer.as_mut_ptr();
    pCharStartBuffer = pCharInputBuffer.as_mut_ptr();
    pCharEndBuffer = pCharInputBuffer.as_mut_ptr();
}
/* Some defines for keys that map into the normal character space */
/* Return the next key press or 0 if no key in the buffer.
 * The key returned will have been remaped to the correct ascii code for the
 * windows key map.
 * All key presses are buffered up (including windows auto repeat).
 */
/* Return the next key press or 0 if no key in the buffer.
 * The key returned will have been remaped to the correct ascii code for the
 * windows key map.
 * All key presses are buffered up (including windows auto repeat).
 */
#[no_mangle]
pub unsafe extern "C" fn inputGetKey() -> UDWORD {
    let mut retVal: UDWORD = 0;
    if pStartBuffer != pEndBuffer {
        retVal = *pStartBuffer;
        currentChar = *pCharStartBuffer;
        pStartBuffer = pStartBuffer.offset(1 as libc::c_int as isize);
        pCharStartBuffer = pCharStartBuffer.offset(1 as libc::c_int as isize);
        if pStartBuffer >=
               pInputBuffer.as_mut_ptr().offset(512 as libc::c_int as isize) {
            pStartBuffer = pInputBuffer.as_mut_ptr();
            pCharStartBuffer = pCharInputBuffer.as_mut_ptr()
        }
    } else { retVal = 0 as libc::c_int as UDWORD }
    return retVal;
}
#[no_mangle]
pub unsafe extern "C" fn inputGetCharKey() -> libc::c_char {
    return currentChar;
}
/* Deal with windows messages to maintain the state of the keyboard and mouse */
#[no_mangle]
pub unsafe extern "C" fn inputProcessEvent(mut event: *mut SDL_Event) {
    let mut code: UDWORD = 0;
    let mut i: UDWORD = 0;
    let mut vk: UDWORD = 0;
    //	FRACT	divX,divY;
//	UDWORD	scrX,scrY;
    match (*event).type_0 as libc::c_int {
        2 => {
            //printf("keydown %s (%i)\n", SDL_GetKeyName(code), event->key.keysym.sym);
            match (*event).key.keysym.sym as libc::c_uint {
                276 => { vk = 0x10000 as libc::c_int as UDWORD }
                275 => { vk = 0x20000 as libc::c_int as UDWORD }
                273 => { vk = 0x30000 as libc::c_int as UDWORD }
                274 => { vk = 0x40000 as libc::c_int as UDWORD }
                278 => { vk = 0x50000 as libc::c_int as UDWORD }
                279 => { vk = 0x60000 as libc::c_int as UDWORD }
                277 => { vk = 0x70000 as libc::c_int as UDWORD }
                127 => { vk = 0x80000 as libc::c_int as UDWORD }
                280 => { vk = 0x90000 as libc::c_int as UDWORD }
                281 => { vk = 0xa0000 as libc::c_int as UDWORD }
                _ => { vk = (*event).key.keysym.sym as UDWORD }
            }
            let mut char_code: libc::c_uchar =
                (*event).key.keysym.unicode as libc::c_uchar;
            //				DBP1(("Code: %x\n", vk));		//This breaks with .NET, [DBP1(("Code: %x\n", vk));]it don't want the ; at end. --Qamly
            if (char_code as libc::c_int) < 32 as libc::c_int {
                char_code = 0 as libc::c_int as libc::c_uchar
            }
            inputAddBuffer(vk, char_code as libc::c_char,
                           1 as libc::c_int as UDWORD);
            code = sdlKeyToKeyCode((*event).key.keysym.sym) as UDWORD;
            if aKeyState[code as usize] as libc::c_uint ==
                   KEY_UP as libc::c_int as libc::c_uint ||
                   aKeyState[code as usize] as libc::c_uint ==
                       KEY_RELEASED as libc::c_int as libc::c_uint ||
                   aKeyState[code as usize] as libc::c_uint ==
                       KEY_PRESSRELEASE as libc::c_int as libc::c_uint {
                aKeyState[code as usize] = KEY_PRESSED
            }
        }
        3 => {
            code = sdlKeyToKeyCode((*event).key.keysym.sym) as UDWORD;
            if aKeyState[code as usize] as libc::c_uint ==
                   KEY_PRESSED as libc::c_int as libc::c_uint {
                aKeyState[code as usize] = KEY_PRESSRELEASE
            } else if aKeyState[code as usize] as libc::c_uint ==
                          KEY_DOWN as libc::c_int as libc::c_uint {
                aKeyState[code as usize] = KEY_RELEASED
            }
        }
        4 => {
            /* Deal with mouse messages */
            if mouseDown(MOUSE_MMB) == 0 {
                /* store the current mouse position */
                mouseXPos = (*event).motion.x as SDWORD;
                mouseYPos = (*event).motion.y as SDWORD;
                /*
				if(mouseXPos>=screenWidth)
				{
					mouseXPos = screenWidth-1;
				}
				if(mouseYPos >= screenHeight)
				{
					mouseYPos = screenHeight-1;
				}
				*/
                /* now see if a drag has started */
                if (aMouseState[dragKey as usize] as libc::c_uint ==
                        KEY_PRESSED as libc::c_int as libc::c_uint ||
                        aMouseState[dragKey as usize] as libc::c_uint ==
                            KEY_DOWN as libc::c_int as libc::c_uint) &&
                       ((if dragX > mouseXPos {
                             (dragX) - mouseXPos
                         } else { (mouseXPos) - dragX }) > 5 as libc::c_int ||
                            (if dragY > mouseYPos {
                                 (dragY) - mouseYPos
                             } else { (mouseYPos) - dragY }) >
                                5 as libc::c_int) {
                    //		DBPRINTF(("dragging\n"));
                    aMouseState[dragKey as usize] = KEY_DRAG
                }
            }
        }
        6 => {
            if aMouseState[(*event).button.button as usize] as libc::c_uint ==
                   KEY_PRESSED as libc::c_int as libc::c_uint {
                aMouseState[(*event).button.button as usize] =
                    KEY_PRESSRELEASE
            } else if aMouseState[(*event).button.button as usize] as
                          libc::c_uint ==
                          KEY_DOWN as libc::c_int as libc::c_uint ||
                          aMouseState[(*event).button.button as usize] as
                              libc::c_uint ==
                              KEY_DRAG as libc::c_int as libc::c_uint {
                aMouseState[(*event).button.button as usize] = KEY_RELEASED
            }
        }
        5 => {
            if aMouseState[(*event).button.button as usize] as libc::c_uint ==
                   KEY_UP as libc::c_int as libc::c_uint ||
                   aMouseState[(*event).button.button as usize] as
                       libc::c_uint ==
                       KEY_RELEASED as libc::c_int as libc::c_uint ||
                   aMouseState[(*event).button.button as usize] as
                       libc::c_uint ==
                       KEY_PRESSRELEASE as libc::c_int as libc::c_uint {
                aMouseState[(*event).button.button as usize] = KEY_PRESSED;
                if ((*event).button.button as libc::c_int) < 4 as libc::c_int
                   {
                    dragKey = (*event).button.button as MOUSE_KEY_CODE;
                    dragX = mouseXPos;
                    dragY = mouseYPos
                }
            }
        }
        1 => {
            /* Lost the window focus, have to take this as a global key up */
            i = 0 as libc::c_int as UDWORD;
            while i < SDLK_LAST as libc::c_int as libc::c_uint {
                if aKeyState[i as usize] as libc::c_uint ==
                       KEY_PRESSED as libc::c_int as libc::c_uint ||
                       aKeyState[i as usize] as libc::c_uint ==
                           KEY_DOWN as libc::c_int as libc::c_uint {
                    aKeyState[i as usize] = KEY_RELEASED
                }
                i = i.wrapping_add(1)
            }
            i = 0 as libc::c_int as UDWORD;
            while i < 6 as libc::c_int as libc::c_uint {
                if aMouseState[i as usize] as libc::c_uint ==
                       KEY_PRESSED as libc::c_int as libc::c_uint ||
                       aMouseState[i as usize] as libc::c_uint ==
                           KEY_DOWN as libc::c_int as libc::c_uint ||
                       aMouseState[i as usize] as libc::c_uint ==
                           KEY_DRAG as libc::c_int as libc::c_uint {
                    aMouseState[i as usize] = KEY_RELEASED
                }
                i = i.wrapping_add(1)
            }
        }
        _ => { }
    };
}
/* This is called once a frame so that the system can tell
 * whether a key was pressed this turn or held down from the last frame.
 */
/* This is called once a frame so that the system can tell
 * whether a key was pressed this turn or held down from the last frame.
 */
#[no_mangle]
pub unsafe extern "C" fn inputNewFrame() {
    let mut i: UDWORD = 0;
    /* Do the keyboard */
    i = 0 as libc::c_int as UDWORD;
    while i < SDLK_LAST as libc::c_int as libc::c_uint {
        if aKeyState[i as usize] as libc::c_uint ==
               KEY_PRESSED as libc::c_int as libc::c_uint {
            aKeyState[i as usize] = KEY_DOWN
        } else if aKeyState[i as usize] as libc::c_uint ==
                      KEY_RELEASED as libc::c_int as libc::c_uint ||
                      aKeyState[i as usize] as libc::c_uint ==
                          KEY_PRESSRELEASE as libc::c_int as libc::c_uint {
            aKeyState[i as usize] = KEY_UP
        }
        i = i.wrapping_add(1)
    }
    /* Do the mouse */
    i = 0 as libc::c_int as UDWORD;
    while i < 6 as libc::c_int as libc::c_uint {
        if aMouseState[i as usize] as libc::c_uint ==
               KEY_PRESSED as libc::c_int as libc::c_uint {
            aMouseState[i as usize] = KEY_DOWN
        } else if aMouseState[i as usize] as libc::c_uint ==
                      KEY_RELEASED as libc::c_int as libc::c_uint ||
                      aMouseState[i as usize] as libc::c_uint ==
                          KEY_DOUBLECLICK as libc::c_int as libc::c_uint ||
                      aMouseState[i as usize] as libc::c_uint ==
                          KEY_PRESSRELEASE as libc::c_int as libc::c_uint {
            aMouseState[i as usize] = KEY_UP
        }
        i = i.wrapping_add(1)
    };
}
/* This returns true if the key is currently depressed */
/* This returns true if the key is currently depressed */
#[no_mangle]
pub unsafe extern "C" fn keyDown(mut code: KEY_CODE) -> BOOL {
    if code as libc::c_uint >= 0 as libc::c_int as libc::c_uint &&
           (code as libc::c_uint) < SDLK_LAST as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Invalid key code: %d\x00" as *const u8 as *const libc::c_char,
              code as libc::c_uint);
    };
    if code as libc::c_uint >= 0 as libc::c_int as libc::c_uint &&
           (code as libc::c_uint) < SDLK_LAST as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"input.c\x00" as *const u8 as *const libc::c_char,
              402 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 8],
                                        &[libc::c_char; 8]>(b"keyDown\x00")).as_ptr(),
              b"(code >= 0) && (code < KEY_MAXSCAN)\x00" as *const u8 as
                  *const libc::c_char);
    };
    return (aKeyState[code as usize] as libc::c_uint !=
                KEY_UP as libc::c_int as libc::c_uint) as libc::c_int;
}
/* This returns true if the key went from being up to being down this frame */
/* This returns true if the key went from being up to being down this frame */
#[no_mangle]
pub unsafe extern "C" fn keyPressed(mut code: KEY_CODE) -> BOOL {
    if code as libc::c_uint >= 0 as libc::c_int as libc::c_uint &&
           (code as libc::c_uint) < SDLK_LAST as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Invalid key code: %d\x00" as *const u8 as *const libc::c_char,
              code as libc::c_uint);
    };
    if code as libc::c_uint >= 0 as libc::c_int as libc::c_uint &&
           (code as libc::c_uint) < SDLK_LAST as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"input.c\x00" as *const u8 as *const libc::c_char,
              409 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 11],
                                        &[libc::c_char; 11]>(b"keyPressed\x00")).as_ptr(),
              b"(code >= 0) && (code < KEY_MAXSCAN)\x00" as *const u8 as
                  *const libc::c_char);
    };
    return (aKeyState[code as usize] as libc::c_uint ==
                KEY_PRESSED as libc::c_int as libc::c_uint ||
                aKeyState[code as usize] as libc::c_uint ==
                    KEY_PRESSRELEASE as libc::c_int as libc::c_uint) as
               libc::c_int;
}
/* This returns true if the key went from being down to being up this frame */
/* This returns true if the key went from being down to being up this frame */
#[no_mangle]
pub unsafe extern "C" fn keyReleased(mut code: KEY_CODE) -> BOOL {
    if code as libc::c_uint >= 0 as libc::c_int as libc::c_uint &&
           (code as libc::c_uint) < SDLK_LAST as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Invalid key code: %d\x00" as *const u8 as *const libc::c_char,
              code as libc::c_uint);
    };
    if code as libc::c_uint >= 0 as libc::c_int as libc::c_uint &&
           (code as libc::c_uint) < SDLK_LAST as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"input.c\x00" as *const u8 as *const libc::c_char,
              416 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 12],
                                        &[libc::c_char; 12]>(b"keyReleased\x00")).as_ptr(),
              b"(code >= 0) && (code < KEY_MAXSCAN)\x00" as *const u8 as
                  *const libc::c_char);
    };
    return (aKeyState[code as usize] as libc::c_uint ==
                KEY_RELEASED as libc::c_int as libc::c_uint ||
                aKeyState[code as usize] as libc::c_uint ==
                    KEY_PRESSRELEASE as libc::c_int as libc::c_uint) as
               libc::c_int;
}
/* These two functions return the current position of the mouse */
/* Return the X coordinate of the mouse */
#[no_mangle]
pub unsafe extern "C" fn mouseX() -> SDWORD { return mouseXPos; }
/* Return the Y coordinate of the mouse */
#[no_mangle]
pub unsafe extern "C" fn mouseY() -> SDWORD { return mouseYPos; }
/* This returns true if the mouse key is currently depressed */
/* This returns true if the mouse key is currently depressed */
#[no_mangle]
pub unsafe extern "C" fn mouseDown(mut code: MOUSE_KEY_CODE) -> BOOL {
    if code as libc::c_uint >= 0 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Invalid mouse key code: %d\x00" as *const u8 as
                  *const libc::c_char, code as libc::c_uint);
    };
    if code as libc::c_uint >= 0 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"input.c\x00" as *const u8 as *const libc::c_char,
              435 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 10],
                                        &[libc::c_char; 10]>(b"mouseDown\x00")).as_ptr(),
              b"(code >= 0)\x00" as *const u8 as *const libc::c_char);
    };
    return (aMouseState[code as usize] as libc::c_uint !=
                KEY_UP as libc::c_int as libc::c_uint) as libc::c_int;
}
/* This returns true if the mouse key was double clicked */
/* This returns true if the mouse key was double clicked */
#[no_mangle]
pub unsafe extern "C" fn mouseDClicked(mut code: MOUSE_KEY_CODE) -> BOOL {
    if code as libc::c_uint >= 0 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Invalid mouse key code: %d\x00" as *const u8 as
                  *const libc::c_char, code as libc::c_uint);
    };
    if code as libc::c_uint >= 0 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"input.c\x00" as *const u8 as *const libc::c_char,
              442 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"mouseDClicked\x00")).as_ptr(),
              b"code >= 0\x00" as *const u8 as *const libc::c_char);
    };
    return (aMouseState[code as usize] as libc::c_uint ==
                KEY_DOUBLECLICK as libc::c_int as libc::c_uint) as
               libc::c_int;
}
/* This returns true if the mouse key went from being up to being down this frame */
/* This returns true if the mouse key went from being up to being down this frame */
#[no_mangle]
pub unsafe extern "C" fn mousePressed(mut code: MOUSE_KEY_CODE) -> BOOL {
    if code as libc::c_uint >= 0 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Invalid mouse key code: %d\x00" as *const u8 as
                  *const libc::c_char, code as libc::c_uint);
    };
    if code as libc::c_uint >= 0 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"input.c\x00" as *const u8 as *const libc::c_char,
              449 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 13],
                                        &[libc::c_char; 13]>(b"mousePressed\x00")).as_ptr(),
              b"(code >= 0)\x00" as *const u8 as *const libc::c_char);
    };
    return (aMouseState[code as usize] as libc::c_uint ==
                KEY_PRESSED as libc::c_int as libc::c_uint ||
                aMouseState[code as usize] as libc::c_uint ==
                    KEY_PRESSRELEASE as libc::c_int as libc::c_uint) as
               libc::c_int;
}
/* This returns true if the mouse key went from being down to being up this frame */
/* This returns true if the mouse key went from being down to being up this frame */
#[no_mangle]
pub unsafe extern "C" fn mouseReleased(mut code: MOUSE_KEY_CODE) -> BOOL {
    if code as libc::c_uint >= 0 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Invalid mouse key code: %d\x00" as *const u8 as
                  *const libc::c_char, code as libc::c_uint);
    };
    if code as libc::c_uint >= 0 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"input.c\x00" as *const u8 as *const libc::c_char,
              457 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"mouseReleased\x00")).as_ptr(),
              b"(code >= 0)\x00" as *const u8 as *const libc::c_char);
    };
    return (aMouseState[code as usize] as libc::c_uint ==
                KEY_RELEASED as libc::c_int as libc::c_uint ||
                aMouseState[code as usize] as libc::c_uint ==
                    KEY_DOUBLECLICK as libc::c_int as libc::c_uint ||
                aMouseState[code as usize] as libc::c_uint ==
                    KEY_PRESSRELEASE as libc::c_int as libc::c_uint) as
               libc::c_int;
}
/* Check for a mouse drag, return the drag start coords if dragging */
/* Check for a mouse drag, return the drag start coords if dragging */
#[no_mangle]
pub unsafe extern "C" fn mouseDrag(mut code: MOUSE_KEY_CODE,
                                   mut px: *mut UDWORD, mut py: *mut UDWORD)
 -> BOOL {
    if code as libc::c_uint >= 0 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Invalid mouse key code: %d\x00" as *const u8 as
                  *const libc::c_char, code as libc::c_uint);
    };
    if code as libc::c_uint >= 0 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"input.c\x00" as *const u8 as *const libc::c_char,
              466 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 10],
                                        &[libc::c_char; 10]>(b"mouseDrag\x00")).as_ptr(),
              b"(code >= 0)\x00" as *const u8 as *const libc::c_char);
    };
    if aMouseState[code as usize] as libc::c_uint ==
           KEY_DRAG as libc::c_int as libc::c_uint {
        *px = dragX as UDWORD;
        *py = dragY as UDWORD;
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
/* Warps the mouse to the given position */
#[no_mangle]
pub unsafe extern "C" fn SetMousePos(mut nowt: UDWORD, mut x: UDWORD,
                                     mut y: UDWORD) {
    static mut mousewarp: libc::c_int = -(1 as libc::c_int);
    if mousewarp == -(1 as libc::c_int) {
        let mut val: libc::c_int = 0;
        mousewarp = 1 as libc::c_int;
        if getWarzoneKeyNumeric(b"nomousewarp\x00" as *const u8 as
                                    *const libc::c_char as *mut STRING,
                                &mut val) != 0 {
            if val != 0 { mousewarp = 0 as libc::c_int }
        }
    }
    if mousewarp != 0 { SDL_WarpMouse(x as Uint16, y as Uint16); };
}
/* Sets the state of the mouse key to down */
/* Sets the state of the mouse key to down */
#[no_mangle]
pub unsafe extern "C" fn setMouseDown(mut code: MOUSE_KEY_CODE) {
    let mut event: SDL_Event = SDL_Event{type_0: 0,};
    event.type_0 = SDL_MOUSEBUTTONDOWN as libc::c_int as Uint8;
    event.button.type_0 = SDL_MOUSEBUTTONDOWN as libc::c_int as Uint8;
    event.button.button = code as Uint8;
    event.button.state = 1 as libc::c_int as Uint8;
    event.button.x = mouseX() as Uint16;
    event.button.y = mouseY() as Uint16;
    SDL_PushEvent(&mut event);
}
/* Sets the state of the mouse key to up */
/* Sets the state of the mouse key to up */
#[no_mangle]
pub unsafe extern "C" fn setMouseUp(mut code: MOUSE_KEY_CODE) {
    let mut event: SDL_Event = SDL_Event{type_0: 0,};
    event.type_0 = SDL_MOUSEBUTTONUP as libc::c_int as Uint8;
    event.button.type_0 = SDL_MOUSEBUTTONUP as libc::c_int as Uint8;
    event.button.button = code as Uint8;
    event.button.state = 0 as libc::c_int as Uint8;
    event.button.x = mouseX() as Uint16;
    event.button.y = mouseY() as Uint16;
    SDL_PushEvent(&mut event);
}
