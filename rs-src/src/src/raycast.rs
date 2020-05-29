use ::libc;
extern "C" {
    /* Arc tangent of Y/X.  */
    /* Arc tangent of Y/X.  */
    /* Arc tangent of Y/X.  */
    /* Arc tangent of Y/X.  */
    /* Arc tangent of Y/X.  */
    /* Arc tangent of Y/X.  */
    /* Arc tangent of Y/X.  */
    #[no_mangle]
    fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn cos(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn sin(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn tan(_: libc::c_double) -> libc::c_double;
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
    /* The maximum map size */
    /* The size and contents of the map */
    #[no_mangle]
    static mut mapWidth: UDWORD;
    #[no_mangle]
    static mut mapHeight: UDWORD;
    #[no_mangle]
    static mut psMapTiles: *mut MAPTILE;
    /* Return height of x,y */
//extern SDWORD map_Height(UDWORD x, UDWORD y);
    #[no_mangle]
    fn map_Height(x: UDWORD, y: UDWORD) -> SWORD;
    #[no_mangle]
    fn clipXY(x: SDWORD, y: SDWORD) -> BOOL;
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
pub type FRACT = libc::c_float;
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
/* Information stored with each tile */
// The name is now changed to MAPTILE to allow correct compilation on the PlayStation
pub type MAPTILE = _maptile;
/*#ifndef PSX
	UBYTE			tileExtraBits;	// We've got more than you... We've got more than you..;-)
#endif*/
// COMPRESSED - bit per player
/*#ifndef PSX
	UBYTE			tileDoorBits;   // same thing - bit per player
#endif*/
// The height at the top left of the tile
// How bright is this tile?
// Which graphics texture is on this tile
// sensor range display.
// This is also used to store the tile flip flags
//  What's been removed - 46 bytes per tile so far
//	BASE_OBJECT		*psObject;		// Any object sitting on the location (e.g. building)
//	UBYTE			onFire;			// Is tile on fire?
//	UBYTE			rippleIndex;	// Current value in ripple table?
//	BOOL			tileVisible[MAX_PLAYERS]; // Which players can see the tile?
//	BOOL			triangleFlip;	// Is the triangle flipped?
//	TYPE_OF_TERRAIN	type;			// The terrain type for the tile
/* The raycast intersection callback.
 * Return FALSE if no more points are required, TRUE otherwise
 */
pub type RAY_CALLBACK
    =
    Option<unsafe extern "C" fn(_: SDWORD, _: SDWORD, _: SDWORD) -> BOOL>;
// Get the tile from tile coords
// control the type of clip method
// 0 - clip on ray length (faster but it doesn't always work :-)
// 1 - clip on coordinates (accurate but possibly a bit slower)
// ray point
pub type RAY_POINT = _ray_point;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ray_point {
    pub x: SDWORD,
    pub y: SDWORD,
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
/* x and y increments for each ray angle */
static mut rayDX: [SDWORD; 360] = [0; 360];
static mut rayDY: [SDWORD; 360] = [0; 360];
static mut rayHDist: [SDWORD; 360] = [0; 360];
static mut rayVDist: [SDWORD; 360] = [0; 360];
//static FRACT	rayTan[NUM_RAYS], rayCos[NUM_RAYS], raySin[NUM_RAYS];
static mut rayFPTan: [SDWORD; 360] = [0; 360];
static mut rayFPInvTan: [SDWORD; 360] = [0; 360];
static mut rayFPInvCos: [SDWORD; 360] = [0; 360];
static mut rayFPInvSin: [SDWORD; 360] = [0; 360];
// maximum length for a visiblity ray
/* Initialise the visibility rays */
/* Initialise the ray tables */
#[no_mangle]
pub unsafe extern "C" fn rayInitialise() -> BOOL {
    let mut i: SDWORD = 0;
    let mut angle: FRACT = 0 as libc::c_int as FRACT;
    let mut val: FRACT = 0.;
    i = 0 as libc::c_int;
    while i < 360 as libc::c_int {
        // Set up the fixed offset tables for calculating the intersection points
        val = tan(angle as libc::c_double) as libc::c_float;
        rayDX[i as usize] =
            ((128 as libc::c_int * ((1 as libc::c_int) << 12 as libc::c_int))
                 as libc::c_float * val) as SDWORD;
        if i <= 360 as libc::c_int / 4 as libc::c_int ||
               i >= 3 as libc::c_int * 360 as libc::c_int / 4 as libc::c_int {
            rayDX[i as usize] = -rayDX[i as usize]
        }
        if val == 0 as libc::c_int as libc::c_float {
            val = 1 as libc::c_int as FRACT
            // Horrible hack to avoid divide by zero.
        }
        rayDY[i as usize] =
            ((128 as libc::c_int * ((1 as libc::c_int) << 12 as libc::c_int))
                 as libc::c_float / val) as SDWORD;
        if i >= 360 as libc::c_int / 2 as libc::c_int {
            rayDY[i as usize] = -rayDY[i as usize]
        }
        // These are used to calculate the initial intersection
        rayFPTan[i as usize] =
            (val * ((1 as libc::c_int) << 12 as libc::c_int) as FRACT) as
                SDWORD;
        rayFPInvTan[i as usize] =
            (((1 as libc::c_int) << 12 as libc::c_int) as FRACT / val) as
                SDWORD;
        // Set up the trig tables for calculating the offset distances
        val = sin(angle as libc::c_double) as libc::c_float;
        if val == 0 as libc::c_int as libc::c_float {
            val = 1 as libc::c_int as FRACT
        }
        rayFPInvSin[i as usize] =
            (((1 as libc::c_int) << 12 as libc::c_int) as FRACT / val) as
                SDWORD;
        if i >= 360 as libc::c_int / 2 as libc::c_int {
            rayVDist[i as usize] =
                (-(128 as libc::c_int) as FRACT / val) as SDWORD
        } else {
            rayVDist[i as usize] =
                (128 as libc::c_int as FRACT / val) as SDWORD
        }
        val = cos(angle as libc::c_double) as libc::c_float;
        if val == 0 as libc::c_int as libc::c_float {
            val = 1 as libc::c_int as FRACT
        }
        rayFPInvCos[i as usize] =
            (((1 as libc::c_int) << 12 as libc::c_int) as FRACT / val) as
                SDWORD;
        if i < 360 as libc::c_int / 4 as libc::c_int ||
               i > 3 as libc::c_int * 360 as libc::c_int / 4 as libc::c_int {
            rayHDist[i as usize] =
                (128 as libc::c_int as FRACT / val) as SDWORD
        } else {
            rayHDist[i as usize] =
                (-(128 as libc::c_int) as FRACT / val) as SDWORD
        }
        angle +=
            (2 as libc::c_int as libc::c_double * 3.141592654f64 /
                 360 as libc::c_int as libc::c_double) as libc::c_float;
        i += 1
    }
    return 1 as libc::c_int;
}
/* cast a ray from x,y (world coords) at angle ray (0-NUM_RAYS) */
//void rayC(UDWORD x, UDWORD y, UDWORD ray, UDWORD length, RAY_CALLBACK callback);
//
// //#ifndef PSX
//
//void rayCast(UDWORD x, UDWORD y, UDWORD ray, UDWORD length, RAY_CALLBACK callback)
//{
//	rayC(x, y, ray, length, callback);
//}
//#else
//
//void rayCast(UDWORD x, UDWORD y, UDWORD ray, UDWORD length, RAY_CALLBACK callback)
//{
//	static UDWORD Tx;
//	static UDWORD Ty;
//	static UDWORD Tray;
//	static UDWORD Tlength;
//	static RAY_CALLBACK Tcallback;
//
//	Tx = x;
//	Ty = y;
//	Tray = ray;
//	Tlength = length;
//	Tcallback = callback;
//	// Stack in the DCache.
//	SetSpDCache();
//	rayC(Tx, Ty, Tray, Tlength, Tcallback);
//	SetSpNormal();
//}
//
//#endif
/* cast a ray from x,y (world coords) at angle ray (0-360)
 * The ray angle starts at zero along the positive y axis and
 * increases towards -ve X.
 *
 * Sorry about the wacky angle set up but that was what I thought
 * warzone used, but turned out not to be after I wrote it.
 */
#[no_mangle]
pub unsafe extern "C" fn rayCast(mut x: UDWORD, mut y: UDWORD,
                                 mut ray: UDWORD, mut length: UDWORD,
                                 mut callback: RAY_CALLBACK) {
    let mut hdInc: SDWORD =
        0 as libc::c_int; // increases in x and y distance per intersection
    let mut vdInc: SDWORD =
        0 as
            libc::c_int; // distance to current horizontal and vertical intersectionse
    let mut hDist: SDWORD = 0; // vertical x increment, horiz y inc
    let mut vDist: SDWORD = 0;
    let mut sVert: RAY_POINT =
        {
            let mut init =
                _ray_point{x: 0 as libc::c_int, y: 0 as libc::c_int,};
            init
        };
    let mut sHoriz: RAY_POINT =
        {
            let mut init =
                _ray_point{x: 0 as libc::c_int, y: 0 as libc::c_int,};
            init
        };
    let mut vdx: SDWORD = 0 as libc::c_int;
    let mut hdy: SDWORD = 0 as libc::c_int;
    // Clipping is done with the position offset by TILE_UNITS/4 to account
	// for the rounding errors when the intersection length is calculated.
	// Bit of a hack but I'm pretty sure it doesn't let through anything
	// that should be clippped.
    // initialise the horizontal intersection calculations
	// and clip to the top and bottom of the map
	// (no horizontal intersection for a horizontal ray)
    if ray != (360 as libc::c_int / 4 as libc::c_int) as libc::c_uint &&
           ray !=
               (3 as libc::c_int * 360 as libc::c_int / 4 as libc::c_int) as
                   libc::c_uint {
        if ray < (360 as libc::c_int / 4 as libc::c_int) as libc::c_uint ||
               ray >
                   (3 as libc::c_int * 360 as libc::c_int / 4 as libc::c_int)
                       as libc::c_uint {
            // intersection
            sHoriz.y =
                (y &
                     !(0x7f as libc::c_int) as
                         libc::c_uint).wrapping_add(128 as libc::c_int as
                                                        libc::c_uint) as
                    SDWORD;
            hdy = 128 as libc::c_int
        } else {
            // intersection
            sHoriz.y =
                (y &
                     !(0x7f as libc::c_int) as
                         libc::c_uint).wrapping_sub(1 as libc::c_int as
                                                        libc::c_uint) as
                    SDWORD;
            hdy = -(128 as libc::c_int)
        }
        // Horizontal x is kept in fixed point form until passed to the callback
		// to avoid rounding errors
		// Horizontal y is in integer form all the time
        sHoriz.x =
            (x <<
                 12 as
                     libc::c_int).wrapping_add(((y as SDWORD - sHoriz.y) *
                                                    rayFPTan[ray as usize]) as
                                                   libc::c_uint) as SDWORD;
        // Set up the distance calculations
        hDist =
            (sHoriz.y - y as SDWORD) * rayFPInvCos[ray as usize] >>
                12 as libc::c_int;
        hdInc = rayHDist[ray as usize]
    } else {
        // ensure no horizontal intersections are calculated
        hDist = length as SDWORD
    }
    // initialise the vertical intersection calculations
	// and clip to the left and right of the map
	// (no vertical intersection for a vertical ray)
    if ray != 0 as libc::c_int as libc::c_uint &&
           ray != (360 as libc::c_int / 2 as libc::c_int) as libc::c_uint {
        if ray >= (360 as libc::c_int / 2 as libc::c_int) as libc::c_uint {
            // intersection
            sVert.x =
                (x &
                     !(0x7f as libc::c_int) as
                         libc::c_uint).wrapping_add(128 as libc::c_int as
                                                        libc::c_uint) as
                    SDWORD;
            vdx = 128 as libc::c_int
        } else {
            // intersection
            sVert.x =
                (x &
                     !(0x7f as libc::c_int) as
                         libc::c_uint).wrapping_sub(1 as libc::c_int as
                                                        libc::c_uint) as
                    SDWORD;
            vdx = -(128 as libc::c_int)
        }
        // Vertical y is kept in fixed point form until passed to the callback
		// to avoid rounding errors
		// Vertical x is in integer form all the time
        sVert.y =
            (y <<
                 12 as
                     libc::c_int).wrapping_add(((x as SDWORD - sVert.x) *
                                                    rayFPInvTan[ray as usize])
                                                   as libc::c_uint) as SDWORD;
        // Set up the distance calculations
        vDist =
            (x as SDWORD - sVert.x) * rayFPInvSin[ray as usize] >>
                12 as libc::c_int;
        vdInc = rayVDist[ray as usize]
    } else {
        // ensure no vertical intersections are calculated
        vDist = length as SDWORD
    }
    if hDist != 0 as libc::c_int && vDist != 0 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"rayCast: zero distance\x00" as *const u8 as
                  *const libc::c_char);
    };
    if hDist != 0 as libc::c_int && vDist != 0 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"raycast.c\x00" as *const u8 as *const libc::c_char,
              289 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 8],
                                        &[libc::c_char; 8]>(b"rayCast\x00")).as_ptr(),
              b"hDist != 0 && vDist != 0\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (hDist == length as SDWORD || hdInc > 0 as libc::c_int) &&
           (vDist == length as SDWORD || vdInc > 0 as libc::c_int) {
    } else {
        debug(LOG_ERROR,
              b"rayCast: negative (or 0) distance increment\x00" as *const u8
                  as *const libc::c_char);
    };
    if (hDist == length as SDWORD || hdInc > 0 as libc::c_int) &&
           (vDist == length as SDWORD || vdInc > 0 as libc::c_int) {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"raycast.c\x00" as *const u8 as *const libc::c_char,
              292 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 8],
                                        &[libc::c_char; 8]>(b"rayCast\x00")).as_ptr(),
              b"(hDist == (SDWORD)length || hdInc > 0) && (vDist == (SDWORD)length || vdInc > 0)\x00"
                  as *const u8 as *const libc::c_char);
    };
    while hDist < length as SDWORD || vDist < length as SDWORD {
        // choose the next closest intersection
        if hDist < vDist {
            // clip to the edge of the map
            if sHoriz.x < 0 as libc::c_int ||
                   sHoriz.x >> 12 as libc::c_int >=
                       (mapWidth << 7 as libc::c_int) as SDWORD ||
                   sHoriz.y < 0 as libc::c_int ||
                   sHoriz.y >= (mapHeight << 7 as libc::c_int) as SDWORD {
                return
            }
            // pass through the current intersection, converting x from fixed point
            if callback.expect("non-null function pointer")(sHoriz.x >>
                                                                12 as
                                                                    libc::c_int,
                                                            sHoriz.y, hDist)
                   == 0 {
                // callback doesn't want any more points so return
                return
            }
            // update for the next intersection
            sHoriz.x += rayDX[ray as usize];
            sHoriz.y += hdy;
            hDist += hdInc
        } else {
            // clip to the edge of the map
            if sVert.x < 0 as libc::c_int ||
                   sVert.x >= (mapWidth << 7 as libc::c_int) as SDWORD ||
                   sVert.y < 0 as libc::c_int ||
                   sVert.y >> 12 as libc::c_int >=
                       (mapHeight << 7 as libc::c_int) as SDWORD {
                return
            }
            // pass through the current intersection, converting y from fixed point
            if callback.expect("non-null function pointer")(sVert.x,
                                                            sVert.y >>
                                                                12 as
                                                                    libc::c_int,
                                                            vDist) == 0 {
                // callback doesn't want any more points so return
                return
            }
            // update for the next intersection
            sVert.x += vdx;
            sVert.y += rayDY[ray as usize];
            vDist += vdInc
        }
        if hDist != 0 as libc::c_int && vDist != 0 as libc::c_int {
        } else {
            debug(LOG_ERROR,
                  b"rayCast: zero distance\x00" as *const u8 as
                      *const libc::c_char);
        };
        if hDist != 0 as libc::c_int && vDist != 0 as libc::c_int {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"raycast.c\x00" as *const u8 as *const libc::c_char,
                  378 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 8],
                                            &[libc::c_char; 8]>(b"rayCast\x00")).as_ptr(),
                  b"hDist != 0 && vDist != 0\x00" as *const u8 as
                      *const libc::c_char);
        };
    };
}
// Calculate the angle to cast a ray between two points
// Calculate the angle to cast a ray between two points
#[no_mangle]
pub unsafe extern "C" fn rayPointsToAngle(mut x1: SDWORD, mut y1: SDWORD,
                                          mut x2: SDWORD, mut y2: SDWORD)
 -> UDWORD {
    let mut xdiff: SDWORD = 0;
    let mut ydiff: SDWORD = 0;
    let mut angle: SDWORD = 0;
    xdiff = x2 - x1;
    ydiff = y1 - y2;
    angle =
        ((360 as libc::c_int / 2 as libc::c_int) as libc::c_double *
             atan2(xdiff as libc::c_double, ydiff as libc::c_double) /
             3.141592654f64) as SDWORD;
    angle += 360 as libc::c_int / 2 as libc::c_int;
    angle = angle % 360 as libc::c_int;
    if angle >= 0 as libc::c_int && angle < 360 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"rayPointsToAngle: angle out of range\x00" as *const u8 as
                  *const libc::c_char);
    };
    if angle >= 0 as libc::c_int && angle < 360 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"raycast.c\x00" as *const u8 as *const libc::c_char,
              401 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 17],
                                        &[libc::c_char; 17]>(b"rayPointsToAngle\x00")).as_ptr(),
              b"angle >= 0 && angle < NUM_RAYS\x00" as *const u8 as
                  *const libc::c_char);
    };
    return angle as UDWORD;
}
/* Distance of a point from a line.
 * NOTE: This is not 100% accurate - it approximates to get the square root
 *
 * This is based on Graphics Gems II setion 1.3
 */
