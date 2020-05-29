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
    fn memMallocRelease(Size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn memFreeRelease(pMemToFree: *mut libc::c_void);
    //*************************************************************************
    //*************************************************************************
    //*************************************************************************
    // Was page provided by resource handler?
    //*************************************************************************
    //*************************************************************************
    #[no_mangle]
    fn pie_ChangeTexPage(tex_index: libc::c_int, s: *mut iSprite,
                         type_0: libc::c_int, bColourKeyed: iBool,
                         bResource: iBool);
    #[no_mangle]
    static mut _TEX_PAGE: [iTexPage; 64];
    #[no_mangle]
    fn pie_AddBMPtoTexPages(s: *mut iSprite, filename: *mut STRING,
                            type_0: libc::c_int, bColourKeyed: iBool,
                            bResource: iBool) -> libc::c_int;
    #[no_mangle]
    fn pie_GetLastPageDownloaded() -> UDWORD;
    #[no_mangle]
    fn calcRadarColour(tileBitmap: *mut iBitmap, tileNumber: UDWORD);
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
pub type UDWORD = libc::c_uint;
pub type SDWORD = libc::c_int;
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
pub type uint8 = libc::c_uchar;
pub type iBitmap = uint8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iColour {
    pub r: uint8,
    pub g: uint8,
    pub b: uint8,
}
pub type iBool = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iSprite {
    pub width: libc::c_uint,
    pub height: libc::c_uint,
    pub bmp: *mut iBitmap,
}
pub type iPalette = [iColour; 256];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iTexture {
    pub xshift: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub bmp: *mut iBitmap,
    pub pPal: *mut iColour,
    pub bColourKeyed: iBool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iTexPage {
    pub tex: iTexture,
    pub type_0: uint8,
    pub name: [libc::c_char; 80],
    pub textPage3dfx: libc::c_uint,
    pub bResource: libc::c_int,
}
pub type TILE_TEX_INFO = _tileTexInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _tileTexInfo {
    pub xOffset: UDWORD,
    pub yOffset: UDWORD,
    pub texPage: UDWORD,
}
#[no_mangle]
pub static mut tempTexStore: iSprite =
    iSprite{width: 0, height: 0, bmp: 0 as *const iBitmap as *mut iBitmap,};
#[no_mangle]
pub static mut tempPal: iPalette = [iColour{r: 0, g: 0, b: 0,}; 256];
// Offset into texture page to left hand edge
// Offset into texture page to top hand edge
// Which textpage is the tile in? TileNumber/16 basically;
/* Stores the graphics data for the terrain tiles textures */
#[no_mangle]
pub static mut tilesPCX: iSprite =
    iSprite{width: 0, height: 0, bmp: 0 as *const iBitmap as *mut iBitmap,};
/* Stores the raw PCX data for the terrain tiles at load file time */
#[no_mangle]
pub static mut tilesRAW: *mut *mut iBitmap =
    0 as *const *mut iBitmap as *mut *mut iBitmap;
/* How many tiles have we loaded */
#[no_mangle]
pub static mut numPCXTiles: UDWORD = 0;
/* How many pages have we loaded */
#[no_mangle]
pub static mut firstTexturePage: SDWORD = 0;
#[no_mangle]
pub static mut numTexturePages: SDWORD = 0;
#[no_mangle]
pub static mut pageId: [libc::c_int; 20] = [0; 20];
#[no_mangle]
pub static mut tileTexInfo: [TILE_TEX_INFO; 100] =
    [TILE_TEX_INFO{xOffset: 0, yOffset: 0, texPage: 0,}; 100];
/*
	Extracts the tile textures into separate texture pages and builds
	a table of which texture page to find each tile in, as well as which one it is
	within that page.

	0123
	4567
	89AB
	CDEF

	The above shows the different possible locations for a tile in the page.
	So we have a table of MAX_TILES showing

	pageNumber and [0..15]

	We must then make sure that we source in that texture page and set the
	texture coordinate for a complete tile to be its position.
*/
#[no_mangle]
pub unsafe extern "C" fn makeTileTexturePages(mut srcWidth: UDWORD,
                                              mut srcHeight: UDWORD,
                                              mut tileWidth: UDWORD,
                                              mut tileHeight: UDWORD,
                                              mut src: *mut libc::c_uchar) {
    let mut current_block: u64;
    let mut i: UDWORD = 0;
    let mut j: UDWORD = 0;
    let mut pageNumber: UDWORD = 0;
    let mut tilesAcross: UDWORD = 0;
    let mut tilesDown: UDWORD = 0;
    let mut tilesAcrossPage: UDWORD = 0;
    let mut tilesDownPage: UDWORD = 0;
    let mut tilesPerPage: UDWORD = 0;
    let mut tilesPerSource: UDWORD = 0;
    let mut tilesProcessed: UDWORD = 0;
    let mut tileStorage: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut presentLoc: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut sprite: iSprite =
        iSprite{width: 0,
                height: 0,
                bmp: 0 as *const iBitmap as *mut iBitmap,};
    /* This is how many pages are already used on hardware */
    firstTexturePage =
        pie_GetLastPageDownloaded().wrapping_add(1 as libc::c_int as
                                                     libc::c_uint) as SDWORD;
    debug(LOG_TEXTURE,
          b"makeTileTexturePages: src(%d,%d) tile(%d,%d) pages used=%d\x00" as
              *const u8 as *const libc::c_char, srcWidth, srcHeight,
          tileWidth, tileHeight, firstTexturePage);
    /* Get enough memory to store one tile */
    pageNumber = 0 as libc::c_int as UDWORD;
    tileStorage =
        memMallocRelease(tileWidth.wrapping_mul(tileHeight).wrapping_mul(4 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint))
            as *mut libc::c_uchar;
    sprite.bmp =
        memMallocRelease((512 as libc::c_int * 512 as libc::c_int *
                              4 as libc::c_int) as size_t) as *mut iBitmap;
    sprite.width = 512 as libc::c_int as libc::c_uint;
    sprite.height = 512 as libc::c_int as libc::c_uint;
    //	memset(sprite.bmp,0,TEXTURE_PAGE_SIZE);
    tilesProcessed = 0 as libc::c_int as UDWORD;
    tilesAcross = srcWidth.wrapping_div(tileWidth);
    tilesDown = srcHeight.wrapping_div(tileHeight);
    tilesPerSource = tilesAcross.wrapping_mul(tilesDown);
    tilesAcrossPage =
        (512 as libc::c_int as libc::c_uint).wrapping_div(tileWidth);
    tilesDownPage =
        (512 as libc::c_int as libc::c_uint).wrapping_div(tileHeight);
    tilesPerPage = tilesAcrossPage.wrapping_mul(tilesDownPage);
    presentLoc = sprite.bmp;
    i = 0 as libc::c_int as UDWORD;
    's_87:
        loop  {
            if !(i < tilesDown) {
                current_block = 13131896068329595644;
                break ;
            }
            j = 0 as libc::c_int as UDWORD;
            while j < tilesAcross {
                getRectFromPage(tileWidth, tileHeight, src, srcWidth,
                                tileStorage);
                putRectIntoPage(tileWidth, tileHeight, presentLoc,
                                512 as libc::c_int as UDWORD, tileStorage);
                tilesProcessed = tilesProcessed.wrapping_add(1);
                presentLoc =
                    presentLoc.offset(tileWidth.wrapping_mul(4 as libc::c_int
                                                                 as
                                                                 libc::c_uint)
                                          as isize);
                src =
                    src.offset(tileWidth.wrapping_mul(4 as libc::c_int as
                                                          libc::c_uint) as
                                   isize);
                /* Have we got all the tiles from the source!? */
                if tilesProcessed == tilesPerSource {
                    // || (tileStorage[0] == 0))//hack probably causes too many texture pages to be used
                    //			   	pie_Download8bitTexturePage(texturePage,PAGE_WIDTH,PAGE_HEIGHT);
                    pageId[pageNumber as usize] =
                        pie_AddBMPtoTexPages(&mut sprite,
                                             b"terrain\x00" as *const u8 as
                                                 *const libc::c_char as
                                                 *mut STRING,
                                             0 as libc::c_int,
                                             1 as libc::c_int,
                                             0 as libc::c_int);
                    current_block = 11086394427076829997;
                    break 's_87 ;
                } else {
                    /* Have we run out of texture page? */
                    if tilesProcessed.wrapping_rem(tilesPerPage) ==
                           0 as libc::c_int as libc::c_uint {
                        debug(LOG_TEXTURE,
                              b"makeTileTexturePages: ran out of texture page ...\x00"
                                  as *const u8 as *const libc::c_char);
                        debug(LOG_TEXTURE,
                              b"tilesDown=%d tilesAcross=%d tilesProcessed=%d tilesPerPage=%d\x00"
                                  as *const u8 as *const libc::c_char,
                              tilesDown, tilesAcross, tilesProcessed,
                              tilesPerPage);
                        /* If so, download this one and reset to start again */
                        pageId[pageNumber as usize] =
                            pie_AddBMPtoTexPages(&mut sprite,
                                                 b"terrain\x00" as *const u8
                                                     as *const libc::c_char as
                                                     *mut STRING,
                                                 0 as libc::c_int,
                                                 1 as libc::c_int,
                                                 0 as libc::c_int);
                        sprite.bmp =
                            memMallocRelease((512 as libc::c_int *
                                                  512 as libc::c_int *
                                                  4 as libc::c_int) as size_t)
                                as *mut iBitmap;
                        pageNumber = pageNumber.wrapping_add(1);
                        presentLoc = sprite.bmp
                    } else if tilesProcessed.wrapping_rem(tilesAcrossPage) ==
                                  0 as libc::c_int as libc::c_uint {
                        /* Right hand side of texture page */
				/* So go to one tile down */
                        presentLoc =
                            presentLoc.offset(tileHeight.wrapping_sub(1 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_uint).wrapping_mul(512
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         libc::c_uint).wrapping_mul(4
                                                                                                                                        as
                                                                                                                                        libc::c_int
                                                                                                                                        as
                                                                                                                                        libc::c_uint)
                                                  as isize)
                    }
                    j = j.wrapping_add(1)
                }
            }
            src =
                src.offset(tileHeight.wrapping_sub(1 as libc::c_int as
                                                       libc::c_uint).wrapping_mul(srcWidth).wrapping_mul(4
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             libc::c_uint)
                               as isize);
            i = i.wrapping_add(1)
        }
    match current_block {
        13131896068329595644 => { numTexturePages = pageNumber as SDWORD }
        _ => { }
    }
    memFreeRelease(tileStorage as *mut libc::c_void);
    tileStorage = 0 as *mut libc::c_uchar;
    buildTileIndexes();
}
#[no_mangle]
pub unsafe extern "C" fn remakeTileTexturePages(mut srcWidth: UDWORD,
                                                mut srcHeight: UDWORD,
                                                mut tileWidth: UDWORD,
                                                mut tileHeight: UDWORD,
                                                mut src: *mut libc::c_uchar) {
    let mut current_block: u64;
    let mut i: UDWORD = 0;
    let mut j: UDWORD = 0;
    let mut pageNumber: UDWORD = 0;
    let mut tilesAcross: UDWORD = 0;
    let mut tilesDown: UDWORD = 0;
    let mut tilesAcrossPage: UDWORD = 0;
    let mut tilesDownPage: UDWORD = 0;
    let mut tilesPerPage: UDWORD = 0;
    let mut tilesPerSource: UDWORD = 0;
    let mut tilesProcessed: UDWORD = 0;
    let mut tileStorage: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut presentLoc: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut sprite: iSprite =
        iSprite{width: 0,
                height: 0,
                bmp: 0 as *const iBitmap as *mut iBitmap,};
    //check enough pages are allocated
    debug(LOG_TEXTURE,
          b"remakeTileTexturePages: src(%d,%d), tile(%d, %d)\x00" as *const u8
              as *const libc::c_char, srcWidth, srcHeight, tileWidth,
          tileHeight);
    /* Get enough memory to store one tile */
    pageNumber = 0 as libc::c_int as UDWORD;
    tileStorage =
        memMallocRelease(tileWidth.wrapping_mul(tileHeight).wrapping_mul(4 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint))
            as *mut libc::c_uchar;
    //	texturePage = MALLOC(TEXTURE_PAGE_SIZE);
    sprite.width = 512 as libc::c_int as libc::c_uint;
    sprite.height = 512 as libc::c_int as libc::c_uint;
    sprite.bmp =
        memMallocRelease((512 as libc::c_int * 512 as libc::c_int *
                              4 as libc::c_int) as size_t) as *mut iBitmap;
    //	memset(sprite.bmp,0,TEXTURE_PAGE_SIZE);
    tilesProcessed = 0 as libc::c_int as UDWORD;
    tilesAcross = srcWidth.wrapping_div(tileWidth);
    tilesDown = srcHeight.wrapping_div(tileHeight);
    tilesPerSource = tilesAcross.wrapping_mul(tilesDown);
    tilesAcrossPage =
        (512 as libc::c_int as libc::c_uint).wrapping_div(tileWidth);
    tilesDownPage =
        (512 as libc::c_int as libc::c_uint).wrapping_div(tileHeight);
    tilesPerPage = tilesAcrossPage.wrapping_mul(tilesDownPage);
    presentLoc = sprite.bmp;
    i = 0 as libc::c_int as UDWORD;
    's_83:
        loop  {
            if !(i < tilesDown) {
                current_block = 15597372965620363352;
                break ;
            }
            j = 0 as libc::c_int as UDWORD;
            while j < tilesAcross {
                getRectFromPage(tileWidth, tileHeight, src, srcWidth,
                                tileStorage);
                putRectIntoPage(tileWidth, tileHeight, presentLoc,
                                512 as libc::c_int as UDWORD, tileStorage);
                tilesProcessed = tilesProcessed.wrapping_add(1);
                presentLoc =
                    presentLoc.offset(tileWidth.wrapping_mul(4 as libc::c_int
                                                                 as
                                                                 libc::c_uint)
                                          as isize);
                src =
                    src.offset(tileWidth.wrapping_mul(4 as libc::c_int as
                                                          libc::c_uint) as
                                   isize);
                /* Have we got all the tiles from the source!? */
                if tilesProcessed == tilesPerSource {
                    // || (tileStorage[0] == 0))//hack probably causes too many texture pages to be used
                    pie_ChangeTexPage(pageId[pageNumber as usize],
                                      &mut sprite, 0 as libc::c_int,
                                      1 as libc::c_int, 0 as libc::c_int);
                    current_block = 5221305508586845081;
                    break 's_83 ;
                } else {
                    /* Have we run out of texture page? */
                    if tilesProcessed.wrapping_rem(tilesPerPage) ==
                           0 as libc::c_int as libc::c_uint {
                        debug(LOG_TEXTURE,
                              b"remakeTileTexturePages: ran out of texture page ...\x00"
                                  as *const u8 as *const libc::c_char);
                        debug(LOG_TEXTURE,
                              b"tilesDown=%d tilesAcross=%d tilesProcessed=%d tilesPerPage=%d\x00"
                                  as *const u8 as *const libc::c_char,
                              tilesDown, tilesAcross, tilesProcessed,
                              tilesPerPage);
                        pie_ChangeTexPage(pageId[pageNumber as usize],
                                          &mut sprite, 0 as libc::c_int,
                                          1 as libc::c_int, 0 as libc::c_int);
                        pageNumber = pageNumber.wrapping_add(1);
                        presentLoc = sprite.bmp
                    } else if tilesProcessed.wrapping_rem(tilesAcrossPage) ==
                                  0 as libc::c_int as libc::c_uint {
                        /* Right hand side of texture page */
				/* So go to one tile down */
                        presentLoc =
                            presentLoc.offset(tileHeight.wrapping_sub(1 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_uint).wrapping_mul(512
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         libc::c_uint).wrapping_mul(4
                                                                                                                                        as
                                                                                                                                        libc::c_int
                                                                                                                                        as
                                                                                                                                        libc::c_uint)
                                                  as isize)
                    }
                    j = j.wrapping_add(1)
                }
            }
            src =
                src.offset(tileHeight.wrapping_sub(1 as libc::c_int as
                                                       libc::c_uint).wrapping_mul(srcWidth).wrapping_mul(4
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             libc::c_uint)
                               as isize);
            i = i.wrapping_add(1)
        }
    match current_block {
        15597372965620363352 => {
            //check numTexturePages == pageNumber;
            if numTexturePages >= pageNumber as SDWORD {
            } else {
                debug(LOG_ERROR,
                      b"New Tertiles too large\x00" as *const u8 as
                          *const libc::c_char);
            };
            if numTexturePages >= pageNumber as SDWORD {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"texture.c\x00" as *const u8 as *const libc::c_char,
                      220 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 23],
                                                &[libc::c_char; 23]>(b"remakeTileTexturePages\x00")).as_ptr(),
                      b"numTexturePages >= (SDWORD)pageNumber\x00" as
                          *const u8 as *const libc::c_char);
            };
        }
        _ => { }
    }
    memFreeRelease(tileStorage as *mut libc::c_void);
    tileStorage = 0 as *mut libc::c_uchar;
    buildTileIndexes();
}
#[no_mangle]
pub unsafe extern "C" fn getTileRadarColours() -> libc::c_int {
    let mut x: UDWORD = 0;
    let mut y: UDWORD = 0;
    let mut i: UDWORD = 0;
    let mut j: UDWORD = 0;
    let mut w: UDWORD = 0;
    let mut h: UDWORD = 0;
    let mut t: UDWORD = 0;
    let mut b: *mut iBitmap = 0 as *mut iBitmap;
    let mut s: *mut iBitmap = 0 as *mut iBitmap;
    let mut tempBMP: [UBYTE; 16384] = [0; 16384];
    w = tilesPCX.width.wrapping_div(128 as libc::c_int as libc::c_uint);
    h = tilesPCX.height.wrapping_div(128 as libc::c_int as libc::c_uint);
    numPCXTiles = w.wrapping_mul(h);
    t = 0 as libc::c_int as UDWORD;
    i = 0 as libc::c_int as UDWORD;
    while i < h {
        j = 0 as libc::c_int as UDWORD;
        while j < w {
            b =
                tilesPCX.bmp.offset(j.wrapping_mul(128 as libc::c_int as
                                                       libc::c_uint) as
                                        isize).offset(i.wrapping_mul(tilesPCX.width).wrapping_mul(128
                                                                                                      as
                                                                                                      libc::c_int
                                                                                                      as
                                                                                                      libc::c_uint)
                                                          as isize);
            s =
                &mut *tempBMP.as_mut_ptr().offset(0 as libc::c_int as isize)
                    as *mut UBYTE;
            if !s.is_null() {
                //copy the bitmap to temp buffer for colour calc
                y = 0 as libc::c_int as UDWORD;
                while y < 128 as libc::c_int as libc::c_uint {
                    x = 0 as libc::c_int as UDWORD;
                    while x < 128 as libc::c_int as libc::c_uint {
                        /* NOP */
                        let fresh0 = x;
                        x = x.wrapping_add(1);
                        let fresh1 = s;
                        s = s.offset(1);
                        *fresh1 = *b.offset(fresh0 as isize)
                    }
                    b = b.offset(tilesPCX.width as isize);
                    y = y.wrapping_add(1)
                }
                calcRadarColour(&mut *tempBMP.as_mut_ptr().offset(0 as
                                                                      libc::c_int
                                                                      as
                                                                      isize),
                                t);
                t = t.wrapping_add(1)
            } else { return 0 as libc::c_int }
            j = j.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn freeTileTextures() {
    let mut i: UDWORD = 0;
    i = 0 as libc::c_int as UDWORD;
    while i < numTexturePages as libc::c_uint {
        memFreeRelease(_TEX_PAGE[(firstTexturePage as
                                      libc::c_uint).wrapping_add(i) as
                                     usize].tex.bmp as *mut libc::c_void);
        _TEX_PAGE[(firstTexturePage as libc::c_uint).wrapping_add(i) as
                      usize].tex.bmp = 0 as *mut iBitmap;
        i = i.wrapping_add(1)
    };
}
unsafe extern "C" fn getTileXIndex(mut tileNumber: UDWORD) -> UDWORD {
    let mut texPage: UDWORD = 0;
    let mut tileInPage: UDWORD = 0;
    let mut xIndex: UDWORD = 0;
    texPage = tileNumber.wrapping_div(16 as libc::c_int as libc::c_uint);
    tileInPage =
        tileNumber.wrapping_sub(texPage.wrapping_mul(16 as libc::c_int as
                                                         libc::c_uint));
    xIndex = tileInPage.wrapping_rem(4 as libc::c_int as libc::c_uint);
    return xIndex;
}
unsafe extern "C" fn getTileYIndex(mut tileNumber: UDWORD) -> UDWORD {
    let mut texPage: UDWORD = 0;
    let mut tileInPage: UDWORD = 0;
    let mut yIndex: UDWORD = 0;
    texPage = tileNumber.wrapping_div(16 as libc::c_int as libc::c_uint);
    tileInPage =
        tileNumber.wrapping_sub(texPage.wrapping_mul(16 as libc::c_int as
                                                         libc::c_uint));
    yIndex = tileInPage.wrapping_div(4 as libc::c_int as libc::c_uint);
    return yIndex;
}
/* Extracts a rectangular buffer from a source buffer, storing result in one contiguous
   chunk	*/
unsafe extern "C" fn getRectFromPage(mut width: UDWORD, mut height: UDWORD,
                                     mut src: *mut libc::c_uchar,
                                     mut bufWidth: UDWORD,
                                     mut dest: *mut libc::c_uchar) {
    let mut i: UDWORD = 0;
    let mut j: UDWORD = 0;
    i = 0 as libc::c_int as UDWORD;
    while i < height {
        j = 0 as libc::c_int as UDWORD;
        while j < width.wrapping_mul(4 as libc::c_int as libc::c_uint) {
            let fresh2 = src;
            src = src.offset(1);
            let fresh3 = dest;
            dest = dest.offset(1);
            *fresh3 = *fresh2;
            j = j.wrapping_add(1)
        }
        src =
            src.offset(bufWidth.wrapping_sub(width).wrapping_mul(4 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint)
                           as isize);
        i = i.wrapping_add(1)
    };
}
/* Inserts a rectangle into a dest rectangle */
unsafe extern "C" fn putRectIntoPage(mut width: UDWORD, mut height: UDWORD,
                                     mut dest: *mut libc::c_uchar,
                                     mut bufWidth: UDWORD,
                                     mut src: *mut libc::c_uchar) {
    let mut i: UDWORD = 0;
    let mut j: UDWORD = 0;
    i = 0 as libc::c_int as UDWORD;
    while i < height {
        j = 0 as libc::c_int as UDWORD;
        while j < width.wrapping_mul(4 as libc::c_int as libc::c_uint) {
            let fresh4 = src;
            src = src.offset(1);
            let fresh5 = dest;
            dest = dest.offset(1);
            *fresh5 = *fresh4;
            j = j.wrapping_add(1)
        }
        dest =
            dest.offset(bufWidth.wrapping_sub(width).wrapping_mul(4 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint)
                            as isize);
        i = i.wrapping_add(1)
    };
}
unsafe extern "C" fn buildTileIndexes() {
    let mut i: UDWORD = 0;
    i = 0 as libc::c_int as UDWORD;
    while i < 100 as libc::c_int as libc::c_uint {
        tileTexInfo[i as usize].xOffset = getTileXIndex(i);
        tileTexInfo[i as usize].yOffset = getTileYIndex(i);
        tileTexInfo[i as usize].texPage =
            pageId[i.wrapping_div(16 as libc::c_int as libc::c_uint) as usize]
                as UDWORD;
        i = i.wrapping_add(1)
        //(i/16) + firstTexturePage;
    };
}
