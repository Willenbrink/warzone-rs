use ::libc;
extern "C" {
    #[no_mangle]
    fn abort() -> !;
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
}
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
pub type _difficulty_level = libc::c_uint;
pub const DL_KILLER: _difficulty_level = 4;
pub const DL_TOUGH: _difficulty_level = 3;
pub const DL_HARD: _difficulty_level = 2;
pub const DL_NORMAL: _difficulty_level = 1;
pub const DL_EASY: _difficulty_level = 0;
pub type DIFFICULTY_LEVEL = _difficulty_level;
// ------------------------------------------------------------------------------------
/* Simple short file - only because there was nowhere else for it logically to go */
/* Handes the difficulty level effects on gameplay */
/*
	Changed to allow seperate modifiers for enemy and player damage.
*/
// ------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------
#[no_mangle]
pub static mut presDifLevel: DIFFICULTY_LEVEL = DL_NORMAL;
#[no_mangle]
pub static mut fDifPlayerModifier: FRACT = 0.;
#[no_mangle]
pub static mut fDifEnemyModifier: FRACT = 0.;
#[no_mangle]
pub unsafe extern "C" fn getModifiers(mut Player: *mut FRACT,
                                      mut Enemy: *mut FRACT) {
    *Player = fDifPlayerModifier;
    *Enemy = fDifEnemyModifier;
}
#[no_mangle]
pub unsafe extern "C" fn setModifiers(mut Player: FRACT, mut Enemy: FRACT) {
    fDifPlayerModifier = Player;
    fDifEnemyModifier = Enemy;
}
// ------------------------------------------------------------------------------------
/* Sets the game difficulty level */
#[no_mangle]
pub unsafe extern "C" fn setDifficultyLevel(mut lev: DIFFICULTY_LEVEL) {
    match lev as libc::c_uint {
        0 => {
            fDifPlayerModifier =
                120 as libc::c_int as libc::c_float /
                    100 as libc::c_int as libc::c_float; // 10 times
            fDifEnemyModifier =
                100 as libc::c_int as libc::c_float /
                    100 as libc::c_int as libc::c_float
        }
        1 => {
            fDifPlayerModifier =
                100 as libc::c_int as libc::c_float /
                    100 as libc::c_int as libc::c_float; // almost nothing
            fDifEnemyModifier =
                100 as libc::c_int as libc::c_float /
                    100 as libc::c_int as libc::c_float
        }
        2 => {
            fDifPlayerModifier =
                80 as libc::c_int as libc::c_float /
                    100 as libc::c_int as
                        libc::c_float; // they do less damage!
            fDifEnemyModifier =
                100 as libc::c_int as libc::c_float /
                    100 as libc::c_int as libc::c_float
        }
        4 => {
            fDifPlayerModifier =
                999 as libc::c_int as libc::c_float /
                    100 as libc::c_int as libc::c_float;
            fDifEnemyModifier =
                1 as libc::c_int as libc::c_float /
                    100 as libc::c_int as libc::c_float
        }
        3 => {
            fDifPlayerModifier =
                100 as libc::c_int as libc::c_float /
                    100 as libc::c_int as libc::c_float;
            fDifEnemyModifier =
                50 as libc::c_int as libc::c_float /
                    100 as libc::c_int as libc::c_float
        }
        _ => {
            debug(LOG_ERROR,
                  b"Invalid difficulty level selected - forcing NORMAL\x00" as
                      *const u8 as *const libc::c_char);
            abort();
        }
    }
    presDifLevel = lev;
}
// ------------------------------------------------------------------------------------
/* Returns the difficulty level */
#[no_mangle]
pub unsafe extern "C" fn getDifficultyLevel() -> DIFFICULTY_LEVEL {
    return presDifLevel;
}
// ------------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn modifyForDifficultyLevel(mut basicVal: SDWORD,
                                                  mut IsPlayer: BOOL)
 -> SDWORD {
    let mut retVal: SDWORD = 0;
    // You can't garantee that we don't want damage modifiers in normal difficulty.
//	/* Unmodified! */
//	if(getDifficultyLevel() == DL_NORMAL)
//	{
//		return(basicVal);
//	}
    //	retVal = basicVal*fDifModifier;
    if IsPlayer != 0 {
        retVal =
            if basicVal as libc::c_float * fDifPlayerModifier >=
                   0 as libc::c_int as libc::c_float {
                ((basicVal as libc::c_float * fDifPlayerModifier) as
                     libc::c_double + 0.5f64) as SDWORD
            } else {
                ((basicVal as libc::c_float * fDifPlayerModifier) as
                     libc::c_double - 0.5f64) as SDWORD
            }
    } else {
        retVal =
            if basicVal as libc::c_float * fDifEnemyModifier >=
                   0 as libc::c_int as libc::c_float {
                ((basicVal as libc::c_float * fDifEnemyModifier) as
                     libc::c_double + 0.5f64) as SDWORD
            } else {
                ((basicVal as libc::c_float * fDifEnemyModifier) as
                     libc::c_double - 0.5f64) as SDWORD
            }
    }
    //	DBPRINTF(("%d : %d %d\n",IsPlayer,basicVal,retVal));
    return retVal;
}
// ------------------------------------------------------------------------------------
