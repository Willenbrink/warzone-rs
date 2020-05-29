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
}
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
/*
 * FindPath.h
 *
 * Routing functions interface.
 *
 */
//bit operators for map movement
/* Turn towards a target */
/* Return the difference in directions */
/*
 * FindPath.c
 *
 * Routing code.
 *
 */
/* Turn printf's */
//#define DEBUG_GROUP1
// Don't need any of findpath anymore !!!!!
/* Return the difference in directions */
#[no_mangle]
pub unsafe extern "C" fn dirDiff(mut start: SDWORD, mut end: SDWORD)
 -> UDWORD {
    let mut retval: SDWORD = 0;
    let mut diff: SDWORD = 0;
    diff = end - start;
    if diff > 0 as libc::c_int {
        if diff < 180 as libc::c_int {
            retval = diff
        } else { retval = 360 as libc::c_int - diff }
    } else if diff > -(180 as libc::c_int) {
        retval = -diff
    } else { retval = 360 as libc::c_int + diff }
    if retval >= 0 as libc::c_int && retval <= 180 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"dirDiff: result out of range\x00" as *const u8 as
                  *const libc::c_char);
    };
    if retval >= 0 as libc::c_int && retval <= 180 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"findpath.c\x00" as *const u8 as *const libc::c_char,
              917 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 8],
                                        &[libc::c_char; 8]>(b"dirDiff\x00")).as_ptr(),
              b"retval >=0 && retval <=180\x00" as *const u8 as
                  *const libc::c_char);
    };
    return retval as UDWORD;
}
