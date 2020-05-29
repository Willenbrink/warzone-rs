use ::libc;
extern "C" {
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
    fn memFreeRelease(pMemToFree: *mut libc::c_void);
    #[no_mangle]
    fn memMallocRelease(Size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn acos(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn asin(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn cos(_: libc::c_double) -> libc::c_double;
    /* Sine of X.  */
    /* Sine of X.  */
    /* Sine of X.  */
    /* Sine of X.  */
    /* Sine of X.  */
    /* Sine of X.  */
    /* Sine of X.  */
    #[no_mangle]
    fn sin(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn sqrt(_: libc::c_double) -> libc::c_double;
}
pub type UDWORD = libc::c_uint;
pub type SDWORD = libc::c_int;
pub type BOOL = libc::c_int;
pub type size_t = libc::c_uint;
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
/* The trig functions */
static mut aSin: *mut FRACT = 0 as *const FRACT as *mut FRACT;
static mut aCos: *mut FRACT = 0 as *const FRACT as *mut FRACT;
static mut aInvCos: *mut FRACT = 0 as *const FRACT as *mut FRACT;
/* Square root table - not needed on PSX cos there is a fast hardware sqrt */
static mut aSqrt: *mut FRACT = 0 as *const FRACT as *mut FRACT;
static mut aInvSin: *mut FRACT = 0 as *const FRACT as *mut FRACT;
/* conversion macros */
/* Initialise the Trig tables */
/* Initialise the Trig tables */
#[no_mangle]
pub unsafe extern "C" fn trigInitialise() -> BOOL {
    let mut val: FRACT = 0.;
    let mut inc: FRACT = 0.;
    let mut count: UDWORD = 0;
    // Allocate the tables
    aSin =
        memMallocRelease((::std::mem::size_of::<FRACT>() as
                              libc::c_ulong).wrapping_mul(360 as libc::c_int
                                                              as
                                                              libc::c_uint))
            as *mut FRACT;
    if aSin.is_null() { return 0 as libc::c_int }
    aCos =
        memMallocRelease((::std::mem::size_of::<FRACT>() as
                              libc::c_ulong).wrapping_mul(360 as libc::c_int
                                                              as
                                                              libc::c_uint))
            as *mut FRACT;
    if aCos.is_null() { return 0 as libc::c_int }
    aInvSin =
        memMallocRelease((::std::mem::size_of::<FRACT>() as
                              libc::c_ulong).wrapping_mul(4096 as libc::c_int
                                                              as
                                                              libc::c_uint))
            as *mut FRACT;
    if aInvSin.is_null() { return 0 as libc::c_int }
    aInvCos =
        memMallocRelease((::std::mem::size_of::<FRACT>() as
                              libc::c_ulong).wrapping_mul(4096 as libc::c_int
                                                              as
                                                              libc::c_uint))
            as *mut FRACT;
    if aInvCos.is_null() { return 0 as libc::c_int }
    //#ifndef PSX
    aSqrt =
        memMallocRelease((::std::mem::size_of::<FRACT>() as
                              libc::c_ulong).wrapping_mul(4096 as libc::c_int
                                                              as
                                                              libc::c_uint))
            as *mut FRACT;
    if aSqrt.is_null() { return 0 as libc::c_int }
    //#endif
    // Initialise the tables
	// inc = 2*PI/TRIG_DEGREES
    inc =
        2 as libc::c_int as libc::c_float / 1 as libc::c_int as libc::c_float
            *
            (3.141592654f64 as libc::c_float /
                 360 as libc::c_int as libc::c_float);
    val =
        0 as libc::c_int as libc::c_float / 1 as libc::c_int as libc::c_float;
    count = 0 as libc::c_int as UDWORD;
    while count < 360 as libc::c_int as libc::c_uint {
        *aSin.offset(count as isize) = sin(val as libc::c_double) as FRACT;
        *aCos.offset(count as isize) = cos(val as libc::c_double) as FRACT;
        val += inc;
        count = count.wrapping_add(1)
    }
    inc =
        2 as libc::c_int as libc::c_float /
            (4096 as libc::c_int - 1 as libc::c_int) as libc::c_float;
    val =
        -(1 as libc::c_int) as libc::c_float /
            1 as libc::c_int as libc::c_float;
    count = 0 as libc::c_int as UDWORD;
    while count < 4096 as libc::c_int as libc::c_uint {
        *aInvSin.offset(count as isize) =
            asin(val as libc::c_double) as FRACT *
                ((360 as libc::c_int / 2 as libc::c_int) as libc::c_float /
                     3.141592654f64 as libc::c_float);
        *aInvCos.offset(count as isize) =
            acos(val as libc::c_double) as FRACT *
                ((360 as libc::c_int / 2 as libc::c_int) as libc::c_float /
                     3.141592654f64 as libc::c_float);
        val += inc;
        count = count.wrapping_add(1)
    }
    //#ifndef PSX
	// Build the sqrt table
    count = 0 as libc::c_int as UDWORD;
    while count < 4096 as libc::c_int as libc::c_uint {
        val =
            count as FRACT /
                (4096 as libc::c_int / 2 as libc::c_int) as FRACT;
        *aSqrt.offset(count as isize) = sqrt(val as libc::c_double) as FRACT;
        count = count.wrapping_add(1)
    }
    //#endif
    return 1 as libc::c_int;
}
/* Shutdown the trig tables */
/* Shutdown the trig tables */
#[no_mangle]
pub unsafe extern "C" fn trigShutDown() {
    memFreeRelease(aSin as *mut libc::c_void);
    aSin = 0 as *mut FRACT;
    memFreeRelease(aCos as *mut libc::c_void);
    aCos = 0 as *mut FRACT;
    memFreeRelease(aInvSin as *mut libc::c_void);
    aInvSin = 0 as *mut FRACT;
    memFreeRelease(aInvCos as *mut libc::c_void);
    aInvCos = 0 as *mut FRACT;
    memFreeRelease(aSqrt as *mut libc::c_void);
    aSqrt = 0 as *mut FRACT;
}
/* Lookup trig functions */
/* Access the trig tables */
#[no_mangle]
pub unsafe extern "C" fn trigSin(mut angle: SDWORD) -> FRACT {
    if angle < 0 as libc::c_int {
        angle = -angle % 360 as libc::c_int;
        angle = 360 as libc::c_int - angle
    } else { angle = angle % 360 as libc::c_int }
    return *aSin.offset((angle % 360 as libc::c_int) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn trigCos(mut angle: SDWORD) -> FRACT {
    if angle < 0 as libc::c_int {
        angle = -angle % 360 as libc::c_int;
        angle = 360 as libc::c_int - angle
    } else { angle = angle % 360 as libc::c_int }
    return *aCos.offset((angle % 360 as libc::c_int) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn trigInvSin(mut val: FRACT) -> FRACT {
    let mut index: SDWORD = 0;
    index =
        (val *
             ((4096 as libc::c_int - 1 as libc::c_int) / 2 as libc::c_int) as
                 FRACT) as SDWORD +
            (4096 as libc::c_int - 1 as libc::c_int) / 2 as libc::c_int;
    return *aInvSin.offset((index & 0xfff as libc::c_int) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn trigInvCos(mut val: FRACT) -> FRACT {
    let mut index: SDWORD = 0;
    index =
        (val *
             ((4096 as libc::c_int - 1 as libc::c_int) / 2 as libc::c_int) as
                 FRACT) as SDWORD +
            (4096 as libc::c_int - 1 as libc::c_int) / 2 as libc::c_int;
    return *aInvCos.offset((index & 0xfff as libc::c_int) as isize);
}
/* Supposedly fast lookup sqrt - unfortunately it's probably slower than the FPU sqrt :-( */
/* Fast lookup sqrt */
#[no_mangle]
pub unsafe extern "C" fn trigIntSqrt(mut val: UDWORD) -> FRACT {
    let mut exp: UDWORD = 0;
    let mut mask: UDWORD = 0;
    if val == 0 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int as libc::c_float /
                   1 as libc::c_int as libc::c_float
    }
    // find the exponent of the number
    mask = 0x80000000 as libc::c_uint; // set the msb in the mask
    exp = 32 as libc::c_int as UDWORD;
    while exp != 0 as libc::c_int as libc::c_uint {
        if val & mask != 0 { break ; }
        mask >>= 1 as libc::c_int;
        exp = exp.wrapping_sub(1)
    }
    // make all exponents even
	// odd exponents result in a mantissa of [1..2) rather than [0..1)
    if exp & 1 as libc::c_int as libc::c_uint != 0 {
        exp =
            (exp as
                 libc::c_uint).wrapping_sub(1 as libc::c_int as libc::c_uint)
                as UDWORD as UDWORD
    }
    // need to shift the top bit to SQRT_BITS - left or right?
    if exp >= 12 as libc::c_int as libc::c_uint {
        val >>=
            exp.wrapping_sub(12 as libc::c_int as
                                 libc::c_uint).wrapping_add(1 as libc::c_int
                                                                as
                                                                libc::c_uint)
    } else {
        val <<=
            ((12 as libc::c_int - 1 as libc::c_int) as
                 libc::c_uint).wrapping_sub(exp)
    }
    // now generate the fractional part for the lookup table
    if val < 4096 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"trigIntSqrt: aargh - table index out of range\x00" as
                  *const u8 as *const libc::c_char);
    };
    if val < 4096 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"trig.c\x00" as *const u8 as *const libc::c_char,
              224 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 12],
                                        &[libc::c_char; 12]>(b"trigIntSqrt\x00")).as_ptr(),
              b"val < SQRT_ACCURACY\x00" as *const u8 as *const libc::c_char);
    };
    return *aSqrt.offset(val as isize) *
               ((1 as libc::c_int as UDWORD) <<
                    exp.wrapping_div(2 as libc::c_int as libc::c_uint)) as
                   FRACT;
}
/* */
// X2-X1
