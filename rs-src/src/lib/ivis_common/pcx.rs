use ::libc;
extern "C" {
    pub type png_struct_def;
    pub type png_info_def;
    /* Store the calling environment in ENV, not saving the signal mask.
   Return 0.  */
    #[no_mangle]
    fn _setjmp(_: *mut __jmp_buf_tag) -> libc::c_int;
    /* Do not save the signal mask.  This is equivalent to the `_setjmp'
   BSD function.  */
    /* Jump to the environment saved in ENV, making the
   `setjmp' call there return VAL, or 1 if VAL is 0.  */
    #[no_mangle]
    fn longjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;
    #[no_mangle]
    fn png_sig_cmp(sig: png_const_bytep, start: size_t, num_to_check: size_t)
     -> libc::c_int;
    #[no_mangle]
    fn png_create_read_struct(user_png_ver: png_const_charp,
                              error_ptr: png_voidp, error_fn: png_error_ptr,
                              warn_fn: png_error_ptr) -> png_structp;
    #[no_mangle]
    fn png_set_longjmp_fn(png_ptr: png_structrp, longjmp_fn: png_longjmp_ptr,
                          jmp_buf_size: size_t) -> *mut jmp_buf;
    #[no_mangle]
    fn png_create_info_struct(png_ptr: png_const_structrp) -> png_infop;
    #[no_mangle]
    fn png_read_info(png_ptr: png_structrp, info_ptr: png_inforp);
    #[no_mangle]
    fn png_set_expand(png_ptr: png_structrp);
    #[no_mangle]
    fn png_set_expand_gray_1_2_4_to_8(png_ptr: png_structrp);
    #[no_mangle]
    fn png_set_palette_to_rgb(png_ptr: png_structrp);
    #[no_mangle]
    fn png_set_tRNS_to_alpha(png_ptr: png_structrp);
    #[no_mangle]
    fn png_set_gray_to_rgb(png_ptr: png_structrp);
    #[no_mangle]
    fn png_set_filler(png_ptr: png_structrp, filler: png_uint_32,
                      flags: libc::c_int);
    #[no_mangle]
    fn png_set_packing(png_ptr: png_structrp);
    #[no_mangle]
    fn png_set_strip_16(png_ptr: png_structrp);
    #[no_mangle]
    fn png_read_update_info(png_ptr: png_structrp, info_ptr: png_inforp);
    #[no_mangle]
    fn png_read_image(png_ptr: png_structrp, image: png_bytepp);
    #[no_mangle]
    fn png_read_end(png_ptr: png_structrp, info_ptr: png_inforp);
    #[no_mangle]
    fn png_destroy_info_struct(png_ptr: png_const_structrp,
                               info_ptr_ptr: png_infopp);
    #[no_mangle]
    fn png_destroy_read_struct(png_ptr_ptr: png_structpp,
                               info_ptr_ptr: png_infopp,
                               end_info_ptr_ptr: png_infopp);
    #[no_mangle]
    fn png_set_read_fn(png_ptr: png_structrp, io_ptr: png_voidp,
                       read_data_fn: png_rw_ptr);
    #[no_mangle]
    fn png_get_io_ptr(png_ptr: png_const_structrp) -> png_voidp;
    #[no_mangle]
    fn png_get_IHDR(png_ptr: png_const_structrp, info_ptr: png_const_inforp,
                    width: *mut png_uint_32, height: *mut png_uint_32,
                    bit_depth: *mut libc::c_int, color_type: *mut libc::c_int,
                    interlace_method: *mut libc::c_int,
                    compression_method: *mut libc::c_int,
                    filter_method: *mut libc::c_int) -> png_uint_32;
    /* Copyright (C) 1991-2019 Free Software Foundation, Inc.
   This file is part of the GNU C Library.

   The GNU C Library is free software; you can redistribute it and/or
   modify it under the terms of the GNU Lesser General Public
   License as published by the Free Software Foundation; either
   version 2.1 of the License, or (at your option) any later version.

   The GNU C Library is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public
   License along with the GNU C Library; if not, see
   <http://www.gnu.org/licenses/>.  */
    /*
 *	ISO C99 Standard: 7.21 String handling	<string.h>
 */
    /* Get size_t and NULL from <stddef.h>.  */
    /* Tell the caller that we provide correct C++ prototypes.  */
    /* Copy N bytes of SRC to DEST.  */
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_uint)
     -> *mut libc::c_void;
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
    /* Use misc or X/Open.  */
    /* Allocate SIZE bytes of memory.  */
    #[no_mangle]
    fn malloc(_: libc::c_uint) -> *mut libc::c_void;
    /* Free a block allocated by `malloc', `realloc' or `calloc'.  */
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
}
pub type size_t = libc::c_uint;
/* Copyright (C) 2001-2019 Free Software Foundation, Inc.
   This file is part of the GNU C Library.

   The GNU C Library is free software; you can redistribute it and/or
   modify it under the terms of the GNU Lesser General Public
   License as published by the Free Software Foundation; either
   version 2.1 of the License, or (at your option) any later version.

   The GNU C Library is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public
   License along with the GNU C Library; if not, see
   <http://www.gnu.org/licenses/>.  */
