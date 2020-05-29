use ::libc;
extern "C" {
    #[no_mangle]
    fn GetTickCount() -> DWORD;
}
pub type __uint64_t = libc::c_ulonglong;
pub type uint64_t = __uint64_t;
pub type UDWORD = libc::c_uint;
pub type SDWORD = libc::c_int;
pub type BOOL = libc::c_int;
pub type DWORD = libc::c_int;
pub type FRACT = libc::c_float;
/*
 * GTime.c
 *
 * Provide a game clock that runs only when the game runs.
 *
 */
//#define DEBUG_GROUP1
//#define RATE_LIMIT
/* The current time in the game world */
#[no_mangle]
pub static mut gameTime: UDWORD = 0;
/* The time for the last frame */
#[no_mangle]
pub static mut frameTime: UDWORD = 0;
/* The current time in the game world ( never stops )*/
#[no_mangle]
pub static mut gameTime2: UDWORD = 0;
/* The time for the last frame (never stops)*/
#[no_mangle]
pub static mut frameTime2: UDWORD = 0;
// the current clock modifier
static mut modifier: FRACT = 0.;
// the amount of game time before the last time clock speed was set
static mut timeOffset: UDWORD = 0;
static mut timeOffset2: UDWORD = 0;
// the tick count the last time the clock speed was set
static mut baseTime: UDWORD = 0;
static mut baseTime2: UDWORD = 0;
/* When the game paused so that gameTime can be adjusted when the game restarts */
static mut pauseStart: SDWORD = 0;
/* Count how many times gameTimeStop has been called without a game time start */
static mut stopCount: UDWORD = 0;
/* The current time in the game world */
/* The time for the last frame */
/* The current time in the game world */
// Never stops.
/* The time for the last frame */
// Never stops.
/* Initialise the game clock */
/* Initialise the game clock */
#[no_mangle]
pub unsafe extern "C" fn gameTimeInit() -> BOOL {
    //gameTime = 0;
    /*start the timer off at 2 so that when the scripts strip the map of objects
    for multiPlayer they will be processed as if they died*/
    gameTime = 2 as libc::c_int as UDWORD;
    timeOffset = 0 as libc::c_int as UDWORD;
    baseTime = GetTickCount() as UDWORD;
    gameTime2 = 0 as libc::c_int as UDWORD;
    timeOffset2 = 0 as libc::c_int as UDWORD;
    baseTime2 = baseTime;
    modifier =
        1 as libc::c_int as libc::c_float / 1 as libc::c_int as libc::c_float;
    stopCount = 0 as libc::c_int as UDWORD;
    return 1 as libc::c_int;
}
/* Useful for periodical stuff */
/* Will return a number that climbs over tickFrequency game ticks and ends up in the required range. */
/*	
	For instance getTimeValueRange(4096,256) will return a number that cycles through
	the values 0..256 every 4.096 seconds...
	Ensure that the first is an integer multiple of the second 
*/
#[no_mangle]
pub unsafe extern "C" fn getTimeValueRange(mut tickFrequency: UDWORD,
                                           mut requiredRange: UDWORD)
 -> UDWORD {
    let mut div1: UDWORD = 0;
    let mut div2: UDWORD = 0;
    div1 = gameTime2.wrapping_rem(tickFrequency);
    div2 = tickFrequency.wrapping_div(requiredRange);
    return div1.wrapping_div(div2);
}
#[no_mangle]
pub unsafe extern "C" fn getStaticTimeValueRange(mut tickFrequency: UDWORD,
                                                 mut requiredRange: UDWORD)
 -> UDWORD {
    let mut div1: UDWORD = 0;
    let mut div2: UDWORD = 0;
    div1 = gameTime.wrapping_rem(tickFrequency);
    div2 = tickFrequency.wrapping_div(requiredRange);
    return div1.wrapping_div(div2);
}
/* Call this each loop to update the game timer */
/* Call this each loop to update the game timer */
#[no_mangle]
pub unsafe extern "C" fn gameTimeUpdate() {
    let mut currTime: UDWORD = 0;
    let mut fpMod: UDWORD = 0;
    let mut newTime: uint64_t = 0;
    let mut extraTime: uint64_t = 0;
    currTime = GetTickCount() as UDWORD;
    (currTime) < baseTime;
    //don't update the game time if gameTimeStop has been called
    if stopCount == 0 as libc::c_int as libc::c_uint {
        // Calculate the new game time
        newTime = currTime.wrapping_sub(baseTime) as uint64_t;
        // convert the modifier to fixed point cos we loose accuracy
        fpMod =
            (modifier *
                 (1000 as libc::c_int as libc::c_float /
                      1 as libc::c_int as libc::c_float)) as SDWORD as UDWORD;
        newTime =
            newTime.wrapping_mul(fpMod as
                                     libc::c_ulonglong).wrapping_div(1000 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_ulonglong);
        newTime =
            (newTime as
                 libc::c_ulonglong).wrapping_add(timeOffset as
                                                     libc::c_ulonglong) as
                uint64_t as uint64_t;
        // Calculate the time for this frame
        frameTime =
            newTime.wrapping_sub(gameTime as libc::c_ulonglong) as UDWORD;
        // Limit the frame time
        if frameTime >
               (1000 as libc::c_int / 6 as libc::c_int) as libc::c_uint {
            extraTime =
                frameTime.wrapping_sub((1000 as libc::c_int /
                                            6 as libc::c_int) as libc::c_uint)
                    as uint64_t; //adjust the addition to base time
            extraTime =
                extraTime.wrapping_mul(1000 as libc::c_int as
                                           libc::c_ulonglong).wrapping_div(fpMod
                                                                               as
                                                                               libc::c_ulonglong);
            baseTime =
                (baseTime as libc::c_uint).wrapping_add(extraTime as UDWORD)
                    as UDWORD as UDWORD;
            newTime =
                gameTime.wrapping_add((1000 as libc::c_int / 6 as libc::c_int)
                                          as libc::c_uint) as uint64_t;
            frameTime = (1000 as libc::c_int / 6 as libc::c_int) as UDWORD
        }
        // Store the game time
        gameTime = newTime as UDWORD
    }
    // now update gameTime2 which does not pause
    newTime = currTime.wrapping_sub(baseTime2) as uint64_t;
    newTime =
        (newTime as
             libc::c_ulonglong).wrapping_add(timeOffset as libc::c_ulonglong)
            as uint64_t as uint64_t;
    // Calculate the time for this frame
    frameTime2 = (newTime as UDWORD).wrapping_sub(gameTime2);
    // Limit the frame time
    if frameTime2 > (1000 as libc::c_int / 6 as libc::c_int) as libc::c_uint {
        baseTime2 =
            (baseTime2 as
                 libc::c_uint).wrapping_add(frameTime2.wrapping_sub((1000 as
                                                                         libc::c_int
                                                                         /
                                                                         6 as
                                                                             libc::c_int)
                                                                        as
                                                                        libc::c_uint))
                as UDWORD as UDWORD;
        newTime =
            gameTime2.wrapping_add((1000 as libc::c_int / 6 as libc::c_int) as
                                       libc::c_uint) as uint64_t;
        frameTime2 = (1000 as libc::c_int / 6 as libc::c_int) as UDWORD
    }
    // Store the game time
    gameTime2 = newTime as UDWORD;
}
// reset the game time modifiers
// reset the game time modifiers
#[no_mangle]
pub unsafe extern "C" fn gameTimeResetMod() {
    timeOffset = gameTime;
    timeOffset2 = gameTime2;
    baseTime = GetTickCount() as UDWORD;
    baseTime2 = GetTickCount() as UDWORD;
    modifier =
        1 as libc::c_int as libc::c_float / 1 as libc::c_int as libc::c_float;
}
// set the time modifier
// set the time modifier
#[no_mangle]
pub unsafe extern "C" fn gameTimeSetMod(mut mod_0: FRACT) {
    gameTimeResetMod();
    modifier = mod_0;
}
// get the current time modifier
// get the current time modifier
#[no_mangle]
pub unsafe extern "C" fn gameTimeGetMod(mut pMod: *mut FRACT) {
    *pMod = modifier;
}
/* Returns TRUE if gameTime is stopped. */
#[no_mangle]
pub unsafe extern "C" fn gameTimeIsStopped() -> BOOL {
    return (stopCount != 0 as libc::c_int as libc::c_uint) as libc::c_int;
}
/* Call this to stop the game timer */
/* Call this to stop the game timer */
#[no_mangle]
pub unsafe extern "C" fn gameTimeStop() {
    if stopCount == 0 as libc::c_int as libc::c_uint {
        pauseStart = GetTickCount()
    }
    stopCount =
        (stopCount as
             libc::c_uint).wrapping_add(1 as libc::c_int as libc::c_uint) as
            UDWORD as UDWORD;
}
/* Call this to restart the game timer after a call to gameTimeStop */
/* Call this to restart the game timer after a call to gameTimeStop */
#[no_mangle]
pub unsafe extern "C" fn gameTimeStart() {
    if stopCount == 1 as libc::c_int as libc::c_uint {
        // shift the base time to now
        timeOffset = gameTime;
        baseTime = GetTickCount() as UDWORD
    }
    if stopCount > 0 as libc::c_int as libc::c_uint {
        stopCount = stopCount.wrapping_sub(1)
    };
}
/*Call this to reset the game timer*/
/*Call this to reset the game timer*/
#[no_mangle]
pub unsafe extern "C" fn gameTimeReset(mut time: UDWORD) {
    // reset the game timers
    gameTime =
        time; //used from save game only so GetTickCount is as valid as anything
    timeOffset = time;
    gameTime2 = time;
    timeOffset2 = time;
    baseTime = GetTickCount() as UDWORD;
    baseTime2 = GetTickCount() as UDWORD;
    modifier =
        1 as libc::c_int as libc::c_float / 1 as libc::c_int as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn getTimeComponents(mut time: UDWORD,
                                           mut hours: *mut UDWORD,
                                           mut minutes: *mut UDWORD,
                                           mut seconds: *mut UDWORD) {
    let mut h: UDWORD = 0;
    let mut m: UDWORD = 0;
    let mut s: UDWORD = 0;
    let mut tph: UDWORD = 0;
    let mut tpm: UDWORD = 0;
    /* Ticks in a minute */
    tpm = (1000 as libc::c_int * 60 as libc::c_int) as UDWORD;
    /* Ticks in an hour */
    tph = tpm.wrapping_mul(60 as libc::c_int as libc::c_uint);
    h = time.wrapping_div(tph);
    m = time.wrapping_sub(h.wrapping_mul(tph)).wrapping_div(tpm);
    s =
        time.wrapping_sub(h.wrapping_mul(tph).wrapping_add(m.wrapping_mul(tpm))).wrapping_div(1000
                                                                                                  as
                                                                                                  libc::c_int
                                                                                                  as
                                                                                                  libc::c_uint);
    *hours = h;
    *minutes = m;
    *seconds = s;
}
