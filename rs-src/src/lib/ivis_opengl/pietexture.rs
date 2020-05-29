use ::libc;
extern "C" {
    #[no_mangle]
    static mut _TEX_INDEX: libc::c_int;
}
pub type UWORD = libc::c_ushort;
pub type UDWORD = libc::c_uint;
pub type SDWORD = libc::c_int;
pub type BOOL = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _textureState {
    pub lastPageDownloaded: UDWORD,
    pub texPage: UDWORD,
}
/* **************************************************************************/
/*
 * pieState.c
 *
 * renderer setup and state control routines for 3D rendering
 *
 */
/* **************************************************************************/
/* **************************************************************************/
/*
 *	Local Definitions
 */
/* **************************************************************************/
pub type TEXTURE_STATE = _textureState;
/* **************************************************************************/
/*
 *	Local Variables
 */
/* **************************************************************************/
#[no_mangle]
pub static mut textureStates: TEXTURE_STATE =
    TEXTURE_STATE{lastPageDownloaded: 0, texPage: 0,};
/* **************************************************************************/
/*
 *	Local ProtoTypes
 */
/* **************************************************************************/
/* **************************************************************************/
/*
 *	Source
 */
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn pie_Download8bitTexturePage(mut bitmap:
                                                         *mut libc::c_void,
                                                     mut Width: UWORD,
                                                     mut Height: UWORD)
 -> BOOL {
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn pie_Reload8bitTexturePage(mut bitmap:
                                                       *mut libc::c_void,
                                                   mut Width: UWORD,
                                                   mut Height: UWORD,
                                                   mut index: SDWORD)
 -> BOOL {
    //	return dtm_ReLoadTexture(index);
    return 0 as libc::c_int;
}
/* **************************************************************************/
/*
 * pieState.h
 *
 * render State controlr all pumpkin image library functions.
 *
 */
/* **************************************************************************/
/* **************************************************************************/
/* **************************************************************************/
/*
 *	Global Definitions
 */
/* **************************************************************************/
/* **************************************************************************/
/*
 *	Global Variables
 */
/* **************************************************************************/
/* **************************************************************************/
/*
 *	Global ProtoTypes
 */
/* **************************************************************************/
//assumes 256*256 page
#[no_mangle]
pub unsafe extern "C" fn pie_GetLastPageDownloaded() -> UDWORD {
    return _TEX_INDEX as UDWORD;
}
