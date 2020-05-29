use ::libc;
extern "C" {
    #[no_mangle]
    fn abort() -> !;
    #[no_mangle]
    fn memMallocRelease(Size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn memFreeRelease(pMemToFree: *mut libc::c_void);
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
    static mut terrainTypes: [UBYTE; 255];
    #[no_mangle]
    static mut mapWidth: UDWORD;
    #[no_mangle]
    static mut mapHeight: UDWORD;
    #[no_mangle]
    static mut psMapTiles: *mut MAPTILE;
    #[no_mangle]
    static mut player: iView;
    #[no_mangle]
    static mut visibleXTiles: UDWORD;
    #[no_mangle]
    static mut visibleYTiles: UDWORD;
    /* The time for the last frame */
    #[no_mangle]
    static mut frameTime: UDWORD;
}
pub type size_t = libc::c_uint;
pub type UBYTE = libc::c_uchar;
pub type UWORD = libc::c_ushort;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iVector {
    pub x: int32,
    pub y: int32,
    pub z: int32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iView {
    pub p: iVector,
    pub r: iVector,
}
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
pub type C2RustUnnamed = libc::c_uint;
pub const ET_LAND: C2RustUnnamed = 1;
pub const ET_WATER: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct environ_data {
    pub bProcess: UBYTE,
    pub type_0: UBYTE,
    pub val: FRACT,
    pub data: UBYTE,
    pub vec: FRACT,
}
// -------------------------------------------------------------------------------
pub type ENVIRON_DATA = environ_data;
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
// -------------------------------------------------------------------------------
#[no_mangle]
pub static mut pEnvironData: *mut ENVIRON_DATA =
    0 as *const ENVIRON_DATA as *mut ENVIRON_DATA;
static mut bWaterOnMap: BOOL = 0 as libc::c_int;
// -------------------------------------------------------------------------------
// -------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn waterOnMap() -> BOOL { return bWaterOnMap; }
// -------------------------------------------------------------------------------
//this function just allocates the memory now for MaxMapWidth, MaxMapHeight
#[no_mangle]
pub unsafe extern "C" fn environInit() -> BOOL {
    pEnvironData =
        memMallocRelease((::std::mem::size_of::<environ_data>() as
                              libc::c_ulong).wrapping_mul(256 as libc::c_int
                                                              as
                                                              libc::c_uint).wrapping_mul(256
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_uint))
            as *mut ENVIRON_DATA;
    if pEnvironData.is_null() {
        debug(LOG_ERROR,
              b"Can\'t get memory for the environment data\x00" as *const u8
                  as *const libc::c_char);
        abort();
    }
    return 1 as libc::c_int;
}
//this function is called whenever the map changes - load new level or return from an offWorld map
#[no_mangle]
pub unsafe extern "C" fn environReset() {
    let mut i: UDWORD = 0;
    let mut j: UDWORD = 0;
    let mut index: UDWORD = 0;
    let mut psTile: *mut MAPTILE = 0 as *mut MAPTILE;
    if pEnvironData.is_null() {
        // loading map preview..
        return
    } //ENVIRON_LAND_INIT_VALUE;
    bWaterOnMap = 0 as libc::c_int;
    i = 0 as libc::c_int as UDWORD;
    while i < mapHeight {
        j = 0 as libc::c_int as UDWORD;
        while j < mapWidth {
            index = i.wrapping_mul(mapWidth).wrapping_add(j);
            psTile = mapTile(j, i);
            if terrainTypes[((*psTile).texture as libc::c_int &
                                 0x1ff as libc::c_int) as usize] as
                   libc::c_int == TER_WATER as libc::c_int {
                bWaterOnMap = 1 as libc::c_int;
                (*pEnvironData.offset(index as isize)).type_0 =
                    ET_WATER as libc::c_int as UBYTE;
                (*pEnvironData.offset(index as isize)).val =
                    (10 as libc::c_int + rand() % 10 as libc::c_int) as FRACT;
                (*pEnvironData.offset(index as isize)).data =
                    (155 as libc::c_int +
                         (100 as libc::c_int - rand() % 200 as libc::c_int))
                        as UBYTE;
                (*pEnvironData.offset(index as isize)).bProcess =
                    1 as libc::c_int as UBYTE
            } else {
                (*pEnvironData.offset(index as isize)).type_0 =
                    ET_LAND as libc::c_int as UBYTE;
                (*pEnvironData.offset(index as isize)).val =
                    0 as libc::c_int as FRACT;
                (*pEnvironData.offset(index as isize)).data =
                    0 as libc::c_int as UBYTE;
                (*pEnvironData.offset(index as isize)).bProcess =
                    0 as libc::c_int as UBYTE
            }
            (*pEnvironData.offset(index as isize)).vec =
                if rand() % 2 as libc::c_int != 0 {
                    -(1 as libc::c_int)
                } else { 1 as libc::c_int } as FRACT;
            j = j.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    };
}
// -------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn environUpdate() {
    let mut i: UDWORD = 0;
    let mut j: UDWORD = 0;
    let mut index: UDWORD = 0;
    let mut value: FRACT = 0.;
    let mut newValue: FRACT = 0.;
    let mut increment: FRACT = 0 as libc::c_int as FRACT;
    let mut lowest: FRACT = 0 as libc::c_int as FRACT;
    let mut highest: FRACT = 0 as libc::c_int as FRACT;
    let mut startX: SDWORD = 0;
    let mut startY: SDWORD = 0;
    let mut endX: SDWORD = 0;
    let mut endY: SDWORD = 0;
    let mut fraction: FRACT = 0.;
    //at the moment this function is getting called between levels and so crashes - quick check here for now
    if pEnvironData.is_null() { return }
    /* Only process the ones on the grid. Find top left */
    if player.p.x >= 0 as libc::c_int {
        startX = player.p.x / 128 as libc::c_int
    } else { startX = 0 as libc::c_int }
    if player.p.z >= 0 as libc::c_int {
        startY = player.p.z / 128 as libc::c_int
    } else { startY = 0 as libc::c_int }
    /* Find bottom right */
    endX = (startX as libc::c_uint).wrapping_add(visibleXTiles) as SDWORD;
    endY = (startY as libc::c_uint).wrapping_add(visibleYTiles) as SDWORD;
    /* Clip, as we may be off map */
    if startX < 0 as libc::c_int { startX = 0 as libc::c_int }
    if startY < 0 as libc::c_int { startY = 0 as libc::c_int }
    if endX as libc::c_uint >
           mapWidth.wrapping_sub(1 as libc::c_int as libc::c_uint) {
        endX =
            mapWidth.wrapping_sub(1 as libc::c_int as libc::c_uint) as SDWORD
    }
    if endY as libc::c_uint >
           mapHeight.wrapping_sub(1 as libc::c_int as libc::c_uint) {
        endY =
            mapHeight.wrapping_sub(1 as libc::c_int as libc::c_uint) as SDWORD
    }
    /* Find frame interval */
    fraction = frameTime as FRACT / 1000 as libc::c_int as libc::c_float;
    /* Go through the grid */
    i = startY as UDWORD;
    while i < endY as libc::c_uint {
        j = startX as UDWORD;
        while j < endX as libc::c_uint {
            /* Get our index */
            index = i.wrapping_mul(mapWidth).wrapping_add(j);
            /* Is it being updated? */
            if (*pEnvironData.offset(index as isize)).bProcess != 0 {
                /* Get old value */
                value = (*pEnvironData.offset(index as isize)).val;
                /* Establish extents for movement */
                match (*pEnvironData.offset(index as isize)).type_0 as
                          libc::c_int {
                    0 => {
                        lowest = 0.0f32;
                        highest = 20.0f32;
                        increment =
                            8.0f32 *
                                (*pEnvironData.offset(index as isize)).vec *
                                fraction
                    }
                    1 => {
                        lowest = 0.0f32;
                        highest = 64.0f32;
                        increment =
                            24.0f32 *
                                (*pEnvironData.offset(index as isize)).vec *
                                fraction
                    }
                    _ => {
                        debug(LOG_ERROR,
                              b"Weird environment type found\x00" as *const u8
                                  as *const libc::c_char);
                        abort();
                    }
                }
                /* Check bounds */
                if value + increment <= lowest {
                    newValue = lowest;
                    /* Flip sign */
                    let ref mut fresh0 =
                        (*pEnvironData.offset(index as isize)).vec;
                    *fresh0 *= -(1 as libc::c_int) as libc::c_float
                } else if value + increment > highest {
                    newValue = highest;
                    /* Flip sign */
                    let ref mut fresh1 =
                        (*pEnvironData.offset(index as isize)).vec;
                    *fresh1 *= -(1 as libc::c_int) as libc::c_float
                } else {
                    /* Bounds are fine */
                    newValue = value + increment
                }
                /* Store away new value */
                (*pEnvironData.offset(index as isize)).val = newValue
            }
            j = j.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    };
}
// -------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn environGetValue(mut x: UDWORD, mut y: UDWORD)
 -> UDWORD {
    let mut retVal: SDWORD = 0;
    retVal =
        (*pEnvironData.offset(y.wrapping_mul(mapWidth).wrapping_add(x) as
                                  isize)).val as SDWORD;
    if retVal < 0 as libc::c_int { retVal = 0 as libc::c_int }
    return retVal as UDWORD;
}
// -------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn environGetData(mut x: UDWORD, mut y: UDWORD)
 -> UDWORD {
    let mut retVal: SDWORD = 0;
    retVal =
        (*pEnvironData.offset(y.wrapping_mul(mapWidth).wrapping_add(x) as
                                  isize)).data as SDWORD;
    if retVal < 0 as libc::c_int { retVal = 0 as libc::c_int }
    return retVal as UDWORD;
}
// -------------------------------------------------------------------------------
/* Return linear interpolated mist value of x,y */
#[no_mangle]
pub unsafe extern "C" fn map_MistValue(mut x: UDWORD, mut y: UDWORD)
 -> UDWORD {
    let mut retVal: UDWORD = 0;
    let mut tileX: UDWORD = 0;
    let mut tileY: UDWORD = 0;
    let mut tileYOffset: UDWORD = 0;
    let mut h0: SDWORD = 0;
    let mut hx: SDWORD = 0;
    let mut hy: SDWORD = 0;
    let mut hxy: SDWORD = 0;
    let mut dx: SDWORD = 0;
    let mut dy: SDWORD = 0;
    let mut ox: SDWORD = 0;
    let mut oy: SDWORD = 0;
    /*	ASSERT( x < (mapWidth << TILE_SHIFT),
		"mapHeight: x coordinate bigger than map width" );
	ASSERT( y < (mapHeight<< TILE_SHIFT),
		"mapHeight: y coordinate bigger than map height" );
*/
    x =
        if x > 0x7fffffff as libc::c_int as libc::c_uint {
            0 as libc::c_int as libc::c_uint
        } else { x }; //negative SDWORD passed as UDWORD
    x =
        if x >= mapWidth << 7 as libc::c_int {
            (mapWidth.wrapping_sub(1 as libc::c_int as libc::c_uint)) <<
                7 as libc::c_int
        } else { x }; //negative SDWORD passed as UDWORD
    y =
        if y > 0x7fffffff as libc::c_int as libc::c_uint {
            0 as libc::c_int as libc::c_uint
        } else { y };
    y =
        if y >= mapHeight << 7 as libc::c_int {
            (mapHeight.wrapping_sub(1 as libc::c_int as libc::c_uint)) <<
                7 as libc::c_int
        } else { y };
    /* Tile comp */
    tileX = x >> 7 as libc::c_int;
    tileY = y >> 7 as libc::c_int;
    /* Inter tile comp */
    ox =
        (x & (128 as libc::c_int - 1 as libc::c_int) as libc::c_uint) as
            SDWORD;
    oy =
        (y & (128 as libc::c_int - 1 as libc::c_int) as libc::c_uint) as
            SDWORD;
    /* If this happens, then get quick height */
    if x == 0 && y == 0 { return map_TileMistValue(tileX, tileY) }
    tileYOffset = tileY.wrapping_mul(mapWidth);
    //	ox = (SDWORD)x - (SDWORD)(tileX << TILE_SHIFT);
//	oy = (SDWORD)y - (SDWORD)(tileY << TILE_SHIFT);
    if ox < 128 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"mapHeight: x offset too big\x00" as *const u8 as
                  *const libc::c_char);
    };
    if ox < 128 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"environ.c\x00" as *const u8 as *const libc::c_char,
              283 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"map_MistValue\x00")).as_ptr(),
              b"ox < TILE_UNITS\x00" as *const u8 as *const libc::c_char);
    };
    if oy < 128 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"mapHeight: y offset too big\x00" as *const u8 as
                  *const libc::c_char);
    };
    if oy < 128 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"environ.c\x00" as *const u8 as *const libc::c_char,
              284 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"map_MistValue\x00")).as_ptr(),
              b"oy < TILE_UNITS\x00" as *const u8 as *const libc::c_char);
    };
    if ox >= 0 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"mapHeight: x offset too small\x00" as *const u8 as
                  *const libc::c_char);
    };
    if ox >= 0 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"environ.c\x00" as *const u8 as *const libc::c_char,
              285 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"map_MistValue\x00")).as_ptr(),
              b"ox >= 0\x00" as *const u8 as *const libc::c_char);
    };
    if oy >= 0 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"mapHeight: y offset too small\x00" as *const u8 as
                  *const libc::c_char);
    };
    if oy >= 0 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"environ.c\x00" as *const u8 as *const libc::c_char,
              286 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"map_MistValue\x00")).as_ptr(),
              b"oy >= 0\x00" as *const u8 as *const libc::c_char);
    };
    //different code for 4 different triangle cases
    if (*psMapTiles.offset(tileX.wrapping_add(tileYOffset) as isize)).texture
           as libc::c_int & 0x800 as libc::c_int != 0 {
        if ox + oy > 128 as libc::c_int {
            //tile split top right to bottom left object if in bottom right half
            ox = 128 as libc::c_int - ox;
            oy = 128 as libc::c_int - oy;
            hy =
                (*pEnvironData.offset(tileX.wrapping_add(tileYOffset).wrapping_add(mapWidth)
                                          as isize)).val as SDWORD;
            hx =
                (*pEnvironData.offset(tileX.wrapping_add(1 as libc::c_int as
                                                             libc::c_uint).wrapping_add(tileYOffset)
                                          as isize)).val as SDWORD;
            hxy =
                (*pEnvironData.offset(tileX.wrapping_add(1 as libc::c_int as
                                                             libc::c_uint).wrapping_add(tileYOffset).wrapping_add(mapWidth)
                                          as isize)).val as SDWORD;
            dx = (hy - hxy) * ox / 128 as libc::c_int;
            dy = (hx - hxy) * oy / 128 as libc::c_int;
            retVal = (hxy + dx + dy) as UDWORD;
            return retVal.wrapping_mul(4 as libc::c_int as libc::c_uint)
        } else {
            //tile split top right to bottom left object if in top left half
            h0 =
                (*pEnvironData.offset(tileX.wrapping_add(tileYOffset) as
                                          isize)).val as SDWORD;
            hy =
                (*pEnvironData.offset(tileX.wrapping_add(tileYOffset).wrapping_add(mapWidth)
                                          as isize)).val as SDWORD;
            hx =
                (*pEnvironData.offset(tileX.wrapping_add(1 as libc::c_int as
                                                             libc::c_uint).wrapping_add(tileYOffset)
                                          as isize)).val as SDWORD;
            dx = (hx - h0) * ox / 128 as libc::c_int;
            dy = (hy - h0) * oy / 128 as libc::c_int;
            retVal = (h0 + dx + dy) as UDWORD;
            return retVal.wrapping_mul(4 as libc::c_int as libc::c_uint)
        }
    } else if ox > oy {
        //tile split topleft to bottom right object if in top right half
        h0 =
            (*pEnvironData.offset(tileX.wrapping_add(tileYOffset) as
                                      isize)).val as SDWORD;
        hx =
            (*pEnvironData.offset(tileX.wrapping_add(1 as libc::c_int as
                                                         libc::c_uint).wrapping_add(tileYOffset)
                                      as isize)).val as SDWORD;
        hxy =
            (*pEnvironData.offset(tileX.wrapping_add(1 as libc::c_int as
                                                         libc::c_uint).wrapping_add(tileYOffset).wrapping_add(mapWidth)
                                      as isize)).val as SDWORD;
        dx = (hx - h0) * ox / 128 as libc::c_int;
        dy = (hxy - hx) * oy / 128 as libc::c_int;
        retVal = (h0 + dx + dy) as UDWORD;
        return retVal.wrapping_mul(4 as libc::c_int as libc::c_uint)
    } else {
        //tile split topleft to bottom right object if in bottom left half
        h0 =
            (*pEnvironData.offset(tileX.wrapping_add(tileYOffset) as
                                      isize)).val as SDWORD;
        hy =
            (*pEnvironData.offset(tileX.wrapping_add(tileYOffset).wrapping_add(mapWidth)
                                      as isize)).val as SDWORD;
        hxy =
            (*pEnvironData.offset(tileX.wrapping_add(1 as libc::c_int as
                                                         libc::c_uint).wrapping_add(tileYOffset).wrapping_add(mapWidth)
                                      as isize)).val as SDWORD;
        dx = (hxy - hy) * ox / 128 as libc::c_int;
        dy = (hy - h0) * oy / 128 as libc::c_int;
        retVal = (h0 + dx + dy) as UDWORD;
        return retVal.wrapping_mul(4 as libc::c_int as libc::c_uint)
    };
}
// -------------------------------------------------------------------------------
/* Return height of tile at x,y */
#[no_mangle]
pub unsafe extern "C" fn map_TileMistValue(mut x: UDWORD, mut y: UDWORD)
 -> UDWORD {
    x =
        if x >= mapWidth {
            mapWidth.wrapping_sub(1 as libc::c_int as libc::c_uint)
        } else { x };
    y =
        if y >= mapHeight {
            mapHeight.wrapping_sub(1 as libc::c_int as libc::c_uint)
        } else { y };
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
              b"environ.c\x00" as *const u8 as *const libc::c_char,
              355 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 18],
                                        &[libc::c_char; 18]>(b"map_TileMistValue\x00")).as_ptr(),
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
              b"environ.c\x00" as *const u8 as *const libc::c_char,
              357 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 18],
                                        &[libc::c_char; 18]>(b"map_TileMistValue\x00")).as_ptr(),
              b"y < mapHeight\x00" as *const u8 as *const libc::c_char);
    };
    return ((*pEnvironData.offset(x.wrapping_add(y.wrapping_mul(mapWidth)) as
                                      isize)).val as SDWORD *
                4 as libc::c_int) as UDWORD;
}
// -------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn environShutDown() {
    if !pEnvironData.is_null() {
        memFreeRelease(pEnvironData as *mut libc::c_void);
        pEnvironData = 0 as *mut ENVIRON_DATA
    };
}
