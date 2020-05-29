use ::libc;
extern "C" {
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_uint)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_uint)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memFreeRelease(pMemToFree: *mut libc::c_void);
    #[no_mangle]
    fn memMallocRelease(Size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn resLoad(pResFile: *mut STRING, blockID: SDWORD,
               pLoadBuffer: *mut libc::c_char, bufferSize: SDWORD,
               psMemHeap: *mut _block_heap) -> BOOL;
    // release the data for a particular block ID
    #[no_mangle]
    fn resReleaseBlockData(blockID: SDWORD);
    #[no_mangle]
    fn StartCursorSnap(SnapBuffer: *mut CURSORSNAP);
    #[no_mangle]
    fn AddCursorSnap(SnapBuffer: *mut CURSORSNAP, PosX: SWORD, PosY: SWORD,
                     FormID: UDWORD, ID: UDWORD, Bias: *mut SNAPBIAS);
    /* Add a form to the widget screen */
    #[no_mangle]
    fn widgAddForm(psScreen: *mut W_SCREEN, psInit: *mut W_FORMINIT) -> BOOL;
    /* Get widget structure */
    #[no_mangle]
    fn widgGetFromID(psScreen: *mut W_SCREEN, id: UDWORD) -> *mut WIDGET;
    /* Execute a set of widgets for one cycle.
 * Return the id of the widget that was activated, or 0 for none.
 */
    #[no_mangle]
    fn widgRunScreen(psScreen: *mut W_SCREEN) -> UDWORD;
    /* Display the screen's widgets in their current state
 * (Call after calling widgRunScreen, this allows the input
 *  processing to be seperated from the display of the widgets).
 */
    #[no_mangle]
    fn widgDisplayScreen(psScreen: *mut W_SCREEN);
    //holder for all StructureStats
    #[no_mangle]
    static mut asStructureStats: *mut STRUCTURE_STATS;
    #[no_mangle]
    static mut numStructureStats: UDWORD;
    #[no_mangle]
    static mut asStructLimits: [*mut STRUCTURE_LIMITS; 8];
    #[no_mangle]
    static mut psWScreen: *mut W_SCREEN;
    #[no_mangle]
    static mut WFont: libc::c_int;
    #[no_mangle]
    static mut DisplayBuffer: *mut libc::c_char;
    #[no_mangle]
    static mut displayBufferSize: UDWORD;
    #[no_mangle]
    fn numForms(total: UDWORD, perForm: UDWORD) -> UWORD;
    // Begin a rendering lock.
    #[no_mangle]
    fn DrawBegin();
    // End a rendering lock.
    #[no_mangle]
    fn DrawEnd();
    #[no_mangle]
    static mut StandardTab: TABDEF;
    #[no_mangle]
    fn intDisplayObjectForm(psWidget: *mut _widget, xOffset: UDWORD,
                            yOffset: UDWORD, pColours: *mut UDWORD);
    #[no_mangle]
    fn intDisplayPlainForm(psWidget: *mut _widget, xOffset: UDWORD,
                           yOffset: UDWORD, pColours: *mut UDWORD);
    #[no_mangle]
    fn intDisplayTab(psWidget: *mut _widget, TabType: UDWORD,
                     Position: UDWORD, Number: UDWORD, Selected: BOOL,
                     Hilight: BOOL, x: UDWORD, y: UDWORD, Width: UDWORD,
                     Height: UDWORD);
    // the block heap for the game data
    #[no_mangle]
    static mut psGameHeap: *mut BLOCK_HEAP;
    /*return the name to display for the interface - valid for OBJECTS and STATS*/
    #[no_mangle]
    fn getName(pNameID: *mut STRING) -> *mut STRING;
    #[no_mangle]
    static mut bForceEditorLoaded: BOOL;
    #[no_mangle]
    fn changeTitleMode(mode: tMode);
    #[no_mangle]
    fn processFrontendSnap(bHideCursor: BOOL);
    #[no_mangle]
    fn addBackdrop();
    #[no_mangle]
    fn addSideText(id: UDWORD, PosX: UDWORD, PosY: UDWORD, txt: *mut STRING);
    #[no_mangle]
    fn addFESlider(id: UDWORD, parent: UDWORD, x: UDWORD, y: UDWORD,
                   stops: UDWORD, pos: UDWORD, attachID: UDWORD);
    #[no_mangle]
    fn getStructureStatSize(Stats: *mut STRUCTURE_STATS) -> UDWORD;
    #[no_mangle]
    fn displayStructureStatButton(Stats: *mut STRUCTURE_STATS, Player: UDWORD,
                                  Rotation: *mut iVector,
                                  Position: *mut iVector, RotXYZ: BOOL,
                                  scale: SDWORD);
    /* **************************************************************************/
/*
 *	Global ProtoTypes
 */
/* **************************************************************************/
    #[no_mangle]
    fn drawBlueBox(x: UDWORD, y: UDWORD, w: UDWORD, h: UDWORD);
    #[no_mangle]
    fn initLoadingScreen(drawbdrop: BOOL, bRenderActive: BOOL);
    #[no_mangle]
    fn closeLoadingScreen();
    /* The current time in the game world */
    #[no_mangle]
    static mut gameTime2: UDWORD;
    #[no_mangle]
    fn addConsoleMessage(messageText: *mut STRING,
                         jusType: CONSOLE_TEXT_JUSTIFICATION) -> BOOL;
    #[no_mangle]
    fn iV_GetImageWidth(ImageFile: *mut IMAGEFILE, ID: UWORD) -> UWORD;
    #[no_mangle]
    fn iV_GetImageHeight(ImageFile: *mut IMAGEFILE, ID: UWORD) -> UWORD;
    #[no_mangle]
    fn iV_SetFont(FontID: libc::c_int);
    #[no_mangle]
    fn iV_SetTextColour(Index: SWORD);
    #[no_mangle]
    fn pie_DrawText(string: *mut STRING, x: UDWORD, y: UDWORD);
    #[no_mangle]
    fn pie_SetGeometricOffset(x: libc::c_int, y: libc::c_int);
    //Sets all states
    #[no_mangle]
    fn pie_SetDepthBufferStatus(depthMode: DEPTH_MODE);
    #[no_mangle]
    static mut ingame: MULTIPLAYERINGAME;
    #[no_mangle]
    fn sendTextMessage(pStr: *mut libc::c_char, cast: BOOL) -> BOOL;
    #[no_mangle]
    fn sendOptions(dest: DPID, play: UDWORD);
    #[no_mangle]
    fn frontendMultiMessages();
    #[no_mangle]
    fn addMultiBut(screen: *mut W_SCREEN, formid: UDWORD, id: UDWORD,
                   x: UDWORD, y: UDWORD, width: UDWORD, height: UDWORD,
                   tipres: UDWORD, norm: UDWORD, hi: UDWORD,
                   showmouseover: BOOL) -> BOOL;
    #[no_mangle]
    fn pie_GlobalRenderBegin();
    #[no_mangle]
    fn eventReset();
    /*
 * multilimit.c
 *
 * interface for setting limits to the game, bots, structlimits etc...
 */
    // FIXME Direct iVis implementation include!
    // ////////////////////////////////////////////////////////////////////////////
// externs
    #[no_mangle]
    static mut FrontImages: *mut IMAGEFILE;
    #[no_mangle]
    static mut InterfaceSnap: CURSORSNAP;
}
pub type size_t = libc::c_uint;
pub type UBYTE = libc::c_uchar;
pub type SBYTE = libc::c_schar;
pub type STRING = libc::c_char;
pub type UWORD = libc::c_ushort;
pub type SWORD = libc::c_short;
pub type UDWORD = libc::c_uint;
pub type SDWORD = libc::c_int;
pub type BOOL = libc::c_int;
pub type CHAR = libc::c_char;
pub type DPID = libc::c_int;
/* Parse the res file */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _block_heap {
    pub init: SDWORD,
    pub ext: SDWORD,
    pub psBlocks: *mut BLOCK_HEAP_MEM,
    pub psNext: *mut _block_heap,
}
pub type BLOCK_HEAP_MEM = _block_heap_mem;
/*
 * Block.h
 *
 * Routines to allocate memory from one large block.
 * Any memory allocated is only available to be reallocated after
 * the whole block has been reset.
 */
