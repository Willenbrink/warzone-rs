use ::libc;
extern "C" {
    /* Append SRC onto DEST.  */
    #[no_mangle]
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    /* Compare S1 and S2.  */
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
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
    /* Use misc.  */
    /* Use extended X/Open || misc. */
    /* Return a random integer between 0 and RAND_MAX inclusive.  */
    #[no_mangle]
    fn rand() -> libc::c_int;
    /* Abort execution and generate a core-dump.  */
    #[no_mangle]
    fn abort() -> !;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn resGetData(pType: *mut STRING, pID: *mut STRING) -> *mut libc::c_void;
}
pub type STRING = libc::c_char;
pub type UWORD = libc::c_ushort;
pub type UDWORD = libc::c_uint;
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
// Routines to provide simple maths functions that work on both PSX & PC
// Use the type "FRACT" instead of FLOAT
//  - This is defined as a float on PC and a 20.12 fixed point number on PSX
//
//  Use:-
//		MAKEFRACT(int);  to convert from a SDWORD to a FRACT
//		MAKEINT(fract);	to convert the other way
//		FRACTmul(fract,fract); to multiply two fract numbers
//		FRACTdiv(fract,fract); to divide two numbers
//		SQRT(fract);		to get square root of a fract (returns a fract)
//      iSQRT(int);			to get a square root of an integer (returns an UDWORD)
//      FRACTCONST(constA,constB);	; Generates a constant of (constA/constB)
//                         e.g. to define 0.5 use FRACTCONST(1,2)
//                              to define 0.114 use FRACTCONT(114,1000)
//
// Also PERCENT(int,int);	// returns a int value 0->100 of the percentage of the first param over the second
//
// This file used to be in the deliverance src directory. But Jeremy quite correctly
// pointed out to me that it should be library based not deliverance based, and hence
// has now been moved to the lib\framework directory
//
// If you are reading this file from the deliverance source directory, please delete it now
// To multiply a FRACT by a integer just use the normal operator 
//   e.g.   FractValue2=FractValue*Interger;
//
// save is true of divide
/* Check the header files have been included from frame.h if they
 * are used outside of the framework library.
 */