/* Define the machine-dependent type `jmp_buf'.  x86-64 version.  */
pub type __jmp_buf = [libc::c_int; 6];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 32],
}
/* Copyright (C) 1991-2019 Free Software Foundation, Inc.
   This file is part of the GNU C Library.

   The GNU C Library is free software; you can redistribute it and/or
   modify it under the terms of the GNU Lesser General Public
   License as published by the Free Software Foundation; either
   version 2.1 of the License, or (at your option) any later version.

   The GNU C Library is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public
   License along with the GNU C Library; if not, see
   <http://www.gnu.org/licenses/>.  */
/*
 *	ISO C99 Standard: 7.13 Nonlocal jumps	<setjmp.h>
 */
/* Calling environment, plus possibly a saved signal mask.  */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
    pub __saved_mask: __sigset_t,
}
/* Saved signal mask.  */
pub type jmp_buf = [__jmp_buf_tag; 1];
/* pngconf.h - machine-configurable file for libpng
 *
 * libpng version 1.6.37
 *
 * Copyright (c) 2018-2019 Cosmin Truta
 * Copyright (c) 1998-2002,2004,2006-2016,2018 Glenn Randers-Pehrson
 * Copyright (c) 1996-1997 Andreas Dilger
 * Copyright (c) 1995-1996 Guy Eric Schalnat, Group 42, Inc.
 *
 * This code is released under the libpng license.
 * For conditions of distribution and use, see the disclaimer
 * and license in png.h
 *
 * Any machine specific code is near the front of this file, so if you
 * are configuring libpng for a machine, you may want to read the section
 * starting here down to where it starts to typedef png_color, png_text,
 * and png_info.
 */
/* else includes may cause problems */
/* From libpng 1.6.0 libpng requires an ANSI X3.159-1989 ("ISOC90") compliant C
 * compiler for correct compilation.  The following header files are required by
 * the standard.  If your compiler doesn't provide these header files, or they
 * do not match the standard, you will need to provide/improve them.
 */
/* Library header files.  These header files are all defined by ISOC90; libpng
 * expects conformant implementations, however, an ISOC90 conformant system need
 * not provide these header files if the functionality cannot be implemented.
 * In this case it will be necessary to disable the relevant parts of libpng in
 * the build of pnglibconf.h.
 *
 * Prior to 1.6.0 string.h was included here; the API changes in 1.6.0 to not
 * include this unnecessary header file.
 */
/* Required for the definition of FILE: */
/* Required for the definition of jmp_buf and the declaration of longjmp: */
/* Required for struct tm: */
/* PNG_BUILDING_SYMBOL_TABLE */
/* Prior to 1.6.0, it was possible to turn off 'const' in declarations,
 * using PNG_NO_CONST.  This is no longer supported.
 */
/* backward compatibility only */
/* This controls optimization of the reading of 16-bit and 32-bit
 * values from PNG files.  It can be set on a per-app-file basis: it
 * just changes whether a macro is used when the function is called.
 * The library builder sets the default; if read functions are not
 * built into the library the macro implementation is forced on.
 */
/* COMPILER SPECIFIC OPTIONS.
 *
 * These options are provided so that a variety of difficult compilers
 * can be used.  Some are fixed at build time (e.g. PNG_API_RULE
 * below) but still have compiler specific implementations, others
 * may be changed on a per-file basis when compiling against libpng.
 */
/* The PNGARG macro was used in versions of libpng prior to 1.6.0 to protect
 * against legacy (pre ISOC90) compilers that did not understand function
 * prototypes.  It is not required for modern C compilers.
 */
/* Function calling conventions.
 * =============================
 * Normally it is not necessary to specify to the compiler how to call
 * a function - it just does it - however on x86 systems derived from
 * Microsoft and Borland C compilers ('IBM PC', 'DOS', 'Windows' systems
 * and some others) there are multiple ways to call a function and the
 * default can be changed on the compiler command line.  For this reason
 * libpng specifies the calling convention of every exported function and
 * every function called via a user supplied function pointer.  This is
 * done in this file by defining the following macros:
 *
 * PNGAPI    Calling convention for exported functions.
 * PNGCBAPI  Calling convention for user provided (callback) functions.
 * PNGCAPI   Calling convention used by the ANSI-C library (required
 *           for longjmp callbacks and sometimes used internally to
 *           specify the calling convention for zlib).
 *
 * These macros should never be overridden.  If it is necessary to
 * change calling convention in a private build this can be done
 * by setting PNG_API_RULE (which defaults to 0) to one of the values
 * below to select the correct 'API' variants.
 *
 * PNG_API_RULE=0 Use PNGCAPI - the 'C' calling convention - throughout.
 *                This is correct in every known environment.
 * PNG_API_RULE=1 Use the operating system convention for PNGAPI and
 *                the 'C' calling convention (from PNGCAPI) for
 *                callbacks (PNGCBAPI).  This is no longer required
 *                in any known environment - if it has to be used
 *                please post an explanation of the problem to the
 *                libpng mailing list.
 *
 * These cases only differ if the operating system does not use the C
 * calling convention, at present this just means the above cases
 * (x86 DOS/Windows systems) and, even then, this does not apply to
 * Cygwin running on those systems.
 *
 * Note that the value must be defined in pnglibconf.h so that what
 * the application uses to call the library matches the conventions
 * set when building the library.
 */
