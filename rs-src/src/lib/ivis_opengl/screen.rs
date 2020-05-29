use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type SDL_BlitMap;
    pub type private_hwdata;
    pub type jvirt_barray_control;
    pub type jvirt_sarray_control;
    pub type jpeg_entropy_encoder;
    pub type jpeg_forward_dct;
    pub type jpeg_downsampler;
    pub type jpeg_color_converter;
    pub type jpeg_marker_writer;
    pub type jpeg_c_coef_controller;
    pub type jpeg_c_prep_controller;
    pub type jpeg_c_main_controller;
    pub type jpeg_comp_master;
    pub type jpeg_color_quantizer;
    pub type jpeg_color_deconverter;
    pub type jpeg_upsampler;
    pub type jpeg_inverse_dct;
    pub type jpeg_entropy_decoder;
    pub type jpeg_marker_reader;
    pub type jpeg_input_controller;
    pub type jpeg_d_post_controller;
    pub type jpeg_d_coef_controller;
    pub type jpeg_d_main_controller;
    pub type jpeg_decomp_master;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_uint) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn SDL_GetError() -> *mut libc::c_char;
    #[no_mangle]
    fn SDL_GetVideoInfo() -> *const SDL_VideoInfo;
    #[no_mangle]
    fn SDL_VideoModeOK(width: libc::c_int, height: libc::c_int,
                       bpp: libc::c_int, flags: Uint32) -> libc::c_int;
    #[no_mangle]
    fn SDL_SetVideoMode(width: libc::c_int, height: libc::c_int,
                        bpp: libc::c_int, flags: Uint32) -> *mut SDL_Surface;
    #[no_mangle]
    fn SDL_SetPalette(surface: *mut SDL_Surface, flags: libc::c_int,
                      colors: *mut SDL_Color, firstcolor: libc::c_int,
                      ncolors: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn SDL_FreeSurface(surface: *mut SDL_Surface);
    #[no_mangle]
    fn SDL_GL_SetAttribute(attr: SDL_GLattr, value: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn SDL_WM_ToggleFullScreen(surface: *mut SDL_Surface) -> libc::c_int;
    #[no_mangle]
    fn glCullFace(mode: GLenum);
    #[no_mangle]
    fn glEnable(cap: GLenum);
    #[no_mangle]
    fn glDisable(cap: GLenum);
    #[no_mangle]
    fn glDepthMask(flag: GLboolean);
    #[no_mangle]
    fn glMatrixMode(mode: GLenum);
    #[no_mangle]
    fn glOrtho(left: GLdouble, right: GLdouble, bottom: GLdouble,
               top: GLdouble, near_val: GLdouble, far_val: GLdouble);
    #[no_mangle]
    fn glViewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei);
    #[no_mangle]
    fn glPushMatrix();
    #[no_mangle]
    fn glLoadIdentity();
    #[no_mangle]
    fn glScalef(x: GLfloat, y: GLfloat, z: GLfloat);
    #[no_mangle]
    fn glBegin(mode: GLenum);
    #[no_mangle]
    fn glEnd();
    #[no_mangle]
    fn glVertex2f(x: GLfloat, y: GLfloat);
    #[no_mangle]
    fn glColor3f(red: GLfloat, green: GLfloat, blue: GLfloat);
    #[no_mangle]
    fn glGenTextures(n: GLsizei, textures: *mut GLuint);
    #[no_mangle]
    fn glTexImage2D(target: GLenum, level: GLint, internalFormat: GLint,
                    width: GLsizei, height: GLsizei, border: GLint,
                    format: GLenum, type_0: GLenum,
                    pixels: *const libc::c_void);
    #[no_mangle]
    fn glTexParameteri(target: GLenum, pname: GLenum, param: GLint);
    #[no_mangle]
    fn glTexEnvf(target: GLenum, pname: GLenum, param: GLfloat);
    #[no_mangle]
    fn glReadPixels(x: GLint, y: GLint, width: GLsizei, height: GLsizei,
                    format: GLenum, type_0: GLenum,
                    pixels: *mut libc::c_void);
    #[no_mangle]
    fn glBindTexture(target: GLenum, texture: GLuint);
    #[no_mangle]
    fn glTexCoord2f(s: GLfloat, t: GLfloat);
    #[no_mangle]
    fn jpeg_std_error(err: *mut jpeg_error_mgr) -> *mut jpeg_error_mgr;
    #[no_mangle]
    fn jpeg_CreateCompress(cinfo: j_compress_ptr, version: libc::c_int,
                           structsize: size_t);
    #[no_mangle]
    fn jpeg_CreateDecompress(cinfo: j_decompress_ptr, version: libc::c_int,
                             structsize: size_t);
    #[no_mangle]
    fn jpeg_destroy_compress(cinfo: j_compress_ptr);
    #[no_mangle]
    fn jpeg_destroy_decompress(cinfo: j_decompress_ptr);
    #[no_mangle]
    fn jpeg_set_defaults(cinfo: j_compress_ptr);
    #[no_mangle]
    fn jpeg_set_quality(cinfo: j_compress_ptr, quality: libc::c_int,
                        force_baseline: boolean);
    #[no_mangle]
    fn jpeg_start_compress(cinfo: j_compress_ptr, write_all_tables: boolean);
    #[no_mangle]
    fn jpeg_write_scanlines(cinfo: j_compress_ptr, scanlines: JSAMPARRAY,
                            num_lines: JDIMENSION) -> JDIMENSION;
    #[no_mangle]
    fn jpeg_finish_compress(cinfo: j_compress_ptr);
    #[no_mangle]
    fn jpeg_read_header(cinfo: j_decompress_ptr, require_image: boolean)
     -> libc::c_int;
    #[no_mangle]
    fn jpeg_start_decompress(cinfo: j_decompress_ptr) -> boolean;
    #[no_mangle]
    fn jpeg_read_scanlines(cinfo: j_decompress_ptr, scanlines: JSAMPARRAY,
                           max_lines: JDIMENSION) -> JDIMENSION;
    #[no_mangle]
    fn jpeg_finish_decompress(cinfo: j_decompress_ptr) -> boolean;
    #[no_mangle]
    fn jpeg_calc_output_dimensions(cinfo: j_decompress_ptr);
    #[no_mangle]
    fn jpeg_resync_to_restart(cinfo: j_decompress_ptr, desired: libc::c_int)
     -> boolean;
    #[no_mangle]
    fn PHYSFS_exists(fname: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn PHYSFS_openWrite(filename: *const libc::c_char) -> *mut PHYSFS_File;
    #[no_mangle]
    fn PHYSFS_close(handle: *mut PHYSFS_File) -> libc::c_int;
    #[no_mangle]
    fn PHYSFS_write(handle: *mut PHYSFS_File, buffer: *const libc::c_void,
                    objSize: PHYSFS_uint32, objCount: PHYSFS_uint32)
     -> PHYSFS_sint64;
    #[no_mangle]
    fn _setjmp(_: *mut __jmp_buf_tag) -> libc::c_int;
    #[no_mangle]
    fn longjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;
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
    fn loadFile(pFileName: *const libc::c_char,
                ppFileData: *mut *mut libc::c_char, pFileSize: *mut UDWORD)
     -> BOOL;
    //render states
    #[no_mangle]
    fn pie_SetTexturePage(num: SDWORD);
}
pub type size_t = libc::c_uint;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type int16_t = __int16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 32],
}
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uintptr_t = libc::c_uint;
pub type Uint8 = uint8_t;
pub type Sint16 = int16_t;
pub type Uint16 = uint16_t;
pub type Uint32 = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_Rect {
    pub x: Sint16,
    pub y: Sint16,
    pub w: Uint16,
    pub h: Uint16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_Color {
    pub r: Uint8,
    pub g: Uint8,
    pub b: Uint8,
    pub unused: Uint8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_Palette {
    pub ncolors: libc::c_int,
    pub colors: *mut SDL_Color,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_PixelFormat {
    pub palette: *mut SDL_Palette,
    pub BitsPerPixel: Uint8,
    pub BytesPerPixel: Uint8,
    pub Rloss: Uint8,
    pub Gloss: Uint8,
    pub Bloss: Uint8,
    pub Aloss: Uint8,
    pub Rshift: Uint8,
    pub Gshift: Uint8,
    pub Bshift: Uint8,
    pub Ashift: Uint8,
    pub Rmask: Uint32,
    pub Gmask: Uint32,
    pub Bmask: Uint32,
    pub Amask: Uint32,
    pub colorkey: Uint32,
    pub alpha: Uint8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_Surface {
    pub flags: Uint32,
    pub format: *mut SDL_PixelFormat,
    pub w: libc::c_int,
    pub h: libc::c_int,
    pub pitch: Uint16,
    pub pixels: *mut libc::c_void,
    pub offset: libc::c_int,
    pub hwdata: *mut private_hwdata,
    pub clip_rect: SDL_Rect,
    pub unused1: Uint32,
    pub locked: Uint32,
    pub map: *mut SDL_BlitMap,
    pub format_version: libc::c_uint,
    pub refcount: libc::c_int,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct SDL_VideoInfo {
    #[bitfield(name = "hw_available", ty = "Uint32", bits = "0..=0")]
    #[bitfield(name = "wm_available", ty = "Uint32", bits = "1..=1")]
    #[bitfield(name = "UnusedBits1", ty = "Uint32", bits = "2..=7")]
    #[bitfield(name = "UnusedBits2", ty = "Uint32", bits = "8..=8")]
    #[bitfield(name = "blit_hw", ty = "Uint32", bits = "9..=9")]
    #[bitfield(name = "blit_hw_CC", ty = "Uint32", bits = "10..=10")]
    #[bitfield(name = "blit_hw_A", ty = "Uint32", bits = "11..=11")]
    #[bitfield(name = "blit_sw", ty = "Uint32", bits = "12..=12")]
    #[bitfield(name = "blit_sw_CC", ty = "Uint32", bits = "13..=13")]
    #[bitfield(name = "blit_sw_A", ty = "Uint32", bits = "14..=14")]
    #[bitfield(name = "blit_fill", ty = "Uint32", bits = "15..=15")]
    #[bitfield(name = "UnusedBits3", ty = "Uint32", bits = "16..=31")]
    pub hw_available_wm_available_UnusedBits1_UnusedBits2_blit_hw_blit_hw_CC_blit_hw_A_blit_sw_blit_sw_CC_blit_sw_A_blit_fill_UnusedBits3: [u8; 4],
    pub video_mem: Uint32,
    pub vfmt: *mut SDL_PixelFormat,
    pub current_w: libc::c_int,
    pub current_h: libc::c_int,
}
pub type SDL_GLattr = libc::c_uint;
pub const SDL_GL_SWAP_CONTROL: SDL_GLattr = 16;
pub const SDL_GL_ACCELERATED_VISUAL: SDL_GLattr = 15;
pub const SDL_GL_MULTISAMPLESAMPLES: SDL_GLattr = 14;
pub const SDL_GL_MULTISAMPLEBUFFERS: SDL_GLattr = 13;
pub const SDL_GL_STEREO: SDL_GLattr = 12;
pub const SDL_GL_ACCUM_ALPHA_SIZE: SDL_GLattr = 11;
pub const SDL_GL_ACCUM_BLUE_SIZE: SDL_GLattr = 10;
pub const SDL_GL_ACCUM_GREEN_SIZE: SDL_GLattr = 9;
pub const SDL_GL_ACCUM_RED_SIZE: SDL_GLattr = 8;
pub const SDL_GL_STENCIL_SIZE: SDL_GLattr = 7;
pub const SDL_GL_DEPTH_SIZE: SDL_GLattr = 6;
pub const SDL_GL_DOUBLEBUFFER: SDL_GLattr = 5;
pub const SDL_GL_BUFFER_SIZE: SDL_GLattr = 4;
pub const SDL_GL_ALPHA_SIZE: SDL_GLattr = 3;
pub const SDL_GL_BLUE_SIZE: SDL_GLattr = 2;
pub const SDL_GL_GREEN_SIZE: SDL_GLattr = 1;
pub const SDL_GL_RED_SIZE: SDL_GLattr = 0;
pub type GLenum = libc::c_uint;
pub type GLboolean = libc::c_uchar;
pub type GLvoid = ();
pub type GLint = libc::c_int;
pub type GLuint = libc::c_uint;
pub type GLsizei = libc::c_int;
pub type GLfloat = libc::c_float;
pub type GLdouble = libc::c_double;
pub type JSAMPLE = libc::c_uchar;
pub type JCOEF = libc::c_short;
pub type JOCTET = libc::c_uchar;
pub type UINT8 = libc::c_uchar;
pub type UINT16 = libc::c_ushort;
pub type JDIMENSION = libc::c_uint;
pub type boolean = libc::c_int;
pub type JSAMPROW = *mut JSAMPLE;
pub type JSAMPARRAY = *mut JSAMPROW;
pub type JBLOCK = [JCOEF; 64];
pub type JBLOCKROW = *mut JBLOCK;
pub type JBLOCKARRAY = *mut JBLOCKROW;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JQUANT_TBL {
    pub quantval: [UINT16; 64],
    pub sent_table: boolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JHUFF_TBL {
    pub bits: [UINT8; 17],
    pub huffval: [UINT8; 256],
    pub sent_table: boolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jpeg_component_info {
    pub component_id: libc::c_int,
    pub component_index: libc::c_int,
    pub h_samp_factor: libc::c_int,
    pub v_samp_factor: libc::c_int,
    pub quant_tbl_no: libc::c_int,
    pub dc_tbl_no: libc::c_int,
    pub ac_tbl_no: libc::c_int,
    pub width_in_blocks: JDIMENSION,
    pub height_in_blocks: JDIMENSION,
    pub DCT_scaled_size: libc::c_int,
    pub downsampled_width: JDIMENSION,
    pub downsampled_height: JDIMENSION,
    pub component_needed: boolean,
    pub MCU_width: libc::c_int,
    pub MCU_height: libc::c_int,
    pub MCU_blocks: libc::c_int,
    pub MCU_sample_width: libc::c_int,
    pub last_col_width: libc::c_int,
    pub last_row_height: libc::c_int,
    pub quant_table: *mut JQUANT_TBL,
    pub dct_table: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jpeg_scan_info {
    pub comps_in_scan: libc::c_int,
    pub component_index: [libc::c_int; 4],
    pub Ss: libc::c_int,
    pub Se: libc::c_int,
    pub Ah: libc::c_int,
    pub Al: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jpeg_marker_struct {
    pub next: jpeg_saved_marker_ptr,
    pub marker: UINT8,
    pub original_length: libc::c_uint,
    pub data_length: libc::c_uint,
    pub data: *mut JOCTET,
}
pub type jpeg_saved_marker_ptr = *mut jpeg_marker_struct;
pub type J_COLOR_SPACE = libc::c_uint;
pub const JCS_RGB565: J_COLOR_SPACE = 16;
pub const JCS_EXT_ARGB: J_COLOR_SPACE = 15;
pub const JCS_EXT_ABGR: J_COLOR_SPACE = 14;
pub const JCS_EXT_BGRA: J_COLOR_SPACE = 13;
pub const JCS_EXT_RGBA: J_COLOR_SPACE = 12;
pub const JCS_EXT_XRGB: J_COLOR_SPACE = 11;
pub const JCS_EXT_XBGR: J_COLOR_SPACE = 10;
pub const JCS_EXT_BGRX: J_COLOR_SPACE = 9;
pub const JCS_EXT_BGR: J_COLOR_SPACE = 8;
pub const JCS_EXT_RGBX: J_COLOR_SPACE = 7;
pub const JCS_EXT_RGB: J_COLOR_SPACE = 6;
pub const JCS_YCCK: J_COLOR_SPACE = 5;
pub const JCS_CMYK: J_COLOR_SPACE = 4;
pub const JCS_YCbCr: J_COLOR_SPACE = 3;
pub const JCS_RGB: J_COLOR_SPACE = 2;
pub const JCS_GRAYSCALE: J_COLOR_SPACE = 1;
pub const JCS_UNKNOWN: J_COLOR_SPACE = 0;
pub type J_DCT_METHOD = libc::c_uint;
pub const JDCT_FLOAT: J_DCT_METHOD = 2;
pub const JDCT_IFAST: J_DCT_METHOD = 1;
pub const JDCT_ISLOW: J_DCT_METHOD = 0;
pub type J_DITHER_MODE = libc::c_uint;
pub const JDITHER_FS: J_DITHER_MODE = 2;
pub const JDITHER_ORDERED: J_DITHER_MODE = 1;
pub const JDITHER_NONE: J_DITHER_MODE = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jpeg_common_struct {
    pub err: *mut jpeg_error_mgr,
    pub mem: *mut jpeg_memory_mgr,
    pub progress: *mut jpeg_progress_mgr,
    pub client_data: *mut libc::c_void,
    pub is_decompressor: boolean,
    pub global_state: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jpeg_progress_mgr {
    pub progress_monitor: Option<unsafe extern "C" fn(_: j_common_ptr) -> ()>,
    pub pass_counter: libc::c_long,
    pub pass_limit: libc::c_long,
    pub completed_passes: libc::c_int,
    pub total_passes: libc::c_int,
}
pub type j_common_ptr = *mut jpeg_common_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jpeg_memory_mgr {
    pub alloc_small: Option<unsafe extern "C" fn(_: j_common_ptr,
                                                 _: libc::c_int, _: size_t)
                                -> *mut libc::c_void>,
    pub alloc_large: Option<unsafe extern "C" fn(_: j_common_ptr,
                                                 _: libc::c_int, _: size_t)
                                -> *mut libc::c_void>,
    pub alloc_sarray: Option<unsafe extern "C" fn(_: j_common_ptr,
                                                  _: libc::c_int,
                                                  _: JDIMENSION,
                                                  _: JDIMENSION)
                                 -> JSAMPARRAY>,
    pub alloc_barray: Option<unsafe extern "C" fn(_: j_common_ptr,
                                                  _: libc::c_int,
                                                  _: JDIMENSION,
                                                  _: JDIMENSION)
                                 -> JBLOCKARRAY>,
    pub request_virt_sarray: Option<unsafe extern "C" fn(_: j_common_ptr,
                                                         _: libc::c_int,
                                                         _: boolean,
                                                         _: JDIMENSION,
                                                         _: JDIMENSION,
                                                         _: JDIMENSION)
                                        -> jvirt_sarray_ptr>,
    pub request_virt_barray: Option<unsafe extern "C" fn(_: j_common_ptr,
                                                         _: libc::c_int,
                                                         _: boolean,
                                                         _: JDIMENSION,
                                                         _: JDIMENSION,
                                                         _: JDIMENSION)
                                        -> jvirt_barray_ptr>,
    pub realize_virt_arrays: Option<unsafe extern "C" fn(_: j_common_ptr)
                                        -> ()>,
    pub access_virt_sarray: Option<unsafe extern "C" fn(_: j_common_ptr,
                                                        _: jvirt_sarray_ptr,
                                                        _: JDIMENSION,
                                                        _: JDIMENSION,
                                                        _: boolean)
                                       -> JSAMPARRAY>,
    pub access_virt_barray: Option<unsafe extern "C" fn(_: j_common_ptr,
                                                        _: jvirt_barray_ptr,
                                                        _: JDIMENSION,
                                                        _: JDIMENSION,
                                                        _: boolean)
                                       -> JBLOCKARRAY>,
    pub free_pool: Option<unsafe extern "C" fn(_: j_common_ptr,
                                               _: libc::c_int) -> ()>,
    pub self_destruct: Option<unsafe extern "C" fn(_: j_common_ptr) -> ()>,
    pub max_memory_to_use: libc::c_long,
    pub max_alloc_chunk: libc::c_long,
}
pub type jvirt_barray_ptr = *mut jvirt_barray_control;
pub type jvirt_sarray_ptr = *mut jvirt_sarray_control;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jpeg_error_mgr {
    pub error_exit: Option<unsafe extern "C" fn(_: j_common_ptr) -> ()>,
    pub emit_message: Option<unsafe extern "C" fn(_: j_common_ptr,
                                                  _: libc::c_int) -> ()>,
    pub output_message: Option<unsafe extern "C" fn(_: j_common_ptr) -> ()>,
    pub format_message: Option<unsafe extern "C" fn(_: j_common_ptr,
                                                    _: *mut libc::c_char)
                                   -> ()>,
    pub reset_error_mgr: Option<unsafe extern "C" fn(_: j_common_ptr) -> ()>,
    pub msg_code: libc::c_int,
    pub msg_parm: C2RustUnnamed,
    pub trace_level: libc::c_int,
    pub num_warnings: libc::c_long,
    pub jpeg_message_table: *const *const libc::c_char,
    pub last_jpeg_message: libc::c_int,
    pub addon_message_table: *const *const libc::c_char,
    pub first_addon_message: libc::c_int,
    pub last_addon_message: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub i: [libc::c_int; 8],
    pub s: [libc::c_char; 80],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jpeg_compress_struct {
    pub err: *mut jpeg_error_mgr,
    pub mem: *mut jpeg_memory_mgr,
    pub progress: *mut jpeg_progress_mgr,
    pub client_data: *mut libc::c_void,
    pub is_decompressor: boolean,
    pub global_state: libc::c_int,
    pub dest: *mut jpeg_destination_mgr,
    pub image_width: JDIMENSION,
    pub image_height: JDIMENSION,
    pub input_components: libc::c_int,
    pub in_color_space: J_COLOR_SPACE,
    pub input_gamma: libc::c_double,
    pub data_precision: libc::c_int,
    pub num_components: libc::c_int,
    pub jpeg_color_space: J_COLOR_SPACE,
    pub comp_info: *mut jpeg_component_info,
    pub quant_tbl_ptrs: [*mut JQUANT_TBL; 4],
    pub dc_huff_tbl_ptrs: [*mut JHUFF_TBL; 4],
    pub ac_huff_tbl_ptrs: [*mut JHUFF_TBL; 4],
    pub arith_dc_L: [UINT8; 16],
    pub arith_dc_U: [UINT8; 16],
    pub arith_ac_K: [UINT8; 16],
    pub num_scans: libc::c_int,
    pub scan_info: *const jpeg_scan_info,
    pub raw_data_in: boolean,
    pub arith_code: boolean,
    pub optimize_coding: boolean,
    pub CCIR601_sampling: boolean,
    pub smoothing_factor: libc::c_int,
    pub dct_method: J_DCT_METHOD,
    pub restart_interval: libc::c_uint,
    pub restart_in_rows: libc::c_int,
    pub write_JFIF_header: boolean,
    pub JFIF_major_version: UINT8,
    pub JFIF_minor_version: UINT8,
    pub density_unit: UINT8,
    pub X_density: UINT16,
    pub Y_density: UINT16,
    pub write_Adobe_marker: boolean,
    pub next_scanline: JDIMENSION,
    pub progressive_mode: boolean,
    pub max_h_samp_factor: libc::c_int,
    pub max_v_samp_factor: libc::c_int,
    pub total_iMCU_rows: JDIMENSION,
    pub comps_in_scan: libc::c_int,
    pub cur_comp_info: [*mut jpeg_component_info; 4],
    pub MCUs_per_row: JDIMENSION,
    pub MCU_rows_in_scan: JDIMENSION,
    pub blocks_in_MCU: libc::c_int,
    pub MCU_membership: [libc::c_int; 10],
    pub Ss: libc::c_int,
    pub Se: libc::c_int,
    pub Ah: libc::c_int,
    pub Al: libc::c_int,
    pub master: *mut jpeg_comp_master,
    pub main: *mut jpeg_c_main_controller,
    pub prep: *mut jpeg_c_prep_controller,
    pub coef: *mut jpeg_c_coef_controller,
    pub marker: *mut jpeg_marker_writer,
    pub cconvert: *mut jpeg_color_converter,
    pub downsample: *mut jpeg_downsampler,
    pub fdct: *mut jpeg_forward_dct,
    pub entropy: *mut jpeg_entropy_encoder,
    pub script_space: *mut jpeg_scan_info,
    pub script_space_size: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jpeg_destination_mgr {
    pub next_output_byte: *mut JOCTET,
    pub free_in_buffer: size_t,
    pub init_destination: Option<unsafe extern "C" fn(_: j_compress_ptr)
                                     -> ()>,
    pub empty_output_buffer: Option<unsafe extern "C" fn(_: j_compress_ptr)
                                        -> boolean>,
    pub term_destination: Option<unsafe extern "C" fn(_: j_compress_ptr)
                                     -> ()>,
}
pub type j_compress_ptr = *mut jpeg_compress_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jpeg_decompress_struct {
    pub err: *mut jpeg_error_mgr,
    pub mem: *mut jpeg_memory_mgr,
    pub progress: *mut jpeg_progress_mgr,
    pub client_data: *mut libc::c_void,
    pub is_decompressor: boolean,
    pub global_state: libc::c_int,
    pub src: *mut jpeg_source_mgr,
    pub image_width: JDIMENSION,
    pub image_height: JDIMENSION,
    pub num_components: libc::c_int,
    pub jpeg_color_space: J_COLOR_SPACE,
    pub out_color_space: J_COLOR_SPACE,
    pub scale_num: libc::c_uint,
    pub scale_denom: libc::c_uint,
    pub output_gamma: libc::c_double,
    pub buffered_image: boolean,
    pub raw_data_out: boolean,
    pub dct_method: J_DCT_METHOD,
    pub do_fancy_upsampling: boolean,
    pub do_block_smoothing: boolean,
    pub quantize_colors: boolean,
    pub dither_mode: J_DITHER_MODE,
    pub two_pass_quantize: boolean,
    pub desired_number_of_colors: libc::c_int,
    pub enable_1pass_quant: boolean,
    pub enable_external_quant: boolean,
    pub enable_2pass_quant: boolean,
    pub output_width: JDIMENSION,
    pub output_height: JDIMENSION,
    pub out_color_components: libc::c_int,
    pub output_components: libc::c_int,
    pub rec_outbuf_height: libc::c_int,
    pub actual_number_of_colors: libc::c_int,
    pub colormap: JSAMPARRAY,
    pub output_scanline: JDIMENSION,
    pub input_scan_number: libc::c_int,
    pub input_iMCU_row: JDIMENSION,
    pub output_scan_number: libc::c_int,
    pub output_iMCU_row: JDIMENSION,
    pub coef_bits: *mut [libc::c_int; 64],
    pub quant_tbl_ptrs: [*mut JQUANT_TBL; 4],
    pub dc_huff_tbl_ptrs: [*mut JHUFF_TBL; 4],
    pub ac_huff_tbl_ptrs: [*mut JHUFF_TBL; 4],
    pub data_precision: libc::c_int,
    pub comp_info: *mut jpeg_component_info,
    pub progressive_mode: boolean,
    pub arith_code: boolean,
    pub arith_dc_L: [UINT8; 16],
    pub arith_dc_U: [UINT8; 16],
    pub arith_ac_K: [UINT8; 16],
    pub restart_interval: libc::c_uint,
    pub saw_JFIF_marker: boolean,
    pub JFIF_major_version: UINT8,
    pub JFIF_minor_version: UINT8,
    pub density_unit: UINT8,
    pub X_density: UINT16,
    pub Y_density: UINT16,
    pub saw_Adobe_marker: boolean,
    pub Adobe_transform: UINT8,
    pub CCIR601_sampling: boolean,
    pub marker_list: jpeg_saved_marker_ptr,
    pub max_h_samp_factor: libc::c_int,
    pub max_v_samp_factor: libc::c_int,
    pub min_DCT_scaled_size: libc::c_int,
    pub total_iMCU_rows: JDIMENSION,
    pub sample_range_limit: *mut JSAMPLE,
    pub comps_in_scan: libc::c_int,
    pub cur_comp_info: [*mut jpeg_component_info; 4],
    pub MCUs_per_row: JDIMENSION,
    pub MCU_rows_in_scan: JDIMENSION,
    pub blocks_in_MCU: libc::c_int,
    pub MCU_membership: [libc::c_int; 10],
    pub Ss: libc::c_int,
    pub Se: libc::c_int,
    pub Ah: libc::c_int,
    pub Al: libc::c_int,
    pub unread_marker: libc::c_int,
    pub master: *mut jpeg_decomp_master,
    pub main: *mut jpeg_d_main_controller,
    pub coef: *mut jpeg_d_coef_controller,
    pub post: *mut jpeg_d_post_controller,
    pub inputctl: *mut jpeg_input_controller,
    pub marker: *mut jpeg_marker_reader,
    pub entropy: *mut jpeg_entropy_decoder,
    pub idct: *mut jpeg_inverse_dct,
    pub upsample: *mut jpeg_upsampler,
    pub cconvert: *mut jpeg_color_deconverter,
    pub cquantize: *mut jpeg_color_quantizer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jpeg_source_mgr {
    pub next_input_byte: *const JOCTET,
    pub bytes_in_buffer: size_t,
    pub init_source: Option<unsafe extern "C" fn(_: j_decompress_ptr) -> ()>,
    pub fill_input_buffer: Option<unsafe extern "C" fn(_: j_decompress_ptr)
                                      -> boolean>,
    pub skip_input_data: Option<unsafe extern "C" fn(_: j_decompress_ptr,
                                                     _: libc::c_long) -> ()>,
    pub resync_to_restart: Option<unsafe extern "C" fn(_: j_decompress_ptr,
                                                       _: libc::c_int)
                                      -> boolean>,
    pub term_source: Option<unsafe extern "C" fn(_: j_decompress_ptr) -> ()>,
}
pub type j_decompress_ptr = *mut jpeg_decompress_struct;
pub type PHYSFS_uint32 = libc::c_uint;
pub type PHYSFS_sint64 = libc::c_longlong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PHYSFS_File {
    pub opaque: *mut libc::c_void,
}
pub type __jmp_buf = [libc::c_int; 6];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
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
pub type STRING = libc::c_char;
pub type UWORD = libc::c_ushort;
pub type UDWORD = libc::c_uint;
pub type SDWORD = libc::c_int;
pub type HANDLE = *mut libc::c_void;
pub type BOOL = libc::c_int;
pub type DWORD = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PALETTEENTRY {
    pub peRed: UBYTE,
    pub peGreen: UBYTE,
    pub peBlue: UBYTE,
    pub peFlags: UBYTE,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pie_image {
    pub width: libc::c_uint,
    pub height: libc::c_uint,
    pub channels: libc::c_uint,
    pub data: *mut libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct my_error_mgr {
    pub pub_0: jpeg_error_mgr,
    pub setjmp_buffer: jmp_buf,
}
pub type my_error_ptr = *mut my_error_mgr;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct my_jpeg_destination_mgr {
    pub pub_0: jpeg_destination_mgr,
    pub file: *mut PHYSFS_File,
    pub buffer: *mut JOCTET,
}
/*
 * Screen.c
 *
 * Basic double buffered display using direct draw.
 *
 */
/* Free up a COM object */
// the following two lines compromise an ugly hack because some jpeglib.h
// actually contain configure-created defines that conflict with ours!
// man, those jpeglib authors should get a frigging clue...
/* Control Whether the back buffer is in system memory for full screen */
/* The Current screen size and bit depth */
#[no_mangle]
pub static mut screenWidth: UDWORD = 0 as libc::c_int as UDWORD;
#[no_mangle]
pub static mut screenHeight: UDWORD = 0 as libc::c_int as UDWORD;
#[no_mangle]
pub static mut screenDepth: UDWORD = 0 as libc::c_int as UDWORD;
#[no_mangle]
pub static mut screen: *mut SDL_Surface =
    0 as *const SDL_Surface as *mut SDL_Surface;
static mut asPalEntries: [SDL_Color; 256] =
    [SDL_Color{r: 0, g: 0, b: 0, unused: 0,}; 256];
#[no_mangle]
pub static mut pBackDropData: *mut UWORD = 0 as *const UWORD as *mut UWORD;
#[no_mangle]
pub static mut bBackDrop: BOOL = 0 as libc::c_int;
#[no_mangle]
pub static mut bUpload: BOOL = 0 as libc::c_int;
//fog
#[no_mangle]
pub static mut fogColour: DWORD = 0 as libc::c_int;
static mut screendump_filename: [libc::c_char; 255] = [0; 255];
static mut screendump_num: libc::c_uint = 0 as libc::c_int as libc::c_uint;
static mut screendump_required: BOOL = 0 as libc::c_int;
/* flag forcing buffers into video memory */
static mut g_bVidMem: BOOL = 0;
static mut backDropWidth: UDWORD = 640 as libc::c_int as UDWORD;
static mut backDropHeight: UDWORD = 480 as libc::c_int as UDWORD;
static mut backDropTexture: GLuint = -(1 as libc::c_int) as GLuint;
#[no_mangle]
pub unsafe extern "C" fn screenGetSDL() -> *mut SDL_Surface { return screen; }
/* Initialise the double buffered display */
/* Initialise the double buffered display */
#[no_mangle]
pub unsafe extern "C" fn screenInitialise(mut width: UDWORD,
                                          mut height: UDWORD,
                                          mut bitDepth: UDWORD,
                                          mut fullScreen: BOOL,
                                          mut bVidMem: BOOL, mut bDDraw: BOOL,
                                          mut hWindow: HANDLE) -> BOOL 
 // The main windows handle
 {
    /* Store the screen information */
    screenWidth = width;
    screenHeight = height;
    screenDepth = 32 as libc::c_int as UDWORD;
    /* store vidmem flag */
    g_bVidMem = bVidMem;
    static mut video_flags: libc::c_int = 0 as libc::c_int;
    let mut bpp: libc::c_int = 0;
    // Calculate the common flags for windowed and fullscreen modes.
    if video_flags == 0 as libc::c_int {
        let mut video_info: *const SDL_VideoInfo = 0 as *const SDL_VideoInfo;
        // Fetch the video info.
        video_info = SDL_GetVideoInfo();
        if video_info.is_null() { return 0 as libc::c_int }
        // The flags to pass to SDL_SetVideoMode.
        video_flags = 0x2 as libc::c_int; // Enable OpenGL in SDL.
        video_flags |=
            0x10000000 as
                libc::c_int; // Don't emulate requested BPP if not available.
        video_flags |=
            0x20000000 as libc::c_int; // Store the palette in hardware.
        // This checks to see if surfaces can be stored in memory.
        if (*video_info).hw_available() != 0 {
            video_flags |= 0x1 as libc::c_int
        } else { video_flags |= 0 as libc::c_int }
        // This checks if hardware blits can be done.
        if (*video_info).blit_hw() != 0 {
            video_flags |= 0x100 as libc::c_int
        }
        bpp =
            SDL_VideoModeOK(width as libc::c_int, height as libc::c_int,
                            screenDepth as libc::c_int,
                            video_flags as Uint32);
        if bpp == 0 {
            debug(LOG_ERROR,
                  b"Error: Video mode %dx%d@%dbpp is not supported!\n\x00" as
                      *const u8 as *const libc::c_char, width, height,
                  screenDepth);
            return 0 as libc::c_int
        }
        match bpp {
            32 | 24 => {
                SDL_GL_SetAttribute(SDL_GL_RED_SIZE, 8 as libc::c_int);
                SDL_GL_SetAttribute(SDL_GL_GREEN_SIZE, 8 as libc::c_int);
                SDL_GL_SetAttribute(SDL_GL_BLUE_SIZE, 8 as libc::c_int);
                SDL_GL_SetAttribute(SDL_GL_ALPHA_SIZE, 8 as libc::c_int);
                SDL_GL_SetAttribute(SDL_GL_DEPTH_SIZE, 16 as libc::c_int);
                SDL_GL_SetAttribute(SDL_GL_STENCIL_SIZE, 8 as libc::c_int);
            }
            16 => {
                debug(LOG_ERROR,
                      b"Warning: Using colour depth of %i instead of %i.\x00"
                          as *const u8 as *const libc::c_char, bpp,
                      screenDepth);
                debug(LOG_ERROR,
                      b"         You will experience graphics glitches!\x00"
                          as *const u8 as *const libc::c_char);
                SDL_GL_SetAttribute(SDL_GL_RED_SIZE, 5 as libc::c_int);
                SDL_GL_SetAttribute(SDL_GL_GREEN_SIZE, 6 as libc::c_int);
                SDL_GL_SetAttribute(SDL_GL_BLUE_SIZE, 5 as libc::c_int);
                SDL_GL_SetAttribute(SDL_GL_DEPTH_SIZE, 16 as libc::c_int);
                SDL_GL_SetAttribute(SDL_GL_STENCIL_SIZE, 8 as libc::c_int);
            }
            8 => {
                debug(LOG_ERROR,
                      b"Error: You don\'t want to play Warzone with a bit depth of %i, do you?\x00"
                          as *const u8 as *const libc::c_char, bpp);
                exit(1 as libc::c_int);
            }
            _ => {
                debug(LOG_ERROR,
                      b"Error: Unsupported bit depth: %i\x00" as *const u8 as
                          *const libc::c_char, bpp);
                exit(1 as libc::c_int);
            }
        }
        // Set the double buffer OpenGL attribute.
        SDL_GL_SetAttribute(SDL_GL_DOUBLEBUFFER, 1 as libc::c_int);
    }
    if fullScreen != 0 {
        video_flags =
            (video_flags as libc::c_uint | 0x80000000 as libc::c_uint) as
                libc::c_int
    }
    screen =
        SDL_SetVideoMode(width as libc::c_int, height as libc::c_int, bpp,
                         video_flags as Uint32);
    if screen.is_null() {
        debug(LOG_ERROR,
              b"Error: SDL_SetVideoMode failed (%s).\n\x00" as *const u8 as
                  *const libc::c_char, SDL_GetError());
        return 0 as libc::c_int
    }
    glViewport(0 as libc::c_int, 0 as libc::c_int, width as GLsizei,
               height as GLsizei);
    glMatrixMode(0x1701 as libc::c_int as GLenum);
    glPushMatrix();
    glLoadIdentity();
    glOrtho(0 as libc::c_int as GLdouble, width as GLdouble,
            height as GLdouble, 0 as libc::c_int as GLdouble,
            1 as libc::c_int as GLdouble, -(1 as libc::c_int) as GLdouble);
    glMatrixMode(0x1702 as libc::c_int as GLenum);
    glScalef((1 as libc::c_int as libc::c_double / 256.0f64) as GLfloat,
             (1 as libc::c_int as libc::c_double / 256.0f64) as GLfloat,
             1 as libc::c_int as GLfloat);
    glMatrixMode(0x1700 as libc::c_int as GLenum);
    glLoadIdentity();
    glCullFace(0x404 as libc::c_int as GLenum);
    glEnable(0xb44 as libc::c_int as GLenum);
    return 1 as libc::c_int;
}
// The main windows handle
/* Release the DD objects */
/* Release the DD objects */
#[no_mangle]
pub unsafe extern "C" fn screenShutDown() {
    if !screen.is_null() { SDL_FreeSurface(screen); };
}
/*
 * Screen.h
 *
 * Interface to the Direct Draw double buffered display.
 *
 */
/* Check the header files have been included from frame.h if they
 * are used outside of the framework library.
 */
/* ------------------------------------------------------------------------------------------- */
/* Legacy stuff 
 * - only used in the sequence video code we have not yet decided whether to port or to junk */
/* Return a pointer to the back buffer surface */
#[no_mangle]
pub unsafe extern "C" fn screenGetSurface() -> *mut libc::c_void {
    return 0 as *mut libc::c_void;
}
/* backDrop */
#[no_mangle]
pub unsafe extern "C" fn screen_SetBackDrop(mut newBackDropBmp: *mut UWORD,
                                            mut width: UDWORD,
                                            mut height: UDWORD) {
    bBackDrop = 1 as libc::c_int;
    pBackDropData = newBackDropBmp;
    backDropWidth = width;
    backDropHeight = height;
}
/* Image structure */
//typedef struct {
//	unsigned int	width;
//	unsigned int	height;
//	unsigned int	channels;
//	unsigned char*	data;
//} pie_image;
#[no_mangle]
pub unsafe extern "C" fn image_init(mut image: *mut pie_image) -> BOOL {
    if image.is_null() { return 1 as libc::c_int }
    (*image).width = 0 as libc::c_int as libc::c_uint;
    (*image).height = 0 as libc::c_int as libc::c_uint;
    (*image).channels = 0 as libc::c_int as libc::c_uint;
    (*image).data = 0 as *mut libc::c_uchar;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn image_create(mut image: *mut pie_image,
                                      mut width: libc::c_uint,
                                      mut height: libc::c_uint,
                                      mut channels: libc::c_uint) -> BOOL {
    if image.is_null() { return 1 as libc::c_int }
    (*image).width = width;
    (*image).height = height;
    (*image).channels = channels;
    if !(*image).data.is_null() { free((*image).data as *mut libc::c_void); }
    (*image).data =
        malloc(width.wrapping_mul(height).wrapping_mul(channels)) as
            *mut libc::c_uchar;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn image_delete(mut image: *mut pie_image) -> BOOL {
    if image.is_null() { return 1 as libc::c_int }
    if !(*image).data.is_null() {
        free((*image).data as *mut libc::c_void);
        (*image).data = 0 as *mut libc::c_uchar
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn my_error_exit(mut cinfo: j_common_ptr) {
    let mut myerr: my_error_ptr = (*cinfo).err as my_error_ptr;
    longjmp((*myerr).setjmp_buffer.as_mut_ptr(), 1 as libc::c_int);
}
unsafe extern "C" fn init_source(mut cinfo: j_decompress_ptr) { }
unsafe extern "C" fn fill_input_buffer(mut cinfo: j_decompress_ptr)
 -> boolean {
    static mut dummy: [JOCTET; 2] =
        [0xff as libc::c_int as JOCTET, 0xd9 as libc::c_int as JOCTET];
    /* Insert a fake EOI marker */
    (*(*cinfo).src).next_input_byte = dummy.as_mut_ptr();
    (*(*cinfo).src).bytes_in_buffer = 2 as libc::c_int as size_t;
    return 1 as libc::c_int;
}
unsafe extern "C" fn skip_input_data(mut cinfo: j_decompress_ptr,
                                     mut num_bytes: libc::c_long) {
    if num_bytes > 0 as libc::c_int as libc::c_long {
        while num_bytes > (*(*cinfo).src).bytes_in_buffer as libc::c_long {
            num_bytes -= (*(*cinfo).src).bytes_in_buffer as libc::c_long;
            fill_input_buffer(cinfo);
        }
        (*(*cinfo).src).next_input_byte =
            (*(*cinfo).src).next_input_byte.offset(num_bytes as size_t as
                                                       isize);
        (*(*cinfo).src).bytes_in_buffer =
            ((*(*cinfo).src).bytes_in_buffer as
                 libc::c_uint).wrapping_sub(num_bytes as size_t) as size_t as
                size_t
    };
}
unsafe extern "C" fn term_source(mut cinfo: j_decompress_ptr) { }
#[no_mangle]
pub unsafe extern "C" fn image_load_from_jpg(mut image: *mut pie_image,
                                             mut filename:
                                                 *const libc::c_char)
 -> BOOL {
    let mut cinfo: jpeg_decompress_struct =
        jpeg_decompress_struct{err: 0 as *mut jpeg_error_mgr,
                               mem: 0 as *mut jpeg_memory_mgr,
                               progress: 0 as *mut jpeg_progress_mgr,
                               client_data: 0 as *mut libc::c_void,
                               is_decompressor: 0,
                               global_state: 0,
                               src: 0 as *mut jpeg_source_mgr,
                               image_width: 0,
                               image_height: 0,
                               num_components: 0,
                               jpeg_color_space: JCS_UNKNOWN,
                               out_color_space: JCS_UNKNOWN,
                               scale_num: 0,
                               scale_denom: 0,
                               output_gamma: 0.,
                               buffered_image: 0,
                               raw_data_out: 0,
                               dct_method: JDCT_ISLOW,
                               do_fancy_upsampling: 0,
                               do_block_smoothing: 0,
                               quantize_colors: 0,
                               dither_mode: JDITHER_NONE,
                               two_pass_quantize: 0,
                               desired_number_of_colors: 0,
                               enable_1pass_quant: 0,
                               enable_external_quant: 0,
                               enable_2pass_quant: 0,
                               output_width: 0,
                               output_height: 0,
                               out_color_components: 0,
                               output_components: 0,
                               rec_outbuf_height: 0,
                               actual_number_of_colors: 0,
                               colormap: 0 as *mut JSAMPROW,
                               output_scanline: 0,
                               input_scan_number: 0,
                               input_iMCU_row: 0,
                               output_scan_number: 0,
                               output_iMCU_row: 0,
                               coef_bits: 0 as *mut [libc::c_int; 64],
                               quant_tbl_ptrs: [0 as *mut JQUANT_TBL; 4],
                               dc_huff_tbl_ptrs: [0 as *mut JHUFF_TBL; 4],
                               ac_huff_tbl_ptrs: [0 as *mut JHUFF_TBL; 4],
                               data_precision: 0,
                               comp_info: 0 as *mut jpeg_component_info,
                               progressive_mode: 0,
                               arith_code: 0,
                               arith_dc_L: [0; 16],
                               arith_dc_U: [0; 16],
                               arith_ac_K: [0; 16],
                               restart_interval: 0,
                               saw_JFIF_marker: 0,
                               JFIF_major_version: 0,
                               JFIF_minor_version: 0,
                               density_unit: 0,
                               X_density: 0,
                               Y_density: 0,
                               saw_Adobe_marker: 0,
                               Adobe_transform: 0,
                               CCIR601_sampling: 0,
                               marker_list: 0 as *mut jpeg_marker_struct,
                               max_h_samp_factor: 0,
                               max_v_samp_factor: 0,
                               min_DCT_scaled_size: 0,
                               total_iMCU_rows: 0,
                               sample_range_limit: 0 as *mut JSAMPLE,
                               comps_in_scan: 0,
                               cur_comp_info:
                                   [0 as *mut jpeg_component_info; 4],
                               MCUs_per_row: 0,
                               MCU_rows_in_scan: 0,
                               blocks_in_MCU: 0,
                               MCU_membership: [0; 10],
                               Ss: 0,
                               Se: 0,
                               Ah: 0,
                               Al: 0,
                               unread_marker: 0,
                               master: 0 as *mut jpeg_decomp_master,
                               main: 0 as *mut jpeg_d_main_controller,
                               coef: 0 as *mut jpeg_d_coef_controller,
                               post: 0 as *mut jpeg_d_post_controller,
                               inputctl: 0 as *mut jpeg_input_controller,
                               marker: 0 as *mut jpeg_marker_reader,
                               entropy: 0 as *mut jpeg_entropy_decoder,
                               idct: 0 as *mut jpeg_inverse_dct,
                               upsample: 0 as *mut jpeg_upsampler,
                               cconvert: 0 as *mut jpeg_color_deconverter,
                               cquantize: 0 as *mut jpeg_color_quantizer,};
    let mut jerr: my_error_mgr =
        my_error_mgr{pub_0:
                         jpeg_error_mgr{error_exit: None,
                                        emit_message: None,
                                        output_message: None,
                                        format_message: None,
                                        reset_error_mgr: None,
                                        msg_code: 0,
                                        msg_parm: C2RustUnnamed{i: [0; 8],},
                                        trace_level: 0,
                                        num_warnings: 0,
                                        jpeg_message_table:
                                            0 as *const *const libc::c_char,
                                        last_jpeg_message: 0,
                                        addon_message_table:
                                            0 as *const *const libc::c_char,
                                        first_addon_message: 0,
                                        last_addon_message: 0,},
                     setjmp_buffer:
                         [__jmp_buf_tag{__jmpbuf: [0; 6],
                                        __mask_was_saved: 0,
                                        __saved_mask:
                                            __sigset_t{__val: [0; 32],},};
                             1],};
    let mut row_stride: libc::c_int = 0;
    let mut image_size: libc::c_int = 0;
    let mut tmp: uintptr_t = 0;
    let mut ptr: [JSAMPARRAY; 1] = [0 as *mut JSAMPROW; 1];
    let mut jsrc: jpeg_source_mgr =
        jpeg_source_mgr{next_input_byte: 0 as *const JOCTET,
                        bytes_in_buffer: 0,
                        init_source: None,
                        fill_input_buffer: None,
                        skip_input_data: None,
                        resync_to_restart: None,
                        term_source: None,};
    let mut buffer: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fsize: UDWORD = 0;
    if image.is_null() { return 1 as libc::c_int }
    if loadFile(filename, &mut buffer, &mut fsize) == 0 {
        debug(LOG_ERROR,
              b"Could not load backdrop file \"%s\"!\x00" as *const u8 as
                  *const libc::c_char, filename);
        return 1 as libc::c_int
    }
    jpeg_CreateDecompress(&mut cinfo, 62 as libc::c_int,
                          ::std::mem::size_of::<jpeg_decompress_struct>() as
                              libc::c_ulong);
    cinfo.src = &mut jsrc;
    jsrc.init_source =
        Some(init_source as unsafe extern "C" fn(_: j_decompress_ptr) -> ());
    jsrc.fill_input_buffer =
        Some(fill_input_buffer as
                 unsafe extern "C" fn(_: j_decompress_ptr) -> boolean);
    jsrc.skip_input_data =
        Some(skip_input_data as
                 unsafe extern "C" fn(_: j_decompress_ptr, _: libc::c_long)
                     -> ());
    jsrc.resync_to_restart =
        Some(jpeg_resync_to_restart as
                 unsafe extern "C" fn(_: j_decompress_ptr, _: libc::c_int)
                     -> boolean);
    jsrc.term_source =
        Some(term_source as unsafe extern "C" fn(_: j_decompress_ptr) -> ());
    jsrc.bytes_in_buffer = fsize;
    jsrc.next_input_byte = buffer as *mut JOCTET;
    cinfo.err = jpeg_std_error(&mut jerr.pub_0);
    jerr.pub_0.error_exit =
        Some(my_error_exit as unsafe extern "C" fn(_: j_common_ptr) -> ());
    if _setjmp(jerr.setjmp_buffer.as_mut_ptr()) != 0 {
        jpeg_destroy_decompress(&mut cinfo);
        debug(LOG_ERROR,
              b"Error during jpg decompression of \"%s\"!\x00" as *const u8 as
                  *const libc::c_char, filename);
        memFreeRelease(buffer as *mut libc::c_void);
        buffer = 0 as *mut libc::c_char;
        return 1 as libc::c_int
    }
    jpeg_read_header(&mut cinfo, 1 as libc::c_int);
    cinfo.out_color_space = JCS_RGB;
    cinfo.quantize_colors = 0 as libc::c_int;
    cinfo.scale_num = 1 as libc::c_int as libc::c_uint;
    cinfo.scale_denom = 1 as libc::c_int as libc::c_uint;
    cinfo.dct_method = JDCT_IFAST;
    cinfo.do_fancy_upsampling = 0 as libc::c_int;
    jpeg_calc_output_dimensions(&mut cinfo);
    row_stride =
        cinfo.output_width.wrapping_mul(cinfo.output_components as
                                            libc::c_uint) as libc::c_int;
    image_size =
        (row_stride as libc::c_uint).wrapping_mul(cinfo.output_height) as
            libc::c_int;
    image_create(image, cinfo.output_width, cinfo.output_height,
                 cinfo.output_components as libc::c_uint);
    jpeg_start_decompress(&mut cinfo);
    tmp = (*image).data as uintptr_t;
    while cinfo.output_scanline < cinfo.output_height {
        ptr[0 as libc::c_int as usize] = tmp as JSAMPARRAY;
        jpeg_read_scanlines(&mut cinfo, ptr.as_mut_ptr() as JSAMPARRAY,
                            1 as libc::c_int as JDIMENSION);
        tmp =
            (tmp as libc::c_uint).wrapping_add(row_stride as libc::c_uint) as
                uintptr_t as uintptr_t
    }
    jpeg_finish_decompress(&mut cinfo);
    jpeg_destroy_decompress(&mut cinfo);
    memFreeRelease(buffer as *mut libc::c_void);
    buffer = 0 as *mut libc::c_char;
    return 0 as libc::c_int;
}
//=====================================================================
//=====================================================================
#[no_mangle]
pub unsafe extern "C" fn screen_SetBackDropFromFile(mut filename:
                                                        *mut libc::c_char) {
    let mut image: pie_image =
        pie_image{width: 0,
                  height: 0,
                  channels: 0,
                  data: 0 as *mut libc::c_uchar,};
    image_init(&mut image);
    if image_load_from_jpg(&mut image, filename) == 0 {
        if backDropTexture == -(1 as libc::c_int) as libc::c_uint {
            glGenTextures(1 as libc::c_int, &mut backDropTexture);
        }
        pie_SetTexturePage(-(1 as libc::c_int));
        glBindTexture(0xde1 as libc::c_int as GLenum, backDropTexture);
        glTexImage2D(0xde1 as libc::c_int as GLenum, 0 as libc::c_int,
                     0x1907 as libc::c_int, image.width as GLsizei,
                     image.height as GLsizei, 0 as libc::c_int,
                     0x1907 as libc::c_int as GLenum,
                     0x1401 as libc::c_int as GLenum,
                     image.data as *const libc::c_void);
        glTexEnvf(0x2300 as libc::c_int as GLenum,
                  0x2200 as libc::c_int as GLenum,
                  0x2100 as libc::c_int as GLfloat);
        glTexParameteri(0xde1 as libc::c_int as GLenum,
                        0x2801 as libc::c_int as GLenum,
                        0x2601 as libc::c_int);
        glTexParameteri(0xde1 as libc::c_int as GLenum,
                        0x2800 as libc::c_int as GLenum,
                        0x2601 as libc::c_int);
        glTexParameteri(0xde1 as libc::c_int as GLenum,
                        0x2802 as libc::c_int as GLenum,
                        0x2900 as libc::c_int);
        glTexParameteri(0xde1 as libc::c_int as GLenum,
                        0x2803 as libc::c_int as GLenum,
                        0x2900 as libc::c_int);
    }
    image_delete(&mut image);
}
//===================================================================
#[no_mangle]
pub unsafe extern "C" fn screen_StopBackDrop() {
    bBackDrop = 0 as libc::c_int;
    //checking [movie]
}
#[no_mangle]
pub unsafe extern "C" fn screen_RestartBackDrop() {
    bBackDrop = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn screen_GetBackDrop() -> BOOL { return bBackDrop; }
//******************************************************************
//slight hack to display maps (or whatever) in background.
//bitmap MUST be 512x512 for now.  -Q
#[no_mangle]
pub unsafe extern "C" fn screen_Upload(mut newBackDropBmp: *mut UWORD) {
    //	pie_image image;
//	image_init(&image);
    if !newBackDropBmp.is_null() {
        //	imagetest=malloc((sizeof(char)*512*512*512));
//	image_load_from_jpg(&image, "texpages\\bdrops\\test1.jpg");
        glGenTextures(1 as libc::c_int, &mut backDropTexture);
        pie_SetTexturePage(-(1 as libc::c_int));
        glBindTexture(0xde1 as libc::c_int as GLenum, backDropTexture);
        glTexImage2D(0xde1 as libc::c_int as GLenum, 0 as libc::c_int,
                     0x1907 as libc::c_int, 512 as libc::c_int,
                     512 as libc::c_int, 0 as libc::c_int,
                     0x1907 as libc::c_int as GLenum,
                     0x1401 as libc::c_int as GLenum,
                     newBackDropBmp as *const libc::c_void);
        glTexEnvf(0x2300 as libc::c_int as GLenum,
                  0x2200 as libc::c_int as GLenum,
                  0x2100 as libc::c_int as GLfloat);
        glTexParameteri(0xde1 as libc::c_int as GLenum,
                        0x2801 as libc::c_int as GLenum,
                        0x2601 as libc::c_int);
        glTexParameteri(0xde1 as libc::c_int as GLenum,
                        0x2800 as libc::c_int as GLenum,
                        0x2601 as libc::c_int);
        glTexParameteri(0xde1 as libc::c_int as GLenum,
                        0x2802 as libc::c_int as GLenum,
                        0x2900 as libc::c_int);
        glTexParameteri(0xde1 as libc::c_int as GLenum,
                        0x2803 as libc::c_int as GLenum,
                        0x2900 as libc::c_int);
        /*
//	image_delete(&image);
//	free(image);
	glDisable(GL_DEPTH_TEST);
	glDepthMask(GL_FALSE);
	pie_SetTexturePage(-1);
	glEnable(GL_TEXTURE_2D);
	glBindTexture(GL_TEXTURE_2D, backDropTexture);
	glPolygonMode(GL_FRONT,GL_LINE);
	glColor3f(1, 1, 1);
	glPushMatrix();
	glTranslatef(0,0,-13);
//	glBegin(GL_QUADS);
//		glVertex3f(-1.0f, 1.0f, 0.0f);				// Top Left
//		glVertex3f( 1.0f, 1.0f, 0.0f);				// Top Right
//		glVertex3f( 1.0f,-1.0f, 0.0f);				// Bottom Right
//		glVertex3f(-1.0f,-1.0f, 0.0f);				// Bottom Left
//		glEnd();
glBegin(GL_TRIANGLE_FAN);
glVertex3f(10, -12, 0);
glVertex3f(10, 12, 0);
glVertex3f(-10, 12, 0);
glVertex3f(-10, -12, 0);
glEnd();
//	glTexCoord2f(0, 0);
//	glVertex2f(0, 0);
//	glTexCoord2f(512, 0);
//	glVertex2f(screenWidth*2, 0);
//	glTexCoord2f(0, 512);
//	glVertex2f(0, screenHeight*2);
//	glTexCoord2f(512, 512);
//	glVertex2f(screenWidth*2, screenHeight*2);
//	glEnd();
	glPopMatrix();
		SDL_GL_SwapBuffers();
		SDL_GL_SwapBuffers();
*/
//	return;
    }
    glDisable(0xb71 as libc::c_int as GLenum);
    glDepthMask(0 as libc::c_int as GLboolean);
    pie_SetTexturePage(-(1 as libc::c_int));
    glEnable(0xde1 as libc::c_int as GLenum);
    glBindTexture(0xde1 as libc::c_int as GLenum, backDropTexture);
    glColor3f(1 as libc::c_int as GLfloat, 1 as libc::c_int as GLfloat,
              1 as libc::c_int as GLfloat);
    glBegin(0x5 as libc::c_int as GLenum);
    glTexCoord2f(0 as libc::c_int as GLfloat, 0 as libc::c_int as GLfloat);
    glVertex2f(0 as libc::c_int as GLfloat, 0 as libc::c_int as GLfloat);
    glTexCoord2f(255 as libc::c_int as GLfloat, 0 as libc::c_int as GLfloat);
    glVertex2f(screenWidth as GLfloat, 0 as libc::c_int as GLfloat);
    glTexCoord2f(0 as libc::c_int as GLfloat, 255 as libc::c_int as GLfloat);
    glVertex2f(0 as libc::c_int as GLfloat, screenHeight as GLfloat);
    glTexCoord2f(255 as libc::c_int as GLfloat,
                 255 as libc::c_int as GLfloat);
    glVertex2f(screenWidth as GLfloat, screenHeight as GLfloat);
    glEnd();
}
/* Toggle the display between full screen or windowed */
/* Swap between windowed and full screen mode */
#[no_mangle]
pub unsafe extern "C" fn screenToggleMode() {
    SDL_WM_ToggleFullScreen(screen);
}
unsafe extern "C" fn init_destination(mut cinfo: j_compress_ptr) {
    let mut dm: *mut my_jpeg_destination_mgr =
        (*cinfo).dest as *mut my_jpeg_destination_mgr;
    /* Allocate the output buffer --- it will be released when done with image */
    (*dm).buffer =
        malloc((4096 as libc::c_int as
                    libc::c_uint).wrapping_mul(::std::mem::size_of::<JOCTET>()
                                                   as libc::c_ulong)) as
            *mut JOCTET;
    (*dm).pub_0.next_output_byte = (*dm).buffer;
    (*dm).pub_0.free_in_buffer = 4096 as libc::c_int as size_t;
}
unsafe extern "C" fn empty_output_buffer(mut cinfo: j_compress_ptr)
 -> boolean {
    let mut dm: *mut my_jpeg_destination_mgr =
        (*cinfo).dest as *mut my_jpeg_destination_mgr;
    PHYSFS_write((*dm).file, (*dm).buffer as *const libc::c_void,
                 4096 as libc::c_int as PHYSFS_uint32,
                 1 as libc::c_int as PHYSFS_uint32);
    (*dm).pub_0.next_output_byte = (*dm).buffer;
    (*dm).pub_0.free_in_buffer = 4096 as libc::c_int as size_t;
    return 1 as libc::c_int;
}
unsafe extern "C" fn term_destination(mut cinfo: j_compress_ptr) {
    let mut dm: *mut my_jpeg_destination_mgr =
        (*cinfo).dest as *mut my_jpeg_destination_mgr;
    PHYSFS_write((*dm).file, (*dm).buffer as *const libc::c_void,
                 (4096 as libc::c_int as
                      libc::c_uint).wrapping_sub((*dm).pub_0.free_in_buffer),
                 1 as libc::c_int as PHYSFS_uint32);
    free((*dm).buffer as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn screenDoDumpToDiskIfRequired() {
    static mut buffer: *mut libc::c_uchar =
        0 as *const libc::c_uchar as *mut libc::c_uchar;
    static mut buffer_size: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut cinfo: jpeg_compress_struct =
        jpeg_compress_struct{err: 0 as *mut jpeg_error_mgr,
                             mem: 0 as *mut jpeg_memory_mgr,
                             progress: 0 as *mut jpeg_progress_mgr,
                             client_data: 0 as *mut libc::c_void,
                             is_decompressor: 0,
                             global_state: 0,
                             dest: 0 as *mut jpeg_destination_mgr,
                             image_width: 0,
                             image_height: 0,
                             input_components: 0,
                             in_color_space: JCS_UNKNOWN,
                             input_gamma: 0.,
                             data_precision: 0,
                             num_components: 0,
                             jpeg_color_space: JCS_UNKNOWN,
                             comp_info: 0 as *mut jpeg_component_info,
                             quant_tbl_ptrs: [0 as *mut JQUANT_TBL; 4],
                             dc_huff_tbl_ptrs: [0 as *mut JHUFF_TBL; 4],
                             ac_huff_tbl_ptrs: [0 as *mut JHUFF_TBL; 4],
                             arith_dc_L: [0; 16],
                             arith_dc_U: [0; 16],
                             arith_ac_K: [0; 16],
                             num_scans: 0,
                             scan_info: 0 as *const jpeg_scan_info,
                             raw_data_in: 0,
                             arith_code: 0,
                             optimize_coding: 0,
                             CCIR601_sampling: 0,
                             smoothing_factor: 0,
                             dct_method: JDCT_ISLOW,
                             restart_interval: 0,
                             restart_in_rows: 0,
                             write_JFIF_header: 0,
                             JFIF_major_version: 0,
                             JFIF_minor_version: 0,
                             density_unit: 0,
                             X_density: 0,
                             Y_density: 0,
                             write_Adobe_marker: 0,
                             next_scanline: 0,
                             progressive_mode: 0,
                             max_h_samp_factor: 0,
                             max_v_samp_factor: 0,
                             total_iMCU_rows: 0,
                             comps_in_scan: 0,
                             cur_comp_info:
                                 [0 as *mut jpeg_component_info; 4],
                             MCUs_per_row: 0,
                             MCU_rows_in_scan: 0,
                             blocks_in_MCU: 0,
                             MCU_membership: [0; 10],
                             Ss: 0,
                             Se: 0,
                             Ah: 0,
                             Al: 0,
                             master: 0 as *mut jpeg_comp_master,
                             main: 0 as *mut jpeg_c_main_controller,
                             prep: 0 as *mut jpeg_c_prep_controller,
                             coef: 0 as *mut jpeg_c_coef_controller,
                             marker: 0 as *mut jpeg_marker_writer,
                             cconvert: 0 as *mut jpeg_color_converter,
                             downsample: 0 as *mut jpeg_downsampler,
                             fdct: 0 as *mut jpeg_forward_dct,
                             entropy: 0 as *mut jpeg_entropy_encoder,
                             script_space: 0 as *mut jpeg_scan_info,
                             script_space_size: 0,};
    let mut jerr: jpeg_error_mgr =
        jpeg_error_mgr{error_exit: None,
                       emit_message: None,
                       output_message: None,
                       format_message: None,
                       reset_error_mgr: None,
                       msg_code: 0,
                       msg_parm: C2RustUnnamed{i: [0; 8],},
                       trace_level: 0,
                       num_warnings: 0,
                       jpeg_message_table: 0 as *const *const libc::c_char,
                       last_jpeg_message: 0,
                       addon_message_table: 0 as *const *const libc::c_char,
                       first_addon_message: 0,
                       last_addon_message: 0,};
    let mut jdest: my_jpeg_destination_mgr =
        my_jpeg_destination_mgr{pub_0:
                                    jpeg_destination_mgr{next_output_byte:
                                                             0 as *mut JOCTET,
                                                         free_in_buffer: 0,
                                                         init_destination:
                                                             None,
                                                         empty_output_buffer:
                                                             None,
                                                         term_destination:
                                                             None,},
                                file: 0 as *mut PHYSFS_File,
                                buffer: 0 as *mut JOCTET,};
    let mut row_pointer: [JSAMPROW; 1] = [0 as *mut JSAMPLE; 1];
    let mut row_stride: libc::c_int = 0;
    if screendump_required == 0 { return }
    row_stride = (*screen).w * 3 as libc::c_int;
    if (row_stride * (*screen).h) as libc::c_uint > buffer_size {
        if !buffer.is_null() { free(buffer as *mut libc::c_void); }
        buffer_size = (row_stride * (*screen).h) as libc::c_uint;
        buffer = malloc(buffer_size) as *mut libc::c_uchar
    }
    glReadPixels(0 as libc::c_int, 0 as libc::c_int, (*screen).w, (*screen).h,
                 0x1907 as libc::c_int as GLenum,
                 0x1401 as libc::c_int as GLenum,
                 buffer as *mut libc::c_void);
    screendump_required = 0 as libc::c_int;
    jdest.file = PHYSFS_openWrite(screendump_filename.as_mut_ptr());
    if jdest.file.is_null() { return }
    debug(LOG_3D,
          b"Saving screenshot %s\n\x00" as *const u8 as *const libc::c_char,
          screendump_filename.as_mut_ptr());
    cinfo.err = jpeg_std_error(&mut jerr);
    jpeg_CreateCompress(&mut cinfo, 62 as libc::c_int,
                        ::std::mem::size_of::<jpeg_compress_struct>() as
                            libc::c_ulong);
    cinfo.dest = &mut jdest.pub_0;
    jdest.pub_0.init_destination =
        Some(init_destination as
                 unsafe extern "C" fn(_: j_compress_ptr) -> ());
    jdest.pub_0.empty_output_buffer =
        Some(empty_output_buffer as
                 unsafe extern "C" fn(_: j_compress_ptr) -> boolean);
    jdest.pub_0.term_destination =
        Some(term_destination as
                 unsafe extern "C" fn(_: j_compress_ptr) -> ());
    cinfo.image_width = (*screen).w as JDIMENSION;
    cinfo.image_height = (*screen).h as JDIMENSION;
    cinfo.input_components = 3 as libc::c_int;
    cinfo.in_color_space = JCS_RGB;
    jpeg_set_defaults(&mut cinfo);
    jpeg_set_quality(&mut cinfo, 75 as libc::c_int, 1 as libc::c_int);
    jpeg_start_compress(&mut cinfo, 1 as libc::c_int);
    while cinfo.next_scanline < cinfo.image_height {
        row_pointer[0 as libc::c_int as usize] =
            &mut *buffer.offset(((*screen).h as
                                     libc::c_uint).wrapping_sub(cinfo.next_scanline).wrapping_sub(1
                                                                                                      as
                                                                                                      libc::c_int
                                                                                                      as
                                                                                                      libc::c_uint).wrapping_mul(row_stride
                                                                                                                                     as
                                                                                                                                     libc::c_uint)
                                    as isize) as *mut libc::c_uchar;
        jpeg_write_scanlines(&mut cinfo, row_pointer.as_mut_ptr(),
                             1 as libc::c_int as JDIMENSION);
    }
    jpeg_finish_compress(&mut cinfo);
    PHYSFS_close(jdest.file);
    jpeg_destroy_compress(&mut cinfo);
}
/* Set palette entries for the display buffer
 * first specifies the first palette entry. count the number of entries
 * The psPalette should have at least first + count entries in it.
 */
/* Set palette entries for the display buffer
 * first specifies the first palette entry. count the number of entries
 * The psPalette should have at least first + count entries in it.
 */
#[no_mangle]
pub unsafe extern "C" fn screenSetPalette(mut first: UDWORD,
                                          mut count: UDWORD,
                                          mut psEntries: *mut PALETTEENTRY) {
    let mut i: UDWORD = 0;
    if first.wrapping_add(count).wrapping_sub(1 as libc::c_int as
                                                  libc::c_uint) <
           256 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"screenSetPalette: invalid entry range\x00" as *const u8 as
                  *const libc::c_char);
    };
    if first.wrapping_add(count).wrapping_sub(1 as libc::c_int as
                                                  libc::c_uint) <
           256 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"screen.c\x00" as *const u8 as *const libc::c_char,
              656 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 17],
                                        &[libc::c_char; 17]>(b"screenSetPalette\x00")).as_ptr(),
              b"(first+count-1 < PAL_MAX)\x00" as *const u8 as
                  *const libc::c_char);
    };
    if count == 0 as libc::c_int as libc::c_uint { return }
    /* ensure that colour 0 is black and 255 is white */
    if (first == 0 as libc::c_int as libc::c_uint ||
            first == 255 as libc::c_int as libc::c_uint) &&
           count == 1 as libc::c_int as libc::c_uint {
        return
    }
    if first == 0 as libc::c_int as libc::c_uint {
        first = 1 as libc::c_int as UDWORD;
        count =
            (count as
                 libc::c_uint).wrapping_sub(1 as libc::c_int as libc::c_uint)
                as UDWORD as UDWORD
    }
    if first.wrapping_add(count).wrapping_sub(1 as libc::c_int as
                                                  libc::c_uint) ==
           256 as libc::c_int as libc::c_uint {
        count =
            (count as
                 libc::c_uint).wrapping_sub(1 as libc::c_int as libc::c_uint)
                as UDWORD as UDWORD
    }
    i = 0 as libc::c_int as UDWORD;
    while i < count {
        asPalEntries[first.wrapping_add(i) as usize].r =
            (*psEntries.offset(first.wrapping_add(i) as isize)).peRed;
        asPalEntries[first.wrapping_add(i) as usize].g =
            (*psEntries.offset(first.wrapping_add(i) as isize)).peGreen;
        asPalEntries[first.wrapping_add(i) as usize].b =
            (*psEntries.offset(first.wrapping_add(i) as isize)).peBlue;
        i = i.wrapping_add(1)
    }
    SDL_SetPalette(screen, 0x1 as libc::c_int | 0x2 as libc::c_int,
                   asPalEntries.as_mut_ptr().offset(first as isize),
                   first as libc::c_int, count as libc::c_int);
}
/* screendump */
#[no_mangle]
pub unsafe extern "C" fn screenDumpToDisk(mut path: *mut libc::c_char)
 -> *mut libc::c_char {
    loop  {
        screendump_num = screendump_num.wrapping_add(1);
        sprintf(screendump_filename.as_mut_ptr(),
                b"%s%swz2100_shot_%03i.jpg\x00" as *const u8 as
                    *const libc::c_char, path,
                b"/\x00" as *const u8 as *const libc::c_char, screendump_num);
        if PHYSFS_exists(screendump_filename.as_mut_ptr()) == 0 { break ; }
    }
    screendump_required = 1 as libc::c_int;
    return screendump_filename.as_mut_ptr();
}
/* Output text to the display screen at location x,y. The remaining arguments are as printf. */
/* Output text to the display screen at location x,y.
 * The remaining arguments are as printf.
 */
#[no_mangle]
pub unsafe extern "C" fn screenTextOut(mut x: UDWORD, mut y: UDWORD,
                                       mut pFormat: *mut STRING,
                                       mut args: ...) {
}
