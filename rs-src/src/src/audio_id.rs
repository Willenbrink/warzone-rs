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
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char)
     -> libc::c_int;
}
pub type STRING = libc::c_char;
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
pub type C2RustUnnamed = libc::c_int;
pub const ID_MAX_SOUND: C2RustUnnamed = 354;
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
pub const ID_SOUND_ARTIFACT: C2RustUnnamed = 28;
pub const ID_SOUND_RESEARCH_FACILITY_REQUIRED: C2RustUnnamed = 27;
pub const ID_SOUND_POWER_GENERATOR_REQUIRED: C2RustUnnamed = 26;
pub const ID_SOUND_POWER_TRANSFER_IN_PROGRESS: C2RustUnnamed = 25;
pub const ID_SOUND_RESOURCE_DEPLETED: C2RustUnnamed = 24;
pub const ID_SOUND_DERRICK_DESTROYED: C2RustUnnamed = 23;
pub const ID_SOUND_DERRICK_UNDER_ATTACK: C2RustUnnamed = 22;
pub const ID_SOUND_RESOURCE_HERE: C2RustUnnamed = 21;
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
pub const NO_SOUND: C2RustUnnamed = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AUDIO_ID {
    pub iID: SDWORD,
    pub pWavStr: *mut STRING,
}
/* **************************************************************************/
static mut asAudioID: [AUDIO_ID; 354] =
    [{
         let mut init =
             AUDIO_ID{iID: ID_SOUND_WINDOWCLOSE as libc::c_int,
                      pWavStr:
                          b"Beep1.ogg\x00" as *const u8 as *const libc::c_char
                              as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_WINDOWOPEN as libc::c_int,
                      pWavStr:
                          b"Beep2.ogg\x00" as *const u8 as *const libc::c_char
                              as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_SELECT as libc::c_int,
                      pWavStr:
                          b"Beep4.ogg\x00" as *const u8 as *const libc::c_char
                              as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_BUTTON_CLICK_5 as libc::c_int,
                      pWavStr:
                          b"Beep5.ogg\x00" as *const u8 as *const libc::c_char
                              as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: FE_AUDIO_MESSAGEEND as libc::c_int,
                      pWavStr:
                          b"Beep6.ogg\x00" as *const u8 as *const libc::c_char
                              as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_ZOOM_ON_RADAR as libc::c_int,
                      pWavStr:
                          b"Beep7.ogg\x00" as *const u8 as *const libc::c_char
                              as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_BUILD_FAIL as libc::c_int,
                      pWavStr:
                          b"Beep8.ogg\x00" as *const u8 as *const libc::c_char
                              as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_MESSAGEEND as libc::c_int,
                      pWavStr:
                          b"Beep9.ogg\x00" as *const u8 as *const libc::c_char
                              as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_GAME_SHUTDOWN as libc::c_int,
                      pWavStr:
                          b"GmeShtDn.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_TURRET_SELECTED as libc::c_int,
                      pWavStr:
                          b"PCV331.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_BODY_SELECTED as libc::c_int,
                      pWavStr:
                          b"PCV332.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_PROPULSION_SELECTED as libc::c_int,
                      pWavStr:
                          b"PCV333.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_DESIGN_COMPLETED as libc::c_int,
                      pWavStr:
                          b"PCV334.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_CONSTRUCTION_STARTED as libc::c_int,
                      pWavStr:
                          b"PCV335.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_STRUCTURE_COMPLETED as libc::c_int,
                      pWavStr:
                          b"PCV336.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_STRUCTURE_UNDER_ATTACK as libc::c_int,
                      pWavStr:
                          b"PCV337.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID:
                          ID_SOUND_STRUCTURE_REPAIR_IN_PROGRESS as
                              libc::c_int,
                      pWavStr:
                          b"PCV339.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_STRUCTURE_DEMOLISHED as libc::c_int,
                      pWavStr:
                          b"PCV340.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID:
                          ID_SOUND_POWER_GENERATOR_UNDER_ATTACK as
                              libc::c_int,
                      pWavStr:
                          b"PCV341.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_POWER_GENERATOR_DESTROYED as libc::c_int,
                      pWavStr:
                          b"PCV342.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_POWER_LOW as libc::c_int,
                      pWavStr:
                          b"PCV343.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_RESOURCE_HERE as libc::c_int,
                      pWavStr:
                          b"PCV344.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_DERRICK_UNDER_ATTACK as libc::c_int,
                      pWavStr:
                          b"PCV345.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_DERRICK_DESTROYED as libc::c_int,
                      pWavStr:
                          b"PCV346.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_RESOURCE_DEPLETED as libc::c_int,
                      pWavStr:
                          b"PCV347.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_POWER_TRANSFER_IN_PROGRESS as libc::c_int,
                      pWavStr:
                          b"PCV348.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_POWER_GENERATOR_REQUIRED as libc::c_int,
                      pWavStr:
                          b"PCV349.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_RESEARCH_FACILITY_REQUIRED as libc::c_int,
                      pWavStr:
                          b"PCV350.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_ARTIFACT as libc::c_int,
                      pWavStr:
                          b"PCV351.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_ARTIFACT_RECOVERED as libc::c_int,
                      pWavStr:
                          b"PCV352.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID:
                          ID_SOUND_NEW_RESEARCH_PROJ_AVAILABLE as libc::c_int,
                      pWavStr:
                          b"PCV353.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_NEW_STRUCTURE_AVAILABLE as libc::c_int,
                      pWavStr:
                          b"PCV354.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_NEW_COMPONENT_AVAILABLE as libc::c_int,
                      pWavStr:
                          b"PCV355.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_NEW_CYBORG_AVAILABLE as libc::c_int,
                      pWavStr:
                          b"PCV356.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_RESEARCH_COMPLETED as libc::c_int,
                      pWavStr:
                          b"PCV357.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_MAJOR_RESEARCH as libc::c_int,
                      pWavStr:
                          b"PCV358.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID:
                          ID_SOUND_STRUCTURE_RESEARCH_COMPLETED as
                              libc::c_int,
                      pWavStr:
                          b"PCV359.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_POWER_RESEARCH_COMPLETED as libc::c_int,
                      pWavStr:
                          b"PCV360.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID:
                          ID_SOUND_COMPUTER_RESEARCH_COMPLETED as libc::c_int,
                      pWavStr:
                          b"PCV361.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_VEHICLE_RESEARCH_COMPLETED as libc::c_int,
                      pWavStr:
                          b"PCV362.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_SYSTEMS_RESEARCH_COMPLETED as libc::c_int,
                      pWavStr:
                          b"PCV363.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_WEAPON_RESEARCH_COMPLETED as libc::c_int,
                      pWavStr:
                          b"PCV364.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_CYBORG_RESEARCH_COMPLETED as libc::c_int,
                      pWavStr:
                          b"PCV365.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_PRODUCTION_STARTED as libc::c_int,
                      pWavStr:
                          b"PCV366.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_DROID_COMPLETED as libc::c_int,
                      pWavStr:
                          b"PCV367.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_PRODUCTION_PAUSED as libc::c_int,
                      pWavStr:
                          b"PCV368.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_PRODUCTION_CANCELLED as libc::c_int,
                      pWavStr:
                          b"PCV369.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_DELIVERY_POINT_ASSIGNED as libc::c_int,
                      pWavStr:
                          b"PCV370.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_DELIVERY_POINT_ASSIGNED_TO as libc::c_int,
                      pWavStr:
                          b"PCV371.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_UNIT_REPAIRED as libc::c_int,
                      pWavStr:
                          b"PCV372.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_SCAVENGERS_DETECTED as libc::c_int,
                      pWavStr:
                          b"PCV373.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_SCAVENGER_BASE_DETECTED as libc::c_int,
                      pWavStr:
                          b"PCV374.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_SCAVENGER_OUTPOST_DETECTED as libc::c_int,
                      pWavStr:
                          b"PCV375.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_POWER_RESOURCE as libc::c_int,
                      pWavStr:
                          b"PCV376.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_ARTEFACT_DISC as libc::c_int,
                      pWavStr:
                          b"PCV377.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_ENEMY_UNIT_DETECTED as libc::c_int,
                      pWavStr:
                          b"PCV378.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_ENEMY_BASE_DETECTED as libc::c_int,
                      pWavStr:
                          b"PCV379.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_ALLY_DETECTED as libc::c_int,
                      pWavStr:
                          b"PCV380.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_ENEMY_TRANSPORT_DETECTED as libc::c_int,
                      pWavStr:
                          b"PCV381.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_ENEMY_LZ_DETECTED as libc::c_int,
                      pWavStr:
                          b"PCV382.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_FRIENDLY_LZ_DETECTED as libc::c_int,
                      pWavStr:
                          b"PCV383.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_NEXUS_TOWER_DETECTED as libc::c_int,
                      pWavStr:
                          b"PCV384.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_NEXUS_TURRET_DETECTED as libc::c_int,
                      pWavStr:
                          b"PCV385.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_NEXUS_UNIT_DETECTED as libc::c_int,
                      pWavStr:
                          b"PCV386.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_ENEMY_BATTERY_DETECTED as libc::c_int,
                      pWavStr:
                          b"PCV387.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_ENEMY_VTOLS_DETECTED as libc::c_int,
                      pWavStr:
                          b"PCV388.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_SCAVENGER_BASE as libc::c_int,
                      pWavStr:
                          b"PCV389.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_SCAVENGER_OUTPOST as libc::c_int,
                      pWavStr:
                          b"PCV390.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID:
                          ID_SOUND_SCAVENGER_OUTPOST_ERADICATED as
                              libc::c_int,
                      pWavStr:
                          b"PCV391.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_SCAVENGER_BASE_ERADICATED as libc::c_int,
                      pWavStr:
                          b"PCV392.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_ENEMY_BASE as libc::c_int,
                      pWavStr:
                          b"PCV393.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_ENEMY_BASE_ERADICATED as libc::c_int,
                      pWavStr:
                          b"PCV394.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_INCOMING_ENEMY_TRANSPORT as libc::c_int,
                      pWavStr:
                          b"PCV395.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_ENEMY_LZ as libc::c_int,
                      pWavStr:
                          b"PCV396.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_LZ1 as libc::c_int,
                      pWavStr:
                          b"PCV397.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_LZ2 as libc::c_int,
                      pWavStr:
                          b"PCV398.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_UNIT_UNDER_ATTACK as libc::c_int,
                      pWavStr:
                          b"PCV399.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_UNIT_DESTROYED as libc::c_int,
                      pWavStr:
                          b"PCV400.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_UNIT_RETREATING as libc::c_int,
                      pWavStr:
                          b"PCV401.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_UNIT_RETURNING_FOR_REPAIR as libc::c_int,
                      pWavStr:
                          b"PCV402.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_ASSIGNED_TO_SENSOR as libc::c_int,
                      pWavStr:
                          b"PCV403.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_SENSOR_LOCKED_ON as libc::c_int,
                      pWavStr:
                          b"PCV404.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_ASSIGNED_TO_COUNTER_RADAR as libc::c_int,
                      pWavStr:
                          b"PCV405.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_ENEMY_BATTERY_LOCATED as libc::c_int,
                      pWavStr:
                          b"PCV406.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID:
                          ID_SOUND_BATTERY_FIRING_COUNTER_ATTACK as
                              libc::c_int,
                      pWavStr:
                          b"PCV407.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_INTERCEPTORS_LAUNCHED as libc::c_int,
                      pWavStr:
                          b"PCV408.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_REARMING as libc::c_int,
                      pWavStr:
                          b"PCV409.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_VTOLS_ENGAGING as libc::c_int,
                      pWavStr:
                          b"PCV410.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_ASSIGNED as libc::c_int,
                      pWavStr:
                          b"PCV411.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_INTERCEPTORS_ASSIGNED as libc::c_int,
                      pWavStr:
                          b"PCV412.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_COMMAND_CONSOLE_ACTIVATED as libc::c_int,
                      pWavStr:
                          b"PCV413.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_SHORT_RANGE as libc::c_int,
                      pWavStr:
                          b"PCV414.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_LONG_RANGE as libc::c_int,
                      pWavStr:
                          b"PCV415.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_OPTIMUM_RANGE as libc::c_int,
                      pWavStr:
                          b"PCV416.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_RETREAT_AT_MEDIUM_DAMAGE as libc::c_int,
                      pWavStr:
                          b"PCV417.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_RETREAT_AT_HEAVY_DAMAGE as libc::c_int,
                      pWavStr:
                          b"PCV418.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_NO_RETREAT as libc::c_int,
                      pWavStr:
                          b"PCV419.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_FIRE_AT_WILL as libc::c_int,
                      pWavStr:
                          b"PCV420.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_RETURN_FIRE as libc::c_int,
                      pWavStr:
                          b"PCV421.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_CEASEFIRE as libc::c_int,
                      pWavStr:
                          b"PCV422.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_HOLD_POSITION as libc::c_int,
                      pWavStr:
                          b"PCV423.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_GUARD as libc::c_int,
                      pWavStr:
                          b"PCV424.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_PURSUE as libc::c_int,
                      pWavStr:
                          b"PCV425.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_PATROL as libc::c_int,
                      pWavStr:
                          b"PCV426.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_RETURN_TO_LZ as libc::c_int,
                      pWavStr:
                          b"PCV427.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_RECYCLING as libc::c_int,
                      pWavStr:
                          b"PCV428.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_SCATTER as libc::c_int,
                      pWavStr:
                          b"PCV429.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_NOT_POSSIBLE_TRY_AGAIN as libc::c_int,
                      pWavStr:
                          b"PCV430.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_NO as libc::c_int,
                      pWavStr:
                          b"PCV431.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_THAT_IS_INCORRECT as libc::c_int,
                      pWavStr:
                          b"PCV432.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_WELL_DONE as libc::c_int,
                      pWavStr:
                          b"PCV433.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_EXCELLENT as libc::c_int,
                      pWavStr:
                          b"PCV434.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_ASSIGNED_TO_COMMANDER as libc::c_int,
                      pWavStr:
                          b"PCV435.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_GROUP_REPORTING as libc::c_int,
                      pWavStr:
                          b"PCV436.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_COMMANDER_REPORTING as libc::c_int,
                      pWavStr:
                          b"PCV437.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_ROUTE_OBSTRUCTED as libc::c_int,
                      pWavStr:
                          b"PCV438.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_NO_ROUTE_AVAILABLE as libc::c_int,
                      pWavStr:
                          b"PCV439.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_REINFORCEMENTS_AVAILABLE as libc::c_int,
                      pWavStr:
                          b"PCV440.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_REINFORCEMENTS_IN_TRANSIT as libc::c_int,
                      pWavStr:
                          b"PCV441.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_TRANSPORT_LANDING as libc::c_int,
                      pWavStr:
                          b"PCV442.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_TRANSPORT_UNDER_ATTACK as libc::c_int,
                      pWavStr:
                          b"PCV443.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_TRANSPORT_REPAIRING as libc::c_int,
                      pWavStr:
                          b"PCV444.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_LZ_COMPROMISED as libc::c_int,
                      pWavStr:
                          b"PCV445.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_LZ_CLEAR as libc::c_int,
                      pWavStr:
                          b"LZ-Clear.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID:
                          ID_SOUND_TRANSPORT_RETURNING_TO_BASE as libc::c_int,
                      pWavStr:
                          b"PCV446.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_TRANSPORT_UNABLE_TO_LAND as libc::c_int,
                      pWavStr:
                          b"PCV447.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_MISSION_OBJECTIVE as libc::c_int,
                      pWavStr:
                          b"PCV448.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_MISSION_UPDATE as libc::c_int,
                      pWavStr:
                          b"PCV449.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_WARZONE_PAUSED as libc::c_int,
                      pWavStr:
                          b"PCV450.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_WARZONE_ACTIVE as libc::c_int,
                      pWavStr:
                          b"PCV451.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_MISSION_RESULTS as libc::c_int,
                      pWavStr:
                          b"PCV452.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_RESEARCH_STOLEN as libc::c_int,
                      pWavStr:
                          b"PCV453.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_TECHNOLOGY_TAKEN as libc::c_int,
                      pWavStr:
                          b"PCV454.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_INCOMING_TRANSMISSION as libc::c_int,
                      pWavStr:
                          b"PCV455.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID:
                          ID_SOUND_INCOMING_INTELLIGENCE_REPORT as
                              libc::c_int,
                      pWavStr:
                          b"PCV456.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_MISSION_FAILED as libc::c_int,
                      pWavStr:
                          b"PCV458.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_MISSION_SUCCESSFUL as libc::c_int,
                      pWavStr:
                          b"PCV459.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_OBJECTIVE_ACCOMPLISHED as libc::c_int,
                      pWavStr:
                          b"PCV460.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_OBJECTIVE_FAILED as libc::c_int,
                      pWavStr:
                          b"PCV461.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_MISSION_TIMER_ACTIVATED as libc::c_int,
                      pWavStr:
                          b"PCV462.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_10_MINUTES_REMAINING as libc::c_int,
                      pWavStr:
                          b"PCV463.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_5_MINUTES_REMAINING as libc::c_int,
                      pWavStr:
                          b"PCV464.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_3_MINUTES_REMAINING as libc::c_int,
                      pWavStr:
                          b"PCV465.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_2_MINUTES_REMAINING as libc::c_int,
                      pWavStr:
                          b"PCV466.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_1_MINUTE_REMAINING as libc::c_int,
                      pWavStr:
                          b"PCV467.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_UNIT_CAPTURED as libc::c_int,
                      pWavStr:
                          b"PCV468.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_SYSTEM_FAILURE_IMMINENT as libc::c_int,
                      pWavStr:
                          b"PCV469.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_YOU_ARE_DEFEATED as libc::c_int,
                      pWavStr:
                          b"PCV470.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_MISSILE_CODES_DECIPHERED as libc::c_int,
                      pWavStr:
                          b"PCV471.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID:
                          ID_SOUND_1ST_MISSILE_CODES_DECIPHERED as
                              libc::c_int,
                      pWavStr:
                          b"PCV472.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID:
                          ID_SOUND_2ND_MISSILE_CODES_DECIPHERED as
                              libc::c_int,
                      pWavStr:
                          b"PCV473.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID:
                          ID_SOUND_3RD_MISSILE_CODES_DECIPHERED as
                              libc::c_int,
                      pWavStr:
                          b"PCV474.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_MISSILE_CODES_CRACKED as libc::c_int,
                      pWavStr:
                          b"PCV475.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_ENTERING_WARZONE as libc::c_int,
                      pWavStr:
                          b"PCV476.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_ALLIANCE_ACC as libc::c_int,
                      pWavStr:
                          b"PCV477.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_ALLIANCE_BRO as libc::c_int,
                      pWavStr:
                          b"PCV478.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_ALLIANCE_OFF as libc::c_int,
                      pWavStr:
                          b"PCV479.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_CLAN_ENTER as libc::c_int,
                      pWavStr:
                          b"PCV480.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_CLAN_EXIT as libc::c_int,
                      pWavStr:
                          b"PCV481.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_GIFT as libc::c_int,
                      pWavStr:
                          b"PCV482.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_POWER_TRANSMIT as libc::c_int,
                      pWavStr:
                          b"PCV483.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SENSOR_DOWNLOAD as libc::c_int,
                      pWavStr:
                          b"PCV484.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_TECHNOLOGY_TRANSFER as libc::c_int,
                      pWavStr:
                          b"PCV485.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_UNITS_TRANSFER as libc::c_int,
                      pWavStr:
                          b"PCV486.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_GROUP as libc::c_int,
                      pWavStr:
                          b"Group.ogg\x00" as *const u8 as *const libc::c_char
                              as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_GROUP_0 as libc::c_int,
                      pWavStr:
                          b"0.ogg\x00" as *const u8 as *const libc::c_char as
                              *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_GROUP_1 as libc::c_int,
                      pWavStr:
                          b"1.ogg\x00" as *const u8 as *const libc::c_char as
                              *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_GROUP_2 as libc::c_int,
                      pWavStr:
                          b"2.ogg\x00" as *const u8 as *const libc::c_char as
                              *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_GROUP_3 as libc::c_int,
                      pWavStr:
                          b"3.ogg\x00" as *const u8 as *const libc::c_char as
                              *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_GROUP_4 as libc::c_int,
                      pWavStr:
                          b"4.ogg\x00" as *const u8 as *const libc::c_char as
                              *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_GROUP_5 as libc::c_int,
                      pWavStr:
                          b"5.ogg\x00" as *const u8 as *const libc::c_char as
                              *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_GROUP_6 as libc::c_int,
                      pWavStr:
                          b"6.ogg\x00" as *const u8 as *const libc::c_char as
                              *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_GROUP_7 as libc::c_int,
                      pWavStr:
                          b"7.ogg\x00" as *const u8 as *const libc::c_char as
                              *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_GROUP_8 as libc::c_int,
                      pWavStr:
                          b"8.ogg\x00" as *const u8 as *const libc::c_char as
                              *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_GROUP_9 as libc::c_int,
                      pWavStr:
                          b"9.ogg\x00" as *const u8 as *const libc::c_char as
                              *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_REPORTING as libc::c_int,
                      pWavStr:
                          b"Reprting.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_COMMANDER as libc::c_int,
                      pWavStr:
                          b"Commnder.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_COM_SCAVS_DETECTED as libc::c_int,
                      pWavStr:
                          b"COM021.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_COM_SCAV_BASE_DETECTED as libc::c_int,
                      pWavStr:
                          b"COM022.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_COM_SCAV_OUTPOST_DETECTED as libc::c_int,
                      pWavStr:
                          b"COM023.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_COM_RESOURCE_DETECTED as libc::c_int,
                      pWavStr:
                          b"COM024.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_COM_ARTEFACT_DETECTED as libc::c_int,
                      pWavStr:
                          b"COM025.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_COM_ENEMY_DETECTED as libc::c_int,
                      pWavStr:
                          b"COM026.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_COM_ENEMY_BASE_DETECTED as libc::c_int,
                      pWavStr:
                          b"COM027.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_COM_ALLY_DETECTED as libc::c_int,
                      pWavStr:
                          b"COM028.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID:
                          ID_SOUND_COM_ENEMY_TRANSPORT_DETECTED as
                              libc::c_int,
                      pWavStr:
                          b"COM029.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_COM_ENEMY_LZ_DETECTED as libc::c_int,
                      pWavStr:
                          b"COM030.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_COM_FRIENDLY_LZ_DETECTED as libc::c_int,
                      pWavStr:
                          b"COM031.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_COM_NEXUS_TOWER_DETECTED as libc::c_int,
                      pWavStr:
                          b"COM032.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_COM_NEXUS_TURRET_DETECTED as libc::c_int,
                      pWavStr:
                          b"COM033.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_COM_NEXUS_DETECTED as libc::c_int,
                      pWavStr:
                          b"COM034.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_COM_ENEMY_BATTERY_DETECTED as libc::c_int,
                      pWavStr:
                          b"COM035.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_COM_ENEMY_VTOLS_DETECTED as libc::c_int,
                      pWavStr:
                          b"COM036.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_COM_ROUTE_OBSTRUCTED as libc::c_int,
                      pWavStr:
                          b"COM037.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_COM_NO_ROUTE_AVAILABLE as libc::c_int,
                      pWavStr:
                          b"COM038.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_COM_UNABLE_TO_COMPLY as libc::c_int,
                      pWavStr:
                          b"COM039.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_COM_RETURNING_FOR_REPAIR as libc::c_int,
                      pWavStr:
                          b"COM040.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID:
                          ID_SOUND_COM_HEADING_FOR_RALLY_POINT as libc::c_int,
                      pWavStr:
                          b"COM041.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_RADIOCLICK_1 as libc::c_int,
                      pWavStr:
                          b"RadClik1.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_RADIOCLICK_2 as libc::c_int,
                      pWavStr:
                          b"RadClik2.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_RADIOCLICK_3 as libc::c_int,
                      pWavStr:
                          b"RadClik3.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_RADIOCLICK_4 as libc::c_int,
                      pWavStr:
                          b"RadClik4.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_RADIOCLICK_5 as libc::c_int,
                      pWavStr:
                          b"RadClik5.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_RADIOCLICK_6 as libc::c_int,
                      pWavStr:
                          b"RadClik6.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_APPROACHING_LZ as libc::c_int,
                      pWavStr:
                          b"T-AproLZ.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_ALRIGHT_BOYS as libc::c_int,
                      pWavStr:
                          b"T-ARBoys.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_GREEN_LIGHT_IN_5 as libc::c_int,
                      pWavStr:
                          b"T-GrnLi5.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_GREEN_LIGHT_IN_4 as libc::c_int,
                      pWavStr:
                          b"T-GrnLi4.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_GREEN_LIGHT_IN_3 as libc::c_int,
                      pWavStr:
                          b"T-GrnLi3.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_GREEN_LIGHT_IN_2 as libc::c_int,
                      pWavStr:
                          b"T-GrnLi2.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_GO_GO_GO as libc::c_int,
                      pWavStr:
                          b"T-GoGoGo,wav\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_PREPARE_FOR_DUST_OFF as libc::c_int,
                      pWavStr:
                          b"T-DustOf.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_ENEMY_LOCATED1 as libc::c_int,
                      pWavStr:
                          b"V-Eloc1.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_ON_OUR_WAY1 as libc::c_int,
                      pWavStr:
                          b"V-OnWay1.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_RETURNING_TO_BASE1 as libc::c_int,
                      pWavStr:
                          b"V-RetBa1.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_LOCKED_ON1 as libc::c_int,
                      pWavStr:
                          b"V-LocOn1.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_COMMENCING_ATTACK_RUN1 as libc::c_int,
                      pWavStr:
                          b"V-AtkRn1.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_ABORTING_ATTACK_RUN1 as libc::c_int,
                      pWavStr:
                          b"V-AbtRn1.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_ENEMY_LOCATED2 as libc::c_int,
                      pWavStr:
                          b"V-Eloc2.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_ON_OUR_WAY2 as libc::c_int,
                      pWavStr:
                          b"V-OnWay2.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_RETURNING_TO_BASE2 as libc::c_int,
                      pWavStr:
                          b"V-RetBa2.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_LOCKED_ON2 as libc::c_int,
                      pWavStr:
                          b"V-LocOn2.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_COMMENCING_ATTACK_RUN2 as libc::c_int,
                      pWavStr:
                          b"V-AtkRn2.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_ABORTING_ATTACK_RUN2 as libc::c_int,
                      pWavStr:
                          b"V-AbtRn2.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_ENEMY_LOCATED3 as libc::c_int,
                      pWavStr:
                          b"V-Eloc3.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_ON_OUR_WAY3 as libc::c_int,
                      pWavStr:
                          b"V-OnWay3.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_RETURNING_TO_BASE3 as libc::c_int,
                      pWavStr:
                          b"V-RetBa3.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_LOCKED_ON3 as libc::c_int,
                      pWavStr:
                          b"V-LocOn3.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_COMMENCING_ATTACK_RUN3 as libc::c_int,
                      pWavStr:
                          b"V-AtkRn3.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_ABORTING_ATTACK_RUN3 as libc::c_int,
                      pWavStr:
                          b"V-AbtRn3.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_COLL_CLEANSE_AND_DESTROY as libc::c_int,
                      pWavStr:
                          b"COl011a.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID:
                          ID_SOUND_COLL_DESTROYING_BIOLOGICALS as libc::c_int,
                      pWavStr:
                          b"COl012a.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_COLL_ATTACK as libc::c_int,
                      pWavStr:
                          b"COl013a.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_COLL_FIRE as libc::c_int,
                      pWavStr:
                          b"COl014a.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_COLL_ENEMY_DETECTED as libc::c_int,
                      pWavStr:
                          b"COl015a.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_COLL_ENGAGING as libc::c_int,
                      pWavStr:
                          b"COl016a.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_COLL_STARTING_ATTACK_RUN as libc::c_int,
                      pWavStr:
                          b"COl017a.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_COLL_DIE as libc::c_int,
                      pWavStr:
                          b"COl018a.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_COLL_INTERCEPT_AND_DESTROY as libc::c_int,
                      pWavStr:
                          b"COl019a.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_COLL_ENEMY_DESTROYED as libc::c_int,
                      pWavStr:
                          b"COl020a.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_ROCKET as libc::c_int,
                      pWavStr:
                          b"rocket.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_ROTARY_LASER as libc::c_int,
                      pWavStr:
                          b"RotLsr.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_GAUSSGUN as libc::c_int,
                      pWavStr:
                          b"GaussGun.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_LARGE_CANNON as libc::c_int,
                      pWavStr:
                          b"LrgCan.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_SMALL_CANNON as libc::c_int,
                      pWavStr:
                          b"SmlCan.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_MEDIUM_CANNON as libc::c_int,
                      pWavStr:
                          b"MedCan.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_FLAME_THROWER as libc::c_int,
                      pWavStr:
                          b"FlmThrow.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_PULSE_LASER as libc::c_int,
                      pWavStr:
                          b"PlsLsr.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_BEAM_LASER as libc::c_int,
                      pWavStr:
                          b"BemLsr.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_MORTAR as libc::c_int,
                      pWavStr:
                          b"Mortar.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_HOWITZ_FLIGHT as libc::c_int,
                      pWavStr:
                          b"HwtzFlgt.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_BABA_MG_1 as libc::c_int,
                      pWavStr:
                          b"MgBar1.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_BABA_MG_2 as libc::c_int,
                      pWavStr:
                          b"MgBar2.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_BABA_MG_3 as libc::c_int,
                      pWavStr:
                          b"MgBar3.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_BABA_MG_HEAVY as libc::c_int,
                      pWavStr:
                          b"MgHeavy.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_BABA_MG_TOWER as libc::c_int,
                      pWavStr:
                          b"MgTower.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_SPLASH as libc::c_int,
                      pWavStr:
                          b"Splash.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_ASSAULT_MG as libc::c_int,
                      pWavStr:
                          b"AsltMG.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_RAPID_CANNON as libc::c_int,
                      pWavStr:
                          b"RapdCan.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_HIVEL_CANNON as libc::c_int,
                      pWavStr:
                          b"HiVelCan.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_NEXUS_TOWER as libc::c_int,
                      pWavStr:
                          b"NxsTower.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_WELD_1 as libc::c_int,
                      pWavStr:
                          b"Weld-1.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_WELD_2 as libc::c_int,
                      pWavStr:
                          b"Weld-2.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_CONSTRUCTION_START as libc::c_int,
                      pWavStr:
                          b"BldStart.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_CONSTRUCTION_LOOP as libc::c_int,
                      pWavStr:
                          b"BldLoop.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_CONSTRUCTION_1 as libc::c_int,
                      pWavStr:
                          b"Build1.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_CONSTRUCTION_2 as libc::c_int,
                      pWavStr:
                          b"Build2.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_CONSTRUCTION_3 as libc::c_int,
                      pWavStr:
                          b"Build3.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_CONSTRUCTION_4 as libc::c_int,
                      pWavStr:
                          b"Build4.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_EXPLOSION_SMALL as libc::c_int,
                      pWavStr:
                          b"SmlExpl.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_EXPLOSION_LASER as libc::c_int,
                      pWavStr:
                          b"LsrExpl.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_EXPLOSION as libc::c_int,
                      pWavStr:
                          b"LrgExpl.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_EXPLOSION_ANTITANK as libc::c_int,
                      pWavStr:
                          b"ATnkExpl.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_RICOCHET_1 as libc::c_int,
                      pWavStr:
                          b"Richet1.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_RICOCHET_2 as libc::c_int,
                      pWavStr:
                          b"Richet2.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_RICOCHET_3 as libc::c_int,
                      pWavStr:
                          b"Richet3.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_BARB_SQUISH as libc::c_int,
                      pWavStr:
                          b"Squish.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_BUILDING_FALL as libc::c_int,
                      pWavStr:
                          b"BldFall.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_NEXUS_EXPLOSION as libc::c_int,
                      pWavStr:
                          b"NxsExpld.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_CONSTRUCTOR_MOVEOFF as libc::c_int,
                      pWavStr:
                          b"Con-Move Off.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_CONSTRUCTOR_MOVE as libc::c_int,
                      pWavStr:
                          b"Con-Move.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_CONSTRUCTOR_SHUTDOWN as libc::c_int,
                      pWavStr:
                          b"Con-Shut down.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_BLIMP_FLIGHT as libc::c_int,
                      pWavStr:
                          b"TFlight.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_BLIMP_IDLE as libc::c_int,
                      pWavStr:
                          b"THover.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_BLIMP_LAND as libc::c_int,
                      pWavStr:
                          b"TLand.ogg\x00" as *const u8 as *const libc::c_char
                              as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_BLIMP_TAKE_OFF as libc::c_int,
                      pWavStr:
                          b"TStart.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_VTOL_LAND as libc::c_int,
                      pWavStr:
                          b"VtolLand.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_VTOL_OFF as libc::c_int,
                      pWavStr:
                          b"VtolOff.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_VTOL_MOVE as libc::c_int,
                      pWavStr:
                          b"VtolMove.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_TREAD as libc::c_int,
                      pWavStr:
                          b"Tread.ogg\x00" as *const u8 as *const libc::c_char
                              as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_HOVER_MOVE as libc::c_int,
                      pWavStr:
                          b"HovMove.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_HOVER_START as libc::c_int,
                      pWavStr:
                          b"HovStart.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_HOVER_STOP as libc::c_int,
                      pWavStr:
                          b"HovStop.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_CYBORG_MOVE as libc::c_int,
                      pWavStr:
                          b"Cyber-Move.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_OIL_PUMP_2 as libc::c_int,
                      pWavStr:
                          b"OilPump.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_POWER_HUM as libc::c_int,
                      pWavStr:
                          b"PowerHum.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_POWER_SPARK as libc::c_int,
                      pWavStr:
                          b"PowerSpk.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_STEAM as libc::c_int,
                      pWavStr:
                          b"Steam.ogg\x00" as *const u8 as *const libc::c_char
                              as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_ECM_TOWER as libc::c_int,
                      pWavStr:
                          b"ECMTower.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_FIRE_ROAR as libc::c_int,
                      pWavStr:
                          b"FreRoar.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_HELP as libc::c_int,
                      pWavStr:
                          b"help.ogg\x00" as *const u8 as *const libc::c_char
                              as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_BARB_SCREAM as libc::c_int,
                      pWavStr:
                          b"Scream.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_BARB_SCREAM2 as libc::c_int,
                      pWavStr:
                          b"Scream2.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_BARB_SCREAM3 as libc::c_int,
                      pWavStr:
                          b"Scream3.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_OF_SILENCE as libc::c_int,
                      pWavStr:
                          b"Silence.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_NOFAULTS as libc::c_int,
                      pWavStr:
                          b"Scream4.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_LANDING_ZONE as libc::c_int,
                      pWavStr:
                          b"LndgZne.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_SATELLITE_UPLINK as libc::c_int,
                      pWavStr:
                          b"Pcv652.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_NASDA_CENTRAL as libc::c_int,
                      pWavStr:
                          b"Pcv653.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_NUCLEAR_REACTOR as libc::c_int,
                      pWavStr:
                          b"Pcv654.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_SAM_SITE as libc::c_int,
                      pWavStr:
                          b"Pcv655.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_MISSILE_SILO as libc::c_int,
                      pWavStr:
                          b"Pcv656.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_MISSILE_NME_DETECTED as libc::c_int,
                      pWavStr:
                          b"NmeDeted.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_STRUCTURE_CAPTURED as libc::c_int,
                      pWavStr:
                          b"Pcv611.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_CIVILIAN_RESCUED as libc::c_int,
                      pWavStr:
                          b"Pcv612.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_CIVILIANS_RESCUED as libc::c_int,
                      pWavStr:
                          b"Pcv613.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_UNITS_RESCUED as libc::c_int,
                      pWavStr:
                          b"Pcv615.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_GROUP_RESCUED as libc::c_int,
                      pWavStr:
                          b"Pcv616.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_GROUP_CAPTURED as libc::c_int,
                      pWavStr:
                          b"Pcv618.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_OBJECTIVE_CAPTURED as libc::c_int,
                      pWavStr:
                          b"Pcv621.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_OBJECTIVE_DESTROYED as libc::c_int,
                      pWavStr:
                          b"Pcv622.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_STRUCTURE_INFECTED as libc::c_int,
                      pWavStr:
                          b"Pcv623.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_GROUP_INFECTED as libc::c_int,
                      pWavStr:
                          b"Pcv625.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_OUT_OF_TIME as libc::c_int,
                      pWavStr:
                          b"Pcv629.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_ENEMY_ESCAPED as libc::c_int,
                      pWavStr:
                          b"Pcv631.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_ENEMY_ESCAPING as libc::c_int,
                      pWavStr:
                          b"Pcv632.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_ENEMY_TRANSPORT_LANDING as libc::c_int,
                      pWavStr:
                          b"Pcv633.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_TEAM_ALPHA_ERADICATED as libc::c_int,
                      pWavStr:
                          b"Pcv635.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_TEAM_BETA_ERADICATED as libc::c_int,
                      pWavStr:
                          b"Pcv636.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_TEAM_GAMMA_ERADICATED as libc::c_int,
                      pWavStr:
                          b"Pcv637.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_TEAM_ALPHA_RESCUED as libc::c_int,
                      pWavStr:
                          b"Pcv638.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_TEAM_BETA_RESCUED as libc::c_int,
                      pWavStr:
                          b"Pcv639.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_TEAM_GAMMA_RESCUED as libc::c_int,
                      pWavStr:
                          b"Pcv640.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_LASER_SATELLITE_FIRING as libc::c_int,
                      pWavStr:
                          b"Pcv650.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_INCOMING_LASER_SAT_STRIKE as libc::c_int,
                      pWavStr:
                          b"Pcv657.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_NEXUS_DEFENCES_ABSORBED as libc::c_int,
                      pWavStr:
                          b"DefAbsrd.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_NEXUS_DEFENCES_NEUTRALISED as libc::c_int,
                      pWavStr:
                          b"DefNut.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_NEXUS_LAUGH1 as libc::c_int,
                      pWavStr:
                          b"Laugh1.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_NEXUS_LAUGH2 as libc::c_int,
                      pWavStr:
                          b"Laugh2.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_NEXUS_LAUGH3 as libc::c_int,
                      pWavStr:
                          b"Laugh3.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_NEXUS_PRODUCTION_COMPLETED as libc::c_int,
                      pWavStr:
                          b"PordComp.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_NEXUS_RESEARCH_ABSORBED as libc::c_int,
                      pWavStr:
                          b"ResAbsrd.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_NEXUS_STRUCTURE_ABSORBED as libc::c_int,
                      pWavStr:
                          b"StrutAbs.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID:
                          ID_SOUND_NEXUS_STRUCTURE_NEUTRALISED as libc::c_int,
                      pWavStr:
                          b"StrutNut.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_NEXUS_SYNAPTIC_LINK as libc::c_int,
                      pWavStr:
                          b"SynpLnk.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_NEXUS_UNIT_ABSORBED as libc::c_int,
                      pWavStr:
                          b"UntAbsrd.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_NEXUS_UNIT_NEUTRALISED as libc::c_int,
                      pWavStr:
                          b"UntNut.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_CYBORG_GROUND as libc::c_int,
                      pWavStr:
                          b"CybGrnd.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_CYBORG_HEAVY as libc::c_int,
                      pWavStr:
                          b"HvCybMov.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_EMP as libc::c_int,
                      pWavStr:
                          b"EMP.ogg\x00" as *const u8 as *const libc::c_char
                              as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_LASER_HEAVY as libc::c_int,
                      pWavStr:
                          b"HevLsr.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_PLASMA_FLAMER as libc::c_int,
                      pWavStr:
                          b"PlasFlm.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_UPLINK as libc::c_int,
                      pWavStr:
                          b"UpLink.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     },
     {
         let mut init =
             AUDIO_ID{iID: ID_SOUND_LAS_SAT_COUNTDOWN as libc::c_int,
                      pWavStr:
                          b"LasStrk.ogg\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,};
         init
     }];
/* **************************************************************************/
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn audioID_GetIDFromStr(mut pWavStr: *mut STRING,
                                              mut piID: *mut SDWORD) -> BOOL {
    let mut i: SDWORD = 0;
    i = 0 as libc::c_int;
    while i < ID_MAX_SOUND as libc::c_int {
        if strcasecmp(pWavStr, asAudioID[i as usize].pWavStr) ==
               0 as libc::c_int {
            if i == asAudioID[i as usize].iID {
            } else {
                debug(LOG_ERROR,
                      b"audioID_GetIDFromStr: %s stored IDs don\'t match\x00"
                          as *const u8 as *const libc::c_char, pWavStr);
            };
            if i == asAudioID[i as usize].iID {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"audio_id.c\x00" as *const u8 as *const libc::c_char,
                      493 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 21],
                                                &[libc::c_char; 21]>(b"audioID_GetIDFromStr\x00")).as_ptr(),
                      b"i == asAudioID[i].iID\x00" as *const u8 as
                          *const libc::c_char);
            };
            *piID = asAudioID[i as usize].iID;
            return 1 as libc::c_int
        }
        i += 1
    }
    *piID = NO_SOUND as libc::c_int;
    return 0 as libc::c_int;
}
/* **************************************************************************/