/* Symbol export
 * =============
 * When building a shared library it is almost always necessary to tell
 * the compiler which symbols to export.  The png.h macro 'PNG_EXPORT'
 * is used to mark the symbols.  On some systems these symbols can be
 * extracted at link time and need no special processing by the compiler,
 * on other systems the symbols are flagged by the compiler and just
 * the declaration requires a special tag applied (unfortunately) in a
 * compiler dependent way.  Some systems can do either.
 *
 * A small number of older systems also require a symbol from a DLL to
 * be flagged to the program that calls it.  This is a problem because
 * we do not know in the header file included by application code that
 * the symbol will come from a shared library, as opposed to a statically
 * linked one.  For this reason the application must tell us by setting
 * the magic flag PNG_USE_DLL to turn on the special processing before
 * it includes png.h.
 *
 * Four additional macros are used to make this happen:
 *
 * PNG_IMPEXP The magic (if any) to cause a symbol to be exported from
 *            the build or imported if PNG_USE_DLL is set - compiler
 *            and system specific.
 *
 * PNG_EXPORT_TYPE(type) A macro that pre or appends PNG_IMPEXP to
 *                       'type', compiler specific.
 *
 * PNG_DLL_EXPORT Set to the magic to use during a libpng build to
 *                make a symbol exported from the DLL.  Not used in the
 *                public header files; see pngpriv.h for how it is used
 *                in the libpng build.
 *
 * PNG_DLL_IMPORT Set to the magic to force the libpng symbols to come
 *                from a DLL - used to define PNG_IMPEXP when
 *                PNG_USE_DLL is set.
 */
/* System specific discovery.
 * ==========================
 * This code is used at build time to find PNG_IMPEXP, the API settings
 * and PNG_EXPORT_TYPE(), it may also set a macro to indicate the DLL
 * import processing is possible.  On Windows systems it also sets
 * compiler-specific macros to the values required to change the calling
 * conventions of the various functions.
 */
/* !Windows */
/* !Windows/x86 && !OS/2 */
/* Use the defaults, or define PNG*API on the command line (but
    * this will have to be done for every compile!)
    */
/* other system, !OS/2 */
/* !Windows/x86 */
/* Now do all the defaulting . */
/* PNG_IMPEXP may be set on the compilation system command line or (if not set)
 * then in an internal header file when building the library, otherwise (when
 * using the library) it is set here.
 */
/* In 1.5.2 the definition of PNG_FUNCTION has been changed to always treat
 * 'attributes' as a storage class - the attributes go at the start of the
 * function definition, and attributes are always appended regardless of the
 * compiler.  This considerably simplifies these macros but may cause problems
 * if any compilers both need function attributes and fail to handle them as
 * a storage class (this is unlikely.)
 */
/* The ordinal value is only relevant when preprocessing png.h for symbol
    * table entries, so we discard it here.  See the .dfn files in the
    * scripts directory.
    */
/* ANSI-C (C90) does not permit a macro to be invoked with an empty argument,
 * so make something non-empty to satisfy the requirement:
 */
/*empty list*/
/* Use PNG_REMOVED to comment out a removed interface. */
/* Support for compiler specific function attributes.  These are used
 * so that where compiler support is available incorrect use of API
 * functions in png.h will generate compiler warnings.
 *
 * Added at libpng-1.2.41.
 */
/* Support for compiler specific function attributes.  These are used
   * so that where compiler support is available, incorrect use of API
   * functions in png.h will generate compiler warnings.  Added at libpng
   * version 1.2.41.  Disabling these removes the warnings but may also produce
   * less efficient code.
   */
/* Clang defines both __clang__ and __GNUC__. Check __clang__ first. */
/* PNG_PEDANTIC_WARNINGS */
/* A floating point API. */
/* No floating point APIs */
/* A fixed point API. */
/* No fixed point APIs */
/* Some typedefs to get us started.  These should be safe on most of the common
 * platforms.
 *
 * png_uint_32 and png_int_32 may, currently, be larger than required to hold a
 * 32-bit value however this is not normally advisable.
 *
 * png_uint_16 and png_int_16 should always be two bytes in size - this is
 * verified at library build time.
 *
 * png_byte must always be one byte in size.
 *
 * The checks below use constants from limits.h, as defined by the ISOC90
 * standard.
 */
pub type png_byte = libc::c_uchar;
pub type png_uint_32 = libc::c_uint;
/* Prior to 1.6.0, it was possible to disable the use of size_t and ptrdiff_t.
 * From 1.6.0 onwards, an ISO C90 compiler, as well as a standard-compliant
 * behavior of sizeof and ptrdiff_t are required.
 * The legacy typedefs are provided here for backwards compatibility.
 */
pub type png_size_t = size_t;
/* Add typedefs for pointers */
pub type png_voidp = *mut libc::c_void;
pub type png_bytep = *mut png_byte;
pub type png_const_bytep = *const png_byte;
pub type png_const_charp = *const libc::c_char;
/* Pointers to pointers; i.e. arrays */
pub type png_bytepp = *mut *mut png_byte;
pub type png_struct = png_struct_def;
pub type png_structp = *mut png_struct;
pub type png_structpp = *mut *mut png_struct;
pub type png_info = png_info_def;
/* png.h - header file for PNG reference library
 *
 * libpng version 1.6.37 - April 14, 2019
 *
 * Copyright (c) 2018-2019 Cosmin Truta
 * Copyright (c) 1998-2002,2004,2006-2018 Glenn Randers-Pehrson
 * Copyright (c) 1996-1997 Andreas Dilger
 * Copyright (c) 1995-1996 Guy Eric Schalnat, Group 42, Inc.
 *
 * This code is released under the libpng license. (See LICENSE, below.)
 *
 * Authors and maintainers:
 *   libpng versions 0.71, May 1995, through 0.88, January 1996: Guy Schalnat
 *   libpng versions 0.89, June 1996, through 0.96, May 1997: Andreas Dilger
 *   libpng versions 0.97, January 1998, through 1.6.35, July 2018:
 *     Glenn Randers-Pehrson
 *   libpng versions 1.6.36, December 2018, through 1.6.37, April 2019:
 *     Cosmin Truta
 *   See also "Contributing Authors", below.
 */
