use ::libc;
extern "C" {
    #[no_mangle]
    fn glTranslatef(x: GLfloat, y: GLfloat, z: GLfloat);
    #[no_mangle]
    fn glScalef(x: GLfloat, y: GLfloat, z: GLfloat);
    #[no_mangle]
    fn glRotatef(angle: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);
    #[no_mangle]
    fn glMultMatrixf(m: *const GLfloat);
    #[no_mangle]
    fn glLoadIdentity();
    #[no_mangle]
    fn glPopMatrix();
    #[no_mangle]
    fn glPushMatrix();
    #[no_mangle]
    fn glFrustum(left: GLdouble, right: GLdouble, bottom: GLdouble,
                 top: GLdouble, near_val: GLdouble, far_val: GLdouble);
    #[no_mangle]
    fn glOrtho(left: GLdouble, right: GLdouble, bottom: GLdouble,
               top: GLdouble, near_val: GLdouble, far_val: GLdouble);
    #[no_mangle]
    fn glMatrixMode(mode: GLenum);
    #[no_mangle]
    fn glDepthRange(near_val: GLclampd, far_val: GLclampd);
    #[no_mangle]
    fn glGetFloatv(pname: GLenum, params: *mut GLfloat);
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
    fn sin(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn pie_GetVideoBufferWidth() -> UDWORD;
    #[no_mangle]
    fn pie_GetVideoBufferHeight() -> UDWORD;
    #[no_mangle]
    static mut psRendSurface: *mut iSurface;
}
pub type GLenum = libc::c_uint;
pub type GLfloat = libc::c_float;
pub type GLdouble = libc::c_double;
pub type GLclampd = libc::c_double;
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
pub type int32 = libc::c_int;
pub type uint8 = libc::c_uchar;
pub type uint16 = libc::c_ushort;
pub type uint32 = libc::c_uint;
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
pub struct iPoint {
    pub x: int32,
    pub y: int32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iVector {
    pub x: int32,
    pub y: int32,
    pub z: int32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iVertex {
    pub x: int32,
    pub y: int32,
    pub z: int32,
    pub u: int32,
    pub v: int32,
    pub g: uint8,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDMATRIX {
    pub a: SDWORD,
    pub b: SDWORD,
    pub c: SDWORD,
    pub d: SDWORD,
    pub e: SDWORD,
    pub f: SDWORD,
    pub g: SDWORD,
    pub h: SDWORD,
    pub i: SDWORD,
    pub j: SDWORD,
    pub k: SDWORD,
    pub l: SDWORD,
}
// 4096/100
static mut aMatrixStack: [SDMATRIX; 8] =
    [SDMATRIX{a: 0,
              b: 0,
              c: 0,
              d: 0,
              e: 0,
              f: 0,
              g: 0,
              h: 0,
              i: 0,
              j: 0,
              k: 0,
              l: 0,}; 8];
// Initialized in run_static_initializers
#[no_mangle]
pub static mut psMatrix: *mut SDMATRIX =
    0 as *const SDMATRIX as *mut SDMATRIX;
#[no_mangle]
pub static mut drawing_interface: BOOL = 1 as libc::c_int;
//*************************************************************************
#[no_mangle]
pub unsafe extern "C" fn pie_VectorNormalise(mut v: *mut iVector) {
    let mut size: int32 = 0;
    let mut av: iVector = iVector{x: 0, y: 0, z: 0,};
    av.x = if (*v).x < 0 as libc::c_int { -(*v).x } else { (*v).x };
    av.y = if (*v).y < 0 as libc::c_int { -(*v).y } else { (*v).y };
    av.z = if (*v).z < 0 as libc::c_int { -(*v).z } else { (*v).z };
    if av.x >= av.y {
        if av.x > av.z {
            size =
                av.x + (av.z >> 2 as libc::c_int) + (av.y >> 2 as libc::c_int)
        } else {
            size =
                av.z + (av.x >> 2 as libc::c_int) + (av.y >> 2 as libc::c_int)
        }
    } else if av.y > av.z {
        size = av.y + (av.z >> 2 as libc::c_int) + (av.x >> 2 as libc::c_int)
    } else {
        size = av.z + (av.y >> 2 as libc::c_int) + (av.x >> 2 as libc::c_int)
    }
    if size > 0 as libc::c_int {
        (*v).x = ((*v).x << 12 as libc::c_int) / size;
        (*v).y = ((*v).y << 12 as libc::c_int) / size;
        (*v).z = ((*v).z << 12 as libc::c_int) / size
    };
}
//*************************************************************************
//*** calculate surface normal
//*
//* params	p1,p2,p3	= points for forming 2 vector for cross product
//*			v			= normal vector returned << FP12_SHIFT
//*
//* eg		if a polygon (with n points in clockwise order) normal
//*			is required, p1 = point 0, p2 = point 1, p3 = point n-1
//*
//******
#[no_mangle]
pub unsafe extern "C" fn pie_SurfaceNormal(mut p1: *mut iVector,
                                           mut p2: *mut iVector,
                                           mut p3: *mut iVector,
                                           mut v: *mut iVector) {
    let mut a: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut b: iVector = iVector{x: 0, y: 0, z: 0,};
    a.x = (*p3).x - (*p1).x;
    a.y = (*p3).y - (*p1).y;
    a.z = (*p3).z - (*p1).z;
    pie_VectorNormalise(&mut a);
    b.x = (*p2).x - (*p1).x;
    b.y = (*p2).y - (*p1).y;
    b.z = (*p2).z - (*p1).z;
    pie_VectorNormalise(&mut b);
    (*v).x = a.y * b.z - a.z * b.y >> 12 as libc::c_int;
    (*v).y = a.z * b.x - a.x * b.z >> 12 as libc::c_int;
    (*v).z = a.x * b.y - a.y * b.x >> 12 as libc::c_int;
    pie_VectorNormalise(v);
}
//*************************************************************************
//*************************************************************************
static mut _MATRIX_ID: SDMATRIX =
    {
        let mut init =
            SDMATRIX{a: (1 as libc::c_int) << 12 as libc::c_int,
                     b: 0 as libc::c_int,
                     c: 0 as libc::c_int,
                     d: 0 as libc::c_int,
                     e: (1 as libc::c_int) << 12 as libc::c_int,
                     f: 0 as libc::c_int,
                     g: 0 as libc::c_int,
                     h: 0 as libc::c_int,
                     i: (1 as libc::c_int) << 12 as libc::c_int,
                     j: 0 as libc::c_long as SDWORD,
                     k: 0 as libc::c_long as SDWORD,
                     l: 0 as libc::c_long as SDWORD,};
        init
    };
static mut _MATRIX_INDEX: SDWORD = 0;
//*************************************************************************
#[no_mangle]
pub static mut aSinTable: [SDWORD; 5120] = [0; 5120];
//*************************************************************************
//*** reset transformation matrix stack and make current identity
//*
//******
#[no_mangle]
pub unsafe extern "C" fn pie_MatReset() {
    // printf("pie_MatReset\n");
    psMatrix =
        &mut *aMatrixStack.as_mut_ptr().offset(0 as libc::c_int as isize) as
            *mut SDMATRIX;
    // make 1st matrix identity
    *psMatrix = _MATRIX_ID;
    glLoadIdentity();
}
//*************************************************************************
//*************************************************************************
//*** create new matrix from current transformation matrix and make current
//*
//******
#[no_mangle]
pub unsafe extern "C" fn pie_MatBegin() {
    _MATRIX_INDEX += 1;
    if _MATRIX_INDEX > 3 as libc::c_int {
        if _MATRIX_INDEX < 8 as libc::c_int {
        } else {
            debug(LOG_ERROR,
                  b"pie_MatBegin past top of the stack\x00" as *const u8 as
                      *const libc::c_char);
        };
        if _MATRIX_INDEX < 8 as libc::c_int {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"piematrix.c\x00" as *const u8 as *const libc::c_char,
                  157 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 13],
                                            &[libc::c_char; 13]>(b"pie_MatBegin\x00")).as_ptr(),
                  b"_MATRIX_INDEX < MATRIX_MAX\x00" as *const u8 as
                      *const libc::c_char);
        };
    }
    psMatrix = psMatrix.offset(1);
    aMatrixStack[_MATRIX_INDEX as usize] =
        aMatrixStack[(_MATRIX_INDEX - 1 as libc::c_int) as usize];
    glPushMatrix();
}
//*************************************************************************
//*** make current transformation matrix previous one on stack
//*
//******
#[no_mangle]
pub unsafe extern "C" fn pie_MatEnd() {
    _MATRIX_INDEX -= 1;
    if _MATRIX_INDEX >= 0 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"pie_MatEnd of the bottom of the stack\x00" as *const u8 as
                  *const libc::c_char);
    };
    if _MATRIX_INDEX >= 0 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"piematrix.c\x00" as *const u8 as *const libc::c_char,
              175 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 11],
                                        &[libc::c_char; 11]>(b"pie_MatEnd\x00")).as_ptr(),
              b"_MATRIX_INDEX >= 0\x00" as *const u8 as *const libc::c_char);
    };
    psMatrix = psMatrix.offset(-1);
    glPopMatrix();
}
#[no_mangle]
pub unsafe extern "C" fn pie_MATTRANS(mut x: libc::c_int, mut y: libc::c_int,
                                      mut z: libc::c_int) {
    let mut matrix: [GLfloat; 16] = [0.; 16];
    (*psMatrix).j = x << 12 as libc::c_int;
    (*psMatrix).k = y << 12 as libc::c_int;
    (*psMatrix).l = z << 12 as libc::c_int;
    glGetFloatv(0xba6 as libc::c_int as GLenum, matrix.as_mut_ptr());
    matrix[12 as libc::c_int as usize] = x as GLfloat;
    matrix[13 as libc::c_int as usize] = y as GLfloat;
    matrix[14 as libc::c_int as usize] = z as GLfloat;
    glLoadIdentity();
    glMultMatrixf(matrix.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn pie_TRANSLATE(mut x: libc::c_int, mut y: libc::c_int,
                                       mut z: libc::c_int) {
    (*psMatrix).j +=
        x * (*psMatrix).a + y * (*psMatrix).d + z * (*psMatrix).g;
    (*psMatrix).k +=
        x * (*psMatrix).b + y * (*psMatrix).e + z * (*psMatrix).h;
    (*psMatrix).l +=
        x * (*psMatrix).c + y * (*psMatrix).f + z * (*psMatrix).i;
    glTranslatef(x as GLfloat, y as GLfloat, z as GLfloat);
}
//*************************************************************************
//*** matrix scale current transformation matrix
//*
//******
#[no_mangle]
pub unsafe extern "C" fn pie_MatScale(mut percent: UDWORD) {
    let mut scaleFactor: SDWORD = 0;
    if percent == 100 as libc::c_int as libc::c_uint { return }
    scaleFactor =
        percent.wrapping_mul(41 as libc::c_int as libc::c_uint) as SDWORD;
    (*psMatrix).a = (*psMatrix).a * scaleFactor / 4096 as libc::c_int;
    (*psMatrix).b = (*psMatrix).b * scaleFactor / 4096 as libc::c_int;
    (*psMatrix).c = (*psMatrix).c * scaleFactor / 4096 as libc::c_int;
    (*psMatrix).d = (*psMatrix).d * scaleFactor / 4096 as libc::c_int;
    (*psMatrix).e = (*psMatrix).e * scaleFactor / 4096 as libc::c_int;
    (*psMatrix).f = (*psMatrix).f * scaleFactor / 4096 as libc::c_int;
    (*psMatrix).g = (*psMatrix).g * scaleFactor / 4096 as libc::c_int;
    (*psMatrix).h = (*psMatrix).h * scaleFactor / 4096 as libc::c_int;
    (*psMatrix).i = (*psMatrix).i * scaleFactor / 4096 as libc::c_int;
    glScalef((0.01f64 * percent as libc::c_double) as GLfloat,
             (0.01f64 * percent as libc::c_double) as GLfloat,
             (0.01f64 * percent as libc::c_double) as GLfloat);
}
//*************************************************************************
//*** matrix rotate y (yaw) current transformation matrix
//*
//******
#[no_mangle]
pub unsafe extern "C" fn pie_MatRotY(mut y: libc::c_int) {
    // printf("pie_MatRotY %i\n", y);
    let mut t: libc::c_int = 0;
    let mut cra: libc::c_int = 0;
    let mut sra: libc::c_int = 0;
    if y != 0 as libc::c_int {
        cra =
            aSinTable[((y as uint16 as libc::c_int >> 4 as libc::c_int) +
                           1024 as libc::c_int) as usize];
        sra =
            aSinTable[(y as uint16 as libc::c_int >> 4 as libc::c_int) as
                          usize];
        t = cra * (*psMatrix).a - sra * (*psMatrix).g >> 12 as libc::c_int;
        (*psMatrix).g =
            sra * (*psMatrix).a + cra * (*psMatrix).g >> 12 as libc::c_int;
        (*psMatrix).a = t;
        t = cra * (*psMatrix).b - sra * (*psMatrix).h >> 12 as libc::c_int;
        (*psMatrix).h =
            sra * (*psMatrix).b + cra * (*psMatrix).h >> 12 as libc::c_int;
        (*psMatrix).b = t;
        t = cra * (*psMatrix).c - sra * (*psMatrix).i >> 12 as libc::c_int;
        (*psMatrix).i =
            sra * (*psMatrix).c + cra * (*psMatrix).i >> 12 as libc::c_int;
        (*psMatrix).c = t
    }
    glRotatef((y as libc::c_double * 22.5f64 / 4096.0f64) as GLfloat,
              0 as libc::c_int as GLfloat, 1 as libc::c_int as GLfloat,
              0 as libc::c_int as GLfloat);
}
//*************************************************************************
//*** matrix rotate z (roll) current transformation matrix
//*
//******
#[no_mangle]
pub unsafe extern "C" fn pie_MatRotZ(mut z: libc::c_int) {
    // printf("pie_MatRotZ %i\n", z);
    let mut t: libc::c_int = 0;
    let mut cra: libc::c_int = 0;
    let mut sra: libc::c_int = 0;
    if z != 0 as libc::c_int {
        cra =
            aSinTable[((z as uint16 as libc::c_int >> 4 as libc::c_int) +
                           1024 as libc::c_int) as usize];
        sra =
            aSinTable[(z as uint16 as libc::c_int >> 4 as libc::c_int) as
                          usize];
        t = cra * (*psMatrix).a + sra * (*psMatrix).d >> 12 as libc::c_int;
        (*psMatrix).d =
            cra * (*psMatrix).d - sra * (*psMatrix).a >> 12 as libc::c_int;
        (*psMatrix).a = t;
        t = cra * (*psMatrix).b + sra * (*psMatrix).e >> 12 as libc::c_int;
        (*psMatrix).e =
            cra * (*psMatrix).e - sra * (*psMatrix).b >> 12 as libc::c_int;
        (*psMatrix).b = t;
        t = cra * (*psMatrix).c + sra * (*psMatrix).f >> 12 as libc::c_int;
        (*psMatrix).f =
            cra * (*psMatrix).f - sra * (*psMatrix).c >> 12 as libc::c_int;
        (*psMatrix).c = t
    }
    glRotatef((z as libc::c_double * 22.5f64 / 4096.0f64) as GLfloat,
              0 as libc::c_int as GLfloat, 0 as libc::c_int as GLfloat,
              1 as libc::c_int as GLfloat);
}
//*************************************************************************
//*** matrix rotate x (pitch) current transformation matrix
//*
//******
#[no_mangle]
pub unsafe extern "C" fn pie_MatRotX(mut x: libc::c_int) {
    // printf("pie_MatRotX %i\n", x);
    let mut cra: libc::c_int = 0;
    let mut sra: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    if x != 0 as libc::c_int {
        cra =
            aSinTable[((x as uint16 as libc::c_int >> 4 as libc::c_int) +
                           1024 as libc::c_int) as usize];
        sra =
            aSinTable[(x as uint16 as libc::c_int >> 4 as libc::c_int) as
                          usize];
        t = cra * (*psMatrix).d + sra * (*psMatrix).g >> 12 as libc::c_int;
        (*psMatrix).g =
            cra * (*psMatrix).g - sra * (*psMatrix).d >> 12 as libc::c_int;
        (*psMatrix).d = t;
        t = cra * (*psMatrix).e + sra * (*psMatrix).h >> 12 as libc::c_int;
        (*psMatrix).h =
            cra * (*psMatrix).h - sra * (*psMatrix).e >> 12 as libc::c_int;
        (*psMatrix).e = t;
        t = cra * (*psMatrix).f + sra * (*psMatrix).i >> 12 as libc::c_int;
        (*psMatrix).i =
            cra * (*psMatrix).i - sra * (*psMatrix).f >> 12 as libc::c_int;
        (*psMatrix).f = t
    }
    glRotatef((x as libc::c_double * 22.5f64 / 4096.0f64) as GLfloat,
              1 as libc::c_int as GLfloat, 0 as libc::c_int as GLfloat,
              0 as libc::c_int as GLfloat);
}
//*************************************************************************
//*** 3D vector perspective projection
//*
//* params	v1 = 3D vector to project
//* 			v2 = pointer to 2D resultant vector
//*
//* on exit	v2 = projected vector
//*
//* returns	rotated and translated z component of v1
//*
//******
#[no_mangle]
pub unsafe extern "C" fn pie_RotateProject(mut x: SDWORD, mut y: SDWORD,
                                           mut z: SDWORD, mut xs: *mut SDWORD,
                                           mut ys: *mut SDWORD) -> int32 {
    let mut zfx: int32 = 0; //just along way off screen
    let mut zfy: int32 = 0; //just along way off screen
    let mut zz: int32 = 0;
    let mut _x: int32 = 0;
    let mut _y: int32 = 0;
    let mut _z: int32 = 0;
    _x =
        x * (*psMatrix).a + y * (*psMatrix).d + z * (*psMatrix).g +
            (*psMatrix).j;
    _y =
        x * (*psMatrix).b + y * (*psMatrix).e + z * (*psMatrix).h +
            (*psMatrix).k;
    _z =
        x * (*psMatrix).c + y * (*psMatrix).f + z * (*psMatrix).i +
            (*psMatrix).l;
    zz = _z >> 10 as libc::c_int;
    zfx = _z >> (*psRendSurface).xpshift;
    zfy = _z >> (*psRendSurface).ypshift;
    if zfx <= 0 as libc::c_int || zfy <= 0 as libc::c_int {
        *xs = (1 as libc::c_int) << 15 as libc::c_int;
        *ys = (1 as libc::c_int) << 15 as libc::c_int
    } else if zz < 256 as libc::c_int {
        *xs = (1 as libc::c_int) << 15 as libc::c_int;
        *ys = (1 as libc::c_int) << 15 as libc::c_int
    } else {
        *xs = (*psRendSurface).xcentre + _x / zfx;
        *ys = (*psRendSurface).ycentre - _y / zfy
    }
    return zz;
}
#[no_mangle]
pub unsafe extern "C" fn pie_RotProj(mut v3d: *mut iVector,
                                     mut v2d: *mut iPoint) -> int32 {
    return pie_RotateProject((*v3d).x, (*v3d).y, (*v3d).z, &mut (*v2d).x,
                             &mut (*v2d).y);
}
#[no_mangle]
pub unsafe extern "C" fn pie_Transform(mut v3d: *mut iVector,
                                       mut v2d: *mut iPoint) -> int32 {
    (*v2d).x = (*v3d).x;
    (*v2d).y = (*v3d).y;
    return (*v3d).z;
}
//*************************************************************************
//*************************************************************************
#[no_mangle]
pub unsafe extern "C" fn pie_PerspectiveBegin() {
    let mut width: libc::c_float = pie_GetVideoBufferWidth() as libc::c_float;
    let mut height: libc::c_float =
        pie_GetVideoBufferHeight() as libc::c_float;
    let mut xangle: libc::c_float = width / 6 as libc::c_int as libc::c_float;
    let mut yangle: libc::c_float =
        height / 6 as libc::c_int as libc::c_float;
    glMatrixMode(0x1701 as libc::c_int as GLenum);
    glLoadIdentity();
    glTranslatef(((2 as libc::c_int * (*psRendSurface).xcentre) as
                      libc::c_float - width) / width,
                 (height -
                      (2 as libc::c_int * (*psRendSurface).ycentre) as
                          libc::c_float) / height,
                 0 as libc::c_int as GLfloat);
    glFrustum(-xangle as GLdouble, xangle as GLdouble, -yangle as GLdouble,
              yangle as GLdouble, 330 as libc::c_int as GLdouble,
              100000 as libc::c_int as GLdouble);
    glScalef(1 as libc::c_int as GLfloat, 1 as libc::c_int as GLfloat,
             -(1 as libc::c_int) as GLfloat);
    glMatrixMode(0x1700 as libc::c_int as GLenum);
}
#[no_mangle]
pub unsafe extern "C" fn pie_PerspectiveEnd() {
    glMatrixMode(0x1701 as libc::c_int as GLenum);
    glLoadIdentity();
    glOrtho(0 as libc::c_int as GLdouble,
            pie_GetVideoBufferWidth() as GLdouble,
            pie_GetVideoBufferHeight() as GLdouble,
            0 as libc::c_int as GLdouble, 1 as libc::c_int as GLdouble,
            -(1 as libc::c_int) as GLdouble);
    glMatrixMode(0x1700 as libc::c_int as GLenum);
}
#[no_mangle]
pub unsafe extern "C" fn pie_Begin3DScene() {
    glDepthRange(0.1f64, 1 as libc::c_int as GLclampd);
    drawing_interface = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn pie_BeginInterface() {
    glDepthRange(0 as libc::c_int as GLclampd, 0.1f64);
    drawing_interface = 1 as libc::c_int;
}
//*************************************************************************
#[no_mangle]
pub unsafe extern "C" fn pie_SetGeometricOffset(mut x: libc::c_int,
                                                mut y: libc::c_int) {
    (*psRendSurface).xcentre = x;
    (*psRendSurface).ycentre = y;
}
// all these routines use the PC format of iVertex ... and are not used on the PSX
//*************************************************************************
#[no_mangle]
pub unsafe extern "C" fn pie_Clockwise(mut s: *mut iVertex) -> BOOL {
    return (((*s.offset(1 as libc::c_int as isize)).y -
                 (*s.offset(0 as libc::c_int as isize)).y) *
                ((*s.offset(2 as libc::c_int as isize)).x -
                     (*s.offset(1 as libc::c_int as isize)).x) <=
                ((*s.offset(1 as libc::c_int as isize)).x -
                     (*s.offset(0 as libc::c_int as isize)).x) *
                    ((*s.offset(2 as libc::c_int as isize)).y -
                         (*s.offset(1 as libc::c_int as isize)).y)) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn pie_PieClockwise(mut s: *mut PIEVERTEX) -> BOOL {
    return (((*s.offset(1 as libc::c_int as isize)).sy -
                 (*s.offset(0 as libc::c_int as isize)).sy) *
                ((*s.offset(2 as libc::c_int as isize)).sx -
                     (*s.offset(1 as libc::c_int as isize)).sx) <=
                ((*s.offset(1 as libc::c_int as isize)).sx -
                     (*s.offset(0 as libc::c_int as isize)).sx) *
                    ((*s.offset(2 as libc::c_int as isize)).sy -
                         (*s.offset(1 as libc::c_int as isize)).sy)) as
               libc::c_int;
}
//*************************************************************************
//*** inverse rotate 3D vector with current rotation matrix
//*
//* params	v1 = pointer to 3D vector to rotate
//* 			v2 = pointer to 3D resultant vector
//*
//* on exit	v2 = inverse-rotated vector
//*
//******
#[no_mangle]
pub unsafe extern "C" fn pie_VectorInverseRotate0(mut v1: *mut iVector,
                                                  mut v2: *mut iVector) {
    let mut x: int32 = 0;
    let mut y: int32 = 0;
    let mut z: int32 = 0;
    x = (*v1).x;
    y = (*v1).y;
    z = (*v1).z;
    (*v2).x =
        x * (*psMatrix).a + y * (*psMatrix).b + z * (*psMatrix).c >>
            12 as libc::c_int;
    (*v2).y =
        x * (*psMatrix).d + y * (*psMatrix).e + z * (*psMatrix).f >>
            12 as libc::c_int;
    (*v2).z =
        x * (*psMatrix).g + y * (*psMatrix).h + z * (*psMatrix).i >>
            12 as libc::c_int;
}
//*************************************************************************
//*************************************************************************
//*************************************************************************
//*************************************************************************
//*************************************************************************
//*************************************************************************
//*** setup transformation matrices/quaternions and trig tables
//*
//******
#[no_mangle]
pub unsafe extern "C" fn pie_MatInit() {
    let mut i: libc::c_uint = 0;
    let mut scsize: libc::c_uint = 0;
    let mut conv: libc::c_double = 0.;
    let mut v: libc::c_double = 0.;
    // sin/cos table
    scsize =
        (4096 as libc::c_int + 4096 as libc::c_int / 4 as libc::c_int) as
            libc::c_uint;
    conv =
        (3.141592654f64 / (0.5f64 * 4096 as libc::c_int as libc::c_double)) as
            libc::c_float as libc::c_double;
    i = 0 as libc::c_int as libc::c_uint;
    while i < scsize {
        v =
            sin(i as libc::c_double * conv) *
                ((1 as libc::c_int) << 12 as libc::c_int) as libc::c_double;
        if v >= 0.0f64 {
            aSinTable[i as usize] = (v + 0.5f64) as int32
        } else { aSinTable[i as usize] = (v - 0.5f64) as int32 }
        i = i.wrapping_add(1)
    }
    // init matrix/quat stack
    pie_MatReset();
}
unsafe extern "C" fn run_static_initializers() {
    psMatrix =
        &mut *aMatrixStack.as_mut_ptr().offset(0 as libc::c_int as isize) as
            *mut SDMATRIX
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
