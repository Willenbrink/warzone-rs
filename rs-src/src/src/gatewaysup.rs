use ::libc;
extern "C" {
    #[no_mangle]
    fn memMallocRelease(Size: size_t) -> *mut libc::c_void;
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
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_uint)
     -> *mut libc::c_void;
    /* Flags for whether texture tiles are flipped in X and Y or rotated */
    // This bit describes the direction the tile is split into 2 triangles (same as triangleFlip)
    // set when the tile has the structure cursor over it
    // NASTY - this should be in tileInfoBits but there isn't any room left
    // units can drive on this even if there is a structure or feature on it
    //#define	BITS_TILE_HIGHLIGHT 0x8
    // show small structures - tank traps / bunkers
    // bit set temporarily by find path to mark a blocking tile
    // bit set to show a gateway on the tile
    // bit set to show a tall structure which camera needs to avoid.
    /*#ifndef PSX	// Extra tile info bits.... WIN32 only
#define	EXTRA_BITS_SENSOR	0x1
#define	EXTRA_BITS_2		0x2
#define	EXTRA_BITS_3		0x4
#define	EXTRA_BITS_4		0x8
#define	EXTRA_BITS_5		0x10
#define	EXTRA_BITS_6		0x20
#define	EXTRA_BITS_7		0x40
#define	EXTRA_BITS_8		0x80
#endif*/
    //#define TILE_HIGHLIGHT(x)		(x->tileInfoBits & BITS_TILE_HIGHLIGHT)
    /*
#ifndef PSX		// I've even set them up for you...:-)
#define TILE_IN_SENSORRANGE(x)	(x->tileExtraBits & EXTRA_BITS_SENSOR)
#define TILE_EXTRA_BIT2_SET(x)	(x->tileExtraBits & EXTRA_BITS_2)
#define TILE_EXTRA_BIT3_SET(x)	(x->tileExtraBits & EXTRA_BITS_3)
#define TILE_EXTRA_BIT4_SET(x)	(x->tileExtraBits & EXTRA_BITS_4)
#define TILE_EXTRA_BIT5_SET(x)	(x->tileExtraBits & EXTRA_BITS_5)
#define TILE_EXTRA_BIT6_SET(x)	(x->tileExtraBits & EXTRA_BITS_6)
#define TILE_EXTRA_BIT7_SET(x)	(x->tileExtraBits & EXTRA_BITS_7)
#define TILE_EXTRA_BIT8_SET(x)	(x->tileExtraBits & EXTRA_BITS_8)
#endif
*/
    /*
#ifndef PSX	// again, done for you again!
#define SET_TILE_SENSOR(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits | EXTRA_BITS_SENSOR))
#define CLEAR_TILE_SENSOR(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits & (~EXTRA_BITS_SENSOR)))
#define SET_TILE_BIT2(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits | EXTRA_BITS_2))
#define CLEAR_TILE_BIT2(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits & (~EXTRA_BITS_2)))
#define SET_TILE_BIT3(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits | EXTRA_BITS_3))
#define CLEAR_TILE_BIT3(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits & (~EXTRA_BITS_3)))
#define SET_TILE_BIT4(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits | EXTRA_BITS_4))
#define CLEAR_TILE_BIT4(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits & (~EXTRA_BITS_4)))
#define SET_TILE_BIT5(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits | EXTRA_BITS_5))
#define CLEAR_TILE_BIT5(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits & (~EXTRA_BITS_5)))
#define SET_TILE_BIT6(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits | EXTRA_BITS_6))
#define CLEAR_TILE_BIT6(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits & (~EXTRA_BITS_6)))
#define SET_TILE_BIT7(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits | EXTRA_BITS_7))
#define CLEAR_TILE_BIT7(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits & (~EXTRA_BITS_7)))
#define SET_TILE_BIT8(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits | EXTRA_BITS_8))
#define CLEAR_TILE_BIT8(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits & (~EXTRA_BITS_8)))
#endif
*/
// Multiplier for the tile height
    /* Allows us to do if(TRI_FLIPPED(psTile)) */
    /* Flips the triangle partition on a tile pointer */
    /* Can player number p see tile t? */
    /* Set a tile to be visible for a player */
    /*#ifndef PSX
#define SET_TILE_DOOR(p,t) t->tileDoorBits = (UBYTE) (t->tileDoorBits | (1<<p))
#define CLEAR_TILE_DOOR(p,t) t->tileDoorBits = (UBYTE) (t->tileDoorBits & (~(1<<p))) // check logic
// Is there a door here for the player?
#define TEST_TILE_DOOR(p,t) ( (t->tileDoorBits) & (1<<p) )
#endif*/
/* Arbitrary maximum number of terrain textures - used in look up table for terrain type */
    #[no_mangle]
    static mut terrainTypes: [UBYTE; 255];
    /* The maximum map size */
    /* The size and contents of the map */
    #[no_mangle]
    static mut mapWidth: UDWORD;
    #[no_mangle]
    static mut mapHeight: UDWORD;
    #[no_mangle]
    static mut psMapTiles: *mut MAPTILE;
    /*
 * Gateway.h
 *
 * Interface to routing gateway code.
 *
 */
    // the list of gateways on the current map
    #[no_mangle]
    static mut psGateways: *mut GATEWAY;
    // the RLE map zones for each tile
    #[no_mangle]
    static mut apRLEZones: *mut *mut UBYTE;
    // the number of map zones
    #[no_mangle]
    static mut gwNumZones: SDWORD;
    // The zone equivalence tables
    #[no_mangle]
    static mut aNumEquiv: *mut UBYTE;
    #[no_mangle]
    static mut apEquivZones: *mut *mut UBYTE;
    // Look up the zone for a coordinate
    #[no_mangle]
    fn gwGetZone(x: SDWORD, y: SDWORD) -> SDWORD;
    // release the RLE Zone map
    #[no_mangle]
    fn gwFreeZoneMap();
    // get the size of the map
    #[no_mangle]
    fn gwMapWidth() -> SDWORD;
    // set the gateway flag on a tile
    // clear the gateway flag on a tile
    // check whether a tile is water
    #[no_mangle]
    fn gwTileIsWater(x: UDWORD, y: UDWORD) -> BOOL;
    // Get number of gateways.
    // Get the gateway list.
    // Get number of zone lines.
    // Get size of a zone line in bytes.
    // create an empty equivalence table
    // release the equivalence table
    // set the zone equivalence for a zone
    #[no_mangle]
    fn gwSetZoneEquiv(zone: SDWORD, numEquiv: SDWORD, pEquiv: *mut UBYTE)
     -> BOOL;
    #[no_mangle]
    fn gwNewEquivTable(numZones: SDWORD) -> BOOL;
    #[no_mangle]
    fn gwMapHeight() -> SDWORD;
}
pub type size_t = libc::c_uint;
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
/*
 * Map.h
 *
 * Definitions for the map structure
 *
 */
