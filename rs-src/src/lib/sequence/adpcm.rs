use ::libc;
#[no_mangle]
pub static mut index_adjust: [libc::c_char; 8] =
    [-(1 as libc::c_int) as libc::c_char, -(1 as libc::c_int) as libc::c_char,
     -(1 as libc::c_int) as libc::c_char, -(1 as libc::c_int) as libc::c_char,
     2 as libc::c_int as libc::c_char, 4 as libc::c_int as libc::c_char,
     6 as libc::c_int as libc::c_char, 8 as libc::c_int as libc::c_char];
#[no_mangle]
pub static mut step_size: [libc::c_short; 89] =
    [7 as libc::c_int as libc::c_short, 8 as libc::c_int as libc::c_short,
     9 as libc::c_int as libc::c_short, 10 as libc::c_int as libc::c_short,
     11 as libc::c_int as libc::c_short, 12 as libc::c_int as libc::c_short,
     13 as libc::c_int as libc::c_short, 14 as libc::c_int as libc::c_short,
     16 as libc::c_int as libc::c_short, 17 as libc::c_int as libc::c_short,
     19 as libc::c_int as libc::c_short, 21 as libc::c_int as libc::c_short,
     23 as libc::c_int as libc::c_short, 25 as libc::c_int as libc::c_short,
     28 as libc::c_int as libc::c_short, 31 as libc::c_int as libc::c_short,
     34 as libc::c_int as libc::c_short, 37 as libc::c_int as libc::c_short,
     41 as libc::c_int as libc::c_short, 45 as libc::c_int as libc::c_short,
     50 as libc::c_int as libc::c_short, 55 as libc::c_int as libc::c_short,
     60 as libc::c_int as libc::c_short, 66 as libc::c_int as libc::c_short,
     73 as libc::c_int as libc::c_short, 80 as libc::c_int as libc::c_short,
     88 as libc::c_int as libc::c_short, 97 as libc::c_int as libc::c_short,
     107 as libc::c_int as libc::c_short, 118 as libc::c_int as libc::c_short,
     130 as libc::c_int as libc::c_short, 143 as libc::c_int as libc::c_short,
     157 as libc::c_int as libc::c_short, 173 as libc::c_int as libc::c_short,
     190 as libc::c_int as libc::c_short, 209 as libc::c_int as libc::c_short,
     230 as libc::c_int as libc::c_short, 253 as libc::c_int as libc::c_short,
     279 as libc::c_int as libc::c_short, 307 as libc::c_int as libc::c_short,
     337 as libc::c_int as libc::c_short, 371 as libc::c_int as libc::c_short,
     408 as libc::c_int as libc::c_short, 449 as libc::c_int as libc::c_short,
     494 as libc::c_int as libc::c_short, 544 as libc::c_int as libc::c_short,
     598 as libc::c_int as libc::c_short, 658 as libc::c_int as libc::c_short,
     724 as libc::c_int as libc::c_short, 796 as libc::c_int as libc::c_short,
     876 as libc::c_int as libc::c_short, 963 as libc::c_int as libc::c_short,
     1060 as libc::c_int as libc::c_short,
     1166 as libc::c_int as libc::c_short,
     1282 as libc::c_int as libc::c_short,
     1411 as libc::c_int as libc::c_short,
     1552 as libc::c_int as libc::c_short,
     1707 as libc::c_int as libc::c_short,
     1878 as libc::c_int as libc::c_short,
     2066 as libc::c_int as libc::c_short,
     2272 as libc::c_int as libc::c_short,
     2499 as libc::c_int as libc::c_short,
     2749 as libc::c_int as libc::c_short,
     3024 as libc::c_int as libc::c_short,
     3327 as libc::c_int as libc::c_short,
     3660 as libc::c_int as libc::c_short,
     4026 as libc::c_int as libc::c_short,
     4428 as libc::c_int as libc::c_short,
     4871 as libc::c_int as libc::c_short,
     5358 as libc::c_int as libc::c_short,
     5894 as libc::c_int as libc::c_short,
     6484 as libc::c_int as libc::c_short,
     7132 as libc::c_int as libc::c_short,
     7845 as libc::c_int as libc::c_short,
     8630 as libc::c_int as libc::c_short,
     9493 as libc::c_int as libc::c_short,
     10442 as libc::c_int as libc::c_short,
     11487 as libc::c_int as libc::c_short,
     12635 as libc::c_int as libc::c_short,
     13899 as libc::c_int as libc::c_short,
     15289 as libc::c_int as libc::c_short,
     16818 as libc::c_int as libc::c_short,
     18500 as libc::c_int as libc::c_short,
     20350 as libc::c_int as libc::c_short,
     22385 as libc::c_int as libc::c_short,
     24623 as libc::c_int as libc::c_short,
     27086 as libc::c_int as libc::c_short,
     29794 as libc::c_int as libc::c_short,
     32767 as libc::c_int as libc::c_short];