pub type FRACT = libc::c_float;
pub type int32 = libc::c_int;
pub type uint8 = libc::c_uchar;
pub type uint16 = libc::c_ushort;
pub type uint32 = libc::c_uint;
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
pub struct iTexAnim {
    pub nFrames: libc::c_int,
    pub playbackRate: libc::c_int,
    pub textureWidth: libc::c_int,
    pub textureHeight: libc::c_int,
}
//*************************************************************************
//
// imd structures
//
//*************************************************************************
pub type BSPPOLYID = uint16;
/* **************************************************************************/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PLANE {
    pub a: FRACT,
    pub b: FRACT,
    pub c: FRACT,
    pub d: FRACT,
    pub vP: iVector,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BSPTREENODE {
    pub link: [*mut BSPTREENODE; 2],
    pub Plane: PLANE,
    pub TriSameDir: BSPPOLYID,
    pub TriOppoDir: BSPPOLYID,
}
pub type PSBSPTREENODE = *mut BSPTREENODE;
// lets hope this can work as a byte ... that will limit it to 255 polygons in 1 imd
pub type VERTEXID = libc::c_int;
// Size of the entry for vertex id in the imd polygon structure (32bits on pc 16bits on PSX)
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iIMDPoly {
    pub flags: uint32,
    pub zcentre: int32,
    pub npnts: libc::c_int,
    pub normal: iVector,
    pub pindex: *mut VERTEXID,
    pub vrt: *mut iVertex,
    pub pTexAnim: *mut iTexAnim,
    pub BSP_NextPoly: BSPPOLYID,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iIMDShape {
    pub flags: uint32,
    pub texpage: int32,
    pub oradius: int32,
    pub sradius: int32,
    pub radius: int32,
    pub visRadius: int32,
    pub xmin: int32,
    pub xmax: int32,
    pub ymin: int32,
    pub ymax: int32,
    pub zmin: int32,
    pub zmax: int32,
    pub ocen: iVector,
    pub numFrames: UWORD,
    pub animInterval: UWORD,
    pub npoints: libc::c_int,
    pub npolys: libc::c_int,
    pub nconnectors: libc::c_int,
    pub points: *mut iVector,
    pub polys: *mut iIMDPoly,
    pub connectors: *mut iVector,
    pub ntexanims: libc::c_int,
    pub texanims: *mut *mut iTexAnim,
    pub next: *mut iIMDShape,
    pub BSPNode: PSBSPTREENODE,
}
/* An imd entry */
pub type MISC_IMD = _misc_imd;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _misc_imd {
    pub pImd: *mut iIMDShape,
    pub pName: *mut STRING,
}
pub const MI_TOO_MANY: C2RustUnnamed = 44;
pub const MI_WRECK0: C2RustUnnamed = 33;
pub const MI_WRECK4: C2RustUnnamed = 37;
pub const MI_DEBRIS0: C2RustUnnamed = 38;
pub const MI_DEBRIS4: C2RustUnnamed = 42;
pub type C2RustUnnamed = libc::c_uint;
pub const MI_FIREWORK: C2RustUnnamed = 43;
pub const MI_DEBRIS3: C2RustUnnamed = 41;
pub const MI_DEBRIS2: C2RustUnnamed = 40;
pub const MI_DEBRIS1: C2RustUnnamed = 39;
pub const MI_WRECK3: C2RustUnnamed = 36;
pub const MI_WRECK2: C2RustUnnamed = 35;
pub const MI_WRECK1: C2RustUnnamed = 34;
pub const MI_BLIP_ARTEFACT: C2RustUnnamed = 32;
pub const MI_BLIP_RESOURCE: C2RustUnnamed = 31;
pub const MI_BLIP_ENEMY: C2RustUnnamed = 30;
pub const MI_SHOCK: C2RustUnnamed = 29;
pub const MI_LANDING: C2RustUnnamed = 28;
pub const MI_KICK: C2RustUnnamed = 27;
pub const MI_SPLASH: C2RustUnnamed = 26;
pub const MI_SNOW: C2RustUnnamed = 25;
pub const MI_RAIN: C2RustUnnamed = 24;
pub const MI_MFLARE: C2RustUnnamed = 23;
pub const MI_TESLA: C2RustUnnamed = 22;
pub const MI_FLAME: C2RustUnnamed = 21;
pub const MI_TRAIL: C2RustUnnamed = 20;
pub const MI_BLOOD: C2RustUnnamed = 19;
pub const MI_TRANSPORTER_SHADOW: C2RustUnnamed = 18;
pub const MI_SHADOW: C2RustUnnamed = 17;
pub const MI_BLIP: C2RustUnnamed = 16;
pub const MI_PLASMA: C2RustUnnamed = 15;
pub const MI_SMALL_STEAM: C2RustUnnamed = 14;
pub const MI_DROID_DAMAGE: C2RustUnnamed = 13;
pub const MI_WATER: C2RustUnnamed = 12;
pub const MI_CYBORG_BODY: C2RustUnnamed = 11;
pub const MI_CYBORG_ARM: C2RustUnnamed = 10;
pub const MI_CYBORG_LEGS: C2RustUnnamed = 9;
pub const MI_CYBORG_HEAD: C2RustUnnamed = 8;
pub const MI_BABA_BODY: C2RustUnnamed = 7;
pub const MI_BABA_ARM: C2RustUnnamed = 6;
pub const MI_BABA_LEGS: C2RustUnnamed = 5;
pub const MI_BABA_HEAD: C2RustUnnamed = 4;
pub const MI_SMALL_SMOKE: C2RustUnnamed = 3;
pub const MI_CONSTRUCTION: C2RustUnnamed = 2;
pub const MI_EXPLOSION_MEDIUM: C2RustUnnamed = 1;
pub const MI_EXPLOSION_SMALL: C2RustUnnamed = 0;
/* Our great big array of imds */
#[no_mangle]
pub static mut miscImds: [MISC_IMD; 45] =
    [{
         let mut init =
             _misc_imd{pImd: 0 as *const iIMDShape as *mut iIMDShape,
                       pName:
                           b"fxsexp\x00" as *const u8 as *const libc::c_char
                               as *mut STRING,};
         init
     },
     {
         let mut init =
             _misc_imd{pImd: 0 as *const iIMDShape as *mut iIMDShape,
                       pName:
                           b"fxlexp\x00" as *const u8 as *const libc::c_char
                               as *mut STRING,};
         init
     },
     {
         let mut init =
             _misc_imd{pImd: 0 as *const iIMDShape as *mut iIMDShape,
                       pName:
                           b"fxdust\x00" as *const u8 as *const libc::c_char
                               as *mut STRING,};
         init
     },
     {
         let mut init =
             _misc_imd{pImd: 0 as *const iIMDShape as *mut iIMDShape,
                       pName:
                           b"fxsmoke\x00" as *const u8 as *const libc::c_char
                               as *mut STRING,};
         init
     },
     {
         let mut init =
             _misc_imd{pImd: 0 as *const iIMDShape as *mut iIMDShape,
                       pName:
                           b"parthead\x00" as *const u8 as *const libc::c_char
                               as *mut STRING,};
         init
     },
     {
         let mut init =
             _misc_imd{pImd: 0 as *const iIMDShape as *mut iIMDShape,
                       pName:
                           b"partlegs\x00" as *const u8 as *const libc::c_char
                               as *mut STRING,};
         init
     },
     {
         let mut init =
             _misc_imd{pImd: 0 as *const iIMDShape as *mut iIMDShape,
                       pName:
                           b"partarm\x00" as *const u8 as *const libc::c_char
                               as *mut STRING,};
         init
     },
     {
         let mut init =
             _misc_imd{pImd: 0 as *const iIMDShape as *mut iIMDShape,
                       pName:
                           b"partbody\x00" as *const u8 as *const libc::c_char
                               as *mut STRING,};
         init
     },
     {
         let mut init =
             _misc_imd{pImd: 0 as *const iIMDShape as *mut iIMDShape,
                       pName:
                           b"cybitrkt\x00" as *const u8 as *const libc::c_char
                               as *mut STRING,};
         init
     },
     {
         let mut init =
             _misc_imd{pImd: 0 as *const iIMDShape as *mut iIMDShape,
                       pName:
                           b"cybitlg1\x00" as *const u8 as *const libc::c_char
                               as *mut STRING,};
         init
     },
     {
         let mut init =
             _misc_imd{pImd: 0 as *const iIMDShape as *mut iIMDShape,
                       pName:
                           b"cybitgun\x00" as *const u8 as *const libc::c_char
                               as *mut STRING,};
         init
     },
     {
         let mut init =
             _misc_imd{pImd: 0 as *const iIMDShape as *mut iIMDShape,
                       pName:
                           b"cybitbod\x00" as *const u8 as *const libc::c_char
                               as *mut STRING,};
         init
     },
     {
         let mut init =
             _misc_imd{pImd: 0 as *const iIMDShape as *mut iIMDShape,
                       pName:
                           b"fxatexp\x00" as *const u8 as *const libc::c_char
                               as *mut STRING,};
         init
     },
     {
         let mut init =
             _misc_imd{pImd: 0 as *const iIMDShape as *mut iIMDShape,
                       pName:
                           b"fxssteam\x00" as *const u8 as *const libc::c_char
                               as *mut STRING,};
         init
     },
     {
         let mut init =
             _misc_imd{pImd: 0 as *const iIMDShape as *mut iIMDShape,
                       pName:
                           b"fxssteam\x00" as *const u8 as *const libc::c_char
                               as *mut STRING,};
         init
     },
     {
         let mut init =
             _misc_imd{pImd: 0 as *const iIMDShape as *mut iIMDShape,
                       pName:
                           b"fxplasma\x00" as *const u8 as *const libc::c_char
                               as *mut STRING,};
         init
     },
     {
         let mut init =
             _misc_imd{pImd: 0 as *const iIMDShape as *mut iIMDShape,
                       pName:
                           b"fxblip\x00" as *const u8 as *const libc::c_char
                               as *mut STRING,};
         init
     },
     {
         let mut init =
             _misc_imd{pImd: 0 as *const iIMDShape as *mut iIMDShape,
                       pName:
                           b"cyshadow\x00" as *const u8 as *const libc::c_char
                               as *mut STRING,};
         init
     },
     {
         let mut init =
             _misc_imd{pImd: 0 as *const iIMDShape as *mut iIMDShape,
                       pName:
                           b"Mitrnshd\x00" as *const u8 as *const libc::c_char
                               as *mut STRING,};
         init
     },
     {
         let mut init =
             _misc_imd{pImd: 0 as *const iIMDShape as *mut iIMDShape,
                       pName:
                           b"fxblood\x00" as *const u8 as *const libc::c_char
                               as *mut STRING,};
         init
     },
     {
         let mut init =
             _misc_imd{pImd: 0 as *const iIMDShape as *mut iIMDShape,
                       pName:
                           b"fxssmoke\x00" as *const u8 as *const libc::c_char
                               as *mut STRING,};
         init
     },
     {
         let mut init =
             _misc_imd{pImd: 0 as *const iIMDShape as *mut iIMDShape,
                       pName:
                           b"fxft\x00" as *const u8 as *const libc::c_char as
                               *mut STRING,};
         init
     },
     {
         let mut init =
             _misc_imd{pImd: 0 as *const iIMDShape as *mut iIMDShape,
                       pName:
                           b"fxpower\x00" as *const u8 as *const libc::c_char
                               as *mut STRING,};
         init
     },
     {
         let mut init =
             _misc_imd{pImd: 0 as *const iIMDShape as *mut iIMDShape,
                       pName:
                           b"fxmflare\x00" as *const u8 as *const libc::c_char
                               as *mut STRING,};
         init
     },
     {
         let mut init =
             _misc_imd{pImd: 0 as *const iIMDShape as *mut iIMDShape,
                       pName:
                           b"mirain\x00" as *const u8 as *const libc::c_char
                               as *mut STRING,};
         init
     },
     {
         let mut init =
             _misc_imd{pImd: 0 as *const iIMDShape as *mut iIMDShape,
                       pName:
                           b"misnow\x00" as *const u8 as *const libc::c_char
                               as *mut STRING,};
         init
     },
     {
         let mut init =
             _misc_imd{pImd: 0 as *const iIMDShape as *mut iIMDShape,
                       pName:
                           b"fxssplsh\x00" as *const u8 as *const libc::c_char
                               as *mut STRING,};
         init
     },
     {
         let mut init =
             _misc_imd{pImd: 0 as *const iIMDShape as *mut iIMDShape,
                       pName:
                           b"fxexpdrt\x00" as *const u8 as *const libc::c_char
                               as *mut STRING,};
         init
     },
     {
         let mut init =
             _misc_imd{pImd: 0 as *const iIMDShape as *mut iIMDShape,
                       pName:
                           b"fxlightr\x00" as *const u8 as *const libc::c_char
                               as *mut STRING,};
         init
     },
     {
         let mut init =
             _misc_imd{pImd: 0 as *const iIMDShape as *mut iIMDShape,
                       pName:
                           b"fxl3dshk\x00" as *const u8 as *const libc::c_char
                               as *mut STRING,};
         init
     },
     {
         let mut init =
             _misc_imd{pImd: 0 as *const iIMDShape as *mut iIMDShape,
                       pName:
                           b"blipenm\x00" as *const u8 as *const libc::c_char
                               as *mut STRING,};
         init
     },
     {
         let mut init =
             _misc_imd{pImd: 0 as *const iIMDShape as *mut iIMDShape,
                       pName:
                           b"blipres\x00" as *const u8 as *const libc::c_char
                               as *mut STRING,};
         init
     },
     {
         let mut init =
             _misc_imd{pImd: 0 as *const iIMDShape as *mut iIMDShape,
                       pName:
                           b"blipart\x00" as *const u8 as *const libc::c_char
                               as *mut STRING,};
         init
     },
     {
         let mut init =
             _misc_imd{pImd: 0 as *const iIMDShape as *mut iIMDShape,
                       pName:
                           b"miwrek1\x00" as *const u8 as *const libc::c_char
                               as *mut STRING,};
         init
     },
     {
         let mut init =
             _misc_imd{pImd: 0 as *const iIMDShape as *mut iIMDShape,
                       pName:
                           b"miwrek2\x00" as *const u8 as *const libc::c_char
                               as *mut STRING,};
         init
     },
     {
         let mut init =
             _misc_imd{pImd: 0 as *const iIMDShape as *mut iIMDShape,
                       pName:
                           b"miwrek3\x00" as *const u8 as *const libc::c_char
                               as *mut STRING,};
         init
     },
     {
         let mut init =
             _misc_imd{pImd: 0 as *const iIMDShape as *mut iIMDShape,
                       pName:
                           b"miwrek4\x00" as *const u8 as *const libc::c_char
                               as *mut STRING,};
         init
     },
     {
         let mut init =
             _misc_imd{pImd: 0 as *const iIMDShape as *mut iIMDShape,
                       pName:
                           b"miwrek5\x00" as *const u8 as *const libc::c_char
                               as *mut STRING,};
         init
     },
     {
         let mut init =
             _misc_imd{pImd: 0 as *const iIMDShape as *mut iIMDShape,
                       pName:
                           b"midebr1\x00" as *const u8 as *const libc::c_char
                               as *mut STRING,};
         init
     },
     {
         let mut init =
             _misc_imd{pImd: 0 as *const iIMDShape as *mut iIMDShape,
                       pName:
                           b"midebr2\x00" as *const u8 as *const libc::c_char
                               as *mut STRING,};
         init
     },
     {
         let mut init =
             _misc_imd{pImd: 0 as *const iIMDShape as *mut iIMDShape,
                       pName:
                           b"midebr3\x00" as *const u8 as *const libc::c_char
                               as *mut STRING,};
         init
     },
     {
         let mut init =
             _misc_imd{pImd: 0 as *const iIMDShape as *mut iIMDShape,
                       pName:
                           b"midebr4\x00" as *const u8 as *const libc::c_char
                               as *mut STRING,};
         init
     },
     {
         let mut init =
             _misc_imd{pImd: 0 as *const iIMDShape as *mut iIMDShape,
                       pName:
                           b"midebr5\x00" as *const u8 as *const libc::c_char
                               as *mut STRING,};
         init
     },
     {
         let mut init =
             _misc_imd{pImd: 0 as *const iIMDShape as *mut iIMDShape,
                       pName:
                           b"fxflecht\x00" as *const u8 as *const libc::c_char
                               as *mut STRING,};
         init
     },
     {
         let mut init =
             _misc_imd{pImd: 0 as *const iIMDShape as *mut iIMDShape,
                       pName:
                           b"END_OF_IMD_LIST\x00" as *const u8 as
                               *const libc::c_char as *mut STRING,};
         init
     }];
// -------------------------------------------------------------------------------
// Load up all the imds into an array
#[no_mangle]
pub unsafe extern "C" fn multiLoadMiscImds() -> BOOL {
    let mut i: UDWORD = 0 as libc::c_int as UDWORD; // hopefully!
    let mut bMoreToProcess: BOOL = 1 as libc::c_int;
    let mut name: [libc::c_char; 15] = [0; 15];
    /* Go thru' the list */
    while bMoreToProcess != 0 {
        sprintf(name.as_mut_ptr(), miscImds[i as usize].pName);
        strcat(name.as_mut_ptr(),
               b".pie\x00" as *const u8 as *const libc::c_char);
        /* see if the resource loader can find it */
        miscImds[i as usize].pImd =
            resGetData(b"IMD\x00" as *const u8 as *const libc::c_char as
                           *mut STRING, name.as_mut_ptr()) as *mut iIMDShape;
        /* If it didn't get it then... */
        if miscImds[i as usize].pImd.is_null() {
            /* Say which one and return FALSE */
            debug(LOG_ERROR,
                  b"Can\'t find misselaneous PIE file : %s\x00" as *const u8
                      as *const libc::c_char, miscImds[i as usize].pName);
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"NULL PIE\x00" as *const u8 as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"miscimd.c\x00" as *const u8 as *const libc::c_char,
                      87 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 18],
                                                &[libc::c_char; 18]>(b"multiLoadMiscImds\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            return 0 as libc::c_int
        }
        /*	If the next one's the end one, then get out now.
			This is cos strcmp will return 0 only at end of list
		*/
        i = i.wrapping_add(1);
        bMoreToProcess =
            strcmp(miscImds[i as usize].pName,
                   b"END_OF_IMD_LIST\x00" as *const u8 as *const libc::c_char)
    }
    return 1 as libc::c_int;
}
// -------------------------------------------------------------------------------
// Returns a pointer to the imd from a #define number passed in - see above
#[no_mangle]
pub unsafe extern "C" fn getImdFromIndex(mut index: UDWORD)
 -> *mut iIMDShape {
    if index < MI_TOO_MANY as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Out of range index in getImdFromIndex\x00" as *const u8 as
                  *const libc::c_char);
    };
    if index < MI_TOO_MANY as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"miscimd.c\x00" as *const u8 as *const libc::c_char,
              101 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 16],
                                        &[libc::c_char; 16]>(b"getImdFromIndex\x00")).as_ptr(),
              b"index<MI_TOO_MANY\x00" as *const u8 as *const libc::c_char);
    };
    return miscImds[index as usize].pImd;
}
// -------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn getRandomWreckageImd() -> *mut iIMDShape {
    let mut WreckageIMD: *mut iIMDShape = 0 as *mut iIMDShape;
    // Get a random wreckage
    WreckageIMD =
        getImdFromIndex((MI_WRECK0 as libc::c_int +
                             rand() %
                                 (MI_WRECK4 as libc::c_int -
                                      MI_WRECK0 as libc::c_int +
                                      1 as libc::c_int)) as UDWORD);
    // if it doesn't exsist (cam2) then get the first one
    if WreckageIMD.is_null() {
        WreckageIMD = getImdFromIndex(MI_WRECK0 as libc::c_int as UDWORD)
    }
    // check to make sure one exists  (will fail on renderfeature if null)
    return WreckageIMD;
}
// -------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn getRandomDebrisImd() -> *mut iIMDShape {
    let mut DebrisIMD: *mut iIMDShape = 0 as *mut iIMDShape;
    DebrisIMD =
        getImdFromIndex((MI_DEBRIS0 as libc::c_int +
                             rand() %
                                 (MI_DEBRIS4 as libc::c_int -
                                      MI_DEBRIS0 as libc::c_int +
                                      1 as libc::c_int)) as UDWORD);
    if !DebrisIMD.is_null() {
    } else {
        debug(LOG_ERROR,
              b"getRandomDebrisImd : NULL PIE\x00" as *const u8 as
                  *const libc::c_char);
    };
    if !DebrisIMD.is_null() {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"miscimd.c\x00" as *const u8 as *const libc::c_char,
              132 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 19],
                                        &[libc::c_char; 19]>(b"getRandomDebrisImd\x00")).as_ptr(),
              b"DebrisIMD != NULL\x00" as *const u8 as *const libc::c_char);
    };
    return DebrisIMD;
}
// -------------------------------------------------------------------------------
//iIMDShape	*pAssemblyPointIMDs[NUM_FACTORY_TYPES][MAX_FACTORY];
#[no_mangle]
pub static mut pAssemblyPointIMDs: [[*mut iIMDShape; 5]; 4] =
    [[0 as *const iIMDShape as *mut iIMDShape; 5]; 4];
