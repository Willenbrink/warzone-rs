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
    /* Return the absolute value of X.  */
    #[no_mangle]
    fn abs(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    static mut scrollMaxY: SDWORD;
    #[no_mangle]
    static mut scrollMinY: SDWORD;
    #[no_mangle]
    static mut scrollMaxX: SDWORD;
    #[no_mangle]
    static mut scrollMinX: SDWORD;
    #[no_mangle]
    static mut psMapTiles: *mut MAPTILE;
    #[no_mangle]
    static mut mapHeight: UDWORD;
    #[no_mangle]
    static mut mapWidth: UDWORD;
    #[no_mangle]
    static mut terrainTypes: [UBYTE; 255];
    /*
 * Gateway.h
 *
 * Interface to routing gateway code.
 *
 */
    // the list of gateways on the current map
    #[no_mangle]
    static mut psGateways: *mut GATEWAY;
    // check if a zone is in the equivalence table for a water zone
    #[no_mangle]
    fn gwZoneInEquiv(mainZone: SDWORD, checkZone: SDWORD) -> BOOL;
    // Look up the zone for a coordinate
    #[no_mangle]
    fn gwGetZone(x: SDWORD, y: SDWORD) -> SDWORD;
}
pub type UBYTE = libc::c_uchar;
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
pub type _gw_link_flags = libc::c_uint;
pub const GWRL_BLOCKED: _gw_link_flags = 4;
pub const GWRL_CHILD: _gw_link_flags = 2;
pub const GWRL_PARENT: _gw_link_flags = 1;
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
/*
 * GatewayRoute.h
 *
 * Interface to the gateway routing code
 *
 */
// what type of terrain the unit can move over
pub type _gwr_terrain_types = libc::c_uint;
pub const GWR_TER_ALL: _gwr_terrain_types = 255;
pub const GWR_TER_WATER: _gwr_terrain_types = 2;
pub const GWR_TER_LAND: _gwr_terrain_types = 1;
// return codes for the router
pub type _gwr_return_codes = libc::c_uint;
// the route did not start on a valid zone
// start and end points in the same zone
pub const GWR_NOZONE: _gwr_return_codes = 4;
// couldn't find a route
pub const GWR_SAMEZONE: _gwr_return_codes = 3;
// chose the nearest gateway to the target
pub const GWR_FAILED: _gwr_return_codes = 2;
// found a route
pub const GWR_NEAREST: _gwr_return_codes = 1;
pub const GWR_OK: _gwr_return_codes = 0;
/*
 * Generate a 'meta' route through the gateways to guide the normal routing
 *
 */
