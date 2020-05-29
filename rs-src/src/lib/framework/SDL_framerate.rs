use ::libc;
extern "C" {
    /*
    SDL - Simple DirectMedia Layer
    Copyright (C) 1997-2012 Sam Lantinga

    This library is free software; you can redistribute it and/or
    modify it under the terms of the GNU Lesser General Public
    License as published by the Free Software Foundation; either
    version 2.1 of the License, or (at your option) any later version.

    This library is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
    Lesser General Public License for more details.

    You should have received a copy of the GNU Lesser General Public
    License along with this library; if not, write to the Free Software
    Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA

    Sam Lantinga
    slouken@libsdl.org
*/
    /* * @file SDL_timer.h
 *  Header for the SDL time management routines
 */
    /* Set up for C function definitions, even when using C++ */
    /* * This is the OS scheduler timeslice, in milliseconds */
    /* * This is the maximum resolution of the SDL timer on all platforms */
    /* *< Experimentally determined */
    /* *
 * Get the number of milliseconds since the SDL library initialization.
 * Note that this value wraps if the program runs for more than ~49 days.
 */
    #[no_mangle]
    fn SDL_GetTicks() -> Uint32;
    /* * Wait a specified number of milliseconds before returning */
    #[no_mangle]
    fn SDL_Delay(ms: Uint32);
}
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type Uint32 = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FPSmanager {
    pub framecount: Uint32,
    pub rateticks: libc::c_float,
    pub lastticks: Uint32,
    pub rate: Uint32,
}
/*

 SDL_framerate: framerate manager

 LGPL (c) A. Schiffler
 
 */
/* 
   Initialize the framerate manager
*/
#[no_mangle]
pub unsafe extern "C" fn SDL_initFramerate(mut manager: *mut FPSmanager) {
    /*
     * Store some sane values 
     */
    (*manager).framecount = 0 as libc::c_int as Uint32;
    (*manager).rate = 30 as libc::c_int as Uint32;
    (*manager).rateticks =
        (1000.0f64 / 30 as libc::c_int as libc::c_float as libc::c_double) as
            libc::c_float;
    (*manager).lastticks = SDL_GetTicks();
}
/* 
   Set the framerate in Hz 
*/
#[no_mangle]
pub unsafe extern "C" fn SDL_setFramerate(mut manager: *mut FPSmanager,
                                          mut rate: libc::c_int)
 -> libc::c_int {
    if rate >= 1 as libc::c_int && rate <= 200 as libc::c_int {
        (*manager).framecount = 0 as libc::c_int as Uint32;
        (*manager).rate = rate as Uint32;
        (*manager).rateticks =
            (1000.0f64 / rate as libc::c_float as libc::c_double) as
                libc::c_float;
        return 0 as libc::c_int
    } else { return -(1 as libc::c_int) };
}
/* 
  Return the current target framerate in Hz 
*/
#[no_mangle]
pub unsafe extern "C" fn SDL_getFramerate(mut manager: *mut FPSmanager)
 -> libc::c_int {
    if manager.is_null() {
        return -(1 as libc::c_int)
    } else { return (*manager).rate as libc::c_int };
}
/*

 SDL_framerate: framerate manager
 
 LGPL (c) A. Schiffler
 
 */
/* Set up for C function definitions, even when using C++ */
/* --- */
/* --------- Definitions */
/* Some rates in Hz */
/* --------- Structure variables */
/* Functions return 0 or value for sucess and -1 for error */
/* 
  Delay execution to maintain a constant framerate. Calculate fps.
*/
#[no_mangle]
pub unsafe extern "C" fn SDL_framerateDelay(mut manager: *mut FPSmanager) {
    let mut current_ticks: Uint32 = 0;
    let mut target_ticks: Uint32 = 0;
    let mut the_delay: Uint32 = 0;
    /*
     * Next frame 
     */
    (*manager).framecount = (*manager).framecount.wrapping_add(1);
    /*
     * Get/calc ticks 
     */
    current_ticks = SDL_GetTicks();
    target_ticks =
        (*manager).lastticks.wrapping_add(((*manager).framecount as
                                               libc::c_float *
                                               (*manager).rateticks) as
                                              Uint32);
    if current_ticks <= target_ticks {
        the_delay = target_ticks.wrapping_sub(current_ticks);
        SDL_Delay(the_delay);
    } else {
        (*manager).framecount = 0 as libc::c_int as Uint32;
        (*manager).lastticks = SDL_GetTicks()
    };
}
