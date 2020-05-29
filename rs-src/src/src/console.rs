use ::libc;
extern "C" {
    /*
 * Input.h
 *
 * Prototypes for the keyboard and mouse input funcitons.
 */
    /* Check the header files have been included from frame.h if they
 * are used outside of the framework library.
 */
    /* The defines for all the key codes */
    /* The largest possible scan code (probably a lot less than this but ...) */
//      but ...    it's not as if it's got to fit into 2meg of mem or anything is it ...
    /* Converts the key code into an ascii string */
    /* Initialise the input module */
    /* Add a key press to the key buffer */
    /* This returns true if the key is currently depressed */
    /* This returns true if the key went from being up to being down this frame */
    /* This returns true if the key went from being down to being up this frame */
    /* These two functions return the current position of the mouse */
    #[no_mangle]
    fn mouseX() -> SDWORD;
    #[no_mangle]
    fn mouseY() -> SDWORD;
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
    fn strlen(_: *const libc::c_char) -> libc::c_uint;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn vsprintf(_: *mut libc::c_char, _: *const libc::c_char,
                _: __builtin_va_list) -> libc::c_int;
    // The maximum time for one frame (stops the clock running away when debugging)
// changed to /6 by ajl. if this needs to go back to ticks/10 then tell me. 
    /* The current time in the game world */
    #[no_mangle]
    static mut gameTime: UDWORD;
    /* The current time in the game world */
    #[no_mangle]
    static mut gameTime2: UDWORD;
    #[no_mangle]
    fn pie_GetVideoBufferWidth() -> UDWORD;
    #[no_mangle]
    fn pie_GetVideoBufferHeight() -> UDWORD;
    //Sets all states
    #[no_mangle]
    fn pie_SetDepthBufferStatus(depthMode: DEPTH_MODE);
    //fog currently on
    #[no_mangle]
    fn pie_SetFogStatus(val: BOOL);
    #[no_mangle]
    fn iV_GetTextLineSize() -> libc::c_int;
    #[no_mangle]
    fn iV_GetTextWidth(String: *mut STRING) -> libc::c_int;
    #[no_mangle]
    fn iV_SetTextColour(Index: SWORD);
    #[no_mangle]
    fn pie_DrawFormattedText(String: *mut libc::c_char, x: UDWORD, y: UDWORD,
                             Width: UDWORD, Justify: UDWORD, DrawBack: BOOL)
     -> UDWORD;
    #[no_mangle]
    fn pie_TransBoxFill(x0: SDWORD, y0: SDWORD, x1: SDWORD, y1: SDWORD);
    #[no_mangle]
    fn audio_PlayTrack(iTrack: libc::c_int);
}
pub type __builtin_va_list = *mut libc::c_char;
pub type va_list = __builtin_va_list;
pub type STRING = libc::c_char;
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
pub type DEPTH_MODE = libc::c_uint;
pub const DEPTH_CMP_ALWAYS_WRT_OFF: DEPTH_MODE = 3;
pub const DEPTH_CMP_LEQ_WRT_OFF: DEPTH_MODE = 2;
pub const DEPTH_CMP_ALWAYS_WRT_ON: DEPTH_MODE = 1;
pub const DEPTH_CMP_LEQ_WRT_ON: DEPTH_MODE = 0;
// Valid values for "Justify" argument of pie_DrawFormattedText().
pub type C2RustUnnamed = libc::c_uint;
// Start from end of last print and then left justify.
// Right justify.
pub const FTEXT_LEFTJUSTIFYAPPEND: C2RustUnnamed = 3;
// Centre justify.
pub const FTEXT_RIGHTJUSTIFY: C2RustUnnamed = 2;
// Left justify.
pub const FTEXT_CENTRE: C2RustUnnamed = 1;
pub const FTEXT_LEFTJUSTIFY: C2RustUnnamed = 0;
pub type CONSOLE_TEXT_JUSTIFICATION = libc::c_uint;
pub const DEFAULT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 3;
pub const CENTRE_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 2;
pub const RIGHT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 1;
pub const LEFT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _console {
    pub topX: UDWORD,
    pub topY: UDWORD,
    pub width: UDWORD,
    pub textDepth: UDWORD,
    pub permanent: BOOL,
}
pub type CONSOLE = _console;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _console_message {
    pub text: [STRING; 255],
    pub timeAdded: UDWORD,
    pub JustifyType: UDWORD,
    pub id: UDWORD,
    pub psNext: *mut _console_message,
}
/* Definition of a message */
pub type CONSOLE_MESSAGE = _console_message;
pub const ID_SOUND_WINDOWCLOSE: C2RustUnnamed_0 = 0;
pub const ID_SOUND_WINDOWOPEN: C2RustUnnamed_0 = 1;
pub type C2RustUnnamed_0 = libc::c_int;
pub const ID_MAX_SOUND: C2RustUnnamed_0 = 354;
pub const ID_SOUND_LAS_SAT_COUNTDOWN: C2RustUnnamed_0 = 353;
pub const ID_SOUND_UPLINK: C2RustUnnamed_0 = 352;
pub const ID_SOUND_PLASMA_FLAMER: C2RustUnnamed_0 = 351;
pub const ID_SOUND_LASER_HEAVY: C2RustUnnamed_0 = 350;
pub const ID_SOUND_EMP: C2RustUnnamed_0 = 349;
pub const ID_SOUND_CYBORG_HEAVY: C2RustUnnamed_0 = 348;
pub const ID_SOUND_CYBORG_GROUND: C2RustUnnamed_0 = 347;
pub const ID_SOUND_NEXUS_UNIT_NEUTRALISED: C2RustUnnamed_0 = 346;
pub const ID_SOUND_NEXUS_UNIT_ABSORBED: C2RustUnnamed_0 = 345;
pub const ID_SOUND_NEXUS_SYNAPTIC_LINK: C2RustUnnamed_0 = 344;
pub const ID_SOUND_NEXUS_STRUCTURE_NEUTRALISED: C2RustUnnamed_0 = 343;
pub const ID_SOUND_NEXUS_STRUCTURE_ABSORBED: C2RustUnnamed_0 = 342;
pub const ID_SOUND_NEXUS_RESEARCH_ABSORBED: C2RustUnnamed_0 = 341;
pub const ID_SOUND_NEXUS_PRODUCTION_COMPLETED: C2RustUnnamed_0 = 340;
pub const ID_SOUND_NEXUS_LAUGH3: C2RustUnnamed_0 = 339;
pub const ID_SOUND_NEXUS_LAUGH2: C2RustUnnamed_0 = 338;
pub const ID_SOUND_NEXUS_LAUGH1: C2RustUnnamed_0 = 337;
pub const ID_SOUND_NEXUS_DEFENCES_NEUTRALISED: C2RustUnnamed_0 = 336;
pub const ID_SOUND_NEXUS_DEFENCES_ABSORBED: C2RustUnnamed_0 = 335;
pub const ID_SOUND_INCOMING_LASER_SAT_STRIKE: C2RustUnnamed_0 = 334;
pub const ID_SOUND_LASER_SATELLITE_FIRING: C2RustUnnamed_0 = 333;
pub const ID_SOUND_TEAM_GAMMA_RESCUED: C2RustUnnamed_0 = 332;
pub const ID_SOUND_TEAM_BETA_RESCUED: C2RustUnnamed_0 = 331;
pub const ID_SOUND_TEAM_ALPHA_RESCUED: C2RustUnnamed_0 = 330;
pub const ID_SOUND_TEAM_GAMMA_ERADICATED: C2RustUnnamed_0 = 329;
pub const ID_SOUND_TEAM_BETA_ERADICATED: C2RustUnnamed_0 = 328;
pub const ID_SOUND_TEAM_ALPHA_ERADICATED: C2RustUnnamed_0 = 327;
pub const ID_SOUND_ENEMY_TRANSPORT_LANDING: C2RustUnnamed_0 = 326;
pub const ID_SOUND_ENEMY_ESCAPING: C2RustUnnamed_0 = 325;
pub const ID_SOUND_ENEMY_ESCAPED: C2RustUnnamed_0 = 324;
pub const ID_SOUND_OUT_OF_TIME: C2RustUnnamed_0 = 323;
pub const ID_SOUND_GROUP_INFECTED: C2RustUnnamed_0 = 322;
pub const ID_SOUND_STRUCTURE_INFECTED: C2RustUnnamed_0 = 321;
pub const ID_SOUND_OBJECTIVE_DESTROYED: C2RustUnnamed_0 = 320;
pub const ID_SOUND_OBJECTIVE_CAPTURED: C2RustUnnamed_0 = 319;
pub const ID_SOUND_GROUP_CAPTURED: C2RustUnnamed_0 = 318;
pub const ID_SOUND_GROUP_RESCUED: C2RustUnnamed_0 = 317;
pub const ID_SOUND_UNITS_RESCUED: C2RustUnnamed_0 = 316;
pub const ID_SOUND_CIVILIANS_RESCUED: C2RustUnnamed_0 = 315;
pub const ID_SOUND_CIVILIAN_RESCUED: C2RustUnnamed_0 = 314;
pub const ID_SOUND_STRUCTURE_CAPTURED: C2RustUnnamed_0 = 313;
pub const ID_SOUND_MISSILE_NME_DETECTED: C2RustUnnamed_0 = 312;
pub const ID_SOUND_MISSILE_SILO: C2RustUnnamed_0 = 311;
pub const ID_SOUND_SAM_SITE: C2RustUnnamed_0 = 310;
pub const ID_SOUND_NUCLEAR_REACTOR: C2RustUnnamed_0 = 309;
pub const ID_SOUND_NASDA_CENTRAL: C2RustUnnamed_0 = 308;
pub const ID_SOUND_SATELLITE_UPLINK: C2RustUnnamed_0 = 307;
pub const ID_SOUND_LANDING_ZONE: C2RustUnnamed_0 = 306;
pub const ID_SOUND_NOFAULTS: C2RustUnnamed_0 = 305;
pub const ID_SOUND_OF_SILENCE: C2RustUnnamed_0 = 304;
pub const ID_SOUND_BARB_SCREAM3: C2RustUnnamed_0 = 303;
pub const ID_SOUND_BARB_SCREAM2: C2RustUnnamed_0 = 302;
pub const ID_SOUND_BARB_SCREAM: C2RustUnnamed_0 = 301;
pub const ID_SOUND_HELP: C2RustUnnamed_0 = 300;
pub const ID_SOUND_FIRE_ROAR: C2RustUnnamed_0 = 299;
pub const ID_SOUND_ECM_TOWER: C2RustUnnamed_0 = 298;
pub const ID_SOUND_STEAM: C2RustUnnamed_0 = 297;
pub const ID_SOUND_POWER_SPARK: C2RustUnnamed_0 = 296;
pub const ID_SOUND_POWER_HUM: C2RustUnnamed_0 = 295;
pub const ID_SOUND_OIL_PUMP_2: C2RustUnnamed_0 = 294;
pub const ID_SOUND_CYBORG_MOVE: C2RustUnnamed_0 = 293;
pub const ID_SOUND_HOVER_STOP: C2RustUnnamed_0 = 292;
pub const ID_SOUND_HOVER_START: C2RustUnnamed_0 = 291;
pub const ID_SOUND_HOVER_MOVE: C2RustUnnamed_0 = 290;
pub const ID_SOUND_TREAD: C2RustUnnamed_0 = 289;
pub const ID_SOUND_VTOL_MOVE: C2RustUnnamed_0 = 288;
pub const ID_SOUND_VTOL_OFF: C2RustUnnamed_0 = 287;
pub const ID_SOUND_VTOL_LAND: C2RustUnnamed_0 = 286;
pub const ID_SOUND_BLIMP_TAKE_OFF: C2RustUnnamed_0 = 285;
pub const ID_SOUND_BLIMP_LAND: C2RustUnnamed_0 = 284;
pub const ID_SOUND_BLIMP_IDLE: C2RustUnnamed_0 = 283;
pub const ID_SOUND_BLIMP_FLIGHT: C2RustUnnamed_0 = 282;
pub const ID_SOUND_CONSTRUCTOR_SHUTDOWN: C2RustUnnamed_0 = 281;
pub const ID_SOUND_CONSTRUCTOR_MOVE: C2RustUnnamed_0 = 280;
pub const ID_SOUND_CONSTRUCTOR_MOVEOFF: C2RustUnnamed_0 = 279;
pub const ID_SOUND_NEXUS_EXPLOSION: C2RustUnnamed_0 = 278;
pub const ID_SOUND_BUILDING_FALL: C2RustUnnamed_0 = 277;
pub const ID_SOUND_BARB_SQUISH: C2RustUnnamed_0 = 276;
pub const ID_SOUND_RICOCHET_3: C2RustUnnamed_0 = 275;
pub const ID_SOUND_RICOCHET_2: C2RustUnnamed_0 = 274;
pub const ID_SOUND_RICOCHET_1: C2RustUnnamed_0 = 273;
pub const ID_SOUND_EXPLOSION_ANTITANK: C2RustUnnamed_0 = 272;
pub const ID_SOUND_EXPLOSION: C2RustUnnamed_0 = 271;
pub const ID_SOUND_EXPLOSION_LASER: C2RustUnnamed_0 = 270;
pub const ID_SOUND_EXPLOSION_SMALL: C2RustUnnamed_0 = 269;
pub const ID_SOUND_CONSTRUCTION_4: C2RustUnnamed_0 = 268;
pub const ID_SOUND_CONSTRUCTION_3: C2RustUnnamed_0 = 267;
pub const ID_SOUND_CONSTRUCTION_2: C2RustUnnamed_0 = 266;
pub const ID_SOUND_CONSTRUCTION_1: C2RustUnnamed_0 = 265;
pub const ID_SOUND_CONSTRUCTION_LOOP: C2RustUnnamed_0 = 264;
pub const ID_SOUND_CONSTRUCTION_START: C2RustUnnamed_0 = 263;
pub const ID_SOUND_WELD_2: C2RustUnnamed_0 = 262;
pub const ID_SOUND_WELD_1: C2RustUnnamed_0 = 261;
pub const ID_SOUND_NEXUS_TOWER: C2RustUnnamed_0 = 260;
pub const ID_SOUND_HIVEL_CANNON: C2RustUnnamed_0 = 259;
pub const ID_SOUND_RAPID_CANNON: C2RustUnnamed_0 = 258;
pub const ID_SOUND_ASSAULT_MG: C2RustUnnamed_0 = 257;
pub const ID_SOUND_SPLASH: C2RustUnnamed_0 = 256;
pub const ID_SOUND_BABA_MG_TOWER: C2RustUnnamed_0 = 255;
pub const ID_SOUND_BABA_MG_HEAVY: C2RustUnnamed_0 = 254;
pub const ID_SOUND_BABA_MG_3: C2RustUnnamed_0 = 253;
pub const ID_SOUND_BABA_MG_2: C2RustUnnamed_0 = 252;
pub const ID_SOUND_BABA_MG_1: C2RustUnnamed_0 = 251;
pub const ID_SOUND_HOWITZ_FLIGHT: C2RustUnnamed_0 = 250;
pub const ID_SOUND_MORTAR: C2RustUnnamed_0 = 249;
pub const ID_SOUND_BEAM_LASER: C2RustUnnamed_0 = 248;
pub const ID_SOUND_PULSE_LASER: C2RustUnnamed_0 = 247;
pub const ID_SOUND_FLAME_THROWER: C2RustUnnamed_0 = 246;
pub const ID_SOUND_MEDIUM_CANNON: C2RustUnnamed_0 = 245;
pub const ID_SOUND_SMALL_CANNON: C2RustUnnamed_0 = 244;
pub const ID_SOUND_LARGE_CANNON: C2RustUnnamed_0 = 243;
pub const ID_SOUND_GAUSSGUN: C2RustUnnamed_0 = 242;
pub const ID_SOUND_ROTARY_LASER: C2RustUnnamed_0 = 241;
pub const ID_SOUND_ROCKET: C2RustUnnamed_0 = 240;
pub const ID_SOUND_COLL_ENEMY_DESTROYED: C2RustUnnamed_0 = 239;
pub const ID_SOUND_COLL_INTERCEPT_AND_DESTROY: C2RustUnnamed_0 = 238;
pub const ID_SOUND_COLL_DIE: C2RustUnnamed_0 = 237;
pub const ID_SOUND_COLL_STARTING_ATTACK_RUN: C2RustUnnamed_0 = 236;
pub const ID_SOUND_COLL_ENGAGING: C2RustUnnamed_0 = 235;
pub const ID_SOUND_COLL_ENEMY_DETECTED: C2RustUnnamed_0 = 234;
pub const ID_SOUND_COLL_FIRE: C2RustUnnamed_0 = 233;
pub const ID_SOUND_COLL_ATTACK: C2RustUnnamed_0 = 232;
pub const ID_SOUND_COLL_DESTROYING_BIOLOGICALS: C2RustUnnamed_0 = 231;
pub const ID_SOUND_COLL_CLEANSE_AND_DESTROY: C2RustUnnamed_0 = 230;
pub const ID_SOUND_ABORTING_ATTACK_RUN3: C2RustUnnamed_0 = 229;
pub const ID_SOUND_COMMENCING_ATTACK_RUN3: C2RustUnnamed_0 = 228;
pub const ID_SOUND_LOCKED_ON3: C2RustUnnamed_0 = 227;
pub const ID_SOUND_RETURNING_TO_BASE3: C2RustUnnamed_0 = 226;
pub const ID_SOUND_ON_OUR_WAY3: C2RustUnnamed_0 = 225;
pub const ID_SOUND_ENEMY_LOCATED3: C2RustUnnamed_0 = 224;
pub const ID_SOUND_ABORTING_ATTACK_RUN2: C2RustUnnamed_0 = 223;
pub const ID_SOUND_COMMENCING_ATTACK_RUN2: C2RustUnnamed_0 = 222;
pub const ID_SOUND_LOCKED_ON2: C2RustUnnamed_0 = 221;
pub const ID_SOUND_RETURNING_TO_BASE2: C2RustUnnamed_0 = 220;
pub const ID_SOUND_ON_OUR_WAY2: C2RustUnnamed_0 = 219;
pub const ID_SOUND_ENEMY_LOCATED2: C2RustUnnamed_0 = 218;
pub const ID_SOUND_ABORTING_ATTACK_RUN1: C2RustUnnamed_0 = 217;
pub const ID_SOUND_COMMENCING_ATTACK_RUN1: C2RustUnnamed_0 = 216;
pub const ID_SOUND_LOCKED_ON1: C2RustUnnamed_0 = 215;
pub const ID_SOUND_RETURNING_TO_BASE1: C2RustUnnamed_0 = 214;
pub const ID_SOUND_ON_OUR_WAY1: C2RustUnnamed_0 = 213;
pub const ID_SOUND_ENEMY_LOCATED1: C2RustUnnamed_0 = 212;
pub const ID_SOUND_PREPARE_FOR_DUST_OFF: C2RustUnnamed_0 = 211;
pub const ID_SOUND_GO_GO_GO: C2RustUnnamed_0 = 210;
pub const ID_SOUND_GREEN_LIGHT_IN_2: C2RustUnnamed_0 = 209;
pub const ID_SOUND_GREEN_LIGHT_IN_3: C2RustUnnamed_0 = 208;
pub const ID_SOUND_GREEN_LIGHT_IN_4: C2RustUnnamed_0 = 207;
pub const ID_SOUND_GREEN_LIGHT_IN_5: C2RustUnnamed_0 = 206;
pub const ID_SOUND_ALRIGHT_BOYS: C2RustUnnamed_0 = 205;
pub const ID_SOUND_APPROACHING_LZ: C2RustUnnamed_0 = 204;
pub const ID_SOUND_RADIOCLICK_6: C2RustUnnamed_0 = 203;
pub const ID_SOUND_RADIOCLICK_5: C2RustUnnamed_0 = 202;
pub const ID_SOUND_RADIOCLICK_4: C2RustUnnamed_0 = 201;
pub const ID_SOUND_RADIOCLICK_3: C2RustUnnamed_0 = 200;
pub const ID_SOUND_RADIOCLICK_2: C2RustUnnamed_0 = 199;
pub const ID_SOUND_RADIOCLICK_1: C2RustUnnamed_0 = 198;
pub const ID_SOUND_COM_HEADING_FOR_RALLY_POINT: C2RustUnnamed_0 = 197;
pub const ID_SOUND_COM_RETURNING_FOR_REPAIR: C2RustUnnamed_0 = 196;
pub const ID_SOUND_COM_UNABLE_TO_COMPLY: C2RustUnnamed_0 = 195;
pub const ID_SOUND_COM_NO_ROUTE_AVAILABLE: C2RustUnnamed_0 = 194;
pub const ID_SOUND_COM_ROUTE_OBSTRUCTED: C2RustUnnamed_0 = 193;
pub const ID_SOUND_COM_ENEMY_VTOLS_DETECTED: C2RustUnnamed_0 = 192;
pub const ID_SOUND_COM_ENEMY_BATTERY_DETECTED: C2RustUnnamed_0 = 191;
pub const ID_SOUND_COM_NEXUS_DETECTED: C2RustUnnamed_0 = 190;
pub const ID_SOUND_COM_NEXUS_TURRET_DETECTED: C2RustUnnamed_0 = 189;
pub const ID_SOUND_COM_NEXUS_TOWER_DETECTED: C2RustUnnamed_0 = 188;
pub const ID_SOUND_COM_FRIENDLY_LZ_DETECTED: C2RustUnnamed_0 = 187;
pub const ID_SOUND_COM_ENEMY_LZ_DETECTED: C2RustUnnamed_0 = 186;
pub const ID_SOUND_COM_ENEMY_TRANSPORT_DETECTED: C2RustUnnamed_0 = 185;
pub const ID_SOUND_COM_ALLY_DETECTED: C2RustUnnamed_0 = 184;
pub const ID_SOUND_COM_ENEMY_BASE_DETECTED: C2RustUnnamed_0 = 183;
pub const ID_SOUND_COM_ENEMY_DETECTED: C2RustUnnamed_0 = 182;
pub const ID_SOUND_COM_ARTEFACT_DETECTED: C2RustUnnamed_0 = 181;
pub const ID_SOUND_COM_RESOURCE_DETECTED: C2RustUnnamed_0 = 180;
pub const ID_SOUND_COM_SCAV_OUTPOST_DETECTED: C2RustUnnamed_0 = 179;
pub const ID_SOUND_COM_SCAV_BASE_DETECTED: C2RustUnnamed_0 = 178;
pub const ID_SOUND_COM_SCAVS_DETECTED: C2RustUnnamed_0 = 177;
pub const ID_SOUND_COMMANDER: C2RustUnnamed_0 = 176;
pub const ID_SOUND_REPORTING: C2RustUnnamed_0 = 175;
pub const ID_SOUND_GROUP_9: C2RustUnnamed_0 = 174;
pub const ID_SOUND_GROUP_8: C2RustUnnamed_0 = 173;
pub const ID_SOUND_GROUP_7: C2RustUnnamed_0 = 172;
pub const ID_SOUND_GROUP_6: C2RustUnnamed_0 = 171;
pub const ID_SOUND_GROUP_5: C2RustUnnamed_0 = 170;
pub const ID_SOUND_GROUP_4: C2RustUnnamed_0 = 169;
pub const ID_SOUND_GROUP_3: C2RustUnnamed_0 = 168;
pub const ID_SOUND_GROUP_2: C2RustUnnamed_0 = 167;
pub const ID_SOUND_GROUP_1: C2RustUnnamed_0 = 166;
pub const ID_SOUND_GROUP_0: C2RustUnnamed_0 = 165;
pub const ID_SOUND_GROUP: C2RustUnnamed_0 = 164;
pub const ID_UNITS_TRANSFER: C2RustUnnamed_0 = 163;
pub const ID_TECHNOLOGY_TRANSFER: C2RustUnnamed_0 = 162;
pub const ID_SENSOR_DOWNLOAD: C2RustUnnamed_0 = 161;
pub const ID_POWER_TRANSMIT: C2RustUnnamed_0 = 160;
pub const ID_GIFT: C2RustUnnamed_0 = 159;
pub const ID_CLAN_EXIT: C2RustUnnamed_0 = 158;
pub const ID_CLAN_ENTER: C2RustUnnamed_0 = 157;
pub const ID_ALLIANCE_OFF: C2RustUnnamed_0 = 156;
pub const ID_ALLIANCE_BRO: C2RustUnnamed_0 = 155;
pub const ID_ALLIANCE_ACC: C2RustUnnamed_0 = 154;
pub const ID_SOUND_ENTERING_WARZONE: C2RustUnnamed_0 = 153;
pub const ID_SOUND_MISSILE_CODES_CRACKED: C2RustUnnamed_0 = 152;
pub const ID_SOUND_3RD_MISSILE_CODES_DECIPHERED: C2RustUnnamed_0 = 151;
pub const ID_SOUND_2ND_MISSILE_CODES_DECIPHERED: C2RustUnnamed_0 = 150;
pub const ID_SOUND_1ST_MISSILE_CODES_DECIPHERED: C2RustUnnamed_0 = 149;
pub const ID_SOUND_MISSILE_CODES_DECIPHERED: C2RustUnnamed_0 = 148;
pub const ID_SOUND_YOU_ARE_DEFEATED: C2RustUnnamed_0 = 147;
pub const ID_SOUND_SYSTEM_FAILURE_IMMINENT: C2RustUnnamed_0 = 146;
pub const ID_SOUND_UNIT_CAPTURED: C2RustUnnamed_0 = 145;
pub const ID_SOUND_1_MINUTE_REMAINING: C2RustUnnamed_0 = 144;
pub const ID_SOUND_2_MINUTES_REMAINING: C2RustUnnamed_0 = 143;
pub const ID_SOUND_3_MINUTES_REMAINING: C2RustUnnamed_0 = 142;
pub const ID_SOUND_5_MINUTES_REMAINING: C2RustUnnamed_0 = 141;
pub const ID_SOUND_10_MINUTES_REMAINING: C2RustUnnamed_0 = 140;
pub const ID_SOUND_MISSION_TIMER_ACTIVATED: C2RustUnnamed_0 = 139;
pub const ID_SOUND_OBJECTIVE_FAILED: C2RustUnnamed_0 = 138;
pub const ID_SOUND_OBJECTIVE_ACCOMPLISHED: C2RustUnnamed_0 = 137;
pub const ID_SOUND_MISSION_SUCCESSFUL: C2RustUnnamed_0 = 136;
pub const ID_SOUND_MISSION_FAILED: C2RustUnnamed_0 = 135;
pub const ID_SOUND_INCOMING_INTELLIGENCE_REPORT: C2RustUnnamed_0 = 134;
pub const ID_SOUND_INCOMING_TRANSMISSION: C2RustUnnamed_0 = 133;
pub const ID_SOUND_TECHNOLOGY_TAKEN: C2RustUnnamed_0 = 132;
pub const ID_SOUND_RESEARCH_STOLEN: C2RustUnnamed_0 = 131;
pub const ID_SOUND_MISSION_RESULTS: C2RustUnnamed_0 = 130;
pub const ID_SOUND_WARZONE_ACTIVE: C2RustUnnamed_0 = 129;
pub const ID_SOUND_WARZONE_PAUSED: C2RustUnnamed_0 = 128;
pub const ID_SOUND_MISSION_UPDATE: C2RustUnnamed_0 = 127;
pub const ID_SOUND_MISSION_OBJECTIVE: C2RustUnnamed_0 = 126;
pub const ID_SOUND_TRANSPORT_UNABLE_TO_LAND: C2RustUnnamed_0 = 125;
pub const ID_SOUND_TRANSPORT_RETURNING_TO_BASE: C2RustUnnamed_0 = 124;
pub const ID_SOUND_LZ_CLEAR: C2RustUnnamed_0 = 123;
pub const ID_SOUND_LZ_COMPROMISED: C2RustUnnamed_0 = 122;
pub const ID_SOUND_TRANSPORT_REPAIRING: C2RustUnnamed_0 = 121;
pub const ID_SOUND_TRANSPORT_UNDER_ATTACK: C2RustUnnamed_0 = 120;
pub const ID_SOUND_TRANSPORT_LANDING: C2RustUnnamed_0 = 119;
pub const ID_SOUND_REINFORCEMENTS_IN_TRANSIT: C2RustUnnamed_0 = 118;
pub const ID_SOUND_REINFORCEMENTS_AVAILABLE: C2RustUnnamed_0 = 117;
pub const ID_SOUND_NO_ROUTE_AVAILABLE: C2RustUnnamed_0 = 116;
pub const ID_SOUND_ROUTE_OBSTRUCTED: C2RustUnnamed_0 = 115;
pub const ID_SOUND_COMMANDER_REPORTING: C2RustUnnamed_0 = 114;
pub const ID_SOUND_GROUP_REPORTING: C2RustUnnamed_0 = 113;
pub const ID_SOUND_ASSIGNED_TO_COMMANDER: C2RustUnnamed_0 = 112;
pub const ID_SOUND_EXCELLENT: C2RustUnnamed_0 = 111;
pub const ID_SOUND_WELL_DONE: C2RustUnnamed_0 = 110;
pub const ID_SOUND_THAT_IS_INCORRECT: C2RustUnnamed_0 = 109;
pub const ID_SOUND_NO: C2RustUnnamed_0 = 108;
pub const ID_SOUND_NOT_POSSIBLE_TRY_AGAIN: C2RustUnnamed_0 = 107;
pub const ID_SOUND_SCATTER: C2RustUnnamed_0 = 106;
pub const ID_SOUND_RECYCLING: C2RustUnnamed_0 = 105;
pub const ID_SOUND_RETURN_TO_LZ: C2RustUnnamed_0 = 104;
pub const ID_SOUND_PATROL: C2RustUnnamed_0 = 103;
pub const ID_SOUND_PURSUE: C2RustUnnamed_0 = 102;
pub const ID_SOUND_GUARD: C2RustUnnamed_0 = 101;
pub const ID_SOUND_HOLD_POSITION: C2RustUnnamed_0 = 100;
pub const ID_SOUND_CEASEFIRE: C2RustUnnamed_0 = 99;
pub const ID_SOUND_RETURN_FIRE: C2RustUnnamed_0 = 98;
pub const ID_SOUND_FIRE_AT_WILL: C2RustUnnamed_0 = 97;
pub const ID_SOUND_NO_RETREAT: C2RustUnnamed_0 = 96;
pub const ID_SOUND_RETREAT_AT_HEAVY_DAMAGE: C2RustUnnamed_0 = 95;
pub const ID_SOUND_RETREAT_AT_MEDIUM_DAMAGE: C2RustUnnamed_0 = 94;
pub const ID_SOUND_OPTIMUM_RANGE: C2RustUnnamed_0 = 93;
pub const ID_SOUND_LONG_RANGE: C2RustUnnamed_0 = 92;
pub const ID_SOUND_SHORT_RANGE: C2RustUnnamed_0 = 91;
pub const ID_SOUND_COMMAND_CONSOLE_ACTIVATED: C2RustUnnamed_0 = 90;
pub const ID_SOUND_INTERCEPTORS_ASSIGNED: C2RustUnnamed_0 = 89;
pub const ID_SOUND_ASSIGNED: C2RustUnnamed_0 = 88;
pub const ID_SOUND_VTOLS_ENGAGING: C2RustUnnamed_0 = 87;
pub const ID_SOUND_REARMING: C2RustUnnamed_0 = 86;
pub const ID_SOUND_INTERCEPTORS_LAUNCHED: C2RustUnnamed_0 = 85;
pub const ID_SOUND_BATTERY_FIRING_COUNTER_ATTACK: C2RustUnnamed_0 = 84;
pub const ID_SOUND_ENEMY_BATTERY_LOCATED: C2RustUnnamed_0 = 83;
pub const ID_SOUND_ASSIGNED_TO_COUNTER_RADAR: C2RustUnnamed_0 = 82;
pub const ID_SOUND_SENSOR_LOCKED_ON: C2RustUnnamed_0 = 81;
pub const ID_SOUND_ASSIGNED_TO_SENSOR: C2RustUnnamed_0 = 80;
pub const ID_SOUND_UNIT_RETURNING_FOR_REPAIR: C2RustUnnamed_0 = 79;
pub const ID_SOUND_UNIT_RETREATING: C2RustUnnamed_0 = 78;
pub const ID_SOUND_UNIT_DESTROYED: C2RustUnnamed_0 = 77;
pub const ID_SOUND_UNIT_UNDER_ATTACK: C2RustUnnamed_0 = 76;
pub const ID_SOUND_LZ2: C2RustUnnamed_0 = 75;
pub const ID_SOUND_LZ1: C2RustUnnamed_0 = 74;
pub const ID_SOUND_ENEMY_LZ: C2RustUnnamed_0 = 73;
pub const ID_SOUND_INCOMING_ENEMY_TRANSPORT: C2RustUnnamed_0 = 72;
pub const ID_SOUND_ENEMY_BASE_ERADICATED: C2RustUnnamed_0 = 71;
pub const ID_SOUND_ENEMY_BASE: C2RustUnnamed_0 = 70;
pub const ID_SOUND_SCAVENGER_BASE_ERADICATED: C2RustUnnamed_0 = 69;
pub const ID_SOUND_SCAVENGER_OUTPOST_ERADICATED: C2RustUnnamed_0 = 68;
pub const ID_SOUND_SCAVENGER_OUTPOST: C2RustUnnamed_0 = 67;
pub const ID_SOUND_SCAVENGER_BASE: C2RustUnnamed_0 = 66;
pub const ID_SOUND_ENEMY_VTOLS_DETECTED: C2RustUnnamed_0 = 65;
pub const ID_SOUND_ENEMY_BATTERY_DETECTED: C2RustUnnamed_0 = 64;
pub const ID_SOUND_NEXUS_UNIT_DETECTED: C2RustUnnamed_0 = 63;
pub const ID_SOUND_NEXUS_TURRET_DETECTED: C2RustUnnamed_0 = 62;
pub const ID_SOUND_NEXUS_TOWER_DETECTED: C2RustUnnamed_0 = 61;
pub const ID_SOUND_FRIENDLY_LZ_DETECTED: C2RustUnnamed_0 = 60;
pub const ID_SOUND_ENEMY_LZ_DETECTED: C2RustUnnamed_0 = 59;
pub const ID_SOUND_ENEMY_TRANSPORT_DETECTED: C2RustUnnamed_0 = 58;
pub const ID_SOUND_ALLY_DETECTED: C2RustUnnamed_0 = 57;
pub const ID_SOUND_ENEMY_BASE_DETECTED: C2RustUnnamed_0 = 56;
pub const ID_SOUND_ENEMY_UNIT_DETECTED: C2RustUnnamed_0 = 55;
pub const ID_SOUND_ARTEFACT_DISC: C2RustUnnamed_0 = 54;
pub const ID_SOUND_POWER_RESOURCE: C2RustUnnamed_0 = 53;
pub const ID_SOUND_SCAVENGER_OUTPOST_DETECTED: C2RustUnnamed_0 = 52;
pub const ID_SOUND_SCAVENGER_BASE_DETECTED: C2RustUnnamed_0 = 51;
pub const ID_SOUND_SCAVENGERS_DETECTED: C2RustUnnamed_0 = 50;
pub const ID_SOUND_UNIT_REPAIRED: C2RustUnnamed_0 = 49;
pub const ID_SOUND_DELIVERY_POINT_ASSIGNED_TO: C2RustUnnamed_0 = 48;
pub const ID_SOUND_DELIVERY_POINT_ASSIGNED: C2RustUnnamed_0 = 47;
pub const ID_SOUND_PRODUCTION_CANCELLED: C2RustUnnamed_0 = 46;
pub const ID_SOUND_PRODUCTION_PAUSED: C2RustUnnamed_0 = 45;
pub const ID_SOUND_DROID_COMPLETED: C2RustUnnamed_0 = 44;
pub const ID_SOUND_PRODUCTION_STARTED: C2RustUnnamed_0 = 43;
pub const ID_SOUND_CYBORG_RESEARCH_COMPLETED: C2RustUnnamed_0 = 42;
pub const ID_SOUND_WEAPON_RESEARCH_COMPLETED: C2RustUnnamed_0 = 41;
pub const ID_SOUND_SYSTEMS_RESEARCH_COMPLETED: C2RustUnnamed_0 = 40;
pub const ID_SOUND_VEHICLE_RESEARCH_COMPLETED: C2RustUnnamed_0 = 39;
pub const ID_SOUND_COMPUTER_RESEARCH_COMPLETED: C2RustUnnamed_0 = 38;
pub const ID_SOUND_POWER_RESEARCH_COMPLETED: C2RustUnnamed_0 = 37;
pub const ID_SOUND_STRUCTURE_RESEARCH_COMPLETED: C2RustUnnamed_0 = 36;
pub const ID_SOUND_MAJOR_RESEARCH: C2RustUnnamed_0 = 35;
pub const ID_SOUND_RESEARCH_COMPLETED: C2RustUnnamed_0 = 34;
pub const ID_SOUND_NEW_CYBORG_AVAILABLE: C2RustUnnamed_0 = 33;
pub const ID_SOUND_NEW_COMPONENT_AVAILABLE: C2RustUnnamed_0 = 32;
pub const ID_SOUND_NEW_STRUCTURE_AVAILABLE: C2RustUnnamed_0 = 31;
pub const ID_SOUND_NEW_RESEARCH_PROJ_AVAILABLE: C2RustUnnamed_0 = 30;
pub const ID_SOUND_ARTIFACT_RECOVERED: C2RustUnnamed_0 = 29;
pub const ID_SOUND_ARTIFACT: C2RustUnnamed_0 = 28;
pub const ID_SOUND_RESEARCH_FACILITY_REQUIRED: C2RustUnnamed_0 = 27;
pub const ID_SOUND_POWER_GENERATOR_REQUIRED: C2RustUnnamed_0 = 26;
pub const ID_SOUND_POWER_TRANSFER_IN_PROGRESS: C2RustUnnamed_0 = 25;
pub const ID_SOUND_RESOURCE_DEPLETED: C2RustUnnamed_0 = 24;
pub const ID_SOUND_DERRICK_DESTROYED: C2RustUnnamed_0 = 23;
pub const ID_SOUND_DERRICK_UNDER_ATTACK: C2RustUnnamed_0 = 22;
pub const ID_SOUND_RESOURCE_HERE: C2RustUnnamed_0 = 21;
pub const ID_SOUND_POWER_LOW: C2RustUnnamed_0 = 20;
pub const ID_SOUND_POWER_GENERATOR_DESTROYED: C2RustUnnamed_0 = 19;
pub const ID_SOUND_POWER_GENERATOR_UNDER_ATTACK: C2RustUnnamed_0 = 18;
pub const ID_SOUND_STRUCTURE_DEMOLISHED: C2RustUnnamed_0 = 17;
pub const ID_SOUND_STRUCTURE_REPAIR_IN_PROGRESS: C2RustUnnamed_0 = 16;
pub const ID_SOUND_STRUCTURE_UNDER_ATTACK: C2RustUnnamed_0 = 15;
pub const ID_SOUND_STRUCTURE_COMPLETED: C2RustUnnamed_0 = 14;
pub const ID_SOUND_CONSTRUCTION_STARTED: C2RustUnnamed_0 = 13;
pub const ID_SOUND_DESIGN_COMPLETED: C2RustUnnamed_0 = 12;
pub const ID_SOUND_PROPULSION_SELECTED: C2RustUnnamed_0 = 11;
pub const ID_SOUND_BODY_SELECTED: C2RustUnnamed_0 = 10;
pub const ID_SOUND_TURRET_SELECTED: C2RustUnnamed_0 = 9;
pub const ID_SOUND_GAME_SHUTDOWN: C2RustUnnamed_0 = 8;
pub const ID_SOUND_MESSAGEEND: C2RustUnnamed_0 = 7;
pub const ID_SOUND_BUILD_FAIL: C2RustUnnamed_0 = 6;
pub const ID_SOUND_ZOOM_ON_RADAR: C2RustUnnamed_0 = 5;
pub const FE_AUDIO_MESSAGEEND: C2RustUnnamed_0 = 4;
pub const ID_SOUND_BUTTON_CLICK_5: C2RustUnnamed_0 = 3;
pub const ID_SOUND_SELECT: C2RustUnnamed_0 = 2;
pub const NO_SOUND: C2RustUnnamed_0 = -1;
// Text of the message
// When was it added to our list?
//UDWORD	screenIndex;							// Info for justification
/* Alex McLean, Pumpkin Studios, EIDOS Interactive */
/* Is the console history on or off */
static mut bConsoleDropped: BOOL = 0 as libc::c_int;
/* Stores the console dimensions and states */
static mut mainConsole: CONSOLE =
    CONSOLE{topX: 0, topY: 0, width: 0, textDepth: 0, permanent: 0,};