// print out the gateways examined while generating the route
//#define DEBUG_GROUP0
// print out the final route
//#define DEBUG_GROUP1
#[no_mangle]
pub static mut gwrDoMessage: BOOL = 0;
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
// the open list
static mut psOpenList: *mut GATEWAY = 0 as *const GATEWAY as *mut GATEWAY;
// estimate the distance to the target from a gateway
#[no_mangle]
pub unsafe extern "C" fn gwrEstimate(mut psGate: *mut GATEWAY, mut x: SDWORD,
                                     mut y: SDWORD) -> SDWORD {
    let mut gwx: SDWORD = 0;
    let mut gwy: SDWORD = 0;
    let mut dx: SDWORD = 0;
    let mut dy: SDWORD = 0;
    gwx =
        ((*psGate).x1 as libc::c_int + (*psGate).x2 as libc::c_int) /
            2 as libc::c_int;
    gwy =
        ((*psGate).y1 as libc::c_int + (*psGate).y2 as libc::c_int) /
            2 as libc::c_int;
    dx = gwx - x;
    dy = gwy - y;
    dx = abs(dx);
    dy = abs(dy);
    return if dx > dy {
               (dx) + dy / 2 as libc::c_int
           } else { (dx / 2 as libc::c_int) + dy };
}
// add a gateway to the open list
#[no_mangle]
pub unsafe extern "C" fn gwrOpenAdd(mut psGate: *mut GATEWAY) {
    (*psGate).flags =
        ((*psGate).flags as libc::c_int & !(GWR_CLOSED as libc::c_int)) as
            UBYTE;
    (*psGate).flags =
        ((*psGate).flags as libc::c_int | GWR_OPEN as libc::c_int) as UBYTE;
    (*psGate).psOpen = psOpenList;
    psOpenList = psGate;
}
// get the next gateway from the open list
#[no_mangle]
pub unsafe extern "C" fn gwrOpenGet() -> *mut GATEWAY {
    let mut minDist: SDWORD = 0;
    let mut dist: SDWORD = 0;
    let mut psCurr: *mut GATEWAY = 0 as *mut GATEWAY;
    let mut psPrev: *mut GATEWAY = 0 as *mut GATEWAY;
    let mut psParent: *mut GATEWAY = 0 as *mut GATEWAY;
    let mut psFound: *mut GATEWAY = 0 as *mut GATEWAY;
    if psOpenList.is_null() { return 0 as *mut GATEWAY }
    psPrev = 0 as *mut GATEWAY;
    minDist = 0x7fffffff as libc::c_int;
    psCurr = psOpenList;
    while !psCurr.is_null() {
        dist = (*psCurr).dist as libc::c_int + (*psCurr).est as libc::c_int;
        if dist < minDist {
            minDist = dist;
            psParent = psPrev;
            psFound = psCurr
        }
        psPrev = psCurr;
        psCurr = (*psCurr).psOpen
    }
    // remove the found gateway from the list
    if psParent.is_null() {
        psOpenList = (*psOpenList).psOpen
    } else { (*psParent).psOpen = (*psFound).psOpen }
    (*psFound).psOpen = 0 as *mut _gateway;
    (*psFound).flags =
        ((*psFound).flags as libc::c_int &
             !(GWR_OPEN as libc::c_int | GWR_CLOSED as libc::c_int)) as UBYTE;
    return psFound;
}
// check whether a gateway should be considered for routing
// (i.e. whether it is inside the current scroll limits)
#[no_mangle]
pub unsafe extern "C" fn gwrConsiderGateway(mut psGate: *mut GATEWAY)
 -> BOOL {
    return ((*psGate).x1 as libc::c_int >= scrollMinX &&
                (*psGate).x1 as libc::c_int <= scrollMaxX &&
                (*psGate).x2 as libc::c_int >= scrollMinX &&
                (*psGate).x2 as libc::c_int <= scrollMaxX &&
                (*psGate).y1 as libc::c_int >= scrollMinY &&
                (*psGate).y1 as libc::c_int <= scrollMaxY &&
                (*psGate).y2 as libc::c_int >= scrollMinY &&
                (*psGate).y2 as libc::c_int <= scrollMaxY &&
                (*psGate).flags as libc::c_int & GWR_BLOCKED as libc::c_int ==
                    0) as libc::c_int;
}
// check whether all the tiles on a gateway are blocked
#[no_mangle]
pub unsafe extern "C" fn gwrBlockedGateway(mut psGate: *mut GATEWAY,
                                           mut player: SDWORD,
                                           mut terrain: UDWORD) -> BOOL {
    //	SDWORD	pos;
    let mut blocked: BOOL = 0;
    let mut psTile: *mut MAPTILE = 0 as *mut MAPTILE;
    blocked = 0 as libc::c_int;
    psTile =
        mapTile((((*psGate).x1 as libc::c_int + (*psGate).x2 as libc::c_int) /
                     2 as libc::c_int) as UDWORD,
                (((*psGate).y1 as libc::c_int + (*psGate).y2 as libc::c_int) /
                     2 as libc::c_int) as UDWORD);
    if terrainTypes[((*psTile).texture as libc::c_int & 0x1ff as libc::c_int)
                        as usize] as libc::c_int == TER_WATER as libc::c_int
           && terrain & GWR_TER_WATER as libc::c_int as libc::c_uint == 0 {
        blocked = 1 as libc::c_int
    }
    if terrainTypes[((*psTile).texture as libc::c_int & 0x1ff as libc::c_int)
                        as usize] as libc::c_int != TER_WATER as libc::c_int
           && terrain & GWR_TER_LAND as libc::c_int as libc::c_uint == 0 {
        blocked = 1 as libc::c_int
    }
    if (*psGate).flags as libc::c_int & GWR_IGNORE as libc::c_int != 0 {
        blocked = 1 as libc::c_int
    }
    /*	blocked = TRUE;
	if (psGate->x1 == psGate->x2)
	{
		for(pos = psGate->y1; pos <= psGate->y2; pos += 1)
		{
			psTile = mapTile(psGate->x1, pos);
			if (!fpathBlockingTile(psGate->x1, pos) &&
				TEST_TILE_VISIBLE(player, psTile))
			{
				blocked = FALSE;
			}
		}
	}
	else
	{
		for(pos = psGate->x1; pos <= psGate->x2; pos += 1)
		{
			psTile = mapTile(pos, psGate->y1);
			if (!fpathBlockingTile(pos, psGate->y1) &&
				TEST_TILE_VISIBLE(player, psTile))
			{
				blocked = FALSE;
			}
		}
	}*/
    return blocked;
}
// A* findpath for the gateway system
// A* findpath for the gateway system
#[no_mangle]
pub unsafe extern "C" fn gwrAStarRoute(mut player: SDWORD,
                                       mut terrain: UDWORD, mut sx: SDWORD,
                                       mut sy: SDWORD, mut fx: SDWORD,
                                       mut fy: SDWORD,
                                       mut ppsRoute: *mut *mut GATEWAY)
 -> SDWORD {
    let mut tileSX: SDWORD = 0;
    let mut tileSY: SDWORD = 0;
    let mut tileFX: SDWORD = 0;
    let mut tileFY: SDWORD = 0;
    let mut currDist: SDWORD = 0;
    let mut retval: SDWORD = 0;
    let mut psCurr: *mut GATEWAY = 0 as *mut GATEWAY;
    let mut psNew: *mut GATEWAY = 0 as *mut GATEWAY;
    let mut psRoute: *mut GATEWAY = 0 as *mut GATEWAY;
    let mut psParent: *mut GATEWAY = 0 as *mut GATEWAY;
    let mut psNext: *mut GATEWAY = 0 as *mut GATEWAY;
    let mut psNearest: *mut GATEWAY = 0 as *mut GATEWAY;
    let mut psPrev: *mut GATEWAY = 0 as *mut GATEWAY;
    let mut zone: SDWORD = 0;
    let mut finalZone: SDWORD = 0;
    let mut link: SDWORD = 0;
    let mut finalLink: SDWORD = 0;
    let mut add: BOOL = 0;
    *ppsRoute = 0 as *mut GATEWAY;
    tileSX = sx >> 7 as libc::c_int;
    tileSY = sy >> 7 as libc::c_int;
    tileFX = fx >> 7 as libc::c_int;
    tileFY = fy >> 7 as libc::c_int;
    // reset the flags on the gateways
    psCurr = psGateways;
    while !psCurr.is_null() {
        (*psCurr).flags =
            ((*psCurr).flags as libc::c_int & !(0x3f as libc::c_int)) as
                UBYTE;
        (*psCurr).dist = 0x7fff as libc::c_int as SWORD;
        link = 0 as libc::c_int;
        while link <
                  (*psCurr).zone1Links as libc::c_int +
                      (*psCurr).zone2Links as libc::c_int {
            let ref mut fresh0 =
                (*(*psCurr).psLinks.offset(link as isize)).flags;
            *fresh0 =
                (*fresh0 as libc::c_int & !(0x3 as libc::c_int)) as SWORD;
            link += 1
        }
        psCurr = (*psCurr).psNext
    }
    // add all the gateways next to the start point
    zone = gwGetZone(tileSX, tileSY);
    finalZone = gwGetZone(tileFX, tileFY);
    if zone == finalZone { return GWR_SAMEZONE as libc::c_int }
    if zone == 0 as libc::c_int || finalZone == 0 as libc::c_int {
        return GWR_NOZONE as libc::c_int
    }
    psOpenList = 0 as *mut GATEWAY;
    psNew = psGateways;
    while !psNew.is_null() {
        add = 0 as libc::c_int;
        if (*psNew).zone1 as libc::c_int == zone {
            (*psNew).flags =
                ((*psNew).flags as libc::c_int | GWR_ZONE1 as libc::c_int) as
                    UBYTE;
            add = 1 as libc::c_int
        } else if (*psNew).zone2 as libc::c_int == zone {
            (*psNew).flags =
                ((*psNew).flags as libc::c_int | GWR_ZONE2 as libc::c_int) as
                    UBYTE;
            add = 1 as libc::c_int
        } else if (*psNew).flags as libc::c_int & GWR_WATERLINK as libc::c_int
                      != 0 &&
                      gwZoneInEquiv((*psNew).zone1 as SDWORD, zone) != 0 {
            (*psNew).flags =
                ((*psNew).flags as libc::c_int | GWR_ZONE2 as libc::c_int) as
                    UBYTE;
            add = 1 as libc::c_int
        }
        if gwrBlockedGateway(psNew, player, terrain) != 0 {
            (*psNew).flags =
                ((*psNew).flags as libc::c_int | GWR_BLOCKED as libc::c_int)
                    as UBYTE;
            add = 0 as libc::c_int
        }
        if add != 0 && gwrConsiderGateway(psNew) != 0 {
            (*psNew).dist = gwrEstimate(psNew, tileSX, tileSY) as SWORD;
            (*psNew).est = gwrEstimate(psNew, tileFX, tileFY) as SWORD;
            (*psNew).psRoute = 0 as *mut _gateway;
            gwrOpenAdd(psNew);
        }
        psNew = (*psNew).psNext
    }
    // search for a route
    psRoute = 0 as *mut GATEWAY;
    psNearest = 0 as *mut GATEWAY;
    while !psOpenList.is_null() {
        psCurr = gwrOpenGet();
        if gwrDoMessage != 0 {
            debug(LOG_ERROR, (*psCurr).y2 as *const libc::c_char);
        }
        if (*psCurr).zone1 as libc::c_int == finalZone ||
               (*psCurr).zone2 as libc::c_int == finalZone ||
               (*psCurr).flags as libc::c_int & GWR_WATERLINK as libc::c_int
                   != 0 &&
                   gwZoneInEquiv((*psCurr).zone1 as SDWORD, finalZone) != 0 {
            // reached the target
            psRoute = psCurr;
            if gwrDoMessage != 0 {
                debug(LOG_ERROR,
                      b"Found route\n\x00" as *const u8 as
                          *const libc::c_char);
            }
            break ;
        } else {
            if psNearest.is_null() ||
                   ((*psCurr).est as libc::c_int) <
                       (*psNearest).est as libc::c_int {
                psNearest = psCurr
            }
            if (*psCurr).flags as libc::c_int & GWR_WATERLINK as libc::c_int
                   != 0 {
                // water link gatway - route can continue to any other
			// gateway apart from the one it came from
                zone = (*psCurr).zone1 as SDWORD;
                link = 0 as libc::c_int;
                finalLink =
                    (*psCurr).zone1Links as libc::c_int +
                        (*psCurr).zone2Links as libc::c_int
            } else if (*psCurr).flags as libc::c_int &
                          GWR_ZONE1 as libc::c_int != 0 {
                // route came from zone1 continue with zone2
                zone = (*psCurr).zone2 as SDWORD;
                link = (*psCurr).zone1Links as SDWORD;
                finalLink =
                    (*psCurr).zone1Links as libc::c_int +
                        (*psCurr).zone2Links as libc::c_int
            } else {
                zone = (*psCurr).zone1 as SDWORD;
                link = 0 as libc::c_int;
                finalLink = (*psCurr).zone1Links as SDWORD
            }
            while link < finalLink {
                // skip any link that is known to be blocked
                if !((*(*psCurr).psLinks.offset(link as isize)).flags as
                         libc::c_int & GWRL_BLOCKED as libc::c_int != 0) {
                    psNew =
                        (*(*psCurr).psLinks.offset(link as isize)).psGateway;
                    currDist =
                        (*psCurr).dist as libc::c_int +
                            (*(*psCurr).psLinks.offset(link as isize)).dist as
                                libc::c_int;
                    // don't loop back on the route
                    if !(psNew == (*psCurr).psRoute ||
                             gwrConsiderGateway(psNew) == 0 ||
                             (*psNew).dist as libc::c_int <= currDist) {
                        // Now insert the gateway into the appropriate list
                        if (*psNew).flags as libc::c_int & 0x3f as libc::c_int
                               == 0 as libc::c_int {
                            // Not in open or closed lists - add to the open list
                            (*psNew).dist = currDist as SWORD;
                            (*psNew).est =
                                gwrEstimate(psNew, tileFX, tileFY) as SWORD;
                            (*psNew).psRoute = psCurr;
                            (*psNew).flags =
                                ((*psNew).flags as libc::c_int |
                                     if (*psNew).zone1 as libc::c_int == zone
                                        {
                                         GWR_ZONE1 as libc::c_int
                                     } else { GWR_ZONE2 as libc::c_int }) as
                                    UBYTE;
                            gwrOpenAdd(psNew);
                            if gwrDoMessage != 0 {
                                debug(LOG_ERROR,
                                      (*psNew).est as *const libc::c_char);
                            }
                        } else if (*psNew).flags as libc::c_int &
                                      GWR_OPEN as libc::c_int != 0 {
                            // already in the open list but this is shorter
                            if gwrDoMessage != 0 {
                                debug(LOG_ERROR,
                                      (*psNew).est as *const libc::c_char);
                            }
                            (*psNew).dist = currDist as SWORD;
                            (*psNew).psRoute = psCurr
                        } else if (*psNew).flags as libc::c_int &
                                      GWR_CLOSED as libc::c_int != 0 {
                            // already in the closed list but this is shorter
                            if gwrDoMessage != 0 {
                                debug(LOG_ERROR,
                                      (*psNew).est as *const libc::c_char);
                            }
                            (*psNew).dist = currDist as SWORD;
                            (*psNew).psRoute = psCurr;
                            gwrOpenAdd(psNew);
                        }
                        (*psNew).flags =
                            ((*psNew).flags as libc::c_int &
                                 !(GWR_ZONE1 as libc::c_int |
                                       GWR_ZONE2 as libc::c_int)) as UBYTE;
                        (*psNew).flags =
                            ((*psNew).flags as libc::c_int |
                                 if (*psNew).zone1 as libc::c_int == zone {
                                     GWR_ZONE1 as libc::c_int
                                 } else { GWR_ZONE2 as libc::c_int }) as UBYTE
                    }
                }
                link += 1 as libc::c_int
            }
            (*psCurr).flags =
                ((*psCurr).flags as libc::c_int &
                     !(GWR_CLOSED as libc::c_int | GWR_OPEN as libc::c_int))
                    as UBYTE;
            (*psCurr).flags =
                ((*psCurr).flags as libc::c_int | GWR_CLOSED as libc::c_int)
                    as UBYTE
        }
    }
    retval = GWR_OK as libc::c_int;
    if psRoute.is_null() {
        if gwrDoMessage != 0 {
            debug(LOG_ERROR,
                  b"Partial route\n\x00" as *const u8 as *const libc::c_char);
        }
        psRoute = psNearest;
        retval = GWR_NEAREST as libc::c_int
    }
    // get the route in the correct order if one was found
	// (it is currently in reverse order).
    if !psRoute.is_null() {
        psParent = 0 as *mut GATEWAY;
        psPrev = 0 as *mut GATEWAY;
        psCurr = psRoute;
        while !psCurr.is_null() {
            psNext = (*psCurr).psRoute;
            if (*psCurr).flags as libc::c_int & GWR_WATERLINK as libc::c_int
                   == 0 {
                (*psCurr).flags =
                    ((*psCurr).flags as libc::c_int |
                         GWR_INROUTE as libc::c_int) as UBYTE;
                link = 0 as libc::c_int;
                while link <
                          (*psCurr).zone1Links as libc::c_int +
                              (*psCurr).zone2Links as libc::c_int {
                    // NB link flags are reversed because the route order is being reversed
                    if (*(*psCurr).psLinks.offset(link as isize)).psGateway ==
                           psPrev {
                        let ref mut fresh1 =
                            (*(*psCurr).psLinks.offset(link as isize)).flags;
                        *fresh1 =
                            (*fresh1 as libc::c_int |
                                 GWRL_CHILD as libc::c_int) as SWORD
                    } else if (*(*psCurr).psLinks.offset(link as
                                                             isize)).psGateway
                                  == psNext {
                        let ref mut fresh2 =
                            (*(*psCurr).psLinks.offset(link as isize)).flags;
                        *fresh2 =
                            (*fresh2 as libc::c_int |
                                 GWRL_PARENT as libc::c_int) as SWORD
                    }
                    link += 1
                }
                (*psCurr).psRoute = psParent;
                psParent = psCurr
            }
            // NB psPrev is not quite the same as psParent as it includes water link gateways
            psPrev = psCurr;
            psCurr = psNext
        }
        psRoute = psParent;
        psCurr = psRoute;
        while !psCurr.is_null() { psCurr = (*psCurr).psRoute }
        *ppsRoute = psRoute
    }
    if psRoute.is_null() { retval = GWR_FAILED as libc::c_int }
    return retval;
}