/*
 * COPYRIGHT NOTICE, DISCLAIMER, and LICENSE
 * =========================================
 *
 * PNG Reference Library License version 2
 * ---------------------------------------
 *
 *  * Copyright (c) 1995-2019 The PNG Reference Library Authors.
 *  * Copyright (c) 2018-2019 Cosmin Truta.
 *  * Copyright (c) 2000-2002, 2004, 2006-2018 Glenn Randers-Pehrson.
 *  * Copyright (c) 1996-1997 Andreas Dilger.
 *  * Copyright (c) 1995-1996 Guy Eric Schalnat, Group 42, Inc.
 *
 * The software is supplied "as is", without warranty of any kind,
 * express or implied, including, without limitation, the warranties
 * of merchantability, fitness for a particular purpose, title, and
 * non-infringement.  In no event shall the Copyright owners, or
 * anyone distributing the software, be liable for any damages or
 * other liability, whether in contract, tort or otherwise, arising
 * from, out of, or in connection with the software, or the use or
 * other dealings in the software, even if advised of the possibility
 * of such damage.
 *
 * Permission is hereby granted to use, copy, modify, and distribute
 * this software, or portions hereof, for any purpose, without fee,
 * subject to the following restrictions:
 *
 *  1. The origin of this software must not be misrepresented; you
 *     must not claim that you wrote the original software.  If you
 *     use this software in a product, an acknowledgment in the product
 *     documentation would be appreciated, but is not required.
 *
 *  2. Altered source versions must be plainly marked as such, and must
 *     not be misrepresented as being the original software.
 *
 *  3. This Copyright notice may not be removed or altered from any
 *     source or altered source distribution.
 *
 *
 * PNG Reference Library License version 1 (for libpng 0.5 through 1.6.35)
 * -----------------------------------------------------------------------
 *
 * libpng versions 1.0.7, July 1, 2000, through 1.6.35, July 15, 2018 are
 * Copyright (c) 2000-2002, 2004, 2006-2018 Glenn Randers-Pehrson, are
 * derived from libpng-1.0.6, and are distributed according to the same
 * disclaimer and license as libpng-1.0.6 with the following individuals
 * added to the list of Contributing Authors:
 *
 *     Simon-Pierre Cadieux
 *     Eric S. Raymond
 *     Mans Rullgard
 *     Cosmin Truta
 *     Gilles Vollant
 *     James Yu
 *     Mandar Sahastrabuddhe
 *     Google Inc.
 *     Vadim Barkov
 *
 * and with the following additions to the disclaimer:
 *
 *     There is no warranty against interference with your enjoyment of
 *     the library or against infringement.  There is no warranty that our
 *     efforts or the library will fulfill any of your particular purposes
 *     or needs.  This library is provided with all faults, and the entire
 *     risk of satisfactory quality, performance, accuracy, and effort is
 *     with the user.
 *
 * Some files in the "contrib" directory and some configure-generated
 * files that are distributed with libpng have other copyright owners, and
 * are released under other open source licenses.
 *
 * libpng versions 0.97, January 1998, through 1.0.6, March 20, 2000, are
 * Copyright (c) 1998-2000 Glenn Randers-Pehrson, are derived from
 * libpng-0.96, and are distributed according to the same disclaimer and
 * license as libpng-0.96, with the following individuals added to the
 * list of Contributing Authors:
 *
 *     Tom Lane
 *     Glenn Randers-Pehrson
 *     Willem van Schaik
 *
 * libpng versions 0.89, June 1996, through 0.96, May 1997, are
 * Copyright (c) 1996-1997 Andreas Dilger, are derived from libpng-0.88,
 * and are distributed according to the same disclaimer and license as
 * libpng-0.88, with the following individuals added to the list of
 * Contributing Authors:
 *
 *     John Bowler
 *     Kevin Bracey
 *     Sam Bushell
 *     Magnus Holmgren
 *     Greg Roelofs
 *     Tom Tanner
 *
 * Some files in the "scripts" directory have other copyright owners,
 * but are released under this license.
 *
 * libpng versions 0.5, May 1995, through 0.88, January 1996, are
 * Copyright (c) 1995-1996 Guy Eric Schalnat, Group 42, Inc.
 *
 * For the purposes of this copyright and license, "Contributing Authors"
 * is defined as the following set of individuals:
 *
 *     Andreas Dilger
 *     Dave Martindale
 *     Guy Eric Schalnat
 *     Paul Schmidt
 *     Tim Wegner
 *
 * The PNG Reference Library is supplied "AS IS".  The Contributing
 * Authors and Group 42, Inc. disclaim all warranties, expressed or
 * implied, including, without limitation, the warranties of
 * merchantability and of fitness for any purpose.  The Contributing
 * Authors and Group 42, Inc. assume no liability for direct, indirect,
 * incidental, special, exemplary, or consequential damages, which may
 * result from the use of the PNG Reference Library, even if advised of
 * the possibility of such damage.
 *
 * Permission is hereby granted to use, copy, modify, and distribute this
 * source code, or portions hereof, for any purpose, without fee, subject
 * to the following restrictions:
 *
 *  1. The origin of this source code must not be misrepresented.
 *
 *  2. Altered versions must be plainly marked as such and must not
 *     be misrepresented as being the original source.
 *
 *  3. This Copyright notice may not be removed or altered from any
 *     source or altered source distribution.
 *
 * The Contributing Authors and Group 42, Inc. specifically permit,
 * without fee, and encourage the use of this source code as a component
 * to supporting the PNG file format in commercial products.  If you use
 * this source code in a product, acknowledgment is not required but would
 * be appreciated.
 *
 * END OF COPYRIGHT NOTICE, DISCLAIMER, and LICENSE.
 *
 * TRADEMARK
 * =========
 *
 * The name "libpng" has not been registered by the Copyright owners
 * as a trademark in any jurisdiction.  However, because libpng has
 * been distributed and maintained world-wide, continually since 1995,
 * the Copyright owners claim "common-law trademark protection" in any
 * jurisdiction where common-law trademark is recognized.
 */