/* Static storage for the maximum possible number of console messages */
static mut consoleStorage: [CONSOLE_MESSAGE; 64] =
    [CONSOLE_MESSAGE{text: [0; 255],
                     timeAdded: 0,
                     JustifyType: 0,
                     id: 0,
                     psNext:
                         0 as *const _console_message as
                             *mut _console_message,}; 64];
static mut history: [UDWORD; 32] = [0; 32];
/* Pointer to linked list of active messages - points to elements of the array above */
static mut consoleMessages: *mut CONSOLE_MESSAGE =
    0 as *const CONSOLE_MESSAGE as *mut CONSOLE_MESSAGE;
/* Where in the array are we - it's cyclic */
static mut messageIndex: UDWORD = 0;
/* How many lines in the console history */
static mut consoleDrop: UDWORD = 32 as libc::c_int as UDWORD;
static mut maxDrop: UDWORD = 0;
/* Console history state */
static mut dropState: UDWORD = 0;
/* How many messages are presently active */
static mut numActiveMessages: UDWORD = 0;
/* How long do messages last for? */
static mut messageDuration: UDWORD = 0;
static mut lastDropChange: UDWORD = 0 as libc::c_int as UDWORD;
/* Is there a box under the console text? */
static mut bTextBoxActive: BOOL = 0;
/* Is the console being displayed? */
static mut bConsoleDisplayEnabled: BOOL = 0;
/* How many lines are displayed */
static mut consoleVisibleLines: UDWORD = 0;
/* Whether new messages are allowed to be added */
static mut allowNewMessages: libc::c_int = 0;
/* What's the default justification */
static mut defJustification: CONSOLE_TEXT_JUSTIFICATION = LEFT_JUSTIFY;
static mut messageId: UDWORD = 0;
// unique ID
// Global string for new console messages.
#[no_mangle]
pub static mut ConsoleString: [libc::c_char; 255] = [0; 255];
/* Sets the system up */
#[no_mangle]
pub unsafe extern "C" fn initConsoleMessages() {
    messageIndex = 0 as libc::c_int as UDWORD;
    /* Console can extend to half screen height */
    maxDrop =
        pie_GetVideoBufferHeight().wrapping_div(iV_GetTextLineSize() as
                                                    libc::c_uint).wrapping_div(2
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   libc::c_uint); //MAX_DROP;
    if maxDrop > 32 as libc::c_int as libc::c_uint {
        maxDrop = 32 as libc::c_int as UDWORD
    }
    consoleDrop = maxDrop;
    dropState = 4 as libc::c_int as UDWORD;
    /* No active messages to begin with */
    numActiveMessages = 0 as libc::c_int as UDWORD;
    lastDropChange = 0 as libc::c_int as UDWORD;
    bConsoleDropped = 0 as libc::c_int;
    /* Linked list is empty */
    consoleMessages = 0 as *mut CONSOLE_MESSAGE;
    /* Setup how long messages are displayed for... */
    setConsoleMessageDuration((1000 as libc::c_int * 3 as libc::c_int) as
                                  UDWORD);
    /* No box under the text */
    setConsoleBackdropStatus(1 as libc::c_int);
    /* Turn on the console display */
    enableConsoleDisplay(1 as libc::c_int);
    /* Set left justification as default */
    setDefaultConsoleJust(LEFT_JUSTIFY);
    /*	Set up the console size and postion
		x,y,width */
    setConsoleSizePos(16 as libc::c_int as UDWORD,
                      16 as libc::c_int as UDWORD,
                      pie_GetVideoBufferWidth().wrapping_sub(32 as libc::c_int
                                                                 as
                                                                 libc::c_uint));
    setConsoleLineInfo((64 as libc::c_int / 4 as libc::c_int +
                            4 as libc::c_int) as UDWORD);
    /* We're not initially having permanent messages */
    setConsolePermanence(0 as libc::c_int, 1 as libc::c_int);
    /* Allow new messages */
    permitNewConsoleMessages(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn toggleConsoleDrop() {
    /* If it's closed ... */
    if bConsoleDropped == 0 as libc::c_int {
        dropState = 1 as libc::c_int as UDWORD;
        consoleDrop = 0 as libc::c_int as UDWORD;
        bConsoleDropped = 1 as libc::c_int;
        audio_PlayTrack(ID_SOUND_WINDOWOPEN as libc::c_int);
    } else {
        /* It's already open (or opening) */
        dropState = 2 as libc::c_int as UDWORD;
        audio_PlayTrack(ID_SOUND_WINDOWCLOSE as libc::c_int);
    };
}
/* Adds a string to the console. */
unsafe extern "C" fn _addConsoleMessage(mut messageText: *mut STRING,
                                        mut jusType:
                                            CONSOLE_TEXT_JUSTIFICATION)
 -> BOOL {
    let mut textLength: UDWORD = 0;
    let mut psMessage: *mut CONSOLE_MESSAGE = 0 as *mut CONSOLE_MESSAGE;
    /* Just don't add it if there's too many already */
    if numActiveMessages >=
           (64 as libc::c_int - 1 as libc::c_int) as libc::c_uint {
        return 0 as libc::c_int
    }
    /* Don't allow it to be added if we've disabled adding of new messages */
    if allowNewMessages == 0 { return 0 as libc::c_int }
    /* Is the string too long? */
    textLength = strlen(messageText);
    if textLength < 255 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Attempt to add a message to the console that exceeds MAX_CONSOLE_STRING_LENGTH\x00"
                  as *const u8 as *const libc::c_char);
    };
    if textLength < 255 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"console.c\x00" as *const u8 as *const libc::c_char,
              206 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 19],
                                        &[libc::c_char; 19]>(b"_addConsoleMessage\x00")).as_ptr(),
              b"textLength<MAX_CONSOLE_STRING_LENGTH\x00" as *const u8 as
                  *const libc::c_char);
    };
    /* Are we using a defualt justification? */
    if jusType as libc::c_uint ==
           DEFAULT_JUSTIFY as libc::c_int as libc::c_uint {
        /* Then set it */
        jusType = defJustification
    }
    /* Precalculate and store (quicker!) the indent for justified text */
    match jusType as libc::c_uint {
        0 => {
            /* Allign to left edge of screen */
            consoleStorage[messageIndex as usize].JustifyType =
                FTEXT_LEFTJUSTIFY as libc::c_int as UDWORD
        }
        1 => {
            /* Allign to right edge of screen */
            consoleStorage[messageIndex as usize].JustifyType =
                FTEXT_RIGHTJUSTIFY as libc::c_int as UDWORD
        }
        2 => {
            /* Allign to centre of the screen,NOT TO CENTRE OF CONSOLE!!!!!! */
            consoleStorage[messageIndex as usize].JustifyType =
                FTEXT_CENTRE as libc::c_int as UDWORD
        }
        _ => {
            /* Gone tits up by the looks of it */
            debug(LOG_ERROR,
                  b"Weirdy type of text justification for console print\x00"
                      as *const u8 as *const libc::c_char);
            abort();
        }
    }
    /* Copy over the text of the message */
    strcpy(consoleStorage[messageIndex as usize].text.as_mut_ptr(),
           messageText);
    /* Set the time when it was added - this might not be needed */
    consoleStorage[messageIndex as usize].timeAdded = gameTime2;
    /* This is the present newest message */
    consoleStorage[messageIndex as usize].psNext = 0 as *mut _console_message;
    consoleStorage[messageIndex as usize].id = 0 as libc::c_int as UDWORD;
    /* Are there no messages? */
    if consoleMessages.is_null() {
        consoleMessages =
            &mut *consoleStorage.as_mut_ptr().offset(messageIndex as isize) as
                *mut CONSOLE_MESSAGE
    } else {
        /* Get to the last element in our message list */
        psMessage = consoleMessages;
        while !(*psMessage).psNext.is_null() {
            /* NOP */
            psMessage = (*psMessage).psNext
        }
        /* Add it to the end */
        (*psMessage).psNext =
            &mut *consoleStorage.as_mut_ptr().offset(messageIndex as isize) as
                *mut CONSOLE_MESSAGE
    }
    /* Move on in our array */
    let fresh0 = messageIndex;
    messageIndex = messageIndex.wrapping_add(1);
    if fresh0 >= (64 as libc::c_int - 1 as libc::c_int) as libc::c_uint {
        /* Reset */
        messageIndex = 0 as libc::c_int as UDWORD
    }
    /* There's one more active console message */
    numActiveMessages = numActiveMessages.wrapping_add(1);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn addConsoleMessage(mut messageText: *mut STRING,
                                           mut jusType:
                                               CONSOLE_TEXT_JUSTIFICATION)
 -> BOOL {
    return _addConsoleMessage(messageText, jusType);
}
#[no_mangle]
pub unsafe extern "C" fn getNumberConsoleMessages() -> UDWORD {
    return numActiveMessages;
}
#[no_mangle]
pub unsafe extern "C" fn updateConsoleMessages() {
    if dropState == 1 as libc::c_int as libc::c_uint {
        if gameTime.wrapping_sub(lastDropChange) >
               15 as libc::c_int as libc::c_uint {
            lastDropChange = gameTime;
            consoleDrop = consoleDrop.wrapping_add(1);
            if consoleDrop > maxDrop {
                //MAX_DROP)
                consoleDrop = maxDrop; //MAX_DROP;
                dropState = 3 as libc::c_int as UDWORD
            }
        }
    } else if dropState == 2 as libc::c_int as libc::c_uint {
        if gameTime.wrapping_sub(lastDropChange) >
               15 as libc::c_int as libc::c_uint {
            lastDropChange = gameTime;
            if consoleDrop != 0 {
                consoleDrop = consoleDrop.wrapping_sub(1)
            } else {
                dropState = 4 as libc::c_int as UDWORD;
                bConsoleDropped = 0 as libc::c_int
            }
        }
    }
    /* Don't do anything for DROP_STATIC */
    /* If there are no messages or we're on permanent then exit */
    if consoleMessages.is_null() || mainConsole.permanent != 0 { return }
    /* Time to kill the top one ?*/
    if gameTime2.wrapping_sub((*consoleMessages).timeAdded) > messageDuration
       {
        let fresh1 = messageId;
        messageId = messageId.wrapping_add(1);
        (*consoleMessages).id = fresh1;
        /* Is this the only message? */
        if (*consoleMessages).psNext.is_null() {
            /* Then list is now empty */
            consoleMessages = 0 as *mut CONSOLE_MESSAGE
        } else {
            /* Otherwise point it at the next one */
            consoleMessages = (*consoleMessages).psNext
        }
        /* There's one less active console message */
        numActiveMessages = numActiveMessages.wrapping_sub(1)
    };
}
/*
	Allows us to specify how long messages will stay on screen for.
*/
#[no_mangle]
pub unsafe extern "C" fn setConsoleMessageDuration(mut time: UDWORD) {
    messageDuration = time;
}
/*
	Allows us to remove the top message on screen.
	This and the function above should be sufficient to allow
	us to put up messages that stay there until we remove them
	ourselves - be sure and reset message duration afterwards
*/
#[no_mangle]
pub unsafe extern "C" fn removeTopConsoleMessage() {
    /* No point unless there is at least one */
    if !consoleMessages.is_null() {
        /* Is this the only message? */
        if (*consoleMessages).psNext.is_null() {
            /* Then list is now empty */
            consoleMessages = 0 as *mut CONSOLE_MESSAGE
        } else {
            /* Otherwise point it at the next one */
            consoleMessages = (*consoleMessages).psNext
        }
        /* There's one less active console message */
        numActiveMessages = numActiveMessages.wrapping_sub(1)
    };
}
/* Clears all console messages */
#[no_mangle]
pub unsafe extern "C" fn flushConsoleMessages() {
    consoleMessages = 0 as *mut CONSOLE_MESSAGE;
    numActiveMessages = 0 as libc::c_int as UDWORD;
    messageId = 0 as libc::c_int as UDWORD;
}
/* Displays all the console messages */
#[no_mangle]
pub unsafe extern "C" fn displayConsoleMessages() {
    let mut psMessage: *mut CONSOLE_MESSAGE = 0 as *mut CONSOLE_MESSAGE;
    let mut numProcessed: UDWORD = 0;
    let mut linePitch: UDWORD = 0;
    let mut boxDepth: UDWORD = 0;
    let mut drop_0: UDWORD = 0;
    let mut MesY: UDWORD = 0;
    let mut clipDepth: UDWORD = 0;
    let mut exceed: UDWORD = 0;
    /* Are there any to display? */
    if consoleMessages.is_null() && bConsoleDropped == 0 {
        /* No point - so get out */
        return
    }
    /* Return if it's disabled */
    if bConsoleDisplayEnabled == 0 { return }
    /* Haven't done any yet */
    numProcessed = 0 as libc::c_int as UDWORD;
    /* Get the travel to the next line */
    linePitch = iV_GetTextLineSize() as UDWORD;
    pie_SetDepthBufferStatus(DEPTH_CMP_ALWAYS_WRT_ON);
    pie_SetFogStatus(0 as libc::c_int);
    iV_SetTextColour(-(1 as libc::c_int) as SWORD);
    drop_0 = 0 as libc::c_int as UDWORD;
    if bConsoleDropped != 0 { drop_0 = displayOldMessages() }
    if consoleMessages.is_null() { return }
    /* Do we want a box under it? */
    if bTextBoxActive != 0 {
        psMessage = consoleMessages;
        exceed = 0 as libc::c_int as UDWORD;
        while !psMessage.is_null() && numProcessed < consoleVisibleLines &&
                  exceed < 4 as libc::c_int as libc::c_uint {
            if iV_GetTextWidth((*psMessage).text.as_mut_ptr()) as UDWORD >
                   mainConsole.width {
                exceed = exceed.wrapping_add(1)
            }
            psMessage = (*psMessage).psNext
        }
        /* How big a box is necessary? */
        boxDepth =
            if numActiveMessages > consoleVisibleLines {
                consoleVisibleLines.wrapping_sub(1 as libc::c_int as
                                                     libc::c_uint)
            } else {
                numActiveMessages.wrapping_sub(1 as libc::c_int as
                                                   libc::c_uint)
            };
        /* Add on the extra - hope it doesn't exceed two lines! */
        boxDepth =
            (boxDepth as libc::c_uint).wrapping_add(exceed) as UDWORD as
                UDWORD;
        /* GET RID OF THE MAGIC NUMBERS BELOW */
        clipDepth =
            mainConsole.topY.wrapping_add(boxDepth.wrapping_mul(linePitch)).wrapping_add(4
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_uint).wrapping_add(drop_0);
        if clipDepth > pie_GetVideoBufferHeight().wrapping_sub(linePitch) {
            clipDepth = pie_GetVideoBufferHeight().wrapping_sub(linePitch)
        }
        pie_TransBoxFill(mainConsole.topX.wrapping_sub(4 as libc::c_int as
                                                           libc::c_uint) as
                             SDWORD,
                         mainConsole.topY.wrapping_sub(mainConsole.textDepth).wrapping_sub(4
                                                                                               as
                                                                                               libc::c_int
                                                                                               as
                                                                                               libc::c_uint).wrapping_add(drop_0).wrapping_add(1
                                                                                                                                                   as
                                                                                                                                                   libc::c_int
                                                                                                                                                   as
                                                                                                                                                   libc::c_uint)
                             as SDWORD,
                         mainConsole.topX.wrapping_add(mainConsole.width) as
                             SDWORD, clipDepth as SDWORD);
    }
    /* Stop when we've drawn enough or we're at the end */
    MesY = mainConsole.topY.wrapping_add(drop_0);
    psMessage = consoleMessages;
    numProcessed = 0 as libc::c_int as UDWORD;
    while !psMessage.is_null() && numProcessed < consoleVisibleLines &&
              MesY < pie_GetVideoBufferHeight().wrapping_sub(linePitch) {
        /* Draw the text string */
        MesY =
            pie_DrawFormattedText((*psMessage).text.as_mut_ptr(),
                                  mainConsole.topX, MesY, mainConsole.width,
                                  (*psMessage).JustifyType, 0 as libc::c_int);
        /* Move on */
        numProcessed = numProcessed.wrapping_add(1);
        psMessage = (*psMessage).psNext
    };
}
/* Do up to the last 8 messages.... Returns how many it did... */
#[no_mangle]
pub unsafe extern "C" fn displayOldMessages() -> UDWORD {
    let mut thisIndex: UDWORD = 0;
    let mut i: UDWORD = 0;
    let mut count: UDWORD = 0;
    let mut bGotIt: BOOL = 0;
    let mut bQuit: BOOL = 0;
    let mut marker: UDWORD = 0 as libc::c_int as UDWORD;
    let mut linePitch: UDWORD = 0;
    let mut MesY: UDWORD = 0;
    //UDWORD	buildWidth;
//STRING	buildData[255];
    /* Check there actually are any messages */
    thisIndex = messageId;
    count = 0 as libc::c_int as UDWORD;
    if thisIndex != 0 {
        bQuit = 0 as libc::c_int;
        while bQuit == 0 {
            i = 0 as libc::c_int as UDWORD;
            bGotIt = 0 as libc::c_int;
            while i < 64 as libc::c_int as libc::c_uint && bGotIt == 0 {
                if consoleStorage[i as usize].id ==
                       thisIndex.wrapping_sub(1 as libc::c_int as
                                                  libc::c_uint) {
                    bGotIt = 1 as libc::c_int;
                    marker = i
                }
                i = i.wrapping_add(1)
            }
            /* We found an older one */
            if bGotIt != 0 {
                let fresh2 = count;
                count = count.wrapping_add(1);
                history[fresh2 as usize] = marker
            } else {
                bQuit = 1 as libc::c_int
                // count holds how many we got
            }
            if thisIndex != 0 {
                /* Look for an older one */
                thisIndex = thisIndex.wrapping_sub(1)
            } else {
                bQuit = 1 as libc::c_int
                // We've reached the big bang - there is nothing older...
            }
            /* History can only hold so many */
            if count >= consoleDrop { bQuit = 1 as libc::c_int }
        }
    }
    if count == 0 {
        /* there are messages - just no old ones yet */
        return 0 as libc::c_int as UDWORD
    }
    if count != 0 {
        /* Get the line pitch */
        linePitch = iV_GetTextLineSize() as UDWORD;
        /* How big a box is necessary? */
		/* GET RID OF THE MAGIC NUMBERS BELOW */
        pie_TransBoxFill(mainConsole.topX.wrapping_sub(4 as libc::c_int as
                                                           libc::c_uint) as
                             SDWORD,
                         mainConsole.topY.wrapping_sub(mainConsole.textDepth).wrapping_sub(4
                                                                                               as
                                                                                               libc::c_int
                                                                                               as
                                                                                               libc::c_uint)
                             as SDWORD,
                         mainConsole.topX.wrapping_add(mainConsole.width) as
                             SDWORD,
                         mainConsole.topY.wrapping_add(count.wrapping_mul(linePitch)).wrapping_add(4
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_uint).wrapping_sub(linePitch)
                             as SDWORD);
    }
    /*
	if(count)
	{
		sprintf(buildData,"%s,%s",__TIME__,__DATE__);

		buildWidth = iV_GetTextWidth(buildData);

		pie_DrawText(buildData,((mainConsole.topX+mainConsole.width) - buildWidth - 16),
			mainConsole.topY);
	}
	*/
    MesY = mainConsole.topY;
    /* Render what we found */
    i = count.wrapping_sub(1 as libc::c_int as libc::c_uint);
    while i > 0 as libc::c_int as libc::c_uint {
        /* Draw the text string */
        MesY =
            pie_DrawFormattedText(consoleStorage[history[i as usize] as
                                                     usize].text.as_mut_ptr(),
                                  mainConsole.topX, MesY, mainConsole.width,
                                  consoleStorage[history[i as usize] as
                                                     usize].JustifyType,
                                  0 as libc::c_int);
        i = i.wrapping_sub(1)
    }
    /* Draw the top one */
    pie_DrawFormattedText(consoleStorage[history[0 as libc::c_int as usize] as
                                             usize].text.as_mut_ptr(),
                          mainConsole.topX, MesY, mainConsole.width,
                          consoleStorage[history[0 as libc::c_int as usize] as
                                             usize].JustifyType,
                          0 as libc::c_int);
    /* Return how much to drop the existing console by... Fix this for lines>screenWIDTH */
    if count != 0 {
        return count.wrapping_mul(linePitch)
    } else { return 0 as libc::c_int as UDWORD };
}
/* Allows toggling of the box under the console text */
#[no_mangle]
pub unsafe extern "C" fn setConsoleBackdropStatus(mut state: BOOL) {
    bTextBoxActive = state;
}
/*
	Turns on and off display of console. It's worth
	noting that this is just the display so if you want
	to make sure that when it's turned back on again, there
	are no messages, the call flushConsoleMessages first.
*/
#[no_mangle]
pub unsafe extern "C" fn enableConsoleDisplay(mut state: BOOL) {
    bConsoleDisplayEnabled = state;
}
/* Sets the default justification for text */
#[no_mangle]
pub unsafe extern "C" fn setDefaultConsoleJust(mut defJ:
                                                   CONSOLE_TEXT_JUSTIFICATION) {
    match defJ as libc::c_uint {
        0 | 1 | 2 => { defJustification = defJ }
        _ => {
            debug(LOG_ERROR,
                  b"Weird default text justification for console\x00" as
                      *const u8 as *const libc::c_char);
            abort();
        }
    };
}
/* Allows positioning of the console on screen */
#[no_mangle]
pub unsafe extern "C" fn setConsoleSizePos(mut x: UDWORD, mut y: UDWORD,
                                           mut width: UDWORD) {
    mainConsole.topX = x;
    mainConsole.topY = y;
    mainConsole.width = width;
    /* Should be done below */
    mainConsole.textDepth = 8 as libc::c_int as UDWORD;
    flushConsoleMessages();
}
/*	Establishes whether the console messages stay there */
#[no_mangle]
pub unsafe extern "C" fn setConsolePermanence(mut state: BOOL,
                                              mut bClearOld: BOOL) {
    if mainConsole.permanent == 1 as libc::c_int && state == 0 as libc::c_int
       {
        if bClearOld != 0 { flushConsoleMessages(); }
        mainConsole.permanent = 0 as libc::c_int
    } else {
        if bClearOld != 0 { flushConsoleMessages(); }
        mainConsole.permanent = state
    };
}
/* TRUE or FALSE as to whether the mouse is presently over the console window */
#[no_mangle]
pub unsafe extern "C" fn mouseOverConsoleBox() -> BOOL {
    if mouseX() as UDWORD > mainConsole.topX &&
           mouseY() as UDWORD > mainConsole.topY &&
           (mouseX() as UDWORD) <
               mainConsole.topX.wrapping_add(mainConsole.width) &&
           (mouseY() as UDWORD) <
               mainConsole.topY.wrapping_add((iV_GetTextLineSize() as
                                                  libc::c_uint).wrapping_mul(numActiveMessages))
       {
        //condition 4
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
}
/* Sets up how many lines are allowed and how many are visible */
#[no_mangle]
pub unsafe extern "C" fn setConsoleLineInfo(mut vis: UDWORD) {
    if vis <= 64 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Request for more visible lines in the console than exist\x00"
                  as *const u8 as *const libc::c_char);
    };
    if vis <= 64 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"console.c\x00" as *const u8 as *const libc::c_char,
              711 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 19],
                                        &[libc::c_char; 19]>(b"setConsoleLineInfo\x00")).as_ptr(),
              b"vis<=MAX_CONSOLE_MESSAGES\x00" as *const u8 as
                  *const libc::c_char);
    };
    consoleVisibleLines = vis;
}
/* get how many lines are allowed and how many are visible */
#[no_mangle]
pub unsafe extern "C" fn getConsoleLineInfo() -> UDWORD {
    return consoleVisibleLines;
}
/* MODULE CONSOLE PROTOTYPES */
#[no_mangle]
pub unsafe extern "C" fn consolePrintf(mut layout: *mut libc::c_char,
                                       mut args: ...) {
    let mut consoleString: [STRING; 255] = [0; 255]; // Formatting info
    let mut arguments: va_list = 0 as *mut libc::c_char;
    /* Boot off the argument List */
    arguments = args.clone();
    /* 'print' it out into our buffer */
    vsprintf(consoleString.as_mut_ptr(), layout, arguments);
    /* Add the message through the normal channels! */
    addConsoleMessage(consoleString.as_mut_ptr(), DEFAULT_JUSTIFY);
}
#[no_mangle]
pub unsafe extern "C" fn permitNewConsoleMessages(mut allow: BOOL) {
    allowNewMessages = allow;
}
#[no_mangle]
pub unsafe extern "C" fn getConsoleDisplayStatus() -> BOOL {
    return bConsoleDisplayEnabled;
}
#[no_mangle]
pub unsafe extern "C" fn conShowReplayWav() { }
/* output warnings directly to the in-game console */
#[no_mangle]
pub unsafe extern "C" fn printf_console(mut pFormat: *const libc::c_char,
                                        mut args: ...) {
}
// / Print to the ingame console in debug mode only
/* like printf_console, bu for release */
#[no_mangle]
pub unsafe extern "C" fn console(mut pFormat: *const libc::c_char,
                                 mut args: ...) {
    let mut aBuffer: [libc::c_char; 500] = [0; 500]; // Output string buffer
    let mut pArgs: va_list = 0 as *mut libc::c_char; // Format arguments
    /* Initialise the argument list */
    pArgs = args.clone();
    /* Print out the string */
    vsprintf(aBuffer.as_mut_ptr(), pFormat, pArgs);
    /* Output it */
    addConsoleMessage(aBuffer.as_mut_ptr(), DEFAULT_JUSTIFY);
}
