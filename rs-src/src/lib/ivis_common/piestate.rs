use ::libc;
extern "C" {
    #[no_mangle]
    fn pie_SetColourKeyedBlack(keyingOn: BOOL);
    #[no_mangle]
    fn pie_SetFogStatus(val: BOOL);
    #[no_mangle]
    fn pie_SetDepthBufferStatus(depthMode: DEPTH_MODE);
    #[no_mangle]
    fn pie_SetColourCombine(colCombMode: COLOUR_MODE);
    #[no_mangle]
    fn pie_SetTranslucencyMode(transMode: TRANSLUCENCY_MODE);
}
pub type UBYTE = libc::c_uchar;
pub type UWORD = libc::c_ushort;
pub type SWORD = libc::c_short;
pub type UDWORD = libc::c_uint;
pub type SDWORD = libc::c_int;
pub type BOOL = libc::c_int;
pub type uint8 = libc::c_uchar;
pub type iBitmap = uint8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iSprite {
    pub width: libc::c_uint,
    pub height: libc::c_uint,
    pub bmp: *mut iBitmap,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IMAGEHEADER {
    pub Type: [UBYTE; 4],
    pub Version: UWORD,
    pub NumImages: UWORD,
    pub BitDepth: UWORD,
    pub NumTPages: UWORD,
    pub TPageFiles: [[UBYTE; 16]; 16],
    pub PalFile: [UBYTE; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IMAGEDEF {
    pub TPageID: UWORD,
    pub PalID: UWORD,
    pub Tu: UWORD,
    pub Tv: UWORD,
    pub Width: UWORD,
    pub Height: UWORD,
    pub XOffset: SWORD,
    pub YOffset: SWORD,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IMAGEFILE {
    pub Header: IMAGEHEADER,
    pub TexturePages: *mut iSprite,
    pub NumCluts: UWORD,
    pub TPageIDs: [UWORD; 16],
    pub ClutIDs: [UWORD; 48],
    pub ImageDefs: *mut IMAGEDEF,
}
pub type REND_MODE = libc::c_uint;
pub const REND_FILTER_ITERATED: REND_MODE = 9;
pub const REND_FILTER_FLAT: REND_MODE = 8;
pub const REND_ALPHA_ITERATED: REND_MODE = 7;
pub const REND_ALPHA_FLAT: REND_MODE = 6;
pub const REND_FLAT: REND_MODE = 5;
pub const REND_ALPHA_TEXT: REND_MODE = 4;
pub const REND_TEXT: REND_MODE = 3;
pub const REND_ADDITIVE_TEX: REND_MODE = 2;
pub const REND_ALPHA_TEX: REND_MODE = 1;
pub const REND_GOURAUD_TEX: REND_MODE = 0;
pub type DEPTH_MODE = libc::c_uint;
pub const DEPTH_CMP_ALWAYS_WRT_OFF: DEPTH_MODE = 3;
pub const DEPTH_CMP_LEQ_WRT_OFF: DEPTH_MODE = 2;
pub const DEPTH_CMP_ALWAYS_WRT_ON: DEPTH_MODE = 1;
pub const DEPTH_CMP_LEQ_WRT_ON: DEPTH_MODE = 0;
pub type TRANSLUCENCY_MODE = libc::c_uint;
pub const TRANS_ADDITIVE: TRANSLUCENCY_MODE = 4;
pub const TRANS_ALPHA: TRANSLUCENCY_MODE = 3;
pub const TRANS_FILTER: TRANSLUCENCY_MODE = 2;
pub const TRANS_DECAL_FOG: TRANSLUCENCY_MODE = 1;
pub const TRANS_DECAL: TRANSLUCENCY_MODE = 0;
pub type FOG_CAP = libc::c_uint;
pub const FOG_CAP_UNDEFINED: FOG_CAP = 3;
pub const FOG_CAP_COLOURED: FOG_CAP = 2;
pub const FOG_CAP_GREY: FOG_CAP = 1;
pub const FOG_CAP_NO: FOG_CAP = 0;
pub type COLOUR_MODE = libc::c_uint;
pub const COLOUR_TEX_CONSTANT: COLOUR_MODE = 3;
pub const COLOUR_TEX_ITERATED: COLOUR_MODE = 2;
pub const COLOUR_FLAT_ITERATED: COLOUR_MODE = 1;
pub const COLOUR_FLAT_CONSTANT: COLOUR_MODE = 0;
pub type TEX_MODE = libc::c_uint;
pub const TEX_NONE: TEX_MODE = 1;
pub const TEX_LOCAL: TEX_MODE = 0;
pub type ALPHA_MODE = libc::c_uint;
pub const ALPHA_CONSTANT: ALPHA_MODE = 1;
pub const ALPHA_ITERATED: ALPHA_MODE = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RENDER_STATE {
    pub depthBuffer: DEPTH_MODE,
    pub translucent: BOOL,
    pub additive: BOOL,
    pub fogCap: FOG_CAP,
    pub fogEnabled: BOOL,
    pub fog: BOOL,
    pub fogColour: UDWORD,
    pub texPage: SDWORD,
    pub rendMode: REND_MODE,
    pub bilinearOn: BOOL,
    pub keyingOn: BOOL,
    pub colourCombine: COLOUR_MODE,
    pub texCombine: TEX_MODE,
    pub alphaCombine: ALPHA_MODE,
    pub transMode: TRANSLUCENCY_MODE,
    pub colour: UDWORD,
}
#[no_mangle]
pub static mut pieStateCount: SDWORD = 0 as libc::c_int;
// FIXME Is this really used somewhere? Or is it just a dummy?
#[no_mangle]
pub static mut rendStates: RENDER_STATE =
    RENDER_STATE{depthBuffer: DEPTH_CMP_LEQ_WRT_ON,
                 translucent: 0,
                 additive: 0,
                 fogCap: FOG_CAP_NO,
                 fogEnabled: 0,
                 fog: 0,
                 fogColour: 0,
                 texPage: 0,
                 rendMode: REND_GOURAUD_TEX,
                 bilinearOn: 0,
                 keyingOn: 0,
                 colourCombine: COLOUR_FLAT_CONSTANT,
                 texCombine: TEX_LOCAL,
                 alphaCombine: ALPHA_ITERATED,
                 transMode: TRANS_DECAL,
                 colour: 0,};
#[no_mangle]
pub unsafe extern "C" fn pie_SetDefaultStates() 
 //Sets all states
 {
    //		pie_SetFogColour(0x00B08f5f);//nicks colour
	//fog off
    rendStates.fogEnabled = 0 as libc::c_int; // enable fog before renderer
    rendStates.fog = 0 as libc::c_int; //to force reset to false
    pie_SetFogStatus(0 as libc::c_int); //nicks colour
    pie_SetFogColour(0 as libc::c_int as UDWORD);
    //depth Buffer on
    rendStates.depthBuffer = DEPTH_CMP_LEQ_WRT_ON; //to force reset to true
    pie_SetDepthBufferStatus(DEPTH_CMP_LEQ_WRT_ON);
    //set render mode
    pie_SetTranslucent(1 as libc::c_int);
    pie_SetAdditive(1 as libc::c_int);
    //basic gouraud textured rendering
    rendStates.texCombine = TEX_NONE; //to force reset to GOURAUD_TEX
    pie_SetTexCombine(TEX_LOCAL); //to force reset to GOURAUD_TEX
    rendStates.colourCombine =
        COLOUR_FLAT_CONSTANT; //to force reset to GOURAUD_TEX
    pie_SetColourCombine(COLOUR_TEX_ITERATED); //to force reset to DECAL
    rendStates.alphaCombine = ALPHA_ITERATED;
    pie_SetAlphaCombine(ALPHA_CONSTANT);
    rendStates.transMode = TRANS_ALPHA;
    pie_SetTranslucencyMode(TRANS_DECAL);
    //chroma keying on black
    rendStates.keyingOn = 0 as libc::c_int; //to force reset to true
    pie_SetColourKeyedBlack(1 as libc::c_int);
    //bilinear filtering
    rendStates.bilinearOn = 0 as libc::c_int; //to force reset to true
    pie_SetBilinear(1 as libc::c_int);
}
//***************************************************************************
//
// pie_SetTranslucent(BOOL val);
//
// Global enable/disable Translucent effects
//
//***************************************************************************
#[no_mangle]
pub unsafe extern "C" fn pie_SetTranslucent(mut val: BOOL) {
    rendStates.translucent = val;
}
#[no_mangle]
pub unsafe extern "C" fn pie_Translucent() -> BOOL {
    return rendStates.translucent;
}
//***************************************************************************
//
// pie_SetAdditive(BOOL val);
//
// Global enable/disable Additive effects
//
//***************************************************************************
#[no_mangle]
pub unsafe extern "C" fn pie_SetAdditive(mut val: BOOL) {
    rendStates.additive = val;
}
#[no_mangle]
pub unsafe extern "C" fn pie_Additive() -> BOOL {
    return rendStates.additive;
}
//***************************************************************************
//
// pie_SetCaps(BOOL val);
//
// HIGHEST LEVEL enable/disable modes
//
//***************************************************************************
#[no_mangle]
pub unsafe extern "C" fn pie_SetFogCap(mut val: FOG_CAP) {
    rendStates.fogCap = val;
}
#[no_mangle]
pub unsafe extern "C" fn pie_GetFogCap() -> FOG_CAP {
    return rendStates.fogCap;
}
//***************************************************************************
//
// pie_EnableFog(BOOL val)
//
// Global enable/disable fog to allow fog to be turned of ingame
//
//***************************************************************************
#[no_mangle]
pub unsafe extern "C" fn pie_EnableFog(mut val: BOOL) {
    if rendStates.fogCap as libc::c_uint ==
           FOG_CAP_NO as libc::c_int as libc::c_uint {
        val = 0 as libc::c_int
    }
    if rendStates.fogEnabled != val {
        rendStates.fogEnabled = val;
        if val == 1 as libc::c_int {
            //			pie_SetFogColour(0x0078684f);//(nicks colour + 404040)/2
            pie_SetFogColour(0xb08f5f as libc::c_int as UDWORD);
            //nicks colour
        } else {
            pie_SetFogColour(0 as libc::c_int as UDWORD);
            //clear background to black
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn pie_GetFogEnabled() -> BOOL {
    return rendStates.fogEnabled;
}
//***************************************************************************
//
// pie_SetFogStatus(BOOL val)
//
// Toggle fog on and off for rendering objects inside or outside the 3D world
//
//***************************************************************************
#[no_mangle]
pub unsafe extern "C" fn pie_GetFogStatus() -> BOOL {
    return rendStates.fog; //check only
}
#[no_mangle]
pub unsafe extern "C" fn pie_SetFogColour(mut colour: UDWORD) {
    let mut grey: UDWORD = 0;
    if rendStates.fogCap as libc::c_uint ==
           FOG_CAP_GREY as libc::c_int as libc::c_uint {
        grey = colour & 0xff as libc::c_int as libc::c_uint;
        colour >>= 8 as libc::c_int;
        grey =
            (grey as
                 libc::c_uint).wrapping_add(colour &
                                                0xff as libc::c_int as
                                                    libc::c_uint) as UDWORD as
                UDWORD;
        colour >>= 8 as libc::c_int;
        grey =
            (grey as
                 libc::c_uint).wrapping_add(colour &
                                                0xff as libc::c_int as
                                                    libc::c_uint) as UDWORD as
                UDWORD;
        grey =
            (grey as
                 libc::c_uint).wrapping_div(3 as libc::c_int as libc::c_uint)
                as UDWORD as UDWORD;
        grey &= 0xff as libc::c_int as libc::c_uint;
        colour =
            grey.wrapping_add(grey <<
                                  8 as
                                      libc::c_int).wrapping_add(grey <<
                                                                    16 as
                                                                        libc::c_int);
        rendStates.fogColour = colour
    } else if rendStates.fogCap as libc::c_uint ==
                  FOG_CAP_NO as libc::c_int as libc::c_uint {
        rendStates.fogColour = 0 as libc::c_int as UDWORD
    } else { rendStates.fogColour = colour };
}
#[no_mangle]
pub unsafe extern "C" fn pie_GetFogColour() -> UDWORD {
    return rendStates.fogColour;
}
#[no_mangle]
pub unsafe extern "C" fn pie_SetRendMode(mut rendMode: REND_MODE) {
    if rendMode as libc::c_uint != rendStates.rendMode as libc::c_uint {
        rendStates.rendMode = rendMode;
        match rendMode as libc::c_uint {
            0 => {
                pie_SetColourCombine(COLOUR_TEX_ITERATED);
                pie_SetTexCombine(TEX_LOCAL);
                pie_SetAlphaCombine(ALPHA_CONSTANT);
                pie_SetTranslucencyMode(TRANS_DECAL);
            }
            1 => {
                pie_SetColourCombine(COLOUR_TEX_ITERATED);
                pie_SetTexCombine(TEX_LOCAL);
                pie_SetAlphaCombine(ALPHA_ITERATED);
                pie_SetTranslucencyMode(TRANS_ALPHA);
            }
            2 => {
                pie_SetColourCombine(COLOUR_TEX_ITERATED);
                pie_SetTexCombine(TEX_LOCAL);
                pie_SetAlphaCombine(ALPHA_ITERATED);
                pie_SetTranslucencyMode(TRANS_ADDITIVE);
            }
            3 => {
                pie_SetColourCombine(COLOUR_TEX_CONSTANT);
                pie_SetTexCombine(TEX_LOCAL);
                pie_SetAlphaCombine(ALPHA_CONSTANT);
                pie_SetTranslucencyMode(TRANS_DECAL);
            }
            4 => {
                pie_SetColourCombine(COLOUR_TEX_CONSTANT);
                pie_SetTexCombine(TEX_LOCAL);
                pie_SetAlphaCombine(ALPHA_CONSTANT);
                pie_SetTranslucencyMode(TRANS_ALPHA);
            }
            6 => {
                pie_SetColourCombine(COLOUR_FLAT_CONSTANT);
                pie_SetTexCombine(TEX_LOCAL);
                pie_SetAlphaCombine(ALPHA_CONSTANT);
                pie_SetTranslucencyMode(TRANS_ALPHA);
            }
            7 => {
                pie_SetColourCombine(COLOUR_FLAT_ITERATED);
                pie_SetTexCombine(TEX_LOCAL);
                pie_SetAlphaCombine(ALPHA_ITERATED);
                pie_SetTranslucencyMode(TRANS_ADDITIVE);
            }
            8 => {
                pie_SetColourCombine(COLOUR_FLAT_CONSTANT);
                pie_SetTexCombine(TEX_LOCAL);
                pie_SetAlphaCombine(ALPHA_CONSTANT);
                pie_SetTranslucencyMode(TRANS_FILTER);
            }
            9 => {
                pie_SetColourCombine(COLOUR_FLAT_CONSTANT);
                pie_SetTexCombine(TEX_LOCAL);
                pie_SetAlphaCombine(ALPHA_ITERATED);
                pie_SetTranslucencyMode(TRANS_ALPHA);
            }
            5 => {
                pie_SetColourCombine(COLOUR_FLAT_CONSTANT);
                pie_SetTexCombine(TEX_LOCAL);
                pie_SetAlphaCombine(ALPHA_CONSTANT);
                pie_SetTranslucencyMode(TRANS_DECAL);
            }
            _ => { }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn pie_SetBilinear(mut bilinearOn: BOOL) {
    if bilinearOn != rendStates.bilinearOn {
        rendStates.bilinearOn = bilinearOn;
        pieStateCount += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn pie_GetBilinear() -> BOOL {
    return rendStates.bilinearOn;
}
unsafe extern "C" fn pie_SetTexCombine(mut texCombMode: TEX_MODE) {
    //ffs
    if texCombMode as libc::c_uint != rendStates.texCombine as libc::c_uint {
        rendStates.texCombine = texCombMode;
        pieStateCount += 1
    };
}
unsafe extern "C" fn pie_SetAlphaCombine(mut alphaCombMode: ALPHA_MODE) {
    //ffs
    if alphaCombMode as libc::c_uint !=
           rendStates.alphaCombine as libc::c_uint {
        rendStates.alphaCombine = alphaCombMode;
        pieStateCount += 1
    };
}
/* **************************************************************************/
// get the constant colour used in text and flat render modes
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn pie_GetColour() -> UDWORD {
    return rendStates.colour;
}
#[no_mangle]
pub unsafe extern "C" fn pie_SetMouse(mut psImageFile: *mut IMAGEFILE,
                                      mut ImageID: UWORD) 
 // FIXME Remove if unused
 {
}
/* **************************************************************************/
/*
 * pieState.h
 *
 * render State controlr all pumpkin image library functions.
 *
 */
/* **************************************************************************/
/* **************************************************************************/
/* **************************************************************************/
/*
 *	Global Definitions
 */
/* **************************************************************************/
// FIXME Needed???
					// 	UBYTE				DDrawDriverName[256];
					// 	UBYTE				D3DDriverName[256];
/* **************************************************************************/
/*
 *	Global Variables
 */
/* **************************************************************************/
/* **************************************************************************/
/*
 *	Global ProtoTypes
 */
/* **************************************************************************/
//Sets all states
//renderer capability
//fog available
//fog currently on
//render states
//mouse states
#[no_mangle]
pub unsafe extern "C" fn pie_SetSwirlyBoxes(mut val: BOOL) 
 // FIXME Remove if unused
 {
}