/*
 * A "png_get_copyright" function is available, for convenient use in "about"
 * boxes and the like:
 *
 *    printf("%s", png_get_copyright(NULL));
 *
 * Also, the PNG logo (in PNG format, of course) is supplied in the
 * files "pngbar.png" and "pngbar.jpg (88x31) and "pngnow.png" (98x31).
 */
/*
 * The contributing authors would like to thank all those who helped
 * with testing, bug fixes, and patience.  This wouldn't have been
 * possible without all of you.
 *
 * Thanks to Frank J. T. Wojcik for helping with the documentation.
 */
/* Note about libpng version numbers:
 *
 *    Due to various miscommunications, unforeseen code incompatibilities
 *    and occasional factors outside the authors' control, version numbering
 *    on the library has not always been consistent and straightforward.
 *    The following table summarizes matters since version 0.89c, which was
 *    the first widely used release:
 *
 *    source                 png.h  png.h  shared-lib
 *    version                string   int  version
 *    -------                ------ -----  ----------
 *    0.89c "1.0 beta 3"     0.89      89  1.0.89
 *    0.90  "1.0 beta 4"     0.90      90  0.90  [should have been 2.0.90]
 *    0.95  "1.0 beta 5"     0.95      95  0.95  [should have been 2.0.95]
 *    0.96  "1.0 beta 6"     0.96      96  0.96  [should have been 2.0.96]
 *    0.97b "1.00.97 beta 7" 1.00.97   97  1.0.1 [should have been 2.0.97]
 *    0.97c                  0.97      97  2.0.97
 *    0.98                   0.98      98  2.0.98
 *    0.99                   0.99      98  2.0.99
 *    0.99a-m                0.99      99  2.0.99
 *    1.00                   1.00     100  2.1.0 [100 should be 10000]
 *    1.0.0      (from here on, the   100  2.1.0 [100 should be 10000]
 *    1.0.1       png.h string is   10001  2.1.0
 *    1.0.1a-e    identical to the  10002  from here on, the shared library
 *    1.0.2       source version)   10002  is 2.V where V is the source code
 *    1.0.2a-b                      10003  version, except as noted.
 *    1.0.3                         10003
 *    1.0.3a-d                      10004
 *    1.0.4                         10004
 *    1.0.4a-f                      10005
 *    1.0.5 (+ 2 patches)           10005
 *    1.0.5a-d                      10006
 *    1.0.5e-r                      10100 (not source compatible)
 *    1.0.5s-v                      10006 (not binary compatible)
 *    1.0.6 (+ 3 patches)           10006 (still binary incompatible)
 *    1.0.6d-f                      10007 (still binary incompatible)
 *    1.0.6g                        10007
 *    1.0.6h                        10007  10.6h (testing xy.z so-numbering)
 *    1.0.6i                        10007  10.6i
 *    1.0.6j                        10007  2.1.0.6j (incompatible with 1.0.0)
 *    1.0.7beta11-14        DLLNUM  10007  2.1.0.7beta11-14 (binary compatible)
 *    1.0.7beta15-18           1    10007  2.1.0.7beta15-18 (binary compatible)
 *    1.0.7rc1-2               1    10007  2.1.0.7rc1-2 (binary compatible)
 *    1.0.7                    1    10007  (still compatible)
 *    ...
 *    1.0.69                  10    10069  10.so.0.69[.0]
 *    ...
 *    1.2.59                  13    10259  12.so.0.59[.0]
 *    ...
 *    1.4.20                  14    10420  14.so.0.20[.0]
 *    ...
 *    1.5.30                  15    10530  15.so.15.30[.0]
 *    ...
 *    1.6.37                  16    10637  16.so.16.37[.0]
 *
 *    Henceforth the source version will match the shared-library major and
 *    minor numbers; the shared-library major version number will be used for
 *    changes in backward compatibility, as it is intended.
 *    The PNG_LIBPNG_VER macro, which is not used within libpng but is
 *    available for applications, is an unsigned integer of the form XYYZZ
 *    corresponding to the source version X.Y.Z (leading zeros in Y and Z).
 *    Beta versions were given the previous public release number plus a
 *    letter, until version 1.0.6j; from then on they were given the upcoming
 *    public release number plus "betaNN" or "rcNN".
 *
 *    Binary incompatibility exists only when applications make direct access
 *    to the info_ptr or png_ptr members through png.h, and the compiled
 *    application is loaded with a different version of the library.
 *
 *    DLLNUM will change each time there are forward or backward changes
 *    in binary compatibility (e.g., when a new feature is added).
 *
 * See libpng.txt or libpng.3 for more information.  The PNG specification
 * is available as a W3C Recommendation and as an ISO/IEC Standard; see
 * <https://www.w3.org/TR/2003/REC-PNG-20031110/>
 */
