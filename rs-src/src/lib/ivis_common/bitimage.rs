use ::libc;
extern "C" {
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_uint;
    /* Compare S1 and S2, ignoring case.  */
    #[no_mangle]
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char)
     -> libc::c_int;
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
    #[no_mangle]
    fn resGetData(pType: *mut STRING, pID: *mut STRING) -> *mut libc::c_void;
    #[no_mangle]
    fn resPresent(pType: *mut STRING, pID: *mut STRING) -> BOOL;
    //*************************************************************************
    #[no_mangle]
    static mut _TEX_INDEX: libc::c_int;
    #[no_mangle]
    static mut _TEX_PAGE: [iTexPage; 64];
    #[no_mangle]
    fn pie_AddBMPtoTexPages(s: *mut iSprite, filename: *mut STRING,
                            type_0: libc::c_int, bColourKeyed: iBool,
                            bResource: iBool) -> libc::c_int;
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
pub struct IMAGEHEADER {
    pub Type: [UBYTE; 4],
    pub Version: UWORD,
    pub NumImages: UWORD,
    pub BitDepth: UWORD,
    pub NumTPages: UWORD,
    pub TPageFiles: [[UBYTE; 16]; 16],
    pub PalFile: [UBYTE; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IMAGEDEF {
    pub TPageID: UWORD,
    pub PalID: UWORD,
    pub Tu: UWORD,
    pub Tv: UWORD,
    pub Width: UWORD,
    pub Height: UWORD,
    pub XOffset: SWORD,
    pub YOffset: SWORD,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IMAGEFILE {
    pub Header: IMAGEHEADER,
    pub TexturePages: *mut iSprite,
    pub NumCluts: UWORD,
    pub TPageIDs: [UWORD; 16],
    pub ClutIDs: [UWORD; 48],
    pub ImageDefs: *mut IMAGEDEF,
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
/* Endianness hacks */
#[inline]
unsafe extern "C" fn endian_uword(mut uword: *mut UWORD) { }
#[inline]
unsafe extern "C" fn endian_sword(mut sword: *mut SWORD) { }
#[no_mangle]
pub unsafe extern "C" fn iV_GetImageWidth(mut ImageFile: *mut IMAGEFILE,
                                          mut ID: UWORD) -> UWORD {
    return (*(*ImageFile).ImageDefs.offset(ID as isize)).Width;
}
#[no_mangle]
pub unsafe extern "C" fn iV_GetImageHeight(mut ImageFile: *mut IMAGEFILE,
                                           mut ID: UWORD) -> UWORD {
    return (*(*ImageFile).ImageDefs.offset(ID as isize)).Height;
}
// Get image width with no coordinate conversion.
//
#[no_mangle]
pub unsafe extern "C" fn iV_GetImageWidthNoCC(mut ImageFile: *mut IMAGEFILE,
                                              mut ID: UWORD) -> UWORD {
    return (*(*ImageFile).ImageDefs.offset(ID as isize)).Width;
}
// Get image height with no coordinate conversion.
//
#[no_mangle]
pub unsafe extern "C" fn iV_GetImageHeightNoCC(mut ImageFile: *mut IMAGEFILE,
                                               mut ID: UWORD) -> UWORD {
    return (*(*ImageFile).ImageDefs.offset(ID as isize)).Height;
}
#[no_mangle]
pub unsafe extern "C" fn iV_GetImageXOffset(mut ImageFile: *mut IMAGEFILE,
                                            mut ID: UWORD) -> SWORD {
    return (*(*ImageFile).ImageDefs.offset(ID as isize)).XOffset;
}
#[no_mangle]
pub unsafe extern "C" fn iV_GetImageYOffset(mut ImageFile: *mut IMAGEFILE,
                                            mut ID: UWORD) -> SWORD {
    return (*(*ImageFile).ImageDefs.offset(ID as isize)).YOffset;
}
#[no_mangle]
pub unsafe extern "C" fn iV_GetImageCenterX(mut ImageFile: *mut IMAGEFILE,
                                            mut ID: UWORD) -> UWORD {
    return ((*(*ImageFile).ImageDefs.offset(ID as isize)).XOffset as
                libc::c_int +
                (*(*ImageFile).ImageDefs.offset(ID as isize)).Width as
                    libc::c_int / 2 as libc::c_int) as UWORD;
}
#[no_mangle]
pub unsafe extern "C" fn iV_GetImageCenterY(mut ImageFile: *mut IMAGEFILE,
                                            mut ID: UWORD) -> UWORD {
    return ((*(*ImageFile).ImageDefs.offset(ID as isize)).YOffset as
                libc::c_int +
                (*(*ImageFile).ImageDefs.offset(ID as isize)).Height as
                    libc::c_int / 2 as libc::c_int) as UWORD;
}
#[no_mangle]
pub unsafe extern "C" fn iV_LoadImageFile(mut FileData: *mut libc::c_char,
                                          mut FileSize: UDWORD)
 -> *mut IMAGEFILE {
    let mut Ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut Header: *mut IMAGEHEADER = 0 as *mut IMAGEHEADER;
    let mut ImageFile: *mut IMAGEFILE = 0 as *mut IMAGEFILE;
    let mut ImageDef: *mut IMAGEDEF = 0 as *mut IMAGEDEF;
    let mut i: libc::c_int = 0;
    Ptr = FileData;
    Header = Ptr as *mut IMAGEHEADER;
    Ptr =
        Ptr.offset(::std::mem::size_of::<IMAGEHEADER>() as libc::c_ulong as
                       isize);
    endian_uword(&mut (*Header).Version);
    endian_uword(&mut (*Header).NumImages);
    endian_uword(&mut (*Header).BitDepth);
    endian_uword(&mut (*Header).NumTPages);
    ImageFile =
        memMallocRelease(::std::mem::size_of::<IMAGEFILE>() as libc::c_ulong)
            as *mut IMAGEFILE;
    if ImageFile.is_null() {
        debug(LOG_ERROR,
              b"Out of memory\x00" as *const u8 as *const libc::c_char);
        return 0 as *mut IMAGEFILE
    }
    (*ImageFile).TexturePages =
        memMallocRelease((::std::mem::size_of::<iSprite>() as
                              libc::c_ulong).wrapping_mul((*Header).NumTPages
                                                              as
                                                              libc::c_uint))
            as *mut iSprite;
    if (*ImageFile).TexturePages.is_null() {
        debug(LOG_ERROR,
              b"Out of memory\x00" as *const u8 as *const libc::c_char);
        return 0 as *mut IMAGEFILE
    }
    (*ImageFile).ImageDefs =
        memMallocRelease((::std::mem::size_of::<IMAGEDEF>() as
                              libc::c_ulong).wrapping_mul((*Header).NumImages
                                                              as
                                                              libc::c_uint))
            as *mut IMAGEDEF;
    if (*ImageFile).ImageDefs.is_null() {
        debug(LOG_ERROR,
              b"Out of memory\x00" as *const u8 as *const libc::c_char);
        return 0 as *mut IMAGEFILE
    }
    (*ImageFile).Header = *Header;
    // Load the texture pages.
    i = 0 as libc::c_int; /* Workaround for MacOS gcc 4.0.0 bug. */
    while i < (*Header).NumTPages as libc::c_int {
        let mut tmp: libc::c_int = 0;
        LoadTextureFile((*Header).TPageFiles[i as usize].as_mut_ptr() as
                            *mut STRING,
                        &mut *(*ImageFile).TexturePages.offset(i as isize),
                        &mut tmp);
        (*ImageFile).TPageIDs[i as usize] = tmp as UWORD;
        i += 1
    }
    ImageDef = Ptr as *mut IMAGEDEF;
    i = 0 as libc::c_int;
    while i < (*Header).NumImages as libc::c_int {
        endian_uword(&mut (*ImageDef).TPageID);
        endian_uword(&mut (*ImageDef).PalID);
        endian_uword(&mut (*ImageDef).Tu);
        endian_uword(&mut (*ImageDef).Tv);
        endian_uword(&mut (*ImageDef).Width);
        endian_uword(&mut (*ImageDef).Height);
        endian_sword(&mut (*ImageDef).XOffset);
        endian_sword(&mut (*ImageDef).YOffset);
        *(*ImageFile).ImageDefs.offset(i as isize) = *ImageDef;
        if (*ImageDef).Width as libc::c_int <= 0 as libc::c_int ||
               (*ImageDef).Height as libc::c_int <= 0 as libc::c_int {
            debug(LOG_ERROR,
                  b"Illegal image size\x00" as *const u8 as
                      *const libc::c_char);
            return 0 as *mut IMAGEFILE
        }
        ImageDef = ImageDef.offset(1);
        i += 1
    }
    return ImageFile;
}
#[no_mangle]
pub unsafe extern "C" fn iV_FreeImageFile(mut ImageFile: *mut IMAGEFILE) {
    //	for(i=0; i<ImageFile->Header.NumTPages; i++) {
//		FREE(ImageFile->TexturePages[i].bmp);
//	}
    memFreeRelease((*ImageFile).TexturePages as *mut libc::c_void);
    (*ImageFile).TexturePages = 0 as *mut iSprite;
    memFreeRelease((*ImageFile).ImageDefs as *mut libc::c_void);
    (*ImageFile).ImageDefs = 0 as *mut IMAGEDEF;
    memFreeRelease(ImageFile as *mut libc::c_void);
    ImageFile = 0 as *mut IMAGEFILE;
}
unsafe extern "C" fn LoadTextureFile(mut FileName: *mut STRING,
                                     mut pSprite: *mut iSprite,
                                     mut texPageID: *mut libc::c_int)
 -> BOOL {
    let mut i: SDWORD = 0;
    let mut real_filename: [libc::c_char; 200] = [0; 200];
    // this is a hideous kludge to avoid having to change .img files, which
	// still contain pcx references
    strcpy(real_filename.as_mut_ptr(), FileName); // strip extension
    real_filename[strlen(real_filename.as_mut_ptr()).wrapping_sub(4 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint)
                      as usize] = '\u{0}' as i32 as libc::c_char;
    strcat(real_filename.as_mut_ptr(),
           b".png\x00" as *const u8 as *const libc::c_char);
    debug(LOG_TEXTURE,
          b"LoadTextureFile: %s\x00" as *const u8 as *const libc::c_char,
          real_filename.as_mut_ptr());
    if resPresent(b"IMGPAGE\x00" as *const u8 as *const libc::c_char as
                      *mut STRING, real_filename.as_mut_ptr()) == 0 {
        debug(LOG_ERROR,
              b"Texture file \"%s\" not preloaded.\x00" as *const u8 as
                  *const libc::c_char, real_filename.as_mut_ptr());
        return 0 as libc::c_int
    } else {
        *pSprite =
            *(resGetData(b"IMGPAGE\x00" as *const u8 as *const libc::c_char as
                             *mut STRING, real_filename.as_mut_ptr()) as
                  *mut iSprite);
        debug(LOG_TEXTURE,
              b"Load texture from resource cache: %s (%d, %d)\x00" as
                  *const u8 as *const libc::c_char,
              real_filename.as_mut_ptr(), (*pSprite).width,
              (*pSprite).height);
    }
    /* Back to beginning */
    i = 0 as libc::c_int;
    /* We have already loaded this one? */
    while i < _TEX_INDEX {
        if strcasecmp(real_filename.as_mut_ptr(),
                      _TEX_PAGE[i as usize].name.as_mut_ptr()) ==
               0 as libc::c_int {
            *texPageID = _TEX_PAGE[i as usize].textPage3dfx as libc::c_int;
            debug(LOG_TEXTURE,
                  b"LoadTextureFile: already loaded\x00" as *const u8 as
                      *const libc::c_char);
            return 1 as libc::c_int
        }
        i += 1
    }
    *texPageID =
        pie_AddBMPtoTexPages(pSprite, real_filename.as_mut_ptr(),
                             1 as libc::c_int, 1 as libc::c_int,
                             1 as libc::c_int);
    return 1 as libc::c_int;
}
