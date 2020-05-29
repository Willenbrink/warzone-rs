use ::libc;
extern "C" {
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn glEnable(cap: GLenum);
    #[no_mangle]
    fn glDisable(cap: GLenum);
    #[no_mangle]
    fn glFlush();
    #[no_mangle]
    fn glDepthMask(flag: GLboolean);
    #[no_mangle]
    fn glPushMatrix();
    #[no_mangle]
    fn glPopMatrix();
    #[no_mangle]
    fn glLoadIdentity();
    #[no_mangle]
    fn glBegin(mode: GLenum);
    #[no_mangle]
    fn glEnd();
    #[no_mangle]
    fn glVertex2f(x: GLfloat, y: GLfloat);
    #[no_mangle]
    fn glColor3f(red: GLfloat, green: GLfloat, blue: GLfloat);
    #[no_mangle]
    fn glTexCoord2f(s: GLfloat, t: GLfloat);
    #[no_mangle]
    fn glTexEnvf(target: GLenum, pname: GLenum, param: GLfloat);
    #[no_mangle]
    fn glTexParameteri(target: GLenum, pname: GLenum, param: GLint);
    #[no_mangle]
    fn glTexImage2D(target: GLenum, level: GLint, internalFormat: GLint,
                    width: GLsizei, height: GLsizei, border: GLint,
                    format: GLenum, type_0: GLenum,
                    pixels: *const libc::c_void);
    #[no_mangle]
    fn glGenTextures(n: GLsizei, textures: *mut GLuint);
    #[no_mangle]
    fn glBindTexture(target: GLenum, texture: GLuint);
    #[no_mangle]
    fn PHYSFS_exists(fname: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn div(__numer: libc::c_int, __denom: libc::c_int) -> div_t;
    #[no_mangle]
    fn memFreeRelease(pMemToFree: *mut libc::c_void);
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn malloc(_: libc::c_uint) -> *mut libc::c_void;
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
    fn keyScanToString(code: KEY_CODE, ascii: *mut STRING,
                       maxStringSize: UDWORD);
    /* This returns true if the key went from being up to being down this frame */
    #[no_mangle]
    fn keyPressed(code: KEY_CODE) -> BOOL;
    /* This returns true if the key went from being down to being up this frame */
    #[no_mangle]
    fn keyReleased(code: KEY_CODE) -> BOOL;
    /* These two functions return the current position of the mouse */
    #[no_mangle]
    fn mouseX() -> SDWORD;
    #[no_mangle]
    fn mouseY() -> SDWORD;
    /* This returns true if the mouse key is currently depressed */
    #[no_mangle]
    fn mouseDown(code: MOUSE_KEY_CODE) -> BOOL;
    /* This returns true if the mouse key went from being down to being up this frame */
    #[no_mangle]
    fn mouseReleased(code: MOUSE_KEY_CODE) -> BOOL;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_uint;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_uint)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_uint)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_uint)
     -> *mut libc::c_void;
    // release the data for a particular block ID
    #[no_mangle]
    fn resReleaseBlockData(blockID: SDWORD);
    /* Get the string from an ID number */
    #[no_mangle]
    fn strresGetString(psRes: *mut STR_RES, id: UDWORD) -> *mut STRING;
    #[no_mangle]
    fn loadFileToBuffer(pFileName: *mut libc::c_char,
                        pFileBuffer: *mut libc::c_char, bufferSize: UDWORD,
                        pSize: *mut UDWORD) -> BOOL;
    /* The Current screen size and bit depth */
    #[no_mangle]
    static mut screenWidth: UDWORD;
    #[no_mangle]
    static mut screenHeight: UDWORD;
    #[no_mangle]
    fn image_init(image: *mut pie_image) -> BOOL;
    #[no_mangle]
    fn image_delete(image: *mut pie_image) -> BOOL;
    #[no_mangle]
    fn image_load_from_jpg(image: *mut pie_image,
                           filename: *const libc::c_char) -> BOOL;
    #[no_mangle]
    fn screen_Upload(newBackDropBmp: *mut UWORD);
    /* Create an empty widget screen */
    #[no_mangle]
    fn widgCreateScreen(ppsScreen: *mut *mut W_SCREEN) -> BOOL;
    /* Release a screen and all its associated data */
    #[no_mangle]
    fn widgReleaseScreen(psScreen: *mut W_SCREEN);
    /* Set the tool tip font for a screen */
    #[no_mangle]
    fn widgSetTipFont(psScreen: *mut W_SCREEN, FontID: libc::c_int);
    /* Add a form to the widget screen */
    #[no_mangle]
    fn widgAddForm(psScreen: *mut W_SCREEN, psInit: *mut W_FORMINIT) -> BOOL;
    /* Add a label to the widget screen */
    #[no_mangle]
    fn widgAddLabel(psScreen: *mut W_SCREEN, psInit: *mut W_LABINIT) -> BOOL;
    /* Add a button to a form */
    #[no_mangle]
    fn widgAddButton(psScreen: *mut W_SCREEN, psInit: *mut W_BUTINIT) -> BOOL;
    /* Add an edit box to a form */
    #[no_mangle]
    fn widgAddEditBox(psScreen: *mut W_SCREEN, psInit: *mut W_EDBINIT)
     -> BOOL;
    /* Add a bar graph to a form */
    #[no_mangle]
    fn widgAddBarGraph(psScreen: *mut W_SCREEN, psInit: *mut W_BARINIT)
     -> BOOL;
    /* Delete a widget from the screen */
    #[no_mangle]
    fn widgDelete(psScreen: *mut W_SCREEN, id: UDWORD);
    /* Return a pointer to a buffer containing the current string of a widget if any.
 * This will always return a valid string pointer.
 * NOTE: The string must be copied out of the buffer
 */
    #[no_mangle]
    fn widgGetString(psScreen: *mut W_SCREEN, id: UDWORD) -> *mut STRING;
    /* Set the text in a widget */
    #[no_mangle]
    fn widgSetString(psScreen: *mut W_SCREEN, id: UDWORD, pText: *mut STRING);
    /* Get the current position of a slider bar */
    #[no_mangle]
    fn widgGetSliderPos(psScreen: *mut W_SCREEN, id: UDWORD) -> UDWORD;
    /* Set the current position of a slider bar */
    #[no_mangle]
    fn widgSetSliderPos(psScreen: *mut W_SCREEN, id: UDWORD, pos: UWORD);
    /* Return the ID of the widget the mouse was over this frame */
    #[no_mangle]
    fn widgGetMouseOver(psScreen: *mut W_SCREEN) -> UDWORD;
    /* Get widget structure */
    /* Find a widget in a screen from its ID number */
    #[no_mangle]
    fn widgGetFromID(psScreen: *mut W_SCREEN, id: UDWORD) -> *mut WIDGET;
    /* Set a button or clickable form's state */
    #[no_mangle]
    fn widgSetButtonState(psScreen: *mut W_SCREEN, id: UDWORD, state: UDWORD);
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
    #[no_mangle]
    fn sliderEnableDrag(Enable: BOOL);
    //*************************************************************************
    //*************************************************************************
    #[no_mangle]
    static mut colours: [uint8; 0];
    #[no_mangle]
    fn pal_GetNearestColour(r: uint8, g: uint8, b: uint8) -> uint8;
    /* **************************************************************************/
/*
 * pieclip.h
 *
 * clipping for all pumpkin image library functions.
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
    /*
 * droid.h
 *
 * Definitions for the droid object.
 *
 */
    // world->screen check - alex
    // percentage of body points remaining at which to repair droid automatically.
    // ditto, but this will repair much sooner..
    /*defines the % to decrease the illumination of a tile when building - gets set 
back when building is destroyed*/
//#define FOUNDATION_ILLUMIN		50
    //storage
    #[no_mangle]
    static mut apsDroidTemplates: [*mut DROID_TEMPLATE; 8];
    //extern BOOL loadDroidPrograms(car *pProgramData, UDWORD bufferSize);
    /*initialise the template build and power points */
    #[no_mangle]
    fn initTemplatePoints();
    /* Calculate the points required to build the droid */
//UDWORD calcDroidBuild(DROID *psDroid);
    /* Calculate the power points required to build/maintain the droid */
    #[no_mangle]
    fn calcTemplatePower(psTemplate: *mut DROID_TEMPLATE) -> UDWORD;
    /*return the name to display for the interface - we don't know if this is 
a string ID or something the user types in*/
    #[no_mangle]
    fn getTemplateName(psTemplate: *mut DROID_TEMPLATE) -> *mut STRING;
    //render states
    #[no_mangle]
    fn pie_SetTexturePage(num: SDWORD);
    #[no_mangle]
    fn pie_SetSwirlyBoxes(val: BOOL);
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
    fn pie_BoxFill(x0: libc::c_int, y0: libc::c_int, x1: libc::c_int,
                   y1: libc::c_int, colour: uint32);
    #[no_mangle]
    fn pie_ImageFileID(ImageFile: *mut IMAGEFILE, ID: UWORD, x: libc::c_int,
                       y: libc::c_int);
    #[no_mangle]
    fn pie_TransBoxFill(x0: SDWORD, y0: SDWORD, x1: SDWORD, y1: SDWORD);
    #[no_mangle]
    fn pie_UniTransBoxFill(x0: SDWORD, y0: SDWORD, x1: SDWORD, y1: SDWORD,
                           rgb: UDWORD, transparency: UDWORD);
    #[no_mangle]
    static mut psRendSurface: *mut iSurface;
    #[no_mangle]
    fn pie_DrawText(string: *mut STRING, x: UDWORD, y: UDWORD);
    #[no_mangle]
    fn iV_SetTextColour(Index: SWORD);
    #[no_mangle]
    fn pie_LoadBackDrop(screenType: SCREENTYPE, b3DFX: BOOL);
    #[no_mangle]
    fn iV_GetImageWidth(ImageFile: *mut IMAGEFILE, ID: UWORD) -> UWORD;
    #[no_mangle]
    fn iV_GetImageHeight(ImageFile: *mut IMAGEFILE, ID: UWORD) -> UWORD;
    #[no_mangle]
    fn iV_SetFont(FontID: libc::c_int);
    #[no_mangle]
    fn iV_GetTextWidth(String: *mut STRING) -> libc::c_int;
    // Shutdown the gateway system
    #[no_mangle]
    fn gwShutDown();
    /*
 * GTime.h
 *
 * Interface to the game clock.
 *
 */
    /* The number of ticks per second for the game clock */
    // The maximum time for one frame (stops the clock running away when debugging)
// changed to /6 by ajl. if this needs to go back to ticks/10 then tell me. 
    /* The current time in the game world */
    #[no_mangle]
    static mut gameTime: UDWORD;
    /* The current time in the game world */
    #[no_mangle]
    static mut gameTime2: UDWORD;
    /* The string resource object */
    #[no_mangle]
    static mut psStringRes: *mut STR_RES;
    /*
 * config.h
 * load and save favourites to the registry.
 */
    #[no_mangle]
    fn loadConfig(bResourceAvailable: BOOL) -> BOOL;
    #[no_mangle]
    fn saveConfig() -> BOOL;
    /* Respond to a mouse click */
    #[no_mangle]
    fn editBoxClicked(psWidget: *mut W_EDITBOX, psContext: *mut W_CONTEXT);
    #[no_mangle]
    static mut IntImages: *mut IMAGEFILE;
    // A few useful defined tabs.
    #[no_mangle]
    static mut StandardTab: TABDEF;
    // Begin a rendering lock.
    #[no_mangle]
    fn DrawBegin();
    // End a rendering lock.
    #[no_mangle]
    fn DrawEnd();
    #[no_mangle]
    static mut ObjectBuffers: [RENDERED_BUTTON; 40];
    #[no_mangle]
    fn GetObjectBuffer() -> SDWORD;
    #[no_mangle]
    fn ClearObjectBuffers();
    #[no_mangle]
    fn ClearStatBuffers();
    #[no_mangle]
    fn intDisplayPowerBar(psWidget: *mut _widget, xOffset: UDWORD,
                          yOffset: UDWORD, pColours: *mut UDWORD);
    #[no_mangle]
    fn intDisplayObjectForm(psWidget: *mut _widget, xOffset: UDWORD,
                            yOffset: UDWORD, pColours: *mut UDWORD);
    #[no_mangle]
    fn intOpenPlainForm(psWidget: *mut _widget, xOffset: UDWORD,
                        yOffset: UDWORD, pColours: *mut UDWORD);
    #[no_mangle]
    fn intDisplayPlainForm(psWidget: *mut _widget, xOffset: UDWORD,
                           yOffset: UDWORD, pColours: *mut UDWORD);
    #[no_mangle]
    fn intDisplayTab(psWidget: *mut _widget, TabType: UDWORD,
                     Position: UDWORD, Number: UDWORD, Selected: BOOL,
                     Hilight: BOOL, x: UDWORD, y: UDWORD, Width: UDWORD,
                     Height: UDWORD);
    #[no_mangle]
    fn intDisplayEditBox(psWidget: *mut _widget, xOffset: UDWORD,
                         yOffset: UDWORD, pColours: *mut UDWORD);
    /* Set the keyboard focus for the screen */
    /* Clear the keyboard focus */
    #[no_mangle]
    fn screenClearFocus(psScreen: *mut W_SCREEN);
    /* Add the droid template buttons to a form */
    #[no_mangle]
    fn intAddTemplateButtons(formID: UDWORD, formWidth: UDWORD,
                             formHeight: UDWORD, butWidth: UDWORD,
                             butHeight: UDWORD, gap: UDWORD,
                             psSelected: *mut DROID_TEMPLATE) -> BOOL;
    /* The widget screen */
    #[no_mangle]
    static mut psWScreen: *mut W_SCREEN;
    /* the widget font */
    #[no_mangle]
    static mut WFont: libc::c_int;
    /* Which is the currently selected player */
    #[no_mangle]
    static mut selectedPlayer: UDWORD;
    #[no_mangle]
    static mut apsTemplateList: *mut *mut DROID_TEMPLATE;
    #[no_mangle]
    static mut DisplayBuffer: *mut libc::c_char;
    #[no_mangle]
    static mut displayBufferSize: UDWORD;
    #[no_mangle]
    fn numForms(total: UDWORD, perForm: UDWORD) -> UWORD;
    // update shadow...
    #[no_mangle]
    fn intSetShadowPower(quantity: UDWORD);
    #[no_mangle]
    fn StartCursorSnap(SnapBuffer: *mut CURSORSNAP);
    #[no_mangle]
    fn AddCursorSnap(SnapBuffer: *mut CURSORSNAP, PosX: SWORD, PosY: SWORD,
                     FormID: UDWORD, ID: UDWORD, Bias: *mut SNAPBIAS);
    // used in multiplayer to force power levels.
    #[no_mangle]
    fn setPower(player: UDWORD, avail: UDWORD);
    /* **************************************************************************/
/*
 *	Global Variables
 */
/* **************************************************************************/
    #[no_mangle]
    static mut bLoadSaveUp: BOOL;
    #[no_mangle]
    static mut sRequestResult: [STRING; 255];
    #[no_mangle]
    static mut bRequestLoad: BOOL;
    /* **************************************************************************/
/*
 *	Global ProtoTypes
 */
/* **************************************************************************/
    #[no_mangle]
    fn drawBlueBox(x: UDWORD, y: UDWORD, w: UDWORD, h: UDWORD);
    #[no_mangle]
    fn addLoadSave(mode: LOADSAVE_MODE, defaultdir: *mut CHAR,
                   extension: *mut CHAR, title: *mut CHAR) -> BOOL;
    #[no_mangle]
    fn runLoadSave(bResetMissionWidgets: BOOL) -> BOOL;
    #[no_mangle]
    fn displayLoadSave() -> BOOL;
    #[no_mangle]
    fn removeWildcards(pStr: *mut libc::c_char);
    #[no_mangle]
    fn pie_SetGeometricOffset(x: libc::c_int, y: libc::c_int);
    // = {0,1,2,3,4,5,6,7}
    #[no_mangle]
    fn initPlayerColours();
    #[no_mangle]
    fn setPlayerColour(player: UDWORD, col: UDWORD) -> BOOL;
    #[no_mangle]
    fn getPlayerColour(pl: UDWORD) -> UBYTE;
    #[no_mangle]
    fn displayComponentButtonTemplate(psTemplate: *mut DROID_TEMPLATE,
                                      Rotation: *mut iVector,
                                      Position: *mut iVector, RotXYZ: BOOL,
                                      scale: SDWORD);
    /* The maximum map size */
    /* The size and contents of the map */
    #[no_mangle]
    static mut mapWidth: UDWORD;
    #[no_mangle]
    static mut mapHeight: UDWORD;
    #[no_mangle]
    static mut psMapTiles: *mut MAPTILE;
    /* The mask to get internal tile coords from a full coordinate */
    /* Shutdown the map module */
    #[no_mangle]
    fn mapShutdown() -> BOOL;
    /* Load the map data */
    #[no_mangle]
    fn mapLoad(pFileData: *mut libc::c_char, fileSize: UDWORD) -> BOOL;
    #[no_mangle]
    fn updateConsoleMessages();
    #[no_mangle]
    fn initConsoleMessages();
    #[no_mangle]
    fn removeTopConsoleMessage();
    #[no_mangle]
    fn displayConsoleMessages();
    #[no_mangle]
    fn flushConsoleMessages();
    #[no_mangle]
    fn setConsoleBackdropStatus(state: BOOL);
    #[no_mangle]
    fn enableConsoleDisplay(state: BOOL);
    #[no_mangle]
    fn setDefaultConsoleJust(defJ: CONSOLE_TEXT_JUSTIFICATION);
    #[no_mangle]
    fn setConsoleSizePos(x: UDWORD, y: UDWORD, width: UDWORD);
    #[no_mangle]
    fn setConsolePermanence(state: BOOL, bClearOld: BOOL);
    #[no_mangle]
    fn getNumberConsoleMessages() -> UDWORD;
    #[no_mangle]
    fn setConsoleLineInfo(vis: UDWORD);
    #[no_mangle]
    fn getConsoleLineInfo() -> UDWORD;
    #[no_mangle]
    fn setRevealStatus(val: BOOL);
    // This dos'nt compile on the PSX.
//typedef enum _titlemode tMode;	// define the type
    #[no_mangle]
    static mut titleMode: tMode;
    // the global case
    //#define DEFAULT_LEVEL	"CAM_2A"
    #[no_mangle]
    static mut pLevelName: [libc::c_char; 257];
    #[no_mangle]
    static mut bForceEditorLoaded: BOOL;
    #[no_mangle]
    fn changeTitleMode(mode: tMode);
    #[no_mangle]
    fn processFrontendSnap(bHideCursor: BOOL);
    #[no_mangle]
    fn addTopForm();
    #[no_mangle]
    fn addBottomForm();
    #[no_mangle]
    fn addBackdrop();
    #[no_mangle]
    fn addTextButton(id: UDWORD, PosX: UDWORD, PosY: UDWORD, txt: *mut STRING,
                     bAlignLeft: BOOL, bGrey: BOOL);
    #[no_mangle]
    fn addSideText(id: UDWORD, PosX: UDWORD, PosY: UDWORD, txt: *mut STRING);
    #[no_mangle]
    fn addFESlider(id: UDWORD, parent: UDWORD, x: UDWORD, y: UDWORD,
                   stops: UDWORD, pos: UDWORD, attachID: UDWORD);
    // multiplayer cheat code.
    #[no_mangle]
    fn resetCheatHash();
    // reset the event system
    #[no_mangle]
    fn eventReset();
    #[no_mangle]
    fn getQwertyKey() -> KEY_CODE;
    // ////////////////////////////////////////////////////////////////////////
// variables
    #[no_mangle]
    static mut NetPlay: NETPLAY;
    // send to player, possibly guaranteed
    #[no_mangle]
    fn NETbcast(msg: *mut NETMSG, guarantee: BOOL) -> BOOL;
    // broadcast to everyone, possibly guaranteed
    #[no_mangle]
    fn NETrecv(msg: *mut NETMSG) -> BOOL;
    // recv file chunk
    #[no_mangle]
    fn NETclose() -> HRESULT;
    // return one of the four flags(dword) about the game.
    #[no_mangle]
    fn NETgetGameFlagsUnjoined(gameid: UDWORD, flag: UDWORD) -> DWORD;
    // set game flag(1-4) to value.		
    #[no_mangle]
    fn NEThaltJoining() -> BOOL;
    // stop new players joining this game
    #[no_mangle]
    fn NETfindGame(asynchronously: BOOL) -> BOOL;
    //from netusers.c
    #[no_mangle]
    fn NETuseNetwork(val: BOOL) -> BOOL;
    // TURN on/off networking.
    #[no_mangle]
    fn NETplayerInfo() -> UDWORD;
    // count players in this game.
    #[no_mangle]
    fn NETchangePlayerName(dpid: UDWORD, newName: *mut libc::c_char) -> BOOL;
    // check for spectator status.
    // from net audio.
    #[no_mangle]
    fn NETprocessAudioCapture() -> BOOL;
    //capture
    #[no_mangle]
    fn NETstopAudioCapture() -> BOOL;
    #[no_mangle]
    fn NETstartAudioCapture() -> BOOL;
    #[no_mangle]
    fn isHumanPlayer(player: UDWORD) -> BOOL;
    #[no_mangle]
    fn sendTextMessage(pStr: *mut libc::c_char, cast: BOOL) -> BOOL;
    #[no_mangle]
    fn sendMap() -> UBYTE;
    #[no_mangle]
    static mut ingame: MULTIPLAYERINGAME;
    #[no_mangle]
    static mut game: MULTIPLAYERGAME;
    #[no_mangle]
    static mut bMultiPlayer: BOOL;
    #[no_mangle]
    static mut player2dpid: [DWORD; 8];
    //extern BOOL SendDroidWaypoint	(UBYTE player, UDWORD	x, UDWORD y);
//extern BOOL SendSingleDroidWaypoint(DROID *psDroid, UDWORD x,UDWORD y);
    // Startup. mulitopt
    // for Init.c
    // for Init.c
    //extern BOOL hostArena			(STRING *sGame, STRING *sPlayer);
//extern BOOL joinArena			(UDWORD gameNumber, STRING *playername);
    // syncing.
    //send/recv  check info
    #[no_mangle]
    fn sendScoreCheck() -> BOOL;
    #[no_mangle]
    fn sendLeavingMsg() -> BOOL;
    #[no_mangle]
    fn hostCampaign(sGame: *mut STRING, sPlayer_0: *mut STRING) -> BOOL;
    #[no_mangle]
    fn joinCampaign(gameNumber_0: UDWORD, playername: *mut STRING) -> BOOL;
    #[no_mangle]
    fn copyTemplateSet(from: UDWORD, to: UDWORD) -> BOOL;
    #[no_mangle]
    fn sendPing() -> BOOL;
    #[no_mangle]
    static mut sForceName: [libc::c_char; 256];
    #[no_mangle]
    fn MultiPlayerLeave(dp: DPID) -> BOOL;
    // A Player has joined the game.
    #[no_mangle]
    fn setupNewPlayer(dpid: DPID, player: UDWORD);
    #[no_mangle]
    static mut Force: FORCE;
    // the selected force.
    //  Force defs.
    #[no_mangle]
    fn removeFromForce(number: UDWORD) -> BOOL;
    // remove a droid from force
    #[no_mangle]
    fn addToForce(templ: *mut DROID_TEMPLATE) -> BOOL;
    // place the force in the game
    #[no_mangle]
    fn saveForce(name: *mut libc::c_char, pfForce: *mut FORCE) -> BOOL;
    #[no_mangle]
    fn loadForce(name: *mut libc::c_char) -> BOOL;
    // to disk 
    #[no_mangle]
    fn loadMultiStats(sPlayerName: *mut STRING, playerStats: *mut PLAYERSTATS)
     -> BOOL;
    // form disk
    #[no_mangle]
    fn getMultiStats(player: UDWORD, bLocal: BOOL) -> PLAYERSTATS;
    // get from net
    #[no_mangle]
    fn setMultiStats(playerDPID: DWORD, plStats: PLAYERSTATS, bLocal: BOOL)
     -> BOOL;
    #[no_mangle]
    fn recvPing(pMsg: *mut NETMSG) -> BOOL;
    #[no_mangle]
    fn recvTextMessage(pMsg: *mut NETMSG) -> BOOL;
    #[no_mangle]
    fn recvAlliance(pMsg: *mut NETMSG, allowAudio: BOOL) -> BOOL;
    //extern BOOL sendWholeStructure		(STRUCTURE *pS, DPID dest);
    #[no_mangle]
    fn recvOptions(pMsg: *mut NETMSG);
    #[no_mangle]
    fn sendOptions(dest: DPID, play: UDWORD);
    #[no_mangle]
    fn recvAudioMsg(pMsg: *mut NETMSG) -> BOOL;
    #[no_mangle]
    fn recvMapFileData(pMsg: *mut NETMSG) -> BOOL;
    #[no_mangle]
    fn recvMapFileRequested(pMsg: *mut NETMSG) -> BOOL;
    /*
 * MultiMenu.h
 *
 * Definition for in game,multiplayer, interface.
 */
// 
    // requester
    #[no_mangle]
    fn addMultiRequest(ToFind: *mut STRING, id: UDWORD, mapCam: UBYTE,
                       numPlayers: UBYTE);
    #[no_mangle]
    static mut multiRequestUp: BOOL;
    #[no_mangle]
    static mut psRScreen: *mut W_SCREEN;
    // requester stuff.
    #[no_mangle]
    fn runMultiRequester(id: UDWORD, contextmode: *mut UDWORD,
                         chosen: *mut STRING, chosenValue: *mut UDWORD)
     -> BOOL;
    #[no_mangle]
    fn rebuildSearchPath(mode: searchPathMode, force: BOOL) -> BOOL;
    // find the level dataset
    #[no_mangle]
    fn levFindDataSet(pName: *mut STRING, ppsDataSet: *mut *mut LEVEL_DATASET)
     -> BOOL;
    /*
 * MultiInt.c
 *
 * Alex Lee, 98. Pumpkin Studios, Bath.
 * Functions to display and handle the multiplayer interface screens
 * Arena and Campaign styles, along with connection and game options.
 */
    // get rid of a couple of warnings.
    // FIXME Direct iVis implementation include!
    /* Includes direct access to render library */
    // FIXME Direct iVis implementation include!
    // FIXME Direct iVis implementation include!
    //#include "editbox.h"
    //#include "texture.h"
    // ////////////////////////////////////////////////////////////////////////////
// vars
    #[no_mangle]
    static mut MultiForcesPath: [libc::c_char; 255];
    #[no_mangle]
    static mut MultiCustomMapsPath: [libc::c_char; 255];
    #[no_mangle]
    static mut MultiPlayersPath: [libc::c_char; 255];
    #[no_mangle]
    static mut FrontImages: *mut IMAGEFILE;
    #[no_mangle]
    static mut InterfaceSnap: CURSORSNAP;
    #[no_mangle]
    static mut bSendingMap: BOOL;
    #[no_mangle]
    fn intDisplayTemplateButton(psWidget: *mut _widget, xOffset: UDWORD,
                                yOffset: UDWORD, pColours: *mut UDWORD);
    #[no_mangle]
    fn plotStructurePreview16(backDropSprite: *mut libc::c_uchar,
                              scale: UBYTE, offX: UDWORD, offY: UDWORD)
     -> BOOL;
    #[no_mangle]
    fn NETsetupTCPIP(addr: *mut LPVOID, machine: *mut libc::c_char) -> BOOL;
    #[no_mangle]
    static mut FEFont: libc::c_int;
}
pub type GLenum = libc::c_uint;
pub type GLboolean = libc::c_uchar;
pub type GLvoid = ();
pub type GLint = libc::c_int;
pub type GLuint = libc::c_uint;
pub type GLsizei = libc::c_int;
pub type GLfloat = libc::c_float;
pub type UBYTE = libc::c_uchar;
pub type STRING = libc::c_char;
pub type UWORD = libc::c_ushort;
pub type SWORD = libc::c_short;
pub type UDWORD = libc::c_uint;
pub type SDWORD = libc::c_int;
pub type HRESULT = libc::c_int;
pub type BOOL = libc::c_int;
pub type CHAR = libc::c_char;
pub type SHORT = libc::c_short;
pub type DWORD = libc::c_int;
pub type LPVOID = *mut libc::c_void;
pub type DPID = libc::c_int;
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
pub struct div_t {
    pub quot: libc::c_int,
    pub rem: libc::c_int,
}
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
pub type _key_code = libc::c_uint;
pub const KEY_KPENTER: _key_code = 271;
pub const KEY_DELETE: _key_code = 127;
pub const KEY_INSERT: _key_code = 277;
pub const KEY_PAGEDOWN: _key_code = 281;
pub const KEY_DOWNARROW: _key_code = 274;
pub const KEY_END: _key_code = 279;
pub const KEY_RIGHTARROW: _key_code = 275;
pub const KEY_LEFTARROW: _key_code = 276;
pub const KEY_PAGEUP: _key_code = 280;
pub const KEY_UPARROW: _key_code = 273;
pub const KEY_HOME: _key_code = 278;
pub const KEY_RALT: _key_code = 307;
pub const KEY_KP_BACKSLASH: _key_code = 267;
pub const KEY_RCTRL: _key_code = 305;
pub const KEY_F12: _key_code = 293;
pub const KEY_F11: _key_code = 292;
pub const KEY_KP_FULLSTOP: _key_code = 266;
pub const KEY_KP_0: _key_code = 256;
pub const KEY_KP_3: _key_code = 259;
pub const KEY_KP_2: _key_code = 258;
pub const KEY_KP_1: _key_code = 257;
pub const KEY_KP_PLUS: _key_code = 270;
pub const KEY_KP_6: _key_code = 262;
pub const KEY_KP_5: _key_code = 261;
pub const KEY_KP_4: _key_code = 260;
pub const KEY_KP_MINUS: _key_code = 269;
pub const KEY_KP_9: _key_code = 265;
pub const KEY_KP_8: _key_code = 264;
pub const KEY_KP_7: _key_code = 263;
pub const KEY_SCROLLLOCK: _key_code = 302;
pub const KEY_NUMLOCK: _key_code = 300;
pub const KEY_F10: _key_code = 291;
pub const KEY_F9: _key_code = 290;
pub const KEY_F8: _key_code = 289;
pub const KEY_F7: _key_code = 288;
pub const KEY_F6: _key_code = 287;
pub const KEY_F5: _key_code = 286;
pub const KEY_F4: _key_code = 285;
pub const KEY_F3: _key_code = 284;
pub const KEY_F2: _key_code = 283;
pub const KEY_F1: _key_code = 282;
pub const KEY_CAPSLOCK: _key_code = 301;
pub const KEY_SPACE: _key_code = 32;
pub const KEY_LALT: _key_code = 308;
pub const KEY_KP_STAR: _key_code = 268;
pub const KEY_RSHIFT: _key_code = 303;
pub const KEY_FORWARDSLASH: _key_code = 47;
pub const KEY_FULLSTOP: _key_code = 46;
pub const KEY_COMMA: _key_code = 44;
pub const KEY_M: _key_code = 109;
pub const KEY_N: _key_code = 110;
pub const KEY_B: _key_code = 98;
pub const KEY_V: _key_code = 118;
pub const KEY_C: _key_code = 99;
pub const KEY_X: _key_code = 120;
pub const KEY_Z: _key_code = 122;
pub const KEY_BACKSLASH: _key_code = 92;
pub const KEY_LSHIFT: _key_code = 304;
pub const KEY_BACKQUOTE: _key_code = 96;
pub const KEY_QUOTE: _key_code = 39;
pub const KEY_SEMICOLON: _key_code = 59;
pub const KEY_L: _key_code = 108;
pub const KEY_K: _key_code = 107;
pub const KEY_J: _key_code = 106;
pub const KEY_H: _key_code = 104;
pub const KEY_G: _key_code = 103;
pub const KEY_F: _key_code = 102;
pub const KEY_D: _key_code = 100;
pub const KEY_S: _key_code = 115;
pub const KEY_A: _key_code = 97;
pub const KEY_LCTRL: _key_code = 306;
pub const KEY_RETURN: _key_code = 13;
pub const KEY_RBRACE: _key_code = 93;
pub const KEY_LBRACE: _key_code = 91;
pub const KEY_P: _key_code = 112;
pub const KEY_O: _key_code = 111;
pub const KEY_I: _key_code = 105;
pub const KEY_U: _key_code = 117;
pub const KEY_Y: _key_code = 121;
pub const KEY_T: _key_code = 116;
pub const KEY_R: _key_code = 114;
pub const KEY_E: _key_code = 101;
pub const KEY_W: _key_code = 119;
pub const KEY_Q: _key_code = 113;
pub const KEY_TAB: _key_code = 9;
pub const KEY_BACKSPACE: _key_code = 8;
pub const KEY_EQUALS: _key_code = 61;
pub const KEY_MINUS: _key_code = 45;
pub const KEY_0: _key_code = 48;
pub const KEY_9: _key_code = 57;
pub const KEY_8: _key_code = 56;
pub const KEY_7: _key_code = 55;
pub const KEY_6: _key_code = 54;
pub const KEY_5: _key_code = 53;
pub const KEY_4: _key_code = 52;
pub const KEY_3: _key_code = 51;
pub const KEY_2: _key_code = 50;
pub const KEY_1: _key_code = 49;
pub const KEY_ESC: _key_code = 27;
pub type KEY_CODE = _key_code;
pub type _mouse_key_code = libc::c_uint;
pub const MOUSE_WDN: _mouse_key_code = 5;
pub const MOUSE_WUP: _mouse_key_code = 4;
pub const MOUSE_RMB: _mouse_key_code = 3;
pub const MOUSE_MMB: _mouse_key_code = 2;
pub const MOUSE_LMB: _mouse_key_code = 1;
pub type MOUSE_KEY_CODE = _mouse_key_code;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _free_object {
    pub psNext: *mut _free_object,
}
pub type FREE_OBJECT = _free_object;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _heap_extension {
    pub pMemory: *mut UBYTE,
    pub psNext: *mut _heap_extension,
}
pub type HEAP_EXTENSION = _heap_extension;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _obj_heap {
    pub objSize: UDWORD,
    pub initAlloc: UDWORD,
    pub extAlloc: UDWORD,
    pub psBlkHeap: *mut _block_heap,
    pub psFree: *mut FREE_OBJECT,
    pub pMemory: *mut UBYTE,
    pub psExt: *mut HEAP_EXTENSION,
}
/*
 * Heap.h
 *
 * Interface to the heap memory routines.
 *
 * Overhead of using the heap is :
 *			24 bytes for the initial block
 *           4 bytes for the extension blocks
 *
 */
/* Include Mem.h to get the DEBUG_MALLOC #define - this controls whether
  * normal or debugging memory management is used.
  */
/* structure used to store the list of free heap objects */
/* Header for each object in DEBUG_MALLOC mode */
/* structure used to store the extra space allocated for the heap */
pub type OBJ_HEAP = _obj_heap;
pub type TREAP_CMP
    =
    Option<unsafe extern "C" fn(_: UDWORD, _: UDWORD) -> SDWORD>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _treap_node {
    pub key: UDWORD,
    pub priority: UDWORD,
    pub pObj: *mut libc::c_void,
    pub psLeft: *mut _treap_node,
    pub psRight: *mut _treap_node,
}
pub type TREAP_NODE = _treap_node;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _treap {
    pub cmp: TREAP_CMP,
    pub psNodes: *mut OBJ_HEAP,
    pub psRoot: *mut TREAP_NODE,
}
pub type TREAP = _treap;
// The size of the objects being stored on the heap
// The initial number of objects allocated
// The number of objects to allocate after the initial
// allocation is used up
// which block heap (if any) this object heap was allocated from
// The currently free objects
// The main memory heap
// Extension memory for the heap
/*
 * StrRes.h
 *
 * String resource interface functions
 *
 */
/* A string block */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _str_block {
    pub apStrings: *mut *mut STRING,
    pub idStart: UDWORD,
    pub idEnd: UDWORD,
    pub psNext: *mut _str_block,
}
pub type STR_BLOCK = _str_block;
/* A String Resource */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _str_res {
    pub psIDTreap: *mut TREAP,
    pub psStrings: *mut STR_BLOCK,
    pub init: UDWORD,
    pub ext: UDWORD,
    pub nextID: UDWORD,
}
pub type STR_RES = _str_res;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pie_image {
    pub width: libc::c_uint,
    pub height: libc::c_uint,
    pub channels: libc::c_uint,
    pub data: *mut libc::c_uchar,
}
// The next free ID
/*
 * WidgBase.h
 *
 * Definitions for the basic widget types.
 */
/* The different base types of widget */
pub type _widget_type = libc::c_uint;
pub const WIDG_SLIDER: _widget_type = 5;
pub const WIDG_BARGRAPH: _widget_type = 4;
pub const WIDG_EDITBOX: _widget_type = 3;
pub const WIDG_BUTTON: _widget_type = 2;
pub const WIDG_LABEL: _widget_type = 1;
pub const WIDG_FORM: _widget_type = 0;
pub type WIDGET_TYPE = _widget_type;
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
/* The optional user callback function */
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
pub type WIDGET = _widget;
/* The common widget data */
/* Button colours */
// Colour for button text
// Colour for button background
// Colour for button border
// 2nd border colour
// Hilite colour
/* The display function prototype */
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
pub type WIDGET_AUDIOCALLBACK
    =
    Option<unsafe extern "C" fn(_: libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_colourdef {
    pub red: UBYTE,
    pub green: UBYTE,
    pub blue: UBYTE,
    pub alpha: UBYTE,
}
// The root form of the screen
// The widget that has keyboard focus
//	PROP_FONT	*psTipFont;			// The font for tool tips
// ID of the IVIS font to use for tool tips.
// Bar graph with a trough showing empty percentage
// Double bar graph, one on top of other
/* ********** Slider styles ***************/
// Plain slider
/* **********************************************************************************/
/* Generic widget colour */
pub type W_COLOURDEF = _w_colourdef;
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
pub type FONT_DISPLAY
    =
    Option<unsafe extern "C" fn(_: UDWORD, _: UDWORD, _: *mut STRING) -> ()>;
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
pub struct _w_labinit {
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
/* Label initialisation structure */
pub type W_LABINIT = _w_labinit;
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
// label text
// Tool tip for the label.
//	PROP_FONT	*psFont;		// label font
// ID of the IVIS font to use for this widget.
/* Button initialisation structure */
pub type W_BUTINIT = _w_butinit;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_edbinit {
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
    pub FontID: libc::c_int,
    pub pBoxDisplay: WIDGET_DISPLAY,
    pub pFontDisplay: FONT_DISPLAY,
}
/* The basic init entries */
// button text
// Tool tip text
//	PROP_FONT	*psFont;		// button font
// ID of the IVIS font to use for this widget.
/* Edit box initialisation structure */
pub type W_EDBINIT = _w_edbinit;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_barinit {
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
    pub orientation: UWORD,
    pub size: UWORD,
    pub minorSize: UWORD,
    pub iRange: UWORD,
    pub sCol: W_COLOURDEF,
    pub sMinorCol: W_COLOURDEF,
    pub pTip: *mut STRING,
}
/* The basic init entries */
// initial contents of the edit box
//	PROP_FONT	*psFont;		// edit box font
// ID of the IVIS font to use for this widget.
// Optional callback to display the form.
// Optional callback to display a string.
// Bar graph fills from left to right
// Bar graph fills from right to left
// Bar graph fills from top to bottom
// Bar graph fills from bottom to top
/* Bar Graph initialisation structure */
pub type W_BARINIT = _w_barinit;
pub type int32 = libc::c_int;
pub type uint8 = libc::c_uchar;
pub type uint32 = libc::c_uint;
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
/* The basic init entries */
// Orientation of the bar on the widget
// Initial percentage of the graph that is filled
// Percentage of second bar graph if there is one
// Maximum range
// Bar colour
// Minor bar colour
// Tool tip text
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
// maximum number of characters in a droid name
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _droid_template {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
    pub aName: [STRING; 60],
    pub NameVersion: UBYTE,
    pub asParts: [SDWORD; 8],
    pub buildPoints: UDWORD,
    pub powerPoints: UDWORD,
    pub storeCount: UDWORD,
    pub numWeaps: UDWORD,
    pub asWeaps: [UDWORD; 1],
    pub droidType: DROID_TYPE,
    pub multiPlayerID: UDWORD,
    pub psNext: *mut _droid_template,
}
pub type DROID_TEMPLATE = _droid_template;
pub type _screenType = libc::c_uint;
pub const SCREEN_COVERMOUNT: _screenType = 8;
pub const SCREEN_SLIDE5: _screenType = 7;
pub const SCREEN_SLIDE4: _screenType = 6;
pub const SCREEN_SLIDE3: _screenType = 5;
pub const SCREEN_SLIDE2: _screenType = 4;
pub const SCREEN_SLIDE1: _screenType = 3;
pub const SCREEN_MISSIONEND: _screenType = 2;
pub const SCREEN_CREDITS: _screenType = 1;
pub const SCREEN_RANDOMBDROP: _screenType = 0;
pub type SCREENTYPE = _screenType;
/* Pointer to next template*/
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
/*
 * WidgInt.h
 *
 * Internal widget library definitions
 */
/* Control whether to use malloc for widgets */
/* Control whether the internal widget string heap should be used */
/* Context information to pass into the widget functions */
pub type W_CONTEXT = _w_context;
pub type W_FORM = _w_form;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_button {
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
    pub state: UDWORD,
    pub pText: *mut STRING,
    pub pTip: *mut STRING,
    pub HilightAudioID: SWORD,
    pub ClickedAudioID: SWORD,
    pub AudioCallback: WIDGET_AUDIOCALLBACK,
    pub FontID: libc::c_int,
}
// Parent screen of the widget
// Parent form of the widget
// Screen offset of the parent form
// mouse position on the form
// Button is hilited
// Button is locked down
// Button is locked but clickable
// Button flashing is enabled
// Button is flashing
pub type W_BUTTON = _w_button;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_editbox {
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
    pub state: UDWORD,
    pub aText: [STRING; 80],
    pub FontID: libc::c_int,
    pub insPos: UWORD,
    pub printStart: UWORD,
    pub printChars: UWORD,
    pub printWidth: UWORD,
    pub pBoxDisplay: WIDGET_DISPLAY,
    pub pFontDisplay: FONT_DISPLAY,
    pub HilightAudioID: SWORD,
    pub ClickedAudioID: SWORD,
    pub AudioCallback: WIDGET_AUDIOCALLBACK,
}
/* The common widget data */
// The current button state
// The text for the button
// The tool tip for the button
// Audio ID for form clicked sound
// Audio ID for form hilighted sound
// Pointer to audio callback function
//	PROP_FONT	*psFont;			// button font
/* Edit Box states */
// No editing is going on
// Insertion editing
// Overwrite editing
// 
//
// disable button from selection
pub type W_EDITBOX = _w_editbox;
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
/* The common widget data */
// The current edit box state
// The text in the edit box
//	PROP_FONT	*psFont;					// The font for the edit box
// The insertion point in the buffer
// Where in the string appears at the far left of the box
// The number of characters appearing in the box
// The pixel width of the characters in the box
// Optional callback to display the edit box background.
// Optional callback to display a string.
// Audio ID for form clicked sound
// Audio ID for form hilighted sound
// Pointer to audio callback function
/* **********************************************/
/* Image ID definition file created by Framer. */
/* **********************************************/
pub type C2RustUnnamed = libc::c_uint;
pub const IMAGE_ASCII64: C2RustUnnamed = 483;
pub const IMAGE_ORD_FIREDES_DOWN: C2RustUnnamed = 482;
pub const IMAGE_ORD_FIREDES_UP: C2RustUnnamed = 481;
pub const IMAGE_TRANSETA_DOWN: C2RustUnnamed = 480;
pub const IMAGE_LAUNCHUP: C2RustUnnamed = 479;
pub const IMAGE_BLUE6: C2RustUnnamed = 478;
pub const IMAGE_BLUE5: C2RustUnnamed = 477;
pub const IMAGE_BLUE4: C2RustUnnamed = 476;
pub const IMAGE_BLUE3: C2RustUnnamed = 475;
pub const IMAGE_BLUE2: C2RustUnnamed = 474;
pub const IMAGE_BLUE1: C2RustUnnamed = 473;
pub const IMAGE_DES_ARMOUR_KINETIC: C2RustUnnamed = 472;
pub const IMAGE_TABHILIGHT_SM: C2RustUnnamed = 471;
pub const IMAGE_TAB1DOWN_SM: C2RustUnnamed = 470;
pub const IMAGE_TAB1SELECTED_SM: C2RustUnnamed = 469;
pub const IMAGE_TAB1_SM: C2RustUnnamed = 468;
pub const IMAGE_TARGET5: C2RustUnnamed = 467;
pub const IMAGE_TARGET4: C2RustUnnamed = 466;
pub const IMAGE_TARGET3: C2RustUnnamed = 465;
pub const IMAGE_TARGET2: C2RustUnnamed = 464;
pub const IMAGE_TARGET1: C2RustUnnamed = 463;
pub const IMAGE_MISSION_CLOCK_UP: C2RustUnnamed = 462;
pub const IMAGE_RES_GRPDAM: C2RustUnnamed = 461;
pub const IMAGE_RES_GRPACC: C2RustUnnamed = 460;
pub const IMAGE_RES_GRPROF: C2RustUnnamed = 459;
pub const IMAGE_RES_GRPREP: C2RustUnnamed = 458;
pub const IMAGE_RES_GRPUPG: C2RustUnnamed = 457;
pub const IMAGE_ASCII131: C2RustUnnamed = 456;
pub const IMAGE_ASCII161: C2RustUnnamed = 455;
pub const IMAGE_ASCII191: C2RustUnnamed = 454;
pub const IMAGE_ASCII208: C2RustUnnamed = 453;
pub const IMAGE_ASCII207: C2RustUnnamed = 452;
pub const IMAGE_ASCII206: C2RustUnnamed = 451;
pub const IMAGE_ASCII205: C2RustUnnamed = 450;
pub const IMAGE_ASCII204: C2RustUnnamed = 449;
pub const IMAGE_ASCII203: C2RustUnnamed = 448;
pub const IMAGE_ASCII202: C2RustUnnamed = 447;
pub const IMAGE_ASCII201: C2RustUnnamed = 446;
pub const IMAGE_ASCII200: C2RustUnnamed = 445;
pub const IMAGE_ASCII198: C2RustUnnamed = 444;
pub const IMAGE_ASCII197: C2RustUnnamed = 443;
pub const IMAGE_ASCII196: C2RustUnnamed = 442;
pub const IMAGE_ASCII195: C2RustUnnamed = 441;
pub const IMAGE_ASCII194: C2RustUnnamed = 440;
pub const IMAGE_ASCII193: C2RustUnnamed = 439;
pub const IMAGE_ASCII192: C2RustUnnamed = 438;
pub const IMAGE_ASCII210: C2RustUnnamed = 437;
pub const IMAGE_ASCII211: C2RustUnnamed = 436;
pub const IMAGE_ASCII214: C2RustUnnamed = 435;
pub const IMAGE_ASCII213: C2RustUnnamed = 434;
pub const IMAGE_ASCII212: C2RustUnnamed = 433;
pub const IMAGE_ASCII216: C2RustUnnamed = 432;
pub const IMAGE_ASCII220: C2RustUnnamed = 431;
pub const IMAGE_ASCII217: C2RustUnnamed = 430;
pub const IMAGE_ASCII219: C2RustUnnamed = 429;
pub const IMAGE_ASCII218: C2RustUnnamed = 428;
pub const IMAGE_ASCII221: C2RustUnnamed = 427;
pub const IMAGE_ASCII223: C2RustUnnamed = 426;
pub const IMAGE_ASCII248: C2RustUnnamed = 425;
pub const IMAGE_ASCII241: C2RustUnnamed = 424;
pub const IMAGE_ASCII253: C2RustUnnamed = 423;
pub const IMAGE_ASCII252: C2RustUnnamed = 422;
pub const IMAGE_ASCII251: C2RustUnnamed = 421;
pub const IMAGE_ASCII250: C2RustUnnamed = 420;
pub const IMAGE_ASCII249: C2RustUnnamed = 419;
pub const IMAGE_ASCII246: C2RustUnnamed = 418;
pub const IMAGE_ASCII245: C2RustUnnamed = 417;
pub const IMAGE_ASCII244: C2RustUnnamed = 416;
pub const IMAGE_ASCII243: C2RustUnnamed = 415;
pub const IMAGE_ASCII242: C2RustUnnamed = 414;
pub const IMAGE_ASCII239: C2RustUnnamed = 413;
pub const IMAGE_ASCII238: C2RustUnnamed = 412;
pub const IMAGE_ASCII237: C2RustUnnamed = 411;
pub const IMAGE_ASCII236: C2RustUnnamed = 410;
pub const IMAGE_ASCII235: C2RustUnnamed = 409;
pub const IMAGE_ASCII234: C2RustUnnamed = 408;
pub const IMAGE_ASCII233: C2RustUnnamed = 407;
pub const IMAGE_ASCII232: C2RustUnnamed = 406;
pub const IMAGE_ASCII231: C2RustUnnamed = 405;
pub const IMAGE_ASCII230: C2RustUnnamed = 404;
pub const IMAGE_ASCII229: C2RustUnnamed = 403;
pub const IMAGE_ASCII228: C2RustUnnamed = 402;
pub const IMAGE_ASCII227: C2RustUnnamed = 401;
pub const IMAGE_ASCII226: C2RustUnnamed = 400;
pub const IMAGE_ASCII225: C2RustUnnamed = 399;
pub const IMAGE_ASCII224: C2RustUnnamed = 398;
pub const IMAGE_ASCII189: C2RustUnnamed = 397;
pub const IMAGE_ASCII188: C2RustUnnamed = 396;
pub const IMAGE_RES_DEFENCE: C2RustUnnamed = 395;
pub const IMAGE_LEV_7: C2RustUnnamed = 394;
pub const IMAGE_LEV_6: C2RustUnnamed = 393;
pub const IMAGE_LEV_5: C2RustUnnamed = 392;
pub const IMAGE_LEV_4: C2RustUnnamed = 391;
pub const IMAGE_LEV_3: C2RustUnnamed = 390;
pub const IMAGE_LEV_2: C2RustUnnamed = 389;
pub const IMAGE_LEV_1: C2RustUnnamed = 388;
pub const IMAGE_LEV_0: C2RustUnnamed = 387;
pub const IMAGE_CDCHANGE_CANCEL: C2RustUnnamed = 386;
pub const IMAGE_CDCHANGE_OK: C2RustUnnamed = 385;
pub const IMAGE_MULTI_CHAN: C2RustUnnamed = 384;
pub const IMAGE_MULTI_NOCHAN: C2RustUnnamed = 383;
pub const IMAGE_ORD_EMBARKDOWN: C2RustUnnamed = 382;
pub const IMAGE_ORD_EMBARKUP: C2RustUnnamed = 381;
pub const IMAGE_ORD_RTRDOWN: C2RustUnnamed = 380;
pub const IMAGE_ORD_RTRUP: C2RustUnnamed = 379;
pub const IMAGE_ORD_GUARDDOWN: C2RustUnnamed = 378;
pub const IMAGE_ORD_GUARDUP: C2RustUnnamed = 377;
pub const IMAGE_ORD_PERSUEDOWN: C2RustUnnamed = 376;
pub const IMAGE_ORD_PERSUEUP: C2RustUnnamed = 375;
pub const IMAGE_ORD_PATROLDOWN: C2RustUnnamed = 374;
pub const IMAGE_ORD_PATROLUP: C2RustUnnamed = 373;
pub const IMAGE_GN_0: C2RustUnnamed = 372;
pub const IMAGE_GN_1: C2RustUnnamed = 371;
pub const IMAGE_GN_2: C2RustUnnamed = 370;
pub const IMAGE_GN_3: C2RustUnnamed = 369;
pub const IMAGE_GN_4: C2RustUnnamed = 368;
pub const IMAGE_GN_5: C2RustUnnamed = 367;
pub const IMAGE_GN_6: C2RustUnnamed = 366;
pub const IMAGE_GN_7: C2RustUnnamed = 365;
pub const IMAGE_GN_8: C2RustUnnamed = 364;
pub const IMAGE_GN_9: C2RustUnnamed = 363;
pub const IMAGE_GN_STAR: C2RustUnnamed = 362;
pub const IMAGE_VDP_HI: C2RustUnnamed = 361;
pub const IMAGE_VDP_UP: C2RustUnnamed = 360;
pub const IMAGE_VDP_DOWN: C2RustUnnamed = 359;
pub const IMAGE_FDP_HI: C2RustUnnamed = 358;
pub const IMAGE_FDP_UP: C2RustUnnamed = 357;
pub const IMAGE_FDP_DOWN: C2RustUnnamed = 356;
pub const IMAGE_CDP_HI: C2RustUnnamed = 355;
pub const IMAGE_CDP_UP: C2RustUnnamed = 354;
pub const IMAGE_CDP_DOWN: C2RustUnnamed = 353;
pub const IMAGE_LOOP_HI: C2RustUnnamed = 352;
pub const IMAGE_LOOP_DOWN: C2RustUnnamed = 351;
pub const IMAGE_LOOP_UP: C2RustUnnamed = 350;
pub const IMAGE_RES_CYBORGTECH: C2RustUnnamed = 349;
pub const IMAGE_CURSOR_DEST: C2RustUnnamed = 348;
pub const IMAGE_DES_EDITBOXRIGHTH: C2RustUnnamed = 347;
pub const IMAGE_DES_EDITBOXMIDH: C2RustUnnamed = 346;
pub const IMAGE_DES_EDITBOXRIGHT: C2RustUnnamed = 345;
pub const IMAGE_DES_EDITBOXMID: C2RustUnnamed = 344;
pub const IMAGE_DES_POWERBAR_RIGHT: C2RustUnnamed = 343;
pub const IMAGE_DES_POWERBAR_LEFT: C2RustUnnamed = 342;
pub const IMAGE_AUDIO_LASTSAMPLEH: C2RustUnnamed = 341;
pub const IMAGE_AUDIO_LASTSAMPLE: C2RustUnnamed = 340;
pub const IMAGE_DES_BINH: C2RustUnnamed = 339;
pub const IMAGE_DES_BIN: C2RustUnnamed = 338;
pub const IMAGE_INFINITE_HI: C2RustUnnamed = 337;
pub const IMAGE_INFINITE_DOWN: C2RustUnnamed = 336;
pub const IMAGE_INFINITE_UP: C2RustUnnamed = 335;
pub const IMAGE_RETICULE_GREY: C2RustUnnamed = 334;
pub const IMAGE_ORD_FACHILITE: C2RustUnnamed = 333;
pub const IMAGE_ORD_FAC5DOWN: C2RustUnnamed = 332;
pub const IMAGE_ORD_FAC5UP: C2RustUnnamed = 331;
pub const IMAGE_ORD_FAC4DOWN: C2RustUnnamed = 330;
pub const IMAGE_ORD_FAC4UP: C2RustUnnamed = 329;
pub const IMAGE_ORD_FAC3DOWN: C2RustUnnamed = 328;
pub const IMAGE_ORD_FAC3UP: C2RustUnnamed = 327;
pub const IMAGE_ORD_FAC2DOWN: C2RustUnnamed = 326;
pub const IMAGE_ORD_FAC2UP: C2RustUnnamed = 325;
pub const IMAGE_ORD_FAC1DOWN: C2RustUnnamed = 324;
pub const IMAGE_ORD_FAC1UP: C2RustUnnamed = 323;
pub const IMAGE_TRANSETA_UP: C2RustUnnamed = 322;
pub const IMAGE_LAUNCHDOWN: C2RustUnnamed = 321;
pub const IMAGE_MISSION_CLOCK: C2RustUnnamed = 320;
pub const IMAGE_NRUTER: C2RustUnnamed = 319;
pub const IMAGE_CMDDROID_EXP: C2RustUnnamed = 318;
pub const IMAGE_MULTI_VIS_HI: C2RustUnnamed = 317;
pub const IMAGE_MULTI_POW_HI: C2RustUnnamed = 316;
pub const IMAGE_MULTI_DRO_HI: C2RustUnnamed = 315;
pub const IMAGE_MULTI_TEK_HI: C2RustUnnamed = 314;
pub const IMAGE_MULTI_TEK: C2RustUnnamed = 313;
pub const IMAGE_MULTI_DRO: C2RustUnnamed = 312;
pub const IMAGE_MULTI_POW: C2RustUnnamed = 311;
pub const IMAGE_MULTI_VIS: C2RustUnnamed = 310;
pub const IMAGE_MULTI_AL_HI: C2RustUnnamed = 309;
pub const IMAGE_MULTI_OFFAL_HI: C2RustUnnamed = 308;
pub const IMAGE_MULTI_NOAL_HI: C2RustUnnamed = 307;
pub const IMAGE_MULTI_AL: C2RustUnnamed = 306;
pub const IMAGE_MULTI_OFFAL: C2RustUnnamed = 305;
pub const IMAGE_MULTI_NOAL: C2RustUnnamed = 304;
pub const IMAGE_RAD_ART3: C2RustUnnamed = 303;
pub const IMAGE_RAD_ART2: C2RustUnnamed = 302;
pub const IMAGE_RAD_ART1: C2RustUnnamed = 301;
pub const IMAGE_RAD_ARTREAD: C2RustUnnamed = 300;
pub const IMAGE_RAD_RES3: C2RustUnnamed = 299;
pub const IMAGE_RAD_RES2: C2RustUnnamed = 298;
pub const IMAGE_RAD_RES1: C2RustUnnamed = 297;
pub const IMAGE_RAD_RESREAD: C2RustUnnamed = 296;
pub const IMAGE_RAD_ENM3: C2RustUnnamed = 295;
pub const IMAGE_RAD_ENM2: C2RustUnnamed = 294;
pub const IMAGE_RAD_ENM1: C2RustUnnamed = 293;
pub const IMAGE_RAD_ENMREAD: C2RustUnnamed = 292;
pub const IMAGE_SLIDER_INFINITY: C2RustUnnamed = 291;
pub const IMAGE_SLIDER_BIGBUT: C2RustUnnamed = 290;
pub const IMAGE_ORD_DESTRUCT2GREY: C2RustUnnamed = 289;
pub const IMAGE_ORD_DESTRUCT2DOWN: C2RustUnnamed = 288;
pub const IMAGE_ORD_DESTRUCT2UP: C2RustUnnamed = 287;
pub const IMAGE_ORD_GOTOHQDOWN: C2RustUnnamed = 286;
pub const IMAGE_ORD_GOTOHQUP: C2RustUnnamed = 285;
pub const IMAGE_ORD_HALTDOWN: C2RustUnnamed = 284;
pub const IMAGE_ORD_HALTUP: C2RustUnnamed = 283;
pub const IMAGE_ORD_HOLDFIREDOWN: C2RustUnnamed = 282;
pub const IMAGE_ORD_HOLDFIREUP: C2RustUnnamed = 281;
pub const IMAGE_ORD_RETFIREDOWN: C2RustUnnamed = 280;
pub const IMAGE_ORD_RETFIREUP: C2RustUnnamed = 279;
pub const IMAGE_ORD_FATWILLDOWN: C2RustUnnamed = 278;
pub const IMAGE_ORD_FATWILLUP: C2RustUnnamed = 277;
pub const IMAGE_ORD_REPAIR3DOWN: C2RustUnnamed = 276;
pub const IMAGE_ORD_REPAIR3UP: C2RustUnnamed = 275;
pub const IMAGE_ORD_REPAIR1DOWN: C2RustUnnamed = 274;
pub const IMAGE_ORD_REPAIR1UP: C2RustUnnamed = 273;
pub const IMAGE_ORD_REPAIR2DOWN: C2RustUnnamed = 272;
pub const IMAGE_ORD_REPAIR2UP: C2RustUnnamed = 271;
pub const IMAGE_ORD_RANGE3DOWN: C2RustUnnamed = 270;
pub const IMAGE_ORD_RANGE3UP: C2RustUnnamed = 269;
pub const IMAGE_ORD_RANGE2DOWN: C2RustUnnamed = 268;
pub const IMAGE_ORD_RANGE2UP: C2RustUnnamed = 267;
pub const IMAGE_ORD_RANGE1DOWN: C2RustUnnamed = 266;
pub const IMAGE_ORD_RANGE1UP: C2RustUnnamed = 265;
pub const IMAGE_ORD_DESTRUCT1DOWN: C2RustUnnamed = 264;
pub const IMAGE_ORD_DESTRUCT1UP: C2RustUnnamed = 263;
pub const IMAGE_RES_QUESTIONMARK: C2RustUnnamed = 262;
pub const IMAGE_RES_STRUCTURETECH: C2RustUnnamed = 261;
pub const IMAGE_RES_SYSTEMTECH: C2RustUnnamed = 260;
pub const IMAGE_RES_POWERTECH: C2RustUnnamed = 259;
pub const IMAGE_RES_COMPUTERTECH: C2RustUnnamed = 258;
pub const IMAGE_RES_WEAPONTECH: C2RustUnnamed = 257;
pub const IMAGE_RES_DROIDTECH: C2RustUnnamed = 256;
pub const IMAGE_QUESTION_MARK: C2RustUnnamed = 255;
pub const IMAGE_RES_MAJOR_ROCKET: C2RustUnnamed = 254;
pub const IMAGE_RES_MINOR_AUTOWEAPONS: C2RustUnnamed = 253;
pub const IMAGE_TRACKS: C2RustUnnamed = 252;
pub const IMAGE_STAR: C2RustUnnamed = 251;
pub const IMAGE_SLIDER_BIG: C2RustUnnamed = 250;
pub const IMAGE_DES_STATBACKMID: C2RustUnnamed = 249;
pub const IMAGE_DES_STATBACKRIGHT: C2RustUnnamed = 248;
pub const IMAGE_DES_STATBACKLEFT: C2RustUnnamed = 247;
pub const IMAGE_SIDETABSEL: C2RustUnnamed = 246;
pub const IMAGE_SIDETABDOWN: C2RustUnnamed = 245;
pub const IMAGE_SIDETABHI: C2RustUnnamed = 244;
pub const IMAGE_SIDETAB: C2RustUnnamed = 243;
pub const IMAGE_CURSOR_NOTPOS: C2RustUnnamed = 242;
pub const IMAGE_CURSOR_PICKUP: C2RustUnnamed = 241;
pub const IMAGE_CURSOR_REPAIR: C2RustUnnamed = 240;
pub const IMAGE_CURSOR_FIX: C2RustUnnamed = 239;
pub const IMAGE_CURSOR_ATTACH: C2RustUnnamed = 238;
pub const IMAGE_CURSOR_BRIDGE: C2RustUnnamed = 237;
pub const IMAGE_CURSOR_EMBARK: C2RustUnnamed = 236;
pub const IMAGE_CURSOR_GUARD: C2RustUnnamed = 235;
pub const IMAGE_CURSOR_MOVE: C2RustUnnamed = 234;
pub const IMAGE_CURSOR_BUILD: C2RustUnnamed = 233;
pub const IMAGE_CURSOR_DEFAULT: C2RustUnnamed = 232;
pub const IMAGE_CURSOR_ECM: C2RustUnnamed = 231;
pub const IMAGE_CURSOR_LOCKON: C2RustUnnamed = 230;
pub const IMAGE_CURSOR_SELECT: C2RustUnnamed = 229;
pub const IMAGE_CURSOR_ATTACK: C2RustUnnamed = 228;
pub const IMAGE_ASCII126: C2RustUnnamed = 227;
pub const IMAGE_ASCII125: C2RustUnnamed = 226;
pub const IMAGE_ASCII124: C2RustUnnamed = 225;
pub const IMAGE_ASCII123: C2RustUnnamed = 224;
pub const IMAGE_ASCII122: C2RustUnnamed = 223;
pub const IMAGE_ASCII121: C2RustUnnamed = 222;
pub const IMAGE_ASCII120: C2RustUnnamed = 221;
pub const IMAGE_ASCII119: C2RustUnnamed = 220;
pub const IMAGE_ASCII118: C2RustUnnamed = 219;
pub const IMAGE_ASCII117: C2RustUnnamed = 218;
pub const IMAGE_ASCII116: C2RustUnnamed = 217;
pub const IMAGE_ASCII115: C2RustUnnamed = 216;
pub const IMAGE_ASCII114: C2RustUnnamed = 215;
pub const IMAGE_ASCII113: C2RustUnnamed = 214;
pub const IMAGE_ASCII112: C2RustUnnamed = 213;
pub const IMAGE_ASCII111: C2RustUnnamed = 212;
pub const IMAGE_ASCII110: C2RustUnnamed = 211;
pub const IMAGE_ASCII109: C2RustUnnamed = 210;
pub const IMAGE_ASCII108: C2RustUnnamed = 209;
pub const IMAGE_ASCII107: C2RustUnnamed = 208;
pub const IMAGE_ASCII106: C2RustUnnamed = 207;
pub const IMAGE_ASCII105: C2RustUnnamed = 206;
pub const IMAGE_ASCII104: C2RustUnnamed = 205;
pub const IMAGE_ASCII103: C2RustUnnamed = 204;
pub const IMAGE_ASCII102: C2RustUnnamed = 203;
pub const IMAGE_ASCII101: C2RustUnnamed = 202;
pub const IMAGE_ASCII100: C2RustUnnamed = 201;
pub const IMAGE_ASCII99: C2RustUnnamed = 200;
pub const IMAGE_ASCII98: C2RustUnnamed = 199;
pub const IMAGE_ASCII97: C2RustUnnamed = 198;
pub const IMAGE_ASCII96: C2RustUnnamed = 197;
pub const IMAGE_ASCII95: C2RustUnnamed = 196;
pub const IMAGE_ASCII94: C2RustUnnamed = 195;
pub const IMAGE_ASCII93: C2RustUnnamed = 194;
pub const IMAGE_ASCII92: C2RustUnnamed = 193;
pub const IMAGE_ASCII91: C2RustUnnamed = 192;
pub const IMAGE_ASCII90: C2RustUnnamed = 191;
pub const IMAGE_ASCII89: C2RustUnnamed = 190;
pub const IMAGE_ASCII88: C2RustUnnamed = 189;
pub const IMAGE_ASCII87: C2RustUnnamed = 188;
pub const IMAGE_ASCII86: C2RustUnnamed = 187;
pub const IMAGE_ASCII85: C2RustUnnamed = 186;
pub const IMAGE_ASCII84: C2RustUnnamed = 185;
pub const IMAGE_ASCII83: C2RustUnnamed = 184;
pub const IMAGE_ASCII82: C2RustUnnamed = 183;
pub const IMAGE_ASCII81: C2RustUnnamed = 182;
pub const IMAGE_ASCII80: C2RustUnnamed = 181;
pub const IMAGE_ASCII79: C2RustUnnamed = 180;
pub const IMAGE_ASCII78: C2RustUnnamed = 179;
pub const IMAGE_ASCII77: C2RustUnnamed = 178;
pub const IMAGE_ASCII76: C2RustUnnamed = 177;
pub const IMAGE_ASCII75: C2RustUnnamed = 176;
pub const IMAGE_ASCII74: C2RustUnnamed = 175;
pub const IMAGE_ASCII73: C2RustUnnamed = 174;
pub const IMAGE_ASCII72: C2RustUnnamed = 173;
pub const IMAGE_ASCII71: C2RustUnnamed = 172;
pub const IMAGE_ASCII70: C2RustUnnamed = 171;
pub const IMAGE_ASCII69: C2RustUnnamed = 170;
pub const IMAGE_ASCII68: C2RustUnnamed = 169;
pub const IMAGE_ASCII67: C2RustUnnamed = 168;
pub const IMAGE_ASCII66: C2RustUnnamed = 167;
pub const IMAGE_ASCII65: C2RustUnnamed = 166;
pub const IMAGE_ASTERISK: C2RustUnnamed = 165;
pub const IMAGE_ASCII63: C2RustUnnamed = 164;
pub const IMAGE_ASCII62: C2RustUnnamed = 163;
pub const IMAGE_ASCII61: C2RustUnnamed = 162;
pub const IMAGE_ASCII60: C2RustUnnamed = 161;
pub const IMAGE_ASCII59: C2RustUnnamed = 160;
pub const IMAGE_ASCII58: C2RustUnnamed = 159;
pub const IMAGE_ASCII57: C2RustUnnamed = 158;
pub const IMAGE_ASCII56: C2RustUnnamed = 157;
pub const IMAGE_ASCII55: C2RustUnnamed = 156;
pub const IMAGE_ASCII54: C2RustUnnamed = 155;
pub const IMAGE_ASCII53: C2RustUnnamed = 154;
pub const IMAGE_ASCII52: C2RustUnnamed = 153;
pub const IMAGE_ASCII51: C2RustUnnamed = 152;
pub const IMAGE_ASCII50: C2RustUnnamed = 151;
pub const IMAGE_ASCII49: C2RustUnnamed = 150;
pub const IMAGE_ASCII48: C2RustUnnamed = 149;
pub const IMAGE_ASCII47: C2RustUnnamed = 148;
pub const IMAGE_ASCII46: C2RustUnnamed = 147;
pub const IMAGE_ASCII45: C2RustUnnamed = 146;
pub const IMAGE_ASCII44: C2RustUnnamed = 145;
pub const IMAGE_ASCII43: C2RustUnnamed = 144;
pub const IMAGE_ASCII42: C2RustUnnamed = 143;
pub const IMAGE_ASCII41: C2RustUnnamed = 142;
pub const IMAGE_ASCII40: C2RustUnnamed = 141;
pub const IMAGE_ASCII39: C2RustUnnamed = 140;
pub const IMAGE_ASCII38: C2RustUnnamed = 139;
pub const IMAGE_ASCII37: C2RustUnnamed = 138;
pub const IMAGE_ASCII36: C2RustUnnamed = 137;
pub const IMAGE_ASCII35: C2RustUnnamed = 136;
pub const IMAGE_ASCII34: C2RustUnnamed = 135;
pub const IMAGE_ASCII33: C2RustUnnamed = 134;
pub const IMAGE_INTEL_CAMPAIGNDOWN: C2RustUnnamed = 133;
pub const IMAGE_INTEL_CAMPAIGN: C2RustUnnamed = 132;
pub const IMAGE_INTEL_MISSIONDOWN: C2RustUnnamed = 131;
pub const IMAGE_INTEL_MISSION: C2RustUnnamed = 130;
pub const IMAGE_INTEL_RESEARCHDOWN: C2RustUnnamed = 129;
pub const IMAGE_INTEL_RESEARCH: C2RustUnnamed = 128;
pub const IMAGE_DES_BARYELLOW: C2RustUnnamed = 127;
pub const IMAGE_DES_BARRED: C2RustUnnamed = 126;
pub const IMAGE_DES_BARBLUE: C2RustUnnamed = 125;
pub const IMAGE_DES_BARBACK: C2RustUnnamed = 124;
pub const IMAGE_9: C2RustUnnamed = 123;
pub const IMAGE_8: C2RustUnnamed = 122;
pub const IMAGE_7: C2RustUnnamed = 121;
pub const IMAGE_6: C2RustUnnamed = 120;
pub const IMAGE_5: C2RustUnnamed = 119;
pub const IMAGE_4: C2RustUnnamed = 118;
pub const IMAGE_3: C2RustUnnamed = 117;
pub const IMAGE_2: C2RustUnnamed = 116;
pub const IMAGE_1: C2RustUnnamed = 115;
pub const IMAGE_0: C2RustUnnamed = 114;
pub const IMAGE_RES_MAJOR_HOVER: C2RustUnnamed = 113;
pub const IMAGE_RES_MAJOR_HEAVYWEP: C2RustUnnamed = 112;
pub const IMAGE_RES_MAJOR_ELECTRONIC: C2RustUnnamed = 111;
pub const IMAGE_RES_MAJOR_PLASCRETE: C2RustUnnamed = 110;
pub const IMAGE_RES_MINOR_RADAR: C2RustUnnamed = 109;
pub const IMAGE_PLASCRETE: C2RustUnnamed = 108;
pub const IMAGE_ECM: C2RustUnnamed = 107;
pub const IMAGE_RES_MINOR_PLASSTEEL: C2RustUnnamed = 106;
pub const IMAGE_HOVERCRAFT: C2RustUnnamed = 105;
pub const IMAGE_CANNON: C2RustUnnamed = 104;
pub const IMAGE_ROCKET: C2RustUnnamed = 103;
pub const IMAGE_DES_BODYPOINTS: C2RustUnnamed = 102;
pub const IMAGE_DES_FIRERATE: C2RustUnnamed = 101;
pub const IMAGE_DES_HOVER: C2RustUnnamed = 100;
pub const IMAGE_DES_POWER: C2RustUnnamed = 99;
pub const IMAGE_DES_DAMAGE: C2RustUnnamed = 98;
pub const IMAGE_DES_WEIGHT: C2RustUnnamed = 97;
pub const IMAGE_DES_ROAD: C2RustUnnamed = 96;
pub const IMAGE_DES_CROSSCOUNTRY: C2RustUnnamed = 95;
pub const IMAGE_DES_ARMOUR_EXPLOSIVE: C2RustUnnamed = 94;
pub const IMAGE_DES_RANGE: C2RustUnnamed = 93;
pub const IMAGE_DES_BUILDRATE: C2RustUnnamed = 92;
pub const IMAGE_DES_TABWEAPONDOWN: C2RustUnnamed = 91;
pub const IMAGE_DES_TABWEAPON: C2RustUnnamed = 90;
pub const IMAGE_DES_EXTRAHI: C2RustUnnamed = 89;
pub const IMAGE_DES_STATSCOMP: C2RustUnnamed = 88;
pub const IMAGE_DES_BACK: C2RustUnnamed = 87;
pub const IMAGE_FRAME_HC3: C2RustUnnamed = 86;
pub const IMAGE_FRAME_HC2: C2RustUnnamed = 85;
pub const IMAGE_FRAME_HC1: C2RustUnnamed = 84;
pub const IMAGE_FRAME_HC0: C2RustUnnamed = 83;
pub const IMAGE_FRAME_VC3: C2RustUnnamed = 82;
pub const IMAGE_FRAME_VC2: C2RustUnnamed = 81;
pub const IMAGE_FRAME_VC1: C2RustUnnamed = 80;
pub const IMAGE_FRAME_VC0: C2RustUnnamed = 79;
pub const IMAGE_DES_EDITBOXLEFTH: C2RustUnnamed = 78;
pub const IMAGE_DES_EDITBOXLEFT: C2RustUnnamed = 77;
pub const IMAGE_DES_POWERBACK: C2RustUnnamed = 76;
pub const IMAGE_DES_STATSCURR: C2RustUnnamed = 75;
pub const IMAGE_DES_STATSBACK: C2RustUnnamed = 74;
pub const IMAGE_DES_POWERCURR: C2RustUnnamed = 73;
pub const IMAGE_FRAME_HBH: C2RustUnnamed = 72;
pub const IMAGE_FRAME_HTH: C2RustUnnamed = 71;
pub const IMAGE_FRAME_HB2: C2RustUnnamed = 70;
pub const IMAGE_FRAME_HT2: C2RustUnnamed = 69;
pub const IMAGE_DES_COMMANDDOWN: C2RustUnnamed = 68;
pub const IMAGE_DES_COMMAND: C2RustUnnamed = 67;
pub const IMAGE_DES_SYSTEMSDOWN: C2RustUnnamed = 66;
pub const IMAGE_DES_SYSTEMS: C2RustUnnamed = 65;
pub const IMAGE_DES_WEAPONSDOWN: C2RustUnnamed = 64;
pub const IMAGE_DES_WEAPONS: C2RustUnnamed = 63;
pub const IMAGE_FRAME_VRH: C2RustUnnamed = 62;
pub const IMAGE_FRAME_VLH: C2RustUnnamed = 61;
pub const IMAGE_FRAME_VR2: C2RustUnnamed = 60;
pub const IMAGE_FRAME_VL2: C2RustUnnamed = 59;
pub const IMAGE_DES_HILIGHT: C2RustUnnamed = 58;
pub const IMAGE_DES_PROPULSIONH: C2RustUnnamed = 57;
pub const IMAGE_DES_PROPULSION: C2RustUnnamed = 56;
pub const IMAGE_DES_BODYH: C2RustUnnamed = 55;
pub const IMAGE_DES_BODY: C2RustUnnamed = 54;
pub const IMAGE_DES_TURRETH: C2RustUnnamed = 53;
pub const IMAGE_DES_TURRET: C2RustUnnamed = 52;
pub const IMAGE_FRAME_VR: C2RustUnnamed = 51;
pub const IMAGE_FRAME_VL: C2RustUnnamed = 50;
pub const IMAGE_SLIDER_BACK: C2RustUnnamed = 49;
pub const IMAGE_FRAME_HB: C2RustUnnamed = 48;
pub const IMAGE_FRAME_HT: C2RustUnnamed = 47;
pub const IMAGE_SLIDER_BUT: C2RustUnnamed = 46;
pub const IMAGE_CLOSEHILIGHT: C2RustUnnamed = 45;
pub const IMAGE_CLOSEDOWN: C2RustUnnamed = 44;
pub const IMAGE_CLOSE: C2RustUnnamed = 43;
pub const IMAGE_CANCEL_HILIGHT: C2RustUnnamed = 42;
pub const IMAGE_CANCEL_DOWN: C2RustUnnamed = 41;
pub const IMAGE_CANCEL_UP: C2RustUnnamed = 40;
pub const IMAGE_RETICULE_BUTDOWN: C2RustUnnamed = 39;
pub const IMAGE_RETICULE_HILIGHT: C2RustUnnamed = 38;
pub const IMAGE_MANUFACTURE_DOWN: C2RustUnnamed = 37;
pub const IMAGE_MANUFACTURE_UP: C2RustUnnamed = 36;
pub const IMAGE_RESEARCH_DOWN: C2RustUnnamed = 35;
pub const IMAGE_RESEARCH_UP: C2RustUnnamed = 34;
pub const IMAGE_BUILD_DOWN: C2RustUnnamed = 33;
pub const IMAGE_BUILD_UP: C2RustUnnamed = 32;
pub const IMAGE_DESIGN_DOWN: C2RustUnnamed = 31;
pub const IMAGE_DESIGN_UP: C2RustUnnamed = 30;
pub const IMAGE_COMMANDDROID_DOWN: C2RustUnnamed = 29;
pub const IMAGE_COMMANDDROID_UP: C2RustUnnamed = 28;
pub const IMAGE_INTELMAP_DOWN: C2RustUnnamed = 27;
pub const IMAGE_INTELMAP_UP: C2RustUnnamed = 26;
pub const IMAGE_PBAR_BOTTOM: C2RustUnnamed = 25;
pub const IMAGE_PBAR_TOP: C2RustUnnamed = 24;
pub const IMAGE_FRAME_C3: C2RustUnnamed = 23;
pub const IMAGE_FRAME_C2: C2RustUnnamed = 22;
pub const IMAGE_FRAME_C1: C2RustUnnamed = 21;
pub const IMAGE_FRAME_C0: C2RustUnnamed = 20;
pub const IMAGE_TABHILIGHT: C2RustUnnamed = 19;
pub const IMAGE_TABSELECTED: C2RustUnnamed = 18;
pub const IMAGE_TAB4DOWN: C2RustUnnamed = 17;
pub const IMAGE_TAB3DOWN: C2RustUnnamed = 16;
pub const IMAGE_TAB2DOWN: C2RustUnnamed = 15;
pub const IMAGE_TAB1DOWN: C2RustUnnamed = 14;
pub const IMAGE_TAB4: C2RustUnnamed = 13;
pub const IMAGE_TAB3: C2RustUnnamed = 12;
pub const IMAGE_TAB2: C2RustUnnamed = 11;
pub const IMAGE_TAB1: C2RustUnnamed = 10;
pub const IMAGE_BUTB_HILITE: C2RustUnnamed = 9;
pub const IMAGE_BUT_HILITE: C2RustUnnamed = 8;
pub const IMAGE_BUTB0_DOWN: C2RustUnnamed = 7;
pub const IMAGE_BUTB0_UP: C2RustUnnamed = 6;
pub const IMAGE_BUT0_DOWN: C2RustUnnamed = 5;
pub const IMAGE_BUT0_UP: C2RustUnnamed = 4;
pub const IMAGE_PBAR_REQUIRED: C2RustUnnamed = 3;
pub const IMAGE_PBAR_USED: C2RustUnnamed = 2;
pub const IMAGE_PBAR_AVAIL: C2RustUnnamed = 1;
pub const IMAGE_PBAR_EMPTY: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BUTTON_SURFACE {
    pub Buffer: *mut uint8,
    pub Surface: *mut iSurface,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RENDERED_BUTTON {
    pub InUse: BOOL,
    pub Initialised: BOOL,
    pub ImdRotation: SDWORD,
    pub State: UDWORD,
    pub Data: *mut libc::c_void,
    pub Data2: *mut libc::c_void,
    pub ButSurf: *mut BUTTON_SURFACE,
}
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
pub type _loadsave_mode = libc::c_uint;
pub const SAVE_FORCE: _loadsave_mode = 6;
pub const LOAD_FORCE: _loadsave_mode = 5;
pub const SAVE_INGAME: _loadsave_mode = 4;
pub const LOAD_INGAME: _loadsave_mode = 3;
pub const SAVE_MISSIONEND: _loadsave_mode = 2;
pub const LOAD_MISSIONEND: _loadsave_mode = 1;
pub const LOAD_FRONTEND: _loadsave_mode = 0;
/* **************************************************************************/
/* 
 * loadsave.h
 */
/* **************************************************************************/
/* **************************************************************************/
/*
 *	Global Definitions
 */
/* **************************************************************************/
pub type LOADSAVE_MODE = _loadsave_mode;
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
/* Information stored with each tile */
// The name is now changed to MAPTILE to allow correct compilation on the PlayStation
pub type MAPTILE = _maptile;
pub type CONSOLE_TEXT_JUSTIFICATION = libc::c_uint;
pub const DEFAULT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 3;
pub const CENTRE_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 2;
pub const RIGHT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 1;
pub const LEFT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 0;
/*#ifndef PSX
	UBYTE			tileExtraBits;	// We've got more than you... We've got more than you..;-)
#endif*/
// COMPRESSED - bit per player
/*#ifndef PSX
	UBYTE			tileDoorBits;   // same thing - bit per player
#endif*/
// The height at the top left of the tile
// How bright is this tile?
// Which graphics texture is on this tile
// sensor range display.
// This is also used to store the tile flip flags
//  What's been removed - 46 bytes per tile so far
//	BASE_OBJECT		*psObject;		// Any object sitting on the location (e.g. building)
//	UBYTE			onFire;			// Is tile on fire?
//	UBYTE			rippleIndex;	// Current value in ripple table?
//	BOOL			tileVisible[MAX_PLAYERS]; // Which players can see the tile?
//	BOOL			triangleFlip;	// Is the triangle flipped?
//	TYPE_OF_TERRAIN	type;			// The terrain type for the tile
/* **********************************************/
/* Image ID definition file created by Framer. */
/* **********************************************/
pub type C2RustUnnamed_0 = libc::c_uint;
pub const IMAGE_NOJOIN: C2RustUnnamed_0 = 281;
pub const IMAGE_TFONT_45: C2RustUnnamed_0 = 280;
pub const IMAGE_TFONT_63: C2RustUnnamed_0 = 279;
pub const IMAGE_TFONT_205: C2RustUnnamed_0 = 278;
pub const IMAGE_TFONT_204: C2RustUnnamed_0 = 277;
pub const IMAGE_TFONT_207: C2RustUnnamed_0 = 276;
pub const IMAGE_TFONT_206: C2RustUnnamed_0 = 275;
pub const IMAGE_TFONT_211: C2RustUnnamed_0 = 274;
pub const IMAGE_TFONT_202: C2RustUnnamed_0 = 273;
pub const IMAGE_TFONT_170: C2RustUnnamed_0 = 272;
pub const IMAGE_TFONT_163: C2RustUnnamed_0 = 271;
pub const IMAGE_TFONT_223: C2RustUnnamed_0 = 270;
pub const IMAGE_MULTIRANK3: C2RustUnnamed_0 = 269;
pub const IMAGE_KEYMAP_DEFAULT: C2RustUnnamed_0 = 268;
pub const IMAGE_NOPENCIL: C2RustUnnamed_0 = 267;
pub const IMAGE_PENCIL: C2RustUnnamed_0 = 266;
pub const IMAGE_TFONT_245: C2RustUnnamed_0 = 265;
pub const IMAGE_TFONT_221: C2RustUnnamed_0 = 264;
pub const IMAGE_TFONT_217: C2RustUnnamed_0 = 263;
pub const IMAGE_TFONT_219: C2RustUnnamed_0 = 262;
pub const IMAGE_TFONT_218: C2RustUnnamed_0 = 261;
pub const IMAGE_TFONT_213: C2RustUnnamed_0 = 260;
pub const IMAGE_TFONT_210: C2RustUnnamed_0 = 259;
pub const IMAGE_TFONT_212: C2RustUnnamed_0 = 258;
pub const IMAGE_TFONT_200: C2RustUnnamed_0 = 257;
pub const IMAGE_TFONT_203: C2RustUnnamed_0 = 256;
pub const IMAGE_TFONT_208: C2RustUnnamed_0 = 255;
pub const IMAGE_TFONT_240: C2RustUnnamed_0 = 254;
pub const IMAGE_TFONT_195: C2RustUnnamed_0 = 253;
pub const IMAGE_TFONT_192: C2RustUnnamed_0 = 252;
pub const IMAGE_TFONT_194: C2RustUnnamed_0 = 251;
pub const IMAGE_TFONT_193: C2RustUnnamed_0 = 250;
pub const IMAGE_TFONT_187: C2RustUnnamed_0 = 249;
pub const IMAGE_TFONT_171: C2RustUnnamed_0 = 248;
pub const IMAGE_TFONT_172: C2RustUnnamed_0 = 247;
pub const IMAGE_TFONT_174: C2RustUnnamed_0 = 246;
pub const IMAGE_TFONT_161: C2RustUnnamed_0 = 245;
pub const IMAGE_TFONT_191: C2RustUnnamed_0 = 244;
pub const IMAGE_TFONT_176: C2RustUnnamed_0 = 243;
pub const IMAGE_TFONT_131: C2RustUnnamed_0 = 242;
pub const IMAGE_TFONT_215: C2RustUnnamed_0 = 241;
pub const IMAGE_TFONT_216: C2RustUnnamed_0 = 240;
pub const IMAGE_TFONT_220: C2RustUnnamed_0 = 239;
pub const IMAGE_TFONT_214: C2RustUnnamed_0 = 238;
pub const IMAGE_TFONT_201: C2RustUnnamed_0 = 237;
pub const IMAGE_TFONT_197: C2RustUnnamed_0 = 236;
pub const IMAGE_TFONT_196: C2RustUnnamed_0 = 235;
pub const IMAGE_TFONT_198: C2RustUnnamed_0 = 234;
pub const IMAGE_TFONT_128: C2RustUnnamed_0 = 233;
pub const IMAGE_TFONT_253: C2RustUnnamed_0 = 232;
pub const IMAGE_TFONT_252: C2RustUnnamed_0 = 231;
pub const IMAGE_TFONT_251: C2RustUnnamed_0 = 230;
pub const IMAGE_TFONT_250: C2RustUnnamed_0 = 229;
pub const IMAGE_TFONT_249: C2RustUnnamed_0 = 228;
pub const IMAGE_TFONT_248: C2RustUnnamed_0 = 227;
pub const IMAGE_TFONT_246: C2RustUnnamed_0 = 226;
pub const IMAGE_TFONT_244: C2RustUnnamed_0 = 225;
pub const IMAGE_TFONT_243: C2RustUnnamed_0 = 224;
pub const IMAGE_TFONT_242: C2RustUnnamed_0 = 223;
pub const IMAGE_TFONT_209: C2RustUnnamed_0 = 222;
pub const IMAGE_TFONT_241: C2RustUnnamed_0 = 221;
pub const IMAGE_TFONT_239: C2RustUnnamed_0 = 220;
pub const IMAGE_TFONT_238: C2RustUnnamed_0 = 219;
pub const IMAGE_TFONT_237: C2RustUnnamed_0 = 218;
pub const IMAGE_TFONT_236: C2RustUnnamed_0 = 217;
pub const IMAGE_TFONT_235: C2RustUnnamed_0 = 216;
pub const IMAGE_TFONT_234: C2RustUnnamed_0 = 215;
pub const IMAGE_TFONT_233: C2RustUnnamed_0 = 214;
pub const IMAGE_TFONT_232: C2RustUnnamed_0 = 213;
pub const IMAGE_TFONT_199: C2RustUnnamed_0 = 212;
pub const IMAGE_TFONT_230: C2RustUnnamed_0 = 211;
pub const IMAGE_TFONT_229: C2RustUnnamed_0 = 210;
pub const IMAGE_TFONT_228: C2RustUnnamed_0 = 209;
pub const IMAGE_TFONT_227: C2RustUnnamed_0 = 208;
pub const IMAGE_TFONT_226: C2RustUnnamed_0 = 207;
pub const IMAGE_TFONT_188: C2RustUnnamed_0 = 206;
pub const IMAGE_TFONT_224: C2RustUnnamed_0 = 205;
pub const IMAGE_TFONT_225: C2RustUnnamed_0 = 204;
pub const IMAGE_TFONT_189: C2RustUnnamed_0 = 203;
pub const IMAGE_WEE_GUY: C2RustUnnamed_0 = 202;
pub const IMAGE_FOG_ON_HI: C2RustUnnamed_0 = 201;
pub const IMAGE_FOG_OFF_HI: C2RustUnnamed_0 = 200;
pub const IMAGE_FOG_ON: C2RustUnnamed_0 = 199;
pub const IMAGE_FOG_OFF: C2RustUnnamed_0 = 198;
pub const IMAGE_PLAYERX: C2RustUnnamed_0 = 197;
pub const IMAGE_MEDAL_DUMMY: C2RustUnnamed_0 = 196;
pub const IMAGE_MULTIRANK2: C2RustUnnamed_0 = 195;
pub const IMAGE_PLAYER_PC: C2RustUnnamed_0 = 194;
pub const IMAGE_TEAM_HI: C2RustUnnamed_0 = 193;
pub const IMAGE_SKIRMISH_HI: C2RustUnnamed_0 = 192;
pub const IMAGE_TEAM: C2RustUnnamed_0 = 191;
pub const IMAGE_SKIRMISH: C2RustUnnamed_0 = 190;
pub const IMAGE_TEAM_OVER: C2RustUnnamed_0 = 189;
pub const IMAGE_SKIRMISH_OVER: C2RustUnnamed_0 = 188;
pub const IMAGE_LAMP_GREEN: C2RustUnnamed_0 = 187;
pub const IMAGE_LAMP_AMBER: C2RustUnnamed_0 = 186;
pub const IMAGE_LAMP_RED: C2RustUnnamed_0 = 185;
pub const IMAGE_COMPUTER_Y_HI: C2RustUnnamed_0 = 184;
pub const IMAGE_COMPUTER_Y: C2RustUnnamed_0 = 183;
pub const IMAGE_COMPUTER_N: C2RustUnnamed_0 = 182;
pub const IMAGE_COMPUTER_N_HI: C2RustUnnamed_0 = 181;
pub const IMAGE_HI56: C2RustUnnamed_0 = 180;
pub const IMAGE_DEFAULTFORCE: C2RustUnnamed_0 = 179;
pub const IMAGE_CLEARFORCE: C2RustUnnamed_0 = 178;
pub const IMAGE_SAVEFORCE: C2RustUnnamed_0 = 177;
pub const IMAGE_LOADFORCE: C2RustUnnamed_0 = 176;
pub const IMAGE_SLIM_HI: C2RustUnnamed_0 = 175;
pub const IMAGE_SLIM: C2RustUnnamed_0 = 174;
pub const IMAGE_RETURN_HI: C2RustUnnamed_0 = 173;
pub const IMAGE_FRAGLIMIT_HI: C2RustUnnamed_0 = 172;
pub const IMAGE_TIMELIMIT_HI: C2RustUnnamed_0 = 171;
pub const IMAGE_NOLIMIT_HI: C2RustUnnamed_0 = 170;
pub const IMAGE_FRAGLIMIT: C2RustUnnamed_0 = 169;
pub const IMAGE_TIMELIMIT: C2RustUnnamed_0 = 168;
pub const IMAGE_NOLIMIT: C2RustUnnamed_0 = 167;
pub const IMAGE_LBASE_HI: C2RustUnnamed_0 = 166;
pub const IMAGE_SBASE_HI: C2RustUnnamed_0 = 165;
pub const IMAGE_NOBASE_HI: C2RustUnnamed_0 = 164;
pub const IMAGE_LBASE: C2RustUnnamed_0 = 163;
pub const IMAGE_SBASE: C2RustUnnamed_0 = 162;
pub const IMAGE_NOBASE: C2RustUnnamed_0 = 161;
pub const IMAGE_TECHHI_HI: C2RustUnnamed_0 = 160;
pub const IMAGE_TECHMED_HI: C2RustUnnamed_0 = 159;
pub const IMAGE_TECHLO_HI: C2RustUnnamed_0 = 158;
pub const IMAGE_TECHHI: C2RustUnnamed_0 = 157;
pub const IMAGE_TECHMED: C2RustUnnamed_0 = 156;
pub const IMAGE_TECHLO: C2RustUnnamed_0 = 155;
pub const IMAGE_BIGOK: C2RustUnnamed_0 = 154;
pub const IMAGE_EDIT_GAME: C2RustUnnamed_0 = 153;
pub const IMAGE_EDIT_MAP: C2RustUnnamed_0 = 152;
pub const IMAGE_EDIT_FORCE: C2RustUnnamed_0 = 151;
pub const IMAGE_EDIT_PLAYER: C2RustUnnamed_0 = 150;
pub const IMAGE_RETURN: C2RustUnnamed_0 = 149;
pub const IMAGE_MULTIRANK1: C2RustUnnamed_0 = 148;
pub const IMAGE_POWLO: C2RustUnnamed_0 = 147;
pub const IMAGE_MEDAL_BRONZE: C2RustUnnamed_0 = 146;
pub const IMAGE_MEDAL_SILVER: C2RustUnnamed_0 = 145;
pub const IMAGE_MEDAL_GOLD: C2RustUnnamed_0 = 144;
pub const IMAGE_CAMPAIGN_OVER: C2RustUnnamed_0 = 143;
pub const IMAGE_ARENA_OVER: C2RustUnnamed_0 = 142;
pub const IMAGE_HI64: C2RustUnnamed_0 = 141;
pub const IMAGE_HI41: C2RustUnnamed_0 = 140;
pub const IMAGE_HI39: C2RustUnnamed_0 = 139;
pub const IMAGE_HI23: C2RustUnnamed_0 = 138;
pub const IMAGE_HI31: C2RustUnnamed_0 = 137;
pub const IMAGE_HI34: C2RustUnnamed_0 = 136;
pub const IMAGE_COM4_HI: C2RustUnnamed_0 = 135;
pub const IMAGE_COM3_HI: C2RustUnnamed_0 = 134;
pub const IMAGE_ALLI_HI: C2RustUnnamed_0 = 133;
pub const IMAGE_OFFALLI_HI: C2RustUnnamed_0 = 132;
pub const IMAGE_NOALLI_HI: C2RustUnnamed_0 = 131;
pub const IMAGE_ALLI: C2RustUnnamed_0 = 130;
pub const IMAGE_OFFALLI: C2RustUnnamed_0 = 129;
pub const IMAGE_NOALLI: C2RustUnnamed_0 = 128;
pub const IMAGE_POWHI_HI: C2RustUnnamed_0 = 127;
pub const IMAGE_POWMED_HI: C2RustUnnamed_0 = 126;
pub const IMAGE_POWLO_HI: C2RustUnnamed_0 = 125;
pub const IMAGE_POWHI: C2RustUnnamed_0 = 124;
pub const IMAGE_POWMED: C2RustUnnamed_0 = 123;
pub const IMAGE_OK: C2RustUnnamed_0 = 122;
pub const IMAGE_NO: C2RustUnnamed_0 = 121;
pub const IMAGE_HOST: C2RustUnnamed_0 = 120;
pub const IMAGE_PLAYER7: C2RustUnnamed_0 = 119;
pub const IMAGE_PLAYER6: C2RustUnnamed_0 = 118;
pub const IMAGE_PLAYER5: C2RustUnnamed_0 = 117;
pub const IMAGE_PLAYER4: C2RustUnnamed_0 = 116;
pub const IMAGE_PLAYER3: C2RustUnnamed_0 = 115;
pub const IMAGE_PLAYER2: C2RustUnnamed_0 = 114;
pub const IMAGE_PLAYER1: C2RustUnnamed_0 = 113;
pub const IMAGE_PLAYER0: C2RustUnnamed_0 = 112;
pub const IMAGE_REFRESH: C2RustUnnamed_0 = 111;
pub const IMAGE_CAMPAIGN: C2RustUnnamed_0 = 110;
pub const IMAGE_ARENA: C2RustUnnamed_0 = 109;
pub const IMAGE_115200: C2RustUnnamed_0 = 108;
pub const IMAGE_56000: C2RustUnnamed_0 = 107;
pub const IMAGE_19200: C2RustUnnamed_0 = 106;
pub const IMAGE_14400: C2RustUnnamed_0 = 105;
pub const IMAGE_CAMPAIGN_HI: C2RustUnnamed_0 = 104;
pub const IMAGE_ARENA_HI: C2RustUnnamed_0 = 103;
pub const IMAGE_115200_HI: C2RustUnnamed_0 = 102;
pub const IMAGE_56000_HI: C2RustUnnamed_0 = 101;
pub const IMAGE_19200_HI: C2RustUnnamed_0 = 100;
pub const IMAGE_14400_HI: C2RustUnnamed_0 = 99;
pub const IMAGE_COM2_HI: C2RustUnnamed_0 = 98;
pub const IMAGE_COM1_HI: C2RustUnnamed_0 = 97;
pub const IMAGE_COM4: C2RustUnnamed_0 = 96;
pub const IMAGE_COM3: C2RustUnnamed_0 = 95;
pub const IMAGE_COM2: C2RustUnnamed_0 = 94;
pub const IMAGE_COM1: C2RustUnnamed_0 = 93;
pub const IMAGE_TFONT_126: C2RustUnnamed_0 = 92;
pub const IMAGE_TFONT_125: C2RustUnnamed_0 = 91;
pub const IMAGE_TFONT_124: C2RustUnnamed_0 = 90;
pub const IMAGE_TFONT_123: C2RustUnnamed_0 = 89;
pub const IMAGE_TFONT_122: C2RustUnnamed_0 = 88;
pub const IMAGE_TFONT_121: C2RustUnnamed_0 = 87;
pub const IMAGE_TFONT_120: C2RustUnnamed_0 = 86;
pub const IMAGE_TFONT_119: C2RustUnnamed_0 = 85;
pub const IMAGE_TFONT_118: C2RustUnnamed_0 = 84;
pub const IMAGE_TFONT_117: C2RustUnnamed_0 = 83;
pub const IMAGE_TFONT_116: C2RustUnnamed_0 = 82;
pub const IMAGE_TFONT_115: C2RustUnnamed_0 = 81;
pub const IMAGE_TFONT_114: C2RustUnnamed_0 = 80;
pub const IMAGE_TFONT_113: C2RustUnnamed_0 = 79;
pub const IMAGE_TFONT_112: C2RustUnnamed_0 = 78;
pub const IMAGE_TFONT_111: C2RustUnnamed_0 = 77;
pub const IMAGE_TFONT_110: C2RustUnnamed_0 = 76;
pub const IMAGE_TFONT_109: C2RustUnnamed_0 = 75;
pub const IMAGE_TFONT_108: C2RustUnnamed_0 = 74;
pub const IMAGE_TFONT_107: C2RustUnnamed_0 = 73;
pub const IMAGE_TFONT_106: C2RustUnnamed_0 = 72;
pub const IMAGE_TFONT_105: C2RustUnnamed_0 = 71;
pub const IMAGE_TFONT_104: C2RustUnnamed_0 = 70;
pub const IMAGE_TFONT_103: C2RustUnnamed_0 = 69;
pub const IMAGE_TFONT_102: C2RustUnnamed_0 = 68;
pub const IMAGE_TFONT_101: C2RustUnnamed_0 = 67;
pub const IMAGE_TFONT_100: C2RustUnnamed_0 = 66;
pub const IMAGE_TFONT_99: C2RustUnnamed_0 = 65;
pub const IMAGE_TFONT_98: C2RustUnnamed_0 = 64;
pub const IMAGE_TFONT_97: C2RustUnnamed_0 = 63;
pub const IMAGE_TFONT_96: C2RustUnnamed_0 = 62;
pub const IMAGE_TFONT_95: C2RustUnnamed_0 = 61;
pub const IMAGE_TFONT_94: C2RustUnnamed_0 = 60;
pub const IMAGE_TFONT_93: C2RustUnnamed_0 = 59;
pub const IMAGE_TFONT_92: C2RustUnnamed_0 = 58;
pub const IMAGE_TFONT_91: C2RustUnnamed_0 = 57;
pub const IMAGE_TFONT_90: C2RustUnnamed_0 = 56;
pub const IMAGE_TFONT_89: C2RustUnnamed_0 = 55;
pub const IMAGE_TFONT_88: C2RustUnnamed_0 = 54;
pub const IMAGE_TFONT_87: C2RustUnnamed_0 = 53;
pub const IMAGE_TFONT_86: C2RustUnnamed_0 = 52;
pub const IMAGE_TFONT_85: C2RustUnnamed_0 = 51;
pub const IMAGE_TFONT_84: C2RustUnnamed_0 = 50;
pub const IMAGE_TFONT_83: C2RustUnnamed_0 = 49;
pub const IMAGE_TFONT_82: C2RustUnnamed_0 = 48;
pub const IMAGE_TFONT_81: C2RustUnnamed_0 = 47;
pub const IMAGE_TFONT_80: C2RustUnnamed_0 = 46;
pub const IMAGE_TFONT_79: C2RustUnnamed_0 = 45;
pub const IMAGE_TFONT_78: C2RustUnnamed_0 = 44;
pub const IMAGE_TFONT_77: C2RustUnnamed_0 = 43;
pub const IMAGE_TFONT_76: C2RustUnnamed_0 = 42;
pub const IMAGE_TFONT_75: C2RustUnnamed_0 = 41;
pub const IMAGE_TFONT_74: C2RustUnnamed_0 = 40;
pub const IMAGE_TFONT_73: C2RustUnnamed_0 = 39;
pub const IMAGE_TFONT_72: C2RustUnnamed_0 = 38;
pub const IMAGE_TFONT_71: C2RustUnnamed_0 = 37;
pub const IMAGE_TFONT_70: C2RustUnnamed_0 = 36;
pub const IMAGE_TFONT_69: C2RustUnnamed_0 = 35;
pub const IMAGE_TFONT_68: C2RustUnnamed_0 = 34;
pub const IMAGE_TFONT_67: C2RustUnnamed_0 = 33;
pub const IMAGE_TFONT_66: C2RustUnnamed_0 = 32;
pub const IMAGE_TFONT_65: C2RustUnnamed_0 = 31;
pub const IMAGE_TFONT_64: C2RustUnnamed_0 = 30;
pub const IMAGE_TFONT_62: C2RustUnnamed_0 = 29;
pub const IMAGE_TFONT_61: C2RustUnnamed_0 = 28;
pub const IMAGE_TFONT_60: C2RustUnnamed_0 = 27;
pub const IMAGE_TFONT_59: C2RustUnnamed_0 = 26;
pub const IMAGE_TFONT_58: C2RustUnnamed_0 = 25;
pub const IMAGE_TFONT_57: C2RustUnnamed_0 = 24;
pub const IMAGE_TFONT_56: C2RustUnnamed_0 = 23;
pub const IMAGE_TFONT_55: C2RustUnnamed_0 = 22;
pub const IMAGE_TFONT_54: C2RustUnnamed_0 = 21;
pub const IMAGE_TFONT_53: C2RustUnnamed_0 = 20;
pub const IMAGE_TFONT_52: C2RustUnnamed_0 = 19;
pub const IMAGE_TFONT_51: C2RustUnnamed_0 = 18;
pub const IMAGE_TFONT_50: C2RustUnnamed_0 = 17;
pub const IMAGE_TFONT_49: C2RustUnnamed_0 = 16;
pub const IMAGE_TFONT_48: C2RustUnnamed_0 = 15;
pub const IMAGE_TFONT_47: C2RustUnnamed_0 = 14;
pub const IMAGE_TFONT_46: C2RustUnnamed_0 = 13;
pub const IMAGE_TFONT_44: C2RustUnnamed_0 = 12;
pub const IMAGE_TFONT_43: C2RustUnnamed_0 = 11;
pub const IMAGE_TFONT_42: C2RustUnnamed_0 = 10;
pub const IMAGE_TFONT_41: C2RustUnnamed_0 = 9;
pub const IMAGE_TFONT_40: C2RustUnnamed_0 = 8;
pub const IMAGE_TFONT_39: C2RustUnnamed_0 = 7;
pub const IMAGE_TFONT_38: C2RustUnnamed_0 = 6;
pub const IMAGE_TFONT_37: C2RustUnnamed_0 = 5;
pub const IMAGE_TFONT_36: C2RustUnnamed_0 = 4;
pub const IMAGE_TFONT_35: C2RustUnnamed_0 = 3;
pub const IMAGE_TFONT_34: C2RustUnnamed_0 = 2;
pub const IMAGE_TFONT_33: C2RustUnnamed_0 = 1;
pub const IMAGE_FE_LOGO: C2RustUnnamed_0 = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SESSIONDESC {
    pub dwSize: DWORD,
    pub dwFlags: DWORD,
    pub host: [libc::c_char; 16],
    pub dwMaxPlayers: DWORD,
    pub dwCurrentPlayers: DWORD,
    pub dwUser1: DWORD,
    pub dwUser2: DWORD,
    pub dwUser3: DWORD,
    pub dwUser4: DWORD,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GAMESTRUCT {
    pub name: [libc::c_char; 64],
    pub desc: SESSIONDESC,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NETMSG {
    pub size: libc::c_ushort,
    pub paddedBytes: libc::c_uchar,
    pub type_0: libc::c_uchar,
    pub destination: libc::c_uchar,
    pub body: [libc::c_char; 8000],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PLAYER {
    pub dpid: DPID,
    pub name: [libc::c_char; 64],
    pub bHost: BOOL,
    pub bSpectator: BOOL,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NETPLAY {
    pub games: [GAMESTRUCT; 12],
    pub players: [PLAYER; 8],
    pub playercount: UDWORD,
    pub dpidPlayer: DPID,
    pub bComms: BOOL,
    pub bHost: BOOL,
    pub bLobbyLaunched: BOOL,
    pub bSpectator: BOOL,
    pub bEncryptAllPackets: BOOL,
    pub cryptKey: [UDWORD; 4],
    pub bCaptureInUse: BOOL,
    pub bAllowCaptureRecord: BOOL,
    pub bAllowCapturePlay: BOOL,
}
pub type _msgtype = libc::c_uint;
pub const NET_SET_TEAMS: _msgtype = 50;
pub const NET_BEACONMSG: _msgtype = 49;
pub const NET_TEAMS_ON: _msgtype = 48;
pub const NET_AITEXTMSG: _msgtype = 47;
pub const NET_REQUESTMAP: _msgtype = 46;
pub const NET_LASSAT: _msgtype = 45;
pub const NET_RESEARCHSTATUS: _msgtype = 44;
pub const NET_DROIDDISEMBARK: _msgtype = 43;
pub const NET_DROIDEMBARK: _msgtype = 42;
pub const NET_SECONDARY_ALL: _msgtype = 41;
pub const NET_WHITEBOARD: _msgtype = 40;
pub const NET_VTOLREARM: _msgtype = 39;
pub const NET_VTOL: _msgtype = 38;
pub const NET_DESTROYXTRA: _msgtype = 37;
pub const NET_SCORESUBMIT: _msgtype = 36;
pub const NET_DMATCHWIN: _msgtype = 35;
pub const NET_ARTIFACTS: _msgtype = 34;
pub const NET_COLOURREQUEST: _msgtype = 33;
pub const NET_DEMOLISH: _msgtype = 32;
pub const NET_GIFT: _msgtype = 31;
pub const NET_ALLIANCE: _msgtype = 30;
pub const NET_FIREUP: _msgtype = 29;
pub const NET_SECONDARY: _msgtype = 28;
pub const NET_KICK: _msgtype = 27;
pub const NET_OPTIONS: _msgtype = 26;
pub const NET_PLAYERRESPONDING: _msgtype = 25;
pub const NET_FEATURES: _msgtype = 24;
pub const NET_WHOLEDROID: _msgtype = 23;
pub const NET_STRUCT: _msgtype = 22;
pub const NET_REQUESTPLAYER: _msgtype = 21;
pub const NET_PLAYERCOMPLETE: _msgtype = 20;
pub const NET_REQUESTDROID: _msgtype = 19;
pub const NET_LEAVING: _msgtype = 18;
pub const NET_TEXTMSG: _msgtype = 17;
pub const NET_RESEARCH: _msgtype = 16;
pub const NET_BUILDFINISHED: _msgtype = 15;
pub const NET_STRUCTDEST: _msgtype = 14;
pub const NET_BUILD: _msgtype = 13;
pub const NET_VERSION: _msgtype = 12;
pub const NET_CHECK_POWER: _msgtype = 11;
pub const NET_CHECK_STRUCT: _msgtype = 10;
pub const NET_CHECK_DROID: _msgtype = 9;
pub const NET_PING: _msgtype = 8;
pub const NET_FEATUREDEST: _msgtype = 7;
pub const NET_TEMPLATEDEST: _msgtype = 6;
pub const NET_TEMPLATE: _msgtype = 5;
pub const NET_GROUPORDER: _msgtype = 4;
pub const NET_DROIDMOVE: _msgtype = 3;
pub const NET_DROIDDEST: _msgtype = 2;
pub const NET_DROIDINFO: _msgtype = 1;
pub const NET_DROID: _msgtype = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MULTIPLAYERGAME {
    pub type_0: UBYTE,
    pub map: [STRING; 128],
    pub version: [libc::c_char; 8],
    pub maxPlayers: UBYTE,
    pub name: [STRING; 128],
    pub bComputerPlayers: BOOL,
    pub fog: BOOL,
    pub power: UDWORD,
    pub base: UBYTE,
    pub alliance: UBYTE,
    pub limit: UBYTE,
    pub bytesPerSec: UWORD,
    pub packetsPerSec: UBYTE,
    pub encryptKey: UBYTE,
    pub skDiff: [UBYTE; 8],
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PLAYERSTATS {
    pub played: DWORD,
    pub wins: DWORD,
    pub loses: DWORD,
    pub totalKills: DWORD,
    pub totalScore: SDWORD,
    pub recentKills: DWORD,
    pub recentScore: SDWORD,
    pub killsToAdd: DWORD,
    pub scoreToAdd: SDWORD,
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
// the WRF/WDG files needed for a particular level
// the WRF/WDG files needed for a particular level
pub type LEVEL_DATASET = _level_dataset;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _level_dataset {
    pub type_0: SWORD,
    pub players: SWORD,
    pub game: SWORD,
    pub pName: *mut STRING,
    pub dataDir: libc::c_int,
    pub apDataFiles: [*mut STRING; 9],
    pub psBaseData: *mut _level_dataset,
    pub psChange: *mut _level_dataset,
    pub psNext: *mut _level_dataset,
}
pub type searchPathMode = libc::c_uint;
pub const mod_multiplay: searchPathMode = 2;
pub const mod_campaign: searchPathMode = 1;
pub const mod_clean: searchPathMode = 0;
pub type FORCE_MEMBER = _forcemember;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _forcemember {
    pub pTempl: *mut DROID_TEMPLATE,
    pub psNext: *mut _forcemember,
}
pub type FORCE = _force;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _force {
    pub pForceTemplates: *mut DROID_TEMPLATE,
    pub pMembers: *mut FORCE_MEMBER,
}
#[no_mangle]
pub static mut bHosted: BOOL = 0 as libc::c_int;
// type of map
// number of players for the map
// index of WRF/WDG that loads the scenario file
// title for the level
// title for the level
// the WRF/WDG files for the level
// in load order
// LEVEL_DATASET that must be loaded for this level to load
// LEVEL_DATASET used when changing to this level from another
// pointer to template to use for this droid
// Pointer to next template
//we have set up a game
#[no_mangle]
pub static mut sPlayer: [libc::c_char; 128] = [0; 128];
// player name (to be used)
#[no_mangle]
pub static mut buildTime: [libc::c_char; 8] = [54, 55, 72, 71, 68, 86, 51, 0];
//RODZ was __TIME__ ;
static mut bColourChooserUp: BOOL = 0 as libc::c_int;
static mut SettingsUp: SWORD = 0 as libc::c_int as SWORD;
static mut InitialProto: UBYTE = 0 as libc::c_int as UBYTE;
static mut psConScreen: *mut W_SCREEN = 0 as *const W_SCREEN as *mut W_SCREEN;
static mut dwSelectedGame: DWORD = 0 as libc::c_int;
//player[] and games[] indexes
static mut gameNumber: UDWORD = 0;
// index to games icons
static mut safeSearch: BOOL = 0 as libc::c_int;
//(479-WHITEH)
static mut psWhiteScreen: *mut W_SCREEN =
    0 as *const W_SCREEN as *mut W_SCREEN;
static mut bWhiteBoardUp: BOOL = 0;
#[no_mangle]
pub static mut whiteBoard: [[UWORD; 150]; 8] = [[0; 150]; 8];
static mut curWhite: UBYTE = 0 as libc::c_int as UBYTE;
static mut hideTime: UDWORD = 0 as libc::c_int as UDWORD;
// players (mid) box
//must match what we got now.
#[no_mangle]
pub unsafe extern "C" fn loadMapPreview() {
    let mut aFileName: [STRING; 256] = [0; 256];
    let mut fileSize: UDWORD = 0;
    let mut pFileData: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut psLevel: *mut LEVEL_DATASET = 0 as *mut LEVEL_DATASET;
    //	iBitmap
    let mut i: UDWORD = 0;
    let mut j: UDWORD = 0;
    let mut x: UDWORD = 0;
    let mut y: UDWORD = 0;
    let mut height: UDWORD = 0;
    let mut offX2: UDWORD = 0;
    let mut offY2: UDWORD = 0;
    let mut scale: UBYTE = 0;
    let mut col: UBYTE = 0;
    let mut psTile: *mut MAPTILE = 0 as *mut MAPTILE;
    let mut WTile: *mut MAPTILE = 0 as *mut MAPTILE;
    let mut oursize: UDWORD = 0;
    let mut ptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut imageData: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if !psMapTiles.is_null() {
        mapShutdown();
        //		gwShutDown();
    } //dunno about background color
    levFindDataSet(game.map.as_mut_ptr(), &mut psLevel); // coltab[height/16];
    rebuildSearchPath((*psLevel).dataDir as searchPathMode, 0 as libc::c_int);
    strcpy(aFileName.as_mut_ptr(),
           (*psLevel).apDataFiles[0 as libc::c_int as usize]);
    aFileName[strlen(aFileName.as_mut_ptr()).wrapping_sub(4 as libc::c_int as
                                                              libc::c_uint) as
                  usize] = '\u{0}' as i32 as STRING;
    strcat(aFileName.as_mut_ptr(),
           b"\\game.map\x00" as *const u8 as *const libc::c_char);
    pFileData = DisplayBuffer;
    if loadFileToBuffer(aFileName.as_mut_ptr(), pFileData, displayBufferSize,
                        &mut fileSize) == 0 {
        debug(LOG_NEVER,
              b"loadgame: Fail5\n\x00" as *const u8 as *const libc::c_char);
    }
    if mapLoad(pFileData, fileSize) == 0 {
        debug(LOG_NEVER,
              b"loadgame: Fail7\n\x00" as *const u8 as *const libc::c_char);
        return
    }
    gwShutDown();
    scale = 1 as libc::c_int as UBYTE;
    if mapHeight < 240 as libc::c_int as libc::c_uint &&
           mapWidth < 320 as libc::c_int as libc::c_uint {
        scale = 2 as libc::c_int as UBYTE
    }
    if mapHeight < 120 as libc::c_int as libc::c_uint &&
           mapWidth < 160 as libc::c_int as libc::c_uint {
        scale = 3 as libc::c_int as UBYTE
    }
    if mapHeight < 60 as libc::c_int as libc::c_uint &&
           mapWidth < 80 as libc::c_int as libc::c_uint {
        scale = 4 as libc::c_int as UBYTE
    }
    if mapHeight < 30 as libc::c_int as libc::c_uint &&
           mapWidth < 40 as libc::c_int as libc::c_uint {
        scale = 5 as libc::c_int as UBYTE
    }
    oursize =
        (::std::mem::size_of::<libc::c_uchar>() as
             libc::c_ulong).wrapping_mul(512 as libc::c_int as
                                             libc::c_uint).wrapping_mul(512 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint);
    imageData =
        malloc(oursize.wrapping_mul(3 as libc::c_int as libc::c_uint)) as
            *mut libc::c_uchar;
    ptr = imageData;
    memset(ptr as *mut libc::c_void, 0x45 as libc::c_int,
           (::std::mem::size_of::<libc::c_uchar>() as
                libc::c_ulong).wrapping_mul(512 as libc::c_int as
                                                libc::c_uint).wrapping_mul(512
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_uint).wrapping_mul(3
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              libc::c_uint));
    psTile = psMapTiles;
    offX2 =
        ((512 as libc::c_int / 2 as libc::c_int) as
             libc::c_uint).wrapping_sub((scale as
                                             libc::c_uint).wrapping_mul(mapWidth).wrapping_div(2
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   libc::c_uint));
    offY2 =
        ((512 as libc::c_int / 2 as libc::c_int) as
             libc::c_uint).wrapping_sub((scale as
                                             libc::c_uint).wrapping_mul(mapHeight).wrapping_div(2
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    libc::c_uint));
    i = 0 as libc::c_int as UDWORD;
    while i < mapHeight {
        WTile = psTile;
        j = 0 as libc::c_int as UDWORD;
        while j < mapWidth {
            height = (*WTile).height as UDWORD;
            col = height as UBYTE;
            x = j.wrapping_mul(scale as libc::c_uint);
            while x <
                      j.wrapping_mul(scale as
                                         libc::c_uint).wrapping_add(scale as
                                                                        libc::c_uint)
                  {
                y = i.wrapping_mul(scale as libc::c_uint);
                while y <
                          i.wrapping_mul(scale as
                                             libc::c_uint).wrapping_add(scale
                                                                            as
                                                                            libc::c_uint)
                      {
                    *imageData.offset((3 as libc::c_int as
                                           libc::c_uint).wrapping_mul(offY2.wrapping_add(y).wrapping_mul(512
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             libc::c_uint).wrapping_add(x.wrapping_add(offX2)))
                                          as isize) = col;
                    *imageData.offset((3 as libc::c_int as
                                           libc::c_uint).wrapping_mul(offY2.wrapping_add(y).wrapping_mul(512
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             libc::c_uint).wrapping_add(x.wrapping_add(offX2))).wrapping_add(1
                                                                                                                                                                                 as
                                                                                                                                                                                 libc::c_int
                                                                                                                                                                                 as
                                                                                                                                                                                 libc::c_uint)
                                          as isize) = col;
                    *imageData.offset((3 as libc::c_int as
                                           libc::c_uint).wrapping_mul(offY2.wrapping_add(y).wrapping_mul(512
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             libc::c_uint).wrapping_add(x.wrapping_add(offX2))).wrapping_add(2
                                                                                                                                                                                 as
                                                                                                                                                                                 libc::c_int
                                                                                                                                                                                 as
                                                                                                                                                                                 libc::c_uint)
                                          as isize) = col;
                    y = y.wrapping_add(1)
                }
                x = x.wrapping_add(1)
            }
            WTile = WTile.offset(1 as libc::c_int as isize);
            j = j.wrapping_add(1)
        }
        psTile = psTile.offset(mapWidth as isize);
        i = i.wrapping_add(1)
    }
    plotStructurePreview16(imageData, scale, offX2, offY2);
    //	  Show_Map(imageData);//imageData		//Don't get rid of this yes!
    screen_Upload(imageData as *mut UWORD); //backDropBmp) ;
    free(imageData as *mut libc::c_void);
    hideTime = gameTime;
    mapShutdown();
}
// leave alone for now please -Q
// I know this don't belong here, but I am using this for testing.
#[no_mangle]
pub unsafe extern "C" fn Show_Map(mut imagedata: *mut libc::c_char) {
    let mut Tex: GLuint = 0;
    //		SDL_GL_SwapBuffers();
    let mut image: pie_image =
        pie_image{width: 0,
                  height: 0,
                  channels: 0,
                  data: 0 as *mut libc::c_uchar,};
    image_init(&mut image);
    //	imagetest=malloc((sizeof(char)*512*512*512));
    image_load_from_jpg(&mut image,
                        b"texpages\\bdrops\\test1.jpg\x00" as *const u8 as
                            *const libc::c_char); //image.data);
    glGenTextures(1 as libc::c_int, &mut Tex);
    pie_SetTexturePage(-(1 as libc::c_int));
    glBindTexture(0xde1 as libc::c_int as GLenum, Tex);
    glTexImage2D(0xde1 as libc::c_int as GLenum, 0 as libc::c_int,
                 0x1907 as libc::c_int, 512 as libc::c_int,
                 512 as libc::c_int, 0 as libc::c_int,
                 0x1907 as libc::c_int as GLenum,
                 0x1401 as libc::c_int as GLenum,
                 imagedata as *const libc::c_void);
    glTexEnvf(0x2300 as libc::c_int as GLenum,
              0x2200 as libc::c_int as GLenum,
              0x2100 as libc::c_int as GLfloat);
    glTexParameteri(0xde1 as libc::c_int as GLenum,
                    0x2801 as libc::c_int as GLenum, 0x2601 as libc::c_int);
    glTexParameteri(0xde1 as libc::c_int as GLenum,
                    0x2800 as libc::c_int as GLenum, 0x2601 as libc::c_int);
    glTexParameteri(0xde1 as libc::c_int as GLenum,
                    0x2802 as libc::c_int as GLenum, 0x2900 as libc::c_int);
    glTexParameteri(0xde1 as libc::c_int as GLenum,
                    0x2803 as libc::c_int as GLenum, 0x2900 as libc::c_int);
    image_delete(&mut image);
    //	free(image);
    glDisable(0xb71 as libc::c_int as GLenum);
    glDepthMask(0 as libc::c_int as GLboolean);
    pie_SetTexturePage(-(1 as libc::c_int));
    glEnable(0xde1 as libc::c_int as GLenum);
    glBindTexture(0xde1 as libc::c_int as GLenum, Tex);
    glColor3f(1 as libc::c_int as GLfloat, 1 as libc::c_int as GLfloat,
              1 as libc::c_int as GLfloat);
    glPushMatrix();
    glLoadIdentity();
    //	glTranslatef(0,0,-13);
//	glBegin(GL_QUADS);
//		glVertex3f(-1.0f, 1.0f, 0.0f);				// Top Left
//		glVertex3f( 1.0f, 1.0f, 0.0f);				// Top Right
//	glVertex3f( 1.0f,-1.0f, 0.0f);				// Bottom Right
//		glVertex3f(-1.0f,-1.0f, 0.0f);				// Bottom Left
//		glEnd();
    glBegin(0x6 as libc::c_int as GLenum);
    //glVertex3f(10, -12, 0);
//glVertex3f(10, 12, 0);
//glVertex3f(-10, 12, 0);
//glVertex3f(-10, -12, 0);
//glEnd();
    glTexCoord2f(0 as libc::c_int as GLfloat, 0 as libc::c_int as GLfloat);
    glVertex2f(0 as libc::c_int as GLfloat, 0 as libc::c_int as GLfloat);
    glTexCoord2f(512 as libc::c_int as GLfloat, 0 as libc::c_int as GLfloat);
    glVertex2f(screenWidth.wrapping_mul(2 as libc::c_int as libc::c_uint) as
                   GLfloat, 0 as libc::c_int as GLfloat);
    glTexCoord2f(0 as libc::c_int as GLfloat, 512 as libc::c_int as GLfloat);
    glVertex2f(0 as libc::c_int as GLfloat,
               screenHeight.wrapping_mul(2 as libc::c_int as libc::c_uint) as
                   GLfloat);
    glTexCoord2f(512 as libc::c_int as GLfloat,
                 512 as libc::c_int as GLfloat);
    glVertex2f(screenWidth.wrapping_mul(2 as libc::c_int as libc::c_uint) as
                   GLfloat,
               screenHeight.wrapping_mul(2 as libc::c_int as libc::c_uint) as
                   GLfloat);
    glEnd();
    glPopMatrix();
    glFlush();
}
/*		//leave alone -Q
void displayMapPreview()
{
	UDWORD i,j,height;
	UBYTE col,coltab[16];
	MAPTILE *psTile,*WTile;
	FEATURE *psFeat;

//	build col table;
	for (col=0; col<16; col+=1)
	{
		coltab[col] = pal_GetNearestColour( col*16,col*16, col*16);
	}

	//	for each tile, calc the height and draw a grayscale value 0-16
	psTile = psMapTiles;
	for (i=0; i<mapHeight; i+=2)
	{
		WTile = psTile;
		for (j=0; j<mapWidth; j+=2)	//Wtile is tile considering at i,j.
		{

			height = WTile->height;

			col = coltab[height/16];
//				pal_GetNearestColour( height, height, height);

			pie_BoxFillIndex(j,i,j+2,i+2,col);


			WTile+=2;
		}
		psTile += mapWidth*2;
	}




}
*/
// ////////////////////////////////////////////////////////////////////////////
// helper func
//sets sWRFILE form game.map
unsafe extern "C" fn decideWRF() {
    // try and load it from the maps directory first,
    strcpy(pLevelName.as_mut_ptr(), MultiCustomMapsPath.as_mut_ptr());
    strcat(pLevelName.as_mut_ptr(), game.map.as_mut_ptr());
    strcat(pLevelName.as_mut_ptr(),
           b".wrf\x00" as *const u8 as *const libc::c_char);
    debug(LOG_WZ, b"decideWRF: %s\x00" as *const u8 as *const libc::c_char,
          pLevelName.as_mut_ptr());
    //if the file exists in the downloaded maps dir then use that one instead.
	// FIXME: Try to incorporate this into physfs setup somehow for sane paths
    if PHYSFS_exists(pLevelName.as_mut_ptr()) == 0 {
        strcpy(pLevelName.as_mut_ptr(), game.map.as_mut_ptr());
        // doesn't exist, must be a predefined one.
    };
}
// Connection option functions
// ////////////////////////////////////////////////////////////////////////////
// Connection Options Screen.
unsafe extern "C" fn OptionsInet(mut parentID: UDWORD) -> BOOL 
 //internet options
 {
    let mut sEdInit: W_EDBINIT =
        W_EDBINIT{formID: 0,
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
                  pText: 0 as *mut STRING,
                  FontID: 0,
                  pBoxDisplay: None,
                  pFontDisplay: None,}; //Connection Settings
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
    let mut sLabInit: W_LABINIT =
        W_LABINIT{formID: 0,
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
                  pText: 0 as *mut STRING,
                  pTip: 0 as *mut STRING,
                  FontID: 0,};
    if ingame.bHostSetup != 0 {
        SettingsUp = -(1 as libc::c_int) as SWORD;
        return 1 as libc::c_int
    }
    widgCreateScreen(&mut psConScreen);
    widgSetTipFont(psConScreen, WFont);
    memset(&mut sFormInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_FORMINIT>() as libc::c_ulong);
    sFormInit.formID = 0 as libc::c_int as UDWORD;
    sFormInit.id = 10130 as libc::c_int as UDWORD;
    sFormInit.style = 0 as libc::c_int as UDWORD;
    sFormInit.x = 220 as libc::c_int as SWORD;
    sFormInit.y = 190 as libc::c_int as SWORD;
    sFormInit.width = 200 as libc::c_int as UWORD;
    sFormInit.height = 100 as libc::c_int as UWORD;
    sFormInit.pDisplay =
        Some(intDisplayFeBox as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    widgAddForm(psConScreen, &mut sFormInit);
    addMultiBut(psConScreen, 10130 as libc::c_int as UDWORD,
                10101 as libc::c_int as UDWORD,
                (200 as libc::c_int - 37 as libc::c_int - 3 as libc::c_int) as
                    UDWORD,
                (100 as libc::c_int - 24 as libc::c_int - 3 as libc::c_int) as
                    UDWORD, 37 as libc::c_int as UDWORD,
                24 as libc::c_int as UDWORD,
                STR_MUL_OK as libc::c_int as UDWORD,
                IMAGE_OK as libc::c_int as UDWORD,
                IMAGE_OK as libc::c_int as UDWORD, 1 as libc::c_int);
    //label.
    memset(&mut sLabInit as *mut W_LABINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_LABINIT>() as libc::c_ulong); // address
    sLabInit.formID =
        10130 as libc::c_int as
            UDWORD; //strresGetString(psStringRes, STR_MUL_IPADDR);
    sLabInit.id = 10131 as libc::c_int as UDWORD;
    sLabInit.style = 2 as libc::c_int as UDWORD;
    sLabInit.x = 0 as libc::c_int as SWORD;
    sLabInit.y = 10 as libc::c_int as SWORD;
    sLabInit.width = 200 as libc::c_int as UWORD;
    sLabInit.height = 20 as libc::c_int as UWORD;
    sLabInit.pText =
        strresGetString(psStringRes, STR_MUL_IPADDR as libc::c_int as UDWORD);
    sLabInit.FontID = WFont;
    widgAddLabel(psConScreen, &mut sLabInit);
    memset(&mut sEdInit as *mut W_EDBINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_EDBINIT>() as libc::c_ulong);
    sEdInit.formID = 10130 as libc::c_int as UDWORD;
    sEdInit.id = 10133 as libc::c_int as UDWORD;
    sEdInit.style = 0 as libc::c_int as UDWORD;
    sEdInit.x = 20 as libc::c_int as SWORD;
    sEdInit.y = 45 as libc::c_int as SWORD;
    sEdInit.width = (200 as libc::c_int - 20 as libc::c_int) as UWORD;
    sEdInit.height = 15 as libc::c_int as UWORD;
    sEdInit.pText =
        b"\x00" as *const u8 as *const libc::c_char as *mut STRING;
    sEdInit.FontID = WFont;
    //	sEdInit.pUserData = (void*)PACKDWORD_TRI(0,IMAGE_DES_EDITBOXLEFTH , IMAGE_DES_EDITBOXLEFT);
//	sEdInit.pBoxDisplay = intDisplayButtonHilight;
    sEdInit.pBoxDisplay =
        Some(intDisplayEditBox as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    if widgAddEditBox(psConScreen, &mut sEdInit) == 0 {
        return 0 as libc::c_int
    }
    SettingsUp = 1 as libc::c_int as SWORD;
    return 1 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////////
// Draw the connections screen.
#[no_mangle]
pub unsafe extern "C" fn startConnectionScreen() -> BOOL {
    addBackdrop(); //background
    addTopForm(); // logo
    addBottomForm(); // don't pretend!!
    SettingsUp = 0 as libc::c_int as SWORD; // goback buttpn levels
    InitialProto = 0 as libc::c_int as UBYTE;
    safeSearch = 0 as libc::c_int;
    NETuseNetwork(1 as libc::c_int);
    addSideText(20026 as libc::c_int as UDWORD, 44 as libc::c_int as UDWORD,
                170 as libc::c_int as UDWORD,
                strresGetString(psStringRes,
                                STR_MUL_SIDECONNECTION as libc::c_int as
                                    UDWORD));
    addMultiBut(psWScreen, 20002 as libc::c_int as UDWORD,
                10102 as libc::c_int as UDWORD, 10 as libc::c_int as UDWORD,
                10 as libc::c_int as UDWORD, 37 as libc::c_int as UDWORD,
                24 as libc::c_int as UDWORD,
                STR_MUL_CANCEL as libc::c_int as UDWORD,
                IMAGE_RETURN as libc::c_int as UDWORD,
                IMAGE_RETURN_HI as libc::c_int as UDWORD, 1 as libc::c_int);
    addConnections(0 as libc::c_int as UDWORD);
    return 1 as libc::c_int;
}
// add connections
unsafe extern "C" fn addConnections(mut begin: UDWORD) {
    let mut pos: UDWORD = 50 as libc::c_int as UDWORD;
    addTextButton((10105 as libc::c_int as libc::c_uint).wrapping_add(begin),
                  20 as libc::c_int as UDWORD, pos,
                  strresGetString(psStringRes,
                                  STR_CON_INTERNET as libc::c_int as UDWORD),
                  0 as libc::c_int, 0 as libc::c_int);
}
/*
 *multiint.h
 * Interface defines/externs for warzone frontend.
 * Alex Lee, pumpkin Studios.
 */
#[no_mangle]
pub unsafe extern "C" fn runConnectionScreen() {
    let mut id: UDWORD = 0;
    static mut chosenproto: UDWORD = 0;
    static mut addr: [libc::c_char; 128] = [0; 128];
    let mut finalconnection: LPVOID = 0 as *mut libc::c_void;
    processFrontendSnap(1 as libc::c_int);
    if SettingsUp as libc::c_int == 1 as libc::c_int {
        id = widgRunScreen(psConScreen)
        // Run the current set of widgets
    } else {
        id = widgRunScreen(psWScreen)
        // Run the current set of widgets
    }
    if id == 10102 as libc::c_int as libc::c_uint {
        //cancel
        changeTitleMode(MULTI); // goback buttpn levels
        bMultiPlayer = 0 as libc::c_int
    }
    if id == 10129 as libc::c_int as libc::c_uint {
        widgDelete(psWScreen, 20002 as libc::c_int as UDWORD);
        SettingsUp = 0 as libc::c_int as SWORD;
        InitialProto =
            (InitialProto as libc::c_int + 5 as libc::c_int) as UBYTE;
        addBottomForm();
        addMultiBut(psWScreen, 20002 as libc::c_int as UDWORD,
                    10102 as libc::c_int as UDWORD,
                    10 as libc::c_int as UDWORD, 10 as libc::c_int as UDWORD,
                    37 as libc::c_int as UDWORD, 24 as libc::c_int as UDWORD,
                    STR_MUL_CANCEL as libc::c_int as UDWORD,
                    IMAGE_RETURN as libc::c_int as UDWORD,
                    IMAGE_RETURN_HI as libc::c_int as UDWORD,
                    1 as libc::c_int);
        addConnections(InitialProto as UDWORD);
    }
    if SettingsUp as libc::c_int == 0 as libc::c_int &&
           id >= 10105 as libc::c_int as libc::c_uint &&
           id <= 10128 as libc::c_int as libc::c_uint {
        /* RODZ
		if (IsEqualGUID(&(NetPlay.protocols[id-CON_TYPESID_START].guid), &DPSPGUID_MODEM))
		{
			chosenproto =1;
			OptionsModem(id);
		}

		else if (IsEqualGUID(&(NetPlay.protocols[id-CON_TYPESID_START].guid), &DPSPGUID_TCPIP))
		{
		*/
        chosenproto = 2 as libc::c_int as UDWORD;
        OptionsInet(id);
        /* RODZ
		}

		else if (IsEqualGUID(&(NetPlay.protocols[id-CON_TYPESID_START].guid), &DPSPGUID_IPX))
		{
			chosenproto =3;
			OptionsIPX(id);
		}

		else if (IsEqualGUID(&(NetPlay.protocols[id-CON_TYPESID_START].guid), &DPSPGUID_SERIAL))
		{
			chosenproto =4;
			baud = 19200;
			com  = 1;
			OptionsCable(id);
			widgSetButtonState(psConScreen, CON_COM1,WBUT_LOCK);
			widgSetButtonState(psConScreen, CON_19200,WBUT_LOCK);
		}

		else if(IsEqualGUID(&(NetPlay.protocols[id-CON_TYPESID_START].guid), &SPGUID_MPLAYER) ) // mplayer
		{
			if(system("multiplay\\MplayNow\\mplaynow.exe") != -1) 		// launch gizmo, if present. If not, tough...
			{
				changeTitleMode(QUIT);									// shut down warzone...
			}
		}
		else if( strncmp(NetPlay.protocols[id-CON_TYPESID_START].name,"Simulator For",12) == 0)	// DIRECTPLAY 6 TEST MODE
		{
			OptionsUnknown(id);
			chosenproto =5;
		}
		else
		{
//  comment to allow no other connectionmethod (+below)
			OptionsUnknown(id);
			finalconnection = NetPlay.protocols[id-CON_TYPESID_START].connection;
		}
		*/
    }
    match id {
        10133 => {
            // settings buttons
            // ip entered
            strcpy(addr.as_mut_ptr(),
                   widgGetString(psConScreen,
                                 10133 as libc::c_int as UDWORD));
        }
        _ => { }
    }
    if id == 10101 as libc::c_int as libc::c_uint ||
           SettingsUp as libc::c_int == -(1 as libc::c_int) {
        if SettingsUp as libc::c_int == 1 as libc::c_int {
            widgReleaseScreen(psConScreen);
            SettingsUp = 0 as libc::c_int as SWORD
        }
        match chosenproto {
            2 => {
                game.bytesPerSec = 1201 as libc::c_int as UWORD;
                game.packetsPerSec = 5 as libc::c_int as UBYTE;
                /* RODZ
			if(chosenproto==1 || chosenproto==2 || chosenproto==4)		// this hack fixes the
			{											// memory leak in netplay
				FREE(finalconnection);					// cant do it in the lib, since requires protochosen!
			}
		}
		else
		{
			DBPRINTF(("Protocol Init Failed."));
		}
*/
                //			return;	//dont work on anything else!
                NETsetupTCPIP(&mut finalconnection, addr.as_mut_ptr()); //inet
            }
            _ => {
                /* RODZ
		case 3:												//ipx
			game.bytesPerSec			= IPXBYTESPERSEC;
			game.packetsPerSec			= IPXPACKETS;
			safeSearch = TRUE;
			for(i=0;
				i<MaxProtocols && !IsEqualGUID(&(NetPlay.protocols[i].guid), &DPSPGUID_IPX);
				i++);
			finalconnection = NetPlay.protocols[i].connection;
			break;
		case 4:												//cable
			game.bytesPerSec			= CABLEBYTESPERSEC;
			game.packetsPerSec			= CABLEPACKETS;
			NETsetupSerial(&finalconnection,com,baud,ONESTOPBIT,NOPARITY,DPCPA_RTSFLOW);
			break;
		case 5:												// dplay6 tester.
			game.bytesPerSec			= INETBYTESPERSEC;
			game.packetsPerSec			= INETPACKETS;
			for(i=0;
					i<MaxProtocols
					&& strncmp(NetPlay.protocols[id-CON_TYPESID_START].name,"Simulator For",12) != 0;
				i++);
			finalconnection = NetPlay.protocols[i].connection;
			break;

		*/
                game.bytesPerSec =
                    1000 as libc::c_int as
                        UWORD; // possibly a lobby, so default.
                game.packetsPerSec = 5 as libc::c_int as UBYTE
            }
        }
        if ingame.bHostSetup != 0 {
            changeTitleMode(MULTIOPTION);
        } else { changeTitleMode(GAMEFIND); }
    }
    StartCursorSnap(&mut InterfaceSnap);
    DrawBegin();
    /*
		if(NETselectProtocol(finalconnection))			// start the connection.
		{
*/
    widgDisplayScreen(psWScreen); // show the widgets currently running
    if SettingsUp as libc::c_int == 1 as libc::c_int {
        widgDisplayScreen(psConScreen);
        // show the widgets currently running
    }
    DrawEnd();
}
//added to show map -Q
// find games
// ////////////////////////////////////////////////////////////////////////////
// Game Chooser Screen.
unsafe extern "C" fn addGames() {
    let mut i: UDWORD = 0;
    let mut gcount: UDWORD = 0 as libc::c_int as UDWORD;
    let mut sButInit: W_BUTINIT =
        W_BUTINIT{formID: 0,
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
                  pText: 0 as *mut STRING,
                  pTip: 0 as *mut STRING,
                  FontID: 0,};
    //count games to see if need two columns.
    i = 0 as libc::c_int as UDWORD;
    while i < 12 as libc::c_int as libc::c_uint {
        // draw games
        if NetPlay.games[i as usize].desc.dwSize != 0 as libc::c_int {
            gcount = gcount.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    }
    memset(&mut sButInit as *mut W_BUTINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_BUTINIT>() as libc::c_ulong);
    sButInit.formID = 20002 as libc::c_int as UDWORD;
    sButInit.style = 0 as libc::c_int as UDWORD;
    sButInit.width = 225 as libc::c_int as UWORD;
    sButInit.height = 40 as libc::c_int as UWORD;
    sButInit.FontID = WFont;
    sButInit.pDisplay =
        Some(displayRemoteGame as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    i = 0 as libc::c_int as UDWORD;
    while i < 12 as libc::c_int as libc::c_uint {
        // draw games
        widgDelete(psWScreen,
                   (10201 as libc::c_int as
                        libc::c_uint).wrapping_add(i)); // remove old icon.
        if NetPlay.games[i as usize].desc.dwSize != 0 as libc::c_int {
            sButInit.id =
                (10201 as libc::c_int as libc::c_uint).wrapping_add(i);
            if gcount < 6 as libc::c_int as libc::c_uint {
                // only center column needed.
                sButInit.x = 125 as libc::c_int as SWORD;
                sButInit.y =
                    (30 as libc::c_int as
                         libc::c_uint).wrapping_add(((5 as libc::c_int +
                                                          40 as libc::c_int)
                                                         as
                                                         libc::c_uint).wrapping_mul(i))
                        as SHORT
            } else if i < 6 as libc::c_int as libc::c_uint {
                //column 1
                sButInit.x = 10 as libc::c_int as SWORD;
                sButInit.y =
                    (30 as libc::c_int as
                         libc::c_uint).wrapping_add(((5 as libc::c_int +
                                                          40 as libc::c_int)
                                                         as
                                                         libc::c_uint).wrapping_mul(i))
                        as SHORT
            } else {
                //column 2
                sButInit.x =
                    (20 as libc::c_int + 225 as libc::c_int) as SWORD;
                sButInit.y =
                    (30 as libc::c_int as
                         libc::c_uint).wrapping_add(((5 as libc::c_int +
                                                          40 as libc::c_int)
                                                         as
                                                         libc::c_uint).wrapping_mul(i.wrapping_sub(6
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_uint)))
                        as SHORT
            }
            sButInit.pTip = NetPlay.games[i as usize].name.as_mut_ptr();
            sButInit.pUserData = i as *mut libc::c_void;
            widgAddButton(psWScreen, &mut sButInit);
        }
        i = i.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn runGameFind() {
    let mut id: UDWORD = 0;
    static mut lastupdate: UDWORD = 0 as libc::c_int as UDWORD;
    if lastupdate > gameTime { lastupdate = 0 as libc::c_int as UDWORD }
    if gameTime.wrapping_sub(lastupdate) > 6000 as libc::c_int as libc::c_uint
       {
        lastupdate = gameTime;
        if safeSearch != 0 {
            NETfindGame(1 as libc::c_int);
            //redraw list
            // find games asynchronously
        } // Run the current set of widgets
        addGames();
    }
    processFrontendSnap(0 as libc::c_int);
    id = widgRunScreen(psWScreen);
    if id == 10102 as libc::c_int as libc::c_uint {
        // ok
        changeTitleMode(PROTOCOL); // find games asynchronously
    }
    if id == 10275 as libc::c_int as libc::c_uint {
        NETfindGame(1 as libc::c_int);
        addGames();
        //redraw list.
    }
    if id >= 10201 as libc::c_int as libc::c_uint &&
           id <= (10201 as libc::c_int + 20 as libc::c_int) as libc::c_uint {
        gameNumber = id.wrapping_sub(10201 as libc::c_int as libc::c_uint);
        if NetPlay.games[gameNumber as usize].desc.dwCurrentPlayers <
               NetPlay.games[gameNumber as usize].desc.dwMaxPlayers &&
               NetPlay.games[gameNumber as usize].desc.dwFlags &
                   1 as libc::c_int == 0 {
            // if still joinable
            // if skirmish, check it wont take the last slot
            if !(NETgetGameFlagsUnjoined(gameNumber,
                                         1 as libc::c_int as UDWORD) ==
                     14 as libc::c_int &&
                     NetPlay.games[gameNumber as usize].desc.dwCurrentPlayers
                         >=
                         NetPlay.games[gameNumber as usize].desc.dwMaxPlayers
                             - 1 as libc::c_int) {
                ingame.localOptionsReceived =
                    0 as libc::c_int; // note we are awaiting options
                strcpy(game.name.as_mut_ptr(),
                       NetPlay.games[gameNumber as
                                         usize].name.as_mut_ptr()); // store name
                //			strcpy(sPlayer,"LastUsed");
//			loadMultiStats(sPlayer,&nullStats);
//			if(NETgetGameFlagsUnjoined(gameNumber,1) == DMATCH)
//			{
//				joinArena(gameNumber,(STRING*)sPlayer);
//			}
//			else
//			{
                joinCampaign(gameNumber, sPlayer.as_mut_ptr() as *mut STRING);
                //			}
                changeTitleMode(MULTIOPTION); // show the widgets currently running
            }
        }
    } //background
    StartCursorSnap(&mut InterfaceSnap); // logo
    DrawBegin();
    widgDisplayScreen(psWScreen);
    if safeSearch != 0 {
        iV_SetFont(FEFont);
        pie_DrawText(strresGetString(psStringRes,
                                     STR_MUL_SEARCHING as libc::c_int as
                                         UDWORD),
                     pie_GetVideoBufferWidth().wrapping_sub(640 as libc::c_int
                                                                as
                                                                libc::c_uint).wrapping_div(2
                                                                                               as
                                                                                               libc::c_int
                                                                                               as
                                                                                               libc::c_uint).wrapping_add(260
                                                                                                                              as
                                                                                                                              libc::c_int
                                                                                                                              as
                                                                                                                              libc::c_uint),
                     pie_GetVideoBufferHeight().wrapping_sub(480 as
                                                                 libc::c_int
                                                                 as
                                                                 libc::c_uint).wrapping_div(2
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                libc::c_uint).wrapping_add(460
                                                                                                                               as
                                                                                                                               libc::c_int
                                                                                                                               as
                                                                                                                               libc::c_uint));
    }
    DrawEnd();
}
#[no_mangle]
pub unsafe extern "C" fn startGameFind() {
    addBackdrop();
    addTopForm();
    addBottomForm();
    addSideText(20026 as libc::c_int as UDWORD, 44 as libc::c_int as UDWORD,
                170 as libc::c_int as UDWORD,
                strresGetString(psStringRes,
                                STR_GAMES_GAMES as libc::c_int as UDWORD));
    // cancel
    addMultiBut(psWScreen, 20002 as libc::c_int as UDWORD,
                10102 as libc::c_int as UDWORD, 10 as libc::c_int as UDWORD,
                5 as libc::c_int as UDWORD, 37 as libc::c_int as UDWORD,
                24 as libc::c_int as UDWORD,
                STR_MUL_CANCEL as libc::c_int as UDWORD,
                IMAGE_RETURN as libc::c_int as UDWORD,
                IMAGE_RETURN_HI as libc::c_int as UDWORD, 1 as libc::c_int);
    if safeSearch == 0 {
        //refresh
        addMultiBut(psWScreen, 20002 as libc::c_int as UDWORD,
                    10275 as libc::c_int as UDWORD,
                    (480 as libc::c_int - 37 as libc::c_int -
                         5 as libc::c_int) as UDWORD,
                    5 as libc::c_int as UDWORD, 37 as libc::c_int as UDWORD,
                    24 as libc::c_int as UDWORD,
                    STR_MUL_REFRESH as libc::c_int as UDWORD,
                    IMAGE_REFRESH as libc::c_int as UDWORD,
                    IMAGE_REFRESH as libc::c_int as UDWORD, 0 as libc::c_int);
        // Find Games button
    }
    NETfindGame(1 as libc::c_int);
    addGames();
    // now add games.
}
// ////////////////////////////////////////////////////////////////////////////
// Game Options Screen.
// ////////////////////////////////////////////////////////////////////////////
unsafe extern "C" fn addBlueForm(mut parent: UDWORD, mut id: UDWORD,
                                 mut txt: *mut STRING, mut x: UDWORD,
                                 mut y: UDWORD, mut w: UDWORD,
                                 mut h: UDWORD) {
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
                   pFormDisplay: None,}; // draw options box.
    let mut sLabInit: W_LABINIT =
        W_LABINIT{formID: 0,
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
                  pText: 0 as *mut STRING,
                  pTip: 0 as *mut STRING,
                  FontID: 0,}; //190;
    memset(&mut sFormInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_FORMINIT>() as libc::c_ulong); //27;
    sFormInit.formID = parent;
    sFormInit.id = id;
    sFormInit.x = x as UWORD as SWORD;
    sFormInit.y = y as UWORD as SWORD;
    sFormInit.style = 0 as libc::c_int as UDWORD;
    sFormInit.width = w as UWORD;
    sFormInit.height = h as UWORD;
    sFormInit.pDisplay =
        Some(intDisplayFeBox as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    widgAddForm(psWScreen, &mut sFormInit);
    if strlen(txt) > 0 as libc::c_int as libc::c_uint {
        memset(&mut sLabInit as *mut W_LABINIT as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<W_LABINIT>() as libc::c_ulong);
        sLabInit.formID = id;
        sLabInit.id = id.wrapping_add(1 as libc::c_int as libc::c_uint);
        sLabInit.style = 0 as libc::c_int as UDWORD;
        sLabInit.x = 3 as libc::c_int as SWORD;
        sLabInit.y = 4 as libc::c_int as SWORD;
        sLabInit.width = 80 as libc::c_int as UWORD;
        sLabInit.height = 20 as libc::c_int as UWORD;
        sLabInit.pText = txt;
        //		sLabInit.pDisplay = displayFeText;
        sLabInit.FontID = WFont;
        widgAddLabel(psWScreen, &mut sLabInit);
    };
}
unsafe extern "C" fn addGameOptions(mut bRedo: BOOL) {
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
    if bRedo != 0 {
        // stop any editing going on
        screenClearFocus(psWScreen); // clear options list
    } // del text..
    widgDelete(psWScreen,
               10250 as libc::c_int as UDWORD); // draw options box.
    widgDelete(psWScreen, 20029 as libc::c_int as UDWORD);
    iV_SetFont(WFont);
    memset(&mut sFormInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_FORMINIT>() as libc::c_ulong);
    sFormInit.formID = 20000 as libc::c_int as UDWORD;
    sFormInit.id = 10250 as libc::c_int as UDWORD;
    sFormInit.x = 40 as libc::c_int as SWORD;
    sFormInit.y = 15 as libc::c_int as SWORD;
    sFormInit.style = 0 as libc::c_int as UDWORD;
    sFormInit.width = 290 as libc::c_int as UWORD;
    sFormInit.height = 330 as libc::c_int as UWORD;
    sFormInit.pDisplay =
        Some(intDisplayPlainForm as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    widgAddForm(psWScreen, &mut sFormInit);
    addSideText(20029 as libc::c_int as UDWORD,
                (40 as libc::c_int - 3 as libc::c_int) as UDWORD,
                15 as libc::c_int as UDWORD,
                strresGetString(psStringRes,
                                STR_MUL_SIDEOPTIONS as libc::c_int as
                                    UDWORD));
    addMultiEditBox(10250 as libc::c_int as UDWORD,
                    10255 as libc::c_int as UDWORD,
                    50 as libc::c_int as UDWORD,
                    (4 as libc::c_int + 30 as libc::c_int + 4 as libc::c_int)
                        as UDWORD, STR_MUL_GAMEIC as libc::c_int as UDWORD,
                    game.name.as_mut_ptr(),
                    IMAGE_EDIT_GAME as libc::c_int as UDWORD,
                    10254 as libc::c_int as UDWORD);
    addMultiEditBox(10250 as libc::c_int as UDWORD,
                    10259 as libc::c_int as UDWORD,
                    50 as libc::c_int as UDWORD,
                    (4 as libc::c_int + 30 as libc::c_int + 4 as libc::c_int +
                         30 as libc::c_int + 4 as libc::c_int) as UDWORD,
                    STR_MUL_MAPIC as libc::c_int as UDWORD,
                    game.map.as_mut_ptr(),
                    IMAGE_EDIT_MAP as libc::c_int as UDWORD,
                    10258 as libc::c_int as UDWORD);
    // buttons.
    // game type
    addBlueForm(10250 as libc::c_int as UDWORD,
                10294 as libc::c_int as UDWORD,
                strresGetString(psStringRes,
                                STR_LABEL_TYPE as libc::c_int as UDWORD),
                50 as libc::c_int as UDWORD,
                (4 as libc::c_int + 30 as libc::c_int + 4 as libc::c_int +
                     30 as libc::c_int + 4 as libc::c_int + 30 as libc::c_int
                     + 4 as libc::c_int + 38 as libc::c_int) as UDWORD,
                226 as libc::c_int as UDWORD, 27 as libc::c_int as UDWORD);
    //	addMultiBut(psWScreen,MULTIOP_GAMETYPE,MULTIOP_ARENA,	MCOL1, 2 , MULTIOP_BUTW,MULTIOP_BUTH,
//				STR_MUL_ARENA,   IMAGE_ARENA,   IMAGE_ARENA_HI,TRUE);		//arena
    addMultiBut(psWScreen, 10294 as libc::c_int as UDWORD,
                10261 as libc::c_int as UDWORD,
                (50 as libc::c_int + 26 as libc::c_int + 10 as libc::c_int) as
                    UDWORD, 2 as libc::c_int as UDWORD,
                35 as libc::c_int as UDWORD, 24 as libc::c_int as UDWORD,
                STR_MUL_CAMPAIGN as libc::c_int as UDWORD,
                IMAGE_CAMPAIGN as libc::c_int as UDWORD,
                IMAGE_CAMPAIGN_HI as libc::c_int as UDWORD,
                1 as libc::c_int); //camp
    addMultiBut(psWScreen, 10294 as libc::c_int as UDWORD,
                10262 as libc::c_int as UDWORD,
                (50 as libc::c_int + 26 as libc::c_int + 10 as libc::c_int +
                     38 as libc::c_int) as UDWORD, 2 as libc::c_int as UDWORD,
                35 as libc::c_int as UDWORD, 24 as libc::c_int as UDWORD,
                STR_MUL_TEAMPLAY as libc::c_int as UDWORD,
                IMAGE_TEAM as libc::c_int as UDWORD,
                IMAGE_TEAM_HI as libc::c_int as UDWORD,
                1 as libc::c_int); //teamplay
    addMultiBut(psWScreen, 10294 as libc::c_int as UDWORD,
                10263 as libc::c_int as UDWORD,
                (50 as libc::c_int + 26 as libc::c_int + 10 as libc::c_int +
                     38 as libc::c_int + 38 as libc::c_int) as UDWORD,
                2 as libc::c_int as UDWORD, 35 as libc::c_int as UDWORD,
                24 as libc::c_int as UDWORD,
                STR_MUL_SKIRMISH as libc::c_int as UDWORD,
                IMAGE_SKIRMISH as libc::c_int as UDWORD,
                IMAGE_SKIRMISH_HI as libc::c_int as UDWORD,
                1 as libc::c_int); //skirmish
    //	widgSetButtonState(psWScreen, MULTIOP_ARENA,	0);
    widgSetButtonState(psWScreen, 10261 as libc::c_int as UDWORD,
                       0 as libc::c_int as UDWORD);
    widgSetButtonState(psWScreen, 10262 as libc::c_int as UDWORD,
                       0 as libc::c_int as UDWORD);
    widgSetButtonState(psWScreen, 10263 as libc::c_int as UDWORD,
                       0 as libc::c_int as UDWORD);
    match game.type_0 as libc::c_int {
        12 => {
            //	case DMATCH:
//		widgSetButtonState(psWScreen,MULTIOP_ARENA,WBUT_LOCK);
//		break;
            widgSetButtonState(psWScreen, 10261 as libc::c_int as UDWORD,
                               0x2 as libc::c_int as UDWORD);
        }
        13 => {
            widgSetButtonState(psWScreen, 10262 as libc::c_int as UDWORD,
                               0x2 as libc::c_int as UDWORD);
        }
        14 => {
            widgSetButtonState(psWScreen, 10263 as libc::c_int as UDWORD,
                               0x2 as libc::c_int as UDWORD);
        }
        _ => { }
    }
    if NetPlay.bComms == 0 {
        widgSetButtonState(psWScreen, 10261 as libc::c_int as UDWORD,
                           0x1 as libc::c_int as UDWORD);
        //		widgSetButtonState(psWScreen, MULTIOP_ARENA,	WBUT_DISABLE);
        widgSetButtonState(psWScreen, 10262 as libc::c_int as UDWORD,
                           0x1 as libc::c_int as UDWORD);
    }
    //just display the game options.
    addMultiEditBox(10250 as libc::c_int as UDWORD,
                    10253 as libc::c_int as UDWORD,
                    50 as libc::c_int as UDWORD, 4 as libc::c_int as UDWORD,
                    STR_MUL_PLAYERIC as libc::c_int as UDWORD,
                    sPlayer.as_mut_ptr() as *mut STRING,
                    IMAGE_EDIT_PLAYER as libc::c_int as UDWORD,
                    10252 as libc::c_int as UDWORD);
    addMultiEditBox(10250 as libc::c_int as UDWORD,
                    10257 as libc::c_int as UDWORD,
                    50 as libc::c_int as UDWORD,
                    (4 as libc::c_int + 30 as libc::c_int + 4 as libc::c_int +
                         30 as libc::c_int + 4 as libc::c_int +
                         30 as libc::c_int + 4 as libc::c_int) as UDWORD,
                    STR_MUL_FORCEIC as libc::c_int as UDWORD,
                    sForceName.as_mut_ptr(),
                    IMAGE_EDIT_FORCE as libc::c_int as UDWORD,
                    10256 as libc::c_int as UDWORD);
    // Fog type
    addBlueForm(10250 as libc::c_int as UDWORD,
                10306 as libc::c_int as UDWORD,
                strresGetString(psStringRes,
                                STR_LABEL_FOG as libc::c_int as UDWORD),
                50 as libc::c_int as UDWORD,
                (4 as libc::c_int + 30 as libc::c_int + 4 as libc::c_int +
                     30 as libc::c_int + 4 as libc::c_int + 30 as libc::c_int
                     + 4 as libc::c_int + 38 as libc::c_int +
                     31 as libc::c_int) as UDWORD,
                226 as libc::c_int as UDWORD,
                27 as libc::c_int as UDWORD); //black stuff
    addMultiBut(psWScreen, 10306 as libc::c_int as UDWORD,
                10310 as libc::c_int as UDWORD,
                (50 as libc::c_int + 26 as libc::c_int + 10 as libc::c_int) as
                    UDWORD, 2 as libc::c_int as UDWORD,
                35 as libc::c_int as UDWORD, 24 as libc::c_int as UDWORD,
                STR_MUL_FOG_ON as libc::c_int as UDWORD,
                IMAGE_FOG_OFF as libc::c_int as UDWORD,
                IMAGE_FOG_OFF_HI as libc::c_int as UDWORD, 1 as libc::c_int);
    addMultiBut(psWScreen, 10306 as libc::c_int as UDWORD,
                10311 as libc::c_int as UDWORD,
                (50 as libc::c_int + 26 as libc::c_int + 10 as libc::c_int +
                     38 as libc::c_int) as UDWORD, 2 as libc::c_int as UDWORD,
                35 as libc::c_int as UDWORD, 24 as libc::c_int as UDWORD,
                STR_MUL_FOG_OFF as libc::c_int as UDWORD,
                IMAGE_FOG_ON as libc::c_int as UDWORD,
                IMAGE_FOG_ON_HI as libc::c_int as UDWORD, 1 as libc::c_int);
    if game.fog != 0 {
        widgSetButtonState(psWScreen, 10310 as libc::c_int as UDWORD,
                           0x2 as libc::c_int as UDWORD);
    } else {
        widgSetButtonState(psWScreen, 10311 as libc::c_int as UDWORD,
                           0x2 as libc::c_int as UDWORD);
    }
    if game.type_0 as libc::c_int != 13 as libc::c_int {
        // alliances
//		if(game.type == DMATCH)
//		{
//			addBlueForm(MULTIOP_OPTIONS,MULTIOP_ALLIANCES,strresGetString(psStringRes,STR_LABEL_ALLI),MCOL0,MROW7,MULTIOP_BLUEFORMW,27);
//		}
//		else
//		{
        addBlueForm(10250 as libc::c_int as UDWORD,
                    10298 as libc::c_int as UDWORD,
                    strresGetString(psStringRes,
                                    STR_LABEL_ALLI as libc::c_int as UDWORD),
                    50 as libc::c_int as UDWORD,
                    (4 as libc::c_int + 30 as libc::c_int + 4 as libc::c_int +
                         30 as libc::c_int + 4 as libc::c_int +
                         30 as libc::c_int + 4 as libc::c_int +
                         38 as libc::c_int + 31 as libc::c_int +
                         31 as libc::c_int + 31 as libc::c_int) as UDWORD,
                    226 as libc::c_int as UDWORD,
                    27 as libc::c_int as UDWORD);
        //		}
        addMultiBut(psWScreen, 10298 as libc::c_int as UDWORD,
                    10270 as libc::c_int as UDWORD,
                    (50 as libc::c_int + 26 as libc::c_int +
                         10 as libc::c_int) as UDWORD,
                    2 as libc::c_int as UDWORD, 35 as libc::c_int as UDWORD,
                    24 as libc::c_int as UDWORD,
                    STR_MUL_ALLIANCEN as libc::c_int as UDWORD,
                    IMAGE_NOALLI as libc::c_int as UDWORD,
                    IMAGE_NOALLI_HI as libc::c_int as UDWORD,
                    1 as libc::c_int); //hilight correct entry
        addMultiBut(psWScreen, 10298 as libc::c_int as UDWORD,
                    10271 as libc::c_int as UDWORD,
                    (50 as libc::c_int + 26 as libc::c_int + 10 as libc::c_int
                         + 38 as libc::c_int) as UDWORD,
                    2 as libc::c_int as UDWORD, 35 as libc::c_int as UDWORD,
                    24 as libc::c_int as UDWORD,
                    STR_MUL_ALLIANCEY as libc::c_int as UDWORD,
                    IMAGE_ALLI as libc::c_int as UDWORD,
                    IMAGE_ALLI_HI as libc::c_int as UDWORD, 1 as libc::c_int);
        widgSetButtonState(psWScreen, 10270 as libc::c_int as UDWORD,
                           0 as libc::c_int as UDWORD);
        widgSetButtonState(psWScreen, 10271 as libc::c_int as UDWORD,
                           0 as libc::c_int as UDWORD);
        match game.alliance as libc::c_int {
            0 => {
                widgSetButtonState(psWScreen, 10270 as libc::c_int as UDWORD,
                                   0x2 as libc::c_int as UDWORD);
            }
            1 => {
                widgSetButtonState(psWScreen, 10271 as libc::c_int as UDWORD,
                                   0x2 as libc::c_int as UDWORD);
            }
            _ => { }
        }
    }
    /*	if(game.type == DMATCH)
	{

		// limit options
		addBlueForm(MULTIOP_OPTIONS,MULTIOP_LIMIT,strresGetString(psStringRes,STR_LABEL_LIMIT) ,MCOL0,MROW8,MULTIOP_BLUEFORMW,27);
		addMultiBut(psWScreen,MULTIOP_LIMIT,MULTIOP_NOLIMIT,MCOL1,2,MULTIOP_BUTW,MULTIOP_BUTH,
			STR_MUL_NOLIM,IMAGE_NOLIMIT,IMAGE_NOLIMIT_HI,TRUE);
		addMultiBut(psWScreen,MULTIOP_LIMIT,MULTIOP_FRAGLIMIT,MCOL2,2,MULTIOP_BUTW,MULTIOP_BUTH,
			STR_MUL_FRAGLIM,IMAGE_FRAGLIMIT,IMAGE_FRAGLIMIT_HI,TRUE);
		addMultiBut(psWScreen,MULTIOP_LIMIT,MULTIOP_TIMELIMIT, MCOL3, 2,MULTIOP_BUTW,MULTIOP_BUTH,
			STR_MUL_TIMELIM,IMAGE_TIMELIMIT,IMAGE_TIMELIMIT_HI,TRUE);
		widgSetButtonState(psWScreen, MULTIOP_NOLIMIT,0);		//hilight correct entry
		widgSetButtonState(psWScreen, MULTIOP_FRAGLIMIT,0);
		widgSetButtonState(psWScreen, MULTIOP_TIMELIMIT ,0);
		switch(game.limit)
		{
		case NOLIMIT:
			widgSetButtonState(psWScreen, MULTIOP_NOLIMIT,WBUT_LOCK);
			break;
		case FRAGLIMIT:
			widgSetButtonState(psWScreen, MULTIOP_FRAGLIMIT,WBUT_LOCK);
			break;
		case TIMELIMIT:
			widgSetButtonState(psWScreen, MULTIOP_TIMELIMIT,WBUT_LOCK);
			break;
		}
	}
*/
    if game.type_0 as libc::c_int == 14 as libc::c_int ||
           game.base as libc::c_int != 2 as libc::c_int {
        //	  ||(game.type != DMATCH && game.base != CAMP_WALLS))
        widgSetButtonState(psWScreen, 10257 as libc::c_int as UDWORD,
                           0x20 as libc::c_int as
                               UDWORD); // disable force buts
        widgSetButtonState(psWScreen, 10256 as libc::c_int as UDWORD,
                           0x1 as libc::c_int as UDWORD);
    }
    if game.type_0 as libc::c_int == 12 as libc::c_int ||
           game.type_0 as libc::c_int == 14 as libc::c_int ||
           game.type_0 as libc::c_int == 13 as libc::c_int {
        // pow levels
        addBlueForm(10250 as libc::c_int as UDWORD,
                    10296 as libc::c_int as UDWORD,
                    strresGetString(psStringRes,
                                    STR_INT_POWER as libc::c_int as UDWORD),
                    50 as libc::c_int as UDWORD,
                    (4 as libc::c_int + 30 as libc::c_int + 4 as libc::c_int +
                         30 as libc::c_int + 4 as libc::c_int +
                         30 as libc::c_int + 4 as libc::c_int +
                         38 as libc::c_int + 31 as libc::c_int +
                         31 as libc::c_int + 31 as libc::c_int +
                         31 as libc::c_int) as UDWORD,
                    226 as libc::c_int as UDWORD,
                    27 as libc::c_int as UDWORD); //hilight correct entry
        addMultiBut(psWScreen, 10296 as libc::c_int as UDWORD,
                    10272 as libc::c_int as UDWORD,
                    (50 as libc::c_int + 26 as libc::c_int +
                         10 as libc::c_int) as UDWORD,
                    2 as libc::c_int as UDWORD, 35 as libc::c_int as UDWORD,
                    24 as libc::c_int as UDWORD,
                    STR_MUL_POWLO as libc::c_int as UDWORD,
                    IMAGE_POWLO as libc::c_int as UDWORD,
                    IMAGE_POWLO_HI as libc::c_int as UDWORD,
                    1 as libc::c_int);
        addMultiBut(psWScreen, 10296 as libc::c_int as UDWORD,
                    10273 as libc::c_int as UDWORD,
                    (50 as libc::c_int + 26 as libc::c_int + 10 as libc::c_int
                         + 38 as libc::c_int) as UDWORD,
                    2 as libc::c_int as UDWORD, 35 as libc::c_int as UDWORD,
                    24 as libc::c_int as UDWORD,
                    STR_MUL_POWMED as libc::c_int as UDWORD,
                    IMAGE_POWMED as libc::c_int as UDWORD,
                    IMAGE_POWMED_HI as libc::c_int as UDWORD,
                    1 as libc::c_int);
        addMultiBut(psWScreen, 10296 as libc::c_int as UDWORD,
                    10274 as libc::c_int as UDWORD,
                    (50 as libc::c_int + 26 as libc::c_int + 10 as libc::c_int
                         + 38 as libc::c_int + 38 as libc::c_int) as UDWORD,
                    2 as libc::c_int as UDWORD, 35 as libc::c_int as UDWORD,
                    24 as libc::c_int as UDWORD,
                    STR_MUL_POWHI as libc::c_int as UDWORD,
                    IMAGE_POWHI as libc::c_int as UDWORD,
                    IMAGE_POWHI_HI as libc::c_int as UDWORD,
                    1 as libc::c_int);
        widgSetButtonState(psWScreen, 10272 as libc::c_int as UDWORD,
                           0 as libc::c_int as UDWORD);
        widgSetButtonState(psWScreen, 10273 as libc::c_int as UDWORD,
                           0 as libc::c_int as UDWORD);
        widgSetButtonState(psWScreen, 10274 as libc::c_int as UDWORD,
                           0 as libc::c_int as UDWORD);
        match game.power {
            100 => {
                widgSetButtonState(psWScreen, 10272 as libc::c_int as UDWORD,
                                   0x2 as libc::c_int as UDWORD);
            }
            400 => {
                widgSetButtonState(psWScreen, 10273 as libc::c_int as UDWORD,
                                   0x2 as libc::c_int as UDWORD);
            }
            700 => {
                widgSetButtonState(psWScreen, 10274 as libc::c_int as UDWORD,
                                   0x2 as libc::c_int as UDWORD);
            }
            _ => { }
        }
        //type clean/base/defence
        addBlueForm(10250 as libc::c_int as UDWORD,
                    10300 as libc::c_int as UDWORD,
                    strresGetString(psStringRes,
                                    STR_LABEL_BASE as libc::c_int as UDWORD),
                    50 as libc::c_int as UDWORD,
                    (4 as libc::c_int + 30 as libc::c_int + 4 as libc::c_int +
                         30 as libc::c_int + 4 as libc::c_int +
                         30 as libc::c_int + 4 as libc::c_int +
                         38 as libc::c_int + 31 as libc::c_int +
                         31 as libc::c_int) as UDWORD,
                    226 as libc::c_int as UDWORD,
                    27 as libc::c_int as UDWORD); //hilight correct entry
        addMultiBut(psWScreen, 10300 as libc::c_int as UDWORD,
                    10267 as libc::c_int as UDWORD,
                    (50 as libc::c_int + 26 as libc::c_int +
                         10 as libc::c_int) as UDWORD,
                    2 as libc::c_int as UDWORD, 35 as libc::c_int as UDWORD,
                    24 as libc::c_int as UDWORD,
                    STR_MUL_CAMPCLEAN as libc::c_int as UDWORD,
                    IMAGE_NOBASE as libc::c_int as UDWORD,
                    IMAGE_NOBASE_HI as libc::c_int as UDWORD,
                    1 as libc::c_int);
        addMultiBut(psWScreen, 10300 as libc::c_int as UDWORD,
                    10268 as libc::c_int as UDWORD,
                    (50 as libc::c_int + 26 as libc::c_int + 10 as libc::c_int
                         + 38 as libc::c_int) as UDWORD,
                    2 as libc::c_int as UDWORD, 35 as libc::c_int as UDWORD,
                    24 as libc::c_int as UDWORD,
                    STR_MUL_CAMPBASE as libc::c_int as UDWORD,
                    IMAGE_SBASE as libc::c_int as UDWORD,
                    IMAGE_SBASE_HI as libc::c_int as UDWORD,
                    1 as libc::c_int);
        addMultiBut(psWScreen, 10300 as libc::c_int as UDWORD,
                    10269 as libc::c_int as UDWORD,
                    (50 as libc::c_int + 26 as libc::c_int + 10 as libc::c_int
                         + 38 as libc::c_int + 38 as libc::c_int) as UDWORD,
                    2 as libc::c_int as UDWORD, 35 as libc::c_int as UDWORD,
                    24 as libc::c_int as UDWORD,
                    STR_MUL_CAMPDEFENCE as libc::c_int as UDWORD,
                    IMAGE_LBASE as libc::c_int as UDWORD,
                    IMAGE_LBASE_HI as libc::c_int as UDWORD,
                    1 as libc::c_int);
        widgSetButtonState(psWScreen, 10267 as libc::c_int as UDWORD,
                           0 as libc::c_int as UDWORD);
        widgSetButtonState(psWScreen, 10268 as libc::c_int as UDWORD,
                           0 as libc::c_int as UDWORD);
        widgSetButtonState(psWScreen, 10269 as libc::c_int as UDWORD,
                           0 as libc::c_int as UDWORD);
        match game.base as libc::c_int {
            0 => {
                widgSetButtonState(psWScreen, 10267 as libc::c_int as UDWORD,
                                   0x2 as libc::c_int as UDWORD);
            }
            1 => {
                widgSetButtonState(psWScreen, 10268 as libc::c_int as UDWORD,
                                   0x2 as libc::c_int as UDWORD);
            }
            2 => {
                widgSetButtonState(psWScreen, 10269 as libc::c_int as UDWORD,
                                   0x2 as libc::c_int as UDWORD);
            }
            _ => { }
        }
    }
    if game.type_0 as libc::c_int == 12 as libc::c_int ||
           game.type_0 as libc::c_int == 13 as libc::c_int {
        //type opposition/no computer opposition
        if game.type_0 as libc::c_int == 12 as libc::c_int {
            addBlueForm(10250 as libc::c_int as UDWORD,
                        10304 as libc::c_int as UDWORD,
                        strresGetString(psStringRes,
                                        STR_MUL_COMP as libc::c_int as
                                            UDWORD),
                        50 as libc::c_int as UDWORD,
                        (4 as libc::c_int + 30 as libc::c_int +
                             4 as libc::c_int + 30 as libc::c_int +
                             4 as libc::c_int + 30 as libc::c_int +
                             4 as libc::c_int + 38 as libc::c_int +
                             31 as libc::c_int + 31 as libc::c_int +
                             31 as libc::c_int + 31 as libc::c_int +
                             31 as libc::c_int) as UDWORD,
                        226 as libc::c_int as UDWORD,
                        27 as libc::c_int as UDWORD); //hilight correct entry
        } else {
            addBlueForm(10250 as libc::c_int as UDWORD,
                        10304 as libc::c_int as UDWORD,
                        strresGetString(psStringRes,
                                        STR_MUL_COMP as libc::c_int as
                                            UDWORD),
                        50 as libc::c_int as UDWORD,
                        (4 as libc::c_int + 30 as libc::c_int +
                             4 as libc::c_int + 30 as libc::c_int +
                             4 as libc::c_int + 30 as libc::c_int +
                             4 as libc::c_int + 38 as libc::c_int +
                             31 as libc::c_int + 31 as libc::c_int +
                             31 as libc::c_int) as UDWORD,
                        226 as libc::c_int as UDWORD,
                        27 as libc::c_int as UDWORD);
        }
        addMultiBut(psWScreen, 10304 as libc::c_int as UDWORD,
                    10308 as libc::c_int as UDWORD,
                    (50 as libc::c_int + 26 as libc::c_int +
                         10 as libc::c_int) as UDWORD,
                    2 as libc::c_int as UDWORD, 35 as libc::c_int as UDWORD,
                    24 as libc::c_int as UDWORD,
                    STR_MUL_COMP_Y as libc::c_int as UDWORD,
                    IMAGE_COMPUTER_Y as libc::c_int as UDWORD,
                    IMAGE_COMPUTER_Y_HI as libc::c_int as UDWORD,
                    1 as libc::c_int);
        addMultiBut(psWScreen, 10304 as libc::c_int as UDWORD,
                    10309 as libc::c_int as UDWORD,
                    (50 as libc::c_int + 26 as libc::c_int + 10 as libc::c_int
                         + 38 as libc::c_int) as UDWORD,
                    2 as libc::c_int as UDWORD, 35 as libc::c_int as UDWORD,
                    24 as libc::c_int as UDWORD,
                    STR_MUL_COMP_N as libc::c_int as UDWORD,
                    IMAGE_COMPUTER_N as libc::c_int as UDWORD,
                    IMAGE_COMPUTER_N_HI as libc::c_int as UDWORD,
                    1 as libc::c_int);
        widgSetButtonState(psWScreen, 10308 as libc::c_int as UDWORD,
                           0 as libc::c_int as UDWORD);
        widgSetButtonState(psWScreen, 10309 as libc::c_int as UDWORD,
                           0 as libc::c_int as UDWORD);
        match game.bComputerPlayers {
            0 => {
                widgSetButtonState(psWScreen, 10309 as libc::c_int as UDWORD,
                                   0x2 as libc::c_int as UDWORD);
            }
            1 => {
                widgSetButtonState(psWScreen, 10308 as libc::c_int as UDWORD,
                                   0x2 as libc::c_int as UDWORD);
            }
            _ => { }
        }
        //remove PC players for 7/8 player maps.
        if game.maxPlayers as libc::c_int > 6 as libc::c_int {
            widgSetButtonState(psWScreen, 10309 as libc::c_int as UDWORD,
                               0x1 as libc::c_int as UDWORD);
            widgSetButtonState(psWScreen, 10308 as libc::c_int as UDWORD,
                               0x1 as libc::c_int as UDWORD);
        }
    }
    // cancel
    addMultiBut(psWScreen, 10250 as libc::c_int as UDWORD,
                10102 as libc::c_int as UDWORD, 6 as libc::c_int as UDWORD,
                6 as libc::c_int as UDWORD,
                iV_GetImageWidth(FrontImages,
                                 IMAGE_RETURN as libc::c_int as UWORD) as
                    UDWORD,
                iV_GetImageHeight(FrontImages,
                                  IMAGE_RETURN as libc::c_int as UWORD) as
                    UDWORD, STR_MUL_CANCEL as libc::c_int as UDWORD,
                IMAGE_RETURN as libc::c_int as UDWORD,
                IMAGE_RETURN_HI as libc::c_int as UDWORD, 1 as libc::c_int);
    // host Games button
    if ingame.bHostSetup != 0 && bHosted == 0 {
        addMultiBut(psWScreen, 10250 as libc::c_int as UDWORD,
                    10276 as libc::c_int as UDWORD,
                    5 as libc::c_int as UDWORD,
                    (4 as libc::c_int + 30 as libc::c_int + 4 as libc::c_int +
                         30 as libc::c_int + 4 as libc::c_int +
                         3 as libc::c_int) as UDWORD,
                    35 as libc::c_int as UDWORD, 28 as libc::c_int as UDWORD,
                    STR_MUL_HOST as libc::c_int as UDWORD,
                    IMAGE_HOST as libc::c_int as UDWORD,
                    IMAGE_HOST as libc::c_int as UDWORD, 0 as libc::c_int);
    }
    // hosted or hosting.
	// limits button.
    if ingame.bHostSetup != 0 {
        //&& (game.type != DMATCH))
        addMultiBut(psWScreen, 10250 as libc::c_int as UDWORD,
                    10277 as libc::c_int as UDWORD,
                    5 as libc::c_int as UDWORD,
                    (4 as libc::c_int + 30 as libc::c_int + 4 as libc::c_int +
                         5 as libc::c_int) as UDWORD,
                    35 as libc::c_int as UDWORD, 28 as libc::c_int as UDWORD,
                    STR_MUL_STRLIM as libc::c_int as UDWORD,
                    IMAGE_SLIM as libc::c_int as UDWORD,
                    IMAGE_SLIM_HI as libc::c_int as UDWORD, 0 as libc::c_int);
    }
    // disable buttons not available in lobby games
    if NetPlay.bLobbyLaunched != 0 {
        widgSetButtonState(psWScreen, 10255 as libc::c_int as UDWORD,
                           0x20 as libc::c_int as UDWORD);
        widgSetButtonState(psWScreen, 10254 as libc::c_int as UDWORD,
                           0x1 as libc::c_int as UDWORD);
        widgSetButtonState(psWScreen, 10253 as libc::c_int as UDWORD,
                           0x20 as libc::c_int as UDWORD);
        widgSetButtonState(psWScreen, 10252 as libc::c_int as UDWORD,
                           0x1 as libc::c_int as UDWORD);
    };
}
// ////////////////////////////////////////////////////////////////////////////
// Colour functions
unsafe extern "C" fn safeToUseColour(mut player: UDWORD, mut col: UDWORD)
 -> BOOL {
    let mut i: UDWORD = 0;
    // if already using it.
    if col == getPlayerColour(player) as libc::c_uint {
        return 1 as libc::c_int
        // already using it.
    }
    // player wants to be colour. check no other player to see if it is using that colour.....
    i = 0 as libc::c_int as UDWORD;
    while i < 8 as libc::c_int as libc::c_uint {
        // if no human (except us) is using it
        if i != player && isHumanPlayer(i) != 0 &&
               getPlayerColour(i) as libc::c_uint == col {
            return 0 as libc::c_int
        }
        i = i.wrapping_add(1)
    }
    // no computer player is using it
/*	if(game.type == SKIRMISH)
	{
		if( (col<game.maxPlayers) )
		{
			for(i=0;i<game.maxPlayers;i++)
			{
				// check no player is using it...
				if( (i!=player) && !isHumanPlayer(i) && (getPlayerColour(i) == col) )
				{
					return FALSE;
				}
			}
		}
	}
	*/
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn chooseColour(mut player: UDWORD) -> BOOL {
    let mut col: UDWORD = 0;
    col = 0 as libc::c_int as UDWORD;
    while col < 8 as libc::c_int as libc::c_uint {
        if safeToUseColour(player, col) != 0 {
            setPlayerColour(player, col);
            return 1 as libc::c_int
        }
        col = col.wrapping_add(1)
    }
    return 1 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////////
// colour chooser.
unsafe extern "C" fn addColourChooser(mut player: UDWORD) {
    let mut i: UDWORD = 0; //,j;
    // delete that players box,
    widgDelete(psWScreen,
               (10232 as libc::c_int as libc::c_uint).wrapping_add(player));
    // add form.
    addBlueForm(10231 as libc::c_int as UDWORD,
                10280 as libc::c_int as UDWORD,
                b"\x00" as *const u8 as *const libc::c_char as *mut STRING,
                10 as libc::c_int as UDWORD,
                ((36 as libc::c_int + 5 as libc::c_int) as
                     libc::c_uint).wrapping_mul(player).wrapping_add(4 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_uint),
                230 as libc::c_int as UDWORD, 36 as libc::c_int as UDWORD);
    // add the flags
    i = 0 as libc::c_int as UDWORD;
    while i < 8 as libc::c_int as libc::c_uint {
        //game.maxPlayers;i++)
        addMultiBut(psWScreen, 10280 as libc::c_int as UDWORD,
                    (10281 as libc::c_int as libc::c_uint).wrapping_add(i),
                    i.wrapping_mul((iV_GetImageWidth(FrontImages,
                                                     IMAGE_PLAYER0 as
                                                         libc::c_int as UWORD)
                                        as libc::c_int + 5 as libc::c_int) as
                                       libc::c_uint).wrapping_add(7 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint),
                    4 as libc::c_int as UDWORD,
                    iV_GetImageWidth(FrontImages,
                                     IMAGE_PLAYER0 as libc::c_int as UWORD) as
                        UDWORD,
                    iV_GetImageHeight(FrontImages,
                                      IMAGE_PLAYER0 as libc::c_int as UWORD)
                        as UDWORD, 0 as libc::c_int as UDWORD,
                    (IMAGE_PLAYER0 as libc::c_int as
                         libc::c_uint).wrapping_add(i),
                    (IMAGE_PLAYER0 as libc::c_int as
                         libc::c_uint).wrapping_add(i), 0 as libc::c_int);
        if safeToUseColour(selectedPlayer, i) == 0 {
            widgSetButtonState(psWScreen,
                               (10281 as libc::c_int as
                                    libc::c_uint).wrapping_add(i),
                               0x1 as libc::c_int as UDWORD);
        }
        i = i.wrapping_add(1)
    }
    //add the position chooser.
    i = 0 as libc::c_int as UDWORD;
    while i < game.maxPlayers as libc::c_uint {
        addMultiBut(psWScreen, 10280 as libc::c_int as UDWORD,
                    (10321 as libc::c_int as libc::c_uint).wrapping_add(i),
                    i.wrapping_mul((iV_GetImageWidth(FrontImages,
                                                     IMAGE_PLAYER0 as
                                                         libc::c_int as UWORD)
                                        as libc::c_int + 5 as libc::c_int) as
                                       libc::c_uint).wrapping_add(7 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint),
                    23 as libc::c_int as UDWORD,
                    (iV_GetImageWidth(FrontImages,
                                      IMAGE_WEE_GUY as libc::c_int as UWORD)
                         as libc::c_int + 7 as libc::c_int) as UDWORD,
                    iV_GetImageHeight(FrontImages,
                                      IMAGE_WEE_GUY as libc::c_int as UWORD)
                        as UDWORD, 0 as libc::c_int as UDWORD,
                    IMAGE_WEE_GUY as libc::c_int as UDWORD,
                    IMAGE_WEE_GUY as libc::c_int as UDWORD,
                    (10 as libc::c_int as libc::c_uint).wrapping_add(i) as
                        BOOL);
        if isHumanPlayer(i) != 0 && i != selectedPlayer {
            widgSetButtonState(psWScreen,
                               (10321 as libc::c_int as
                                    libc::c_uint).wrapping_add(i),
                               0x1 as libc::c_int as UDWORD);
        }
        i = i.wrapping_add(1)
    }
    bColourChooserUp = 1 as libc::c_int;
}
unsafe extern "C" fn closeColourChooser() {
    bColourChooserUp = 0 as libc::c_int;
    widgDelete(psWScreen, 10280 as libc::c_int as UDWORD);
}
unsafe extern "C" fn SendColourRequest(mut player: UDWORD, mut col: UBYTE,
                                       mut chosenPlayer: UBYTE) -> BOOL {
    let mut m: NETMSG =
        NETMSG{size: 0,
               paddedBytes: 0,
               type_0: 0,
               destination: 0,
               body: [0; 8000],};
    m.body[0 as libc::c_int as usize] = player as UBYTE as libc::c_char;
    m.body[1 as libc::c_int as usize] = col as libc::c_char;
    m.body[2 as libc::c_int as usize] = chosenPlayer as libc::c_char;
    m.type_0 = NET_COLOURREQUEST as libc::c_int as libc::c_uchar;
    m.size = 3 as libc::c_int as libc::c_ushort;
    if NetPlay.bHost != 0 {
        // do or request the change.
        recvColourRequest(&mut m); // do the change, remember only the host can do this to avoid confusion.
        return 1 as libc::c_int
    } else { return NETbcast(&mut m, 1 as libc::c_int) };
}
//extern BOOL multiPlayerRequest		(NETMSG *pMsg);
#[no_mangle]
pub unsafe extern "C" fn recvColourRequest(mut pMsg: *mut NETMSG) -> BOOL {
    let mut player: UDWORD = 0;
    let mut col: UDWORD = 0;
    let mut oldcol: UDWORD = 0;
    let mut chosenPlayer: UBYTE = 0;
    let mut dpid: DPID = 0;
    if NetPlay.bHost == 0 {
        //only host should act.
        return 1 as libc::c_int
    }
    player = (*pMsg).body[0 as libc::c_int as usize] as UDWORD;
    col = (*pMsg).body[1 as libc::c_int as usize] as UDWORD;
    chosenPlayer = (*pMsg).body[2 as libc::c_int as usize] as UDWORD as UBYTE;
    if chosenPlayer as libc::c_int == 0xff as libc::c_int {
        // colour change.
        if safeToUseColour(player, col) == 0 { return 0 as libc::c_int }
        setPlayerColour(player, col);
        sendOptions(player2dpid[player as usize], player);
        // tell everyone && update requesting player.
    } else {
        // base change.
        if isHumanPlayer(chosenPlayer as UDWORD) != 0 {
            return 0 as libc::c_int
        } // remove player,
        dpid =
            player2dpid[player as
                            usize]; // if they never joined, reset the flag
        player2dpid[player as usize] =
            0 as libc::c_int; // retain player colour
        ingame.JoiningInProgress[player as usize] =
            0 as libc::c_int; // reset old colour
        oldcol = getPlayerColour(chosenPlayer as UDWORD) as UDWORD;
        setPlayerColour(chosenPlayer as UDWORD,
                        getPlayerColour(player) as UDWORD);
        setPlayerColour(player, oldcol);
        //		chooseColour	(chosenPlayer);				// pick an unused colour.
        setupNewPlayer(dpid,
                       chosenPlayer as
                           UDWORD); // setup all the guff for that player.
        sendOptions(dpid,
                    chosenPlayer as
                        UDWORD); // bring netplay up to date with changes.
        NETplayerInfo();
        if player == selectedPlayer {
            // if host changing
            selectedPlayer = chosenPlayer as UDWORD
        }
    }
    return 1 as libc::c_int;
}
// options (rhs) boxV
// ////////////////////////////////////////////////////////////////////////////
// box for players.
#[no_mangle]
pub unsafe extern "C" fn addPlayerBox(mut players: BOOL) -> UDWORD {
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
    let mut sButInit: W_BUTINIT =
        W_BUTINIT{formID: 0,
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
                  pText: 0 as *mut STRING,
                  pTip: 0 as *mut STRING,
                  FontID: 0,};
    let mut i: UDWORD = 0 as libc::c_int as UDWORD;
    // if background isn't there, then return since were not ready to draw the box yet!
    if widgGetFromID(psWScreen, 20000 as libc::c_int as UDWORD).is_null() {
        return 0 as libc::c_int as UDWORD
    }
    if bHosted != 0 || ingame.localJoiningInProgress != 0 {
        NETplayerInfo();
    } else {
        NETplayerInfo();
        // get player info.
    } // del player window
    widgDelete(psWScreen, 10231 as libc::c_int as UDWORD); // del text too,
    widgDelete(psWScreen,
               20028 as libc::c_int as UDWORD); // draw player window
    memset(&mut sFormInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_FORMINIT>() as libc::c_ulong);
    sFormInit.formID = 20000 as libc::c_int as UDWORD;
    sFormInit.id = 10231 as libc::c_int as UDWORD;
    sFormInit.x = 373 as libc::c_int as SWORD;
    sFormInit.y = 15 as libc::c_int as SWORD;
    sFormInit.style = 0 as libc::c_int as UDWORD;
    sFormInit.width = 250 as libc::c_int as UWORD;
    sFormInit.height = 330 as libc::c_int as UWORD;
    sFormInit.pDisplay =
        Some(intDisplayPlainForm as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    widgAddForm(psWScreen, &mut sFormInit);
    addSideText(20028 as libc::c_int as UDWORD,
                (373 as libc::c_int - 3 as libc::c_int) as UDWORD,
                15 as libc::c_int as UDWORD,
                strresGetString(psStringRes,
                                STR_MUL_SIDEPLAYERS as libc::c_int as
                                    UDWORD));
    if players != 0 {
        i = 0 as libc::c_int as UDWORD;
        while i < game.maxPlayers as libc::c_uint {
            if ingame.localOptionsReceived != 0 &&
                   NetPlay.players[i as usize].dpid != 0 {
                // only draw if real player!
                memset(&mut sButInit as *mut W_BUTINIT as *mut libc::c_void,
                       0 as libc::c_int,
                       ::std::mem::size_of::<W_BUTINIT>() as
                           libc::c_ulong); //Players[i].name;
                sButInit.formID =
                    10231 as libc::c_int as UDWORD; //intDisplayButtonHilight;
                sButInit.id =
                    (10232 as libc::c_int as libc::c_uint).wrapping_add(i);
                sButInit.style = 0 as libc::c_int as UDWORD;
                sButInit.x = 10 as libc::c_int as SWORD;
                sButInit.y =
                    ((36 as libc::c_int + 5 as libc::c_int) as
                         libc::c_uint).wrapping_mul(i).wrapping_add(4 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_uint)
                        as SHORT;
                sButInit.width = 230 as libc::c_int as UWORD;
                sButInit.height = 36 as libc::c_int as UWORD;
                sButInit.pTip = 0 as *mut STRING;
                sButInit.FontID = WFont;
                sButInit.pDisplay =
                    Some(displayPlayer as
                             unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                                  _: UDWORD, _: *mut UDWORD)
                                 -> ());
                sButInit.pUserData = i as *mut libc::c_void;
                if bColourChooserUp != 0 &&
                       NetPlay.players[i as usize].dpid ==
                           player2dpid[selectedPlayer as usize] {
                    addColourChooser(i);
                } else { widgAddButton(psWScreen, &mut sButInit); }
            } else if game.type_0 as libc::c_int == 14 as libc::c_int {
                // skirmish player
                memset(&mut sFormInit as *mut W_FORMINIT as *mut libc::c_void,
                       0 as libc::c_int,
                       ::std::mem::size_of::<W_BUTINIT>() as
                           libc::c_ulong); //Players[i].name;
                sFormInit.formID =
                    10231 as libc::c_int as UDWORD; //intDisplayButtonHilight;
                sFormInit.id =
                    (10232 as libc::c_int as libc::c_uint).wrapping_add(i);
                sFormInit.style = 0 as libc::c_int as UDWORD;
                sFormInit.x = 10 as libc::c_int as SWORD;
                sFormInit.y =
                    ((36 as libc::c_int + 5 as libc::c_int) as
                         libc::c_uint).wrapping_mul(i).wrapping_add(4 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_uint)
                        as SHORT;
                sFormInit.width = 230 as libc::c_int as UWORD;
                sFormInit.height = 36 as libc::c_int as UWORD;
                sFormInit.pTip = 0 as *mut STRING;
                sFormInit.pDisplay =
                    Some(displayPlayer as
                             unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                                  _: UDWORD, _: *mut UDWORD)
                                 -> ());
                sFormInit.pUserData = i as *mut libc::c_void;
                widgAddForm(psWScreen, &mut sFormInit);
                addFESlider((10313 as libc::c_int as
                                 libc::c_uint).wrapping_add(i), sFormInit.id,
                            43 as libc::c_int as UDWORD,
                            9 as libc::c_int as UDWORD,
                            20 as libc::c_int as UDWORD,
                            game.skDiff[i as usize] as UDWORD,
                            0 as libc::c_int as UDWORD);
            }
            i = i.wrapping_add(1)
        }
    }
    if ingame.bHostSetup != 0 {
        // if hosting.
        sliderEnableDrag(1 as libc::c_int);
    } else { sliderEnableDrag(0 as libc::c_int); }
    return i;
}
// ////////////////////////////////////////////////////////////////////////////
unsafe extern "C" fn SendFireUp() {
    let mut m: NETMSG =
        NETMSG{size: 0,
               paddedBytes: 0,
               type_0: 0,
               destination: 0,
               body: [0; 8000],};
    m.type_0 = NET_FIREUP as libc::c_int as libc::c_uchar;
    m.size = 1 as libc::c_int as libc::c_ushort;
    NETbcast(&mut m, 1 as libc::c_int);
}
// host kick a player from a game.
#[no_mangle]
pub unsafe extern "C" fn kickPlayer(mut dpid: DPID) {
    let mut m: NETMSG =
        NETMSG{size: 0,
               paddedBytes: 0,
               type_0: 0,
               destination: 0,
               body: [0; 8000],};
    // send a kick msg
    m.type_0 = NET_KICK as libc::c_int as libc::c_uchar;
    m.size = 4 as libc::c_int as libc::c_ushort;
    memcpy(&mut *m.body.as_mut_ptr().offset(0 as libc::c_int as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut dpid as *mut DPID as *const libc::c_void,
           ::std::mem::size_of::<DPID>() as libc::c_ulong);
    NETbcast(&mut m, 1 as libc::c_int);
}
// Game option functions
unsafe extern "C" fn addOkBut() {
    addMultiBut(psWScreen, 10250 as libc::c_int as UDWORD,
                10101 as libc::c_int as UDWORD, 5 as libc::c_int as UDWORD,
                (4 as libc::c_int + 30 as libc::c_int + 4 as libc::c_int +
                     30 as libc::c_int + 4 as libc::c_int + 3 as libc::c_int)
                    as UDWORD,
                iV_GetImageWidth(FrontImages,
                                 IMAGE_BIGOK as libc::c_int as UWORD) as
                    UDWORD,
                iV_GetImageHeight(FrontImages,
                                  IMAGE_BIGOK as libc::c_int as UWORD) as
                    UDWORD, STR_MUL_OK as libc::c_int as UDWORD,
                IMAGE_BIGOK as libc::c_int as UDWORD,
                IMAGE_BIGOK as libc::c_int as UDWORD, 0 as libc::c_int);
}
// players (mid) box
unsafe extern "C" fn addChatBox() {
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
                   pFormDisplay: None,}; // add the form
    let mut sEdInit: W_EDBINIT =
        W_EDBINIT{formID: 0,
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
                  pText: 0 as *mut STRING,
                  FontID: 0,
                  pBoxDisplay: None,
                  pFontDisplay: None,}; // wait till open!
    if !widgGetFromID(psWScreen, 20001 as libc::c_int as UDWORD).is_null() {
        widgDelete(psWScreen,
                   20001 as libc::c_int as UDWORD); //intDisplayPlainForm;
    } // add the chatbox.
    if !widgGetFromID(psWScreen, 10278 as libc::c_int as UDWORD).is_null() {
        return
    } // use x lines on chat window
    memset(&mut sFormInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_FORMINIT>() as
               libc::c_ulong); // add the edit box
    sFormInit.formID = 20000 as libc::c_int as UDWORD;
    sFormInit.id = 10278 as libc::c_int as UDWORD;
    sFormInit.x = 40 as libc::c_int as SWORD;
    sFormInit.y = 350 as libc::c_int as SWORD;
    sFormInit.style = 0 as libc::c_int as UDWORD;
    sFormInit.width =
        (373 as libc::c_int + 250 as libc::c_int - 40 as libc::c_int) as
            UWORD;
    sFormInit.height = 115 as libc::c_int as UWORD;
    sFormInit.disableChildren = 1 as libc::c_int;
    sFormInit.pDisplay =
        Some(intOpenPlainForm as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    widgAddForm(psWScreen, &mut sFormInit);
    addSideText(20030 as libc::c_int as UDWORD,
                (40 as libc::c_int - 3 as libc::c_int) as UDWORD,
                350 as libc::c_int as UDWORD,
                strresGetString(psStringRes,
                                STR_MUL_CHAT as libc::c_int as UDWORD));
    flushConsoleMessages();
    initConsoleMessages();
    enableConsoleDisplay(1 as libc::c_int);
    setConsoleBackdropStatus(0 as libc::c_int);
    setDefaultConsoleJust(LEFT_JUSTIFY);
    setConsoleSizePos(((40 as libc::c_int + 4 as libc::c_int) as
                           libc::c_uint).wrapping_add(pie_GetVideoBufferWidth().wrapping_sub(640
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_uint).wrapping_div(2
                                                                                                                                as
                                                                                                                                libc::c_int
                                                                                                                                as
                                                                                                                                libc::c_uint)),
                      ((350 as libc::c_int + 10 as libc::c_int) as
                           libc::c_uint).wrapping_add(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                                  as
                                                                                                  libc::c_int
                                                                                                  as
                                                                                                  libc::c_uint).wrapping_div(2
                                                                                                                                 as
                                                                                                                                 libc::c_int
                                                                                                                                 as
                                                                                                                                 libc::c_uint)),
                      (373 as libc::c_int + 250 as libc::c_int -
                           40 as libc::c_int - 4 as libc::c_int) as UDWORD);
    setConsolePermanence(1 as libc::c_int, 1 as libc::c_int);
    setConsoleLineInfo(5 as libc::c_int as UDWORD);
    memset(&mut sEdInit as *mut W_EDBINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_EDBINIT>() as libc::c_ulong);
    sEdInit.formID = 10278 as libc::c_int as UDWORD;
    sEdInit.id = 10279 as libc::c_int as UDWORD;
    sEdInit.x = 4 as libc::c_int as SWORD;
    sEdInit.y = (115 as libc::c_int - 14 as libc::c_int) as SWORD;
    sEdInit.style = 0 as libc::c_int as UDWORD;
    sEdInit.width =
        (373 as libc::c_int + 250 as libc::c_int - 40 as libc::c_int -
             8 as libc::c_int) as UWORD;
    sEdInit.height = 9 as libc::c_int as UWORD;
    sEdInit.FontID = WFont;
    sEdInit.pUserData = 0 as *mut libc::c_void;
    sEdInit.pBoxDisplay =
        Some(displayChatEdit as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    widgAddEditBox(psWScreen, &mut sEdInit);
}
// ////////////////////////////////////////////////////////////////////////////
unsafe extern "C" fn disableMultiButs() {
    // edit box icons.
    widgSetButtonState(psWScreen, 10254 as libc::c_int as UDWORD,
                       0x1 as libc::c_int as UDWORD);
    widgSetButtonState(psWScreen, 10258 as libc::c_int as UDWORD,
                       0x1 as libc::c_int as UDWORD);
    // edit boxes
    widgSetButtonState(psWScreen, 10255 as libc::c_int as UDWORD,
                       0x20 as libc::c_int as UDWORD);
    widgSetButtonState(psWScreen, 10259 as libc::c_int as UDWORD,
                       0x20 as libc::c_int as UDWORD);
    // force choice.
    if game.type_0 as libc::c_int == 14 as libc::c_int ||
           game.base as libc::c_int != 2 as libc::c_int {
        //	if  (game.type == SKIRMISH ||(game.type != DMATCH && game.base != CAMP_WALLS ))
        widgSetButtonState(psWScreen, 10256 as libc::c_int as UDWORD,
                           0x1 as libc::c_int as UDWORD);
        widgSetButtonState(psWScreen, 10257 as libc::c_int as UDWORD,
                           0x20 as libc::c_int as UDWORD);
    }
    if game.type_0 as libc::c_int != 12 as libc::c_int {
        widgSetButtonState(psWScreen, 10261 as libc::c_int as UDWORD,
                           0x1 as libc::c_int as UDWORD);
    }
    //	if(game.type != DMATCH)		widgSetButtonState(psWScreen, MULTIOP_ARENA,	WBUT_DISABLE);
    if game.type_0 as libc::c_int != 14 as libc::c_int {
        widgSetButtonState(psWScreen, 10263 as libc::c_int as UDWORD,
                           0x1 as libc::c_int as UDWORD); //fog
    }
    if game.type_0 as libc::c_int != 13 as libc::c_int {
        widgSetButtonState(psWScreen, 10262 as libc::c_int as UDWORD,
                           0x1 as libc::c_int as UDWORD);
    }
    if NetPlay.bHost == 0 {
        if game.fog != 0 {
            widgSetButtonState(psWScreen, 10311 as libc::c_int as UDWORD,
                               0x1 as libc::c_int as UDWORD);
        }
        if game.fog == 0 {
            widgSetButtonState(psWScreen, 10310 as libc::c_int as UDWORD,
                               0x1 as libc::c_int as UDWORD);
        }
        //		if(  game.type == DMATCH )
//		{
//			if(game.limit != NOLIMIT)	widgSetButtonState(psWScreen, MULTIOP_NOLIMIT,WBUT_DISABLE);	// limit levels
//			if(game.limit != FRAGLIMIT)	widgSetButtonState(psWScreen, MULTIOP_FRAGLIMIT,WBUT_DISABLE);
//			if(game.limit != TIMELIMIT)	widgSetButtonState(psWScreen, MULTIOP_TIMELIMIT,WBUT_DISABLE);
//		}
        if game.type_0 as libc::c_int == 12 as libc::c_int {
            if game.bComputerPlayers != 0 {
                widgSetButtonState(psWScreen, 10309 as libc::c_int as UDWORD,
                                   0x1 as libc::c_int as
                                       UDWORD); // pow levels
            } // camapign subtype.
            if game.bComputerPlayers == 0 {
                widgSetButtonState(psWScreen, 10308 as libc::c_int as UDWORD,
                                   0x1 as libc::c_int as
                                       UDWORD); // pow levels
            }
        } //alliance settings.
        if game.type_0 as libc::c_int == 12 as libc::c_int ||
               game.type_0 as libc::c_int == 13 as libc::c_int ||
               game.type_0 as libc::c_int == 14 as libc::c_int {
            if game.base as libc::c_int != 0 as libc::c_int {
                widgSetButtonState(psWScreen, 10267 as libc::c_int as UDWORD,
                                   0x1 as libc::c_int as UDWORD);
            }
            if game.base as libc::c_int != 1 as libc::c_int {
                widgSetButtonState(psWScreen, 10268 as libc::c_int as UDWORD,
                                   0x1 as libc::c_int as UDWORD);
            }
            if game.base as libc::c_int != 2 as libc::c_int {
                widgSetButtonState(psWScreen, 10269 as libc::c_int as UDWORD,
                                   0x1 as libc::c_int as UDWORD);
            }
            if game.power != 100 as libc::c_int as libc::c_uint {
                widgSetButtonState(psWScreen, 10272 as libc::c_int as UDWORD,
                                   0x1 as libc::c_int as UDWORD);
            }
            if game.power != 400 as libc::c_int as libc::c_uint {
                widgSetButtonState(psWScreen, 10273 as libc::c_int as UDWORD,
                                   0x1 as libc::c_int as UDWORD);
            }
            if game.power != 700 as libc::c_int as libc::c_uint {
                widgSetButtonState(psWScreen, 10274 as libc::c_int as UDWORD,
                                   0x1 as libc::c_int as UDWORD);
            }
        }
        if game.type_0 as libc::c_int == 12 as libc::c_int ||
               game.type_0 as libc::c_int == 14 as libc::c_int {
            if game.alliance as libc::c_int != 0 as libc::c_int {
                widgSetButtonState(psWScreen, 10270 as libc::c_int as UDWORD,
                                   0x1 as libc::c_int as UDWORD);
            }
            if game.alliance as libc::c_int != 1 as libc::c_int {
                widgSetButtonState(psWScreen, 10271 as libc::c_int as UDWORD,
                                   0x1 as libc::c_int as UDWORD);
            }
        }
    };
}
// //////////////////////////////////////////////////////////////////////////
#[no_mangle]
pub unsafe extern "C" fn stopJoining() {
    dwSelectedGame = 0 as libc::c_int;
    saveConfig();
    if bWhiteBoardUp != 0 { removeWhiteBoard(); }
    if NetPlay.bLobbyLaunched != 0 {
        changeTitleMode(QUIT);
        return
    } else {
        if bHosted != 0 {
            // cancel a hosted game.
            sendLeavingMsg(); // say goodbye
            NETclose(); // quit running game.
            bHosted = 0 as libc::c_int; // stop host mode.
            widgDelete(psWScreen,
                       20000 as libc::c_int as
                           UDWORD); // refresh options screen.
            startMultiOptions(1 as libc::c_int);
            ingame.localJoiningInProgress = 0 as libc::c_int;
            return
        } else {
            if ingame.localJoiningInProgress != 0 {
                //			startMultiOptions(FALSE);
                // cancel a joined game.
                sendLeavingMsg(); // say goodbye
                NETclose(); // quit running game.
                ingame.localJoiningInProgress =
                    0 as libc::c_int; // reset local flags
                ingame.localOptionsReceived = 0 as libc::c_int;
                if ingame.bHostSetup == 0 && NetPlay.bHost != 0 {
                    // joining and host was transfered.
                    NetPlay.bHost = 0 as libc::c_int
                }
                if NetPlay.bComms != 0 {
                    // not even connected.
                    changeTitleMode(GAMEFIND);
                    selectedPlayer = 0 as libc::c_int as UDWORD
                } else {
                    changeTitleMode(MULTI);
                    selectedPlayer = 0 as libc::c_int as UDWORD
                }
                return
            }
        }
        if NetPlay.bComms != 0 {
            // not even connected.
            changeTitleMode(PROTOCOL);
            selectedPlayer = 0 as libc::c_int as UDWORD
        } else {
            changeTitleMode(MULTI);
            selectedPlayer = 0 as libc::c_int as UDWORD
        }
        if ingame.bHostSetup != 0 {
            pie_LoadBackDrop(SCREEN_RANDOMBDROP, 0 as libc::c_int);
        }
    };
}
// //////////////////////////////////////////////////////////////////////////
unsafe extern "C" fn chooseSkirmishColours() {
    let mut col: UDWORD = 0;
    let mut i: UDWORD = 0;
    let mut k: UDWORD = 0;
    let mut taken: BOOL = 0;
    col = 0 as libc::c_int as UDWORD;
    i = 0 as libc::c_int as UDWORD;
    while i < 8 as libc::c_int as libc::c_uint {
        // assign each pc player a colour.
        if isHumanPlayer(i) == 0 {
            // pick a colour for this player.
            taken = 1 as libc::c_int; // go to next unused colour.
            while taken != 0 {
                taken = 0 as libc::c_int;
                k = 0 as libc::c_int as UDWORD;
                while k < 8 as libc::c_int as libc::c_uint {
                    if isHumanPlayer(k) != 0 &&
                           getPlayerColour(k) as libc::c_uint == col {
                        taken = 1 as libc::c_int
                        // already taken.
                    }
                    k = k.wrapping_add(1)
                }
                if taken != 0 { col = col.wrapping_add(1) }
            }
            setPlayerColour(i, col);
            col = col.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    };
}
// //////////////////////////////////////////////////////////////////////////
unsafe extern "C" fn processMultiopWidgets(mut id: UDWORD) {
    let mut playerStats: PLAYERSTATS =
        PLAYERSTATS{played: 0,
                    wins: 0,
                    loses: 0,
                    totalKills: 0,
                    totalScore: 0,
                    recentKills: 0,
                    recentScore: 0,
                    killsToAdd: 0,
                    scoreToAdd: 0,};
    let mut i: UDWORD = 0;
    let mut tmp: [STRING; 255] = [0; 255];
    // host, who is setting up the game
    if ingame.bHostSetup != 0 && bHosted == 0 {
        match id {
            10255 => {
                // Options buttons
                // we get this when nec.
                strcpy(game.name.as_mut_ptr(),
                       widgGetString(psWScreen,
                                     10255 as libc::c_int as
                                         UDWORD)); // del text too,
            }
            10259 => {
                widgSetString(psWScreen, 10259 as libc::c_int as UDWORD,
                              game.map.as_mut_ptr());
            }
            10258 => {
                widgDelete(psWScreen, 10231 as libc::c_int as UDWORD);
                widgDelete(psWScreen, 20028 as libc::c_int as UDWORD);
                strcpy(tmp.as_mut_ptr(), MultiCustomMapsPath.as_mut_ptr());
                strcat(tmp.as_mut_ptr(),
                       b"*.wrf\x00" as *const u8 as *const libc::c_char);
                debug(LOG_WZ,
                      b"processMultiopWidgets[MULTIOP_MAP_ICON]: %s\x00" as
                          *const u8 as *const libc::c_char, tmp.as_mut_ptr());
                addMultiRequest(tmp.as_mut_ptr(),
                                10259 as libc::c_int as UDWORD,
                                1 as libc::c_int as UBYTE,
                                2 as libc::c_int as UBYTE);
            }
            10261 => {
                //		case MULTIOP_ARENA:										// turn on arena game
//			widgSetButtonState(psWScreen, MULTIOP_ARENA, WBUT_LOCK);
//			widgSetButtonState(psWScreen, MULTIOP_CAMPAIGN, 0);
//			widgSetButtonState(psWScreen, MULTIOP_SKIRMISH,0);
//			widgSetButtonState(psWScreen, MULTIOP_TEAMPLAY,0);
//			game.type = DMATCH;
//
//			widgSetString(psWScreen, MULTIOP_MAP, "DeadValley");
//			strcpy(game.map,widgGetString(psWScreen, MULTIOP_MAP));
//			game.maxPlayers =8;
//
//			addGameOptions();
//			widgSetButtonState(psWScreen, MULTIOP_FNAME,WEDBS_FIXED);
//			widgSetButtonState(psWScreen, MULTIOP_FNAME_ICON,0);
//			break;
                // turn on campaign game
                //			widgSetButtonState(psWScreen, MULTIOP_ARENA, 0);
                widgSetButtonState(psWScreen, 10261 as libc::c_int as UDWORD,
                                   0x2 as libc::c_int as UDWORD);
                widgSetButtonState(psWScreen, 10263 as libc::c_int as UDWORD,
                                   0 as libc::c_int as UDWORD);
                widgSetButtonState(psWScreen, 10262 as libc::c_int as UDWORD,
                                   0 as libc::c_int as UDWORD);
                game.type_0 = 12 as libc::c_int as UBYTE;
                widgSetString(psWScreen, 10259 as libc::c_int as UDWORD,
                              b"Rush\x00" as *const u8 as *const libc::c_char
                                  as *mut STRING);
                strcpy(game.map.as_mut_ptr(),
                       widgGetString(psWScreen,
                                     10259 as libc::c_int as UDWORD));
                game.maxPlayers = 4 as libc::c_int as UBYTE;
                addGameOptions(0 as libc::c_int);
            }
            10263 => {
                //			widgSetButtonState(psWScreen, MULTIOP_ARENA, 0);
                widgSetButtonState(psWScreen, 10261 as libc::c_int as UDWORD,
                                   0 as libc::c_int as UDWORD);
                widgSetButtonState(psWScreen, 10263 as libc::c_int as UDWORD,
                                   0x2 as libc::c_int as UDWORD);
                widgSetButtonState(psWScreen, 10262 as libc::c_int as UDWORD,
                                   0 as libc::c_int as UDWORD);
                game.type_0 = 14 as libc::c_int as UBYTE;
                widgSetString(psWScreen, 10259 as libc::c_int as UDWORD,
                              b"Sk-Rush\x00" as *const u8 as
                                  *const libc::c_char as *mut STRING);
                strcpy(game.map.as_mut_ptr(),
                       widgGetString(psWScreen,
                                     10259 as libc::c_int as UDWORD));
                game.maxPlayers = 4 as libc::c_int as UBYTE;
                addGameOptions(0 as libc::c_int);
            }
            10262 => {
                //			widgSetButtonState(psWScreen, MULTIOP_ARENA, 0);
                widgSetButtonState(psWScreen, 10261 as libc::c_int as UDWORD,
                                   0 as libc::c_int as UDWORD);
                widgSetButtonState(psWScreen, 10263 as libc::c_int as UDWORD,
                                   0 as libc::c_int as UDWORD);
                widgSetButtonState(psWScreen, 10262 as libc::c_int as UDWORD,
                                   0x2 as libc::c_int as UDWORD);
                widgSetString(psWScreen, 10259 as libc::c_int as UDWORD,
                              b"Rush\x00" as *const u8 as *const libc::c_char
                                  as *mut STRING);
                strcpy(game.map.as_mut_ptr(),
                       widgGetString(psWScreen,
                                     10259 as libc::c_int as UDWORD));
                game.type_0 = 13 as libc::c_int as UBYTE;
                game.maxPlayers = 4 as libc::c_int as UBYTE;
                game.alliance = 1 as libc::c_int as UBYTE;
                addGameOptions(0 as libc::c_int);
            }
            10254 | _ => { }
        }
    }
    // host who is setting up or has hosted
    (ingame.bHostSetup) != 0; // || NetPlay.bHost)
    match id {
        10308 => {
            widgSetButtonState(psWScreen, 10308 as libc::c_int as UDWORD,
                               0x2 as libc::c_int as UDWORD);
            widgSetButtonState(psWScreen, 10309 as libc::c_int as UDWORD,
                               0 as libc::c_int as UDWORD);
            game.bComputerPlayers = 1 as libc::c_int;
            if bHosted != 0 {
                sendOptions(0 as libc::c_int, 0 as libc::c_int as UDWORD);
            }
        }
        10309 => {
            widgSetButtonState(psWScreen, 10308 as libc::c_int as UDWORD,
                               0 as libc::c_int as UDWORD);
            widgSetButtonState(psWScreen, 10309 as libc::c_int as UDWORD,
                               0x2 as libc::c_int as UDWORD);
            game.bComputerPlayers = 0 as libc::c_int;
            if bHosted != 0 {
                sendOptions(0 as libc::c_int, 0 as libc::c_int as UDWORD);
            }
        }
        10310 => {
            widgSetButtonState(psWScreen, 10310 as libc::c_int as UDWORD,
                               0x2 as libc::c_int as UDWORD);
            widgSetButtonState(psWScreen, 10311 as libc::c_int as UDWORD,
                               0 as libc::c_int as UDWORD);
            game.fog = 1 as libc::c_int;
            if bHosted != 0 {
                sendOptions(0 as libc::c_int, 0 as libc::c_int as UDWORD);
            }
        }
        10311 => {
            widgSetButtonState(psWScreen, 10310 as libc::c_int as UDWORD,
                               0 as libc::c_int as UDWORD);
            widgSetButtonState(psWScreen, 10311 as libc::c_int as UDWORD,
                               0x2 as libc::c_int as UDWORD);
            game.fog = 0 as libc::c_int;
            if bHosted != 0 {
                sendOptions(0 as libc::c_int, 0 as libc::c_int as UDWORD);
            }
        }
        10267 => {
            /*		case MULTIOP_TECH_LOW:
			widgSetButtonState(psWScreen, MULTIOP_TECH_LOW,WBUT_LOCK);
			widgSetButtonState(psWScreen, MULTIOP_TECH_MED,0);
			widgSetButtonState(psWScreen, MULTIOP_TECH_HI ,0);
			game.techLevel = 1;
			if(bHosted)
			{
				sendOptions(0,0);
			}
			break;

		case MULTIOP_TECH_MED:
			widgSetButtonState(psWScreen, MULTIOP_TECH_LOW,0);
			widgSetButtonState(psWScreen, MULTIOP_TECH_MED,WBUT_LOCK);
			widgSetButtonState(psWScreen, MULTIOP_TECH_HI ,0);
			game.techLevel = 2;
			if(bHosted)
			{
				sendOptions(0,0);
			}
			break;

		case MULTIOP_TECH_HI:
			widgSetButtonState(psWScreen, MULTIOP_TECH_LOW,0);
			widgSetButtonState(psWScreen, MULTIOP_TECH_MED,0);
			widgSetButtonState(psWScreen, MULTIOP_TECH_HI ,WBUT_LOCK);
			game.techLevel = 3;
			if(bHosted)
			{
				sendOptions(0,0);
			}
			break;
*/
            game.base = 0 as libc::c_int as UBYTE; // 0;
            addGameOptions(0 as libc::c_int); //2;
            if bHosted != 0 {
                sendOptions(0 as libc::c_int, 0 as libc::c_int as UDWORD);
                disableMultiButs();
                addOkBut();
            }
        }
        10268 => {
            game.base = 1 as libc::c_int as UBYTE;
            addGameOptions(0 as libc::c_int);
            if bHosted != 0 {
                disableMultiButs();
                sendOptions(0 as libc::c_int, 0 as libc::c_int as UDWORD);
                addOkBut();
            }
        }
        10269 => {
            game.base = 2 as libc::c_int as UBYTE;
            addGameOptions(0 as libc::c_int);
            if bHosted != 0 {
                sendOptions(0 as libc::c_int, 0 as libc::c_int as UDWORD);
                disableMultiButs();
                addOkBut();
            }
        }
        10270 => {
            widgSetButtonState(psWScreen, 10270 as libc::c_int as UDWORD,
                               0x2 as libc::c_int as UDWORD);
            widgSetButtonState(psWScreen, 10271 as libc::c_int as UDWORD,
                               0 as libc::c_int as UDWORD);
            game.alliance = 0 as libc::c_int as UBYTE;
            if bHosted != 0 {
                sendOptions(0 as libc::c_int, 0 as libc::c_int as UDWORD);
            }
        }
        10271 => {
            widgSetButtonState(psWScreen, 10270 as libc::c_int as UDWORD,
                               0 as libc::c_int as UDWORD);
            widgSetButtonState(psWScreen, 10271 as libc::c_int as UDWORD,
                               0x2 as libc::c_int as UDWORD);
            game.alliance = 1 as libc::c_int as UBYTE;
            if bHosted != 0 {
                sendOptions(0 as libc::c_int, 0 as libc::c_int as UDWORD);
            }
        }
        10272 => {
            // set power level to low
            widgSetButtonState(psWScreen, 10272 as libc::c_int as UDWORD,
                               0x2 as libc::c_int as UDWORD);
            widgSetButtonState(psWScreen, 10273 as libc::c_int as UDWORD,
                               0 as libc::c_int as UDWORD);
            widgSetButtonState(psWScreen, 10274 as libc::c_int as UDWORD,
                               0 as libc::c_int as UDWORD);
            game.power = 100 as libc::c_int as UDWORD;
            if bHosted != 0 {
                sendOptions(0 as libc::c_int, 0 as libc::c_int as UDWORD);
            }
        }
        10273 => {
            // set power to med
            widgSetButtonState(psWScreen, 10272 as libc::c_int as UDWORD,
                               0 as libc::c_int as UDWORD);
            widgSetButtonState(psWScreen, 10273 as libc::c_int as UDWORD,
                               0x2 as libc::c_int as UDWORD);
            widgSetButtonState(psWScreen, 10274 as libc::c_int as UDWORD,
                               0 as libc::c_int as UDWORD);
            game.power = 400 as libc::c_int as UDWORD;
            if bHosted != 0 {
                sendOptions(0 as libc::c_int, 0 as libc::c_int as UDWORD);
            }
        }
        10274 => {
            // set power to hi
            widgSetButtonState(psWScreen, 10272 as libc::c_int as UDWORD,
                               0 as libc::c_int as UDWORD);
            widgSetButtonState(psWScreen, 10273 as libc::c_int as UDWORD,
                               0 as libc::c_int as UDWORD);
            widgSetButtonState(psWScreen, 10274 as libc::c_int as UDWORD,
                               0x2 as libc::c_int as UDWORD);
            game.power = 700 as libc::c_int as UDWORD;
            if bHosted != 0 {
                sendOptions(0 as libc::c_int, 0 as libc::c_int as UDWORD);
            }
        }
        10289 => {
            // set power level to low
            widgSetButtonState(psWScreen, 10289 as libc::c_int as UDWORD,
                               0x2 as libc::c_int as UDWORD);
            widgSetButtonState(psWScreen, 10290 as libc::c_int as UDWORD,
                               0 as libc::c_int as UDWORD);
            widgSetButtonState(psWScreen, 10291 as libc::c_int as UDWORD,
                               0 as libc::c_int as UDWORD);
            game.limit = 0 as libc::c_int as UBYTE;
            if bHosted != 0 {
                sendOptions(0 as libc::c_int, 0 as libc::c_int as UDWORD);
            }
        }
        10290 => {
            // set power to med
            widgSetButtonState(psWScreen, 10289 as libc::c_int as UDWORD,
                               0 as libc::c_int as UDWORD);
            widgSetButtonState(psWScreen, 10290 as libc::c_int as UDWORD,
                               0x2 as libc::c_int as UDWORD);
            widgSetButtonState(psWScreen, 10291 as libc::c_int as UDWORD,
                               0 as libc::c_int as UDWORD);
            game.limit = 1 as libc::c_int as UBYTE;
            if bHosted != 0 {
                sendOptions(0 as libc::c_int, 0 as libc::c_int as UDWORD);
            }
        }
        10291 => {
            // set power to hi
            widgSetButtonState(psWScreen, 10289 as libc::c_int as UDWORD,
                               0 as libc::c_int as UDWORD);
            widgSetButtonState(psWScreen, 10290 as libc::c_int as UDWORD,
                               0 as libc::c_int as UDWORD);
            widgSetButtonState(psWScreen, 10291 as libc::c_int as UDWORD,
                               0x2 as libc::c_int as UDWORD);
            game.limit = 2 as libc::c_int as UBYTE;
            if bHosted != 0 {
                sendOptions(0 as libc::c_int, 0 as libc::c_int as UDWORD);
            }
        }
        _ => { }
    }
    // these work all the time.
    match id {
        10312 => {
            if bWhiteBoardUp != 0 {
                removeWhiteBoard(); // del text too,
            } else { addWhiteBoard(); }
        }
        10277 => { changeTitleMode(MULTILIMIT); }
        10257 => {
            strcpy(sForceName.as_mut_ptr(),
                   widgGetString(psWScreen, 10257 as libc::c_int as UDWORD));
            removeWildcards(sForceName.as_mut_ptr());
        }
        10256 => {
            widgDelete(psWScreen, 10231 as libc::c_int as UDWORD);
            widgDelete(psWScreen, 20028 as libc::c_int as UDWORD);
            strcpy(tmp.as_mut_ptr(), MultiForcesPath.as_mut_ptr());
            strcat(tmp.as_mut_ptr(),
                   b"*.for\x00" as *const u8 as *const libc::c_char);
            addMultiRequest(tmp.as_mut_ptr(), 10257 as libc::c_int as UDWORD,
                            0 as libc::c_int as UBYTE,
                            0 as libc::c_int as UBYTE);
        }
        10253 => {
            strcpy(sPlayer.as_mut_ptr(),
                   widgGetString(psWScreen, 10253 as libc::c_int as UDWORD));
            // chop to 15 chars..
            while strlen(sPlayer.as_mut_ptr()) >
                      15 as libc::c_int as libc::c_uint {
                // clip name.
                sPlayer[strlen(sPlayer.as_mut_ptr()).wrapping_sub(1 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint)
                            as usize] = '\u{0}' as i32 as libc::c_char
            }
            // update string.
            widgSetString(psWScreen, 10253 as libc::c_int as UDWORD,
                          sPlayer.as_mut_ptr()); // update if joined.
            removeWildcards(sPlayer.as_mut_ptr() as
                                *mut STRING); // del text too,
            sprintf(tmp.as_mut_ptr(),
                    b"-> %s\x00" as *const u8 as *const libc::c_char,
                    sPlayer.as_mut_ptr()); // game name
            sendTextMessage(tmp.as_mut_ptr(), 1 as libc::c_int); // pname
            NETchangePlayerName(NetPlay.dpidPlayer as UDWORD,
                                sPlayer.as_mut_ptr() as
                                    *mut STRING); // add the name
            loadMultiStats(sPlayer.as_mut_ptr() as *mut STRING,
                           &mut playerStats);
            setMultiStats(NetPlay.dpidPlayer, playerStats, 0 as libc::c_int);
            setMultiStats(NetPlay.dpidPlayer, playerStats, 1 as libc::c_int);
        }
        10252 => {
            widgDelete(psWScreen, 10231 as libc::c_int as UDWORD);
            widgDelete(psWScreen, 20028 as libc::c_int as UDWORD);
            strcpy(tmp.as_mut_ptr(), MultiPlayersPath.as_mut_ptr());
            strcat(tmp.as_mut_ptr(),
                   b"*.sta\x00" as *const u8 as *const libc::c_char);
            addMultiRequest(tmp.as_mut_ptr(), 10253 as libc::c_int as UDWORD,
                            0 as libc::c_int as UBYTE,
                            0 as libc::c_int as UBYTE);
        }
        10276 => {
            if game.type_0 as libc::c_int != 14 as libc::c_int {
                strcpy(sForceName.as_mut_ptr(),
                       widgGetString(psWScreen,
                                     10257 as libc::c_int as UDWORD));
            }
            strcpy(game.name.as_mut_ptr(),
                   widgGetString(psWScreen, 10255 as libc::c_int as UDWORD));
            strcpy(sPlayer.as_mut_ptr() as *mut STRING,
                   widgGetString(psWScreen, 10253 as libc::c_int as UDWORD));
            strcpy(game.map.as_mut_ptr(),
                   widgGetString(psWScreen, 10259 as libc::c_int as UDWORD));
            removeWildcards(sPlayer.as_mut_ptr() as *mut STRING);
            removeWildcards(sForceName.as_mut_ptr());
            //		if (game.type == DMATCH)
//		{
//			hostArena((STRING*)game.name,(STRING*)sPlayer);
//			bHosted = TRUE;
//		}
//		else
//		{
            hostCampaign(game.name.as_mut_ptr(),
                         sPlayer.as_mut_ptr() as *mut STRING);
            bHosted = 1 as libc::c_int;
            //		}
            // wait for players, when happy, send options.
            if NetPlay.bLobbyLaunched != 0 {
                i = 0 as libc::c_int as UDWORD;
                while i < 8 as libc::c_int as libc::c_uint {
                    // send options to everyone.
                    if isHumanPlayer(i) != 0 {
                        sendOptions(player2dpid[i as usize],
                                    i); // update game options box.
                    }
                    i = i.wrapping_add(1)
                }
            }
            widgDelete(psWScreen, 10275 as libc::c_int as UDWORD);
            widgDelete(psWScreen, 10276 as libc::c_int as UDWORD);
            ingame.localOptionsReceived = 1 as libc::c_int;
            addGameOptions(0 as libc::c_int);
            addChatBox();
            addOkBut();
            disableMultiButs();
        }
        10279 => {
            // don't send empty lines to other players in the lobby
            if !(strcmp(widgGetString(psWScreen,
                                      10279 as libc::c_int as UDWORD),
                        b"\x00" as *const u8 as *const libc::c_char) == 0) {
                sendTextMessage(widgGetString(psWScreen,
                                              10279 as libc::c_int as UDWORD),
                                1 as libc::c_int); //send
                widgSetString(psWScreen, 10279 as libc::c_int as UDWORD,
                              b"\x00" as *const u8 as *const libc::c_char as
                                  *mut STRING); // clear box
            }
        }
        10101 => {
            decideWRF(); // set up swrf & game.map
            strcpy(sForceName.as_mut_ptr(),
                   widgGetString(psWScreen, 10257 as libc::c_int as UDWORD));
            removeWildcards(sForceName.as_mut_ptr());
            bMultiPlayer = 1 as libc::c_int;
            if NetPlay.bHost != 0 {
                // /////////
			//
                if game.type_0 as libc::c_int == 14 as libc::c_int {
                    chooseSkirmishColours();
                    sendOptions(0 as libc::c_int, 0 as libc::c_int as UDWORD);
                }
                //bcast a fireup message
                resetCheatHash();
                NEThaltJoining();
                SendFireUp();
            }
            // end of skirmish col choose
			// //////
            // stop new players entering.
            // set the fog correctly..
            setRevealStatus(game.fog);
            if bWhiteBoardUp != 0 { removeWhiteBoard(); }
            changeTitleMode(STARTGAME);
            bHosted = 0 as libc::c_int;
            if NetPlay.bHost != 0 {
                sendTextMessage(strresGetString(psStringRes,
                                                STR_MUL_STARTING as
                                                    libc::c_int as UDWORD),
                                1 as libc::c_int);
            }
            return
        }
        10102 => { stopJoining(); }
        _ => { }
    }
    if id >= 10232 as libc::c_int as libc::c_uint &&
           id <= 10249 as libc::c_int as libc::c_uint {
        // clicked on a player
        if NetPlay.players[id.wrapping_sub(10232 as libc::c_int as
                                               libc::c_uint) as usize].dpid ==
               player2dpid[selectedPlayer as usize] {
            addColourChooser(id.wrapping_sub(10232 as libc::c_int as
                                                 libc::c_uint));
        } else if NetPlay.bHost != 0 {
            if mouseDown(MOUSE_RMB) != 0 {
                // options for kick out, etc..
                // both buttons....
                kickPlayer(NetPlay.players[id.wrapping_sub(10232 as
                                                               libc::c_int as
                                                               libc::c_uint)
                                               as usize].dpid);
                // kick out that player.
            }
        }
    }
    if id >= 10313 as libc::c_int as libc::c_uint &&
           id <= 10320 as libc::c_int as libc::c_uint {
        // choseskirmish difficulty.
        game.skDiff[id.wrapping_sub(10313 as libc::c_int as libc::c_uint) as
                        usize] = widgGetSliderPos(psWScreen, id) as UBYTE;
        if id.wrapping_sub(10313 as libc::c_int as libc::c_uint) ==
               (game.maxPlayers as libc::c_int - 1 as libc::c_int) as
                   libc::c_uint &&
               game.skDiff[id.wrapping_sub(10313 as libc::c_int as
                                               libc::c_uint) as usize] as
                   libc::c_int == 0 as libc::c_int {
            game.skDiff[id.wrapping_sub(10313 as libc::c_int as libc::c_uint)
                            as usize] = 1 as libc::c_int as UBYTE;
            widgSetSliderPos(psWScreen, id, 1 as libc::c_int as UWORD);
        }
        sendOptions(0 as libc::c_int, 0 as libc::c_int as UDWORD);
    }
    // don't kill last player
    if id >= 10281 as libc::c_int as libc::c_uint &&
           id <= 10288 as libc::c_int as libc::c_uint {
        // chose a new colour.
        SendColourRequest(selectedPlayer,
                          id.wrapping_sub(10281 as libc::c_int as
                                              libc::c_uint) as UBYTE,
                          0xff as libc::c_int as UBYTE);
        closeColourChooser();
        addPlayerBox((ingame.bHostSetup == 0 || bHosted != 0) as libc::c_int);
    }
    // request a player number.
    if id >= 10321 as libc::c_int as libc::c_uint &&
           id <= 10330 as libc::c_int as libc::c_uint {
        // chose a new colour.
        SendColourRequest(selectedPlayer, 0xff as libc::c_int as UBYTE,
                          id.wrapping_sub(10321 as libc::c_int as
                                              libc::c_uint) as UBYTE);
        closeColourChooser();
        addPlayerBox((ingame.bHostSetup == 0 || bHosted != 0) as libc::c_int);
    };
}
// ////////////////////////////////////////////////////////////////////////////
// Net message handling
#[no_mangle]
pub unsafe extern "C" fn frontendMultiMessages() {
    let mut msg: NETMSG =
        NETMSG{size: 0,
               paddedBytes: 0,
               type_0: 0,
               destination: 0,
               body: [0; 8000],}; // a blank msg.
    let mut i: UDWORD = 0;
    let mut dp: DPID = 0;
    let mut bTemp: UBYTE = 0;
    while NETrecv(&mut msg) != 0 {
        let mut current_block_44: u64;
        match msg.type_0 as libc::c_int {
            255 => {
                recvAudioMsg(&mut msg);
                current_block_44 = 9353995356876505083;
            }
            46 => {
                recvMapFileRequested(&mut msg);
                current_block_44 = 9353995356876505083;
            }
            254 => {
                recvMapFileData(&mut msg);
                current_block_44 = 9353995356876505083;
            }
            26 => {
                // incoming options file.
                recvOptions(&mut msg);
                ingame.localOptionsReceived = 1 as libc::c_int;
                if titleMode as libc::c_uint ==
                       MULTIOPTION as libc::c_int as libc::c_uint {
                    addGameOptions(1 as libc::c_int);
                    disableMultiButs();
                    addChatBox();
                }
                current_block_44 = 9353995356876505083;
            }
            40 => {
                if bWhiteBoardUp == 0 { addWhiteBoard(); }
                memcpy(&mut *whiteBoard.as_mut_ptr().offset(*msg.body.as_mut_ptr().offset(0
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              isize)
                                                                as libc::c_int
                                                                as isize) as
                           *mut [UWORD; 150] as *mut libc::c_void,
                       &mut *msg.body.as_mut_ptr().offset(1 as libc::c_int as
                                                              isize) as
                           *mut libc::c_char as *const libc::c_void,
                       (150 as libc::c_int * 2 as libc::c_int) as
                           libc::c_uint);
                current_block_44 = 9353995356876505083;
            }
            30 => {
                recvAlliance(&mut msg, 0 as libc::c_int);
                current_block_44 = 9353995356876505083;
            }
            33 => {
                recvColourRequest(&mut msg);
                current_block_44 = 9353995356876505083;
            }
            8 => {
                // diagnostic ping msg.
                recvPing(&mut msg);
                current_block_44 = 9353995356876505083;
            }
            18 => {
                // remote player leaving.
                memcpy(&mut dp as *mut DPID as *mut libc::c_void,
                       &mut *msg.body.as_mut_ptr().offset(0 as libc::c_int as
                                                              isize) as
                           *mut libc::c_char as *const libc::c_void,
                       ::std::mem::size_of::<DPID>() as libc::c_ulong);
                memcpy(&mut bTemp as *mut UBYTE as *mut libc::c_void,
                       &mut *msg.body.as_mut_ptr().offset(4 as libc::c_int as
                                                              isize) as
                           *mut libc::c_char as *const libc::c_void,
                       ::std::mem::size_of::<UBYTE>() as libc::c_ulong);
                MultiPlayerLeave(dp);
                if bTemp != 0 {
                    // host has quit, need to quit too.
                    stopJoining();
                }
                current_block_44 = 9353995356876505083;
            }
            25 => {
                // remote player is now playing.
                memcpy(&mut i as *mut UDWORD as *mut libc::c_void,
                       &mut *msg.body.as_mut_ptr().offset(0 as libc::c_int as
                                                              isize) as
                           *mut libc::c_char as *const libc::c_void,
                       ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
                ingame.JoiningInProgress[i as usize] = 0 as libc::c_int;
                current_block_44 = 9353995356876505083;
            }
            29 => {
                // campaign game started.. can fire the whole shebang up...
                if ingame.localOptionsReceived != 0 {
                    resetCheatHash();
                    decideWRF();
                    if game.type_0 as libc::c_int != 14 as libc::c_int {
                        // force stuff.
                        strcpy(sForceName.as_mut_ptr(),
                               widgGetString(psWScreen,
                                             10257 as libc::c_int as UDWORD));
                        removeWildcards(sForceName.as_mut_ptr());
                    }
                    // set the fog correctly..
                    setRevealStatus(game.fog);
                    bMultiPlayer = 1 as libc::c_int;
                    if bWhiteBoardUp != 0 { removeWhiteBoard(); }
                    changeTitleMode(STARTGAME);
                    bHosted = 0 as libc::c_int;
                    current_block_44 = 9353995356876505083;
                } else { current_block_44 = 5204762799467536617; }
            }
            27 => { current_block_44 = 5204762799467536617; }
            17 => {
                // Chat message
                if ingame.localOptionsReceived != 0 {
                    recvTextMessage(&mut msg);
                }
                current_block_44 = 9353995356876505083;
            }
            _ => { current_block_44 = 9353995356876505083; }
        }
        match current_block_44 {
            5204762799467536617 => {
                // player is forcing someone to leave
                memcpy(&mut dp as *mut DPID as *mut libc::c_void,
                       &mut *msg.body.as_mut_ptr().offset(0 as libc::c_int as
                                                              isize) as
                           *mut libc::c_char as *const libc::c_void,
                       ::std::mem::size_of::<DPID>() as libc::c_ulong);
                if NetPlay.dpidPlayer == dp {
                    // we've been told to leave.
                    stopJoining(); //,i;
                }
            }
            _ => { }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn runMultiOptions() {
    static mut lastrefresh: UDWORD = 0 as libc::c_int as UDWORD;
    let mut id: UDWORD = 0;
    let mut value: UDWORD = 0;
    let mut sTemp: [STRING; 128] = [0; 128];
    let mut playerStats: PLAYERSTATS =
        PLAYERSTATS{played: 0,
                    wins: 0,
                    loses: 0,
                    totalKills: 0,
                    totalScore: 0,
                    recentKills: 0,
                    recentScore: 0,
                    killsToAdd: 0,
                    scoreToAdd: 0,};
    let mut context: W_CONTEXT =
        W_CONTEXT{psScreen: 0 as *mut W_SCREEN,
                  psForm: 0 as *mut _w_form,
                  xOffset: 0,
                  yOffset: 0,
                  mx: 0,
                  my: 0,};
    let mut k: KEY_CODE = 0 as KEY_CODE;
    let mut str: [STRING; 3] = [0; 3];
    processFrontendSnap(0 as libc::c_int);
    frontendMultiMessages();
    // keep sending the map if required.
    if bSendingMap != 0 { sendMap(); }
    // update boxes?
    if lastrefresh > gameTime { lastrefresh = 0 as libc::c_int as UDWORD }
    if gameTime.wrapping_sub(lastrefresh) >
           2000 as libc::c_int as libc::c_uint {
        lastrefresh = gameTime;
        if multiRequestUp == 0 &&
               (bHosted != 0 ||
                    ingame.localJoiningInProgress != 0 &&
                        NetPlay.bLobbyLaunched == 0 ||
                    NetPlay.bLobbyLaunched != 0 &&
                        ingame.localOptionsReceived != 0) {
            // store the slider settings if they are up,
            id = 0 as libc::c_int as UDWORD;
            while id < 8 as libc::c_int as libc::c_uint {
                if !widgGetFromID(psWScreen,
                                  (10313 as libc::c_int as
                                       libc::c_uint).wrapping_add(id)).is_null()
                   {
                    value =
                        widgGetSliderPos(psWScreen,
                                         (10313 as libc::c_int as
                                              libc::c_uint).wrapping_add(id));
                    if value != game.skDiff[id as usize] as libc::c_uint {
                        if value == 0 as libc::c_int as libc::c_uint &&
                               id ==
                                   (game.maxPlayers as libc::c_int -
                                        1 as libc::c_int) as libc::c_uint {
                            game.skDiff[id as usize] =
                                1 as libc::c_int as UBYTE;
                            widgSetSliderPos(psWScreen,
                                             id.wrapping_add(10313 as
                                                                 libc::c_int
                                                                 as
                                                                 libc::c_uint),
                                             1 as libc::c_int as UWORD);
                        } else { game.skDiff[id as usize] = value as UBYTE }
                        if NetPlay.bHost != 0 {
                            sendOptions(0 as libc::c_int,
                                        0 as libc::c_int as UDWORD);
                        }
                    }
                }
                id = id.wrapping_add(1)
            }
            addPlayerBox(1 as libc::c_int);
            // update the player box.
        }
    }
    // NET AUDIO CAPTURE
    if ingame.localJoiningInProgress != 0 &&
           game.bytesPerSec as libc::c_int == 2000 as libc::c_int {
        if keyPressed(KEY_KP_FULLSTOP) != 0 && NetPlay.bCaptureInUse == 0 {
            // noone else talking.
            NETstartAudioCapture();
        }
        if keyReleased(KEY_KP_FULLSTOP) != 0 {
            // manage the capture buffer
            // stop capture
            NETstopAudioCapture();
        }
        NETprocessAudioCapture();
    }
    // update scores and pings if far enough into the game
    if ingame.localOptionsReceived != 0 && ingame.localJoiningInProgress != 0
       {
        sendScoreCheck();
        sendPing();
    }
    // check for being able to go!
//	if(ingame.localJoiningInProgress
//		&& (widgGetFromID(psWScreen,CON_OK) == NULL)	// oks not yet there.
//		&& ingame.localOptionsReceived					// weve got the options
//		&& (game.type == DMATCH))						// it's a dmatch game
//	{
//		for(i=0;i<MAX_PLAYERS;i++)
//		{
//			if( isHumanPlayer(i) && !ingame.JoiningInProgress[i])			// only go when someones ready..
//			{
//				addOkBut();
//				break;
//			}
//		}
//	}
    // if typing and not in an edit box then jump to chat box.
    k = getQwertyKey(); // start it up!
    if k as libc::c_uint != 0 && (*psWScreen).psFocus.is_null() {
        context.psScreen = psWScreen;
        context.psForm = (*psWScreen).psForm as *mut W_FORM;
        context.xOffset = 0 as libc::c_int;
        context.yOffset = 0 as libc::c_int;
        context.mx = mouseX();
        context.my = mouseY();
        keyScanToString(k, &mut str as *mut [STRING; 3] as *mut libc::c_char,
                        3 as libc::c_int as UDWORD);
        if !widgGetFromID(psWScreen, 10279 as libc::c_int as UDWORD).is_null()
           {
            widgSetString(psWScreen, 10279 as libc::c_int as UDWORD,
                          &mut str as *mut [STRING; 3] as *mut libc::c_char);
            editBoxClicked(widgGetFromID(psWScreen,
                                         10279 as libc::c_int as UDWORD) as
                               *mut W_EDITBOX, &mut context);
        }
    }
    // chat box handling
    if !widgGetFromID(psWScreen, 10278 as libc::c_int as UDWORD).is_null() {
        while getNumberConsoleMessages() > getConsoleLineInfo() {
            removeTopConsoleMessage();
        }
        updateConsoleMessages();
        // run the chatbox
    }
    // widget handling
    if multiRequestUp != 0 {
        id = widgRunScreen(psRScreen); // a requester box is up.
        if runMultiRequester(id, &mut id,
                             &mut sTemp as *mut [STRING; 128] as
                                 *mut libc::c_char, &mut value) != 0 {
            match id {
                10253 => {
                    strcpy(sPlayer.as_mut_ptr() as *mut STRING,
                           sTemp.as_mut_ptr()); // run the widgets.
                    widgSetString(psWScreen, 10253 as libc::c_int as UDWORD,
                                  sTemp.as_mut_ptr()); // show the widgets currently running
                    sprintf(sTemp.as_mut_ptr(),
                            b" -> %s\x00" as *const u8 as *const libc::c_char,
                            sPlayer.as_mut_ptr());
                    sendTextMessage(sTemp.as_mut_ptr(), 1 as libc::c_int);
                    NETchangePlayerName(NetPlay.dpidPlayer as UDWORD,
                                        sPlayer.as_mut_ptr() as *mut STRING);
                    loadMultiStats(sPlayer.as_mut_ptr() as *mut STRING,
                                   &mut playerStats);
                    setMultiStats(NetPlay.dpidPlayer, playerStats,
                                  0 as libc::c_int);
                    setMultiStats(NetPlay.dpidPlayer, playerStats,
                                  1 as libc::c_int);
                }
                10257 => {
                    strcpy(sForceName.as_mut_ptr(), sTemp.as_mut_ptr());
                    widgSetString(psWScreen, 10257 as libc::c_int as UDWORD,
                                  sTemp.as_mut_ptr());
                }
                10259 => {
                    strcpy(game.map.as_mut_ptr(), sTemp.as_mut_ptr());
                    game.maxPlayers = value as UBYTE;
                    loadMapPreview();
                    widgSetString(psWScreen, 10259 as libc::c_int as UDWORD,
                                  sTemp.as_mut_ptr());
                    addGameOptions(0 as libc::c_int);
                }
                _ => { }
            }
            addPlayerBox((ingame.bHostSetup == 0) as libc::c_int);
        }
    } else {
        if hideTime != 0 as libc::c_int as libc::c_uint {
            if gameTime.wrapping_sub(hideTime) <
                   1500 as libc::c_int as libc::c_uint {
                return
            }
            hideTime = 0 as libc::c_int as UDWORD
        }
        id = widgRunScreen(psWScreen);
        processMultiopWidgets(id);
    }
    StartCursorSnap(&mut InterfaceSnap);
    DrawBegin();
    widgDisplayScreen(psWScreen);
    if multiRequestUp != 0 {
        widgDisplayScreen(psRScreen);
        // show the Requester running
    } // switch to small font.
    if !widgGetFromID(psWScreen, 10278 as libc::c_int as UDWORD).is_null() {
        iV_SetFont(WFont);
        displayConsoleMessages();
        // draw the chatbox
    }
    runWhiteBoard();
    //	if(psMapTiles)
//	{
//		displayMapPreview();
//	}
    DrawEnd();
}
// ////////////////////////////////////////////////////////////////////////////
#[no_mangle]
pub unsafe extern "C" fn startMultiOptions(mut bReenter: BOOL) -> BOOL {
    let mut nullStats: PLAYERSTATS =
        PLAYERSTATS{played: 0,
                    wins: 0,
                    loses: 0,
                    totalKills: 0,
                    totalScore: 0,
                    recentKills: 0,
                    recentScore: 0,
                    killsToAdd: 0,
                    scoreToAdd: 0,}; // get favourite options
    let mut i: UBYTE = 0; // force a colour clearout.
    addBackdrop();
    addTopForm();
    if bReenter == 0 {
        loadConfig(1 as libc::c_int);
        initPlayerColours();
        i = 0 as libc::c_int as UBYTE;
        while (i as libc::c_int) < 8 as libc::c_int {
            //			game.skirmishPlayers[i] = 1; // clear out skirmish setting
//			game.skDiff[i] = (rand()%19)+1;	//1-20
            game.skDiff[i as usize] = 10 as libc::c_int as UBYTE;
            i = i.wrapping_add(1)
        }
        /*		//set defaults for game.
		game.power					= LEV_MED;
		if(NetPlay.bComms)
		{
			game.type				= CAMPAIGN;
			strcpy(game.map, DEFAULTCAMPAIGNMAP);
		}
		else
		{
			game.type				= SKIRMISH;
			strcpy(game.map, DEFAULTSKIRMISHMAP);
		}
		game.techLevel				= 1;
		game.base					= CAMP_BASE;
//		game.bHaveServer			= FALSE;
		game.limit					= NOLIMIT;
//		game.packetsPerSec			= 6;
		game.maxPlayers				= 4;
		game.bComputerPlayers		= FALSE;
		strcpy(sForceName,	"Default");
*/
        // set the encrypt key.
        if NetPlay.bHost != 0 { game.encryptKey = 0 as libc::c_int as UBYTE }
        if NetPlay.bComms == 0 {
            // firce skirmish if no comms.
            game.type_0 = 14 as libc::c_int as UBYTE; // note buildtime.
            strcpy(game.map.as_mut_ptr(),
                   b"Sk-Rush\x00" as *const u8 as *const libc::c_char);
            game.maxPlayers = 4 as libc::c_int as UBYTE
        }
        strncpy(game.version.as_mut_ptr(), buildTime.as_mut_ptr(),
                8 as libc::c_int as libc::c_uint);
        ingame.localOptionsReceived = 0 as libc::c_int;
        if ingame.numStructureLimits != 0 {
            ingame.numStructureLimits = 0 as libc::c_int as UDWORD;
            memFreeRelease(ingame.pStructureLimits as *mut libc::c_void);
            ingame.pStructureLimits = 0 as *mut UBYTE
        }
        // check the registry for setup entries and set game options.
//#ifndef NOREGCHECK
//		NETcheckRegistryEntries("Warzone2100",S_WARZONEGUID);		// check for registry entries.. warn if not ok...
//#endif
        if NetPlay.bLobbyLaunched != 0 {
            game.bytesPerSec =
                1201 as libc::c_int as
                    UWORD; // maximum bitrate achieved before dropping checks.
            game.packetsPerSec = 5 as libc::c_int as UBYTE
        } // Players
        loadMultiStats(sPlayer.as_mut_ptr() as *mut STRING,
                       &mut nullStats); //whiteboard icon
    }
    addPlayerBox(0 as libc::c_int);
    addGameOptions(0 as libc::c_int);
    if NetPlay.bLobbyLaunched != 0 {
        if NetPlay.bHost == 0 {
            ingame.localJoiningInProgress = 1 as libc::c_int;
            widgDelete(psWScreen, 10275 as libc::c_int as UDWORD);
        }
        ingame.localOptionsReceived = 0 as libc::c_int
    }
    addChatBox();
    addMultiBut(psWScreen, 20000 as libc::c_int as UDWORD,
                10312 as libc::c_int as UDWORD,
                (40 as libc::c_int - 15 as libc::c_int) as UDWORD,
                (350 as libc::c_int + 115 as libc::c_int - 15 as libc::c_int)
                    as UDWORD, 9 as libc::c_int as UDWORD,
                9 as libc::c_int as UDWORD, 0 as libc::c_int as UDWORD,
                IMAGE_PENCIL as libc::c_int as UDWORD,
                IMAGE_PENCIL as libc::c_int as UDWORD, 1 as libc::c_int);
    // going back to multiop after setting limits up..
    if bReenter != 0 && bHosted != 0 { disableMultiButs(); addOkBut(); }
    return 1 as libc::c_int;
}
// draw available templates
// ////////////////////////////////////////////////////////////////////////////
// ////////////////////////////////////////////////////////////////////////////
// Force Select Screen.
// ////////////////////////////////////////////////////////////////////////////
unsafe extern "C" fn CurrentForce() {
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
                   pFormDisplay: None,}; //???
    let mut numButtons: UDWORD = 0;
    let mut butPerForm: UDWORD = 0;
    let mut i: UDWORD = 0;
    let mut pF: *mut FORCE_MEMBER = 0 as *mut FORCE_MEMBER;
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
                   pFormDisplay: None,};
    let mut aButText: [STRING; 6] = [0; 6];
    let mut BufferID: SDWORD = 0;
    widgDelete(psWScreen, 10307 as libc::c_int as UDWORD);
    widgDelete(psWScreen, 20029 as libc::c_int as UDWORD);
    ClearObjectBuffers();
    /* Count the number of minor tabs needed for the template form
	 * Also need one for the new template button */
    numButtons = 0 as libc::c_int as UDWORD;
    pF = Force.pMembers;
    while !pF.is_null() {
        numButtons = numButtons.wrapping_add(1);
        pF = (*pF).psNext
    }
    /* Calculate how many buttons will go on a single form */
    butPerForm =
        ((138 as libc::c_int - 0 as libc::c_int - 2 as libc::c_int) /
             (60 as libc::c_int + 2 as libc::c_int) *
             ((255 as libc::c_int - 0 as libc::c_int - 2 as libc::c_int) /
                  (46 as libc::c_int + 2 as libc::c_int))) as UDWORD;
    /* add a form to place the tabbed form on */
    memset(&mut sFormInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_FORMINIT>() as libc::c_ulong);
    sFormInit.formID = 20000 as libc::c_int as UDWORD;
    sFormInit.id = 10307 as libc::c_int as UDWORD;
    sFormInit.style = 0 as libc::c_int as UDWORD;
    sFormInit.x =
        (80 as libc::c_int + 138 as libc::c_int + 33 as libc::c_int +
             140 as libc::c_int + 33 as libc::c_int) as SWORD;
    sFormInit.y = 170 as libc::c_int as SWORD;
    sFormInit.width = 138 as libc::c_int as UWORD;
    sFormInit.height = (255 as libc::c_int + 4 as libc::c_int) as UWORD;
    sFormInit.pDisplay =
        Some(intDisplayPlainForm as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    widgAddForm(psWScreen, &mut sFormInit);
    addSideText(20029 as libc::c_int as UDWORD,
                (80 as libc::c_int + 138 as libc::c_int + 33 as libc::c_int +
                     140 as libc::c_int + 33 as libc::c_int -
                     1 as libc::c_int) as UDWORD,
                170 as libc::c_int as UDWORD,
                strresGetString(psStringRes,
                                STR_MUL_SIDEFORCE as libc::c_int as UDWORD));
    /* Add the design templates form */
    memset(&mut sFormInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_FORMINIT>() as
               libc::c_ulong); //(DES_TAB_HEIGHT/2)+2;
    sFormInit.formID = 10307 as libc::c_int as UDWORD;
    sFormInit.id = 10308 as libc::c_int as UDWORD;
    sFormInit.style = 1 as libc::c_int as UDWORD;
    sFormInit.x = 2 as libc::c_int as SWORD;
    sFormInit.y = 2 as libc::c_int as SWORD;
    sFormInit.width = 138 as libc::c_int as UWORD;
    sFormInit.height = 255 as libc::c_int as UWORD;
    sFormInit.numMajor = numForms(numButtons, butPerForm);
    sFormInit.majorPos = 1 as libc::c_int as UWORD;
    sFormInit.minorPos = 0 as libc::c_int as UWORD;
    sFormInit.majorSize = 26 as libc::c_int as UWORD;
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
    /* Put the buttons on it */
    memset(aButText.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           6 as libc::c_int as libc::c_uint); //??
    memset(&mut sButInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_BUTINIT>() as libc::c_ulong);
    /* Set up the button struct */
    sButInit.formID = 10308 as libc::c_int as UDWORD;
    sButInit.id = 10350 as libc::c_int as UDWORD;
    sButInit.style = 4 as libc::c_int as UDWORD;
    sButInit.x = 2 as libc::c_int as SWORD;
    sButInit.y = 2 as libc::c_int as SWORD;
    sButInit.width = 60 as libc::c_int as UWORD;
    sButInit.height = 46 as libc::c_int as UWORD;
    pF = Force.pMembers;
    while !pF.is_null() {
        /* Set the tip and add the button */
        strncpy(aButText.as_mut_ptr(), getTemplateName((*pF).pTempl),
                5 as libc::c_int as libc::c_uint);
        sButInit.pTip = getTemplateName((*pF).pTempl);
        BufferID = GetObjectBuffer();
        if BufferID >= 0 as libc::c_int {
        } else {
            debug(LOG_ERROR,
                  b"Unable to aquire Obj Buffer.\x00" as *const u8 as
                      *const libc::c_char);
        };
        if BufferID >= 0 as libc::c_int {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"multiint.c\x00" as *const u8 as *const libc::c_char,
                  2903 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 13],
                                            &[libc::c_char; 13]>(b"CurrentForce\x00")).as_ptr(),
                  b"BufferID >= 0\x00" as *const u8 as *const libc::c_char);
        };
        ObjectBuffers[BufferID as usize].InUse = 1 as libc::c_int;
        ObjectBuffers[BufferID as usize].Data =
            (*pF).pTempl as *mut libc::c_void;
        sButInit.pUserData =
            &mut *ObjectBuffers.as_mut_ptr().offset(BufferID as isize) as
                *mut RENDERED_BUTTON as *mut libc::c_void;
        sButInit.pDisplay =
            Some(intDisplayTemplateButton as
                     unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                          _: UDWORD, _: *mut UDWORD) -> ());
        widgAddForm(psWScreen, &mut sButInit);
        /* Update the init struct for the next button */
        sButInit.id =
            (sButInit.id as
                 libc::c_uint).wrapping_add(1 as libc::c_int as libc::c_uint)
                as UDWORD as UDWORD;
        sButInit.x =
            (sButInit.x as libc::c_int + 60 as libc::c_int + 2 as libc::c_int)
                as SWORD;
        if sButInit.x as libc::c_int + 60 as libc::c_int + 2 as libc::c_int >
               138 as libc::c_int {
            sButInit.x = 2 as libc::c_int as SWORD;
            sButInit.y =
                (sButInit.y as libc::c_int + 46 as libc::c_int +
                     2 as libc::c_int) as SWORD
        }
        if sButInit.y as libc::c_int + 46 as libc::c_int + 2 as libc::c_int >
               255 as libc::c_int {
            sButInit.y = 2 as libc::c_int as SWORD;
            sButInit.majorID =
                (sButInit.majorID as libc::c_int + 1 as libc::c_int) as UWORD
        }
        pF = (*pF).psNext
    };
}
// Force selection functions
// ////////////////////////////////
unsafe extern "C" fn AvailableForces() {
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
    let mut psTempl: *mut DROID_TEMPLATE = 0 as *mut DROID_TEMPLATE;
    let mut numButtons: UDWORD = 0;
    let mut butPerForm: UDWORD = 0;
    let mut i: UDWORD = 0;
    /* init template list */
    memset(apsTemplateList as *mut libc::c_void, 0 as libc::c_int,
           (::std::mem::size_of::<*mut DROID_TEMPLATE>() as
                libc::c_ulong).wrapping_mul(40 as libc::c_int as
                                                libc::c_uint));
    numButtons = 0 as libc::c_int as UDWORD;
    psTempl = apsDroidTemplates[0 as libc::c_int as usize];
    while !psTempl.is_null() && numButtons < 40 as libc::c_int as libc::c_uint
          {
        let ref mut fresh0 = *apsTemplateList.offset(numButtons as isize);
        *fresh0 = psTempl;
        /* Count the number of minor tabs needed for the template form */
        numButtons = numButtons.wrapping_add(1);
        /* next template */
        psTempl = (*psTempl).psNext
    }
    widgDelete(psWScreen, 5020 as libc::c_int as UDWORD);
    widgDelete(psWScreen, 20027 as libc::c_int as UDWORD);
    ClearStatBuffers();
    /* Calculate how many buttons will go on a single form */
    butPerForm =
        ((138 as libc::c_int - 0 as libc::c_int - 2 as libc::c_int) /
             (60 as libc::c_int + 2 as libc::c_int) *
             ((255 as libc::c_int - 0 as libc::c_int - 2 as libc::c_int) /
                  (46 as libc::c_int + 2 as libc::c_int))) as UDWORD;
    /* add a form to place the tabbed form on */
    memset(&mut sFormInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_FORMINIT>() as libc::c_ulong);
    sFormInit.formID = 20000 as libc::c_int as UDWORD;
    sFormInit.id = 5020 as libc::c_int as UDWORD;
    sFormInit.style = 0 as libc::c_int as UDWORD;
    sFormInit.x = 80 as libc::c_int as SWORD;
    sFormInit.y = 170 as libc::c_int as SWORD;
    sFormInit.width = 138 as libc::c_int as UWORD;
    sFormInit.height = (255 as libc::c_int + 4 as libc::c_int) as UWORD;
    sFormInit.pDisplay =
        Some(intDisplayPlainForm as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    widgAddForm(psWScreen, &mut sFormInit);
    addSideText(20027 as libc::c_int as UDWORD,
                (80 as libc::c_int - 1 as libc::c_int) as UDWORD,
                170 as libc::c_int as UDWORD,
                strresGetString(psStringRes,
                                STR_MUL_SIDETEMPLATES as libc::c_int as
                                    UDWORD));
    /* Add the design templates form */
    memset(&mut sFormInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_FORMINIT>() as libc::c_ulong);
    sFormInit.formID = 5020 as libc::c_int as UDWORD;
    sFormInit.id = 5002 as libc::c_int as UDWORD;
    sFormInit.style = 1 as libc::c_int as UDWORD;
    sFormInit.x = 2 as libc::c_int as SWORD;
    sFormInit.y = 2 as libc::c_int as SWORD;
    sFormInit.width = 138 as libc::c_int as UWORD;
    sFormInit.height = 255 as libc::c_int as UWORD;
    sFormInit.numMajor = numForms(numButtons, butPerForm);
    sFormInit.majorPos = 1 as libc::c_int as UWORD;
    sFormInit.minorPos = 0 as libc::c_int as UWORD;
    sFormInit.majorSize = 26 as libc::c_int as UWORD;
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
    /* Put the buttons on it */
    intAddTemplateButtons(5002 as libc::c_int as UDWORD,
                          138 as libc::c_int as UDWORD,
                          255 as libc::c_int as UDWORD,
                          60 as libc::c_int as UDWORD,
                          46 as libc::c_int as UDWORD,
                          2 as libc::c_int as UDWORD,
                          0 as *mut DROID_TEMPLATE);
}
// ////////////////////////////////////////////////////////////////////////////
#[no_mangle]
pub unsafe extern "C" fn runForceSelect() {
    let mut currID: UDWORD = 0;
    let mut id: UDWORD = 0;
    let mut i: UDWORD = 0;
    let mut psTempl: *mut DROID_TEMPLATE = 0 as *mut DROID_TEMPLATE;
    let mut posx: UDWORD = 0;
    let mut posy: UDWORD = 0;
    let mut pF: *mut FORCE_MEMBER = 0 as *mut FORCE_MEMBER;
    let mut dir: [STRING; 256] = [0; 256];
    posx = 5 as libc::c_int as UDWORD;
    posy = 5 as libc::c_int as UDWORD;
    processFrontendSnap(0 as libc::c_int);
    if bLoadSaveUp != 0 {
        if runLoadSave(0 as libc::c_int) != 0 {
            // check for file name.
            if strlen(sRequestResult.as_mut_ptr()) != 0 {
                debug(LOG_NEVER,
                      b"Returned %s\x00" as *const u8 as *const libc::c_char,
                      sRequestResult.as_mut_ptr()); // update force screen..
                if bRequestLoad != 0 {
                    loadForce(sRequestResult.as_mut_ptr()); // update force screen..
                    AvailableForces();
                    CurrentForce();
                } else {
                    saveForce(sRequestResult.as_mut_ptr(), &mut Force);
                    AvailableForces();
                    CurrentForce();
                }
            }
        }
    } else {
        // MouseOver stuff.
        id = widgGetMouseOver(psWScreen);
        if id >= 5300 as libc::c_int as libc::c_uint &&
               id <= 5339 as libc::c_int as libc::c_uint {
            currID = 5300 as libc::c_int as UDWORD;
            psTempl = apsDroidTemplates[selectedPlayer as usize];
            while !psTempl.is_null() {
                if currID == id { break ; }
                currID = currID.wrapping_add(1);
                psTempl = (*psTempl).psNext
            }
            intSetShadowPower(calcTemplatePower(psTempl));
            //update the power bars
        } else {
            intSetShadowPower(0 as libc::c_int as UDWORD); // run the widgets.
        }
        id = widgRunScreen(psWScreen);
        if id >= 5300 as libc::c_int as libc::c_uint &&
               id <= 5339 as libc::c_int as libc::c_uint {
            currID = 5300 as libc::c_int as UDWORD;
            psTempl = apsDroidTemplates[selectedPlayer as usize];
            while !psTempl.is_null() {
                if currID == id { break ; }
                currID = currID.wrapping_add(1);
                psTempl = (*psTempl).psNext
            }
            if addToForce(psTempl) == 0 {
                // store this droid in force list.
                widgDisplayScreen(psWScreen);
                return
            }
            CurrentForce();
        }
        if id >= 10350 as libc::c_int as libc::c_uint &&
               id <= 10420 as libc::c_int as libc::c_uint {
            // FORCE droid selected
            pF = Force.pMembers; // dont delete if it's frozen
            i = 0 as libc::c_int as UDWORD; // goto that force element;
            while i < id.wrapping_sub(10350 as libc::c_int as libc::c_uint) {
                pF = (*pF).psNext; //delete it from force.
                i = i.wrapping_add(1)
            }
            removeFromForce(id.wrapping_sub(10350 as libc::c_int as
                                                libc::c_uint));
            CurrentForce();
        }
        match id {
            10102 => {
                // dont continue, return to options
                while !Force.pMembers.is_null() {
                    removeFromForce(0 as libc::c_int as UDWORD);
                    // delete each force member.
                } // just in case.
                selectedPlayer = 0 as libc::c_int as UDWORD;
                changeTitleMode(MULTI);
                eventReset();
                //			resReleaseBlockData(500);
                resReleaseBlockData(501 as libc::c_int);
                resReleaseBlockData(502 as libc::c_int);
                bForceEditorLoaded = 0 as libc::c_int
            }
            10302 => {
                // clear force
                while !Force.pMembers.is_null() {
                    removeFromForce(0 as libc::c_int as UDWORD);
                    // delete each force member.
                } // update force screen
                AvailableForces(); // update force screen
                CurrentForce(); // show the widgets currently running
            }
            10301 => {
                strcpy(dir.as_mut_ptr(),
                       b"multiplay/forces/default.for\x00" as *const u8 as
                           *const libc::c_char);
                loadForce(dir.as_mut_ptr());
                AvailableForces();
                CurrentForce();
            }
            10300 => {
                addLoadSave(LOAD_FORCE, MultiForcesPath.as_mut_ptr(),
                            b"for\x00" as *const u8 as *const libc::c_char as
                                *mut CHAR,
                            strresGetString(psStringRes,
                                            STR_MUL_LOAD as libc::c_int as
                                                UDWORD));
            }
            10309 => {
                addLoadSave(SAVE_FORCE, MultiForcesPath.as_mut_ptr(),
                            b"for\x00" as *const u8 as *const libc::c_char as
                                *mut CHAR,
                            strresGetString(psStringRes,
                                            STR_MUL_SAVE as libc::c_int as
                                                UDWORD));
            }
            _ => { }
        }
    }
    StartCursorSnap(&mut InterfaceSnap);
    DrawBegin();
    widgDisplayScreen(psWScreen);
    if bLoadSaveUp != 0 { displayLoadSave(); }
    DrawEnd();
}
// ////////////////////////////////////////////////////////////////////////////
#[no_mangle]
pub unsafe extern "C" fn startForceSelect() -> BOOL {
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
                   pFormDisplay: None,}; // start with default force.
    let mut sBarInit: W_BARINIT =
        W_BARINIT{formID: 0,
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
                  orientation: 0,
                  size: 0,
                  minorSize: 0,
                  iRange: 0,
                  sCol: W_COLOURDEF{red: 0, green: 0, blue: 0, alpha: 0,},
                  sMinorCol:
                      W_COLOURDEF{red: 0, green: 0, blue: 0, alpha: 0,},
                  pTip: 0 as *mut STRING,};
    let mut dir: [STRING; 256] = [0; 256];
    let mut psTempl: *mut DROID_TEMPLATE = 0 as *mut DROID_TEMPLATE;
    copyTemplateSet(4 as libc::c_int as UDWORD, 0 as libc::c_int as UDWORD);
    initTemplatePoints();
    selectedPlayer = 0 as libc::c_int as UDWORD;
    setPower(selectedPlayer, 1200 as libc::c_int as UDWORD);
    strcpy(dir.as_mut_ptr(),
           b"multiplay/forces/default.for\x00" as *const u8 as
               *const libc::c_char);
    if loadForce(dir.as_mut_ptr()) == 0 {
        debug(LOG_NEVER,
              b"Error Loading Force\x00" as *const u8 as *const libc::c_char);
    }
    addBackdrop();
    addTopForm();
    // init power profiles.
    psTempl =
        apsDroidTemplates[0 as libc::c_int as usize]; // available forces
    while !psTempl.is_null() {
        (*psTempl).powerPoints = calcTemplatePower(psTempl); // current force
        psTempl = (*psTempl).psNext
    } // stats box
    AvailableForces(); // stats box
    CurrentForce();
    memset(&mut sFormInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_FORMINIT>() as libc::c_ulong);
    sFormInit.formID = 20000 as libc::c_int as UDWORD;
    sFormInit.id = 10303 as libc::c_int as UDWORD;
    sFormInit.style = 0 as libc::c_int as UDWORD;
    sFormInit.x =
        (80 as libc::c_int + 138 as libc::c_int + 33 as libc::c_int) as SWORD;
    sFormInit.y = 170 as libc::c_int as SWORD;
    sFormInit.width = 140 as libc::c_int as UWORD;
    sFormInit.height = 255 as libc::c_int as UWORD;
    sFormInit.pDisplay =
        Some(intDisplayPlainForm as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    widgAddForm(psWScreen, &mut sFormInit);
    addSideText(20028 as libc::c_int as UDWORD,
                (80 as libc::c_int + 138 as libc::c_int + 33 as libc::c_int -
                     1 as libc::c_int) as UDWORD,
                170 as libc::c_int as UDWORD,
                strresGetString(psStringRes,
                                STR_MUL_SIDEINFO as libc::c_int as UDWORD));
    sFormInit.formID = 10303 as libc::c_int as UDWORD;
    sFormInit.id = 10304 as libc::c_int as UDWORD;
    sFormInit.style = 0 as libc::c_int as UDWORD;
    sFormInit.x = 70 as libc::c_int as SWORD;
    sFormInit.y = 220 as libc::c_int as SWORD;
    sFormInit.width = 100 as libc::c_int as UWORD;
    sFormInit.height = 100 as libc::c_int as UWORD;
    sFormInit.pDisplay =
        Some(displayForceDroid as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    //	sFormInit.pUserData		= (VOID*)Force->pTempl;
    widgAddForm(psWScreen, &mut sFormInit);
    // name/techlevel/powerrequired/powerleft
    memset(&mut sBarInit as *mut W_BARINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_BARINIT>() as libc::c_ulong); //power bar
    sBarInit.formID = 20000 as libc::c_int as UDWORD; //IDPOW_FORM;
    sBarInit.id = 102 as libc::c_int as UDWORD; // cancel
    sBarInit.style = 1 as libc::c_int as UDWORD; // clear.
    sBarInit.orientation = 0x1 as libc::c_int as UWORD; // default.
    sBarInit.x =
        ((23 as libc::c_int + 132 as libc::c_int + 6 as libc::c_int) as
             libc::c_uint).wrapping_add(pie_GetVideoBufferWidth().wrapping_sub(640
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   libc::c_uint).wrapping_div(2
                                                                                                                  as
                                                                                                                  libc::c_int
                                                                                                                  as
                                                                                                                  libc::c_uint)).wrapping_sub(pie_GetVideoBufferWidth().wrapping_sub(640
                                                                                                                                                                                         as
                                                                                                                                                                                         libc::c_int
                                                                                                                                                                                         as
                                                                                                                                                                                         libc::c_uint).wrapping_div(2
                                                                                                                                                                                                                        as
                                                                                                                                                                                                                        libc::c_int
                                                                                                                                                                                                                        as
                                                                                                                                                                                                                        libc::c_uint))
            as SWORD; // load
    sBarInit.y =
        (324 as libc::c_int as
             libc::c_uint).wrapping_add(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint)).wrapping_add(115
                                                                                                                    as
                                                                                                                    libc::c_int
                                                                                                                    as
                                                                                                                    libc::c_uint).wrapping_add(6
                                                                                                                                                   as
                                                                                                                                                   libc::c_int
                                                                                                                                                   as
                                                                                                                                                   libc::c_uint).wrapping_sub(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                                                                                                                                                          as
                                                                                                                                                                                                                          libc::c_int
                                                                                                                                                                                                                          as
                                                                                                                                                                                                                          libc::c_uint))
            as SWORD; // save
    sBarInit.width = 308 as libc::c_int as UWORD;
    sBarInit.height =
        iV_GetImageHeight(IntImages,
                          IMAGE_PBAR_EMPTY as libc::c_int as UWORD);
    sBarInit.sCol.red = 0xcc as libc::c_int as UBYTE;
    sBarInit.sCol.green = 0 as libc::c_int as UBYTE;
    sBarInit.sCol.blue = 0 as libc::c_int as UBYTE;
    sBarInit.pDisplay =
        Some(intDisplayPowerBar as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    sBarInit.iRange =
        (5 as libc::c_int * 100 as libc::c_int /
             (60 as libc::c_int - 8 as libc::c_int)) as UWORD;
    widgAddBarGraph(psWScreen, &mut sBarInit);
    addMultiBut(psWScreen, 10303 as libc::c_int as UDWORD,
                10102 as libc::c_int as UDWORD, 10 as libc::c_int as UDWORD,
                10 as libc::c_int as UDWORD, 37 as libc::c_int as UDWORD,
                24 as libc::c_int as UDWORD,
                STR_MUL_CANCEL as libc::c_int as UDWORD,
                IMAGE_RETURN as libc::c_int as UDWORD,
                IMAGE_RETURN_HI as libc::c_int as UDWORD, 0 as libc::c_int);
    addMultiBut(psWScreen, 10303 as libc::c_int as UDWORD,
                10302 as libc::c_int as UDWORD, 11 as libc::c_int as UDWORD,
                93 as libc::c_int as UDWORD, 56 as libc::c_int as UDWORD,
                38 as libc::c_int as UDWORD,
                STR_MUL_CLEAR as libc::c_int as UDWORD,
                IMAGE_CLEARFORCE as libc::c_int as UDWORD,
                IMAGE_CLEARFORCE as libc::c_int as UDWORD, 1 as libc::c_int);
    addMultiBut(psWScreen, 10303 as libc::c_int as UDWORD,
                10301 as libc::c_int as UDWORD, 73 as libc::c_int as UDWORD,
                93 as libc::c_int as UDWORD, 56 as libc::c_int as UDWORD,
                38 as libc::c_int as UDWORD,
                STR_MUL_DEFAULT as libc::c_int as UDWORD,
                IMAGE_DEFAULTFORCE as libc::c_int as UDWORD,
                IMAGE_DEFAULTFORCE as libc::c_int as UDWORD,
                1 as libc::c_int);
    addMultiBut(psWScreen, 10303 as libc::c_int as UDWORD,
                10300 as libc::c_int as UDWORD, 73 as libc::c_int as UDWORD,
                50 as libc::c_int as UDWORD, 56 as libc::c_int as UDWORD,
                38 as libc::c_int as UDWORD,
                STR_MUL_LOAD as libc::c_int as UDWORD,
                IMAGE_LOADFORCE as libc::c_int as UDWORD,
                IMAGE_LOADFORCE as libc::c_int as UDWORD, 1 as libc::c_int);
    addMultiBut(psWScreen, 10303 as libc::c_int as UDWORD,
                10309 as libc::c_int as UDWORD, 11 as libc::c_int as UDWORD,
                50 as libc::c_int as UDWORD, 56 as libc::c_int as UDWORD,
                38 as libc::c_int as UDWORD,
                STR_MUL_SAVE as libc::c_int as UDWORD,
                IMAGE_SAVEFORCE as libc::c_int as UDWORD,
                IMAGE_SAVEFORCE as libc::c_int as UDWORD, 1 as libc::c_int);
    return 1 as libc::c_int;
}
// whiteboard funcs
// ///////////////////////////////////////////////////////////////////////////////////////
// ///////////////////////////////////////////////////////////////////////////////////////
// whiteboard functions
// removewhiteboard
unsafe extern "C" fn removeWhiteBoard() -> BOOL {
    bWhiteBoardUp = 0 as libc::c_int;
    widgReleaseScreen(psWhiteScreen);
    return 1 as libc::c_int;
}
// runwhiteboard
unsafe extern "C" fn runWhiteBoard() -> BOOL {
    let mut m: NETMSG =
        NETMSG{size: 0,
               paddedBytes: 0,
               type_0: 0,
               destination: 0,
               body: [0; 8000],};
    static mut lastSent: UDWORD = 0 as libc::c_int as UDWORD;
    let mut x: UDWORD = 0;
    let mut y: UDWORD = 0;
    let mut mx: UDWORD = 0;
    let mut my: UDWORD = 0;
    if bWhiteBoardUp == 0 { return 1 as libc::c_int }
    mx = mouseX() as UDWORD;
    my = mouseY() as UDWORD;
    widgDisplayScreen(psWhiteScreen);
    // if mouse over && mouse down.
    if mx >
           ((40 as libc::c_int + 5 as libc::c_int) as
                libc::c_uint).wrapping_add(pie_GetVideoBufferWidth().wrapping_sub(640
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      libc::c_uint).wrapping_div(2
                                                                                                                     as
                                                                                                                     libc::c_int
                                                                                                                     as
                                                                                                                     libc::c_uint))
           &&
           mx <
               ((40 as libc::c_int + 5 as libc::c_int) as
                    libc::c_uint).wrapping_add(pie_GetVideoBufferWidth().wrapping_sub(640
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          libc::c_uint).wrapping_div(2
                                                                                                                         as
                                                                                                                         libc::c_int
                                                                                                                         as
                                                                                                                         libc::c_uint)).wrapping_add((373
                                                                                                                                                          as
                                                                                                                                                          libc::c_int
                                                                                                                                                          +
                                                                                                                                                          250
                                                                                                                                                              as
                                                                                                                                                              libc::c_int
                                                                                                                                                          -
                                                                                                                                                          40
                                                                                                                                                              as
                                                                                                                                                              libc::c_int
                                                                                                                                                          -
                                                                                                                                                          10
                                                                                                                                                              as
                                                                                                                                                              libc::c_int)
                                                                                                                                                         as
                                                                                                                                                         libc::c_uint)
           &&
           my >
               ((350 as libc::c_int + 5 as libc::c_int) as
                    libc::c_uint).wrapping_add(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           libc::c_uint).wrapping_div(2
                                                                                                                          as
                                                                                                                          libc::c_int
                                                                                                                          as
                                                                                                                          libc::c_uint))
           &&
           my <
               ((350 as libc::c_int + 5 as libc::c_int) as
                    libc::c_uint).wrapping_add(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           libc::c_uint).wrapping_div(2
                                                                                                                          as
                                                                                                                          libc::c_int
                                                                                                                          as
                                                                                                                          libc::c_uint)).wrapping_add((115
                                                                                                                                                           as
                                                                                                                                                           libc::c_int
                                                                                                                                                           -
                                                                                                                                                           25
                                                                                                                                                               as
                                                                                                                                                               libc::c_int)
                                                                                                                                                          as
                                                                                                                                                          libc::c_uint)
           && (mouseDown(MOUSE_LMB) != 0 || mouseReleased(MOUSE_LMB) != 0) {
        if mouseDown(MOUSE_LMB) != 0 {
            x =
                mx.wrapping_sub(((40 as libc::c_int + 5 as libc::c_int) as
                                     libc::c_uint).wrapping_add(pie_GetVideoBufferWidth().wrapping_sub(640
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_uint).wrapping_div(2
                                                                                                                                          as
                                                                                                                                          libc::c_int
                                                                                                                                          as
                                                                                                                                          libc::c_uint))); //add a point
            y =
                my.wrapping_sub(((350 as libc::c_int + 5 as libc::c_int) as
                                     libc::c_uint).wrapping_add(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                                            as
                                                                                                            libc::c_int
                                                                                                            as
                                                                                                            libc::c_uint).wrapping_div(2
                                                                                                                                           as
                                                                                                                                           libc::c_int
                                                                                                                                           as
                                                                                                                                           libc::c_uint)));
            whiteBoard[selectedPlayer as usize][curWhite as usize] =
                y.wrapping_sub(1 as libc::c_int as
                                   libc::c_uint).wrapping_mul((373 as
                                                                   libc::c_int
                                                                   +
                                                                   250 as
                                                                       libc::c_int
                                                                   -
                                                                   40 as
                                                                       libc::c_int
                                                                   -
                                                                   10 as
                                                                       libc::c_int)
                                                                  as
                                                                  libc::c_uint).wrapping_add(x)
                    as UWORD
        }
        if mouseReleased(MOUSE_LMB) != 0 {
            whiteBoard[selectedPlayer as usize][curWhite as usize] =
                0 as libc::c_int as UWORD
            //set wrap point
            //add a non-point
        }
        curWhite = curWhite.wrapping_add(1);
        if curWhite as libc::c_int == 150 as libc::c_int {
            curWhite = 0 as libc::c_int as UBYTE
        }
        whiteBoard[selectedPlayer as usize][curWhite as usize] =
            1 as libc::c_int as UWORD
    }
    // possibly send our bit.
    if ingame.localOptionsReceived != 0 || bHosted != 0 {
        if lastSent > gameTime { lastSent = 0 as libc::c_int as UDWORD }
    }
    if gameTime.wrapping_sub(lastSent) > 2000 as libc::c_int as libc::c_uint {
        lastSent = gameTime;
        m.type_0 = NET_WHITEBOARD as libc::c_int as libc::c_uchar;
        m.body[0 as libc::c_int as usize] =
            selectedPlayer as UBYTE as libc::c_char;
        memcpy(&mut *m.body.as_mut_ptr().offset(1 as libc::c_int as isize) as
                   *mut libc::c_char as *mut libc::c_void,
               &mut *whiteBoard.as_mut_ptr().offset(selectedPlayer as isize)
                   as *mut [UWORD; 150] as *const libc::c_void,
               (150 as libc::c_int * 2 as libc::c_int) as libc::c_uint);
        m.size =
            (2 as libc::c_int * 150 as libc::c_int + 1 as libc::c_int) as
                libc::c_ushort;
        NETbcast(&mut m, 0 as libc::c_int);
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn displayWhiteBoard(mut psWidget: *mut _widget,
                                           mut xOffset: UDWORD,
                                           mut yOffset: UDWORD,
                                           mut pColours: *mut UDWORD) {
    let mut x: UDWORD =
        pie_GetVideoBufferWidth().wrapping_sub(640 as libc::c_int as
                                                   libc::c_uint).wrapping_div(2
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  libc::c_uint).wrapping_add(xOffset).wrapping_add((*psWidget).x
                                                                                                                                       as
                                                                                                                                       libc::c_uint);
    let mut y: UDWORD =
        pie_GetVideoBufferHeight().wrapping_sub(480 as libc::c_int as
                                                    libc::c_uint).wrapping_div(2
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   libc::c_uint).wrapping_add(yOffset).wrapping_add((*psWidget).y
                                                                                                                                        as
                                                                                                                                        libc::c_uint);
    //	UDWORD	w = psWidget->width;
//	UDWORD	h = psWidget->height;
    let mut i: UDWORD = 0;
    let mut d: div_t = div_t{quot: 0, rem: 0,};
    let mut j: UBYTE = 0;
    let mut col: UBYTE = 0;
    let mut oldPoint: UWORD = 0;
    let mut newPoint: UWORD = 0;
    let mut oldx: UWORD = 0;
    let mut oldy: UWORD = 0;
    let mut newx: UWORD = 0;
    let mut newy: UWORD = 0;
    // white poly
//	pie_BoxFillIndex(x,y,x+w,y+h,COL_WHITE);
    // each line.
    i = 0 as libc::c_int as UDWORD;
    while i < 8 as libc::c_int as libc::c_uint {
        if isHumanPlayer(i) != 0 {
            match getPlayerColour(i) as libc::c_int {
                0 => {
                    col =
                        *colours.as_mut_ptr().offset(2 as libc::c_int as
                                                         isize)
                }
                1 => {
                    col =
                        *colours.as_mut_ptr().offset(14 as libc::c_int as
                                                         isize)
                }
                2 => {
                    col =
                        *colours.as_mut_ptr().offset(7 as libc::c_int as
                                                         isize)
                }
                3 => {
                    col =
                        *colours.as_mut_ptr().offset(0 as libc::c_int as
                                                         isize)
                }
                4 => {
                    col =
                        *colours.as_mut_ptr().offset(12 as libc::c_int as
                                                         isize)
                }
                5 => {
                    col =
                        *colours.as_mut_ptr().offset(9 as libc::c_int as
                                                         isize)
                }
                6 => {
                    col =
                        *colours.as_mut_ptr().offset(13 as libc::c_int as
                                                         isize)
                }
                7 | _ => {
                    col =
                        *colours.as_mut_ptr().offset(11 as libc::c_int as
                                                         isize)
                }
            }
            oldPoint = whiteBoard[i as usize][0 as libc::c_int as usize];
            d =
                div(oldPoint as libc::c_int,
                    373 as libc::c_int + 250 as libc::c_int -
                        40 as libc::c_int - 10 as libc::c_int);
            oldy = d.quot as UWORD;
            oldx = d.rem as UWORD;
            j = 1 as libc::c_int as UBYTE;
            while (j as libc::c_int) < 150 as libc::c_int {
                newPoint = whiteBoard[i as usize][j as usize];
                d =
                    div(newPoint as libc::c_int,
                        373 as libc::c_int + 250 as libc::c_int -
                            40 as libc::c_int - 10 as libc::c_int);
                newy = d.quot as UWORD;
                newx = d.rem as UWORD;
                if newPoint as libc::c_int > 1 as libc::c_int &&
                       oldPoint as libc::c_int > 1 as libc::c_int {
                    pie_Line(x.wrapping_add(oldx as libc::c_uint) as
                                 libc::c_int,
                             y.wrapping_add(oldy as libc::c_uint) as
                                 libc::c_int,
                             x.wrapping_add(newx as libc::c_uint) as
                                 libc::c_int,
                             y.wrapping_add(newy as libc::c_uint) as
                                 libc::c_int, col as uint32);
                    // draw line!
                }
                oldPoint = newPoint;
                oldx = newx;
                oldy = newy;
                j = j.wrapping_add(1)
            }
        }
        i = i.wrapping_add(1)
    }
    // overlay close widget.
    pie_ImageFileID(FrontImages, IMAGE_NOPENCIL as libc::c_int as UWORD,
                    ((40 as libc::c_int - 15 as libc::c_int) as
                         libc::c_uint).wrapping_add(pie_GetVideoBufferWidth().wrapping_sub(640
                                                                                               as
                                                                                               libc::c_int
                                                                                               as
                                                                                               libc::c_uint).wrapping_div(2
                                                                                                                              as
                                                                                                                              libc::c_int
                                                                                                                              as
                                                                                                                              libc::c_uint))
                        as libc::c_int,
                    (350 as libc::c_int as
                         libc::c_uint).wrapping_add(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                libc::c_uint).wrapping_div(2
                                                                                                                               as
                                                                                                                               libc::c_int
                                                                                                                               as
                                                                                                                               libc::c_uint)).wrapping_add(115
                                                                                                                                                               as
                                                                                                                                                               libc::c_int
                                                                                                                                                               as
                                                                                                                                                               libc::c_uint).wrapping_sub(15
                                                                                                                                                                                              as
                                                                                                                                                                                              libc::c_int
                                                                                                                                                                                              as
                                                                                                                                                                                              libc::c_uint)
                        as libc::c_int);
}
// add whiteboard
unsafe extern "C" fn addWhiteBoard() -> BOOL {
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
    // clear whiteboard
    memset(&mut whiteBoard as *mut [[UWORD; 150]; 8] as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<[[UWORD; 150]; 8]>() as
               libc::c_ulong); //Connection Settings
    widgCreateScreen(&mut psWhiteScreen);
    widgSetTipFont(psWhiteScreen, WFont);
    memset(&mut sFormInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_FORMINIT>() as libc::c_ulong);
    sFormInit.formID = 0 as libc::c_int as UDWORD;
    sFormInit.id = 999 as libc::c_int as UDWORD;
    sFormInit.style = 0 as libc::c_int as UDWORD;
    sFormInit.x =
        ((40 as libc::c_int + 5 as libc::c_int) as
             libc::c_uint).wrapping_add(pie_GetVideoBufferWidth().wrapping_sub(640
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   libc::c_uint).wrapping_div(2
                                                                                                                  as
                                                                                                                  libc::c_int
                                                                                                                  as
                                                                                                                  libc::c_uint)).wrapping_sub(pie_GetVideoBufferWidth().wrapping_sub(640
                                                                                                                                                                                         as
                                                                                                                                                                                         libc::c_int
                                                                                                                                                                                         as
                                                                                                                                                                                         libc::c_uint).wrapping_div(2
                                                                                                                                                                                                                        as
                                                                                                                                                                                                                        libc::c_int
                                                                                                                                                                                                                        as
                                                                                                                                                                                                                        libc::c_uint))
            as SWORD;
    sFormInit.y =
        ((350 as libc::c_int + 5 as libc::c_int) as
             libc::c_uint).wrapping_add(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint).wrapping_div(2
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   libc::c_uint)).wrapping_sub(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                                                                                                                           as
                                                                                                                                                                                           libc::c_int
                                                                                                                                                                                           as
                                                                                                                                                                                           libc::c_uint).wrapping_div(2
                                                                                                                                                                                                                          as
                                                                                                                                                                                                                          libc::c_int
                                                                                                                                                                                                                          as
                                                                                                                                                                                                                          libc::c_uint))
            as SWORD;
    sFormInit.width =
        (373 as libc::c_int + 250 as libc::c_int - 40 as libc::c_int -
             10 as libc::c_int) as UWORD;
    sFormInit.height = (115 as libc::c_int - 25 as libc::c_int) as UWORD;
    sFormInit.pDisplay =
        Some(displayWhiteBoard as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    widgAddForm(psWhiteScreen, &mut sFormInit);
    bWhiteBoardUp = 1 as libc::c_int;
    return 1 as libc::c_int;
}
// Drawing Functions
// ///////////////////////////////////////////////////////////////////////////////////////
// ///////////////////////////////////////////////////////////////////////////////////////
// Drawing functions
#[no_mangle]
pub unsafe extern "C" fn displayChatEdit(mut psWidget: *mut _widget,
                                         mut xOffset: UDWORD,
                                         mut yOffset: UDWORD,
                                         mut pColours: *mut UDWORD) {
    let mut x: UDWORD =
        xOffset.wrapping_add((*psWidget).x as
                                 libc::c_uint); // 4 is the magic number.
    let mut y: UDWORD =
        yOffset.wrapping_add((*psWidget).y as
                                 libc::c_uint).wrapping_sub(4 as libc::c_int
                                                                as
                                                                libc::c_uint);
    pie_Line(x as libc::c_int, y as libc::c_int,
             x.wrapping_add((*psWidget).width as libc::c_uint) as libc::c_int,
             y as libc::c_int,
             pal_GetNearestColour(100 as libc::c_int as uint8,
                                  100 as libc::c_int as uint8,
                                  160 as libc::c_int as uint8) as uint32);
    AddCursorSnap(&mut InterfaceSnap,
                  x.wrapping_add(((*psWidget).width as libc::c_int /
                                      2 as libc::c_int) as libc::c_uint) as
                      SWORD,
                  y.wrapping_add(((*psWidget).height as libc::c_int /
                                      2 as libc::c_int) as libc::c_uint) as
                      SWORD, (*psWidget).formID, (*psWidget).id,
                  0 as *mut SNAPBIAS);
}
// ////////////////////////////////////////////////////////////////////////////
#[no_mangle]
pub unsafe extern "C" fn displayRemoteGame(mut psWidget: *mut _widget,
                                           mut xOffset: UDWORD,
                                           mut yOffset: UDWORD,
                                           mut pColours: *mut UDWORD) {
    let mut x: UDWORD = xOffset.wrapping_add((*psWidget).x as libc::c_uint);
    let mut y: UDWORD = yOffset.wrapping_add((*psWidget).y as libc::c_uint);
    let mut Hilight: BOOL = 0 as libc::c_int;
    let mut Down: BOOL = 0 as libc::c_int;
    let mut i: UDWORD = 0;
    let mut tmp: [STRING; 8] = [0; 8];
    let mut png: UDWORD = 0;
    i = (*psWidget).pUserData as libc::c_int as UDWORD;
    // collate info
    if (*(psWidget as *mut W_BUTTON)).state &
           0x4 as libc::c_int as libc::c_uint != 0 {
        Hilight = 1 as libc::c_int
    }
    if (*(psWidget as *mut W_BUTTON)).state &
           (0x2 as libc::c_int | 0x4 as libc::c_int) as libc::c_uint != 0 {
        //LOCK WCLICK_DOWN | WCLICK_LOCKED | WCLICK_CLICKLOCK))
        Down = 1 as libc::c_int
    }
    // Draw blue boxes.
    drawBlueBox(x, y, (*psWidget).width as UDWORD,
                (*psWidget).height as UDWORD);
    drawBlueBox(x, y, 94 as libc::c_int as UDWORD,
                (*psWidget).height as UDWORD);
    drawBlueBox(x, y, 55 as libc::c_int as UDWORD,
                (*psWidget).height as UDWORD);
    //draw game info
    iV_SetFont(WFont); // font
    iV_SetTextColour(-(1 as libc::c_int) as SWORD); //colour
    //draw type overlay.
//	if(NETgetGameFlagsUnjoined(i,1) == DMATCH)
//	{
//		iV_DrawTransImage(FrontImages,IMAGE_ARENA_OVER,x+59,y+3);
//	}
//	else
    if NETgetGameFlagsUnjoined(i, 1 as libc::c_int as UDWORD) ==
           12 as libc::c_int {
        pie_ImageFileID(FrontImages,
                        IMAGE_CAMPAIGN_OVER as libc::c_int as UWORD,
                        x.wrapping_add(59 as libc::c_int as libc::c_uint) as
                            libc::c_int,
                        y.wrapping_add(3 as libc::c_int as libc::c_uint) as
                            libc::c_int);
    } else if NETgetGameFlagsUnjoined(i, 1 as libc::c_int as UDWORD) ==
                  13 as libc::c_int {
        pie_ImageFileID(FrontImages, IMAGE_TEAM_OVER as libc::c_int as UWORD,
                        x.wrapping_add(62 as libc::c_int as libc::c_uint) as
                            libc::c_int,
                        y.wrapping_add(3 as libc::c_int as libc::c_uint) as
                            libc::c_int);
    } else {
        pie_ImageFileID(FrontImages,
                        IMAGE_SKIRMISH_OVER as libc::c_int as UWORD,
                        x.wrapping_add(62 as libc::c_int as libc::c_uint) as
                            libc::c_int,
                        y.wrapping_add(3 as libc::c_int as libc::c_uint) as
                            libc::c_int);
        // SKIRMISH
    }
    // ping rating
    png = NETgetGameFlagsUnjoined(i, 2 as libc::c_int as UDWORD) as UDWORD;
    if png >= 0 as libc::c_int as libc::c_uint &&
           png < 600 as libc::c_int as libc::c_uint {
        pie_ImageFileID(FrontImages, IMAGE_LAMP_GREEN as libc::c_int as UWORD,
                        x.wrapping_add(70 as libc::c_int as libc::c_uint) as
                            libc::c_int,
                        y.wrapping_add(26 as libc::c_int as libc::c_uint) as
                            libc::c_int);
    } else if png >= 600 as libc::c_int as libc::c_uint &&
                  png < 1200 as libc::c_int as libc::c_uint {
        pie_ImageFileID(FrontImages, IMAGE_LAMP_AMBER as libc::c_int as UWORD,
                        x.wrapping_add(70 as libc::c_int as libc::c_uint) as
                            libc::c_int,
                        y.wrapping_add(26 as libc::c_int as libc::c_uint) as
                            libc::c_int);
    } else {
        pie_ImageFileID(FrontImages, IMAGE_LAMP_RED as libc::c_int as UWORD,
                        x.wrapping_add(70 as libc::c_int as libc::c_uint) as
                            libc::c_int,
                        y.wrapping_add(26 as libc::c_int as libc::c_uint) as
                            libc::c_int);
    }
    //draw game name
    while iV_GetTextWidth(NetPlay.games[i as usize].name.as_mut_ptr()) >
              (*psWidget).width as libc::c_int - 110 as libc::c_int {
        NetPlay.games[i as
                          usize].name[strlen(NetPlay.games[i as
                                                               usize].name.as_mut_ptr()).wrapping_sub(1
                                                                                                          as
                                                                                                          libc::c_int
                                                                                                          as
                                                                                                          libc::c_uint)
                                          as usize] =
            '\u{0}' as i32 as libc::c_char
    } // name
    pie_DrawText(NetPlay.games[i as usize].name.as_mut_ptr(),
                 x.wrapping_add(100 as libc::c_int as libc::c_uint),
                 y.wrapping_add(24 as libc::c_int as libc::c_uint));
    // get game info.
    if NetPlay.games[i as usize].desc.dwFlags & 1 as libc::c_int != 0 ||
           NetPlay.games[i as usize].desc.dwCurrentPlayers >=
               NetPlay.games[i as usize].desc.dwMaxPlayers ||
           NETgetGameFlagsUnjoined(gameNumber, 1 as libc::c_int as UDWORD) ==
               14 as libc::c_int &&
               NetPlay.games[gameNumber as usize].desc.dwCurrentPlayers >=
                   NetPlay.games[gameNumber as usize].desc.dwMaxPlayers -
                       1 as libc::c_int {
        // need some sort of closed thing here!
        pie_ImageFileID(FrontImages, IMAGE_NOJOIN as libc::c_int as UWORD,
                        x.wrapping_add(18 as libc::c_int as libc::c_uint) as
                            libc::c_int,
                        y.wrapping_add(11 as libc::c_int as libc::c_uint) as
                            libc::c_int);
    } else {
        pie_DrawText(strresGetString(psStringRes,
                                     STR_MUL_PLAYERS as libc::c_int as
                                         UDWORD),
                     x.wrapping_add(5 as libc::c_int as libc::c_uint),
                     y.wrapping_add(18 as libc::c_int as libc::c_uint));
        sprintf(tmp.as_mut_ptr(),
                b"%d/%d\x00" as *const u8 as *const libc::c_char,
                NetPlay.games[i as usize].desc.dwCurrentPlayers,
                NetPlay.games[i as usize].desc.dwMaxPlayers);
        pie_DrawText(tmp.as_mut_ptr(),
                     x.wrapping_add(17 as libc::c_int as libc::c_uint),
                     y.wrapping_add(33 as libc::c_int as libc::c_uint));
    }
    AddCursorSnap(&mut InterfaceSnap,
                  x.wrapping_add(((*psWidget).width as libc::c_int /
                                      2 as libc::c_int) as libc::c_uint) as
                      SWORD,
                  y.wrapping_add(((*psWidget).height as libc::c_int /
                                      2 as libc::c_int) as libc::c_uint) as
                      SWORD, (*psWidget).formID, (*psWidget).id,
                  0 as *mut SNAPBIAS);
}
// ////////////////////////////////////////////////////////////////////////////
unsafe extern "C" fn bestPlayer(mut player: UDWORD) -> UDWORD {
    let mut i: UDWORD = 0;
    let mut myscore: UDWORD = 0;
    let mut score: UDWORD = 0;
    let mut count: UDWORD = 1 as libc::c_int as UDWORD;
    myscore = getMultiStats(player, 0 as libc::c_int).totalScore as UDWORD;
    i = 0 as libc::c_int as UDWORD;
    while i < 8 as libc::c_int as libc::c_uint {
        if isHumanPlayer(i) != 0 && i != player {
            score = getMultiStats(i, 0 as libc::c_int).totalScore as UDWORD;
            if score >= myscore { count = count.wrapping_add(1) }
        }
        i = i.wrapping_add(1)
    }
    return count;
}
// ////////////////////////////////////////////////////////////////////////////
#[no_mangle]
pub unsafe extern "C" fn displayPlayer(mut psWidget: *mut _widget,
                                       mut xOffset: UDWORD,
                                       mut yOffset: UDWORD,
                                       mut pColours: *mut UDWORD) {
    let mut x: UDWORD = xOffset.wrapping_add((*psWidget).x as libc::c_uint);
    let mut y: UDWORD = yOffset.wrapping_add((*psWidget).y as libc::c_uint);
    let mut Hilight: BOOL = 0 as libc::c_int;
    let mut j: UDWORD = 0;
    let mut i: UDWORD = 0;
    let mut eval: UDWORD = 0;
    let mut stat: PLAYERSTATS =
        PLAYERSTATS{played: 0,
                    wins: 0,
                    loses: 0,
                    totalKills: 0,
                    totalScore: 0,
                    recentKills: 0,
                    recentScore: 0,
                    killsToAdd: 0,
                    scoreToAdd: 0,};
    if (*(psWidget as *mut W_BUTTON)).state &
           (0x4 as libc::c_int | 0x1 as libc::c_int | 0x8 as libc::c_int |
                0x10 as libc::c_int) as libc::c_uint != 0 {
        Hilight = 1 as libc::c_int
    }
    i = (*psWidget).pUserData as libc::c_int as UDWORD;
    //bluboxes.
    drawBlueBox(x, y, (*psWidget).width as UDWORD,
                (*psWidget).height as UDWORD); // right
    if ingame.localOptionsReceived != 0 &&
           NetPlay.players[i as usize].dpid != 0 {
        // only draw if real player!
        //bluboxes.
        drawBlueBox(x, y, (*psWidget).width as UDWORD,
                    (*psWidget).height as UDWORD); // right
        drawBlueBox(x, y, 60 as libc::c_int as UDWORD,
                    (*psWidget).height as UDWORD); // left.
        drawBlueBox(x, y, 31 as libc::c_int as UDWORD,
                    (*psWidget).height as
                        UDWORD); // get the in game playernumber.
        j = 0 as libc::c_int as UDWORD; // font
        while player2dpid[j as usize] != NetPlay.players[i as usize].dpid &&
                  j < 8 as libc::c_int as libc::c_uint {
            j = j.wrapping_add(1)
        } // colour
        iV_SetFont(WFont);
        iV_SetTextColour(-(1 as libc::c_int) as SWORD);
        while iV_GetTextWidth(NetPlay.players[i as usize].name.as_mut_ptr()) >
                  (*psWidget).width as libc::c_int - 68 as libc::c_int
              // name
              {
            // clip name.
            NetPlay.players[i as
                                usize].name[strlen(NetPlay.players[i as
                                                                       usize].name.as_mut_ptr()).wrapping_sub(1
                                                                                                                  as
                                                                                                                  libc::c_int
                                                                                                                  as
                                                                                                                  libc::c_uint)
                                                as usize] =
                '\u{0}' as i32 as libc::c_char
        }
        pie_DrawText(NetPlay.players[i as usize].name.as_mut_ptr(),
                     x.wrapping_add(65 as libc::c_int as libc::c_uint),
                     y.wrapping_add(22 as libc::c_int as libc::c_uint));
        if ingame.PingTimes[j as usize] >= 0 as libc::c_int as libc::c_uint &&
               ingame.PingTimes[j as usize] <
                   600 as libc::c_int as libc::c_uint {
            pie_ImageFileID(FrontImages,
                            IMAGE_LAMP_GREEN as libc::c_int as UWORD,
                            x as libc::c_int, y as libc::c_int);
        } else if ingame.PingTimes[j as usize] >=
                      600 as libc::c_int as libc::c_uint &&
                      ingame.PingTimes[j as usize] <
                          1200 as libc::c_int as libc::c_uint {
            pie_ImageFileID(FrontImages,
                            IMAGE_LAMP_AMBER as libc::c_int as UWORD,
                            x as libc::c_int, y as libc::c_int);
        } else {
            pie_ImageFileID(FrontImages,
                            IMAGE_LAMP_RED as libc::c_int as UWORD,
                            x as libc::c_int, y as libc::c_int);
        }
        match j {
            0 => {
                pie_ImageFileID(IntImages, IMAGE_GN_0 as libc::c_int as UWORD,
                                x.wrapping_add(4 as libc::c_int as
                                                   libc::c_uint) as
                                    libc::c_int,
                                y.wrapping_add(29 as libc::c_int as
                                                   libc::c_uint) as
                                    libc::c_int);
            }
            1 => {
                pie_ImageFileID(IntImages, IMAGE_GN_1 as libc::c_int as UWORD,
                                x.wrapping_add(5 as libc::c_int as
                                                   libc::c_uint) as
                                    libc::c_int,
                                y.wrapping_add(29 as libc::c_int as
                                                   libc::c_uint) as
                                    libc::c_int);
            }
            2 => {
                pie_ImageFileID(IntImages, IMAGE_GN_2 as libc::c_int as UWORD,
                                x.wrapping_add(4 as libc::c_int as
                                                   libc::c_uint) as
                                    libc::c_int,
                                y.wrapping_add(29 as libc::c_int as
                                                   libc::c_uint) as
                                    libc::c_int);
            }
            3 => {
                pie_ImageFileID(IntImages, IMAGE_GN_3 as libc::c_int as UWORD,
                                x.wrapping_add(4 as libc::c_int as
                                                   libc::c_uint) as
                                    libc::c_int,
                                y.wrapping_add(29 as libc::c_int as
                                                   libc::c_uint) as
                                    libc::c_int);
            }
            4 => {
                pie_ImageFileID(IntImages, IMAGE_GN_4 as libc::c_int as UWORD,
                                x.wrapping_add(4 as libc::c_int as
                                                   libc::c_uint) as
                                    libc::c_int,
                                y.wrapping_add(29 as libc::c_int as
                                                   libc::c_uint) as
                                    libc::c_int);
            }
            5 => {
                pie_ImageFileID(IntImages, IMAGE_GN_5 as libc::c_int as UWORD,
                                x.wrapping_add(4 as libc::c_int as
                                                   libc::c_uint) as
                                    libc::c_int,
                                y.wrapping_add(29 as libc::c_int as
                                                   libc::c_uint) as
                                    libc::c_int);
            }
            6 => {
                pie_ImageFileID(IntImages, IMAGE_GN_6 as libc::c_int as UWORD,
                                x.wrapping_add(4 as libc::c_int as
                                                   libc::c_uint) as
                                    libc::c_int,
                                y.wrapping_add(29 as libc::c_int as
                                                   libc::c_uint) as
                                    libc::c_int);
            }
            7 => {
                pie_ImageFileID(IntImages, IMAGE_GN_7 as libc::c_int as UWORD,
                                x.wrapping_add(4 as libc::c_int as
                                                   libc::c_uint) as
                                    libc::c_int,
                                y.wrapping_add(29 as libc::c_int as
                                                   libc::c_uint) as
                                    libc::c_int);
            }
            _ => { }
        }
        eval = bestPlayer(j);
        match eval {
            1 => {
                pie_ImageFileID(IntImages, IMAGE_GN_1 as libc::c_int as UWORD,
                                x.wrapping_add(5 as libc::c_int as
                                                   libc::c_uint) as
                                    libc::c_int,
                                y.wrapping_add(3 as libc::c_int as
                                                   libc::c_uint) as
                                    libc::c_int);
            }
            2 => {
                pie_ImageFileID(IntImages, IMAGE_GN_2 as libc::c_int as UWORD,
                                x.wrapping_add(4 as libc::c_int as
                                                   libc::c_uint) as
                                    libc::c_int,
                                y.wrapping_add(3 as libc::c_int as
                                                   libc::c_uint) as
                                    libc::c_int);
            }
            3 => {
                pie_ImageFileID(IntImages, IMAGE_GN_3 as libc::c_int as UWORD,
                                x.wrapping_add(4 as libc::c_int as
                                                   libc::c_uint) as
                                    libc::c_int,
                                y.wrapping_add(3 as libc::c_int as
                                                   libc::c_uint) as
                                    libc::c_int);
            }
            4 => {
                pie_ImageFileID(IntImages, IMAGE_GN_4 as libc::c_int as UWORD,
                                x.wrapping_add(4 as libc::c_int as
                                                   libc::c_uint) as
                                    libc::c_int,
                                y.wrapping_add(3 as libc::c_int as
                                                   libc::c_uint) as
                                    libc::c_int);
            }
            5 => {
                pie_ImageFileID(IntImages, IMAGE_GN_5 as libc::c_int as UWORD,
                                x.wrapping_add(4 as libc::c_int as
                                                   libc::c_uint) as
                                    libc::c_int,
                                y.wrapping_add(3 as libc::c_int as
                                                   libc::c_uint) as
                                    libc::c_int);
            }
            6 => {
                pie_ImageFileID(IntImages, IMAGE_GN_6 as libc::c_int as UWORD,
                                x.wrapping_add(4 as libc::c_int as
                                                   libc::c_uint) as
                                    libc::c_int,
                                y.wrapping_add(3 as libc::c_int as
                                                   libc::c_uint) as
                                    libc::c_int);
            }
            7 => {
                pie_ImageFileID(IntImages, IMAGE_GN_7 as libc::c_int as UWORD,
                                x.wrapping_add(4 as libc::c_int as
                                                   libc::c_uint) as
                                    libc::c_int,
                                y.wrapping_add(3 as libc::c_int as
                                                   libc::c_uint) as
                                    libc::c_int);
            }
            _ => { }
        }
        if getMultiStats(j, 0 as libc::c_int).played < 5 as libc::c_int {
            pie_ImageFileID(FrontImages,
                            IMAGE_MEDAL_DUMMY as libc::c_int as UWORD,
                            x.wrapping_add(37 as libc::c_int as libc::c_uint)
                                as libc::c_int,
                            y.wrapping_add(13 as libc::c_int as libc::c_uint)
                                as libc::c_int);
        } else {
            stat = getMultiStats(j, 0 as libc::c_int);
            // ping rating
            // player number
//		iV_DrawTransImage(FrontImages,IMAGE_WEE_GUY,x,y+23);
            // ranking against other players.
            // star 1 total droid kills
            eval = stat.totalKills as UDWORD;
            if eval > 600 as libc::c_int as libc::c_uint {
                pie_ImageFileID(FrontImages,
                                IMAGE_MULTIRANK1 as libc::c_int as UWORD,
                                x.wrapping_add(37 as libc::c_int as
                                                   libc::c_uint) as
                                    libc::c_int,
                                y.wrapping_add(3 as libc::c_int as
                                                   libc::c_uint) as
                                    libc::c_int);
            } else if eval > 300 as libc::c_int as libc::c_uint {
                pie_ImageFileID(FrontImages,
                                IMAGE_MULTIRANK2 as libc::c_int as UWORD,
                                x.wrapping_add(37 as libc::c_int as
                                                   libc::c_uint) as
                                    libc::c_int,
                                y.wrapping_add(3 as libc::c_int as
                                                   libc::c_uint) as
                                    libc::c_int);
            } else if eval > 150 as libc::c_int as libc::c_uint {
                pie_ImageFileID(FrontImages,
                                IMAGE_MULTIRANK3 as libc::c_int as UWORD,
                                x.wrapping_add(37 as libc::c_int as
                                                   libc::c_uint) as
                                    libc::c_int,
                                y.wrapping_add(3 as libc::c_int as
                                                   libc::c_uint) as
                                    libc::c_int);
            }
            // star 2 games played
            eval = stat.played as UDWORD;
            if eval > 200 as libc::c_int as libc::c_uint {
                pie_ImageFileID(FrontImages,
                                IMAGE_MULTIRANK1 as libc::c_int as UWORD,
                                x.wrapping_add(37 as libc::c_int as
                                                   libc::c_uint) as
                                    libc::c_int,
                                y.wrapping_add(13 as libc::c_int as
                                                   libc::c_uint) as
                                    libc::c_int);
            } else if eval > 100 as libc::c_int as libc::c_uint {
                pie_ImageFileID(FrontImages,
                                IMAGE_MULTIRANK2 as libc::c_int as UWORD,
                                x.wrapping_add(37 as libc::c_int as
                                                   libc::c_uint) as
                                    libc::c_int,
                                y.wrapping_add(13 as libc::c_int as
                                                   libc::c_uint) as
                                    libc::c_int);
            } else if eval > 50 as libc::c_int as libc::c_uint {
                pie_ImageFileID(FrontImages,
                                IMAGE_MULTIRANK3 as libc::c_int as UWORD,
                                x.wrapping_add(37 as libc::c_int as
                                                   libc::c_uint) as
                                    libc::c_int,
                                y.wrapping_add(13 as libc::c_int as
                                                   libc::c_uint) as
                                    libc::c_int);
            }
            // star 3 games won.
            eval = stat.wins as UDWORD;
            if eval > 80 as libc::c_int as libc::c_uint {
                pie_ImageFileID(FrontImages,
                                IMAGE_MULTIRANK1 as libc::c_int as UWORD,
                                x.wrapping_add(37 as libc::c_int as
                                                   libc::c_uint) as
                                    libc::c_int,
                                y.wrapping_add(23 as libc::c_int as
                                                   libc::c_uint) as
                                    libc::c_int);
            } else if eval > 40 as libc::c_int as libc::c_uint {
                pie_ImageFileID(FrontImages,
                                IMAGE_MULTIRANK2 as libc::c_int as UWORD,
                                x.wrapping_add(37 as libc::c_int as
                                                   libc::c_uint) as
                                    libc::c_int,
                                y.wrapping_add(23 as libc::c_int as
                                                   libc::c_uint) as
                                    libc::c_int);
            } else if eval > 10 as libc::c_int as libc::c_uint {
                pie_ImageFileID(FrontImages,
                                IMAGE_MULTIRANK3 as libc::c_int as UWORD,
                                x.wrapping_add(37 as libc::c_int as
                                                   libc::c_uint) as
                                    libc::c_int,
                                y.wrapping_add(23 as libc::c_int as
                                                   libc::c_uint) as
                                    libc::c_int);
            }
            // medals.
            if stat.loses > 2 as libc::c_int && stat.wins > 2 as libc::c_int
                   && stat.wins > 2 as libc::c_int * stat.loses {
                // bronze requirement.
                if stat.wins > 4 as libc::c_int * stat.loses {
                    // silver requirement.
                    if stat.wins > 8 as libc::c_int * stat.loses {
                        // gold requirement
                        pie_ImageFileID(FrontImages,
                                        IMAGE_MEDAL_GOLD as libc::c_int as
                                            UWORD,
                                        x.wrapping_add(49 as libc::c_int as
                                                           libc::c_uint) as
                                            libc::c_int,
                                        y.wrapping_add(11 as libc::c_int as
                                                           libc::c_uint) as
                                            libc::c_int);
                    } else {
                        pie_ImageFileID(FrontImages,
                                        IMAGE_MEDAL_SILVER as libc::c_int as
                                            UWORD,
                                        x.wrapping_add(49 as libc::c_int as
                                                           libc::c_uint) as
                                            libc::c_int,
                                        y.wrapping_add(11 as libc::c_int as
                                                           libc::c_uint) as
                                            libc::c_int);
                    }
                } else {
                    pie_ImageFileID(FrontImages,
                                    IMAGE_MEDAL_BRONZE as libc::c_int as
                                        UWORD,
                                    x.wrapping_add(49 as libc::c_int as
                                                       libc::c_uint) as
                                        libc::c_int,
                                    y.wrapping_add(11 as libc::c_int as
                                                       libc::c_uint) as
                                        libc::c_int);
                }
            }
        }
        match getPlayerColour(j) as libc::c_int {
            0 => {
                //flag icon
                pie_ImageFileID(FrontImages,
                                IMAGE_PLAYER0 as libc::c_int as UWORD,
                                x.wrapping_add(7 as libc::c_int as
                                                   libc::c_uint) as
                                    libc::c_int,
                                y.wrapping_add(9 as libc::c_int as
                                                   libc::c_uint) as
                                    libc::c_int);
            }
            1 => {
                pie_ImageFileID(FrontImages,
                                IMAGE_PLAYER1 as libc::c_int as UWORD,
                                x.wrapping_add(7 as libc::c_int as
                                                   libc::c_uint) as
                                    libc::c_int,
                                y.wrapping_add(9 as libc::c_int as
                                                   libc::c_uint) as
                                    libc::c_int);
            }
            2 => {
                pie_ImageFileID(FrontImages,
                                IMAGE_PLAYER2 as libc::c_int as UWORD,
                                x.wrapping_add(7 as libc::c_int as
                                                   libc::c_uint) as
                                    libc::c_int,
                                y.wrapping_add(9 as libc::c_int as
                                                   libc::c_uint) as
                                    libc::c_int);
            }
            3 => {
                pie_ImageFileID(FrontImages,
                                IMAGE_PLAYER3 as libc::c_int as UWORD,
                                x.wrapping_add(7 as libc::c_int as
                                                   libc::c_uint) as
                                    libc::c_int,
                                y.wrapping_add(9 as libc::c_int as
                                                   libc::c_uint) as
                                    libc::c_int);
            }
            4 => {
                pie_ImageFileID(FrontImages,
                                IMAGE_PLAYER4 as libc::c_int as UWORD,
                                x.wrapping_add(7 as libc::c_int as
                                                   libc::c_uint) as
                                    libc::c_int,
                                y.wrapping_add(9 as libc::c_int as
                                                   libc::c_uint) as
                                    libc::c_int);
            }
            5 => {
                pie_ImageFileID(FrontImages,
                                IMAGE_PLAYER5 as libc::c_int as UWORD,
                                x.wrapping_add(7 as libc::c_int as
                                                   libc::c_uint) as
                                    libc::c_int,
                                y.wrapping_add(9 as libc::c_int as
                                                   libc::c_uint) as
                                    libc::c_int);
            }
            6 => {
                pie_ImageFileID(FrontImages,
                                IMAGE_PLAYER6 as libc::c_int as UWORD,
                                x.wrapping_add(7 as libc::c_int as
                                                   libc::c_uint) as
                                    libc::c_int,
                                y.wrapping_add(9 as libc::c_int as
                                                   libc::c_uint) as
                                    libc::c_int);
            }
            7 => {
                pie_ImageFileID(FrontImages,
                                IMAGE_PLAYER7 as libc::c_int as UWORD,
                                x.wrapping_add(7 as libc::c_int as
                                                   libc::c_uint) as
                                    libc::c_int,
                                y.wrapping_add(9 as libc::c_int as
                                                   libc::c_uint) as
                                    libc::c_int);
            }
            _ => { }
        }
        game.skDiff[i as usize] = 0xff as libc::c_int as UBYTE
    } else {
        //unknown bugfix
//		game.skirmishPlayers[i] =TRUE;	// don't clear this one!
        //bluboxes.
        drawBlueBox(x, y, (*psWidget).width as UDWORD,
                    (*psWidget).height as UDWORD); // right
        drawBlueBox(x, y, 31 as libc::c_int as UDWORD,
                    (*psWidget).height as UDWORD); // left.
        //		if(game.type == SKIRMISH && game.skirmishPlayers[i])
        if game.type_0 as libc::c_int == 14 as libc::c_int &&
               game.skDiff[i as usize] as libc::c_int != 0 {
            pie_ImageFileID(FrontImages,
                            IMAGE_PLAYER_PC as libc::c_int as UWORD,
                            x.wrapping_add(2 as libc::c_int as libc::c_uint)
                                as libc::c_int,
                            y.wrapping_add(9 as libc::c_int as libc::c_uint)
                                as libc::c_int);
        }
    }
    AddCursorSnap(&mut InterfaceSnap,
                  x.wrapping_add(((*psWidget).width as libc::c_int /
                                      2 as libc::c_int) as libc::c_uint) as
                      SWORD,
                  y.wrapping_add(((*psWidget).height as libc::c_int /
                                      2 as libc::c_int) as libc::c_uint) as
                      SWORD, (*psWidget).formID, (*psWidget).id,
                  0 as *mut SNAPBIAS);
}
// ////////////////////////////////////////////////////////////////////////////
// Display blue box
#[no_mangle]
pub unsafe extern "C" fn intDisplayFeBox(mut psWidget: *mut _widget,
                                         mut xOffset: UDWORD,
                                         mut yOffset: UDWORD,
                                         mut pColours: *mut UDWORD) {
    let mut x: UDWORD = xOffset.wrapping_add((*psWidget).x as libc::c_uint);
    let mut y: UDWORD = yOffset.wrapping_add((*psWidget).y as libc::c_uint);
    let mut w: UDWORD = (*psWidget).width as UDWORD;
    let mut h: UDWORD = (*psWidget).height as UDWORD;
    drawBlueBox(x, y, w, h);
}
// ////////////////////////////////////////////////////////////////////////////
// Display edit box
#[no_mangle]
pub unsafe extern "C" fn displayMultiEditBox(mut psWidget: *mut _widget,
                                             mut xOffset: UDWORD,
                                             mut yOffset: UDWORD,
                                             mut pColours: *mut UDWORD) {
    let mut x: UDWORD =
        xOffset.wrapping_add((*psWidget).x as libc::c_uint); // box on end.
    let mut y: UDWORD =
        yOffset.wrapping_add((*psWidget).y as
                                 libc::c_uint); //icon descriptor.
    let mut im: UWORD = (*psWidget).pUserData as UDWORD as UWORD;
    drawBlueBox(x, y, (*psWidget).width as UDWORD,
                (*psWidget).height as UDWORD);
    drawBlueBox(x.wrapping_add((*psWidget).width as libc::c_uint), y,
                (*psWidget).height as UDWORD, (*psWidget).height as UDWORD);
    pie_ImageFileID(FrontImages, im,
                    x.wrapping_add((*psWidget).width as
                                       libc::c_uint).wrapping_add(2 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint)
                        as libc::c_int,
                    y.wrapping_add(4 as libc::c_int as libc::c_uint) as
                        libc::c_int);
    if (*(psWidget as *mut W_EDITBOX)).state &
           0x20 as libc::c_int as libc::c_uint != 0 {
        // disabled
        pie_SetSwirlyBoxes(0 as libc::c_int);
        pie_UniTransBoxFill(x as SDWORD, y as SDWORD,
                            x.wrapping_add((*psWidget).width as
                                               libc::c_uint).wrapping_add((*psWidget).height
                                                                              as
                                                                              libc::c_uint)
                                as SDWORD,
                            y.wrapping_add((*psWidget).height as libc::c_uint)
                                as SDWORD,
                            ((16 as libc::c_int) << 16 as libc::c_int |
                                 (16 as libc::c_int) << 8 as libc::c_int |
                                 128 as libc::c_int) as UDWORD,
                            128 as libc::c_int as UDWORD);
        pie_SetSwirlyBoxes(1 as libc::c_int);
    }
    AddCursorSnap(&mut InterfaceSnap,
                  x.wrapping_add(((*psWidget).width as libc::c_int /
                                      2 as libc::c_int) as libc::c_uint) as
                      SWORD,
                  y.wrapping_add(((*psWidget).height as libc::c_int /
                                      2 as libc::c_int) as libc::c_uint) as
                      SWORD, (*psWidget).formID, (*psWidget).id,
                  0 as *mut SNAPBIAS);
}
// ////////////////////////////////////////////////////////////////////////////
// Display one of two images depending on if the widget is hilighted by the mouse.
#[no_mangle]
pub unsafe extern "C" fn displayMultiBut(mut psWidget: *mut _widget,
                                         mut xOffset: UDWORD,
                                         mut yOffset: UDWORD,
                                         mut pColours: *mut UDWORD) {
    let mut x: UDWORD = xOffset.wrapping_add((*psWidget).x as libc::c_uint);
    let mut y: UDWORD = yOffset.wrapping_add((*psWidget).y as libc::c_uint);
    let mut Hilight: BOOL = 0 as libc::c_int;
    let mut Down: UDWORD = 0 as libc::c_int as UDWORD;
    let mut hiToUse: UWORD = 0 as libc::c_int as UWORD;
    let mut Grey: UDWORD = 0 as libc::c_int as UDWORD;
    let mut im: UWORD =
        ((*psWidget).pUserData as UDWORD >> 10 as libc::c_int &
             0x3ff as libc::c_int as libc::c_uint) as UWORD;
    let mut im2: UWORD =
        ((*psWidget).pUserData as UDWORD &
             0x3ff as libc::c_int as libc::c_uint) as UWORD;
    let mut usehl: BOOL =
        ((*psWidget).pUserData as UDWORD >> 20 as libc::c_int &
             0x3ff as libc::c_int as libc::c_uint) as UWORD as BOOL;
    //	BOOL	snap = 1;
    //evaluate
    if usehl == 1 as libc::c_int &&
           (*(psWidget as *mut W_BUTTON)).state &
               0x4 as libc::c_int as libc::c_uint != 0 {
        Hilight = 1 as libc::c_int;
        match iV_GetImageWidth(FrontImages, im) as libc::c_int {
            30 => {
                //pick a hilight.
                hiToUse = IMAGE_HI34 as libc::c_int as UWORD
            }
            60 => { hiToUse = IMAGE_HI64 as libc::c_int as UWORD }
            19 => { hiToUse = IMAGE_HI23 as libc::c_int as UWORD }
            27 => { hiToUse = IMAGE_HI31 as libc::c_int as UWORD }
            35 => { hiToUse = IMAGE_HI39 as libc::c_int as UWORD }
            37 => { hiToUse = IMAGE_HI41 as libc::c_int as UWORD }
            56 => { hiToUse = IMAGE_HI56 as libc::c_int as UWORD }
            _ => { hiToUse = 0 as libc::c_int as UWORD }
        }
        if im as libc::c_int == IMAGE_RETURN as libc::c_int {
            hiToUse = IMAGE_RETURN_HI as libc::c_int as UWORD
        }
    }
    if (*(psWidget as *mut W_BUTTON)).state &
           (0x1 as libc::c_int | 0x8 as libc::c_int | 0x10 as libc::c_int) as
               libc::c_uint != 0 {
        Down = 1 as libc::c_int as UDWORD
    }
    if (*(psWidget as *mut W_BUTTON)).state &
           0x2 as libc::c_int as libc::c_uint != 0 {
        Grey = 1 as libc::c_int as UDWORD
    }
    // now display
    pie_ImageFileID(FrontImages, im, x as libc::c_int, y as libc::c_int);
    // hilight with a number...just for player selector
    if usehl >= 10 as libc::c_int {
        pie_ImageFileID(IntImages,
                        (IMAGE_ASCII48 as libc::c_int - 10 as libc::c_int +
                             usehl) as UWORD,
                        x.wrapping_add(11 as libc::c_int as libc::c_uint) as
                            libc::c_int,
                        y.wrapping_add(8 as libc::c_int as libc::c_uint) as
                            libc::c_int);
    }
    // hilights etc..
    if Hilight != 0 && Grey == 0 {
        if Down != 0 {
            pie_ImageFileID(FrontImages, im2, x as libc::c_int,
                            y as libc::c_int);
        }
        if hiToUse != 0 {
            pie_ImageFileID(FrontImages, hiToUse, x as libc::c_int,
                            y as libc::c_int);
        }
    } else if Down != 0 {
        pie_ImageFileID(FrontImages, im2, x as libc::c_int, y as libc::c_int);
    }
    if Grey != 0 {
        // disabled, render something over it!
        pie_TransBoxFill(x as SDWORD, y as SDWORD,
                         x.wrapping_add((*psWidget).width as libc::c_uint) as
                             SDWORD,
                         y.wrapping_add((*psWidget).height as libc::c_uint) as
                             SDWORD);
    } else {
        // add a snap.
        AddCursorSnap(&mut InterfaceSnap,
                      x.wrapping_add(((*psWidget).width as libc::c_int /
                                          2 as libc::c_int) as libc::c_uint)
                          as SWORD,
                      y.wrapping_add(((*psWidget).height as libc::c_int /
                                          2 as libc::c_int) as libc::c_uint)
                          as SWORD, (*psWidget).formID, (*psWidget).id,
                      0 as *mut SNAPBIAS);
    };
}
// ///////////////////////////////////////////////////////////////////////////////////////
// ///////////////////////////////////////////////////////////////////////////////////////
// common widgets
#[no_mangle]
pub unsafe extern "C" fn addMultiEditBox(mut formid: UDWORD, mut id: UDWORD,
                                         mut x: UDWORD, mut y: UDWORD,
                                         mut tip: UDWORD,
                                         mut tipres: *mut STRING,
                                         mut icon: UDWORD, mut iconid: UDWORD)
 -> BOOL {
    let mut sEdInit: W_EDBINIT =
        W_EDBINIT{formID: 0,
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
                  pText: 0 as *mut STRING,
                  FontID: 0,
                  pBoxDisplay: None,
                  pFontDisplay: None,}; // editbox
    memset(&mut sEdInit as *mut W_EDBINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_EDBINIT>() as libc::c_ulong);
    sEdInit.formID = formid;
    sEdInit.id = id;
    sEdInit.style = 0 as libc::c_int as UDWORD;
    sEdInit.x = x as libc::c_short;
    sEdInit.y = y as libc::c_short;
    sEdInit.width = 196 as libc::c_int as UWORD;
    sEdInit.height = 30 as libc::c_int as UWORD;
    sEdInit.pText = tipres;
    sEdInit.FontID = WFont;
    sEdInit.pUserData = icon as *mut libc::c_void;
    sEdInit.pBoxDisplay =
        Some(displayMultiEditBox as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    if widgAddEditBox(psWScreen, &mut sEdInit) == 0 {
        return 0 as libc::c_int
    }
    // note drawing is done by the editbox draw tho...
    addMultiBut(psWScreen, 10250 as libc::c_int as UDWORD, iconid,
                x.wrapping_add(196 as libc::c_int as
                                   libc::c_uint).wrapping_add(2 as libc::c_int
                                                                  as
                                                                  libc::c_uint),
                y.wrapping_add(2 as libc::c_int as libc::c_uint),
                30 as libc::c_int as UDWORD, 30 as libc::c_int as UDWORD, tip,
                icon, icon, 0 as libc::c_int);
    return 1 as libc::c_int;
}
// / end of globals.
// ////////////////////////////////////////////////////////////////////////////
// Function protos
// widget functions
// ///////////////////////////////////////////////////////////////////////////////////////
#[no_mangle]
pub unsafe extern "C" fn addMultiBut(mut screen: *mut W_SCREEN,
                                     mut formid: UDWORD, mut id: UDWORD,
                                     mut x: UDWORD, mut y: UDWORD,
                                     mut width: UDWORD, mut height: UDWORD,
                                     mut tipres: UDWORD, mut norm: UDWORD,
                                     mut hi: UDWORD, mut hiIt: BOOL) -> BOOL {
    let mut sButInit: W_BUTINIT =
        W_BUTINIT{formID: 0,
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
                  pText: 0 as *mut STRING,
                  pTip: 0 as *mut STRING,
                  FontID: 0,};
    memset(&mut sButInit as *mut W_BUTINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_BUTINIT>() as libc::c_ulong);
    sButInit.formID = formid;
    sButInit.id = id;
    sButInit.style = 0 as libc::c_int as UDWORD;
    sButInit.x = x as libc::c_short;
    sButInit.y = y as libc::c_short;
    sButInit.width = width as libc::c_ushort;
    sButInit.height = height as libc::c_ushort;
    if tipres != 0 { sButInit.pTip = strresGetString(psStringRes, tipres) }
    sButInit.FontID = WFont;
    sButInit.pDisplay =
        Some(displayMultiBut as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    /*
	if (hiIt == 1)
	{
		sButInit.pUserData = (void*)PACKDWORD_TRI(0,norm , hi);
	}
	else if (hiIt == 0)
	{
		sButInit.pUserData = (void*)PACKDWORD_TRI(1,norm , hi);
	}
	else
	{}
	*/
    sButInit.pUserData =
        (((hiIt & 0x3ff as libc::c_int) << 20 as libc::c_int) as libc::c_uint
             |
             (norm & 0x3ff as libc::c_int as libc::c_uint) <<
                 10 as libc::c_int |
             hi & 0x3ff as libc::c_int as libc::c_uint) as
            *mut libc::c_void; //45
    if widgAddButton(screen, &mut sButInit) == 0 {
        return 0 as libc::c_int
    } //above droid.
    return 1 as libc::c_int; //2500;		//scale them to 200% in button render
}
#[no_mangle]
pub unsafe extern "C" fn displayForceDroid(mut psWidget: *mut _widget,
                                           mut xOffset: UDWORD,
                                           mut yOffset: UDWORD,
                                           mut pColours: *mut UDWORD) {
    let mut Rotation: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut Position: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut x: UDWORD = ((*psWidget).x as libc::c_uint).wrapping_add(xOffset);
    let mut y: UDWORD = ((*psWidget).y as libc::c_uint).wrapping_add(yOffset);
    let mut tlx: UDWORD = 0;
    let mut tly: UDWORD = 0;
    let mut brx: UDWORD = 0;
    let mut bry: UDWORD = 0;
    tlx = x.wrapping_sub(58 as libc::c_int as libc::c_uint);
    tly = y.wrapping_sub(80 as libc::c_int as libc::c_uint);
    brx = x.wrapping_add(58 as libc::c_int as libc::c_uint);
    bry = y.wrapping_add(23 as libc::c_int as libc::c_uint);
    pie_BoxFill(tlx as libc::c_int, tly as libc::c_int, brx as libc::c_int,
                bry as libc::c_int, 0x6067a0 as libc::c_int as uint32);
    pie_BoxFill(tlx.wrapping_add(1 as libc::c_int as libc::c_uint) as
                    libc::c_int,
                tly.wrapping_add(1 as libc::c_int as libc::c_uint) as
                    libc::c_int,
                brx.wrapping_sub(1 as libc::c_int as libc::c_uint) as
                    libc::c_int,
                bry.wrapping_sub(1 as libc::c_int as libc::c_uint) as
                    libc::c_int, 0x2f3050 as libc::c_int as uint32);
    pie_Set2DClip(tlx.wrapping_add(1 as libc::c_int as libc::c_uint) as UWORD
                      as libc::c_int,
                  tly.wrapping_add(1 as libc::c_int as libc::c_uint) as UWORD
                      as libc::c_int,
                  brx.wrapping_sub(1 as libc::c_int as libc::c_uint) as UWORD
                      as libc::c_int,
                  bry.wrapping_sub(1 as libc::c_int as libc::c_uint) as UWORD
                      as libc::c_int);
    pie_SetGeometricOffset(x as libc::c_int, y as libc::c_int);
    Rotation.x = -(20 as libc::c_int);
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
    Position.z = 2000 as libc::c_int;
    if !Force.pMembers.is_null() {
        displayComponentButtonTemplate((*Force.pMembers).pTempl,
                                       &mut Rotation, &mut Position,
                                       1 as libc::c_int,
                                       2 as libc::c_int * 72 as libc::c_int);
    }
    pie_Set2DClip(0 as libc::c_int, 0 as libc::c_int,
                  (*psRendSurface).width - 0 as libc::c_int,
                  (*psRendSurface).height - 0 as libc::c_int);
}
