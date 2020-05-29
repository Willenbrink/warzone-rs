use ::libc;
extern "C" {
    #[no_mangle]
    fn rand() -> libc::c_int;
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
    fn pie_Draw3DShape(shape: *mut iIMDShape, frame: libc::c_int,
                       team: libc::c_int, colour: UDWORD, specular: UDWORD,
                       pieFlag: libc::c_int, pieData: libc::c_int);
    //*************************************************************************
    #[no_mangle]
    fn pie_MatBegin();
    #[no_mangle]
    fn pie_MatEnd();
    #[no_mangle]
    fn pie_TRANSLATE(x: libc::c_int, y: libc::c_int, z: libc::c_int);
    #[no_mangle]
    fn pie_MatScale(percent: UDWORD);
    #[no_mangle]
    fn pie_MatRotX(x: libc::c_int);
    #[no_mangle]
    fn pie_MatRotY(y: libc::c_int);
    #[no_mangle]
    fn clipXY(x: SDWORD, y: SDWORD) -> BOOL;
    #[no_mangle]
    static mut player: iView;
    #[no_mangle]
    static mut terrainMidX: UDWORD;
    #[no_mangle]
    static mut terrainMidY: UDWORD;
    #[no_mangle]
    static mut rx: int32;
    #[no_mangle]
    static mut rz: int32;
    #[no_mangle]
    static mut visibleXTiles: UDWORD;
    #[no_mangle]
    static mut visibleYTiles: UDWORD;
    /* The time for the last frame */
    #[no_mangle]
    static mut frameTime: UDWORD;
    #[no_mangle]
    fn getImdFromIndex(index: UDWORD) -> *mut iIMDShape;
    #[no_mangle]
    static mut terrainTypes: [UBYTE; 255];
    #[no_mangle]
    static mut mapWidth: UDWORD;
    #[no_mangle]
    static mut mapHeight: UDWORD;
    #[no_mangle]
    static mut psMapTiles: *mut MAPTILE;
    #[no_mangle]
    fn map_Height(x: UDWORD, y: UDWORD) -> SWORD;
    #[no_mangle]
    fn gamePaused() -> BOOL;
    #[no_mangle]
    fn addEffect(pos: *mut iVector, group: EFFECT_GROUP, type_0: EFFECT_TYPE,
                 specified: BOOL, imd: *mut iIMDShape, lit: BOOL);
    #[no_mangle]
    fn effectSetSize(size: UDWORD);
    #[no_mangle]
    fn lightDoFogAndIllumination(brightness: UBYTE, dx: SDWORD, dz: SDWORD,
                                 pSpecular: *mut UDWORD) -> UDWORD;
    /* add an object to the current render list */
    #[no_mangle]
    fn bucketAddTypeToList(objectType: RENDER_TYPE, object: *mut libc::c_void)
     -> BOOL;
    #[no_mangle]
    static mut selectedPlayer: UDWORD;
}
pub type UBYTE = libc::c_uchar;
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
pub type FRACT = libc::c_float;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PIEVECTORF {
    pub x: FRACT,
    pub y: FRACT,
    pub z: FRACT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iView {
    pub p: iVector,
    pub r: iVector,
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
pub type C2RustUnnamed = libc::c_uint;
pub const MI_TOO_MANY: C2RustUnnamed = 44;
pub const MI_FIREWORK: C2RustUnnamed = 43;
pub const MI_DEBRIS4: C2RustUnnamed = 42;
pub const MI_DEBRIS3: C2RustUnnamed = 41;
pub const MI_DEBRIS2: C2RustUnnamed = 40;
pub const MI_DEBRIS1: C2RustUnnamed = 39;
pub const MI_DEBRIS0: C2RustUnnamed = 38;
pub const MI_WRECK4: C2RustUnnamed = 37;
pub const MI_WRECK3: C2RustUnnamed = 36;
pub const MI_WRECK2: C2RustUnnamed = 35;
pub const MI_WRECK1: C2RustUnnamed = 34;
pub const MI_WRECK0: C2RustUnnamed = 33;
pub const MI_BLIP_ARTEFACT: C2RustUnnamed = 32;
pub const MI_BLIP_RESOURCE: C2RustUnnamed = 31;
pub const MI_BLIP_ENEMY: C2RustUnnamed = 30;
pub const MI_SHOCK: C2RustUnnamed = 29;
pub const MI_LANDING: C2RustUnnamed = 28;
pub const MI_KICK: C2RustUnnamed = 27;
pub const MI_SPLASH: C2RustUnnamed = 26;
pub const MI_SNOW: C2RustUnnamed = 25;
pub const MI_RAIN: C2RustUnnamed = 24;
pub const MI_MFLARE: C2RustUnnamed = 23;
pub const MI_TESLA: C2RustUnnamed = 22;
pub const MI_FLAME: C2RustUnnamed = 21;
pub const MI_TRAIL: C2RustUnnamed = 20;
pub const MI_BLOOD: C2RustUnnamed = 19;
pub const MI_TRANSPORTER_SHADOW: C2RustUnnamed = 18;
pub const MI_SHADOW: C2RustUnnamed = 17;
pub const MI_BLIP: C2RustUnnamed = 16;
pub const MI_PLASMA: C2RustUnnamed = 15;
pub const MI_SMALL_STEAM: C2RustUnnamed = 14;
pub const MI_DROID_DAMAGE: C2RustUnnamed = 13;
pub const MI_WATER: C2RustUnnamed = 12;
pub const MI_CYBORG_BODY: C2RustUnnamed = 11;
pub const MI_CYBORG_ARM: C2RustUnnamed = 10;
pub const MI_CYBORG_LEGS: C2RustUnnamed = 9;
pub const MI_CYBORG_HEAD: C2RustUnnamed = 8;
pub const MI_BABA_BODY: C2RustUnnamed = 7;
pub const MI_BABA_ARM: C2RustUnnamed = 6;
pub const MI_BABA_LEGS: C2RustUnnamed = 5;
pub const MI_BABA_HEAD: C2RustUnnamed = 4;
pub const MI_SMALL_SMOKE: C2RustUnnamed = 3;
pub const MI_CONSTRUCTION: C2RustUnnamed = 2;
pub const MI_EXPLOSION_MEDIUM: C2RustUnnamed = 1;
pub const MI_EXPLOSION_SMALL: C2RustUnnamed = 0;
pub type _terrain_type = libc::c_uint;
pub const TER_MAX: _terrain_type = 12;
pub const TER_SLUSH: _terrain_type = 11;
pub const TER_SHEETICE: _terrain_type = 10;
pub const TER_RUBBLE: _terrain_type = 9;
pub const TER_CLIFFFACE: _terrain_type = 8;
pub const TER_WATER: _terrain_type = 7;
pub const TER_ROAD: _terrain_type = 6;
pub const TER_PINKROCK: _terrain_type = 5;
pub const TER_REDBRUSH: _terrain_type = 4;
pub const TER_GREENMUD: _terrain_type = 3;
pub const TER_BAKEDEARTH: _terrain_type = 2;
pub const TER_SANDYBRUSH: _terrain_type = 1;
pub const TER_SAND: _terrain_type = 0;
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
pub type MAPTILE = _maptile;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _atmosParticle {
    pub status: UBYTE,
    pub type_0: UBYTE,
    pub size: UDWORD,
    pub position: PIEVECTORF,
    pub velocity: PIEVECTORF,
    pub imd: *mut iIMDShape,
}
pub type ATPART = _atmosParticle;
pub type WT_CLASS = libc::c_uint;
pub const WT_NONE: WT_CLASS = 2;
pub const WT_SNOWING: WT_CLASS = 1;
pub const WT_RAINING: WT_CLASS = 0;
pub const APS_INACTIVE: C2RustUnnamed_0 = 1;
pub type AP_TYPE = libc::c_uint;
pub const AP_SNOW: AP_TYPE = 1;
pub const AP_RAIN: AP_TYPE = 0;
pub const APS_ACTIVE: C2RustUnnamed_0 = 0;
pub type EFFECT_TYPE = libc::c_uint;
pub const FIREWORK_TYPE_LAUNCHER: EFFECT_TYPE = 42;
pub const FIREWORK_TYPE_STARBURST: EFFECT_TYPE = 41;
pub const WAYPOINT_TYPE: EFFECT_TYPE = 40;
pub const SAT_LASER_STANDARD: EFFECT_TYPE = 39;
pub const DESTRUCTION_TYPE_SKYSCRAPER: EFFECT_TYPE = 38;
pub const DESTRUCTION_TYPE_FEATURE: EFFECT_TYPE = 37;
pub const DESTRUCTION_TYPE_WALL_SECTION: EFFECT_TYPE = 36;
pub const DESTRUCTION_TYPE_POWER_STATION: EFFECT_TYPE = 35;
pub const DESTRUCTION_TYPE_STRUCTURE: EFFECT_TYPE = 34;
pub const DESTRUCTION_TYPE_DROID: EFFECT_TYPE = 33;
pub const DUST_TYPE_NORMAL: EFFECT_TYPE = 32;
pub const BLOOD_TYPE_NORMAL: EFFECT_TYPE = 31;
pub const CONSTRUCTION_TYPE_DRIFTING: EFFECT_TYPE = 30;
pub const FIRE_TYPE_SMOKY_BLUE: EFFECT_TYPE = 29;
pub const FIRE_TYPE_SMOKY: EFFECT_TYPE = 28;
pub const FIRE_TYPE_LOCALISED: EFFECT_TYPE = 27;
pub const SMOKE_TYPE_TRAIL: EFFECT_TYPE = 26;
pub const SMOKE_TYPE_STEAM: EFFECT_TYPE = 25;
pub const SMOKE_TYPE_BILLOW: EFFECT_TYPE = 24;
pub const SMOKE_TYPE_DRIFTING_SMALL: EFFECT_TYPE = 23;
pub const SMOKE_TYPE_DRIFTING_HIGH: EFFECT_TYPE = 22;
pub const SMOKE_TYPE_DRIFTING: EFFECT_TYPE = 21;
pub const GRAVITON_TYPE_GIBLET: EFFECT_TYPE = 20;
pub const GRAVITON_TYPE_EMITTING_ST: EFFECT_TYPE = 19;
pub const GRAVITON_TYPE_EMITTING_DR: EFFECT_TYPE = 18;
pub const GRAVITON_TYPE_STANDARD: EFFECT_TYPE = 17;
pub const EXPLOSION_TYPE_SHOCKWAVE: EFFECT_TYPE = 16;
pub const EXPLOSION_TYPE_LAND_LIGHT: EFFECT_TYPE = 15;
pub const EXPLOSION_TYPE_KICKUP: EFFECT_TYPE = 14;
pub const EXPLOSION_TYPE_PLASMA: EFFECT_TYPE = 13;
pub const EXPLOSION_TYPE_FLARE: EFFECT_TYPE = 12;
pub const EXPLOSION_TYPE_DISCOVERY: EFFECT_TYPE = 11;
pub const EXPLOSION_TYPE_TESLA: EFFECT_TYPE = 10;
pub const EXPLOSION_TYPE_LASER: EFFECT_TYPE = 9;
pub const EXPLOSION_TYPE_FLAMETHROWER: EFFECT_TYPE = 8;
pub const EXPLOSION_TYPE_SPECIFIED_FIXME: EFFECT_TYPE = 7;
pub const EXPLOSION_TYPE_SPECIFIED_SOLID: EFFECT_TYPE = 6;
pub const EXPLOSION_TYPE_NOT_FACING: EFFECT_TYPE = 5;
pub const EXPLOSION_TYPE_SPECIFIED: EFFECT_TYPE = 4;
pub const EXPLOSION_TYPE_LARGE: EFFECT_TYPE = 3;
pub const EXPLOSION_TYPE_MEDIUM: EFFECT_TYPE = 2;
pub const EXPLOSION_TYPE_VERY_SMALL: EFFECT_TYPE = 1;
pub const EXPLOSION_TYPE_SMALL: EFFECT_TYPE = 0;
pub type EFFECT_GROUP = libc::c_uint;
pub const EFFECT_FIREWORK: EFFECT_GROUP = 11;
pub const EFFECT_FIRE: EFFECT_GROUP = 10;
pub const EFFECT_DUST_BALL: EFFECT_GROUP = 9;
pub const EFFECT_SAT_LASER: EFFECT_GROUP = 8;
pub const EFFECT_DESTRUCTION: EFFECT_GROUP = 7;
pub const EFFECT_BLOOD: EFFECT_GROUP = 6;
pub const EFFECT_WAYPOINT: EFFECT_GROUP = 5;
pub const EFFECT_GRAVITON: EFFECT_GROUP = 4;
pub const EFFECT_STRUCTURE: EFFECT_GROUP = 3;
pub const EFFECT_SMOKE: EFFECT_GROUP = 2;
pub const EFFECT_CONSTRUCTION: EFFECT_GROUP = 1;
pub const EFFECT_EXPLOSION: EFFECT_GROUP = 0;
/* bucket3D.h */
pub type RENDER_TYPE = _render_type;
pub type _render_type = libc::c_uint;
pub const RENDER_PARTICLE: _render_type = 16;
pub const RENDER_DELIVPOINT: _render_type = 15;
pub const RENDER_MIST: _render_type = 14;
pub const RENDER_WATERTILE: _render_type = 13;
pub const RENDER_TILE: _render_type = 12;
pub const RENDER_SMOKE: _render_type = 11;
pub const RENDER_GRAVITON: _render_type = 10;
pub const RENDER_EFFECT: _render_type = 9;
pub const RENDER_EXPLOSION: _render_type = 8;
pub const RENDER_ANIMATION: _render_type = 7;
pub const RENDER_SHADOW: _render_type = 6;
pub const RENDER_PROJECTILE_TRANSPARENT: _render_type = 5;
pub const RENDER_PROJECTILE: _render_type = 4;
pub const RENDER_PROXMSG: _render_type = 3;
pub const RENDER_FEATURE: _render_type = 2;
pub const RENDER_STRUCTURE: _render_type = 1;
pub const RENDER_DROID: _render_type = 0;
pub type C2RustUnnamed_0 = libc::c_uint;
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
    return psMapTiles.offset(x as
                                 isize).offset(y.wrapping_mul(mapWidth) as
                                                   isize);
}
#[no_mangle]
pub static mut asAtmosParts: [ATPART; 1024] =
    [ATPART{status: 0,
            type_0: 0,
            size: 0,
            position: PIEVECTORF{x: 0., y: 0., z: 0.,},
            velocity: PIEVECTORF{x: 0., y: 0., z: 0.,},
            imd: 0 as *const iIMDShape as *mut iIMDShape,}; 1024];
static mut fraction: FRACT = 0.;
static mut freeParticle: UDWORD = 0;
static mut weather: UDWORD = 0;
// -----------------------------------------------------------------------------
// -----------------------------------------------------------------------------
// -----------------------------------------------------------------------------
// -----------------------------------------------------------------------------
/* Setup all the particles */
#[no_mangle]
pub unsafe extern "C" fn atmosInitSystem() {
    let mut i: UDWORD = 0;
    i = 0 as libc::c_int as UDWORD;
    while i < (32 as libc::c_int * 32 as libc::c_int) as libc::c_uint {
        /* None are being used initially */
        asAtmosParts[i as usize].status =
            APS_INACTIVE as libc::c_int as UBYTE;
        i = i.wrapping_add(1)
    }
    /* Start at the beginning */
    freeParticle = 0 as libc::c_int as UDWORD;
    /* No weather to start with */
    weather = WT_NONE as libc::c_int as UDWORD;
}
// -----------------------------------------------------------------------------
/* Move the particles */
#[no_mangle]
pub unsafe extern "C" fn atmosUpdateSystem() {
    let mut i: UDWORD = 0;
    let mut numberToAdd: UDWORD = 0;
    let mut pos: iVector = iVector{x: 0, y: 0, z: 0,};
    /* Establish how long the last game frame took */
    fraction = frameTime as FRACT / 1000 as libc::c_int as libc::c_float;
    //	if(weather==WT_NONE)
 //	{
 //		return;
 //	}
    i = 0 as libc::c_int as UDWORD;
    while i < (32 as libc::c_int * 32 as libc::c_int) as libc::c_uint {
        /* See if it's active */
        if asAtmosParts[i as usize].status as libc::c_int ==
               APS_ACTIVE as libc::c_int {
            processParticle(&mut *asAtmosParts.as_mut_ptr().offset(i as
                                                                       isize));
        }
        i = i.wrapping_add(1)
    }
    /* This bit below needs to go into a "precipitation function" */
    if gamePaused() == 0 && weather != WT_NONE as libc::c_int as libc::c_uint
       {
        numberToAdd =
            if weather == WT_SNOWING as libc::c_int as libc::c_uint {
                2 as libc::c_int
            } else { 4 as libc::c_int } as UDWORD;
        /* Temporary stuff - just adds a few particles! */
        i = 0 as libc::c_int as UDWORD;
        while i < numberToAdd {
            pos.x =
                (player.p.x as
                     libc::c_uint).wrapping_add(visibleXTiles.wrapping_div(2
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_uint).wrapping_mul(128
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              libc::c_uint))
                    as int32;
            pos.z =
                (player.p.z as
                     libc::c_uint).wrapping_add(visibleYTiles.wrapping_div(2
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_uint).wrapping_mul(128
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              libc::c_uint))
                    as int32;
            pos.x =
                (pos.x as
                     libc::c_uint).wrapping_add(visibleXTiles.wrapping_div(2
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_uint).wrapping_sub((rand()
                                                                                                               as
                                                                                                               libc::c_uint).wrapping_rem(visibleXTiles)).wrapping_mul(128
                                                                                                                                                                           as
                                                                                                                                                                           libc::c_int
                                                                                                                                                                           as
                                                                                                                                                                           libc::c_uint))
                    as int32 as int32;
            pos.z =
                (pos.z as
                     libc::c_uint).wrapping_add(visibleXTiles.wrapping_div(2
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_uint).wrapping_sub((rand()
                                                                                                               as
                                                                                                               libc::c_uint).wrapping_rem(visibleXTiles)).wrapping_mul(128
                                                                                                                                                                           as
                                                                                                                                                                           libc::c_int
                                                                                                                                                                           as
                                                                                                                                                                           libc::c_uint))
                    as int32 as int32;
            pos.y = 1000 as libc::c_int;
            /* If we've got one on the grid */
            if pos.x > 0 as libc::c_int && pos.z > 0 as libc::c_int &&
                   pos.x <
                       mapWidth.wrapping_sub(1 as libc::c_int as
                                                 libc::c_uint).wrapping_mul(128
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_uint)
                           as SDWORD &&
                   pos.z <
                       mapHeight.wrapping_sub(1 as libc::c_int as
                                                  libc::c_uint).wrapping_mul(128
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_uint)
                           as SDWORD {
                /* On grid, so which particle shall we add? */
                match weather {
                    1 => { atmosAddParticle(&mut pos, AP_SNOW); }
                    0 => { atmosAddParticle(&mut pos, AP_RAIN); }
                    2 | _ => { }
                }
            }
            i = i.wrapping_add(1)
        }
    };
}
// -----------------------------------------------------------------------------
/* Moves one of the particles */
#[no_mangle]
pub unsafe extern "C" fn processParticle(mut psPart: *mut ATPART) {
    let mut groundHeight: SDWORD = 0;
    let mut pos: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut x: UDWORD = 0;
    let mut y: UDWORD = 0;
    let mut psTile: *mut MAPTILE = 0 as *mut MAPTILE;
    /* Only move if the game isn't paused */
    if gamePaused() == 0 {
        /* Move the particle - frame rate controlled */
        (*psPart).position.x += (*psPart).velocity.x * fraction;
        (*psPart).position.y += (*psPart).velocity.y * fraction;
        (*psPart).position.z += (*psPart).velocity.z * fraction;
        /* Wrap it around if it's gone off grid... */
        testParticleWrap(psPart);
        /* If it's gone off the WORLD... */
        if (*psPart).position.x < 0 as libc::c_int as libc::c_float ||
               (*psPart).position.z < 0 as libc::c_int as libc::c_float ||
               (*psPart).position.x >
                   mapWidth.wrapping_sub(1 as libc::c_int as
                                             libc::c_uint).wrapping_mul(128 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint)
                       as libc::c_float ||
               (*psPart).position.z >
                   mapHeight.wrapping_sub(1 as libc::c_int as
                                              libc::c_uint).wrapping_mul(128
                                                                             as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint)
                       as libc::c_float {
            /* The kill it */
            (*psPart).status = APS_INACTIVE as libc::c_int as UBYTE;
            return
        }
        /* What height is the ground under it? Only do if low enough...*/
        if (*psPart).position.y <
               (255 as libc::c_int * 2 as libc::c_int) as libc::c_float {
            /* Get ground height */
            groundHeight =
                map_Height((*psPart).position.x as SDWORD as UDWORD,
                           (*psPart).position.z as SDWORD as UDWORD) as
                    SDWORD;
            /* Are we below ground? */
            if ((*psPart).position.y as SDWORD) < groundHeight ||
                   (*psPart).position.y < 0.0f32 {
                /* Kill it and return */
                (*psPart).status = APS_INACTIVE as libc::c_int as UBYTE;
                if (*psPart).type_0 as libc::c_int == AP_RAIN as libc::c_int {
                    x =
                        ((*psPart).position.x as SDWORD >> 7 as libc::c_int)
                            as UDWORD;
                    y =
                        ((*psPart).position.z as SDWORD >> 7 as libc::c_int)
                            as UDWORD;
                    psTile = mapTile(x, y);
                    if terrainTypes[((*psTile).texture as libc::c_int &
                                         0x1ff as libc::c_int) as usize] as
                           libc::c_int == TER_WATER as libc::c_int &&
                           (*psTile).tileVisBits as libc::c_int &
                               (1 as libc::c_int) << selectedPlayer != 0 {
                        pos.x = (*psPart).position.x as SDWORD;
                        pos.z = (*psPart).position.z as SDWORD;
                        pos.y = groundHeight;
                        effectSetSize(60 as libc::c_int as UDWORD);
                        addEffect(&mut pos, EFFECT_EXPLOSION,
                                  EXPLOSION_TYPE_SPECIFIED, 1 as libc::c_int,
                                  getImdFromIndex(MI_SPLASH as libc::c_int as
                                                      UDWORD),
                                  0 as libc::c_int);
                    }
                }
                return
            }
        }
        if (*psPart).type_0 as libc::c_int == AP_SNOW as libc::c_int {
            if rand() % 30 as libc::c_int == 1 as libc::c_int {
                (*psPart).velocity.z =
                    (40 as libc::c_int - rand() % 80 as libc::c_int) as FRACT
            }
            if rand() % 30 as libc::c_int == 1 as libc::c_int {
                (*psPart).velocity.x =
                    (40 as libc::c_int - rand() % 80 as libc::c_int) as FRACT
            }
        }
    };
}
// -----------------------------------------------------------------------------
/* Adds a particle to the system if it can */
#[no_mangle]
pub unsafe extern "C" fn atmosAddParticle(mut pos: *mut iVector,
                                          mut type_0: AP_TYPE) {
    let mut activeCount: UDWORD = 0;
    let mut i: UDWORD = 0;
    i = freeParticle;
    activeCount = 0 as libc::c_int as UDWORD;
    while asAtmosParts[i as usize].status as libc::c_int ==
              APS_ACTIVE as libc::c_int &&
              activeCount <
                  (32 as libc::c_int * 32 as libc::c_int) as libc::c_uint {
        activeCount = activeCount.wrapping_add(1);
        /* Check for wrap around */
        if i >=
               (32 as libc::c_int * 32 as libc::c_int - 1 as libc::c_int) as
                   libc::c_uint {
            /* Go back to the first one */
            i = 0 as libc::c_int as UDWORD
        }
        i = i.wrapping_add(1)
    }
    /* Check the list isn't just full of essential effects */
    if activeCount >=
           (32 as libc::c_int * 32 as libc::c_int - 1 as libc::c_int) as
               libc::c_uint {
        /* All of the particles active!?!? */
        return
    } else { freeParticle = i }
    /* Record it's type */
    asAtmosParts[freeParticle as usize].type_0 = type_0 as UBYTE;
    /* Make it active */
    asAtmosParts[freeParticle as usize].status =
        APS_ACTIVE as libc::c_int as UBYTE;
    /* Setup the imd */
    match type_0 as libc::c_uint {
        1 => {
            asAtmosParts[freeParticle as usize].imd =
                getImdFromIndex(MI_SNOW as libc::c_int as UDWORD);
            asAtmosParts[freeParticle as usize].size =
                80 as libc::c_int as UDWORD
        }
        0 => {
            asAtmosParts[freeParticle as usize].imd =
                getImdFromIndex(MI_RAIN as libc::c_int as UDWORD);
            asAtmosParts[freeParticle as usize].size =
                50 as libc::c_int as UDWORD
        }
        _ => { }
    }
    /* Setup position */
    asAtmosParts[freeParticle as usize].position.x = (*pos).x as FRACT;
    asAtmosParts[freeParticle as usize].position.y = (*pos).y as FRACT;
    asAtmosParts[freeParticle as usize].position.z = (*pos).z as FRACT;
    /* Setup its velocity */
    if type_0 as libc::c_uint == AP_RAIN as libc::c_int as libc::c_uint {
        asAtmosParts[freeParticle as usize].velocity.x =
            (rand() % 50 as libc::c_int) as FRACT;
        asAtmosParts[freeParticle as usize].velocity.y =
            (0 as libc::c_int -
                 (rand() % 300 as libc::c_int + 700 as libc::c_int)) as FRACT;
        asAtmosParts[freeParticle as usize].velocity.z =
            (rand() % 50 as libc::c_int) as FRACT
    } else {
        asAtmosParts[freeParticle as usize].velocity.x =
            (40 as libc::c_int - rand() % 80 as libc::c_int) as FRACT;
        asAtmosParts[freeParticle as usize].velocity.y =
            (0 as libc::c_int -
                 (rand() % 40 as libc::c_int + 80 as libc::c_int)) as FRACT;
        asAtmosParts[freeParticle as usize].velocity.z =
            (40 as libc::c_int - rand() % 80 as libc::c_int) as FRACT
    };
}
// -----------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn atmosDrawParticles() {
    let mut i: UDWORD = 0;
    if weather == WT_NONE as libc::c_int as libc::c_uint { return }
    /* Traverse the list */
    i = 0 as libc::c_int as UDWORD;
    while i < (32 as libc::c_int * 32 as libc::c_int) as libc::c_uint {
        /* Don't bother unless it's active */
        if asAtmosParts[i as usize].status as libc::c_int ==
               APS_ACTIVE as libc::c_int {
            /* Is it on the grid */
            if clipXY(asAtmosParts[i as usize].position.x as SDWORD as UDWORD
                          as SDWORD,
                      asAtmosParts[i as usize].position.z as SDWORD as UDWORD
                          as SDWORD) != 0 {
                /* Add it to the bucket */
                bucketAddTypeToList(RENDER_PARTICLE,
                                    &mut *asAtmosParts.as_mut_ptr().offset(i
                                                                               as
                                                                               isize)
                                        as *mut ATPART as *mut libc::c_void);
            }
        }
        i = i.wrapping_add(1)
    };
}
// -----------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn renderParticle(mut psPart: *mut ATPART) {
    let mut dv: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut brightness: UDWORD = 0;
    let mut specular: UDWORD = 0;
    let mut centreX: SDWORD = 0;
    let mut centreZ: SDWORD = 0;
    let mut x: SDWORD = 0;
    let mut y: SDWORD = 0;
    let mut z: SDWORD = 0;
    x = (*psPart).position.x as SDWORD;
    y = (*psPart).position.y as SDWORD;
    z = (*psPart).position.z as SDWORD;
    /* Transform it */
    dv.x =
        (x as
             UDWORD).wrapping_sub(player.p.x as
                                      libc::c_uint).wrapping_sub(terrainMidX.wrapping_mul(128
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              libc::c_uint))
            as int32; /* Push the indentity matrix */
    dv.y = y as UDWORD as int32; /* Get the x,z translation components */
    dv.z =
        terrainMidY.wrapping_mul(128 as libc::c_int as
                                     libc::c_uint).wrapping_sub((z as
                                                                     UDWORD).wrapping_sub(player.p.z
                                                                                              as
                                                                                              libc::c_uint))
            as int32; /* Translate */
    pie_MatBegin();
    pie_TRANSLATE(dv.x, dv.y, dv.z);
    rx = player.p.x & 128 as libc::c_int - 1 as libc::c_int;
    rz = player.p.z & 128 as libc::c_int - 1 as libc::c_int;
    pie_TRANSLATE(rx, 0 as libc::c_int, -rz);
    /* Make it face camera */
    pie_MatRotY(-player.r.y);
    pie_MatRotX(-player.r.x);
    /* Scale it... */
    pie_MatScale((*psPart).size);
    /* Draw it... */
    centreX =
        (player.p.x as
             libc::c_uint).wrapping_add(visibleXTiles.wrapping_div(2 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint)
                                            << 7 as libc::c_int) as SDWORD;
    centreZ =
        (player.p.z as
             libc::c_uint).wrapping_add(visibleYTiles.wrapping_div(2 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint)
                                            << 7 as libc::c_int) as SDWORD;
    brightness =
        lightDoFogAndIllumination(255 as libc::c_int as UBYTE, centreX - x,
                                  centreZ - z, &mut specular);
    pie_Draw3DShape((*psPart).imd, 0 as libc::c_int, 0 as libc::c_int,
                    brightness, 0 as libc::c_int as UDWORD,
                    0x8 as libc::c_int, 0 as libc::c_int);
    pie_MatEnd();
}
// -----------------------------------------------------------------------------
/*	Makes a particle wrap around - if it goes off the grid, then it returns
	on the other side - provided it's still on world... Which it should be */