#[no_mangle]
pub static mut pred_val: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut step_idx: libc::c_int = 0 as libc::c_int;
/* This code is "borrowed" from the ALSA library 
   http://www.alsa-project.org */
#[no_mangle]
pub unsafe extern "C" fn adpcm_decode_sample(mut code: libc::c_char)
 -> libc::c_short {
    let mut pred_diff: libc::c_short =
        0; /* Predicted difference to next sample */
    let mut step: libc::c_short = 0; /* holds previous step_size value */
    let mut sign: libc::c_char = 0;
    let mut i: libc::c_int = 0;
    /* Separate sign and magnitude */
    sign = (code as libc::c_int & 0x8 as libc::c_int) as libc::c_char;
    code = (code as libc::c_int & 0x7 as libc::c_int) as libc::c_char;
    /*
	 * Computes pred_diff = (code + 0.5) * step / 4,
	 * but see comment in adpcm_coder.
	 */
    step = step_size[step_idx as usize];
    /* Compute difference and new predicted value */
    pred_diff = (step as libc::c_int >> 3 as libc::c_int) as libc::c_short;
    i = 0x4 as libc::c_int;
    while i != 0 {
        if code as libc::c_int & i != 0 {
            pred_diff =
                (pred_diff as libc::c_int + step as libc::c_int) as
                    libc::c_short
        }
        i >>= 1 as libc::c_int;
        step = (step as libc::c_int >> 1 as libc::c_int) as libc::c_short
    }
    pred_val +=
        if sign as libc::c_int != 0 {
            -(pred_diff as libc::c_int)
        } else { pred_diff as libc::c_int };
    /* Clamp output value */
    if pred_val > 32767 as libc::c_int {
        pred_val = 32767 as libc::c_int
    } else if pred_val < -(32768 as libc::c_int) {
        pred_val = -(32768 as libc::c_int)
    }
    /* Find new step_size index value */
    step_idx += index_adjust[code as libc::c_int as usize] as libc::c_int;
    if step_idx < 0 as libc::c_int {
        step_idx = 0 as libc::c_int
    } else if step_idx > 88 as libc::c_int { step_idx = 88 as libc::c_int }
    return pred_val as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn adpcm_init() {
    pred_val = 0 as libc::c_int;
    step_idx = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn adpcm_decode(mut input: *mut libc::c_uchar,
                                      mut input_size: libc::c_uint,
                                      mut output: *mut *mut libc::c_short) {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < input_size {
        let mut two_samples: libc::c_uchar = *input.offset(i as isize);
        let fresh0 = *output;
        *output = (*output).offset(1);
        *fresh0 =
            adpcm_decode_sample((two_samples as libc::c_int >>
                                     4 as libc::c_int) as libc::c_char);
        let fresh1 = *output;
        *output = (*output).offset(1);
        *fresh1 =
            adpcm_decode_sample((two_samples as libc::c_int &
                                     15 as libc::c_int) as libc::c_char);
        i = i.wrapping_add(1)
    };
}
