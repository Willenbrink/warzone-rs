use ::libc;
extern "C" {
    /* Copy SRC to DEST.  */
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    /* Compare S1 and S2, ignoring case.  */
    #[no_mangle]
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn glGenTextures(n: GLsizei, textures: *mut GLuint);
    #[no_mangle]
    fn glTexParameteri(target: GLenum, pname: GLenum, param: GLint);
    #[no_mangle]
    fn glTexEnvf(target: GLenum, pname: GLenum, param: GLfloat);
    #[no_mangle]
    fn gluBuild2DMipmaps(target: GLenum, internalFormat: GLint,
                         width: GLsizei, height: GLsizei, format: GLenum,
                         type_0: GLenum, data: *const libc::c_void) -> GLint;
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
    fn memFreeRelease(pMemToFree: *mut libc::c_void);
    //render states
    #[no_mangle]
    fn pie_SetTexturePage(num: SDWORD);
    // PNG
    #[no_mangle]
    fn pie_PNGLoadMemToBuffer(pngimage: *mut libc::c_char, s: *mut iSprite,
                              pal: *mut iColour) -> BOOL;
}
pub type GLenum = libc::c_uint;
pub type GLint = libc::c_int;
pub type GLuint = libc::c_uint;
pub type GLsizei = libc::c_int;
pub type GLfloat = libc::c_float;
pub type STRING = libc::c_char;
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
pub type uint32 = libc::c_uint;
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
pub struct iTexPage {
    pub tex: iTexture,
    pub type_0: uint8,
    pub name: [libc::c_char; 80],
    pub textPage3dfx: libc::c_uint,
    pub bResource: libc::c_int,
}
//we need windows.h for below inculde.  --Qamly
//*************************************************************************
#[no_mangle]
pub static mut _TEX_PAGE: [iTexPage; 64] =
    [iTexPage{tex:
                  iTexture{xshift: 0,
                           width: 0,
                           height: 0,
                           bmp: 0 as *const iBitmap as *mut iBitmap,
                           pPal: 0 as *const iColour as *mut iColour,
                           bColourKeyed: 0,},
              type_0: 0,
              name: [0; 80],
              textPage3dfx: 0,
              bResource: 0,}; 64];