// control whether the debugging block malloc is used
/* *********************************************************************************/
/*                    type definitions                                            */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _block_heap_mem {
    pub size: SDWORD,
    pub pFree: *mut UBYTE,
    pub pMem: *mut UBYTE,
    pub pLastAllocated: *mut UBYTE,
    pub psNext: *mut _block_heap_mem,
}
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
pub type BLOCK_HEAP = _block_heap;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SNAPBIAS {
    pub NDxBias: SWORD,
    pub NDyBias: SWORD,
    pub UDxBias: SWORD,
    pub UDyBias: SWORD,
    pub RDxBias: SWORD,
    pub RDyBias: SWORD,
    pub DDxBias: SWORD,
    pub DDyBias: SWORD,
    pub LDxBias: SWORD,
    pub LDyBias: SWORD,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SNAPCOORD {
    pub FormID: UDWORD,
    pub ID: UDWORD,
    pub SnapX: SWORD,
    pub SnapY: SWORD,
    pub Bias: *mut SNAPBIAS,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CURSORSNAP {
    pub MaxSnaps: UWORD,
    pub NumSnaps: SWORD,
    pub CurrentSnap: SWORD,
    pub NewCurrentFormID: UDWORD,
    pub NewCurrentID: UDWORD,
    pub SnapCoords: *mut SNAPCOORD,
}
pub type _widget_type = libc::c_uint;
pub const WIDG_SLIDER: _widget_type = 5;
pub const WIDG_BARGRAPH: _widget_type = 4;
pub const WIDG_EDITBOX: _widget_type = 3;
pub const WIDG_BUTTON: _widget_type = 2;
pub const WIDG_LABEL: _widget_type = 1;
pub const WIDG_FORM: _widget_type = 0;
pub type WIDGET_TYPE = _widget_type;
/*
 * WidgBase.h
 *
 * Definitions for the basic widget types.
 */
/* The different base types of widget */
/* Button colours */
// Colour for button text
// Colour for button background
// Colour for button border
// 2nd border colour
// Hilite colour
/* The display function prototype */
/* The optional user callback function */
/* The common widget data */
/* ID of the widgets base form. */
/* The user set ID number for the widget */
/* This is returned when e.g. a button is pressed */
/* The widget type */
/* The style of the widget */
/* The location of the widget */
/* The size of the widget */
/* Display the widget */
/* User callback (if any) */
/* Pointer to a user data block (if any) */
/* User data (if any) */
/* Pointer to the next widget in the screen list */
/* The base widget data type */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _widget {
    pub formID: UDWORD,
    pub id: UDWORD,
    pub type_0: WIDGET_TYPE,
    pub style: UDWORD,
    pub x: SWORD,
    pub y: SWORD,
    pub width: UWORD,
    pub height: UWORD,
    pub display: WIDGET_DISPLAY,
    pub callback: WIDGET_CALLBACK,
    pub pUserData: *mut libc::c_void,
    pub UserData: UDWORD,
    pub psNext: *mut _widget,
}
pub type WIDGET_CALLBACK
    =
    Option<unsafe extern "C" fn(_: *mut _widget, _: *mut _w_context) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_context {
    pub psScreen: *mut W_SCREEN,
    pub psForm: *mut _w_form,
    pub xOffset: SDWORD,
    pub yOffset: SDWORD,
    pub mx: SDWORD,
    pub my: SDWORD,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_form {
    pub formID: UDWORD,
    pub id: UDWORD,
    pub type_0: WIDGET_TYPE,
    pub style: UDWORD,
    pub x: SWORD,
    pub y: SWORD,
    pub width: UWORD,
    pub height: UWORD,
    pub display: WIDGET_DISPLAY,
    pub callback: WIDGET_CALLBACK,
    pub pUserData: *mut libc::c_void,
    pub UserData: UDWORD,
    pub psNext: *mut _widget,
    pub disableChildren: BOOL,
    pub Ax0: UWORD,
    pub Ay0: UWORD,
    pub Ax1: UWORD,
    pub Ay1: UWORD,
    pub animCount: UDWORD,
    pub startTime: UDWORD,
    pub aColours: [UDWORD; 8],
    pub psLastHiLite: *mut WIDGET,
    pub psWidgets: *mut WIDGET,
}
pub type WIDGET = _widget;
pub type WIDGET_DISPLAY
    =
    Option<unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                _: *mut UDWORD) -> ()>;
/* The screen structure which stores all info for a widget screen */
pub type W_SCREEN = _w_screen;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_screen {
    pub psForm: *mut WIDGET,
    pub psFocus: *mut WIDGET,
    pub TipFontID: libc::c_int,
}
// The root form of the screen
// The widget that has keyboard focus
//	PROP_FONT	*psTipFont;			// The font for tool tips
// ID of the IVIS font to use for tool tips.
/* Upper limits for major and minor tabs on a tab form.
 * Not the best way to do it I know, but it keeps the memory
 * management MUCH simpler.
 */
//15		// Maximum number of major tabs on a tab form
//15		// Maximum number of minor tabs off a major
// Tab types passed into tab display callbacks.
pub type TAB_DISPLAY
    =
    Option<unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                _: UDWORD, _: BOOL, _: BOOL, _: UDWORD,
                                _: UDWORD, _: UDWORD, _: UDWORD) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_forminit {
    pub formID: UDWORD,
    pub majorID: UWORD,
    pub minorID: UWORD,
    pub id: UDWORD,
    pub style: UDWORD,
    pub x: SWORD,
    pub y: SWORD,
    pub width: UWORD,
    pub height: UWORD,
    pub pDisplay: WIDGET_DISPLAY,
    pub pCallback: WIDGET_CALLBACK,
    pub pUserData: *mut libc::c_void,
    pub UserData: UDWORD,
    pub disableChildren: BOOL,
    pub majorPos: UWORD,
    pub minorPos: UWORD,
    pub majorSize: UWORD,
    pub minorSize: UWORD,
    pub majorOffset: SWORD,
    pub minorOffset: SWORD,
    pub tabVertOffset: SWORD,
    pub tabHorzOffset: SWORD,
    pub tabMajorThickness: UWORD,
    pub tabMinorThickness: UWORD,
    pub tabMajorGap: UWORD,
    pub tabMinorGap: UWORD,
    pub numMajor: UWORD,
    pub aNumMinors: [UWORD; 9],
    pub pTip: *mut STRING,
    pub apMajorTips: [*mut STRING; 9],
    pub apMinorTips: [[*mut STRING; 5]; 9],
    pub pTabDisplay: TAB_DISPLAY,
    pub pFormDisplay: WIDGET_DISPLAY,
}
/* Form initialisation structure */
pub type W_FORMINIT = _w_forminit;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_butinit {
    pub formID: UDWORD,
    pub majorID: UWORD,
    pub minorID: UWORD,
    pub id: UDWORD,
    pub style: UDWORD,
    pub x: SWORD,
    pub y: SWORD,
    pub width: UWORD,
    pub height: UWORD,
    pub pDisplay: WIDGET_DISPLAY,
    pub pCallback: WIDGET_CALLBACK,
    pub pUserData: *mut libc::c_void,
    pub UserData: UDWORD,
    pub pText: *mut STRING,
    pub pTip: *mut STRING,
    pub FontID: libc::c_int,
}
/* The basic init entries */
/* Data for a tabbed form */
// Position of the tabs on the form
// Size of the tabs (in pixels)
// Tab start offset.
// Tab form overlap offset.
// Tab form overlap offset.
// The thickness of the tabs
// The thickness of the tabs
// The space between tabs
// The space between tabs
// Number of major tabs
// Number of minor tabs for each major
// Tool tip for the form itself
// Tool tips for the major tabs
// Tool tips for the minor tabs
// Optional callback for displaying a tab.
// Optional callback to display the form.
/* Button initialisation structure */
pub type W_BUTINIT = _w_butinit;
pub type int32 = libc::c_int;
pub type uint8 = libc::c_uchar;
/* The basic init entries */
// button text
// Tool tip text
//	PROP_FONT	*psFont;		// button font
// ID of the IVIS font to use for this widget.
/* **************************************************************************/
/*
 * pieTypes.h
 *
 * type defines for simple pies.
 *
 */
/* **************************************************************************/
/* **************************************************************************/
/* **************************************************************************/
/*
 *	Global Definitions
 */
/* **************************************************************************/
/* **************************************************************************/
/*
 *	Global Macros
 */
/* **************************************************************************/
/* **************************************************************************/
/*
 *	Global Type Definitions
 */
/* **************************************************************************/
pub type uint16 = libc::c_ushort;
pub type uint32 = libc::c_uint;
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
//*************************************************************************
//
// imd structures
//
//*************************************************************************
// lets hope this can work as a byte ... that will limit it to 255 polygons in 1 imd
// Size of the entry for vertex id in the imd polygon structure (32bits on pc 16bits on PSX)
// warning.... this is not used on the playstation version !
// the polygon number for the next in the BSP list ... or BSPPOLYID_TERMINATE for no more
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
pub type PSBSPTREENODE = *mut BSPTREENODE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BSPTREENODE {
    pub link: [*mut BSPTREENODE; 2],
    pub Plane: PLANE,
    pub TriSameDir: BSPPOLYID,
    pub TriOppoDir: BSPPOLYID,
}
pub type BSPPOLYID = uint16;
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
pub struct iTexAnim {
    pub nFrames: libc::c_int,
    pub playbackRate: libc::c_int,
    pub textureWidth: libc::c_int,
    pub textureHeight: libc::c_int,
}
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
pub type VERTEXID = libc::c_int;
pub type _tech_level = libc::c_uint;
pub const MAX_TECH_LEVELS: _tech_level = 6;
pub const TECH_LEVEL_ALL: _tech_level = 5;
pub const TECH_LEVEL_TWO_THREE: _tech_level = 4;
pub const TECH_LEVEL_ONE_TWO: _tech_level = 3;
pub const TECH_LEVEL_THREE: _tech_level = 2;
pub const TECH_LEVEL_TWO: _tech_level = 1;
pub const TECH_LEVEL_ONE: _tech_level = 0;
pub type TECH_LEVEL = _tech_level;
pub type _weapon_class = libc::c_uint;
pub const NUM_WEAPON_CLASS: _weapon_class = 2;
pub const WC_HEAT: _weapon_class = 1;
pub const WC_KINETIC: _weapon_class = 0;
pub type WEAPON_CLASS = _weapon_class;
pub type _weapon_subclass = libc::c_uint;
pub const NUM_WEAPON_SUBCLASS: _weapon_subclass = 17;
pub const WSC_EMP: _weapon_subclass = 16;
pub const WSC_COMMAND: _weapon_subclass = 15;
pub const WSC_BOMB: _weapon_subclass = 14;
pub const WSC_LAS_SAT: _weapon_subclass = 13;
pub const WSC_SLOWROCKET: _weapon_subclass = 12;
pub const WSC_SLOWMISSILE: _weapon_subclass = 11;
pub const WSC_AAGUN: _weapon_subclass = 10;
pub const WSC_ELECTRONIC: _weapon_subclass = 9;
pub const WSC_HOWITZERS: _weapon_subclass = 8;
pub const WSC_FLAME: _weapon_subclass = 7;
pub const WSC_GAUSS: _weapon_subclass = 6;
pub const WSC_ENERGY: _weapon_subclass = 5;
pub const WSC_ROCKET: _weapon_subclass = 4;
pub const WSC_MISSILE: _weapon_subclass = 3;
pub const WSC_MORTARS: _weapon_subclass = 2;
pub const WSC_CANNON: _weapon_subclass = 1;
pub const WSC_MGUN: _weapon_subclass = 0;
pub type WEAPON_SUBCLASS = _weapon_subclass;
pub type _movement_model = libc::c_uint;
pub const NUM_MOVEMENT_MODEL: _movement_model = 6;
pub const MM_SWEEP: _movement_model = 5;
pub const MM_ERRATICDIRECT: _movement_model = 4;
pub const MM_HOMINGINDIRECT: _movement_model = 3;
pub const MM_HOMINGDIRECT: _movement_model = 2;
pub const MM_INDIRECT: _movement_model = 1;
pub const MM_DIRECT: _movement_model = 0;
pub type MOVEMENT_MODEL = _movement_model;
pub type _weapon_effect = libc::c_uint;
pub const WE_NUMEFFECTS: _weapon_effect = 6;
pub const WE_ANTI_AIRCRAFT: _weapon_effect = 5;
pub const WE_FLAMER: _weapon_effect = 4;
pub const WE_ARTILLERY_ROUND: _weapon_effect = 3;
pub const WE_BUNKER_BUSTER: _weapon_effect = 2;
pub const WE_ANTI_TANK: _weapon_effect = 1;
pub const WE_ANTI_PERSONNEL: _weapon_effect = 0;
pub type WEAPON_EFFECT = _weapon_effect;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _weapon_stats {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
    pub techLevel: TECH_LEVEL,
    pub buildPower: UDWORD,
    pub buildPoints: UDWORD,
    pub weight: UDWORD,
    pub hitPoints: UDWORD,
    pub systemPoints: UDWORD,
    pub body: UDWORD,
    pub design: BOOL,
    pub pIMD: *mut iIMDShape,
    pub shortRange: UDWORD,
    pub longRange: UDWORD,
    pub minRange: UDWORD,
    pub shortHit: UDWORD,
    pub longHit: UDWORD,
    pub firePause: UDWORD,
    pub numExplosions: UDWORD,
    pub numRounds: UBYTE,
    pub reloadTime: UDWORD,
    pub damage: UDWORD,
    pub radius: UDWORD,
    pub radiusHit: UDWORD,
    pub radiusDamage: UDWORD,
    pub incenTime: UDWORD,
    pub incenDamage: UDWORD,
    pub incenRadius: UDWORD,
    pub flightSpeed: UDWORD,
    pub indirectHeight: UDWORD,
    pub fireOnMove: FIREONMOVE,
    pub weaponClass: WEAPON_CLASS,
    pub weaponSubClass: WEAPON_SUBCLASS,
    pub movementModel: MOVEMENT_MODEL,
    pub weaponEffect: WEAPON_EFFECT,
    pub recoilValue: UDWORD,
    pub rotate: UBYTE,
    pub maxElevation: UBYTE,
    pub minElevation: SBYTE,
    pub facePlayer: UBYTE,
    pub faceInFlight: UBYTE,
    pub effectSize: UBYTE,
    pub lightWorld: BOOL,
    pub surfaceToAir: UBYTE,
    pub vtolAttackRuns: UBYTE,
    pub directLife: UDWORD,
    pub radiusLife: UDWORD,
    pub pMountGraphic: *mut iIMDShape,
    pub pMuzzleGraphic: *mut iIMDShape,
    pub pInFlightGraphic: *mut iIMDShape,
    pub pTargetHitGraphic: *mut iIMDShape,
    pub pTargetMissGraphic: *mut iIMDShape,
    pub pWaterHitGraphic: *mut iIMDShape,
    pub pTrailGraphic: *mut iIMDShape,
    pub iAudioFireID: SDWORD,
    pub iAudioImpactID: SDWORD,
}
pub type FIREONMOVE = _fireonmove;
pub type _fireonmove = libc::c_uint;
pub const FOM_YES: _fireonmove = 2;
pub const FOM_PARTIAL: _fireonmove = 1;
pub const FOM_NO: _fireonmove = 0;
pub type _sensor_type = libc::c_uint;
pub const SUPER_SENSOR: _sensor_type = 4;
pub const VTOL_INTERCEPT_SENSOR: _sensor_type = 3;
pub const VTOL_CB_SENSOR: _sensor_type = 2;
pub const INDIRECT_CB_SENSOR: _sensor_type = 1;
pub const STANDARD_SENSOR: _sensor_type = 0;
pub type SENSOR_TYPE = _sensor_type;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _sensor_stats {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
    pub techLevel: TECH_LEVEL,
    pub buildPower: UDWORD,
    pub buildPoints: UDWORD,
    pub weight: UDWORD,
    pub hitPoints: UDWORD,
    pub systemPoints: UDWORD,
    pub body: UDWORD,
    pub design: BOOL,
    pub pIMD: *mut iIMDShape,
    pub range: UDWORD,
    pub power: UDWORD,
    pub location: UDWORD,
    pub type_0: SENSOR_TYPE,
    pub time: UDWORD,
    pub pMountGraphic: *mut iIMDShape,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ecm_stats {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
    pub techLevel: TECH_LEVEL,
    pub buildPower: UDWORD,
    pub buildPoints: UDWORD,
    pub weight: UDWORD,
    pub hitPoints: UDWORD,
    pub systemPoints: UDWORD,
    pub body: UDWORD,
    pub design: BOOL,
    pub pIMD: *mut iIMDShape,
    pub range: UDWORD,
    pub power: UDWORD,
    pub location: UDWORD,
    pub pMountGraphic: *mut iIMDShape,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _function {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
    pub type_0: UBYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _structure_stats {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
    pub type_0: UDWORD,
    pub techLevel: TECH_LEVEL,
    pub strength: STRUCT_STRENGTH,
    pub terrainType: UDWORD,
    pub baseWidth: UDWORD,
    pub baseBreadth: UDWORD,
    pub foundationType: UDWORD,
    pub buildPoints: UDWORD,
    pub height: UDWORD,
    pub armourValue: UDWORD,
    pub bodyPoints: UDWORD,
    pub repairSystem: UDWORD,
    pub powerToBuild: UDWORD,
    pub resistance: UDWORD,
    pub sizeModifier: UDWORD,
    pub pIMD: *mut iIMDShape,
    pub pBaseIMD: *mut iIMDShape,
    pub pECM: *mut _ecm_stats,
    pub pSensor: *mut _sensor_stats,
    pub psWeapStat: *mut _weapon_stats,
    pub numFuncs: UDWORD,
    pub defaultFunc: SDWORD,
    pub asFuncList: *mut *mut _function,
}
/*
 * StructureDef.h
 *
 * Structure definitions for structures
 *
 */
// Used to indicate any kind of building when calling intGotoNextStructureType()
/* Defines for indexing an appropriate IMD object given a buildings purpose. */
//draw as factory 2	
//corner wall - no gun
//control centre for command droids
//the demolish structure type - should only be one stat with this type
//added for updates - AB 8/6/99
//REF_WALLH,     //the following are needed for the demo
//REF_WALLV,		
//REF_CORNER1,
//REF_CORNER2,
//REF_CORNER3,
//REF_CORNER4,
//REF_GATE1,
//REF_GATE2,
//REF_GATE3,
//REF_GATE4,
//REF_TOWER1,
//REF_TOWER2,
//REF_TOWER3,
//REF_TOWER4,
//REF_VANH,
//REF_VANV,
//REF_JEEP,
//REF_TANKERH,
//REF_TANKERV,
//need to keep a count of how many types for IMD loading
//Delivery Points NOT wayPoints
//proximity messages that are data generated
//proximity messages that are in game generated
//SAVE ONLY delivery point for factory currently assigned to commander
/*the type of position obj - FlagPos or ProxDisp*/
/*when the Position was last drawn*/
/*screen coords and radius of Position imd */
/*which player the Position belongs to*/
/*flag to indicate whether the Position 
										is to be highlighted*/
//the world coords of the Position
//UDWORD		frameNumber;					//when the Position was last drawn
	//UDWORD		screenX, screenY, screenR;		//screen coords and radius of Position imd
	//UDWORD		player;							//which player the Position belongs to
	//BOOL		selected;						//flag to indicate whether the 
												//Position is to be highlighted
//indicates whether the first, second etc factory
//indicates whether standard, cyborg or vtol factory
//	UBYTE		factorySub;						//sub value. needed to order production points.
//	UBYTE		primary;
//only allowed one weapon per structure (more memory for Tim) 
pub type STRUCT_STRENGTH = _struct_strength;
pub type _struct_strength = libc::c_uint;
pub const NUM_STRUCT_STRENGTH: _struct_strength = 4;
pub const STRENGTH_BUNKER: _struct_strength = 3;
pub const STRENGTH_HARD: _struct_strength = 2;
pub const STRENGTH_MEDIUM: _struct_strength = 1;
pub const STRENGTH_SOFT: _struct_strength = 0;
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
pub type STRUCTURE_STATS = _structure_stats;
/*highest number the limit can be set to */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _structure_limits {
    pub limit: UBYTE,
    pub currentQuantity: UBYTE,
    pub globalLimit: UBYTE,
}
pub type STRUCTURE_LIMITS = _structure_limits;
// multiplayer only. sets the max value selectable (limits changed by player)
/*
 * Text.h (was Strings.h)
 *
 * String management functions
 *
 */
//the two defines below are MUTUALLY EXCLUSIVE! don't have both defined...
//#define RESOURCE_NAMES
/* ID numbers for all the fixed strings */
pub type _fixed_str_id = libc::c_uint;
pub const STR_MAX_ID: _fixed_str_id = 442;
pub const STR_SEQ_MINIMAL: _fixed_str_id = 441;
pub const STR_SEQ_WINDOW: _fixed_str_id = 440;
pub const STR_SEQ_FULL: _fixed_str_id = 439;
pub const STR_SEQ_PLAYBACK: _fixed_str_id = 438;
pub const STR_GAM_FORMATION_OFF: _fixed_str_id = 437;
pub const STR_GAM_FORMATION_ON: _fixed_str_id = 436;
pub const STR_GAM_BUILD_NO_REOPEN: _fixed_str_id = 435;
pub const STR_GAM_BUILD_REOPEN: _fixed_str_id = 434;
pub const STR_BIND_REOPEN_BUILD: _fixed_str_id = 433;
pub const STR_FE_SUBTITLES: _fixed_str_id = 432;
pub const STR_DORD_VTOL_FACTORY: _fixed_str_id = 431;
pub const STR_DORD_FIREDES: _fixed_str_id = 430;
pub const STR_BIND_RADJUMP: _fixed_str_id = 429;
pub const STR_BIND_RELOAD: _fixed_str_id = 428;
pub const STR_GAM_NORMAL_SPEED: _fixed_str_id = 427;
pub const STR_GAM_SLOW_DOWN: _fixed_str_id = 426;
pub const STR_GAM_SPEED_UP: _fixed_str_id = 425;
pub const STR_BIND_NORMAL_SPEED: _fixed_str_id = 424;
pub const STR_BIND_SLOW_DOWN: _fixed_str_id = 423;
pub const STR_BIND_SPEED_UP: _fixed_str_id = 422;
pub const STR_BIND_LEFT: _fixed_str_id = 421;
pub const STR_BIND_RIGHT: _fixed_str_id = 420;
pub const STR_BIND_DOWN: _fixed_str_id = 419;
pub const STR_BIND_UP: _fixed_str_id = 418;
pub const STR_BIND_CONSOLE: _fixed_str_id = 417;
pub const STR_BIND_SELCYBORG: _fixed_str_id = 416;
pub const STR_BIND_SELPOWER: _fixed_str_id = 415;
pub const STR_BIND_SELRESEARCH: _fixed_str_id = 414;
pub const STR_BIND_SELFACTORY: _fixed_str_id = 413;
pub const STR_BIND_CMD9: _fixed_str_id = 412;
pub const STR_BIND_CMD8: _fixed_str_id = 411;
pub const STR_BIND_CMD7: _fixed_str_id = 410;
pub const STR_BIND_CMD6: _fixed_str_id = 409;
pub const STR_BIND_CMD5: _fixed_str_id = 408;
pub const STR_BIND_CMD4: _fixed_str_id = 407;
pub const STR_BIND_CMD3: _fixed_str_id = 406;
pub const STR_BIND_CMD2: _fixed_str_id = 405;
pub const STR_BIND_CMD1: _fixed_str_id = 404;
pub const STR_GAM_COMNOTFOUND: _fixed_str_id = 403;
pub const STR_GAM_SENNOTFOUND: _fixed_str_id = 402;
pub const STR_GAM_CONNOTFOUND: _fixed_str_id = 401;
pub const STR_BIND_COMJ: _fixed_str_id = 400;
pub const STR_BIND_SENJ: _fixed_str_id = 399;
// number of string ID's
//added by alex 26-01-99
pub const STR_BIND_CONJ: _fixed_str_id = 398;
pub const STR_FE_CYAN: _fixed_str_id = 397;
pub const STR_FE_PINK: _fixed_str_id = 396;
pub const STR_FE_BLUE: _fixed_str_id = 395;
pub const STR_FE_RED: _fixed_str_id = 394;
pub const STR_FE_BLACK: _fixed_str_id = 393;
pub const STR_FE_GREY: _fixed_str_id = 392;
pub const STR_FE_ORANGE: _fixed_str_id = 391;
pub const STR_FE_GREEN: _fixed_str_id = 390;
pub const STR_FE_OFF: _fixed_str_id = 389;
pub const STR_FE_ON: _fixed_str_id = 388;
pub const STR_FE_MFLIP: _fixed_str_id = 387;
pub const STR_FE_SSHAKE: _fixed_str_id = 386;
pub const STR_SEL_NO_STRUCTURE: _fixed_str_id = 385;
// something else.
pub const STR_GAM_DERRICK_BURNING: _fixed_str_id = 384;
pub const STR_BIND_ASIMIL: _fixed_str_id = 383;
pub const STR_BIND_AWHE: _fixed_str_id = 382;
pub const STR_BIND_AVTOL: _fixed_str_id = 381;
pub const STR_BIND_ALL: _fixed_str_id = 380;
pub const STR_BIND_ATR: _fixed_str_id = 379;
pub const STR_BIND_ASCR: _fixed_str_id = 378;
pub const STR_BIND_RECY: _fixed_str_id = 377;
pub const STR_BIND_AHOV: _fixed_str_id = 376;
pub const STR_BIND_AHTR: _fixed_str_id = 375;
pub const STR_BIND_ABDU: _fixed_str_id = 374;
pub const STR_BIND_ACU: _fixed_str_id = 373;
pub const STR_BIND_NDAM: _fixed_str_id = 372;
pub const STR_BIND_HDAM: _fixed_str_id = 371;
pub const STR_BIND_LDAM: _fixed_str_id = 370;
pub const STR_BIND_SCAT: _fixed_str_id = 369;
pub const STR_BIND_LONGR: _fixed_str_id = 368;
pub const STR_BIND_SENDT: _fixed_str_id = 367;
pub const STR_BIND_DSTOP: _fixed_str_id = 366;
pub const STR_BIND_REPA: _fixed_str_id = 365;
pub const STR_BIND_PATR: _fixed_str_id = 364;
pub const STR_BIND_PURS: _fixed_str_id = 363;
pub const STR_BIND_SHOR: _fixed_str_id = 362;
pub const STR_BIND_SPLIM: _fixed_str_id = 361;
pub const STR_BIND_DEFR: _fixed_str_id = 360;
pub const STR_BIND_RTB: _fixed_str_id = 359;
pub const STR_BIND_FAW: _fixed_str_id = 358;
pub const STR_BIND_ENGAG: _fixed_str_id = 357;
pub const STR_BIND_UNITJ: _fixed_str_id = 356;
pub const STR_BIND_CEASE: _fixed_str_id = 355;
pub const STR_BIND_CENTV: _fixed_str_id = 354;
pub const STR_BIND_OVERL: _fixed_str_id = 353;
pub const STR_BIND_REPJ: _fixed_str_id = 352;
pub const STR_BIND_RESJ: _fixed_str_id = 351;
pub const STR_BIND_ORD: _fixed_str_id = 350;
pub const STR_BIND_PB: _fixed_str_id = 349;
pub const STR_BIND_RR: _fixed_str_id = 348;
pub const STR_BIND_RP: _fixed_str_id = 347;
pub const STR_BIND_RL: _fixed_str_id = 346;
pub const STR_BIND_PF: _fixed_str_id = 345;
pub const STR_BIND_ZOUT: _fixed_str_id = 344;
pub const STR_BIND_ZIN: _fixed_str_id = 343;
pub const STR_BIND_ROUT: _fixed_str_id = 342;
pub const STR_BIND_RIN: _fixed_str_id = 341;
pub const STR_BIND_OPT: _fixed_str_id = 340;
pub const STR_BIND_TRACK: _fixed_str_id = 339;
pub const STR_BIND_NORTH: _fixed_str_id = 338;
pub const STR_BIND_AUDOFF: _fixed_str_id = 337;
pub const STR_BIND_AUDON: _fixed_str_id = 336;
pub const STR_BIND_MULOP: _fixed_str_id = 335;
pub const STR_BIND_GR9: _fixed_str_id = 334;
pub const STR_BIND_GR8: _fixed_str_id = 333;
pub const STR_BIND_GR7: _fixed_str_id = 332;
pub const STR_BIND_GR6: _fixed_str_id = 331;
pub const STR_BIND_GR5: _fixed_str_id = 330;
pub const STR_BIND_GR4: _fixed_str_id = 329;
pub const STR_BIND_GR3: _fixed_str_id = 328;
pub const STR_BIND_GR2: _fixed_str_id = 327;
pub const STR_BIND_GR1: _fixed_str_id = 326;
pub const STR_BIND_AS9: _fixed_str_id = 325;
pub const STR_BIND_AS8: _fixed_str_id = 324;
pub const STR_BIND_AS7: _fixed_str_id = 323;
pub const STR_BIND_AS6: _fixed_str_id = 322;
pub const STR_BIND_AS5: _fixed_str_id = 321;
pub const STR_BIND_AS4: _fixed_str_id = 320;
pub const STR_BIND_AS3: _fixed_str_id = 319;
pub const STR_BIND_AS2: _fixed_str_id = 318;
pub const STR_BIND_AS1: _fixed_str_id = 317;
pub const STR_BIND_PREV: _fixed_str_id = 316;
pub const STR_BIND_PAUSE: _fixed_str_id = 315;
pub const STR_BIND_SHOT: _fixed_str_id = 314;
pub const STR_BIND_BARS: _fixed_str_id = 313;
pub const STR_BIND_TOGCON: _fixed_str_id = 312;
pub const STR_BIND_TOGRAD: _fixed_str_id = 311;
pub const STR_BIND_CHOCOM: _fixed_str_id = 310;
pub const STR_BIND_CHOINT: _fixed_str_id = 309;
pub const STR_BIND_CHODES: _fixed_str_id = 308;
pub const STR_BIND_CHOBUI: _fixed_str_id = 307;
pub const STR_BIND_CHORES: _fixed_str_id = 306;
pub const STR_BIND_CHOMAN: _fixed_str_id = 305;
pub const STR_KM_KEYMAP_SIDE: _fixed_str_id = 304;
// keymap editor.
pub const STR_KM_KEYMAP: _fixed_str_id = 303;
pub const STR_GAM_MAXUNITSREACHED: _fixed_str_id = 302;
pub const STR_GAME_RESTART: _fixed_str_id = 301;
pub const STR_DORD_CYBORG_FACTORY: _fixed_str_id = 300;
pub const STR_GAME_LOADED: _fixed_str_id = 299;
pub const STR_GAME_SAVED: _fixed_str_id = 298;
pub const STR_GAME_REPLAY: _fixed_str_id = 297;
pub const STR_CD_CHANGE_1OR2: _fixed_str_id = 296;
pub const STR_CD_CHANGE: _fixed_str_id = 295;
pub const STR_DL_LEVEL_ACE: _fixed_str_id = 294;
pub const STR_DL_LEVEL_SPECIAL: _fixed_str_id = 293;
pub const STR_DL_LEVEL_ELITE: _fixed_str_id = 292;
pub const STR_DL_LEVEL_CRACK: _fixed_str_id = 291;
pub const STR_DL_LEVEL_VETERAN: _fixed_str_id = 290;
pub const STR_DL_LEVEL_PROF: _fixed_str_id = 289;
pub const STR_DL_LEVEL_REGULAR: _fixed_str_id = 288;
pub const STR_DL_LEVEL_TRAINED: _fixed_str_id = 287;
pub const STR_DL_LEVEL_GREEN: _fixed_str_id = 286;
pub const STR_DL_LEVEL_ROOKIE: _fixed_str_id = 285;
pub const STR_MR_LEVEL_ACE: _fixed_str_id = 284;
pub const STR_MR_LEVEL_SPECIAL: _fixed_str_id = 283;
pub const STR_MR_LEVEL_ELITE: _fixed_str_id = 282;
pub const STR_MR_LEVEL_CRACK: _fixed_str_id = 281;
pub const STR_MR_LEVEL_VETERAN: _fixed_str_id = 280;
pub const STR_MR_LEVEL_PROF: _fixed_str_id = 279;
pub const STR_MR_LEVEL_REGULAR: _fixed_str_id = 278;
pub const STR_MR_LEVEL_TRAINED: _fixed_str_id = 277;
pub const STR_MR_LEVEL_GREEN: _fixed_str_id = 276;
pub const STR_MR_LEVEL_ROOKIE: _fixed_str_id = 275;
pub const STR_MR_BABAS_RUN_OVER: _fixed_str_id = 274;
pub const STR_MR_SHOTS_OFF_TARGET: _fixed_str_id = 273;
pub const STR_MR_SHOTS_ON_TARGET: _fixed_str_id = 272;
pub const STR_MR_GAME_TIME: _fixed_str_id = 271;
pub const STR_MR_MISSION_TIME: _fixed_str_id = 270;
pub const STR_MR_ARTEFACTS_FOUND: _fixed_str_id = 269;
pub const STR_MR_STR_NOW: _fixed_str_id = 268;
pub const STR_MR_STR_LOST: _fixed_str_id = 267;
pub const STR_MR_STR_BLOWN_UP: _fixed_str_id = 266;
pub const STR_MR_STR_BUILT: _fixed_str_id = 265;
pub const STR_MR_AV_UNIT_EL: _fixed_str_id = 264;
pub const STR_MR_UNITS_NOW: _fixed_str_id = 263;
pub const STR_MR_UNITS_LOST: _fixed_str_id = 262;
pub const STR_MR_UNITS_KILLED: _fixed_str_id = 261;
pub const STR_MR_UNITS_BUILT: _fixed_str_id = 260;
pub const STR_MR_RANKINGS: _fixed_str_id = 259;
pub const STR_MR_FORCE_INFO: _fixed_str_id = 258;
pub const STR_MR_STRUCTURE_LOSSES: _fixed_str_id = 257;
pub const STR_MR_UNIT_LOSSES: _fixed_str_id = 256;
pub const STR_MR_CONTINUE: _fixed_str_id = 255;
pub const STR_MR_QUIT_TO_MAIN: _fixed_str_id = 254;
pub const STR_MR_LOAD_GAME: _fixed_str_id = 253;
pub const STR_MR_SAVE_GAME: _fixed_str_id = 252;
pub const STR_MR_OBJECTIVE_FAILED: _fixed_str_id = 251;
pub const STR_MR_OBJECTIVE_ACHIEVED: _fixed_str_id = 250;
pub const STR_GP_ALLIGN: _fixed_str_id = 249;
pub const STR_GP_CENTERED: _fixed_str_id = 248;
pub const STR_GP_ASSIGNED: _fixed_str_id = 247;
pub const STR_GP_SELECTED: _fixed_str_id = 246;
pub const STR_GAM_REPNOTFOUND: _fixed_str_id = 245;
pub const STR_GAM_RESNOTFOUND: _fixed_str_id = 244;
pub const STR_GAM_REWREPN: _fixed_str_id = 243;
pub const STR_GAM_REWREPA: _fixed_str_id = 242;
pub const STR_GAM_REWNOWT: _fixed_str_id = 241;
pub const STR_GAM_REWWEAP: _fixed_str_id = 240;
pub const STR_GAM_REWBODY: _fixed_str_id = 239;
pub const STR_GAM_REWPROP: _fixed_str_id = 238;
pub const STR_GAM_REWELEC: _fixed_str_id = 237;
pub const STR_GAM_JOINING: _fixed_str_id = 236;
pub const STR_GAM_GAMEOVER: _fixed_str_id = 235;
pub const STR_GAM_REINF: _fixed_str_id = 234;
pub const STR_GAM_UNITSEL: _fixed_str_id = 233;
pub const STR_GAM_RESREWARD: _fixed_str_id = 232;
pub const STR_GAM_NOHQ: _fixed_str_id = 231;
pub const STR_GAM_GOHQ: _fixed_str_id = 230;
pub const STR_GAM_STRREST: _fixed_str_id = 229;
pub const STR_GAM_ELECDAM: _fixed_str_id = 228;
pub const STR_GAM_STRDAM: _fixed_str_id = 227;
pub const STR_GAM_POWCON: _fixed_str_id = 226;
pub const STR_GAM_RESREM: _fixed_str_id = 225;
pub const STR_GAM_ENERGY: _fixed_str_id = 224;
pub const STR_GAM_NORTH: _fixed_str_id = 223;
pub const STR_GAM_UNITLOST: _fixed_str_id = 222;
pub const STR_GAM_DROIDSTATE: _fixed_str_id = 221;
pub const STR_GAM_DERRICK: _fixed_str_id = 220;
pub const STR_DORD_FACTORY: _fixed_str_id = 219;
pub const STR_DORD_RECYCLE: _fixed_str_id = 218;
pub const STR_DORD_ARMRECYCLE: _fixed_str_id = 217;
pub const STR_DORD_EMBARK: _fixed_str_id = 216;
pub const STR_DORD_RETBASE: _fixed_str_id = 215;
pub const STR_DORD_RETREPAIR: _fixed_str_id = 214;
pub const STR_DORD_HOLDPOS: _fixed_str_id = 213;
pub const STR_DORD_GUARD: _fixed_str_id = 212;
pub const STR_DORD_PERSUE: _fixed_str_id = 211;
pub const STR_DORD_PATROL: _fixed_str_id = 210;
pub const STR_DORD_FIRE3: _fixed_str_id = 209;
pub const STR_DORD_FIRE2: _fixed_str_id = 208;
pub const STR_DORD_FIRE1: _fixed_str_id = 207;
pub const STR_DORD_REPAIR3: _fixed_str_id = 206;
pub const STR_DORD_REPAIR2: _fixed_str_id = 205;
pub const STR_DORD_REPAIR1: _fixed_str_id = 204;
pub const STR_DORD_RANGE3: _fixed_str_id = 203;
pub const STR_DORD_RANGE2: _fixed_str_id = 202;
pub const STR_DORD_RANGE1: _fixed_str_id = 201;
pub const STR_MUL_RESPOND: _fixed_str_id = 200;
pub const STR_MUL_JOINING: _fixed_str_id = 199;
pub const STR_MUL_LEAVE: _fixed_str_id = 198;
pub const STR_HARD: _fixed_str_id = 197;
pub const STR_NORMAL: _fixed_str_id = 196;
pub const STR_EASY: _fixed_str_id = 195;
pub const STR_FE_DIFFICULTY: _fixed_str_id = 194;
pub const STR_FE_TRANSPARENCY: _fixed_str_id = 193;
pub const STR_FE_FOG: _fixed_str_id = 192;
pub const STR_FE_EFFECTS: _fixed_str_id = 191;
pub const STR_FE_TEXTURE: _fixed_str_id = 190;
pub const STR_FE_AUDIO: _fixed_str_id = 189;
pub const STR_FE_GRAPHICS: _fixed_str_id = 188;
pub const STR_FE_GAME: _fixed_str_id = 187;
pub const STR_FE_GLIDE: _fixed_str_id = 186;
pub const STR_FE_OPENGL: _fixed_str_id = 185;
pub const STR_FE_DIRECTX: _fixed_str_id = 184;
pub const STR_FE_SOFTWARE: _fixed_str_id = 183;
pub const STR_FE_VIDEO: _fixed_str_id = 182;
pub const STR_FE_GOODFOG: _fixed_str_id = 181;
pub const STR_FE_CRAPFOG: _fixed_str_id = 180;
pub const STR_FE_CLAN: _fixed_str_id = 179;
pub const STR_FE_MUSIC: _fixed_str_id = 178;
pub const STR_FE_3D_FX: _fixed_str_id = 177;
pub const STR_FE_FX: _fixed_str_id = 176;
pub const STR_FE_GAMMA: _fixed_str_id = 175;
pub const STR_FE_SCROLL: _fixed_str_id = 174;
pub const STR_FE_MOUSE: _fixed_str_id = 173;
pub const STR_FE_SIDEOPTIONS: _fixed_str_id = 172;
pub const STR_FE_SKIRMISH: _fixed_str_id = 171;
pub const STR_FE_FORCEEDIT: _fixed_str_id = 170;
pub const STR_FE_JOIN: _fixed_str_id = 169;
pub const STR_FE_HOST: _fixed_str_id = 168;
pub const STR_FE_SIDEMULTI: _fixed_str_id = 167;
pub const STR_FE_RETURN: _fixed_str_id = 166;
pub const STR_FE_FASTPLAY: _fixed_str_id = 165;
pub const STR_FE_TUT: _fixed_str_id = 164;
pub const STR_FE_LOAD: _fixed_str_id = 163;
pub const STR_FE_NEW: _fixed_str_id = 162;
pub const STR_FE_SIDETUT: _fixed_str_id = 161;
pub const STR_FE_SIDESINGLE2: _fixed_str_id = 160;
pub const STR_FE_SIDESINGLE1: _fixed_str_id = 159;
pub const STR_FE_QUIT: _fixed_str_id = 158;
pub const STR_FE_INTRO: _fixed_str_id = 157;
pub const STR_FE_OPTIONS: _fixed_str_id = 156;
pub const STR_FE_MULTI: _fixed_str_id = 155;
pub const STR_FE_SINGLE: _fixed_str_id = 154;
// FrontEnd STrings
pub const STR_FE_SIDEMAIN: _fixed_str_id = 153;
pub const STR_GAME_RESUME: _fixed_str_id = 152;
//ingame ops.
pub const STR_GAME_QUIT: _fixed_str_id = 151;
pub const STR_GAME_NAME: _fixed_str_id = 150;
pub const STR_PLAYER_NAME: _fixed_str_id = 149;
pub const STR_COMPATIBLE: _fixed_str_id = 148;
pub const STR_MUL_ARTIF: _fixed_str_id = 147;
pub const STR_GIFT_POW: _fixed_str_id = 146;
pub const STR_GIFT_TEC: _fixed_str_id = 145;
pub const STR_GIFT_DRO: _fixed_str_id = 144;
pub const STR_GIFT_VIS: _fixed_str_id = 143;
pub const STR_ALLI_FRM: _fixed_str_id = 142;
pub const STR_ALLI_BRK: _fixed_str_id = 141;
pub const STR_ALLI_OFF: _fixed_str_id = 140;
pub const STR_ALLI_REQ: _fixed_str_id = 139;
pub const STR_ALLI_POW: _fixed_str_id = 138;
pub const STR_ALLI_DRO: _fixed_str_id = 137;
pub const STR_ALLI_TEC: _fixed_str_id = 136;
pub const STR_ALLI_VIS: _fixed_str_id = 135;
pub const STR_ALLI_STATE: _fixed_str_id = 134;
pub const STR_MUL_FOG_OFF: _fixed_str_id = 133;
pub const STR_MUL_FOG_ON: _fixed_str_id = 132;
pub const STR_MUL_TEAMPLAY: _fixed_str_id = 131;
pub const STR_MUL_SKIRMISH: _fixed_str_id = 130;
pub const STR_MUL_STRLIM: _fixed_str_id = 129;
pub const STR_MUL_COMP_N: _fixed_str_id = 128;
pub const STR_MUL_COMP_Y: _fixed_str_id = 127;
pub const STR_MUL_COMP: _fixed_str_id = 126;
pub const STR_MUL_PLAY: _fixed_str_id = 125;
pub const STR_MUL_PLAYERS: _fixed_str_id = 124;
pub const STR_LABEL_FOG: _fixed_str_id = 123;
pub const STR_LABEL_LIMIT: _fixed_str_id = 122;
pub const STR_LABEL_BASE: _fixed_str_id = 121;
pub const STR_LABEL_TEC: _fixed_str_id = 120;
pub const STR_LABEL_ALLI: _fixed_str_id = 119;
pub const STR_LABEL_TYPE: _fixed_str_id = 118;
pub const STR_GAMES_GAMES: _fixed_str_id = 117;
pub const STR_CON_MORE: _fixed_str_id = 116;
pub const STR_CON_CABLE: _fixed_str_id = 115;
pub const STR_CON_LAN: _fixed_str_id = 114;
pub const STR_CON_INTERNET: _fixed_str_id = 113;
pub const STR_CON_MODEM: _fixed_str_id = 112;
pub const STR_MUL_FRAGLIM: _fixed_str_id = 111;
pub const STR_MUL_TIMELIM: _fixed_str_id = 110;
pub const STR_MUL_NOLIM: _fixed_str_id = 109;
pub const STR_MUL_ALLIANCEY: _fixed_str_id = 108;
pub const STR_MUL_ALLIANCEN: _fixed_str_id = 107;
pub const STR_MUL_GAMEIC: _fixed_str_id = 106;
pub const STR_MUL_MAPIC: _fixed_str_id = 105;
pub const STR_MUL_FORCEIC: _fixed_str_id = 104;
pub const STR_MUL_PLAYERIC: _fixed_str_id = 103;
pub const STR_MUL_CAMPBASE: _fixed_str_id = 102;
pub const STR_MUL_CAMPDEFENCE: _fixed_str_id = 101;
pub const STR_MUL_CAMPCLEAN: _fixed_str_id = 100;
pub const STR_MUL_TECH3: _fixed_str_id = 99;
pub const STR_MUL_TECH2: _fixed_str_id = 98;
pub const STR_MUL_TECH1: _fixed_str_id = 97;
pub const STR_MUL_POWHI: _fixed_str_id = 96;
pub const STR_MUL_POWMED: _fixed_str_id = 95;
pub const STR_MUL_POWLO: _fixed_str_id = 94;
pub const STR_MUL_PING: _fixed_str_id = 93;
pub const STR_MUL_KILLS: _fixed_str_id = 92;
pub const STR_MUL_SCORE: _fixed_str_id = 91;
pub const STR_MUL_ALLIANCES: _fixed_str_id = 90;
pub const STR_MUL_STARTING: _fixed_str_id = 89;
pub const STR_MUL_CHAT: _fixed_str_id = 88;
pub const STR_MUL_SIDEINFO: _fixed_str_id = 87;
pub const STR_MUL_SIDETEMPLATES: _fixed_str_id = 86;
pub const STR_MUL_SIDEFORCE: _fixed_str_id = 85;
pub const STR_MUL_SIDEPLAYERS: _fixed_str_id = 84;
pub const STR_MUL_SIDEGAMES: _fixed_str_id = 83;
pub const STR_MUL_SIDEOPTIONS: _fixed_str_id = 82;
pub const STR_MUL_SIDECONNECTION: _fixed_str_id = 81;
pub const STR_MUL_115200: _fixed_str_id = 80;
pub const STR_MUL_56000: _fixed_str_id = 79;
pub const STR_MUL_19200: _fixed_str_id = 78;
pub const STR_MUL_14400: _fixed_str_id = 77;
pub const STR_MUL_SEARCHING: _fixed_str_id = 76;
pub const STR_MUL_DESIGN: _fixed_str_id = 75;
pub const STR_MUL_SAVE: _fixed_str_id = 74;
pub const STR_MUL_LOAD: _fixed_str_id = 73;
pub const STR_MUL_DEFAULT: _fixed_str_id = 72;
pub const STR_MUL_CLEAR: _fixed_str_id = 71;
pub const STR_MUL_HOST: _fixed_str_id = 70;
//	STR_MUL_RESLO,	
//	STR_MUL_RESMED,		
//	STR_MUL_RESHI,		
//	STR_MUL_HELPON,	
//	STR_MUL_HELPOFF,		
pub const STR_MUL_REFRESH: _fixed_str_id = 69;
pub const STR_MUL_CAMPAIGN: _fixed_str_id = 68;
pub const STR_MUL_ARENA: _fixed_str_id = 67;
pub const STR_MUL_MAXPLAY: _fixed_str_id = 66;
pub const STR_MUL_GAME: _fixed_str_id = 65;
pub const STR_MUL_PLAYER: _fixed_str_id = 64;
pub const STR_MUL_OK: _fixed_str_id = 63;
pub const STR_MUL_CANCEL: _fixed_str_id = 62;
pub const STR_MUL_COM4: _fixed_str_id = 61;
pub const STR_MUL_COM3: _fixed_str_id = 60;
pub const STR_MUL_COM2: _fixed_str_id = 59;
pub const STR_MUL_COM1: _fixed_str_id = 58;
pub const STR_MUL_IPADDR: _fixed_str_id = 57;
// multiplayer strings
pub const STR_MUL_PHONENO: _fixed_str_id = 56;
pub const STR_INT_POWER: _fixed_str_id = 55;
pub const STR_INT_POWERACCRUED: _fixed_str_id = 54;
pub const STR_INT_LOOP: _fixed_str_id = 53;
pub const STR_INT_DPOINT: _fixed_str_id = 52;
pub const STR_INT_TRANSLAUNCH: _fixed_str_id = 51;
pub const STR_INT_TRANSPORTER: _fixed_str_id = 50;
pub const STR_INT_RESCOMPLETED: _fixed_str_id = 49;
pub const STR_INT_MISMESSAGE: _fixed_str_id = 48;
pub const STR_INT_GENMESSAGE: _fixed_str_id = 47;
pub const STR_INT_RESMESSAGE: _fixed_str_id = 46;
pub const STR_INT_PWRUSAGE: _fixed_str_id = 45;
pub const STR_INT_BLDSPEED: _fixed_str_id = 44;
// Other interface strings
pub const STR_INT_BLDPROGRESS: _fixed_str_id = 43;
pub const STR_DES_TEMPBODY: _fixed_str_id = 42;
pub const STR_DES_TEMPPOWER: _fixed_str_id = 41;
pub const STR_DES_TURRET: _fixed_str_id = 40;
pub const STR_DES_PROPULSION: _fixed_str_id = 39;
pub const STR_DES_BODY: _fixed_str_id = 38;
pub const STR_DES_COMMAND: _fixed_str_id = 37;
pub const STR_DES_OTHER: _fixed_str_id = 36;
pub const STR_DES_WEAPONS: _fixed_str_id = 35;
pub const STR_DES_WATER: _fixed_str_id = 34;
pub const STR_DES_OFFROAD: _fixed_str_id = 33;
pub const STR_DES_ROAD: _fixed_str_id = 32;
pub const STR_DES_AIR: _fixed_str_id = 31;
pub const STR_DES_ROF: _fixed_str_id = 30;
pub const STR_DES_DAMAGE: _fixed_str_id = 29;
pub const STR_DES_RANGE: _fixed_str_id = 28;
pub const STR_DES_BUILD_POINTS: _fixed_str_id = 27;
pub const STR_DES_ECM_POWER: _fixed_str_id = 26;
pub const STR_DES_SENSOR_POWER: _fixed_str_id = 25;
pub const STR_DES_SENSOR_RANGE: _fixed_str_id = 24;
pub const STR_DES_POWERUSE: _fixed_str_id = 23;
pub const STR_DES_WEIGHT: _fixed_str_id = 22;
pub const STR_DES_POWER: _fixed_str_id = 21;
pub const STR_DES_ARMOUR_HEAT: _fixed_str_id = 20;
pub const STR_DES_ARMOUR_KIN: _fixed_str_id = 19;
pub const STR_DES_NEW: _fixed_str_id = 18;
pub const STR_DES_DEL: _fixed_str_id = 17;
pub const STR_DES_CANCEL: _fixed_str_id = 16;
pub const STR_DES_STORE: _fixed_str_id = 15;
// Design screen strings
pub const STR_DES_NEWVEH: _fixed_str_id = 14;
pub const STR_MISC_PAUSED: _fixed_str_id = 13;
pub const STR_MISC_QUIT: _fixed_str_id = 12;
pub const STR_MISC_SAVEGAME: _fixed_str_id = 11;
pub const STR_MISC_LOADGAME: _fixed_str_id = 10;
// Miscellaneous strings
pub const STR_MISC_CLOSE: _fixed_str_id = 9;
pub const STR_RET_COMMAND: _fixed_str_id = 8;
pub const STR_RET_CLOSE: _fixed_str_id = 7;
pub const STR_RET_BUILD: _fixed_str_id = 6;
pub const STR_RET_RESEARCH: _fixed_str_id = 5;
pub const STR_RET_DESIGN: _fixed_str_id = 4;
pub const STR_RET_MANUFACTURE: _fixed_str_id = 3;
pub const STR_RET_INTELLIGENCE: _fixed_str_id = 2;
// Reticule strings
pub const STR_RET_OPTIONS: _fixed_str_id = 1;
// The default (id == 0) string
pub const STR_DEFAULT: _fixed_str_id = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TABDEF {
    pub MajorUp: SWORD,
    pub MajorDown: SWORD,
    pub MajorHilight: SWORD,
    pub MajorSelected: SWORD,
    pub MinorUp: SWORD,
    pub MinorDown: SWORD,
    pub MinorHilight: SWORD,
    pub MinorSelected: SWORD,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_slider {
    pub formID: UDWORD,
    pub id: UDWORD,
    pub type_0: WIDGET_TYPE,
    pub style: UDWORD,
    pub x: SWORD,
    pub y: SWORD,
    pub width: UWORD,
    pub height: UWORD,
    pub display: WIDGET_DISPLAY,
    pub callback: WIDGET_CALLBACK,
    pub pUserData: *mut libc::c_void,
    pub UserData: UDWORD,
    pub psNext: *mut _widget,
    pub orientation: UWORD,
    pub numStops: UWORD,
    pub barSize: UWORD,
    pub pos: UWORD,
    pub state: UWORD,
    pub pTip: *mut STRING,
}
/* Slider state */
// Slider is being dragged
// Slider is hilited
pub type W_SLIDER = _w_slider;
/* The common widget data */
// The orientation of the slider
// Number of stop positions on the slider
// Thickness of slider bar
// Current stop position of the slider
// Slider state
// Tool tip
/* **********************************************/
/* Image ID definition file created by Framer. */
/* **********************************************/
pub type C2RustUnnamed = libc::c_uint;
pub const IMAGE_NOJOIN: C2RustUnnamed = 281;
pub const IMAGE_TFONT_45: C2RustUnnamed = 280;
pub const IMAGE_TFONT_63: C2RustUnnamed = 279;
pub const IMAGE_TFONT_205: C2RustUnnamed = 278;
pub const IMAGE_TFONT_204: C2RustUnnamed = 277;
pub const IMAGE_TFONT_207: C2RustUnnamed = 276;
pub const IMAGE_TFONT_206: C2RustUnnamed = 275;
pub const IMAGE_TFONT_211: C2RustUnnamed = 274;
pub const IMAGE_TFONT_202: C2RustUnnamed = 273;
pub const IMAGE_TFONT_170: C2RustUnnamed = 272;
pub const IMAGE_TFONT_163: C2RustUnnamed = 271;
pub const IMAGE_TFONT_223: C2RustUnnamed = 270;
pub const IMAGE_MULTIRANK3: C2RustUnnamed = 269;
pub const IMAGE_KEYMAP_DEFAULT: C2RustUnnamed = 268;
pub const IMAGE_NOPENCIL: C2RustUnnamed = 267;
pub const IMAGE_PENCIL: C2RustUnnamed = 266;
pub const IMAGE_TFONT_245: C2RustUnnamed = 265;
pub const IMAGE_TFONT_221: C2RustUnnamed = 264;
pub const IMAGE_TFONT_217: C2RustUnnamed = 263;
pub const IMAGE_TFONT_219: C2RustUnnamed = 262;
pub const IMAGE_TFONT_218: C2RustUnnamed = 261;
pub const IMAGE_TFONT_213: C2RustUnnamed = 260;
pub const IMAGE_TFONT_210: C2RustUnnamed = 259;
pub const IMAGE_TFONT_212: C2RustUnnamed = 258;
pub const IMAGE_TFONT_200: C2RustUnnamed = 257;
pub const IMAGE_TFONT_203: C2RustUnnamed = 256;
pub const IMAGE_TFONT_208: C2RustUnnamed = 255;
pub const IMAGE_TFONT_240: C2RustUnnamed = 254;
pub const IMAGE_TFONT_195: C2RustUnnamed = 253;
pub const IMAGE_TFONT_192: C2RustUnnamed = 252;
pub const IMAGE_TFONT_194: C2RustUnnamed = 251;
pub const IMAGE_TFONT_193: C2RustUnnamed = 250;
pub const IMAGE_TFONT_187: C2RustUnnamed = 249;
pub const IMAGE_TFONT_171: C2RustUnnamed = 248;
pub const IMAGE_TFONT_172: C2RustUnnamed = 247;
pub const IMAGE_TFONT_174: C2RustUnnamed = 246;
pub const IMAGE_TFONT_161: C2RustUnnamed = 245;
pub const IMAGE_TFONT_191: C2RustUnnamed = 244;
pub const IMAGE_TFONT_176: C2RustUnnamed = 243;
pub const IMAGE_TFONT_131: C2RustUnnamed = 242;
pub const IMAGE_TFONT_215: C2RustUnnamed = 241;
pub const IMAGE_TFONT_216: C2RustUnnamed = 240;
pub const IMAGE_TFONT_220: C2RustUnnamed = 239;
pub const IMAGE_TFONT_214: C2RustUnnamed = 238;
pub const IMAGE_TFONT_201: C2RustUnnamed = 237;
pub const IMAGE_TFONT_197: C2RustUnnamed = 236;
pub const IMAGE_TFONT_196: C2RustUnnamed = 235;
pub const IMAGE_TFONT_198: C2RustUnnamed = 234;
pub const IMAGE_TFONT_128: C2RustUnnamed = 233;
pub const IMAGE_TFONT_253: C2RustUnnamed = 232;
pub const IMAGE_TFONT_252: C2RustUnnamed = 231;
pub const IMAGE_TFONT_251: C2RustUnnamed = 230;
pub const IMAGE_TFONT_250: C2RustUnnamed = 229;
pub const IMAGE_TFONT_249: C2RustUnnamed = 228;
pub const IMAGE_TFONT_248: C2RustUnnamed = 227;
pub const IMAGE_TFONT_246: C2RustUnnamed = 226;
pub const IMAGE_TFONT_244: C2RustUnnamed = 225;
pub const IMAGE_TFONT_243: C2RustUnnamed = 224;
pub const IMAGE_TFONT_242: C2RustUnnamed = 223;
pub const IMAGE_TFONT_209: C2RustUnnamed = 222;
pub const IMAGE_TFONT_241: C2RustUnnamed = 221;
pub const IMAGE_TFONT_239: C2RustUnnamed = 220;
pub const IMAGE_TFONT_238: C2RustUnnamed = 219;
pub const IMAGE_TFONT_237: C2RustUnnamed = 218;
pub const IMAGE_TFONT_236: C2RustUnnamed = 217;
pub const IMAGE_TFONT_235: C2RustUnnamed = 216;
pub const IMAGE_TFONT_234: C2RustUnnamed = 215;
pub const IMAGE_TFONT_233: C2RustUnnamed = 214;
pub const IMAGE_TFONT_232: C2RustUnnamed = 213;
pub const IMAGE_TFONT_199: C2RustUnnamed = 212;
pub const IMAGE_TFONT_230: C2RustUnnamed = 211;
pub const IMAGE_TFONT_229: C2RustUnnamed = 210;
pub const IMAGE_TFONT_228: C2RustUnnamed = 209;
pub const IMAGE_TFONT_227: C2RustUnnamed = 208;
pub const IMAGE_TFONT_226: C2RustUnnamed = 207;
pub const IMAGE_TFONT_188: C2RustUnnamed = 206;
pub const IMAGE_TFONT_224: C2RustUnnamed = 205;
pub const IMAGE_TFONT_225: C2RustUnnamed = 204;
pub const IMAGE_TFONT_189: C2RustUnnamed = 203;
pub const IMAGE_WEE_GUY: C2RustUnnamed = 202;
pub const IMAGE_FOG_ON_HI: C2RustUnnamed = 201;
pub const IMAGE_FOG_OFF_HI: C2RustUnnamed = 200;
pub const IMAGE_FOG_ON: C2RustUnnamed = 199;
pub const IMAGE_FOG_OFF: C2RustUnnamed = 198;
pub const IMAGE_PLAYERX: C2RustUnnamed = 197;
pub const IMAGE_MEDAL_DUMMY: C2RustUnnamed = 196;
pub const IMAGE_MULTIRANK2: C2RustUnnamed = 195;
pub const IMAGE_PLAYER_PC: C2RustUnnamed = 194;
pub const IMAGE_TEAM_HI: C2RustUnnamed = 193;
pub const IMAGE_SKIRMISH_HI: C2RustUnnamed = 192;
pub const IMAGE_TEAM: C2RustUnnamed = 191;
pub const IMAGE_SKIRMISH: C2RustUnnamed = 190;
pub const IMAGE_TEAM_OVER: C2RustUnnamed = 189;
pub const IMAGE_SKIRMISH_OVER: C2RustUnnamed = 188;
pub const IMAGE_LAMP_GREEN: C2RustUnnamed = 187;
pub const IMAGE_LAMP_AMBER: C2RustUnnamed = 186;
pub const IMAGE_LAMP_RED: C2RustUnnamed = 185;
pub const IMAGE_COMPUTER_Y_HI: C2RustUnnamed = 184;
pub const IMAGE_COMPUTER_Y: C2RustUnnamed = 183;
pub const IMAGE_COMPUTER_N: C2RustUnnamed = 182;
pub const IMAGE_COMPUTER_N_HI: C2RustUnnamed = 181;
pub const IMAGE_HI56: C2RustUnnamed = 180;
pub const IMAGE_DEFAULTFORCE: C2RustUnnamed = 179;
pub const IMAGE_CLEARFORCE: C2RustUnnamed = 178;
pub const IMAGE_SAVEFORCE: C2RustUnnamed = 177;
pub const IMAGE_LOADFORCE: C2RustUnnamed = 176;
pub const IMAGE_SLIM_HI: C2RustUnnamed = 175;
pub const IMAGE_SLIM: C2RustUnnamed = 174;
pub const IMAGE_RETURN_HI: C2RustUnnamed = 173;
pub const IMAGE_FRAGLIMIT_HI: C2RustUnnamed = 172;
pub const IMAGE_TIMELIMIT_HI: C2RustUnnamed = 171;
pub const IMAGE_NOLIMIT_HI: C2RustUnnamed = 170;
pub const IMAGE_FRAGLIMIT: C2RustUnnamed = 169;
pub const IMAGE_TIMELIMIT: C2RustUnnamed = 168;
pub const IMAGE_NOLIMIT: C2RustUnnamed = 167;
pub const IMAGE_LBASE_HI: C2RustUnnamed = 166;
pub const IMAGE_SBASE_HI: C2RustUnnamed = 165;
pub const IMAGE_NOBASE_HI: C2RustUnnamed = 164;
pub const IMAGE_LBASE: C2RustUnnamed = 163;
pub const IMAGE_SBASE: C2RustUnnamed = 162;
pub const IMAGE_NOBASE: C2RustUnnamed = 161;
pub const IMAGE_TECHHI_HI: C2RustUnnamed = 160;
pub const IMAGE_TECHMED_HI: C2RustUnnamed = 159;
pub const IMAGE_TECHLO_HI: C2RustUnnamed = 158;
pub const IMAGE_TECHHI: C2RustUnnamed = 157;
pub const IMAGE_TECHMED: C2RustUnnamed = 156;
pub const IMAGE_TECHLO: C2RustUnnamed = 155;
pub const IMAGE_BIGOK: C2RustUnnamed = 154;
pub const IMAGE_EDIT_GAME: C2RustUnnamed = 153;
pub const IMAGE_EDIT_MAP: C2RustUnnamed = 152;
pub const IMAGE_EDIT_FORCE: C2RustUnnamed = 151;
pub const IMAGE_EDIT_PLAYER: C2RustUnnamed = 150;
pub const IMAGE_RETURN: C2RustUnnamed = 149;
pub const IMAGE_MULTIRANK1: C2RustUnnamed = 148;
pub const IMAGE_POWLO: C2RustUnnamed = 147;
pub const IMAGE_MEDAL_BRONZE: C2RustUnnamed = 146;
pub const IMAGE_MEDAL_SILVER: C2RustUnnamed = 145;
pub const IMAGE_MEDAL_GOLD: C2RustUnnamed = 144;
pub const IMAGE_CAMPAIGN_OVER: C2RustUnnamed = 143;
pub const IMAGE_ARENA_OVER: C2RustUnnamed = 142;
pub const IMAGE_HI64: C2RustUnnamed = 141;
pub const IMAGE_HI41: C2RustUnnamed = 140;
pub const IMAGE_HI39: C2RustUnnamed = 139;
pub const IMAGE_HI23: C2RustUnnamed = 138;
pub const IMAGE_HI31: C2RustUnnamed = 137;
pub const IMAGE_HI34: C2RustUnnamed = 136;
pub const IMAGE_COM4_HI: C2RustUnnamed = 135;
pub const IMAGE_COM3_HI: C2RustUnnamed = 134;
pub const IMAGE_ALLI_HI: C2RustUnnamed = 133;
pub const IMAGE_OFFALLI_HI: C2RustUnnamed = 132;
pub const IMAGE_NOALLI_HI: C2RustUnnamed = 131;
pub const IMAGE_ALLI: C2RustUnnamed = 130;
pub const IMAGE_OFFALLI: C2RustUnnamed = 129;
pub const IMAGE_NOALLI: C2RustUnnamed = 128;
pub const IMAGE_POWHI_HI: C2RustUnnamed = 127;
pub const IMAGE_POWMED_HI: C2RustUnnamed = 126;
pub const IMAGE_POWLO_HI: C2RustUnnamed = 125;
pub const IMAGE_POWHI: C2RustUnnamed = 124;
pub const IMAGE_POWMED: C2RustUnnamed = 123;
pub const IMAGE_OK: C2RustUnnamed = 122;
pub const IMAGE_NO: C2RustUnnamed = 121;
pub const IMAGE_HOST: C2RustUnnamed = 120;
pub const IMAGE_PLAYER7: C2RustUnnamed = 119;
pub const IMAGE_PLAYER6: C2RustUnnamed = 118;
pub const IMAGE_PLAYER5: C2RustUnnamed = 117;
pub const IMAGE_PLAYER4: C2RustUnnamed = 116;
pub const IMAGE_PLAYER3: C2RustUnnamed = 115;
pub const IMAGE_PLAYER2: C2RustUnnamed = 114;
pub const IMAGE_PLAYER1: C2RustUnnamed = 113;
pub const IMAGE_PLAYER0: C2RustUnnamed = 112;
pub const IMAGE_REFRESH: C2RustUnnamed = 111;
pub const IMAGE_CAMPAIGN: C2RustUnnamed = 110;
pub const IMAGE_ARENA: C2RustUnnamed = 109;
pub const IMAGE_115200: C2RustUnnamed = 108;
pub const IMAGE_56000: C2RustUnnamed = 107;
pub const IMAGE_19200: C2RustUnnamed = 106;
pub const IMAGE_14400: C2RustUnnamed = 105;
pub const IMAGE_CAMPAIGN_HI: C2RustUnnamed = 104;
pub const IMAGE_ARENA_HI: C2RustUnnamed = 103;
pub const IMAGE_115200_HI: C2RustUnnamed = 102;
pub const IMAGE_56000_HI: C2RustUnnamed = 101;
pub const IMAGE_19200_HI: C2RustUnnamed = 100;
pub const IMAGE_14400_HI: C2RustUnnamed = 99;
pub const IMAGE_COM2_HI: C2RustUnnamed = 98;
pub const IMAGE_COM1_HI: C2RustUnnamed = 97;
pub const IMAGE_COM4: C2RustUnnamed = 96;
pub const IMAGE_COM3: C2RustUnnamed = 95;
pub const IMAGE_COM2: C2RustUnnamed = 94;
pub const IMAGE_COM1: C2RustUnnamed = 93;
pub const IMAGE_TFONT_126: C2RustUnnamed = 92;
pub const IMAGE_TFONT_125: C2RustUnnamed = 91;
pub const IMAGE_TFONT_124: C2RustUnnamed = 90;
pub const IMAGE_TFONT_123: C2RustUnnamed = 89;
pub const IMAGE_TFONT_122: C2RustUnnamed = 88;
pub const IMAGE_TFONT_121: C2RustUnnamed = 87;
pub const IMAGE_TFONT_120: C2RustUnnamed = 86;
pub const IMAGE_TFONT_119: C2RustUnnamed = 85;
pub const IMAGE_TFONT_118: C2RustUnnamed = 84;
pub const IMAGE_TFONT_117: C2RustUnnamed = 83;
pub const IMAGE_TFONT_116: C2RustUnnamed = 82;
pub const IMAGE_TFONT_115: C2RustUnnamed = 81;
pub const IMAGE_TFONT_114: C2RustUnnamed = 80;
pub const IMAGE_TFONT_113: C2RustUnnamed = 79;
pub const IMAGE_TFONT_112: C2RustUnnamed = 78;
pub const IMAGE_TFONT_111: C2RustUnnamed = 77;
pub const IMAGE_TFONT_110: C2RustUnnamed = 76;
pub const IMAGE_TFONT_109: C2RustUnnamed = 75;
pub const IMAGE_TFONT_108: C2RustUnnamed = 74;
pub const IMAGE_TFONT_107: C2RustUnnamed = 73;
pub const IMAGE_TFONT_106: C2RustUnnamed = 72;
pub const IMAGE_TFONT_105: C2RustUnnamed = 71;
pub const IMAGE_TFONT_104: C2RustUnnamed = 70;
pub const IMAGE_TFONT_103: C2RustUnnamed = 69;
pub const IMAGE_TFONT_102: C2RustUnnamed = 68;
pub const IMAGE_TFONT_101: C2RustUnnamed = 67;
pub const IMAGE_TFONT_100: C2RustUnnamed = 66;
pub const IMAGE_TFONT_99: C2RustUnnamed = 65;
pub const IMAGE_TFONT_98: C2RustUnnamed = 64;
pub const IMAGE_TFONT_97: C2RustUnnamed = 63;
pub const IMAGE_TFONT_96: C2RustUnnamed = 62;
pub const IMAGE_TFONT_95: C2RustUnnamed = 61;
pub const IMAGE_TFONT_94: C2RustUnnamed = 60;
pub const IMAGE_TFONT_93: C2RustUnnamed = 59;
pub const IMAGE_TFONT_92: C2RustUnnamed = 58;
pub const IMAGE_TFONT_91: C2RustUnnamed = 57;
pub const IMAGE_TFONT_90: C2RustUnnamed = 56;
pub const IMAGE_TFONT_89: C2RustUnnamed = 55;
pub const IMAGE_TFONT_88: C2RustUnnamed = 54;
pub const IMAGE_TFONT_87: C2RustUnnamed = 53;
pub const IMAGE_TFONT_86: C2RustUnnamed = 52;
pub const IMAGE_TFONT_85: C2RustUnnamed = 51;
pub const IMAGE_TFONT_84: C2RustUnnamed = 50;
pub const IMAGE_TFONT_83: C2RustUnnamed = 49;
pub const IMAGE_TFONT_82: C2RustUnnamed = 48;
pub const IMAGE_TFONT_81: C2RustUnnamed = 47;
pub const IMAGE_TFONT_80: C2RustUnnamed = 46;
pub const IMAGE_TFONT_79: C2RustUnnamed = 45;
pub const IMAGE_TFONT_78: C2RustUnnamed = 44;
pub const IMAGE_TFONT_77: C2RustUnnamed = 43;
pub const IMAGE_TFONT_76: C2RustUnnamed = 42;
pub const IMAGE_TFONT_75: C2RustUnnamed = 41;
pub const IMAGE_TFONT_74: C2RustUnnamed = 40;
pub const IMAGE_TFONT_73: C2RustUnnamed = 39;
pub const IMAGE_TFONT_72: C2RustUnnamed = 38;
pub const IMAGE_TFONT_71: C2RustUnnamed = 37;
pub const IMAGE_TFONT_70: C2RustUnnamed = 36;
pub const IMAGE_TFONT_69: C2RustUnnamed = 35;
pub const IMAGE_TFONT_68: C2RustUnnamed = 34;
pub const IMAGE_TFONT_67: C2RustUnnamed = 33;
pub const IMAGE_TFONT_66: C2RustUnnamed = 32;
pub const IMAGE_TFONT_65: C2RustUnnamed = 31;
pub const IMAGE_TFONT_64: C2RustUnnamed = 30;
pub const IMAGE_TFONT_62: C2RustUnnamed = 29;
pub const IMAGE_TFONT_61: C2RustUnnamed = 28;
pub const IMAGE_TFONT_60: C2RustUnnamed = 27;
pub const IMAGE_TFONT_59: C2RustUnnamed = 26;
pub const IMAGE_TFONT_58: C2RustUnnamed = 25;
pub const IMAGE_TFONT_57: C2RustUnnamed = 24;
pub const IMAGE_TFONT_56: C2RustUnnamed = 23;
pub const IMAGE_TFONT_55: C2RustUnnamed = 22;
pub const IMAGE_TFONT_54: C2RustUnnamed = 21;
pub const IMAGE_TFONT_53: C2RustUnnamed = 20;
pub const IMAGE_TFONT_52: C2RustUnnamed = 19;
pub const IMAGE_TFONT_51: C2RustUnnamed = 18;
pub const IMAGE_TFONT_50: C2RustUnnamed = 17;
pub const IMAGE_TFONT_49: C2RustUnnamed = 16;
pub const IMAGE_TFONT_48: C2RustUnnamed = 15;
pub const IMAGE_TFONT_47: C2RustUnnamed = 14;
pub const IMAGE_TFONT_46: C2RustUnnamed = 13;
pub const IMAGE_TFONT_44: C2RustUnnamed = 12;
pub const IMAGE_TFONT_43: C2RustUnnamed = 11;
pub const IMAGE_TFONT_42: C2RustUnnamed = 10;
pub const IMAGE_TFONT_41: C2RustUnnamed = 9;
pub const IMAGE_TFONT_40: C2RustUnnamed = 8;
pub const IMAGE_TFONT_39: C2RustUnnamed = 7;
pub const IMAGE_TFONT_38: C2RustUnnamed = 6;
pub const IMAGE_TFONT_37: C2RustUnnamed = 5;
pub const IMAGE_TFONT_36: C2RustUnnamed = 4;
pub const IMAGE_TFONT_35: C2RustUnnamed = 3;
pub const IMAGE_TFONT_34: C2RustUnnamed = 2;
pub const IMAGE_TFONT_33: C2RustUnnamed = 1;
pub const IMAGE_FE_LOGO: C2RustUnnamed = 0;
pub type _title_mode = libc::c_uint;
pub const GAME3: _title_mode = 18;
pub const GAME2: _title_mode = 17;
pub const KEYMAP: _title_mode = 16;
pub const LOADSAVEGAME: _title_mode = 15;
pub const QUIT: _title_mode = 14;
pub const SHOWINTRO: _title_mode = 13;
pub const STARTGAME: _title_mode = 12;
pub const MULTILIMIT: _title_mode = 11;
pub const GAMEFIND: _title_mode = 10;
pub const FORCESELECT: _title_mode = 9;
pub const MULTIOPTION: _title_mode = 8;
pub const PROTOCOL: _title_mode = 7;
pub const CREDITS: _title_mode = 6;
pub const TUTORIAL: _title_mode = 5;
pub const GAME: _title_mode = 4;
pub const OPTIONS: _title_mode = 3;
pub const MULTI: _title_mode = 2;
pub const SINGLE: _title_mode = 1;
pub const TITLE: _title_mode = 0;
/* 
 * Frontend.h
 */
// determines which option screen to use. when in GS_TITLE_SCREEN mode.
pub type tMode = _title_mode;
pub type CONSOLE_TEXT_JUSTIFICATION = libc::c_uint;
pub const DEFAULT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 3;
pub const CENTRE_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 2;
pub const RIGHT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 1;
pub const LEFT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 0;
pub type DEPTH_MODE = libc::c_uint;
pub const DEPTH_CMP_ALWAYS_WRT_OFF: DEPTH_MODE = 3;
pub const DEPTH_CMP_LEQ_WRT_OFF: DEPTH_MODE = 2;
pub const DEPTH_CMP_ALWAYS_WRT_ON: DEPTH_MODE = 1;
pub const DEPTH_CMP_LEQ_WRT_ON: DEPTH_MODE = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MULTIPLAYERINGAME {
    pub PingTimes: [UDWORD; 8],
    pub localOptionsReceived: BOOL,
    pub localJoiningInProgress: BOOL,
    pub JoiningInProgress: [BOOL; 8],
    pub bHostSetup: BOOL,
    pub startTime: UDWORD,
    pub modem: UDWORD,
    pub numStructureLimits: UDWORD,
    pub pStructureLimits: *mut UBYTE,
    pub skScores: [[UDWORD; 2]; 8],
    pub phrases: [[CHAR; 255]; 5],
}
// 0 intro mode
// 1 single player menu
// 2 multiplayer menu
// 3 options menu	
// 4
// 5  tutorial/fastplay	
// 6  credits
// 7  MULTIPLAYER, select proto
// 8 MULTIPLAYER, select game options
// 9 MULTIPLAYER, Force design screen
// 10 MULTIPLAYER, gamefinder.
// 11 MULTIPLAYER, Limit the multistuff.
// 12 Fire up the game
// 13 reshow the intro
// 14 leaving game
// 15 loading a save game
// 16 keymap editor
// 17 second options menu.
// 18 third options menu.
//		GRAPHICS,					// 5
//		VIDEO,
//	DEMOMODE,					// demo mode. remove for release?
// ////////////////////////////////////////////////////////////////////////////
#[no_mangle]
pub unsafe extern "C" fn useStruct(mut count: UDWORD, mut i: UDWORD) -> BOOL {
    //	STRUCTURE_STATS	*pStat = asStructureStats+i;
    if count >= (4 as libc::c_int * 8 as libc::c_int) as libc::c_uint {
        return 0 as libc::c_int
    }
    // now see if we loaded that stat..
    if (*asStructLimits[0 as libc::c_int as
                            usize].offset(i as isize)).globalLimit as
           libc::c_int == 255 as libc::c_int {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
/*
 * multilimit.h
 */
// ////////////////////////////////////////////////////////////////////////////
// protos.
// ////////////////////////////////////////////////////////////////////////////
#[no_mangle]
pub unsafe extern "C" fn startLimitScreen() -> BOOL {
    let mut sButInit: W_FORMINIT =
        W_FORMINIT{formID: 0,
                   majorID: 0,
                   minorID: 0,
                   id: 0,
                   style: 0,
                   x: 0,
                   y: 0,
                   width: 0,
                   height: 0,
                   pDisplay: None,
                   pCallback: None,
                   pUserData: 0 as *mut libc::c_void,
                   UserData: 0,
                   disableChildren: 0,
                   majorPos: 0,
                   minorPos: 0,
                   majorSize: 0,
                   minorSize: 0,
                   majorOffset: 0,
                   minorOffset: 0,
                   tabVertOffset: 0,
                   tabHorzOffset: 0,
                   tabMajorThickness: 0,
                   tabMinorThickness: 0,
                   tabMajorGap: 0,
                   tabMinorGap: 0,
                   numMajor: 0,
                   aNumMinors: [0; 9],
                   pTip: 0 as *mut STRING,
                   apMajorTips: [0 as *mut STRING; 9],
                   apMinorTips: [[0 as *mut STRING; 5]; 9],
                   pTabDisplay: None,
                   pFormDisplay: None,}; //background
    let mut sFormInit: W_FORMINIT =
        W_FORMINIT{formID: 0,
                   majorID: 0,
                   minorID: 0,
                   id: 0,
                   style: 0,
                   x: 0,
                   y: 0,
                   width: 0,
                   height: 0,
                   pDisplay: None,
                   pCallback: None,
                   pUserData: 0 as *mut libc::c_void,
                   UserData: 0,
                   disableChildren: 0,
                   majorPos: 0,
                   minorPos: 0,
                   majorSize: 0,
                   minorSize: 0,
                   majorOffset: 0,
                   minorOffset: 0,
                   tabVertOffset: 0,
                   tabHorzOffset: 0,
                   tabMajorThickness: 0,
                   tabMinorThickness: 0,
                   tabMajorGap: 0,
                   tabMinorGap: 0,
                   numMajor: 0,
                   aNumMinors: [0; 9],
                   pTip: 0 as *mut STRING,
                   apMajorTips: [0 as *mut STRING; 9],
                   apMinorTips: [[0 as *mut STRING; 5]; 9],
                   pTabDisplay: None,
                   pFormDisplay: None,};
    let mut numButtons: UDWORD = 0 as libc::c_int as UDWORD;
    let mut i: UDWORD = 0;
    addBackdrop();
    //printf("[startLimitScreen]========= Enable priority here?\n");
	// load stats...
    if bForceEditorLoaded == 0 {
        initLoadingScreen(1 as libc::c_int,
                          1 as libc::c_int); //changed by jeremy mar8
        /*		if (!resLoad("wrf\\forcedit.wrf", 500,
					 DisplayBuffer, displayBufferSize,
					 psGameHeap))				//need the object heaps to have been set up before loading
		{
			return FALSE;
		}
*/
        if resLoad(b"wrf\\piestats.wrf\x00" as *const u8 as
                       *const libc::c_char as *mut STRING, 501 as libc::c_int,
                   DisplayBuffer, displayBufferSize as SDWORD, psGameHeap) ==
               0 {
            //need the object heaps to have been set up before loading
            return 0 as libc::c_int
        }
        if resLoad(b"wrf\\forcedit2.wrf\x00" as *const u8 as
                       *const libc::c_char as *mut STRING, 502 as libc::c_int,
                   DisplayBuffer, displayBufferSize as SDWORD, psGameHeap) ==
               0 {
            //need the object heaps to have been set up before loading
            return 0 as libc::c_int
        } //force to black
        pie_GlobalRenderBegin(); // draw sidetext...
        bForceEditorLoaded = 1 as libc::c_int; // draw blue form...
        closeLoadingScreen();
    }
    addSideText(20027 as libc::c_int as UDWORD,
                (25 as libc::c_int - 2 as libc::c_int) as UDWORD,
                30 as libc::c_int as UDWORD,
                b"LIMITS\x00" as *const u8 as *const libc::c_char as
                    *mut STRING);
    memset(&mut sFormInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_FORMINIT>() as libc::c_ulong);
    sFormInit.formID = 20000 as libc::c_int as UDWORD;
    sFormInit.id = 22000 as libc::c_int as UDWORD;
    sFormInit.style = 0 as libc::c_int as UDWORD;
    sFormInit.x = 25 as libc::c_int as SWORD;
    sFormInit.y = 30 as libc::c_int as SWORD;
    sFormInit.width = 580 as libc::c_int as UWORD;
    sFormInit.height = 430 as libc::c_int as UWORD;
    sFormInit.pDisplay =
        Some(intDisplayPlainForm as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    widgAddForm(psWScreen, &mut sFormInit);
    // return button.
//	addMultiBut(psWScreen,IDLIMITS,IDLIMITS_RETURN,
//					8,5,
//					iV_GetImageWidth(FrontImages,IMAGE_RETURN),
//					iV_GetImageHeight(FrontImages,IMAGE_RETURN),
//					STR_MUL_CANCEL,IMAGE_RETURN,IMAGE_RETURN_HI,TRUE);
    // ok button
//	addMultiBut(psWScreen,IDLIMITS,IDLIMITS_OK,
//					LIMITS_OKX,LIMITS_OKY,
//					iV_GetImageWidth(FrontImages,IMAGE_BIGOK),
//					iV_GetImageHeight(FrontImages,IMAGE_BIGOK),
//					STR_MUL_OK,IMAGE_BIGOK,IMAGE_BIGOK,TRUE);
    addMultiBut(psWScreen, 22000 as libc::c_int as UDWORD,
                (22000 as libc::c_int + 1 as libc::c_int) as UDWORD,
                (580 as libc::c_int - 90 as libc::c_int - 40 as libc::c_int)
                    as UDWORD,
                (430 as libc::c_int - 42 as libc::c_int) as UDWORD,
                iV_GetImageWidth(FrontImages,
                                 IMAGE_RETURN as libc::c_int as UWORD) as
                    UDWORD,
                iV_GetImageHeight(FrontImages,
                                  IMAGE_RETURN as libc::c_int as UWORD) as
                    UDWORD, STR_MUL_CANCEL as libc::c_int as UDWORD,
                IMAGE_NO as libc::c_int as UDWORD,
                IMAGE_NO as libc::c_int as UDWORD, 1 as libc::c_int);
    // ok button
    addMultiBut(psWScreen, 22000 as libc::c_int as UDWORD,
                (22000 as libc::c_int + 2 as libc::c_int) as UDWORD,
                (580 as libc::c_int - 90 as libc::c_int) as UDWORD,
                (430 as libc::c_int - 42 as libc::c_int) as UDWORD,
                iV_GetImageWidth(FrontImages,
                                 IMAGE_BIGOK as libc::c_int as UWORD) as
                    UDWORD,
                iV_GetImageHeight(FrontImages,
                                  IMAGE_BIGOK as libc::c_int as UWORD) as
                    UDWORD, STR_MUL_OK as libc::c_int as UDWORD,
                IMAGE_OK as libc::c_int as UDWORD,
                IMAGE_OK as libc::c_int as UDWORD, 1 as libc::c_int);
    // Count the number of minor tabs needed
    numButtons = 0 as libc::c_int as UDWORD;
    i = 0 as libc::c_int as UDWORD;
    while i < numStructureStats {
        if useStruct(numButtons, i) != 0 {
            numButtons = numButtons.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    }
    if numButtons > (4 as libc::c_int * 8 as libc::c_int) as libc::c_uint {
        numButtons = (4 as libc::c_int * 8 as libc::c_int) as UDWORD
    }
    // add tab form..
    memset(&mut sFormInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_FORMINIT>() as libc::c_ulong); // !!
    sFormInit.formID = 22000 as libc::c_int as UDWORD; //(DES_TAB_HEIGHT/2)+2;
    sFormInit.id = (22000 as libc::c_int + 3 as libc::c_int) as UDWORD;
    sFormInit.style = 1 as libc::c_int as UDWORD;
    sFormInit.x = 50 as libc::c_int as SWORD;
    sFormInit.y = 10 as libc::c_int as SWORD;
    sFormInit.width = (580 as libc::c_int - 100 as libc::c_int) as UWORD;
    sFormInit.height = (430 as libc::c_int - 4 as libc::c_int) as UWORD;
    sFormInit.numMajor = numForms(numButtons, 8 as libc::c_int as UDWORD);
    sFormInit.majorPos = 1 as libc::c_int as UWORD;
    sFormInit.minorPos = 0 as libc::c_int as UWORD;
    sFormInit.majorSize = (26 as libc::c_int + 3 as libc::c_int) as UWORD;
    sFormInit.majorOffset = 2 as libc::c_int as SWORD;
    sFormInit.tabVertOffset = (11 as libc::c_int / 2 as libc::c_int) as SWORD;
    sFormInit.tabMajorThickness = 11 as libc::c_int as UWORD;
    sFormInit.pFormDisplay =
        Some(intDisplayObjectForm as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    sFormInit.pUserData =
        &mut StandardTab as *mut TABDEF as *mut libc::c_void;
    sFormInit.pTabDisplay =
        Some(intDisplayTab as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: UDWORD, _: BOOL, _: BOOL, _: UDWORD,
                                      _: UDWORD, _: UDWORD, _: UDWORD) -> ());
    i = 0 as libc::c_int as UDWORD;
    while i < sFormInit.numMajor as libc::c_uint {
        sFormInit.aNumMinors[i as usize] = 1 as libc::c_int as UWORD;
        i = i.wrapping_add(1)
    }
    widgAddForm(psWScreen, &mut sFormInit);
    //Put the buttons on it
    memset(&mut sButInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_BUTINIT>() as libc::c_ulong); //IDLIMITS;
    sButInit.formID = (22000 as libc::c_int + 3 as libc::c_int) as UDWORD;
    sButInit.style = 0 as libc::c_int as UDWORD;
    sButInit.width = 480 as libc::c_int as UWORD;
    sButInit.height = 40 as libc::c_int as UWORD;
    sButInit.pDisplay =
        Some(displayStructureBar as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    sButInit.x = 2 as libc::c_int as SWORD;
    sButInit.y = 5 as libc::c_int as SWORD;
    sButInit.id = (22000 as libc::c_int + 4 as libc::c_int) as UDWORD;
    numButtons = 0 as libc::c_int as UDWORD;
    i = 0 as libc::c_int as UDWORD;
    while i < numStructureStats {
        if useStruct(numButtons, i) != 0 {
            numButtons = numButtons.wrapping_add(1);
            sButInit.pUserData = i as *mut libc::c_void;
            widgAddForm(psWScreen, &mut sButInit);
            sButInit.id = sButInit.id.wrapping_add(1);
            addFESlider(sButInit.id,
                        sButInit.id.wrapping_sub(1 as libc::c_int as
                                                     libc::c_uint),
                        290 as libc::c_int as UDWORD,
                        11 as libc::c_int as UDWORD,
                        (*asStructLimits[0 as libc::c_int as
                                             usize].offset(i as
                                                               isize)).globalLimit
                            as UDWORD,
                        (*asStructLimits[0 as libc::c_int as
                                             usize].offset(i as isize)).limit
                            as UDWORD, 0 as libc::c_int as UDWORD);
            sButInit.id = sButInit.id.wrapping_add(1);
            if sButInit.y as libc::c_int + 40 as libc::c_int +
                   2 as libc::c_int >
                   8 as libc::c_int * (40 as libc::c_int + 2 as libc::c_int) -
                       4 as libc::c_int {
                sButInit.y = 5 as libc::c_int as SWORD;
                sButInit.majorID =
                    (sButInit.majorID as libc::c_int + 1 as libc::c_int) as
                        UWORD
            } else {
                sButInit.y =
                    (sButInit.y as libc::c_int +
                         (40 as libc::c_int + 5 as libc::c_int)) as SWORD
            }
        }
        i = i.wrapping_add(1)
    }
    return 1 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////////
#[no_mangle]
pub unsafe extern "C" fn runLimitScreen() {
    let mut id: UDWORD = 0; // network stuff.
    let mut statid: UDWORD = 0; // Run the current set of widgets
    processFrontendSnap(0 as libc::c_int);
    frontendMultiMessages();
    id = widgRunScreen(psWScreen);
    // sliders
    if id > (22000 as libc::c_int + 4 as libc::c_int) as libc::c_uint &&
           id < (22000 as libc::c_int + 99 as libc::c_int) as libc::c_uint {
        statid =
            (*widgGetFromID(psWScreen,
                            id.wrapping_sub(1 as libc::c_int as
                                                libc::c_uint))).pUserData as
                UDWORD;
        if statid != 0 {
            (*asStructLimits[0 as libc::c_int as
                                 usize].offset(statid as isize)).limit =
                (*(widgGetFromID(psWScreen, id) as *mut W_SLIDER)).pos as
                    UBYTE
        }
    } else {
        // icons that are always about.
        match id {
            22001 => {
                // reset the sliders..
                eventReset();
                //			resReleaseBlockData(500);
                resReleaseBlockData(501 as libc::c_int);
                resReleaseBlockData(502 as libc::c_int);
                //			eventReset();
                bForceEditorLoaded = 0 as libc::c_int;
                changeTitleMode(MULTIOPTION);
                // make some noize.
                if ingame.localOptionsReceived == 0 {
                    addConsoleMessage(b"Limits Reset To Default Values\x00" as
                                          *const u8 as *const libc::c_char as
                                          *mut STRING,
                                      DEFAULT_JUSTIFY); // show the widgets currently running
                } else {
                    sendTextMessage(b"Limits Reset To Default Values\x00" as
                                        *const u8 as *const libc::c_char as
                                        *mut libc::c_char, 1 as libc::c_int);
                }
            }
            22002 => { createLimitSet(); changeTitleMode(MULTIOPTION); }
            _ => { }
        }
    }
    StartCursorSnap(&mut InterfaceSnap);
    DrawBegin();
    widgDisplayScreen(psWScreen);
    DrawEnd();
}
// ////////////////////////////////////////////////////////////////////////////
#[no_mangle]
pub unsafe extern "C" fn createLimitSet() {
    let mut i: UDWORD = 0;
    let mut numchanges: UDWORD = 0;
    let mut pChanges: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut pEntry: *mut UBYTE = 0 as *mut UBYTE;
    if ingame.numStructureLimits != 0 {
        // free the old set if required.
        ingame.numStructureLimits =
            0 as libc::c_int as UDWORD; // count number of changes
        memFreeRelease(ingame.pStructureLimits as *mut libc::c_void);
        ingame.pStructureLimits = 0 as *mut UBYTE
    }
    numchanges = 0 as libc::c_int as UDWORD;
    i = 0 as libc::c_int as UDWORD;
    while i < numStructureStats {
        if (*asStructLimits[0 as libc::c_int as
                                usize].offset(i as isize)).limit as
               libc::c_int != 255 as libc::c_int {
            numchanges = numchanges.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    }
    //close your eyes now
    pChanges =
        memMallocRelease(numchanges.wrapping_mul((::std::mem::size_of::<UDWORD>()
                                                      as
                                                      libc::c_ulong).wrapping_add(::std::mem::size_of::<UBYTE>()
                                                                                      as
                                                                                      libc::c_ulong))); // allocate some mem for this.
    pEntry = pChanges as *mut UBYTE;
    i = 0 as libc::c_int as UDWORD;
    while i < numStructureStats {
        // prepare chunk.
        if (*asStructLimits[0 as libc::c_int as
                                usize].offset(i as isize)).limit as
               libc::c_int != 255 as libc::c_int {
            memcpy(pEntry as *mut libc::c_void,
                   &mut i as *mut UDWORD as *const libc::c_void,
                   ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
            pEntry =
                pEntry.offset(::std::mem::size_of::<UDWORD>() as libc::c_ulong
                                  as isize);
            memcpy(pEntry as *mut libc::c_void,
                   &mut (*(*asStructLimits.as_mut_ptr().offset(0 as
                                                                   libc::c_int
                                                                   as
                                                                   isize)).offset(i
                                                                                      as
                                                                                      isize)).limit
                       as *mut UBYTE as *const libc::c_void,
                   ::std::mem::size_of::<UBYTE>() as libc::c_ulong);
            pEntry =
                pEntry.offset(::std::mem::size_of::<UBYTE>() as libc::c_ulong
                                  as isize)
        }
        i = i.wrapping_add(1)
    }
    // you can open them again.
    ingame.numStructureLimits = numchanges;
    ingame.pStructureLimits = pChanges as *mut UBYTE;
    sendOptions(0 as libc::c_int, 0 as libc::c_int as UDWORD);
}
// ////////////////////////////////////////////////////////////////////////////
#[no_mangle]
pub unsafe extern "C" fn applyLimitSet() {
    let mut pEntry: *mut UBYTE = 0 as *mut UBYTE;
    let mut i: UDWORD = 0;
    let mut val: UBYTE = 0;
    let mut id: UDWORD = 0;
    if ingame.numStructureLimits == 0 as libc::c_int as libc::c_uint {
        return
    }
    // get the limits
	// decode and apply
    pEntry = ingame.pStructureLimits;
    i = 0 as libc::c_int as UDWORD;
    while i < ingame.numStructureLimits {
        // prepare chunk.
        memcpy(&mut id as *mut UDWORD as *mut libc::c_void,
               pEntry as *const libc::c_void,
               ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
        pEntry =
            pEntry.offset(::std::mem::size_of::<UDWORD>() as libc::c_ulong as
                              isize);
        memcpy(&mut val as *mut UBYTE as *mut libc::c_void,
               pEntry as *const libc::c_void,
               ::std::mem::size_of::<UBYTE>() as libc::c_ulong);
        pEntry =
            pEntry.offset(::std::mem::size_of::<UBYTE>() as libc::c_ulong as
                              isize);
        if id < numStructureStats {
            (*asStructLimits[0 as libc::c_int as
                                 usize].offset(id as isize)).limit = val;
            (*asStructLimits[1 as libc::c_int as
                                 usize].offset(id as isize)).limit = val;
            (*asStructLimits[2 as libc::c_int as
                                 usize].offset(id as isize)).limit = val;
            (*asStructLimits[3 as libc::c_int as
                                 usize].offset(id as isize)).limit = val;
            (*asStructLimits[4 as libc::c_int as
                                 usize].offset(id as isize)).limit = val;
            (*asStructLimits[5 as libc::c_int as
                                 usize].offset(id as isize)).limit = val;
            (*asStructLimits[6 as libc::c_int as
                                 usize].offset(id as isize)).limit = val;
            (*asStructLimits[7 as libc::c_int as
                                 usize].offset(id as isize)).limit = val
        }
        i = i.wrapping_add(1)
    }
    // free.
    if ingame.numStructureLimits != 0 {
        memFreeRelease(ingame.pStructureLimits as *mut libc::c_void);
        ingame.pStructureLimits = 0 as *mut UBYTE;
        ingame.numStructureLimits = 0 as libc::c_int as UDWORD
    };
}
// ////////////////////////////////////////////////////////////////////////////
#[no_mangle]
pub unsafe extern "C" fn displayStructureBar(mut psWidget: *mut _widget,
                                             mut xOffset: UDWORD,
                                             mut yOffset: UDWORD,
                                             mut pColours: *mut UDWORD) {
    let mut x: UDWORD = xOffset.wrapping_add((*psWidget).x as libc::c_uint);
    let mut y: UDWORD = yOffset.wrapping_add((*psWidget).y as libc::c_uint);
    let mut w: UDWORD = (*psWidget).width as UDWORD;
    let mut h: UDWORD = (*psWidget).height as UDWORD;
    let mut stat: *mut STRUCTURE_STATS =
        asStructureStats.offset((*psWidget).pUserData as UDWORD as isize);
    let mut Rotation: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut Position: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut str: [CHAR; 3] = [0; 3];
    let mut scale: UDWORD = 0;
    let mut Radius: UDWORD = 0;
    drawBlueBox(x, y, w, h);
    // draw image
    pie_SetGeometricOffset(x.wrapping_add(35 as libc::c_int as libc::c_uint)
                               as libc::c_int,
                           y.wrapping_add(((*psWidget).height as libc::c_int /
                                               2 as libc::c_int) as
                                              libc::c_uint).wrapping_add(9 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint)
                               as libc::c_int); //45
    Rotation.x =
        -(15 as
              libc::c_int); //getStructureStatSize(stat)  * 38 * OBJECT_RADIUS;
    Rotation.y =
        gameTime2.wrapping_div(45 as libc::c_int as
                                   libc::c_uint).wrapping_rem(360 as
                                                                  libc::c_int
                                                                  as
                                                                  libc::c_uint)
            as int32;
    Rotation.z = 0 as libc::c_int;
    Position.x = 0 as libc::c_int;
    Position.y = 0 as libc::c_int;
    Position.z = 2000 as libc::c_int * 2 as libc::c_int;
    Radius = getStructureStatSize(stat);
    if Radius <= 128 as libc::c_int as libc::c_uint {
        scale = 55 as libc::c_int as UDWORD
    } else if Radius <= 256 as libc::c_int as libc::c_uint {
        scale = 25 as libc::c_int as UDWORD
    } else { scale = 25 as libc::c_int as UDWORD }
    pie_SetDepthBufferStatus(DEPTH_CMP_LEQ_WRT_ON);
    displayStructureStatButton(stat, 0 as libc::c_int as UDWORD,
                               &mut Rotation, &mut Position, 1 as libc::c_int,
                               scale as SDWORD);
    pie_SetDepthBufferStatus(DEPTH_CMP_ALWAYS_WRT_ON);
    // draw name
    iV_SetFont(WFont); // font
    iV_SetTextColour(-(1 as libc::c_int) as SWORD); //colour
    pie_DrawText(getName((*stat).pName),
                 x.wrapping_add(80 as libc::c_int as libc::c_uint),
                 y.wrapping_add(((*psWidget).height as libc::c_int /
                                     2 as libc::c_int) as
                                    libc::c_uint).wrapping_add(3 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uint));
    // draw limit
    sprintf(str.as_mut_ptr(), b"%d\x00" as *const u8 as *const libc::c_char,
            (*(widgGetFromID(psWScreen,
                             (*psWidget).id.wrapping_add(1 as libc::c_int as
                                                             libc::c_uint)) as
                   *mut W_SLIDER)).pos as libc::c_int);
    pie_DrawText(str.as_mut_ptr(),
                 x.wrapping_add(270 as libc::c_int as libc::c_uint),
                 y.wrapping_add(((*psWidget).height as libc::c_int /
                                     2 as libc::c_int) as
                                    libc::c_uint).wrapping_add(3 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uint));
    // add snap
    AddCursorSnap(&mut InterfaceSnap,
                  x.wrapping_add(10 as libc::c_int as libc::c_uint) as SWORD,
                  y.wrapping_add(10 as libc::c_int as libc::c_uint) as SWORD,
                  (*psWidget).formID, (*psWidget).id, 0 as *mut SNAPBIAS);
}