/* This is not the place to learn how to use libpng. The file libpng-manual.txt
 * describes how to use libpng, and the file example.c summarizes it
 * with some code on which to build.  This file is useful for looking
 * at the actual function definitions and structure components.  If that
 * file has been stripped from your copy of libpng, you can find it at
 * <http://www.libpng.org/pub/png/libpng-manual.txt>
 *
 * If you just need to read a PNG file and don't want to read the documentation
 * skip to the end of this file and read the section entitled 'simplified API'.
 */
/* Version information for png.h - this should match the version in png.c */
/* These should match the first 3 components of PNG_LIBPNG_VER_STRING: */
/* This should be zero for a public release, or non-zero for a
 * development version.  [Deprecated]
 */
/* Release Status */
/* Release-Specific Flags */
/* Can be OR'ed with
                                       PNG_LIBPNG_BUILD_STABLE only */
/* Cannot be OR'ed with
                                       PNG_LIBPNG_BUILD_SPECIAL */
/* Cannot be OR'ed with
                                       PNG_LIBPNG_BUILD_PRIVATE */
/* Careful here.  At one time, Guy wanted to use 082, but that
 * would be octal.  We must not include leading zeros.
 * Versions 0.7 through 1.0.0 were in the range 0 to 100 here
 * (only version 1.0.0 was mis-numbered 100 instead of 10000).
 * From version 1.0.1 it is:
 * XXYYZZ, where XX=major, YY=minor, ZZ=release
 */
/* 1.6.37 */
/* Library configuration: these options cannot be changed after
 * the library has been built.
 */
/* If pnglibconf.h is missing, you can
 * copy scripts/pnglibconf.h.prebuilt to pnglibconf.h
 */
/* Machine specific configuration. */
/*
 * Added at libpng-1.2.8
 *
 * Ref MSDN: Private as priority over Special
 * VS_FF_PRIVATEBUILD File *was not* built using standard release
 * procedures. If this value is given, the StringFileInfo block must
 * contain a PrivateBuild string.
 *
 * VS_FF_SPECIALBUILD File *was* built by the original company using
 * standard release procedures but is a variation of the standard
 * file of the same version number. If this value is given, the
 * StringFileInfo block must contain a SpecialBuild string.
 */
/* From pnglibconf.h */
/* Inhibit C++ name-mangling for libpng functions but not for system calls. */
/* __cplusplus */
/* Version information for C files, stored in png.c.  This had better match
 * the version above.
 */
/* This file is arranged in several sections:
 *
 * 1. [omitted]
 * 2. Any configuration options that can be specified by for the application
 *    code when it is built.  (Build time configuration is in pnglibconf.h)
 * 3. Type definitions (base types are defined in pngconf.h), structure
 *    definitions.
 * 4. Exported library functions.
 * 5. Simplified API.
 * 6. Implementation options.
 *
 * The library source code has additional files (principally pngpriv.h) that
 * allow configuration of the library.
 */
/* Section 1: [omitted] */
/* Section 2: run time configuration
 * See pnglibconf.h for build time configuration
 *
 * Run time configuration allows the application to choose between
 * implementations of certain arithmetic APIs.  The default is set
 * at build time and recorded in pnglibconf.h, but it is safe to
 * override these (and only these) settings.  Note that this won't
 * change what the library does, only application code, and the
 * settings can (and probably should) be made on a per-file basis
 * by setting the #defines before including png.h
 *
 * Use macros to read integers from PNG data or use the exported
 * functions?
 *   PNG_USE_READ_MACROS: use the macros (see below)  Note that
 *     the macros evaluate their argument multiple times.
 *   PNG_NO_USE_READ_MACROS: call the relevant library function.
 *
 * Use the alternative algorithm for compositing alpha samples that
 * does not use division?
 *   PNG_READ_COMPOSITE_NODIV_SUPPORTED: use the 'no division'
 *      algorithm.
 *   PNG_NO_READ_COMPOSITE_NODIV: use the 'division' algorithm.
 *
 * How to handle benign errors if PNG_ALLOW_BENIGN_ERRORS is
 * false?
 *   PNG_ALLOW_BENIGN_ERRORS: map calls to the benign error
 *      APIs to png_warning.
 * Otherwise the calls are mapped to png_error.
 */
/* Section 3: type definitions, including structures and compile time
 * constants.
 * See pngconf.h for base types that vary by machine/system
 */
/* dispose_op flags from inside fcTL */
/* blend_op flags from inside fcTL */
/* PNG_APNG_SUPPORTED */
/* This triggers a compiler error in png.c, if png.c and png.h
 * do not agree upon the version number.
 */
/* Basic control structions.  Read libpng-manual.txt or libpng.3 for more info.
 *
 * png_struct is the cache of information used while reading or writing a single
 * PNG file.  One of these is always required, although the simplified API
 * (below) hides the creation and destruction of it.
 */