/* Distance of a point from a line.
 * NOTE: This is not 100% accurate - it approximates to get the square root
 *
 * This is based on Graphics Gems II setion 1.3
 */
#[no_mangle]
pub unsafe extern "C" fn rayPointDist(mut x1: SDWORD, mut y1: SDWORD,
                                      mut x2: SDWORD, mut y2: SDWORD,
                                      mut px: SDWORD, mut py: SDWORD)
 -> SDWORD {
    let mut a: SDWORD = 0;
    let mut lxd: SDWORD = 0;
    let mut lyd: SDWORD = 0;
    let mut dist: SDWORD = 0;
    lxd = x2 - x1;
    lyd = y2 - y1;
    a = (py - y1) * lxd - (px - x1) * lyd;
    if a < 0 as libc::c_int { a = -a }
    if lxd < 0 as libc::c_int { lxd = -lxd }
    if lyd < 0 as libc::c_int { lyd = -lyd }
    if lxd < lyd {
        dist = a / (lxd + lyd - lxd / 2 as libc::c_int)
    } else { dist = a / (lxd + lyd - lyd / 2 as libc::c_int) }
    return dist;
}
//-----------------------------------------------------------------------------------
/*	Gets the maximum terrain height along a certain direction to the edge of the grid
	from wherever you specify, as well as the distance away
*/
// typedef BOOL (*RAY_CALLBACK)(SDWORD x, SDWORD y, SDWORD dist);
//void rayCast(UDWORD x, UDWORD y, UDWORD ray, UDWORD length, RAY_CALLBACK callback)
//#define TEST_RAY
/* Nasty global vars - put into a structure? */
//-----------------------------------------------------------------------------------
#[no_mangle]
pub static mut gHeight: SDWORD = 0;
#[no_mangle]
pub static mut gPitch: FRACT = 0.;
#[no_mangle]
pub static mut gStartTileX: UDWORD = 0;
#[no_mangle]
pub static mut gStartTileY: UDWORD = 0;
#[no_mangle]
pub static mut gHighestHeight: SDWORD = 0;
#[no_mangle]
pub static mut gHOrigHeight: SDWORD = 0;
#[no_mangle]
pub static mut gHMinDist: SDWORD = 0;
#[no_mangle]
pub static mut gHPitch: FRACT = 0.;
//-----------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn getTileTallObj(mut x: UDWORD, mut y: UDWORD)
 -> UDWORD {
    let mut i: UDWORD = 0;
    let mut j: UDWORD = 0;
    let mut TallObj: UDWORD = 0 as libc::c_int as UDWORD;
    x = x >> 7 as libc::c_int;
    y = y >> 7 as libc::c_int;
    j = y;
    while j < y.wrapping_add(2 as libc::c_int as libc::c_uint) {
        i = x;
        while i < x.wrapping_add(2 as libc::c_int as libc::c_uint) {
            TallObj |=
                ((*mapTile(i, j)).tileInfoBits as libc::c_int &
                     0x80 as libc::c_int) as libc::c_uint;
            i = i.wrapping_add(1)
        }
        j = j.wrapping_add(1)
    }
    return TallObj;
}
//-----------------------------------------------------------------------------------
unsafe extern "C" fn getTileHighestCallback(mut x: SDWORD, mut y: SDWORD,
                                            mut dist: SDWORD) -> BOOL {
    let mut heightDif: SDWORD = 0;
    let mut height: UDWORD = 0;
    //iVector	pos;
    if clipXY(x, y) != 0 {
        height = map_Height(x as UDWORD, y as UDWORD) as UDWORD;
        if height > gHighestHeight as libc::c_uint && dist >= gHMinDist {
            heightDif =
                height.wrapping_sub(gHOrigHeight as libc::c_uint) as SDWORD;
            gHPitch =
                (atan2(heightDif as FRACT as libc::c_double,
                       (6 as libc::c_int * 128 as libc::c_int) as FRACT as
                           libc::c_double) * 180.0f64 / 3.141592654f64) as
                    FRACT;
            //		pos.x = x;
//		pos.y = height;
//		pos.z = y;
//		addEffect(&pos,EFFECT_EXPLOSION,EXPLOSION_TYPE_SMALL,FALSE,NULL,0);
            //MAKEFRACT(dist-(TILE_UNITS*3))));
            gHighestHeight = height as SDWORD
        }
    } else { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
//-----------------------------------------------------------------------------------
/* Will return false when we've hit the edge of the grid */
unsafe extern "C" fn getTileHeightCallback(mut x: SDWORD, mut y: SDWORD,
                                           mut dist: SDWORD) -> BOOL {
    let mut height: SDWORD = 0;
    let mut heightDif: SDWORD = 0;
    let mut newPitch: FRACT = 0.;
    let mut HasTallStructure: BOOL = 0 as libc::c_int;
    /* Are we still on the grid? */
    if clipXY(x, y) != 0 {
        HasTallStructure =
            (*mapTile((x >> 7 as libc::c_int) as UDWORD,
                      (y >> 7 as libc::c_int) as UDWORD)).tileInfoBits as
                libc::c_int & 0x80 as libc::c_int;
        if dist > 128 as libc::c_int || HasTallStructure != 0 {
            // Only do it the current tile is > TILE_UNITS away from the starting tile. Or..
		// there is a tall structure  on the current tile and the current tile is not the starting tile.
//		if( (dist>TILE_UNITS) ||
//			( (HasTallStructure = TILE_HAS_TALLSTRUCTURE(mapTile(x>>TILE_SHIFT,y>>TILE_SHIFT))) &&
//			((x >> TILE_SHIFT != gStartTileX) || (y >> TILE_SHIFT != gStartTileY)) ) ) {
			/* Get height at this intersection point */
            height = map_Height(x as UDWORD, y as UDWORD) as SDWORD;
            if HasTallStructure != 0 {
                height += 300 as libc::c_int
                //---
                //		if(height > gMaxRayHeight)
	//		{
	//			gMaxRayHeight = height;
	//			gRayDist = dist;
	//			return(TRUE);
	//		}
                //TALLOBJECT_ADJUST;
            }
            if height <= gHeight {
                heightDif = 0 as libc::c_int
            } else { heightDif = height - gHeight }
            newPitch =
                (atan2(heightDif as FRACT as libc::c_double,
                       dist as FRACT as libc::c_double) * 180.0f64 /
                     3.141592654f64) as FRACT;
            if newPitch > gPitch {
                /* Work out the angle to this point from start point */
                /* Is this the steepest we've found? */
                /* Yes, then keep a record of it */
                gPitch = newPitch
            }
        }
    } else {
        /* We've hit edge of grid - so exit!! */
        return 0 as libc::c_int
    }
    /* Not at edge yet - so exit */
    return 1 as libc::c_int;
}
// Calculates the maximum height and distance found along a line from any
// point to the edge of the grid
#[no_mangle]
pub unsafe extern "C" fn getBestPitchToEdgeOfGrid(mut x: UDWORD,
                                                  mut y: UDWORD,
                                                  mut direction: UDWORD,
                                                  mut pitch: *mut SDWORD) {
    /* Set global var to clear */
    gPitch = 0 as libc::c_int as FRACT;
    gHeight = map_Height(x, y) as SDWORD;
    gStartTileX = x >> 7 as libc::c_int;
    gStartTileY = y >> 7 as libc::c_int;
    //#ifdef TEST_RAY
//DBPRINTF(("%d\n",direction);
//#endif
    rayCast(x, y, direction.wrapping_rem(360 as libc::c_int as libc::c_uint),
            5430 as libc::c_int as UDWORD,
            Some(getTileHeightCallback as
                     unsafe extern "C" fn(_: SDWORD, _: SDWORD, _: SDWORD)
                         -> BOOL));
    *pitch = gPitch as SDWORD;
}
//-----------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn getPitchToHighestPoint(mut x: UDWORD, mut y: UDWORD,
                                                mut direction: UDWORD,
                                                mut thresholdDistance: UDWORD,
                                                mut pitch: *mut SDWORD) {
    gHPitch = 0 as libc::c_int as FRACT;
    gHOrigHeight = map_Height(x, y) as SDWORD;
    gHighestHeight = map_Height(x, y) as SDWORD;
    gHMinDist = thresholdDistance as SDWORD;
    rayCast(x, y, direction.wrapping_rem(360 as libc::c_int as libc::c_uint),
            3000 as libc::c_int as UDWORD,
            Some(getTileHighestCallback as
                     unsafe extern "C" fn(_: SDWORD, _: SDWORD, _: SDWORD)
                         -> BOOL));
    *pitch = gHPitch as SDWORD;
}
//-----------------------------------------------------------------------------------