//*************************************************************************
#[no_mangle]
pub static mut _TEX_INDEX: libc::c_int = 0;
//*************************************************************************
unsafe extern "C" fn _tex_get_top_bit(mut n: uint32) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut mask: uint32 = 0x80000000 as libc::c_uint;
    i = 31 as libc::c_int;
    while n & mask == 0 as libc::c_int as libc::c_uint {
        mask >>= 1 as libc::c_int;
        i -= 1
    }
    return i;
}
//*************************************************************************
//*************************************************************************
//*************************************************************************
// Was page provided by resource handler?
//*************************************************************************
//*************************************************************************
/* *************************************************************************
	Add an image buffer given in s as a new texture page in the texture
	table.  We check first if the given image has already been loaded,
	as a sanity check (should never happen).  The texture numbers are
	stored in a special texture table, not in the resource system, for
	some unknown reason.

	Returns the texture number of the image.
**************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn pie_AddBMPtoTexPages(mut s: *mut iSprite,
                                              mut filename: *mut STRING,
                                              mut type_0: libc::c_int,
                                              mut bColourKeyed: iBool,
                                              mut bResource: iBool)
 -> libc::c_int {
    let mut i: libc::c_int = 0 as libc::c_int;
    debug(LOG_TEXTURE,
          b"pie_AddBMPtoTexPages: %s type=%d col=%d res=%d\x00" as *const u8
              as *const libc::c_char, filename, type_0, bColourKeyed,
          bResource);
    /* Have we already loaded this one? (Should generally not happen here.) */
    while i < _TEX_INDEX {
        if strcasecmp(filename, _TEX_PAGE[i as usize].name.as_mut_ptr()) ==
               0 as libc::c_int {
            // this happens with terrain for some reason, which is necessary
            debug(LOG_TEXTURE,
                  b"pie_AddBMPtoTexPages: %s loaded again\x00" as *const u8 as
                      *const libc::c_char, filename);
        }
        i += 1
    }
    /* Get next available texture page */
    i = _TEX_INDEX;
    /* Have we used up too many? */
    if _TEX_INDEX >= 64 as libc::c_int {
        debug(LOG_ERROR,
              b"pie_AddBMPtoTexPages: too many texture pages\x00" as *const u8
                  as *const libc::c_char);
        return -(1 as libc::c_int)
    }
    /* Stick the name into the tex page structures */
    strcpy(_TEX_PAGE[i as usize].name.as_mut_ptr(), filename);
    /* Store away all the info */
	/* DID come from a resource */
    _TEX_PAGE[i as usize].bResource = bResource;
    // Default values
    _TEX_PAGE[i as usize].tex.bmp = 0 as *mut iBitmap;
    _TEX_PAGE[i as usize].tex.width = 256 as libc::c_int;
    _TEX_PAGE[i as usize].tex.height = 256 as libc::c_int;
    _TEX_PAGE[i as usize].tex.xshift = 0 as libc::c_int;
    _TEX_PAGE[i as usize].tex.bmp = (*s).bmp;
    _TEX_PAGE[i as usize].tex.width = (*s).width as libc::c_int;
    _TEX_PAGE[i as usize].tex.height = (*s).height as libc::c_int;
    _TEX_PAGE[i as usize].tex.xshift = _tex_get_top_bit((*s).width);
    _TEX_PAGE[i as usize].tex.bColourKeyed = bColourKeyed;
    _TEX_PAGE[i as usize].type_0 = type_0 as uint8;
    glGenTextures(1 as libc::c_int,
                  &mut (*_TEX_PAGE.as_mut_ptr().offset(i as
                                                           isize)).textPage3dfx);
    pie_SetTexturePage(i);
    if (*s).width & (*s).width.wrapping_sub(1 as libc::c_int as libc::c_uint)
           == 0 as libc::c_int as libc::c_uint &&
           (*s).height &
               (*s).height.wrapping_sub(1 as libc::c_int as libc::c_uint) ==
               0 as libc::c_int as libc::c_uint {
        gluBuild2DMipmaps(0xde1 as libc::c_int as GLenum,
                          0x1908 as libc::c_int, (*s).width as GLsizei,
                          (*s).height as GLsizei,
                          0x1908 as libc::c_int as GLenum,
                          0x1401 as libc::c_int as GLenum,
                          (*s).bmp as *const libc::c_void);
        /*
		glTexImage2D(GL_TEXTURE_2D, 0, GL_RGBA, s->width, s->height, 0,
			     GL_RGBA, GL_UNSIGNED_BYTE, s->bmp);
*/
    } else {
        debug(LOG_TEXTURE,
              b"pie_AddBMPtoTexPages: non POT texture %s\x00" as *const u8 as
                  *const libc::c_char, filename);
    }
    glTexEnvf(0x2300 as libc::c_int as GLenum,
              0x2200 as libc::c_int as GLenum,
              0x2100 as libc::c_int as GLfloat);
    glTexParameteri(0xde1 as libc::c_int as GLenum,
                    0x2801 as libc::c_int as GLenum, 0x2701 as libc::c_int);
    glTexParameteri(0xde1 as libc::c_int as GLenum,
                    0x2800 as libc::c_int as GLenum, 0x2601 as libc::c_int);
    glTexParameteri(0xde1 as libc::c_int as GLenum,
                    0x2802 as libc::c_int as GLenum, 0x2900 as libc::c_int);
    glTexParameteri(0xde1 as libc::c_int as GLenum,
                    0x2803 as libc::c_int as GLenum, 0x2900 as libc::c_int);
    /* Send back the texpage number so we can store it in the IMD */
    _TEX_INDEX += 1;
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn pie_ChangeTexPage(mut tex_index: libc::c_int,
                                           mut s: *mut iSprite,
                                           mut type_0: libc::c_int,
                                           mut bColourKeyed: iBool,
                                           mut bResource: iBool) {
    /* DID come from a resource */
    _TEX_PAGE[tex_index as usize].bResource = bResource;
    // Default values
    _TEX_PAGE[tex_index as usize].tex.bmp = 0 as *mut iBitmap;
    _TEX_PAGE[tex_index as usize].tex.width = 256 as libc::c_int;
    _TEX_PAGE[tex_index as usize].tex.height = 256 as libc::c_int;
    _TEX_PAGE[tex_index as usize].tex.xshift = 0 as libc::c_int;
    _TEX_PAGE[tex_index as usize].tex.bmp = (*s).bmp;
    _TEX_PAGE[tex_index as usize].tex.width = (*s).width as libc::c_int;
    _TEX_PAGE[tex_index as usize].tex.height = (*s).height as libc::c_int;
    _TEX_PAGE[tex_index as usize].tex.xshift = _tex_get_top_bit((*s).width);
    _TEX_PAGE[tex_index as usize].tex.bColourKeyed = bColourKeyed;
    _TEX_PAGE[tex_index as usize].type_0 = type_0 as uint8;
    pie_SetTexturePage(tex_index);
}
/* *************************************************************************
	Return the texture number for a given texture resource.  We keep
	textures in a separate data structure _TEX_PAGE apart from the
	normal resource system.
**************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn iV_GetTexture(mut filename: *mut libc::c_char)
 -> libc::c_int {
    let mut i: libc::c_int = 0 as libc::c_int;
    /* Have we already loaded this one then? (Yes. Always.) */
    while i < _TEX_INDEX {
        if strcasecmp(filename, _TEX_PAGE[i as usize].name.as_mut_ptr()) ==
               0 as libc::c_int {
            return i
        }
        i += 1
    }
    /* This should never happen - by now all textures should have been loaded. */
    debug(LOG_ERROR,
          b"*** texture %s not loaded! ***\x00" as *const u8 as
              *const libc::c_char, filename);
    debug(LOG_ERROR,
          b"Available texture pages in memory:\x00" as *const u8 as
              *const libc::c_char);
    i = 0 as libc::c_int;
    while i < _TEX_INDEX {
        debug(LOG_ERROR,
              b"   %02d : %s\x00" as *const u8 as *const libc::c_char, i,
              _TEX_PAGE[i as usize].name.as_mut_ptr());
        i += 1
    }
    debug(LOG_ERROR,
          b"This error probably means you did not specify for this texture\x00"
              as *const u8 as *const libc::c_char);
    debug(LOG_ERROR,
          b"to be preloaded in the appropriate wrf files before referencing\x00"
              as *const u8 as *const libc::c_char);
    debug(LOG_ERROR,
          b"it in some pie file.  Remember that patches override several\x00"
              as *const u8 as *const libc::c_char);
    debug(LOG_ERROR,
          b"standard wrf files as well.\x00" as *const u8 as
              *const libc::c_char);
    return -(1 as libc::c_int);
}
// According to logfile not used, deprecating
#[no_mangle]
pub unsafe extern "C" fn pie_ReloadTexPage(mut filename: *mut STRING,
                                           mut pBuffer: *mut libc::c_char)
 -> libc::c_int {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut s: iSprite =
        iSprite{width: 0, height: 0, bmp: 0 as *mut iBitmap,};
    // Log call to check validity of deprecation
    debug(LOG_NEVER,
          b"pie_ReloadTexPage called\x00" as *const u8 as
              *const libc::c_char);
    /* Have we already loaded this one then? */
    while strcasecmp(filename, _TEX_PAGE[i as usize].name.as_mut_ptr()) !=
              0 as libc::c_int {
        i += 1;
        if i >= _TEX_INDEX {
            debug(LOG_TEXTURE,
                  b"Texture %s not in resources\x00" as *const u8 as
                      *const libc::c_char, filename);
            return -(1 as libc::c_int)
        }
    }
    //got the old texture page so load bmp straight in
    s.width = _TEX_PAGE[i as usize].tex.width as libc::c_uint;
    s.height = _TEX_PAGE[i as usize].tex.height as libc::c_uint;
    s.bmp = _TEX_PAGE[i as usize].tex.bmp;
    pie_PNGLoadMemToBuffer(pBuffer, &mut s, 0 as *mut iColour);
    return i;
}
//*************************************************************************
/*
	Alex - fixed this so it doesn't try to free up the memory if it got the page from resource
	handler - this is because the resource handler will deal with freeing it, and in all probability
	will have already done so by the time this is called, thus avoiding an 'already freed' moan.
*/
#[no_mangle]
pub unsafe extern "C" fn pie_TexShutDown() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    j = 0 as libc::c_int;
    while i < _TEX_INDEX {
        /*	Only free up the ones that were NOT allocated through resource handler cos they'll already
			be free */
        if _TEX_PAGE[i as usize].bResource == 0 as libc::c_int {
            if !_TEX_PAGE[i as usize].tex.bmp.is_null() {
                j += 1;
                memFreeRelease(_TEX_PAGE[i as usize].tex.bmp as
                                   *mut libc::c_void);
                _TEX_PAGE[i as usize].tex.bmp = 0 as *mut iBitmap
            }
        }
        i += 1
    }
    debug(LOG_NEVER,
          b"pie_TexShutDown successful - freed %d texture pages\n\x00" as
              *const u8 as *const libc::c_char, j);
}
#[no_mangle]
pub unsafe extern "C" fn pie_TexInit() {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < _TEX_INDEX {
        _TEX_PAGE[i as usize].tex.bmp = 0 as *mut iBitmap;
        _TEX_PAGE[i as usize].tex.width = 0 as libc::c_int;
        _TEX_PAGE[i as usize].tex.height = 0 as libc::c_int;
        _TEX_PAGE[i as usize].tex.xshift = 0 as libc::c_int;
        i += 1
    };
}
// Check that a texture is  <= 256x256 and 2^n x 2^n in size.
//
#[no_mangle]
pub unsafe extern "C" fn iV_TexSizeIsLegal(mut Width: UDWORD,
                                           mut Height: UDWORD) -> BOOL {
    if Width > 256 as libc::c_int as libc::c_uint ||
           Height > 256 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int
    }
    if iV_IsPower2(Width) == 0 { return 0 as libc::c_int }
    //  For now don't limit height to 2^n.
    if iV_IsPower2(Height) == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
// Return TRUE if the given value is 2^n.
//
#[no_mangle]
pub unsafe extern "C" fn iV_IsPower2(mut Value: UDWORD) -> BOOL {
    let mut Bits: libc::c_int = 0 as libc::c_int;
    while Value != 0 {
        if Value & 1 as libc::c_int as libc::c_uint != 0 { Bits += 1 }
        Value = Value >> 1 as libc::c_int
    }
    if Bits != 1 as libc::c_int { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