/* png_info contains information read from or to be written to a PNG file.  One
 * or more of these must exist while reading or creating a PNG file.  The
 * information is not used by libpng during read but is used to control what
 * gets written when a PNG file is created.  "png_get_" function calls read
 * information during read and "png_set_" functions calls write information
 * when creating a PNG.
 * been moved into a separate header file that is not accessible to
 * applications.  Read libpng-manual.txt or libpng.3 for more info.
 */
pub type png_infop = *mut png_info;
pub type png_infopp = *mut *mut png_info;
/* Types with names ending 'p' are pointer types.  The corresponding types with
 * names ending 'rp' are identical pointer types except that the pointer is
 * marked 'restrict', which means that it is the only pointer to the object
 * passed to the function.  Applications should not use the 'restrict' types;
 * it is always valid to pass 'p' to a pointer with a function argument of the
 * corresponding 'rp' type.  Different compilers have different rules with
 * regard to type matching in the presence of 'restrict'.  For backward
 * compatibility libpng callbacks never have 'restrict' in their parameters and,
 * consequentially, writing portable application code is extremely difficult if
 * an attempt is made to use 'restrict'.
 */
pub type png_structrp = *mut png_struct;
pub type png_const_structrp = *const png_struct;
pub type png_inforp = *mut png_info;
pub type png_const_inforp = *const png_info;
pub type png_error_ptr
    =
    Option<unsafe extern "C" fn(_: png_structp, _: png_const_charp) -> ()>;
pub type png_rw_ptr
    =
    Option<unsafe extern "C" fn(_: png_structp, _: png_bytep, _: size_t)
               -> ()>;