#[no_mangle]
pub unsafe extern "C" fn initMiscImds() -> BOOL {
    let mut facName: [STRING; 11] =
        *::std::mem::transmute::<&[u8; 11],
                                 &mut [STRING; 11]>(b"MINUM0.pie\x00");
    let mut cybName: [STRING; 12] =
        *::std::mem::transmute::<&[u8; 12],
                                 &mut [STRING; 12]>(b"MICNUM0.pie\x00");
    let mut vtolName: [STRING; 12] =
        *::std::mem::transmute::<&[u8; 12],
                                 &mut [STRING; 12]>(b"MIVNUM0.pie\x00");
    let mut pieNum: [STRING; 2] = [0; 2];
    let mut i: UDWORD = 0;
    /* Do the new loading system */
    multiLoadMiscImds();
    /* Now load the multi array stuff */
    i = 0 as libc::c_int as UDWORD;
    while i < 5 as libc::c_int as libc::c_uint {
        sprintf(pieNum.as_mut_ptr(),
                b"%d\x00" as *const u8 as *const libc::c_char,
                i.wrapping_add(1 as libc::c_int as libc::c_uint));
        facName[5 as libc::c_int as usize] = *pieNum.as_mut_ptr();
        pAssemblyPointIMDs[0 as libc::c_int as usize][i as usize] =
            resGetData(b"IMD\x00" as *const u8 as *const libc::c_char as
                           *mut STRING, facName.as_mut_ptr()) as
                *mut iIMDShape;
        if pAssemblyPointIMDs[0 as libc::c_int as usize][i as usize].is_null()
           {
            debug(LOG_ERROR,
                  b"Can\'t find assembly point graphic for factory\x00" as
                      *const u8 as *const libc::c_char);
            abort();
        }
        //#else
//		pAssemblyPointIMDs[CYBORG_FLAG][i] = pAssemblyPointIMDs[FACTORY_FLAG][i];
//		pAssemblyPointIMDs[VTOL_FLAG][i] = pAssemblyPointIMDs[FACTORY_FLAG][i];
//#endif
        cybName[6 as libc::c_int as usize] = *pieNum.as_mut_ptr();
        pAssemblyPointIMDs[1 as libc::c_int as usize][i as usize] =
            resGetData(b"IMD\x00" as *const u8 as *const libc::c_char as
                           *mut STRING, cybName.as_mut_ptr()) as
                *mut iIMDShape;
        if pAssemblyPointIMDs[1 as libc::c_int as usize][i as usize].is_null()
           {
            debug(LOG_ERROR,
                  b"Can\'t find assembly point graphic for cyborg factory\x00"
                      as *const u8 as *const libc::c_char);
            abort();
        }
        vtolName[6 as libc::c_int as usize] = *pieNum.as_mut_ptr();
        pAssemblyPointIMDs[2 as libc::c_int as usize][i as usize] =
            resGetData(b"IMD\x00" as *const u8 as *const libc::c_char as
                           *mut STRING, vtolName.as_mut_ptr()) as
                *mut iIMDShape;
        if pAssemblyPointIMDs[2 as libc::c_int as usize][i as usize].is_null()
           {
            debug(LOG_ERROR,
                  b"Can\'t find assembly point graphic for vtol factory\x00"
                      as *const u8 as *const libc::c_char);
            abort();
        }
        pAssemblyPointIMDs[3 as libc::c_int as usize][i as usize] =
            resGetData(b"IMD\x00" as *const u8 as *const libc::c_char as
                           *mut STRING,
                       b"mirnum1.pie\x00" as *const u8 as *const libc::c_char
                           as *mut STRING) as *mut iIMDShape;
        if pAssemblyPointIMDs[3 as libc::c_int as usize][i as usize].is_null()
           {
            debug(LOG_ERROR,
                  b"Can\'t find assembly point graphic for repair facility\x00"
                      as *const u8 as *const libc::c_char);
            abort();
        }
        i = i.wrapping_add(1)
    }
    return 1 as libc::c_int;
}
//#ifndef PSX