#[no_mangle]
pub unsafe extern "C" fn testParticleWrap(mut psPart: *mut ATPART) {
    /* Gone off left side */
    if (*psPart).position.x < player.p.x as libc::c_float {
        (*psPart).position.x +=
            visibleXTiles.wrapping_mul(128 as libc::c_int as libc::c_uint) as
                libc::c_float
    } else if (*psPart).position.x >
                  (player.p.x as
                       libc::c_uint).wrapping_add(visibleXTiles.wrapping_mul(128
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_uint))
                      as libc::c_float {
        (*psPart).position.x -=
            visibleXTiles.wrapping_mul(128 as libc::c_int as libc::c_uint) as
                libc::c_float
    }
    /* Gone off right side */
    /* Gone off top */
    if (*psPart).position.z < player.p.z as libc::c_float {
        (*psPart).position.z +=
            visibleYTiles.wrapping_mul(128 as libc::c_int as libc::c_uint) as
                libc::c_float
    } else if (*psPart).position.z >
                  (player.p.z as
                       libc::c_uint).wrapping_add(visibleYTiles.wrapping_mul(128
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_uint))
                      as libc::c_float {
        (*psPart).position.z -=
            visibleYTiles.wrapping_mul(128 as libc::c_int as libc::c_uint) as
                libc::c_float
    };
}
/* Gone off bottom */
// -----------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn atmosSetWeatherType(mut type_0: WT_CLASS) {
    if type_0 as libc::c_uint == WT_NONE as libc::c_int as libc::c_uint {
        atmosInitSystem();
    } else { weather = type_0 as UDWORD };
}
// -----------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn atmosGetWeatherType() -> WT_CLASS {
    return weather as WT_CLASS;
}