pub type png_longjmp_ptr
    =
    Option<unsafe extern "C" fn(_: *mut __jmp_buf_tag, _: libc::c_int) -> ()>;
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
pub type uint8 = libc::c_uchar;
pub type iBitmap = uint8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iColour {
    pub r: uint8,
    pub g: uint8,
    pub b: uint8,
}
pub type iBool = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iSprite {
    pub width: libc::c_uint,
    pub height: libc::c_uint,
    pub bmp: *mut iBitmap,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wzpng_io_buf {
    pub length: png_size_t,
    pub buffer: *mut libc::c_char,
}
unsafe extern "C" fn wzpng_read_data(mut ctx: png_structp,
                                     mut area: png_bytep,
                                     mut size: png_size_t) {
    let mut buf: *mut wzpng_io_buf =
        png_get_io_ptr(ctx as *const png_struct) as *mut wzpng_io_buf;
    if size <= (*buf).length {
        memcpy(area as *mut libc::c_void,
               (*buf).buffer as *const libc::c_void, size);
        (*buf).buffer = (*buf).buffer.offset(size as isize);
        (*buf).length =
            ((*buf).length as libc::c_uint).wrapping_sub(size) as png_size_t
                as png_size_t
    };
}
#[no_mangle]
pub unsafe extern "C" fn pie_PNGLoadMem(mut pngimage: *mut libc::c_char,
                                        mut s: *mut iSprite,
                                        mut pal: *mut iColour) -> iBool {
    let mut info_ptr: png_infop = 0 as *mut png_info;
    let mut PNG_BYTES_TO_CHECK: libc::c_uint = 0;
    let mut png_ptr: png_structp = 0 as png_structp;
    let mut buf: *mut wzpng_io_buf =
        malloc(::std::mem::size_of::<wzpng_io_buf>() as libc::c_ulong) as
            *mut wzpng_io_buf;
    (*buf).buffer = pngimage;
    (*buf).length = 10000000 as libc::c_int as png_size_t;
    PNG_BYTES_TO_CHECK = 4 as libc::c_int as libc::c_uint;
    if png_sig_cmp(pngimage as *mut png_byte as png_const_bytep,
                   0 as libc::c_int as png_size_t, PNG_BYTES_TO_CHECK) != 0 {
        debug(LOG_3D,
              b"pie_PNGLoadMem: Did not recognize PNG header in buffer\x00" as
                  *const u8 as *const libc::c_char);
    } else {
        png_ptr =
            png_create_read_struct(b"1.6.37\x00" as *const u8 as
                                       *const libc::c_char,
                                   0 as *mut libc::c_void, None, None);
        if png_ptr.is_null() {
            debug(LOG_3D,
                  b"pie_PNGLoadMem: Unable to create png struct\x00" as
                      *const u8 as *const libc::c_char);
        } else {
            info_ptr = png_create_info_struct(png_ptr as *const png_struct);
            if info_ptr.is_null() {
                debug(LOG_3D,
                      b"pie_PNGLoadMem: Unable to create png info struct\x00"
                          as *const u8 as *const libc::c_char);
            } else if _setjmp((*png_set_longjmp_fn(png_ptr,
                                                   ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                                           *mut __jmp_buf_tag,
                                                                                                       _:
                                                                                                           libc::c_int)
                                                                                      ->
                                                                                          !>,
                                                                           png_longjmp_ptr>(Some(longjmp
                                                                                                     as
                                                                                                     unsafe extern "C" fn(_:
                                                                                                                              *mut __jmp_buf_tag,
                                                                                                                          _:
                                                                                                                              libc::c_int)
                                                                                                         ->
                                                                                                             !)),
                                                   ::std::mem::size_of::<jmp_buf>()
                                                       as
                                                       libc::c_ulong)).as_mut_ptr())
                          != 0 {
                debug(LOG_3D,
                      b"pie_PNGLoadMem: Error decoding PNG data\x00" as
                          *const u8 as *const libc::c_char);
            } else {
                let mut bit_depth: libc::c_int = 0;
                let mut color_type: libc::c_int = 0;
                let mut interlace_type: libc::c_int = 0;
                let mut width: png_uint_32 = 0;
                let mut height: png_uint_32 = 0;
                /* Set up the input control */
                png_set_read_fn(png_ptr, buf as png_voidp,
                                Some(wzpng_read_data as
                                         unsafe extern "C" fn(_: png_structp,
                                                              _: png_bytep,
                                                              _: png_size_t)
                                             -> ()));
                /* Read PNG header info */
                png_read_info(png_ptr, info_ptr);
                png_get_IHDR(png_ptr as *const png_struct,
                             info_ptr as *const png_info, &mut width,
                             &mut height, &mut bit_depth, &mut color_type,
                             &mut interlace_type, 0 as *mut libc::c_int,
                             0 as *mut libc::c_int);
                /* tell libpng to strip 16 bit/color files down to 8 bits/color */
                png_set_strip_16(png_ptr);
                /* Extract multiple pixels with bit depths of 1, 2, and 4 from a single
		 * byte into separate bytes (useful for paletted and grayscale images).
		 */
                png_set_packing(png_ptr);
                /* More transformations to ensure we end up with 32bpp, 4 channel RGBA */
                png_set_gray_to_rgb(png_ptr);
                png_set_palette_to_rgb(png_ptr);
                png_set_tRNS_to_alpha(png_ptr);
                png_set_filler(png_ptr, 0xff as libc::c_int as png_uint_32,
                               1 as libc::c_int);
                //TODO FIXME CRITICAL
    //png_set_gray_1_2_4_to_8(png_ptr);
    // is deprecated, instead we use ..expand.. which apparently handles alpha differently.
                png_set_expand_gray_1_2_4_to_8(png_ptr);
                /* scale greyscale values to the range 0..255 */
                if color_type == 0 as libc::c_int { png_set_expand(png_ptr); }
                png_read_update_info(png_ptr, info_ptr);
                let mut w: png_uint_32 = 0;
                let mut h: png_uint_32 = 0;
                png_get_IHDR(png_ptr as *const png_struct,
                             info_ptr as *const png_info,
                             &mut w as *mut png_uint_32,
                             &mut h as *mut png_uint_32, &mut bit_depth,
                             &mut color_type, &mut interlace_type,
                             0 as *mut libc::c_int, 0 as *mut libc::c_int);
                (*s).width = w;
                (*s).height = h;
                // Freeing s->bmp before allocating new mem would give a HEAP error on Windows (Invalid Address specified to RtlFreeHeap( x, x )).
      //TODO FIXME CRITICAL
			//s->bmp = malloc(w*h*info_ptr->channels);
                (*s).bmp =
                    malloc(w.wrapping_mul(h).wrapping_mul(4 as libc::c_int as
                                                              libc::c_uint))
                        as *mut iBitmap;
                let mut row_pointers: *mut png_bytep =
                    malloc((*s).height.wrapping_mul(::std::mem::size_of::<png_bytep>()
                                                        as libc::c_ulong)) as
                        *mut png_bytep;
                let mut pdata: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                let mut i: libc::c_uint = 0;
                //TODO FIXME CRITICAL
			//const unsigned int line_size = s->width*info_ptr->channels;
                let line_size: libc::c_uint =
                    (*s).width.wrapping_mul(4 as libc::c_int as
                                                libc::c_uint); //s->width*info_ptr->channels;
                i = 0 as libc::c_int as libc::c_uint;
                pdata = (*s).bmp;
                while i < (*s).height {
                    let ref mut fresh0 = *row_pointers.offset(i as isize);
                    *fresh0 = pdata as png_bytep;
                    i = i.wrapping_add(1);
                    pdata = pdata.offset(line_size as isize)
                }
                /* Read the entire image in one go */
                png_read_image(png_ptr, row_pointers);
                free(row_pointers as *mut libc::c_void);
                /* read rest of file, get additional chunks in info_ptr - REQUIRED */
                png_read_end(png_ptr, info_ptr);
                if !info_ptr.is_null() {
                    png_destroy_info_struct(png_ptr as *const png_struct,
                                            &mut info_ptr);
                }
                if !png_ptr.is_null() {
                    png_destroy_read_struct(&mut png_ptr, 0 as png_infopp,
                                            0 as png_infopp);
                }
                free(buf as *mut libc::c_void);
                return 1 as libc::c_int
            }
        }
    }
    if !info_ptr.is_null() {
        png_destroy_info_struct(png_ptr as *const png_struct, &mut info_ptr);
    }
    if !png_ptr.is_null() {
        png_destroy_read_struct(&mut png_ptr, 0 as png_infopp,
                                0 as png_infopp);
    }
    free(buf as *mut libc::c_void);
    return 0 as libc::c_int;
}
// PNG
#[no_mangle]
pub unsafe extern "C" fn pie_PNGLoadMemToBuffer(mut pngimage:
                                                    *mut libc::c_char,
                                                mut s: *mut iSprite,
                                                mut pal: *mut iColour)
 -> BOOL {
    return pie_PNGLoadMem(pngimage, s, pal);
}