// Visibility bits - can also be accessed as a byte (as a whole).
/* The different types of terrain as far as the game is concerned */
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
/* Information stored with each tile */
// The name is now changed to MAPTILE to allow correct compilation on the PlayStation
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
// sensor range display.
// This is also used to store the tile flip flags
//  What's been removed - 46 bytes per tile so far
//	BASE_OBJECT		*psObject;		// Any object sitting on the location (e.g. building)
//	UBYTE			onFire;			// Is tile on fire?
//	UBYTE			rippleIndex;	// Current value in ripple table?
//	BOOL			tileVisible[MAX_PLAYERS]; // Which players can see the tile?
//	BOOL			triangleFlip;	// Is the triangle flipped?
//	TYPE_OF_TERRAIN	type;			// The terrain type for the tile
/*
 * GatewayDef.h
 *
 * Structure definitions for routing gateways.
 *
 */
// flags for the gateway links
// the link is part of the current route - to the previous gateway
// the link is part of the current route - to the next gateway
// the route between the two zones is blocked
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
/*
 * Additional functions for the Gateway system.
 * Only needed for map preprocessing.
 *
 */
// segment printf's
//#define DEBUG_GROUP1
// stack printf's
//#define DEBUG_GROUP2
// gwProcessMap printf's
//#define DEBUG_GROUP3
// RLE zone map size
//#define DEBUG_GROUP4
// equivalence printf's
//#define DEBUG_GROUP5
// Structures and defines for SeedFill().
pub type Pixel = libc::c_int;
/* xmax and ymax (inclusive) */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Segment {
    pub y: libc::c_int,
    pub xl: libc::c_int,
    pub xr: libc::c_int,
    pub dy: libc::c_int,
}
/* Return a pointer to the tile structure at x,y */
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
    //return psMapTiles + x + (y << mapShift); //width no longer a power of 2
    return psMapTiles.offset(x as
                                 isize).offset(y.wrapping_mul(mapWidth) as
                                                   isize);
}
/* max depth of stack */
/* push new segment on stack */
/* pop segment off stack */
// whether the flood fill is running over water
#[no_mangle]
pub static mut bGwWaterFlood: BOOL = 0 as libc::c_int;
// disable this on the psx
#[no_mangle]
pub static mut stack: [Segment; 10000] =
    [Segment{y: 0, xl: 0, xr: 0, dy: 0,}; 10000];
