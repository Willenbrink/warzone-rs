use ::libc;
extern "C" {
    #[no_mangle]
    fn sqrt(_: libc::c_double) -> libc::c_double;
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
    /* Abort execution and generate a core-dump.  */
    #[no_mangle]
    fn abort() -> !;
    #[no_mangle]
    fn memMallocRelease(Size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn memFreeRelease(pMemToFree: *mut libc::c_void);
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_uint)
     -> *mut libc::c_void;
    #[no_mangle]
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    static mut psMapTiles: *mut MAPTILE;
    #[no_mangle]
    static mut mapHeight: UDWORD;
    #[no_mangle]
    static mut mapWidth: UDWORD;
    #[no_mangle]
    static mut terrainTypes: [UBYTE; 255];
    // reset the astar counters
    #[no_mangle]
    fn astarResetCounters();
    // A* findpath
    #[no_mangle]
    fn fpathAStarRoute(routeMode: SDWORD, psRoutePoints: *mut ASTAR_ROUTE,
                       sx: SDWORD, sy: SDWORD, fx: SDWORD, fy: SDWORD)
     -> SDWORD;
    #[no_mangle]
    static mut fpathBlockingTile:
           Option<unsafe extern "C" fn(_: SDWORD, _: SDWORD) -> BOOL>;
    // Check if the map tile at a location blocks a droid
    #[no_mangle]
    fn fpathGroundBlockingTile(x: SDWORD, y: SDWORD) -> BOOL;
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
pub type FRACT = libc::c_float;
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
pub struct _astar_route {
    pub asPos: [POINT; 50],
    pub finalX: SDWORD,
    pub finalY: SDWORD,
    pub numPoints: SDWORD,
}
/*
 * AStar.h
 *
 */
// the buffer to store a route in
pub type ASTAR_ROUTE = _astar_route;
// return codes for astar
pub type C2RustUnnamed = libc::c_uint;
// found a partial route to a nearby position
// routing cannot be finished this frame
// and should be continued the next frame
pub const ASR_NEAREST: C2RustUnnamed = 3;
// no route
pub const ASR_PARTIAL: C2RustUnnamed = 2;
// found a route
pub const ASR_FAILED: C2RustUnnamed = 1;
pub const ASR_OK: C2RustUnnamed = 0;
// route modes for astar
pub type C2RustUnnamed_0 = libc::c_uint;
// continue a route that was partially completed the last frame
// start a new route
pub const ASR_CONTINUE: C2RustUnnamed_0 = 1;
pub const ASR_NEWROUTE: C2RustUnnamed_0 = 0;
/*
 * GatewayDef.h
 *
 * Structure definitions for routing gateways.
 *
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gateway_link {
    pub psGateway: *mut _gateway,
    pub dist: SWORD,
    pub flags: SWORD,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gateway {
    pub x1: UBYTE,
    pub y1: UBYTE,
    pub x2: UBYTE,
    pub y2: UBYTE,
    pub zone1: UBYTE,
    pub zone2: UBYTE,
    pub psNext: *mut _gateway,
    pub psLinks: *mut GATEWAY_LINK,
    pub zone1Links: UBYTE,
    pub zone2Links: UBYTE,
    pub flags: UBYTE,
    pub dist: SWORD,
    pub est: SWORD,
    pub psOpen: *mut _gateway,
    pub psRoute: *mut _gateway,
}
pub type GATEWAY_LINK = _gateway_link;
// the flags that get reset by the router
pub type GATEWAY = _gateway;
// zone to the left/above the gateway
// zone to the right/below the gateway
// array of links to other zones
// number of links
// Data for the gateway router
// open or closed node
// distance so far and estimate to end
// Previous point in the route
// types of node for the gateway router
pub type _gw_node_flags = libc::c_uint;
// the gateway is a land/water link
// the gateway is to be ignored by the router
pub const GWR_WATERLINK: _gw_node_flags = 128;
// the gateway is totally blocked
pub const GWR_IGNORE: _gw_node_flags = 64;
// the gateway is part of the final route
pub const GWR_BLOCKED: _gw_node_flags = 32;
// the route goes from zone2 to zone1
pub const GWR_INROUTE: _gw_node_flags = 16;
// the route goes from zone1 to zone2
pub const GWR_ZONE2: _gw_node_flags = 8;
pub const GWR_ZONE1: _gw_node_flags = 4;
pub const GWR_CLOSED: _gw_node_flags = 2;
pub const GWR_OPEN: _gw_node_flags = 1;
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
/*
 * Gateway.c
 *
 * Routing gateway code.
 *
 */
// gateway linking printf's
//#define DEBUG_GROUP0
// water gate printf's
//#define DEBUG_GROUP1
// the list of gateways on the current map
#[no_mangle]
pub static mut psGateways: *mut GATEWAY = 0 as *const GATEWAY as *mut GATEWAY;
// the RLE map zones for each tile
#[no_mangle]
pub static mut apRLEZones: *mut *mut UBYTE =
    0 as *const *mut UBYTE as *mut *mut UBYTE;
// the number of map zones
#[no_mangle]
pub static mut gwNumZones: SDWORD = 0;
// The zone equivalence tables - shows which land zones
// border on a water zone
#[no_mangle]
pub static mut aNumEquiv: *mut UBYTE = 0 as *const UBYTE as *mut UBYTE;
#[no_mangle]
pub static mut apEquivZones: *mut *mut UBYTE =
    0 as *const *mut UBYTE as *mut *mut UBYTE;
// note which zones have a gateway to them and can therefore be reached
#[no_mangle]
pub static mut aZoneReachable: *mut UBYTE = 0 as *const UBYTE as *mut UBYTE;
/*
 * Gateway.h
 *
 * Interface to routing gateway code.
 *
 */
// the list of gateways on the current map
// the RLE map zones for each tile
// the number of map zones
// The zone equivalence tables
// Initialise the gateway system
// Initialise the gateway system
#[no_mangle]
pub unsafe extern "C" fn gwInitialise() -> BOOL {
    if psGateways.is_null() {
    } else {
        debug(LOG_ERROR,
              b"gwInitialise: gatway list has not been reset\x00" as *const u8
                  as *const libc::c_char);
    };
    if psGateways.is_null() {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"gateway.c\x00" as *const u8 as *const libc::c_char,
              97 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 13],
                                        &[libc::c_char; 13]>(b"gwInitialise\x00")).as_ptr(),
              b"psGateways == NULL\x00" as *const u8 as *const libc::c_char);
    };
    psGateways = 0 as *mut GATEWAY;
    //	if (!gwLinkGateways()) return FALSE;
    return 1 as libc::c_int;
}
// Shutdown the gateway system
// Shutdown the gateway system
#[no_mangle]
pub unsafe extern "C" fn gwShutDown() {
    let mut psNext: *mut GATEWAY = 0 as *mut GATEWAY;
    while !psGateways.is_null() {
        psNext = (*psGateways).psNext;
        gwFreeGateway(psGateways);
        psGateways = psNext
    }
    gwFreeZoneMap();
    gwFreeEquivTable();
    if !aZoneReachable.is_null() {
        memFreeRelease(aZoneReachable as *mut libc::c_void);
        aZoneReachable = 0 as *mut UBYTE
    };
}
// Add a gateway to the system
// Add a gateway to the system
#[no_mangle]
pub unsafe extern "C" fn gwNewGateway(mut x1: SDWORD, mut y1: SDWORD,
                                      mut x2: SDWORD, mut y2: SDWORD)
 -> BOOL {
    let mut psNew: *mut GATEWAY = 0 as *mut GATEWAY;
    let mut pos: SDWORD = 0;
    let mut temp: SDWORD = 0;
    if x1 < 0 as libc::c_int || x1 >= gwMapWidth() || y1 < 0 as libc::c_int ||
           y1 >= gwMapHeight() || x2 < 0 as libc::c_int || x2 >= gwMapWidth()
           || y2 < 0 as libc::c_int || y2 >= gwMapHeight() ||
           x1 != x2 && y1 != y2 {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"gwNewGateway: invalid coordinates\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"gateway.c\x00" as *const u8 as *const libc::c_char,
                  159 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 13],
                                            &[libc::c_char; 13]>(b"gwNewGateway\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    psNew =
        memMallocRelease(::std::mem::size_of::<GATEWAY>() as libc::c_ulong) as
            *mut GATEWAY;
    if psNew.is_null() {
        debug(LOG_ERROR,
              b"gwNewGateway: out of memory\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    // make sure the first coordinate is always the smallest
    if x2 < x1 {
        // y is the same, swap x
        temp = x2;
        x2 = x1;
        x1 = temp
    } else if y2 < y1 {
        // x is the same, swap y
        temp = y2;
        y2 = y1;
        y1 = temp
    }
    // initialise the gateway
    (*psNew).x1 = x1 as UBYTE;
    (*psNew).y1 = y1 as UBYTE;
    (*psNew).x2 = x2 as UBYTE;
    (*psNew).y2 = y2 as UBYTE;
    (*psNew).zone1 = 0 as libc::c_int as UBYTE;
    (*psNew).zone2 = 0 as libc::c_int as UBYTE;
    (*psNew).psLinks = 0 as *mut GATEWAY_LINK;
    (*psNew).flags = 0 as libc::c_int as UBYTE;
    // add the gateway to the list
    (*psNew).psNext = psGateways;
    psGateways = psNew;
    // set the map flags
    if x1 == x2 {
        // vertical gateway
        pos = y1;
        while pos <= y2 { gwSetGatewayFlag(x1, pos); pos += 1 }
    } else {
        // horizontal gateway
        pos = x1;
        while pos <= x2 { gwSetGatewayFlag(pos, y1); pos += 1 }
    }
    return 1 as libc::c_int;
}
// Add a land/water link gateway to the system
// Add a land/water link gateway to the system
#[no_mangle]
pub unsafe extern "C" fn gwNewLinkGateway(mut x: SDWORD, mut y: SDWORD)
 -> BOOL {
    let mut psNew: *mut GATEWAY = 0 as *mut GATEWAY;
    if x < 0 as libc::c_int || x >= gwMapWidth() || y < 0 as libc::c_int ||
           y >= gwMapHeight() {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"gwNewLinkGateway: invalid coordinates\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"gateway.c\x00" as *const u8 as *const libc::c_char,
                  231 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 17],
                                            &[libc::c_char; 17]>(b"gwNewLinkGateway\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    psNew =
        memMallocRelease(::std::mem::size_of::<GATEWAY>() as libc::c_ulong) as
            *mut GATEWAY;
    if psNew.is_null() {
        debug(LOG_ERROR,
              b"gwNewGateway: out of memory\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    // initialise the gateway
    (*psNew).x1 = x as UBYTE;
    (*psNew).y1 = y as UBYTE;
    (*psNew).x2 = x as UBYTE;
    (*psNew).y2 = y as UBYTE;
    (*psNew).zone1 = 0 as libc::c_int as UBYTE;
    (*psNew).zone2 = 0 as libc::c_int as UBYTE;
    (*psNew).psLinks = 0 as *mut GATEWAY_LINK;
    (*psNew).flags = GWR_WATERLINK as libc::c_int as UBYTE;
    // add the gateway to the list
    (*psNew).psNext = psGateways;
    psGateways = psNew;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gwBlockingTile(mut x: SDWORD, mut y: SDWORD)
 -> BOOL {
    let mut psTile: *mut MAPTILE = 0 as *mut MAPTILE;
    if x < 1 as libc::c_int || y < 1 as libc::c_int ||
           x >= mapWidth as SDWORD - 1 as libc::c_int ||
           y >= mapHeight as SDWORD - 1 as libc::c_int {
        // coords off map - auto blocking tile
        return 1 as libc::c_int
    }
    psTile = mapTile(x as UDWORD, y as UDWORD);
    if terrainTypes[((*psTile).texture as libc::c_int & 0x1ff as libc::c_int)
                        as usize] as libc::c_int ==
           TER_CLIFFFACE as libc::c_int {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
// scan for a particular zone on the map
// given a start point
unsafe extern "C" fn gwFindZone(mut zone: SDWORD, mut cx: SDWORD,
                                mut cy: SDWORD, mut px: *mut SDWORD,
                                mut py: *mut SDWORD) -> BOOL {
    let mut x: SDWORD = 0;
    let mut y: SDWORD = 0;
    let mut dist: SDWORD = 0;
    let mut maxDist: SDWORD = 0;
    maxDist =
        if gwMapWidth() > gwMapHeight() {
            gwMapWidth()
        } else { gwMapHeight() };
    dist = 0 as libc::c_int;
    while dist < maxDist {
        // scan accross the top
        y = cy - dist;
        x = cx - dist;
        while x <= cx + dist {
            if x >= 0 as libc::c_int && x < gwMapWidth() &&
                   y >= 0 as libc::c_int && y < gwMapHeight() &&
                   gwGetZone(x, y) == zone {
                *px = x;
                *py = y;
                return 1 as libc::c_int
            }
            x += 1 as libc::c_int
        }
        // scan down the left
        x = cx - dist;
        y = cy - dist;
        while y <= cy + dist {
            if x >= 0 as libc::c_int && x < gwMapWidth() &&
                   y >= 0 as libc::c_int && y < gwMapHeight() &&
                   gwGetZone(x, y) == zone {
                *px = x;
                *py = y;
                return 1 as libc::c_int
            }
            y += 1 as libc::c_int
        }
        // scan down the right
        x = cx + dist;
        y = cy - dist;
        while y <= cy + dist {
            if x >= 0 as libc::c_int && x < gwMapWidth() &&
                   y >= 0 as libc::c_int && y < gwMapHeight() &&
                   gwGetZone(x, y) == zone {
                *px = x;
                *py = y;
                return 1 as libc::c_int
            }
            y += 1 as libc::c_int
        }
        // scan accross the bottom
        y = cy + dist;
        x = cx - dist;
        while x <= cx + dist {
            if x >= 0 as libc::c_int && x < gwMapWidth() &&
                   y >= 0 as libc::c_int && y < gwMapHeight() &&
                   gwGetZone(x, y) == zone {
                *px = x;
                *py = y;
                return 1 as libc::c_int
            }
            x += 1 as libc::c_int
        }
        dist += 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
// find a rough center position for a zone
unsafe extern "C" fn gwCalcZoneCenter(mut zone: SDWORD, mut px: *mut SDWORD,
                                      mut py: *mut SDWORD) {
    let mut xsum: SDWORD = 0;
    let mut ysum: SDWORD = 0;
    let mut numtiles: SDWORD = 0;
    let mut x: SDWORD = 0;
    let mut y: SDWORD = 0;
    numtiles = 0 as libc::c_int;
    ysum = numtiles;
    xsum = ysum;
    y = 0 as libc::c_int;
    while y < gwMapHeight() {
        x = 0 as libc::c_int;
        while x < gwMapWidth() {
            if gwGetZone(x, y) == zone {
                xsum += x;
                ysum += y;
                numtiles += 1 as libc::c_int
            }
            x += 1 as libc::c_int
        }
        y += 1 as libc::c_int
    }
    if numtiles != 0 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"gwCalcZoneCenter: zone not found on map\x00" as *const u8 as
                  *const libc::c_char);
    };
    if numtiles != 0 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"gateway.c\x00" as *const u8 as *const libc::c_char,
              374 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 17],
                                        &[libc::c_char; 17]>(b"gwCalcZoneCenter\x00")).as_ptr(),
              b"numtiles != 0\x00" as *const u8 as *const libc::c_char);
    };
    x = xsum / numtiles;
    y = ysum / numtiles;
    if gwFindZone(zone, x, y, px, py) == 0 { *px = x; *py = y };
}
// check all the zones are of reasonable sizes
// check all the zones are of reasonable sizes
#[no_mangle]
pub unsafe extern "C" fn gwCheckZoneSizes() {
    let mut zone: SDWORD = 0;
    let mut xsum: SDWORD = 0;
    let mut ysum: SDWORD = 0;
    let mut numtiles: SDWORD = 0;
    let mut inzone: SDWORD = 0;
    let mut x: SDWORD = 0;
    let mut y: SDWORD = 0;
    let mut cx: SDWORD = 0;
    let mut cy: SDWORD = 0;
    zone = 1 as libc::c_int;
    while zone < 0xff as libc::c_int {
        inzone = 0 as libc::c_int;
        numtiles = inzone;
        ysum = numtiles;
        xsum = ysum;
        y = 0 as libc::c_int;
        while y < gwMapHeight() {
            x = 0 as libc::c_int;
            while x < gwMapWidth() {
                if gwGetZone(x, y) == zone {
                    xsum += x;
                    ysum += y;
                    numtiles += 1 as libc::c_int;
                    if gwBlockingTile(x, y) == 0 {
                        inzone += 1 as libc::c_int
                    }
                }
                x += 1 as libc::c_int
            }
            y += 1 as libc::c_int
        }
        if numtiles > 0 as libc::c_int {
            x = xsum / numtiles;
            y = ysum / numtiles;
            if gwFindZone(zone, x, y, &mut cx, &mut cy) == 0 {
                cx = x;
                cy = y
            }
            if inzone > 600 as libc::c_int {
                debug(LOG_ERROR,
                      b"gwCheckZoneSizes: warning zone %d at (%d,%d) is too large %d tiles (max %d)\n\x00"
                          as *const u8 as *const libc::c_char, zone, cx, cy,
                      inzone, 600 as libc::c_int);
            }
        }
        zone += 1 as libc::c_int
    };
}
// add the land/water link gateways
// add the land/water link gateways
#[no_mangle]
pub unsafe extern "C" fn gwGenerateLinkGates() -> BOOL {
    let mut zone: SDWORD = 0;
    let mut cx: SDWORD = 0;
    let mut cy: SDWORD = 0;
    if !apEquivZones.is_null() {
    } else {
        debug(LOG_ERROR,
              b"gwGenerateLinkGates: no zone equivalence table\x00" as
                  *const u8 as *const libc::c_char);
    };
    if !apEquivZones.is_null() {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"gateway.c\x00" as *const u8 as *const libc::c_char,
              439 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 20],
                                        &[libc::c_char; 20]>(b"gwGenerateLinkGates\x00")).as_ptr(),
              b"apEquivZones != NULL\x00" as *const u8 as
                  *const libc::c_char);
    };
    debug(LOG_NEVER,
          b"Generating water link Gateways....\x00" as *const u8 as
              *const libc::c_char);
    zone = 1 as libc::c_int;
    while zone < gwNumZones {
        if *aNumEquiv.offset(zone as isize) as libc::c_int > 0 as libc::c_int
           {
            //			loadingScreenCallback();
            // got a water zone that borders on land
			// find it's center
            gwCalcZoneCenter(zone, &mut cx, &mut cy);
            if gwNewLinkGateway(cx, cy) == 0 { return 0 as libc::c_int }
        }
        zone += 1 as libc::c_int
    }
    debug(LOG_NEVER, b"Done\n\x00" as *const u8 as *const libc::c_char);
    return 1 as libc::c_int;
}
// Get number of gateways.
// Return the number of gateways.
#[no_mangle]
pub unsafe extern "C" fn gwNumGateways() -> UDWORD {
    let mut psCurr: *mut GATEWAY = 0 as *mut GATEWAY;
    let mut NumGateways: UDWORD = 0 as libc::c_int as UDWORD;
    psCurr = psGateways;
    while !psCurr.is_null() {
        NumGateways = NumGateways.wrapping_add(1);
        psCurr = (*psCurr).psNext
    }
    return NumGateways;
}
// Get the gateway list.
#[no_mangle]
pub unsafe extern "C" fn gwGetGateways() -> *mut GATEWAY {
    return psGateways;
}
// Release a gateway
// Release a gateway
#[no_mangle]
pub unsafe extern "C" fn gwFreeGateway(mut psDel: *mut GATEWAY) {
    let mut pos: SDWORD = 0;
    let mut psPrev: *mut GATEWAY = 0 as *mut GATEWAY;
    let mut psCurr: *mut GATEWAY = 0 as *mut GATEWAY;
    psPrev = 0 as *mut GATEWAY;
    psCurr = psGateways;
    while !psCurr.is_null() {
        if psCurr == psDel { break ; }
        psPrev = psCurr;
        psCurr = (*psCurr).psNext
    }
    if !psCurr.is_null() {
    } else {
        debug(LOG_ERROR,
              b"LIST_REMOVE: gateway.c(%d): entry not found\x00" as *const u8
                  as *const libc::c_char, 494 as libc::c_int);
    };
    if !psCurr.is_null() {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"gateway.c\x00" as *const u8 as *const libc::c_char,
              494 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"gwFreeGateway\x00")).as_ptr(),
              b"psCurr!=NULL\x00" as *const u8 as *const libc::c_char);
    };
    if psPrev.is_null() {
        psGateways = (*psGateways).psNext
    } else if !psCurr.is_null() { (*psPrev).psNext = (*psCurr).psNext }
    if !psMapTiles.is_null() {
        // this lines fixes the bug where we were closing the gateways after freeing the map
        // clear the map flags
        if (*psDel).x1 as libc::c_int == (*psDel).x2 as libc::c_int {
            // vertical gateway
            pos = (*psDel).y1 as SDWORD;
            while pos <= (*psDel).y2 as libc::c_int {
                gwClearGatewayFlag((*psDel).x1 as SDWORD, pos);
                pos += 1
            }
        } else {
            // horizontal gateway
            pos = (*psDel).x1 as SDWORD;
            while pos <= (*psDel).x2 as libc::c_int {
                gwClearGatewayFlag(pos, (*psDel).y1 as SDWORD);
                pos += 1
            }
        }
    }
    if !(*psDel).psLinks.is_null() {
        memFreeRelease((*psDel).psLinks as *mut libc::c_void);
        (*psDel).psLinks = 0 as *mut GATEWAY_LINK
    }
    memFreeRelease(psDel as *mut libc::c_void);
    psDel = 0 as *mut GATEWAY;
}
// load a gateway list
// load a gateway list
#[no_mangle]
pub unsafe extern "C" fn gwLoadGateways(mut pFileBuffer: *mut libc::c_char,
                                        mut fileSize: UDWORD) -> BOOL {
    let mut numGW: SDWORD = 0;
    let mut x1: SDWORD = 0;
    let mut y1: SDWORD = 0;
    let mut x2: SDWORD = 0;
    let mut y2: SDWORD = 0;
    let mut pPos: *mut libc::c_char = 0 as *mut libc::c_char;
    // get the number of gateways
    pPos = pFileBuffer;
    sscanf(pPos as *mut STRING, b"%d\x00" as *const u8 as *const libc::c_char,
           &mut numGW as *mut SDWORD);
    while *pPos as libc::c_int != '\n' as i32 &&
              pPos < pFileBuffer.offset(fileSize as isize) {
        pPos = pPos.offset(1 as libc::c_int as isize)
    }
    pPos = pPos.offset(1 as libc::c_int as isize);
    while pPos < pFileBuffer.offset(fileSize as isize) &&
              numGW > 0 as libc::c_int {
        sscanf(pPos as *mut STRING,
               b"%d %d %d %d\x00" as *const u8 as *const libc::c_char,
               &mut x1 as *mut SDWORD, &mut y1 as *mut SDWORD,
               &mut x2 as *mut SDWORD, &mut y2 as *mut SDWORD);
        if gwNewGateway(x1, y1, x2, y2) == 0 { return 0 as libc::c_int }
        while *pPos as libc::c_int != '\n' as i32 &&
                  pPos < pFileBuffer.offset(fileSize as isize) {
            pPos = pPos.offset(1 as libc::c_int as isize)
        }
        pPos = pPos.offset(1 as libc::c_int as isize);
        numGW -= 1 as libc::c_int
    }
    return 1 as libc::c_int;
}
// check if a zone is in the equivalence table for a water zone
// check if a zone is in the equivalence table for a water zone
#[no_mangle]
pub unsafe extern "C" fn gwZoneInEquiv(mut mainZone: SDWORD,
                                       mut checkZone: SDWORD) -> BOOL {
    let mut i: SDWORD = 0;
    if apEquivZones.is_null() { return 0 as libc::c_int }
    //	ASSERT( apEquivZones != NULL,
//		"gwZoneInEquiv: no zone equivalence table" );
    i = 0 as libc::c_int;
    while i < *aNumEquiv.offset(mainZone as isize) as libc::c_int {
        if *(*apEquivZones.offset(mainZone as isize)).offset(i as isize) as
               libc::c_int == checkZone {
            return 1 as libc::c_int
        }
        i += 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
// find a route between two gateways and return
// its length
#[no_mangle]
pub unsafe extern "C" fn gwRouteLength(mut psStart: *mut GATEWAY,
                                       mut psEnd: *mut GATEWAY) -> SDWORD {
    let mut ret: SDWORD = 0;
    let mut sx: SDWORD = 0;
    let mut sy: SDWORD = 0;
    let mut ex: SDWORD = 0;
    let mut ey: SDWORD = 0;
    let mut xdiff: SDWORD = 0;
    let mut ydiff: SDWORD = 0;
    let mut i: SDWORD = 0;
    let mut sRoute: ASTAR_ROUTE =
        ASTAR_ROUTE{asPos: [POINT{x: 0, y: 0,}; 50],
                    finalX: 0,
                    finalY: 0,
                    numPoints: 0,};
    let mut routeMode: SDWORD = 0;
    let mut dist: SDWORD = 0;
    fpathBlockingTile =
        Some(gwBlockingTile as
                 unsafe extern "C" fn(_: SDWORD, _: SDWORD) -> BOOL);
    sx =
        ((*psStart).x1 as libc::c_int + (*psStart).x2 as libc::c_int) /
            2 as libc::c_int;
    sy =
        ((*psStart).y1 as libc::c_int + (*psStart).y2 as libc::c_int) /
            2 as libc::c_int;
    ex =
        ((*psEnd).x1 as libc::c_int + (*psEnd).x2 as libc::c_int) /
            2 as libc::c_int;
    ey =
        ((*psEnd).y1 as libc::c_int + (*psEnd).y2 as libc::c_int) /
            2 as libc::c_int;
    // force the router to finish a route
    routeMode = ASR_NEWROUTE as libc::c_int;
    sRoute.asPos[0 as libc::c_int as usize].x = -(1 as libc::c_int);
    sRoute.asPos[0 as libc::c_int as usize].y = -(1 as libc::c_int);
    loop  {
        astarResetCounters();
        sRoute.numPoints = 0 as libc::c_int;
        ret =
            fpathAStarRoute(routeMode, &mut sRoute, sx << 7 as libc::c_int,
                            sy << 7 as libc::c_int, ex << 7 as libc::c_int,
                            ey << 7 as libc::c_int);
        if ret == ASR_PARTIAL as libc::c_int {
            routeMode = ASR_CONTINUE as libc::c_int
        }
        if !(ret == ASR_PARTIAL as libc::c_int) { break ; }
    }
    if ret != ASR_FAILED as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"gwRouteLength: no route between gateways at (%d,%d) and (%d,%d)\x00"
                  as *const u8 as *const libc::c_char, sx, sy, ex, ey);
    };
    if ret != ASR_FAILED as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"gateway.c\x00" as *const u8 as *const libc::c_char,
              619 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"gwRouteLength\x00")).as_ptr(),
              b"ret != ASR_FAILED\x00" as *const u8 as *const libc::c_char);
    };
    // calculate the length of the route
    dist = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < sRoute.numPoints {
        xdiff = sx - sRoute.asPos[i as usize].x;
        ydiff = sy - sRoute.asPos[i as usize].y;
        dist +=
            sqrt((xdiff * xdiff + ydiff * ydiff) as libc::c_double) as FRACT
                as SDWORD;
        sx = sRoute.asPos[i as usize].x;
        sy = sRoute.asPos[i as usize].y;
        i += 1 as libc::c_int
    }
    xdiff = sx - ex;
    ydiff = sy - ey;
    dist +=
        sqrt((xdiff * xdiff + ydiff * ydiff) as libc::c_double) as FRACT as
            SDWORD;
    fpathBlockingTile =
        Some(fpathGroundBlockingTile as
                 unsafe extern "C" fn(_: SDWORD, _: SDWORD) -> BOOL);
    return dist;
}
// check that the initial flood fill tiles are not on a blocking tile
#[no_mangle]
pub unsafe extern "C" fn gwCheckFloodTiles(mut psGate: *mut GATEWAY) -> BOOL {
    let mut floodX: SDWORD = 0;
    let mut floodY: SDWORD = 0;
    // first zone is left/above
    if (*psGate).x1 as libc::c_int == (*psGate).x2 as libc::c_int {
        // vertical - go left
        floodX = (*psGate).x1 as libc::c_int - 1 as libc::c_int;
        floodY =
            ((*psGate).y2 as libc::c_int - (*psGate).y1 as libc::c_int) /
                2 as libc::c_int + (*psGate).y1 as libc::c_int
    } else {
        // horizontal - go above
        floodX =
            ((*psGate).x2 as libc::c_int - (*psGate).x1 as libc::c_int) /
                2 as libc::c_int + (*psGate).x1 as libc::c_int;
        floodY = (*psGate).y1 as libc::c_int - 1 as libc::c_int
    }
    if gwBlockingTile(floodX, floodY) != 0 { return 0 as libc::c_int }
    // second zone is right/below
    if (*psGate).x1 as libc::c_int == (*psGate).x2 as libc::c_int {
        // vertical - go right
        floodX = (*psGate).x1 as libc::c_int + 1 as libc::c_int;
        floodY =
            ((*psGate).y2 as libc::c_int - (*psGate).y1 as libc::c_int) /
                2 as libc::c_int + (*psGate).y1 as libc::c_int
    } else {
        // horizontal - go below
        floodX =
            ((*psGate).x2 as libc::c_int - (*psGate).x1 as libc::c_int) /
                2 as libc::c_int + (*psGate).x1 as libc::c_int;
        floodY = (*psGate).y1 as libc::c_int + 1 as libc::c_int
    }
    if gwBlockingTile(floodX, floodY) != 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
// link all the gateways together
// link all the gateways together
// link all the gateways together
#[no_mangle]
pub unsafe extern "C" fn gwLinkGateways() -> BOOL {
    let mut psCurr: *mut GATEWAY = 0 as *mut GATEWAY;
    let mut psLink: *mut GATEWAY = 0 as *mut GATEWAY;
    let mut x: SDWORD = 0;
    let mut y: SDWORD = 0;
    let mut gwX: SDWORD = 0;
    let mut gwY: SDWORD = 0;
    let mut zone1Links: SDWORD = 0;
    let mut zone2Links: SDWORD = 0;
    let mut link: SDWORD = 0;
    let mut zone: SDWORD = 0;
    let mut otherZone: SDWORD = 0;
    let mut zoneLinks: SDWORD = 0;
    let mut bZone1: BOOL = 0;
    let mut bAddLink: BOOL = 0;
    // note which zones have a gateway
    aZoneReachable =
        memMallocRelease((::std::mem::size_of::<UBYTE>() as
                              libc::c_ulong).wrapping_mul(gwNumZones as
                                                              libc::c_uint))
            as *mut UBYTE;
    if aZoneReachable.is_null() {
        debug(LOG_ERROR,
              b"gwLinkGateways: out of memory\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    memset(aZoneReachable as *mut libc::c_void, 0 as libc::c_int,
           (::std::mem::size_of::<UBYTE>() as
                libc::c_ulong).wrapping_mul(gwNumZones as libc::c_uint));
    // initialise the zones for the gateways
    psCurr = psGateways;
    while !psCurr.is_null() {
        // a gateway is always in it's own zone1
        (*psCurr).zone1 =
            gwGetZone((*psCurr).x1 as SDWORD, (*psCurr).y1 as SDWORD) as
                UBYTE;
        if (*psCurr).flags as libc::c_int & GWR_WATERLINK as libc::c_int != 0
           {
            // a water link gateway is only in one zone
            x = (*psCurr).x1 as SDWORD;
            y = (*psCurr).y1 as SDWORD
        } else if (*psCurr).x1 as libc::c_int == (*psCurr).x2 as libc::c_int {
            // vertical - go right
            x = (*psCurr).x1 as libc::c_int + 1 as libc::c_int;
            y =
                ((*psCurr).y2 as libc::c_int - (*psCurr).y1 as libc::c_int) /
                    2 as libc::c_int + (*psCurr).y1 as libc::c_int
        } else {
            // horizontal - go below
            x =
                ((*psCurr).x2 as libc::c_int - (*psCurr).x1 as libc::c_int) /
                    2 as libc::c_int + (*psCurr).x1 as libc::c_int;
            y = (*psCurr).y1 as libc::c_int + 1 as libc::c_int
        }
        (*psCurr).zone2 = gwGetZone(x, y) as UBYTE;
        if (*psCurr).flags as libc::c_int & GWR_WATERLINK as libc::c_int != 0
               || gwCheckFloodTiles(psCurr) != 0 {
        } else {
            debug(LOG_ERROR,
                  b"gwLinkGateways: Gateway at (%d,%d)->(%d,%d) is too close to a blocking tile. Zones %d, %d\x00"
                      as *const u8 as *const libc::c_char,
                  (*psCurr).x1 as libc::c_int, (*psCurr).y1 as libc::c_int,
                  (*psCurr).x2 as libc::c_int, (*psCurr).y2 as libc::c_int,
                  (*psCurr).zone1 as libc::c_int,
                  (*psCurr).zone2 as libc::c_int);
        };
        if (*psCurr).flags as libc::c_int & GWR_WATERLINK as libc::c_int != 0
               || gwCheckFloodTiles(psCurr) != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"gateway.c\x00" as *const u8 as *const libc::c_char,
                  746 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 15],
                                            &[libc::c_char; 15]>(b"gwLinkGateways\x00")).as_ptr(),
                  b"(psCurr->flags & GWR_WATERLINK) || gwCheckFloodTiles(psCurr)\x00"
                      as *const u8 as *const libc::c_char);
        };
        *aZoneReachable.offset((*psCurr).zone1 as isize) =
            1 as libc::c_int as UBYTE;
        *aZoneReachable.offset((*psCurr).zone2 as isize) =
            1 as libc::c_int as UBYTE;
        psCurr = (*psCurr).psNext
    }
    // now link all the gateways together
    psCurr = psGateways;
    while !psCurr.is_null() {
        //		loadingScreenCallback();
        gwX =
            ((*psCurr).x1 as libc::c_int + (*psCurr).x2 as libc::c_int) /
                2 as libc::c_int;
        gwY =
            ((*psCurr).y1 as libc::c_int + (*psCurr).y2 as libc::c_int) /
                2 as libc::c_int;
        // count the number of links
        zone1Links = 0 as libc::c_int;
        zone2Links = 0 as libc::c_int;
        psLink = psGateways;
        while !psLink.is_null() {
            if !(psLink == psCurr) {
                if (*psLink).zone1 as libc::c_int ==
                       (*psCurr).zone1 as libc::c_int ||
                       (*psLink).zone2 as libc::c_int ==
                           (*psCurr).zone1 as libc::c_int ||
                       (*psLink).flags as libc::c_int &
                           GWR_WATERLINK as libc::c_int != 0 &&
                           gwZoneInEquiv((*psLink).zone1 as SDWORD,
                                         (*psCurr).zone1 as SDWORD) != 0 &&
                           gwZoneInEquiv((*psLink).zone1 as SDWORD,
                                         (*psCurr).zone2 as SDWORD) == 0 {
                    zone1Links += 1 as libc::c_int
                }
                if (*psCurr).flags as libc::c_int &
                       GWR_WATERLINK as libc::c_int != 0 {
                    // calculating links for a water link gateway
                    if gwZoneInEquiv((*psCurr).zone1 as SDWORD,
                                     (*psLink).zone1 as SDWORD) != 0 ||
                           gwZoneInEquiv((*psCurr).zone1 as SDWORD,
                                         (*psLink).zone2 as SDWORD) != 0 {
                        zone2Links += 1 as libc::c_int
                    }
                } else if (*psLink).zone1 as libc::c_int ==
                              (*psCurr).zone2 as libc::c_int ||
                              (*psLink).zone2 as libc::c_int ==
                                  (*psCurr).zone2 as libc::c_int ||
                              (*psLink).flags as libc::c_int &
                                  GWR_WATERLINK as libc::c_int != 0 &&
                                  gwZoneInEquiv((*psLink).zone1 as SDWORD,
                                                (*psCurr).zone2 as SDWORD) !=
                                      0 &&
                                  gwZoneInEquiv((*psLink).zone1 as SDWORD,
                                                (*psCurr).zone1 as SDWORD) ==
                                      0 {
                    zone2Links += 1 as libc::c_int
                }
            }
            // don't link a gateway to itself
            psLink = (*psLink).psNext
        }
        if zone1Links + zone2Links > 0 as libc::c_int {
            (*psCurr).psLinks =
                memMallocRelease((::std::mem::size_of::<GATEWAY_LINK>() as
                                      libc::c_ulong).wrapping_mul((zone1Links
                                                                       +
                                                                       zone2Links)
                                                                      as
                                                                      libc::c_uint))
                    as *mut GATEWAY_LINK;
            if (*psCurr).psLinks.is_null() {
                debug(LOG_ERROR,
                      b"gwLinkGateways: out of memory\x00" as *const u8 as
                          *const libc::c_char);
                abort();
            }
        } else { (*psCurr).psLinks = 0 as *mut GATEWAY_LINK }
        (*psCurr).zone1Links = zone1Links as UBYTE;
        (*psCurr).zone2Links = zone2Links as UBYTE;
        // generate the links starting with all those through zone1
        link = 0 as libc::c_int;
        zone = (*psCurr).zone1 as SDWORD;
        otherZone = (*psCurr).zone2 as SDWORD;
        zoneLinks = zone1Links;
        bZone1 = 1 as libc::c_int;
        while link < zone1Links + zone2Links {
            psLink = psGateways;
            while !psLink.is_null() && link < zoneLinks {
                if !(psLink == psCurr) {
                    bAddLink = 0 as libc::c_int;
                    if bZone1 == 0 &&
                           (*psCurr).flags as libc::c_int &
                               GWR_WATERLINK as libc::c_int != 0 {
                        // calculating links for a water link gateway
                        if gwZoneInEquiv((*psCurr).zone1 as SDWORD,
                                         (*psLink).zone1 as SDWORD) != 0 ||
                               gwZoneInEquiv((*psCurr).zone1 as SDWORD,
                                             (*psLink).zone2 as SDWORD) != 0 {
                            bAddLink = 1 as libc::c_int
                        }
                    } else if (*psLink).zone1 as libc::c_int == zone ||
                                  (*psLink).zone2 as libc::c_int == zone ||
                                  (*psLink).flags as libc::c_int &
                                      GWR_WATERLINK as libc::c_int != 0 &&
                                      gwZoneInEquiv((*psLink).zone1 as SDWORD,
                                                    zone) != 0 &&
                                      gwZoneInEquiv((*psLink).zone1 as SDWORD,
                                                    otherZone) == 0 {
                        bAddLink = 1 as libc::c_int
                    }
                    if bAddLink != 0 {
                        let ref mut fresh0 =
                            (*(*psCurr).psLinks.offset(link as
                                                           isize)).psGateway;
                        *fresh0 = psLink;
                        (*(*psCurr).psLinks.offset(link as isize)).flags =
                            0 as libc::c_int as SWORD;
                        (*(*psCurr).psLinks.offset(link as isize)).dist =
                            gwRouteLength(psCurr, psLink) as SWORD;
                        link += 1 as libc::c_int
                    }
                }
                // don't link a gateway to itself
                psLink = (*psLink).psNext
            }
            // found all the links to zone1, now do it for zone2
            zone = (*psCurr).zone2 as SDWORD;
            otherZone = (*psCurr).zone1 as SDWORD;
            zoneLinks = zone1Links + zone2Links;
            bZone1 = 0 as libc::c_int
        }
        psCurr = (*psCurr).psNext
    }
    return 1 as libc::c_int;
}
// Get number of zone lines.
/* *****************************************************************************************************/
/*                            RLE Zone data access functions                                          */
// Get number of zone lines.
#[no_mangle]
pub unsafe extern "C" fn gwNumZoneLines() -> UDWORD {
    return gwMapHeight() as UDWORD;
}
// Get size of a zone line in bytes.
// Get the size of a zone line.
#[no_mangle]
pub unsafe extern "C" fn gwZoneLineSize(mut Line: UDWORD) -> UDWORD {
    let mut pCode: *mut UBYTE = 0 as *mut UBYTE;
    let mut pos: UDWORD = 0 as libc::c_int as UDWORD;
    let mut x: UDWORD = 0 as libc::c_int as UDWORD;
    if Line < gwMapHeight() as UDWORD {
    } else {
        debug(LOG_ERROR,
              b"gwNewZoneLine : Invalid line requested\x00" as *const u8 as
                  *const libc::c_char);
    };
    if Line < gwMapHeight() as UDWORD {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"gateway.c\x00" as *const u8 as *const libc::c_char,
              892 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 15],
                                        &[libc::c_char; 15]>(b"gwZoneLineSize\x00")).as_ptr(),
              b"Line < (UDWORD)gwMapHeight()\x00" as *const u8 as
                  *const libc::c_char);
    };
    if !apRLEZones.is_null() {
    } else {
        debug(LOG_ERROR,
              b"gwNewZoneLine : NULL Zone map\x00" as *const u8 as
                  *const libc::c_char);
    };
    if !apRLEZones.is_null() {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"gateway.c\x00" as *const u8 as *const libc::c_char,
              893 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 15],
                                        &[libc::c_char; 15]>(b"gwZoneLineSize\x00")).as_ptr(),
              b"apRLEZones != NULL\x00" as *const u8 as *const libc::c_char);
    };
    pCode = *apRLEZones.offset(Line as isize);
    while x < gwMapWidth() as UDWORD {
        x =
            (x as
                 libc::c_uint).wrapping_add(*pCode.offset(pos as isize) as
                                                libc::c_uint) as UDWORD as
                UDWORD;
        pos =
            (pos as
                 libc::c_uint).wrapping_add(2 as libc::c_int as libc::c_uint)
                as UDWORD as UDWORD
    }
    return pos;
}
// Create a new empty zone map but don't allocate the actual zones yet.
// Create a new empty zone map but don't allocate the actual zones yet.
//
#[no_mangle]
pub unsafe extern "C" fn gwNewZoneMap() -> BOOL {
    let mut i: UWORD = 0;
    if !apRLEZones.is_null() { gwFreeZoneMap(); }
    apRLEZones =
        memMallocRelease((::std::mem::size_of::<*mut UBYTE>() as
                              libc::c_ulong).wrapping_mul(gwMapHeight() as
                                                              libc::c_uint))
            as *mut *mut UBYTE;
    if apRLEZones.is_null() {
        debug(LOG_ERROR,
              b"gwNewZoneMap: Out of memory\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    i = 0 as libc::c_int as UWORD;
    while (i as libc::c_int) < gwMapHeight() {
        let ref mut fresh1 = *apRLEZones.offset(i as isize);
        *fresh1 = 0 as *mut UBYTE;
        i = i.wrapping_add(1)
    }
    return 1 as libc::c_int;
}
// Create a new empty zone map line in the zone map.
// Create a new empty zone map line in the zone map.
//
#[no_mangle]
pub unsafe extern "C" fn gwNewZoneLine(mut Line: UDWORD, mut Size: UDWORD)
 -> *mut UBYTE {
    if Line < gwMapHeight() as UDWORD {
    } else {
        debug(LOG_ERROR,
              b"gwNewZoneLine : Invalid line requested\x00" as *const u8 as
                  *const libc::c_char);
    };
    if Line < gwMapHeight() as UDWORD {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"gateway.c\x00" as *const u8 as *const libc::c_char,
              937 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"gwNewZoneLine\x00")).as_ptr(),
              b"Line < (UDWORD)gwMapHeight()\x00" as *const u8 as
                  *const libc::c_char);
    };
    if !apRLEZones.is_null() {
    } else {
        debug(LOG_ERROR,
              b"gwNewZoneLine : NULL Zone map\x00" as *const u8 as
                  *const libc::c_char);
    };
    if !apRLEZones.is_null() {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"gateway.c\x00" as *const u8 as *const libc::c_char,
              938 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"gwNewZoneLine\x00")).as_ptr(),
              b"apRLEZones != NULL\x00" as *const u8 as *const libc::c_char);
    };
    if !(*apRLEZones.offset(Line as isize)).is_null() {
        memFreeRelease(*apRLEZones.offset(Line as isize) as
                           *mut libc::c_void);
        let ref mut fresh2 = *apRLEZones.offset(Line as isize);
        *fresh2 = 0 as *mut UBYTE
    }
    let ref mut fresh3 = *apRLEZones.offset(Line as isize);
    *fresh3 = memMallocRelease(Size) as *mut UBYTE;
    if (*apRLEZones.offset(Line as isize)).is_null() {
        debug(LOG_ERROR,
              b"gwNewZoneLine: Out of memory\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    return *apRLEZones.offset(Line as isize);
}
// Create a NULL zone map for when there is no zone info loaded
// Create a NULL zone map for when there is no zone info loaded
#[no_mangle]
pub unsafe extern "C" fn gwCreateNULLZoneMap() -> BOOL {
    let mut y: SDWORD = 0;
    let mut pBuf: *mut UBYTE = 0 as *mut UBYTE;
    if gwNewZoneMap() == 0 { return 0 as libc::c_int }
    y = 0 as libc::c_int;
    while y < gwMapHeight() {
        pBuf = gwNewZoneLine(y as UDWORD, 2 as libc::c_int as UDWORD);
        if pBuf.is_null() { return 0 as libc::c_int }
        *pBuf.offset(0 as libc::c_int as isize) = gwMapWidth() as UBYTE;
        *pBuf.offset(1 as libc::c_int as isize) = 0 as libc::c_int as UBYTE;
        y += 1
    }
    return 1 as libc::c_int;
}
// release the RLE Zone map
// release the RLE Zone map
#[no_mangle]
pub unsafe extern "C" fn gwFreeZoneMap() {
    let mut i: SDWORD = 0;
    if !apRLEZones.is_null() {
        i = 0 as libc::c_int;
        while i < gwMapHeight() {
            memFreeRelease(*apRLEZones.offset(i as isize) as
                               *mut libc::c_void);
            let ref mut fresh4 = *apRLEZones.offset(i as isize);
            *fresh4 = 0 as *mut UBYTE;
            i += 1
        }
        memFreeRelease(apRLEZones as *mut libc::c_void);
        apRLEZones = 0 as *mut *mut UBYTE
    };
}
// Look up the zone for a coordinate
// Look up the zone for a coordinate
#[no_mangle]
pub unsafe extern "C" fn gwGetZone(mut x: SDWORD, mut y: SDWORD) -> SDWORD {
    let mut xPos: SDWORD = 0;
    let mut zone: SDWORD = 0;
    let mut rlePos: SDWORD = 0;
    zone = 0 as libc::c_int;
    if x >= 0 as libc::c_int && x < gwMapWidth() && y >= 0 as libc::c_int &&
           y < gwMapHeight() {
        rlePos = 0 as libc::c_int;
        xPos = 0 as libc::c_int;
        loop  {
            xPos +=
                *(*apRLEZones.offset(y as isize)).offset(rlePos as isize) as
                    libc::c_int;
            zone =
                *(*apRLEZones.offset(y as
                                         isize)).offset(rlePos as
                                                            isize).offset(1 as
                                                                              libc::c_int
                                                                              as
                                                                              isize)
                    as SDWORD;
            rlePos += 2 as libc::c_int;
            if !(xPos <= x) { break ; }
        }
        // xPos is where the next zone starts
    } else {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"gwGetZone: invalid coordinates\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"gateway.c\x00" as *const u8 as *const libc::c_char,
                  1018 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 10],
                                            &[libc::c_char; 10]>(b"gwGetZone\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
    }
    return zone;
}
// create an empty equivalence table
/* *****************************************************************************************************/
/*                   Zone equivalence data access functions                                           */
// create an empty equivalence table
#[no_mangle]
pub unsafe extern "C" fn gwNewEquivTable(mut numZones: SDWORD) -> BOOL {
    let mut i: SDWORD = 0;
    if numZones < 0xff as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"gwNewEquivTable: invalid number of zones\x00" as *const u8 as
                  *const libc::c_char);
    };
    if numZones < 0xff as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"gateway.c\x00" as *const u8 as *const libc::c_char,
              1036 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 16],
                                        &[libc::c_char; 16]>(b"gwNewEquivTable\x00")).as_ptr(),
              b"numZones < UBYTE_MAX\x00" as *const u8 as
                  *const libc::c_char);
    };
    gwNumZones = numZones;
    aNumEquiv =
        memMallocRelease((::std::mem::size_of::<UBYTE>() as
                              libc::c_ulong).wrapping_mul(numZones as
                                                              libc::c_uint))
            as *mut UBYTE;
    if aNumEquiv.is_null() {
        debug(LOG_ERROR,
              b"gwNewEquivTable: out of memory\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    i = 0 as libc::c_int;
    while i < numZones {
        *aNumEquiv.offset(i as isize) = 0 as libc::c_int as UBYTE;
        i += 1 as libc::c_int
    }
    apEquivZones =
        memMallocRelease((::std::mem::size_of::<*mut UBYTE>() as
                              libc::c_ulong).wrapping_mul(numZones as
                                                              libc::c_uint))
            as *mut *mut UBYTE;
    if apEquivZones.is_null() {
        debug(LOG_ERROR,
              b"gwNewEquivTable: out of memory\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    i = 0 as libc::c_int;
    while i < numZones {
        let ref mut fresh5 = *apEquivZones.offset(i as isize);
        *fresh5 = 0 as *mut UBYTE;
        i += 1 as libc::c_int
    }
    return 1 as libc::c_int;
}
// release the equivalence table
// release the equivalence table
#[no_mangle]
pub unsafe extern "C" fn gwFreeEquivTable() {
    let mut i: SDWORD = 0;
    if !aNumEquiv.is_null() {
        memFreeRelease(aNumEquiv as *mut libc::c_void);
        aNumEquiv = 0 as *mut UBYTE
    }
    if !apEquivZones.is_null() {
        i = 0 as libc::c_int;
        while i < gwNumZones {
            if !(*apEquivZones.offset(i as isize)).is_null() {
                memFreeRelease(*apEquivZones.offset(i as isize) as
                                   *mut libc::c_void);
                let ref mut fresh6 = *apEquivZones.offset(i as isize);
                *fresh6 = 0 as *mut UBYTE
            }
            i += 1 as libc::c_int
        }
        memFreeRelease(apEquivZones as *mut libc::c_void);
        apEquivZones = 0 as *mut *mut UBYTE
    }
    gwNumZones = 0 as libc::c_int;
}
// set the zone equivalence for a zone
// set the zone equivalence for a zone
#[no_mangle]
pub unsafe extern "C" fn gwSetZoneEquiv(mut zone: SDWORD,
                                        mut numEquiv: SDWORD,
                                        mut pEquiv: *mut UBYTE) -> BOOL {
    let mut i: SDWORD = 0;
    if !aNumEquiv.is_null() && !apEquivZones.is_null() {
    } else {
        debug(LOG_ERROR,
              b"gwSetZoneEquiv: equivalence arrays not initialised\x00" as
                  *const u8 as *const libc::c_char);
    };
    if !aNumEquiv.is_null() && !apEquivZones.is_null() {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"gateway.c\x00" as *const u8 as *const libc::c_char,
              1096 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 15],
                                        &[libc::c_char; 15]>(b"gwSetZoneEquiv\x00")).as_ptr(),
              b"aNumEquiv != NULL && apEquivZones != NULL\x00" as *const u8 as
                  *const libc::c_char);
    };
    if zone < gwNumZones {
    } else {
        debug(LOG_ERROR,
              b"gwSetZoneEquiv: invalid zone\x00" as *const u8 as
                  *const libc::c_char);
    };
    if zone < gwNumZones {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"gateway.c\x00" as *const u8 as *const libc::c_char,
              1098 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 15],
                                        &[libc::c_char; 15]>(b"gwSetZoneEquiv\x00")).as_ptr(),
              b"zone < gwNumZones\x00" as *const u8 as *const libc::c_char);
    };
    if numEquiv <= gwNumZones {
    } else {
        debug(LOG_ERROR,
              b"gwSetZoneEquiv: invalid number of zone equivalents\x00" as
                  *const u8 as *const libc::c_char);
    };
    if numEquiv <= gwNumZones {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"gateway.c\x00" as *const u8 as *const libc::c_char,
              1100 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 15],
                                        &[libc::c_char; 15]>(b"gwSetZoneEquiv\x00")).as_ptr(),
              b"numEquiv <= gwNumZones\x00" as *const u8 as
                  *const libc::c_char);
    };
    let ref mut fresh7 = *apEquivZones.offset(zone as isize);
    *fresh7 =
        memMallocRelease((::std::mem::size_of::<UBYTE>() as
                              libc::c_ulong).wrapping_mul(numEquiv as
                                                              libc::c_uint))
            as *mut UBYTE;
    if (*apEquivZones.offset(zone as isize)).is_null() {
        debug(LOG_ERROR,
              b"gwSetZoneEquiv: out of memory\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    *aNumEquiv.offset(zone as isize) = numEquiv as UBYTE;
    i = 0 as libc::c_int;
    while i < numEquiv {
        *(*apEquivZones.offset(zone as isize)).offset(i as isize) =
            *pEquiv.offset(i as isize);
        i += 1 as libc::c_int
    }
    return 1 as libc::c_int;
}
// get the size of the map
/* *****************************************************************************************************/
/*                   Gateway data access functions                                                    */
// get the size of the map
#[no_mangle]
pub unsafe extern "C" fn gwMapWidth() -> SDWORD { return mapWidth as SDWORD; }
#[no_mangle]
pub unsafe extern "C" fn gwMapHeight() -> SDWORD {
    return mapHeight as SDWORD;
}
// set the gateway flag on a tile
// set the gateway flag on a tile
#[no_mangle]
pub unsafe extern "C" fn gwSetGatewayFlag(mut x: SDWORD, mut y: SDWORD) {
    let ref mut fresh8 = (*mapTile(x as UDWORD, y as UDWORD)).tileInfoBits;
    *fresh8 = (*fresh8 as libc::c_int | 0x40 as libc::c_int) as UBYTE;
}
// clear the gateway flag on a tile
// clear the gateway flag on a tile
#[no_mangle]
pub unsafe extern "C" fn gwClearGatewayFlag(mut x: SDWORD, mut y: SDWORD) {
    let ref mut fresh9 = (*mapTile(x as UDWORD, y as UDWORD)).tileInfoBits;
    *fresh9 = (*fresh9 as libc::c_int & !(0x40 as libc::c_int)) as UBYTE;
}
// check whether a tile is water
// check whether a tile is water
#[no_mangle]
pub unsafe extern "C" fn gwTileIsWater(mut x: UDWORD, mut y: UDWORD) -> BOOL {
    return (terrainTypes[((*mapTile(x, y)).texture as libc::c_int &
                              0x1ff as libc::c_int) as usize] as libc::c_int
                == TER_WATER as libc::c_int) as libc::c_int;
}
// see if a zone is reachable
// see if a zone is reachable
#[no_mangle]
pub unsafe extern "C" fn gwZoneReachable(mut zone: SDWORD) -> BOOL {
    if zone >= 0 as libc::c_int && zone < gwNumZones {
    } else {
        debug(LOG_ERROR,
              b"gwZoneReachable: invalid zone\x00" as *const u8 as
                  *const libc::c_char);
    };
    if zone >= 0 as libc::c_int && zone < gwNumZones {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"gateway.c\x00" as *const u8 as *const libc::c_char,
              1191 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 16],
                                        &[libc::c_char; 16]>(b"gwZoneReachable\x00")).as_ptr(),
              b"zone >= 0 && zone < gwNumZones\x00" as *const u8 as
                  *const libc::c_char);
    };
    return *aZoneReachable.offset(zone as isize) as BOOL;
}
// get the terrain type of a map tile
/*SDWORD gwTileTerrainType(SDWORD x, SDWORD y)
{
	return TERRAIN_TYPE(mapTile((UDWORD)x,(UDWORD)y));
}*/
// check if the gateway flag is set on a tile
/*BOOL gwTileIsGateway(SDWORD x, SDWORD y)
{
	return (mapTile((UDWORD)x,(UDWORD)y)->tileInfoBits & BITS_GATEWAY) != 0;
}*/
