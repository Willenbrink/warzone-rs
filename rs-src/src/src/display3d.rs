use ::libc;
extern "C" {
    pub type _formation;
    /* Write formatted output to S.  */
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    /* Use misc.  */
    /* Use extended X/Open || misc. */
    /* Return a random integer between 0 and RAND_MAX inclusive.  */
    #[no_mangle]
    fn rand() -> libc::c_int;
    /* Abort execution and generate a core-dump.  */
    #[no_mangle]
    fn abort() -> !;
    /* Return the absolute value of X.  */
    #[no_mangle]
    fn abs(_: libc::c_int) -> libc::c_int;
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
    /* These two functions return the current position of the mouse */
    #[no_mangle]
    fn mouseX() -> SDWORD;
    #[no_mangle]
    fn mouseY() -> SDWORD;
    /* This returns true if the mouse key is currently depressed */
    #[no_mangle]
    fn mouseDown(code: MOUSE_KEY_CODE) -> BOOL;
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
    fn pie_RemainingPasses();
    #[no_mangle]
    fn pie_EndLighting();
    #[no_mangle]
    fn pie_BeginLighting(x: libc::c_float, y: libc::c_float,
                         z: libc::c_float);
    /* Returns the current frame we're on - used to establish whats on screen */
    #[no_mangle]
    fn frameGetFrameNumber() -> UDWORD;
    //*************************************************************************
    #[no_mangle]
    static mut colours: [uint8; 0];
    #[no_mangle]
    fn pal_BuildAdjustedShadeTable();
    #[no_mangle]
    fn pal_GetNearestColour(r: uint8, g: uint8, b: uint8) -> uint8;
    #[no_mangle]
    fn pie_Draw3DShape(shape: *mut iIMDShape, frame: libc::c_int,
                       team: libc::c_int, colour: UDWORD, specular: UDWORD,
                       pieFlag: libc::c_int, pieData: libc::c_int);
    #[no_mangle]
    fn pie_DrawTriangle(pv: *mut iVertex, texPage: *mut iTexture,
                        renderFlags: UDWORD, offset: *mut iPoint);
    #[no_mangle]
    fn pie_DrawPoly(numVrts: SDWORD, aVrts: *mut PIEVERTEX, texPage: SDWORD,
                    psEffects: *mut libc::c_void);
    #[no_mangle]
    fn SetBSPObjectPos(x: SDWORD, y: SDWORD, z: SDWORD);
    #[no_mangle]
    fn SetBSPCameraPos(x: SDWORD, y: SDWORD, z: SDWORD);
    //Sets all states
    #[no_mangle]
    fn pie_SetDepthBufferStatus(depthMode: DEPTH_MODE);
    #[no_mangle]
    fn pie_SetGammaValue(val: libc::c_float);
    #[no_mangle]
    fn pie_GetFogEnabled() -> BOOL;
    //fog currently on
    #[no_mangle]
    fn pie_SetFogStatus(val: BOOL);
    #[no_mangle]
    fn pie_GetFogColour() -> UDWORD;
    //render states
    #[no_mangle]
    fn pie_SetTexturePage(num: SDWORD);
    /* **************************************************************************/
/*
 *	Global ProtoTypes
 */
/* **************************************************************************/
    #[no_mangle]
    fn pie_Set2DClip(x0: libc::c_int, y0: libc::c_int, x1: libc::c_int,
                     y1: libc::c_int);
    #[no_mangle]
    fn pie_GetVideoBufferWidth() -> UDWORD;
    #[no_mangle]
    fn pie_GetVideoBufferHeight() -> UDWORD;
    #[no_mangle]
    static mut aSinTable: [SDWORD; 0];
    //*************************************************************************
    #[no_mangle]
    fn pie_MatBegin();
    #[no_mangle]
    fn pie_MatEnd();
    #[no_mangle]
    fn pie_MATTRANS(x: libc::c_int, y: libc::c_int, z: libc::c_int);
    #[no_mangle]
    fn pie_TRANSLATE(x: libc::c_int, y: libc::c_int, z: libc::c_int);
    #[no_mangle]
    fn pie_MatScale(percent: UDWORD);
    #[no_mangle]
    fn pie_MatRotX(x: libc::c_int);
    #[no_mangle]
    fn pie_MatRotY(y: libc::c_int);
    #[no_mangle]
    fn pie_MatRotZ(z: libc::c_int);
    #[no_mangle]
    fn pie_RotProj(v3d: *mut iVector, v2d: *mut iPoint) -> int32;
    #[no_mangle]
    fn pie_RotateProject(x: SDWORD, y: SDWORD, z: SDWORD, xs: *mut SDWORD,
                         ys: *mut SDWORD) -> int32;
    //*************************************************************************
    #[no_mangle]
    fn pie_PerspectiveBegin();
    #[no_mangle]
    fn pie_PerspectiveEnd();
    #[no_mangle]
    fn pie_SetGeometricOffset(x: libc::c_int, y: libc::c_int);
    #[no_mangle]
    fn pie_Begin3DScene();
    #[no_mangle]
    fn pie_BeginInterface();
    #[no_mangle]
    fn pie_GlobalRenderBegin();
    #[no_mangle]
    fn pie_LocalRenderBegin();
    #[no_mangle]
    fn pie_LocalRenderEnd();
    #[no_mangle]
    fn pie_GetResScalingFactor() -> UDWORD;
    #[no_mangle]
    fn pie_TransColouredTriangle(vrt: *mut PIEVERTEX, rgb: UDWORD,
                                 trans: UDWORD);
    #[no_mangle]
    fn iV_GetTextLineSize() -> libc::c_int;
    #[no_mangle]
    fn iV_GetTextWidth(String: *mut STRING) -> libc::c_int;
    #[no_mangle]
    fn iV_SetTextColour(Index: SWORD);
    #[no_mangle]
    fn pie_DrawText(string: *mut STRING, x: UDWORD, y: UDWORD);
    #[no_mangle]
    static mut rendSurface: iSurface;
    #[no_mangle]
    static mut psRendSurface: *mut iSurface;
    /* **************************************************************************/
/*
 * pieBlitFunc.h
 *
 * patch for exisitng ivis rectangle draw functions.
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
 *	Global Variables
 */
/* **************************************************************************/
    /* **************************************************************************/
/*
 *	Global ProtoTypes
 */
/* **************************************************************************/
    #[no_mangle]
    fn pie_Line(x0: libc::c_int, y0: libc::c_int, x1: libc::c_int,
                y1: libc::c_int, colour: uint32);
    #[no_mangle]
    fn pie_UniTransBoxFill(x0: SDWORD, y0: SDWORD, x1: SDWORD, y1: SDWORD,
                           rgb: UDWORD, transparency: UDWORD);
    #[no_mangle]
    fn pie_TransBoxFill(x0: SDWORD, y0: SDWORD, x1: SDWORD, y1: SDWORD);
    #[no_mangle]
    fn pie_ImageFileID(ImageFile: *mut IMAGEFILE, ID: UWORD, x: libc::c_int,
                       y: libc::c_int);
    #[no_mangle]
    fn pie_BoxFill(x0: libc::c_int, y0: libc::c_int, x1: libc::c_int,
                   y1: libc::c_int, colour: uint32);
    #[no_mangle]
    fn pie_BoxFillIndex(x0: libc::c_int, y0: libc::c_int, x1: libc::c_int,
                        y1: libc::c_int, colour: UBYTE);
    #[no_mangle]
    fn pie_Box(x0: libc::c_int, y0: libc::c_int, x1: libc::c_int,
               y1: libc::c_int, colour: uint32);
    //extern void (*iV_DrawColourTransImage)(IMAGEFILE *ImageFile,UWORD ID,int x,int y,UWORD ColourIndex);
    #[no_mangle]
    static mut iV_UniBitmapDepth:
           Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_int,
                                       _: libc::c_int, _: libc::c_int,
                                       _: libc::c_int, _: libc::c_int,
                                       _: libc::c_int, _: libc::c_int,
                                       _: libc::c_int, _: libc::c_uchar,
                                       _: libc::c_int) -> ()>;
    /* **************************************************************************/
    #[no_mangle]
    fn GetRealCameraPos(Camera: *mut OBJPOS, Distance: SDWORD,
                        CameraLoc: *mut iVector);
    #[no_mangle]
    fn initDemoCamera();
    #[no_mangle]
    fn processDemoCam();
    #[no_mangle]
    fn demoGetStatus() -> BOOL;
    #[no_mangle]
    fn demoProcessTilesIn();
    #[no_mangle]
    fn gamePaused() -> BOOL;
    #[no_mangle]
    fn atmosInitSystem();
    #[no_mangle]
    fn atmosUpdateSystem();
    #[no_mangle]
    fn atmosDrawParticles();
    #[no_mangle]
    fn getPitchToHighestPoint(x: UDWORD, y: UDWORD, direction: UDWORD,
                              thresholdDistance: UDWORD,
                              pitch_0: *mut SDWORD);
    /* uwCycles=0 for infinite looping */
    #[no_mangle]
    fn animObj_Add(pParentObj: *mut libc::c_void, iAnimID: libc::c_int,
                   udwStartDelay: UDWORD, uwCycles: UWORD)
     -> *mut ANIM_OBJECT;
    #[no_mangle]
    fn animObj_Find(pParentObj: *mut libc::c_void, iAnimID: libc::c_int)
     -> *mut ANIM_OBJECT;
    #[no_mangle]
    fn animObj_GetFrame3D(psObj: *mut ANIM_OBJECT, uwObj: UWORD,
                          psPos: *mut VECTOR3D, psVecRot: *mut VECTOR3D,
                          psVecScale: *mut VECTOR3D) -> UWORD;
    #[no_mangle]
    fn audio_PlayObjStaticTrack(psObj: *mut libc::c_void, iTrack: libc::c_int)
     -> BOOL;
    #[no_mangle]
    fn audio_StopObjTrack(psObj: *mut libc::c_void, iTrack: libc::c_int);
    #[no_mangle]
    fn getDroidLevel(psDroid: *mut DROID) -> UDWORD;
    #[no_mangle]
    fn droidUnderRepair(psDroid: *mut DROID) -> BOOL;
    #[no_mangle]
    fn vtolDroid(psDroid: *mut DROID) -> BOOL;
    #[no_mangle]
    fn getNumAttackRuns(psDroid: *mut DROID) -> UWORD;
    #[no_mangle]
    fn droidResistance(psDroid: *mut DROID) -> SWORD;
    /*Looks through the players list of structures to see if a HQ exists - will look
through the list of structures at Home Base when on an offWorld mission map*/
    #[no_mangle]
    fn radarCheckForHQ(player_0: UDWORD) -> BOOL;
    /*Access functions for the upgradeable stats of a structure*/
    #[no_mangle]
    fn structureBody(psStruct: *mut STRUCTURE) -> UDWORD;
    #[no_mangle]
    fn structureResistance(psStats: *mut STRUCTURE_STATS, player_0: UBYTE)
     -> UDWORD;
    /*Used for determining how much of the structure to draw as being built or demolished*/
    #[no_mangle]
    fn structHeightScale(psStruct: *mut STRUCTURE) -> FRACT;
    /* The lists of objects allocated */
    #[no_mangle]
    static mut apsDroidLists: [*mut DROID; 8];
    #[no_mangle]
    static mut apsStructLists: [*mut STRUCTURE; 8];
    #[no_mangle]
    static mut apsFeatureLists: [*mut FEATURE; 8];
    #[no_mangle]
    static mut apsFlagPosLists: [*mut FLAG_POSITION; 8];
    #[no_mangle]
    static mut asSensorStats: *mut SENSOR_STATS;
    //extern PROGRAM_STATS		*asProgramStats;
    #[no_mangle]
    static mut asWeaponStats: *mut WEAPON_STATS;
    /*Access functions for the upgradeable stats of a weapon*/
    #[no_mangle]
    fn weaponFirePause(psStats: *mut WEAPON_STATS, player_0: UBYTE) -> UDWORD;
    /*
#ifndef PSX		// I've even set them up for you...:-)
#define TILE_IN_SENSORRANGE(x)	(x->tileExtraBits & EXTRA_BITS_SENSOR)
#define TILE_EXTRA_BIT2_SET(x)	(x->tileExtraBits & EXTRA_BITS_2)
#define TILE_EXTRA_BIT3_SET(x)	(x->tileExtraBits & EXTRA_BITS_3)
#define TILE_EXTRA_BIT4_SET(x)	(x->tileExtraBits & EXTRA_BITS_4)
#define TILE_EXTRA_BIT5_SET(x)	(x->tileExtraBits & EXTRA_BITS_5)
#define TILE_EXTRA_BIT6_SET(x)	(x->tileExtraBits & EXTRA_BITS_6)
#define TILE_EXTRA_BIT7_SET(x)	(x->tileExtraBits & EXTRA_BITS_7)
#define TILE_EXTRA_BIT8_SET(x)	(x->tileExtraBits & EXTRA_BITS_8)
#endif
*/
    /*
#ifndef PSX	// again, done for you again!
#define SET_TILE_SENSOR(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits | EXTRA_BITS_SENSOR))
#define CLEAR_TILE_SENSOR(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits & (~EXTRA_BITS_SENSOR)))
#define SET_TILE_BIT2(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits | EXTRA_BITS_2))
#define CLEAR_TILE_BIT2(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits & (~EXTRA_BITS_2)))
#define SET_TILE_BIT3(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits | EXTRA_BITS_3))
#define CLEAR_TILE_BIT3(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits & (~EXTRA_BITS_3)))
#define SET_TILE_BIT4(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits | EXTRA_BITS_4))
#define CLEAR_TILE_BIT4(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits & (~EXTRA_BITS_4)))
#define SET_TILE_BIT5(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits | EXTRA_BITS_5))
#define CLEAR_TILE_BIT5(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits & (~EXTRA_BITS_5)))
#define SET_TILE_BIT6(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits | EXTRA_BITS_6))
#define CLEAR_TILE_BIT6(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits & (~EXTRA_BITS_6)))
#define SET_TILE_BIT7(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits | EXTRA_BITS_7))
#define CLEAR_TILE_BIT7(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits & (~EXTRA_BITS_7)))
#define SET_TILE_BIT8(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits | EXTRA_BITS_8))
#define CLEAR_TILE_BIT8(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits & (~EXTRA_BITS_8)))
#endif
*/
// Multiplier for the tile height
    /* Allows us to do if(TRI_FLIPPED(psTile)) */
    /* Flips the triangle partition on a tile pointer */
    /* Can player number p see tile t? */
    /* Set a tile to be visible for a player */
    /*#ifndef PSX
#define SET_TILE_DOOR(p,t) t->tileDoorBits = (UBYTE) (t->tileDoorBits | (1<<p))
#define CLEAR_TILE_DOOR(p,t) t->tileDoorBits = (UBYTE) (t->tileDoorBits & (~(1<<p))) // check logic
// Is there a door here for the player?
#define TEST_TILE_DOOR(p,t) ( (t->tileDoorBits) & (1<<p) )
#endif*/
/* Arbitrary maximum number of terrain textures - used in look up table for terrain type */
    #[no_mangle]
    static mut terrainTypes: [UBYTE; 255];
    /* The maximum map size */
    /* The size and contents of the map */
    #[no_mangle]
    static mut mapWidth: UDWORD;
    #[no_mangle]
    static mut mapHeight: UDWORD;
    #[no_mangle]
    static mut psMapTiles: *mut MAPTILE;
    /* Return height of x,y */
//extern SDWORD map_Height(UDWORD x, UDWORD y);
    #[no_mangle]
    fn map_Height(x: UDWORD, y: UDWORD) -> SWORD;
    // sensorrangedisplay.
    #[no_mangle]
    static mut bDisplaySensorRange: BOOL;
    #[no_mangle]
    fn initBulletTable();
    #[no_mangle]
    fn inQuad(pt: *mut POINT, quad: *mut QUAD) -> libc::c_int;
    // The maximum time for one frame (stops the clock running away when debugging)
// changed to /6 by ajl. if this needs to go back to ticks/10 then tell me. 
    /* The current time in the game world */
    #[no_mangle]
    static mut gameTime: UDWORD;
    /* The current time in the game world */
    #[no_mangle]
    static mut gameTime2: UDWORD;
    // Never stops.
    /* The time for the last frame */
    #[no_mangle]
    static mut frameTime2: UDWORD;
    /* Useful for periodical stuff */
/* Will return a number that climbs over tickFrequency game ticks and ends up in the required range. */
/*	
	For instance getTimeValueRange(4096,256) will return a number that cycles through
	the values 0..256 every 4.096 seconds...
	Ensure that the first is an integer multiple of the second 
*/
    #[no_mangle]
    fn getTimeValueRange(tickFrequency: UDWORD, requiredRange: UDWORD)
     -> UDWORD;
    #[no_mangle]
    fn getStaticTimeValueRange(tickFrequency: UDWORD, requiredRange: UDWORD)
     -> UDWORD;
    #[no_mangle]
    fn getImdFromIndex(index: UDWORD) -> *mut iIMDShape;
    //extern iIMDShape	*pAssemblyPointIMDs[NUM_FACTORY_TYPES][MAX_FACTORY];
    #[no_mangle]
    static mut pAssemblyPointIMDs: [[*mut iIMDShape; 5]; 4];
    /* Maximum number of effects in the world - need to investigate what this should be */
/* EXTERNAL REFERENCES */
    #[no_mangle]
    fn effectGiveAuxVar(var: UDWORD);
    #[no_mangle]
    fn processEffects();
    #[no_mangle]
    fn addEffect(pos: *mut iVector, group: EFFECT_GROUP, type_0: EFFECT_TYPE,
                 specified: BOOL, imd: *mut iIMDShape, lit: BOOL);
    #[no_mangle]
    fn drawEffects();
    #[no_mangle]
    fn effectSetSize(size: UDWORD);
    #[no_mangle]
    static mut buildSite: HIGHLIGHT;
    #[no_mangle]
    static mut buildState: UDWORD;
    /* Which is the currently selected player */
    #[no_mangle]
    static mut selectedPlayer: UDWORD;
    #[no_mangle]
    static mut outlineColour3D: UDWORD;
    /* The list of proximity displays allocated */
    #[no_mangle]
    static mut apsProxDisp: [*mut PROXIMITY_DISPLAY; 8];
    #[no_mangle]
    fn intBuildSelectMode() -> BOOL;
    #[no_mangle]
    static mut mX: SDWORD;
    #[no_mangle]
    static mut mY: SDWORD;
    #[no_mangle]
    fn scroll();
    // deal with selecting a droid
    #[no_mangle]
    fn dealWithDroidSelect(psDroid: *mut DROID, bDragBox: BOOL);
    #[no_mangle]
    fn getDrawShadows() -> BOOL;
    #[no_mangle]
    static mut dragBox3D: _dragBox;
    #[no_mangle]
    static mut wallDrag: _dragBox;
    #[no_mangle]
    static mut gameStats: BOOL;
    #[no_mangle]
    static mut gammaValue: libc::c_float;
    //extern BOOL	forceWidgetsOn;
    #[no_mangle]
    fn mouseTarget() -> *mut BASE_OBJECT;
    #[no_mangle]
    fn getRotActive() -> BOOL;
    #[no_mangle]
    fn getDesiredPitch() -> SDWORD;
    // check whether the queue order keys are pressed
    #[no_mangle]
    fn ctrlShiftDown() -> BOOL;
    #[no_mangle]
    static mut IntImages: *mut IMAGEFILE;
    #[no_mangle]
    fn drawRadarBlips();
    #[no_mangle]
    fn drawRadar();
    #[no_mangle]
    fn SetRadarStrobe(x: UDWORD, y: UDWORD);
    #[no_mangle]
    static mut theSun: iVector;
    #[no_mangle]
    fn lightDoFogAndIllumination(brightness: UBYTE, dx: SDWORD, dz: SDWORD,
                                 pSpecular: *mut UDWORD) -> UDWORD;
    #[no_mangle]
    static mut ConsoleString: [libc::c_char; 255];
    #[no_mangle]
    fn addConsoleMessage(messageText: *mut STRING,
                         jusType: CONSOLE_TEXT_JUSTIFICATION) -> BOOL;
    #[no_mangle]
    fn initConsoleMessages();
    #[no_mangle]
    fn displayConsoleMessages();
    #[no_mangle]
    fn flushConsoleMessages();
    #[no_mangle]
    fn setConsolePermanence(state: BOOL, bClearOld: BOOL);
    #[no_mangle]
    fn permitNewConsoleMessages(allow: BOOL);
    #[no_mangle]
    fn proj_GetFirst() -> *mut PROJ_OBJECT;
    #[no_mangle]
    fn proj_GetNext() -> *mut PROJ_OBJECT;
    #[no_mangle]
    fn gfxVisible(psObj: *mut PROJ_OBJECT) -> BOOL;
    #[no_mangle]
    fn objectShimmy(psObj: *mut BASE_OBJECT);
    /* add an object to the current render list */
    #[no_mangle]
    fn bucketAddTypeToList(objectType: RENDER_TYPE, object: *mut libc::c_void)
     -> BOOL;
    /* render Objects in list */
    #[no_mangle]
    fn bucketRenderCurrentList() -> BOOL;
    /* MapDisplay.h */
    /*Flag to switch code for bucket sorting in renderFeatures etc
  for the renderMapToBuffer code */
  /*This is no longer used but may be useful for testing so I've left it in - maybe
  get rid of it eventually? - AB 1/4/98*/
    #[no_mangle]
    static mut doBucket: BOOL;
    #[no_mangle]
    fn getPlayerColour(pl: UDWORD) -> UBYTE;
    #[no_mangle]
    fn displayComponentObject(psObj: *mut BASE_OBJECT);
    //testing only - remove when decided
    #[no_mangle]
    fn updateLightLevels();
    /* Externally referenced functions */
    #[no_mangle]
    fn initWarCam();
    #[no_mangle]
    fn getWarCamStatus() -> BOOL;
    #[no_mangle]
    fn camToggleStatus();
    #[no_mangle]
    fn processWarCam() -> BOOL;
    #[no_mangle]
    fn targetInitialise();
    #[no_mangle]
    fn targetOpenList(psTargeting: *mut BASE_OBJECT);
    #[no_mangle]
    fn targetCloseList();
    #[no_mangle]
    fn targetAdd(psObj: *mut BASE_OBJECT);
    #[no_mangle]
    fn targetGetCurrent() -> *mut BASE_OBJECT;
    #[no_mangle]
    fn targetMarkCurrent();
    #[no_mangle]
    fn getDebugMappingStatus() -> BOOL;
    #[no_mangle]
    static mut DirectControl: BOOL;
    #[no_mangle]
    static mut psDrivenDroid: *mut DROID;
    #[no_mangle]
    fn bobTransporterHeight() -> SDWORD;
    #[no_mangle]
    fn war_GetTranslucent() -> BOOL;
    #[no_mangle]
    static mut bAllowOtherKeyPresses: BOOL;
    // dirty but necessary
    #[no_mangle]
    static mut sTextToSend: [STRING; 255];
    #[no_mangle]
    fn orderStateObj(psDroid: *mut DROID, order: DROID_ORDER,
                     ppsObj: *mut *mut BASE_OBJECT) -> BOOL;
    #[no_mangle]
    fn orderState(psDroid: *mut DROID, order: DROID_ORDER) -> BOOL;
    // the game description.
    #[no_mangle]
    static mut bMultiPlayer: BOOL;
    #[no_mangle]
    fn isHumanPlayer(player_0: UDWORD) -> BOOL;
    // -------------------------------------------------------------------------------
    #[no_mangle]
    fn waterOnMap() -> BOOL;
    #[no_mangle]
    fn environGetData(x: UDWORD, y: UDWORD) -> UDWORD;
    #[no_mangle]
    fn avUpdateTiles();
    #[no_mangle]
    fn avGetObjLightLevel(psObj: *mut BASE_OBJECT, origLevel: UDWORD)
     -> UDWORD;
    #[no_mangle]
    fn getRevealStatus() -> BOOL;
    #[no_mangle]
    static mut tileTexInfo: [TILE_TEX_INFO; 100];
    // get the index of the command droid
    #[no_mangle]
    fn cmdDroidGetIndex(psCommander: *mut DROID) -> SDWORD;
    /* Points for flipping the texture around if the tile is flipped or rotated */
    #[no_mangle]
    static mut sP1: POINT;
    #[no_mangle]
    static mut sP2: POINT;
    #[no_mangle]
    static mut sP3: POINT;
    #[no_mangle]
    static mut sP4: POINT;
    #[no_mangle]
    static mut psP1: *mut POINT;
    #[no_mangle]
    static mut psP2: *mut POINT;
    #[no_mangle]
    static mut psP3: *mut POINT;
    #[no_mangle]
    static mut psP4: *mut POINT;
    #[no_mangle]
    static mut psPTemp: *mut POINT;
}
pub type UBYTE = libc::c_uchar;
pub type SBYTE = libc::c_schar;
pub type STRING = libc::c_char;
pub type UWORD = libc::c_ushort;
pub type SWORD = libc::c_short;
pub type UDWORD = libc::c_uint;
pub type SDWORD = libc::c_int;
pub type BOOL = libc::c_int;
pub type BYTE = libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct POINT {
    pub x: libc::c_int,
    pub y: libc::c_int,
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
pub type _mouse_key_code = libc::c_uint;
pub const MOUSE_WDN: _mouse_key_code = 5;
pub const MOUSE_WUP: _mouse_key_code = 4;
pub const MOUSE_RMB: _mouse_key_code = 3;
pub const MOUSE_MMB: _mouse_key_code = 2;
pub const MOUSE_LMB: _mouse_key_code = 1;
pub type MOUSE_KEY_CODE = _mouse_key_code;
pub type FRACT = libc::c_float;
pub type int32 = libc::c_int;
pub type uint8 = libc::c_uchar;
pub type uint16 = libc::c_ushort;
pub type uint32 = libc::c_uint;
//*************************************************************************
//
// Simple derived types
//
//*************************************************************************
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iClip {
    pub left: libc::c_int,
    pub top: libc::c_int,
    pub right: libc::c_int,
    pub bottom: libc::c_int,
}
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
pub struct iPoint {
    pub x: int32,
    pub y: int32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iSprite {
    pub width: libc::c_uint,
    pub height: libc::c_uint,
    pub bmp: *mut iBitmap,
}
pub type iPalette = [iColour; 256];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iVector {
    pub x: int32,
    pub y: int32,
    pub z: int32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iTexture {
    pub xshift: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub bmp: *mut iBitmap,
    pub pPal: *mut iColour,
    pub bColourKeyed: iBool,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iView {
    pub p: iVector,
    pub r: iVector,
}
/* **************************************************************************/
/*
 * ivisdef.h
 *
 * type defines for all ivis library functions.
 *
 */
/* **************************************************************************/
/* **************************************************************************/
/* **************************************************************************/
/*
 *	Global Definitions
 */
/* **************************************************************************/
// now defined for all versions (optional BSP handled on all formats)
//Not really needed I guess, however, see debug.c comments.  -Qamly
// texture animation defines
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
//*************************************************************************
//
// Basic types (now defined in pieTypes.h)
//
//*************************************************************************
//*************************************************************************
//
// Simple derived types (now defined in pieTypes.h)
//
//*************************************************************************
//*************************************************************************
//
// screen surface structure
//
//*************************************************************************
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iSurface {
    pub usr: libc::c_int,
    pub flags: uint32,
    pub xcentre: libc::c_int,
    pub ycentre: libc::c_int,
    pub xpshift: libc::c_int,
    pub ypshift: libc::c_int,
    pub clip: iClip,
    pub buffer: *mut uint8,
    pub scantable: [int32; 1024],
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub size: int32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iTexAnim {
    pub nFrames: libc::c_int,
    pub playbackRate: libc::c_int,
    pub textureWidth: libc::c_int,
    pub textureHeight: libc::c_int,
}
//*************************************************************************
//
// imd structures
//
//*************************************************************************
pub type BSPPOLYID = uint16;
// only needed when generating the tree
pub type WORLDCOORD = UDWORD;
pub type ANGLE = SWORD;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OBJPOS {
    pub x: WORLDCOORD,
    pub y: WORLDCOORD,
    pub z: WORLDCOORD,
    pub pitch: ANGLE,
    pub yaw: ANGLE,
    pub roll: ANGLE,
}
/* **************************************************************************/
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
pub struct BSPTREENODE {
    pub link: [*mut BSPTREENODE; 2],
    pub Plane: PLANE,
    pub TriSameDir: BSPPOLYID,
    pub TriOppoDir: BSPPOLYID,
}
pub type PSBSPTREENODE = *mut BSPTREENODE;
// lets hope this can work as a byte ... that will limit it to 255 polygons in 1 imd
pub type VERTEXID = libc::c_int;
// Size of the entry for vertex id in the imd polygon structure (32bits on pc 16bits on PSX)
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PIELIGHTBYTES {
    pub b: UBYTE,
    pub g: UBYTE,
    pub r: UBYTE,
    pub a: UBYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union PIELIGHT {
    pub byte: PIELIGHTBYTES,
    pub argb: UDWORD,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PIEVERTEX {
    pub sx: SDWORD,
    pub sy: SDWORD,
    pub sz: SDWORD,
    pub tu: UWORD,
    pub tv: UWORD,
    pub light: PIELIGHT,
    pub specular: PIELIGHT,
}
pub type DEPTH_MODE = libc::c_uint;
pub const DEPTH_CMP_ALWAYS_WRT_OFF: DEPTH_MODE = 3;
pub const DEPTH_CMP_LEQ_WRT_OFF: DEPTH_MODE = 2;
pub const DEPTH_CMP_ALWAYS_WRT_ON: DEPTH_MODE = 1;
pub const DEPTH_CMP_LEQ_WRT_ON: DEPTH_MODE = 0;
/* **************************************************************************/
/*
 * base.h
 *
 * Definitions for the base object type.
 *
 */
/* **************************************************************************/
/* **************************************************************************/
/* **************************************************************************/
// Droids
// All Buildings
// Things like roads, trees, bridges, fires
// Comes out of guns, stupid :-)
// for the camera tracking
/* **************************************************************************/
/* The type of object */
/* ID number of the object */
/* Object's location */
/* Object's direction +ve rotation about y axis*/
/* Object's pitch +ve nose up*/
/* Object's roll +ve left up, right down */
/* screen coordinate details */
/* Which player the object belongs to */
/* Which group selection is the droid currently in? */
/* Whether the object is selected */
/* might want this elsewhere */
/* Which cluster the object is a member of */
/* Whether object is visible to specific player */
/* When an object was destroyed, if 0 still alive */
/*BOOL			    (*damage)(pointerType	*psObject, */
/*UDWORD		damage, */
/* UDWORD		weaponClass); - Damage function */
/*UDWORD				emissionInterval;	how frequently does it puff out damage smoke?*/
/* when did it last puff out smoke? */
/* Data for fire damage */
/* TRUE if the object is in a fire */
/* When the object entered the fire */
/* How much damage has been done since the object */
/* entered the fire */
/* Pointer to next object in list */
/* Pointer to previois object in list */
/* **************************************************************************/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _base_object {
    pub type_0: OBJECT_TYPE,
    pub id: UDWORD,
    pub x: UWORD,
    pub y: UWORD,
    pub z: UWORD,
    pub direction: UWORD,
    pub pitch: SWORD,
    pub roll: SWORD,
    pub psNext: *mut _base_object,
    pub sDisplay: SCREEN_DISP_DATA,
    pub player: UBYTE,
    pub group: UBYTE,
    pub selected: UBYTE,
    pub cluster: UBYTE,
    pub visible: [UBYTE; 8],
    pub died: UDWORD,
    pub lastEmission: UDWORD,
    pub inFire: BOOL,
    pub burnStart: UDWORD,
    pub burnDamage: UDWORD,
}
/*
 * DisplayDef.h
 *
 * Definitions of the display structures
 *
 */
// ffs am
//#define BOUNDARY_X		(DISP_WIDTH/20)	   // proportional to resolution - Alex M
//#define	BOUNDARY_Y		(DISP_WIDTH/16)
//#define	BOUNDARY_X		(24)
//#define	BOUNDARY_Y		(24)
pub type SCREEN_DISP_DATA = _screen_disp_data;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _screen_disp_data {
    pub imd: *mut iIMDShape,
    pub frameNumber: UDWORD,
    pub screenX: UDWORD,
    pub screenY: UDWORD,
    pub screenR: UDWORD,
}
pub type OBJECT_TYPE = _object_type;
pub type _object_type = libc::c_uint;
pub const OBJ_TARGET: _object_type = 4;
pub const OBJ_BULLET: _object_type = 3;
pub const OBJ_FEATURE: _object_type = 2;
pub const OBJ_STRUCTURE: _object_type = 1;
pub const OBJ_DROID: _object_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _base_stats {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
}
pub type _component_type = libc::c_uint;
pub const COMP_NUMCOMPONENTS: _component_type = 9;
pub const COMP_WEAPON: _component_type = 8;
pub const COMP_CONSTRUCT: _component_type = 7;
pub const COMP_SENSOR: _component_type = 6;
pub const COMP_ECM: _component_type = 5;
pub const COMP_REPAIRUNIT: _component_type = 4;
pub const COMP_PROPULSION: _component_type = 3;
pub const COMP_BRAIN: _component_type = 2;
pub const COMP_BODY: _component_type = 1;
pub const COMP_UNKNOWN: _component_type = 0;
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
pub type SENSOR_STATS = _sensor_stats;
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
//UDWORD		dummy;
//	BOOL		drawnThisFrame;		// for sorting - have we drawn the imd already?
// last frame it was drawn
//	UDWORD		animFrame;			// anim Frame
//	SDWORD		bucketDepth;
//	BOOL		onScreen;
//	UDWORD		numTiles;
//no longer used 7/8/98
/*typedef struct _program_stats
{
	// Common stats - This structure doesn't actually need all the stats
	COMPONENT_STATS;		
	UDWORD		slots;				// How many brain slots the program takes
	UDWORD		order;				// The order activated by the program if any
	UDWORD		special;			// The special ability that the droid can perform
									// with this program
} PROGRAM_STATS;*/
/*these are defined in Access database - if you change them in there,
  then change them here! (and the rest of the code)
  They are made up values for now - defined when Jim does it!*/
/*typedef enum _program_orders
{
	ORDER_STOP,
	ORDER_SCAVANGE,
	ORDER_ATTACK,
	ORDER_GUARD,
	ORDER_AID,
	ORDER_BUILD,
	ORDER_DEMOLISH,
	ORDER_REPAIR,
} PROGRAM_ORDERS;*/
//no capability - droid must stop
//partial capability - droid has 50% chance to hit
//full capability - droid fires normally on move
pub type WEAPON_STATS = _weapon_stats;
/* Common stats */
// Max distance to target for short range shot
// Max distance to target for long range shot
// Min distance to target for shot
//Use Movement Model now for projectile type - AB 15/6/98
	//BOOL			direct;				// Whether the weapon fires directly or indirectly
// Chance to hit at short range
// Chance to hit at long range
// Time between each weapon fire
// The number of explosions per shot
// The number of rounds per salvo
// Time to reload the round of ammo (salvo fire)
// How much damage the weapon causes
// Basic blast radius of weapon
// Chance to hit in the blast radius
// Damage done in the blast radius
// How long the round burns
// Damage done each burn cycle
// Burn radius of the round
// speed ammo travels at
// how high the ammo travels for indirect fire
// indicates whether the droid has to stop before firing
// the class of weapon - see enum WEAPON_CLASS
// the subclass to which the weapon belongs - see 
// enum WEAPON_SUBCLASS
// which projectile model to use for the bullet
// which type of warhead is associated with the weapon
//Use Movement Model now for projectile type - AB 15/6/98
	//BOOL			homingRound;		// flag to indicate whether homing or not
// used to compare with weight to see if recoils or not
// amount the weapon(turret) can rotate 0 = none
// max amount the turret can be elevated up			
// min amount the turret can be elevated down
// flag to make the (explosion) effect face the player when drawn
// flag to make the inflight effect face the player when drawn
// size of the effect 100 = normal, 50 = half etc
// flag to indicate whether the effect lights up the world
// indicates how good in the air - SHOOT_ON_GROUND, SHOOT_IN_AIR or both
// number of attack runs a VTOL droid can do with this weapon
/* Graphics control stats */
// How long a direct fire weapon is visible
// Measured in 1/100 sec.
// How long a blast radius is visible
/* Graphics used for the weapon */
// The turret mount to use
// The muzzle flash 
// The ammo in flight
// The ammo hitting a target
// The ammo missing a target
// The ammo hitting water
// The trail used for in flight
/* Audio */
/*Common stats for all Structure Functions*/
/* Unique ID of the item */
/* Text name of the component */
/* The type of Function */
/*Common struct for all functions*/
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
/*
 * MoveDef.h
 *
 * Structure definitions for movement structures.
 *
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _path_point {
    pub x: UBYTE,
    pub y: UBYTE,
}
pub type PATH_POINT = _path_point;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _move_control {
    pub Status: UBYTE,
    pub Mask: UBYTE,
    pub Position: UBYTE,
    pub numPoints: UBYTE,
    pub asPath: [PATH_POINT; 100],
    pub DestinationX: SDWORD,
    pub DestinationY: SDWORD,
    pub srcX: SDWORD,
    pub srcY: SDWORD,
    pub targetX: SDWORD,
    pub targetY: SDWORD,
    pub fx: FRACT,
    pub fy: FRACT,
    pub speed: FRACT,
    pub boundX: SWORD,
    pub boundY: SWORD,
    pub dir: SWORD,
    pub bumpDir: SWORD,
    pub bumpTime: UDWORD,
    pub lastBump: UWORD,
    pub pauseTime: UWORD,
    pub bumpX: UWORD,
    pub bumpY: UWORD,
    pub shuffleStart: UDWORD,
    pub psFormation: *mut _formation,
    pub iVertSpeed: SWORD,
    pub iAttackRuns: UWORD,
    pub fz: FRACT,
}
pub type MOVE_CONTROL = _move_control;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _viewdata {
    pub pName: *mut STRING,
    pub type_0: VIEW_TYPE,
    pub numText: UBYTE,
    pub ppTextMsg: *mut *mut STRING,
    pub pData: *mut libc::c_void,
}
pub type VIEW_TYPE = _view_type;
pub type _view_type = libc::c_uint;
pub const VIEW_TYPES: _view_type = 4;
pub const VIEW_RPLX: _view_type = 3;
pub const VIEW_PROX: _view_type = 2;
pub const VIEW_RPL: _view_type = 1;
pub const VIEW_RES: _view_type = 0;
/* Only needed for Alex's movement update ? */
//	UDWORD	timeStarted;
//	UDWORD	arrivalTime;
//	UDWORD	pathStarted;
//	BOOL	startedMoving;
//	UDWORD	lastTime;
//	BOOL	speedChange;
// research view
// full screen view sequence - flic
// proximity view - no view really!
// full screen view sequence - flic.	extended format
/*
 * Weapons.h
 *
 * Definitions for the weapons
 *
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _weapon {
    pub nStat: UDWORD,
    pub hitPoints: UDWORD,
    pub ammo: UDWORD,
    pub lastFired: UDWORD,
    pub recoilValue: UDWORD,
}
pub type WEAPON = _weapon;
pub type BASE_OBJECT = _base_object;
/* **************************************************************************/
/*
 * Anim.h
 *
 * Animation types and function headers
 *
 * Gareth Jones 11/7/97
 */
/* **************************************************************************/
/* **************************************************************************/
/* **************************************************************************/
/* **************************************************************************/
pub type C2RustUnnamed = libc::c_uint;
pub const ANIM_3D_TRANS: C2RustUnnamed = 2;
pub const ANIM_3D_FRAMES: C2RustUnnamed = 1;
pub const ANIM_2D: C2RustUnnamed = 0;
/* **************************************************************************/
/* ensure ANIM2D/3D structs same size */
/* width of container bitmap */
/* ensure ANIM2D/3D structs same size */
/* **************************************************************************/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ANIM_STATE {
    pub uwFrame: UWORD,
    pub vecPos: VECTOR3D,
    pub vecAngle: VECTOR3D,
    pub vecScale: VECTOR3D,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VECTOR3D {
    pub x: SDWORD,
    pub y: SDWORD,
    pub z: SDWORD,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BASEANIM {
    pub szFileName: [libc::c_char; 256],
    pub animType: libc::c_char,
    pub uwID: UWORD,
    pub uwFrameRate: UWORD,
    pub uwStates: UWORD,
    pub uwObj: UWORD,
    pub uwAnimTime: UWORD,
    pub ubType: UBYTE,
    pub psStates: *mut ANIM_STATE,
    pub psNext: *mut BASEANIM,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ANIM3D {
    pub szFileName: [libc::c_char; 256],
    pub animType: libc::c_char,
    pub uwID: UWORD,
    pub uwFrameRate: UWORD,
    pub uwStates: UWORD,
    pub uwObj: UWORD,
    pub uwAnimTime: UWORD,
    pub ubType: UBYTE,
    pub psStates: *mut ANIM_STATE,
    pub psNext: *mut BASEANIM,
    pub psFrames: *mut iIMDShape,
    pub apFrame: *mut *mut iIMDShape,
}
/* **************************************************************************/
/*
 * Animobj.h
 *
 * Animation object types and function headers
 *
 * Gareth Jones 14/11/97
 */
/* **************************************************************************/
/* **************************************************************************/
/* **************************************************************************/
/* **************************************************************************/
/* forward struct declarations */
/* **************************************************************************/
/* typedefs */
/* **************************************************************************/
/* struct member macros */
/* this must be the last entry in this structure */
/* **************************************************************************/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ANIM_OBJECT {
    pub psNext: *mut ANIM_OBJECT,
    pub uwID: UWORD,
    pub psAnim: *mut ANIM3D,
    pub psParent: *mut libc::c_void,
    pub udwStartTime: UDWORD,
    pub udwStartDelay: UDWORD,
    pub uwCycles: UWORD,
    pub bVisible: BOOL,
    pub pDoneFunc: ANIMOBJDONEFUNC,
    pub apComponents: [COMPONENT_OBJECT; 10],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct COMPONENT_OBJECT {
    pub position: VECTOR3D,
    pub orientation: VECTOR3D,
    pub psParent: *mut libc::c_void,
    pub psShape: *mut iIMDShape,
}
pub type ANIMOBJDONEFUNC
    =
    Option<unsafe extern "C" fn(_: *mut ANIM_OBJECT) -> ()>;
/*
 * DroidDef.h
 *
 * Droid structure definitions
 */
/* The number of components in the asParts / asBits arrays */
//(COMP_NUMCOMPONENTS - 2)
/* The maximum number of droid weapons and programs */
//3
//#define DROID_MAXPROGS		3
// This should really be logarithmic
//defines how many times to perform the iteration on looking for a blank location
//used to get a location next to a droid - withinh one tile
/* The different types of droid */
// NOTE, if you add to, or change this list then you'll need
// to update the DroidSelectionWeights lookup table in Display.c
pub type _droid_type = libc::c_uint;
// Any droid, Only used as a parameter for intGotoNextDroidType(droidtype).
//cyborg repair droid - new for update 7/6/99
pub const DROID_ANY: _droid_type = 13;
//cyborg repair droid - new for update 28/5/99
pub const DROID_CYBORG_SUPER: _droid_type = 12;
//cyborg constructor droid - new for update 28/5/99
pub const DROID_CYBORG_REPAIR: _droid_type = 11;
// Default droid
pub const DROID_CYBORG_CONSTRUCT: _droid_type = 10;
// Repair droid
pub const DROID_DEFAULT: _droid_type = 9;
// Command droid
pub const DROID_REPAIR: _droid_type = 8;
// guess what this is!
pub const DROID_COMMAND: _droid_type = 7;
// cyborg-type thang
pub const DROID_TRANSPORTER: _droid_type = 6;
// person
pub const DROID_CYBORG: _droid_type = 5;
// Constructor droid
pub const DROID_PERSON: _droid_type = 4;
// ECM droid
pub const DROID_CONSTRUCT: _droid_type = 3;
// Sensor droid
pub const DROID_ECM: _droid_type = 2;
// Weapon droid
pub const DROID_SENSOR: _droid_type = 1;
pub const DROID_WEAPON: _droid_type = 0;
pub type DROID_TYPE = _droid_type;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _component {
    pub nStat: UBYTE,
}
pub type COMPONENT = _component;
// Allowing a maximum of 255 stats per file
//UDWORD					hitPoints; NOT USED?
/* Possibly a set of function pointers here to be called when
	 * damage is done to a component, power is low ...
	 */
// maximum number of queued orders
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _order_list {
    pub order: SDWORD,
    pub psOrderTarget: *mut libc::c_void,
    pub x: UWORD,
    pub y: UWORD,
    pub x2: UWORD,
    pub y2: UWORD,
}
pub type ORDER_LIST = _order_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _droid {
    pub type_0: OBJECT_TYPE,
    pub id: UDWORD,
    pub x: UWORD,
    pub y: UWORD,
    pub z: UWORD,
    pub direction: UWORD,
    pub pitch: SWORD,
    pub roll: SWORD,
    pub psNext: *mut _droid,
    pub sDisplay: SCREEN_DISP_DATA,
    pub player: UBYTE,
    pub group: UBYTE,
    pub selected: UBYTE,
    pub cluster: UBYTE,
    pub visible: [UBYTE; 8],
    pub died: UDWORD,
    pub lastEmission: UDWORD,
    pub inFire: BOOL,
    pub burnStart: UDWORD,
    pub burnDamage: UDWORD,
    pub aName: [STRING; 60],
    pub droidType: DROID_TYPE,
    pub asBits: [COMPONENT; 8],
    pub weight: UDWORD,
    pub baseSpeed: UDWORD,
    pub sensorRange: UDWORD,
    pub sensorPower: UDWORD,
    pub ECMMod: UDWORD,
    pub originalBody: UDWORD,
    pub body: UDWORD,
    pub armour: [UDWORD; 2],
    pub numKills: UWORD,
    pub turretRotation: UWORD,
    pub turretPitch: UWORD,
    pub NameVersion: UBYTE,
    pub currRayAng: UBYTE,
    pub resistance: SWORD,
    pub asWeaps: [WEAPON; 1],
    pub psGroup: *mut _droid_group,
    pub psGrpNext: *mut _droid,
    pub psBaseStruct: *mut _structure,
    pub listSize: SDWORD,
    pub asOrderList: [ORDER_LIST; 10],
    pub order: SDWORD,
    pub orderX: UWORD,
    pub orderY: UWORD,
    pub orderX2: UWORD,
    pub orderY2: UWORD,
    pub lastHitWeapon: UDWORD,
    pub timeLastHit: UDWORD,
    pub bTargetted: BOOL,
    pub psTarget: *mut _base_object,
    pub psTarStats: *mut _base_stats,
    pub secondaryOrder: UDWORD,
    pub lastSync: UDWORD,
    pub action: SDWORD,
    pub actionX: UDWORD,
    pub actionY: UDWORD,
    pub psActionTarget: *mut _base_object,
    pub actionStarted: UDWORD,
    pub actionPoints: UDWORD,
    pub powerAccrued: UWORD,
    pub illumination: UBYTE,
    pub updateFlags: UBYTE,
    pub sMove: MOVE_CONTROL,
    pub psCurAnim: *mut ANIM_OBJECT,
    pub iAudioID: SDWORD,
}
//line build requires two sets of coords
//this structure is used whenever an instance of a building is required in game
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _structure {
    pub type_0: OBJECT_TYPE,
    pub id: UDWORD,
    pub x: UWORD,
    pub y: UWORD,
    pub z: UWORD,
    pub direction: UWORD,
    pub pitch: SWORD,
    pub roll: SWORD,
    pub psNext: *mut _structure,
    pub sDisplay: SCREEN_DISP_DATA,
    pub player: UBYTE,
    pub group: UBYTE,
    pub selected: UBYTE,
    pub cluster: UBYTE,
    pub visible: [UBYTE; 8],
    pub died: UDWORD,
    pub lastEmission: UDWORD,
    pub inFire: BOOL,
    pub burnStart: UDWORD,
    pub burnDamage: UDWORD,
    pub pStructureType: *mut STRUCTURE_STATS,
    pub status: UBYTE,
    pub currentBuildPts: SWORD,
    pub currentPowerAccrued: SWORD,
    pub body: UWORD,
    pub armour: UWORD,
    pub resistance: SWORD,
    pub lastResistance: UDWORD,
    pub sensorRange: UWORD,
    pub sensorPower: UWORD,
    pub turretRotation: UWORD,
    pub turretPitch: UWORD,
    pub timeLastHit: UDWORD,
    pub lastHitWeapon: UDWORD,
    pub radarX: UWORD,
    pub radarY: UWORD,
    pub ecmPower: UWORD,
    pub pFunctionality: *mut FUNCTIONALITY,
    pub targetted: UBYTE,
    pub asWeaps: [WEAPON; 1],
    pub psTarget: *mut BASE_OBJECT,
    pub psCurAnim: *mut ANIM_OBJECT,
}
//this structure is used to hold the permenant stats for each type of building
/* basic stats */
/* the type of structure */
/* technology level of the structure */
/* strength against the weapon effects */
/*The type of terrain the structure has to be 
									  built next to - may be none*/
/*The width of the base in tiles*/
/*The breadth of the base in tiles*/
/*The type of foundation for the structure*/
/*The number of build points required to build
									  the structure*/
/*The height above/below the terrain - negative
									  values denote below the terrain*/
/*The armour value for the structure - can be 
									  upgraded */
/*The structure's body points - A structure goes
									  off-line when 50% of its body points are lost*/
/*The repair system points are added to the body
									  points until fully restored . The points are 
									  then added to the Armour Points*/
/*How much power the structure requires to build*/
//NOT USED ANYMORE - AB 24/01/99
	/*UDWORD		minimumPower;		The minimum power requirement to start building
								      the structure*/
/*The number used to determine whether a 
									  structure can resist an enemy takeover - 
									  0 = cannot be attacked electrically*/
//NOT USED ANYMORE - AB 24/01/99
	/*UDWORD		quantityLimit;		The maximum number that a player can have - 
									  0 = no limit 1 = only 1 allowed etc*/
/*The larger the target, the easier to hit*/
/*The IMD to draw for this structure */
/*The base IMD to draw for this structure */
/*Which ECM is standard for the structure - 
									  if any*/
/*Which Sensor is standard for the structure - 
									  if any*/
//NOT USED ANYMORE - AB 24/01/99
	/*UDWORD		weaponSlots;		/Number of weapons that can be attached to the
									  building/
	UDWORD		numWeaps;			/Number of weapons for default /
	SDWORD		defaultWeap;		/The default weapon/
    
	struct _weapon_stats **asWeapList;		/List of pointers to default weapons/
    */
//can only have one weapon now
/*Number of functions for default*/
/*The default function*/
/*List of pointers to allowable functions - 
									  unalterable*/
//this is sizeof(FACTORY) the largest at present 11-2-99 - increased AB 22-04-99
pub type FUNCTIONALITY = [UBYTE; 40];
pub type STRUCTURE_STATS = _structure_stats;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _droid_group {
    pub type_0: SWORD,
    pub refCount: SWORD,
    pub psList: *mut DROID,
    pub psCommander: *mut DROID,
    pub sRunData: RUN_DATA,
}
//
// orderdef.h 
//
// order releated structures.
// data for barbarians retreating
pub type RUN_DATA = _run_data;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _run_data {
    pub sPos: POINT,
    pub forceLevel: UBYTE,
    pub healthLevel: UBYTE,
    pub leadership: UBYTE,
}
pub type DROID = _droid;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const NUM_DIFF_BUILDINGS: C2RustUnnamed_0 = 22;
pub const REF_SAT_UPLINK: C2RustUnnamed_0 = 21;
pub const REF_MISSILE_SILO: C2RustUnnamed_0 = 20;
pub const REF_REARM_PAD: C2RustUnnamed_0 = 19;
pub const REF_LAB: C2RustUnnamed_0 = 18;
pub const REF_VTOL_FACTORY: C2RustUnnamed_0 = 17;
pub const REF_CYBORG_FACTORY: C2RustUnnamed_0 = 16;
pub const REF_DEMOLISH: C2RustUnnamed_0 = 15;
pub const REF_BRIDGE: C2RustUnnamed_0 = 14;
pub const REF_COMMAND_CONTROL: C2RustUnnamed_0 = 13;
pub const REF_REPAIR_FACILITY: C2RustUnnamed_0 = 12;
pub const REF_RESEARCH_MODULE: C2RustUnnamed_0 = 11;
pub const REF_RESEARCH: C2RustUnnamed_0 = 10;
pub const REF_BLASTDOOR: C2RustUnnamed_0 = 9;
pub const REF_WALLCORNER: C2RustUnnamed_0 = 8;
pub const REF_WALL: C2RustUnnamed_0 = 7;
pub const REF_DEFENSE: C2RustUnnamed_0 = 6;
pub const REF_RESOURCE_EXTRACTOR: C2RustUnnamed_0 = 5;
pub const REF_POWER_MODULE: C2RustUnnamed_0 = 4;
pub const REF_POWER_GEN: C2RustUnnamed_0 = 3;
pub const REF_FACTORY_MODULE: C2RustUnnamed_0 = 2;
pub const REF_FACTORY: C2RustUnnamed_0 = 1;
pub const REF_HQ: C2RustUnnamed_0 = 0;
pub type _position_type = libc::c_uint;
pub const POS_TEMPDELIVERY: _position_type = 3;
pub const POS_PROXOBJ: _position_type = 2;
pub const POS_PROXDATA: _position_type = 1;
pub const POS_DELIVERY: _position_type = 0;
pub type POSITION_TYPE = _position_type;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _flag_position {
    pub type_0: POSITION_TYPE,
    pub frameNumber: UDWORD,
    pub screenX: UDWORD,
    pub screenY: UDWORD,
    pub screenR: UDWORD,
    pub player: UDWORD,
    pub selected: BOOL,
    pub coords: iVector,
    pub factoryInc: UBYTE,
    pub factoryType: UBYTE,
    pub psNext: *mut _flag_position,
}
pub type FLAG_POSITION = _flag_position;
pub type _struct_states = libc::c_uint;
pub const SS_BEING_DEMOLISHED: _struct_states = 2;
pub const SS_BUILT: _struct_states = 1;
pub const SS_BEING_BUILT: _struct_states = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _res_extractor {
    pub power: UDWORD,
    pub timeLastUpdated: UDWORD,
    pub active: BOOL,
    pub psPowerGen: *mut _structure,
}
pub type RES_EXTRACTOR = _res_extractor;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _power_gen {
    pub power: UDWORD,
    pub multiplier: UDWORD,
    pub capacity: UDWORD,
    pub apResExtractors: [*mut _structure; 4],
}
pub type POWER_GEN = _power_gen;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct REPAIR_FACILITY {
    pub power: UDWORD,
    pub timeStarted: UDWORD,
    pub psObj: *mut BASE_OBJECT,
    pub powerAccrued: UDWORD,
    pub psDeliveryPoint: *mut FLAG_POSITION,
    pub currentPtsAdded: UDWORD,
    pub psGroup: *mut _droid_group,
    pub psGrpNext: *mut _droid,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _rearm_pad {
    pub reArmPoints: UDWORD,
    pub timeStarted: UDWORD,
    pub psObj: *mut BASE_OBJECT,
    pub currentPtsAdded: UDWORD,
}
pub type REARM_PAD = _rearm_pad;
pub type STRUCTURE = _structure;
// position to retreat to
// number of units below which might run
// %health value below which to turn and run - FOR GROUPS ONLY
// basic chance to run
/*owning power generator*/
/*pointers to the res ext
																associated with this gen*/
/* stores the amount of body points added to the unit
                                               that is being worked on */
/* **************************************************************************/
/*
 * BulletDef.h
 *
 * Structure Definitions for the bullet object.
 *
 */
/* **************************************************************************/
/* **************************************************************************/
pub type PROJ_STATE = libc::c_uint;
pub const PROJ_POSTIMPACT: PROJ_STATE = 2;
pub const PROJ_IMPACT: PROJ_STATE = 1;
pub const PROJ_INFLIGHT: PROJ_STATE = 0;
/* **************************************************************************/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PROJ_OBJECT {
    pub type_0: OBJECT_TYPE,
    pub id: UDWORD,
    pub x: UWORD,
    pub y: UWORD,
    pub z: UWORD,
    pub direction: UWORD,
    pub pitch: SWORD,
    pub roll: SWORD,
    pub psNext: *mut PROJ_OBJECT,
    pub state: UBYTE,
    pub airTarget: UBYTE,
    pub player: UBYTE,
    pub bVisible: UBYTE,
    pub psWStats: *mut WEAPON_STATS,
    pub psSource: *mut BASE_OBJECT,
    pub psDest: *mut BASE_OBJECT,
    pub startX: UDWORD,
    pub startY: UDWORD,
    pub tarX: UDWORD,
    pub tarY: UDWORD,
    pub vXY: SDWORD,
    pub vZ: SDWORD,
    pub srcHeight: UDWORD,
    pub altChange: SDWORD,
    pub born: UDWORD,
    pub targetRadius: UDWORD,
    pub pInFlightFunc: PROJECTILE_FUNC,
}
pub type PROJECTILE_FUNC
    =
    Option<unsafe extern "C" fn(_: *mut PROJ_OBJECT) -> ()>;
/*
 * FeatureDef.h
 *
 * Structure definitions for features
 *
 */
pub type _feature_type = libc::c_uint;
pub const FEAT_SKYSCRAPER: _feature_type = 12;
pub const FEAT_TREE: _feature_type = 11;
pub const FEAT_OIL_DRUM: _feature_type = 10;
pub const FEAT_LOS_OBJ: _feature_type = 9;
pub const FEAT_DROID: _feature_type = 8;
pub const FEAT_BUILDING: _feature_type = 7;
pub const FEAT_VEHICLE: _feature_type = 6;
pub const FEAT_BOULDER: _feature_type = 5;
pub const FEAT_OIL_RESOURCE: _feature_type = 4;
pub const FEAT_GEN_ARTE: _feature_type = 3;
pub const FEAT_TANK: _feature_type = 2;
pub const FEAT_HOVER: _feature_type = 1;
pub const FEAT_BUILD_WRECK: _feature_type = 0;
pub type FEATURE_TYPE = _feature_type;
//FEAT_MESA,	no longer used
	//FEAT_MESA2,	
	//FEAT_CLIFF,
	//FEAT_STACK,
	//FEAT_BUILD_WRECK1,
	//FEAT_BUILD_WRECK2,
	//FEAT_BUILD_WRECK3,
	//FEAT_BUILD_WRECK4,
	//FEAT_BOULDER1,
	//FEAT_BOULDER2,
	//FEAT_BOULDER3,
	//FEAT_FUTCAR,
	//FEAT_FUTVAN,
/* Stats for a feature */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _feature_stats {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
    pub subType: FEATURE_TYPE,
    pub psImd: *mut iIMDShape,
    pub baseWidth: UWORD,
    pub baseBreadth: UWORD,
    pub tileDraw: BOOL,
    pub allowLOS: BOOL,
    pub visibleAtStart: BOOL,
    pub damageable: BOOL,
    pub body: UDWORD,
    pub armour: UDWORD,
}
pub type FEATURE_STATS = _feature_stats;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _feature {
    pub type_0: OBJECT_TYPE,
    pub id: UDWORD,
    pub x: UWORD,
    pub y: UWORD,
    pub z: UWORD,
    pub direction: UWORD,
    pub pitch: SWORD,
    pub roll: SWORD,
    pub psNext: *mut _feature,
    pub sDisplay: SCREEN_DISP_DATA,
    pub player: UBYTE,
    pub group: UBYTE,
    pub selected: UBYTE,
    pub cluster: UBYTE,
    pub visible: [UBYTE; 8],
    pub died: UDWORD,
    pub lastEmission: UDWORD,
    pub inFire: BOOL,
    pub burnStart: UDWORD,
    pub burnDamage: UDWORD,
    pub psStats: *mut FEATURE_STATS,
    pub startTime: UDWORD,
    pub body: UDWORD,
    pub gfxScaling: UWORD,
    pub timeLastHit: UDWORD,
    pub bTargetted: BOOL,
}
pub type FEATURE = _feature;
pub type _terrain_type = libc::c_uint;
pub const TER_MAX: _terrain_type = 12;
pub const TER_SLUSH: _terrain_type = 11;
pub const TER_SHEETICE: _terrain_type = 10;
pub const TER_RUBBLE: _terrain_type = 9;
pub const TER_CLIFFFACE: _terrain_type = 8;
pub const TER_WATER: _terrain_type = 7;
pub const TER_ROAD: _terrain_type = 6;
pub const TER_PINKROCK: _terrain_type = 5;
pub const TER_REDBRUSH: _terrain_type = 4;
pub const TER_GREENMUD: _terrain_type = 3;
pub const TER_BAKEDEARTH: _terrain_type = 2;
pub const TER_SANDYBRUSH: _terrain_type = 1;
pub const TER_SAND: _terrain_type = 0;
// Feature armour
/* Information stored with each tile */
// The name is now changed to MAPTILE to allow correct compilation on the PlayStation
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _maptile {
    pub tileInfoBits: UBYTE,
    pub tileVisBits: UBYTE,
    pub height: UBYTE,
    pub illumination: UBYTE,
    pub texture: UWORD,
    pub bMaxed: UBYTE,
    pub level: UBYTE,
    pub inRange: UBYTE,
}
pub type MAPTILE = _maptile;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _t_quad {
    pub coords: [POINT; 4],
}
pub type QUAD = _t_quad;
pub type _message_type = libc::c_uint;
pub const MSG_TYPES: _message_type = 4;
pub const MSG_PROXIMITY: _message_type = 3;
pub const MSG_MISSION: _message_type = 2;
pub const MSG_CAMPAIGN: _message_type = 1;
pub const MSG_RESEARCH: _message_type = 0;
// sensor range display.
// This is also used to store the tile flip flags
//  What's been removed - 46 bytes per tile so far
//	BASE_OBJECT		*psObject;		// Any object sitting on the location (e.g. building)
//	UBYTE			onFire;			// Is tile on fire?
//	UBYTE			rippleIndex;	// Current value in ripple table?
//	BOOL			tileVisible[MAX_PLAYERS]; // Which players can see the tile?
//	BOOL			triangleFlip;	// Is the triangle flipped?
//	TYPE_OF_TERRAIN	type;			// The terrain type for the tile
/*
 * messageDef.h
 *
 * Message structure definitions
 */
pub type MESSAGE_TYPE = _message_type;
pub type _prox_type = libc::c_uint;
pub const PROX_TYPES: _prox_type = 3;
pub const PROX_ARTEFACT: _prox_type = 2;
pub const PROX_RESOURCE: _prox_type = 1;
pub const PROX_ENEMY: _prox_type = 0;
pub type PROX_TYPE = _prox_type;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _view_proximity {
    pub x: UDWORD,
    pub y: UDWORD,
    pub z: UDWORD,
    pub proxType: PROX_TYPE,
    pub audioID: SDWORD,
}
// Research message
// Campaign message
// Mission Report messages
// Proximity message
//enemy proximity message
//resource proximity message
//artefact proximity message
// info required to view a proximity message
pub type VIEW_PROXIMITY = _view_proximity;
pub type VIEWDATA = _viewdata;
pub type MSG_VIEWDATA = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _message {
    pub type_0: MESSAGE_TYPE,
    pub id: UDWORD,
    pub pViewData: *mut MSG_VIEWDATA,
    pub read: BOOL,
    pub player: UDWORD,
    pub psNext: *mut _message,
}
//world coords for position of Proximity message
/*ID of the audio track to play - if any */
//name ID of the message - used for loading in and identifying
//the type of view
//the number of textmessages associated with this data
//Pointer to text messages - if any
/*the data required to view - either a
							  VIEW_RESEARCH, VIEW_PROXIMITY or VIEW_REPLAY*/
//base structure for each message
pub type MESSAGE = _message;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _proximity_display {
    pub type_0: POSITION_TYPE,
    pub frameNumber: UDWORD,
    pub screenX: UDWORD,
    pub screenY: UDWORD,
    pub screenR: UDWORD,
    pub player: UDWORD,
    pub selected: BOOL,
    pub psMessage: *mut MESSAGE,
    pub radarX: UDWORD,
    pub radarY: UDWORD,
    pub timeLastDrawn: UDWORD,
    pub strobe: UDWORD,
    pub buttonID: UDWORD,
    pub psNext: *mut _proximity_display,
}
//The type of message
//ID number of the message
//VIEWDATA		*pViewData;				//Pointer to view data - if any - should be some!
//Pointer to view data - if any - should be some!
//flag to indicate whether message has been read
//which player this message belongs to
//pointer to the next in the list
//used to display the proximity messages
pub type PROXIMITY_DISPLAY = _proximity_display;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const MI_TOO_MANY: C2RustUnnamed_1 = 44;
pub const MI_FIREWORK: C2RustUnnamed_1 = 43;
pub const MI_DEBRIS4: C2RustUnnamed_1 = 42;
pub const MI_DEBRIS3: C2RustUnnamed_1 = 41;
pub const MI_DEBRIS2: C2RustUnnamed_1 = 40;
pub const MI_DEBRIS1: C2RustUnnamed_1 = 39;
pub const MI_DEBRIS0: C2RustUnnamed_1 = 38;
pub const MI_WRECK4: C2RustUnnamed_1 = 37;
pub const MI_WRECK3: C2RustUnnamed_1 = 36;
pub const MI_WRECK2: C2RustUnnamed_1 = 35;
pub const MI_WRECK1: C2RustUnnamed_1 = 34;
pub const MI_WRECK0: C2RustUnnamed_1 = 33;
pub const MI_BLIP_ARTEFACT: C2RustUnnamed_1 = 32;
pub const MI_BLIP_RESOURCE: C2RustUnnamed_1 = 31;
pub const MI_BLIP_ENEMY: C2RustUnnamed_1 = 30;
pub const MI_SHOCK: C2RustUnnamed_1 = 29;
pub const MI_LANDING: C2RustUnnamed_1 = 28;
pub const MI_KICK: C2RustUnnamed_1 = 27;
pub const MI_SPLASH: C2RustUnnamed_1 = 26;
pub const MI_SNOW: C2RustUnnamed_1 = 25;
pub const MI_RAIN: C2RustUnnamed_1 = 24;
pub const MI_MFLARE: C2RustUnnamed_1 = 23;
pub const MI_TESLA: C2RustUnnamed_1 = 22;
pub const MI_FLAME: C2RustUnnamed_1 = 21;
pub const MI_TRAIL: C2RustUnnamed_1 = 20;
pub const MI_BLOOD: C2RustUnnamed_1 = 19;
pub const MI_TRANSPORTER_SHADOW: C2RustUnnamed_1 = 18;
pub const MI_SHADOW: C2RustUnnamed_1 = 17;
pub const MI_BLIP: C2RustUnnamed_1 = 16;
pub const MI_PLASMA: C2RustUnnamed_1 = 15;
pub const MI_SMALL_STEAM: C2RustUnnamed_1 = 14;
pub const MI_DROID_DAMAGE: C2RustUnnamed_1 = 13;
pub const MI_WATER: C2RustUnnamed_1 = 12;
pub const MI_CYBORG_BODY: C2RustUnnamed_1 = 11;
pub const MI_CYBORG_ARM: C2RustUnnamed_1 = 10;
pub const MI_CYBORG_LEGS: C2RustUnnamed_1 = 9;
pub const MI_CYBORG_HEAD: C2RustUnnamed_1 = 8;
pub const MI_BABA_BODY: C2RustUnnamed_1 = 7;
pub const MI_BABA_ARM: C2RustUnnamed_1 = 6;
pub const MI_BABA_LEGS: C2RustUnnamed_1 = 5;
pub const MI_BABA_HEAD: C2RustUnnamed_1 = 4;
pub const MI_SMALL_SMOKE: C2RustUnnamed_1 = 3;
pub const MI_CONSTRUCTION: C2RustUnnamed_1 = 2;
pub const MI_EXPLOSION_MEDIUM: C2RustUnnamed_1 = 1;
pub const MI_EXPLOSION_SMALL: C2RustUnnamed_1 = 0;
pub type EFFECT_GROUP = libc::c_uint;
pub const EFFECT_FIREWORK: EFFECT_GROUP = 11;
pub const EFFECT_FIRE: EFFECT_GROUP = 10;
pub const EFFECT_DUST_BALL: EFFECT_GROUP = 9;
pub const EFFECT_SAT_LASER: EFFECT_GROUP = 8;
pub const EFFECT_DESTRUCTION: EFFECT_GROUP = 7;
pub const EFFECT_BLOOD: EFFECT_GROUP = 6;
pub const EFFECT_WAYPOINT: EFFECT_GROUP = 5;
pub const EFFECT_GRAVITON: EFFECT_GROUP = 4;
pub const EFFECT_STRUCTURE: EFFECT_GROUP = 3;
pub const EFFECT_SMOKE: EFFECT_GROUP = 2;
pub const EFFECT_CONSTRUCTION: EFFECT_GROUP = 1;
pub const EFFECT_EXPLOSION: EFFECT_GROUP = 0;
pub type EFFECT_TYPE = libc::c_uint;
pub const FIREWORK_TYPE_LAUNCHER: EFFECT_TYPE = 42;
pub const FIREWORK_TYPE_STARBURST: EFFECT_TYPE = 41;
pub const WAYPOINT_TYPE: EFFECT_TYPE = 40;
pub const SAT_LASER_STANDARD: EFFECT_TYPE = 39;
pub const DESTRUCTION_TYPE_SKYSCRAPER: EFFECT_TYPE = 38;
pub const DESTRUCTION_TYPE_FEATURE: EFFECT_TYPE = 37;
pub const DESTRUCTION_TYPE_WALL_SECTION: EFFECT_TYPE = 36;
pub const DESTRUCTION_TYPE_POWER_STATION: EFFECT_TYPE = 35;
pub const DESTRUCTION_TYPE_STRUCTURE: EFFECT_TYPE = 34;
pub const DESTRUCTION_TYPE_DROID: EFFECT_TYPE = 33;
pub const DUST_TYPE_NORMAL: EFFECT_TYPE = 32;
pub const BLOOD_TYPE_NORMAL: EFFECT_TYPE = 31;
pub const CONSTRUCTION_TYPE_DRIFTING: EFFECT_TYPE = 30;
pub const FIRE_TYPE_SMOKY_BLUE: EFFECT_TYPE = 29;
pub const FIRE_TYPE_SMOKY: EFFECT_TYPE = 28;
pub const FIRE_TYPE_LOCALISED: EFFECT_TYPE = 27;
pub const SMOKE_TYPE_TRAIL: EFFECT_TYPE = 26;
pub const SMOKE_TYPE_STEAM: EFFECT_TYPE = 25;
pub const SMOKE_TYPE_BILLOW: EFFECT_TYPE = 24;
pub const SMOKE_TYPE_DRIFTING_SMALL: EFFECT_TYPE = 23;
pub const SMOKE_TYPE_DRIFTING_HIGH: EFFECT_TYPE = 22;
pub const SMOKE_TYPE_DRIFTING: EFFECT_TYPE = 21;
pub const GRAVITON_TYPE_GIBLET: EFFECT_TYPE = 20;
pub const GRAVITON_TYPE_EMITTING_ST: EFFECT_TYPE = 19;
pub const GRAVITON_TYPE_EMITTING_DR: EFFECT_TYPE = 18;
pub const GRAVITON_TYPE_STANDARD: EFFECT_TYPE = 17;
pub const EXPLOSION_TYPE_SHOCKWAVE: EFFECT_TYPE = 16;
pub const EXPLOSION_TYPE_LAND_LIGHT: EFFECT_TYPE = 15;
pub const EXPLOSION_TYPE_KICKUP: EFFECT_TYPE = 14;
pub const EXPLOSION_TYPE_PLASMA: EFFECT_TYPE = 13;
pub const EXPLOSION_TYPE_FLARE: EFFECT_TYPE = 12;
pub const EXPLOSION_TYPE_DISCOVERY: EFFECT_TYPE = 11;
pub const EXPLOSION_TYPE_TESLA: EFFECT_TYPE = 10;
pub const EXPLOSION_TYPE_LASER: EFFECT_TYPE = 9;
pub const EXPLOSION_TYPE_FLAMETHROWER: EFFECT_TYPE = 8;
pub const EXPLOSION_TYPE_SPECIFIED_FIXME: EFFECT_TYPE = 7;
pub const EXPLOSION_TYPE_SPECIFIED_SOLID: EFFECT_TYPE = 6;
pub const EXPLOSION_TYPE_NOT_FACING: EFFECT_TYPE = 5;
pub const EXPLOSION_TYPE_SPECIFIED: EFFECT_TYPE = 4;
pub const EXPLOSION_TYPE_LARGE: EFFECT_TYPE = 3;
pub const EXPLOSION_TYPE_MEDIUM: EFFECT_TYPE = 2;
pub const EXPLOSION_TYPE_VERY_SMALL: EFFECT_TYPE = 1;
pub const EXPLOSION_TYPE_SMALL: EFFECT_TYPE = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _highlight {
    pub xTL: UWORD,
    pub yTL: UWORD,
    pub xBR: UWORD,
    pub yBR: UWORD,
}
pub type HIGHLIGHT = _highlight;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dragBox {
    pub x1: UDWORD,
    pub y1: UDWORD,
    pub x2: UDWORD,
    pub y2: UDWORD,
    pub status: UDWORD,
    pub lastTime: UDWORD,
    pub boxColourIndex: UDWORD,
}
//message associated with this 'button'
//Used to store the radar coords - if to be drawn
//stores the time the 'button' was last drawn for animation
//id of image last used
//id of the button for the interface
//pointer to the next in the list
// Top left of box to highlight
// Bottom right of box to highlight
/* **********************************************/
/* Image ID definition file created by Framer. */
/* **********************************************/
pub type C2RustUnnamed_2 = libc::c_uint;
pub const IMAGE_ASCII64: C2RustUnnamed_2 = 483;
pub const IMAGE_ORD_FIREDES_DOWN: C2RustUnnamed_2 = 482;
pub const IMAGE_ORD_FIREDES_UP: C2RustUnnamed_2 = 481;
pub const IMAGE_TRANSETA_DOWN: C2RustUnnamed_2 = 480;
pub const IMAGE_LAUNCHUP: C2RustUnnamed_2 = 479;
pub const IMAGE_BLUE6: C2RustUnnamed_2 = 478;
pub const IMAGE_BLUE5: C2RustUnnamed_2 = 477;
pub const IMAGE_BLUE4: C2RustUnnamed_2 = 476;
pub const IMAGE_BLUE3: C2RustUnnamed_2 = 475;
pub const IMAGE_BLUE2: C2RustUnnamed_2 = 474;
pub const IMAGE_BLUE1: C2RustUnnamed_2 = 473;
pub const IMAGE_DES_ARMOUR_KINETIC: C2RustUnnamed_2 = 472;
pub const IMAGE_TABHILIGHT_SM: C2RustUnnamed_2 = 471;
pub const IMAGE_TAB1DOWN_SM: C2RustUnnamed_2 = 470;
pub const IMAGE_TAB1SELECTED_SM: C2RustUnnamed_2 = 469;
pub const IMAGE_TAB1_SM: C2RustUnnamed_2 = 468;
pub const IMAGE_TARGET5: C2RustUnnamed_2 = 467;
pub const IMAGE_TARGET4: C2RustUnnamed_2 = 466;
pub const IMAGE_TARGET3: C2RustUnnamed_2 = 465;
pub const IMAGE_TARGET2: C2RustUnnamed_2 = 464;
pub const IMAGE_TARGET1: C2RustUnnamed_2 = 463;
pub const IMAGE_MISSION_CLOCK_UP: C2RustUnnamed_2 = 462;
pub const IMAGE_RES_GRPDAM: C2RustUnnamed_2 = 461;
pub const IMAGE_RES_GRPACC: C2RustUnnamed_2 = 460;
pub const IMAGE_RES_GRPROF: C2RustUnnamed_2 = 459;
pub const IMAGE_RES_GRPREP: C2RustUnnamed_2 = 458;
pub const IMAGE_RES_GRPUPG: C2RustUnnamed_2 = 457;
pub const IMAGE_ASCII131: C2RustUnnamed_2 = 456;
pub const IMAGE_ASCII161: C2RustUnnamed_2 = 455;
pub const IMAGE_ASCII191: C2RustUnnamed_2 = 454;
pub const IMAGE_ASCII208: C2RustUnnamed_2 = 453;
pub const IMAGE_ASCII207: C2RustUnnamed_2 = 452;
pub const IMAGE_ASCII206: C2RustUnnamed_2 = 451;
pub const IMAGE_ASCII205: C2RustUnnamed_2 = 450;
pub const IMAGE_ASCII204: C2RustUnnamed_2 = 449;
pub const IMAGE_ASCII203: C2RustUnnamed_2 = 448;
pub const IMAGE_ASCII202: C2RustUnnamed_2 = 447;
pub const IMAGE_ASCII201: C2RustUnnamed_2 = 446;
pub const IMAGE_ASCII200: C2RustUnnamed_2 = 445;
pub const IMAGE_ASCII198: C2RustUnnamed_2 = 444;
pub const IMAGE_ASCII197: C2RustUnnamed_2 = 443;
pub const IMAGE_ASCII196: C2RustUnnamed_2 = 442;
pub const IMAGE_ASCII195: C2RustUnnamed_2 = 441;
pub const IMAGE_ASCII194: C2RustUnnamed_2 = 440;
pub const IMAGE_ASCII193: C2RustUnnamed_2 = 439;
pub const IMAGE_ASCII192: C2RustUnnamed_2 = 438;
pub const IMAGE_ASCII210: C2RustUnnamed_2 = 437;
pub const IMAGE_ASCII211: C2RustUnnamed_2 = 436;
pub const IMAGE_ASCII214: C2RustUnnamed_2 = 435;
pub const IMAGE_ASCII213: C2RustUnnamed_2 = 434;
pub const IMAGE_ASCII212: C2RustUnnamed_2 = 433;
pub const IMAGE_ASCII216: C2RustUnnamed_2 = 432;
pub const IMAGE_ASCII220: C2RustUnnamed_2 = 431;
pub const IMAGE_ASCII217: C2RustUnnamed_2 = 430;
pub const IMAGE_ASCII219: C2RustUnnamed_2 = 429;
pub const IMAGE_ASCII218: C2RustUnnamed_2 = 428;
pub const IMAGE_ASCII221: C2RustUnnamed_2 = 427;
pub const IMAGE_ASCII223: C2RustUnnamed_2 = 426;
pub const IMAGE_ASCII248: C2RustUnnamed_2 = 425;
pub const IMAGE_ASCII241: C2RustUnnamed_2 = 424;
pub const IMAGE_ASCII253: C2RustUnnamed_2 = 423;
pub const IMAGE_ASCII252: C2RustUnnamed_2 = 422;
pub const IMAGE_ASCII251: C2RustUnnamed_2 = 421;
pub const IMAGE_ASCII250: C2RustUnnamed_2 = 420;
pub const IMAGE_ASCII249: C2RustUnnamed_2 = 419;
pub const IMAGE_ASCII246: C2RustUnnamed_2 = 418;
pub const IMAGE_ASCII245: C2RustUnnamed_2 = 417;
pub const IMAGE_ASCII244: C2RustUnnamed_2 = 416;
pub const IMAGE_ASCII243: C2RustUnnamed_2 = 415;
pub const IMAGE_ASCII242: C2RustUnnamed_2 = 414;
pub const IMAGE_ASCII239: C2RustUnnamed_2 = 413;
pub const IMAGE_ASCII238: C2RustUnnamed_2 = 412;
pub const IMAGE_ASCII237: C2RustUnnamed_2 = 411;
pub const IMAGE_ASCII236: C2RustUnnamed_2 = 410;
pub const IMAGE_ASCII235: C2RustUnnamed_2 = 409;
pub const IMAGE_ASCII234: C2RustUnnamed_2 = 408;
pub const IMAGE_ASCII233: C2RustUnnamed_2 = 407;
pub const IMAGE_ASCII232: C2RustUnnamed_2 = 406;
pub const IMAGE_ASCII231: C2RustUnnamed_2 = 405;
pub const IMAGE_ASCII230: C2RustUnnamed_2 = 404;
pub const IMAGE_ASCII229: C2RustUnnamed_2 = 403;
pub const IMAGE_ASCII228: C2RustUnnamed_2 = 402;
pub const IMAGE_ASCII227: C2RustUnnamed_2 = 401;
pub const IMAGE_ASCII226: C2RustUnnamed_2 = 400;
pub const IMAGE_ASCII225: C2RustUnnamed_2 = 399;
pub const IMAGE_ASCII224: C2RustUnnamed_2 = 398;
pub const IMAGE_ASCII189: C2RustUnnamed_2 = 397;
pub const IMAGE_ASCII188: C2RustUnnamed_2 = 396;
pub const IMAGE_RES_DEFENCE: C2RustUnnamed_2 = 395;
pub const IMAGE_LEV_7: C2RustUnnamed_2 = 394;
pub const IMAGE_LEV_6: C2RustUnnamed_2 = 393;
pub const IMAGE_LEV_5: C2RustUnnamed_2 = 392;
pub const IMAGE_LEV_4: C2RustUnnamed_2 = 391;
pub const IMAGE_LEV_3: C2RustUnnamed_2 = 390;
pub const IMAGE_LEV_2: C2RustUnnamed_2 = 389;
pub const IMAGE_LEV_1: C2RustUnnamed_2 = 388;
pub const IMAGE_LEV_0: C2RustUnnamed_2 = 387;
pub const IMAGE_CDCHANGE_CANCEL: C2RustUnnamed_2 = 386;
pub const IMAGE_CDCHANGE_OK: C2RustUnnamed_2 = 385;
pub const IMAGE_MULTI_CHAN: C2RustUnnamed_2 = 384;
pub const IMAGE_MULTI_NOCHAN: C2RustUnnamed_2 = 383;
pub const IMAGE_ORD_EMBARKDOWN: C2RustUnnamed_2 = 382;
pub const IMAGE_ORD_EMBARKUP: C2RustUnnamed_2 = 381;
pub const IMAGE_ORD_RTRDOWN: C2RustUnnamed_2 = 380;
pub const IMAGE_ORD_RTRUP: C2RustUnnamed_2 = 379;
pub const IMAGE_ORD_GUARDDOWN: C2RustUnnamed_2 = 378;
pub const IMAGE_ORD_GUARDUP: C2RustUnnamed_2 = 377;
pub const IMAGE_ORD_PERSUEDOWN: C2RustUnnamed_2 = 376;
pub const IMAGE_ORD_PERSUEUP: C2RustUnnamed_2 = 375;
pub const IMAGE_ORD_PATROLDOWN: C2RustUnnamed_2 = 374;
pub const IMAGE_ORD_PATROLUP: C2RustUnnamed_2 = 373;
pub const IMAGE_GN_0: C2RustUnnamed_2 = 372;
pub const IMAGE_GN_1: C2RustUnnamed_2 = 371;
pub const IMAGE_GN_2: C2RustUnnamed_2 = 370;
pub const IMAGE_GN_3: C2RustUnnamed_2 = 369;
pub const IMAGE_GN_4: C2RustUnnamed_2 = 368;
pub const IMAGE_GN_5: C2RustUnnamed_2 = 367;
pub const IMAGE_GN_6: C2RustUnnamed_2 = 366;
pub const IMAGE_GN_7: C2RustUnnamed_2 = 365;
pub const IMAGE_GN_8: C2RustUnnamed_2 = 364;
pub const IMAGE_GN_9: C2RustUnnamed_2 = 363;
pub const IMAGE_GN_STAR: C2RustUnnamed_2 = 362;
pub const IMAGE_VDP_HI: C2RustUnnamed_2 = 361;
pub const IMAGE_VDP_UP: C2RustUnnamed_2 = 360;
pub const IMAGE_VDP_DOWN: C2RustUnnamed_2 = 359;
pub const IMAGE_FDP_HI: C2RustUnnamed_2 = 358;
pub const IMAGE_FDP_UP: C2RustUnnamed_2 = 357;
pub const IMAGE_FDP_DOWN: C2RustUnnamed_2 = 356;
pub const IMAGE_CDP_HI: C2RustUnnamed_2 = 355;
pub const IMAGE_CDP_UP: C2RustUnnamed_2 = 354;
pub const IMAGE_CDP_DOWN: C2RustUnnamed_2 = 353;
pub const IMAGE_LOOP_HI: C2RustUnnamed_2 = 352;
pub const IMAGE_LOOP_DOWN: C2RustUnnamed_2 = 351;
pub const IMAGE_LOOP_UP: C2RustUnnamed_2 = 350;
pub const IMAGE_RES_CYBORGTECH: C2RustUnnamed_2 = 349;
pub const IMAGE_CURSOR_DEST: C2RustUnnamed_2 = 348;
pub const IMAGE_DES_EDITBOXRIGHTH: C2RustUnnamed_2 = 347;
pub const IMAGE_DES_EDITBOXMIDH: C2RustUnnamed_2 = 346;
pub const IMAGE_DES_EDITBOXRIGHT: C2RustUnnamed_2 = 345;
pub const IMAGE_DES_EDITBOXMID: C2RustUnnamed_2 = 344;
pub const IMAGE_DES_POWERBAR_RIGHT: C2RustUnnamed_2 = 343;
pub const IMAGE_DES_POWERBAR_LEFT: C2RustUnnamed_2 = 342;
pub const IMAGE_AUDIO_LASTSAMPLEH: C2RustUnnamed_2 = 341;
pub const IMAGE_AUDIO_LASTSAMPLE: C2RustUnnamed_2 = 340;
pub const IMAGE_DES_BINH: C2RustUnnamed_2 = 339;
pub const IMAGE_DES_BIN: C2RustUnnamed_2 = 338;
pub const IMAGE_INFINITE_HI: C2RustUnnamed_2 = 337;
pub const IMAGE_INFINITE_DOWN: C2RustUnnamed_2 = 336;
pub const IMAGE_INFINITE_UP: C2RustUnnamed_2 = 335;
pub const IMAGE_RETICULE_GREY: C2RustUnnamed_2 = 334;
pub const IMAGE_ORD_FACHILITE: C2RustUnnamed_2 = 333;
pub const IMAGE_ORD_FAC5DOWN: C2RustUnnamed_2 = 332;
pub const IMAGE_ORD_FAC5UP: C2RustUnnamed_2 = 331;
pub const IMAGE_ORD_FAC4DOWN: C2RustUnnamed_2 = 330;
pub const IMAGE_ORD_FAC4UP: C2RustUnnamed_2 = 329;
pub const IMAGE_ORD_FAC3DOWN: C2RustUnnamed_2 = 328;
pub const IMAGE_ORD_FAC3UP: C2RustUnnamed_2 = 327;
pub const IMAGE_ORD_FAC2DOWN: C2RustUnnamed_2 = 326;
pub const IMAGE_ORD_FAC2UP: C2RustUnnamed_2 = 325;
pub const IMAGE_ORD_FAC1DOWN: C2RustUnnamed_2 = 324;
pub const IMAGE_ORD_FAC1UP: C2RustUnnamed_2 = 323;
pub const IMAGE_TRANSETA_UP: C2RustUnnamed_2 = 322;
pub const IMAGE_LAUNCHDOWN: C2RustUnnamed_2 = 321;
pub const IMAGE_MISSION_CLOCK: C2RustUnnamed_2 = 320;
pub const IMAGE_NRUTER: C2RustUnnamed_2 = 319;
pub const IMAGE_CMDDROID_EXP: C2RustUnnamed_2 = 318;
pub const IMAGE_MULTI_VIS_HI: C2RustUnnamed_2 = 317;
pub const IMAGE_MULTI_POW_HI: C2RustUnnamed_2 = 316;
pub const IMAGE_MULTI_DRO_HI: C2RustUnnamed_2 = 315;
pub const IMAGE_MULTI_TEK_HI: C2RustUnnamed_2 = 314;
pub const IMAGE_MULTI_TEK: C2RustUnnamed_2 = 313;
pub const IMAGE_MULTI_DRO: C2RustUnnamed_2 = 312;
pub const IMAGE_MULTI_POW: C2RustUnnamed_2 = 311;
pub const IMAGE_MULTI_VIS: C2RustUnnamed_2 = 310;
pub const IMAGE_MULTI_AL_HI: C2RustUnnamed_2 = 309;
pub const IMAGE_MULTI_OFFAL_HI: C2RustUnnamed_2 = 308;
pub const IMAGE_MULTI_NOAL_HI: C2RustUnnamed_2 = 307;
pub const IMAGE_MULTI_AL: C2RustUnnamed_2 = 306;
pub const IMAGE_MULTI_OFFAL: C2RustUnnamed_2 = 305;
pub const IMAGE_MULTI_NOAL: C2RustUnnamed_2 = 304;
pub const IMAGE_RAD_ART3: C2RustUnnamed_2 = 303;
pub const IMAGE_RAD_ART2: C2RustUnnamed_2 = 302;
pub const IMAGE_RAD_ART1: C2RustUnnamed_2 = 301;
pub const IMAGE_RAD_ARTREAD: C2RustUnnamed_2 = 300;
pub const IMAGE_RAD_RES3: C2RustUnnamed_2 = 299;
pub const IMAGE_RAD_RES2: C2RustUnnamed_2 = 298;
pub const IMAGE_RAD_RES1: C2RustUnnamed_2 = 297;
pub const IMAGE_RAD_RESREAD: C2RustUnnamed_2 = 296;
pub const IMAGE_RAD_ENM3: C2RustUnnamed_2 = 295;
pub const IMAGE_RAD_ENM2: C2RustUnnamed_2 = 294;
pub const IMAGE_RAD_ENM1: C2RustUnnamed_2 = 293;
pub const IMAGE_RAD_ENMREAD: C2RustUnnamed_2 = 292;
pub const IMAGE_SLIDER_INFINITY: C2RustUnnamed_2 = 291;
pub const IMAGE_SLIDER_BIGBUT: C2RustUnnamed_2 = 290;
pub const IMAGE_ORD_DESTRUCT2GREY: C2RustUnnamed_2 = 289;
pub const IMAGE_ORD_DESTRUCT2DOWN: C2RustUnnamed_2 = 288;
pub const IMAGE_ORD_DESTRUCT2UP: C2RustUnnamed_2 = 287;
pub const IMAGE_ORD_GOTOHQDOWN: C2RustUnnamed_2 = 286;
pub const IMAGE_ORD_GOTOHQUP: C2RustUnnamed_2 = 285;
pub const IMAGE_ORD_HALTDOWN: C2RustUnnamed_2 = 284;
pub const IMAGE_ORD_HALTUP: C2RustUnnamed_2 = 283;
pub const IMAGE_ORD_HOLDFIREDOWN: C2RustUnnamed_2 = 282;
pub const IMAGE_ORD_HOLDFIREUP: C2RustUnnamed_2 = 281;
pub const IMAGE_ORD_RETFIREDOWN: C2RustUnnamed_2 = 280;
pub const IMAGE_ORD_RETFIREUP: C2RustUnnamed_2 = 279;
pub const IMAGE_ORD_FATWILLDOWN: C2RustUnnamed_2 = 278;
pub const IMAGE_ORD_FATWILLUP: C2RustUnnamed_2 = 277;
pub const IMAGE_ORD_REPAIR3DOWN: C2RustUnnamed_2 = 276;
pub const IMAGE_ORD_REPAIR3UP: C2RustUnnamed_2 = 275;
pub const IMAGE_ORD_REPAIR1DOWN: C2RustUnnamed_2 = 274;
pub const IMAGE_ORD_REPAIR1UP: C2RustUnnamed_2 = 273;
pub const IMAGE_ORD_REPAIR2DOWN: C2RustUnnamed_2 = 272;
pub const IMAGE_ORD_REPAIR2UP: C2RustUnnamed_2 = 271;
pub const IMAGE_ORD_RANGE3DOWN: C2RustUnnamed_2 = 270;
pub const IMAGE_ORD_RANGE3UP: C2RustUnnamed_2 = 269;
pub const IMAGE_ORD_RANGE2DOWN: C2RustUnnamed_2 = 268;
pub const IMAGE_ORD_RANGE2UP: C2RustUnnamed_2 = 267;
pub const IMAGE_ORD_RANGE1DOWN: C2RustUnnamed_2 = 266;
pub const IMAGE_ORD_RANGE1UP: C2RustUnnamed_2 = 265;
pub const IMAGE_ORD_DESTRUCT1DOWN: C2RustUnnamed_2 = 264;
pub const IMAGE_ORD_DESTRUCT1UP: C2RustUnnamed_2 = 263;
pub const IMAGE_RES_QUESTIONMARK: C2RustUnnamed_2 = 262;
pub const IMAGE_RES_STRUCTURETECH: C2RustUnnamed_2 = 261;
pub const IMAGE_RES_SYSTEMTECH: C2RustUnnamed_2 = 260;
pub const IMAGE_RES_POWERTECH: C2RustUnnamed_2 = 259;
pub const IMAGE_RES_COMPUTERTECH: C2RustUnnamed_2 = 258;
pub const IMAGE_RES_WEAPONTECH: C2RustUnnamed_2 = 257;
pub const IMAGE_RES_DROIDTECH: C2RustUnnamed_2 = 256;
pub const IMAGE_QUESTION_MARK: C2RustUnnamed_2 = 255;
pub const IMAGE_RES_MAJOR_ROCKET: C2RustUnnamed_2 = 254;
pub const IMAGE_RES_MINOR_AUTOWEAPONS: C2RustUnnamed_2 = 253;
pub const IMAGE_TRACKS: C2RustUnnamed_2 = 252;
pub const IMAGE_STAR: C2RustUnnamed_2 = 251;
pub const IMAGE_SLIDER_BIG: C2RustUnnamed_2 = 250;
pub const IMAGE_DES_STATBACKMID: C2RustUnnamed_2 = 249;
pub const IMAGE_DES_STATBACKRIGHT: C2RustUnnamed_2 = 248;
pub const IMAGE_DES_STATBACKLEFT: C2RustUnnamed_2 = 247;
pub const IMAGE_SIDETABSEL: C2RustUnnamed_2 = 246;
pub const IMAGE_SIDETABDOWN: C2RustUnnamed_2 = 245;
pub const IMAGE_SIDETABHI: C2RustUnnamed_2 = 244;
pub const IMAGE_SIDETAB: C2RustUnnamed_2 = 243;
pub const IMAGE_CURSOR_NOTPOS: C2RustUnnamed_2 = 242;
pub const IMAGE_CURSOR_PICKUP: C2RustUnnamed_2 = 241;
pub const IMAGE_CURSOR_REPAIR: C2RustUnnamed_2 = 240;
pub const IMAGE_CURSOR_FIX: C2RustUnnamed_2 = 239;
pub const IMAGE_CURSOR_ATTACH: C2RustUnnamed_2 = 238;
pub const IMAGE_CURSOR_BRIDGE: C2RustUnnamed_2 = 237;
pub const IMAGE_CURSOR_EMBARK: C2RustUnnamed_2 = 236;
pub const IMAGE_CURSOR_GUARD: C2RustUnnamed_2 = 235;
pub const IMAGE_CURSOR_MOVE: C2RustUnnamed_2 = 234;
pub const IMAGE_CURSOR_BUILD: C2RustUnnamed_2 = 233;
pub const IMAGE_CURSOR_DEFAULT: C2RustUnnamed_2 = 232;
pub const IMAGE_CURSOR_ECM: C2RustUnnamed_2 = 231;
pub const IMAGE_CURSOR_LOCKON: C2RustUnnamed_2 = 230;
pub const IMAGE_CURSOR_SELECT: C2RustUnnamed_2 = 229;
pub const IMAGE_CURSOR_ATTACK: C2RustUnnamed_2 = 228;
pub const IMAGE_ASCII126: C2RustUnnamed_2 = 227;
pub const IMAGE_ASCII125: C2RustUnnamed_2 = 226;
pub const IMAGE_ASCII124: C2RustUnnamed_2 = 225;
pub const IMAGE_ASCII123: C2RustUnnamed_2 = 224;
pub const IMAGE_ASCII122: C2RustUnnamed_2 = 223;
pub const IMAGE_ASCII121: C2RustUnnamed_2 = 222;
pub const IMAGE_ASCII120: C2RustUnnamed_2 = 221;
pub const IMAGE_ASCII119: C2RustUnnamed_2 = 220;
pub const IMAGE_ASCII118: C2RustUnnamed_2 = 219;
pub const IMAGE_ASCII117: C2RustUnnamed_2 = 218;
pub const IMAGE_ASCII116: C2RustUnnamed_2 = 217;
pub const IMAGE_ASCII115: C2RustUnnamed_2 = 216;
pub const IMAGE_ASCII114: C2RustUnnamed_2 = 215;
pub const IMAGE_ASCII113: C2RustUnnamed_2 = 214;
pub const IMAGE_ASCII112: C2RustUnnamed_2 = 213;
pub const IMAGE_ASCII111: C2RustUnnamed_2 = 212;
pub const IMAGE_ASCII110: C2RustUnnamed_2 = 211;
pub const IMAGE_ASCII109: C2RustUnnamed_2 = 210;
pub const IMAGE_ASCII108: C2RustUnnamed_2 = 209;
pub const IMAGE_ASCII107: C2RustUnnamed_2 = 208;
pub const IMAGE_ASCII106: C2RustUnnamed_2 = 207;
pub const IMAGE_ASCII105: C2RustUnnamed_2 = 206;
pub const IMAGE_ASCII104: C2RustUnnamed_2 = 205;
pub const IMAGE_ASCII103: C2RustUnnamed_2 = 204;
pub const IMAGE_ASCII102: C2RustUnnamed_2 = 203;
pub const IMAGE_ASCII101: C2RustUnnamed_2 = 202;
pub const IMAGE_ASCII100: C2RustUnnamed_2 = 201;
pub const IMAGE_ASCII99: C2RustUnnamed_2 = 200;
pub const IMAGE_ASCII98: C2RustUnnamed_2 = 199;
pub const IMAGE_ASCII97: C2RustUnnamed_2 = 198;
pub const IMAGE_ASCII96: C2RustUnnamed_2 = 197;
pub const IMAGE_ASCII95: C2RustUnnamed_2 = 196;
pub const IMAGE_ASCII94: C2RustUnnamed_2 = 195;
pub const IMAGE_ASCII93: C2RustUnnamed_2 = 194;
pub const IMAGE_ASCII92: C2RustUnnamed_2 = 193;
pub const IMAGE_ASCII91: C2RustUnnamed_2 = 192;
pub const IMAGE_ASCII90: C2RustUnnamed_2 = 191;
pub const IMAGE_ASCII89: C2RustUnnamed_2 = 190;
pub const IMAGE_ASCII88: C2RustUnnamed_2 = 189;
pub const IMAGE_ASCII87: C2RustUnnamed_2 = 188;
pub const IMAGE_ASCII86: C2RustUnnamed_2 = 187;
pub const IMAGE_ASCII85: C2RustUnnamed_2 = 186;
pub const IMAGE_ASCII84: C2RustUnnamed_2 = 185;
pub const IMAGE_ASCII83: C2RustUnnamed_2 = 184;
pub const IMAGE_ASCII82: C2RustUnnamed_2 = 183;
pub const IMAGE_ASCII81: C2RustUnnamed_2 = 182;
pub const IMAGE_ASCII80: C2RustUnnamed_2 = 181;
pub const IMAGE_ASCII79: C2RustUnnamed_2 = 180;
pub const IMAGE_ASCII78: C2RustUnnamed_2 = 179;
pub const IMAGE_ASCII77: C2RustUnnamed_2 = 178;
pub const IMAGE_ASCII76: C2RustUnnamed_2 = 177;
pub const IMAGE_ASCII75: C2RustUnnamed_2 = 176;
pub const IMAGE_ASCII74: C2RustUnnamed_2 = 175;
pub const IMAGE_ASCII73: C2RustUnnamed_2 = 174;
pub const IMAGE_ASCII72: C2RustUnnamed_2 = 173;
pub const IMAGE_ASCII71: C2RustUnnamed_2 = 172;
pub const IMAGE_ASCII70: C2RustUnnamed_2 = 171;
pub const IMAGE_ASCII69: C2RustUnnamed_2 = 170;
pub const IMAGE_ASCII68: C2RustUnnamed_2 = 169;
pub const IMAGE_ASCII67: C2RustUnnamed_2 = 168;
pub const IMAGE_ASCII66: C2RustUnnamed_2 = 167;
pub const IMAGE_ASCII65: C2RustUnnamed_2 = 166;
pub const IMAGE_ASTERISK: C2RustUnnamed_2 = 165;
pub const IMAGE_ASCII63: C2RustUnnamed_2 = 164;
pub const IMAGE_ASCII62: C2RustUnnamed_2 = 163;
pub const IMAGE_ASCII61: C2RustUnnamed_2 = 162;
pub const IMAGE_ASCII60: C2RustUnnamed_2 = 161;
pub const IMAGE_ASCII59: C2RustUnnamed_2 = 160;
pub const IMAGE_ASCII58: C2RustUnnamed_2 = 159;
pub const IMAGE_ASCII57: C2RustUnnamed_2 = 158;
pub const IMAGE_ASCII56: C2RustUnnamed_2 = 157;
pub const IMAGE_ASCII55: C2RustUnnamed_2 = 156;
pub const IMAGE_ASCII54: C2RustUnnamed_2 = 155;
pub const IMAGE_ASCII53: C2RustUnnamed_2 = 154;
pub const IMAGE_ASCII52: C2RustUnnamed_2 = 153;
pub const IMAGE_ASCII51: C2RustUnnamed_2 = 152;
pub const IMAGE_ASCII50: C2RustUnnamed_2 = 151;
pub const IMAGE_ASCII49: C2RustUnnamed_2 = 150;
pub const IMAGE_ASCII48: C2RustUnnamed_2 = 149;
pub const IMAGE_ASCII47: C2RustUnnamed_2 = 148;
pub const IMAGE_ASCII46: C2RustUnnamed_2 = 147;
pub const IMAGE_ASCII45: C2RustUnnamed_2 = 146;
pub const IMAGE_ASCII44: C2RustUnnamed_2 = 145;
pub const IMAGE_ASCII43: C2RustUnnamed_2 = 144;
pub const IMAGE_ASCII42: C2RustUnnamed_2 = 143;
pub const IMAGE_ASCII41: C2RustUnnamed_2 = 142;
pub const IMAGE_ASCII40: C2RustUnnamed_2 = 141;
pub const IMAGE_ASCII39: C2RustUnnamed_2 = 140;
pub const IMAGE_ASCII38: C2RustUnnamed_2 = 139;
pub const IMAGE_ASCII37: C2RustUnnamed_2 = 138;
pub const IMAGE_ASCII36: C2RustUnnamed_2 = 137;
pub const IMAGE_ASCII35: C2RustUnnamed_2 = 136;
pub const IMAGE_ASCII34: C2RustUnnamed_2 = 135;
pub const IMAGE_ASCII33: C2RustUnnamed_2 = 134;
pub const IMAGE_INTEL_CAMPAIGNDOWN: C2RustUnnamed_2 = 133;
pub const IMAGE_INTEL_CAMPAIGN: C2RustUnnamed_2 = 132;
pub const IMAGE_INTEL_MISSIONDOWN: C2RustUnnamed_2 = 131;
pub const IMAGE_INTEL_MISSION: C2RustUnnamed_2 = 130;
pub const IMAGE_INTEL_RESEARCHDOWN: C2RustUnnamed_2 = 129;
pub const IMAGE_INTEL_RESEARCH: C2RustUnnamed_2 = 128;
pub const IMAGE_DES_BARYELLOW: C2RustUnnamed_2 = 127;
pub const IMAGE_DES_BARRED: C2RustUnnamed_2 = 126;
pub const IMAGE_DES_BARBLUE: C2RustUnnamed_2 = 125;
pub const IMAGE_DES_BARBACK: C2RustUnnamed_2 = 124;
pub const IMAGE_9: C2RustUnnamed_2 = 123;
pub const IMAGE_8: C2RustUnnamed_2 = 122;
pub const IMAGE_7: C2RustUnnamed_2 = 121;
pub const IMAGE_6: C2RustUnnamed_2 = 120;
pub const IMAGE_5: C2RustUnnamed_2 = 119;
pub const IMAGE_4: C2RustUnnamed_2 = 118;
pub const IMAGE_3: C2RustUnnamed_2 = 117;
pub const IMAGE_2: C2RustUnnamed_2 = 116;
pub const IMAGE_1: C2RustUnnamed_2 = 115;
pub const IMAGE_0: C2RustUnnamed_2 = 114;
pub const IMAGE_RES_MAJOR_HOVER: C2RustUnnamed_2 = 113;
pub const IMAGE_RES_MAJOR_HEAVYWEP: C2RustUnnamed_2 = 112;
pub const IMAGE_RES_MAJOR_ELECTRONIC: C2RustUnnamed_2 = 111;
pub const IMAGE_RES_MAJOR_PLASCRETE: C2RustUnnamed_2 = 110;
pub const IMAGE_RES_MINOR_RADAR: C2RustUnnamed_2 = 109;
pub const IMAGE_PLASCRETE: C2RustUnnamed_2 = 108;
pub const IMAGE_ECM: C2RustUnnamed_2 = 107;
pub const IMAGE_RES_MINOR_PLASSTEEL: C2RustUnnamed_2 = 106;
pub const IMAGE_HOVERCRAFT: C2RustUnnamed_2 = 105;
pub const IMAGE_CANNON: C2RustUnnamed_2 = 104;
pub const IMAGE_ROCKET: C2RustUnnamed_2 = 103;
pub const IMAGE_DES_BODYPOINTS: C2RustUnnamed_2 = 102;
pub const IMAGE_DES_FIRERATE: C2RustUnnamed_2 = 101;
pub const IMAGE_DES_HOVER: C2RustUnnamed_2 = 100;
pub const IMAGE_DES_POWER: C2RustUnnamed_2 = 99;
pub const IMAGE_DES_DAMAGE: C2RustUnnamed_2 = 98;
pub const IMAGE_DES_WEIGHT: C2RustUnnamed_2 = 97;
pub const IMAGE_DES_ROAD: C2RustUnnamed_2 = 96;
pub const IMAGE_DES_CROSSCOUNTRY: C2RustUnnamed_2 = 95;
pub const IMAGE_DES_ARMOUR_EXPLOSIVE: C2RustUnnamed_2 = 94;
pub const IMAGE_DES_RANGE: C2RustUnnamed_2 = 93;
pub const IMAGE_DES_BUILDRATE: C2RustUnnamed_2 = 92;
pub const IMAGE_DES_TABWEAPONDOWN: C2RustUnnamed_2 = 91;
pub const IMAGE_DES_TABWEAPON: C2RustUnnamed_2 = 90;
pub const IMAGE_DES_EXTRAHI: C2RustUnnamed_2 = 89;
pub const IMAGE_DES_STATSCOMP: C2RustUnnamed_2 = 88;
pub const IMAGE_DES_BACK: C2RustUnnamed_2 = 87;
pub const IMAGE_FRAME_HC3: C2RustUnnamed_2 = 86;
pub const IMAGE_FRAME_HC2: C2RustUnnamed_2 = 85;
pub const IMAGE_FRAME_HC1: C2RustUnnamed_2 = 84;
pub const IMAGE_FRAME_HC0: C2RustUnnamed_2 = 83;
pub const IMAGE_FRAME_VC3: C2RustUnnamed_2 = 82;
pub const IMAGE_FRAME_VC2: C2RustUnnamed_2 = 81;
pub const IMAGE_FRAME_VC1: C2RustUnnamed_2 = 80;
pub const IMAGE_FRAME_VC0: C2RustUnnamed_2 = 79;
pub const IMAGE_DES_EDITBOXLEFTH: C2RustUnnamed_2 = 78;
pub const IMAGE_DES_EDITBOXLEFT: C2RustUnnamed_2 = 77;
pub const IMAGE_DES_POWERBACK: C2RustUnnamed_2 = 76;
pub const IMAGE_DES_STATSCURR: C2RustUnnamed_2 = 75;
pub const IMAGE_DES_STATSBACK: C2RustUnnamed_2 = 74;
pub const IMAGE_DES_POWERCURR: C2RustUnnamed_2 = 73;
pub const IMAGE_FRAME_HBH: C2RustUnnamed_2 = 72;
pub const IMAGE_FRAME_HTH: C2RustUnnamed_2 = 71;
pub const IMAGE_FRAME_HB2: C2RustUnnamed_2 = 70;
pub const IMAGE_FRAME_HT2: C2RustUnnamed_2 = 69;
pub const IMAGE_DES_COMMANDDOWN: C2RustUnnamed_2 = 68;
pub const IMAGE_DES_COMMAND: C2RustUnnamed_2 = 67;
pub const IMAGE_DES_SYSTEMSDOWN: C2RustUnnamed_2 = 66;
pub const IMAGE_DES_SYSTEMS: C2RustUnnamed_2 = 65;
pub const IMAGE_DES_WEAPONSDOWN: C2RustUnnamed_2 = 64;
pub const IMAGE_DES_WEAPONS: C2RustUnnamed_2 = 63;
pub const IMAGE_FRAME_VRH: C2RustUnnamed_2 = 62;
pub const IMAGE_FRAME_VLH: C2RustUnnamed_2 = 61;
pub const IMAGE_FRAME_VR2: C2RustUnnamed_2 = 60;
pub const IMAGE_FRAME_VL2: C2RustUnnamed_2 = 59;
pub const IMAGE_DES_HILIGHT: C2RustUnnamed_2 = 58;
pub const IMAGE_DES_PROPULSIONH: C2RustUnnamed_2 = 57;
pub const IMAGE_DES_PROPULSION: C2RustUnnamed_2 = 56;
pub const IMAGE_DES_BODYH: C2RustUnnamed_2 = 55;
pub const IMAGE_DES_BODY: C2RustUnnamed_2 = 54;
pub const IMAGE_DES_TURRETH: C2RustUnnamed_2 = 53;
pub const IMAGE_DES_TURRET: C2RustUnnamed_2 = 52;
pub const IMAGE_FRAME_VR: C2RustUnnamed_2 = 51;
pub const IMAGE_FRAME_VL: C2RustUnnamed_2 = 50;
pub const IMAGE_SLIDER_BACK: C2RustUnnamed_2 = 49;
pub const IMAGE_FRAME_HB: C2RustUnnamed_2 = 48;
pub const IMAGE_FRAME_HT: C2RustUnnamed_2 = 47;
pub const IMAGE_SLIDER_BUT: C2RustUnnamed_2 = 46;
pub const IMAGE_CLOSEHILIGHT: C2RustUnnamed_2 = 45;
pub const IMAGE_CLOSEDOWN: C2RustUnnamed_2 = 44;
pub const IMAGE_CLOSE: C2RustUnnamed_2 = 43;
pub const IMAGE_CANCEL_HILIGHT: C2RustUnnamed_2 = 42;
pub const IMAGE_CANCEL_DOWN: C2RustUnnamed_2 = 41;
pub const IMAGE_CANCEL_UP: C2RustUnnamed_2 = 40;
pub const IMAGE_RETICULE_BUTDOWN: C2RustUnnamed_2 = 39;
pub const IMAGE_RETICULE_HILIGHT: C2RustUnnamed_2 = 38;
pub const IMAGE_MANUFACTURE_DOWN: C2RustUnnamed_2 = 37;
pub const IMAGE_MANUFACTURE_UP: C2RustUnnamed_2 = 36;
pub const IMAGE_RESEARCH_DOWN: C2RustUnnamed_2 = 35;
pub const IMAGE_RESEARCH_UP: C2RustUnnamed_2 = 34;
pub const IMAGE_BUILD_DOWN: C2RustUnnamed_2 = 33;
pub const IMAGE_BUILD_UP: C2RustUnnamed_2 = 32;
pub const IMAGE_DESIGN_DOWN: C2RustUnnamed_2 = 31;
pub const IMAGE_DESIGN_UP: C2RustUnnamed_2 = 30;
pub const IMAGE_COMMANDDROID_DOWN: C2RustUnnamed_2 = 29;
pub const IMAGE_COMMANDDROID_UP: C2RustUnnamed_2 = 28;
pub const IMAGE_INTELMAP_DOWN: C2RustUnnamed_2 = 27;
pub const IMAGE_INTELMAP_UP: C2RustUnnamed_2 = 26;
pub const IMAGE_PBAR_BOTTOM: C2RustUnnamed_2 = 25;
pub const IMAGE_PBAR_TOP: C2RustUnnamed_2 = 24;
pub const IMAGE_FRAME_C3: C2RustUnnamed_2 = 23;
pub const IMAGE_FRAME_C2: C2RustUnnamed_2 = 22;
pub const IMAGE_FRAME_C1: C2RustUnnamed_2 = 21;
pub const IMAGE_FRAME_C0: C2RustUnnamed_2 = 20;
pub const IMAGE_TABHILIGHT: C2RustUnnamed_2 = 19;
pub const IMAGE_TABSELECTED: C2RustUnnamed_2 = 18;
pub const IMAGE_TAB4DOWN: C2RustUnnamed_2 = 17;
pub const IMAGE_TAB3DOWN: C2RustUnnamed_2 = 16;
pub const IMAGE_TAB2DOWN: C2RustUnnamed_2 = 15;
pub const IMAGE_TAB1DOWN: C2RustUnnamed_2 = 14;
pub const IMAGE_TAB4: C2RustUnnamed_2 = 13;
pub const IMAGE_TAB3: C2RustUnnamed_2 = 12;
pub const IMAGE_TAB2: C2RustUnnamed_2 = 11;
pub const IMAGE_TAB1: C2RustUnnamed_2 = 10;
pub const IMAGE_BUTB_HILITE: C2RustUnnamed_2 = 9;
pub const IMAGE_BUT_HILITE: C2RustUnnamed_2 = 8;
pub const IMAGE_BUTB0_DOWN: C2RustUnnamed_2 = 7;
pub const IMAGE_BUTB0_UP: C2RustUnnamed_2 = 6;
pub const IMAGE_BUT0_DOWN: C2RustUnnamed_2 = 5;
pub const IMAGE_BUT0_UP: C2RustUnnamed_2 = 4;
pub const IMAGE_PBAR_REQUIRED: C2RustUnnamed_2 = 3;
pub const IMAGE_PBAR_USED: C2RustUnnamed_2 = 2;
pub const IMAGE_PBAR_AVAIL: C2RustUnnamed_2 = 1;
pub const IMAGE_PBAR_EMPTY: C2RustUnnamed_2 = 0;
pub const DACTION_WAITDURINGREARM: _droid_action = 35;
pub type CONSOLE_TEXT_JUSTIFICATION = libc::c_uint;
pub const DEFAULT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 3;
pub const CENTRE_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 2;
pub const RIGHT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 1;
pub const LEFT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 0;
pub type DEF_COLOURS = _defaultColours;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _defaultColours {
    pub red: UBYTE,
    pub green: UBYTE,
    pub blue: UBYTE,
    pub yellow: UBYTE,
    pub purple: UBYTE,
    pub white: UBYTE,
    pub black: UBYTE,
    pub cyan: UBYTE,
}
pub const GT_COMMAND: _group_type = 1;
pub type DROID_ORDER = _droid_order;
pub type _droid_order = libc::c_uint;
pub const DORDER_RTR_SPECIFIED: _droid_order = 37;
pub const DORDER_LEAVEMAP: _droid_order = 36;
pub const DORDER_RECOVER: _droid_order = 35;
pub const DORDER_SCOUT_ATTACKWALL: _droid_order = 34;
pub const DORDER_MOVE_ATTACKWALL: _droid_order = 33;
pub const DORDER_REARM: _droid_order = 32;
pub const DORDER_PATROL: _droid_order = 31;
pub const DORDER_CLEARWRECK: _droid_order = 30;
pub const DORDER_RUNBURN: _droid_order = 29;
pub const DORDER_SCOUT: _droid_order = 28;
pub const DORDER_RESTORE: _droid_order = 27;
pub const DORDER_DROIDREPAIR: _droid_order = 26;
pub const DORDER_GUARD: _droid_order = 25;
pub const DORDER_TRANSPORTRETURN: _droid_order = 24;
pub const DORDER_TRANSPORTIN: _droid_order = 23;
pub const DORDER_TRANSPORTOUT: _droid_order = 22;
pub const DORDER_RECYCLE: _droid_order = 21;
pub const DORDER_BUILDMODULE: _droid_order = 20;
pub const DORDER_COMMAND: _droid_order = 19;
pub const DORDER_ATTACKTARGET: _droid_order = 18;
pub const DORDER_DISEMBARK: _droid_order = 17;
pub const DORDER_EMBARK: _droid_order = 16;
pub const DORDER_RUN: _droid_order = 15;
pub const DORDER_RTR: _droid_order = 14;
pub const DORDER_RTB: _droid_order = 13;
pub const DORDER_DESTRUCT: _droid_order = 12;
pub const DORDER_RETREAT: _droid_order = 11;
pub const DORDER_FIRESUPPORT: _droid_order = 10;
pub const DORDER_OBSERVE: _droid_order = 9;
pub const DORDER_REPAIR: _droid_order = 8;
pub const DORDER_DEMOLISH: _droid_order = 7;
pub const DORDER_LINEBUILD: _droid_order = 6;
pub const DORDER_HELPBUILD: _droid_order = 5;
pub const DORDER_BUILD: _droid_order = 4;
pub const DORDER_ATTACK: _droid_order = 3;
pub const DORDER_MOVE: _droid_order = 2;
pub const DORDER_STOP: _droid_order = 1;
pub const DORDER_NONE: _droid_order = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SVMESH {
    pub x: SDWORD,
    pub y: SDWORD,
    pub z: SDWORD,
    pub tu: UWORD,
    pub tv: UWORD,
    pub light: PIELIGHT,
    pub specular: PIELIGHT,
    pub sx: SDWORD,
    pub sy: SDWORD,
    pub sz: SDWORD,
    pub wx: SDWORD,
    pub wy: SDWORD,
    pub wz: SDWORD,
    pub water_height: SDWORD,
    pub wlight: PIELIGHT,
    pub drawInfo: UBYTE,
    pub bWater: UBYTE,
}
pub const DACTION_CLEARWRECK: _droid_action = 16;
pub const DACTION_DEMOLISH: _droid_action = 4;
pub const DACTION_RESTORE: _droid_action = 15;
pub const DACTION_REPAIR: _droid_action = 5;
pub const DACTION_BUILD: _droid_action = 2;
/* bucket3D.h */
pub type RENDER_TYPE = _render_type;
pub type _render_type = libc::c_uint;
pub const RENDER_PARTICLE: _render_type = 16;
pub const RENDER_DELIVPOINT: _render_type = 15;
pub const RENDER_MIST: _render_type = 14;
pub const RENDER_WATERTILE: _render_type = 13;
pub const RENDER_TILE: _render_type = 12;
pub const RENDER_SMOKE: _render_type = 11;
pub const RENDER_GRAVITON: _render_type = 10;
pub const RENDER_EFFECT: _render_type = 9;
pub const RENDER_EXPLOSION: _render_type = 8;
pub const RENDER_ANIMATION: _render_type = 7;
pub const RENDER_SHADOW: _render_type = 6;
pub const RENDER_PROJECTILE_TRANSPARENT: _render_type = 5;
pub const RENDER_PROJECTILE: _render_type = 4;
pub const RENDER_PROXMSG: _render_type = 3;
pub const RENDER_FEATURE: _render_type = 2;
pub const RENDER_STRUCTURE: _render_type = 1;
pub const RENDER_DROID: _render_type = 0;
pub const ID_SOUND_OIL_PUMP_2: C2RustUnnamed_3 = 294;
pub const DACTION_WAITDURINGREPAIR: _droid_action = 28;
pub type TILE_BUCKET = _tile_bucket;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _tile_bucket {
    pub i: UDWORD,
    pub j: UDWORD,
    pub depth: SDWORD,
}
pub type TILE_TEX_INFO = _tileTexInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _tileTexInfo {
    pub xOffset: UDWORD,
    pub yOffset: UDWORD,
    pub texPage: UDWORD,
}
pub type C2RustUnnamed_3 = libc::c_int;
pub const ID_MAX_SOUND: C2RustUnnamed_3 = 354;
pub const ID_SOUND_LAS_SAT_COUNTDOWN: C2RustUnnamed_3 = 353;
pub const ID_SOUND_UPLINK: C2RustUnnamed_3 = 352;
pub const ID_SOUND_PLASMA_FLAMER: C2RustUnnamed_3 = 351;
pub const ID_SOUND_LASER_HEAVY: C2RustUnnamed_3 = 350;
pub const ID_SOUND_EMP: C2RustUnnamed_3 = 349;
pub const ID_SOUND_CYBORG_HEAVY: C2RustUnnamed_3 = 348;
pub const ID_SOUND_CYBORG_GROUND: C2RustUnnamed_3 = 347;
pub const ID_SOUND_NEXUS_UNIT_NEUTRALISED: C2RustUnnamed_3 = 346;
pub const ID_SOUND_NEXUS_UNIT_ABSORBED: C2RustUnnamed_3 = 345;
pub const ID_SOUND_NEXUS_SYNAPTIC_LINK: C2RustUnnamed_3 = 344;
pub const ID_SOUND_NEXUS_STRUCTURE_NEUTRALISED: C2RustUnnamed_3 = 343;
pub const ID_SOUND_NEXUS_STRUCTURE_ABSORBED: C2RustUnnamed_3 = 342;
pub const ID_SOUND_NEXUS_RESEARCH_ABSORBED: C2RustUnnamed_3 = 341;
pub const ID_SOUND_NEXUS_PRODUCTION_COMPLETED: C2RustUnnamed_3 = 340;
pub const ID_SOUND_NEXUS_LAUGH3: C2RustUnnamed_3 = 339;
pub const ID_SOUND_NEXUS_LAUGH2: C2RustUnnamed_3 = 338;
pub const ID_SOUND_NEXUS_LAUGH1: C2RustUnnamed_3 = 337;
pub const ID_SOUND_NEXUS_DEFENCES_NEUTRALISED: C2RustUnnamed_3 = 336;
pub const ID_SOUND_NEXUS_DEFENCES_ABSORBED: C2RustUnnamed_3 = 335;
pub const ID_SOUND_INCOMING_LASER_SAT_STRIKE: C2RustUnnamed_3 = 334;
pub const ID_SOUND_LASER_SATELLITE_FIRING: C2RustUnnamed_3 = 333;
pub const ID_SOUND_TEAM_GAMMA_RESCUED: C2RustUnnamed_3 = 332;
pub const ID_SOUND_TEAM_BETA_RESCUED: C2RustUnnamed_3 = 331;
pub const ID_SOUND_TEAM_ALPHA_RESCUED: C2RustUnnamed_3 = 330;
pub const ID_SOUND_TEAM_GAMMA_ERADICATED: C2RustUnnamed_3 = 329;
pub const ID_SOUND_TEAM_BETA_ERADICATED: C2RustUnnamed_3 = 328;
pub const ID_SOUND_TEAM_ALPHA_ERADICATED: C2RustUnnamed_3 = 327;
pub const ID_SOUND_ENEMY_TRANSPORT_LANDING: C2RustUnnamed_3 = 326;
pub const ID_SOUND_ENEMY_ESCAPING: C2RustUnnamed_3 = 325;
pub const ID_SOUND_ENEMY_ESCAPED: C2RustUnnamed_3 = 324;
pub const ID_SOUND_OUT_OF_TIME: C2RustUnnamed_3 = 323;
pub const ID_SOUND_GROUP_INFECTED: C2RustUnnamed_3 = 322;
pub const ID_SOUND_STRUCTURE_INFECTED: C2RustUnnamed_3 = 321;
pub const ID_SOUND_OBJECTIVE_DESTROYED: C2RustUnnamed_3 = 320;
pub const ID_SOUND_OBJECTIVE_CAPTURED: C2RustUnnamed_3 = 319;
pub const ID_SOUND_GROUP_CAPTURED: C2RustUnnamed_3 = 318;
pub const ID_SOUND_GROUP_RESCUED: C2RustUnnamed_3 = 317;
pub const ID_SOUND_UNITS_RESCUED: C2RustUnnamed_3 = 316;
pub const ID_SOUND_CIVILIANS_RESCUED: C2RustUnnamed_3 = 315;
pub const ID_SOUND_CIVILIAN_RESCUED: C2RustUnnamed_3 = 314;
pub const ID_SOUND_STRUCTURE_CAPTURED: C2RustUnnamed_3 = 313;
pub const ID_SOUND_MISSILE_NME_DETECTED: C2RustUnnamed_3 = 312;
pub const ID_SOUND_MISSILE_SILO: C2RustUnnamed_3 = 311;
pub const ID_SOUND_SAM_SITE: C2RustUnnamed_3 = 310;
pub const ID_SOUND_NUCLEAR_REACTOR: C2RustUnnamed_3 = 309;
pub const ID_SOUND_NASDA_CENTRAL: C2RustUnnamed_3 = 308;
pub const ID_SOUND_SATELLITE_UPLINK: C2RustUnnamed_3 = 307;
pub const ID_SOUND_LANDING_ZONE: C2RustUnnamed_3 = 306;
pub const ID_SOUND_NOFAULTS: C2RustUnnamed_3 = 305;
pub const ID_SOUND_OF_SILENCE: C2RustUnnamed_3 = 304;
pub const ID_SOUND_BARB_SCREAM3: C2RustUnnamed_3 = 303;
pub const ID_SOUND_BARB_SCREAM2: C2RustUnnamed_3 = 302;
pub const ID_SOUND_BARB_SCREAM: C2RustUnnamed_3 = 301;
pub const ID_SOUND_HELP: C2RustUnnamed_3 = 300;
pub const ID_SOUND_FIRE_ROAR: C2RustUnnamed_3 = 299;
pub const ID_SOUND_ECM_TOWER: C2RustUnnamed_3 = 298;
pub const ID_SOUND_STEAM: C2RustUnnamed_3 = 297;
pub const ID_SOUND_POWER_SPARK: C2RustUnnamed_3 = 296;
pub const ID_SOUND_POWER_HUM: C2RustUnnamed_3 = 295;
pub const ID_SOUND_CYBORG_MOVE: C2RustUnnamed_3 = 293;
pub const ID_SOUND_HOVER_STOP: C2RustUnnamed_3 = 292;
pub const ID_SOUND_HOVER_START: C2RustUnnamed_3 = 291;
pub const ID_SOUND_HOVER_MOVE: C2RustUnnamed_3 = 290;
pub const ID_SOUND_TREAD: C2RustUnnamed_3 = 289;
pub const ID_SOUND_VTOL_MOVE: C2RustUnnamed_3 = 288;
pub const ID_SOUND_VTOL_OFF: C2RustUnnamed_3 = 287;
pub const ID_SOUND_VTOL_LAND: C2RustUnnamed_3 = 286;
pub const ID_SOUND_BLIMP_TAKE_OFF: C2RustUnnamed_3 = 285;
pub const ID_SOUND_BLIMP_LAND: C2RustUnnamed_3 = 284;
pub const ID_SOUND_BLIMP_IDLE: C2RustUnnamed_3 = 283;
pub const ID_SOUND_BLIMP_FLIGHT: C2RustUnnamed_3 = 282;
pub const ID_SOUND_CONSTRUCTOR_SHUTDOWN: C2RustUnnamed_3 = 281;
pub const ID_SOUND_CONSTRUCTOR_MOVE: C2RustUnnamed_3 = 280;
pub const ID_SOUND_CONSTRUCTOR_MOVEOFF: C2RustUnnamed_3 = 279;
pub const ID_SOUND_NEXUS_EXPLOSION: C2RustUnnamed_3 = 278;
pub const ID_SOUND_BUILDING_FALL: C2RustUnnamed_3 = 277;
pub const ID_SOUND_BARB_SQUISH: C2RustUnnamed_3 = 276;
pub const ID_SOUND_RICOCHET_3: C2RustUnnamed_3 = 275;
pub const ID_SOUND_RICOCHET_2: C2RustUnnamed_3 = 274;
pub const ID_SOUND_RICOCHET_1: C2RustUnnamed_3 = 273;
pub const ID_SOUND_EXPLOSION_ANTITANK: C2RustUnnamed_3 = 272;
pub const ID_SOUND_EXPLOSION: C2RustUnnamed_3 = 271;
pub const ID_SOUND_EXPLOSION_LASER: C2RustUnnamed_3 = 270;
pub const ID_SOUND_EXPLOSION_SMALL: C2RustUnnamed_3 = 269;
pub const ID_SOUND_CONSTRUCTION_4: C2RustUnnamed_3 = 268;
pub const ID_SOUND_CONSTRUCTION_3: C2RustUnnamed_3 = 267;
pub const ID_SOUND_CONSTRUCTION_2: C2RustUnnamed_3 = 266;
pub const ID_SOUND_CONSTRUCTION_1: C2RustUnnamed_3 = 265;
pub const ID_SOUND_CONSTRUCTION_LOOP: C2RustUnnamed_3 = 264;
pub const ID_SOUND_CONSTRUCTION_START: C2RustUnnamed_3 = 263;
pub const ID_SOUND_WELD_2: C2RustUnnamed_3 = 262;
pub const ID_SOUND_WELD_1: C2RustUnnamed_3 = 261;
pub const ID_SOUND_NEXUS_TOWER: C2RustUnnamed_3 = 260;
pub const ID_SOUND_HIVEL_CANNON: C2RustUnnamed_3 = 259;
pub const ID_SOUND_RAPID_CANNON: C2RustUnnamed_3 = 258;
pub const ID_SOUND_ASSAULT_MG: C2RustUnnamed_3 = 257;
pub const ID_SOUND_SPLASH: C2RustUnnamed_3 = 256;
pub const ID_SOUND_BABA_MG_TOWER: C2RustUnnamed_3 = 255;
pub const ID_SOUND_BABA_MG_HEAVY: C2RustUnnamed_3 = 254;
pub const ID_SOUND_BABA_MG_3: C2RustUnnamed_3 = 253;
pub const ID_SOUND_BABA_MG_2: C2RustUnnamed_3 = 252;
pub const ID_SOUND_BABA_MG_1: C2RustUnnamed_3 = 251;
pub const ID_SOUND_HOWITZ_FLIGHT: C2RustUnnamed_3 = 250;
pub const ID_SOUND_MORTAR: C2RustUnnamed_3 = 249;
pub const ID_SOUND_BEAM_LASER: C2RustUnnamed_3 = 248;
pub const ID_SOUND_PULSE_LASER: C2RustUnnamed_3 = 247;
pub const ID_SOUND_FLAME_THROWER: C2RustUnnamed_3 = 246;
pub const ID_SOUND_MEDIUM_CANNON: C2RustUnnamed_3 = 245;
pub const ID_SOUND_SMALL_CANNON: C2RustUnnamed_3 = 244;
pub const ID_SOUND_LARGE_CANNON: C2RustUnnamed_3 = 243;
pub const ID_SOUND_GAUSSGUN: C2RustUnnamed_3 = 242;
pub const ID_SOUND_ROTARY_LASER: C2RustUnnamed_3 = 241;
pub const ID_SOUND_ROCKET: C2RustUnnamed_3 = 240;
pub const ID_SOUND_COLL_ENEMY_DESTROYED: C2RustUnnamed_3 = 239;
pub const ID_SOUND_COLL_INTERCEPT_AND_DESTROY: C2RustUnnamed_3 = 238;
pub const ID_SOUND_COLL_DIE: C2RustUnnamed_3 = 237;
pub const ID_SOUND_COLL_STARTING_ATTACK_RUN: C2RustUnnamed_3 = 236;
pub const ID_SOUND_COLL_ENGAGING: C2RustUnnamed_3 = 235;
pub const ID_SOUND_COLL_ENEMY_DETECTED: C2RustUnnamed_3 = 234;
pub const ID_SOUND_COLL_FIRE: C2RustUnnamed_3 = 233;
pub const ID_SOUND_COLL_ATTACK: C2RustUnnamed_3 = 232;
pub const ID_SOUND_COLL_DESTROYING_BIOLOGICALS: C2RustUnnamed_3 = 231;
pub const ID_SOUND_COLL_CLEANSE_AND_DESTROY: C2RustUnnamed_3 = 230;
pub const ID_SOUND_ABORTING_ATTACK_RUN3: C2RustUnnamed_3 = 229;
pub const ID_SOUND_COMMENCING_ATTACK_RUN3: C2RustUnnamed_3 = 228;
pub const ID_SOUND_LOCKED_ON3: C2RustUnnamed_3 = 227;
pub const ID_SOUND_RETURNING_TO_BASE3: C2RustUnnamed_3 = 226;
pub const ID_SOUND_ON_OUR_WAY3: C2RustUnnamed_3 = 225;
pub const ID_SOUND_ENEMY_LOCATED3: C2RustUnnamed_3 = 224;
pub const ID_SOUND_ABORTING_ATTACK_RUN2: C2RustUnnamed_3 = 223;
pub const ID_SOUND_COMMENCING_ATTACK_RUN2: C2RustUnnamed_3 = 222;
pub const ID_SOUND_LOCKED_ON2: C2RustUnnamed_3 = 221;
pub const ID_SOUND_RETURNING_TO_BASE2: C2RustUnnamed_3 = 220;
pub const ID_SOUND_ON_OUR_WAY2: C2RustUnnamed_3 = 219;
pub const ID_SOUND_ENEMY_LOCATED2: C2RustUnnamed_3 = 218;
pub const ID_SOUND_ABORTING_ATTACK_RUN1: C2RustUnnamed_3 = 217;
pub const ID_SOUND_COMMENCING_ATTACK_RUN1: C2RustUnnamed_3 = 216;
pub const ID_SOUND_LOCKED_ON1: C2RustUnnamed_3 = 215;
pub const ID_SOUND_RETURNING_TO_BASE1: C2RustUnnamed_3 = 214;
pub const ID_SOUND_ON_OUR_WAY1: C2RustUnnamed_3 = 213;
pub const ID_SOUND_ENEMY_LOCATED1: C2RustUnnamed_3 = 212;
pub const ID_SOUND_PREPARE_FOR_DUST_OFF: C2RustUnnamed_3 = 211;
pub const ID_SOUND_GO_GO_GO: C2RustUnnamed_3 = 210;
pub const ID_SOUND_GREEN_LIGHT_IN_2: C2RustUnnamed_3 = 209;
pub const ID_SOUND_GREEN_LIGHT_IN_3: C2RustUnnamed_3 = 208;
pub const ID_SOUND_GREEN_LIGHT_IN_4: C2RustUnnamed_3 = 207;
pub const ID_SOUND_GREEN_LIGHT_IN_5: C2RustUnnamed_3 = 206;
pub const ID_SOUND_ALRIGHT_BOYS: C2RustUnnamed_3 = 205;
pub const ID_SOUND_APPROACHING_LZ: C2RustUnnamed_3 = 204;
pub const ID_SOUND_RADIOCLICK_6: C2RustUnnamed_3 = 203;
pub const ID_SOUND_RADIOCLICK_5: C2RustUnnamed_3 = 202;
pub const ID_SOUND_RADIOCLICK_4: C2RustUnnamed_3 = 201;
pub const ID_SOUND_RADIOCLICK_3: C2RustUnnamed_3 = 200;
pub const ID_SOUND_RADIOCLICK_2: C2RustUnnamed_3 = 199;
pub const ID_SOUND_RADIOCLICK_1: C2RustUnnamed_3 = 198;
pub const ID_SOUND_COM_HEADING_FOR_RALLY_POINT: C2RustUnnamed_3 = 197;
pub const ID_SOUND_COM_RETURNING_FOR_REPAIR: C2RustUnnamed_3 = 196;
pub const ID_SOUND_COM_UNABLE_TO_COMPLY: C2RustUnnamed_3 = 195;
pub const ID_SOUND_COM_NO_ROUTE_AVAILABLE: C2RustUnnamed_3 = 194;
pub const ID_SOUND_COM_ROUTE_OBSTRUCTED: C2RustUnnamed_3 = 193;
pub const ID_SOUND_COM_ENEMY_VTOLS_DETECTED: C2RustUnnamed_3 = 192;
pub const ID_SOUND_COM_ENEMY_BATTERY_DETECTED: C2RustUnnamed_3 = 191;
pub const ID_SOUND_COM_NEXUS_DETECTED: C2RustUnnamed_3 = 190;
pub const ID_SOUND_COM_NEXUS_TURRET_DETECTED: C2RustUnnamed_3 = 189;
pub const ID_SOUND_COM_NEXUS_TOWER_DETECTED: C2RustUnnamed_3 = 188;
pub const ID_SOUND_COM_FRIENDLY_LZ_DETECTED: C2RustUnnamed_3 = 187;
pub const ID_SOUND_COM_ENEMY_LZ_DETECTED: C2RustUnnamed_3 = 186;
pub const ID_SOUND_COM_ENEMY_TRANSPORT_DETECTED: C2RustUnnamed_3 = 185;
pub const ID_SOUND_COM_ALLY_DETECTED: C2RustUnnamed_3 = 184;
pub const ID_SOUND_COM_ENEMY_BASE_DETECTED: C2RustUnnamed_3 = 183;
pub const ID_SOUND_COM_ENEMY_DETECTED: C2RustUnnamed_3 = 182;
pub const ID_SOUND_COM_ARTEFACT_DETECTED: C2RustUnnamed_3 = 181;
pub const ID_SOUND_COM_RESOURCE_DETECTED: C2RustUnnamed_3 = 180;
pub const ID_SOUND_COM_SCAV_OUTPOST_DETECTED: C2RustUnnamed_3 = 179;
pub const ID_SOUND_COM_SCAV_BASE_DETECTED: C2RustUnnamed_3 = 178;
pub const ID_SOUND_COM_SCAVS_DETECTED: C2RustUnnamed_3 = 177;
pub const ID_SOUND_COMMANDER: C2RustUnnamed_3 = 176;
pub const ID_SOUND_REPORTING: C2RustUnnamed_3 = 175;
pub const ID_SOUND_GROUP_9: C2RustUnnamed_3 = 174;
pub const ID_SOUND_GROUP_8: C2RustUnnamed_3 = 173;
pub const ID_SOUND_GROUP_7: C2RustUnnamed_3 = 172;
pub const ID_SOUND_GROUP_6: C2RustUnnamed_3 = 171;
pub const ID_SOUND_GROUP_5: C2RustUnnamed_3 = 170;
pub const ID_SOUND_GROUP_4: C2RustUnnamed_3 = 169;
pub const ID_SOUND_GROUP_3: C2RustUnnamed_3 = 168;
pub const ID_SOUND_GROUP_2: C2RustUnnamed_3 = 167;
pub const ID_SOUND_GROUP_1: C2RustUnnamed_3 = 166;
pub const ID_SOUND_GROUP_0: C2RustUnnamed_3 = 165;
pub const ID_SOUND_GROUP: C2RustUnnamed_3 = 164;
pub const ID_UNITS_TRANSFER: C2RustUnnamed_3 = 163;
pub const ID_TECHNOLOGY_TRANSFER: C2RustUnnamed_3 = 162;
pub const ID_SENSOR_DOWNLOAD: C2RustUnnamed_3 = 161;
pub const ID_POWER_TRANSMIT: C2RustUnnamed_3 = 160;
pub const ID_GIFT: C2RustUnnamed_3 = 159;
pub const ID_CLAN_EXIT: C2RustUnnamed_3 = 158;
pub const ID_CLAN_ENTER: C2RustUnnamed_3 = 157;
pub const ID_ALLIANCE_OFF: C2RustUnnamed_3 = 156;
pub const ID_ALLIANCE_BRO: C2RustUnnamed_3 = 155;
pub const ID_ALLIANCE_ACC: C2RustUnnamed_3 = 154;
pub const ID_SOUND_ENTERING_WARZONE: C2RustUnnamed_3 = 153;
pub const ID_SOUND_MISSILE_CODES_CRACKED: C2RustUnnamed_3 = 152;
pub const ID_SOUND_3RD_MISSILE_CODES_DECIPHERED: C2RustUnnamed_3 = 151;
pub const ID_SOUND_2ND_MISSILE_CODES_DECIPHERED: C2RustUnnamed_3 = 150;
pub const ID_SOUND_1ST_MISSILE_CODES_DECIPHERED: C2RustUnnamed_3 = 149;
pub const ID_SOUND_MISSILE_CODES_DECIPHERED: C2RustUnnamed_3 = 148;
pub const ID_SOUND_YOU_ARE_DEFEATED: C2RustUnnamed_3 = 147;
pub const ID_SOUND_SYSTEM_FAILURE_IMMINENT: C2RustUnnamed_3 = 146;
pub const ID_SOUND_UNIT_CAPTURED: C2RustUnnamed_3 = 145;
pub const ID_SOUND_1_MINUTE_REMAINING: C2RustUnnamed_3 = 144;
pub const ID_SOUND_2_MINUTES_REMAINING: C2RustUnnamed_3 = 143;
pub const ID_SOUND_3_MINUTES_REMAINING: C2RustUnnamed_3 = 142;
pub const ID_SOUND_5_MINUTES_REMAINING: C2RustUnnamed_3 = 141;
pub const ID_SOUND_10_MINUTES_REMAINING: C2RustUnnamed_3 = 140;
pub const ID_SOUND_MISSION_TIMER_ACTIVATED: C2RustUnnamed_3 = 139;
pub const ID_SOUND_OBJECTIVE_FAILED: C2RustUnnamed_3 = 138;
pub const ID_SOUND_OBJECTIVE_ACCOMPLISHED: C2RustUnnamed_3 = 137;
pub const ID_SOUND_MISSION_SUCCESSFUL: C2RustUnnamed_3 = 136;
pub const ID_SOUND_MISSION_FAILED: C2RustUnnamed_3 = 135;
pub const ID_SOUND_INCOMING_INTELLIGENCE_REPORT: C2RustUnnamed_3 = 134;
pub const ID_SOUND_INCOMING_TRANSMISSION: C2RustUnnamed_3 = 133;
pub const ID_SOUND_TECHNOLOGY_TAKEN: C2RustUnnamed_3 = 132;
pub const ID_SOUND_RESEARCH_STOLEN: C2RustUnnamed_3 = 131;
pub const ID_SOUND_MISSION_RESULTS: C2RustUnnamed_3 = 130;
pub const ID_SOUND_WARZONE_ACTIVE: C2RustUnnamed_3 = 129;
pub const ID_SOUND_WARZONE_PAUSED: C2RustUnnamed_3 = 128;
pub const ID_SOUND_MISSION_UPDATE: C2RustUnnamed_3 = 127;
pub const ID_SOUND_MISSION_OBJECTIVE: C2RustUnnamed_3 = 126;
pub const ID_SOUND_TRANSPORT_UNABLE_TO_LAND: C2RustUnnamed_3 = 125;
pub const ID_SOUND_TRANSPORT_RETURNING_TO_BASE: C2RustUnnamed_3 = 124;
pub const ID_SOUND_LZ_CLEAR: C2RustUnnamed_3 = 123;
pub const ID_SOUND_LZ_COMPROMISED: C2RustUnnamed_3 = 122;
pub const ID_SOUND_TRANSPORT_REPAIRING: C2RustUnnamed_3 = 121;
pub const ID_SOUND_TRANSPORT_UNDER_ATTACK: C2RustUnnamed_3 = 120;
pub const ID_SOUND_TRANSPORT_LANDING: C2RustUnnamed_3 = 119;
pub const ID_SOUND_REINFORCEMENTS_IN_TRANSIT: C2RustUnnamed_3 = 118;
pub const ID_SOUND_REINFORCEMENTS_AVAILABLE: C2RustUnnamed_3 = 117;
pub const ID_SOUND_NO_ROUTE_AVAILABLE: C2RustUnnamed_3 = 116;
pub const ID_SOUND_ROUTE_OBSTRUCTED: C2RustUnnamed_3 = 115;
pub const ID_SOUND_COMMANDER_REPORTING: C2RustUnnamed_3 = 114;
pub const ID_SOUND_GROUP_REPORTING: C2RustUnnamed_3 = 113;
pub const ID_SOUND_ASSIGNED_TO_COMMANDER: C2RustUnnamed_3 = 112;
pub const ID_SOUND_EXCELLENT: C2RustUnnamed_3 = 111;
pub const ID_SOUND_WELL_DONE: C2RustUnnamed_3 = 110;
pub const ID_SOUND_THAT_IS_INCORRECT: C2RustUnnamed_3 = 109;
pub const ID_SOUND_NO: C2RustUnnamed_3 = 108;
pub const ID_SOUND_NOT_POSSIBLE_TRY_AGAIN: C2RustUnnamed_3 = 107;
pub const ID_SOUND_SCATTER: C2RustUnnamed_3 = 106;
pub const ID_SOUND_RECYCLING: C2RustUnnamed_3 = 105;
pub const ID_SOUND_RETURN_TO_LZ: C2RustUnnamed_3 = 104;
pub const ID_SOUND_PATROL: C2RustUnnamed_3 = 103;
pub const ID_SOUND_PURSUE: C2RustUnnamed_3 = 102;
pub const ID_SOUND_GUARD: C2RustUnnamed_3 = 101;
pub const ID_SOUND_HOLD_POSITION: C2RustUnnamed_3 = 100;
pub const ID_SOUND_CEASEFIRE: C2RustUnnamed_3 = 99;
pub const ID_SOUND_RETURN_FIRE: C2RustUnnamed_3 = 98;
pub const ID_SOUND_FIRE_AT_WILL: C2RustUnnamed_3 = 97;
pub const ID_SOUND_NO_RETREAT: C2RustUnnamed_3 = 96;
pub const ID_SOUND_RETREAT_AT_HEAVY_DAMAGE: C2RustUnnamed_3 = 95;
pub const ID_SOUND_RETREAT_AT_MEDIUM_DAMAGE: C2RustUnnamed_3 = 94;
pub const ID_SOUND_OPTIMUM_RANGE: C2RustUnnamed_3 = 93;
pub const ID_SOUND_LONG_RANGE: C2RustUnnamed_3 = 92;
pub const ID_SOUND_SHORT_RANGE: C2RustUnnamed_3 = 91;
pub const ID_SOUND_COMMAND_CONSOLE_ACTIVATED: C2RustUnnamed_3 = 90;
pub const ID_SOUND_INTERCEPTORS_ASSIGNED: C2RustUnnamed_3 = 89;
pub const ID_SOUND_ASSIGNED: C2RustUnnamed_3 = 88;
pub const ID_SOUND_VTOLS_ENGAGING: C2RustUnnamed_3 = 87;
pub const ID_SOUND_REARMING: C2RustUnnamed_3 = 86;
pub const ID_SOUND_INTERCEPTORS_LAUNCHED: C2RustUnnamed_3 = 85;
pub const ID_SOUND_BATTERY_FIRING_COUNTER_ATTACK: C2RustUnnamed_3 = 84;
pub const ID_SOUND_ENEMY_BATTERY_LOCATED: C2RustUnnamed_3 = 83;
pub const ID_SOUND_ASSIGNED_TO_COUNTER_RADAR: C2RustUnnamed_3 = 82;
pub const ID_SOUND_SENSOR_LOCKED_ON: C2RustUnnamed_3 = 81;
pub const ID_SOUND_ASSIGNED_TO_SENSOR: C2RustUnnamed_3 = 80;
pub const ID_SOUND_UNIT_RETURNING_FOR_REPAIR: C2RustUnnamed_3 = 79;
pub const ID_SOUND_UNIT_RETREATING: C2RustUnnamed_3 = 78;
pub const ID_SOUND_UNIT_DESTROYED: C2RustUnnamed_3 = 77;
pub const ID_SOUND_UNIT_UNDER_ATTACK: C2RustUnnamed_3 = 76;
pub const ID_SOUND_LZ2: C2RustUnnamed_3 = 75;
pub const ID_SOUND_LZ1: C2RustUnnamed_3 = 74;
pub const ID_SOUND_ENEMY_LZ: C2RustUnnamed_3 = 73;
pub const ID_SOUND_INCOMING_ENEMY_TRANSPORT: C2RustUnnamed_3 = 72;
pub const ID_SOUND_ENEMY_BASE_ERADICATED: C2RustUnnamed_3 = 71;
pub const ID_SOUND_ENEMY_BASE: C2RustUnnamed_3 = 70;
pub const ID_SOUND_SCAVENGER_BASE_ERADICATED: C2RustUnnamed_3 = 69;
pub const ID_SOUND_SCAVENGER_OUTPOST_ERADICATED: C2RustUnnamed_3 = 68;
pub const ID_SOUND_SCAVENGER_OUTPOST: C2RustUnnamed_3 = 67;
pub const ID_SOUND_SCAVENGER_BASE: C2RustUnnamed_3 = 66;
pub const ID_SOUND_ENEMY_VTOLS_DETECTED: C2RustUnnamed_3 = 65;
pub const ID_SOUND_ENEMY_BATTERY_DETECTED: C2RustUnnamed_3 = 64;
pub const ID_SOUND_NEXUS_UNIT_DETECTED: C2RustUnnamed_3 = 63;
pub const ID_SOUND_NEXUS_TURRET_DETECTED: C2RustUnnamed_3 = 62;
pub const ID_SOUND_NEXUS_TOWER_DETECTED: C2RustUnnamed_3 = 61;
pub const ID_SOUND_FRIENDLY_LZ_DETECTED: C2RustUnnamed_3 = 60;
pub const ID_SOUND_ENEMY_LZ_DETECTED: C2RustUnnamed_3 = 59;
pub const ID_SOUND_ENEMY_TRANSPORT_DETECTED: C2RustUnnamed_3 = 58;
pub const ID_SOUND_ALLY_DETECTED: C2RustUnnamed_3 = 57;
pub const ID_SOUND_ENEMY_BASE_DETECTED: C2RustUnnamed_3 = 56;
pub const ID_SOUND_ENEMY_UNIT_DETECTED: C2RustUnnamed_3 = 55;
pub const ID_SOUND_ARTEFACT_DISC: C2RustUnnamed_3 = 54;
pub const ID_SOUND_POWER_RESOURCE: C2RustUnnamed_3 = 53;
pub const ID_SOUND_SCAVENGER_OUTPOST_DETECTED: C2RustUnnamed_3 = 52;
pub const ID_SOUND_SCAVENGER_BASE_DETECTED: C2RustUnnamed_3 = 51;
pub const ID_SOUND_SCAVENGERS_DETECTED: C2RustUnnamed_3 = 50;
pub const ID_SOUND_UNIT_REPAIRED: C2RustUnnamed_3 = 49;
pub const ID_SOUND_DELIVERY_POINT_ASSIGNED_TO: C2RustUnnamed_3 = 48;
pub const ID_SOUND_DELIVERY_POINT_ASSIGNED: C2RustUnnamed_3 = 47;
pub const ID_SOUND_PRODUCTION_CANCELLED: C2RustUnnamed_3 = 46;
pub const ID_SOUND_PRODUCTION_PAUSED: C2RustUnnamed_3 = 45;
pub const ID_SOUND_DROID_COMPLETED: C2RustUnnamed_3 = 44;
pub const ID_SOUND_PRODUCTION_STARTED: C2RustUnnamed_3 = 43;
pub const ID_SOUND_CYBORG_RESEARCH_COMPLETED: C2RustUnnamed_3 = 42;
pub const ID_SOUND_WEAPON_RESEARCH_COMPLETED: C2RustUnnamed_3 = 41;
pub const ID_SOUND_SYSTEMS_RESEARCH_COMPLETED: C2RustUnnamed_3 = 40;
pub const ID_SOUND_VEHICLE_RESEARCH_COMPLETED: C2RustUnnamed_3 = 39;
pub const ID_SOUND_COMPUTER_RESEARCH_COMPLETED: C2RustUnnamed_3 = 38;
pub const ID_SOUND_POWER_RESEARCH_COMPLETED: C2RustUnnamed_3 = 37;
pub const ID_SOUND_STRUCTURE_RESEARCH_COMPLETED: C2RustUnnamed_3 = 36;
pub const ID_SOUND_MAJOR_RESEARCH: C2RustUnnamed_3 = 35;
pub const ID_SOUND_RESEARCH_COMPLETED: C2RustUnnamed_3 = 34;
pub const ID_SOUND_NEW_CYBORG_AVAILABLE: C2RustUnnamed_3 = 33;
pub const ID_SOUND_NEW_COMPONENT_AVAILABLE: C2RustUnnamed_3 = 32;
pub const ID_SOUND_NEW_STRUCTURE_AVAILABLE: C2RustUnnamed_3 = 31;
pub const ID_SOUND_NEW_RESEARCH_PROJ_AVAILABLE: C2RustUnnamed_3 = 30;
pub const ID_SOUND_ARTIFACT_RECOVERED: C2RustUnnamed_3 = 29;
pub const ID_SOUND_ARTIFACT: C2RustUnnamed_3 = 28;
pub const ID_SOUND_RESEARCH_FACILITY_REQUIRED: C2RustUnnamed_3 = 27;
pub const ID_SOUND_POWER_GENERATOR_REQUIRED: C2RustUnnamed_3 = 26;
pub const ID_SOUND_POWER_TRANSFER_IN_PROGRESS: C2RustUnnamed_3 = 25;
pub const ID_SOUND_RESOURCE_DEPLETED: C2RustUnnamed_3 = 24;
pub const ID_SOUND_DERRICK_DESTROYED: C2RustUnnamed_3 = 23;
pub const ID_SOUND_DERRICK_UNDER_ATTACK: C2RustUnnamed_3 = 22;
pub const ID_SOUND_RESOURCE_HERE: C2RustUnnamed_3 = 21;
pub const ID_SOUND_POWER_LOW: C2RustUnnamed_3 = 20;
pub const ID_SOUND_POWER_GENERATOR_DESTROYED: C2RustUnnamed_3 = 19;
pub const ID_SOUND_POWER_GENERATOR_UNDER_ATTACK: C2RustUnnamed_3 = 18;
pub const ID_SOUND_STRUCTURE_DEMOLISHED: C2RustUnnamed_3 = 17;
pub const ID_SOUND_STRUCTURE_REPAIR_IN_PROGRESS: C2RustUnnamed_3 = 16;
pub const ID_SOUND_STRUCTURE_UNDER_ATTACK: C2RustUnnamed_3 = 15;
pub const ID_SOUND_STRUCTURE_COMPLETED: C2RustUnnamed_3 = 14;
pub const ID_SOUND_CONSTRUCTION_STARTED: C2RustUnnamed_3 = 13;
pub const ID_SOUND_DESIGN_COMPLETED: C2RustUnnamed_3 = 12;
pub const ID_SOUND_PROPULSION_SELECTED: C2RustUnnamed_3 = 11;
pub const ID_SOUND_BODY_SELECTED: C2RustUnnamed_3 = 10;
pub const ID_SOUND_TURRET_SELECTED: C2RustUnnamed_3 = 9;
pub const ID_SOUND_GAME_SHUTDOWN: C2RustUnnamed_3 = 8;
pub const ID_SOUND_MESSAGEEND: C2RustUnnamed_3 = 7;
pub const ID_SOUND_BUILD_FAIL: C2RustUnnamed_3 = 6;
pub const ID_SOUND_ZOOM_ON_RADAR: C2RustUnnamed_3 = 5;
pub const FE_AUDIO_MESSAGEEND: C2RustUnnamed_3 = 4;
pub const ID_SOUND_BUTTON_CLICK_5: C2RustUnnamed_3 = 3;
pub const ID_SOUND_SELECT: C2RustUnnamed_3 = 2;
pub const ID_SOUND_WINDOWOPEN: C2RustUnnamed_3 = 1;
pub const ID_SOUND_WINDOWCLOSE: C2RustUnnamed_3 = 0;
pub const NO_SOUND: C2RustUnnamed_3 = -1;
pub type _droid_action = libc::c_uint;
pub const DACTION_FIRESUPPORT_RETREAT: _droid_action = 39;
pub const DACTION_RETURNTOPOS: _droid_action = 38;
pub const DACTION_CLEARREARMPAD: _droid_action = 37;
pub const DACTION_VTOLATTACK: _droid_action = 36;
pub const DACTION_MOVETOREARMPOINT: _droid_action = 34;
pub const DACTION_WAITFORREARM: _droid_action = 33;
pub const DACTION_MOVETOREARM: _droid_action = 32;
pub const DACTION_MOVETOCLEAR: _droid_action = 31;
pub const DACTION_MOVETORESTORE: _droid_action = 30;
pub const DACTION_MOVETODROIDREPAIR: _droid_action = 29;
pub const DACTION_MOVETOREPAIRPOINT: _droid_action = 27;
pub const DACTION_WAITFORREPAIR: _droid_action = 26;
pub const DACTION_MOVETOOBSERVE: _droid_action = 25;
pub const DACTION_ROTATETOATTACK: _droid_action = 24;
pub const DACTION_MOVETOATTACK: _droid_action = 23;
pub const DACTION_FOUNDATION_WANDER: _droid_action = 22;
pub const DACTION_BUILDWANDER: _droid_action = 21;
pub const DACTION_MOVETOREPAIR: _droid_action = 20;
pub const DACTION_MOVETODEMOLISH: _droid_action = 19;
pub const DACTION_MOVETOBUILD: _droid_action = 18;
pub const DACTION_MOVEFIRE: _droid_action = 17;
pub const DACTION_DROIDREPAIR: _droid_action = 14;
pub const DACTION_TRANSPORTIN: _droid_action = 13;
pub const DACTION_TRANSPORTWAITTOFLYIN: _droid_action = 12;
pub const DACTION_TRANSPORTOUT: _droid_action = 11;
pub const DACTION_DESTRUCT: _droid_action = 10;
pub const DACTION_SULK: _droid_action = 9;
pub const DACTION_FIRESUPPORT: _droid_action = 8;
pub const DACTION_OBSERVE: _droid_action = 7;
pub const DACTION_ATTACK: _droid_action = 6;
pub const DACTION_BUILD_FOUNDATION: _droid_action = 3;
pub const DACTION_MOVE: _droid_action = 1;
pub const DACTION_NONE: _droid_action = 0;
pub type _group_type = libc::c_uint;
pub const GT_TRANSPORTER: _group_type = 2;
pub const GT_NORMAL: _group_type = 0;
// Offset into texture page to left hand edge
// Offset into texture page to top hand edge
// Which textpage is the tile in? TileNumber/16 basically;
/* Return a pointer to the tile structure at x,y */
#[inline]
unsafe extern "C" fn mapTile(mut x: UDWORD, mut y: UDWORD) -> *mut MAPTILE {
    if x < mapWidth {
    } else {
        debug(LOG_ERROR,
              b"mapTile: x coordinate bigger than map width\x00" as *const u8
                  as *const libc::c_char);
    };
    if x < mapWidth {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"./map.h\x00" as *const u8 as *const libc::c_char,
              285 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 8],
                                        &[libc::c_char; 8]>(b"mapTile\x00")).as_ptr(),
              b"x < mapWidth\x00" as *const u8 as *const libc::c_char);
    };
    if y < mapHeight {
    } else {
        debug(LOG_ERROR,
              b"mapTile: y coordinate bigger than map height\x00" as *const u8
                  as *const libc::c_char);
    };
    if y < mapHeight {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"./map.h\x00" as *const u8 as *const libc::c_char,
              287 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 8],
                                        &[libc::c_char; 8]>(b"mapTile\x00")).as_ptr(),
              b"y < mapHeight\x00" as *const u8 as *const libc::c_char);
    };
    //return psMapTiles + x + (y << mapShift); //width no longer a power of 2
    return psMapTiles.offset(x as
                                 isize).offset(y.wrapping_mul(mapWidth) as
                                                   isize);
}
/* Return height of tile at x,y */
//static inline SDWORD map_TileHeight(UDWORD x, UDWORD y)
#[inline]
unsafe extern "C" fn map_TileHeight(mut x: UDWORD, mut y: UDWORD) -> SWORD {
    if x >= mapWidth || y >= mapHeight { return 0 as libc::c_int as SWORD }
    return ((*psMapTiles.offset(x.wrapping_add(y.wrapping_mul(mapWidth)) as
                                    isize)).height as libc::c_int *
                2 as libc::c_int) as SWORD;
}
// Returns TRUE if drive mode is active.
//
#[inline]
unsafe extern "C" fn driveModeActive() -> BOOL { return DirectControl; }
// Return TRUE if the specified droid is the driven droid.
//
#[inline]
unsafe extern "C" fn driveIsDriven(mut psDroid: *mut DROID) -> BOOL {
    return if DirectControl != 0 && !psDrivenDroid.is_null() &&
                  psDroid == psDrivenDroid {
               1 as libc::c_int
           } else { 0 as libc::c_int };
}
//BOOL driveHasDriven(void);
//BOOL driveModeActive(void);
//BOOL driveIsDriven(DROID *psDroid);
//BOOL driveIsFollower(DROID *psDroid);
#[inline]
unsafe extern "C" fn driveGetDriven() -> *mut DROID { return psDrivenDroid; }
#[no_mangle]
pub static mut bRender3DOnly: BOOL = 0;
#[no_mangle]
pub static mut bSensorDisplay: BOOL = 1 as libc::c_int;
#[no_mangle]
pub static mut bDrawBlips: BOOL = 1 as libc::c_int;
#[no_mangle]
pub static mut bDrawProximitys: BOOL = 1 as libc::c_int;
#[no_mangle]
pub static mut godMode: BOOL = 0;
/* *******************  Variables  ********************/
// Should be cleaned up properly and be put in structures.
static mut WaterTileID: UWORD = 17 as libc::c_int as UWORD;
static mut RiverBedTileID: UWORD = 5 as libc::c_int as UWORD;
static mut waterRealValue: FRACT = 0 as libc::c_int as FRACT;
static mut vOffset: SWORD = 1 as libc::c_int as SWORD;
static mut separation: FRACT = 0 as libc::c_int as FRACT;
static mut acceleration: SDWORD = 0 as libc::c_int;
static mut heightSpeed: SDWORD = 0 as libc::c_int;
static mut aSep: FRACT = 0.;
static mut aAccel: SDWORD = 0 as libc::c_int;
static mut aSpeed: SDWORD = 0 as libc::c_int;
#[no_mangle]
pub static mut barMode: UDWORD = 0 as libc::c_int as UDWORD;
// configured in configuration.c
/* Is the scene spinning round - just for showcase stuff */
#[no_mangle]
pub static mut spinScene: BOOL = 0 as libc::c_int;
/* Initial 3D world origins */
#[no_mangle]
pub static mut mapX: UDWORD = 45 as libc::c_int as UDWORD;
#[no_mangle]
pub static mut mapY: UDWORD = 80 as libc::c_int as UDWORD;
/* Have we made a selection by clicking the mouse - used for dragging etc */
#[no_mangle]
pub static mut selectAttempt: BOOL = 0 as libc::c_int;
/* Vectors that hold the player and camera directions and positions */
#[no_mangle]
pub static mut player: iView =
    iView{p: iVector{x: 0, y: 0, z: 0,}, r: iVector{x: 0, y: 0, z: 0,},};
#[no_mangle]
pub static mut camera: iView =
    iView{p: iVector{x: 0, y: 0, z: 0,}, r: iVector{x: 0, y: 0, z: 0,},};
/* Temporary rotation vectors to store rotations for droids etc */
#[no_mangle]
pub static mut imdRot: iVector = iVector{x: 0, y: 0, z: 0,};
#[no_mangle]
pub static mut imdRot2: iVector = iVector{x: 0, y: 0, z: 0,};
/* How far away are we from the terrain */
#[no_mangle]
pub static mut distance: UDWORD = 2500 as libc::c_int as UDWORD;
//(DISTANCE - (DISTANCE/6));
/* Are we outlining the terrain tile triangles */
#[no_mangle]
pub static mut terrainOutline: UDWORD = 0 as libc::c_int as UDWORD;
/* Stores the screen coordinates of the transformed terrain tiles */
#[no_mangle]
pub static mut tileScreenInfo: [[SVMESH; 33]; 33] =
    [[SVMESH{x: 0,
             y: 0,
             z: 0,
             tu: 0,
             tv: 0,
             light: PIELIGHT{byte: PIELIGHTBYTES{b: 0, g: 0, r: 0, a: 0,},},
             specular:
                 PIELIGHT{byte: PIELIGHTBYTES{b: 0, g: 0, r: 0, a: 0,},},
             sx: 0,
             sy: 0,
             sz: 0,
             wx: 0,
             wy: 0,
             wz: 0,
             water_height: 0,
             wlight: PIELIGHT{byte: PIELIGHTBYTES{b: 0, g: 0, r: 0, a: 0,},},
             drawInfo: 0,
             bWater: 0,}; 33]; 33];
/* Stores the tilepointers for rendered tiles */
#[no_mangle]
pub static mut tileIJ: [[TILE_BUCKET; 33]; 33] =
    [[TILE_BUCKET{i: 0, j: 0, depth: 0,}; 33]; 33];
/* File size - used for any loads. Move to another specific file handling module? */
#[no_mangle]
pub static mut fileSize: SDWORD = 0;
/* Stores the texture for a specific tile */
static mut texturePage: iTexture =
    {
        let mut init =
            iTexture{xshift: 6 as libc::c_int,
                     width: 64 as libc::c_int,
                     height: 64 as libc::c_int,
                     bmp: 0 as *const iBitmap as *mut iBitmap,
                     pPal: 0 as *const iColour as *mut iColour,
                     bColourKeyed: 0,};
        init
    };
/* Pointer to which tile the mouse is currently over */
#[no_mangle]
pub static mut tile3dOver: *mut MAPTILE = 0 as *const MAPTILE as *mut MAPTILE;
/* Records the present X and Y values for the current mouse tile (in tiles */
#[no_mangle]
pub static mut mouseTileX: SDWORD = 0;
#[no_mangle]
pub static mut mouseTileY: SDWORD = 0;
/* World coordinates that the mouse is over */
#[no_mangle]
pub static mut tile3dX: UDWORD = 0;
#[no_mangle]
pub static mut tile3dY: UDWORD = 0;
/* Offsets for the screen being shrunk/expanded - how far in, how far down */
#[no_mangle]
pub static mut xOffset: UDWORD = 0 as libc::c_int as UDWORD;
#[no_mangle]
pub static mut yOffset: UDWORD = 0 as libc::c_int as UDWORD;
/* Do we want the radar to be rendered */
#[no_mangle]
pub static mut radarOnScreen: BOOL = 0 as libc::c_int;
/* Show unit/building gun/sensor range*/
#[no_mangle]
pub static mut rangeOnScreen: BOOL = 0 as libc::c_int;
// For now, most likely will change later!  -Q 5-10-05   A very nice effect - Per
/* Temporary values for the terrain render - top left corner of grid to be rendered */
#[no_mangle]
pub static mut playerXTile: int32 = 0;
#[no_mangle]
pub static mut playerZTile: int32 = 0;
#[no_mangle]
pub static mut rx: int32 = 0;
#[no_mangle]
pub static mut rz: int32 = 0;
/* Have we located the mouse? */
#[no_mangle]
pub static mut mouseLocated: BOOL = 1 as libc::c_int;
/* The box used for multiple selection - present screen coordinates */
/* The game palette */
#[no_mangle]
pub static mut gamePal: iPalette = [iColour{r: 0, g: 0, b: 0,}; 256];
#[no_mangle]
pub static mut currentGameFrame: UDWORD = 0;
static mut numTiles: UDWORD = 0 as libc::c_int as UDWORD;
static mut tileZ: SDWORD = 8000 as libc::c_int;
static mut dragQuad: QUAD = QUAD{coords: [POINT{x: 0, y: 0,}; 4],};
#[no_mangle]
pub static mut cameraHeight: UDWORD = 400 as libc::c_int as UDWORD;
//UDWORD	averageHeight;
// The maximum number of points for flattenImd
static mut alteredPoints: [iVector; 255] = [iVector{x: 0, y: 0, z: 0,}; 255];
//number of tiles visible
#[no_mangle]
pub static mut visibleXTiles: UDWORD = 0;
#[no_mangle]
pub static mut visibleYTiles: UDWORD = 0;
#[no_mangle]
pub static mut terrainMidX: UDWORD = 0;
#[no_mangle]
pub static mut terrainMidY: UDWORD = 0;
#[no_mangle]
pub static mut terrainMaxX: UDWORD = 0;
#[no_mangle]
pub static mut terrainMaxY: UDWORD = 0;
static mut underwaterTile: UDWORD = 17 as libc::c_int as UDWORD;
static mut rubbleTile: UDWORD = 67 as libc::c_int as UDWORD;
//WATER_TILE;
#[no_mangle]
pub static mut geoOffset: UDWORD = 0;
static mut numTilesAveraged: UDWORD = 0;
static mut averageCentreTerrainHeight: UDWORD = 0;
static mut bReloadBars: BOOL = 1 as libc::c_int;
static mut bEnergyBars: BOOL = 1 as libc::c_int;
static mut bTinyBars: BOOL = 0 as libc::c_int;
static mut edgeTile: MAPTILE =
    MAPTILE{tileInfoBits: 0,
            tileVisBits: 0,
            height: 0,
            illumination: 0,
            texture: 0,
            bMaxed: 0,
            level: 0,
            inRange: 0,};
static mut lastTargetAssignation: UDWORD = 0 as libc::c_int as UDWORD;
static mut lastDestAssignation: UDWORD = 0 as libc::c_int as UDWORD;
static mut bSensorTargetting: BOOL = 0 as libc::c_int;
static mut bDestTargetting: BOOL = 0 as libc::c_int;
static mut psSensorObj: *mut BASE_OBJECT =
    0 as *const BASE_OBJECT as *mut BASE_OBJECT;
static mut destTargetX: UDWORD = 0;
static mut destTargetY: UDWORD = 0;
static mut destTileX: UDWORD = 0 as libc::c_int as UDWORD;
static mut destTileY: UDWORD = 0 as libc::c_int as UDWORD;
/* Colour strobe values for the strobing drag selection box */
#[no_mangle]
pub static mut boxPulseColours: [UBYTE; 10] =
    [233 as libc::c_int as UBYTE, 232 as libc::c_int as UBYTE,
     231 as libc::c_int as UBYTE, 230 as libc::c_int as UBYTE,
     229 as libc::c_int as UBYTE, 228 as libc::c_int as UBYTE,
     227 as libc::c_int as UBYTE, 226 as libc::c_int as UBYTE,
     225 as libc::c_int as UBYTE, 224 as libc::c_int as UBYTE];
#[no_mangle]
pub static mut lightLevel: UDWORD = 12 as libc::c_int as UDWORD;
#[no_mangle]
pub static mut tCon: UDWORD = 0;
#[no_mangle]
pub static mut tIgn: UDWORD = 0;
#[no_mangle]
pub static mut tCal: UDWORD = 0;
#[no_mangle]
pub static mut defaultColours: DEF_COLOURS =
    DEF_COLOURS{red: 0,
                green: 0,
                blue: 0,
                yellow: 0,
                purple: 0,
                white: 0,
                black: 0,
                cyan: 0,};
#[no_mangle]
pub static mut pitch: SDWORD = 0;
/* *******************  Functions  ********************/
#[no_mangle]
pub unsafe extern "C" fn GetCameraDistance() -> UDWORD { return distance; }
#[no_mangle]
pub unsafe extern "C" fn displayMultiChat() {
    let mut pixelLength: UDWORD = 0;
    let mut pixelHeight: UDWORD = 0;
    pixelLength = iV_GetTextWidth(sTextToSend.as_mut_ptr()) as UDWORD;
    pixelHeight = iV_GetTextLineSize() as UDWORD;
    if gameTime2.wrapping_rem(500 as libc::c_int as libc::c_uint) <
           250 as libc::c_int as libc::c_uint {
        pie_BoxFillIndex((23 as libc::c_int as
                              libc::c_uint).wrapping_add(pixelLength).wrapping_add(3
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       libc::c_uint)
                             as libc::c_int,
                         (474 as libc::c_int as
                              libc::c_uint).wrapping_add(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                                     as
                                                                                                     libc::c_int
                                                                                                     as
                                                                                                     libc::c_uint)).wrapping_sub(pixelHeight.wrapping_div(4
                                                                                                                                                              as
                                                                                                                                                              libc::c_int
                                                                                                                                                              as
                                                                                                                                                              libc::c_uint))
                             as libc::c_int,
                         (23 as libc::c_int as
                              libc::c_uint).wrapping_add(pixelLength).wrapping_add(10
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       libc::c_uint)
                             as libc::c_int,
                         (473 as libc::c_int as
                              libc::c_uint).wrapping_add(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                                     as
                                                                                                     libc::c_int
                                                                                                     as
                                                                                                     libc::c_uint))
                             as libc::c_int, 255 as libc::c_int as UBYTE);
    }
    /* GET RID OF THE MAGIC NUMBERS BELOW */
    pie_TransBoxFill(23 as libc::c_int + 1 as libc::c_int,
                     (474 as libc::c_int as
                          libc::c_uint).wrapping_add(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_uint)).wrapping_sub(pixelHeight)
                         as SDWORD,
                     ((23 as libc::c_int + 1 as libc::c_int) as
                          libc::c_uint).wrapping_add(pixelLength).wrapping_add(2
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   libc::c_uint)
                         as SDWORD,
                     (473 as libc::c_int as
                          libc::c_uint).wrapping_add(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_uint))
                         as SDWORD);
    pie_DrawText(sTextToSend.as_mut_ptr(),
                 (23 as libc::c_int + 3 as libc::c_int) as UDWORD,
                 (469 as libc::c_int as
                      libc::c_uint).wrapping_add(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_uint)));
}
// Optimisation to stop it being calculated every frame
static mut gridCentreX: SDWORD = 0;
static mut gridCentreZ: SDWORD = 0;
static mut gridVarCalls: SDWORD = 0;
#[no_mangle]
pub unsafe extern "C" fn getCentreX() -> SDWORD {
    gridVarCalls += 1;
    return gridCentreX;
}
#[no_mangle]
pub unsafe extern "C" fn getCentreZ() -> SDWORD { return gridCentreZ; }
/* Render the 3D world */
#[no_mangle]
pub unsafe extern "C" fn draw3DScene() {
    //SDWORD	angle;
    let mut bPlayerHasHQ: BOOL = 0 as libc::c_int;
    /* Set the droids on-screen display coordinates for selection later */
    mX = mouseX();
    mY = mouseY();
    // the world centre - used for decaying lighting etc
    gridCentreX =
        (player.p.x as
             libc::c_uint).wrapping_add(visibleXTiles.wrapping_div(2 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint)
                                            << 7 as libc::c_int) as SDWORD;
    gridCentreZ =
        (player.p.z as
             libc::c_uint).wrapping_add(visibleYTiles.wrapping_div(2 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint)
                                            << 7 as libc::c_int) as SDWORD;
    /* What frame number are we on? */
    currentGameFrame = frameGetFrameNumber();
    /* Lock the surface */
    pie_GlobalRenderBegin(); //only begins scene if it wasn't already begun
    pie_LocalRenderBegin();
    /* Build the drag quad */
    if dragBox3D.status == 2 as libc::c_int as libc::c_uint {
        dragQuad.coords[0 as libc::c_int as usize].x =
            dragBox3D.x1 as libc::c_int; // TOP LEFT
        dragQuad.coords[0 as libc::c_int as usize].y =
            dragBox3D.y1 as libc::c_int; // TOP RIGHT
        dragQuad.coords[1 as libc::c_int as usize].x =
            dragBox3D.x2 as libc::c_int; // BOTTOM RIGHT
        dragQuad.coords[1 as libc::c_int as usize].y =
            dragBox3D.y1 as libc::c_int; // BOTTOM LEFT
        dragQuad.coords[2 as libc::c_int as usize].x =
            dragBox3D.x2 as libc::c_int;
        dragQuad.coords[2 as libc::c_int as usize].y =
            dragBox3D.y2 as libc::c_int;
        dragQuad.coords[3 as libc::c_int as usize].x =
            dragBox3D.x1 as libc::c_int;
        dragQuad.coords[3 as libc::c_int as usize].y =
            dragBox3D.y2 as libc::c_int
    }
    pie_Begin3DScene();
    displayTerrain();
    pie_BeginInterface();
    updateLightLevels();
    drawDroidSelections();
    /* Show the selected delivery point */
//	drawDeliveryPointSelection(0);
    drawStructureSelections();
    //	drawBuildingLines();
    bPlayerHasHQ = radarCheckForHQ(selectedPlayer);
    //	if(radarOnScreen AND (bPlayerHasHQ || (bMultiPlayer && (game.type == DMATCH)) ))
    if radarOnScreen != 0 && bPlayerHasHQ != 0 {
        pie_SetDepthBufferStatus(DEPTH_CMP_ALWAYS_WRT_ON);
        pie_SetFogStatus(0 as libc::c_int);
        drawRadar();
        if doWeDrawRadarBlips() != 0 { drawRadarBlips(); }
        pie_SetDepthBufferStatus(DEPTH_CMP_LEQ_WRT_ON);
        pie_SetFogStatus(1 as libc::c_int);
    }
    /* Unlock the surface */
    pie_LocalRenderEnd();
    if bRender3DOnly == 0 {
        (gameStats) != 0;
        //		if(getWarCamStatus())
//		{
//			dispWarCamLogo();
//		}
        pie_SetFogStatus(0 as libc::c_int);
        displayConsoleMessages();
    }
    /* Ensure that any text messages are displayed at bottom of screen */
    //	scoreDataToScreen();
    pie_SetDepthBufferStatus(DEPTH_CMP_ALWAYS_WRT_OFF);
    pie_SetFogStatus(0 as libc::c_int);
    iV_SetTextColour(-(1 as libc::c_int) as SWORD);
    //	if(widgetsOn)
//	{
//		iV_DrawText("Warzone 2100 - Pumpkin Studios - QA(4) ",190,470);
//	}
    //----------------------------------------------------------
//----------------------------------------------------------
//----------------------------------------------------------
 	/* Dont remove this folks!!!! */
    if bAllowOtherKeyPresses == 0 {
        displayMultiChat();
    } else if gamePaused() != 0 {
        pie_DrawText(b"Developed by Pumpkin Studios\x00" as *const u8 as
                         *const libc::c_char as *mut STRING,
                     (23 as libc::c_int + 3 as libc::c_int) as UDWORD,
                     (467 as libc::c_int as
                          libc::c_uint).wrapping_add(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_uint)));
        pie_DrawText(b"Published by EIDOS Interactive\x00" as *const u8 as
                         *const libc::c_char as *mut STRING,
                     pie_GetVideoBufferWidth().wrapping_sub(196 as libc::c_int
                                                                as
                                                                libc::c_uint),
                     (467 as libc::c_int as
                          libc::c_uint).wrapping_add(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_uint)));
    }
    // FIXME: This wasn't shown before. Do we want to keep it? Or change it?
    /*
	if(mousePressed(MOUSE_LMB))
	{
		{
			if(apsDroidLists[0])
			{
				iVector	pos;
				UDWORD	i;
				pos.x = apsDroidLists[0]->x;
				pos.z = apsDroidLists[0]->y;
				pos.y = map_Height(pos.x,pos.z);
				addEffect(&pos,EFFECT_SAT_LASER,SAT_LASER_STANDARD,FALSE,NULL,0);
			}

		}
	}
	*/
 //----------------------------------------------------------
//----------------------------------------------------------
//----------------------------------------------------------
    if getDebugMappingStatus() != 0 && demoGetStatus() == 0 &&
           gamePaused() == 0 {
        pie_DrawText(b"DEBUG \x00" as *const u8 as *const libc::c_char as
                         *mut STRING,
                     (23 as libc::c_int + 134 as libc::c_int) as UDWORD,
                     (440 as libc::c_int as
                          libc::c_uint).wrapping_add(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_uint)));
    }
    while player.r.y >
              65536 as libc::c_int / 360 as libc::c_int * 360 as libc::c_int {
        player.r.y -=
            65536 as libc::c_int / 360 as libc::c_int * 360 as libc::c_int
    }
    /* If we don't have an active camera track, then track terrain height! */
    if getWarCamStatus() == 0 {
        /*	player.p.y = averageCentreTerrainHeight; */
	/* Move the autonomous camera if necessary */
        trackHeight((2 as libc::c_int as
                         libc::c_uint).wrapping_mul(averageCentreTerrainHeight)
                        as SDWORD);
    } else { processWarCam(); }
    if demoGetStatus() != 0 {
        flushConsoleMessages();
        setConsolePermanence(1 as libc::c_int, 1 as libc::c_int);
        permitNewConsoleMessages(1 as libc::c_int);
        addConsoleMessage(b"Warzone 2100 : Pumpkin Studios \x00" as *const u8
                              as *const libc::c_char as *mut STRING,
                          RIGHT_JUSTIFY);
        permitNewConsoleMessages(0 as libc::c_int);
    }
    //	sprintf(buildInfo,"WallDrag from %d,%d to %d,%d", wallDrag.x1,wallDrag.y1,wallDrag.x2,wallDrag.y2);
   //	iV_DrawText(buildInfo,100,180);
	/*
 	sprintf(buildInfo,"Gridvar calls : %d", gridVarCalls);
	iV_DrawText(buildInfo,100,180);

	sprintf(buildInfo,"Instructions saved : %d", gridVarCalls*24);
	iV_DrawText(buildInfo,100,200);
		gridVarCalls = 0;
	*/
    //	sprintf(buildInfo,"Average Grid Height : %d", averageCentreTerrainHeight);
 //	iV_DrawText(buildInfo,100,240);
 //	sprintf(buildInfo,"Height : %d", player.p.y);
 //	iV_DrawText(buildInfo,100,260);
    processDemoCam(); //this does squat, but leave it for now I guess -Q
    processSensorTarget();
    processDestinationTarget();
    testEffect();
    if bSensorDisplay != 0 {
        showDroidSensorRanges();
        //shows sensor data for units/droids/whatever...-Q 5-10-05
    };
}
/* Draws the 3D textured terrain */
#[no_mangle]
pub unsafe extern "C" fn displayTerrain() {
    //	SDWORD	x,y;
    tileZ = 8000 as libc::c_int;
    //	x = ((visibleXTiles/2)*128);
//	y = ((visibleYTiles/2)*128);
    //	x += player.p.x;
//	y += player.p.z;
    camera.p.z = distance as int32;
    camera.p.y = 0 as libc::c_int;
    camera.p.x = 0 as libc::c_int;
    /* SetUpClipping window - to below the backdrop */
    pie_Set2DClip(xOffset as libc::c_int, yOffset as libc::c_int,
                  ((*psRendSurface).width as
                       libc::c_uint).wrapping_sub(xOffset) as libc::c_int,
                  ((*psRendSurface).height as
                       libc::c_uint).wrapping_sub(yOffset) as libc::c_int);
    /* Set 3D world origins */
    pie_SetGeometricOffset(rendSurface.width >> 1 as libc::c_int,
                           geoOffset as libc::c_int);
    pie_PerspectiveBegin();
    /* We haven't yet located which tile mouse is over */
    mouseLocated = 0 as libc::c_int;
    numTiles = 0 as libc::c_int as UDWORD;
    /* Setup tiles */
    preprocessTiles();
    /* Now, draw the terrain */
    drawTiles(&mut camera, &mut player);
    pie_PerspectiveEnd();
    /* Show the drag Box if necessary */
    drawDragBox();
    /* Have we released the drag box? */
    if dragBox3D.status == 2 as libc::c_int as libc::c_uint {
        dragBox3D.status = 0 as libc::c_int as UDWORD
    };
}
// Parameter is the vector to store the camera position
#[no_mangle]
pub unsafe extern "C" fn CalcBSPCameraPos(mut NewBSPCamera: *mut iVector) {
    let mut Camera: OBJPOS =
        OBJPOS{x: 0, y: 0, z: 0, pitch: 0, yaw: 0, roll: 0,};
    /* The 128's should really be TILE_UNITS - sort this out later ! */
    Camera.x =
        (player.p.x as
             libc::c_uint).wrapping_add(terrainMidX.wrapping_mul(128 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint));
    Camera.y = player.p.y as WORLDCOORD;
    Camera.z =
        (player.p.z as
             libc::c_uint).wrapping_add(terrainMidY.wrapping_mul(128 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint));
    Camera.pitch = player.r.x as SWORD;
    Camera.yaw = player.r.y as SWORD;
    GetRealCameraPos(&mut Camera, GetCameraDistance() as SDWORD,
                     NewBSPCamera);
}
//was FALSE	**COUGH and I spend 2 days making my own. LOL -Q 5-10-05
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn doWeDrawRadarBlips() -> BOOL { return bDrawBlips; }
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn doWeDrawProximitys() -> BOOL {
    return bDrawProximitys;
}
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn setBlipDraw(mut val: BOOL) { bDrawBlips = val; }
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn setProximityDraw(mut val: BOOL) {
    bDrawProximitys = val;
}
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn drawTiles(mut camera_0: *mut iView,
                                   mut player_0: *mut iView) {
    let mut i: SDWORD = 0;
    let mut j: SDWORD = 0;
    let mut zMax: SDWORD = 0;
    let mut BSPCamera: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut psTile: *mut MAPTILE = 0 as *mut MAPTILE;
    let mut specular: UDWORD = 0;
    let mut tilesRejected: UDWORD = 0;
    let mut edgeX: UDWORD = 0;
    let mut edgeY: UDWORD = 0;
    let mut IsWaterTile: BOOL = 0 as libc::c_int;
    let mut PushedDown: BOOL = 0 as libc::c_int;
    let mut TileIllum: UBYTE = 0;
    let mut TextNum: UWORD = 0;
    let mut shiftVal: UDWORD = 0 as libc::c_int as UDWORD;
    let mut altVal: UDWORD = 0 as libc::c_int as UDWORD;
    let mut fraction: FRACT = 0.;
    let mut realX: UDWORD = 0;
    let mut realY: UDWORD = 0;
    let mut bEdgeTile: BOOL = 0;
    let mut tmp_y: SDWORD = 0;
    static mut angle: libc::c_float = 0.0f64 as libc::c_float;
    // Animate the water texture, just cycles the V coordinate through half the tiles height.
    if gamePaused() == 0 {
        fraction = frameTime2 as FRACT / 1000 as libc::c_int as libc::c_float;
        waterRealValue += fraction * 4 as libc::c_int as libc::c_float;
        vOffset = waterRealValue as SDWORD as SWORD;
        if vOffset as libc::c_int >= 64 as libc::c_int / 2 as libc::c_int {
            vOffset = 0 as libc::c_int as SWORD;
            waterRealValue = 0 as libc::c_int as FRACT
        }
    }
    /* Is the scene spinning? - showcase demo stuff */
    if spinScene != 0 {
        (*player_0).r.y +=
            65536 as libc::c_int / 360 as libc::c_int * 3 as libc::c_int
    }
    /* ---------------------------------------------------------------- */
	/* Do boundary and extent checking                                  */
	/* ---------------------------------------------------------------- */
	/* Get the mid point of the grid */
    terrainMidX = visibleXTiles >> 1 as libc::c_int;
    terrainMidY = visibleYTiles >> 1 as libc::c_int;
    CalcBSPCameraPos(&mut BSPCamera);
    SetBSPCameraPos(BSPCamera.x, BSPCamera.y, BSPCamera.z);
    /* Find our position in tile coordinates */
    playerXTile = (*player_0).p.x >> 7 as libc::c_int;
    playerZTile = (*player_0).p.z >> 7 as libc::c_int;
    /* Get the x,z translation components */
    rx = (*player_0).p.x & 128 as libc::c_int - 1 as libc::c_int;
    rz = (*player_0).p.z & 128 as libc::c_int - 1 as libc::c_int;
    /* ---------------------------------------------------------------- */
	/* Set up the geometry                                              */
	/* ---------------------------------------------------------------- */
    /* ---------------------------------------------------------------- */
	/* Push identity matrix onto stack */
    pie_MatBegin();
    // Now, scale the world according to what resolution we're running in
    pie_MatScale(pie_GetResScalingFactor());
    /* Set the camera position */
    pie_MATTRANS((*camera_0).p.x, (*camera_0).p.y, (*camera_0).p.z);
    /* Rotate for the player */
    pie_MatRotZ((*player_0).r.z);
    pie_MatRotX((*player_0).r.x);
    pie_MatRotY((*player_0).r.y);
    /* Translate */
    pie_TRANSLATE(-rx, -(*player_0).p.y, rz);
    angle = (angle as libc::c_double + 0.01f64) as libc::c_float;
    // RODZ uncomment the following line to see an OpenGL lighting demo
    if getDrawShadows() != 0 {
        pie_BeginLighting(50 as libc::c_int as libc::c_float,
                          -(300 as libc::c_int) as libc::c_float,
                          -(300 as libc::c_int) as libc::c_float);
    }
    /* ---------------------------------------------------------------- */
	/* Rotate and project all the tiles within the grid                 */
	/* ---------------------------------------------------------------- */
	/*	We track the height here - so make sure we get the average heights
		of the tiles in the grid
	*/
    averageCentreTerrainHeight = 0 as libc::c_int as UDWORD;
    numTilesAveraged = 0 as libc::c_int as UDWORD;
    i = 0 as libc::c_int;
    while i < visibleYTiles as SDWORD + 1 as libc::c_int {
        /* Go through the x's */
        j = 0 as libc::c_int; //map_TileHeight(edgeX,edgeY);
        while j < visibleXTiles as SDWORD + 1 as libc::c_int {
            tileScreenInfo[i as usize][j as usize].bWater =
                0 as libc::c_int as UBYTE;
            if playerXTile + j < 0 as libc::c_int ||
                   playerZTile + i < 0 as libc::c_int ||
                   playerXTile + j >
                       mapWidth.wrapping_sub(1 as libc::c_int as libc::c_uint)
                           as SDWORD ||
                   playerZTile + i >
                       mapHeight.wrapping_sub(1 as libc::c_int as
                                                  libc::c_uint) as SDWORD {
                edgeX = (playerXTile + j) as UDWORD;
                edgeY = (playerZTile + i) as UDWORD;
                if playerXTile + j < 0 as libc::c_int {
                    edgeX = 0 as libc::c_int as UDWORD
                } else if playerXTile + j >
                              mapWidth.wrapping_sub(1 as libc::c_int as
                                                        libc::c_uint) as
                                  SDWORD {
                    edgeX =
                        mapWidth.wrapping_sub(1 as libc::c_int as
                                                  libc::c_uint)
                }
                if playerZTile + i < 0 as libc::c_int {
                    edgeY = 0 as libc::c_int as UDWORD
                } else if playerZTile + i >
                              mapHeight.wrapping_sub(1 as libc::c_int as
                                                         libc::c_uint) as
                                  SDWORD {
                    edgeY =
                        mapHeight.wrapping_sub(1 as libc::c_int as
                                                   libc::c_uint)
                }
                tileScreenInfo[i as usize][j as usize].x =
                    ((j as libc::c_uint).wrapping_sub(terrainMidX) <<
                         7 as libc::c_int) as SDWORD;
                tileScreenInfo[i as usize][j as usize].y = 0 as libc::c_int;
                tileScreenInfo[i as usize][j as usize].z =
                    (terrainMidY.wrapping_sub(i as libc::c_uint) <<
                         7 as libc::c_int) as SDWORD;
                tileScreenInfo[i as usize][j as usize].sz =
                    pie_RotProj(&mut (*(*tileScreenInfo.as_mut_ptr().offset(i
                                                                                as
                                                                                isize)).as_mut_ptr().offset(j
                                                                                                                as
                                                                                                                isize)).x
                                    as *mut SDWORD as *mut iVector,
                                &mut (*(*tileScreenInfo.as_mut_ptr().offset(i
                                                                                as
                                                                                isize)).as_mut_ptr().offset(j
                                                                                                                as
                                                                                                                isize)).sx
                                    as *mut SDWORD as *mut iPoint);
                if pie_GetFogEnabled() != 0 {
                    tileScreenInfo[i as usize][j as usize].light.argb =
                        0xff030303 as libc::c_uint;
                    tileScreenInfo[i as usize][j as usize].specular.argb =
                        pie_GetFogColour()
                } else {
                    tileScreenInfo[i as usize][j as usize].light.argb =
                        lightDoFogAndIllumination((*mapTile(edgeX,
                                                            edgeY)).illumination,
                                                  rx -
                                                      tileScreenInfo[i as
                                                                         usize][j
                                                                                    as
                                                                                    usize].x,
                                                  (rz as
                                                       libc::c_uint).wrapping_sub((i
                                                                                       as
                                                                                       libc::c_uint).wrapping_sub(terrainMidY)
                                                                                      <<
                                                                                      7
                                                                                          as
                                                                                          libc::c_int)
                                                      as SDWORD,
                                                  &mut specular)
                }
                if playerXTile + j < -(1 as libc::c_int) ||
                       playerZTile + i < -(1 as libc::c_int) ||
                       playerXTile + j >
                           mapWidth.wrapping_sub(1 as libc::c_int as
                                                     libc::c_uint) as SDWORD
                       ||
                       playerZTile + i >
                           mapHeight.wrapping_sub(1 as libc::c_int as
                                                      libc::c_uint) as SDWORD
                   {
                    tileScreenInfo[i as usize][j as usize].drawInfo =
                        0 as libc::c_int as UBYTE
                } else {
                    tileScreenInfo[i as usize][j as usize].drawInfo =
                        1 as libc::c_int as UBYTE
                }
            } else {
                tileScreenInfo[i as usize][j as usize].drawInfo =
                    1 as libc::c_int as UBYTE;
                psTile =
                    mapTile((playerXTile + j) as UDWORD,
                            (playerZTile + i) as UDWORD);
                /* Get a pointer to the tile at this location */
                tileScreenInfo[i as usize][j as usize].x =
                    ((j as libc::c_uint).wrapping_sub(terrainMidX) <<
                         7 as libc::c_int) as SDWORD;
                if terrainTypes[((*psTile).texture as libc::c_int &
                                     0x1ff as libc::c_int) as usize] as
                       libc::c_int == TER_WATER as libc::c_int {
                    tileScreenInfo[i as usize][j as usize].bWater =
                        1 as libc::c_int as UBYTE
                }
                tileScreenInfo[i as usize][j as usize].y =
                    map_TileHeight((playerXTile + j) as UDWORD,
                                   (playerZTile + i) as UDWORD) as SDWORD;
                tileScreenInfo[i as usize][j as usize].z =
                    (terrainMidY.wrapping_sub(i as libc::c_uint) <<
                         7 as libc::c_int) as SDWORD;
                /* Is it in the centre and therefore worth averaging height over? */
                if i > 32 as libc::c_int / 4 as libc::c_int &&
                       i <
                           3 as libc::c_int * 32 as libc::c_int /
                               4 as libc::c_int &&
                       j > 32 as libc::c_int / 4 as libc::c_int &&
                       j <
                           3 as libc::c_int * 32 as libc::c_int /
                               4 as libc::c_int {
                    averageCentreTerrainHeight =
                        (averageCentreTerrainHeight as
                             libc::c_uint).wrapping_add(tileScreenInfo[i as
                                                                           usize][j
                                                                                      as
                                                                                      usize].y
                                                            as libc::c_uint)
                            as UDWORD as UDWORD;
                    numTilesAveraged = numTilesAveraged.wrapping_add(1)
                }
                realX = (playerXTile + j) as UDWORD;
                realY = (playerZTile + i) as UDWORD;
                bEdgeTile = 0 as libc::c_int;
                if realX <= 1 as libc::c_int as libc::c_uint ||
                       realY <= 1 as libc::c_int as libc::c_uint ||
                       realX >=
                           mapWidth.wrapping_sub(2 as libc::c_int as
                                                     libc::c_uint) ||
                       realY >=
                           mapHeight.wrapping_sub(2 as libc::c_int as
                                                      libc::c_uint) {
                    bEdgeTile = 1 as libc::c_int
                }
                if getRevealStatus() != 0 {
                    if godMode != 0 {
                        TileIllum = (*psTile).illumination
                    } else {
                        TileIllum =
                            if (*psTile).level as libc::c_int ==
                                   0xff as libc::c_int {
                                1 as libc::c_int
                            } else { (*psTile).level as libc::c_int } as UBYTE
                        //avGetTileLevel(realX,realY);
                    }
                } else { TileIllum = (*psTile).illumination }
                if bDisplaySensorRange != 0 { TileIllum = (*psTile).inRange }
                TextNum =
                    ((*psTile).texture as libc::c_int & 0x1ff as libc::c_int)
                        as UWORD;
                IsWaterTile =
                    (terrainTypes[((*psTile).texture as libc::c_int &
                                       0x1ff as libc::c_int) as usize] as
                         libc::c_int == TER_WATER as libc::c_int) as
                        libc::c_int;
                // If it's the main water tile then..
                PushedDown = 0 as libc::c_int;
                if TextNum as libc::c_int == WaterTileID as libc::c_int &&
                       bEdgeTile == 0 {
                    // Push the terrain down for the river bed.
                    PushedDown =
                        1 as
                            libc::c_int; //environGetValue(playerXTile+j,playerZTile+i);
                    shiftVal =
                        (127 as libc::c_int as
                             libc::c_uint).wrapping_add((3 as libc::c_int as
                                                             libc::c_uint).wrapping_mul(environGetData((playerXTile
                                                                                                            +
                                                                                                            j)
                                                                                                           as
                                                                                                           UDWORD,
                                                                                                       (playerZTile
                                                                                                            +
                                                                                                            i)
                                                                                                           as
                                                                                                           UDWORD)).wrapping_div(2
                                                                                                                                     as
                                                                                                                                     libc::c_int
                                                                                                                                     as
                                                                                                                                     libc::c_uint));
                    altVal = 0 as libc::c_int as UDWORD;
                    tileScreenInfo[i as usize][j as usize].y =
                        (tileScreenInfo[i as usize][j as usize].y as
                             libc::c_uint).wrapping_sub(shiftVal.wrapping_add(altVal))
                            as SDWORD as SDWORD;
                    // And darken it.
                    TileIllum =
                        (TileIllum as libc::c_int * 3 as libc::c_int /
                             4 as libc::c_int) as UBYTE
                }
                tileScreenInfo[i as usize][j as usize].sz =
                    pie_RotProj(&mut *(*tileScreenInfo.as_mut_ptr().offset(i
                                                                               as
                                                                               isize)).as_mut_ptr().offset(j
                                                                                                               as
                                                                                                               isize)
                                    as *mut SVMESH as *mut iVector,
                                &mut (*(*tileScreenInfo.as_mut_ptr().offset(i
                                                                                as
                                                                                isize)).as_mut_ptr().offset(j
                                                                                                                as
                                                                                                                isize)).sx
                                    as *mut SDWORD as *mut iPoint);
                tileScreenInfo[i as usize][j as usize].light.argb =
                    lightDoFogAndIllumination(TileIllum,
                                              rx -
                                                  tileScreenInfo[i as
                                                                     usize][j
                                                                                as
                                                                                usize].x,
                                              (rz as
                                                   libc::c_uint).wrapping_sub((i
                                                                                   as
                                                                                   libc::c_uint).wrapping_sub(terrainMidY)
                                                                                  <<
                                                                                  7
                                                                                      as
                                                                                      libc::c_int)
                                                  as SDWORD, &mut specular);
                tileScreenInfo[i as usize][j as usize].specular.argb =
                    specular;
                // If it's any water tile..
                if IsWaterTile != 0 {
                    // If it's the main water tile then bring it back up because it was pushed down
						// for the river bed calc.
                    tmp_y = tileScreenInfo[i as usize][j as usize].y;
                    if PushedDown != 0 {
                        //TextNum == WaterTileID) {
                        tileScreenInfo[i as usize][j as usize].y =
                            (tileScreenInfo[i as usize][j as usize].y as
                                 libc::c_uint).wrapping_add(shiftVal.wrapping_add((2
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       libc::c_uint).wrapping_mul(altVal)))
                                as SDWORD as SDWORD
                    }
                    // Transform it into the wx,wy mesh members.
                    tileScreenInfo[i as usize][j as usize].wz =
                        pie_RotProj(&mut *(*tileScreenInfo.as_mut_ptr().offset(i
                                                                                   as
                                                                                   isize)).as_mut_ptr().offset(j
                                                                                                                   as
                                                                                                                   isize)
                                        as *mut SVMESH as *mut iVector,
                                    &mut (*(*tileScreenInfo.as_mut_ptr().offset(i
                                                                                    as
                                                                                    isize)).as_mut_ptr().offset(j
                                                                                                                    as
                                                                                                                    isize)).wx
                                        as *mut SDWORD as *mut iPoint);
                    tileScreenInfo[i as usize][j as usize].wlight.argb =
                        lightDoFogAndIllumination(TileIllum,
                                                  rx -
                                                      tileScreenInfo[i as
                                                                         usize][j
                                                                                    as
                                                                                    usize].x,
                                                  (rz as
                                                       libc::c_uint).wrapping_sub((i
                                                                                       as
                                                                                       libc::c_uint).wrapping_sub(terrainMidY)
                                                                                      <<
                                                                                      7
                                                                                          as
                                                                                          libc::c_int)
                                                      as SDWORD,
                                                  &mut specular);
                    tileScreenInfo[i as usize][j as usize].water_height =
                        tileScreenInfo[i as usize][j as usize].y;
                    tileScreenInfo[i as usize][j as usize].y = tmp_y
                } else {
                    // If it was'nt a water tile then need to ensure wx,wy are valid because
						// a water tile might be sharing verticies with it.
                    tileScreenInfo[i as usize][j as usize].wx =
                        tileScreenInfo[i as usize][j as usize].sx;
                    tileScreenInfo[i as usize][j as usize].wy =
                        tileScreenInfo[i as usize][j as usize].sy;
                    tileScreenInfo[i as usize][j as usize].wz =
                        tileScreenInfo[i as usize][j as usize].sz;
                    tileScreenInfo[i as usize][j as usize].water_height =
                        tileScreenInfo[i as usize][j as usize].y
                }
            }
            j += 1
        }
        i += 1
    }
    /* Work out the average height */
    if numTilesAveraged != 0 {
        // might not be if off map
        averageCentreTerrainHeight =
            (averageCentreTerrainHeight as
                 libc::c_uint).wrapping_div(numTilesAveraged) as UDWORD as
                UDWORD
    } else {
        averageCentreTerrainHeight =
            (128 as libc::c_int * 2 as libc::c_int) as UDWORD
    }
    /* This is done here as effects can light the terrain - pause mode problems though */
    processEffects();
    atmosUpdateSystem();
    (waterOnMap()) != 0;
    if getRevealStatus() != 0 { avUpdateTiles(); }
    //	doBuildingLights();
	/* ---------------------------------------------------------------- */
	/* Draw all the tiles or add them to bucket sort                     */
	/* ---------------------------------------------------------------- */
    tilesRejected = 0 as libc::c_int as UDWORD;
    i = 0 as libc::c_int;
    while i < visibleYTiles as SDWORD {
        j = 0 as libc::c_int;
        while j < visibleXTiles as SDWORD {
            if tileScreenInfo[i as usize][j as usize].drawInfo as libc::c_int
                   == 1 as libc::c_int {
                tileIJ[i as usize][j as usize].i = i as UDWORD;
                tileIJ[i as usize][j as usize].j = j as UDWORD;
                //get distance of furthest corner
                zMax =
                    if tileScreenInfo[i as usize][j as usize].sz >
                           tileScreenInfo[(i + 1 as libc::c_int) as
                                              usize][j as usize].sz {
                        tileScreenInfo[i as usize][j as usize].sz
                    } else {
                        tileScreenInfo[(i + 1 as libc::c_int) as
                                           usize][j as usize].sz
                    };
                zMax =
                    if zMax >
                           tileScreenInfo[(i + 1 as libc::c_int) as
                                              usize][(j + 1 as libc::c_int) as
                                                         usize].sz {
                        zMax
                    } else {
                        tileScreenInfo[(i + 1 as libc::c_int) as
                                           usize][(j + 1 as libc::c_int) as
                                                      usize].sz
                    };
                zMax =
                    if zMax >
                           tileScreenInfo[i as
                                              usize][(j + 1 as libc::c_int) as
                                                         usize].sz {
                        zMax
                    } else {
                        tileScreenInfo[i as
                                           usize][(j + 1 as libc::c_int) as
                                                      usize].sz
                    };
                tileIJ[i as usize][j as usize].depth = zMax;
                if i as UDWORD > mapHeight || j as UDWORD > mapWidth {
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"Weirdy tile coords\x00" as *const u8 as
                                  *const libc::c_char);
                    };
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                                  *const u8 as *const libc::c_char,
                              b"display3d.c\x00" as *const u8 as
                                  *const libc::c_char, 968 as libc::c_int,
                              (*::std::mem::transmute::<&[u8; 10],
                                                        &[libc::c_char; 10]>(b"drawTiles\x00")).as_ptr(),
                              b"FALSE\x00" as *const u8 as
                                  *const libc::c_char);
                    };
                }
                bucketAddTypeToList(RENDER_TILE,
                                    &mut *(*tileIJ.as_mut_ptr().offset(i as
                                                                           isize)).as_mut_ptr().offset(j
                                                                                                           as
                                                                                                           isize)
                                        as *mut TILE_BUCKET as
                                        *mut libc::c_void);
                bucketAddTypeToList(RENDER_WATERTILE,
                                    &mut *(*tileIJ.as_mut_ptr().offset(i as
                                                                           isize)).as_mut_ptr().offset(j
                                                                                                           as
                                                                                                           isize)
                                        as *mut TILE_BUCKET as
                                        *mut libc::c_void);
            } else { tilesRejected = tilesRejected.wrapping_add(1) }
            j += 1
            //bucket
        }
        i += 1
    }
    targetOpenList(driveGetDriven() as *mut BASE_OBJECT);
    /* ---------------------------------------------------------------- */
	/* Now display all the static objects                               */
	/* ---------------------------------------------------------------- */
    displayStaticObjects(); //bucket render implemented
    displayFeatures(); //bucket render implemented
    displayDynamicObjects(); //bucket render implemented
    if doWeDrawProximitys() != 0 {
        displayProximityMsgs();
        //bucket render implemented
    } //bucket render implemented
    displayDelivPoints(); //bucket render implemented
    display3DProjectiles();
    drawEffects();
    atmosDrawParticles();
    bucketRenderCurrentList();
    pie_RemainingPasses();
    pie_EndLighting();
    targetCloseList();
    if driveModeActive() != 0 {
        // If were in driving mode then mark the current target.
        let mut psObj: *mut BASE_OBJECT = targetGetCurrent();
        if !psObj.is_null() { targetMarkCurrent(); }
    }
    if gamePaused() == 0 { doConstructionLines(); }
    /* Clear the matrix stack */
    pie_MatEnd();
    locateMouse();
}
#[no_mangle]
pub unsafe extern "C" fn init3DView() -> libc::c_int {
    // the world centre - used for decaying lighting etc
    gridCentreX =
        (player.p.x as
             libc::c_uint).wrapping_add(visibleXTiles.wrapping_div(2 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint)
                                            << 7 as libc::c_int) as SDWORD;
    gridCentreZ =
        (player.p.z as
             libc::c_uint).wrapping_add(visibleYTiles.wrapping_div(2 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint)
                                            << 7 as libc::c_int) as SDWORD;
    pie_SetGammaValue(gammaValue);
    edgeTile.texture = 0 as libc::c_int as UWORD;
    bEnergyBars = 1 as libc::c_int;
    /* Base Level */
    geoOffset = 192 as libc::c_int as UDWORD;
    //set up how many tiles to draw
    visibleXTiles = 32 as libc::c_int as UDWORD;
    visibleYTiles = 32 as libc::c_int as UDWORD;
    /* There are no drag boxes */
    dragBox3D.status = 0 as libc::c_int as UDWORD;
    /* Arbitrary choice - from direct read! */
    theSun.x = 0 as libc::c_int;
    theSun.y = -(3441 as libc::c_int);
    theSun.z = 2619 as libc::c_int;
    /* Make sure and change these to comply with map.c */
    imdRot.x = -(35 as libc::c_int);
    /* Maximum map size */
    terrainMaxX = 128 as libc::c_int as UDWORD;
    terrainMaxY = 128 as libc::c_int as UDWORD;
    //	terrainSizeX = distance/100;
//	terrainSizeY = distance/100;
//	terrainMidX = (terrainSizeX>>1);
//	terrainMidY = terrainSizeY>>1;
    /* Get all the init stuff out of here? */
    initWarCam();
    /* Init the game messaging system */
    initConsoleMessages();
    /* Initialise the effects system */
  //	initEffectsSystem();
    atmosInitSystem();
    initDemoCamera();
    /* Set up the sine table for the bullets */
    initBulletTable();
    /* Set up light values */
   //	initLighting();
    /* Build our shade table for gouraud shading - 256*16 values with best match from 256 colour table */
    pal_BuildAdjustedShadeTable();
    getDefaultColours();
    /* No initial rotations */
    imdRot2.x = 0 as libc::c_int;
    imdRot.y = 0 as libc::c_int;
    imdRot2.z = 0 as libc::c_int;
    /* Set up the player */
/*	player.p.y = 0;
	player.p.x = mapWidth/2*TILE_UNITS;
	player.p.z = mapHeight/2*TILE_UNITS;

	setViewAngle(-30);
	player.r.y = DEG(-45); */
    bRender3DOnly = 0 as libc::c_int;
    targetInitialise();
    /* Set up the fog tbale for the 3dfx */
//	pie_SetFogTable(0x00000000,65536,65536);
    return 1 as libc::c_int;
}
// set the view position from save game
#[no_mangle]
pub unsafe extern "C" fn disp3d_setView(mut newView: *mut iView) {
    memcpy(&mut player as *mut iView as *mut libc::c_void,
           newView as *const libc::c_void,
           ::std::mem::size_of::<iView>() as libc::c_ulong);
}
// get the view position for save game
#[no_mangle]
pub unsafe extern "C" fn disp3d_getView(mut newView: *mut iView) {
    memcpy(newView as *mut libc::c_void,
           &mut player as *mut iView as *const libc::c_void,
           ::std::mem::size_of::<iView>() as libc::c_ulong);
}
/* John's routine - deals with flipping around the vertex ordering for source textures
   when flips and rotations are being done */
#[no_mangle]
pub unsafe extern "C" fn flipsAndRots(mut texture: libc::c_int) {
    /* Store the source rect as four points */
    sP1.x = 1 as libc::c_int;
    sP1.y = 1 as libc::c_int;
    sP2.x = 63 as libc::c_int;
    sP2.y = 1 as libc::c_int;
    sP3.x = 63 as libc::c_int;
    sP3.y = 63 as libc::c_int;
    sP4.x = 1 as libc::c_int;
    sP4.y = 63 as libc::c_int;
    /* Store pointers to the points */
    psP1 = &mut sP1;
    psP2 = &mut sP2;
    psP3 = &mut sP3;
    psP4 = &mut sP4;
    if texture & 0x8000 as libc::c_int != 0 {
        psPTemp = psP1;
        psP1 = psP2;
        psP2 = psPTemp;
        psPTemp = psP3;
        psP3 = psP4;
        psP4 = psPTemp
    }
    if texture & 0x4000 as libc::c_int != 0 {
        psPTemp = psP1;
        psP1 = psP4;
        psP4 = psPTemp;
        psPTemp = psP2;
        psP2 = psP3;
        psP3 = psPTemp
    }
    match (texture & 0x3000 as libc::c_int) >> 12 as libc::c_int {
        1 => {
            psPTemp = psP1;
            psP1 = psP4;
            psP4 = psP3;
            psP3 = psP2;
            psP2 = psPTemp
        }
        2 => {
            psPTemp = psP1;
            psP1 = psP3;
            psP3 = psPTemp;
            psPTemp = psP4;
            psP4 = psP2;
            psP2 = psPTemp
        }
        3 => {
            psPTemp = psP1;
            psP1 = psP2;
            psP2 = psP3;
            psP3 = psP4;
            psP4 = psPTemp
        }
        _ => { }
    };
}
/* Establishes whether it's worth trying to render a droid - is it actually on the grid? */
#[no_mangle]
pub unsafe extern "C" fn clipDroid(mut psDroid: *mut DROID) -> BOOL {
    if (*psDroid).x as libc::c_uint >= player.p.x as UDWORD &&
           ((*psDroid).x as libc::c_uint) <
               (player.p.x as
                    UDWORD).wrapping_add(visibleXTiles.wrapping_mul(128 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_uint))
           && (*psDroid).y as libc::c_uint >= player.p.z as UDWORD &&
           ((*psDroid).y as libc::c_uint) <
               (player.p.z as
                    UDWORD).wrapping_add(visibleYTiles.wrapping_mul(128 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_uint))
       {
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
}
/* Clips anything - not necessarily a droid */
#[no_mangle]
pub unsafe extern "C" fn clipXY(mut x: SDWORD, mut y: SDWORD) -> BOOL {
    if x > player.p.x &&
           x <
               (player.p.x as
                    libc::c_uint).wrapping_add(visibleXTiles.wrapping_mul(128
                                                                              as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_uint))
                   as SDWORD && y > player.p.z &&
           y <
               (player.p.z as
                    libc::c_uint).wrapping_add(visibleYTiles.wrapping_mul(128
                                                                              as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_uint))
                   as SDWORD {
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
}
/*	Get the onscreen corrdinates of a Object Position so we can draw a 'button' in
the Intelligence screen.  VERY similar to above function*/
#[no_mangle]
pub unsafe extern "C" fn calcFlagPosScreenCoords(mut pX: *mut SDWORD,
                                                 mut pY: *mut SDWORD,
                                                 mut pR: *mut SDWORD) {
    let mut centX: SDWORD = 0;
    let mut centY: SDWORD = 0;
    let mut centZ: SDWORD = 0;
    let mut cX: SDWORD = 0;
    let mut cY: SDWORD = 0;
    let mut radius: UDWORD = 0;
    /* Ensure correct context */
    /* Get it's absolute dimensions */
    centZ = 0 as libc::c_int;
    centY = centZ;
    centX = centY;
    /* How big a box do we want - will ultimately be calculated using xmax, ymax, zmax etc */
    radius = 22 as libc::c_int as UDWORD;
    /* Pop matrices and get the screen coordinates for last point*/
    pie_RotateProject(centX, centY, centZ, &mut cX, &mut cY);
    /*store the coords*/
    *pX = cX;
    *pY = cY;
    *pR = radius as SDWORD;
}
/* Renders the bullets and their effects in 3D */
#[no_mangle]
pub unsafe extern "C" fn display3DProjectiles() {
    let mut psObj: *mut PROJ_OBJECT = 0 as *mut PROJ_OBJECT; /* end switch */
    psObj = proj_GetFirst();
    while !psObj.is_null() {
        match (*psObj).state as libc::c_int {
            0 => {
                // if source or destination is visible
//			if(   ((psObj->psSource != NULL) && psObj->psSource->visible[selectedPlayer])
//			   || ((psObj->psDest != NULL)   && psObj->psDest->visible[selectedPlayer]  )  )
                if gfxVisible(psObj) != 0 {
                    //			if(GFX_VISIBLE(psObj))
                    /* don't display first frame of trajectory (projectile on firing object) */
                    if gameTime != (*psObj).born {
                        /* Draw a bullet at psObj->x for X coord
										psObj->y for Z coord
										whatever for Y (height) coord - arcing ?
					*/
                        /* these guys get drawn last */
                        if (*(*psObj).psWStats).weaponSubClass as libc::c_uint
                               == WSC_ROCKET as libc::c_int as libc::c_uint ||
                               (*(*psObj).psWStats).weaponSubClass as
                                   libc::c_uint ==
                                   WSC_MISSILE as libc::c_int as libc::c_uint
                               ||
                               (*(*psObj).psWStats).weaponSubClass as
                                   libc::c_uint ==
                                   WSC_COMMAND as libc::c_int as libc::c_uint
                               ||
                               (*(*psObj).psWStats).weaponSubClass as
                                   libc::c_uint ==
                                   WSC_SLOWMISSILE as libc::c_int as
                                       libc::c_uint ||
                               (*(*psObj).psWStats).weaponSubClass as
                                   libc::c_uint ==
                                   WSC_SLOWROCKET as libc::c_int as
                                       libc::c_uint ||
                               (*(*psObj).psWStats).weaponSubClass as
                                   libc::c_uint ==
                                   WSC_ENERGY as libc::c_int as libc::c_uint
                               ||
                               (*(*psObj).psWStats).weaponSubClass as
                                   libc::c_uint ==
                                   WSC_EMP as libc::c_int as libc::c_uint {
                            bucketAddTypeToList(RENDER_PROJECTILE_TRANSPARENT,
                                                psObj as *mut libc::c_void);
                        } else {
                            bucketAddTypeToList(RENDER_PROJECTILE,
                                                psObj as *mut libc::c_void);
                        }
                    }
                }
            }
            1 | 2 | _ => { }
        }
        psObj = proj_GetNext()
    };
}
/* end of function display3DProjectiles */
#[no_mangle]
pub unsafe extern "C" fn renderProjectile(mut psCurr: *mut PROJ_OBJECT) {
    let mut psStats: *mut WEAPON_STATS = 0 as *mut WEAPON_STATS;
    let mut dv: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut pIMD: *mut iIMDShape = 0 as *mut iIMDShape;
    let mut brightness: UDWORD = 0;
    let mut specular: UDWORD = 0;
    //	SDWORD		centreX, centreZ;
    psStats = (*psCurr).psWStats;
    /* Reject flame or command since they have interim drawn fx */
    if (*psStats).weaponSubClass as libc::c_uint ==
           WSC_FLAME as libc::c_int as libc::c_uint ||
           (*psStats).weaponSubClass as libc::c_uint ==
               WSC_COMMAND as libc::c_int as libc::c_uint ||
           (*psStats).weaponSubClass as libc::c_uint ==
               WSC_ELECTRONIC as libc::c_int as libc::c_uint ||
           (*psStats).weaponSubClass as libc::c_uint ==
               WSC_EMP as libc::c_int as libc::c_uint ||
           bMultiPlayer != 0 &&
               (*psStats).weaponSubClass as libc::c_uint ==
                   WSC_LAS_SAT as libc::c_int as libc::c_uint {
        //		OR psStats->weaponSubClass == WSC_ROCKET)
        /* We don't do projectiles from these guys, cos there's an effect instead */
        return
    }
    //the weapon stats holds the reference to which graphic to use
	/*Need to draw the graphic depending on what the projectile is doing - hitting target,
	missing target, in flight etc - JUST DO IN FLIGHT FOR NOW! */
    pIMD = (*psStats).pInFlightGraphic;
    if clipXY((*psCurr).x as SDWORD, (*psCurr).y as SDWORD) != 0 {
        /* Get bullet's x coord */
        dv.x =
            (((*psCurr).x as libc::c_int - player.p.x) as
                 libc::c_uint).wrapping_sub(terrainMidX.wrapping_mul(128 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_uint))
                as int32;
        /* Get it's y coord (z coord in the 3d world */
        dv.z =
            terrainMidY.wrapping_mul(128 as libc::c_int as
                                         libc::c_uint).wrapping_sub(((*psCurr).y
                                                                         as
                                                                         libc::c_int
                                                                         -
                                                                         player.p.z)
                                                                        as
                                                                        libc::c_uint)
                as int32;
        /* What's the present height of the bullet? */
        dv.y = (*psCurr).z as int32;
        /* Set up the matrix */
        pie_MatBegin();
        /* Translate to the correct position */
        pie_TRANSLATE(dv.x, dv.y, dv.z);
        /* Get the x,z translation components */
        rx = player.p.x & 128 as libc::c_int - 1 as libc::c_int;
        rz = player.p.z & 128 as libc::c_int - 1 as libc::c_int;
        /* Translate */
        pie_TRANSLATE(rx, 0 as libc::c_int, -rz);
        /* Rotate it to the direction it's facing */
        imdRot2.y =
            65536 as libc::c_int / 360 as libc::c_int *
                (*psCurr).direction as libc::c_int;
        pie_MatRotY(-imdRot2.y);
        /* pitch it */
        imdRot2.x =
            65536 as libc::c_int / 360 as libc::c_int *
                (*psCurr).pitch as libc::c_int;
        pie_MatRotX(imdRot2.x);
        /* Spin the bullet around - remove later */
//		centreX = ( player.p.x + ((visibleXTiles/2)<<TILE_SHIFT) );
//		centreZ = ( player.p.z + ((visibleYTiles/2)<<TILE_SHIFT) );
        brightness =
            lightDoFogAndIllumination(255 as libc::c_int as UBYTE,
                                      getCentreX() -
                                          (*psCurr).x as libc::c_int,
                                      getCentreZ() -
                                          (*psCurr).y as libc::c_int,
                                      &mut specular);
        if (*psStats).weaponSubClass as libc::c_uint ==
               WSC_ROCKET as libc::c_int as libc::c_uint ||
               (*psStats).weaponSubClass as libc::c_uint ==
                   WSC_MISSILE as libc::c_int as libc::c_uint ||
               (*psStats).weaponSubClass as libc::c_uint ==
                   WSC_SLOWROCKET as libc::c_int as libc::c_uint ||
               (*psStats).weaponSubClass as libc::c_uint ==
                   WSC_SLOWMISSILE as libc::c_int as libc::c_uint {
            pie_Draw3DShape(pIMD, 0 as libc::c_int, 0 as libc::c_int,
                            brightness, 0 as libc::c_int as UDWORD,
                            0x4 as libc::c_int, 164 as libc::c_int);
            //			pie_Draw3DShape(pIMD, 0, 0, brightness, specular, pie_NO_BILINEAR, 0);
        } else {
            pie_Draw3DShape(pIMD, 0 as libc::c_int, 0 as libc::c_int,
                            brightness, specular, 0x8 as libc::c_int,
                            0 as libc::c_int);
        }
        pie_MatEnd();
    };
    /* Flush matrices */
}
#[no_mangle]
pub unsafe extern "C" fn renderAnimComponent(mut psObj:
                                                 *mut COMPONENT_OBJECT) {
    let mut dv: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut posX: SDWORD = 0;
    let mut posY: SDWORD = 0;
    let mut posZ: SDWORD = 0;
    let mut iPlayer: SDWORD = 0;
    let mut psParentObj: *mut BASE_OBJECT =
        (*psObj).psParent as *mut BASE_OBJECT;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut psStructure: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut brightness: UDWORD = 0;
    let mut specular: UDWORD = 0;
    //	SDWORD		centreX, centreZ;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"renderAnimComponent: invalid parent object pointer\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"display3d.c\x00" as *const u8 as *const libc::c_char,
              1400 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 20],
                                        &[libc::c_char; 20]>(b"renderAnimComponent\x00")).as_ptr(),
              b"PTRVALID(psParentObj, sizeof(SIMPLE_OBJECT))\x00" as *const u8
                  as *const libc::c_char);
    };
    /* only draw visible bits */
    if (*psParentObj).type_0 as libc::c_uint ==
           OBJ_DROID as libc::c_int as libc::c_uint && godMode == 0 &&
           demoGetStatus() == 0 {
        if (*(psParentObj as *mut DROID)).visible[selectedPlayer as usize] as
               libc::c_int != 0xff as libc::c_int {
            return
        }
    }
    posX = (*psParentObj).x as libc::c_int + (*psObj).position.x;
    posY = (*psParentObj).y as libc::c_int + (*psObj).position.y;
    posZ = (*psParentObj).z as libc::c_int + (*psObj).position.z;
    //	centreX = ( player.p.x + ((visibleXTiles/2)<<TILE_SHIFT) );
//	centreZ = ( player.p.z + ((visibleYTiles/2)<<TILE_SHIFT) );
    /* render */
    if clipXY(posX, posY) != 0 {
        (*psParentObj).sDisplay.frameNumber = currentGameFrame;
        /* Push the indentity matrix */
        pie_MatBegin();
        /* get parent object translation */
        dv.x =
            (((*psParentObj).x as libc::c_int - player.p.x) as
                 libc::c_uint).wrapping_sub(terrainMidX.wrapping_mul(128 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_uint))
                as int32;
        dv.z =
            terrainMidY.wrapping_mul(128 as libc::c_int as
                                         libc::c_uint).wrapping_sub(((*psParentObj).y
                                                                         as
                                                                         libc::c_int
                                                                         -
                                                                         player.p.z)
                                                                        as
                                                                        libc::c_uint)
                as int32;
        dv.y = (*psParentObj).z as int32;
        /* parent object translation */
        pie_TRANSLATE(dv.x, dv.y, dv.z);
        /* Get the x,z translation components */
        rx = player.p.x & 128 as libc::c_int - 1 as libc::c_int;
        rz = player.p.z & 128 as libc::c_int - 1 as libc::c_int;
        /* Translate */
        pie_TRANSLATE(rx, 0 as libc::c_int, -rz);
        /* parent object rotations */
        imdRot2.y =
            65536 as libc::c_int / 360 as libc::c_int *
                (*psParentObj).direction as libc::c_int;
        pie_MatRotY(-imdRot2.y);
        imdRot2.x =
            65536 as libc::c_int / 360 as libc::c_int *
                (*psParentObj).pitch as libc::c_int;
        pie_MatRotX(imdRot2.x);
        /* object (animation) translations - ivis z and y flipped */
        pie_TRANSLATE((*psObj).position.x, (*psObj).position.z,
                      (*psObj).position.y);
        /* object (animation) rotations */
        pie_MatRotY(-(*psObj).orientation.z);
        pie_MatRotZ(-(*psObj).orientation.y);
        pie_MatRotX(-(*psObj).orientation.x);
        /* Set frame numbers - look into this later?? FIXME!!!!!!!! */
        if (*psParentObj).type_0 as libc::c_uint ==
               OBJ_DROID as libc::c_int as libc::c_uint {
            psDroid = psParentObj as *mut DROID;
            if (*psDroid).droidType as libc::c_uint ==
                   DROID_PERSON as libc::c_int as libc::c_uint {
                iPlayer =
                    (*psParentObj).player as libc::c_int - 6 as libc::c_int;
                pie_MatScale(75 as libc::c_int as UDWORD);
            } else {
                iPlayer =
                    getPlayerColour((*psParentObj).player as UDWORD) as SDWORD
            }
            /* Get the onscreen coordinates so we can draw a bounding box */
            calcScreenCoords(psDroid);
            targetAdd(psDroid as *mut BASE_OBJECT);
        } else {
            iPlayer =
                getPlayerColour((*psParentObj).player as UDWORD) as SDWORD
        }
        //brightness and fog calculation
        if (*psParentObj).type_0 as libc::c_uint ==
               OBJ_STRUCTURE as libc::c_int as libc::c_uint {
            psStructure = psParentObj as *mut STRUCTURE;
            brightness =
                (200 as libc::c_int as
                     libc::c_uint).wrapping_sub((100 as libc::c_int as
                                                     libc::c_uint).wrapping_sub((((*psStructure).body
                                                                                      as
                                                                                      libc::c_int
                                                                                      *
                                                                                      100
                                                                                          as
                                                                                          libc::c_int)
                                                                                     as
                                                                                     libc::c_uint).wrapping_div(structureBody(psStructure))));
            let mut sX: SDWORD = 0;
            let mut sY: SDWORD = 0;
            pie_RotateProject(0 as libc::c_int, 0 as libc::c_int,
                              0 as libc::c_int, &mut sX, &mut sY);
            (*psStructure).sDisplay.screenX = sX as UDWORD;
            (*psStructure).sDisplay.screenY = sY as UDWORD;
            targetAdd(psStructure as *mut BASE_OBJECT);
        } else { brightness = 255 as libc::c_int as UDWORD }
        if getRevealStatus() != 0 && godMode == 0 {
            brightness = avGetObjLightLevel(psParentObj, brightness)
        }
        brightness =
            lightDoFogAndIllumination(brightness as UBYTE,
                                      getCentreX() - posX,
                                      getCentreZ() - posY, &mut specular);
        pie_Draw3DShape((*psObj).psShape, 0 as libc::c_int, iPlayer,
                        brightness, specular, 0x8 as libc::c_int,
                        0 as libc::c_int);
        /* clear stack */
        pie_MatEnd();
    };
}
/*	Renders ONE terrain tile and any droids, structures, features that are on it. Sorts the objects though, so's there
	only drawn once and drawn at the appropriate time so tles aren't drawn over them */
#[no_mangle]
pub unsafe extern "C" fn drawTexturedTile(mut i: UDWORD, mut j: UDWORD) {
    let mut tileNumber: UDWORD = 0;
    let mut renderFlag: UDWORD = 0;
    //UDWORD	n;
    let mut p: [iVertex; 4] =
        [iVertex{x: 0, y: 0, z: 0, u: 0, v: 0, g: 0,}; 4];
    //iVertex clip[iV_POLY_MAX_POINTS];
    let mut psTile: *mut MAPTILE = 0 as *mut MAPTILE;
    let mut tileOutlined: BOOL = 0 as libc::c_int;
    let mut realX: UDWORD = 0;
    let mut realY: UDWORD = 0;
    let mut topL: UDWORD = 0;
    let mut botL: UDWORD = 0;
    let mut topR: UDWORD = 0;
    let mut botR: UDWORD = 0;
    let mut offset: iPoint = iPoint{x: 0, y: 0,};
    let mut bEdgeTile: BOOL = 0;
    bEdgeTile = 0 as libc::c_int;
    /* Get the actual tile to render */
    realX = (playerXTile as libc::c_uint).wrapping_add(j);
    realY = (playerZTile as libc::c_uint).wrapping_add(i);
    /* Get a pointer to the tile we're going to render */
    if realX < 0 as libc::c_int as libc::c_uint ||
           realY < 0 as libc::c_int as libc::c_uint ||
           realX > mapWidth.wrapping_sub(2 as libc::c_int as libc::c_uint) ||
           realY > mapHeight.wrapping_sub(2 as libc::c_int as libc::c_uint) {
        psTile = &mut edgeTile;
        bEdgeTile = 1 as libc::c_int;
        (*psTile).texture =
            ((*psTile).texture as libc::c_int & !(0x400 as libc::c_int)) as
                UWORD
    } else { psTile = mapTile(realX, realY) }
    if (*psTile).tileInfoBits as libc::c_int & 0x4 as libc::c_int != 0 {
        /* Bomb out if we're not supposed to draw this tile! */
        return
    }
    // tiles are always visible for now - john.
//	if ( TEST_TILE_VISIBLE(selectedPlayer, psTile) OR godMode)
    numTiles = numTiles.wrapping_add(1);
    if bEdgeTile != 0 {
        topR = 0 as libc::c_int as UDWORD;
        botR = topR;
        botL = botR;
        topL = botL
    } else {
        //penumbra
        topL =
            tileScreenInfo[i.wrapping_add(0 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(0 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].light.argb;
        botL =
            tileScreenInfo[i.wrapping_add(1 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(0 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].light.argb;
        botR =
            tileScreenInfo[i.wrapping_add(1 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(1 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].light.argb;
        topR =
            tileScreenInfo[i.wrapping_add(0 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(1 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].light.argb
        //no penumbra
				/*
				topL = mapTile(realX,realY)->illumination;
				botL = mapTile(realX,realY+1)->illumination;
				botR = mapTile(realX+1,realY+1)->illumination;
				topR = mapTile(realX+1,realY)->illumination;
				*/
    }
    /* get the appropriate tile texture */
    if (*psTile).texture as libc::c_int & 0x400 as libc::c_int != 0 {
        (*psTile).texture =
            ((*psTile).texture as libc::c_int & !(0x400 as libc::c_int)) as
                UWORD;
        //tileNumber = psTile->texture;
        tileNumber = 22 as libc::c_int as UDWORD;
        tileOutlined = 1 as libc::c_int
    } else { tileNumber = (*psTile).texture as UDWORD }
    pie_SetTexturePage(tileTexInfo[(tileNumber &
                                        0x1ff as libc::c_int as libc::c_uint)
                                       as usize].texPage as SDWORD);
    //temp
//			pie_DrawTile(&tileScreenInfo[0][0],&tileScreenInfo[0][1],&tileScreenInfo[1][0],
//				&tileScreenInfo[1][1],tileTexInfo[tileNumber & 0xff].texPage);
//temp
    /* Check for flipped and rotated tiles */
    flipsAndRots((tileNumber & !(0x1ff as libc::c_int) as libc::c_uint) as
                     libc::c_int);
    if (*psTile).texture as libc::c_int & 0x800 as libc::c_int != 0 {
        /* Get the screen coordinates to render into for the texturer */
        p[0 as libc::c_int as usize].x =
            tileScreenInfo[i.wrapping_add(0 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(0 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].sx;
        p[0 as libc::c_int as usize].y =
            tileScreenInfo[i.wrapping_add(0 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(0 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].sy;
        p[0 as libc::c_int as usize].z =
            tileScreenInfo[i.wrapping_add(0 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(0 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].sz;
        p[1 as libc::c_int as usize].x =
            tileScreenInfo[i.wrapping_add(0 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(1 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].sx;
        p[1 as libc::c_int as usize].y =
            tileScreenInfo[i.wrapping_add(0 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(1 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].sy;
        p[1 as libc::c_int as usize].z =
            tileScreenInfo[i.wrapping_add(0 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(1 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].sz;
        p[2 as libc::c_int as usize].x =
            tileScreenInfo[i.wrapping_add(1 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(0 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].sx;
        p[2 as libc::c_int as usize].y =
            tileScreenInfo[i.wrapping_add(1 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(0 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].sy;
        p[2 as libc::c_int as usize].z =
            tileScreenInfo[i.wrapping_add(1 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(0 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].sz;
        /* Get the U,V values for the indexing into the texture */
        p[0 as libc::c_int as usize].u = (*psP1).x;
        p[0 as libc::c_int as usize].v = (*psP1).y;
        p[1 as libc::c_int as usize].u = (*psP2).x;
        p[1 as libc::c_int as usize].v = (*psP2).y;
        p[2 as libc::c_int as usize].u = (*psP4).x;
        p[2 as libc::c_int as usize].v = (*psP4).y;
        /* Get the intensity values	for shading */
        p[0 as libc::c_int as usize].g = topL as UBYTE;
        p[1 as libc::c_int as usize].g = topR as UBYTE;
        p[2 as libc::c_int as usize].g = botL as UBYTE
    } else {
        /* Get the screen coordinates to render into for the texturer */
        p[0 as libc::c_int as usize].x =
            tileScreenInfo[i.wrapping_add(0 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(0 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].sx;
        p[0 as libc::c_int as usize].y =
            tileScreenInfo[i.wrapping_add(0 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(0 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].sy;
        p[0 as libc::c_int as usize].z =
            tileScreenInfo[i.wrapping_add(0 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(0 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].sz;
        p[1 as libc::c_int as usize].x =
            tileScreenInfo[i.wrapping_add(0 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(1 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].sx;
        p[1 as libc::c_int as usize].y =
            tileScreenInfo[i.wrapping_add(0 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(1 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].sy;
        p[1 as libc::c_int as usize].z =
            tileScreenInfo[i.wrapping_add(0 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(1 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].sz;
        p[2 as libc::c_int as usize].x =
            tileScreenInfo[i.wrapping_add(1 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(1 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].sx;
        p[2 as libc::c_int as usize].y =
            tileScreenInfo[i.wrapping_add(1 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(1 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].sy;
        p[2 as libc::c_int as usize].z =
            tileScreenInfo[i.wrapping_add(1 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(1 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].sz;
        /* Get the U,V values for the indexing into the texture */
        p[0 as libc::c_int as usize].u = (*psP1).x;
        p[0 as libc::c_int as usize].v = (*psP1).y;
        p[1 as libc::c_int as usize].u = (*psP2).x;
        p[1 as libc::c_int as usize].v = (*psP2).y;
        p[2 as libc::c_int as usize].u = (*psP3).x;
        p[2 as libc::c_int as usize].v = (*psP3).y;
        /* Get the intensity values	for shading */
        p[0 as libc::c_int as usize].g = topL as UBYTE;
        p[1 as libc::c_int as usize].g = topR as UBYTE;
        p[2 as libc::c_int as usize].g = botR as UBYTE
    }
    renderFlag = 0 as libc::c_int as UDWORD;
    pie_DrawTriangle(p.as_mut_ptr(), &mut texturePage, renderFlag,
                     &mut offset);
    if (*psTile).texture as libc::c_int & 0x800 as libc::c_int != 0 {
        /* Set up the texel coordinates */
        p[0 as libc::c_int as usize].x =
            tileScreenInfo[i.wrapping_add(0 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(1 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].sx;
        p[0 as libc::c_int as usize].y =
            tileScreenInfo[i.wrapping_add(0 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(1 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].sy;
        p[0 as libc::c_int as usize].z =
            tileScreenInfo[i.wrapping_add(0 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(1 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].sz;
        p[1 as libc::c_int as usize].x =
            tileScreenInfo[i.wrapping_add(1 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(1 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].sx;
        p[1 as libc::c_int as usize].y =
            tileScreenInfo[i.wrapping_add(1 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(1 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].sy;
        p[1 as libc::c_int as usize].z =
            tileScreenInfo[i.wrapping_add(1 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(1 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].sz;
        p[2 as libc::c_int as usize].x =
            tileScreenInfo[i.wrapping_add(1 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(0 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].sx;
        p[2 as libc::c_int as usize].y =
            tileScreenInfo[i.wrapping_add(1 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(0 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].sy;
        p[2 as libc::c_int as usize].z =
            tileScreenInfo[i.wrapping_add(1 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(0 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].sz;
        /* Set up U,V */
        p[0 as libc::c_int as usize].u = (*psP2).x;
        p[0 as libc::c_int as usize].v = (*psP2).y;
        p[1 as libc::c_int as usize].u = (*psP3).x;
        p[1 as libc::c_int as usize].v = (*psP3).y;
        p[2 as libc::c_int as usize].u = (*psP4).x;
        p[2 as libc::c_int as usize].v = (*psP4).y;
        /* Set up shading vars */
        p[0 as libc::c_int as usize].g = topR as UBYTE;
        p[1 as libc::c_int as usize].g = botR as UBYTE;
        p[2 as libc::c_int as usize].g = botL as UBYTE
    } else {
        /* Set up the texel coordinates */
        p[0 as libc::c_int as usize].x =
            tileScreenInfo[i.wrapping_add(0 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(0 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].sx;
        p[0 as libc::c_int as usize].y =
            tileScreenInfo[i.wrapping_add(0 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(0 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].sy;
        p[0 as libc::c_int as usize].z =
            tileScreenInfo[i.wrapping_add(0 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(0 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].sz;
        p[1 as libc::c_int as usize].x =
            tileScreenInfo[i.wrapping_add(1 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(1 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].sx;
        p[1 as libc::c_int as usize].y =
            tileScreenInfo[i.wrapping_add(1 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(1 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].sy;
        p[1 as libc::c_int as usize].z =
            tileScreenInfo[i.wrapping_add(1 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(1 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].sz;
        p[2 as libc::c_int as usize].x =
            tileScreenInfo[i.wrapping_add(1 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(0 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].sx;
        p[2 as libc::c_int as usize].y =
            tileScreenInfo[i.wrapping_add(1 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(0 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].sy;
        p[2 as libc::c_int as usize].z =
            tileScreenInfo[i.wrapping_add(1 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(0 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].sz;
        /* Set up U,V */
        p[0 as libc::c_int as usize].u = (*psP1).x;
        p[0 as libc::c_int as usize].v = (*psP1).y;
        p[1 as libc::c_int as usize].u = (*psP3).x;
        p[1 as libc::c_int as usize].v = (*psP3).y;
        p[2 as libc::c_int as usize].u = (*psP4).x;
        p[2 as libc::c_int as usize].v = (*psP4).y;
        /* Set up shading vars */
        p[0 as libc::c_int as usize].g = topL as UBYTE;
        p[1 as libc::c_int as usize].g = botR as UBYTE;
        p[2 as libc::c_int as usize].g = botL as UBYTE
    }
    pie_DrawTriangle(p.as_mut_ptr(), &mut texturePage, renderFlag,
                     &mut offset);
    // end tile-draw
    // -------------------------------------------------------------------------
    if terrainOutline != 0 || tileOutlined != 0 {
        /*iV_Line(tileScreenInfo[i+0][j+0].sx,tileScreenInfo[i+0][j+0].sy,
    					tileScreenInfo[i+0][j+1].sx,tileScreenInfo[i+0][j+1].sy,255);
    				iV_Line(tileScreenInfo[i+0][j+1].sx,tileScreenInfo[i+0][j+1].sy,
    					tileScreenInfo[i+1][j+1].sx,tileScreenInfo[i+1][j+1].sy,255);
    				iV_Line(tileScreenInfo[i+1][j+1].sx,tileScreenInfo[i+1][j+1].sy,
    					tileScreenInfo[i+1][j+0].sx,tileScreenInfo[i+1][j+0].sy,255);
    				iV_Line(tileScreenInfo[i+1][j+0].sx,tileScreenInfo[i+1][j+0].sy,
    					tileScreenInfo[i+0][j+0].sx,tileScreenInfo[i+0][j+0].sy,255);*/
        pie_Line(tileScreenInfo[i.wrapping_add(0 as libc::c_int as
                                                   libc::c_uint) as
                                    usize][j.wrapping_add(0 as libc::c_int as
                                                              libc::c_uint) as
                                               usize].sx,
                 tileScreenInfo[i.wrapping_add(0 as libc::c_int as
                                                   libc::c_uint) as
                                    usize][j.wrapping_add(0 as libc::c_int as
                                                              libc::c_uint) as
                                               usize].sy,
                 tileScreenInfo[i.wrapping_add(0 as libc::c_int as
                                                   libc::c_uint) as
                                    usize][j.wrapping_add(1 as libc::c_int as
                                                              libc::c_uint) as
                                               usize].sx,
                 tileScreenInfo[i.wrapping_add(0 as libc::c_int as
                                                   libc::c_uint) as
                                    usize][j.wrapping_add(1 as libc::c_int as
                                                              libc::c_uint) as
                                               usize].sy, outlineColour3D);
        pie_Line(tileScreenInfo[i.wrapping_add(0 as libc::c_int as
                                                   libc::c_uint) as
                                    usize][j.wrapping_add(1 as libc::c_int as
                                                              libc::c_uint) as
                                               usize].sx,
                 tileScreenInfo[i.wrapping_add(0 as libc::c_int as
                                                   libc::c_uint) as
                                    usize][j.wrapping_add(1 as libc::c_int as
                                                              libc::c_uint) as
                                               usize].sy,
                 tileScreenInfo[i.wrapping_add(1 as libc::c_int as
                                                   libc::c_uint) as
                                    usize][j.wrapping_add(1 as libc::c_int as
                                                              libc::c_uint) as
                                               usize].sx,
                 tileScreenInfo[i.wrapping_add(1 as libc::c_int as
                                                   libc::c_uint) as
                                    usize][j.wrapping_add(1 as libc::c_int as
                                                              libc::c_uint) as
                                               usize].sy, outlineColour3D);
        pie_Line(tileScreenInfo[i.wrapping_add(1 as libc::c_int as
                                                   libc::c_uint) as
                                    usize][j.wrapping_add(1 as libc::c_int as
                                                              libc::c_uint) as
                                               usize].sx,
                 tileScreenInfo[i.wrapping_add(1 as libc::c_int as
                                                   libc::c_uint) as
                                    usize][j.wrapping_add(1 as libc::c_int as
                                                              libc::c_uint) as
                                               usize].sy,
                 tileScreenInfo[i.wrapping_add(1 as libc::c_int as
                                                   libc::c_uint) as
                                    usize][j.wrapping_add(0 as libc::c_int as
                                                              libc::c_uint) as
                                               usize].sx,
                 tileScreenInfo[i.wrapping_add(1 as libc::c_int as
                                                   libc::c_uint) as
                                    usize][j.wrapping_add(0 as libc::c_int as
                                                              libc::c_uint) as
                                               usize].sy, outlineColour3D);
        pie_Line(tileScreenInfo[i.wrapping_add(1 as libc::c_int as
                                                   libc::c_uint) as
                                    usize][j.wrapping_add(0 as libc::c_int as
                                                              libc::c_uint) as
                                               usize].sx,
                 tileScreenInfo[i.wrapping_add(1 as libc::c_int as
                                                   libc::c_uint) as
                                    usize][j.wrapping_add(0 as libc::c_int as
                                                              libc::c_uint) as
                                               usize].sy,
                 tileScreenInfo[i.wrapping_add(0 as libc::c_int as
                                                   libc::c_uint) as
                                    usize][j.wrapping_add(0 as libc::c_int as
                                                              libc::c_uint) as
                                               usize].sx,
                 tileScreenInfo[i.wrapping_add(0 as libc::c_int as
                                                   libc::c_uint) as
                                    usize][j.wrapping_add(0 as libc::c_int as
                                                              libc::c_uint) as
                                               usize].sy, outlineColour3D);
        /*
   						if(TRI_FLIPPED(psTile))
						{
							iV_Line(tileScreenInfo[i+0][j+0].x,tileScreenInfo[i+0][j+0].y,
    							tileScreenInfo[i+1][j+1].x,tileScreenInfo[i+1][j+1].y,255);
						}
						else
						{
							iV_Line(tileScreenInfo[i+0][j+1].x,tileScreenInfo[i+0][j+1].y,
    						tileScreenInfo[i+1][j+0].x,tileScreenInfo[i+1][j+0].y,255);
						}
*/
    };
}
/* Draw the buildings */
#[no_mangle]
pub unsafe extern "C" fn displayStaticObjects() {
    let mut psStructure: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut clan: UDWORD = 0;
    let mut test: UDWORD = 0 as libc::c_int as UDWORD;
    let mut psAnimObj: *mut ANIM_OBJECT = 0 as *mut ANIM_OBJECT;
    /* Go through all the players */
    clan = 0 as libc::c_int as UDWORD;
    while clan < 8 as libc::c_int as libc::c_uint {
        /* Now go all buildings for that player */
        psStructure = apsStructLists[clan as usize];
        while !psStructure.is_null() {
            test = test.wrapping_add(1);
            /* Worth rendering the structure? */
            if clipXY((*psStructure).x as SDWORD, (*psStructure).y as SDWORD)
                   != 0 {
                //don't use #ifndef BUCKET for now - need to do it this way for renderMapToBuffer
                if doBucket == 0 {
                    //over-ride the BUCKET def
                    renderStructure(psStructure);
                } else {
                    if (*(*psStructure).pStructureType).type_0 ==
                           REF_RESOURCE_EXTRACTOR as libc::c_int as
                               libc::c_uint &&
                           (*psStructure).psCurAnim.is_null() &&
                           (*psStructure).currentBuildPts as libc::c_int >
                               (*(*psStructure).pStructureType).buildPoints as
                                   SDWORD {
                        (*psStructure).psCurAnim =
                            animObj_Add(psStructure as *mut libc::c_void,
                                        5 as libc::c_int,
                                        0 as libc::c_int as UDWORD,
                                        0 as libc::c_int as UWORD)
                    }
                    if (*psStructure).psCurAnim.is_null() ||
                           (*(*psStructure).psCurAnim).bVisible ==
                               0 as libc::c_int ||
                           {
                               psAnimObj =
                                   animObj_Find(psStructure as
                                                    *mut libc::c_void,
                                                (*(*psStructure).psCurAnim).uwID
                                                    as libc::c_int);
                               psAnimObj.is_null()
                           } {
                        bucketAddTypeToList(RENDER_STRUCTURE,
                                            psStructure as *mut libc::c_void);
                    } else if (*psStructure).visible[selectedPlayer as usize]
                                  as libc::c_int != 0 || godMode != 0 {
                        //check not a resource extractors
                        if (*(*psStructure).pStructureType).type_0 !=
                               REF_RESOURCE_EXTRACTOR as libc::c_int as
                                   libc::c_uint {
                            displayAnimation(psAnimObj, 0 as libc::c_int);
                        } else if (*((*psStructure).pFunctionality as
                                         *mut RES_EXTRACTOR)).active != 0 {
                            displayAnimation(psAnimObj, 0 as libc::c_int);
                            if selectedPlayer ==
                                   (*psStructure).player as libc::c_uint {
                                audio_PlayObjStaticTrack(psStructure as
                                                             *mut libc::c_void,
                                                         ID_SOUND_OIL_PUMP_2
                                                             as libc::c_int);
                            }
                        } else {
                            //check that a power gen exists before animationg res extrac
							//else if (getPowerGenExists(psStructure->player))
							/*check the building is active*/
                            /* hold anim on first frame */
                            displayAnimation(psAnimObj, 1 as libc::c_int);
                            audio_StopObjTrack(psStructure as
                                                   *mut libc::c_void,
                                               ID_SOUND_OIL_PUMP_2 as
                                                   libc::c_int);
                        }
                    }
                }
            }
            psStructure = (*psStructure).psNext
        }
        clan = clan.wrapping_add(1)
    };
}
//draw Factory Delivery Points
#[no_mangle]
pub unsafe extern "C" fn displayDelivPoints() {
    let mut psDelivPoint: *mut FLAG_POSITION = 0 as *mut FLAG_POSITION;
    //only do the selected players'
	/* go through all DPs for that player */
    psDelivPoint = apsFlagPosLists[selectedPlayer as usize];
    while !psDelivPoint.is_null() {
        if clipXY((*psDelivPoint).coords.x, (*psDelivPoint).coords.y) != 0 {
            if doBucket == 0 {
                renderDeliveryPoint(psDelivPoint);
            } else {
                bucketAddTypeToList(RENDER_DELIVPOINT,
                                    psDelivPoint as *mut libc::c_void);
            }
        }
        psDelivPoint = (*psDelivPoint).psNext
    };
}
/* Draw the features */
#[no_mangle]
pub unsafe extern "C" fn displayFeatures() {
    let mut psFeature: *mut FEATURE = 0 as *mut FEATURE;
    let mut clan: UDWORD = 0;
    /* player can only be 0 for the features */
    clan = 0 as libc::c_int as UDWORD;
    /* Go through all the features */
    psFeature = apsFeatureLists[clan as usize];
    while !psFeature.is_null() {
        /* Is the feature worth rendering? */
        if clipXY((*psFeature).x as SDWORD, (*psFeature).y as SDWORD) != 0 {
            //don't use #ifndef BUCKET for now - need to do it this way for renderMapToBuffer
            if doBucket == 0 {
                //over-ride the BUCKET def
                renderFeature(psFeature);
            } else {
                bucketAddTypeToList(RENDER_FEATURE,
                                    psFeature as *mut libc::c_void);
            }
        }
        psFeature = (*psFeature).psNext
    };
}
/* Draw the Proximity messages for the **SELECTED PLAYER ONLY***/
#[no_mangle]
pub unsafe extern "C" fn displayProximityMsgs() {
    let mut psProxDisp: *mut PROXIMITY_DISPLAY = 0 as *mut PROXIMITY_DISPLAY;
    let mut pViewProximity: *mut VIEW_PROXIMITY = 0 as *mut VIEW_PROXIMITY;
    let mut x: UDWORD = 0;
    let mut y: UDWORD = 0;
    /* Go through all the proximity Displays*/
    psProxDisp = apsProxDisp[selectedPlayer as usize];
    while !psProxDisp.is_null() {
        if ((*(*psProxDisp).psMessage).read as *mut VIEW_PROXIMITY).is_null()
           {
            if (*psProxDisp).type_0 as libc::c_uint ==
                   POS_PROXDATA as libc::c_int as libc::c_uint {
                pViewProximity =
                    (*((*(*psProxDisp).psMessage).pViewData as
                           *mut VIEWDATA)).pData as *mut VIEW_PROXIMITY;
                x = (*pViewProximity).x;
                y = (*pViewProximity).y
            } else {
                x =
                    (*((*(*psProxDisp).psMessage).pViewData as
                           *mut BASE_OBJECT)).x as UDWORD;
                y =
                    (*((*(*psProxDisp).psMessage).pViewData as
                           *mut BASE_OBJECT)).y as UDWORD
            }
            /* Is the Message worth rendering? */
			//if(clipXY(pViewProximity->x,pViewProximity->y))
            if clipXY(x as SDWORD, y as SDWORD) != 0 {
                //don't use #ifndef BUCKET for now - need to do it this way for renderMapToBuffer
                if doBucket == 0 {
                    //over-ride the BUCKET def
                    renderProximityMsg(psProxDisp);
                } else {
                    bucketAddTypeToList(RENDER_PROXMSG,
                                        psProxDisp as *mut libc::c_void);
                }
            }
        }
        psProxDisp = (*psProxDisp).psNext
    };
}
#[no_mangle]
pub unsafe extern "C" fn displayAnimation(mut psAnimObj: *mut ANIM_OBJECT,
                                          mut bHoldOnFirstFrame: BOOL) {
    let mut i: UWORD = 0;
    let mut uwFrame: UWORD = 0;
    let mut vecPos: VECTOR3D = VECTOR3D{x: 0, y: 0, z: 0,};
    let mut vecRot: VECTOR3D = VECTOR3D{x: 0, y: 0, z: 0,};
    let mut vecScale: VECTOR3D = VECTOR3D{x: 0, y: 0, z: 0,};
    let mut psComp: *mut COMPONENT_OBJECT = 0 as *mut COMPONENT_OBJECT;
    i = 0 as libc::c_int as UWORD;
    while (i as libc::c_int) < (*(*psAnimObj).psAnim).uwObj as libc::c_int {
        if bHoldOnFirstFrame == 1 as libc::c_int {
            uwFrame = 0 as libc::c_int as UWORD;
            vecPos.z = 0 as libc::c_int;
            vecPos.y = vecPos.z;
            vecPos.x = vecPos.y;
            vecRot.z = 0 as libc::c_int;
            vecRot.y = vecRot.z;
            vecRot.x = vecRot.y;
            vecScale.z = 0 as libc::c_int;
            vecScale.y = vecScale.z;
            vecScale.x = vecScale.y
        } else {
            uwFrame =
                animObj_GetFrame3D(psAnimObj, i, &mut vecPos, &mut vecRot,
                                   &mut vecScale)
        }
        if uwFrame as libc::c_int != 0xfffe as libc::c_int {
            if (*(*psAnimObj).psAnim).animType as libc::c_int ==
                   ANIM_3D_TRANS as libc::c_int {
                psComp =
                    &mut *(*psAnimObj).apComponents.as_mut_ptr().offset(i as
                                                                            isize)
                        as *mut COMPONENT_OBJECT
            } else {
                psComp =
                    &mut *(*psAnimObj).apComponents.as_mut_ptr().offset(uwFrame
                                                                            as
                                                                            isize)
                        as *mut COMPONENT_OBJECT
            }
            (*psComp).position.x = vecPos.x;
            (*psComp).position.y = vecPos.y;
            (*psComp).position.z = vecPos.z;
            (*psComp).orientation.x = vecRot.x;
            (*psComp).orientation.y = vecRot.y;
            (*psComp).orientation.z = vecRot.z;
            bucketAddTypeToList(RENDER_ANIMATION,
                                psComp as *mut libc::c_void);
        }
        i = i.wrapping_add(1)
    };
}
/* Draw the droids */
#[no_mangle]
pub unsafe extern "C" fn displayDynamicObjects() {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut psAnimObj: *mut ANIM_OBJECT = 0 as *mut ANIM_OBJECT;
    let mut clan: UDWORD = 0;
    /* Need to go through all the droid lists */
    clan = 0 as libc::c_int as UDWORD;
    while clan < 8 as libc::c_int as libc::c_uint {
        psDroid = apsDroidLists[clan as usize];
        while !psDroid.is_null() {
            /* Find out whether the droid is worth rendering */
            if clipXY((*psDroid).x as SDWORD, (*psDroid).y as SDWORD) != 0 {
                /* No point in adding it if you can't see it? */
                if (*psDroid).visible[selectedPlayer as usize] as libc::c_int
                       != 0 || godMode != 0 || demoGetStatus() != 0 {
                    (*psDroid).sDisplay.frameNumber = currentGameFrame;
                    //showDroidSelection(psDroid);
                    if doBucket == 0 {
                        //don't use #ifndef BUCKET for now - need to do it this way for renderMapToBuffer
                        //over-ride the BUCKET def
                        renderDroid(psDroid);
                    } else {
                        /* draw droid even if animating (still need to draw weapons) */
                        bucketAddTypeToList(RENDER_DROID,
                                            psDroid as *mut libc::c_void);
                        //							bucketAddTypeToList(RENDER_SHADOW, psDroid);
                        /* draw anim if visible */
                        if !(*psDroid).psCurAnim.is_null() &&
                               (*(*psDroid).psCurAnim).bVisible ==
                                   1 as libc::c_int &&
                               {
                                   psAnimObj =
                                       animObj_Find(psDroid as
                                                        *mut libc::c_void,
                                                    (*(*psDroid).psCurAnim).uwID
                                                        as libc::c_int);
                                   !psAnimObj.is_null()
                               } {
                            displayAnimation(psAnimObj, 0 as libc::c_int);
                        }
                    }
                }
            }
            psDroid = (*psDroid).psNext
            // end clipDroid
        }
        clan = clan.wrapping_add(1)
        // end for
    };
    // end for clan
}
// end Fn
/* Sets the player's position and view angle - defaults player rotations as well */
#[no_mangle]
pub unsafe extern "C" fn setViewPos(mut x: UDWORD, mut y: UDWORD,
                                    mut Pan: BOOL) {
    //BOOL	changed = FALSE;
    let mut midX: SDWORD = 0;
    let mut midY: SDWORD = 0;
    /* Find centre of grid thats actually DRAWN */
    midX =
        x.wrapping_sub(visibleXTiles.wrapping_div(2 as libc::c_int as
                                                      libc::c_uint)) as
            SDWORD;
    midY =
        y.wrapping_sub(visibleYTiles.wrapping_div(2 as libc::c_int as
                                                      libc::c_uint)) as
            SDWORD;
    player.p.x = midX * 128 as libc::c_int;
    player.p.z = midY * 128 as libc::c_int;
    player.r.z = 0 as libc::c_int;
    if getWarCamStatus() != 0 { camToggleStatus(); }
    SetRadarStrobe(midX as UDWORD, midY as UDWORD);
    scroll();
}
#[no_mangle]
pub unsafe extern "C" fn getPlayerPos(mut px: *mut SDWORD,
                                      mut py: *mut SDWORD) {
    *px =
        (player.p.x as
             libc::c_uint).wrapping_add(visibleXTiles.wrapping_div(2 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint).wrapping_mul(128
                                                                                                      as
                                                                                                      libc::c_int
                                                                                                      as
                                                                                                      libc::c_uint))
            as SDWORD;
    *py =
        (player.p.z as
             libc::c_uint).wrapping_add(visibleYTiles.wrapping_div(2 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint).wrapping_mul(128
                                                                                                      as
                                                                                                      libc::c_int
                                                                                                      as
                                                                                                      libc::c_uint))
            as SDWORD;
}
#[no_mangle]
pub unsafe extern "C" fn setPlayerPos(mut x: SDWORD, mut y: SDWORD) {
    let mut midX: SDWORD = 0;
    let mut midY: SDWORD = 0;
    if x > 0 as libc::c_int &&
           x <
               mapWidth.wrapping_mul(128 as libc::c_int as libc::c_uint) as
                   SDWORD && y > 0 as libc::c_int &&
           y <
               mapHeight.wrapping_mul(128 as libc::c_int as libc::c_uint) as
                   SDWORD {
    } else {
        debug(LOG_ERROR,
              b"setPlayerPos: position off map\x00" as *const u8 as
                  *const libc::c_char);
    };
    if x > 0 as libc::c_int &&
           x <
               mapWidth.wrapping_mul(128 as libc::c_int as libc::c_uint) as
                   SDWORD && y > 0 as libc::c_int &&
           y <
               mapHeight.wrapping_mul(128 as libc::c_int as libc::c_uint) as
                   SDWORD {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"display3d.c\x00" as *const u8 as *const libc::c_char,
              2027 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 13],
                                        &[libc::c_char; 13]>(b"setPlayerPos\x00")).as_ptr(),
              b"(x > 0) && (x < (SDWORD)(mapWidth*TILE_UNITS)) && (y > 0) && (y < (SDWORD)(mapHeight*TILE_UNITS))\x00"
                  as *const u8 as *const libc::c_char);
    };
    // Find centre of grid thats actually DRAWN
    midX =
        ((x >> 7 as libc::c_int) as
             libc::c_uint).wrapping_sub(visibleXTiles.wrapping_div(2 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint))
            as SDWORD;
    midY =
        ((y >> 7 as libc::c_int) as
             libc::c_uint).wrapping_sub(visibleYTiles.wrapping_div(2 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint))
            as SDWORD;
    player.p.x = midX * 128 as libc::c_int;
    player.p.z = midY * 128 as libc::c_int;
    player.r.z = 0 as libc::c_int;
    SetRadarStrobe(midX as UDWORD, midY as UDWORD);
}
#[no_mangle]
pub unsafe extern "C" fn setViewAngle(mut angle: SDWORD) {
    player.r.x =
        65536 as libc::c_int / 360 as libc::c_int *
            (360 as libc::c_int + angle);
}
#[no_mangle]
pub unsafe extern "C" fn getViewDistance() -> UDWORD { return distance; }
#[no_mangle]
pub unsafe extern "C" fn setViewDistance(mut dist: UDWORD) {
    dist = distance;
}
#[no_mangle]
pub unsafe extern "C" fn renderFeature(mut psFeature: *mut FEATURE) {
    let mut featX: UDWORD = 0;
    let mut featY: UDWORD = 0;
    let mut rotation: SDWORD = 0;
    //SDWORD		centreX,centreZ;
    let mut brightness: UDWORD = 0;
    let mut specular: UDWORD = 0;
    let mut dv: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut vecTemp: *mut iVector = 0 as *mut iVector;
    let mut bForceDraw: BOOL = 0;
    //	if(psFeature->psStats->subType == FEAT_BUILD_WRECK)
//	{
//		return;//don't draw 'em
//	}
    bForceDraw =
        (getRevealStatus() == 0 &&
             (*(*psFeature).psStats).visibleAtStart != 0) as libc::c_int;
    if (*psFeature).visible[selectedPlayer as usize] as libc::c_int != 0 ||
           godMode != 0 || demoGetStatus() != 0 || bForceDraw != 0 {
        (*psFeature).sDisplay.frameNumber = currentGameFrame;
        /* Get it's x and y coordinates so we don't have to deref. struct later */
        featX = (*psFeature).x as UDWORD;
        featY = (*psFeature).y as UDWORD;
        /* Daft hack to get around the oild derrick issue */
        if (*mapTile(featX >> 7 as libc::c_int,
                     featY >> 7 as libc::c_int)).tileInfoBits as libc::c_int &
               0x2 as libc::c_int == 0 {
            return
        }
        dv.x =
            featX.wrapping_sub(player.p.x as
                                   libc::c_uint).wrapping_sub(terrainMidX.wrapping_mul(128
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           libc::c_uint))
                as int32;
        dv.z =
            terrainMidY.wrapping_mul(128 as libc::c_int as
                                         libc::c_uint).wrapping_sub(featY.wrapping_sub(player.p.z
                                                                                           as
                                                                                           libc::c_uint))
                as int32;
        /* features sits at the height of the tile it's centre is on */
        dv.y =
            (*psFeature).z as
                int32; // world x,y,z coord of structure ... this is needed for the BSP code
        SetBSPObjectPos(featX as SDWORD, dv.y, featY as SDWORD);
        /* Push the indentity matrix */
        pie_MatBegin();
        /* Translate the feature  - N.B. We can also do rotations here should we require
		   buildings to face different ways - Don't know if this is necessary - should be IMO */
        pie_TRANSLATE(dv.x, dv.y, dv.z);
        /* Get the x,z translation components */
        rx = player.p.x & 128 as libc::c_int - 1 as libc::c_int;
        rz = player.p.z & 128 as libc::c_int - 1 as libc::c_int;
        /* Translate */
        pie_TRANSLATE(rx, 0 as libc::c_int, -rz);
        rotation =
            65536 as libc::c_int / 360 as libc::c_int *
                (*psFeature).direction as libc::c_int;
        pie_MatRotY(-rotation);
        //		centreX = ( player.p.x + ((visibleXTiles/2)<<TILE_SHIFT) );
//		centreZ = ( player.p.z + ((visibleYTiles/2)<<TILE_SHIFT) );
        brightness = 200 as libc::c_int as UDWORD; //? HUH?
        //		if(psFeature->sDisplay.imd->ymax>300)
        if (*(*psFeature).psStats).subType as libc::c_uint ==
               FEAT_SKYSCRAPER as libc::c_int as libc::c_uint {
            objectShimmy(psFeature as *mut BASE_OBJECT);
        }
        if godMode != 0 || demoGetStatus() != 0 || bForceDraw != 0 {
            brightness = 200 as libc::c_int as UDWORD
        } else if getRevealStatus() != 0 {
            brightness =
                avGetObjLightLevel(psFeature as *mut BASE_OBJECT, brightness)
        }
        brightness =
            lightDoFogAndIllumination(brightness as UBYTE,
                                      (getCentreX() as
                                           libc::c_uint).wrapping_sub(featX)
                                          as SDWORD,
                                      (getCentreZ() as
                                           libc::c_uint).wrapping_sub(featY)
                                          as SDWORD, &mut specular);
        if (*(*psFeature).psStats).subType as libc::c_uint ==
               FEAT_OIL_RESOURCE as libc::c_int as libc::c_uint {
            vecTemp = (*(*psFeature).sDisplay.imd).points;
            flattenImd((*psFeature).sDisplay.imd, (*psFeature).x as UDWORD,
                       (*psFeature).y as UDWORD, 0 as libc::c_int as UDWORD);
            /* currentGameFrame/2 set anim running - GJ hack */
            pie_Draw3DShape((*psFeature).sDisplay.imd,
                            currentGameFrame.wrapping_div(2 as libc::c_int as
                                                              libc::c_uint) as
                                libc::c_int, 0 as libc::c_int, brightness,
                            specular, 0 as libc::c_int, 0 as libc::c_int);
            (*(*psFeature).sDisplay.imd).points = vecTemp
        } else {
            pie_Draw3DShape((*psFeature).sDisplay.imd, 0 as libc::c_int,
                            0 as libc::c_int, brightness, specular,
                            0 as libc::c_int, 0 as libc::c_int);
            //pie_TRANSLUCENT, psFeature->visible[selectedPlayer]);
        }
        let mut sX: SDWORD = 0;
        let mut sY: SDWORD = 0;
        pie_RotateProject(0 as libc::c_int, 0 as libc::c_int,
                          0 as libc::c_int, &mut sX, &mut sY);
        (*psFeature).sDisplay.screenX = sX as UDWORD;
        (*psFeature).sDisplay.screenY = sY as UDWORD;
        targetAdd(psFeature as *mut BASE_OBJECT);
        pie_MatEnd();
    };
}
#[no_mangle]
pub unsafe extern "C" fn renderProximityMsg(mut psProxDisp:
                                                *mut PROXIMITY_DISPLAY) {
    let mut msgX: UDWORD = 0 as libc::c_int as UDWORD;
    let mut msgY: UDWORD = 0 as libc::c_int as UDWORD;
    let mut dv: iVector =
        { let mut init = iVector{x: 0 as libc::c_int, y: 0, z: 0,}; init };
    let mut pViewProximity: *mut VIEW_PROXIMITY = 0 as *mut VIEW_PROXIMITY;
    let mut x: SDWORD = 0;
    let mut y: SDWORD = 0;
    let mut r: SDWORD = 0;
    let mut proxImd: *mut iIMDShape = 0 as *mut iIMDShape;
    //	SDWORD		centreX,centreZ;
    let mut brightness: UDWORD = 0;
    let mut specular: UDWORD = 0;
    //store the frame number for when deciding what has been clicked on
    (*psProxDisp).frameNumber = currentGameFrame;
    /* Get it's x and y coordinates so we don't have to deref. struct later */
    if (*psProxDisp).type_0 as libc::c_uint ==
           POS_PROXDATA as libc::c_int as libc::c_uint {
        pViewProximity =
            (*((*(*psProxDisp).psMessage).pViewData as *mut VIEWDATA)).pData
                as *mut VIEW_PROXIMITY;
        if !pViewProximity.is_null() {
            msgX = (*pViewProximity).x;
            msgY = (*pViewProximity).y;
            /* message sits at the height specified at input*/
            dv.y =
                (*pViewProximity).z.wrapping_add(64 as libc::c_int as
                                                     libc::c_uint) as int32
        }
    } else if (*psProxDisp).type_0 as libc::c_uint ==
                  POS_PROXOBJ as libc::c_int as libc::c_uint {
        msgX =
            (*((*(*psProxDisp).psMessage).pViewData as *mut BASE_OBJECT)).x as
                UDWORD;
        msgY =
            (*((*(*psProxDisp).psMessage).pViewData as *mut BASE_OBJECT)).y as
                UDWORD;
        /* message sits at the height specified at input*/
        dv.y =
            (*((*(*psProxDisp).psMessage).pViewData as *mut BASE_OBJECT)).z as
                libc::c_int + 64 as libc::c_int
    } else {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Buggered proximity message type\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"display3d.c\x00" as *const u8 as *const libc::c_char,
                  2193 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 19],
                                            &[libc::c_char; 19]>(b"renderProximityMsg\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
    }
    //	centreX = ( player.p.x + ((visibleXTiles/2)<<TILE_SHIFT) );
//	centreZ = ( player.p.z + ((visibleYTiles/2)<<TILE_SHIFT) );
    brightness =
        lightDoFogAndIllumination(255 as libc::c_int as UBYTE,
                                  (getCentreX() as
                                       libc::c_uint).wrapping_sub(msgX) as
                                      SDWORD,
                                  (getCentreZ() as
                                       libc::c_uint).wrapping_sub(msgY) as
                                      SDWORD, &mut specular);
    dv.x =
        msgX.wrapping_sub(player.p.x as
                              libc::c_uint).wrapping_sub(terrainMidX.wrapping_mul(128
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      libc::c_uint))
            as int32;
    dv.z =
        terrainMidY.wrapping_mul(128 as libc::c_int as
                                     libc::c_uint).wrapping_sub(msgY.wrapping_sub(player.p.z
                                                                                      as
                                                                                      libc::c_uint))
            as int32;
    /* Push the indentity matrix */
    pie_MatBegin();
    /* Translate the message */
    pie_TRANSLATE(dv.x, dv.y, dv.z);
    /* Get the x,z translation components */
    rx = player.p.x & 128 as libc::c_int - 1 as libc::c_int;
    rz = player.p.z & 128 as libc::c_int - 1 as libc::c_int;
    /* Translate */
    pie_TRANSLATE(rx, 0 as libc::c_int, -rz);
    //get the appropriate IMD
    if !pViewProximity.is_null() {
        match (*pViewProximity).proxType as libc::c_uint {
            0 => {
                proxImd =
                    getImdFromIndex(MI_BLIP_ENEMY as libc::c_int as UDWORD)
            }
            1 => {
                proxImd =
                    getImdFromIndex(MI_BLIP_RESOURCE as libc::c_int as UDWORD)
            }
            2 => {
                proxImd =
                    getImdFromIndex(MI_BLIP_ARTEFACT as libc::c_int as UDWORD)
            }
            _ => {
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Buggered proximity message type\x00" as *const u8
                              as *const libc::c_char);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"display3d.c\x00" as *const u8 as
                              *const libc::c_char, 2228 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 19],
                                                    &[libc::c_char; 19]>(b"renderProximityMsg\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
            }
        }
    } else {
        //object Proximity displays are for oil resources and artefacts
        if (*((*(*psProxDisp).psMessage).pViewData as
                  *mut BASE_OBJECT)).type_0 as libc::c_uint ==
               OBJ_FEATURE as libc::c_int as libc::c_uint {
        } else {
            debug(LOG_ERROR,
                  b"renderProximityMsg: invalid feature\x00" as *const u8 as
                      *const libc::c_char);
        };
        if (*((*(*psProxDisp).psMessage).pViewData as
                  *mut BASE_OBJECT)).type_0 as libc::c_uint ==
               OBJ_FEATURE as libc::c_int as libc::c_uint {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"display3d.c\x00" as *const u8 as *const libc::c_char,
                  2236 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 19],
                                            &[libc::c_char; 19]>(b"renderProximityMsg\x00")).as_ptr(),
                  b"((BASE_OBJECT *)psProxDisp->psMessage->pViewData)->type == OBJ_FEATURE\x00"
                      as *const u8 as *const libc::c_char);
        };
        if (*(*((*(*psProxDisp).psMessage).pViewData as
                    *mut FEATURE)).psStats).subType as libc::c_uint ==
               FEAT_OIL_RESOURCE as libc::c_int as libc::c_uint {
            //resource
            proxImd =
                getImdFromIndex(MI_BLIP_RESOURCE as libc::c_int as UDWORD)
        } else {
            //artefact
            proxImd =
                getImdFromIndex(MI_BLIP_ARTEFACT as libc::c_int as UDWORD)
        }
    }
    //	iV_MatrixRotateY(DEG(gameTime/10));
    pie_MatRotY(-player.r.y);
    pie_MatRotX(-player.r.x);
    if gamePaused() == 0 {
        pie_Draw3DShape(proxImd,
                        getTimeValueRange(1000 as libc::c_int as UDWORD,
                                          4 as libc::c_int as UDWORD) as
                            libc::c_int, 0 as libc::c_int, brightness,
                        specular, 0x4 as libc::c_int, 192 as libc::c_int);
    } else {
        pie_Draw3DShape(proxImd, 0 as libc::c_int, 0 as libc::c_int,
                        brightness, specular, 0x4 as libc::c_int,
                        192 as libc::c_int);
    }
    //get the screen coords for determining when clicked on
    calcFlagPosScreenCoords(&mut x, &mut y, &mut r);
    (*psProxDisp).screenX = x as UDWORD;
    (*psProxDisp).screenY = y as UDWORD;
    (*psProxDisp).screenR = r as UDWORD;
    //storeProximityScreenCoords(psMessage, x, y);
    pie_MatEnd();
}
#[no_mangle]
pub unsafe extern "C" fn renderStructure(mut psStructure: *mut STRUCTURE) {
    //MAPTILE		*psTile;
    let mut structX: SDWORD = 0;
    let mut structY: SDWORD = 0;
    let mut sX: SDWORD = 0;
    let mut sY: SDWORD = 0;
    let mut baseImd: *mut iIMDShape = 0 as *mut iIMDShape;
    let mut strImd: *mut iIMDShape = 0 as *mut iIMDShape;
    let mut mountImd: *mut iIMDShape = 0 as *mut iIMDShape;
    let mut weaponImd: *mut iIMDShape = 0 as *mut iIMDShape;
    let mut flashImd: *mut iIMDShape = 0 as *mut iIMDShape;
    let mut rotation: SDWORD = 0;
    let mut frame: SDWORD = 0;
    let mut playerFrame: SDWORD = 0;
    let mut animFrame: SDWORD = 0;
    let mut nWeaponStat: UDWORD = 0;
    //UDWORD			stage;
    let mut buildingBrightness: UDWORD = 0;
    let mut specular: UDWORD = 0;
    //SDWORD			centreX,centreZ;
    let mut dv: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut i: SDWORD = 0;
    let mut lImd: *mut iIMDShape = 0 as *mut iIMDShape;
    let mut imd: *mut iIMDShape = 0 as *mut iIMDShape;
    let mut temp: *mut iVector = 0 as *mut iVector;
    let mut brightVar: SDWORD = 0;
    let mut bHitByElectronic: BOOL = 0 as libc::c_int;
    let mut yVar: SDWORD = 0;
    let mut pRepImd: *mut iIMDShape = 0 as *mut iIMDShape;
    let mut psRepairFac: *mut REPAIR_FACILITY = 0 as *mut REPAIR_FACILITY;
    //REF_DEFENSE no longer drawn as sloping wall
	//if(psStructure->pStructureType->type>=REF_WALL AND
	//	psStructure->pStructureType->type<=REF_TOWER4)
    if (*(*psStructure).pStructureType).type_0 ==
           REF_WALL as libc::c_int as libc::c_uint ||
           (*(*psStructure).pStructureType).type_0 ==
               REF_WALLCORNER as libc::c_int as libc::c_uint {
        renderWallSection(psStructure);
        return
    }
    if (*(*psStructure).pStructureType).type_0 ==
           REF_DEFENSE as libc::c_int as libc::c_uint {
        renderDefensiveStructure(psStructure);
        return
    }
    // -------------------------------------------------------------------------------
	/* Power stations and factories have pulsing lights  */
    if (*(*psStructure).sDisplay.imd).numFrames as libc::c_int >
           0 as libc::c_int {
        /*OK, so we've got a hack for a new structure - its a 2x2 wall but
        we've called it a BLAST_DOOR cos we don't want it to use the wallDrag code
        So its got clan colour trim and not really an anim - these HACKS just keep
        coming back to haunt us hey? - AB 02/09/99*/
        if bMultiPlayer != 0 &&
               (*(*psStructure).pStructureType).type_0 ==
                   REF_BLASTDOOR as libc::c_int as libc::c_uint {
            animFrame =
                getPlayerColour((*psStructure).player as UDWORD) as SDWORD
        } else {
            //calculate an animation frame
            animFrame =
                gameTime.wrapping_rem(4000 as libc::c_int as
                                          libc::c_uint).wrapping_div(1000 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_uint)
                    as SDWORD
            //(gameTime * STRUCTURE_ANIM_RATE)/GAME_TICKS_PER_SEC;//one frame per second
            //psStructure->sDisplay.animFrame = animFrame;
        }
    } else { animFrame = 0 as libc::c_int } // psStructure->player
    playerFrame = getPlayerColour((*psStructure).player as UDWORD) as SDWORD;
    // -------------------------------------------------------------------------------
    if (*psStructure).visible[selectedPlayer as usize] as libc::c_int != 0 ||
           godMode != 0 || demoGetStatus() != 0 {
        (*psStructure).sDisplay.frameNumber = currentGameFrame;
        /* Get it's x and y coordinates so we don't have to deref. struct later */
        structX =
            (*psStructure).x as
                SDWORD; // world x,y,z coord of structure ... this is needed for the BSP code
        structY = (*psStructure).y as SDWORD;
        dv.x =
            ((structX - player.p.x) as
                 libc::c_uint).wrapping_sub(terrainMidX.wrapping_mul(128 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_uint))
                as int32;
        dv.z =
            terrainMidY.wrapping_mul(128 as libc::c_int as
                                         libc::c_uint).wrapping_sub((structY -
                                                                         player.p.z)
                                                                        as
                                                                        libc::c_uint)
                as int32;
        dv.y =
            map_TileHeight((structX >> 7 as libc::c_int) as UDWORD,
                           (structY >> 7 as libc::c_int) as UDWORD) as int32;
        SetBSPObjectPos(structX, dv.y, structY);
        /* Push the indentity matrix */
        pie_MatBegin();
        /* Translate the building  - N.B. We can also do rotations here should we require
		   buildings to face different ways - Don't know if this is necessary - should be IMO */
        pie_TRANSLATE(dv.x, dv.y, dv.z);
        /* Get the x,z translation components */
        rx = player.p.x & 128 as libc::c_int - 1 as libc::c_int;
        rz = player.p.z & 128 as libc::c_int - 1 as libc::c_int;
        /* Translate */
        pie_TRANSLATE(rx, 0 as libc::c_int, -rz);
        /* OK - here is where we establish which IMD to draw for the building - luckily static objects,
		buildings in other words are NOT made up of components - much quicker! */
        rotation =
            65536 as libc::c_int / 360 as libc::c_int *
                (*psStructure).direction as libc::c_int;
        pie_MatRotY(-rotation);
        //		centreX = ( player.p.x + ((visibleXTiles/2)<<TILE_SHIFT) );
//		centreZ = ( player.p.z + ((visibleYTiles/2)<<TILE_SHIFT) );
        bHitByElectronic = 0 as libc::c_int;
        if gameTime2.wrapping_sub((*psStructure).timeLastHit) <
               (1000 as libc::c_int / 5 as libc::c_int) as libc::c_uint &&
               (*psStructure).lastHitWeapon ==
                   WSC_ELECTRONIC as libc::c_int as libc::c_uint {
            bHitByElectronic = 1 as libc::c_int
        }
        buildingBrightness =
            (200 as libc::c_int as
                 libc::c_uint).wrapping_sub((100 as libc::c_int as
                                                 libc::c_uint).wrapping_sub((((*psStructure).body
                                                                                  as
                                                                                  libc::c_int
                                                                                  *
                                                                                  100
                                                                                      as
                                                                                      libc::c_int)
                                                                                 as
                                                                                 libc::c_uint).wrapping_div(structureBody(psStructure))));
        if (*psStructure).selected != 0 {
            if gamePaused() == 0 {
                brightVar =
                    getStaticTimeValueRange(990 as libc::c_int as UDWORD,
                                            110 as libc::c_int as UDWORD) as
                        SDWORD;
                if brightVar > 55 as libc::c_int {
                    brightVar = 110 as libc::c_int - brightVar
                }
            } else { brightVar = 55 as libc::c_int }
            buildingBrightness = (200 as libc::c_int + brightVar) as UDWORD
        }
        if godMode != 0 || demoGetStatus() != 0 {
            buildingBrightness = buildingBrightness
        } else if getRevealStatus() != 0 {
            buildingBrightness =
                avGetObjLightLevel(psStructure as *mut BASE_OBJECT,
                                   buildingBrightness)
        }
        buildingBrightness =
            lightDoFogAndIllumination(buildingBrightness as UBYTE,
                                      getCentreX() - structX,
                                      getCentreZ() - structY, &mut specular);
        /* Draw the building's base first */
        baseImd = (*(*psStructure).pStructureType).pBaseIMD;
        if !baseImd.is_null() {
            pie_Draw3DShape(baseImd, 0 as libc::c_int, 0 as libc::c_int,
                            buildingBrightness, specular, 0 as libc::c_int,
                            0 as libc::c_int);
        }
        // override
        if bHitByElectronic != 0 {
            buildingBrightness = 150 as libc::c_int as UDWORD
        }
        imd = (*psStructure).sDisplay.imd;
        if !imd.is_null() && bHitByElectronic != 0 {
            // Get a copy of the points
            memcpy(alteredPoints.as_mut_ptr() as *mut libc::c_void,
                   (*imd).points as *const libc::c_void,
                   ((*imd).npoints as
                        libc::c_uint).wrapping_mul(::std::mem::size_of::<iVector>()
                                                       as libc::c_ulong));
            i = 0 as libc::c_int;
            while i < (*imd).npoints {
                yVar = 10 as libc::c_int - rand() % 20 as libc::c_int;
                alteredPoints[i as usize].x +=
                    yVar - rand() % 2 as libc::c_int * yVar;
                alteredPoints[i as usize].z +=
                    yVar - rand() % 2 as libc::c_int * yVar;
                i += 1
            }
            temp = (*imd).points;
            (*imd).points = alteredPoints.as_mut_ptr()
        }
        //first check if partially built - ANOTHER HACK!
        if (*psStructure).status as libc::c_int ==
               SS_BEING_BUILT as libc::c_int ||
               (*psStructure).status as libc::c_int ==
                   SS_BEING_DEMOLISHED as libc::c_int ||
               (*psStructure).status as libc::c_int ==
                   SS_BEING_BUILT as libc::c_int &&
                   (*(*psStructure).pStructureType).type_0 ==
                       REF_RESOURCE_EXTRACTOR as libc::c_int as libc::c_uint {
            pie_Draw3DShape(imd, 0 as libc::c_int, playerFrame,
                            buildingBrightness, specular, 0x10 as libc::c_int,
                            (structHeightScale(psStructure) *
                                 256 as libc::c_int as libc::c_float) as
                                SDWORD);
            if bHitByElectronic != 0 { (*imd).points = temp }
        } else if (*psStructure).status as libc::c_int ==
                      SS_BUILT as libc::c_int {
            /* They're already built!!!! */
            pie_Draw3DShape(imd, animFrame, 0 as libc::c_int,
                            buildingBrightness, specular, 0 as libc::c_int,
                            0 as libc::c_int);
            if bHitByElectronic != 0 { (*imd).points = temp }
            if (*(*psStructure).sDisplay.imd).nconnectors == 1 as libc::c_int
               {
                weaponImd = 0 as *mut iIMDShape;
                mountImd = 0 as *mut iIMDShape;
                flashImd = 0 as *mut iIMDShape;
                strImd = (*psStructure).sDisplay.imd;
                if (*psStructure).asWeaps[0 as libc::c_int as usize].nStat >
                       0 as libc::c_int as libc::c_uint {
                    nWeaponStat =
                        (*psStructure).asWeaps[0 as libc::c_int as
                                                   usize].nStat;
                    weaponImd =
                        (*asWeaponStats.offset(nWeaponStat as isize)).pIMD;
                    mountImd =
                        (*asWeaponStats.offset(nWeaponStat as
                                                   isize)).pMountGraphic;
                    flashImd =
                        (*asWeaponStats.offset(nWeaponStat as
                                                   isize)).pMuzzleGraphic
                }
                if weaponImd.is_null() {
                    //check for ECM
                    if !(*(*psStructure).pStructureType).pECM.is_null() {
                        weaponImd =
                            (*(*(*psStructure).pStructureType).pECM).pIMD;
                        mountImd =
                            (*(*(*psStructure).pStructureType).pECM).pMountGraphic;
                        flashImd = 0 as *mut iIMDShape
                    }
                }
                if weaponImd.is_null() {
                    //check for sensor
                    if !(*(*psStructure).pStructureType).pSensor.is_null() {
                        weaponImd =
                            (*(*(*psStructure).pStructureType).pSensor).pIMD;
                        /* No recoil for sensors */
                        (*psStructure).asWeaps[0 as libc::c_int as
                                                   usize].recoilValue =
                            0 as libc::c_int as UDWORD;
                        mountImd =
                            (*(*(*psStructure).pStructureType).pSensor).pMountGraphic;
                        flashImd = 0 as *mut iIMDShape
                    }
                }
                //draw Weapon/ECM/Sensor for structure
                if !weaponImd.is_null() {
                    pie_MatBegin(); //pie_TRANSLUCENT, psStructure->visible[selectedPlayer]);
                    pie_TRANSLATE((*(*strImd).connectors).x,
                                  (*(*strImd).connectors).z,
                                  (*(*strImd).connectors).y); //pie_TRANSLUCENT, psStructure->visible[selectedPlayer]);
                    pie_MatRotY(65536 as libc::c_int / 360 as libc::c_int *
                                    -((*psStructure).turretRotation as
                                          SDWORD));
                    if !mountImd.is_null() {
                        pie_TRANSLATE(0 as libc::c_int, 0 as libc::c_int,
                                      (*psStructure).asWeaps[0 as libc::c_int
                                                                 as
                                                                 usize].recoilValue.wrapping_div(3
                                                                                                     as
                                                                                                     libc::c_int
                                                                                                     as
                                                                                                     libc::c_uint)
                                          as libc::c_int);
                        pie_Draw3DShape(mountImd, animFrame, 0 as libc::c_int,
                                        buildingBrightness, specular,
                                        0 as libc::c_int, 0 as libc::c_int);
                        if (*mountImd).nconnectors != 0 {
                            pie_TRANSLATE((*(*mountImd).connectors).x,
                                          (*(*mountImd).connectors).z,
                                          (*(*mountImd).connectors).y);
                        }
                    }
                    pie_MatRotX(65536 as libc::c_int / 360 as libc::c_int *
                                    (*psStructure).turretPitch as
                                        libc::c_int);
                    pie_TRANSLATE(0 as libc::c_int, 0 as libc::c_int,
                                  (*psStructure).asWeaps[0 as libc::c_int as
                                                             usize].recoilValue
                                      as libc::c_int);
                    pie_Draw3DShape(weaponImd, playerFrame, 0 as libc::c_int,
                                    buildingBrightness, specular,
                                    0 as libc::c_int, 0 as libc::c_int);
                    if (*(*psStructure).pStructureType).type_0 ==
                           REF_REPAIR_FACILITY as libc::c_int as libc::c_uint
                       {
                        psRepairFac =
                            (*psStructure).pFunctionality as
                                *mut REPAIR_FACILITY;
                        //draw repair flash if the Repair Facility has a target which it has started work on
                        if (*weaponImd).nconnectors != 0 &&
                               !(*psRepairFac).psObj.is_null() &&
                               (*(*psRepairFac).psObj).type_0 as libc::c_uint
                                   == OBJ_DROID as libc::c_int as libc::c_uint
                               &&
                               (*((*psRepairFac).psObj as *mut DROID)).action
                                   == DACTION_WAITDURINGREPAIR as libc::c_int
                           {
                            pie_TRANSLATE((*(*weaponImd).connectors).x,
                                          (*(*weaponImd).connectors).z -
                                              12 as libc::c_int,
                                          (*(*weaponImd).connectors).y);
                            pRepImd =
                                getImdFromIndex(MI_FLAME as libc::c_int as
                                                    UDWORD);
                            pie_MatRotY(65536 as libc::c_int /
                                            360 as libc::c_int *
                                            (*psStructure).turretRotation as
                                                SDWORD);
                            pie_MatRotY(-player.r.y);
                            pie_MatRotX(-player.r.x);
                            pie_Draw3DShape(pRepImd,
                                            getStaticTimeValueRange(100 as
                                                                        libc::c_int
                                                                        as
                                                                        UDWORD,
                                                                    (*pRepImd).numFrames
                                                                        as
                                                                        UDWORD)
                                                as libc::c_int,
                                            0 as libc::c_int,
                                            buildingBrightness,
                                            0 as libc::c_int as UDWORD,
                                            0x4 as libc::c_int,
                                            192 as libc::c_int);
                            pie_MatRotX(player.r.x);
                            pie_MatRotY(player.r.y);
                            pie_MatRotY(65536 as libc::c_int /
                                            360 as libc::c_int *
                                            (*psStructure).turretRotation as
                                                SDWORD);
                        }
                    } else if (*weaponImd).nconnectors != 0 &&
                                  (*psStructure).visible[selectedPlayer as
                                                             usize] as
                                      libc::c_int >
                                      0xff as libc::c_int / 2 as libc::c_int {
                        //we have a droid weapon so do we draw a muzzle flash
                        /* Now we need to move to the end fo the barrel */
                        pie_TRANSLATE((*(*weaponImd).connectors.offset(0 as
                                                                           libc::c_int
                                                                           as
                                                                           isize)).x,
                                      (*(*weaponImd).connectors.offset(0 as
                                                                           libc::c_int
                                                                           as
                                                                           isize)).z,
                                      (*(*weaponImd).connectors.offset(0 as
                                                                           libc::c_int
                                                                           as
                                                                           isize)).y);
                        //and draw the muzzle flash
						//animate for the duration of the flash only
                        if !flashImd.is_null() {
                            //assume no clan colours formuzzle effects
                            if (*flashImd).numFrames as libc::c_int ==
                                   0 as libc::c_int ||
                                   (*flashImd).animInterval as libc::c_int <=
                                       0 as libc::c_int {
                                //no anim so display one frame for a fixed time
                                if gameTime <
                                       (*(*psStructure).asWeaps.as_mut_ptr()).lastFired.wrapping_add((1000
                                                                                                          as
                                                                                                          libc::c_int
                                                                                                          /
                                                                                                          10
                                                                                                              as
                                                                                                              libc::c_int)
                                                                                                         as
                                                                                                         libc::c_uint)
                                   {
                                    pie_Draw3DShape(flashImd,
                                                    0 as libc::c_int,
                                                    0 as libc::c_int,
                                                    buildingBrightness,
                                                    specular,
                                                    0x4 as libc::c_int,
                                                    12 as libc::c_int);
                                    //muzzle flash
                                }
                            } else {
                                frame =
                                    gameTime.wrapping_sub((*(*psStructure).asWeaps.as_mut_ptr()).lastFired).wrapping_div((*flashImd).animInterval
                                                                                                                             as
                                                                                                                             libc::c_uint)
                                        as SDWORD;
                                if frame <
                                       (*flashImd).numFrames as libc::c_int {
                                    pie_Draw3DShape(flashImd, frame,
                                                    0 as libc::c_int,
                                                    buildingBrightness,
                                                    specular,
                                                    0x4 as libc::c_int,
                                                    12 as libc::c_int);
                                    //muzzle flash
                                }
                            }
                        }
                    }
                    pie_MatEnd();
                }
            } else if (*(*psStructure).sDisplay.imd).nconnectors >
                          1 as libc::c_int {
                // add some lights if we have the connectors for it
                i =
                    0 as
                        libc::c_int; //pie_TRANSLUCENT, psStructure->visible[selectedPlayer]);
                while i < (*(*psStructure).sDisplay.imd).nconnectors {
                    pie_MatBegin();
                    pie_TRANSLATE((*(*(*psStructure).sDisplay.imd).connectors).x,
                                  (*(*(*psStructure).sDisplay.imd).connectors).z,
                                  (*(*(*psStructure).sDisplay.imd).connectors).y);
                    lImd =
                        getImdFromIndex(MI_LANDING as libc::c_int as UDWORD);
                    pie_Draw3DShape(lImd,
                                    getStaticTimeValueRange(1024 as
                                                                libc::c_int as
                                                                UDWORD,
                                                            (*lImd).numFrames
                                                                as UDWORD) as
                                        libc::c_int, 0 as libc::c_int,
                                    buildingBrightness, specular,
                                    0 as libc::c_int, 0 as libc::c_int);
                    pie_MatEnd();
                    i += 1
                }
            } else if (*psStructure).asWeaps[0 as libc::c_int as usize].nStat
                          > 0 as libc::c_int as libc::c_uint {
                flashImd = 0 as *mut iIMDShape;
                strImd = (*psStructure).sDisplay.imd;
                //its a baba machine gun
                //get an imd to draw on the connector priority is weapon, ECM, sensor
					//check for weapon
                nWeaponStat =
                    (*psStructure).asWeaps[0 as libc::c_int as usize].nStat;
                flashImd =
                    (*asWeaponStats.offset(nWeaponStat as
                                               isize)).pMuzzleGraphic;
                //draw Weapon/ECM/Sensor for structure
                if !flashImd.is_null() {
                    pie_MatBegin();
                    if (*strImd).ymax > 80 as libc::c_int {
                        //babatower
                        pie_TRANSLATE(0 as libc::c_int, 80 as libc::c_int,
                                      0 as libc::c_int);
                        pie_MatRotY(65536 as libc::c_int / 360 as libc::c_int
                                        *
                                        -((*psStructure).turretRotation as
                                              SDWORD));
                        pie_TRANSLATE(0 as libc::c_int, 0 as libc::c_int,
                                      -(20 as libc::c_int));
                    } else {
                        //baba bunker
                        pie_TRANSLATE(0 as libc::c_int, 10 as libc::c_int,
                                      0 as libc::c_int);
                        pie_MatRotY(65536 as libc::c_int / 360 as libc::c_int
                                        *
                                        -((*psStructure).turretRotation as
                                              SDWORD));
                        pie_TRANSLATE(0 as libc::c_int, 0 as libc::c_int,
                                      -(40 as libc::c_int));
                    }
                    pie_MatRotX(65536 as libc::c_int / 360 as libc::c_int *
                                    (*psStructure).turretPitch as
                                        libc::c_int);
                    //and draw the muzzle flash
						//animate for the duration of the flash only
						//assume no clan colours formuzzle effects
                    if (*flashImd).numFrames as libc::c_int ==
                           0 as libc::c_int ||
                           (*flashImd).animInterval as libc::c_int <=
                               0 as libc::c_int {
                        //no anim so display one frame for a fixed time
                        if gameTime <
                               (*(*psStructure).asWeaps.as_mut_ptr()).lastFired.wrapping_add((1000
                                                                                                  as
                                                                                                  libc::c_int
                                                                                                  /
                                                                                                  10
                                                                                                      as
                                                                                                      libc::c_int)
                                                                                                 as
                                                                                                 libc::c_uint)
                           {
                            pie_Draw3DShape(flashImd, 0 as libc::c_int,
                                            0 as libc::c_int,
                                            buildingBrightness, specular,
                                            0 as libc::c_int,
                                            0 as libc::c_int);
                            //muzzle flash
                        }
                    } else {
                        frame =
                            gameTime.wrapping_sub((*(*psStructure).asWeaps.as_mut_ptr()).lastFired).wrapping_div((*flashImd).animInterval
                                                                                                                     as
                                                                                                                     libc::c_uint)
                                as SDWORD;
                        if frame < (*flashImd).numFrames as libc::c_int {
                            pie_Draw3DShape(flashImd, 0 as libc::c_int,
                                            0 as libc::c_int,
                                            buildingBrightness, specular,
                                            0 as libc::c_int,
                                            0 as libc::c_int);
                            //muzzle flash
                        }
                    }
                    pie_MatEnd();
                }
            }
        }
        pie_RotateProject(0 as libc::c_int, 0 as libc::c_int,
                          0 as libc::c_int, &mut sX, &mut sY);
        (*psStructure).sDisplay.screenX = sX as UDWORD;
        (*psStructure).sDisplay.screenY = sY as UDWORD;
        pie_MatEnd();
        targetAdd(psStructure as *mut BASE_OBJECT);
    };
}
//---
#[no_mangle]
pub unsafe extern "C" fn renderDefensiveStructure(mut psStructure:
                                                      *mut STRUCTURE) {
    let mut structX: SDWORD = 0;
    let mut structY: SDWORD = 0;
    let mut sX: SDWORD = 0;
    let mut sY: SDWORD = 0;
    let mut strImd: *mut iIMDShape = 0 as *mut iIMDShape;
    let mut mountImd: *mut iIMDShape = 0 as *mut iIMDShape;
    let mut weaponImd: *mut iIMDShape = 0 as *mut iIMDShape;
    let mut flashImd: *mut iIMDShape = 0 as *mut iIMDShape;
    let mut rotation: SDWORD = 0;
    let mut frame: SDWORD = 0;
    let mut playerFrame: SDWORD = 0;
    let mut animFrame: SDWORD = 0;
    let mut nWeaponStat: UDWORD = 0;
    let mut brightness: UDWORD = 0;
    let mut buildingBrightness: UDWORD = 0;
    let mut specular: UDWORD = 0;
    //SDWORD			centreX,centreZ;
    let mut dv: iVector = iVector{x: 0, y: 0, z: 0,}; // psStructure->player
    let mut i: SDWORD = 0;
    let mut imd: *mut iIMDShape = 0 as *mut iIMDShape;
    let mut lImd: *mut iIMDShape = 0 as *mut iIMDShape;
    let mut temp: *mut iVector = 0 as *mut iVector;
    let mut pointHeight: SDWORD = 0;
    let mut strHeight: SDWORD = 0;
    let mut shift: SDWORD = 0;
    let mut brightVar: SDWORD = 0;
    playerFrame = getPlayerColour((*psStructure).player as UDWORD) as SDWORD;
    animFrame = playerFrame;
    // -------------------------------------------------------------------------------
    if (*psStructure).visible[selectedPlayer as usize] as libc::c_int != 0 ||
           godMode != 0 || demoGetStatus() != 0 {
        /* Mark it as having been drawn */
        (*psStructure).sDisplay.frameNumber = currentGameFrame;
        /* Get it's x and y coordinates so we don't have to deref. struct later */
        structX = (*psStructure).x as SDWORD;
        structY = (*psStructure).y as SDWORD;
        /* Get coordinates of world centre - hmmm. optimise this out? */
//		centreX = ( player.p.x + ((visibleXTiles/2)<<TILE_SHIFT) );
//		centreZ = ( player.p.z + ((visibleYTiles/2)<<TILE_SHIFT) );
		// Play with the imd so its flattened
        imd = (*psStructure).sDisplay.imd;
        if !imd.is_null() {
            // Get a copy of the points
            memcpy(alteredPoints.as_mut_ptr() as *mut libc::c_void,
                   (*imd).points as *const libc::c_void,
                   ((*imd).npoints as
                        libc::c_uint).wrapping_mul(::std::mem::size_of::<iVector>()
                                                       as libc::c_ulong));
            // Get the height of the centre point for reference
            strHeight =
                (*psStructure).z as
                    SDWORD; //map_Height(structX,structY) + 64;
            //	 Now we got through the shape looking for vertices on the edge
            i =
                0 as
                    libc::c_int; //map_TileHeight(structX>>TILE_SHIFT, structY>>TILE_SHIFT)+64;
            while i < (*imd).npoints {
                if alteredPoints[i as usize].y <= 0 as libc::c_int {
                    pointHeight =
                        map_Height((structX + alteredPoints[i as usize].x) as
                                       UDWORD,
                                   (structY - alteredPoints[i as usize].z) as
                                       UDWORD) as
                            SDWORD; // world x,y,z coord of structure ... this is needed for the BSP code
                    shift = strHeight - pointHeight;
                    alteredPoints[i as usize].y -= shift
                }
                i += 1
            }
        }
        dv.x =
            ((structX - player.p.x) as
                 libc::c_uint).wrapping_sub(terrainMidX.wrapping_mul(128 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_uint))
                as int32;
        dv.z =
            terrainMidY.wrapping_mul(128 as libc::c_int as
                                         libc::c_uint).wrapping_sub((structY -
                                                                         player.p.z)
                                                                        as
                                                                        libc::c_uint)
                as int32;
        dv.y = (*psStructure).z as int32;
        SetBSPObjectPos(structX, dv.y, structY);
        /* Push the indentity matrix */
        pie_MatBegin();
        /* Translate the building  - N.B. We can also do rotations here should we require
		   buildings to face different ways - Don't know if this is necessary - should be IMO */
        pie_TRANSLATE(dv.x, dv.y, dv.z);
        /* Get the x,z translation components */
        rx = player.p.x & 128 as libc::c_int - 1 as libc::c_int;
        rz = player.p.z & 128 as libc::c_int - 1 as libc::c_int;
        /* Translate */
        pie_TRANSLATE(rx, 0 as libc::c_int, -rz);
        rotation =
            65536 as libc::c_int / 360 as libc::c_int *
                (*psStructure).direction as libc::c_int;
        pie_MatRotY(-rotation);
        /* Get the buildings brightness level - proportional to how damaged it is */
        buildingBrightness =
            (200 as libc::c_int as
                 libc::c_uint).wrapping_sub((100 as libc::c_int as
                                                 libc::c_uint).wrapping_sub((((*psStructure).body
                                                                                  as
                                                                                  libc::c_int
                                                                                  *
                                                                                  100
                                                                                      as
                                                                                      libc::c_int)
                                                                                 as
                                                                                 libc::c_uint).wrapping_div(structureBody(psStructure))));
        /* If it's selected, then it's brighter */
        if (*psStructure).selected != 0 {
            if gamePaused() == 0 {
                brightVar =
                    getStaticTimeValueRange(990 as libc::c_int as UDWORD,
                                            110 as libc::c_int as UDWORD) as
                        SDWORD;
                if brightVar > 55 as libc::c_int {
                    brightVar = 110 as libc::c_int - brightVar
                }
            } else { brightVar = 55 as libc::c_int }
            buildingBrightness = (200 as libc::c_int + brightVar) as UDWORD
        }
        if godMode != 0 || demoGetStatus() != 0 {
            buildingBrightness = buildingBrightness
        } else if getRevealStatus() != 0 {
            buildingBrightness =
                avGetObjLightLevel(psStructure as *mut BASE_OBJECT,
                                   buildingBrightness)
        }
        brightness =
            lightDoFogAndIllumination(buildingBrightness as UBYTE,
                                      getCentreX() - structX,
                                      getCentreZ() - structY, &mut specular);
        //first check if partially built - ANOTHER HACK!
        if (*psStructure).status as libc::c_int ==
               SS_BEING_BUILT as libc::c_int ||
               (*psStructure).status as libc::c_int ==
                   SS_BEING_DEMOLISHED as libc::c_int {
            temp = (*imd).points;
            (*imd).points = alteredPoints.as_mut_ptr();
            pie_Draw3DShape(imd, 0 as libc::c_int, playerFrame, brightness,
                            specular, 0x10 as libc::c_int,
                            (structHeightScale(psStructure) *
                                 256 as libc::c_int as libc::c_float) as
                                SDWORD);
            (*imd).points = temp
        } else if (*psStructure).status as libc::c_int ==
                      SS_BUILT as libc::c_int {
            temp = (*imd).points;
            (*imd).points = alteredPoints.as_mut_ptr();
            pie_Draw3DShape(imd, animFrame, 0 as libc::c_int, brightness,
                            specular, 0 as libc::c_int, 0 as libc::c_int);
            (*imd).points = temp;
            // It might have weapons on it
            if (*(*psStructure).sDisplay.imd).nconnectors == 1 as libc::c_int
               {
                weaponImd = 0 as *mut iIMDShape; //weapon is gun ecm or sensor
                mountImd = 0 as *mut iIMDShape;
                flashImd = 0 as *mut iIMDShape;
                strImd = (*psStructure).sDisplay.imd;
                //get an imd to draw on the connector priority is weapon, ECM, sensor
				//check for weapon
                if (*psStructure).asWeaps[0 as libc::c_int as usize].nStat >
                       0 as libc::c_int as libc::c_uint {
                    nWeaponStat =
                        (*psStructure).asWeaps[0 as libc::c_int as
                                                   usize].nStat;
                    weaponImd =
                        (*asWeaponStats.offset(nWeaponStat as isize)).pIMD;
                    mountImd =
                        (*asWeaponStats.offset(nWeaponStat as
                                                   isize)).pMountGraphic;
                    flashImd =
                        (*asWeaponStats.offset(nWeaponStat as
                                                   isize)).pMuzzleGraphic
                }
                if weaponImd.is_null() {
                    //check for ECM
                    if !(*(*psStructure).pStructureType).pECM.is_null() {
                        weaponImd =
                            (*(*(*psStructure).pStructureType).pECM).pIMD;
                        mountImd =
                            (*(*(*psStructure).pStructureType).pECM).pMountGraphic;
                        flashImd = 0 as *mut iIMDShape
                    }
                }
                if weaponImd.is_null() {
                    //check for sensor
                    if !(*(*psStructure).pStructureType).pSensor.is_null() {
                        weaponImd =
                            (*(*(*psStructure).pStructureType).pSensor).pIMD;
                        /* No recoil for sensors */
                        (*psStructure).asWeaps[0 as libc::c_int as
                                                   usize].recoilValue =
                            0 as libc::c_int as UDWORD;
                        mountImd =
                            (*(*(*psStructure).pStructureType).pSensor).pMountGraphic;
                        flashImd = 0 as *mut iIMDShape
                    }
                }
                //draw Weapon/ECM/Sensor for structure
                if !weaponImd.is_null() {
                    pie_MatBegin();
                    pie_TRANSLATE((*(*strImd).connectors).x,
                                  (*(*strImd).connectors).z,
                                  (*(*strImd).connectors).y);
                    pie_MatRotY(65536 as libc::c_int / 360 as libc::c_int *
                                    -((*psStructure).turretRotation as
                                          SDWORD));
                    if !mountImd.is_null() {
                        pie_TRANSLATE(0 as libc::c_int, 0 as libc::c_int,
                                      (*psStructure).asWeaps[0 as libc::c_int
                                                                 as
                                                                 usize].recoilValue.wrapping_div(3
                                                                                                     as
                                                                                                     libc::c_int
                                                                                                     as
                                                                                                     libc::c_uint)
                                          as libc::c_int);
                        pie_Draw3DShape(mountImd, animFrame, 0 as libc::c_int,
                                        brightness, specular,
                                        0 as libc::c_int, 0 as libc::c_int);
                        if (*mountImd).nconnectors != 0 {
                            pie_TRANSLATE((*(*mountImd).connectors).x,
                                          (*(*mountImd).connectors).z,
                                          (*(*mountImd).connectors).y);
                        }
                    }
                    pie_MatRotX(65536 as libc::c_int / 360 as libc::c_int *
                                    (*psStructure).turretPitch as
                                        libc::c_int);
                    pie_TRANSLATE(0 as libc::c_int, 0 as libc::c_int,
                                  (*psStructure).asWeaps[0 as libc::c_int as
                                                             usize].recoilValue
                                      as libc::c_int);
                    pie_Draw3DShape(weaponImd, animFrame, 0 as libc::c_int,
                                    brightness, specular, 0 as libc::c_int,
                                    0 as libc::c_int);
                    //we have a droid weapon so do we draw a muzzle flash
                    if (*weaponImd).nconnectors != 0 &&
                           (*psStructure).visible[selectedPlayer as usize] as
                               libc::c_int >
                               0xff as libc::c_int / 2 as libc::c_int {
                        /* Now we need to move to the end fo the barrel */
                        pie_TRANSLATE((*(*weaponImd).connectors.offset(0 as
                                                                           libc::c_int
                                                                           as
                                                                           isize)).x,
                                      (*(*weaponImd).connectors.offset(0 as
                                                                           libc::c_int
                                                                           as
                                                                           isize)).z,
                                      (*(*weaponImd).connectors.offset(0 as
                                                                           libc::c_int
                                                                           as
                                                                           isize)).y);
                        //and draw the muzzle flash
						//animate for the duration of the flash only
                        if !flashImd.is_null() {
                            //assume no clan colours formuzzle effects
                            if (*flashImd).numFrames as libc::c_int ==
                                   0 as libc::c_int ||
                                   (*flashImd).animInterval as libc::c_int <=
                                       0 as libc::c_int {
                                //no anim so display one frame for a fixed time
                                if gameTime <
                                       (*(*psStructure).asWeaps.as_mut_ptr()).lastFired.wrapping_add((1000
                                                                                                          as
                                                                                                          libc::c_int
                                                                                                          /
                                                                                                          10
                                                                                                              as
                                                                                                              libc::c_int)
                                                                                                         as
                                                                                                         libc::c_uint)
                                   {
                                    pie_Draw3DShape(flashImd,
                                                    0 as libc::c_int,
                                                    0 as libc::c_int,
                                                    brightness, specular,
                                                    0x4 as libc::c_int,
                                                    128 as libc::c_int);
                                    //muzzle flash
                                }
                            } else {
                                frame =
                                    gameTime.wrapping_sub((*(*psStructure).asWeaps.as_mut_ptr()).lastFired).wrapping_div((*flashImd).animInterval
                                                                                                                             as
                                                                                                                             libc::c_uint)
                                        as SDWORD;
                                if frame <
                                       (*flashImd).numFrames as libc::c_int {
                                    pie_Draw3DShape(flashImd, frame,
                                                    0 as libc::c_int,
                                                    brightness, specular,
                                                    0x4 as libc::c_int,
                                                    20 as libc::c_int);
                                    //muzzle flash
                                }
                            }
                        }
                    }
                    pie_MatEnd();
                }
            } else if (*(*psStructure).sDisplay.imd).nconnectors >
                          1 as libc::c_int {
                // add some lights if we have the connectors for it
                i =
                    0 as
                        libc::c_int; //pie_TRANSLUCENT, psStructure->visible[selectedPlayer]);
                while i < (*(*psStructure).sDisplay.imd).nconnectors {
                    pie_MatBegin();
                    pie_TRANSLATE((*(*(*psStructure).sDisplay.imd).connectors).x,
                                  (*(*(*psStructure).sDisplay.imd).connectors).z,
                                  (*(*(*psStructure).sDisplay.imd).connectors).y);
                    lImd =
                        getImdFromIndex(MI_LANDING as libc::c_int as UDWORD);
                    pie_Draw3DShape(lImd,
                                    getStaticTimeValueRange(1024 as
                                                                libc::c_int as
                                                                UDWORD,
                                                            (*lImd).numFrames
                                                                as UDWORD) as
                                        libc::c_int, 0 as libc::c_int,
                                    brightness, specular, 0 as libc::c_int,
                                    0 as libc::c_int);
                    pie_MatEnd();
                    i += 1
                }
            } else if (*psStructure).asWeaps[0 as libc::c_int as usize].nStat
                          > 0 as libc::c_int as libc::c_uint {
                flashImd = 0 as *mut iIMDShape;
                strImd = (*psStructure).sDisplay.imd;
                //its a baba machine gun
                //get an imd to draw on the connector priority is weapon, ECM, sensor
					//check for weapon
                nWeaponStat =
                    (*psStructure).asWeaps[0 as libc::c_int as usize].nStat;
                flashImd =
                    (*asWeaponStats.offset(nWeaponStat as
                                               isize)).pMuzzleGraphic;
                //draw Weapon/ECM/Sensor for structure
                if !flashImd.is_null() {
                    pie_MatBegin();
                    //horrendous hack
                    if (*strImd).ymax > 80 as libc::c_int {
                        //babatower
                        pie_TRANSLATE(0 as libc::c_int, 80 as libc::c_int,
                                      0 as libc::c_int);
                        pie_MatRotY(65536 as libc::c_int / 360 as libc::c_int
                                        *
                                        -((*psStructure).turretRotation as
                                              SDWORD));
                        pie_TRANSLATE(0 as libc::c_int, 0 as libc::c_int,
                                      -(20 as libc::c_int));
                    } else {
                        //baba bunker
                        pie_TRANSLATE(0 as libc::c_int, 10 as libc::c_int,
                                      0 as libc::c_int);
                        pie_MatRotY(65536 as libc::c_int / 360 as libc::c_int
                                        *
                                        -((*psStructure).turretRotation as
                                              SDWORD));
                        pie_TRANSLATE(0 as libc::c_int, 0 as libc::c_int,
                                      -(40 as libc::c_int));
                    }
                    pie_MatRotX(65536 as libc::c_int / 360 as libc::c_int *
                                    (*psStructure).turretPitch as
                                        libc::c_int);
                    //and draw the muzzle flash
						//animate for the duration of the flash only
						//assume no clan colours formuzzle effects
                    if (*flashImd).numFrames as libc::c_int ==
                           0 as libc::c_int ||
                           (*flashImd).animInterval as libc::c_int <=
                               0 as libc::c_int {
                        //no anim so display one frame for a fixed time
                        if gameTime <
                               (*(*psStructure).asWeaps.as_mut_ptr()).lastFired.wrapping_add((1000
                                                                                                  as
                                                                                                  libc::c_int
                                                                                                  /
                                                                                                  10
                                                                                                      as
                                                                                                      libc::c_int)
                                                                                                 as
                                                                                                 libc::c_uint)
                           {
                            pie_Draw3DShape(flashImd, 0 as libc::c_int,
                                            0 as libc::c_int, brightness,
                                            specular, 0 as libc::c_int,
                                            0 as libc::c_int);
                            //muzzle flash
                        }
                    } else {
                        frame =
                            gameTime.wrapping_sub((*(*psStructure).asWeaps.as_mut_ptr()).lastFired).wrapping_div((*flashImd).animInterval
                                                                                                                     as
                                                                                                                     libc::c_uint)
                                as SDWORD;
                        if frame < (*flashImd).numFrames as libc::c_int {
                            pie_Draw3DShape(flashImd, 0 as libc::c_int,
                                            0 as libc::c_int, brightness,
                                            specular, 0 as libc::c_int,
                                            0 as libc::c_int);
                            //muzzle flash
                        }
                    }
                    pie_MatEnd();
                }
            }
        }
        imd = (*psStructure).sDisplay.imd;
        temp = (*imd).points;
        pie_RotateProject(0 as libc::c_int, 0 as libc::c_int,
                          0 as libc::c_int, &mut sX, &mut sY);
        (*psStructure).sDisplay.screenX = sX as UDWORD;
        (*psStructure).sDisplay.screenY = sY as UDWORD;
        pie_MatEnd();
        targetAdd(psStructure as *mut BASE_OBJECT);
    };
}
//---
/*draw the delivery points */
#[no_mangle]
pub unsafe extern "C" fn renderDeliveryPoint(mut psPosition:
                                                 *mut FLAG_POSITION) {
    let mut dv: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut x: SDWORD = 0;
    let mut y: SDWORD = 0;
    let mut r: SDWORD = 0;
    let mut temp: *mut iVector = 0 as *mut iVector;
    //	SDWORD			centreX, centreZ;
    let mut buildingBrightness: SDWORD = 0;
    let mut specular: SDWORD = 0;
    //store the frame number for when deciding what has been clicked on
    (*psPosition).frameNumber = currentGameFrame;
    dv.x =
        (((*psPosition).coords.x - player.p.x) as
             libc::c_uint).wrapping_sub(terrainMidX.wrapping_mul(128 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint))
            as int32;
    dv.z =
        terrainMidY.wrapping_mul(128 as libc::c_int as
                                     libc::c_uint).wrapping_sub(((*psPosition).coords.y
                                                                     -
                                                                     player.p.z)
                                                                    as
                                                                    libc::c_uint)
            as int32;
    dv.y = (*psPosition).coords.z;
    // world x,y,z coord of deliv point ... this is needed for the BSP code
	//SetBSPObjectPos(posX,dv.y,posY);
    /* Push the indentity matrix */
    pie_MatBegin();
    pie_TRANSLATE(dv.x, dv.y, dv.z);
    /* Get the x,z translation components */
    rx = player.p.x & 128 as libc::c_int - 1 as libc::c_int;
    rz = player.p.z & 128 as libc::c_int - 1 as libc::c_int;
    /* Translate */
    pie_TRANSLATE(rx, 0 as libc::c_int, -rz);
    //quick check for invalid data
	//ASSERT( psPosition->factoryType < NUM_FACTORY_TYPES AND
    if ((*psPosition).factoryType as libc::c_int) < 4 as libc::c_int &&
           ((*psPosition).factoryInc as libc::c_int) < 5 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Invalid assembly point\x00" as *const u8 as
                  *const libc::c_char); //they are all big now so make this one smaller too
    };
    if ((*psPosition).factoryType as libc::c_int) < 4 as libc::c_int &&
           ((*psPosition).factoryInc as libc::c_int) < 5 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"display3d.c\x00" as *const u8 as *const libc::c_char,
              2985 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 20],
                                        &[libc::c_char; 20]>(b"renderDeliveryPoint\x00")).as_ptr(),
              b"psPosition->factoryType < NUM_FLAG_TYPES && psPosition->factoryInc < MAX_FACTORY\x00"
                  as *const u8 as *const libc::c_char);
    };
    if (*psPosition).selected == 0 {
        temp =
            (*pAssemblyPointIMDs[(*psPosition).factoryType as
                                     usize][(*psPosition).factoryInc as
                                                usize]).points;
        flattenImd(pAssemblyPointIMDs[(*psPosition).factoryType as
                                          usize][(*psPosition).factoryInc as
                                                     usize],
                   (*psPosition).coords.x as UDWORD,
                   (*psPosition).coords.y as UDWORD,
                   0 as libc::c_int as UDWORD);
    }
    pie_MatScale(50 as libc::c_int as UDWORD);
    //	centreX = ( player.p.x + ((visibleXTiles/2)<<TILE_SHIFT) );
//	centreZ = ( player.p.z + ((visibleYTiles/2)<<TILE_SHIFT) );
    buildingBrightness = 255 as libc::c_int;
    buildingBrightness =
        lightDoFogAndIllumination(buildingBrightness as UBYTE,
                                  getCentreX() - (*psPosition).coords.x,
                                  getCentreZ() - (*psPosition).coords.y,
                                  &mut specular as *mut SDWORD as *mut UDWORD)
            as SDWORD;
    //	pie_Draw3DShape(pAssemblyPointIMDs[psPosition->factoryInc], 0, 0,
	//	buildingBrightness, 0, pie_TRANSLUCENT | pie_NO_BILINEAR, EFFECT_DELIVERY_POINT_TRANSPARENCY);
    pie_Draw3DShape(pAssemblyPointIMDs[(*psPosition).factoryType as
                                           usize][(*psPosition).factoryInc as
                                                      usize],
                    0 as libc::c_int, 0 as libc::c_int,
                    buildingBrightness as UDWORD, specular as UDWORD,
                    0x8 as libc::c_int, 0 as libc::c_int);
    if (*psPosition).selected == 0 {
        (*pAssemblyPointIMDs[(*psPosition).factoryType as
                                 usize][(*psPosition).factoryInc as
                                            usize]).points = temp
    }
    //get the screen coords for the DP
    calcFlagPosScreenCoords(&mut x, &mut y, &mut r);
    (*psPosition).screenX = x as UDWORD;
    (*psPosition).screenY = y as UDWORD;
    (*psPosition).screenR = r as UDWORD;
    pie_MatEnd();
}
//void	postprocessTiles(void);
#[no_mangle]
pub unsafe extern "C" fn renderWallSection(mut psStructure: *mut STRUCTURE)
 -> BOOL {
    let mut structX: SDWORD = 0;
    let mut structY: SDWORD = 0;
    //	SDWORD			centreX,centreZ;
    let mut brightness: UDWORD = 0;
    let mut imd: *mut iIMDShape = 0 as *mut iIMDShape;
    let mut rotation: SDWORD = 0;
    let mut dv: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut i: UDWORD = 0;
    let mut centreHeight: UDWORD = 0;
    let mut pointHeight: UDWORD = 0;
    let mut temp: *mut iVector = 0 as *mut iVector;
    let mut shift: SDWORD = 0;
    let mut buildingBrightness: UDWORD = 0;
    let mut specular: UDWORD = 0;
    let mut sX: SDWORD = 0;
    let mut sY: SDWORD = 0;
    let mut brightVar: SDWORD = 0;
    if (*psStructure).visible[selectedPlayer as usize] as libc::c_int != 0 ||
           godMode != 0 || demoGetStatus() != 0 {
        (*psStructure).sDisplay.frameNumber = currentGameFrame;
        /* Get it's x and y coordinates so we don't have to deref. struct later */
        structX = (*psStructure).x as SDWORD;
        structY = (*psStructure).y as SDWORD;
        //		centreX = ( player.p.x + ((visibleXTiles/2)<<TILE_SHIFT) );
 //		centreZ = ( player.p.z + ((visibleYTiles/2)<<TILE_SHIFT) );
        buildingBrightness =
            (200 as libc::c_int as
                 libc::c_uint).wrapping_sub((100 as libc::c_int as
                                                 libc::c_uint).wrapping_sub((((*psStructure).body
                                                                                  as
                                                                                  libc::c_int
                                                                                  *
                                                                                  100
                                                                                      as
                                                                                      libc::c_int)
                                                                                 as
                                                                                 libc::c_uint).wrapping_div(structureBody(psStructure))));
        if (*psStructure).selected != 0 {
            if gamePaused() == 0 {
                brightVar =
                    getStaticTimeValueRange(990 as libc::c_int as UDWORD,
                                            110 as libc::c_int as UDWORD) as
                        SDWORD;
                if brightVar > 55 as libc::c_int {
                    brightVar = 110 as libc::c_int - brightVar
                }
            } else { brightVar = 55 as libc::c_int }
            buildingBrightness = (200 as libc::c_int + brightVar) as UDWORD
        }
        if !(godMode != 0 || demoGetStatus() != 0) {
            if getRevealStatus() != 0 {
                buildingBrightness =
                    avGetObjLightLevel(psStructure as *mut BASE_OBJECT,
                                       buildingBrightness)
            }
        }
        brightness =
            lightDoFogAndIllumination(buildingBrightness as UBYTE,
                                      getCentreX() - structX,
                                      getCentreZ() - structY, &mut specular);
        //	brightness = lightDoFogAndIllumination(pie_MAX_BRIGHT_LEVEL,centreX-structX,centreZ-structY);
		/*
		Right, now the tricky bit, we need to bugger about with the coordinates of the imd to make it
		fit tightly to the ground and to neighbours.
		*/
        imd = (*(*psStructure).pStructureType).pBaseIMD;
        if !imd.is_null() {
            // Get a copy of the points
            memcpy(alteredPoints.as_mut_ptr() as *mut libc::c_void,
                   (*imd).points as *const libc::c_void,
                   ((*imd).npoints as
                        libc::c_uint).wrapping_mul(::std::mem::size_of::<iVector>()
                                                       as libc::c_ulong));
            // Get the height of the centre point for reference
            centreHeight =
                map_Height(structX as UDWORD, structY as UDWORD) as UDWORD;
            //	 Now we got through the shape looking for vertices on the edge
            i = 0 as libc::c_int as UDWORD;
            while i < (*imd).npoints as UDWORD {
                pointHeight =
                    map_Height((structX + alteredPoints[i as usize].x) as
                                   UDWORD,
                               (structY - alteredPoints[i as usize].z) as
                                   UDWORD) as UDWORD;
                shift = centreHeight.wrapping_sub(pointHeight) as SDWORD;
                alteredPoints[i as usize].y -= shift;
                i = i.wrapping_add(1)
            }
        }
        /* Establish where it is in the world */
        dv.x =
            ((structX - player.p.x) as
                 libc::c_uint).wrapping_sub(terrainMidX.wrapping_mul(128 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_uint))
                as
                int32; // world x,y,z coord of structure ... this is needed for the BSP code
        dv.z =
            terrainMidY.wrapping_mul(128 as libc::c_int as
                                         libc::c_uint).wrapping_sub((structY -
                                                                         player.p.z)
                                                                        as
                                                                        libc::c_uint)
                as int32;
        dv.y = map_Height(structX as UDWORD, structY as UDWORD) as int32;
        SetBSPObjectPos(structX, dv.y, structY);
        /* Push the indentity matrix */
        pie_MatBegin();
        /* Translate */
        pie_TRANSLATE(dv.x, dv.y, dv.z);
        /* Get the x,z translation components */
        rx = player.p.x & 128 as libc::c_int - 1 as libc::c_int;
        rz = player.p.z & 128 as libc::c_int - 1 as libc::c_int;
        /* Translate */
        pie_TRANSLATE(rx, 0 as libc::c_int, -rz);
        rotation =
            65536 as libc::c_int / 360 as libc::c_int *
                (*psStructure).direction as libc::c_int;
        pie_MatRotY(-rotation);
        //	objectShimmy((BASE_OBJECT*)psStructure);
        if !imd.is_null() {
            // Make the imd pointer to the vertex list point to ours
            temp = (*imd).points;
            (*imd).points = alteredPoints.as_mut_ptr();
            // Actually render it
            pie_Draw3DShape(imd, 0 as libc::c_int,
                            getPlayerColour((*psStructure).player as UDWORD)
                                as libc::c_int, brightness, specular,
                            0 as libc::c_int, 0 as libc::c_int);
            (*imd).points = temp
        }
        imd = (*psStructure).sDisplay.imd;
        temp = (*imd).points;
        flattenImd(imd, structX as UDWORD, structY as UDWORD,
                   (*psStructure).direction as UDWORD);
        /* Actually render it */
        if (*psStructure).status as libc::c_int ==
               SS_BEING_BUILT as libc::c_int ||
               (*psStructure).status as libc::c_int ==
                   SS_BEING_DEMOLISHED as libc::c_int ||
               (*psStructure).status as libc::c_int ==
                   SS_BEING_BUILT as libc::c_int &&
                   (*(*psStructure).pStructureType).type_0 ==
                       REF_RESOURCE_EXTRACTOR as libc::c_int as libc::c_uint {
            pie_Draw3DShape((*psStructure).sDisplay.imd, 0 as libc::c_int,
                            getPlayerColour((*psStructure).player as UDWORD)
                                as libc::c_int, brightness, specular,
                            0x10 as libc::c_int,
                            (structHeightScale(psStructure) *
                                 256 as libc::c_int as libc::c_float) as
                                SDWORD);
        } else if (*psStructure).status as libc::c_int ==
                      SS_BUILT as libc::c_int {
            pie_Draw3DShape(imd, 0 as libc::c_int,
                            getPlayerColour((*psStructure).player as UDWORD)
                                as libc::c_int, brightness, specular,
                            0 as libc::c_int, 0 as libc::c_int);
            /*
			pie_Draw3DShape(imd, 0,getPlayerColour( psStructure->player), brightness, specular, 0,0);//pie_TRANSLUCENT, psStructure->visible[selectedPlayer]);
			*/
        }
        (*imd).points = temp;
        // Macro definition declares variables so this needs to be bracketed and indented.
        pie_RotateProject(0 as libc::c_int, 0 as libc::c_int,
                          0 as libc::c_int, &mut sX, &mut sY);
        (*psStructure).sDisplay.screenX = sX as UDWORD;
        (*psStructure).sDisplay.screenY = sY as UDWORD;
        pie_MatEnd();
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
/* renderShadow: draws shadow under droid */
#[no_mangle]
pub unsafe extern "C" fn renderShadow(mut psDroid: *mut DROID,
                                      mut psShadowIMD: *mut iIMDShape) {
    let mut dv: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut pVecTemp: *mut iVector = 0 as *mut iVector;
    let mut shadowScale: SDWORD = 0;
    let mut brightness: UDWORD = 0;
    let mut specular: UDWORD = 0;
    //	SDWORD centreX, centreZ;
    dv.x =
        (((*psDroid).x as libc::c_int - player.p.x) as
             libc::c_uint).wrapping_sub(terrainMidX.wrapping_mul(128 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint))
            as int32;
    if (*psDroid).droidType as libc::c_uint ==
           DROID_TRANSPORTER as libc::c_int as libc::c_uint {
        dv.x -= bobTransporterHeight() / 2 as libc::c_int
    }
    dv.z =
        terrainMidY.wrapping_mul(128 as libc::c_int as
                                     libc::c_uint).wrapping_sub(((*psDroid).y
                                                                     as
                                                                     libc::c_int
                                                                     -
                                                                     player.p.z)
                                                                    as
                                                                    libc::c_uint)
            as int32;
    dv.y =
        map_Height((*psDroid).x as UDWORD, (*psDroid).y as UDWORD) as int32;
    /* Push the indentity matrix */
    pie_MatBegin();
    pie_TRANSLATE(dv.x, dv.y, dv.z);
    /* Get the x,z translation components */
    rx = player.p.x & 128 as libc::c_int - 1 as libc::c_int;
    rz = player.p.z & 128 as libc::c_int - 1 as libc::c_int;
    /* Translate */
    pie_TRANSLATE(rx, 0 as libc::c_int, -rz);
    if (*psDroid).droidType as libc::c_uint ==
           DROID_TRANSPORTER as libc::c_int as libc::c_uint {
        pie_MatRotY(65536 as libc::c_int / 360 as libc::c_int *
                        -((*psDroid).direction as libc::c_int));
    }
    pVecTemp = (*psShadowIMD).points;
    if (*psDroid).droidType as libc::c_uint ==
           DROID_TRANSPORTER as libc::c_int as libc::c_uint {
        flattenImd(psShadowIMD, (*psDroid).x as UDWORD,
                   (*psDroid).y as UDWORD, 0 as libc::c_int as UDWORD);
        shadowScale =
            100 as libc::c_int -
                (*psDroid).z as libc::c_int / 100 as libc::c_int;
        if shadowScale < 50 as libc::c_int { shadowScale = 50 as libc::c_int }
    } else {
        pie_MatRotY(65536 as libc::c_int / 360 as libc::c_int *
                        -((*psDroid).direction as libc::c_int));
        pie_MatRotX(65536 as libc::c_int / 360 as libc::c_int *
                        (*psDroid).pitch as libc::c_int);
        pie_MatRotZ(65536 as libc::c_int / 360 as libc::c_int *
                        (*psDroid).roll as libc::c_int);
    }
    // set up lighting
//	centreX = ( player.p.x + ((visibleXTiles/2)<<TILE_SHIFT) );
//	centreZ = ( player.p.z + ((visibleYTiles/2)<<TILE_SHIFT) );
    brightness =
        lightDoFogAndIllumination(255 as libc::c_int as UBYTE,
                                  getCentreX() - (*psDroid).x as libc::c_int,
                                  getCentreZ() - (*psDroid).y as libc::c_int,
                                  &mut specular);
    pie_Draw3DShape(psShadowIMD, 0 as libc::c_int, 0 as libc::c_int,
                    brightness, specular, 0x2 as libc::c_int,
                    128 as libc::c_int);
    (*psShadowIMD).points = pVecTemp;
    pie_MatEnd();
}
/* Draw the droids */
#[no_mangle]
pub unsafe extern "C" fn renderDroid(mut psDroid: *mut DROID) {
    //PROPULSION_STATS	*psPropStats;
//	ASSERT( psDroid->x != 0 && psDroid->y != 0,
//		"moveUpdateUnit: unit at (0,0)" );
    //	psPropStats = asPropulsionStats + psDroid->asBits[COMP_PROPULSION].nStat;
   //	ASSERT( PTRVALID(psPropStats, sizeof(PROPULSION_STATS)),
	//		"moveUpdateDroid: invalid propulsion stats pointer" );
    /*
	if ( psPropStats->propulsionType == LIFT )
	{
		if ( psDroid->droidType != DROID_TRANSPORTER )
		{
			renderShadow( psDroid, getImdFromIndex(MI_SHADOW) );
		}

	}
	*/
    displayComponentObject(psDroid as *mut BASE_OBJECT);
    targetAdd(psDroid as *mut BASE_OBJECT);
}
// end Fn
/* Draws the strobing 3D drag box that is used for multiple selection */
#[no_mangle]
pub unsafe extern "C" fn drawDragBox() {
    if dragBox3D.status == 1 as libc::c_int as libc::c_uint &&
           buildState == 99 as libc::c_int as libc::c_uint {
        if gameTime.wrapping_sub(dragBox3D.lastTime) >
               50 as libc::c_int as libc::c_uint {
            dragBox3D.boxColourIndex =
                dragBox3D.boxColourIndex.wrapping_add(1);
            if dragBox3D.boxColourIndex >= 10 as libc::c_int as libc::c_uint {
                dragBox3D.boxColourIndex = 0 as libc::c_int as UDWORD
            }
            dragBox3D.lastTime = gameTime
        }
        pie_SetDepthBufferStatus(DEPTH_CMP_ALWAYS_WRT_OFF);
        pie_Box(dragBox3D.x1.wrapping_add(dragBox3D.boxColourIndex.wrapping_div(2
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint))
                    as libc::c_int,
                dragBox3D.y1.wrapping_add(dragBox3D.boxColourIndex.wrapping_div(2
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint))
                    as libc::c_int,
                (mX as
                     libc::c_uint).wrapping_sub(dragBox3D.boxColourIndex.wrapping_div(2
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          libc::c_uint))
                    as libc::c_int,
                (mY as
                     libc::c_uint).wrapping_sub(dragBox3D.boxColourIndex.wrapping_div(2
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          libc::c_uint))
                    as libc::c_int,
                boxPulseColours[dragBox3D.boxColourIndex as usize] as uint32);
        if war_GetTranslucent() != 0 {
            //	pie_doWeirdBoxFX(dragBox3D.x1+dragBox3D.boxColourIndex/2+1,dragBox3D.y1+dragBox3D.boxColourIndex/2+1,
  		//			(mX-dragBox3D.boxColourIndex/2),(mY-dragBox3D.boxColourIndex/2));
            pie_UniTransBoxFill(dragBox3D.x1.wrapping_add(dragBox3D.boxColourIndex.wrapping_div(2
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    libc::c_uint)).wrapping_add(1
                                                                                                                                    as
                                                                                                                                    libc::c_int
                                                                                                                                    as
                                                                                                                                    libc::c_uint)
                                    as SDWORD,
                                dragBox3D.y1.wrapping_add(dragBox3D.boxColourIndex.wrapping_div(2
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    libc::c_uint)).wrapping_add(1
                                                                                                                                    as
                                                                                                                                    libc::c_int
                                                                                                                                    as
                                                                                                                                    libc::c_uint)
                                    as SDWORD,
                                (mX as
                                     libc::c_uint).wrapping_sub(dragBox3D.boxColourIndex.wrapping_div(2
                                                                                                          as
                                                                                                          libc::c_int
                                                                                                          as
                                                                                                          libc::c_uint)).wrapping_sub(1
                                                                                                                                          as
                                                                                                                                          libc::c_int
                                                                                                                                          as
                                                                                                                                          libc::c_uint)
                                    as SDWORD,
                                (mY as
                                     libc::c_uint).wrapping_sub(dragBox3D.boxColourIndex.wrapping_div(2
                                                                                                          as
                                                                                                          libc::c_int
                                                                                                          as
                                                                                                          libc::c_uint)).wrapping_sub(1
                                                                                                                                          as
                                                                                                                                          libc::c_int
                                                                                                                                          as
                                                                                                                                          libc::c_uint)
                                    as SDWORD,
                                0xffffff as libc::c_int as UDWORD,
                                16 as libc::c_int as UDWORD);
        }
        pie_SetDepthBufferStatus(DEPTH_CMP_LEQ_WRT_ON);
    };
}
// display reload bars for structures and droids
#[no_mangle]
pub unsafe extern "C" fn drawWeaponReloadBar(mut psObj: *mut BASE_OBJECT,
                                             mut psWeap: *mut WEAPON) {
    let mut psStats: *mut WEAPON_STATS = 0 as *mut WEAPON_STATS;
    let mut bSalvo: BOOL = 0;
    let mut firingStage: UDWORD = 0;
    let mut interval: UDWORD = 0;
    let mut damLevel: UDWORD = 0;
    let mut scrX: SDWORD = 0;
    let mut scrY: SDWORD = 0;
    let mut scrR: SDWORD = 0;
    let mut scale: SDWORD = 0;
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    /* ****************/
	// display unit resistance instead of reload!
    let mut mulH: FRACT = 0.;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    if ctrlShiftDown() != 0 &&
           (*psObj).type_0 as libc::c_uint ==
               OBJ_DROID as libc::c_int as libc::c_uint {
        psDroid = psObj as *mut DROID;
        scrX = (*psObj).sDisplay.screenX as SDWORD;
        scrY = (*psObj).sDisplay.screenY as SDWORD;
        scrR = (*psObj).sDisplay.screenR as SDWORD;
        scrY += scrR + 2 as libc::c_int;
        if (*psDroid).resistance != 0 {
            mulH =
                (*psDroid).resistance as FRACT /
                    droidResistance(psDroid) as FRACT
        } else { mulH = 100 as libc::c_int as FRACT }
        firingStage = mulH as SDWORD as UDWORD;
        firingStage =
            ((2 as libc::c_int * scrR * 10000 as libc::c_int /
                  100 as libc::c_int) as
                 libc::c_uint).wrapping_mul(firingStage).wrapping_div(10000 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_uint);
        if firingStage >= (2 as libc::c_int * scrR) as UDWORD {
            firingStage =
                (2 as libc::c_int * scrR - 1 as libc::c_int) as UDWORD
        }
        pie_BoxFill(scrX - scrR - 1 as libc::c_int,
                    6 as libc::c_int + scrY + 0 as libc::c_int,
                    scrX - scrR + 2 as libc::c_int * scrR,
                    6 as libc::c_int + scrY + 3 as libc::c_int,
                    0x20202 as libc::c_int as uint32);
        pie_BoxFill(scrX - scrR, 6 as libc::c_int + scrY + 1 as libc::c_int,
                    ((scrX - scrR) as libc::c_uint).wrapping_add(firingStage)
                        as libc::c_int,
                    6 as libc::c_int + scrY + 2 as libc::c_int,
                    0xffffff as libc::c_int as uint32);
        return
    }
    /* ******** ********/
    if (*psWeap).nStat == 0 as libc::c_int as libc::c_uint {
        // no weapon
        return
    }
    psStats = asWeaponStats.offset((*psWeap).nStat as isize);
    /* Justifiable only when greater than a one second reload
		or intra salvo time  */
    bSalvo = 0 as libc::c_int;
    if (*psStats).numRounds as libc::c_int > 1 as libc::c_int {
        bSalvo = 1 as libc::c_int
    }
    if bSalvo != 0 &&
           (*psStats).reloadTime > 1000 as libc::c_int as libc::c_uint ||
           (*psStats).firePause > 1000 as libc::c_int as libc::c_uint ||
           (*psObj).type_0 as libc::c_uint ==
               OBJ_DROID as libc::c_int as libc::c_uint &&
               vtolDroid(psObj as *mut DROID) != 0 {
        if (*psObj).type_0 as libc::c_uint ==
               OBJ_DROID as libc::c_int as libc::c_uint &&
               vtolDroid(psObj as *mut DROID) != 0 {
            //deal with VTOLs
            firingStage =
                (getNumAttackRuns(psObj as *mut DROID) as libc::c_int -
                     (*(psObj as *mut DROID)).sMove.iAttackRuns as
                         libc::c_int) as UDWORD;
            //compare with max value
            interval = getNumAttackRuns(psObj as *mut DROID) as UDWORD
        } else {
            firingStage = gameTime.wrapping_sub((*psWeap).lastFired);
            if bSalvo != 0 {
                interval = (*psStats).reloadTime
            } else { interval = weaponFirePause(psStats, (*psObj).player) }
            //we haven't calculated the damLevel yet! DOH!
			/*if (damLevel < HEAVY_DAMAGE_LEVEL)
			{
				interval += interval;
			}*/
        }
        scrX = (*psObj).sDisplay.screenX as SDWORD;
        scrY = (*psObj).sDisplay.screenY as SDWORD;
        scrR = (*psObj).sDisplay.screenR as SDWORD;
        match (*psObj).type_0 as libc::c_uint {
            0 => {
                damLevel =
                    (*(psObj as
                           *mut DROID)).body.wrapping_mul(100 as libc::c_int
                                                              as
                                                              libc::c_uint).wrapping_div((*(psObj
                                                                                                as
                                                                                                *mut DROID)).originalBody);
                scrY += scrR + 2 as libc::c_int
            }
            1 => {
                psStruct = psObj as *mut STRUCTURE;
                damLevel =
                    (((*psStruct).body as libc::c_int * 100 as libc::c_int) as
                         libc::c_uint).wrapping_div(structureBody(psStruct));
                scale =
                    if (*(*psStruct).pStructureType).baseWidth >
                           (*(*psStruct).pStructureType).baseBreadth {
                        (*(*psStruct).pStructureType).baseWidth
                    } else { (*(*psStruct).pStructureType).baseBreadth } as
                        SDWORD;
                scrY += scale * 10 as libc::c_int - 1 as libc::c_int;
                scrR = scale * 20 as libc::c_int
            }
            _ => {
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"drawWeaponReloadBars: invalid object type\x00" as
                              *const u8 as *const libc::c_char);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"display3d.c\x00" as *const u8 as
                              *const libc::c_char, 3394 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 20],
                                                    &[libc::c_char; 20]>(b"drawWeaponReloadBar\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
                damLevel = 100 as libc::c_int as UDWORD
            }
        }
        //now we know what it is!!
        if damLevel < 25 as libc::c_int as libc::c_uint {
            interval =
                (interval as libc::c_uint).wrapping_add(interval) as UDWORD as
                    UDWORD
        }
        if firingStage < interval {
            /* Get a percentage */
            firingStage =
                firingStage.wrapping_mul(100 as libc::c_int as
                                             libc::c_uint).wrapping_div(interval);
            /* Scale it into an appropriate range */
            firingStage =
                ((2 as libc::c_int * scrR * 10000 as libc::c_int /
                      100 as libc::c_int) as
                     libc::c_uint).wrapping_mul(firingStage).wrapping_div(10000
                                                                              as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_uint);
            if firingStage >= (2 as libc::c_int * scrR) as UDWORD {
                firingStage =
                    (2 as libc::c_int * scrR - 1 as libc::c_int) as UDWORD
            }
            /* Power bars */
            pie_BoxFill(scrX - scrR - 1 as libc::c_int,
                        6 as libc::c_int + scrY + 0 as libc::c_int,
                        scrX - scrR + 2 as libc::c_int * scrR,
                        6 as libc::c_int + scrY + 3 as libc::c_int,
                        0x20202 as libc::c_int as uint32);
            pie_BoxFill(scrX - scrR,
                        6 as libc::c_int + scrY + 1 as libc::c_int,
                        ((scrX - scrR) as
                             libc::c_uint).wrapping_add(firingStage) as
                            libc::c_int,
                        6 as libc::c_int + scrY + 2 as libc::c_int,
                        0xffffff as libc::c_int as uint32);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn drawStructureSelections() {
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut scrX: SDWORD = 0;
    let mut scrY: SDWORD = 0;
    let mut scrR: SDWORD = 0;
    let mut longPowerCol: UDWORD = 0 as libc::c_int as UDWORD;
    let mut powerCol: UBYTE = 0;
    let mut health: UDWORD = 0;
    let mut width: UDWORD = 0;
    let mut scale: UDWORD = 0;
    let mut i: UDWORD = 0;
    let mut psClickedOn: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut bMouseOverStructure: BOOL = 0 as libc::c_int;
    let mut bMouseOverOwnStructure: BOOL = 0 as libc::c_int;
    let mut mulH: FRACT = 0.;
    psClickedOn = mouseTarget();
    if !psClickedOn.is_null() &&
           (*psClickedOn).type_0 as libc::c_uint ==
               OBJ_STRUCTURE as libc::c_int as libc::c_uint {
        bMouseOverStructure = 1 as libc::c_int;
        if (*psClickedOn).player as libc::c_uint == selectedPlayer {
            bMouseOverOwnStructure = 1 as libc::c_int
        }
    }
    pie_SetDepthBufferStatus(DEPTH_CMP_ALWAYS_WRT_ON);
    pie_SetFogStatus(0 as libc::c_int);
    /* Go thru' all the buildings */
    psStruct = apsStructLists[selectedPlayer as usize];
    while !psStruct.is_null() {
        if clipXY((*psStruct).x as SDWORD, (*psStruct).y as SDWORD) != 0 {
            /* If it's selected */
            if (*psStruct).selected as libc::c_int != 0 ||
                   bMouseOverOwnStructure != 0 &&
                       psStruct == psClickedOn as *mut STRUCTURE &&
                       (*(psClickedOn as *mut STRUCTURE)).status as
                           libc::c_int == SS_BUILT as libc::c_int &&
                       (*psStruct).sDisplay.frameNumber == currentGameFrame {
                //----
                scale =
                    if (*(*psStruct).pStructureType).baseWidth >
                           (*(*psStruct).pStructureType).baseBreadth {
                        (*(*psStruct).pStructureType).baseWidth
                    } else { (*(*psStruct).pStructureType).baseBreadth };
                width = scale.wrapping_mul(20 as libc::c_int as libc::c_uint);
                scrX = (*psStruct).sDisplay.screenX as SDWORD;
                scrY =
                    (*psStruct).sDisplay.screenY.wrapping_add(scale.wrapping_mul(10
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_uint))
                        as SDWORD;
                scrR = width as SDWORD;
                //health = PERCENT(psStruct->body, psStruct->baseBodyPoints);
                if ctrlShiftDown() != 0 {
                    //show resistance values if CTRL/SHIFT depressed
                    let mut resistance: UDWORD =
                        structureResistance((*psStruct).pStructureType,
                                            (*psStruct).player);
                    if resistance != 0 {
                        health =
                            (((*psStruct).resistance as libc::c_int *
                                  100 as libc::c_int) as
                                 libc::c_uint).wrapping_div(resistance)
                    } else { health = 100 as libc::c_int as UDWORD }
                } else {
                    //show body points
                    health =
                        (((*psStruct).body as libc::c_int *
                              100 as libc::c_int) as
                             libc::c_uint).wrapping_div(structureBody(psStruct))
                }
                if health > 100 as libc::c_int as libc::c_uint {
                    health = 100 as libc::c_int as UDWORD
                }
                if health > 75 as libc::c_int as libc::c_uint {
                    longPowerCol = 0xff00 as libc::c_int as UDWORD
                    //green
                } else if health >= 50 as libc::c_int as libc::c_uint {
                    longPowerCol = 0xffff00 as libc::c_int as UDWORD
                    //yellow
                } else {
                    longPowerCol = 0xff0000 as libc::c_int as UDWORD
                    //red
                }
                mulH = health as FRACT / 100 as libc::c_int as libc::c_float;
                mulH *= width as FRACT;
                health = mulH as SDWORD as UDWORD;
                //				health = (((width*10000)/100)*health)/10000;
                if health > width { health = width }
                health =
                    (health as
                         libc::c_uint).wrapping_mul(2 as libc::c_int as
                                                        libc::c_uint) as
                        UDWORD as UDWORD;
                pie_BoxFill(scrX - scrR - 1 as libc::c_int,
                            scrY - 1 as libc::c_int,
                            scrX + scrR + 1 as libc::c_int,
                            scrY + 2 as libc::c_int,
                            0x20202 as libc::c_int as uint32);
                pie_BoxFill(scrX - scrR, scrY,
                            ((scrX - scrR) as
                                 libc::c_uint).wrapping_add(health) as
                                libc::c_int, scrY + 1 as libc::c_int,
                            longPowerCol);
                drawWeaponReloadBar(psStruct as *mut BASE_OBJECT,
                                    (*psStruct).asWeaps.as_mut_ptr());
            } else if (*psStruct).status as libc::c_int ==
                          SS_BEING_BUILT as libc::c_int &&
                          (*psStruct).sDisplay.frameNumber == currentGameFrame
             {
                scale =
                    if (*(*psStruct).pStructureType).baseWidth >
                           (*(*psStruct).pStructureType).baseBreadth {
                        (*(*psStruct).pStructureType).baseWidth
                    } else { (*(*psStruct).pStructureType).baseBreadth };
                width = scale.wrapping_mul(20 as libc::c_int as libc::c_uint);
                scrX = (*psStruct).sDisplay.screenX as SDWORD;
                scrY =
                    (*psStruct).sDisplay.screenY.wrapping_add(scale.wrapping_mul(10
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_uint))
                        as SDWORD;
                scrR = width as SDWORD;
                //				health = PERCENT(psStruct->body, psStruct->baseBodyPoints);
                health =
                    (((*psStruct).currentBuildPts as libc::c_int *
                          100 as libc::c_int) as
                         libc::c_uint).wrapping_div((*(*psStruct).pStructureType).buildPoints); // belt and braces
                if health >= 100 as libc::c_int as libc::c_uint {
                    health = 100 as libc::c_int as UDWORD
                }
                powerCol =
                    *colours.as_mut_ptr().offset(14 as libc::c_int as isize);
                mulH = health as FRACT / 100 as libc::c_int as libc::c_float;
                mulH *= width as FRACT;
                health = mulH as SDWORD as UDWORD;
                //						health = (((width*10000)/100)*health)/10000;
                if health > width { health = width }
                health =
                    (health as
                         libc::c_uint).wrapping_mul(2 as libc::c_int as
                                                        libc::c_uint) as
                        UDWORD as UDWORD;
                //						pie_BoxFillIndex(scrX - scrR-1,scrY + scrR+2,scrX + scrR+1,scrY+scrR+6,1);
//						pie_BoxFillIndex(scrX - scrR,scrY + scrR+3,scrX - scrR+health,scrY+scrR+5,powerCol);
                pie_BoxFillIndex(scrX - scrR - 1 as libc::c_int,
                                 scrY - 1 as libc::c_int,
                                 scrX + scrR + 1 as libc::c_int,
                                 scrY + 2 as libc::c_int,
                                 1 as libc::c_int as UBYTE);
                pie_BoxFillIndex(scrX - scrR, scrY,
                                 ((scrX - scrR) as
                                      libc::c_uint).wrapping_add(health) as
                                     libc::c_int, scrY + 1 as libc::c_int,
                                 powerCol);
            }
            //----
        }
        psStruct = (*psStruct).psNext
    }
    i = 0 as libc::c_int as UDWORD;
    while i < 8 as libc::c_int as libc::c_uint {
        /* Go thru' all the buildings */
        psStruct = apsStructLists[i as usize];
        while !psStruct.is_null() {
            if i != selectedPlayer {
                // only see enemy buildings being targetted, not yours!
                if clipXY((*psStruct).x as SDWORD, (*psStruct).y as SDWORD) !=
                       0 {
                    /* If it's targetted and on-screen */
                    if (*psStruct).targetted != 0 {
                        if (*psStruct).sDisplay.frameNumber ==
                               currentGameFrame {
                            //health = PERCENT(psStruct->body, psStruct->baseBodyPoints);
	//						health = PERCENT(psStruct->body, structureBody(psStruct));
                            (*psStruct).targetted = 0 as libc::c_int as UBYTE;
                            scrX = (*psStruct).sDisplay.screenX as SDWORD;
                            scrY =
                                (*psStruct).sDisplay.screenY.wrapping_sub(((*(*psStruct).sDisplay.imd).ymax
                                                                               /
                                                                               4
                                                                                   as
                                                                                   libc::c_int)
                                                                              as
                                                                              libc::c_uint)
                                    as SDWORD;
                            pie_ImageFileID(IntImages,
                                            getTargettingGfx() as UWORD, scrX,
                                            scrY);
                            /*
							scrR = (gameTime%1000)/50;
							if(health>REPAIRLEV_HIGH) powerCol = COL_GREEN;
							else if(health>REPAIRLEV_LOW) powerCol = COL_YELLOW;
							else powerCol = COL_RED;

							iV_Line(scrX-scrR,scrY,scrX+scrR,scrY,255);//powerCol);
							iV_Line(scrX,scrY-scrR,scrX,scrY+scrR,255);//powerCol);
							*/
                        }
                    }
                }
            }
            psStruct = (*psStruct).psNext
        }
        i = i.wrapping_add(1)
    }
    if bMouseOverStructure != 0 && bMouseOverOwnStructure == 0 {
        if mouseDown(MOUSE_RMB) != 0 {
            psStruct = psClickedOn as *mut STRUCTURE;
            if (*psStruct).status as libc::c_int == SS_BUILT as libc::c_int {
                //----
                //----
                scale =
                    if (*(*psStruct).pStructureType).baseWidth >
                           (*(*psStruct).pStructureType).baseBreadth {
                        (*(*psStruct).pStructureType).baseWidth
                    } else { (*(*psStruct).pStructureType).baseBreadth };
                width = scale.wrapping_mul(20 as libc::c_int as libc::c_uint);
                scrX = (*psStruct).sDisplay.screenX as SDWORD;
                scrY =
                    (*psStruct).sDisplay.screenY.wrapping_add(scale.wrapping_mul(10
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_uint))
                        as SDWORD;
                scrR = width as SDWORD;
                //health = PERCENT(psStruct->body, psStruct->baseBodyPoints);
                if ctrlShiftDown() != 0 {
                    //show resistance values if CTRL/SHIFT depressed
                    let mut resistance_0: UDWORD =
                        structureResistance((*psStruct).pStructureType,
                                            (*psStruct).player);
                    if resistance_0 != 0 {
                        health =
                            (((*psStruct).resistance as libc::c_int *
                                  100 as libc::c_int) as
                                 libc::c_uint).wrapping_div(resistance_0)
                    } else { health = 100 as libc::c_int as UDWORD }
                } else {
                    //show body points
                    health =
                        (((*psStruct).body as libc::c_int *
                              100 as libc::c_int) as
                             libc::c_uint).wrapping_div(structureBody(psStruct))
                } //red
                if health > 75 as libc::c_int as libc::c_uint {
                    longPowerCol = 0xff00 as libc::c_int as UDWORD
                } else if health > 50 as libc::c_int as libc::c_uint { //green
                    longPowerCol = 0xffff00 as libc::c_int as UDWORD
                } else {
                    longPowerCol = 0xff0000 as libc::c_int as UDWORD
                } //yellow
                health =
                    width.wrapping_mul(10000 as libc::c_int as
                                           libc::c_uint).wrapping_div(100 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_uint).wrapping_mul(health).wrapping_div(10000
                                                                                                                              as
                                                                                                                              libc::c_int
                                                                                                                              as
                                                                                                                              libc::c_uint);
                health =
                    (health as
                         libc::c_uint).wrapping_mul(2 as libc::c_int as
                                                        libc::c_uint) as
                        UDWORD as UDWORD;
                pie_BoxFill(scrX - scrR - 1 as libc::c_int,
                            scrY - 1 as libc::c_int,
                            scrX + scrR + 1 as libc::c_int,
                            scrY + 2 as libc::c_int,
                            0x20202 as libc::c_int as uint32);
                pie_BoxFill(scrX - scrR, scrY,
                            ((scrX - scrR) as
                                 libc::c_uint).wrapping_add(health) as
                                libc::c_int, scrY + 1 as libc::c_int,
                            longPowerCol);
            } else if (*psStruct).status as libc::c_int ==
                          SS_BEING_BUILT as libc::c_int {
                scale =
                    if (*(*psStruct).pStructureType).baseWidth >
                           (*(*psStruct).pStructureType).baseBreadth {
                        (*(*psStruct).pStructureType).baseWidth
                    } else { (*(*psStruct).pStructureType).baseBreadth };
                width = scale.wrapping_mul(20 as libc::c_int as libc::c_uint);
                scrX = (*psStruct).sDisplay.screenX as SDWORD;
                scrY =
                    (*psStruct).sDisplay.screenY.wrapping_add(scale.wrapping_mul(10
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_uint))
                        as SDWORD;
                scrR = width as SDWORD;
                //				health = PERCENT(psStruct->body, psStruct->baseBodyPoints);
                health =
                    (((*psStruct).currentBuildPts as libc::c_int *
                          100 as libc::c_int) as
                         libc::c_uint).wrapping_div((*(*psStruct).pStructureType).buildPoints);
                powerCol =
                    *colours.as_mut_ptr().offset(2 as libc::c_int as isize);
                health =
                    width.wrapping_mul(10000 as libc::c_int as
                                           libc::c_uint).wrapping_div(100 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_uint).wrapping_mul(health).wrapping_div(10000
                                                                                                                              as
                                                                                                                              libc::c_int
                                                                                                                              as
                                                                                                                              libc::c_uint);
                health =
                    (health as
                         libc::c_uint).wrapping_mul(2 as libc::c_int as
                                                        libc::c_uint) as
                        UDWORD as UDWORD;
                //				pie_BoxFillIndex(scrX - scrR-1,scrY + scrR+2,scrX + scrR+1,scrY+scrR+6,1);
//				pie_BoxFillIndex(scrX - scrR,scrY + scrR+3,scrX - scrR+health,scrY+scrR+5,powerCol);
                pie_BoxFillIndex(scrX - scrR - 1 as libc::c_int,
                                 scrY - 1 as libc::c_int,
                                 scrX + scrR + 1 as libc::c_int,
                                 scrY + 2 as libc::c_int,
                                 1 as libc::c_int as UBYTE);
                pie_BoxFillIndex(scrX - scrR - 1 as libc::c_int, scrY,
                                 ((scrX - scrR) as
                                      libc::c_uint).wrapping_add(health) as
                                     libc::c_int, scrY + 1 as libc::c_int,
                                 powerCol);
            }
        }
    }
    pie_SetDepthBufferStatus(DEPTH_CMP_LEQ_WRT_ON);
}
// Amount to push terrain below water.
/* *******************  Prototypes  ********************/
// TODO: Declare as many static as possible.
#[no_mangle]
pub unsafe extern "C" fn getTargettingGfx() -> UDWORD {
    let mut index: UDWORD = 0;
    index =
        getTimeValueRange(1000 as libc::c_int as UDWORD,
                          10 as libc::c_int as UDWORD);
    match index {
        0 | 1 | 2 => {
            return (IMAGE_TARGET1 as libc::c_int as
                        libc::c_uint).wrapping_add(index)
        }
        _ => {
            if index & 0x1 as libc::c_int as libc::c_uint != 0 {
                return IMAGE_TARGET4 as libc::c_int as UDWORD
            } else { return IMAGE_TARGET5 as libc::c_int as UDWORD }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn eitherSelected(mut psDroid: *mut DROID) -> BOOL {
    let mut retVal: BOOL = 0;
    let mut psObj: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    retVal = 0 as libc::c_int;
    if (*psDroid).selected != 0 { retVal = 1 as libc::c_int }
    if !(*psDroid).psGroup.is_null() {
        if !(*(*psDroid).psGroup).psCommander.is_null() {
            if (*(*(*psDroid).psGroup).psCommander).selected != 0 {
                retVal = 1 as libc::c_int
            }
        }
    }
    if orderStateObj(psDroid, DORDER_FIRESUPPORT, &mut psObj) != 0 {
        if !psObj.is_null() && (*psObj).selected as libc::c_int != 0 {
            retVal = 1 as libc::c_int
        }
    }
    return retVal;
}
#[no_mangle]
pub unsafe extern "C" fn drawDroidPowerBar(mut psDroid: *mut DROID) { }
#[no_mangle]
pub unsafe extern "C" fn drawDroidRanking(mut psDroid: *mut DROID) { }
#[no_mangle]
pub unsafe extern "C" fn drawDroidReloadBar(mut psDroid: *mut DROID) { }
#[no_mangle]
pub unsafe extern "C" fn doHighlight(mut psDroid: *mut DROID) -> BOOL {
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn showDroidSelection(mut psDroid: *mut DROID) {
    /* First, let's see if we need to - Get out if it's not selected and no appropriate to highlight */
    if (*psDroid).selected == 0 && doHighlight(psDroid) == 0 {
        /* No need - so get out now */
        return
    }
    /* Right, we're going to process this one, so first do power bars */
    drawDroidPowerBar(psDroid);
    /* Now, display it's rank */
    drawDroidRanking(psDroid);
    /* And the reload bars... */
    drawDroidReloadBar(psDroid);
}
#[no_mangle]
pub unsafe extern "C" fn drawDeliveryPointSelection() {
    let mut psDelivPoint: *mut FLAG_POSITION = 0 as *mut FLAG_POSITION;
    let mut scrX: SDWORD = 0;
    let mut scrY: SDWORD = 0;
    let mut scrR: SDWORD = 0;
    //draw the selected Delivery Point if any
    psDelivPoint = apsFlagPosLists[selectedPlayer as usize];
    while !psDelivPoint.is_null() {
        if (*psDelivPoint).selected != 0 &&
               (*psDelivPoint).frameNumber == currentGameFrame {
            scrX = (*psDelivPoint).screenX as SDWORD;
            scrY = (*psDelivPoint).screenY as SDWORD;
            scrR = (*psDelivPoint).screenR as SDWORD;
            /* Three DFX clips properly right now - not sure if software does */
            if scrX + scrR > 0 as libc::c_int &&
                   scrY + scrR > 0 as libc::c_int &&
                   ((scrX - scrR) as libc::c_uint) < pie_GetVideoBufferWidth()
                   &&
                   ((scrY - scrR) as libc::c_uint) <
                       pie_GetVideoBufferHeight() {
                pie_Box(scrX - scrR, scrY - scrR, scrX + scrR, scrY + scrR,
                        110 as libc::c_int as uint32);
            }
        }
        psDelivPoint = (*psDelivPoint).psNext
    };
}
#[no_mangle]
pub unsafe extern "C" fn drawDroidSelections() {
    let mut scrX: SDWORD = 0;
    let mut scrY: SDWORD = 0;
    let mut scrR: SDWORD = 0;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut damage: UDWORD = 0;
    let mut longPowerCol: UDWORD = 0 as libc::c_int as UDWORD;
    let mut boxCol: UBYTE = 0;
    let mut longBoxCol: UDWORD = 0;
    let mut psClickedOn: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut bMouseOverDroid: BOOL = 0 as libc::c_int;
    let mut bMouseOverOwnDroid: BOOL = 0 as libc::c_int;
    let mut bBeingTracked: BOOL = 0;
    let mut i: UDWORD = 0;
    let mut index: UDWORD = 0;
    let mut psFeature: *mut FEATURE = 0 as *mut FEATURE;
    let mut mulH: FRACT = 0.;
    psClickedOn = mouseTarget();
    if !psClickedOn.is_null() &&
           (*psClickedOn).type_0 as libc::c_uint ==
               OBJ_DROID as libc::c_int as libc::c_uint {
        bMouseOverDroid = 1 as libc::c_int;
        if (*psClickedOn).player as libc::c_uint == selectedPlayer &&
               (*psClickedOn).selected == 0 {
            bMouseOverOwnDroid = 1 as libc::c_int
        }
    }
    match barMode {
        0 => { bEnergyBars = 1 as libc::c_int; bTinyBars = 0 as libc::c_int }
        1 => { bEnergyBars = 0 as libc::c_int; bTinyBars = 0 as libc::c_int }
        2 => { bEnergyBars = 0 as libc::c_int; bTinyBars = 1 as libc::c_int }
        3 => { return }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Invalid energy bar display value\x00" as *const u8 as
                          *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"display3d.c\x00" as *const u8 as *const libc::c_char,
                      3804 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 20],
                                                &[libc::c_char; 20]>(b"drawDroidSelections\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
        }
    }
    pie_SetDepthBufferStatus(DEPTH_CMP_ALWAYS_WRT_ON);
    pie_SetFogStatus(0 as libc::c_int);
    psDroid = apsDroidLists[selectedPlayer as usize];
    while !psDroid.is_null() {
        bBeingTracked = 0 as libc::c_int;
        /* If it's selected and on screen or it's the one the mouse is over OR*/
		// ABSOLUTELY MAD LOGICAL EXPRESSION!!! :-)
        if eitherSelected(psDroid) != 0 &&
               (*psDroid).sDisplay.frameNumber == currentGameFrame ||
               bMouseOverOwnDroid != 0 && psDroid == psClickedOn as *mut DROID
               ||
               droidUnderRepair(psDroid) != 0 &&
                   (*psDroid).sDisplay.frameNumber == currentGameFrame {
            //show resistance values if CTRL/SHIFT depressed (now done in reload bar)
//            if (ctrlShiftDown())
//          {
//                if (psDroid->resistance)
//                {
//                    damage = PERCENT(psDroid->resistance, droidResistance(psDroid));
//                }
//                else
//                {
//                    damage = 100;
//                }
//            }
//            else
//            {
            damage =
                (*psDroid).body.wrapping_mul(100 as libc::c_int as
                                                 libc::c_uint).wrapping_div((*psDroid).originalBody);
            //            }
            if damage > 75 as libc::c_int as libc::c_uint { //red
                longPowerCol = 0xff00 as libc::c_int as UDWORD
            } else if damage > 50 as libc::c_int as libc::c_uint { //green
                longPowerCol = 0xffff00 as libc::c_int as UDWORD
            } else {
                longPowerCol = 0xff0000 as libc::c_int as UDWORD
            } //yellow
            //show resistance values if CTRL/SHIFT depressed(now done in reload bar)
//            if (ctrlShiftDown())
//          {
//                if (psDroid->resistance)
//                {
//                    mulH = MAKEFRACT(psDroid->resistance) / MAKEFRACT(droidResistance(psDroid));
//                }
//                else
//                {
//                    mulH = 100;
//                }
//            }
//            else
//            {
            mulH =
                (*psDroid).body as FRACT / (*psDroid).originalBody as FRACT;
            //            }
            damage =
                (mulH * (*psDroid).sDisplay.screenR as FRACT) as SDWORD as
                    UDWORD; // (((psDroid->sDisplay.screenR*10000)/100)*damage)/10000;
            if damage > (*psDroid).sDisplay.screenR {
                damage = (*psDroid).sDisplay.screenR
            }
            damage =
                (damage as
                     libc::c_uint).wrapping_mul(2 as libc::c_int as
                                                    libc::c_uint) as UDWORD as
                    UDWORD;
            scrX = (*psDroid).sDisplay.screenX as SDWORD;
            scrY = (*psDroid).sDisplay.screenY as SDWORD;
            scrR = (*psDroid).sDisplay.screenR as SDWORD;
            /* Yeah, yeah yeah - hardcoded palette entries - need to change to #defined colour names */
			/* Three DFX clips properly right now - not sure if software does */
//			if((scrX+scrR)>0 AND (scrY+scrR)>0 AND (scrX-scrR)<DISP_WIDTH
//				AND (scrY-scrR)<DISP_HEIGHT)
            if driveModeActive() == 0 || driveIsDriven(psDroid) != 0 {
                boxCol = defaultColours.white;
                longBoxCol = 0xffffff as libc::c_int as UDWORD
            } else {
                boxCol = defaultColours.green;
                longBoxCol = 0xff00 as libc::c_int as UDWORD
            }
            if (*psDroid).selected != 0 {
                /* Selection Lines */
                if bEnergyBars != 0 {
                    pie_BoxFill(scrX - scrR, scrY + scrR - 7 as libc::c_int,
                                scrX - scrR + 1 as libc::c_int, scrY + scrR,
                                longBoxCol);
                    pie_BoxFill(scrX - scrR, scrY + scrR,
                                scrX - scrR + 7 as libc::c_int,
                                scrY + scrR + 1 as libc::c_int, longBoxCol);
                    pie_BoxFill(scrX + scrR - 7 as libc::c_int, scrY + scrR,
                                scrX + scrR, scrY + scrR + 1 as libc::c_int,
                                longBoxCol);
                    pie_BoxFill(scrX + scrR, scrY + scrR - 7 as libc::c_int,
                                scrX + scrR + 1 as libc::c_int,
                                scrY + scrR + 1 as libc::c_int, longBoxCol);
                } else if bTinyBars != 0 {
                    pie_BoxFill(scrX - scrR - 3 as libc::c_int,
                                scrY - 3 as libc::c_int,
                                scrX - scrR + 3 as libc::c_int,
                                scrY + 3 as libc::c_int,
                                0x10101 as libc::c_int as uint32);
                    pie_BoxFill(scrX - scrR - 2 as libc::c_int,
                                scrY - 2 as libc::c_int,
                                scrX - scrR + 2 as libc::c_int,
                                scrY + 2 as libc::c_int, longPowerCol);
                } else {
                    pie_BoxFill(scrX - scrR, scrY + scrR - 7 as libc::c_int,
                                scrX - scrR + 1 as libc::c_int, scrY + scrR,
                                longPowerCol);
                    pie_BoxFill(scrX - scrR, scrY + scrR,
                                scrX - scrR + 7 as libc::c_int,
                                scrY + scrR + 1 as libc::c_int, longPowerCol);
                    pie_BoxFill(scrX + scrR - 7 as libc::c_int, scrY + scrR,
                                scrX + scrR, scrY + scrR + 1 as libc::c_int,
                                longPowerCol);
                    pie_BoxFill(scrX + scrR, scrY + scrR - 7 as libc::c_int,
                                scrX + scrR + 1 as libc::c_int,
                                scrY + scrR + 1 as libc::c_int, longPowerCol);
                }
            }
            if bEnergyBars != 0 {
                /* Power bars */
                pie_BoxFill(scrX - scrR - 1 as libc::c_int,
                            scrY + scrR + 2 as libc::c_int,
                            scrX + scrR + 1 as libc::c_int,
                            scrY + scrR + 5 as libc::c_int,
                            0x20202 as libc::c_int as uint32);
                pie_BoxFill(scrX - scrR, scrY + scrR + 3 as libc::c_int,
                            ((scrX - scrR) as
                                 libc::c_uint).wrapping_add(damage) as
                                libc::c_int, scrY + scrR + 4 as libc::c_int,
                            longPowerCol);
            }
            /* Write the droid rank out */
            if scrX + scrR > 0 as libc::c_int &&
                   scrY + scrR > 0 as libc::c_int &&
                   ((scrX - scrR) as libc::c_uint) < pie_GetVideoBufferWidth()
                   &&
                   ((scrY - scrR) as libc::c_uint) <
                       pie_GetVideoBufferHeight() {
                drawDroidRank(psDroid);
                drawDroidSensorLock(psDroid);
                if (*psDroid).droidType as libc::c_uint ==
                       DROID_COMMAND as libc::c_int as libc::c_uint ||
                       !(*psDroid).psGroup.is_null() &&
                           (*(*psDroid).psGroup).type_0 as libc::c_int ==
                               GT_COMMAND as libc::c_int {
                    drawDroidCmndNo(psDroid);
                } else if (*psDroid).group as libc::c_int !=
                              0xff as libc::c_int {
                    drawDroidGroupNumber(psDroid);
                }
            }
            if bReloadBars != 0 {
                drawWeaponReloadBar(psDroid as *mut BASE_OBJECT,
                                    (*psDroid).asWeaps.as_mut_ptr());
            }
        }
        psDroid = (*psDroid).psNext
    }
    /* Are we over an enemy droid */
    if bMouseOverDroid != 0 && bMouseOverOwnDroid == 0 {
        if mouseDown(MOUSE_RMB) != 0 {
            if (*psClickedOn).player as libc::c_uint != selectedPlayer &&
                   (*psClickedOn).sDisplay.frameNumber == currentGameFrame {
                psDroid = psClickedOn as *mut DROID;
                //show resistance values if CTRL/SHIFT depressed
                if ctrlShiftDown() != 0 {
                    if (*psDroid).resistance != 0 {
                        damage =
                            ((*psDroid).resistance as libc::c_int *
                                 100 as libc::c_int /
                                 droidResistance(psDroid) as libc::c_int) as
                                UDWORD
                    } else { damage = 100 as libc::c_int as UDWORD }
                } else {
                    damage =
                        (*psDroid).body.wrapping_mul(100 as libc::c_int as
                                                         libc::c_uint).wrapping_div((*psDroid).originalBody)
                } //red
                if damage > 75 as libc::c_int as libc::c_uint {
                    longPowerCol = 0xff00 as libc::c_int as UDWORD
                } else if damage > 50 as libc::c_int as libc::c_uint { //green
                    longPowerCol = 0xffff00 as libc::c_int as UDWORD
                } else {
                    longPowerCol = 0xff0000 as libc::c_int as UDWORD
                } //yellow
                //show resistance values if CTRL/SHIFT depressed
                if ctrlShiftDown() != 0 {
                    if (*psDroid).resistance != 0 {
                        mulH =
                            (*psDroid).resistance as FRACT /
                                droidResistance(psDroid) as FRACT
                    } else { mulH = 100 as libc::c_int as FRACT }
                } else {
                    mulH =
                        (*psDroid).body as FRACT /
                            (*psDroid).originalBody as FRACT
                } // (((psDroid->sDisplay.screenR*10000)/100)*damage)/10000;
                damage =
                    (mulH * (*psDroid).sDisplay.screenR as FRACT) as SDWORD as
                        UDWORD;
                //			    damage = MAKEINT(MAKEFRACT(psDroid->body) / MAKEFRACT(psDroid->originalBody));// (((psDroid->sDisplay.screenR*10000)/100)*damage)/10000;
//				damage = (((psDroid->sDisplay.screenR*10000)/100)*damage)/10000;
                if damage > (*psDroid).sDisplay.screenR {
                    damage = (*psDroid).sDisplay.screenR
                }
                damage =
                    (damage as
                         libc::c_uint).wrapping_mul(2 as libc::c_int as
                                                        libc::c_uint) as
                        UDWORD as UDWORD;
                scrX = (*psDroid).sDisplay.screenX as SDWORD;
                scrY = (*psDroid).sDisplay.screenY as SDWORD;
                scrR = (*psDroid).sDisplay.screenR as SDWORD;
                /* Yeah, yeah yeah - hardcoded palette entries - need to change to #defined colour names */
				/* Three DFX clips properly right now - not sure if software does */
                if scrX + scrR > 0 as libc::c_int &&
                       scrY + scrR > 0 as libc::c_int &&
                       ((scrX - scrR) as libc::c_uint) <
                           pie_GetVideoBufferWidth() &&
                       ((scrY - scrR) as libc::c_uint) <
                           pie_GetVideoBufferHeight() {
                    if driveModeActive() == 0 || driveIsDriven(psDroid) != 0 {
                        boxCol = defaultColours.white;
                        longBoxCol = 0xffffff as libc::c_int as UDWORD
                    } else {
                        boxCol = defaultColours.green;
                        longBoxCol = 0xff00 as libc::c_int as UDWORD
                    }
                    //we always want to show the enemy health/resistance as energyBar - AB 18/06/99
					//if(bEnergyBars)
                    /* Power bars */
                    pie_BoxFill(scrX - scrR - 1 as libc::c_int,
                                scrY + scrR + 2 as libc::c_int,
                                scrX + scrR + 1 as libc::c_int,
                                scrY + scrR + 5 as libc::c_int,
                                0x20202 as libc::c_int as uint32);
                    pie_BoxFill(scrX - scrR, scrY + scrR + 3 as libc::c_int,
                                ((scrX - scrR) as
                                     libc::c_uint).wrapping_add(damage) as
                                    libc::c_int,
                                scrY + scrR + 4 as libc::c_int, longPowerCol);
                }
            }
        }
    }
    i = 0 as libc::c_int as UDWORD;
    while i < 8 as libc::c_int as libc::c_uint {
        /* Go thru' all the droidss */
        psDroid = apsDroidLists[i as usize];
        while !psDroid.is_null() {
            if i != selectedPlayer && (*psDroid).died == 0 &&
                   (*psDroid).sDisplay.frameNumber == currentGameFrame {
                /* If it's selected */
                if (*psDroid).bTargetted != 0 &&
                       (*psDroid).visible[selectedPlayer as usize] as
                           libc::c_int == 0xff as libc::c_int {
                    (*psDroid).bTargetted = 0 as libc::c_int;
                    scrX = (*psDroid).sDisplay.screenX as SDWORD;
                    scrY =
                        (*psDroid).sDisplay.screenY.wrapping_sub(8 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint)
                            as SDWORD;
                    index =
                        (IMAGE_BLUE1 as libc::c_int as
                             libc::c_uint).wrapping_add(getTimeValueRange(1020
                                                                              as
                                                                              libc::c_int
                                                                              as
                                                                              UDWORD,
                                                                          5 as
                                                                              libc::c_int
                                                                              as
                                                                              UDWORD));
                    pie_ImageFileID(IntImages, index as UWORD, scrX, scrY);
                }
            }
            psDroid = (*psDroid).psNext
        }
        i = i.wrapping_add(1)
    }
    psFeature = apsFeatureLists[0 as libc::c_int as usize];
    while !psFeature.is_null() {
        if (*psFeature).died == 0 &&
               (*psFeature).sDisplay.frameNumber == currentGameFrame {
            if (*psFeature).bTargetted != 0 {
                (*psFeature).bTargetted = 0 as libc::c_int;
                scrX = (*psFeature).sDisplay.screenX as SDWORD;
                scrY =
                    (*psFeature).sDisplay.screenY.wrapping_sub(((*(*psFeature).sDisplay.imd).ymax
                                                                    /
                                                                    4 as
                                                                        libc::c_int)
                                                                   as
                                                                   libc::c_uint)
                        as SDWORD;
                pie_ImageFileID(IntImages, getTargettingGfx() as UWORD, scrX,
                                scrY);
            }
        }
        psFeature = (*psFeature).psNext
    }
    pie_SetDepthBufferStatus(DEPTH_CMP_LEQ_WRT_ON);
}
/* ---------------------------------------------------------------------------- */
#[no_mangle]
pub unsafe extern "C" fn drawBuildingLines() {
    let mut first: iVector =
        iVector{x: 0, y: 0, z: 0,}; //buildSite.xTL * 128;
    let mut second: iVector =
        iVector{x: 0, y: 0, z: 0,}; //buildSite.yTL * 128;
    if buildState == 102 as libc::c_int as libc::c_uint ||
           buildState == 100 as libc::c_int as libc::c_uint {
        pie_SetDepthBufferStatus(DEPTH_CMP_ALWAYS_WRT_ON); //mouseTileX<<TILE_SHIFT;//buildSite.xBR * 128;
        pie_SetFogStatus(0 as
                             libc::c_int); //mouseTileY<<TILE_SHIFT;//buildSite.yBR * 128;
        first.x = 1000 as libc::c_int;
        first.y = 116 as libc::c_int;
        first.z = 1000 as libc::c_int;
        second.x = 3000 as libc::c_int;
        second.y = 116 as libc::c_int;
        second.z = 3000 as libc::c_int;
        draw3dLine(&mut first, &mut second,
                   (rand() % 255 as libc::c_int) as UBYTE);
        pie_SetDepthBufferStatus(DEPTH_CMP_LEQ_WRT_ON);
    };
}
#[no_mangle]
pub unsafe extern "C" fn drawDroidGroupNumber(mut psDroid: *mut DROID) {
    let mut id: UWORD = 0;
    let mut id2: UDWORD = 0;
    let mut bDraw: BOOL = 0;
    let mut xShift: SDWORD = 0;
    let mut yShift: SDWORD = 0;
    bDraw = 1 as libc::c_int;
    id2 = 0xffffffff as libc::c_uint;
    id = id2 as UWORD;
    /* Is the unit in a group? */
    if !(*psDroid).psGroup.is_null() &&
           (*(*psDroid).psGroup).type_0 as libc::c_int ==
               GT_COMMAND as libc::c_int {
        id2 = IMAGE_GN_STAR as libc::c_int as UDWORD
    }
    //else
    match (*psDroid).group as libc::c_int {
        0 => { id = IMAGE_GN_0 as libc::c_int as UWORD }
        1 => { id = IMAGE_GN_1 as libc::c_int as UWORD }
        2 => { id = IMAGE_GN_2 as libc::c_int as UWORD }
        3 => { id = IMAGE_GN_3 as libc::c_int as UWORD }
        4 => { id = IMAGE_GN_4 as libc::c_int as UWORD }
        5 => { id = IMAGE_GN_5 as libc::c_int as UWORD }
        6 => { id = IMAGE_GN_6 as libc::c_int as UWORD }
        7 => { id = IMAGE_GN_7 as libc::c_int as UWORD }
        8 => { id = IMAGE_GN_8 as libc::c_int as UWORD }
        9 => { id = IMAGE_GN_9 as libc::c_int as UWORD }
        _ => { bDraw = 0 as libc::c_int }
    } // yeah yeah, I know
    if bDraw != 0 {
        xShift = 28 as libc::c_int;
        yShift = 17 as libc::c_int;
        xShift =
            (xShift as
                 libc::c_uint).wrapping_mul(pie_GetResScalingFactor()).wrapping_div(100
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        libc::c_uint)
                as SDWORD;
        yShift =
            (yShift as
                 libc::c_uint).wrapping_mul(pie_GetResScalingFactor()).wrapping_div(100
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        libc::c_uint)
                as SDWORD;
        pie_ImageFileID(IntImages, id,
                        (*psDroid).sDisplay.screenX.wrapping_sub(xShift as
                                                                     libc::c_uint)
                            as libc::c_int,
                        (*psDroid).sDisplay.screenY.wrapping_add(yShift as
                                                                     libc::c_uint)
                            as libc::c_int);
        if id2 != 0xffffffff as libc::c_uint {
            pie_ImageFileID(IntImages, id2 as UWORD,
                            (*psDroid).sDisplay.screenX.wrapping_sub(xShift as
                                                                         libc::c_uint)
                                as libc::c_int,
                            (*psDroid).sDisplay.screenY.wrapping_add(yShift as
                                                                         libc::c_uint).wrapping_sub(8
                                                                                                        as
                                                                                                        libc::c_int
                                                                                                        as
                                                                                                        libc::c_uint)
                                as libc::c_int);
        }
    };
}
/* ---------------------------------------------------------------------------- */
#[no_mangle]
pub unsafe extern "C" fn drawDroidCmndNo(mut psDroid: *mut DROID) {
    let mut id: UWORD = 0; // yeah yeah, I know
    let mut id2: UDWORD = 0;
    let mut bDraw: BOOL = 0;
    let mut xShift: SDWORD = 0;
    let mut yShift: SDWORD = 0;
    let mut index: SDWORD = 0;
    bDraw = 1 as libc::c_int;
    id2 = 0xffffffff as libc::c_uint;
    id = id2 as UWORD;
    id2 = IMAGE_GN_STAR as libc::c_int as UDWORD;
    index = 0x7fffffff as libc::c_int;
    if (*psDroid).droidType as libc::c_uint ==
           DROID_COMMAND as libc::c_int as libc::c_uint {
        index = cmdDroidGetIndex(psDroid)
    } else if !(*psDroid).psGroup.is_null() &&
                  (*(*psDroid).psGroup).type_0 as libc::c_int ==
                      GT_COMMAND as libc::c_int {
        index = cmdDroidGetIndex((*(*psDroid).psGroup).psCommander)
    }
    match index {
        1 => { id = IMAGE_GN_1 as libc::c_int as UWORD }
        2 => { id = IMAGE_GN_2 as libc::c_int as UWORD }
        3 => { id = IMAGE_GN_3 as libc::c_int as UWORD }
        4 => { id = IMAGE_GN_4 as libc::c_int as UWORD }
        5 => { id = IMAGE_GN_5 as libc::c_int as UWORD }
        6 => { id = IMAGE_GN_6 as libc::c_int as UWORD }
        7 => { id = IMAGE_GN_7 as libc::c_int as UWORD }
        8 => { id = IMAGE_GN_8 as libc::c_int as UWORD }
        9 => { id = IMAGE_GN_9 as libc::c_int as UWORD }
        _ => { bDraw = 0 as libc::c_int }
    }
    if bDraw != 0 {
        xShift = 28 as libc::c_int;
        yShift = 17 as libc::c_int;
        xShift =
            (xShift as
                 libc::c_uint).wrapping_mul(pie_GetResScalingFactor()).wrapping_div(100
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        libc::c_uint)
                as SDWORD;
        yShift =
            (yShift as
                 libc::c_uint).wrapping_mul(pie_GetResScalingFactor()).wrapping_div(100
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        libc::c_uint)
                as SDWORD;
        pie_ImageFileID(IntImages, id2 as UWORD,
                        (*psDroid).sDisplay.screenX.wrapping_sub(xShift as
                                                                     libc::c_uint).wrapping_sub(6
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    libc::c_uint)
                            as libc::c_int,
                        (*psDroid).sDisplay.screenY.wrapping_add(yShift as
                                                                     libc::c_uint)
                            as libc::c_int);
        pie_ImageFileID(IntImages, id,
                        (*psDroid).sDisplay.screenX.wrapping_sub(xShift as
                                                                     libc::c_uint)
                            as libc::c_int,
                        (*psDroid).sDisplay.screenY.wrapping_add(yShift as
                                                                     libc::c_uint)
                            as libc::c_int);
    };
}
/* ---------------------------------------------------------------------------- */
/* ---------------------------------------------------------------------------- */
#[no_mangle]
pub unsafe extern "C" fn draw3dLine(mut src: *mut iVector,
                                    mut dest: *mut iVector, mut col: UBYTE) {
    let mut null: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut vec: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut srcS: iPoint = iPoint{x: 0, y: 0,};
    let mut destS: iPoint = iPoint{x: 0, y: 0,};
    null.z = 0 as libc::c_int;
    null.y = null.z;
    null.x = null.y;
    vec.x =
        (((*src).x - player.p.x) as
             libc::c_uint).wrapping_sub(terrainMidX.wrapping_mul(128 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint))
            as int32;
    vec.z =
        terrainMidY.wrapping_mul(128 as libc::c_int as
                                     libc::c_uint).wrapping_sub(((*src).z -
                                                                     player.p.z)
                                                                    as
                                                                    libc::c_uint)
            as int32;
    vec.y = (*src).y;
    pie_MatBegin();
    /* Translate */
    pie_TRANSLATE(vec.x, vec.y, vec.z);
    rx = player.p.x & 128 as libc::c_int - 1 as libc::c_int;
    rz = player.p.z & 128 as libc::c_int - 1 as libc::c_int;
    /* Translate */
    pie_TRANSLATE(rx, 0 as libc::c_int, -rz);
    /* Project - no rotation being done */
    pie_RotProj(&mut null, &mut srcS);
    pie_MatEnd();
    vec.x =
        (((*dest).x - player.p.x) as
             libc::c_uint).wrapping_sub(terrainMidX.wrapping_mul(128 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint))
            as int32;
    vec.z =
        terrainMidY.wrapping_mul(128 as libc::c_int as
                                     libc::c_uint).wrapping_sub(((*dest).z -
                                                                     player.p.z)
                                                                    as
                                                                    libc::c_uint)
            as int32;
    vec.y = (*dest).y;
    pie_MatBegin();
    /* Translate */
    pie_TRANSLATE(vec.x, vec.y, vec.z);
    rx = player.p.x & 128 as libc::c_int - 1 as libc::c_int;
    rz = player.p.z & 128 as libc::c_int - 1 as libc::c_int;
    /* Translate */
    pie_TRANSLATE(rx, 0 as libc::c_int, -rz);
    /* Project - no rotation being done */
    pie_RotProj(&mut null, &mut destS);
    pie_MatEnd();
    pie_Line(srcS.x, srcS.y, destS.x, destS.y, col as uint32);
}
/*	Get the onscreen corrdinates of a droid - so we can draw a bounding box - this need to be severely
	speeded up and the accuracy increased to allow variable size bouding boxes */
#[no_mangle]
pub unsafe extern "C" fn calcScreenCoords(mut psDroid: *mut DROID) {
    //BOOL	setMouse = FALSE;
    let mut centX: SDWORD = 0;
    let mut centY: SDWORD = 0;
    let mut centZ: SDWORD = 0;
    let mut cX: SDWORD = 0;
    let mut cY: SDWORD = 0;
    let mut cZ: SDWORD = 0;
    let mut imd: *mut iIMDShape = 0 as *mut iIMDShape;
    let mut radius: UDWORD = 0;
    let mut pt: POINT = POINT{x: 0, y: 0,};
    /* Ennsure correct context */
    /* which IMD are we using? */
    imd = (*psDroid).sDisplay.imd;
    /* Get it's absolute dimensions */
    centZ = 0 as libc::c_int;
    centY = centZ;
    centX = centY;
    /* How big a box do we want - will ultimately be calculated using xmax, ymax, zmax etc */
    if (*psDroid).droidType as libc::c_uint ==
           DROID_TRANSPORTER as libc::c_int as libc::c_uint {
        radius = 45 as libc::c_int as UDWORD
    } else { radius = 22 as libc::c_int as UDWORD }
    /* Pop matrices and get the screen corrdinates */
    cZ = pie_RotateProject(centX, centY, centZ, &mut cX, &mut cY);
    radius =
        radius.wrapping_mul(pie_GetResScalingFactor()).wrapping_mul(80 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_uint).wrapping_div(cZ
                                                                                                       as
                                                                                                       libc::c_uint);
    /* Deselect all the droids if we've released the drag box */
    if dragBox3D.status == 2 as libc::c_int as libc::c_uint {
        pt.x = cX;
        pt.y = cY;
        if inQuad(&mut pt, &mut dragQuad) != 0 &&
               (*psDroid).player as libc::c_uint == selectedPlayer {
            //don't allow Transporter Droids to be selected here
            //unless we're in multiPlayer mode!!!!
            if (*psDroid).droidType as libc::c_uint !=
                   DROID_TRANSPORTER as libc::c_int as libc::c_uint ||
                   bMultiPlayer != 0 {
                dealWithDroidSelect(psDroid, 1 as libc::c_int);
                /*				psDroid->selected = TRUE;
				if (bInTutorial)
				{
					psCBSelectedDroid = psDroid;
					eventFireCallbackTrigger(CALL_DROID_SELECTED);
					psCBSelectedDroid = NULL;
				}*/
            }
        }
    }
    cY -= 4 as libc::c_int;
    /* Store away the screen coordinates so we can select the droids without doing a trasform */
    (*psDroid).sDisplay.screenX = cX as UDWORD;
    (*psDroid).sDisplay.screenY = cY as UDWORD;
    (*psDroid).sDisplay.screenR = radius;
}
#[no_mangle]
pub unsafe extern "C" fn preprocessTiles() {
    let mut i: UDWORD = 0;
    let mut j: UDWORD = 0;
    let mut left: UDWORD = 0;
    let mut right: UDWORD = 0;
    let mut up: UDWORD = 0;
    let mut down: UDWORD = 0;
    let mut size: UDWORD = 0;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    //BASE_OBJECT *psObj;
    let mut order: SDWORD = 0;
    //UDWORD	tileCount;
    /* Set up the highlights if we're putting down a wall */
    if wallDrag.status == 3 as libc::c_int as libc::c_uint ||
           wallDrag.status == 1 as libc::c_int as libc::c_uint {
        /* Ensure the start point is always shown */
        (*mapTile(wallDrag.x1, wallDrag.y1)).texture =
            ((*mapTile(wallDrag.x1, wallDrag.y1)).texture as libc::c_int |
                 0x400 as libc::c_int) as UWORD;
        if wallDrag.x1 == wallDrag.x2 || wallDrag.y1 == wallDrag.y2 {
            /* First process the ones inside the wall dragging area */
            left =
                if wallDrag.x1 < wallDrag.x2 {
                    wallDrag.x1
                } else { wallDrag.x2 };
            right =
                (if wallDrag.x1 > wallDrag.x2 {
                     wallDrag.x1
                 } else {
                     wallDrag.x2
                 }).wrapping_add(1 as libc::c_int as libc::c_uint);
            up =
                if wallDrag.y1 < wallDrag.y2 {
                    wallDrag.y1
                } else { wallDrag.y2 };
            down =
                (if wallDrag.y1 > wallDrag.y2 {
                     wallDrag.y1
                 } else {
                     wallDrag.y2
                 }).wrapping_add(1 as libc::c_int as libc::c_uint);
            i = left;
            while i < right {
                j = up;
                while j < down {
                    (*mapTile(i, j)).texture =
                        ((*mapTile(i, j)).texture as libc::c_int |
                             0x400 as libc::c_int) as UWORD;
                    j = j.wrapping_add(1)
                }
                i = i.wrapping_add(1)
            }
        }
    } else if buildState == 102 as libc::c_int as libc::c_uint ||
                  buildState == 100 as libc::c_int as libc::c_uint {
        /* Only bother if we're placing a building */
        /* Now do the ones inside the building highlight */
        left = buildSite.xTL as UDWORD;
        right = (buildSite.xBR as libc::c_int + 1 as libc::c_int) as UDWORD;
        up = buildSite.yTL as UDWORD;
        down = (buildSite.yBR as libc::c_int + 1 as libc::c_int) as UDWORD;
        i = left;
        while i < right {
            j = up;
            while j < down {
                (*mapTile(i, j)).texture =
                    ((*mapTile(i, j)).texture as libc::c_int |
                         0x400 as libc::c_int) as UWORD;
                j = j.wrapping_add(1)
                //				tileCount++;
//				averageHeight+=map_TileHeight(i,j);
            }
            i = i.wrapping_add(1)
        }
    }
    //don't display until we're releasing this feature in an update!
    if intBuildSelectMode() != 0 {
        //and there may be multiple building sites that need highlighting - AB 26/04/99
        if ctrlShiftDown() != 0 {
            //this just highlights the current interface selected unit
            //psObj = getCurrentSelected();
            //if (psObj AND psObj->type == OBJ_DROID)
            //this highlights ALL constructor units' build sites
            psDroid = apsDroidLists[selectedPlayer as usize];
            while !psDroid.is_null() {
                //psDroid = (DROID *)psObj;
                if (*psDroid).droidType as libc::c_uint ==
                       DROID_CONSTRUCT as libc::c_int as libc::c_uint ||
                       (*psDroid).droidType as libc::c_uint ==
                           DROID_CYBORG_CONSTRUCT as libc::c_int as
                               libc::c_uint {
                    //draw the current build site if its a line of structures
                    if (*psDroid).order == DORDER_LINEBUILD as libc::c_int {
                        left =
                            ((*psDroid).orderX as libc::c_int >>
                                 7 as libc::c_int) as UDWORD;
                        right =
                            (((*psDroid).orderX2 as libc::c_int >>
                                  7 as libc::c_int) + 1 as libc::c_int) as
                                UDWORD;
                        if left > right {
                            size = left;
                            left = right;
                            right = size
                        }
                        up =
                            ((*psDroid).orderY as libc::c_int >>
                                 7 as libc::c_int) as UDWORD;
                        down =
                            (((*psDroid).orderY2 as libc::c_int >>
                                  7 as libc::c_int) + 1 as libc::c_int) as
                                UDWORD;
                        if up > down { size = up; up = down; down = size }
                        //hilight the tiles
                        i = left;
                        while i < right {
                            j = up;
                            while j < down {
                                (*mapTile(i, j)).texture =
                                    ((*mapTile(i, j)).texture as libc::c_int |
                                         0x400 as libc::c_int) as UWORD;
                                j = j.wrapping_add(1)
                            }
                            i = i.wrapping_add(1)
                        }
                    }
                    //now look thru' the list of orders to see if more building sites
                    order = 0 as libc::c_int;
                    while order < (*psDroid).listSize {
                        if (*psDroid).asOrderList[order as usize].order ==
                               DORDER_BUILD as libc::c_int {
                            //set up coords for tiles
                            size =
                                (*((*psDroid).asOrderList[order as
                                                              usize].psOrderTarget
                                       as *mut STRUCTURE_STATS)).baseWidth;
                            left =
                                (((*psDroid).asOrderList[order as usize].x as
                                      libc::c_int >> 7 as libc::c_int) as
                                     libc::c_uint).wrapping_sub(size.wrapping_div(2
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      libc::c_uint));
                            right = left.wrapping_add(size);
                            size =
                                (*((*psDroid).asOrderList[order as
                                                              usize].psOrderTarget
                                       as *mut STRUCTURE_STATS)).baseBreadth;
                            up =
                                (((*psDroid).asOrderList[order as usize].y as
                                      libc::c_int >> 7 as libc::c_int) as
                                     libc::c_uint).wrapping_sub(size.wrapping_div(2
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      libc::c_uint));
                            down = up.wrapping_add(size);
                            //hilight the tiles
                            i = left;
                            while i < right {
                                j = up;
                                while j < down {
                                    (*mapTile(i, j)).texture =
                                        ((*mapTile(i, j)).texture as
                                             libc::c_int |
                                             0x400 as libc::c_int) as UWORD;
                                    j = j.wrapping_add(1)
                                }
                                i = i.wrapping_add(1)
                            }
                        } else if (*psDroid).asOrderList[order as usize].order
                                      == DORDER_LINEBUILD as libc::c_int {
                            //need to highlight the length of the wall
                            left =
                                ((*psDroid).asOrderList[order as usize].x as
                                     libc::c_int >> 7 as libc::c_int) as
                                    UDWORD;
                            right =
                                ((*psDroid).asOrderList[order as usize].x2 as
                                     libc::c_int >> 7 as libc::c_int) as
                                    UDWORD;
                            if left > right {
                                size = left;
                                left = right;
                                right = size
                            }
                            up =
                                ((*psDroid).asOrderList[order as usize].y as
                                     libc::c_int >> 7 as libc::c_int) as
                                    UDWORD;
                            down =
                                ((*psDroid).asOrderList[order as usize].y2 as
                                     libc::c_int >> 7 as libc::c_int) as
                                    UDWORD;
                            if up > down { size = up; up = down; down = size }
                            //hilight the tiles
                            i = left;
                            while i <= right {
                                j = up;
                                while j <= down {
                                    (*mapTile(i, j)).texture =
                                        ((*mapTile(i, j)).texture as
                                             libc::c_int |
                                             0x400 as libc::c_int) as UWORD;
                                    j = j.wrapping_add(1)
                                }
                                i = i.wrapping_add(1)
                            }
                        }
                        order += 1
                    }
                }
                psDroid = (*psDroid).psNext
            }
        }
    };
    //	if(tileCount)
//	{
    //		averageHeight = averageHeight/tileCount;
//	}
}
//this never gets called!
/*void	postprocessTiles(void)
{
UDWORD	i,j;
UDWORD	left,right,up,down;
MAPTILE	*psTile;

	// Clear the highlights if we're putting down a wall
	if(wallDrag.status == DRAG_PLACING OR wallDrag.status == DRAG_DRAGGING)
	{
		// Only show valid walls
		if( (wallDrag.x1 == wallDrag.x2) OR (wallDrag.y1 == wallDrag.y2) )
		{
			// First process the ones inside the wall dragging area
			left = min(wallDrag.x1,wallDrag.x2);
			right = max(wallDrag.x1,wallDrag.x2);
			up = min(wallDrag.y1,wallDrag.y2);
			down = max(wallDrag.y1,wallDrag.y2);

			for(i=left; i<right; i++)
			{
				for(j=up; j<down; j++)
				{
					psTile = mapTile(i,j);
					CLEAR_TILE_HIGHLIGHT(psTile);
				}
			}
		}
	}
	// Now do the ones inside the building highlight
	left = buildSite.xTL;
	right = buildSite.xBR;
	up = buildSite.yTL;
	down = buildSite.yBR;

	for(i=left; i<right; i++)
	{
		for(j=up; j<down; j++)
		{
			psTile = mapTile(i,j);
			CLEAR_TILE_HIGHLIGHT(psTile);
		}
	}
}*/
/* This is slow - speed it up */
#[no_mangle]
pub unsafe extern "C" fn locateMouse() {
    let mut i: UDWORD = 0;
    let mut j: UDWORD = 0;
    let mut pt: POINT = POINT{x: 0, y: 0,};
    let mut quad: QUAD = QUAD{coords: [POINT{x: 0, y: 0,}; 4],};
    let mut nearestZ: SDWORD = 0x7fffffff as libc::c_int;
    let mut tileZ_0: SDWORD = 0;
    let mut bWaterTile: BOOL = 0;
    //UDWORD	bX,bY;
    pt.x = mX;
    pt.y = mY;
    i = 0 as libc::c_int as UDWORD;
    while i < visibleXTiles {
        j = 0 as libc::c_int as UDWORD;
        while j < visibleYTiles {
            bWaterTile =
                tileScreenInfo[i as usize][j as usize].bWater as BOOL;
            tileZ_0 =
                if bWaterTile != 0 {
                    tileScreenInfo[i as usize][j as usize].wz
                } else { tileScreenInfo[i as usize][j as usize].sz };
            if tileZ_0 <= nearestZ {
                quad.coords[0 as libc::c_int as usize].x =
                    if bWaterTile != 0 {
                        tileScreenInfo[i as usize][j as usize].wx
                    } else { tileScreenInfo[i as usize][j as usize].sx };
                quad.coords[0 as libc::c_int as usize].y =
                    if bWaterTile != 0 {
                        tileScreenInfo[i as usize][j as usize].wy
                    } else { tileScreenInfo[i as usize][j as usize].sy };
                quad.coords[1 as libc::c_int as usize].x =
                    if bWaterTile != 0 {
                        tileScreenInfo[i as
                                           usize][j.wrapping_add(1 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint)
                                                      as usize].wx
                    } else {
                        tileScreenInfo[i as
                                           usize][j.wrapping_add(1 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint)
                                                      as usize].sx
                    };
                quad.coords[1 as libc::c_int as usize].y =
                    if bWaterTile != 0 {
                        tileScreenInfo[i as
                                           usize][j.wrapping_add(1 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint)
                                                      as usize].wy
                    } else {
                        tileScreenInfo[i as
                                           usize][j.wrapping_add(1 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint)
                                                      as usize].sy
                    };
                quad.coords[2 as libc::c_int as usize].x =
                    if bWaterTile != 0 {
                        tileScreenInfo[i.wrapping_add(1 as libc::c_int as
                                                          libc::c_uint) as
                                           usize][j.wrapping_add(1 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint)
                                                      as usize].wx
                    } else {
                        tileScreenInfo[i.wrapping_add(1 as libc::c_int as
                                                          libc::c_uint) as
                                           usize][j.wrapping_add(1 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint)
                                                      as usize].sx
                    };
                quad.coords[2 as libc::c_int as usize].y =
                    if bWaterTile != 0 {
                        tileScreenInfo[i.wrapping_add(1 as libc::c_int as
                                                          libc::c_uint) as
                                           usize][j.wrapping_add(1 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint)
                                                      as usize].wy
                    } else {
                        tileScreenInfo[i.wrapping_add(1 as libc::c_int as
                                                          libc::c_uint) as
                                           usize][j.wrapping_add(1 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint)
                                                      as usize].sy
                    };
                quad.coords[3 as libc::c_int as usize].x =
                    if bWaterTile != 0 {
                        tileScreenInfo[i.wrapping_add(1 as libc::c_int as
                                                          libc::c_uint) as
                                           usize][j as usize].wx
                    } else {
                        tileScreenInfo[i.wrapping_add(1 as libc::c_int as
                                                          libc::c_uint) as
                                           usize][j as usize].sx
                    };
                quad.coords[3 as libc::c_int as usize].y =
                    if bWaterTile != 0 {
                        tileScreenInfo[i.wrapping_add(1 as libc::c_int as
                                                          libc::c_uint) as
                                           usize][j as usize].wy
                    } else {
                        tileScreenInfo[i.wrapping_add(1 as libc::c_int as
                                                          libc::c_uint) as
                                           usize][j as usize].sy
                    };
                /* We've got a match for our mouse coords */
                if inQuad(&mut pt, &mut quad) != 0 {
                    mouseTileX =
                        (playerXTile as libc::c_uint).wrapping_add(j) as
                            SDWORD;
                    mouseTileY =
                        (playerZTile as libc::c_uint).wrapping_add(i) as
                            SDWORD;
                    if mouseTileX < 0 as libc::c_int {
                        mouseTileX = 0 as libc::c_int
                    }
                    if mouseTileX as libc::c_uint >
                           mapWidth.wrapping_sub(1 as libc::c_int as
                                                     libc::c_uint) {
                        mouseTileX =
                            mapWidth.wrapping_sub(1 as libc::c_int as
                                                      libc::c_uint) as SDWORD
                    }
                    if mouseTileY < 0 as libc::c_int {
                        mouseTileY = 0 as libc::c_int
                    }
                    if mouseTileY as libc::c_uint >
                           mapHeight.wrapping_sub(1 as libc::c_int as
                                                      libc::c_uint) {
                        mouseTileY =
                            mapHeight.wrapping_sub(1 as libc::c_int as
                                                       libc::c_uint) as SDWORD
                    }
                    tile3dX = (playerXTile as libc::c_uint).wrapping_add(j);
                    tile3dY = (playerZTile as libc::c_uint).wrapping_add(i);
                    if tile3dX > 0 as libc::c_int as libc::c_uint &&
                           tile3dY > 0 as libc::c_int as libc::c_uint &&
                           tile3dX <=
                               mapWidth.wrapping_sub(1 as libc::c_int as
                                                         libc::c_uint) &&
                           tile3dY <=
                               mapHeight.wrapping_sub(1 as libc::c_int as
                                                          libc::c_uint) {
                        tile3dOver = mapTile(tile3dX, tile3dY)
                    }
                    /* Store away z value */
                    nearestZ = tileZ_0
                }
            }
            j = j.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    };
}
/*
HACK for IAN VISIT, but works....
FIXME: This is function is not used. Should it be? - Per
*/
#[no_mangle]
pub unsafe extern "C" fn renderSky() {
    let texPage: libc::c_int = 0 as libc::c_int;
    let mut skyShift: SDWORD = 0;
    let mut index: SDWORD = 0;
    let mut width: SDWORD = 0;
    skyShift += 2 as libc::c_int;
    index = (player.r.y + skyShift) / 20 as libc::c_int;
    while index > 0 as libc::c_int { index -= 256 as libc::c_int }
    while index < 640 as libc::c_int {
        if index >= 384 as libc::c_int {
            width = 640 as libc::c_int - index
        } else { width = 256 as libc::c_int }
        iV_UniBitmapDepth.expect("non-null function pointer")(texPage,
                                                              0 as
                                                                  libc::c_int,
                                                              0 as
                                                                  libc::c_int,
                                                              width,
                                                              128 as
                                                                  libc::c_int,
                                                              index,
                                                              0 as
                                                                  libc::c_int,
                                                              width,
                                                              192 as
                                                                  libc::c_int,
                                                              200 as
                                                                  libc::c_int
                                                                  as
                                                                  libc::c_uchar,
                                                              65000 as
                                                                  libc::c_int);
        index += 256 as libc::c_int
    }
    index = (player.r.y + skyShift / 2 as libc::c_int) / 15 as libc::c_int;
    while index > 0 as libc::c_int { index -= 256 as libc::c_int }
    while index < 640 as libc::c_int {
        if index >= 384 as libc::c_int {
            width = 640 as libc::c_int - index
        } else { width = 256 as libc::c_int }
        iV_UniBitmapDepth.expect("non-null function pointer")(texPage,
                                                              0 as
                                                                  libc::c_int,
                                                              128 as
                                                                  libc::c_int,
                                                              width,
                                                              127 as
                                                                  libc::c_int,
                                                              index,
                                                              0 as
                                                                  libc::c_int,
                                                              width,
                                                              192 as
                                                                  libc::c_int,
                                                              200 as
                                                                  libc::c_int
                                                                  as
                                                                  libc::c_uchar,
                                                              64000 as
                                                                  libc::c_int);
        index += 256 as libc::c_int
    };
}
/* Flattens an imd to the landscape and handles 4 different rotations */
#[no_mangle]
pub unsafe extern "C" fn flattenImd(mut imd: *mut iIMDShape,
                                    mut structX: UDWORD, mut structY: UDWORD,
                                    mut direction: UDWORD) -> *mut iIMDShape {
    let mut i: UDWORD = 0;
    let mut pointHeight: UDWORD = 0;
    let mut centreHeight: UDWORD = 0;
    let mut shift: SDWORD = 0;
    // CHECK WHETHER THE NUMBER OF POINTS IN THE IMD WILL FIT IN THE ARRAY
    if (*imd).npoints < 255 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"flattenImd: too many points in the PIE to flatten it\x00" as
                  *const u8 as *const libc::c_char);
    };
    if (*imd).npoints < 255 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"display3d.c\x00" as *const u8 as *const libc::c_char,
              4668 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 11],
                                        &[libc::c_char; 11]>(b"flattenImd\x00")).as_ptr(),
              b"imd->npoints < MAX_FLATTEN_POINTS\x00" as *const u8 as
                  *const libc::c_char);
    };
    /* Get a copy of the points */
    memcpy(alteredPoints.as_mut_ptr() as *mut libc::c_void,
           (*imd).points as *const libc::c_void,
           ((*imd).npoints as
                libc::c_uint).wrapping_mul(::std::mem::size_of::<iVector>() as
                                               libc::c_ulong));
    /* Get the height of the centre point for reference */
    centreHeight = map_Height(structX, structY) as UDWORD;
    /* Now we go through the shape looking for vertices on the edge */
  	/* Flip reference coords if we're on a vertical wall */
    /* Little hack below 'cos sometimes they're not exactly 90 degree alligned. */
    direction =
        (direction as
             libc::c_uint).wrapping_div(90 as libc::c_int as libc::c_uint) as
            UDWORD as UDWORD;
    direction =
        (direction as
             libc::c_uint).wrapping_mul(90 as libc::c_int as libc::c_uint) as
            UDWORD as UDWORD;
    match direction {
        0 => {
            i = 0 as libc::c_int as UDWORD;
            while i < (*imd).npoints as UDWORD {
                if abs(alteredPoints[i as usize].x) >= 63 as libc::c_int ||
                       abs(alteredPoints[i as usize].z) >= 63 as libc::c_int {
                    pointHeight =
                        map_Height(structX.wrapping_add(alteredPoints[i as
                                                                          usize].x
                                                            as libc::c_uint),
                                   structY.wrapping_sub(alteredPoints[i as
                                                                          usize].z
                                                            as libc::c_uint))
                            as UDWORD;
                    shift = centreHeight.wrapping_sub(pointHeight) as SDWORD;
                    alteredPoints[i as usize].y -= shift - 4 as libc::c_int
                }
                i = i.wrapping_add(1)
            }
        }
        90 => {
            i = 0 as libc::c_int as UDWORD;
            while i < (*imd).npoints as UDWORD {
                if abs(alteredPoints[i as usize].x) >= 63 as libc::c_int ||
                       abs(alteredPoints[i as usize].z) >= 63 as libc::c_int {
                    pointHeight =
                        map_Height(structX.wrapping_sub(alteredPoints[i as
                                                                          usize].z
                                                            as libc::c_uint),
                                   structY.wrapping_sub(alteredPoints[i as
                                                                          usize].x
                                                            as libc::c_uint))
                            as UDWORD;
                    shift = centreHeight.wrapping_sub(pointHeight) as SDWORD;
                    alteredPoints[i as usize].y -= shift - 4 as libc::c_int
                }
                i = i.wrapping_add(1)
            }
        }
        180 => {
            i = 0 as libc::c_int as UDWORD;
            while i < (*imd).npoints as UDWORD {
                if abs(alteredPoints[i as usize].x) >= 63 as libc::c_int ||
                       abs(alteredPoints[i as usize].z) >= 63 as libc::c_int {
                    pointHeight =
                        map_Height(structX.wrapping_sub(alteredPoints[i as
                                                                          usize].x
                                                            as libc::c_uint),
                                   structY.wrapping_add(alteredPoints[i as
                                                                          usize].z
                                                            as libc::c_uint))
                            as UDWORD;
                    shift = centreHeight.wrapping_sub(pointHeight) as SDWORD;
                    alteredPoints[i as usize].y -= shift - 4 as libc::c_int
                }
                i = i.wrapping_add(1)
            }
        }
        270 => {
            i = 0 as libc::c_int as UDWORD;
            while i < (*imd).npoints as UDWORD {
                if abs(alteredPoints[i as usize].x) >= 63 as libc::c_int ||
                       abs(alteredPoints[i as usize].z) >= 63 as libc::c_int {
                    pointHeight =
                        map_Height(structX.wrapping_add(alteredPoints[i as
                                                                          usize].z
                                                            as libc::c_uint),
                                   structY.wrapping_add(alteredPoints[i as
                                                                          usize].x
                                                            as libc::c_uint))
                            as UDWORD;
                    shift = centreHeight.wrapping_sub(pointHeight) as SDWORD;
                    alteredPoints[i as usize].y -= shift - 4 as libc::c_int
                }
                i = i.wrapping_add(1)
            }
        }
        _ => {
            debug(LOG_ERROR,
                  b"Weirdy direction for a structure in renderWall\x00" as
                      *const u8 as *const libc::c_char);
            abort();
        }
    }
    /*
			if (psStructure-> direction == 90)
			{
				for(i=0; i<imd->npoints; i++)
		  		{
						pointHeight = map_Height(structX-alteredPoints[i].z,structY-alteredPoints[i].x);
		  				shift = centreHeight - pointHeight;
						alteredPoints[i].y -= shift;
		  		}
			}
			else
			{
				for(i=0; i<imd->npoints; i++)
		  		{
						pointHeight = map_Height(structX+alteredPoints[i].x,structY-alteredPoints[i].z);
		  				shift = centreHeight - pointHeight;
						alteredPoints[i].y -= shift;
		  		}
			}
*/
    (*imd).points = alteredPoints.as_mut_ptr();
    return imd;
}
#[no_mangle]
pub unsafe extern "C" fn getDefaultColours() {
    defaultColours.red =
        pal_GetNearestColour(255 as libc::c_int as uint8,
                             0 as libc::c_int as uint8,
                             0 as libc::c_int as uint8);
    defaultColours.green =
        pal_GetNearestColour(0 as libc::c_int as uint8,
                             255 as libc::c_int as uint8,
                             0 as libc::c_int as uint8);
    defaultColours.blue =
        pal_GetNearestColour(0 as libc::c_int as uint8,
                             0 as libc::c_int as uint8,
                             255 as libc::c_int as uint8);
    defaultColours.yellow =
        pal_GetNearestColour(255 as libc::c_int as uint8,
                             255 as libc::c_int as uint8,
                             0 as libc::c_int as uint8);
    defaultColours.purple =
        pal_GetNearestColour(255 as libc::c_int as uint8,
                             0 as libc::c_int as uint8,
                             255 as libc::c_int as uint8);
    defaultColours.cyan =
        pal_GetNearestColour(0 as libc::c_int as uint8,
                             255 as libc::c_int as uint8,
                             255 as libc::c_int as uint8);
    defaultColours.black =
        pal_GetNearestColour(0 as libc::c_int as uint8,
                             0 as libc::c_int as uint8,
                             0 as libc::c_int as uint8);
    defaultColours.white =
        pal_GetNearestColour(255 as libc::c_int as uint8,
                             255 as libc::c_int as uint8,
                             255 as libc::c_int as uint8);
}
// -------------------------------------------------------------------------------------
/* New improved (and much faster) tile drawer */
// -------------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn drawTerrainTile(mut i: UDWORD, mut j: UDWORD) {
    let mut actualX: SDWORD = 0;
    let mut actualY: SDWORD = 0;
    let mut psTile: *mut MAPTILE = 0 as *mut MAPTILE;
    let mut bOutlined: BOOL = 0;
    let mut tileNumber: UDWORD = 0;
    let mut renderFlag: UDWORD = 0;
    let mut offset: iPoint = iPoint{x: 0, y: 0,};
    let mut aVrts: [PIEVERTEX; 3] =
        [PIEVERTEX{sx: 0,
                   sy: 0,
                   sz: 0,
                   tu: 0,
                   tv: 0,
                   light:
                       PIELIGHT{byte:
                                    PIELIGHTBYTES{b: 0, g: 0, r: 0, a: 0,},},
                   specular:
                       PIELIGHT{byte:
                                    PIELIGHTBYTES{b: 0,
                                                  g: 0,
                                                  r: 0,
                                                  a: 0,},},}; 3];
    let mut oldColours: [BYTE; 4] =
        [0 as libc::c_int as BYTE, 0 as libc::c_int as BYTE,
         0 as libc::c_int as BYTE, 0 as libc::c_int as BYTE];
    let mut oldColoursWord: [UDWORD; 4] =
        [0 as libc::c_int as UDWORD, 0 as libc::c_int as UDWORD,
         0 as libc::c_int as UDWORD, 0 as libc::c_int as UDWORD];
    /* Get the correct tile index for the x coordinate */
    actualX = (playerXTile as libc::c_uint).wrapping_add(j) as SDWORD;
    /* Get the correct tile index for the y coordinate */
    actualY = (playerZTile as libc::c_uint).wrapping_add(i) as SDWORD;
    //	ASSERT( actualX<mapWidth,"X Coordinate invalid in tile draw" );
   //	ASSERT( actualY<mapWidth,"Y Coordinate invalid in tile draw" );
    /* Let's just get out now if we're not supposed to draw it */
    if actualX < 0 as libc::c_int || actualY < 0 as libc::c_int ||
           actualX as libc::c_uint >
               mapWidth.wrapping_sub(1 as libc::c_int as libc::c_uint) ||
           actualY as libc::c_uint >
               mapHeight.wrapping_sub(1 as libc::c_int as libc::c_uint) {
        psTile = &mut edgeTile;
        (*psTile).texture =
            ((*psTile).texture as libc::c_int & !(0x400 as libc::c_int)) as
                UWORD
    } else { psTile = mapTile(actualX as UDWORD, actualY as UDWORD) }
    if (*psTile).tileInfoBits as libc::c_int & 0x4 as libc::c_int != 0 {
        /* This tile isn't being drawn */
        return
    }
    /* what tile texture number is it? */
    tileNumber = (*psTile).texture as UDWORD;
    // If it's a water tile then force it to be the river bed texture.
    if terrainTypes[((*psTile).texture as libc::c_int & 0x1ff as libc::c_int)
                        as usize] as libc::c_int == TER_WATER as libc::c_int {
        tileNumber = RiverBedTileID as UDWORD
    }
    /* Is the tile highlighted? Perhaps because there's a building foundation on it */
    bOutlined = 0 as libc::c_int;
    if (*psTile).texture as libc::c_int & 0x400 as libc::c_int != 0 {
        /* Clear it for next time round */
        (*psTile).texture =
            ((*psTile).texture as libc::c_int & !(0x400 as libc::c_int)) as
                UWORD;
        bOutlined = 1 as libc::c_int;
        //set tilenumber
//		tileNumber = FOUNDATION_TEXTURE;
        if i <
               (32 as libc::c_int + 1 as libc::c_int - 1 as libc::c_int) as
                   libc::c_uint &&
               j <
                   (32 as libc::c_int + 1 as libc::c_int - 1 as libc::c_int)
                       as libc::c_uint {
            // FIXME
            if outlineColour3D == 255 as libc::c_int as libc::c_uint {
                oldColoursWord[0 as libc::c_int as usize] =
                    tileScreenInfo[i.wrapping_add(0 as libc::c_int as
                                                      libc::c_uint) as
                                       usize][j.wrapping_add(0 as libc::c_int
                                                                 as
                                                                 libc::c_uint)
                                                  as usize].light.argb;
                oldColoursWord[1 as libc::c_int as usize] =
                    tileScreenInfo[i.wrapping_add(0 as libc::c_int as
                                                      libc::c_uint) as
                                       usize][j.wrapping_add(1 as libc::c_int
                                                                 as
                                                                 libc::c_uint)
                                                  as usize].light.argb;
                oldColoursWord[2 as libc::c_int as usize] =
                    tileScreenInfo[i.wrapping_add(1 as libc::c_int as
                                                      libc::c_uint) as
                                       usize][j.wrapping_add(1 as libc::c_int
                                                                 as
                                                                 libc::c_uint)
                                                  as usize].light.argb;
                oldColoursWord[3 as libc::c_int as usize] =
                    tileScreenInfo[i.wrapping_add(1 as libc::c_int as
                                                      libc::c_uint) as
                                       usize][j.wrapping_add(0 as libc::c_int
                                                                 as
                                                                 libc::c_uint)
                                                  as usize].light.argb;
                /*
				tileScreenInfo[i+0][j+0].light.argb = 0x00f0f0f0;
				tileScreenInfo[i+0][j+1].light.argb = 0x00f0f0f0;
				tileScreenInfo[i+1][j+1].light.argb = 0x00f0f0f0;
				tileScreenInfo[i+1][j+0].light.argb = 0x00f0f0f0;
				*/
                tileScreenInfo[i.wrapping_add(0 as libc::c_int as
                                                  libc::c_uint) as
                                   usize][j.wrapping_add(0 as libc::c_int as
                                                             libc::c_uint) as
                                              usize].light.byte.b =
                    255 as libc::c_int as UBYTE;
                tileScreenInfo[i.wrapping_add(0 as libc::c_int as
                                                  libc::c_uint) as
                                   usize][j.wrapping_add(1 as libc::c_int as
                                                             libc::c_uint) as
                                              usize].light.byte.b =
                    255 as libc::c_int as UBYTE;
                tileScreenInfo[i.wrapping_add(1 as libc::c_int as
                                                  libc::c_uint) as
                                   usize][j.wrapping_add(1 as libc::c_int as
                                                             libc::c_uint) as
                                              usize].light.byte.b =
                    255 as libc::c_int as UBYTE;
                tileScreenInfo[i.wrapping_add(1 as libc::c_int as
                                                  libc::c_uint) as
                                   usize][j.wrapping_add(0 as libc::c_int as
                                                             libc::c_uint) as
                                              usize].light.byte.b =
                    255 as libc::c_int as UBYTE;
                tileScreenInfo[i.wrapping_add(0 as libc::c_int as
                                                  libc::c_uint) as
                                   usize][j.wrapping_add(0 as libc::c_int as
                                                             libc::c_uint) as
                                              usize].light.byte.g =
                    255 as libc::c_int as UBYTE;
                tileScreenInfo[i.wrapping_add(0 as libc::c_int as
                                                  libc::c_uint) as
                                   usize][j.wrapping_add(1 as libc::c_int as
                                                             libc::c_uint) as
                                              usize].light.byte.g =
                    255 as libc::c_int as UBYTE;
                tileScreenInfo[i.wrapping_add(1 as libc::c_int as
                                                  libc::c_uint) as
                                   usize][j.wrapping_add(1 as libc::c_int as
                                                             libc::c_uint) as
                                              usize].light.byte.g =
                    255 as libc::c_int as UBYTE;
                tileScreenInfo[i.wrapping_add(1 as libc::c_int as
                                                  libc::c_uint) as
                                   usize][j.wrapping_add(0 as libc::c_int as
                                                             libc::c_uint) as
                                              usize].light.byte.g =
                    255 as libc::c_int as UBYTE;
                tileScreenInfo[i.wrapping_add(0 as libc::c_int as
                                                  libc::c_uint) as
                                   usize][j.wrapping_add(0 as libc::c_int as
                                                             libc::c_uint) as
                                              usize].light.byte.r =
                    255 as libc::c_int as UBYTE;
                tileScreenInfo[i.wrapping_add(0 as libc::c_int as
                                                  libc::c_uint) as
                                   usize][j.wrapping_add(1 as libc::c_int as
                                                             libc::c_uint) as
                                              usize].light.byte.r =
                    255 as libc::c_int as UBYTE;
                tileScreenInfo[i.wrapping_add(1 as libc::c_int as
                                                  libc::c_uint) as
                                   usize][j.wrapping_add(1 as libc::c_int as
                                                             libc::c_uint) as
                                              usize].light.byte.r =
                    255 as libc::c_int as UBYTE;
                tileScreenInfo[i.wrapping_add(1 as libc::c_int as
                                                  libc::c_uint) as
                                   usize][j.wrapping_add(0 as libc::c_int as
                                                             libc::c_uint) as
                                              usize].light.byte.r =
                    255 as libc::c_int as UBYTE
            } else {
                oldColours[0 as libc::c_int as usize] =
                    tileScreenInfo[i.wrapping_add(0 as libc::c_int as
                                                      libc::c_uint) as
                                       usize][j.wrapping_add(0 as libc::c_int
                                                                 as
                                                                 libc::c_uint)
                                                  as usize].light.byte.r;
                oldColours[1 as libc::c_int as usize] =
                    tileScreenInfo[i.wrapping_add(0 as libc::c_int as
                                                      libc::c_uint) as
                                       usize][j.wrapping_add(1 as libc::c_int
                                                                 as
                                                                 libc::c_uint)
                                                  as usize].light.byte.r;
                oldColours[2 as libc::c_int as usize] =
                    tileScreenInfo[i.wrapping_add(1 as libc::c_int as
                                                      libc::c_uint) as
                                       usize][j.wrapping_add(1 as libc::c_int
                                                                 as
                                                                 libc::c_uint)
                                                  as usize].light.byte.r;
                oldColours[3 as libc::c_int as usize] =
                    tileScreenInfo[i.wrapping_add(1 as libc::c_int as
                                                      libc::c_uint) as
                                       usize][j.wrapping_add(0 as libc::c_int
                                                                 as
                                                                 libc::c_uint)
                                                  as usize].light.byte.r;
                tileScreenInfo[i.wrapping_add(0 as libc::c_int as
                                                  libc::c_uint) as
                                   usize][j.wrapping_add(0 as libc::c_int as
                                                             libc::c_uint) as
                                              usize].light.byte.r =
                    255 as libc::c_int as UBYTE;
                tileScreenInfo[i.wrapping_add(0 as libc::c_int as
                                                  libc::c_uint) as
                                   usize][j.wrapping_add(1 as libc::c_int as
                                                             libc::c_uint) as
                                              usize].light.byte.r =
                    255 as libc::c_int as UBYTE;
                tileScreenInfo[i.wrapping_add(1 as libc::c_int as
                                                  libc::c_uint) as
                                   usize][j.wrapping_add(1 as libc::c_int as
                                                             libc::c_uint) as
                                              usize].light.byte.r =
                    255 as libc::c_int as UBYTE;
                tileScreenInfo[i.wrapping_add(1 as libc::c_int as
                                                  libc::c_uint) as
                                   usize][j.wrapping_add(0 as libc::c_int as
                                                             libc::c_uint) as
                                              usize].light.byte.r =
                    255 as libc::c_int as UBYTE
            }
        }
    }
    /*
	else if(TILE_IN_SENSORRANGE(psTile))
	{
		if(tileScreenInfo[i+0][j+0].light.byte.g < 128)
		{
			tileScreenInfo[i+0][j+0].light.byte.g *= 2;
		}
		else
		{
			tileScreenInfo[i+0][j+0].light.byte.g =255;
		}

		if(tileScreenInfo[i+0][j+1].light.byte.g < 128)
		{
			tileScreenInfo[i+0][j+1].light.byte.g *= 2;
		}
		else
		{
			tileScreenInfo[i+0][j+1].light.byte.g =255;
		}

		if(tileScreenInfo[i+1][j+1].light.byte.g < 128)
		{
			tileScreenInfo[i+1][j+1].light.byte.g *= 2;
		}
		else
		{
			tileScreenInfo[i+1][j+1].light.byte.g =255;
		}
		if(tileScreenInfo[i+1][j+0].light.byte.g < 128)
		{
			tileScreenInfo[i+1][j+0].light.byte.g *= 2;
		}
		else
		{
			tileScreenInfo[i+1][j+0].light.byte.g =255;
		}

	}
	*/
	/* Get the right texture page; it is pre stored and indexed on
	 * the graphics card */
    pie_SetTexturePage(tileTexInfo[(tileNumber &
                                        0x1ff as libc::c_int as libc::c_uint)
                                       as usize].texPage as SDWORD);
    /* set up the texture size info */
    renderFlag = 0 as libc::c_int as UDWORD;
    offset.x =
        tileTexInfo[(tileNumber & 0x1ff as libc::c_int as libc::c_uint) as
                        usize].xOffset.wrapping_mul(64 as libc::c_int as
                                                        libc::c_uint) as
            int32;
    offset.y =
        tileTexInfo[(tileNumber & 0x1ff as libc::c_int as libc::c_uint) as
                        usize].yOffset.wrapping_mul(64 as libc::c_int as
                                                        libc::c_uint) as
            int32;
    /* Check for rotations and flips - this sets up the coordinates for texturing */
    flipsAndRots((tileNumber & !(0x1ff as libc::c_int) as libc::c_uint) as
                     libc::c_int);
    tileScreenInfo[i.wrapping_add(0 as libc::c_int as libc::c_uint) as
                       usize][j.wrapping_add(0 as libc::c_int as libc::c_uint)
                                  as usize].tu =
        ((*psP1).x + offset.x) as UWORD;
    tileScreenInfo[i.wrapping_add(0 as libc::c_int as libc::c_uint) as
                       usize][j.wrapping_add(0 as libc::c_int as libc::c_uint)
                                  as usize].tv =
        ((*psP1).y + offset.y) as UWORD;
    tileScreenInfo[i.wrapping_add(0 as libc::c_int as libc::c_uint) as
                       usize][j.wrapping_add(1 as libc::c_int as libc::c_uint)
                                  as usize].tu =
        ((*psP2).x + offset.x) as UWORD;
    tileScreenInfo[i.wrapping_add(0 as libc::c_int as libc::c_uint) as
                       usize][j.wrapping_add(1 as libc::c_int as libc::c_uint)
                                  as usize].tv =
        ((*psP2).y + offset.y) as UWORD;
    tileScreenInfo[i.wrapping_add(1 as libc::c_int as libc::c_uint) as
                       usize][j.wrapping_add(1 as libc::c_int as libc::c_uint)
                                  as usize].tu =
        ((*psP3).x + offset.x) as UWORD;
    tileScreenInfo[i.wrapping_add(1 as libc::c_int as libc::c_uint) as
                       usize][j.wrapping_add(1 as libc::c_int as libc::c_uint)
                                  as usize].tv =
        ((*psP3).y + offset.y) as UWORD;
    tileScreenInfo[i.wrapping_add(1 as libc::c_int as libc::c_uint) as
                       usize][j.wrapping_add(0 as libc::c_int as libc::c_uint)
                                  as usize].tu =
        ((*psP4).x + offset.x) as UWORD;
    tileScreenInfo[i.wrapping_add(1 as libc::c_int as libc::c_uint) as
                       usize][j.wrapping_add(0 as libc::c_int as libc::c_uint)
                                  as usize].tv =
        ((*psP4).y + offset.y) as UWORD;
    /* The first triangle */
    if (*psTile).texture as libc::c_int & 0x800 as libc::c_int != 0 {
        memcpy(&mut *aVrts.as_mut_ptr().offset(0 as libc::c_int as isize) as
                   *mut PIEVERTEX as *mut libc::c_void,
               &mut *(*tileScreenInfo.as_mut_ptr().offset(i.wrapping_add(0 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint)
                                                              as
                                                              isize)).as_mut_ptr().offset(j.wrapping_add(0
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             libc::c_uint)
                                                                                              as
                                                                                              isize)
                   as *mut SVMESH as *const libc::c_void,
               ::std::mem::size_of::<PIEVERTEX>() as libc::c_ulong);
        memcpy(&mut *aVrts.as_mut_ptr().offset(1 as libc::c_int as isize) as
                   *mut PIEVERTEX as *mut libc::c_void,
               &mut *(*tileScreenInfo.as_mut_ptr().offset(i.wrapping_add(0 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint)
                                                              as
                                                              isize)).as_mut_ptr().offset(j.wrapping_add(1
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             libc::c_uint)
                                                                                              as
                                                                                              isize)
                   as *mut SVMESH as *const libc::c_void,
               ::std::mem::size_of::<PIEVERTEX>() as libc::c_ulong);
        memcpy(&mut *aVrts.as_mut_ptr().offset(2 as libc::c_int as isize) as
                   *mut PIEVERTEX as *mut libc::c_void,
               &mut *(*tileScreenInfo.as_mut_ptr().offset(i.wrapping_add(1 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint)
                                                              as
                                                              isize)).as_mut_ptr().offset(j.wrapping_add(0
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             libc::c_uint)
                                                                                              as
                                                                                              isize)
                   as *mut SVMESH as *const libc::c_void,
               ::std::mem::size_of::<PIEVERTEX>() as libc::c_ulong);
        pie_DrawPoly(3 as libc::c_int, aVrts.as_mut_ptr(),
                     tileTexInfo[(tileNumber &
                                      0x1ff as libc::c_int as libc::c_uint) as
                                     usize].texPage as SDWORD,
                     0 as *mut libc::c_void);
    } else {
        memcpy(&mut *aVrts.as_mut_ptr().offset(0 as libc::c_int as isize) as
                   *mut PIEVERTEX as *mut libc::c_void,
               &mut *(*tileScreenInfo.as_mut_ptr().offset(i.wrapping_add(0 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint)
                                                              as
                                                              isize)).as_mut_ptr().offset(j.wrapping_add(0
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             libc::c_uint)
                                                                                              as
                                                                                              isize)
                   as *mut SVMESH as *const libc::c_void,
               ::std::mem::size_of::<PIEVERTEX>() as libc::c_ulong);
        memcpy(&mut *aVrts.as_mut_ptr().offset(1 as libc::c_int as isize) as
                   *mut PIEVERTEX as *mut libc::c_void,
               &mut *(*tileScreenInfo.as_mut_ptr().offset(i.wrapping_add(0 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint)
                                                              as
                                                              isize)).as_mut_ptr().offset(j.wrapping_add(1
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             libc::c_uint)
                                                                                              as
                                                                                              isize)
                   as *mut SVMESH as *const libc::c_void,
               ::std::mem::size_of::<PIEVERTEX>() as libc::c_ulong);
        memcpy(&mut *aVrts.as_mut_ptr().offset(2 as libc::c_int as isize) as
                   *mut PIEVERTEX as *mut libc::c_void,
               &mut *(*tileScreenInfo.as_mut_ptr().offset(i.wrapping_add(1 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint)
                                                              as
                                                              isize)).as_mut_ptr().offset(j.wrapping_add(1
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             libc::c_uint)
                                                                                              as
                                                                                              isize)
                   as *mut SVMESH as *const libc::c_void,
               ::std::mem::size_of::<PIEVERTEX>() as libc::c_ulong);
        pie_DrawPoly(3 as libc::c_int, aVrts.as_mut_ptr(),
                     tileTexInfo[(tileNumber &
                                      0x1ff as libc::c_int as libc::c_uint) as
                                     usize].texPage as SDWORD,
                     0 as *mut libc::c_void);
    }
    /* The second triangle */
    if (*psTile).texture as libc::c_int & 0x800 as libc::c_int != 0 {
        memcpy(&mut *aVrts.as_mut_ptr().offset(0 as libc::c_int as isize) as
                   *mut PIEVERTEX as *mut libc::c_void,
               &mut *(*tileScreenInfo.as_mut_ptr().offset(i.wrapping_add(0 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint)
                                                              as
                                                              isize)).as_mut_ptr().offset(j.wrapping_add(1
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             libc::c_uint)
                                                                                              as
                                                                                              isize)
                   as *mut SVMESH as *const libc::c_void,
               ::std::mem::size_of::<PIEVERTEX>() as libc::c_ulong);
        memcpy(&mut *aVrts.as_mut_ptr().offset(1 as libc::c_int as isize) as
                   *mut PIEVERTEX as *mut libc::c_void,
               &mut *(*tileScreenInfo.as_mut_ptr().offset(i.wrapping_add(1 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint)
                                                              as
                                                              isize)).as_mut_ptr().offset(j.wrapping_add(1
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             libc::c_uint)
                                                                                              as
                                                                                              isize)
                   as *mut SVMESH as *const libc::c_void,
               ::std::mem::size_of::<PIEVERTEX>() as libc::c_ulong);
        memcpy(&mut *aVrts.as_mut_ptr().offset(2 as libc::c_int as isize) as
                   *mut PIEVERTEX as *mut libc::c_void,
               &mut *(*tileScreenInfo.as_mut_ptr().offset(i.wrapping_add(1 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint)
                                                              as
                                                              isize)).as_mut_ptr().offset(j.wrapping_add(0
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             libc::c_uint)
                                                                                              as
                                                                                              isize)
                   as *mut SVMESH as *const libc::c_void,
               ::std::mem::size_of::<PIEVERTEX>() as libc::c_ulong);
        pie_DrawPoly(3 as libc::c_int, aVrts.as_mut_ptr(),
                     tileTexInfo[(tileNumber &
                                      0x1ff as libc::c_int as libc::c_uint) as
                                     usize].texPage as SDWORD,
                     0 as *mut libc::c_void);
    } else {
        memcpy(&mut *aVrts.as_mut_ptr().offset(0 as libc::c_int as isize) as
                   *mut PIEVERTEX as *mut libc::c_void,
               &mut *(*tileScreenInfo.as_mut_ptr().offset(i.wrapping_add(0 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint)
                                                              as
                                                              isize)).as_mut_ptr().offset(j.wrapping_add(0
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             libc::c_uint)
                                                                                              as
                                                                                              isize)
                   as *mut SVMESH as *const libc::c_void,
               ::std::mem::size_of::<PIEVERTEX>() as libc::c_ulong);
        memcpy(&mut *aVrts.as_mut_ptr().offset(1 as libc::c_int as isize) as
                   *mut PIEVERTEX as *mut libc::c_void,
               &mut *(*tileScreenInfo.as_mut_ptr().offset(i.wrapping_add(1 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint)
                                                              as
                                                              isize)).as_mut_ptr().offset(j.wrapping_add(1
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             libc::c_uint)
                                                                                              as
                                                                                              isize)
                   as *mut SVMESH as *const libc::c_void,
               ::std::mem::size_of::<PIEVERTEX>() as libc::c_ulong);
        memcpy(&mut *aVrts.as_mut_ptr().offset(2 as libc::c_int as isize) as
                   *mut PIEVERTEX as *mut libc::c_void,
               &mut *(*tileScreenInfo.as_mut_ptr().offset(i.wrapping_add(1 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint)
                                                              as
                                                              isize)).as_mut_ptr().offset(j.wrapping_add(0
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             libc::c_uint)
                                                                                              as
                                                                                              isize)
                   as *mut SVMESH as *const libc::c_void,
               ::std::mem::size_of::<PIEVERTEX>() as libc::c_ulong);
        pie_DrawPoly(3 as libc::c_int, aVrts.as_mut_ptr(),
                     tileTexInfo[(tileNumber &
                                      0x1ff as libc::c_int as libc::c_uint) as
                                     usize].texPage as SDWORD,
                     0 as *mut libc::c_void);
    }
    /* Outline the tile if necessary */
    if terrainOutline != 0 {
        pie_Line(tileScreenInfo[i.wrapping_add(0 as libc::c_int as
                                                   libc::c_uint) as
                                    usize][j.wrapping_add(0 as libc::c_int as
                                                              libc::c_uint) as
                                               usize].sx,
                 tileScreenInfo[i.wrapping_add(0 as libc::c_int as
                                                   libc::c_uint) as
                                    usize][j.wrapping_add(0 as libc::c_int as
                                                              libc::c_uint) as
                                               usize].sy,
                 tileScreenInfo[i.wrapping_add(0 as libc::c_int as
                                                   libc::c_uint) as
                                    usize][j.wrapping_add(1 as libc::c_int as
                                                              libc::c_uint) as
                                               usize].sx,
                 tileScreenInfo[i.wrapping_add(0 as libc::c_int as
                                                   libc::c_uint) as
                                    usize][j.wrapping_add(1 as libc::c_int as
                                                              libc::c_uint) as
                                               usize].sy,
                 255 as libc::c_int as uint32);
        pie_Line(tileScreenInfo[i.wrapping_add(0 as libc::c_int as
                                                   libc::c_uint) as
                                    usize][j.wrapping_add(1 as libc::c_int as
                                                              libc::c_uint) as
                                               usize].sx,
                 tileScreenInfo[i.wrapping_add(0 as libc::c_int as
                                                   libc::c_uint) as
                                    usize][j.wrapping_add(1 as libc::c_int as
                                                              libc::c_uint) as
                                               usize].sy,
                 tileScreenInfo[i.wrapping_add(1 as libc::c_int as
                                                   libc::c_uint) as
                                    usize][j.wrapping_add(1 as libc::c_int as
                                                              libc::c_uint) as
                                               usize].sx,
                 tileScreenInfo[i.wrapping_add(1 as libc::c_int as
                                                   libc::c_uint) as
                                    usize][j.wrapping_add(1 as libc::c_int as
                                                              libc::c_uint) as
                                               usize].sy,
                 255 as libc::c_int as uint32);
        pie_Line(tileScreenInfo[i.wrapping_add(1 as libc::c_int as
                                                   libc::c_uint) as
                                    usize][j.wrapping_add(1 as libc::c_int as
                                                              libc::c_uint) as
                                               usize].sx,
                 tileScreenInfo[i.wrapping_add(1 as libc::c_int as
                                                   libc::c_uint) as
                                    usize][j.wrapping_add(1 as libc::c_int as
                                                              libc::c_uint) as
                                               usize].sy,
                 tileScreenInfo[i.wrapping_add(1 as libc::c_int as
                                                   libc::c_uint) as
                                    usize][j.wrapping_add(0 as libc::c_int as
                                                              libc::c_uint) as
                                               usize].sx,
                 tileScreenInfo[i.wrapping_add(1 as libc::c_int as
                                                   libc::c_uint) as
                                    usize][j.wrapping_add(0 as libc::c_int as
                                                              libc::c_uint) as
                                               usize].sy,
                 255 as libc::c_int as uint32);
        pie_Line(tileScreenInfo[i.wrapping_add(1 as libc::c_int as
                                                   libc::c_uint) as
                                    usize][j.wrapping_add(0 as libc::c_int as
                                                              libc::c_uint) as
                                               usize].sx,
                 tileScreenInfo[i.wrapping_add(1 as libc::c_int as
                                                   libc::c_uint) as
                                    usize][j.wrapping_add(0 as libc::c_int as
                                                              libc::c_uint) as
                                               usize].sy,
                 tileScreenInfo[i.wrapping_add(0 as libc::c_int as
                                                   libc::c_uint) as
                                    usize][j.wrapping_add(0 as libc::c_int as
                                                              libc::c_uint) as
                                               usize].sx,
                 tileScreenInfo[i.wrapping_add(0 as libc::c_int as
                                                   libc::c_uint) as
                                    usize][j.wrapping_add(0 as libc::c_int as
                                                              libc::c_uint) as
                                               usize].sy,
                 255 as libc::c_int as uint32);
    }
    if bOutlined != 0 {
        if outlineColour3D == 255 as libc::c_int as libc::c_uint {
            tileScreenInfo[i.wrapping_add(0 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(0 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].light.argb =
                oldColoursWord[0 as libc::c_int as usize];
            tileScreenInfo[i.wrapping_add(0 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(1 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].light.argb =
                oldColoursWord[1 as libc::c_int as usize];
            tileScreenInfo[i.wrapping_add(1 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(1 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].light.argb =
                oldColoursWord[2 as libc::c_int as usize];
            tileScreenInfo[i.wrapping_add(1 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(0 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].light.argb =
                oldColoursWord[3 as libc::c_int as usize]
        } else {
            tileScreenInfo[i.wrapping_add(0 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(0 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].light.byte.r =
                oldColours[0 as libc::c_int as usize];
            tileScreenInfo[i.wrapping_add(0 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(1 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].light.byte.r =
                oldColours[1 as libc::c_int as usize];
            tileScreenInfo[i.wrapping_add(1 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(1 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].light.byte.r =
                oldColours[2 as libc::c_int as usize];
            tileScreenInfo[i.wrapping_add(1 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(0 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].light.byte.r =
                oldColours[3 as libc::c_int as usize]
        }
    };
}
// Render a water edge tile
//
#[no_mangle]
pub unsafe extern "C" fn drawTerrainWEdgeTile(mut i: UDWORD, mut j: UDWORD) {
    let mut actualX: UDWORD = 0;
    let mut actualY: UDWORD = 0;
    let mut psTile: *mut MAPTILE = 0 as *mut MAPTILE;
    //BOOL	bOutlined;
    let mut tileNumber: UDWORD = 0;
    let mut renderFlag: UDWORD = 0;
    let mut offset: iPoint = iPoint{x: 0, y: 0,};
    let mut aVrts: [PIEVERTEX; 3] =
        [PIEVERTEX{sx: 0,
                   sy: 0,
                   sz: 0,
                   tu: 0,
                   tv: 0,
                   light:
                       PIELIGHT{byte:
                                    PIELIGHTBYTES{b: 0, g: 0, r: 0, a: 0,},},
                   specular:
                       PIELIGHT{byte:
                                    PIELIGHTBYTES{b: 0,
                                                  g: 0,
                                                  r: 0,
                                                  a: 0,},},}; 3];
    /* Get the correct tile index for the x coordinate */
    actualX = (playerXTile as libc::c_uint).wrapping_add(j);
    /* Get the correct tile index for the y coordinate */
    actualY = (playerZTile as libc::c_uint).wrapping_add(i);
    /* Let's just get out now if we're not supposed to draw it */
    if actualX < 0 as libc::c_int as libc::c_uint ||
           actualY < 0 as libc::c_int as libc::c_uint ||
           actualX > mapWidth.wrapping_sub(1 as libc::c_int as libc::c_uint)
           ||
           actualY > mapHeight.wrapping_sub(1 as libc::c_int as libc::c_uint)
       {
        psTile = &mut edgeTile;
        (*psTile).texture =
            ((*psTile).texture as libc::c_int & !(0x400 as libc::c_int)) as
                UWORD
    } else { psTile = mapTile(actualX, actualY) }
    /* what tile texture number is it? */
    tileNumber = (*psTile).texture as UDWORD;
    /* 3dfx is pre stored and indexed */
    pie_SetTexturePage(tileTexInfo[(tileNumber &
                                        0x1ff as libc::c_int as libc::c_uint)
                                       as usize].texPage as SDWORD);
    /* set up the texture size info */
    renderFlag = 0 as libc::c_int as UDWORD;
    offset.x =
        tileTexInfo[(tileNumber & 0x1ff as libc::c_int as libc::c_uint) as
                        usize].xOffset.wrapping_mul(64 as libc::c_int as
                                                        libc::c_uint) as
            int32;
    offset.y =
        tileTexInfo[(tileNumber & 0x1ff as libc::c_int as libc::c_uint) as
                        usize].yOffset.wrapping_mul(64 as libc::c_int as
                                                        libc::c_uint) as
            int32;
    /* Check for rotations and flips - this sets up the coordinates for texturing */
    flipsAndRots((tileNumber & !(0x1ff as libc::c_int) as libc::c_uint) as
                     libc::c_int);
    tileScreenInfo[i.wrapping_add(0 as libc::c_int as libc::c_uint) as
                       usize][j.wrapping_add(0 as libc::c_int as libc::c_uint)
                                  as usize].tu =
        ((*psP1).x + offset.x) as UWORD;
    tileScreenInfo[i.wrapping_add(0 as libc::c_int as libc::c_uint) as
                       usize][j.wrapping_add(0 as libc::c_int as libc::c_uint)
                                  as usize].tv =
        ((*psP1).y + offset.y) as UWORD;
    tileScreenInfo[i.wrapping_add(0 as libc::c_int as libc::c_uint) as
                       usize][j.wrapping_add(1 as libc::c_int as libc::c_uint)
                                  as usize].tu =
        ((*psP2).x + offset.x) as UWORD;
    tileScreenInfo[i.wrapping_add(0 as libc::c_int as libc::c_uint) as
                       usize][j.wrapping_add(1 as libc::c_int as libc::c_uint)
                                  as usize].tv =
        ((*psP2).y + offset.y) as UWORD;
    tileScreenInfo[i.wrapping_add(1 as libc::c_int as libc::c_uint) as
                       usize][j.wrapping_add(1 as libc::c_int as libc::c_uint)
                                  as usize].tu =
        ((*psP3).x + offset.x) as UWORD;
    tileScreenInfo[i.wrapping_add(1 as libc::c_int as libc::c_uint) as
                       usize][j.wrapping_add(1 as libc::c_int as libc::c_uint)
                                  as usize].tv =
        ((*psP3).y + offset.y) as UWORD;
    tileScreenInfo[i.wrapping_add(1 as libc::c_int as libc::c_uint) as
                       usize][j.wrapping_add(0 as libc::c_int as libc::c_uint)
                                  as usize].tu =
        ((*psP4).x + offset.x) as UWORD;
    tileScreenInfo[i.wrapping_add(1 as libc::c_int as libc::c_uint) as
                       usize][j.wrapping_add(0 as libc::c_int as libc::c_uint)
                                  as usize].tv =
        ((*psP4).y + offset.y) as UWORD;
    /* The first triangle */
    if (*psTile).texture as libc::c_int & 0x800 as libc::c_int != 0 {
        memcpy(&mut *aVrts.as_mut_ptr().offset(0 as libc::c_int as isize) as
                   *mut PIEVERTEX as *mut libc::c_void,
               &mut *(*tileScreenInfo.as_mut_ptr().offset(i.wrapping_add(0 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint)
                                                              as
                                                              isize)).as_mut_ptr().offset(j.wrapping_add(0
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             libc::c_uint)
                                                                                              as
                                                                                              isize)
                   as *mut SVMESH as *const libc::c_void,
               ::std::mem::size_of::<PIEVERTEX>() as libc::c_ulong);
        aVrts[0 as libc::c_int as usize].sx =
            tileScreenInfo[i.wrapping_add(0 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(0 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].x;
        aVrts[0 as libc::c_int as usize].sy =
            tileScreenInfo[i.wrapping_add(0 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(0 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].water_height;
        aVrts[0 as libc::c_int as usize].sz =
            tileScreenInfo[i.wrapping_add(0 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(0 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].z;
        memcpy(&mut *aVrts.as_mut_ptr().offset(1 as libc::c_int as isize) as
                   *mut PIEVERTEX as *mut libc::c_void,
               &mut *(*tileScreenInfo.as_mut_ptr().offset(i.wrapping_add(0 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint)
                                                              as
                                                              isize)).as_mut_ptr().offset(j.wrapping_add(1
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             libc::c_uint)
                                                                                              as
                                                                                              isize)
                   as *mut SVMESH as *const libc::c_void,
               ::std::mem::size_of::<PIEVERTEX>() as libc::c_ulong);
        aVrts[1 as libc::c_int as usize].sx =
            tileScreenInfo[i.wrapping_add(0 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(1 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].x;
        aVrts[1 as libc::c_int as usize].sy =
            tileScreenInfo[i.wrapping_add(0 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(1 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].water_height;
        aVrts[1 as libc::c_int as usize].sz =
            tileScreenInfo[i.wrapping_add(0 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(1 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].z;
        memcpy(&mut *aVrts.as_mut_ptr().offset(2 as libc::c_int as isize) as
                   *mut PIEVERTEX as *mut libc::c_void,
               &mut *(*tileScreenInfo.as_mut_ptr().offset(i.wrapping_add(1 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint)
                                                              as
                                                              isize)).as_mut_ptr().offset(j.wrapping_add(0
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             libc::c_uint)
                                                                                              as
                                                                                              isize)
                   as *mut SVMESH as *const libc::c_void,
               ::std::mem::size_of::<PIEVERTEX>() as libc::c_ulong);
        aVrts[2 as libc::c_int as usize].sx =
            tileScreenInfo[i.wrapping_add(1 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(0 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].x;
        aVrts[2 as libc::c_int as usize].sy =
            tileScreenInfo[i.wrapping_add(1 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(0 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].water_height;
        aVrts[2 as libc::c_int as usize].sz =
            tileScreenInfo[i.wrapping_add(1 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(0 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].z;
        pie_DrawPoly(3 as libc::c_int, aVrts.as_mut_ptr(),
                     tileTexInfo[(tileNumber &
                                      0x1ff as libc::c_int as libc::c_uint) as
                                     usize].texPage as SDWORD,
                     0 as *mut libc::c_void);
    } else {
        memcpy(&mut *aVrts.as_mut_ptr().offset(0 as libc::c_int as isize) as
                   *mut PIEVERTEX as *mut libc::c_void,
               &mut *(*tileScreenInfo.as_mut_ptr().offset(i.wrapping_add(0 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint)
                                                              as
                                                              isize)).as_mut_ptr().offset(j.wrapping_add(0
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             libc::c_uint)
                                                                                              as
                                                                                              isize)
                   as *mut SVMESH as *const libc::c_void,
               ::std::mem::size_of::<PIEVERTEX>() as libc::c_ulong);
        aVrts[0 as libc::c_int as usize].sx =
            tileScreenInfo[i.wrapping_add(0 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(0 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].x;
        aVrts[0 as libc::c_int as usize].sy =
            tileScreenInfo[i.wrapping_add(0 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(0 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].water_height;
        aVrts[0 as libc::c_int as usize].sz =
            tileScreenInfo[i.wrapping_add(0 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(0 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].z;
        memcpy(&mut *aVrts.as_mut_ptr().offset(1 as libc::c_int as isize) as
                   *mut PIEVERTEX as *mut libc::c_void,
               &mut *(*tileScreenInfo.as_mut_ptr().offset(i.wrapping_add(0 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint)
                                                              as
                                                              isize)).as_mut_ptr().offset(j.wrapping_add(1
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             libc::c_uint)
                                                                                              as
                                                                                              isize)
                   as *mut SVMESH as *const libc::c_void,
               ::std::mem::size_of::<PIEVERTEX>() as libc::c_ulong);
        aVrts[1 as libc::c_int as usize].sx =
            tileScreenInfo[i.wrapping_add(0 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(1 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].x;
        aVrts[1 as libc::c_int as usize].sy =
            tileScreenInfo[i.wrapping_add(0 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(1 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].water_height;
        aVrts[1 as libc::c_int as usize].sz =
            tileScreenInfo[i.wrapping_add(0 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(1 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].z;
        memcpy(&mut *aVrts.as_mut_ptr().offset(2 as libc::c_int as isize) as
                   *mut PIEVERTEX as *mut libc::c_void,
               &mut *(*tileScreenInfo.as_mut_ptr().offset(i.wrapping_add(1 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint)
                                                              as
                                                              isize)).as_mut_ptr().offset(j.wrapping_add(1
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             libc::c_uint)
                                                                                              as
                                                                                              isize)
                   as *mut SVMESH as *const libc::c_void,
               ::std::mem::size_of::<PIEVERTEX>() as libc::c_ulong);
        aVrts[2 as libc::c_int as usize].sx =
            tileScreenInfo[i.wrapping_add(1 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(1 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].x;
        aVrts[2 as libc::c_int as usize].sy =
            tileScreenInfo[i.wrapping_add(1 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(1 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].water_height;
        aVrts[2 as libc::c_int as usize].sz =
            tileScreenInfo[i.wrapping_add(1 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(1 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].z;
        pie_DrawPoly(3 as libc::c_int, aVrts.as_mut_ptr(),
                     tileTexInfo[(tileNumber &
                                      0x1ff as libc::c_int as libc::c_uint) as
                                     usize].texPage as SDWORD,
                     0 as *mut libc::c_void);
    }
    /* The second triangle */
    if (*psTile).texture as libc::c_int & 0x800 as libc::c_int != 0 {
        memcpy(&mut *aVrts.as_mut_ptr().offset(0 as libc::c_int as isize) as
                   *mut PIEVERTEX as *mut libc::c_void,
               &mut *(*tileScreenInfo.as_mut_ptr().offset(i.wrapping_add(0 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint)
                                                              as
                                                              isize)).as_mut_ptr().offset(j.wrapping_add(1
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             libc::c_uint)
                                                                                              as
                                                                                              isize)
                   as *mut SVMESH as *const libc::c_void,
               ::std::mem::size_of::<PIEVERTEX>() as libc::c_ulong);
        aVrts[0 as libc::c_int as usize].sx =
            tileScreenInfo[i.wrapping_add(0 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(1 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].x;
        aVrts[0 as libc::c_int as usize].sy =
            tileScreenInfo[i.wrapping_add(0 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(1 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].water_height;
        aVrts[0 as libc::c_int as usize].sz =
            tileScreenInfo[i.wrapping_add(0 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(1 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].z;
        memcpy(&mut *aVrts.as_mut_ptr().offset(1 as libc::c_int as isize) as
                   *mut PIEVERTEX as *mut libc::c_void,
               &mut *(*tileScreenInfo.as_mut_ptr().offset(i.wrapping_add(1 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint)
                                                              as
                                                              isize)).as_mut_ptr().offset(j.wrapping_add(1
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             libc::c_uint)
                                                                                              as
                                                                                              isize)
                   as *mut SVMESH as *const libc::c_void,
               ::std::mem::size_of::<PIEVERTEX>() as libc::c_ulong);
        aVrts[1 as libc::c_int as usize].sx =
            tileScreenInfo[i.wrapping_add(1 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(1 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].x;
        aVrts[1 as libc::c_int as usize].sy =
            tileScreenInfo[i.wrapping_add(1 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(1 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].water_height;
        aVrts[1 as libc::c_int as usize].sz =
            tileScreenInfo[i.wrapping_add(1 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(1 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].z;
        memcpy(&mut *aVrts.as_mut_ptr().offset(2 as libc::c_int as isize) as
                   *mut PIEVERTEX as *mut libc::c_void,
               &mut *(*tileScreenInfo.as_mut_ptr().offset(i.wrapping_add(1 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint)
                                                              as
                                                              isize)).as_mut_ptr().offset(j.wrapping_add(0
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             libc::c_uint)
                                                                                              as
                                                                                              isize)
                   as *mut SVMESH as *const libc::c_void,
               ::std::mem::size_of::<PIEVERTEX>() as libc::c_ulong);
        aVrts[2 as libc::c_int as usize].sx =
            tileScreenInfo[i.wrapping_add(1 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(0 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].x;
        aVrts[2 as libc::c_int as usize].sy =
            tileScreenInfo[i.wrapping_add(1 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(0 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].water_height;
        aVrts[2 as libc::c_int as usize].sz =
            tileScreenInfo[i.wrapping_add(1 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(0 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].z;
        pie_DrawPoly(3 as libc::c_int, aVrts.as_mut_ptr(),
                     tileTexInfo[(tileNumber &
                                      0x1ff as libc::c_int as libc::c_uint) as
                                     usize].texPage as SDWORD,
                     0 as *mut libc::c_void);
    } else {
        memcpy(&mut *aVrts.as_mut_ptr().offset(0 as libc::c_int as isize) as
                   *mut PIEVERTEX as *mut libc::c_void,
               &mut *(*tileScreenInfo.as_mut_ptr().offset(i.wrapping_add(0 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint)
                                                              as
                                                              isize)).as_mut_ptr().offset(j.wrapping_add(0
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             libc::c_uint)
                                                                                              as
                                                                                              isize)
                   as *mut SVMESH as *const libc::c_void,
               ::std::mem::size_of::<PIEVERTEX>() as libc::c_ulong);
        aVrts[0 as libc::c_int as usize].sx =
            tileScreenInfo[i.wrapping_add(0 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(0 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].x;
        aVrts[0 as libc::c_int as usize].sy =
            tileScreenInfo[i.wrapping_add(0 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(0 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].water_height;
        aVrts[0 as libc::c_int as usize].sz =
            tileScreenInfo[i.wrapping_add(0 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(0 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].z;
        memcpy(&mut *aVrts.as_mut_ptr().offset(1 as libc::c_int as isize) as
                   *mut PIEVERTEX as *mut libc::c_void,
               &mut *(*tileScreenInfo.as_mut_ptr().offset(i.wrapping_add(1 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint)
                                                              as
                                                              isize)).as_mut_ptr().offset(j.wrapping_add(1
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             libc::c_uint)
                                                                                              as
                                                                                              isize)
                   as *mut SVMESH as *const libc::c_void,
               ::std::mem::size_of::<PIEVERTEX>() as libc::c_ulong);
        aVrts[1 as libc::c_int as usize].sx =
            tileScreenInfo[i.wrapping_add(1 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(1 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].x;
        aVrts[1 as libc::c_int as usize].sy =
            tileScreenInfo[i.wrapping_add(1 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(1 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].water_height;
        aVrts[1 as libc::c_int as usize].sz =
            tileScreenInfo[i.wrapping_add(1 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(1 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].z;
        memcpy(&mut *aVrts.as_mut_ptr().offset(2 as libc::c_int as isize) as
                   *mut PIEVERTEX as *mut libc::c_void,
               &mut *(*tileScreenInfo.as_mut_ptr().offset(i.wrapping_add(1 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint)
                                                              as
                                                              isize)).as_mut_ptr().offset(j.wrapping_add(0
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             libc::c_uint)
                                                                                              as
                                                                                              isize)
                   as *mut SVMESH as *const libc::c_void,
               ::std::mem::size_of::<PIEVERTEX>() as libc::c_ulong);
        aVrts[2 as libc::c_int as usize].sx =
            tileScreenInfo[i.wrapping_add(1 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(0 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].x;
        aVrts[2 as libc::c_int as usize].sy =
            tileScreenInfo[i.wrapping_add(1 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(0 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].water_height;
        aVrts[2 as libc::c_int as usize].sz =
            tileScreenInfo[i.wrapping_add(1 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(0 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].z;
        pie_DrawPoly(3 as libc::c_int, aVrts.as_mut_ptr(),
                     tileTexInfo[(tileNumber &
                                      0x1ff as libc::c_int as libc::c_uint) as
                                     usize].texPage as SDWORD,
                     0 as *mut libc::c_void);
    };
}
//fast version - optimised
// Render a water tile.
//
#[no_mangle]
pub unsafe extern "C" fn drawTerrainWaterTile(mut i: UDWORD, mut j: UDWORD) {
    let mut actualX: UDWORD = 0;
    let mut actualY: UDWORD = 0;
    let mut psTile: *mut MAPTILE = 0 as *mut MAPTILE;
    //BOOL	bOutlined;
    let mut tileNumber: UDWORD = 0;
    //UDWORD	renderFlag;
    let mut offset: iPoint = iPoint{x: 0, y: 0,};
    let mut aVrts: [PIEVERTEX; 3] =
        [PIEVERTEX{sx: 0,
                   sy: 0,
                   sz: 0,
                   tu: 0,
                   tv: 0,
                   light:
                       PIELIGHT{byte:
                                    PIELIGHTBYTES{b: 0, g: 0, r: 0, a: 0,},},
                   specular:
                       PIELIGHT{byte:
                                    PIELIGHTBYTES{b: 0,
                                                  g: 0,
                                                  r: 0,
                                                  a: 0,},},}; 3];
    /* Get the correct tile index for the x coordinate */
    actualX = (playerXTile as libc::c_uint).wrapping_add(j);
    /* Get the correct tile index for the y coordinate */
    actualY = (playerZTile as libc::c_uint).wrapping_add(i);
    /* Let's just get out now if we're not supposed to draw it */
    if actualX < 0 as libc::c_int as libc::c_uint ||
           actualY < 0 as libc::c_int as libc::c_uint ||
           actualX > mapWidth.wrapping_sub(1 as libc::c_int as libc::c_uint)
           ||
           actualY > mapHeight.wrapping_sub(1 as libc::c_int as libc::c_uint)
       {
        return
        //		psTile = &edgeTile;
//		CLEAR_TILE_HIGHLIGHT(psTile);
    }
    psTile = mapTile(actualX, actualY);
    // If it's a water tile then draw the water
    if terrainTypes[((*psTile).texture as libc::c_int & 0x1ff as libc::c_int)
                        as usize] as libc::c_int == TER_WATER as libc::c_int {
        tileNumber = getWaterTileNum();
        // Draw the main water tile.
        /* 3dfx is pre stored and indexed */
        pie_SetTexturePage(tileTexInfo[(tileNumber &
                                            0x1ff as libc::c_int as
                                                libc::c_uint) as
                                           usize].texPage as
                               SDWORD); //jps 15 apr99
        offset.x =
            tileTexInfo[(tileNumber & 0x1ff as libc::c_int as libc::c_uint) as
                            usize].xOffset.wrapping_mul(64 as libc::c_int as
                                                            libc::c_uint) as
                int32; //jps 15 apr99
        offset.y =
            tileTexInfo[(tileNumber & 0x1ff as libc::c_int as libc::c_uint) as
                            usize].yOffset.wrapping_mul(64 as libc::c_int as
                                                            libc::c_uint) as
                int32;
        tileScreenInfo[i.wrapping_add(0 as libc::c_int as libc::c_uint) as
                           usize][j.wrapping_add(0 as libc::c_int as
                                                     libc::c_uint) as
                                      usize].tu =
            (offset.x + 1 as libc::c_int) as UWORD;
        tileScreenInfo[i.wrapping_add(0 as libc::c_int as libc::c_uint) as
                           usize][j.wrapping_add(0 as libc::c_int as
                                                     libc::c_uint) as
                                      usize].tv = offset.y as UWORD;
        tileScreenInfo[i.wrapping_add(0 as libc::c_int as libc::c_uint) as
                           usize][j.wrapping_add(1 as libc::c_int as
                                                     libc::c_uint) as
                                      usize].tu =
            (offset.x + 63 as libc::c_int) as UWORD;
        tileScreenInfo[i.wrapping_add(0 as libc::c_int as libc::c_uint) as
                           usize][j.wrapping_add(1 as libc::c_int as
                                                     libc::c_uint) as
                                      usize].tv = offset.y as UWORD;
        tileScreenInfo[i.wrapping_add(1 as libc::c_int as libc::c_uint) as
                           usize][j.wrapping_add(1 as libc::c_int as
                                                     libc::c_uint) as
                                      usize].tu =
            (offset.x + 63 as libc::c_int) as UWORD;
        tileScreenInfo[i.wrapping_add(1 as libc::c_int as libc::c_uint) as
                           usize][j.wrapping_add(1 as libc::c_int as
                                                     libc::c_uint) as
                                      usize].tv =
            (offset.y + 31 as libc::c_int) as UWORD;
        tileScreenInfo[i.wrapping_add(1 as libc::c_int as libc::c_uint) as
                           usize][j.wrapping_add(0 as libc::c_int as
                                                     libc::c_uint) as
                                      usize].tu =
            (offset.x + 1 as libc::c_int) as UWORD;
        tileScreenInfo[i.wrapping_add(1 as libc::c_int as libc::c_uint) as
                           usize][j.wrapping_add(0 as libc::c_int as
                                                     libc::c_uint) as
                                      usize].tv =
            (offset.y + 31 as libc::c_int) as UWORD;
        memcpy(&mut *aVrts.as_mut_ptr().offset(0 as libc::c_int as isize) as
                   *mut PIEVERTEX as *mut libc::c_void,
               &mut *(*tileScreenInfo.as_mut_ptr().offset(i.wrapping_add(0 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint)
                                                              as
                                                              isize)).as_mut_ptr().offset(j.wrapping_add(0
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             libc::c_uint)
                                                                                              as
                                                                                              isize)
                   as *mut SVMESH as *const libc::c_void,
               ::std::mem::size_of::<PIEVERTEX>() as libc::c_ulong);
        aVrts[0 as libc::c_int as usize].sx =
            tileScreenInfo[i.wrapping_add(0 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(0 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].x;
        aVrts[0 as libc::c_int as usize].sy =
            tileScreenInfo[i.wrapping_add(0 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(0 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].water_height;
        aVrts[0 as libc::c_int as usize].sz =
            tileScreenInfo[i.wrapping_add(0 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(0 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].z;
        aVrts[0 as libc::c_int as usize].light =
            tileScreenInfo[i.wrapping_add(0 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(0 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].wlight;
        aVrts[0 as libc::c_int as usize].light.byte.a =
            255 as libc::c_int as UBYTE;
        memcpy(&mut *aVrts.as_mut_ptr().offset(1 as libc::c_int as isize) as
                   *mut PIEVERTEX as *mut libc::c_void,
               &mut *(*tileScreenInfo.as_mut_ptr().offset(i.wrapping_add(0 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint)
                                                              as
                                                              isize)).as_mut_ptr().offset(j.wrapping_add(1
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             libc::c_uint)
                                                                                              as
                                                                                              isize)
                   as *mut SVMESH as *const libc::c_void,
               ::std::mem::size_of::<PIEVERTEX>() as libc::c_ulong);
        aVrts[1 as libc::c_int as usize].sx =
            tileScreenInfo[i.wrapping_add(0 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(1 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].x;
        aVrts[1 as libc::c_int as usize].sy =
            tileScreenInfo[i.wrapping_add(0 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(1 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].water_height;
        aVrts[1 as libc::c_int as usize].sz =
            tileScreenInfo[i.wrapping_add(0 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(1 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].z;
        aVrts[1 as libc::c_int as usize].light =
            tileScreenInfo[i.wrapping_add(0 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(1 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].wlight;
        aVrts[1 as libc::c_int as usize].light.byte.a =
            255 as libc::c_int as UBYTE;
        memcpy(&mut *aVrts.as_mut_ptr().offset(2 as libc::c_int as isize) as
                   *mut PIEVERTEX as *mut libc::c_void,
               &mut *(*tileScreenInfo.as_mut_ptr().offset(i.wrapping_add(1 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint)
                                                              as
                                                              isize)).as_mut_ptr().offset(j.wrapping_add(1
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             libc::c_uint)
                                                                                              as
                                                                                              isize)
                   as *mut SVMESH as *const libc::c_void,
               ::std::mem::size_of::<PIEVERTEX>() as libc::c_ulong);
        aVrts[2 as libc::c_int as usize].sx =
            tileScreenInfo[i.wrapping_add(1 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(1 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].x;
        aVrts[2 as libc::c_int as usize].sy =
            tileScreenInfo[i.wrapping_add(1 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(1 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].water_height;
        aVrts[2 as libc::c_int as usize].sz =
            tileScreenInfo[i.wrapping_add(1 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(1 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].z;
        aVrts[2 as libc::c_int as usize].light =
            tileScreenInfo[i.wrapping_add(1 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(1 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].wlight;
        aVrts[2 as libc::c_int as usize].light.byte.a =
            255 as libc::c_int as UBYTE;
        pie_DrawPoly(3 as libc::c_int, aVrts.as_mut_ptr(),
                     tileTexInfo[(tileNumber &
                                      0x1ff as libc::c_int as libc::c_uint) as
                                     usize].texPage as SDWORD,
                     &mut waterRealValue as *mut FRACT as *mut libc::c_void);
        memcpy(&mut *aVrts.as_mut_ptr().offset(1 as libc::c_int as isize) as
                   *mut PIEVERTEX as *mut libc::c_void,
               &mut *aVrts.as_mut_ptr().offset(2 as libc::c_int as isize) as
                   *mut PIEVERTEX as *const libc::c_void,
               ::std::mem::size_of::<PIEVERTEX>() as libc::c_ulong);
        memcpy(&mut *aVrts.as_mut_ptr().offset(2 as libc::c_int as isize) as
                   *mut PIEVERTEX as *mut libc::c_void,
               &mut *(*tileScreenInfo.as_mut_ptr().offset(i.wrapping_add(1 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint)
                                                              as
                                                              isize)).as_mut_ptr().offset(j.wrapping_add(0
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             libc::c_uint)
                                                                                              as
                                                                                              isize)
                   as *mut SVMESH as *const libc::c_void,
               ::std::mem::size_of::<PIEVERTEX>() as libc::c_ulong);
        aVrts[2 as libc::c_int as usize].sx =
            tileScreenInfo[i.wrapping_add(1 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(0 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].x;
        aVrts[2 as libc::c_int as usize].sy =
            tileScreenInfo[i.wrapping_add(1 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(0 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].water_height;
        aVrts[2 as libc::c_int as usize].sz =
            tileScreenInfo[i.wrapping_add(1 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(0 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].z;
        aVrts[2 as libc::c_int as usize].light =
            tileScreenInfo[i.wrapping_add(1 as libc::c_int as libc::c_uint) as
                               usize][j.wrapping_add(0 as libc::c_int as
                                                         libc::c_uint) as
                                          usize].wlight;
        aVrts[2 as libc::c_int as usize].light.byte.a =
            255 as libc::c_int as UBYTE;
        pie_DrawPoly(3 as libc::c_int, aVrts.as_mut_ptr(),
                     tileTexInfo[(tileNumber &
                                      0x1ff as libc::c_int as libc::c_uint) as
                                     usize].texPage as SDWORD,
                     &mut waterRealValue as *mut FRACT as *mut libc::c_void);
        if (*psTile).texture as libc::c_int & 0x1ff as libc::c_int !=
               WaterTileID as libc::c_int {
            drawTerrainWEdgeTile(i, j);
        }
    };
}
// -------------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn getSuggestedPitch() -> UDWORD {
    let mut worldAngle: UDWORD = 0;
    let mut xPos: UDWORD = 0;
    let mut yPos: UDWORD = 0;
    let mut pitch_0: SDWORD = 0;
    worldAngle =
        (player.r.y as
             UDWORD).wrapping_div((65536 as libc::c_int / 360 as libc::c_int)
                                      as
                                      libc::c_uint).wrapping_rem(360 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint);
    /* Now, we need to track angle too - to avoid near z clip! */
    xPos =
        (player.p.x as
             libc::c_uint).wrapping_add(visibleXTiles.wrapping_div(2 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint).wrapping_mul(128
                                                                                                      as
                                                                                                      libc::c_int
                                                                                                      as
                                                                                                      libc::c_uint));
    yPos =
        (player.p.z as
             libc::c_uint).wrapping_add(visibleYTiles.wrapping_div(2 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint).wrapping_mul(128
                                                                                                      as
                                                                                                      libc::c_int
                                                                                                      as
                                                                                                      libc::c_uint));
    // 	getBestPitchToEdgeOfGrid(xPos,yPos,360-worldAngle,&pitch);
    getPitchToHighestPoint(xPos, yPos,
                           (360 as libc::c_int as
                                libc::c_uint).wrapping_sub(worldAngle),
                           0 as libc::c_int as UDWORD, &mut pitch_0);
    if pitch_0 < abs(-(14 as libc::c_int)) {
        pitch_0 = abs(-(14 as libc::c_int))
    }
    if pitch_0 > abs(-(50 as libc::c_int)) {
        pitch_0 = abs(-(50 as libc::c_int))
    }
    return pitch_0 as UDWORD;
}
// -------------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn trackHeight(mut desiredHeight: SDWORD) {
    let mut fraction: FRACT = 0.;
    let mut pitch_0: UDWORD = 0;
    let mut angConcern: SDWORD = 0;
    let mut desPitch: UDWORD = 0;
    /* What fraction of a second did last game loop take */
    fraction = frameTime2 as FRACT / 1000 as libc::c_int as FRACT;
    /* How far are we from desired hieght? */
    separation = (desiredHeight - player.p.y) as FRACT;
    /* Work out accelertion... */
    acceleration =
        (64 as libc::c_int as libc::c_float /
             10 as libc::c_int as libc::c_float *
             2 as libc::c_int as libc::c_float * separation -
             4 as libc::c_int as libc::c_float /
                 1 as libc::c_int as libc::c_float * heightSpeed as FRACT) as
            SDWORD;
    /* ...and now speed */
    heightSpeed += (acceleration as FRACT * fraction) as SDWORD;
    /* Adjust the height accordingly */
    player.p.y += (heightSpeed as FRACT * fraction) as SDWORD;
    /* Now do auto pitch as well, but only if we're not using mouselook and not tracking */
    if getWarCamStatus() == 0 && getRotActive() == 0 {
        /* Get the suggested pitch */
        pitch_0 = getSuggestedPitch();
        //			flushConsoleMessages();
//			CONPRINTF(ConsoleString,(ConsoleString,"Player.r.x : %d",player.r.x/182));
//			CONPRINTF(ConsoleString,(ConsoleString,"Pitch : %d",pitch));
        while player.r.x < 0 as libc::c_int
              /* Make sure this isn't negative */
              {
            player.r.x +=
                65536 as libc::c_int / 360 as libc::c_int * 360 as libc::c_int
        }
        while player.r.x >
                  65536 as libc::c_int / 360 as libc::c_int *
                      360 as libc::c_int
              /* Or too much */
              {
            player.r.x -=
                65536 as libc::c_int / 360 as libc::c_int * 360 as libc::c_int
        }
        desPitch = (360 as libc::c_int - getDesiredPitch()) as UDWORD;
        if !(abs(pitch_0.wrapping_sub(desPitch) as libc::c_int) <
                 2 as libc::c_int) {
            /* What's the desired pitch from the player */
            /* Do nothing if we're within 2 degrees of optimum */
            /* Force adjust if too low - stops near z clip */
            if pitch_0 > desPitch {
                angConcern =
                    ((65536 as libc::c_int / 360 as libc::c_int) as
                         libc::c_uint).wrapping_mul((360 as libc::c_int as
                                                         libc::c_uint).wrapping_sub(pitch_0))
                        as SDWORD;
                aSep = (angConcern - player.r.x) as FRACT;
                aAccel =
                    (64 as libc::c_int as libc::c_float /
                         10 as libc::c_int as libc::c_float * aSep -
                         4 as libc::c_int as libc::c_float /
                             1 as libc::c_int as libc::c_float *
                             aSpeed as FRACT) as SDWORD;
                aSpeed += (aAccel as FRACT * fraction) as SDWORD;
                player.r.x += (aSpeed as FRACT * fraction) as SDWORD
            } else {
                /* Else, move towards player's last selected pitch */
                angConcern =
                    ((65536 as libc::c_int / 360 as libc::c_int) as
                         libc::c_uint).wrapping_mul((360 as libc::c_int as
                                                         libc::c_uint).wrapping_sub(desPitch))
                        as SDWORD;
                aSep = (angConcern - player.r.x) as FRACT;
                aAccel =
                    (64 as libc::c_int as libc::c_float /
                         10 as libc::c_int as libc::c_float * aSep -
                         4 as libc::c_int as libc::c_float /
                             1 as libc::c_int as libc::c_float *
                             aSpeed as FRACT) as SDWORD;
                aSpeed += (aAccel as FRACT * fraction) as SDWORD;
                player.r.x += (aSpeed as FRACT * fraction) as SDWORD
            }
        }
    };
}
// -------------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn toggleEnergyBars() {
    barMode = barMode.wrapping_add(1);
    if barMode > 3 as libc::c_int as libc::c_uint {
        barMode = 0 as libc::c_int as UDWORD
    };
    //	bEnergyBars = !bEnergyBars;
}
// -------------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn toggleReloadBarDisplay() {
    bReloadBars = (bReloadBars == 0) as libc::c_int;
}
// -------------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn assignSensorTarget(mut psObj: *mut BASE_OBJECT) {
    bSensorTargetting = 1 as libc::c_int;
    lastTargetAssignation = gameTime2;
    psSensorObj = psObj;
}
//void	assignSensorTarget( DROID *psDroid );
// -------------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn assignDestTarget() {
    bDestTargetting = 1 as libc::c_int;
    lastDestAssignation = gameTime2;
    destTargetX = mouseX() as UDWORD;
    destTargetY = mouseY() as UDWORD;
    destTileX = mouseTileX as UDWORD;
    destTileY = mouseTileY as UDWORD;
}
// -------------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn processSensorTarget() {
    let mut x: SWORD = 0;
    let mut y: SWORD = 0;
    let mut offset: SWORD = 0;
    let mut x0: SWORD = 0;
    let mut y0: SWORD = 0;
    let mut x1: SWORD = 0;
    let mut y1: SWORD = 0;
    let mut index: UDWORD = 0;
    if bSensorTargetting != 0 {
        if gameTime2.wrapping_sub(lastTargetAssignation) <
               (4 as libc::c_int * 1000 as libc::c_int / 5 as libc::c_int) as
                   libc::c_uint {
            if (*psSensorObj).died == 0 &&
                   (*psSensorObj).sDisplay.frameNumber == currentGameFrame {
                x = (*psSensorObj).sDisplay.screenX as SWORD;
                y = (*psSensorObj).sDisplay.screenY as SWORD;
                if gamePaused() == 0 {
                    index =
                        (IMAGE_BLUE1 as libc::c_int as
                             libc::c_uint).wrapping_add(getStaticTimeValueRange(1020
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    UDWORD,
                                                                                5
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    UDWORD))
                } else { index = IMAGE_BLUE1 as libc::c_int as UDWORD }
                pie_ImageFileID(IntImages, index as UWORD, x as libc::c_int,
                                y as libc::c_int);
                offset =
                    (12 as libc::c_int as
                         libc::c_uint).wrapping_add(((4 as libc::c_int *
                                                          1000 as libc::c_int
                                                          / 5 as libc::c_int)
                                                         as
                                                         libc::c_uint).wrapping_sub(gameTime2.wrapping_sub(lastTargetAssignation)).wrapping_div(2
                                                                                                                                                    as
                                                                                                                                                    libc::c_int
                                                                                                                                                    as
                                                                                                                                                    libc::c_uint))
                        as SWORD;
                x0 = (x as libc::c_int - offset as libc::c_int) as SWORD;
                y0 = (y as libc::c_int - offset as libc::c_int) as SWORD;
                x1 = (x as libc::c_int + offset as libc::c_int) as SWORD;
                y1 = (y as libc::c_int + offset as libc::c_int) as SWORD;
                pie_Line(x0 as libc::c_int, y0 as libc::c_int,
                         x0 as libc::c_int + 8 as libc::c_int,
                         y0 as libc::c_int,
                         *colours.as_mut_ptr().offset(15 as libc::c_int as
                                                          isize) as uint32);
                pie_Line(x0 as libc::c_int, y0 as libc::c_int,
                         x0 as libc::c_int,
                         y0 as libc::c_int + 8 as libc::c_int,
                         *colours.as_mut_ptr().offset(15 as libc::c_int as
                                                          isize) as uint32);
                pie_Line(x1 as libc::c_int, y0 as libc::c_int,
                         x1 as libc::c_int - 8 as libc::c_int,
                         y0 as libc::c_int,
                         *colours.as_mut_ptr().offset(15 as libc::c_int as
                                                          isize) as uint32);
                pie_Line(x1 as libc::c_int, y0 as libc::c_int,
                         x1 as libc::c_int,
                         y0 as libc::c_int + 8 as libc::c_int,
                         *colours.as_mut_ptr().offset(15 as libc::c_int as
                                                          isize) as uint32);
                pie_Line(x1 as libc::c_int, y1 as libc::c_int,
                         x1 as libc::c_int - 8 as libc::c_int,
                         y1 as libc::c_int,
                         *colours.as_mut_ptr().offset(15 as libc::c_int as
                                                          isize) as uint32);
                pie_Line(x1 as libc::c_int, y1 as libc::c_int,
                         x1 as libc::c_int,
                         y1 as libc::c_int - 8 as libc::c_int,
                         *colours.as_mut_ptr().offset(15 as libc::c_int as
                                                          isize) as uint32);
                pie_Line(x0 as libc::c_int, y1 as libc::c_int,
                         x0 as libc::c_int + 8 as libc::c_int,
                         y1 as libc::c_int,
                         *colours.as_mut_ptr().offset(15 as libc::c_int as
                                                          isize) as uint32);
                pie_Line(x0 as libc::c_int, y1 as libc::c_int,
                         x0 as libc::c_int,
                         y1 as libc::c_int - 8 as libc::c_int,
                         *colours.as_mut_ptr().offset(15 as libc::c_int as
                                                          isize) as uint32);
            } else { bSensorTargetting = 0 as libc::c_int }
        } else { bSensorTargetting = 0 as libc::c_int }
    };
}
// -------------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn processDestinationTarget() {
    let mut x: SWORD = 0;
    let mut y: SWORD = 0;
    let mut offset: SWORD = 0;
    let mut x0: SWORD = 0;
    let mut y0: SWORD = 0;
    let mut x1: SWORD = 0;
    let mut y1: SWORD = 0;
    if bDestTargetting != 0 {
        if gameTime2.wrapping_sub(lastDestAssignation) <
               (1000 as libc::c_int / 4 as libc::c_int) as libc::c_uint {
            x = destTargetX as SWORD;
            y = destTargetY as SWORD;
            offset =
                ((1000 as libc::c_int / 4 as libc::c_int) as
                     libc::c_uint).wrapping_sub(gameTime2.wrapping_sub(lastDestAssignation)).wrapping_div(2
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              libc::c_uint)
                    as SWORD;
            x0 = (x as libc::c_int - offset as libc::c_int) as SWORD;
            y0 = (y as libc::c_int - offset as libc::c_int) as SWORD;
            x1 = (x as libc::c_int + offset as libc::c_int) as SWORD;
            y1 = (y as libc::c_int + offset as libc::c_int) as SWORD;
            pie_BoxFillIndex(x0 as libc::c_int, y0 as libc::c_int,
                             x0 as libc::c_int + 2 as libc::c_int,
                             y0 as libc::c_int + 2 as libc::c_int,
                             *colours.as_mut_ptr().offset(15 as libc::c_int as
                                                              isize));
            //		iV_Line(x0,y1,x0,y1-2,COL_WHITE);
            pie_BoxFillIndex(x1 as libc::c_int - 2 as libc::c_int,
                             y0 as libc::c_int - 2 as libc::c_int,
                             x1 as libc::c_int, y0 as libc::c_int,
                             *colours.as_mut_ptr().offset(15 as libc::c_int as
                                                              isize));
            pie_BoxFillIndex(x1 as libc::c_int - 2 as libc::c_int,
                             y1 as libc::c_int - 2 as libc::c_int,
                             x1 as libc::c_int, y1 as libc::c_int,
                             *colours.as_mut_ptr().offset(15 as libc::c_int as
                                                              isize));
            pie_BoxFillIndex(x0 as libc::c_int, y1 as libc::c_int,
                             x0 as libc::c_int + 2 as libc::c_int,
                             y1 as libc::c_int + 2 as libc::c_int,
                             *colours.as_mut_ptr().offset(15 as libc::c_int as
                                                              isize));
        } else { bDestTargetting = 0 as libc::c_int }
    };
}
//		iV_Line(x0,y0,x0,y0+2,COL_WHITE);
//		iV_Line(x1,y0,x1,y0+2,COL_WHITE);
//		iV_Line(x1,y1,x1,y1-2,COL_WHITE);
// -------------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn setEnergyBarDisplay(mut val: BOOL) {
    bEnergyBars = val;
}
// -------------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn setUnderwaterTile(mut num: UDWORD) {
    underwaterTile = num;
}
// -------------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn setRubbleTile(mut num: UDWORD) { rubbleTile = num; }
// -------------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn getWaterTileNum() -> UDWORD {
    return underwaterTile;
}
//extern void	assignSensorTarget( DROID *psDroid );
// -------------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn getRubbleTileNum() -> UDWORD { return rubbleTile; }
// -------------------------------------------------------------------------------------
#[no_mangle]
pub static mut lastSpinVal: UDWORD = 0;
#[no_mangle]
pub unsafe extern "C" fn testEffect2(mut player_0: UDWORD) {
    let mut val: SDWORD = 0;
    let mut radius: SDWORD = 0;
    let mut angle: UDWORD = 0;
    let mut psStructure: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut xDif: SDWORD = 0;
    let mut yDif: SDWORD = 0;
    let mut pos: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut numConnected: UDWORD = 0;
    let mut psPowerGen: *mut POWER_GEN = 0 as *mut POWER_GEN;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut gameDiv: UDWORD = 0;
    let mut i: UDWORD = 0;
    let mut psReArmPad: *mut REARM_PAD = 0 as *mut REARM_PAD;
    let mut psChosenObj: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut bFXSize: UWORD = 0;
    psStructure = apsStructLists[player_0 as usize];
    while !psStructure.is_null() {
        if (*psStructure).status as libc::c_int == SS_BUILT as libc::c_int {
            if (*(*psStructure).pStructureType).type_0 ==
                   REF_POWER_GEN as libc::c_int as libc::c_uint &&
                   (*psStructure).visible[selectedPlayer as usize] as
                       libc::c_int != 0 {
                psPowerGen = (*psStructure).pFunctionality as *mut POWER_GEN;
                numConnected = 0 as libc::c_int as UDWORD;
                i = 0 as libc::c_int as UDWORD;
                while i < 4 as libc::c_int as libc::c_uint {
                    if !(*psPowerGen).apResExtractors[i as usize].is_null() {
                        numConnected = numConnected.wrapping_add(1)
                    }
                    i = i.wrapping_add(1)
                }
                /* No effect if nothing connected */
                if !(numConnected == 0) {
                    match numConnected {
                        1 | 2 => {
                            gameDiv =
                                1440 as libc::c_int as
                                    UDWORD; // really fast!!!
                            val = 4 as libc::c_int
                        }
                        3 | 4 | _ => {
                            gameDiv = 1080 as libc::c_int as UDWORD;
                            val = 3 as libc::c_int
                        }
                    }
                    angle = gameTime2.wrapping_rem(gameDiv);
                    val = angle.wrapping_div(val as libc::c_uint) as SDWORD;
                    /* New addition - it shows how many are connected... */
                    i = 0 as libc::c_int as UDWORD; // around the spire
                    while i < numConnected {
                        radius =
                            (32 as libc::c_int as
                                 libc::c_uint).wrapping_sub(i.wrapping_mul(2
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_uint))
                                as SDWORD; // cos it's fixed point
                        xDif =
                            radius *
                                *aSinTable.as_mut_ptr().offset(((65536 as
                                                                     libc::c_int
                                                                     /
                                                                     360 as
                                                                         libc::c_int
                                                                     * val) as
                                                                    uint16 as
                                                                    libc::c_int
                                                                    >>
                                                                    4 as
                                                                        libc::c_int)
                                                                   as
                                                                   isize); // 64 up to get to base of spire
                        yDif =
                            radius *
                                *aSinTable.as_mut_ptr().offset((((65536 as
                                                                      libc::c_int
                                                                      /
                                                                      360 as
                                                                          libc::c_int
                                                                      * val)
                                                                     as uint16
                                                                     as
                                                                     libc::c_int
                                                                     >>
                                                                     4 as
                                                                         libc::c_int)
                                                                    +
                                                                    1024 as
                                                                        libc::c_int)
                                                                   as
                                                                   isize); // half normal plasma size...
                        xDif = xDif / 4096 as libc::c_int;
                        yDif = yDif / 4096 as libc::c_int;
                        pos.x = (*psStructure).x as libc::c_int + xDif;
                        pos.z = (*psStructure).y as libc::c_int + yDif;
                        pos.y =
                            ((map_Height(pos.x as UDWORD, pos.z as UDWORD) as
                                  libc::c_int + 64 as libc::c_int) as
                                 libc::c_uint).wrapping_add(i.wrapping_mul(20
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_uint))
                                as int32;
                        effectGiveAuxVar(50 as libc::c_int as UDWORD);
                        addEffect(&mut pos, EFFECT_EXPLOSION,
                                  EXPLOSION_TYPE_LASER, 0 as libc::c_int,
                                  0 as *mut iIMDShape, 0 as libc::c_int);
                        pos.x = (*psStructure).x as libc::c_int - xDif;
                        pos.z = (*psStructure).y as libc::c_int - yDif;
                        //					pos.y = map_Height(pos.x,pos.z) + 64 + (i*20);	// 64 up to get to base of spire
                        effectGiveAuxVar(50 as libc::c_int as
                                             UDWORD); // half normal plasma size...
                        addEffect(&mut pos, EFFECT_EXPLOSION,
                                  EXPLOSION_TYPE_LASER, 0 as libc::c_int,
                                  0 as *mut iIMDShape, 0 as libc::c_int);
                        i = i.wrapping_add(1)
                    }
                }
            } else if (*(*psStructure).pStructureType).type_0 ==
                          REF_REARM_PAD as libc::c_int as libc::c_uint &&
                          (*psStructure).visible[selectedPlayer as usize] as
                              libc::c_int != 0 {
                psReArmPad = (*psStructure).pFunctionality as *mut REARM_PAD;
                psChosenObj = (*psReArmPad).psObj;
                if !psChosenObj.is_null() {
                    if (*(psChosenObj as
                              *mut DROID)).visible[selectedPlayer as usize] !=
                           0 {
                        bFXSize = 0 as libc::c_int as UWORD;
                        psDroid = psChosenObj as *mut DROID;
                        if (*psDroid).died == 0 &&
                               (*psDroid).action ==
                                   DACTION_WAITDURINGREARM as libc::c_int {
                            bFXSize = 30 as libc::c_int as UWORD
                        }
                        /* Might be a re-arm pad! */
                        /* Then it's repairing...? */
                        if gamePaused() == 0 {
                            lastSpinVal =
                                getTimeValueRange(720 as libc::c_int as
                                                      UDWORD,
                                                  360 as libc::c_int as
                                                      UDWORD);
                            val = lastSpinVal as SDWORD
                            // grab an angle - 4 seconds cyclic
                        } else {
                            val = lastSpinVal as SDWORD
                        } // cos it's fixed point
                        radius =
                            (*(*psStructure).sDisplay.imd).radius; // half normal plasma size...
                        xDif =
                            radius *
                                *aSinTable.as_mut_ptr().offset(((65536 as
                                                                     libc::c_int
                                                                     /
                                                                     360 as
                                                                         libc::c_int
                                                                     * val) as
                                                                    uint16 as
                                                                    libc::c_int
                                                                    >>
                                                                    4 as
                                                                        libc::c_int)
                                                                   as
                                                                   isize); // buildings are level!
                        yDif =
                            radius *
                                *aSinTable.as_mut_ptr().offset((((65536 as
                                                                      libc::c_int
                                                                      /
                                                                      360 as
                                                                          libc::c_int
                                                                      * val)
                                                                     as uint16
                                                                     as
                                                                     libc::c_int
                                                                     >>
                                                                     4 as
                                                                         libc::c_int)
                                                                    +
                                                                    1024 as
                                                                        libc::c_int)
                                                                   as isize);
                        xDif = xDif / 4096 as libc::c_int;
                        yDif = yDif / 4096 as libc::c_int;
                        pos.x = (*psStructure).x as libc::c_int + xDif;
                        pos.z = (*psStructure).y as libc::c_int + yDif;
                        pos.y =
                            map_Height(pos.x as UDWORD, pos.z as UDWORD) as
                                libc::c_int +
                                (*(*psStructure).sDisplay.imd).ymax;
                        effectGiveAuxVar((30 as libc::c_int +
                                              bFXSize as libc::c_int) as
                                             UDWORD);
                        addEffect(&mut pos, EFFECT_EXPLOSION,
                                  EXPLOSION_TYPE_LASER, 0 as libc::c_int,
                                  0 as *mut iIMDShape, 0 as libc::c_int);
                        pos.x = (*psStructure).x as libc::c_int - xDif;
                        pos.z = (*psStructure).y as libc::c_int - yDif;
                        //				pos.y = map_Height(pos.x,pos.z) + psStructure->sDisplay->ymax;
                        effectGiveAuxVar((30 as libc::c_int +
                                              bFXSize as libc::c_int) as
                                             UDWORD); // half normal plasma size...
                        addEffect(&mut pos, EFFECT_EXPLOSION,
                                  EXPLOSION_TYPE_LASER, 0 as libc::c_int,
                                  0 as *mut iIMDShape, 0 as libc::c_int);
                    }
                }
            }
        }
        //return;
                    //keep looking for another!
        psStructure = (*psStructure).psNext
    };
}
#[no_mangle]
pub unsafe extern "C" fn testEffect() {
    let mut i: UDWORD = 0;
    /* Hardware only effect, and then only if you've got additive! */
	// if ( RODZ AND war_GetAdditive() ) )
    /* Only do for player 0 power stations */
    if bMultiPlayer != 0 {
        i = 0 as libc::c_int as UDWORD;
        while i < 8 as libc::c_int as libc::c_uint {
            if isHumanPlayer(i) != 0 && !apsStructLists[i as usize].is_null()
               {
                testEffect2(i);
            }
            i = i.wrapping_add(1)
        }
    } else if !apsStructLists[0 as libc::c_int as usize].is_null() {
        testEffect2(0 as libc::c_int as UDWORD);
    };
}
#[no_mangle]
pub unsafe extern "C" fn showDroidSensorRanges() {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    if rangeOnScreen != 0 {
        // note, we still have to decide what to do with multiple units selected, since it will draw it for all of them! -Q 5-10-05
        psDroid = apsDroidLists[selectedPlayer as usize];
        while !psDroid.is_null() {
            if (*psDroid).selected != 0 {
                showSensorRange2(psDroid as *mut BASE_OBJECT);
            }
            psDroid = (*psDroid).psNext
        }
        psStruct = apsStructLists[selectedPlayer as usize];
        while !psStruct.is_null() {
            if (*psStruct).selected != 0 {
                showSensorRange2(psStruct as *mut BASE_OBJECT);
            }
            psStruct = (*psStruct).psNext
        }
    };
    //end if we want to display...
}
#[no_mangle]
pub unsafe extern "C" fn showSensorRange1(mut psDroid: *mut DROID) 
 //this one doesn't do a circle, it displays 30 or so units at a time
 {
    let mut val: SDWORD = 0; // cos it's fixed point
    let mut radius: SDWORD = 0; // 64 up to get to base of spire
    let mut angle: UDWORD = 0; // half normal plasma size...
    let mut xDif: SDWORD = 0;
    let mut yDif: SDWORD = 0;
    let mut sensorRange: UDWORD = 0;
    let mut pos: iVector = iVector{x: 0, y: 0, z: 0,};
    angle = gameTime.wrapping_rem(3600 as libc::c_int as libc::c_uint);
    val = angle.wrapping_div(10 as libc::c_int as libc::c_uint) as SDWORD;
    sensorRange =
        (*asSensorStats.offset((*psDroid).asBits[COMP_SENSOR as libc::c_int as
                                                     usize].nStat as
                                   isize)).range;
    radius = sensorRange as SDWORD;
    xDif =
        radius *
            *aSinTable.as_mut_ptr().offset(((65536 as libc::c_int /
                                                 360 as libc::c_int * val) as
                                                uint16 as libc::c_int >>
                                                4 as libc::c_int) as isize);
    yDif =
        radius *
            *aSinTable.as_mut_ptr().offset((((65536 as libc::c_int /
                                                  360 as libc::c_int * val) as
                                                 uint16 as libc::c_int >>
                                                 4 as libc::c_int) +
                                                1024 as libc::c_int) as
                                               isize);
    xDif = xDif / 4096 as libc::c_int;
    yDif = yDif / 4096 as libc::c_int;
    pos.x = (*psDroid).x as libc::c_int - xDif;
    pos.z = (*psDroid).y as libc::c_int - yDif;
    pos.y =
        map_Height(pos.x as UDWORD, pos.z as UDWORD) as libc::c_int +
            16 as libc::c_int;
    effectGiveAuxVar(80 as libc::c_int as UDWORD);
    addEffect(&mut pos, EFFECT_EXPLOSION, EXPLOSION_TYPE_LASER,
              0 as libc::c_int, 0 as *mut iIMDShape, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn showSensorRange2(mut psObj: *mut BASE_OBJECT) {
    let mut radius: SDWORD = 0;
    let mut xDif: SDWORD = 0;
    let mut yDif: SDWORD = 0;
    let mut sensorRange: UDWORD = 0;
    let mut pos: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut i: UDWORD = 0;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut bBuilding: BOOL = 0 as libc::c_int;
    i = 0 as libc::c_int as UDWORD;
    while i < 360 as libc::c_int as libc::c_uint {
        if (*psObj).type_0 as libc::c_uint ==
               OBJ_DROID as libc::c_int as libc::c_uint {
            psDroid = psObj as *mut DROID;
            sensorRange =
                (*asSensorStats.offset((*psDroid).asBits[COMP_SENSOR as
                                                             libc::c_int as
                                                             usize].nStat as
                                           isize)).range
        } else {
            psStruct = psObj as *mut STRUCTURE;
            sensorRange = (*psStruct).sensorRange as UDWORD;
            bBuilding = 1 as libc::c_int
        }
        //		printf("sensorRange=%d\n",sensorRange);
        radius = sensorRange as SDWORD; // cos it's fixed point
        xDif =
            radius *
                *aSinTable.as_mut_ptr().offset((((65536 as libc::c_int /
                                                      360 as libc::c_int) as
                                                     libc::c_uint).wrapping_mul(i)
                                                    as uint16 as libc::c_int
                                                    >> 4 as libc::c_int) as
                                                   isize); // 64 up to get to base of spire
        yDif =
            radius *
                *aSinTable.as_mut_ptr().offset(((((65536 as libc::c_int /
                                                       360 as libc::c_int) as
                                                      libc::c_uint).wrapping_mul(i)
                                                     as uint16 as libc::c_int
                                                     >> 4 as libc::c_int) +
                                                    1024 as libc::c_int) as
                                                   isize); // half normal plasma size...
        xDif = xDif / 4096 as libc::c_int;
        yDif = yDif / 4096 as libc::c_int;
        pos.x = (*psObj).x as libc::c_int - xDif;
        pos.z = (*psObj).y as libc::c_int - yDif;
        pos.y =
            map_Height(pos.x as UDWORD, pos.z as UDWORD) as libc::c_int +
                16 as libc::c_int;
        effectGiveAuxVar(80 as libc::c_int as UDWORD);
        if bBuilding != 0 {
            addEffect(&mut pos, EFFECT_EXPLOSION, EXPLOSION_TYPE_SMALL,
                      0 as libc::c_int, 0 as *mut iIMDShape,
                      0 as libc::c_int);
        } else {
            addEffect(&mut pos, EFFECT_EXPLOSION, EXPLOSION_TYPE_LASER,
                      0 as libc::c_int, 0 as *mut iIMDShape,
                      0 as libc::c_int);
        }
        i = i.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn debugToggleSensorDisplay() {
    if bSensorDisplay != 0 {
        bSensorDisplay = 0 as libc::c_int
    } else { bSensorDisplay = 1 as libc::c_int };
}
/*returns the graphic ID for a droid rank*/
#[no_mangle]
pub unsafe extern "C" fn getDroidRankGraphic(mut psDroid: *mut DROID)
 -> UDWORD {
    let mut gfxId: UDWORD = 0;
    /* Not found yet */
    gfxId = 0xffffffff as libc::c_uint;
    //#ifdef JOHN
	/* Establish the numerical value of the droid's rank */
    match getDroidLevel(psDroid) {
        0 => { }
        1 => { gfxId = IMAGE_LEV_0 as libc::c_int as UDWORD }
        2 => { gfxId = IMAGE_LEV_1 as libc::c_int as UDWORD }
        3 => { gfxId = IMAGE_LEV_2 as libc::c_int as UDWORD }
        4 => { gfxId = IMAGE_LEV_3 as libc::c_int as UDWORD }
        5 => { gfxId = IMAGE_LEV_4 as libc::c_int as UDWORD }
        6 => { gfxId = IMAGE_LEV_5 as libc::c_int as UDWORD }
        7 => { gfxId = IMAGE_LEV_6 as libc::c_int as UDWORD }
        8 => { gfxId = IMAGE_LEV_7 as libc::c_int as UDWORD }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Weird droid level in drawDroidRank\x00" as *const u8
                          as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"display3d.c\x00" as *const u8 as *const libc::c_char,
                      5828 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 20],
                                                &[libc::c_char; 20]>(b"getDroidRankGraphic\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
        }
    }
    //#else
  	/*
	switch(getDroidLevel(psDroid))
	{
		case 0:
//			gfxId = IMAGE_GN_0;	// Unexperienced
			break;
		case 1:
			gfxId = IMAGE_GN_1;
			break;
		case 2:
			gfxId = IMAGE_GN_2;
			break;
		case 3:
			gfxId = IMAGE_GN_3;
			break;
		case 4:
			gfxId = IMAGE_GN_4;
			break;
		case 5:
			gfxId = IMAGE_GN_5;
			break;
		case 6:
			gfxId = IMAGE_GN_6;	// Experienced
			break;
		case 7:
			gfxId = IMAGE_GN_7;
			break;
		case 8:
			gfxId = IMAGE_GN_8;
			break;
		default:
			ASSERT( FALSE, "Weird droid level in drawDroidRank" );
		break;
	}
	*/
//#endif
    /*	John's routing debug code
	switch (psDroid->sMove.Status)
	{
		case MOVEINACTIVE:
			gfxId = IMAGE_GN_0;	// Unexperienced
			break;
		case MOVENAVIGATE:
			gfxId = IMAGE_GN_1;
			break;
		case MOVETURN:
			gfxId = IMAGE_GN_2;
			break;
		case MOVEPAUSE:
			gfxId = IMAGE_GN_3;
			break;
		case MOVEPOINTTOPOINT:
			gfxId = IMAGE_GN_4;
			break;
		case MOVEROUTE:
			gfxId = IMAGE_GN_5;
			break;
		case MOVEWAITROUTE:
			gfxId = IMAGE_GN_6;	// Experienced
			break;
		case MOVESHUFFLE:
			gfxId = IMAGE_GN_7;
			break;
		case MOVEROUTESHUFFLE:
			gfxId = IMAGE_GN_8;
			break;
		default:
		break;
	}*/
    return gfxId;
}
/*	DOES : Assumes matrix context set and that z-buffer write is force enabled (Always).
	Will render a graphic depiction of the droid's present rank.
	BY : Alex McLean.
*/
#[no_mangle]
pub unsafe extern "C" fn drawDroidRank(mut psDroid: *mut DROID) {
    //UDWORD	droidLevel;
    let mut gfxId: UDWORD = 0;
    gfxId = getDroidRankGraphic(psDroid);
    /* Did we get one? - We should have... */
    if gfxId != 0xffffffff as libc::c_uint {
        /* Render the rank graphic at the correct location */
        // remove hardcoded numbers?!
        pie_ImageFileID(IntImages, gfxId as UWORD,
                        (*psDroid).sDisplay.screenX.wrapping_add(20 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint)
                            as libc::c_int,
                        (*psDroid).sDisplay.screenY.wrapping_add(8 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint)
                            as libc::c_int);
    };
}
/*	DOES : Assumes matrix context set and that z-buffer write is force enabled (Always).
	Will render a graphic depiction of the droid's present rank.
*/
#[no_mangle]
pub unsafe extern "C" fn drawDroidSensorLock(mut psDroid: *mut DROID) {
    //if on fire support duty - must be locked to a Sensor Droid/Structure
    if orderState(psDroid, DORDER_FIRESUPPORT) != 0 {
        /* Render the sensor graphic at the correct location - which is what?!*/
        pie_ImageFileID(IntImages, IMAGE_GN_STAR as libc::c_int as UWORD,
                        (*psDroid).sDisplay.screenX.wrapping_add(20 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint)
                            as libc::c_int,
                        (*psDroid).sDisplay.screenY.wrapping_sub(20 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint)
                            as libc::c_int);
    };
}
unsafe extern "C" fn doConstructionLines() {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut i: UDWORD = 0;
    //	pie_SetDepthBufferStatus(DEPTH_CMP_ALWAYS_WRT_ON);
  //	pie_SetFogStatus(FALSE);
    i =
        0 as libc::c_int as
            UDWORD; //Defining the variable trans to eleminate a runtime debug error
    while i < 8 as libc::c_int as libc::c_uint {
        psDroid = apsDroidLists[i as usize];
        while !psDroid.is_null() {
            if clipXY((*psDroid).x as SDWORD, (*psDroid).y as SDWORD) != 0 {
                if (*psDroid).visible[selectedPlayer as usize] as libc::c_int
                       == 0xff as libc::c_int &&
                       (*psDroid).sMove.Status as libc::c_int !=
                           12 as libc::c_int {
                    if (*psDroid).action == DACTION_BUILD as libc::c_int {
                        if !(*psDroid).psTarget.is_null() {
                            if (*(*psDroid).psTarget).type_0 as libc::c_uint
                                   ==
                                   OBJ_STRUCTURE as libc::c_int as
                                       libc::c_uint {
                                addConstructionLine(psDroid,
                                                    (*psDroid).psTarget as
                                                        *mut STRUCTURE);
                            }
                        }
                    } else if (*psDroid).action ==
                                  DACTION_DEMOLISH as libc::c_int ||
                                  (*psDroid).action ==
                                      DACTION_REPAIR as libc::c_int ||
                                  (*psDroid).action ==
                                      DACTION_CLEARWRECK as libc::c_int ||
                                  (*psDroid).action ==
                                      DACTION_RESTORE as libc::c_int {
                        if !(*psDroid).psActionTarget.is_null() {
                            if (*(*psDroid).psActionTarget).type_0 as
                                   libc::c_uint ==
                                   OBJ_STRUCTURE as libc::c_int as
                                       libc::c_uint {
                                addConstructionLine(psDroid,
                                                    (*psDroid).psActionTarget
                                                        as *mut STRUCTURE);
                            }
                        }
                    }
                }
            }
            psDroid = (*psDroid).psNext
        }
        i = i.wrapping_add(1)
    }
    pie_SetDepthBufferStatus(DEPTH_CMP_LEQ_WRT_ON);
}
unsafe extern "C" fn addConstructionLine(mut psDroid: *mut DROID,
                                         mut psStructure: *mut STRUCTURE) {
    let mut pts: [PIEVERTEX; 3] =
        [PIEVERTEX{sx: 0,
                   sy: 0,
                   sz: 0,
                   tu: 0,
                   tv: 0,
                   light:
                       PIELIGHT{byte:
                                    PIELIGHTBYTES{b: 0, g: 0, r: 0, a: 0,},},
                   specular:
                       PIELIGHT{byte:
                                    PIELIGHTBYTES{b: 0,
                                                  g: 0,
                                                  r: 0,
                                                  a: 0,},},}; 3];
    let mut each: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut point: *mut iVector = 0 as *mut iVector;
    let mut pointIndex: UDWORD = 0;
    let mut realY: SDWORD = 0;
    let mut null: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut vec: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut rx_0: SDWORD = 0;
    let mut rz_0: SDWORD = 0;
    let mut colour: UDWORD = 0;
    let mut specular: UDWORD = 0;
    let mut trans: UDWORD = 0;
    trans = 0 as libc::c_int as UDWORD;
    null.z = 0 as libc::c_int;
    null.y = null.z;
    null.x = null.y;
    each.x = (*psDroid).x as int32;
    each.z = (*psDroid).y as int32;
    each.y = (*psDroid).z as libc::c_int + 24 as libc::c_int;
    vec.x =
        ((each.x - player.p.x) as
             libc::c_uint).wrapping_sub(terrainMidX.wrapping_mul(128 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint))
            as int32;
    vec.z =
        terrainMidY.wrapping_mul(128 as libc::c_int as
                                     libc::c_uint).wrapping_sub((each.z -
                                                                     player.p.z)
                                                                    as
                                                                    libc::c_uint)
            as int32;
    vec.y = each.y;
    rx_0 = player.p.x & 128 as libc::c_int - 1 as libc::c_int;
    rz_0 = player.p.z & 128 as libc::c_int - 1 as libc::c_int;
    pts[0 as libc::c_int as usize].sx = vec.x + rx_0;
    pts[0 as libc::c_int as usize].sy = vec.y;
    pts[0 as libc::c_int as usize].sz = vec.z - rz_0;
    pointIndex =
        (rand() % ((*(*psStructure).sDisplay.imd).npoints - 1 as libc::c_int))
            as UDWORD;
    point =
        &mut *(*(*psStructure).sDisplay.imd).points.offset(pointIndex as
                                                               isize) as
            *mut iVector;
    each.x = (*psStructure).x as libc::c_int + (*point).x;
    realY =
        (structHeightScale(psStructure) * (*point).y as libc::c_float) as
            SDWORD;
    each.y = (*psStructure).z as libc::c_int + realY;
    each.z = (*psStructure).y as libc::c_int - (*point).z;
    if rand() % 8 as libc::c_int == 0 as libc::c_int {
        effectSetSize(30 as libc::c_int as UDWORD);
        addEffect(&mut each, EFFECT_EXPLOSION, EXPLOSION_TYPE_SPECIFIED,
                  1 as libc::c_int,
                  getImdFromIndex(MI_PLASMA as libc::c_int as UDWORD),
                  0 as libc::c_int);
    }
    vec.x =
        ((each.x - player.p.x) as
             libc::c_uint).wrapping_sub(terrainMidX.wrapping_mul(128 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint))
            as int32;
    vec.z =
        terrainMidY.wrapping_mul(128 as libc::c_int as
                                     libc::c_uint).wrapping_sub((each.z -
                                                                     player.p.z)
                                                                    as
                                                                    libc::c_uint)
            as int32;
    vec.y = each.y;
    rx_0 = player.p.x & 128 as libc::c_int - 1 as libc::c_int;
    rz_0 = player.p.z & 128 as libc::c_int - 1 as libc::c_int;
    pts[1 as libc::c_int as usize].sx = vec.x + rx_0;
    pts[1 as libc::c_int as usize].sy = vec.y;
    pts[1 as libc::c_int as usize].sz = vec.z - rz_0;
    pointIndex =
        (rand() % ((*(*psStructure).sDisplay.imd).npoints - 1 as libc::c_int))
            as UDWORD;
    point =
        &mut *(*(*psStructure).sDisplay.imd).points.offset(pointIndex as
                                                               isize) as
            *mut iVector;
    each.x = (*psStructure).x as libc::c_int + (*point).x;
    realY =
        (structHeightScale(psStructure) * (*point).y as libc::c_float) as
            SDWORD;
    each.y = (*psStructure).z as libc::c_int + realY;
    each.z = (*psStructure).y as libc::c_int - (*point).z;
    vec.x =
        ((each.x - player.p.x) as
             libc::c_uint).wrapping_sub(terrainMidX.wrapping_mul(128 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint))
            as int32;
    vec.z =
        terrainMidY.wrapping_mul(128 as libc::c_int as
                                     libc::c_uint).wrapping_sub((each.z -
                                                                     player.p.z)
                                                                    as
                                                                    libc::c_uint)
            as int32;
    vec.y = each.y;
    rx_0 = player.p.x & 128 as libc::c_int - 1 as libc::c_int;
    rz_0 = player.p.z & 128 as libc::c_int - 1 as libc::c_int;
    pts[2 as libc::c_int as usize].sx = vec.x + rx_0;
    pts[2 as libc::c_int as usize].sy = vec.y;
    pts[2 as libc::c_int as usize].sz = vec.z - rz_0;
    // set the colour
    colour = 0xff as libc::c_int as UDWORD;
    colour =
        lightDoFogAndIllumination(colour as UBYTE,
                                  getCentreX() - (*psDroid).x as libc::c_int,
                                  getCentreZ() - (*psDroid).y as libc::c_int,
                                  &mut specular);
    colour &= 0xff as libc::c_int as libc::c_uint;
    if (*psDroid).action == DACTION_DEMOLISH as libc::c_int ||
           (*psDroid).action == DACTION_CLEARWRECK as libc::c_int {
        colour <<= 16 as libc::c_int
        //red
    }
    pts[0 as libc::c_int as usize].light.argb = 0xff000000 as libc::c_uint;
    pts[1 as libc::c_int as usize].light.argb = 0xff000000 as libc::c_uint;
    pts[2 as libc::c_int as usize].light.argb = 0xff000000 as libc::c_uint;
    pts[0 as libc::c_int as usize].tu = 0 as libc::c_int as UWORD;
    pts[0 as libc::c_int as usize].tv = 0 as libc::c_int as UWORD;
    pts[0 as libc::c_int as usize].specular.argb = colour;
    pts[1 as libc::c_int as usize].tu = 0 as libc::c_int as UWORD;
    pts[1 as libc::c_int as usize].tv = 0 as libc::c_int as UWORD;
    pts[1 as libc::c_int as usize].specular.argb = 0 as libc::c_int as UDWORD;
    pts[2 as libc::c_int as usize].tu = 0 as libc::c_int as UWORD;
    pts[2 as libc::c_int as usize].tv = 0 as libc::c_int as UWORD;
    pts[2 as libc::c_int as usize].specular.argb = 0 as libc::c_int as UDWORD;
    pie_TransColouredTriangle(&mut pts as *mut [PIEVERTEX; 3] as
                                  *mut PIEVERTEX, colour, trans);
}