#[no_mangle]
pub static mut sp: *mut Segment = unsafe { stack.as_ptr() as *mut _ };
/* stack of filled segments */
// Flood fill a map zone from a given point
// stopping at blocking tiles
/*
 * fill: set the pixel at (x,y) and all of its 4-connected neighbors
 * with the same pixel value to the new pixel value nv.
 * A 4-connected neighbor is a pixel above, below, left, or right of a pixel.
 */
#[no_mangle]
pub unsafe extern "C" fn gwSeedFill(mut x: SDWORD, mut y: SDWORD,
                                    mut nv: SDWORD) {
    let mut l: libc::c_int = 0; /* old pixel value */
    let mut x1: libc::c_int = 0; /* stack of filled segments */
    let mut x2: libc::c_int = 0; /* read pv at seed point */
    let mut dy: libc::c_int = 0;
    let mut ov: Pixel = 0;
    let mut stack_0: [Segment; 10000] =
        [Segment{y: 0, xl: 0, xr: 0, dy: 0,}; 10000];
    let mut sp_0: *mut Segment = stack_0.as_mut_ptr();
    ov = gwGetZone(x, y);
    if ov == nv { return }
    if sp_0 < stack_0.as_mut_ptr().offset(10000 as libc::c_int as isize) {
        (*sp_0).y = y;
        (*sp_0).xl = x;
        (*sp_0).xr = x;
        (*sp_0).dy = 1 as libc::c_int;
        sp_0 = sp_0.offset(1)
    }
    /* needed in some cases */
    if sp_0 < stack_0.as_mut_ptr().offset(10000 as libc::c_int as isize) {
        (*sp_0).y = y + 1 as libc::c_int;
        (*sp_0).xl = x;
        (*sp_0).xr = x;
        (*sp_0).dy = -(1 as libc::c_int);
        sp_0 = sp_0.offset(1)
    }
    /* seed segment (popped 1st) */
    while sp_0 > stack_0.as_mut_ptr() {
        let mut current_block_76: u64;
        /* pop segment off stack and fill a neighboring scan line */
        sp_0 = sp_0.offset(-1);
        dy = (*sp_0).dy;
        y = (*sp_0).y + dy;
        x1 = (*sp_0).xl;
        x2 = (*sp_0).xr;
        /*
		 * segment of scan line y-dy for x1<=x<=x2 was previously filled,
		 * now explore adjacent pixels in scan line y
		 */
        x = x1;
        while gwFloodBlock(x, y) == 0 && gwGetZone(x, y) == ov {
            gwSetZone(x, y, nv);
            x -= 1
        }
        if x >= x1 {
            current_block_76 = 14499356060253910203;
        } else {
            l = x + 1 as libc::c_int;
            if l < x1 {
                if sp_0 <
                       stack_0.as_mut_ptr().offset(10000 as libc::c_int as
                                                       isize) {
                    (*sp_0).y = y;
                    (*sp_0).xl = l;
                    (*sp_0).xr = x1 - 1 as libc::c_int;
                    (*sp_0).dy = -dy;
                    sp_0 = sp_0.offset(1)
                }
                /* leak on left? */
            }
            x = x1 + 1 as libc::c_int;
            current_block_76 = 10399321362245223758;
        }
        loop  {
            match current_block_76 {
                10399321362245223758 => {
                    while gwFloodBlock(x, y) == 0 && gwGetZone(x, y) == ov {
                        gwSetZone(x, y, nv);
                        x += 1
                    }
                    if sp_0 <
                           stack_0.as_mut_ptr().offset(10000 as libc::c_int as
                                                           isize) {
                        (*sp_0).y = y;
                        (*sp_0).xl = l;
                        (*sp_0).xr = x - 1 as libc::c_int;
                        (*sp_0).dy = dy;
                        sp_0 = sp_0.offset(1)
                    }
                    if x > x2 + 1 as libc::c_int {
                        if sp_0 <
                               stack_0.as_mut_ptr().offset(10000 as
                                                               libc::c_int as
                                                               isize) {
                            (*sp_0).y = y;
                            (*sp_0).xl = x2 + 1 as libc::c_int;
                            (*sp_0).xr = x - 1 as libc::c_int;
                            (*sp_0).dy = -dy;
                            sp_0 = sp_0.offset(1)
                        }
                        /* leak on right? */
                    }
                    current_block_76 = 14499356060253910203;
                }
                _ => {
                    x += 1;
                    while x <= x2 &&
                              (gwFloodBlock(x, y) != 0 ||
                                   gwGetZone(x, y) != ov) {
                        x += 1
                    }
                    l = x;
                    if x <= x2 {
                        current_block_76 = 10399321362245223758;
                    } else { break ; }
                }
            }
        }
    };
}
// set the tiles a gateway covers to a zone
#[no_mangle]
pub unsafe extern "C" fn gwSetGatewayZone(mut psGate: *mut GATEWAY,
                                          mut zone: SDWORD) {
    let mut pos: SDWORD = 0;
    if (*psGate).x1 as libc::c_int == (*psGate).x2 as libc::c_int {
        pos = (*psGate).y1 as SDWORD;
        while pos <= (*psGate).y2 as libc::c_int {
            gwSetZone((*psGate).x1 as SDWORD, pos, zone);
            pos += 1
        }
    } else {
        pos = (*psGate).x1 as SDWORD;
        while pos <= (*psGate).x2 as libc::c_int {
            gwSetZone(pos, (*psGate).y1 as SDWORD, zone);
            pos += 1
        }
    };
}
// find the first zone on a line
#[no_mangle]
pub unsafe extern "C" fn gwFindFirstZone(mut y: SDWORD) -> SDWORD {
    let mut x: SDWORD = 0;
    let mut zone: SDWORD = 0;
    zone = 0 as libc::c_int;
    while zone == 0 as libc::c_int && y < gwMapHeight() {
        x = 0 as libc::c_int;
        while x < gwMapWidth() && zone == 0 as libc::c_int {
            zone = gwGetZone(x, y);
            x += 1 as libc::c_int
        }
        y += 1
    }
    return zone;
}
// Process the map to create all the map zones
// Process the map to create all the map zones
#[no_mangle]
pub unsafe extern "C" fn gwProcessMap() -> BOOL {
    let mut psCurr: *mut GATEWAY = 0 as *mut GATEWAY;
    let mut currZone: SDWORD = 0;
    let mut zoneTest: SDWORD = 0;
    let mut floodX: SDWORD = 0;
    let mut floodY: SDWORD = 0;
    let mut gatewayZone: SDWORD = 0;
    let mut prevZone: SDWORD = 0;
    let mut nextZone: SDWORD = 0;
    let mut x: SDWORD = 0;
    let mut y: SDWORD = 0;
    // create the blank zone map
    if gwCreateBlankZoneMap() == 0 { return 0 as libc::c_int }
    // nothing more to do if there are no gateways
    if psGateways.is_null() { return 1 as libc::c_int }
    // reset the gateway zone entries
    psCurr = psGateways;
    while !psCurr.is_null() {
        (*psCurr).zone1 = 0 as libc::c_int as UBYTE;
        (*psCurr).zone2 = 0 as libc::c_int as UBYTE;
        psCurr = (*psCurr).psNext
    }
    // flood fill from all the gateways
    psCurr = psGateways;
    currZone = 1 as libc::c_int;
    loop  {
        // do a flood fill from the current gateway
        if (*psCurr).zone1 as libc::c_int == 0 as libc::c_int {
            // first zone is left/above
            if (*psCurr).x1 as libc::c_int == (*psCurr).x2 as libc::c_int {
                // vertical - go left
                floodX = (*psCurr).x1 as libc::c_int - 1 as libc::c_int;
                floodY =
                    ((*psCurr).y2 as libc::c_int -
                         (*psCurr).y1 as libc::c_int) / 2 as libc::c_int +
                        (*psCurr).y1 as libc::c_int
            } else {
                // horizontal - go above
                floodX =
                    ((*psCurr).x2 as libc::c_int -
                         (*psCurr).x1 as libc::c_int) / 2 as libc::c_int +
                        (*psCurr).x1 as libc::c_int;
                floodY = (*psCurr).y1 as libc::c_int - 1 as libc::c_int
            }
        } else if (*psCurr).x1 as libc::c_int == (*psCurr).x2 as libc::c_int {
            // psCurr->zone2 == 0
            // second zone is right/below
            // vertical - go right
            floodX = (*psCurr).x1 as libc::c_int + 1 as libc::c_int;
            floodY =
                ((*psCurr).y2 as libc::c_int - (*psCurr).y1 as libc::c_int) /
                    2 as libc::c_int + (*psCurr).y1 as libc::c_int
        } else {
            // horizontal - go below
            floodX =
                ((*psCurr).x2 as libc::c_int - (*psCurr).x1 as libc::c_int) /
                    2 as libc::c_int + (*psCurr).x1 as libc::c_int;
            floodY = (*psCurr).y1 as libc::c_int + 1 as libc::c_int
        }
        // see if a previous flood fill reached this gateway
        zoneTest = gwGetZone(floodX, floodY);
        if zoneTest != 0 as libc::c_int {
            // simple case just have to link the gateway to the zone
            gatewayZone = zoneTest
        } else {
            // check the zones havn't overflowed
            if currZone > 0xff as libc::c_int {
                debug(LOG_ERROR,
                      b"gwProcessMap: too many zones\n\x00" as *const u8 as
                          *const libc::c_char);
                abort();
            }
            // floodFill
            if gwTileIsWater(floodX as UDWORD, floodY as UDWORD) != 0 {
                bGwWaterFlood = 1 as libc::c_int
            }
            gwSeedFill(floodX, floodY, currZone);
            bGwWaterFlood = 0 as libc::c_int;
            gatewayZone = currZone;
            currZone += 1 as libc::c_int
        }
        // set the gateway zone
        if (*psCurr).zone1 as libc::c_int == 0 as libc::c_int {
            (*psCurr).zone1 = gatewayZone as UBYTE;
            // the gateway is always in it's own zone1
            gwSetGatewayZone(psCurr, gatewayZone);
        } else { (*psCurr).zone2 = gatewayZone as UBYTE }
        // see if there are any gateways left to process
        psCurr = psGateways;
        while !psCurr.is_null() {
            if (*psCurr).zone1 as libc::c_int == 0 as libc::c_int ||
                   (*psCurr).zone2 as libc::c_int == 0 as libc::c_int {
                break ;
            }
            psCurr = (*psCurr).psNext
        }
        if psCurr.is_null() { break ; }
    }
    // fill in any areas that are left
    y = 0 as libc::c_int;
    while y < gwMapHeight() {
        x = 0 as libc::c_int;
        while x < gwMapWidth() {
            if gwTileIsWater(x as UDWORD, y as UDWORD) != 0 &&
                   gwGetZone(x, y) == 0 as libc::c_int {
                // check the zones havn't overflowed
                if currZone > 0xff as libc::c_int {
                    debug(LOG_ERROR,
                          b"gwProcessMap: too many zones\n\x00" as *const u8
                              as *const libc::c_char);
                    abort();
                }
                bGwWaterFlood = 1 as libc::c_int;
                gwSeedFill(x, y, currZone);
                bGwWaterFlood = 0 as libc::c_int;
                currZone += 1 as libc::c_int
            } else if gwFloodBlock(x, y) == 0 &&
                          gwGetZone(x, y) == 0 as libc::c_int {
                // check the zones havn't overflowed
                if currZone > 0xff as libc::c_int {
                    debug(LOG_ERROR,
                          b"gwProcessMap: too many zones\n\x00" as *const u8
                              as *const libc::c_char);
                    abort();
                }
                gwSeedFill(x, y, currZone);
                currZone += 1 as libc::c_int
            }
            x += 1 as libc::c_int
        }
        y += 1 as libc::c_int
    }
    // now average out the zones so that blocking tiles are in the same zone as their neighbour
    y = 0 as libc::c_int;
    while y < gwMapHeight() {
        prevZone = gwFindFirstZone(y);
        x = 0 as libc::c_int;
        while x < gwMapWidth() {
            nextZone = gwGetZone(x, y);
            if gwFloodBlock(x, y) != 0 && nextZone == 0 as libc::c_int {
                gwSetZone(x, y, prevZone);
            } else if nextZone != 0 as libc::c_int { prevZone = nextZone }
            x += 1 as libc::c_int
        }
        y += 1 as libc::c_int
    }
    if gwGenerateZoneEquiv(currZone) == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
// see if a zone is already in the equivalence array
unsafe extern "C" fn gwEquivZonePresent(mut aEquiv: *mut UBYTE,
                                        mut numEquiv: SDWORD,
                                        mut zone: SDWORD) -> BOOL {
    let mut i: SDWORD = 0;
    i = 0 as libc::c_int;
    while i < numEquiv {
        if *aEquiv.offset(i as isize) as libc::c_int == zone {
            return 1 as libc::c_int
        }
        i += 1
    }
    return 0 as libc::c_int;
}
// check a neighbour tile for equivalence
unsafe extern "C" fn gwCheckNeighbourEquiv(mut zone: SDWORD, mut x: SDWORD,
                                           mut y: SDWORD) {
    let mut nZone: SDWORD = 0;
    nZone = gwGetZone(x, y);
    if nZone != zone && gwFloodBlock(x, y) == 0 &&
           gwEquivZonePresent(*apEquivZones.offset(zone as isize),
                              *aNumEquiv.offset(zone as isize) as SDWORD,
                              nZone) == 0 {
        *(*apEquivZones.offset(zone as
                                   isize)).offset(*aNumEquiv.offset(zone as
                                                                        isize)
                                                      as isize) =
            nZone as UBYTE;
        let ref mut fresh0 = *aNumEquiv.offset(zone as isize);
        *fresh0 = (*fresh0 as libc::c_int + 1 as libc::c_int) as UBYTE
    };
}
// generate the zone equivalence tables
// generate the zone equivalence tables
#[no_mangle]
pub unsafe extern "C" fn gwGenerateZoneEquiv(mut numZones: SDWORD) -> BOOL {
    let mut x: SDWORD = 0;
    let mut y: SDWORD = 0;
    let mut i: SDWORD = 0;
    let mut aEquiv: [UBYTE; 255] = [0; 255];
    let mut numEquiv: SDWORD = 0;
    let mut localZone: SDWORD = 0;
    if apEquivZones.is_null() {
        if gwNewEquivTable(numZones) == 0 { return 0 as libc::c_int }
    }
    // just allocate the maximum space when generating the table
    memset(aEquiv.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<[UBYTE; 255]>() as libc::c_ulong);
    i = 0 as libc::c_int;
    while i < gwNumZones {
        if gwSetZoneEquiv(i, gwNumZones, aEquiv.as_mut_ptr()) == 0 {
            return 0 as libc::c_int
        }
        *aNumEquiv.offset(i as isize) = 0 as libc::c_int as UBYTE;
        i += 1 as libc::c_int
    }
    // go over the map - skipping edge tiles to avoid going over the
	// edge of the map
    numEquiv = 0 as libc::c_int;
    y = 1 as libc::c_int;
    while y < gwMapHeight() - 1 as libc::c_int {
        x = 1 as libc::c_int;
        while x < gwMapWidth() - 1 as libc::c_int {
            if gwTileIsWater(x as UDWORD, y as UDWORD) != 0 {
                // found a water tile - see if it is next to a different zone
                localZone = gwGetZone(x, y);
                gwCheckNeighbourEquiv(localZone, x - 1 as libc::c_int, y);
                gwCheckNeighbourEquiv(localZone, x, y - 1 as libc::c_int);
                gwCheckNeighbourEquiv(localZone, x + 1 as libc::c_int, y);
                gwCheckNeighbourEquiv(localZone, x, y + 1 as libc::c_int);
            }
            x += 1 as libc::c_int
        }
        y += 1 as libc::c_int
    }
    return 1 as libc::c_int;
}
// Create a new blank RLE Zone map suitable for creating zones in
/* *****************************************************************************************************/
/*                            RLE Zone data access functions                                          */
// Create a new blank RLE Zone map suitable for creating zones in
#[no_mangle]
pub unsafe extern "C" fn gwCreateBlankZoneMap() -> BOOL {
    let mut i: SDWORD = 0;
    if !apRLEZones.is_null() { gwFreeZoneMap(); }
    apRLEZones =
        memMallocRelease((::std::mem::size_of::<*mut UBYTE>() as
                              libc::c_ulong).wrapping_mul(gwMapHeight() as
                                                              libc::c_uint))
            as *mut *mut UBYTE;
    if apRLEZones.is_null() {
        debug(LOG_ERROR,
              b"gwCreateBlankZoneMap: Out of memory\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    i = 0 as libc::c_int;
    while i < gwMapHeight() {
        let ref mut fresh1 = *apRLEZones.offset(i as isize);
        *fresh1 =
            memMallocRelease((gwMapWidth() * 2 as libc::c_int) as size_t) as
                *mut UBYTE;
        if (*apRLEZones.offset(i as isize)).is_null() {
            debug(LOG_ERROR,
                  b"gwCreateBlankZoneMap: Out of memory\x00" as *const u8 as
                      *const libc::c_char);
            abort();
        }
        i += 1
    }
    // set all the zones to zero
    i = 0 as libc::c_int;
    while i < gwMapHeight() {
        **apRLEZones.offset(i as isize) = gwMapWidth() as UBYTE;
        *(*apRLEZones.offset(i as isize)).offset(1 as libc::c_int as isize) =
            0 as libc::c_int as UBYTE;
        i += 1
    }
    return 1 as libc::c_int;
}
// Decompress a line of the zone map
// pBuffer should point to a buffer of gwMapWidth() bytes
#[no_mangle]
pub unsafe extern "C" fn gwDecompressLine(mut line: SDWORD,
                                          mut pBuffer: *mut UBYTE) {
    let mut rlePos: SDWORD = 0;
    let mut bufPos: SDWORD = 0;
    let mut count: SDWORD = 0;
    let mut zone: SDWORD = 0;
    let mut store: SDWORD = 0;
    rlePos = 0 as libc::c_int;
    bufPos = 0 as libc::c_int;
    while bufPos < gwMapWidth() {
        count =
            *(*apRLEZones.offset(line as isize)).offset(rlePos as isize) as
                SDWORD;
        zone =
            *(*apRLEZones.offset(line as
                                     isize)).offset(rlePos as
                                                        isize).offset(1 as
                                                                          libc::c_int
                                                                          as
                                                                          isize)
                as SDWORD;
        rlePos += 2 as libc::c_int;
        store = 0 as libc::c_int;
        while store < count {
            if bufPos < gwMapWidth() {
            } else {
                debug(LOG_ERROR,
                      b"gwDecompressLine: Invalid RLE code\x00" as *const u8
                          as *const libc::c_char);
            };
            if bufPos < gwMapWidth() {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"gatewaysup.c\x00" as *const u8 as *const libc::c_char,
                      556 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 17],
                                                &[libc::c_char; 17]>(b"gwDecompressLine\x00")).as_ptr(),
                      b"bufPos < gwMapWidth()\x00" as *const u8 as
                          *const libc::c_char);
            };
            *pBuffer.offset(bufPos as isize) = zone as UBYTE;
            bufPos += 1 as libc::c_int;
            store += 1
        }
    };
}
// Compress a line of the zone map
// pBuffer should point to a buffer of gwMapWidth() bytes
#[no_mangle]
pub unsafe extern "C" fn gwCompressLine(mut line: SDWORD,
                                        mut pBuffer: *mut UBYTE) {
    let mut rlePos: SDWORD = 0;
    let mut bufPos: SDWORD = 0;
    let mut count: SDWORD = 0;
    let mut zone: SDWORD = 0;
    rlePos = 0 as libc::c_int;
    bufPos = 0 as libc::c_int;
    while bufPos < gwMapWidth() {
        zone = *pBuffer.offset(bufPos as isize) as SDWORD;
        count = 0 as libc::c_int;
        while *pBuffer.offset(bufPos as isize) as libc::c_int == zone &&
                  bufPos < gwMapWidth() {
            count += 1 as libc::c_int;
            bufPos += 1 as libc::c_int
        }
        *(*apRLEZones.offset(line as isize)).offset(rlePos as isize) =
            count as UBYTE;
        *(*apRLEZones.offset(line as
                                 isize)).offset(rlePos as
                                                    isize).offset(1 as
                                                                      libc::c_int
                                                                      as
                                                                      isize) =
            zone as UBYTE;
        rlePos += 2 as libc::c_int
    };
}
// Set the zone for a coordinate
// Set the zone for a coordinate
#[no_mangle]
pub unsafe extern "C" fn gwSetZone(mut x: SDWORD, mut y: SDWORD,
                                   mut zone: SDWORD) {
    let mut aBuffer: [UBYTE; 255] = [0; 255];
    if x >= 0 as libc::c_int && x < gwMapWidth() && y >= 0 as libc::c_int &&
           y < gwMapHeight() {
    } else {
        debug(LOG_ERROR,
              b"gwSetZone: invalid coordinates\x00" as *const u8 as
                  *const libc::c_char);
    };
    if x >= 0 as libc::c_int && x < gwMapWidth() && y >= 0 as libc::c_int &&
           y < gwMapHeight() {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"gatewaysup.c\x00" as *const u8 as *const libc::c_char,
              596 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 10],
                                        &[libc::c_char; 10]>(b"gwSetZone\x00")).as_ptr(),
              b"(x >= 0) && (x < gwMapWidth()) && (y >= 0) && (y < gwMapHeight())\x00"
                  as *const u8 as *const libc::c_char);
    };
    gwDecompressLine(y, aBuffer.as_mut_ptr());
    aBuffer[x as usize] = zone as UBYTE;
    gwCompressLine(y, aBuffer.as_mut_ptr());
}
// check for a blocking tile for the flood fill
/* *****************************************************************************************************/
/*                   Gateway data access functions                                                    */
// check for a blocking tile for the flood fill
#[no_mangle]
pub unsafe extern "C" fn gwFloodBlock(mut x: SDWORD, mut y: SDWORD) -> BOOL {
    let mut psTile: *mut MAPTILE = 0 as *mut MAPTILE;
    let mut type_0: SDWORD = 0;
    let mut gateway: BOOL = 0;
    if x < 0 as libc::c_int || x >= gwMapWidth() || y < 0 as libc::c_int ||
           y >= gwMapHeight() {
        return 1 as libc::c_int
    }
    psTile = mapTile(x as UDWORD, y as UDWORD);
    type_0 =
        terrainTypes[((*psTile).texture as libc::c_int & 0x1ff as libc::c_int)
                         as usize] as SDWORD;
    gateway =
        ((*psTile).tileInfoBits as libc::c_int & 0x40 as libc::c_int !=
             0 as libc::c_int) as libc::c_int;
    return (gateway != 0 ||
                bGwWaterFlood == 0 &&
                    (type_0 == TER_CLIFFFACE as libc::c_int ||
                         type_0 == TER_WATER as libc::c_int) ||
                bGwWaterFlood != 0 && type_0 != TER_WATER as libc::c_int) as
               libc::c_int;
}
