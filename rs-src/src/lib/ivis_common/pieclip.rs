use ::libc;
extern "C" {
    #[no_mangle]
    static mut psRendSurface: *mut iSurface;
}
pub type UBYTE = libc::c_uchar;
pub type UWORD = libc::c_ushort;
pub type UDWORD = libc::c_uint;
pub type SDWORD = libc::c_int;
pub type BOOL = libc::c_int;
pub type int32 = libc::c_int;
pub type uint8 = libc::c_uchar;
pub type uint32 = libc::c_uint;
//*************************************************************************
//
// Simple derived types
//
//*************************************************************************
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iClip {
    pub left: libc::c_int,
    pub top: libc::c_int,
    pub right: libc::c_int,
    pub bottom: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iSurface {
    pub usr: libc::c_int,
    pub flags: uint32,
    pub xcentre: libc::c_int,
    pub ycentre: libc::c_int,
    pub xpshift: libc::c_int,
    pub ypshift: libc::c_int,
    pub clip: iClip,
    pub buffer: *mut uint8,
    pub scantable: [int32; 1024],
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub size: int32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PIELIGHTBYTES {
    pub b: UBYTE,
    pub g: UBYTE,
    pub r: UBYTE,
    pub a: UBYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union PIELIGHT {
    pub byte: PIELIGHTBYTES,
    pub argb: UDWORD,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PIEVERTEX {
    pub sx: SDWORD,
    pub sy: SDWORD,
    pub sz: SDWORD,
    pub tu: UWORD,
    pub tv: UWORD,
    pub light: PIELIGHT,
    pub specular: PIELIGHT,
}
static mut bClipSpecular: BOOL = 1 as libc::c_int;
static mut videoBufferWidth: UDWORD = 640 as libc::c_int as UDWORD;
static mut videoBufferHeight: UDWORD = 480 as libc::c_int as UDWORD;
#[no_mangle]
pub unsafe extern "C" fn pie_SetVideoBuffer(mut width: UDWORD,
                                            mut height: UDWORD) -> BOOL {
    videoBufferWidth = width;
    videoBufferHeight = height;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn pie_GetVideoBufferWidth() -> UDWORD {
    return videoBufferWidth;
}
#[no_mangle]
pub unsafe extern "C" fn pie_GetVideoBufferHeight() -> UDWORD {
    return videoBufferHeight;
}
#[no_mangle]
pub unsafe extern "C" fn pie_Set2DClip(mut x0: libc::c_int,
                                       mut y0: libc::c_int,
                                       mut x1: libc::c_int,
                                       mut y1: libc::c_int) {
    (*psRendSurface).clip.left = x0;
    (*psRendSurface).clip.top = y0;
    (*psRendSurface).clip.right = x1;
    (*psRendSurface).clip.bottom = y1;
}
unsafe extern "C" fn pie_ClipXT(mut s1: *mut PIEVERTEX,
                                mut s2: *mut PIEVERTEX,
                                mut clip: *mut PIEVERTEX) -> libc::c_int {
    let mut n: libc::c_int = 0;
    let mut dx: libc::c_int = 0;
    let mut t: int32 = 0;
    n = 1 as libc::c_int;
    if (*s2).sx >= (*s1).sx {
        if (*s1).sx < (*psRendSurface).clip.left {
            if (*s2).sx <= (*psRendSurface).clip.left {
                return 0 as libc::c_int
            }
            dx = (*s2).sx - (*s1).sx;
            if dx != 0 as libc::c_int {
                (*clip).sy =
                    (*s1).sy +
                        ((*s2).sy - (*s1).sy) *
                            ((*psRendSurface).clip.left - (*s1).sx) / dx
            } else { (*clip).sy = (*s1).sy }
            (*clip).sx = (*psRendSurface).clip.left;
            // clip uv
            t = ((*clip).sx - (*s1).sx << 15 as libc::c_int) / dx;
            (*clip).tu =
                ((*s1).tu as libc::c_int +
                     (t * ((*s2).tu as libc::c_int - (*s1).tu as libc::c_int)
                          >> 15 as libc::c_int)) as UWORD;
            (*clip).tv =
                ((*s1).tv as libc::c_int +
                     (t * ((*s2).tv as libc::c_int - (*s1).tv as libc::c_int)
                          >> 15 as libc::c_int)) as UWORD;
            (*clip).sz =
                (*s1).sz + (t * ((*s2).sz - (*s1).sz) >> 15 as libc::c_int);
            (*clip).light.byte.r =
                ((*s1).light.byte.r as libc::c_int +
                     (t *
                          ((*s2).light.byte.r as libc::c_int -
                               (*s1).light.byte.r as libc::c_int) >>
                          15 as libc::c_int)) as UBYTE;
            (*clip).light.byte.g =
                ((*s1).light.byte.g as libc::c_int +
                     (t *
                          ((*s2).light.byte.g as libc::c_int -
                               (*s1).light.byte.g as libc::c_int) >>
                          15 as libc::c_int)) as UBYTE;
            (*clip).light.byte.b =
                ((*s1).light.byte.b as libc::c_int +
                     (t *
                          ((*s2).light.byte.b as libc::c_int -
                               (*s1).light.byte.b as libc::c_int) >>
                          15 as libc::c_int)) as UBYTE;
            (*clip).light.byte.a =
                ((*s1).light.byte.a as libc::c_int +
                     (t *
                          ((*s2).light.byte.a as libc::c_int -
                               (*s1).light.byte.a as libc::c_int) >>
                          15 as libc::c_int)) as UBYTE;
            if bClipSpecular != 0 {
                (*clip).specular.byte.r =
                    ((*s1).specular.byte.r as libc::c_int +
                         (t *
                              ((*s2).specular.byte.r as libc::c_int -
                                   (*s1).specular.byte.r as libc::c_int) >>
                              15 as libc::c_int)) as UBYTE;
                (*clip).specular.byte.g =
                    ((*s1).specular.byte.g as libc::c_int +
                         (t *
                              ((*s2).specular.byte.g as libc::c_int -
                                   (*s1).specular.byte.g as libc::c_int) >>
                              15 as libc::c_int)) as UBYTE;
                (*clip).specular.byte.b =
                    ((*s1).specular.byte.b as libc::c_int +
                         (t *
                              ((*s2).specular.byte.b as libc::c_int -
                                   (*s1).specular.byte.b as libc::c_int) >>
                              15 as libc::c_int)) as UBYTE;
                (*clip).specular.byte.a =
                    ((*s1).specular.byte.a as libc::c_int +
                         (t *
                              ((*s2).specular.byte.a as libc::c_int -
                                   (*s1).specular.byte.a as libc::c_int) >>
                              15 as libc::c_int)) as UBYTE
            }
        } else { *clip = *s1 }
        if (*s2).sx > (*psRendSurface).clip.right {
            if (*s1).sx > (*psRendSurface).clip.right {
                return 0 as libc::c_int
            }
            clip = clip.offset(1);
            dx = (*s2).sx - (*s1).sx;
            if dx != 0 as libc::c_int {
                (*clip).sy =
                    (*s2).sy -
                        ((*s2).sy - (*s1).sy) *
                            ((*s2).sx - (*psRendSurface).clip.right) / dx
            } else { (*clip).sy = (*s2).sy }
            (*clip).sx = (*psRendSurface).clip.right;
            // clip uv
            t = ((*clip).sx - (*s1).sx << 15 as libc::c_int) / dx;
            (*clip).tu =
                ((*s1).tu as libc::c_int +
                     (t * ((*s2).tu as libc::c_int - (*s1).tu as libc::c_int)
                          >> 15 as libc::c_int)) as UWORD;
            (*clip).tv =
                ((*s1).tv as libc::c_int +
                     (t * ((*s2).tv as libc::c_int - (*s1).tv as libc::c_int)
                          >> 15 as libc::c_int)) as UWORD;
            (*clip).sz =
                (*s1).sz + (t * ((*s2).sz - (*s1).sz) >> 15 as libc::c_int);
            (*clip).light.byte.r =
                ((*s1).light.byte.r as libc::c_int +
                     (t *
                          ((*s2).light.byte.r as libc::c_int -
                               (*s1).light.byte.r as libc::c_int) >>
                          15 as libc::c_int)) as UBYTE;
            (*clip).light.byte.g =
                ((*s1).light.byte.g as libc::c_int +
                     (t *
                          ((*s2).light.byte.g as libc::c_int -
                               (*s1).light.byte.g as libc::c_int) >>
                          15 as libc::c_int)) as UBYTE;
            (*clip).light.byte.b =
                ((*s1).light.byte.b as libc::c_int +
                     (t *
                          ((*s2).light.byte.b as libc::c_int -
                               (*s1).light.byte.b as libc::c_int) >>
                          15 as libc::c_int)) as UBYTE;
            (*clip).light.byte.a =
                ((*s1).light.byte.a as libc::c_int +
                     (t *
                          ((*s2).light.byte.a as libc::c_int -
                               (*s1).light.byte.a as libc::c_int) >>
                          15 as libc::c_int)) as UBYTE;
            if bClipSpecular != 0 {
                (*clip).specular.byte.r =
                    ((*s1).specular.byte.r as libc::c_int +
                         (t *
                              ((*s2).specular.byte.r as libc::c_int -
                                   (*s1).specular.byte.r as libc::c_int) >>
                              15 as libc::c_int)) as UBYTE;
                (*clip).specular.byte.g =
                    ((*s1).specular.byte.g as libc::c_int +
                         (t *
                              ((*s2).specular.byte.g as libc::c_int -
                                   (*s1).specular.byte.g as libc::c_int) >>
                              15 as libc::c_int)) as UBYTE;
                (*clip).specular.byte.b =
                    ((*s1).specular.byte.b as libc::c_int +
                         (t *
                              ((*s2).specular.byte.b as libc::c_int -
                                   (*s1).specular.byte.b as libc::c_int) >>
                              15 as libc::c_int)) as UBYTE;
                (*clip).specular.byte.a =
                    ((*s1).specular.byte.a as libc::c_int +
                         (t *
                              ((*s2).specular.byte.a as libc::c_int -
                                   (*s1).specular.byte.a as libc::c_int) >>
                              15 as libc::c_int)) as UBYTE
            }
            n = 2 as libc::c_int
        }
        return n
    } else {
        if (*s1).sx > (*psRendSurface).clip.right {
            if (*s2).sx >= (*psRendSurface).clip.right {
                return 0 as libc::c_int
            }
            dx = (*s1).sx - (*s2).sx;
            if dx != 0 as libc::c_int {
                (*clip).sy =
                    (*s1).sy -
                        ((*s1).sy - (*s2).sy) *
                            ((*s1).sx - (*psRendSurface).clip.right) / dx
            } else { (*clip).sy = (*s1).sy }
            (*clip).sx = (*psRendSurface).clip.right;
            // clip uv
            t = ((*clip).sx - (*s1).sx << 15 as libc::c_int) / dx;
            (*clip).tu =
                ((*s1).tu as libc::c_int +
                     (t * ((*s1).tu as libc::c_int - (*s2).tu as libc::c_int)
                          >> 15 as libc::c_int)) as UWORD;
            (*clip).tv =
                ((*s1).tv as libc::c_int +
                     (t * ((*s1).tv as libc::c_int - (*s2).tv as libc::c_int)
                          >> 15 as libc::c_int)) as UWORD;
            (*clip).sz =
                (*s1).sz + (t * ((*s1).sz - (*s2).sz) >> 15 as libc::c_int);
            (*clip).light.byte.r =
                ((*s1).light.byte.r as libc::c_int +
                     (t *
                          ((*s1).light.byte.r as libc::c_int -
                               (*s2).light.byte.r as libc::c_int) >>
                          15 as libc::c_int)) as UBYTE;
            (*clip).light.byte.g =
                ((*s1).light.byte.g as libc::c_int +
                     (t *
                          ((*s1).light.byte.g as libc::c_int -
                               (*s2).light.byte.g as libc::c_int) >>
                          15 as libc::c_int)) as UBYTE;
            (*clip).light.byte.b =
                ((*s1).light.byte.b as libc::c_int +
                     (t *
                          ((*s1).light.byte.b as libc::c_int -
                               (*s2).light.byte.b as libc::c_int) >>
                          15 as libc::c_int)) as UBYTE;
            (*clip).light.byte.a =
                ((*s1).light.byte.a as libc::c_int +
                     (t *
                          ((*s1).light.byte.a as libc::c_int -
                               (*s2).light.byte.a as libc::c_int) >>
                          15 as libc::c_int)) as UBYTE;
            if bClipSpecular != 0 {
                (*clip).specular.byte.r =
                    ((*s1).specular.byte.r as libc::c_int +
                         (t *
                              ((*s1).specular.byte.r as libc::c_int -
                                   (*s2).specular.byte.r as libc::c_int) >>
                              15 as libc::c_int)) as UBYTE;
                (*clip).specular.byte.g =
                    ((*s1).specular.byte.g as libc::c_int +
                         (t *
                              ((*s1).specular.byte.g as libc::c_int -
                                   (*s2).specular.byte.g as libc::c_int) >>
                              15 as libc::c_int)) as UBYTE;
                (*clip).specular.byte.b =
                    ((*s1).specular.byte.b as libc::c_int +
                         (t *
                              ((*s1).specular.byte.b as libc::c_int -
                                   (*s2).specular.byte.b as libc::c_int) >>
                              15 as libc::c_int)) as UBYTE;
                (*clip).specular.byte.a =
                    ((*s1).specular.byte.a as libc::c_int +
                         (t *
                              ((*s1).specular.byte.a as libc::c_int -
                                   (*s2).specular.byte.a as libc::c_int) >>
                              15 as libc::c_int)) as UBYTE
            }
        } else { *clip = *s1 }
        if (*s2).sx < (*psRendSurface).clip.left {
            if (*s1).sx < (*psRendSurface).clip.left {
                return 0 as libc::c_int
            }
            clip = clip.offset(1);
            dx = (*s1).sx - (*s2).sx;
            if dx != 0 as libc::c_int {
                (*clip).sy =
                    (*s2).sy +
                        ((*s1).sy - (*s2).sy) *
                            ((*psRendSurface).clip.left - (*s2).sx) / dx
            } else { (*clip).sy = (*s2).sy }
            (*clip).sx = (*psRendSurface).clip.left;
            // clip uv
            t = ((*clip).sx - (*s1).sx << 15 as libc::c_int) / dx;
            (*clip).tu =
                ((*s1).tu as libc::c_int +
                     (t * ((*s1).tu as libc::c_int - (*s2).tu as libc::c_int)
                          >> 15 as libc::c_int)) as UWORD;
            (*clip).tv =
                ((*s1).tv as libc::c_int +
                     (t * ((*s1).tv as libc::c_int - (*s2).tv as libc::c_int)
                          >> 15 as libc::c_int)) as UWORD;
            (*clip).sz =
                (*s1).sz + (t * ((*s1).sz - (*s2).sz) >> 15 as libc::c_int);
            (*clip).light.byte.r =
                ((*s1).light.byte.r as libc::c_int +
                     (t *
                          ((*s1).light.byte.r as libc::c_int -
                               (*s2).light.byte.r as libc::c_int) >>
                          15 as libc::c_int)) as UBYTE;
            (*clip).light.byte.g =
                ((*s1).light.byte.g as libc::c_int +
                     (t *
                          ((*s1).light.byte.g as libc::c_int -
                               (*s2).light.byte.g as libc::c_int) >>
                          15 as libc::c_int)) as UBYTE;
            (*clip).light.byte.b =
                ((*s1).light.byte.b as libc::c_int +
                     (t *
                          ((*s1).light.byte.b as libc::c_int -
                               (*s2).light.byte.b as libc::c_int) >>
                          15 as libc::c_int)) as UBYTE;
            (*clip).light.byte.a =
                ((*s1).light.byte.a as libc::c_int +
                     (t *
                          ((*s1).light.byte.a as libc::c_int -
                               (*s2).light.byte.a as libc::c_int) >>
                          15 as libc::c_int)) as UBYTE;
            if bClipSpecular != 0 {
                (*clip).specular.byte.r =
                    ((*s1).specular.byte.r as libc::c_int +
                         (t *
                              ((*s1).specular.byte.r as libc::c_int -
                                   (*s2).specular.byte.r as libc::c_int) >>
                              15 as libc::c_int)) as UBYTE;
                (*clip).specular.byte.g =
                    ((*s1).specular.byte.g as libc::c_int +
                         (t *
                              ((*s1).specular.byte.g as libc::c_int -
                                   (*s2).specular.byte.g as libc::c_int) >>
                              15 as libc::c_int)) as UBYTE;
                (*clip).specular.byte.b =
                    ((*s1).specular.byte.b as libc::c_int +
                         (t *
                              ((*s1).specular.byte.b as libc::c_int -
                                   (*s2).specular.byte.b as libc::c_int) >>
                              15 as libc::c_int)) as UBYTE;
                (*clip).specular.byte.a =
                    ((*s1).specular.byte.a as libc::c_int +
                         (t *
                              ((*s1).specular.byte.a as libc::c_int -
                                   (*s2).specular.byte.a as libc::c_int) >>
                              15 as libc::c_int)) as UBYTE
            }
            n = 2 as libc::c_int
        }
        return n
    };
}
unsafe extern "C" fn pie_ClipYT(mut s1: *mut PIEVERTEX,
                                mut s2: *mut PIEVERTEX,
                                mut clip: *mut PIEVERTEX) -> libc::c_int {
    let mut n: libc::c_int = 0;
    let mut dy: libc::c_int = 0;
    let mut t: int32 = 0;
    n = 1 as libc::c_int;
    if (*s2).sy >= (*s1).sy {
        if (*s1).sy < (*psRendSurface).clip.top {
            if (*s2).sy <= (*psRendSurface).clip.top {
                return 0 as libc::c_int
            }
            dy = (*s2).sy - (*s1).sy;
            if dy != 0 as libc::c_int {
                (*clip).sx =
                    (*s1).sx +
                        ((*s2).sx - (*s1).sx) *
                            ((*psRendSurface).clip.top - (*s1).sy) / dy
            } else { (*clip).sx = (*s1).sx }
            (*clip).sy = (*psRendSurface).clip.top;
            // clip uv
            t = ((*clip).sy - (*s1).sy << 15 as libc::c_int) / dy;
            (*clip).tu =
                ((*s1).tu as libc::c_int +
                     (t * ((*s2).tu as libc::c_int - (*s1).tu as libc::c_int)
                          >> 15 as libc::c_int)) as UWORD;
            (*clip).tv =
                ((*s1).tv as libc::c_int +
                     (t * ((*s2).tv as libc::c_int - (*s1).tv as libc::c_int)
                          >> 15 as libc::c_int)) as UWORD;
            (*clip).sz =
                (*s1).sz + (t * ((*s2).sz - (*s1).sz) >> 15 as libc::c_int);
            (*clip).light.byte.r =
                ((*s1).light.byte.r as libc::c_int +
                     (t *
                          ((*s2).light.byte.r as libc::c_int -
                               (*s1).light.byte.r as libc::c_int) >>
                          15 as libc::c_int)) as UBYTE;
            (*clip).light.byte.g =
                ((*s1).light.byte.g as libc::c_int +
                     (t *
                          ((*s2).light.byte.g as libc::c_int -
                               (*s1).light.byte.g as libc::c_int) >>
                          15 as libc::c_int)) as UBYTE;
            (*clip).light.byte.b =
                ((*s1).light.byte.b as libc::c_int +
                     (t *
                          ((*s2).light.byte.b as libc::c_int -
                               (*s1).light.byte.b as libc::c_int) >>
                          15 as libc::c_int)) as UBYTE;
            (*clip).light.byte.a =
                ((*s1).light.byte.a as libc::c_int +
                     (t *
                          ((*s2).light.byte.a as libc::c_int -
                               (*s1).light.byte.a as libc::c_int) >>
                          15 as libc::c_int)) as UBYTE;
            if bClipSpecular != 0 {
                (*clip).specular.byte.r =
                    ((*s1).specular.byte.r as libc::c_int +
                         (t *
                              ((*s2).specular.byte.r as libc::c_int -
                                   (*s1).specular.byte.r as libc::c_int) >>
                              15 as libc::c_int)) as UBYTE;
                (*clip).specular.byte.g =
                    ((*s1).specular.byte.g as libc::c_int +
                         (t *
                              ((*s2).specular.byte.g as libc::c_int -
                                   (*s1).specular.byte.g as libc::c_int) >>
                              15 as libc::c_int)) as UBYTE;
                (*clip).specular.byte.b =
                    ((*s1).specular.byte.b as libc::c_int +
                         (t *
                              ((*s2).specular.byte.b as libc::c_int -
                                   (*s1).specular.byte.b as libc::c_int) >>
                              15 as libc::c_int)) as UBYTE;
                (*clip).specular.byte.a =
                    ((*s1).specular.byte.a as libc::c_int +
                         (t *
                              ((*s2).specular.byte.a as libc::c_int -
                                   (*s1).specular.byte.a as libc::c_int) >>
                              15 as libc::c_int)) as UBYTE
            }
        } else { *clip = *s1 }
        if (*s2).sy > (*psRendSurface).clip.bottom {
            if (*s1).sy > (*psRendSurface).clip.bottom {
                return 0 as libc::c_int
            }
            clip = clip.offset(1);
            dy = (*s2).sy - (*s1).sy;
            if dy != 0 as libc::c_int {
                (*clip).sx =
                    (*s2).sx -
                        ((*s2).sx - (*s1).sx) *
                            ((*s2).sy - (*psRendSurface).clip.bottom) / dy
            } else { (*clip).sx = (*s2).sx }
            (*clip).sy = (*psRendSurface).clip.bottom;
            // clip uv
            t = ((*clip).sy - (*s1).sy << 15 as libc::c_int) / dy;
            (*clip).tu =
                ((*s1).tu as libc::c_int +
                     (t * ((*s2).tu as libc::c_int - (*s1).tu as libc::c_int)
                          >> 15 as libc::c_int)) as UWORD;
            (*clip).tv =
                ((*s1).tv as libc::c_int +
                     (t * ((*s2).tv as libc::c_int - (*s1).tv as libc::c_int)
                          >> 15 as libc::c_int)) as UWORD;
            (*clip).sz =
                (*s1).sz + (t * ((*s2).sz - (*s1).sz) >> 15 as libc::c_int);
            (*clip).light.byte.r =
                ((*s1).light.byte.r as libc::c_int +
                     (t *
                          ((*s2).light.byte.r as libc::c_int -
                               (*s1).light.byte.r as libc::c_int) >>
                          15 as libc::c_int)) as UBYTE;
            (*clip).light.byte.g =
                ((*s1).light.byte.g as libc::c_int +
                     (t *
                          ((*s2).light.byte.g as libc::c_int -
                               (*s1).light.byte.g as libc::c_int) >>
                          15 as libc::c_int)) as UBYTE;
            (*clip).light.byte.b =
                ((*s1).light.byte.b as libc::c_int +
                     (t *
                          ((*s2).light.byte.b as libc::c_int -
                               (*s1).light.byte.b as libc::c_int) >>
                          15 as libc::c_int)) as UBYTE;
            (*clip).light.byte.a =
                ((*s1).light.byte.a as libc::c_int +
                     (t *
                          ((*s2).light.byte.a as libc::c_int -
                               (*s1).light.byte.a as libc::c_int) >>
                          15 as libc::c_int)) as UBYTE;
            if bClipSpecular != 0 {
                (*clip).specular.byte.r =
                    ((*s1).specular.byte.r as libc::c_int +
                         (t *
                              ((*s2).specular.byte.r as libc::c_int -
                                   (*s1).specular.byte.r as libc::c_int) >>
                              15 as libc::c_int)) as UBYTE;
                (*clip).specular.byte.g =
                    ((*s1).specular.byte.g as libc::c_int +
                         (t *
                              ((*s2).specular.byte.g as libc::c_int -
                                   (*s1).specular.byte.g as libc::c_int) >>
                              15 as libc::c_int)) as UBYTE;
                (*clip).specular.byte.b =
                    ((*s1).specular.byte.b as libc::c_int +
                         (t *
                              ((*s2).specular.byte.b as libc::c_int -
                                   (*s1).specular.byte.b as libc::c_int) >>
                              15 as libc::c_int)) as UBYTE;
                (*clip).specular.byte.a =
                    ((*s1).specular.byte.a as libc::c_int +
                         (t *
                              ((*s2).specular.byte.a as libc::c_int -
                                   (*s1).specular.byte.a as libc::c_int) >>
                              15 as libc::c_int)) as UBYTE
            }
            n = 2 as libc::c_int
        }
        return n
    } else {
        if (*s1).sy > (*psRendSurface).clip.bottom {
            if (*s2).sy >= (*psRendSurface).clip.bottom {
                return 0 as libc::c_int
            }
            dy = (*s1).sy - (*s2).sy;
            if dy != 0 as libc::c_int {
                (*clip).sx =
                    (*s1).sx -
                        ((*s1).sx - (*s2).sx) *
                            ((*s1).sy - (*psRendSurface).clip.bottom) / dy
            } else { (*clip).sx = (*s1).sx }
            (*clip).sy = (*psRendSurface).clip.bottom;
            // clip uv
            t = ((*clip).sy - (*s1).sy << 15 as libc::c_int) / dy;
            (*clip).tu =
                ((*s1).tu as libc::c_int +
                     (t * ((*s1).tu as libc::c_int - (*s2).tu as libc::c_int)
                          >> 15 as libc::c_int)) as UWORD;
            (*clip).tv =
                ((*s1).tv as libc::c_int +
                     (t * ((*s1).tv as libc::c_int - (*s2).tv as libc::c_int)
                          >> 15 as libc::c_int)) as UWORD;
            (*clip).sz =
                (*s1).sz + (t * ((*s1).sz - (*s2).sz) >> 15 as libc::c_int);
            (*clip).light.byte.r =
                ((*s1).light.byte.r as libc::c_int +
                     (t *
                          ((*s1).light.byte.r as libc::c_int -
                               (*s2).light.byte.r as libc::c_int) >>
                          15 as libc::c_int)) as UBYTE;
            (*clip).light.byte.g =
                ((*s1).light.byte.g as libc::c_int +
                     (t *
                          ((*s1).light.byte.g as libc::c_int -
                               (*s2).light.byte.g as libc::c_int) >>
                          15 as libc::c_int)) as UBYTE;
            (*clip).light.byte.b =
                ((*s1).light.byte.b as libc::c_int +
                     (t *
                          ((*s1).light.byte.b as libc::c_int -
                               (*s2).light.byte.b as libc::c_int) >>
                          15 as libc::c_int)) as UBYTE;
            (*clip).light.byte.a =
                ((*s1).light.byte.a as libc::c_int +
                     (t *
                          ((*s1).light.byte.a as libc::c_int -
                               (*s2).light.byte.a as libc::c_int) >>
                          15 as libc::c_int)) as UBYTE;
            if bClipSpecular != 0 {
                (*clip).specular.byte.r =
                    ((*s1).specular.byte.r as libc::c_int +
                         (t *
                              ((*s1).specular.byte.r as libc::c_int -
                                   (*s2).specular.byte.r as libc::c_int) >>
                              15 as libc::c_int)) as UBYTE;
                (*clip).specular.byte.g =
                    ((*s1).specular.byte.g as libc::c_int +
                         (t *
                              ((*s1).specular.byte.g as libc::c_int -
                                   (*s2).specular.byte.g as libc::c_int) >>
                              15 as libc::c_int)) as UBYTE;
                (*clip).specular.byte.b =
                    ((*s1).specular.byte.b as libc::c_int +
                         (t *
                              ((*s1).specular.byte.b as libc::c_int -
                                   (*s2).specular.byte.b as libc::c_int) >>
                              15 as libc::c_int)) as UBYTE;
                (*clip).specular.byte.a =
                    ((*s1).specular.byte.a as libc::c_int +
                         (t *
                              ((*s1).specular.byte.a as libc::c_int -
                                   (*s2).specular.byte.a as libc::c_int) >>
                              15 as libc::c_int)) as UBYTE
            }
        } else { *clip = *s1 }
        if (*s2).sy < (*psRendSurface).clip.top {
            if (*s1).sy < (*psRendSurface).clip.top {
                return 0 as libc::c_int
            }
            clip = clip.offset(1);
            dy = (*s1).sy - (*s2).sy;
            if dy != 0 as libc::c_int {
                (*clip).sx =
                    (*s2).sx +
                        ((*s1).sx - (*s2).sx) *
                            ((*psRendSurface).clip.top - (*s2).sy) / dy
            } else { (*clip).sx = (*s2).sx }
            (*clip).sy = (*psRendSurface).clip.top;
            // clip uv
            t = ((*clip).sy - (*s1).sy << 15 as libc::c_int) / dy;
            (*clip).tu =
                ((*s1).tu as libc::c_int +
                     (t * ((*s1).tu as libc::c_int - (*s2).tu as libc::c_int)
                          >> 15 as libc::c_int)) as UWORD;
            (*clip).tv =
                ((*s1).tv as libc::c_int +
                     (t * ((*s1).tv as libc::c_int - (*s2).tv as libc::c_int)
                          >> 15 as libc::c_int)) as UWORD;
            (*clip).sz =
                (*s1).sz + (t * ((*s1).sz - (*s2).sz) >> 15 as libc::c_int);
            (*clip).light.byte.r =
                ((*s1).light.byte.r as libc::c_int +
                     (t *
                          ((*s1).light.byte.r as libc::c_int -
                               (*s2).light.byte.r as libc::c_int) >>
                          15 as libc::c_int)) as UBYTE;
            (*clip).light.byte.g =
                ((*s1).light.byte.g as libc::c_int +
                     (t *
                          ((*s1).light.byte.g as libc::c_int -
                               (*s2).light.byte.g as libc::c_int) >>
                          15 as libc::c_int)) as UBYTE;
            (*clip).light.byte.b =
                ((*s1).light.byte.b as libc::c_int +
                     (t *
                          ((*s1).light.byte.b as libc::c_int -
                               (*s2).light.byte.b as libc::c_int) >>
                          15 as libc::c_int)) as UBYTE;
            (*clip).light.byte.a =
                ((*s1).light.byte.a as libc::c_int +
                     (t *
                          ((*s1).light.byte.a as libc::c_int -
                               (*s2).light.byte.a as libc::c_int) >>
                          15 as libc::c_int)) as UBYTE;
            if bClipSpecular != 0 {
                (*clip).specular.byte.r =
                    ((*s1).specular.byte.r as libc::c_int +
                         (t *
                              ((*s1).specular.byte.r as libc::c_int -
                                   (*s2).specular.byte.r as libc::c_int) >>
                              15 as libc::c_int)) as UBYTE;
                (*clip).specular.byte.g =
                    ((*s1).specular.byte.g as libc::c_int +
                         (t *
                              ((*s1).specular.byte.g as libc::c_int -
                                   (*s2).specular.byte.g as libc::c_int) >>
                              15 as libc::c_int)) as UBYTE;
                (*clip).specular.byte.b =
                    ((*s1).specular.byte.b as libc::c_int +
                         (t *
                              ((*s1).specular.byte.b as libc::c_int -
                                   (*s2).specular.byte.b as libc::c_int) >>
                              15 as libc::c_int)) as UBYTE;
                (*clip).specular.byte.a =
                    ((*s1).specular.byte.a as libc::c_int +
                         (t *
                              ((*s1).specular.byte.a as libc::c_int -
                                   (*s2).specular.byte.a as libc::c_int) >>
                              15 as libc::c_int)) as UBYTE
            }
            n = 2 as libc::c_int
        }
        return n
    };
}
#[no_mangle]
pub unsafe extern "C" fn pie_ClipTextured(mut npoints: libc::c_int,
                                          mut points: *mut PIEVERTEX,
                                          mut clip: *mut PIEVERTEX,
                                          mut bSpecular: BOOL)
 -> libc::c_int {
    static mut xclip: [PIEVERTEX; 20] =
        [PIEVERTEX{sx: 0,
                   sy: 0,
                   sz: 0,
                   tu: 0,
                   tv: 0,
                   light:
                       PIELIGHT{byte:
                                    PIELIGHTBYTES{b: 0, g: 0, r: 0, a: 0,},},
                   specular:
                       PIELIGHT{byte:
                                    PIELIGHTBYTES{b: 0,
                                                  g: 0,
                                                  r: 0,
                                                  a: 0,},},}; 20];
    let mut p0: *mut PIEVERTEX = 0 as *mut PIEVERTEX;
    let mut p1: *mut PIEVERTEX = 0 as *mut PIEVERTEX;
    let mut n1: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    bClipSpecular = bSpecular;
    p0 = &mut *points.offset(0 as libc::c_int as isize) as *mut PIEVERTEX;
    p1 = &mut *points.offset(1 as libc::c_int as isize) as *mut PIEVERTEX;
    i = 0 as libc::c_int;
    n1 = 0 as libc::c_int;
    while i < npoints {
        if i == npoints - 1 as libc::c_int {
            p1 =
                &mut *points.offset(0 as libc::c_int as isize) as
                    *mut PIEVERTEX
        }
        if (*p0).sx == (1 as libc::c_int) << 15 as libc::c_int ||
               (*p0).sy == (-(1 as libc::c_int)) << 15 as libc::c_int {
            //check for invalid points jps19aug97
            return 0 as libc::c_int
        }
        n1 +=
            pie_ClipXT(p0, p1, &mut *xclip.as_mut_ptr().offset(n1 as isize));
        i += 1;
        p0 = p0.offset(1);
        p1 = p1.offset(1)
    }
    p0 =
        &mut *xclip.as_mut_ptr().offset(0 as libc::c_int as isize) as
            *mut PIEVERTEX;
    p1 =
        &mut *xclip.as_mut_ptr().offset(1 as libc::c_int as isize) as
            *mut PIEVERTEX;
    i = 0 as libc::c_int;
    n = 0 as libc::c_int;
    while i < n1 {
        if i == n1 - 1 as libc::c_int {
            p1 =
                &mut *xclip.as_mut_ptr().offset(0 as libc::c_int as isize) as
                    *mut PIEVERTEX
        }
        n += pie_ClipYT(p0, p1, &mut *clip.offset(n as isize));
        p0 = p0.offset(1);
        p1 = p1.offset(1);
        i += 1
    }
    return n;
}
/* **************************************************************************/
/*
 * pieclip.h
 *
 * clipping for all pumpkin image library functions.
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
 *	Global ProtoTypes
 */
/* **************************************************************************/
//*************************************************************************
/* Alex - much faster tri clipper - won't clip owt else tho' */
#[no_mangle]
pub unsafe extern "C" fn pie_ClipTexturedTriangleFast(mut v1: *mut PIEVERTEX,
                                                      mut v2: *mut PIEVERTEX,
                                                      mut v3: *mut PIEVERTEX,
                                                      mut clipped:
                                                          *mut PIEVERTEX,
                                                      mut bSpecular: BOOL)
 -> libc::c_int {
    static mut xClip: [PIEVERTEX; 20] =
        [PIEVERTEX{sx: 0,
                   sy: 0,
                   sz: 0,
                   tu: 0,
                   tv: 0,
                   light:
                       PIELIGHT{byte:
                                    PIELIGHTBYTES{b: 0, g: 0, r: 0, a: 0,},},
                   specular:
                       PIELIGHT{byte:
                                    PIELIGHTBYTES{b: 0,
                                                  g: 0,
                                                  r: 0,
                                                  a: 0,},},};
            20]; // plus 4 hopefully is limit?
    static mut p0: *mut PIEVERTEX = 0 as *const PIEVERTEX as *mut PIEVERTEX;
    static mut p1: *mut PIEVERTEX = 0 as *const PIEVERTEX as *mut PIEVERTEX;
    let mut numPreY: UDWORD = 0;
    let mut numAll: UDWORD = 0;
    let mut i: UDWORD = 0;
    bClipSpecular = bSpecular;
    numPreY = 0 as libc::c_int as UDWORD;
    if (*v1).sx > (1 as libc::c_int) << 14 as libc::c_int ||
           (*v1).sy > (1 as libc::c_int) << 14 as libc::c_int {
        /* bomb out for out of range points */
        return 0 as libc::c_int
    }
    numPreY =
        (numPreY as
             libc::c_uint).wrapping_add(pie_ClipXT(v1, v2,
                                                   &mut *xClip.as_mut_ptr().offset(numPreY
                                                                                       as
                                                                                       isize))
                                            as libc::c_uint) as UDWORD as
            UDWORD;
    if (*v2).sx > (1 as libc::c_int) << 14 as libc::c_int ||
           (*v2).sy > (1 as libc::c_int) << 14 as libc::c_int {
        /* bomb out for out of range points */
        return 0 as libc::c_int
    }
    numPreY =
        (numPreY as
             libc::c_uint).wrapping_add(pie_ClipXT(v2, v3,
                                                   &mut *xClip.as_mut_ptr().offset(numPreY
                                                                                       as
                                                                                       isize))
                                            as libc::c_uint) as UDWORD as
            UDWORD;
    if (*v3).sx > (1 as libc::c_int) << 14 as libc::c_int ||
           (*v3).sy > (1 as libc::c_int) << 14 as libc::c_int {
        /* bomb out for out of range points */
        return 0 as libc::c_int
    }
    numPreY =
        (numPreY as
             libc::c_uint).wrapping_add(pie_ClipXT(v3, v1,
                                                   &mut *xClip.as_mut_ptr().offset(numPreY
                                                                                       as
                                                                                       isize))
                                            as libc::c_uint) as UDWORD as
            UDWORD;
    /* We've now clipped against x axis - now for Y */
    p0 =
        &mut *xClip.as_mut_ptr().offset(0 as libc::c_int as isize) as
            *mut PIEVERTEX;
    p1 =
        &mut *xClip.as_mut_ptr().offset(1 as libc::c_int as isize) as
            *mut PIEVERTEX;
    i = 0 as libc::c_int as UDWORD;
    numAll = 0 as libc::c_int as UDWORD;
    while i < numPreY {
        if i == numPreY.wrapping_sub(1 as libc::c_int as libc::c_uint) {
            p1 =
                &mut *xClip.as_mut_ptr().offset(0 as libc::c_int as isize) as
                    *mut PIEVERTEX
        }
        numAll =
            (numAll as
                 libc::c_uint).wrapping_add(pie_ClipYT(p0, p1,
                                                       &mut *clipped.offset(numAll
                                                                                as
                                                                                isize))
                                                as libc::c_uint) as UDWORD as
                UDWORD;
        p0 = p0.offset(1);
        p1 = p1.offset(1);
        i = i.wrapping_add(1)
    }
    return numAll as libc::c_int;
}
