use ::libc;
extern "C" {
    /* *
 * \fn void PHYSFS_freeList(void *listVar)
 * \brief Deallocate resources of lists returned by PhysicsFS.
 *
 * Certain PhysicsFS functions return lists of information that are
 *  dynamically allocated. Use this function to free those resources.
 *
 * It is safe to pass a NULL here, but doing so will cause a crash in versions
 *  before PhysicsFS 2.1.0.
 *
 *   \param listVar List of information specified as freeable by this function.
 *                  Passing NULL is safe; it is a valid no-op.
 *
 * \sa PHYSFS_getCdRomDirs
 * \sa PHYSFS_enumerateFiles
 * \sa PHYSFS_getSearchPath
 */
    #[no_mangle]
    fn PHYSFS_freeList(listVar: *mut libc::c_void);
    /* *
 * \fn const char *PHYSFS_getDirSeparator(void)
 * \brief Get platform-dependent dir separator string.
 *
 * This returns "\\" on win32, "/" on Unix, and ":" on MacOS. It may be more
 *  than one character, depending on the platform, and your code should take
 *  that into account. Note that this is only useful for setting up the
 *  search/write paths, since access into those dirs always use '/'
 *  (platform-independent notation) to separate directories. This is also
 *  handy for getting platform-independent access when using stdio calls.
 *
 *   \return READ ONLY null-terminated string of platform's dir separator.
 */
    #[no_mangle]
    fn PHYSFS_getDirSeparator() -> *const libc::c_char;
    /* *
 * \fn const char *PHYSFS_getWriteDir(void)
 * \brief Get path where PhysicsFS will allow file writing.
 *
 * Get the current write dir. The default write dir is NULL.
 *
 *  \return READ ONLY string of write dir in platform-dependent notation,
 *           OR NULL IF NO WRITE PATH IS CURRENTLY SET.
 *
 * \sa PHYSFS_setWriteDir
 */
    #[no_mangle]
    fn PHYSFS_getWriteDir() -> *const libc::c_char;
    /* *
 * \fn int PHYSFS_addToSearchPath(const char *newDir, int appendToPath)
 * \brief Add an archive or directory to the search path.
 *
 * \deprecated As of PhysicsFS 2.0, use PHYSFS_mount() instead. This
 *             function just wraps it anyhow.
 *
 * This function is equivalent to:
 *
 * \code
 *  PHYSFS_mount(newDir, NULL, appendToPath);
 * \endcode
 *
 * You must use this and not PHYSFS_mount if binary compatibility with
 *  PhysicsFS 1.0 is important (which it may not be for many people).
 *
 * \sa PHYSFS_mount
 * \sa PHYSFS_removeFromSearchPath
 * \sa PHYSFS_getSearchPath
 */
    #[no_mangle]
    fn PHYSFS_addToSearchPath(newDir: *const libc::c_char,
                              appendToPath: libc::c_int) -> libc::c_int;
    /* *
 * \fn int PHYSFS_removeFromSearchPath(const char *oldDir)
 * \brief Remove a directory or archive from the search path.
 *
 * \deprecated As of PhysicsFS 2.1, use PHYSFS_unmount() instead. This
 *             function just wraps it anyhow. There's no functional difference
 *             except the vocabulary changed from "adding to the search path"
 *             to "mounting" when that functionality was extended, and thus
 *             the preferred way to accomplish this function's work is now
 *             called "unmounting."
 *
 * This function is equivalent to:
 *
 * \code
 *  PHYSFS_unmount(oldDir);
 * \endcode
 *
 * You must use this and not PHYSFS_unmount if binary compatibility with
 *  PhysicsFS 1.0 is important (which it may not be for many people).
 *
 * \sa PHYSFS_addToSearchPath
 * \sa PHYSFS_getSearchPath
 * \sa PHYSFS_unmount
 */
    #[no_mangle]
    fn PHYSFS_removeFromSearchPath(oldDir: *const libc::c_char)
     -> libc::c_int;
    /* *
 * \fn char **PHYSFS_enumerateFiles(const char *dir)
 * \brief Get a file listing of a search path's directory.
 *
 * \warning In PhysicsFS versions prior to 2.1, this function would return
 *          as many items as it could in the face of a failure condition
 *          (out of memory, disk i/o error, etc). Since this meant apps
 *          couldn't distinguish between complete success and partial failure,
 *          and since the function could always return NULL to report
 *          catastrophic failures anyway, in PhysicsFS 2.1 this function's
 *          policy changed: it will either return a list of complete results
 *          or it will return NULL for any failure of any kind, so we can
 *          guarantee that the enumeration ran to completion and has no gaps
 *          in its results.
 *
 * Matching directories are interpolated. That is, if "C:\mydir" is in the
 *  search path and contains a directory "savegames" that contains "x.sav",
 *  "y.sav", and "z.sav", and there is also a "C:\userdir" in the search path
 *  that has a "savegames" subdirectory with "w.sav", then the following code:
 *
 * \code
 * char **rc = PHYSFS_enumerateFiles("savegames");
 * char **i;
 *
 * for (i = rc; *i != NULL; i++)
 *     printf(" * We've got [%s].\n", *i);
 *
 * PHYSFS_freeList(rc);
 * \endcode
 *
 *  \...will print:
 *
 * \verbatim
 * We've got [x.sav].
 * We've got [y.sav].
 * We've got [z.sav].
 * We've got [w.sav].\endverbatim
 *
 * Feel free to sort the list however you like. However, the returned data
 *  will always contain no duplicates, and will be always sorted in alphabetic
 *  (rather: case-sensitive Unicode) order for you.
 *
 * Don't forget to call PHYSFS_freeList() with the return value from this
 *  function when you are done with it.
 *
 *    \param dir directory in platform-independent notation to enumerate.
 *   \return Null-terminated array of null-terminated strings, or NULL for
 *           failure cases.
 *
 * \sa PHYSFS_enumerate
 */
    #[no_mangle]
    fn PHYSFS_enumerateFiles(dir: *const libc::c_char)
     -> *mut *mut libc::c_char;
    /* *
 * \fn int PHYSFS_exists(const char *fname)
 * \brief Determine if a file exists in the search path.
 *
 * Reports true if there is an entry anywhere in the search path by the
 *  name of (fname).
 *
 * Note that entries that are symlinks are ignored if
 *  PHYSFS_permitSymbolicLinks(1) hasn't been called, so you
 *  might end up further down in the search path than expected.
 *
 *    \param fname filename in platform-independent notation.
 *   \return non-zero if filename exists. zero otherwise.
 */
    #[no_mangle]
    fn PHYSFS_exists(fname: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn memFreeRelease(pMemToFree: *mut libc::c_void);
    #[no_mangle]
    fn memMallocRelease(Size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn memGetBlockHeap() -> *mut _block_heap;
    #[no_mangle]
    fn memSetBlockHeap(psHeap: *mut _block_heap);
    #[no_mangle]
    fn abort() -> !;
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
    fn loadFile(pFileName: *const libc::c_char,
                ppFileData: *mut *mut libc::c_char, pFileSize: *mut UDWORD)
     -> BOOL;
    #[no_mangle]
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_uint;
    #[no_mangle]
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_uint)
     -> *mut libc::c_void;
    #[no_mangle]
    fn resLoad(pResFile: *mut STRING, blockID: SDWORD,
               pLoadBuffer: *mut libc::c_char, bufferSize: SDWORD,
               psMemHeap: *mut _block_heap) -> BOOL;
    /* Release all the resources currently loaded and the resource load functions */
    #[no_mangle]
    fn resReleaseAll();
    /* Release all the resources currently loaded but keep the resource load functions */
    #[no_mangle]
    fn resReleaseAllData();
    #[no_mangle]
    fn resGetData(pType: *mut STRING, pID: *mut STRING) -> *mut libc::c_void;
    #[no_mangle]
    fn blkCreate(ppsHeap: *mut *mut BLOCK_HEAP, init: SDWORD, ext: SDWORD)
     -> BOOL;
    #[no_mangle]
    fn blkDestroy(psHeap: *mut BLOCK_HEAP);
    #[no_mangle]
    fn blkReset(psHeap: *mut BLOCK_HEAP);
    #[no_mangle]
    fn frameSetCursorFromRes(resID: WORD);
    // initialise droid module
    #[no_mangle]
    fn droidInit() -> BOOL;
    #[no_mangle]
    fn setSelectedGroup(groupNumber: UDWORD);
    /* **************************************************************************/
    #[no_mangle]
    fn animObj_Init(pDiedFunc: ANIMOBJDIEDTESTFUNC) -> BOOL;
    /* **************************************************************************/
    #[no_mangle]
    fn anim_Init(_: GETSHAPEFUNC) -> BOOL;
    /*
 * Objects.h
 *
 * A header file that groups together all the object header files
 *
 */
    /* Initialise the object system */
    #[no_mangle]
    fn objInitialise() -> BOOL;
    #[no_mangle]
    fn animObj_Shutdown() -> BOOL;
    #[no_mangle]
    fn anim_Shutdown() -> BOOL;
    /*
 * Research.h
 *
 * structures required for research stats
 *
 */
    //used for loading in the research stats into the appropriate list
    /* The store for the research stats */
    //List of pointers to arrays of PLAYER_RESEARCH[numResearch] for each player
    //used for Callbacks to say which topic was last researched
    /* Default level of sensor, repair and ECM */
    //extern BOOL loadResearch(void);
    //Load the pre-requisites for a research list
    //Load the artefacts for a research list
    //Load the pre-requisites for a research list
    //Load the Structures for a research list
    /*function to check what can be researched for a particular player at any one 
  instant. Returns the number to research*/
//extern UBYTE fillResearchList(UBYTE *plist, UDWORD playerID, UWORD topic, 
//							   UWORD limit);
//needs to be UWORD sized for Patches
    /* process the results of a completed research topic */
    //this just inits all the research arrays
    //this free the memory used for the research
    #[no_mangle]
    fn ResearchRelease() -> BOOL;
    /* Shutdown the object system */
    #[no_mangle]
    fn objShutdown() -> BOOL;
    #[no_mangle]
    fn mechShutdown() -> BOOL;
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
    /*
 * Move.h
 *
 * Interface for the unit movement system
 *
 */
    /* The base movement speed */
    // The next object that should get the router when a lot of units are
// in a MOVEROUTE state
    /* Initialise the movement system */
    /* Update the base speed for all movement */
    /* Set a target location for a droid to move to  - returns a BOOL based on if there is a path to the destination (TRUE if there is a path)*/
    /* Set a target location for a droid to move to  - returns a BOOL based on if there is a path to the destination (TRUE if there is a path)*/
// the droid will not join a formation when it gets to the location
    // move a droid directly to a location (used by vtols only)
    // Get a droid to turn towards a locaton
    /* Stop a droid */
    /*Stops a droid dead in its tracks - doesn't allow for any little skidding bits*/
    /* Get a droid to do a frame's worth of moving */
    /* Frame update for the movement of a tracked droid */
    /* update body and turret to local slope */
    #[no_mangle]
    fn moveSetFormationSpeedLimiting(_: BOOL);
    #[no_mangle]
    fn moveInitialise() -> BOOL;
    #[no_mangle]
    fn visInitialise() -> BOOL;
    /*
 * AI.h
 *
 * Definitions for the AI system structures
 *
 */
    // states of alliance between players
    // for setting values only.
    //alliance possibilities for games.
    //#define GROUP_WINS		2
    // alliances
    /* Check no alliance has formed*/
    /* Initialise the AI system */
    #[no_mangle]
    fn aiInitialise() -> BOOL;
    /*
 * Map.h
 *
 * Definitions for the map structure
 *
 */
    // Visibility bits - can also be accessed as a byte (as a whole).
    /* The different types of terrain as far as the game is concerned */
    /* change these if you change above - maybe wrap up in enumerate? */
    /* Flags for whether texture tiles are flipped in X and Y or rotated */
    // This bit describes the direction the tile is split into 2 triangles (same as triangleFlip)
    // set when the tile has the structure cursor over it
    // NASTY - this should be in tileInfoBits but there isn't any room left
    // units can drive on this even if there is a structure or feature on it
    //#define	BITS_TILE_HIGHLIGHT 0x8
    // show small structures - tank traps / bunkers
    // bit set temporarily by find path to mark a blocking tile
    // bit set to show a gateway on the tile
    // bit set to show a tall structure which camera needs to avoid.
    /*#ifndef PSX	// Extra tile info bits.... WIN32 only
#define	EXTRA_BITS_SENSOR	0x1
#define	EXTRA_BITS_2		0x2
#define	EXTRA_BITS_3		0x4
#define	EXTRA_BITS_4		0x8
#define	EXTRA_BITS_5		0x10
#define	EXTRA_BITS_6		0x20
#define	EXTRA_BITS_7		0x40
#define	EXTRA_BITS_8		0x80
#endif*/
    //#define TILE_HIGHLIGHT(x)		(x->tileInfoBits & BITS_TILE_HIGHLIGHT)
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
    /* Information stored with each tile */
// The name is now changed to MAPTILE to allow correct compilation on the PlayStation
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
    /* The maximum map size */
    /* The size and contents of the map */
    /* The shift on the y coord when calculating into the map */
    /* The number of units accross a tile */
    /* The shift on a coordinate to get the tile coordinate */
    /* The mask to get internal tile coords from a full coordinate */
    /* Shutdown the map module */
    #[no_mangle]
    fn mapShutdown() -> BOOL;
    /* Shutdown the AI system */
    #[no_mangle]
    fn aiShutdown() -> BOOL;
    /* **************************************************************************/
    /* **************************************************************************/
    /* **************************************************************************/
    #[no_mangle]
    fn audio_CheckAllUnloaded();
    /*
 * ObjMem.h
 *
 * Routines for managing object's memory
 *
 */
    //the died flag for a droid is set to this when it gets added to the non-current list
    /* The memory heaps for the different object types */
    // the memory heap for templates
    /* The lists of objects allocated */
    /* The list of destroyed objects */
    /* Initialise the object heaps */
    /* Release the object heaps */
    /* General housekeeping for the object system */
    /* Create a new droid */
    /* add the droid to the Droid Lists */
    /*destroy a droid */
    /* Remove all droids */
    /*Remove a single Droid from its list*/
    /*Removes all droids that may be stored in the mission lists*/
    /*Removes all droids that may be stored in the limbo lists*/
    /* Create a new structure */
    /* add the structure to the Structure Lists */
    /* Destroy a structure */
    /* Remove all structures */
    /*Remove a single Structure from a list*/
    /* Create a new Feature */
    /* add the feature to the Feature Lists */
    /* Destroy a feature */
    /* Remove all features */
    /* Create a new Flag Position */
    /* add the Flag Position to the Flag Position Lists */
    /* Remove a Flag Position from the Lists */
    // free all flag positions
    #[no_mangle]
    fn freeAllFlagPositions();
    #[no_mangle]
    fn freeAllFeatures();
    #[no_mangle]
    fn freeAllDroids();
    #[no_mangle]
    fn freeAllStructs();
    #[no_mangle]
    static mut scrollMaxY: SDWORD;
    #[no_mangle]
    static mut scrollMinY: SDWORD;
    #[no_mangle]
    static mut scrollMaxX: SDWORD;
    #[no_mangle]
    static mut scrollMinX: SDWORD;
    /*
 * Structure.h
 *
 * Definitions for the structures.
 *
 */
    // how long to wait between CALL_STRUCT_ATTACKED's - plus how long to flash on radar for
    // This should really be logarithmic
    /* explosion data for when a structure is blown up - used by features as well*/
    //production loop max
    //10
    /*This should correspond to the structLimits! */
    //used to flag when the Factory is ready to start building
    //distance that VTOLs can be away from the reArm pad
    //Value is stored for easy access to this structure stat
    // the structure that was last hit
    //stores which player the production list has been set up for
    //holder for all StructureStats
    //holds the upgrades attained through research for structure stats
    //holds the upgrades for the functionality of structures through research
    //used to hold the modifiers cross refd by weapon effect and structureStrength
    /*Load the Structure Strength Modifiers from the file exported from Access*/
    /* Set the type of droid for a factory to build */
    //temp test function for creating structures at the start of the game
    // Set the tile no draw flags for a structure
    //builds a specified structure at a given location
    /* The main update routine for all Structures */
    /* Release all resources associated with a structure */
    /* Remove a structure and free it's memory */
    // remove a structure from a game without any visible effects
// bDestroy = TRUE if the object is to be destroyed
// (for example used to change the type of wall at a location)
    //fills the list with Structures that can be built
    /* checks that the location is a valid one to build on and sets the outline colour
x and y in tile-coords*/
    /* for a new structure, find a location along an edge which the droid can get
to and return this as the destination for the droid */
//extern BOOL getDroidDestination(STRUCTURE_STATS *psPositionStats, UDWORD structX, 
//	UDWORD structY, UDWORD * pDroidX, UDWORD *pDroidY);
/*for a structure or feature, find a location along an edge which the droid can get
to and return this as the destination for the droid*/
    /* check along the width of a structure for an empty space */
    /* check along the length of a structure for an empty space */
    //initialise the structure limits structure
    /* set the current number of structures of each type built */
    /* get a stat inc based on the name */
    /*check to see if the structure is 'doing' anything  - return TRUE if idle*/
    /*checks to see if any structure exists of a specified type with a specified status */
    /*sets the point new droids go to - x/y in world coords for a Factory*/
    //extern void createAssemblyPoint(STRUCTURE* psStruct);
    /* consider delivery points when selected by player*/
    /*called when a structure has been built - checks through the list of callbacks 
for the scripts*/
    /*initialises the flag before a new data set is loaded up*/
    //called at start of missions
    /* get demolish stat */
    /*find a location near to the factory to start the droid of*/
    /*sets the flag to indicate a Power Generator Exists - so do Oil Derrick anim*/
//extern void setPowerGenExists(BOOL state, UDWORD player);
/*returns teh status of the flag*/
//extern BOOL getPowerGenExists(UDWORD player);
    /*sets the flag to indicate a HQ Exists - so draw Radar*/
    #[no_mangle]
    fn setHQExists(state: BOOL, player: UDWORD);
    #[no_mangle]
    static mut apsStructLists: [*mut STRUCTURE; 8];
    #[no_mangle]
    static mut mapHeight: UDWORD;
    #[no_mangle]
    static mut mapWidth: UDWORD;
    /* The time for the last frame */
    /* The current time in the game world */
    // Never stops.
    /* The time for the last frame */
    // Never stops.
    /* Initialise the game clock */
    /* Call this each loop to update the game timer */
    /* Returns TRUE if gameTime is stopped. */
    /* Call this to stop the game timer */
    /* Call this to restart the game timer after a call to gameTimeStop */
    /*Call this to reset the game timer*/
    // reset the game time modifiers
    #[no_mangle]
    fn gameTimeResetMod();
    #[no_mangle]
    static mut psMapTiles: *mut MAPTILE;
    #[no_mangle]
    fn audio_StopAll();
    #[no_mangle]
    fn gameTimeInit() -> BOOL;
    #[no_mangle]
    fn audio_Disabled() -> BOOL;
    #[no_mangle]
    fn audio_Shutdown() -> BOOL;
    #[no_mangle]
    fn pie_GetVideoBufferWidth() -> UDWORD;
    #[no_mangle]
    fn pie_GetVideoBufferHeight() -> UDWORD;
    #[no_mangle]
    fn audio_Init(pStopTrackCallback: AUDIO_CALLBACK) -> BOOL;
    /*checks to see if an electronic warfare weapon is attached to the droid*/
    /*checks to see if the droid is currently being repaired by another*/
    //count how many Command Droids exist in the world at any one moment
    /* Set up a droid to clear a wrecked building feature - returns true if successful */
    /* Update a construction droid while it is clearing 
   returns TRUE while continues */
    /*For a given repair droid, check if there are any damaged droids within 
a defined range*/
    //access function
    /*returns TRUE if a VTOL Weapon Droid which has completed all runs*/
    /*Checks a vtol for being fully armed and fully repaired to see if ready to 
leave reArm pad */
    /*this mends the VTOL when it has been returned to home base whilst on an
offworld mission*/
    /*checks if the droid is a VTOL droid and updates the attack runs as required*/
    /*returns a count of the base number of attack runs for the weapon attached to the droid*/
    //assign rearmPad to the VTOL
    //don't use this function any more - the droid checks each frame for this to have died
//look through all droids to see if any are associated with the ReArming Pad
//extern void releaseVTOLPad(STRUCTURE *psReArmPad);
// true if a vtol is waiting to be rearmed by a particular rearm pad
    // true if a vtol droid currently returning to be rearmed
    // true if a droid is currently attacking
    // see if there are any other vtols attacking the same target
// but still rearming
    /*compares the droid sensor type with the droid weapon type to see if the 
FIRE_SUPPORT order can be assigned*/
    // return whether a droid has a CB sensor on it
    // give a droid from one player to another - used in Electronic Warfare and multiplayer
    /*calculates the electronic resistance of a droid based on its experience level*/
    /*this is called to check the weapon is 'allowed'. Check if VTOL, the weapon is 
direct fire. Also check numVTOLattackRuns for the weapon is not zero - return 
TRUE if valid weapon*/
    /*called when a Template is deleted in the Design screen*/
    // Select a droid and do any necessary housekeeping.
    // De-select a droid and do any necessary housekeeping.
    /*calculate the power cost to repair a droid*/
    /*power cost for One repair point*/
    /* audio finished callback */
    #[no_mangle]
    fn droidAudioTrackStopped(psSample: *mut AUDIO_SAMPLE) -> BOOL;
    /*
 * Display.h
 *
 * Definitions for the display system structures and routines.
 *
 */
    /* Initialise the display system */
    #[no_mangle]
    fn dispInitialise() -> BOOL;
    #[no_mangle]
    static mut godMode: BOOL;
    #[no_mangle]
    fn displayInitVars();
    // Initialise the findpath routine
    #[no_mangle]
    fn astarInitialise() -> BOOL;
    // Shutdown the findpath routine
    #[no_mangle]
    fn fpathShutDown();
    /*
 * Message.h
 *
 * Functions for the messages shown in the Intelligence Map View
 */
    /* The lists of messages allocated */
    /* The current tutorial message - there is only ever one at a time. They are displayed 
when called by the script. They are not to be re-displayed*/
//extern MESSAGE		tutorialMessage;
/* The IMD to use for the proximity messages */
    /* The list of proximity displays allocated */
    //allocates the viewdata heap
    /* Initialise the message heaps */
    /* Release the message heaps */
    #[no_mangle]
    fn messageShutdown() -> BOOL;
    #[no_mangle]
    fn initMessage() -> BOOL;
    //destroys the viewdata heap
    #[no_mangle]
    fn viewDataHeapShutDown();
    #[no_mangle]
    fn initViewData() -> BOOL;
    #[no_mangle]
    fn intShutDown();
    #[no_mangle]
    static mut DisplayBuffer: *mut libc::c_char;
    #[no_mangle]
    static mut displayBufferSize: UDWORD;
    #[no_mangle]
    fn freeMessages();
    #[no_mangle]
    fn intResetPreviousObj();
    #[no_mangle]
    fn intResetScreen(NoAnim: BOOL);
    #[no_mangle]
    fn intInitialise() -> BOOL;
    #[no_mangle]
    static mut selectedPlayer: UDWORD;
    #[no_mangle]
    fn widgShutDown();
    #[no_mangle]
    fn widgReset();
    #[no_mangle]
    fn widgInitialise(psInit: *mut W_HEAPINIT) -> BOOL;
    #[no_mangle]
    fn SetMaxDist(MaxX: SDWORD, MaxY: SDWORD);
    #[no_mangle]
    fn snapInitVars();
    #[no_mangle]
    fn frontendInitVars() -> BOOL;
    #[no_mangle]
    fn initLoadingScreen(drawbdrop: BOOL, bRenderActive: BOOL);
    #[no_mangle]
    fn closeLoadingScreen();
    #[no_mangle]
    fn setScriptWinLoseVideo(val: UBYTE);
    // FIXME Stubfile!
/* **************************************************************************/
/*
 * Ani.h
 *
 * Warzone animation function wrappers
 *
 * Gareth Jones 16/12/97
 */
/* **************************************************************************/
    /* **************************************************************************/
    /* **************************************************************************/
    #[no_mangle]
    fn anim_GetShapeFunc(pStr: *mut STRING) -> *mut libc::c_void;
    #[no_mangle]
    fn driveInitVars(Restart: BOOL);
    //function prototypes
    /* reset object list */
    #[no_mangle]
    fn bucketSetupList() -> BOOL;
    /*
 * Data.h
 *
 * Data loading functions
 */
    /* Pass all the data loading functions to the framework library */
    #[no_mangle]
    fn dataInitLoadFuncs() -> BOOL;
    /*
 * RayCast.h
 *
 * Raycaster functions
 */
    // maximum length for a visiblity ray
    /* Initialise the visibility rays */
    #[no_mangle]
    fn rayInitialise() -> BOOL;
    /* Initialise the string system */
    #[no_mangle]
    fn stringsInitialise() -> BOOL;
    #[no_mangle]
    fn enableConsoleDisplay(state: BOOL);
    //renderer capability
    #[no_mangle]
    fn pie_SetTranslucent(val: BOOL);
    #[no_mangle]
    fn pie_SetAdditive(val: BOOL);
    //mouse states
    #[no_mangle]
    fn pie_SetMouse(ImageFile: *mut IMAGEFILE, ImageID: UWORD);
    /*
 * config.h
 * load and save favourites to the registry.
 */
    #[no_mangle]
    fn loadConfig(bResourceAvailable: BOOL) -> BOOL;
    #[no_mangle]
    fn saveConfig() -> BOOL;
    #[no_mangle]
    fn closeConfig();
    /* **************************************************************************/
/*
 *	Global ProtoTypes
 */
/* **************************************************************************/
    #[no_mangle]
    fn pie_Initialise() -> BOOL;
    //*************************************************************************
    #[no_mangle]
    fn pie_TexShutDown();
    //*************************************************************************
    #[no_mangle]
    fn iV_Reset(bResetPal: libc::c_int);
    #[no_mangle]
    fn iV_ShutDown();
    #[no_mangle]
    fn iV_CreateFontIndirect(ImageFile: *mut IMAGEFILE,
                             AsciiTable: *mut UWORD, SpaceSize: libc::c_int)
     -> libc::c_int;
    // masks for the secondary order state
    //call this *AFTER* every mission so it gets reset
    #[no_mangle]
    fn initRunData();
    #[no_mangle]
    fn grpInitialise() -> BOOL;
    #[no_mangle]
    fn grpShutDown();
    #[no_mangle]
    static mut radarOnScreen: BOOL;
    #[no_mangle]
    fn init3DView() -> libc::c_int;
    #[no_mangle]
    fn setEnergyBarDisplay(val: BOOL);
    #[no_mangle]
    fn atmosInitSystem();
    #[no_mangle]
    fn environInit() -> BOOL;
    #[no_mangle]
    fn environShutDown();
    #[no_mangle]
    fn war_SetFog(val: BOOL);
    #[no_mangle]
    fn war_GetTranslucent() -> BOOL;
    #[no_mangle]
    fn war_GetAdditive() -> BOOL;
    #[no_mangle]
    fn war_GetPlayAudioCDs() -> BOOL;
    // ////////////////////////////////////////////////////////////////////////////
// Game Options and stats. 
    #[no_mangle]
    static mut game: MULTIPLAYERGAME;
    // the game description.
    #[no_mangle]
    static mut bMultiPlayer: BOOL;
    // Startup. mulitopt
    #[no_mangle]
    fn multiTemplateSetup() -> BOOL;
    #[no_mangle]
    fn multiInitialise() -> BOOL;
    // for Init.c
    #[no_mangle]
    fn multiShutdown() -> BOOL;
    #[no_mangle]
    fn multiGameInit() -> BOOL;
    #[no_mangle]
    fn multiGameShutdown() -> BOOL;
    #[no_mangle]
    fn eventReset();
    #[no_mangle]
    fn eventTimeReset(initTime: UDWORD);
    #[no_mangle]
    fn eventFireCallbackTrigger(callback: TRIGGER_TYPE);
    // Initialise the script system
    #[no_mangle]
    fn scrTabInitialise() -> BOOL;
    // Shut down the script system
    #[no_mangle]
    fn scrShutDown();
    // reset the script value module
    #[no_mangle]
    fn scrvReset();
    #[no_mangle]
    fn initMiscImds() -> BOOL;
    #[no_mangle]
    fn keyClearMappings();
    #[no_mangle]
    fn keyInitMappings(bForceDefaults: BOOL);
    #[no_mangle]
    fn processDebugMappings(val: BOOL);
    #[no_mangle]
    fn Edit3DInitVars();
    // = {0,1,2,3,4,5,6,7}
    #[no_mangle]
    fn initPlayerColours();
    #[no_mangle]
    fn setPlayerColour(player: UDWORD, col: UDWORD) -> BOOL;
    #[no_mangle]
    fn getPlayerColour(pl: UDWORD) -> UBYTE;
    // initialise the findpath module
    #[no_mangle]
    fn fpathInitialise() -> BOOL;
    #[no_mangle]
    static mut bDisableLobby: BOOL;
    #[no_mangle]
    static mut IntImages: *mut IMAGEFILE;
    /*allocate the space for the playerPower*/
    #[no_mangle]
    fn allocPlayerPower() -> BOOL;
    /*Free the space used for playerPower */
    #[no_mangle]
    fn releasePlayerPower();
    #[no_mangle]
    fn InitRadar() -> BOOL;
    #[no_mangle]
    fn ShutdownRadar() -> BOOL;
    #[no_mangle]
    fn SetFormAudioIDs(OpenID: libc::c_int, CloseID: libc::c_int);
    // Initialise the formation system
    #[no_mangle]
    fn formationInitialise() -> BOOL;
    // Shutdown the formation system
    #[no_mangle]
    fn formationShutDown();
    /* **************************************************************************/
    /* **************************************************************************/
    #[no_mangle]
    fn cdAudio_Open(user_musicdir: *mut libc::c_char) -> BOOL;
    #[no_mangle]
    fn cdAudio_Close() -> BOOL;
    #[no_mangle]
    fn cdAudio_PlayTrack(iTrack: SDWORD) -> BOOL;
    #[no_mangle]
    fn cdAudio_Stop() -> BOOL;
    /* **************************************************************************/
    /* **************************************************************************/
    /* **************************************************************************/
    #[no_mangle]
    fn mixer_Open() -> BOOL;
    #[no_mangle]
    fn mixer_Close();
    #[no_mangle]
    fn preProcessVisibility();
    /*
 * Mission.h
 *
 * Mission defines for the game
 *
 */
    /*the number of areas that can be defined to prevent buildings being placed - 
used for Transporter Landing Zones 0-7 are for players, 8 = LIMBO_LANDING*/
    // Set by scrFlyInTransporter. True if were currenly tracking the transporter.
    #[no_mangle]
    static mut bTrackingTransporter: BOOL;
    #[no_mangle]
    fn initMission();
    #[no_mangle]
    fn missionShutDown() -> BOOL;
    //this is called everytime the game is quit
    #[no_mangle]
    fn releaseMission();
    // mission results.
    //timer display for transporter timer
    //position defines
    // status of the mission result screens.
    #[no_mangle]
    static mut MissionResUp: BOOL;
    #[no_mangle]
    fn intRemoveMissionResultNoAnim();
    // reset the vtol landing pos
    #[no_mangle]
    fn resetVTOLLandingPos();
    /*
 * Transporter.h
 *
 * Functions for the display/functionality of the Transporter
 */
    //The Transporter base form
    //The Transporter Contents form
    //The Droid base form
    //The Transporter Launch button
    //The Transporter capacity label
    //initialises Transporter variables
    #[no_mangle]
    fn initTransporters();
    // whether an object is in a fire
    // whether an object has just left the fire, but is still burning
    // how long an object burns for after leaving a fire
    // how much damaga a second an object takes when it is burning
    #[no_mangle]
    fn proj_InitSystem() -> BOOL;
    #[no_mangle]
    fn proj_Shutdown() -> BOOL;
    // parse a level description data file
    #[no_mangle]
    fn levParse(pBuffer: *mut libc::c_char, size: SDWORD,
                datadir: libc::c_int) -> BOOL;
    // shutdown the level system
    #[no_mangle]
    fn levShutDown();
    //get the type of level currently being loaded of GTYPE type
    #[no_mangle]
    fn getLevelLoadType() -> SDWORD;
    #[no_mangle]
    static mut loopMissionState: LOOP_MISSION_STATE;
    //set all the pause states to the state value
    #[no_mangle]
    fn setAllPauseStates(state: BOOL);
    /*
 * CmdDroid.h
 *
 * Typedef's for command droids
 *
 */
    // The number of available command droids for each player
//extern SWORD	numCommandDroids[MAX_PLAYERS];
    // The command droids for each player
//extern COMMAND_DROID	asCommandDroids[MAX_PLAYERS][MAX_CMDDROIDS];
    // Initialise the command droids
    #[no_mangle]
    fn cmdDroidInit() -> BOOL;
    // ShutDown the command droids
    #[no_mangle]
    fn cmdDroidShutDown();
    // note that commander experience should be increased
    #[no_mangle]
    fn cmdDroidMultiExpBoost(bDoit: BOOL);
    #[no_mangle]
    fn effectResetUpdates();
    // reset the script externals for a new level
    #[no_mangle]
    fn scrExternReset();
    // The number of tiles per grid
    // The size of the grid
//#define GRID_WIDTH	(MAP_MAXWIDTH/GRID_SIZE)
//#define GRID_HEIGHT	(MAP_MAXHEIGHT/GRID_SIZE)
    // The map grid 
//extern GRID_ARRAY	*apsMapGrid[GRID_WIDTH][GRID_HEIGHT];
    // initialise the grid system
    #[no_mangle]
    fn gridInitialise() -> BOOL;
    // shutdown the grid system
    #[no_mangle]
    fn gridShutDown();
    // reset the grid system
    #[no_mangle]
    fn gridReset();
    // initialise the cluster system
    #[no_mangle]
    fn clustInitialise();
    #[no_mangle]
    fn gwShutDown();
    #[no_mangle]
    fn gwInitialise() -> BOOL;
    //extern void	initLighting( void );
    #[no_mangle]
    fn initLighting(x1: UDWORD, y1: UDWORD, x2: UDWORD, y2: UDWORD);
    #[no_mangle]
    fn addSubdirs(basedir: *const libc::c_char, subdir: *const libc::c_char,
                  appendToPath: BOOL, checkList: *mut *mut libc::c_char);
    #[no_mangle]
    fn removeSubdirs(basedir: *const libc::c_char,
                     subdir: *const libc::c_char,
                     checkList: *mut *mut libc::c_char);
    /*
 * Init.c
 *
 * Game initialisation routines.
 *
 */
    #[no_mangle]
    static mut UserMusicPath: [libc::c_char; 0];
    #[no_mangle]
    fn statsInitVars();
    #[no_mangle]
    fn structureInitVars();
    #[no_mangle]
    fn messageInitVars() -> BOOL;
    #[no_mangle]
    fn researchInitVars() -> BOOL;
    #[no_mangle]
    fn featureInitVars();
    #[no_mangle]
    fn radarInitVars();
    #[no_mangle]
    static mut global_mods: [*mut libc::c_char; 0];
    #[no_mangle]
    static mut campaign_mods: [*mut libc::c_char; 0];
    #[no_mangle]
    static mut multiplay_mods: [*mut libc::c_char; 0];
    #[no_mangle]
    static mut FEFont: libc::c_int;
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
pub type WORD = libc::c_short;
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
/* **************************************************************************/
/* structs */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AUDIO_SAMPLE {
    pub iTrack: SDWORD,
    pub iSample: ALuint,
    pub x: SDWORD,
    pub y: SDWORD,
    pub z: SDWORD,
    pub iLoops: SDWORD,
    pub bRemove: BOOL,
    pub pCallback: AUDIO_CALLBACK,
    pub psObj: *mut libc::c_void,
    pub psPrev: *mut AUDIO_SAMPLE,
    pub psNext: *mut AUDIO_SAMPLE,
}
pub type AUDIO_CALLBACK
    =
    Option<unsafe extern "C" fn(_: *mut AUDIO_SAMPLE) -> BOOL>;
pub type ALuint = libc::c_uint;
pub type W_HEAPINIT = _w_heapinit;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_heapinit {
    pub barInit: UDWORD,
    pub barExt: UDWORD,
    pub butInit: UDWORD,
    pub butExt: UDWORD,
    pub edbInit: UDWORD,
    pub edbExt: UDWORD,
    pub formInit: UDWORD,
    pub formExt: UDWORD,
    pub cFormInit: UDWORD,
    pub cFormExt: UDWORD,
    pub tFormInit: UDWORD,
    pub tFormExt: UDWORD,
    pub labInit: UDWORD,
    pub labExt: UDWORD,
    pub sldInit: UDWORD,
    pub sldExt: UDWORD,
}
pub const ID_SOUND_WINDOWCLOSE: C2RustUnnamed_3 = 0;
pub const IMAGE_CURSOR_DEFAULT: C2RustUnnamed_2 = 232;
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
pub struct iSprite {
    pub width: libc::c_uint,
    pub height: libc::c_uint,
    pub bmp: *mut iBitmap,
}
pub type iBitmap = uint8;
pub type uint8 = libc::c_uchar;
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
pub const IMAGE_TFONT_63: C2RustUnnamed_1 = 279;
pub const IMAGE_TFONT_253: C2RustUnnamed_1 = 232;
pub const IMAGE_TFONT_252: C2RustUnnamed_1 = 231;
pub const IMAGE_TFONT_251: C2RustUnnamed_1 = 230;
pub const IMAGE_TFONT_250: C2RustUnnamed_1 = 229;
pub const IMAGE_TFONT_249: C2RustUnnamed_1 = 228;
pub const IMAGE_TFONT_248: C2RustUnnamed_1 = 227;
pub const IMAGE_TFONT_246: C2RustUnnamed_1 = 226;
pub const IMAGE_TFONT_245: C2RustUnnamed_1 = 265;
pub const IMAGE_TFONT_244: C2RustUnnamed_1 = 225;
pub const IMAGE_TFONT_243: C2RustUnnamed_1 = 224;
pub const IMAGE_TFONT_242: C2RustUnnamed_1 = 223;
pub const IMAGE_TFONT_241: C2RustUnnamed_1 = 221;
pub const IMAGE_TFONT_240: C2RustUnnamed_1 = 254;
pub const IMAGE_TFONT_239: C2RustUnnamed_1 = 220;
pub const IMAGE_TFONT_238: C2RustUnnamed_1 = 219;
pub const IMAGE_TFONT_237: C2RustUnnamed_1 = 218;
pub const IMAGE_TFONT_236: C2RustUnnamed_1 = 217;
pub const IMAGE_TFONT_235: C2RustUnnamed_1 = 216;
pub const IMAGE_TFONT_234: C2RustUnnamed_1 = 215;
pub const IMAGE_TFONT_233: C2RustUnnamed_1 = 214;
pub const IMAGE_TFONT_232: C2RustUnnamed_1 = 213;
pub const IMAGE_TFONT_230: C2RustUnnamed_1 = 211;
pub const IMAGE_TFONT_229: C2RustUnnamed_1 = 210;
pub const IMAGE_TFONT_228: C2RustUnnamed_1 = 209;
pub const IMAGE_TFONT_227: C2RustUnnamed_1 = 208;
pub const IMAGE_TFONT_226: C2RustUnnamed_1 = 207;
pub const IMAGE_TFONT_225: C2RustUnnamed_1 = 204;
pub const IMAGE_TFONT_224: C2RustUnnamed_1 = 205;
pub const IMAGE_TFONT_221: C2RustUnnamed_1 = 264;
pub const IMAGE_TFONT_220: C2RustUnnamed_1 = 239;
pub const IMAGE_TFONT_219: C2RustUnnamed_1 = 262;
pub const IMAGE_TFONT_218: C2RustUnnamed_1 = 261;
pub const IMAGE_TFONT_217: C2RustUnnamed_1 = 263;
pub const IMAGE_TFONT_216: C2RustUnnamed_1 = 240;
pub const IMAGE_TFONT_215: C2RustUnnamed_1 = 241;
pub const IMAGE_TFONT_214: C2RustUnnamed_1 = 238;
pub const IMAGE_TFONT_213: C2RustUnnamed_1 = 260;
pub const IMAGE_TFONT_212: C2RustUnnamed_1 = 258;
pub const IMAGE_TFONT_211: C2RustUnnamed_1 = 274;
pub const IMAGE_TFONT_210: C2RustUnnamed_1 = 259;
pub const IMAGE_TFONT_209: C2RustUnnamed_1 = 222;
pub const IMAGE_TFONT_208: C2RustUnnamed_1 = 255;
pub const IMAGE_TFONT_207: C2RustUnnamed_1 = 276;
pub const IMAGE_TFONT_206: C2RustUnnamed_1 = 275;
pub const IMAGE_TFONT_205: C2RustUnnamed_1 = 278;
pub const IMAGE_TFONT_204: C2RustUnnamed_1 = 277;
pub const IMAGE_TFONT_203: C2RustUnnamed_1 = 256;
pub const IMAGE_TFONT_202: C2RustUnnamed_1 = 273;
pub const IMAGE_TFONT_201: C2RustUnnamed_1 = 237;
pub const IMAGE_TFONT_200: C2RustUnnamed_1 = 257;
pub const IMAGE_TFONT_199: C2RustUnnamed_1 = 212;
pub const IMAGE_TFONT_198: C2RustUnnamed_1 = 234;
pub const IMAGE_TFONT_197: C2RustUnnamed_1 = 236;
pub const IMAGE_TFONT_196: C2RustUnnamed_1 = 235;
pub const IMAGE_TFONT_195: C2RustUnnamed_1 = 253;
pub const IMAGE_TFONT_194: C2RustUnnamed_1 = 251;
pub const IMAGE_TFONT_193: C2RustUnnamed_1 = 250;
pub const IMAGE_TFONT_192: C2RustUnnamed_1 = 252;
pub const IMAGE_TFONT_45: C2RustUnnamed_1 = 280;
pub const IMAGE_TFONT_126: C2RustUnnamed_1 = 92;
pub const IMAGE_TFONT_125: C2RustUnnamed_1 = 91;
pub const IMAGE_TFONT_124: C2RustUnnamed_1 = 90;
pub const IMAGE_TFONT_123: C2RustUnnamed_1 = 89;
pub const IMAGE_TFONT_122: C2RustUnnamed_1 = 88;
pub const IMAGE_TFONT_121: C2RustUnnamed_1 = 87;
pub const IMAGE_TFONT_120: C2RustUnnamed_1 = 86;
pub const IMAGE_TFONT_119: C2RustUnnamed_1 = 85;
pub const IMAGE_TFONT_118: C2RustUnnamed_1 = 84;
pub const IMAGE_TFONT_117: C2RustUnnamed_1 = 83;
pub const IMAGE_TFONT_116: C2RustUnnamed_1 = 82;
pub const IMAGE_TFONT_115: C2RustUnnamed_1 = 81;
pub const IMAGE_TFONT_114: C2RustUnnamed_1 = 80;
pub const IMAGE_TFONT_113: C2RustUnnamed_1 = 79;
pub const IMAGE_TFONT_112: C2RustUnnamed_1 = 78;
pub const IMAGE_TFONT_111: C2RustUnnamed_1 = 77;
pub const IMAGE_TFONT_110: C2RustUnnamed_1 = 76;
pub const IMAGE_TFONT_109: C2RustUnnamed_1 = 75;
pub const IMAGE_TFONT_108: C2RustUnnamed_1 = 74;
pub const IMAGE_TFONT_107: C2RustUnnamed_1 = 73;
pub const IMAGE_TFONT_106: C2RustUnnamed_1 = 72;
pub const IMAGE_TFONT_105: C2RustUnnamed_1 = 71;
pub const IMAGE_TFONT_104: C2RustUnnamed_1 = 70;
pub const IMAGE_TFONT_103: C2RustUnnamed_1 = 69;
pub const IMAGE_TFONT_102: C2RustUnnamed_1 = 68;
pub const IMAGE_TFONT_101: C2RustUnnamed_1 = 67;
pub const IMAGE_TFONT_100: C2RustUnnamed_1 = 66;
pub const IMAGE_TFONT_99: C2RustUnnamed_1 = 65;
pub const IMAGE_TFONT_98: C2RustUnnamed_1 = 64;
pub const IMAGE_TFONT_97: C2RustUnnamed_1 = 63;
pub const IMAGE_TFONT_96: C2RustUnnamed_1 = 62;
pub const IMAGE_TFONT_95: C2RustUnnamed_1 = 61;
pub const IMAGE_TFONT_94: C2RustUnnamed_1 = 60;
pub const IMAGE_TFONT_93: C2RustUnnamed_1 = 59;
pub const IMAGE_TFONT_92: C2RustUnnamed_1 = 58;
pub const IMAGE_TFONT_91: C2RustUnnamed_1 = 57;
pub const IMAGE_TFONT_90: C2RustUnnamed_1 = 56;
pub const IMAGE_TFONT_89: C2RustUnnamed_1 = 55;
pub const IMAGE_TFONT_88: C2RustUnnamed_1 = 54;
pub const IMAGE_TFONT_87: C2RustUnnamed_1 = 53;
pub const IMAGE_TFONT_86: C2RustUnnamed_1 = 52;
pub const IMAGE_TFONT_85: C2RustUnnamed_1 = 51;
pub const IMAGE_TFONT_84: C2RustUnnamed_1 = 50;
pub const IMAGE_TFONT_83: C2RustUnnamed_1 = 49;
pub const IMAGE_TFONT_82: C2RustUnnamed_1 = 48;
pub const IMAGE_TFONT_81: C2RustUnnamed_1 = 47;
pub const IMAGE_TFONT_80: C2RustUnnamed_1 = 46;
pub const IMAGE_TFONT_79: C2RustUnnamed_1 = 45;
pub const IMAGE_TFONT_78: C2RustUnnamed_1 = 44;
pub const IMAGE_TFONT_77: C2RustUnnamed_1 = 43;
pub const IMAGE_TFONT_76: C2RustUnnamed_1 = 42;
pub const IMAGE_TFONT_75: C2RustUnnamed_1 = 41;
pub const IMAGE_TFONT_74: C2RustUnnamed_1 = 40;
pub const IMAGE_TFONT_73: C2RustUnnamed_1 = 39;
pub const IMAGE_TFONT_72: C2RustUnnamed_1 = 38;
pub const IMAGE_TFONT_71: C2RustUnnamed_1 = 37;
pub const IMAGE_TFONT_70: C2RustUnnamed_1 = 36;
pub const IMAGE_TFONT_69: C2RustUnnamed_1 = 35;
pub const IMAGE_TFONT_68: C2RustUnnamed_1 = 34;
pub const IMAGE_TFONT_67: C2RustUnnamed_1 = 33;
pub const IMAGE_TFONT_66: C2RustUnnamed_1 = 32;
pub const IMAGE_TFONT_65: C2RustUnnamed_1 = 31;
pub const IMAGE_TFONT_64: C2RustUnnamed_1 = 30;
pub const IMAGE_TFONT_62: C2RustUnnamed_1 = 29;
pub const IMAGE_TFONT_61: C2RustUnnamed_1 = 28;
pub const IMAGE_TFONT_60: C2RustUnnamed_1 = 27;
pub const IMAGE_TFONT_59: C2RustUnnamed_1 = 26;
pub const IMAGE_TFONT_58: C2RustUnnamed_1 = 25;
pub const IMAGE_TFONT_57: C2RustUnnamed_1 = 24;
pub const IMAGE_TFONT_56: C2RustUnnamed_1 = 23;
pub const IMAGE_TFONT_55: C2RustUnnamed_1 = 22;
pub const IMAGE_TFONT_54: C2RustUnnamed_1 = 21;
pub const IMAGE_TFONT_53: C2RustUnnamed_1 = 20;
pub const IMAGE_TFONT_52: C2RustUnnamed_1 = 19;
pub const IMAGE_TFONT_51: C2RustUnnamed_1 = 18;
pub const IMAGE_TFONT_50: C2RustUnnamed_1 = 17;
pub const IMAGE_TFONT_49: C2RustUnnamed_1 = 16;
pub const IMAGE_TFONT_48: C2RustUnnamed_1 = 15;
pub const IMAGE_TFONT_47: C2RustUnnamed_1 = 14;
pub const IMAGE_TFONT_46: C2RustUnnamed_1 = 13;
pub const IMAGE_TFONT_44: C2RustUnnamed_1 = 12;
pub const IMAGE_TFONT_43: C2RustUnnamed_1 = 11;
pub const IMAGE_TFONT_42: C2RustUnnamed_1 = 10;
pub const IMAGE_TFONT_41: C2RustUnnamed_1 = 9;
pub const IMAGE_TFONT_40: C2RustUnnamed_1 = 8;
pub const IMAGE_TFONT_39: C2RustUnnamed_1 = 7;
pub const IMAGE_TFONT_38: C2RustUnnamed_1 = 6;
pub const IMAGE_TFONT_37: C2RustUnnamed_1 = 5;
pub const IMAGE_TFONT_36: C2RustUnnamed_1 = 4;
pub const IMAGE_TFONT_35: C2RustUnnamed_1 = 3;
pub const IMAGE_TFONT_34: C2RustUnnamed_1 = 2;
pub const IMAGE_TFONT_33: C2RustUnnamed_1 = 1;
pub type BASE_OBJECT = _base_object;
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
pub type SCREEN_DISP_DATA = _screen_disp_data;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _screen_disp_data {
    pub imd: *mut iIMDShape,
    pub frameNumber: UDWORD,
    pub screenX: UDWORD,
    pub screenY: UDWORD,
    pub screenR: UDWORD,
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
pub type uint16 = libc::c_ushort;
// only needed when generating the tree
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
pub struct iVector {
    pub x: int32,
    pub y: int32,
    pub z: int32,
}
pub type int32 = libc::c_int;
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
pub type VERTEXID = libc::c_int;
pub type uint32 = libc::c_uint;
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
pub type OBJECT_TYPE = _object_type;
pub type _object_type = libc::c_uint;
pub const OBJ_TARGET: _object_type = 4;
pub const OBJ_BULLET: _object_type = 3;
pub const OBJ_FEATURE: _object_type = 2;
pub const OBJ_STRUCTURE: _object_type = 1;
pub const OBJ_DROID: _object_type = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VECTOR3D {
    pub x: SDWORD,
    pub y: SDWORD,
    pub z: SDWORD,
}
pub type ANIMOBJDONEFUNC
    =
    Option<unsafe extern "C" fn(_: *mut ANIM_OBJECT) -> ()>;
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
pub struct ANIM_STATE {
    pub uwFrame: UWORD,
    pub vecPos: VECTOR3D,
    pub vecAngle: VECTOR3D,
    pub vecScale: VECTOR3D,
}
// Droids
// All Buildings
// Things like roads, trees, bridges, fires
// Comes out of guns, stupid :-)
// for the camera tracking
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
/* the subject the structure is working on*/
/* Number of upgrade modules added*/
/* The time the building started on the subject*/
/* Research Points produced per research cycle*/
/* Time taken to research the topic*/
/* The topic with the most research points 
										   that was last performed*/
/* used to keep track of power before 
										   researching a topic*/
/* The time the research facility was put on hold*/
/* The max size of body the factory 
											   can produce*/
/* The number of droids to produce OR for 
											   selectedPlayer, how many loops to perform*/
/* how many times the loop has been performed*/
//struct _propulsion_types*	propulsionType;		
	//UBYTE				propulsionType;		/* The type of propulsion the facility 
	//										   can produce*/
/* Droid Build Points Produced Per 
											   Build Cycle*/
/* used to keep track of power before building a droid*/
/* the subject the structure is working on */
/* The time the building started on the subject*/
/* Time taken to build one droid */
/* The time the factory was put on hold*/
/* Place for the new droids to assemble at */
// formation for the droids that are produced
// command droid to produce droids for (if any)
// secondary order state for all units coming out of the factory
                                            // added AB 22/04/99
//these are no longer required - yipee!
	// The group the droids produced by this factory belong to - used for Missions
	//struct _droid_group		*psGroup;
	//struct _droid			*psGrpNext;
/*The max amount of power that can be extracted*/
/*time the Res Extr last got points*/
/*indicates when the extractor is on ie digging up oil*/
/*owning power generator*/
/*The max power that can be used - NOT USED 21/04/98*/
/*Factor to multiply output by - percentage*/
/* Number of upgrade modules added*/
//struct _structure	*apResExtractors[NUM_POWER_MODULES + 1];/*pointers to the res ext
/*pointers to the res ext
																associated with this gen*/
/* Power used in repairing */
/* Time repair started on current object */
/* Object being repaired */
/* used to keep track of power before 
											   repairing a droid */
/* Place for the repaired droids to assemble
                                               at */
/* stores the amount of body points added to the unit
                                               that is being worked on */
// The group the droids to be repaired by this facility belong to
/* rearm points per cycle				 */
/* Time reArm started on current object	 */
/* Object being rearmed		             */
/* stores the amount of body points added to the unit
                                               that is being worked on */
//this structure is used whenever an instance of a building is required in game
pub type STRUCTURE = _structure;
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
/* The common structure elements for all objects */
//	UDWORD		ref;
/* pointer to the structure stats for this 
											   type of building */
/* defines whether the structure is being 
											   built, doing nothing or performing a function*/
//SDWORD		currentBuildPts;			/* the build points currently assigned to this 
/* the build points currently assigned to this 
											   structure */
/* the power accrued for building this structure*/
/* current body points */
//UDWORD		body;						/* current body points */
	//UDWORD		baseBodyPoints;				/* undamaged body points */
/* current armour points */
//UDWORD		armour;						/* current armour points */
	//SDWORD		resistance;					/* current resistance points 
/* current resistance points 
											   0 = cannot be attacked electrically*/
/* time the resistance was last increased*/
//UDWORD		repair;						/* current repair points */
// repair doesn't seem to be used anywhere ... I'll take it out for the mo. and remove it from the new savegame stuff
// talk to me if you are having problems with this
//	UWORD		repair;						/* current repair points */
/* The other structure data.  These are all derived from the functions
	 * but stored here for easy access - will need to add more for variable stuff!
	 */
	//the sensor stats need to be stored since the actual sensor stat can change with research
/*UDWORD		turretRotation;				// weapon, ECM and sensor direction and pitch
	UDWORD		turretRotRate;				// weapon, ECM and sensor direction and pitch
	UDWORD		turretPitch;				// weapon, ECM and sensor direction and pitch*/
// weapon, ECM and sensor direction and pitch
//UWORD		turretRotRate;				// weapon, ECM and sensor direction and pitch - THIS IS A CONSTANT
// weapon, ECM and sensor direction and pitch
//the time the structure was last attacked
//the ecm power needs to be stored since the actual ecm stat can change with research
//FRACT		heightScale;	
/* pointer to structure that contains fields
											   necessary for functionality */
/* The weapons on the structure */
	//UWORD		numWeaps;
/* anim data */
/*
 * Weapons.h
 *
 * Definitions for the weapons
 *
 */
pub type WEAPON = _weapon;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _weapon {
    pub nStat: UDWORD,
    pub hitPoints: UDWORD,
    pub ammo: UDWORD,
    pub lastFired: UDWORD,
    pub recoilValue: UDWORD,
}
pub type FUNCTIONALITY = [UBYTE; 40];
pub type STRUCTURE_STATS = _structure_stats;
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
// The stats for the weapon type
// When the weapon last fired
/*
 * FunctionDef.h
 *
 * Structure defs for functions.
 *
 */
//DEFENSIVE_STRUCTURE_TYPE,
	//RADAR_MAP_TYPE,
	//POWER_REG_TYPE,
	//POWER_RELAY_TYPE,
	//ARMOUR_UPGRADE_TYPE,
	//REPAIR_UPGRADE_TYPE,
	//RESISTANCE_UPGRADE_TYPE,
	//DROID_DESIGN_TYPE,
	//MAP_MARKER_TYPE,
	//SKY_DOME_MAP_TYPE,
	//BODY_UPGRADE_TYPE,
	//HQ_TYPE,
/* The number of function types */
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
/*
 * StatsDef.h
 *
 * Structure definitions for the stats system
 *
 */
/* Elements common to all stats structures */
/* Unique ID of the item */
/* pointer to the text id name (i.e. short language-independant name) */
/* Stats common to all stats structs */
/*
if any types are added BEFORE 'COMP_BODY' - then Save/Load Game will have to be 
altered since it loops through the components from 1->MAX_COMP
*/
//COMP_ARMOUR,
//COMP_POWERPLANT,
//COMP_PROGRAM,		//this needs to be removed when save games changes
//ALL components and structures and research topics have a tech level to which they belong
/* **********************************************************************************
 *
 * Stats structures type definitions
 */
/* Stats common to all droid components */
/* Basic stats */
/* technology level of the component */
/* Power required to build the component */
/* Time required to build the component */
/* Component's weight */
/* Component's hit points */
/* Space the component takes in the droid */
/* Component's body points */
/* flag to indicate whether this component can*/
/* be used in the design screen*/
/* The IMD to draw for this component */
/* Stats common to all components */
/* LOC used for holding locations for Sensors and ECM's*/
/* SIZE used for specifying body size */
//only using KINETIC and HEAT for now
//bullets etc
//WC_EXPLOSIVE,	//rockets etc - classed as WC_KINETIC now to save space in struct _droid AB 25/11/98 
//laser etc
//WC_MISC,		//others we haven't thought of! - classed as WC_HEAT now to save space in struct _droid AB 25/11/98 
// weapon subclasses used to define which weapons are affected by weapon upgrade functions
//WSC_ARTILLARY,
//WSC_CLOSECOMBAT,
// used to define which projectile model to use for the weapon
//used to modify the damage to a propuslion type (or structure) based on weapon
pub type WEAPON_EFFECT = _weapon_effect;
pub type _weapon_effect = libc::c_uint;
pub const WE_NUMEFFECTS: _weapon_effect = 6;
pub const WE_ANTI_AIRCRAFT: _weapon_effect = 5;
pub const WE_FLAMER: _weapon_effect = 4;
pub const WE_ARTILLERY_ROUND: _weapon_effect = 3;
pub const WE_BUNKER_BUSTER: _weapon_effect = 2;
pub const WE_ANTI_TANK: _weapon_effect = 1;
pub const WE_ANTI_PERSONNEL: _weapon_effect = 0;
pub type MOVEMENT_MODEL = _movement_model;
pub type _movement_model = libc::c_uint;
pub const NUM_MOVEMENT_MODEL: _movement_model = 6;
pub const MM_SWEEP: _movement_model = 5;
pub const MM_ERRATICDIRECT: _movement_model = 4;
pub const MM_HOMINGINDIRECT: _movement_model = 3;
pub const MM_HOMINGDIRECT: _movement_model = 2;
pub const MM_INDIRECT: _movement_model = 1;
pub const MM_DIRECT: _movement_model = 0;
pub type WEAPON_SUBCLASS = _weapon_subclass;
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
pub type WEAPON_CLASS = _weapon_class;
pub type _weapon_class = libc::c_uint;
pub const NUM_WEAPON_CLASS: _weapon_class = 2;
pub const WC_HEAT: _weapon_class = 1;
pub const WC_KINETIC: _weapon_class = 0;
/* common stats - N.B. system points for a body
	 * are the limit for all other components
	 */
//UDWORD		configuration;		// Body shape
	//UDWORD		armourMult;			// How configuration affects body shape
									// Actually could be a fractional value
									// store x 10
// How big the body is - affects how hit
//UDWORD		bodyPoints;			// How much damage the body can take before destruction
// The number of weapon slots on the body
// A measure of how much protection the armour provides
// cross-ref with the weapon types
// A measure of how much energy the power plant outputs
//list of IMDs to use for propulsion unit - up to numPropulsionStats
//pointer to which flame graphic to use - for VTOLs only at the moment
/* Common stats */
// Program capacity
//UDWORD		AICap;				// AI capacity
	//UDWORD		AISpeed;			// AI Learning Speed
//weapon stats associated with this brain - for Command Droids
//defines the left and right sides for propulsion IMDs
/* Common stats */
// Max speed for the droid
// Type of propulsion used - index 
// into PropulsionTable
//works as all of the above together! - new for updates - added 11/06/99 AB
/* Common stats */
// Sensor range
// Sensor power (put against ecm power)
// specifies whether the Sensor is default or for the Turret
// used for combat
//time delay before associated weapon droids 'know' where the
//attack is from
// The turret mount to use
/* Common stats */
// ECM range
// ECM power (put against sensor power)
// specifies whether the ECM is default or for the Turret
// The turret mount to use
/* Adjustment to armour damage for non-penetrating hit */
/* Adjustment to armour damage for penetrating hit */
/* Common stats */
// How much damage is restored to Body Points 
// and armour each Repair Cycle
// whether armour can be repaired or not
// specifies whether the Repair is default or for the Turret
// time delay for repair cycle
// The turret mount to use
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
pub type FIREONMOVE = _fireonmove;
pub type _fireonmove = libc::c_uint;
pub const FOM_YES: _fireonmove = 2;
pub const FOM_PARTIAL: _fireonmove = 1;
pub const FOM_NO: _fireonmove = 0;
pub type TECH_LEVEL = _tech_level;
pub type _tech_level = libc::c_uint;
pub const MAX_TECH_LEVELS: _tech_level = 6;
pub const TECH_LEVEL_ALL: _tech_level = 5;
pub const TECH_LEVEL_TWO_THREE: _tech_level = 4;
pub const TECH_LEVEL_ONE_TWO: _tech_level = 3;
pub const TECH_LEVEL_THREE: _tech_level = 2;
pub const TECH_LEVEL_TWO: _tech_level = 1;
pub const TECH_LEVEL_ONE: _tech_level = 0;
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
pub type SENSOR_TYPE = _sensor_type;
pub type _sensor_type = libc::c_uint;
pub const SUPER_SENSOR: _sensor_type = 4;
pub const VTOL_INTERCEPT_SENSOR: _sensor_type = 3;
pub const VTOL_CB_SENSOR: _sensor_type = 2;
pub const INDIRECT_CB_SENSOR: _sensor_type = 1;
pub const STANDARD_SENSOR: _sensor_type = 0;
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
pub type STRUCT_STRENGTH = _struct_strength;
pub type _struct_strength = libc::c_uint;
pub const NUM_STRUCT_STRENGTH: _struct_strength = 4;
pub const STRENGTH_BUNKER: _struct_strength = 3;
pub const STRENGTH_HARD: _struct_strength = 2;
pub const STRENGTH_MEDIUM: _struct_strength = 1;
pub const STRENGTH_SOFT: _struct_strength = 0;
//no capability - droid must stop
//partial capability - droid has 50% chance to hit
//full capability - droid fires normally on move
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
// Weapon droid
// Sensor droid
// ECM droid
// Constructor droid
// person
// cyborg-type thang
// guess what this is!
// Command droid
// Repair droid
// Default droid
//cyborg constructor droid - new for update 28/5/99
//cyborg repair droid - new for update 28/5/99
//cyborg repair droid - new for update 7/6/99
// Any droid, Only used as a parameter for intGotoNextDroidType(droidtype).
//	UDWORD					nStat;
// Allowing a maximum of 255 stats per file
//UDWORD					hitPoints; NOT USED?
/* Possibly a set of function pointers here to be called when
	 * damage is done to a component, power is low ...
	 */
// maximum number of queued orders
//struct _base_object	*psObj;
//this needs to cope with objects and stats
//line build requires two sets of coords
// maximum number of characters in a droid name
// FOR DROID TEMPLATES
	// On the PC the pName entry in STATS_BASE is redundant and can be assumed to be NULL,
	// on the PSX the NameHash entry is used. If it is database generated template, the hashed version of the short name of the template is stored. If it is a user generated template NULL is stored.
/* basic stats */
// on the PC this contains the full editable ascii name of the template
	// on the PSX this is not used, the full name is NON-EDITABLE and is generated from the template components e.g. Viper Mk I
// Version number used in name (e.g. Viper Mk "I" would be stored as 1 - Viper Mk "X" as 10)  - copied to droid structure
/* The droid components.  This array is indexed by COMPONENT_TYPE
	 * so the ECM would be accessed using asParts[COMP_ECM].
	 * COMP_BRAIN is an index into the asCommandDroids array NOT asBrainStats
	 */
/*total build points required to manufacture
												  the droid*/
/*total power points required to build/maintain
												  the droid */
/*used to load in weaps and progs*/
/* The weapon systems */
/* Number of weapons*/
/* weapon indices */
/* The programs */
	//UDWORD			numProgs;					/* Number of programs*/
	//UDWORD			asProgs[DROID_MAXPROGS];	/* program indices*/
// The type of droid
//#ifndef PSX
// multiplayer unique descriptor(cant use id's for templates)
// used for save games as well now - AB 29/10/98
//#endif
/* Pointer to next template*/
pub type DROID = _droid;
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
/* The common structure elements for all objects */
//Ascii name of the droid - This is generated from the droid template and can not be changed by the game player after creation.
//	UBYTE 		NameVersion;			// Version number used for generating on-the-fly names (e.g. Viper Mk "I" would be stored as 1 - Viper Mk "X" as 10)  - copied from droid template
// The type of droid
/* Possibly a set of function pointers here to be called when
	 * damage is done to a component, power is low ...
	 */
//	DROID_TEMPLATE	*pTemplate;		//defines the droids components
/* Holds the specifics for the component parts - allows damage
	 * per part to be calculated. Indexed by COMPONENT_TYPE.
	 * Weapons and Programs need to be dealt with separately.
	 * COMP_BRAIN is an index into the asCommandDroids array NOT asBrainStats
	 */
/* The other droid data.  These are all derived from the components
	 * but stored here for easy access
	 */
//the base speed dependant on propulsion type
//the original body points
// the current body points
//UDWORD		power;
//tjc	UDWORD		imdNum;
//UWORD		turretRotRate; THIS IS A CONSTANT
//*
// Version number used for generating on-the-fly names (e.g. Viper Mk "I" would be stored as 1 - Viper Mk "X" as 10)  - copied from droid template
//	UDWORD		numKills;
//used in Electronic Warfare
//	SDWORD		activeWeapon;		// The currently selected weapon
	//UDWORD		numWeaps;
//SDWORD		activeProg;			// The currently running program
	//UDWORD		numProgs;
	//PROGRAM		asProgs[DROID_MAXPROGS];
// The group the droid belongs to
//a structure that this droid might be associated with
                                                    //for vtols its the rearming pad
// queued orders
/* Order data */
// 	struct _base_object	*psLastAttacker;
// secondary order data
// multiplayer synchronisation value.
/* Action data */
// Action target object
// Game time action started
// number of points done by action since start
//UWORD				actionHeight;		// height to level the ground to for foundation,
											// possibly use it for other data as well? Yup! - powerAccrued!
// renamed the above variable since this is what its used for now!
//UBYTE				tileNumber;			// tile number for foundation NOT USED ANYMORE
/* Movement control data */
//	void				*lastTile;
	/* AI data */
//	AI_DATA				sAI;
	/* anim data */
/*
 * MoveDef.h
 *
 * Structure definitions for movement structures.
 *
 */
//	SDWORD		XCoordinate,YCoordinate;
pub type MOVE_CONTROL = _move_control;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _formation {
    pub refCount: SWORD,
    pub size: SWORD,
    pub rankDist: SWORD,
    pub dir: SWORD,
    pub x: SDWORD,
    pub y: SDWORD,
    pub asLines: [F_LINE; 4],
    pub numLines: SWORD,
    pub maxRank: UBYTE,
    pub free: SBYTE,
    pub asMembers: [F_MEMBER; 20],
    pub iSpeed: UDWORD,
    pub psNext: *mut _formation,
}
// Inactive, Navigating or moving point to point status
// Mask used for the creation of this path	
//	SBYTE	Direction;					// Direction object should be moving (0-7) 0=Up,1=Up-Right
//	SDWORD	Speed;						// Speed at which object moves along the movement list
// Position in asPath
// number of points in asPath
//	PATH_POINT	MovementList[TRAVELSIZE];
// Pointer to list of block X,Y coordinates.
// When initialised list is terminated by (0xffff,0xffff)
										// Values prefixed by 0x8000 are pixel coordinates instead of
										// block coordinates
// DestinationX,Y should match objects current X,Y
//		location for this movement to be complete.
//   	UDWORD	Direction3D;				// *** not necessary
//	UDWORD	TargetDir;					// *** not necessary Direction the object should be facing
//	SDWORD	Gradient;					// Gradient of line
//	SDWORD	DeltaX;						// Distance from start to end position of current movement X
//	SDWORD	DeltaY;						// Distance from start to end position of current movement Y
//	SDWORD	XStep;						// Adjustment to the characters X position each movement
//	SDWORD	YStep;						// Adjustment to the characters Y position each movement
//	SDWORD	DestPixelX;					// Pixel coordinate destination for pixel movement (NOT the final X)
//	SDWORD	DestPixelY;					// Pixel coordiante destination for pixel movement (NOT the final Y)
/* Stuff for John's movement update */
// droid location as a fract
//	FRACT	dx,dy;						// x and y change for current direction
	// NOTE: this is supposed to replace Speed
// Speed of motion
// Vector for the end of path boundary
// direction of motion (not the direction the droid is facing)
// direction at last bump
// time of first bump with something
// time of last bump with a droid - relative to bumpTime
// when MOVEPAUSE started - relative to bumpTime
// position of last bump
// when a shuffle started
// formation the droid is currently a member of
/* vtol movement - GJ */
// added for vtol movement
/* Only needed for Alex's movement update ? */
//	UDWORD	timeStarted;
//	UDWORD	arrivalTime;
//	UDWORD	pathStarted;
//	BOOL	startedMoving;
//	UDWORD	lastTime;
//	BOOL	speedChange;
// information about a formation member
pub type F_MEMBER = _f_member;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _f_member {
    pub line: SBYTE,
    pub next: SBYTE,
    pub dist: SWORD,
    pub psObj: *mut BASE_OBJECT,
}
// which line this member is on
// the next member on this line
// distance along the line
// the member unit
/*
 * FormationDef.h
 *
 */
// maximum number of lines in a formation
// maximum number of unit members of a formation (cannot be more that 128)
// information about a formation line
// a linked list of the formation members on this line is maintained
// using their index in the asMembers array.  (-1 == 'NULL')
// (cuts down the memory use over proper pointers)
pub type F_LINE = _f_line;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _f_line {
    pub xoffset: SWORD,
    pub yoffset: SWORD,
    pub dir: SWORD,
    pub member: SBYTE,
}
pub type PATH_POINT = _path_point;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _path_point {
    pub x: UBYTE,
    pub y: UBYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _base_stats {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
}
pub type ORDER_LIST = _order_list;
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
// position relative to center
// orientation of line
// first member in the 'linked list' of members
/*
 * Group.h
 *
 * Link droids together into a group for AI etc.
 *
 */
// standard group
// command droid group
// transporter group
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _droid_group {
    pub type_0: SWORD,
    pub refCount: SWORD,
    pub psList: *mut DROID,
    pub psCommander: *mut DROID,
    pub sRunData: RUN_DATA,
}
pub type RUN_DATA = _run_data;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _run_data {
    pub sPos: POINT,
    pub forceLevel: UBYTE,
    pub healthLevel: UBYTE,
    pub leadership: UBYTE,
}
pub type COMPONENT = _component;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _component {
    pub nStat: UBYTE,
}
pub type DROID_TYPE = _droid_type;
pub type _droid_type = libc::c_uint;
pub const DROID_ANY: _droid_type = 13;
pub const DROID_CYBORG_SUPER: _droid_type = 12;
pub const DROID_CYBORG_REPAIR: _droid_type = 11;
pub const DROID_CYBORG_CONSTRUCT: _droid_type = 10;
pub const DROID_DEFAULT: _droid_type = 9;
pub const DROID_REPAIR: _droid_type = 8;
pub const DROID_COMMAND: _droid_type = 7;
pub const DROID_TRANSPORTER: _droid_type = 6;
pub const DROID_CYBORG: _droid_type = 5;
pub const DROID_PERSON: _droid_type = 4;
pub const DROID_CONSTRUCT: _droid_type = 3;
pub const DROID_ECM: _droid_type = 2;
pub const DROID_SENSOR: _droid_type = 1;
pub const DROID_WEAPON: _droid_type = 0;
pub type ANIMOBJDIEDTESTFUNC
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> BOOL>;
/* **************************************************************************/
pub type GETSHAPEFUNC
    =
    Option<unsafe extern "C" fn(_: *mut STRING) -> *mut libc::c_void>;
pub const ID_SOUND_WINDOWOPEN: C2RustUnnamed_3 = 1;
/*
 * Interp.h
 *
 * Script interpreter definitions
 */
/* The possible value types for scripts */
// Basic types
//	VAL_FLOAT,
// events and triggers
/* Type used by the compiler for functions that do not return a value */
// user defined types should start with this id
// flag to specify a variable reference rather than simple value
/* A value consists of its type and value */
// VAL_BOOL
// VAL_INT
//		float		fval;		// VAL_FLOAT
// VAL_STRING
// VAL_OBJECT
// VAL_VOIDPTR
// maximum number of equivalent types for a type
// type equivalences
// the type that the others are equivalent to
// number of equivalent types
// the equivalent types
/* Opcodes for the script interpreter */
// Push value onto stack
// Push a pointer to a variable onto the stack
// Pop value from stack
// Push the value of a global variable onto the stack
// Pop a value from the stack into a global variable
// Push the value of a global array variable onto the stack
// Pop a value from the stack into a global array variable
// Call the 'C' function pointed to by the next value
// Call the variable access 'C' function pointed to by the next value
// Jump to a different location in the script
// Jump if the top stack value is true
// Jump if the top stack value is false
// Call a binary maths/boolean operator
// Call a unary maths/boolean operator
// End the program
// temporarily pause the current event
// The following operations are secondary data to OP_BINARYOP and OP_UNARYOP
// Maths operators
// Boolean operators
//String cancatenation
// Comparison operators
//custom (in-script) function call
//local var
//variable of object type (pointer)
/* How far the opcode is shifted up a UDWORD to allow other data to be
 * stored in the same UDWORD
 */
// maximum sizes for arrays
/* The mask for the number of array elements stored in the data part of an opcode */
/* The type of function called by an OP_CALL */
/* The type of function called to access an object or in-game variable */
/* The possible storage types for a variable */
// Public variable
// Private variable
// A value stored in an objects data space.
// An external value accessed by function call
// A local variable
/* Variable debugging info for a script */
/* Array info for a script */
// the base index of the array values
// the array data type
/* Array debug info for a script */
/* Line debugging information for a script */
// Offset in the compiled script that corresponds to
// this line in the original script.
// the trigger/event that starts at this line
/* Different types of triggers */
pub type TRIGGER_TYPE = _trigger_type;
pub type _trigger_type = libc::c_uint;
pub const TR_CALLBACKSTART: _trigger_type = 5;
pub const TR_PAUSE: _trigger_type = 4;
pub const TR_EVERY: _trigger_type = 3;
pub const TR_WAIT: _trigger_type = 2;
pub const TR_CODE: _trigger_type = 1;
pub const TR_INIT: _trigger_type = 0;
pub const CALL_GAMEINIT: _scr_callback_types = 5;
// Trigger fires when the script is first run
// Trigger uses script code
// Trigger after a time pause
// Trigger at repeated intervals
// Event has paused for an interval and will restart in the middle of it's code
// The user defined callback triggers should start with this id
// User saved game - at the start of a level.
pub const GTYPE_SAVE_MIDMISSION: C2RustUnnamed_0 = 4;
pub const REF_HQ: C2RustUnnamed = 0;
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
pub type LOOP_MISSION_STATE = libc::c_uint;
pub const LMS_CLEAROBJECTS: LOOP_MISSION_STATE = 5;
pub const LMS_LOADGAME: LOOP_MISSION_STATE = 4;
pub const LMS_NEWLEVEL: LOOP_MISSION_STATE = 3;
pub const LMS_SAVECONTINUE: LOOP_MISSION_STATE = 2;
pub const LMS_SETUPMISSION: LOOP_MISSION_STATE = 1;
pub const LMS_NORMAL: LOOP_MISSION_STATE = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _wzSearchPath {
    pub path: [libc::c_char; 260],
    pub priority: libc::c_uint,
    pub higherPriority: *mut _wzSearchPath,
    pub lowerPriority: *mut _wzSearchPath,
}
pub type wzSearchPath = _wzSearchPath;
pub type searchPathMode = libc::c_uint;
pub const mod_multiplay: searchPathMode = 2;
pub const mod_campaign: searchPathMode = 1;
pub const mod_clean: searchPathMode = 0;
pub type C2RustUnnamed = libc::c_uint;
pub const NUM_DIFF_BUILDINGS: C2RustUnnamed = 22;
pub const REF_SAT_UPLINK: C2RustUnnamed = 21;
pub const REF_MISSILE_SILO: C2RustUnnamed = 20;
pub const REF_REARM_PAD: C2RustUnnamed = 19;
pub const REF_LAB: C2RustUnnamed = 18;
pub const REF_VTOL_FACTORY: C2RustUnnamed = 17;
pub const REF_CYBORG_FACTORY: C2RustUnnamed = 16;
pub const REF_DEMOLISH: C2RustUnnamed = 15;
pub const REF_BRIDGE: C2RustUnnamed = 14;
pub const REF_COMMAND_CONTROL: C2RustUnnamed = 13;
pub const REF_REPAIR_FACILITY: C2RustUnnamed = 12;
pub const REF_RESEARCH_MODULE: C2RustUnnamed = 11;
pub const REF_RESEARCH: C2RustUnnamed = 10;
pub const REF_BLASTDOOR: C2RustUnnamed = 9;
pub const REF_WALLCORNER: C2RustUnnamed = 8;
pub const REF_WALL: C2RustUnnamed = 7;
pub const REF_DEFENSE: C2RustUnnamed = 6;
pub const REF_RESOURCE_EXTRACTOR: C2RustUnnamed = 5;
pub const REF_POWER_MODULE: C2RustUnnamed = 4;
pub const REF_POWER_GEN: C2RustUnnamed = 3;
pub const REF_FACTORY_MODULE: C2RustUnnamed = 2;
pub const REF_FACTORY: C2RustUnnamed = 1;
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
/*
 * game.h	  
 *
 * load and save game routines.
 * Very likely to ALL change! All the headers are separately defined at the 
 * moment - they probably don't need to be - if no difference make into one. 
 * Also the struct defintions throughout the game could be re-ordered to contain 
 * the variables required for saving so that don't need to create a load more here!
 */
/* **************************************************************************/
/*
 *	Global Definitions
 */
/* **************************************************************************/
//the version number used in save games
//#define VERSION_1				1				//demo save games
//#define VERSION_2				2				//names saved for components/structures/features
//#define VERSION_3				3				//changes to SAVE_GAME
//#define VERSION_4				4				//changes to SAVE_GAME
//#define VERSION_5				5				//different types of save game added
//#define VERSION_6				6				//level name added to a user save game
//string ID names saved for comps an objects
//research status saved out for user saved games
//power and experience saved out for user saved games
//includes gateways and zones in .map file.
//newstyle save game with extending structure checked in 13 Nov.
//mission and order stuff checked in 20 Nov.
//odds and ends to 24 Nov. and hashed scripts
//
//
//beta save game
//objId and new struct stats included
//droid name savegame validity stamps
//alliances, colours, radar zoom
//MAX_NAME_ SIZE replaced by MAX_SAVE_NAME_SIZE
//timeStartHold saved out for research facilities
//asRundData
//limbo droids and camstart droids saved properly, no cluster save
//reinforceTime, droid move, droid resistance
//limbo droid, research module hold fixed, cleaned by Alex
//reArm pads
//unit not the "d" word, experience and repair facility currentPtsAdded
//rearm pads currentPtsAdded saved
//mission scroll limits saved
//script external variables saved
//mission cheat time saved
//factory secondary order saved
//skirmish save 
//used in the loadGame
pub type C2RustUnnamed_0 = libc::c_uint;
// User saved game - in the middle of a level
// Stand alone mission.
pub const GTYPE_SAVE_START: C2RustUnnamed_0 = 3;
// Scenario scroll area expansion.
pub const GTYPE_MISSION: C2RustUnnamed_0 = 2;
// Initial scenario state.
pub const GTYPE_SCENARIO_EXPAND: C2RustUnnamed_0 = 1;
pub const GTYPE_SCENARIO_START: C2RustUnnamed_0 = 0;
pub type _scr_callback_types = libc::c_uint;
pub const CALL_BEACON: _scr_callback_types = 62;
pub const CALL_TRANSPORTER_LANDED_B: _scr_callback_types = 61;
pub const CALL_STRUCTBUILT: _scr_callback_types = 60;
pub const CALL_AI_MSG: _scr_callback_types = 59;
pub const CALL_CONSOLE: _scr_callback_types = 58;
pub const CALL_ALLIANCEOFFER: _scr_callback_types = 57;
pub const CALL_PLAYERLEFT: _scr_callback_types = 56;
pub const CALL_UNITTAKEOVER: _scr_callback_types = 55;
pub const CALL_VTOL_OFF_MAP: _scr_callback_types = 54;
pub const CALL_CLUSTER_EMPTY: _scr_callback_types = 53;
pub const CALL_NO_REINFORCEMENTS_LEFT: _scr_callback_types = 52;
pub const CALL_ALL_ONSCREEN_DROIDS_SELECTED: _scr_callback_types = 51;
pub const CALL_TRANSPORTER_LANDED: _scr_callback_types = 50;
pub const CALL_TRANSPORTER_OFFMAP: _scr_callback_types = 49;
pub const CALL_OBJECTCLOSE: _scr_callback_types = 48;
pub const CALL_OBJECTOPEN: _scr_callback_types = 47;
pub const CALL_FEATURE_DESTROYED: _scr_callback_types = 46;
pub const CALL_DROID_DESTROYED: _scr_callback_types = 45;
pub const CALL_STRUCT_DESTROYED: _scr_callback_types = 44;
pub const CALL_OBJ_DESTROYED: _scr_callback_types = 43;
pub const CALL_OBJ_SEEN: _scr_callback_types = 42;
pub const CALL_FEATURE_SEEN: _scr_callback_types = 41;
pub const CALL_DROID_SEEN: _scr_callback_types = 40;
pub const CALL_STRUCT_SEEN: _scr_callback_types = 39;
pub const CALL_ATTACKED: _scr_callback_types = 38;
pub const CALL_DROID_ATTACKED: _scr_callback_types = 37;
pub const CALL_STRUCT_ATTACKED: _scr_callback_types = 36;
pub const CALL_NEWDROID: _scr_callback_types = 35;
pub const CALL_RESEARCHCOMPLETED: _scr_callback_types = 34;
pub const CALL_DESIGN_PROPULSION: _scr_callback_types = 33;
pub const CALL_DESIGN_BODY: _scr_callback_types = 32;
pub const CALL_DESIGN_COMMAND: _scr_callback_types = 31;
pub const CALL_DESIGN_SYSTEM: _scr_callback_types = 30;
pub const CALL_DESIGN_WEAPON: _scr_callback_types = 29;
pub const CALL_DESIGN_QUIT: _scr_callback_types = 28;
pub const CALL_DROID_SELECTED: _scr_callback_types = 27;
pub const CALL_BUTTON_PRESSED: _scr_callback_types = 26;
pub const CALL_MANULIST: _scr_callback_types = 25;
pub const CALL_MANURUN: _scr_callback_types = 24;
pub const CALL_RESEARCHLIST: _scr_callback_types = 23;
pub const CALL_BUILDGRID: _scr_callback_types = 22;
pub const CALL_BUILDLIST: _scr_callback_types = 21;
pub const CALL_ELECTRONIC_TAKEOVER: _scr_callback_types = 20;
pub const CALL_MISSION_TIME: _scr_callback_types = 19;
pub const CALL_TRANSPORTER_REINFORCE: _scr_callback_types = 18;
pub const CALL_START_NEXT_LEVEL: _scr_callback_types = 17;
pub const CALL_LAUNCH_TRANSPORTER: _scr_callback_types = 16;
pub const CALL_VIDEO_QUIT: _scr_callback_types = 15;
pub const CALL_MISSION_END: _scr_callback_types = 14;
pub const CALL_MISSION_START: _scr_callback_types = 13;
pub const CALL_FACTORY_BUILT: _scr_callback_types = 12;
pub const CALL_RESEARCH_BUILT: _scr_callback_types = 11;
pub const CALL_RESEX_BUILT: _scr_callback_types = 10;
pub const CALL_POWERGEN_BUILT: _scr_callback_types = 9;
pub const CALL_DROIDBUILT: _scr_callback_types = 8;
pub const CALL_DROIDDESIGNED: _scr_callback_types = 7;
pub const CALL_DELIVPOINTMOVED: _scr_callback_types = 6;
/* **********************************************/
/* Image ID definition file created by Framer. */
/* **********************************************/
pub type C2RustUnnamed_1 = libc::c_uint;
pub const IMAGE_NOJOIN: C2RustUnnamed_1 = 281;
pub const IMAGE_TFONT_170: C2RustUnnamed_1 = 272;
pub const IMAGE_TFONT_163: C2RustUnnamed_1 = 271;
pub const IMAGE_TFONT_223: C2RustUnnamed_1 = 270;
pub const IMAGE_MULTIRANK3: C2RustUnnamed_1 = 269;
pub const IMAGE_KEYMAP_DEFAULT: C2RustUnnamed_1 = 268;
pub const IMAGE_NOPENCIL: C2RustUnnamed_1 = 267;
pub const IMAGE_PENCIL: C2RustUnnamed_1 = 266;
pub const IMAGE_TFONT_187: C2RustUnnamed_1 = 249;
pub const IMAGE_TFONT_171: C2RustUnnamed_1 = 248;
pub const IMAGE_TFONT_172: C2RustUnnamed_1 = 247;
pub const IMAGE_TFONT_174: C2RustUnnamed_1 = 246;
pub const IMAGE_TFONT_161: C2RustUnnamed_1 = 245;
pub const IMAGE_TFONT_191: C2RustUnnamed_1 = 244;
pub const IMAGE_TFONT_176: C2RustUnnamed_1 = 243;
pub const IMAGE_TFONT_131: C2RustUnnamed_1 = 242;
pub const IMAGE_TFONT_128: C2RustUnnamed_1 = 233;
pub const IMAGE_TFONT_188: C2RustUnnamed_1 = 206;
pub const IMAGE_TFONT_189: C2RustUnnamed_1 = 203;
pub const IMAGE_WEE_GUY: C2RustUnnamed_1 = 202;
pub const IMAGE_FOG_ON_HI: C2RustUnnamed_1 = 201;
pub const IMAGE_FOG_OFF_HI: C2RustUnnamed_1 = 200;
pub const IMAGE_FOG_ON: C2RustUnnamed_1 = 199;
pub const IMAGE_FOG_OFF: C2RustUnnamed_1 = 198;
pub const IMAGE_PLAYERX: C2RustUnnamed_1 = 197;
pub const IMAGE_MEDAL_DUMMY: C2RustUnnamed_1 = 196;
pub const IMAGE_MULTIRANK2: C2RustUnnamed_1 = 195;
pub const IMAGE_PLAYER_PC: C2RustUnnamed_1 = 194;
pub const IMAGE_TEAM_HI: C2RustUnnamed_1 = 193;
pub const IMAGE_SKIRMISH_HI: C2RustUnnamed_1 = 192;
pub const IMAGE_TEAM: C2RustUnnamed_1 = 191;
pub const IMAGE_SKIRMISH: C2RustUnnamed_1 = 190;
pub const IMAGE_TEAM_OVER: C2RustUnnamed_1 = 189;
pub const IMAGE_SKIRMISH_OVER: C2RustUnnamed_1 = 188;
pub const IMAGE_LAMP_GREEN: C2RustUnnamed_1 = 187;
pub const IMAGE_LAMP_AMBER: C2RustUnnamed_1 = 186;
pub const IMAGE_LAMP_RED: C2RustUnnamed_1 = 185;
pub const IMAGE_COMPUTER_Y_HI: C2RustUnnamed_1 = 184;
pub const IMAGE_COMPUTER_Y: C2RustUnnamed_1 = 183;
pub const IMAGE_COMPUTER_N: C2RustUnnamed_1 = 182;
pub const IMAGE_COMPUTER_N_HI: C2RustUnnamed_1 = 181;
pub const IMAGE_HI56: C2RustUnnamed_1 = 180;
pub const IMAGE_DEFAULTFORCE: C2RustUnnamed_1 = 179;
pub const IMAGE_CLEARFORCE: C2RustUnnamed_1 = 178;
pub const IMAGE_SAVEFORCE: C2RustUnnamed_1 = 177;
pub const IMAGE_LOADFORCE: C2RustUnnamed_1 = 176;
pub const IMAGE_SLIM_HI: C2RustUnnamed_1 = 175;
pub const IMAGE_SLIM: C2RustUnnamed_1 = 174;
pub const IMAGE_RETURN_HI: C2RustUnnamed_1 = 173;
pub const IMAGE_FRAGLIMIT_HI: C2RustUnnamed_1 = 172;
pub const IMAGE_TIMELIMIT_HI: C2RustUnnamed_1 = 171;
pub const IMAGE_NOLIMIT_HI: C2RustUnnamed_1 = 170;
pub const IMAGE_FRAGLIMIT: C2RustUnnamed_1 = 169;
pub const IMAGE_TIMELIMIT: C2RustUnnamed_1 = 168;
pub const IMAGE_NOLIMIT: C2RustUnnamed_1 = 167;
pub const IMAGE_LBASE_HI: C2RustUnnamed_1 = 166;
pub const IMAGE_SBASE_HI: C2RustUnnamed_1 = 165;
pub const IMAGE_NOBASE_HI: C2RustUnnamed_1 = 164;
pub const IMAGE_LBASE: C2RustUnnamed_1 = 163;
pub const IMAGE_SBASE: C2RustUnnamed_1 = 162;
pub const IMAGE_NOBASE: C2RustUnnamed_1 = 161;
pub const IMAGE_TECHHI_HI: C2RustUnnamed_1 = 160;
pub const IMAGE_TECHMED_HI: C2RustUnnamed_1 = 159;
pub const IMAGE_TECHLO_HI: C2RustUnnamed_1 = 158;
pub const IMAGE_TECHHI: C2RustUnnamed_1 = 157;
pub const IMAGE_TECHMED: C2RustUnnamed_1 = 156;
pub const IMAGE_TECHLO: C2RustUnnamed_1 = 155;
pub const IMAGE_BIGOK: C2RustUnnamed_1 = 154;
pub const IMAGE_EDIT_GAME: C2RustUnnamed_1 = 153;
pub const IMAGE_EDIT_MAP: C2RustUnnamed_1 = 152;
pub const IMAGE_EDIT_FORCE: C2RustUnnamed_1 = 151;
pub const IMAGE_EDIT_PLAYER: C2RustUnnamed_1 = 150;
pub const IMAGE_RETURN: C2RustUnnamed_1 = 149;
pub const IMAGE_MULTIRANK1: C2RustUnnamed_1 = 148;
pub const IMAGE_POWLO: C2RustUnnamed_1 = 147;
pub const IMAGE_MEDAL_BRONZE: C2RustUnnamed_1 = 146;
pub const IMAGE_MEDAL_SILVER: C2RustUnnamed_1 = 145;
pub const IMAGE_MEDAL_GOLD: C2RustUnnamed_1 = 144;
pub const IMAGE_CAMPAIGN_OVER: C2RustUnnamed_1 = 143;
pub const IMAGE_ARENA_OVER: C2RustUnnamed_1 = 142;
pub const IMAGE_HI64: C2RustUnnamed_1 = 141;
pub const IMAGE_HI41: C2RustUnnamed_1 = 140;
pub const IMAGE_HI39: C2RustUnnamed_1 = 139;
pub const IMAGE_HI23: C2RustUnnamed_1 = 138;
pub const IMAGE_HI31: C2RustUnnamed_1 = 137;
pub const IMAGE_HI34: C2RustUnnamed_1 = 136;
pub const IMAGE_COM4_HI: C2RustUnnamed_1 = 135;
pub const IMAGE_COM3_HI: C2RustUnnamed_1 = 134;
pub const IMAGE_ALLI_HI: C2RustUnnamed_1 = 133;
pub const IMAGE_OFFALLI_HI: C2RustUnnamed_1 = 132;
pub const IMAGE_NOALLI_HI: C2RustUnnamed_1 = 131;
pub const IMAGE_ALLI: C2RustUnnamed_1 = 130;
pub const IMAGE_OFFALLI: C2RustUnnamed_1 = 129;
pub const IMAGE_NOALLI: C2RustUnnamed_1 = 128;
pub const IMAGE_POWHI_HI: C2RustUnnamed_1 = 127;
pub const IMAGE_POWMED_HI: C2RustUnnamed_1 = 126;
pub const IMAGE_POWLO_HI: C2RustUnnamed_1 = 125;
pub const IMAGE_POWHI: C2RustUnnamed_1 = 124;
pub const IMAGE_POWMED: C2RustUnnamed_1 = 123;
pub const IMAGE_OK: C2RustUnnamed_1 = 122;
pub const IMAGE_NO: C2RustUnnamed_1 = 121;
pub const IMAGE_HOST: C2RustUnnamed_1 = 120;
pub const IMAGE_PLAYER7: C2RustUnnamed_1 = 119;
pub const IMAGE_PLAYER6: C2RustUnnamed_1 = 118;
pub const IMAGE_PLAYER5: C2RustUnnamed_1 = 117;
pub const IMAGE_PLAYER4: C2RustUnnamed_1 = 116;
pub const IMAGE_PLAYER3: C2RustUnnamed_1 = 115;
pub const IMAGE_PLAYER2: C2RustUnnamed_1 = 114;
pub const IMAGE_PLAYER1: C2RustUnnamed_1 = 113;
pub const IMAGE_PLAYER0: C2RustUnnamed_1 = 112;
pub const IMAGE_REFRESH: C2RustUnnamed_1 = 111;
pub const IMAGE_CAMPAIGN: C2RustUnnamed_1 = 110;
pub const IMAGE_ARENA: C2RustUnnamed_1 = 109;
pub const IMAGE_115200: C2RustUnnamed_1 = 108;
pub const IMAGE_56000: C2RustUnnamed_1 = 107;
pub const IMAGE_19200: C2RustUnnamed_1 = 106;
pub const IMAGE_14400: C2RustUnnamed_1 = 105;
pub const IMAGE_CAMPAIGN_HI: C2RustUnnamed_1 = 104;
pub const IMAGE_ARENA_HI: C2RustUnnamed_1 = 103;
pub const IMAGE_115200_HI: C2RustUnnamed_1 = 102;
pub const IMAGE_56000_HI: C2RustUnnamed_1 = 101;
pub const IMAGE_19200_HI: C2RustUnnamed_1 = 100;
pub const IMAGE_14400_HI: C2RustUnnamed_1 = 99;
pub const IMAGE_COM2_HI: C2RustUnnamed_1 = 98;
pub const IMAGE_COM1_HI: C2RustUnnamed_1 = 97;
pub const IMAGE_COM4: C2RustUnnamed_1 = 96;
pub const IMAGE_COM3: C2RustUnnamed_1 = 95;
pub const IMAGE_COM2: C2RustUnnamed_1 = 94;
pub const IMAGE_COM1: C2RustUnnamed_1 = 93;
pub const IMAGE_FE_LOGO: C2RustUnnamed_1 = 0;
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
pub const ID_SOUND_OIL_PUMP_2: C2RustUnnamed_3 = 294;
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
pub const NO_SOUND: C2RustUnnamed_3 = -1;
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
    return psMapTiles.offset(x as
                                 isize).offset(y.wrapping_mul(mapWidth) as
                                                   isize);
}
// the sizes for the game block heap
// the sizes for the campaign map block heap
// the sizes for the mission block heap
// the block heap for the game data
#[no_mangle]
pub static mut psGameHeap: *mut BLOCK_HEAP =
    0 as *const BLOCK_HEAP as *mut BLOCK_HEAP;
// the block heap for the campaign map
#[no_mangle]
pub static mut psMapHeap: *mut BLOCK_HEAP =
    0 as *const BLOCK_HEAP as *mut BLOCK_HEAP;
// the block heap for the pre WRF data
#[no_mangle]
pub static mut psMissionHeap: *mut BLOCK_HEAP =
    0 as *const BLOCK_HEAP as *mut BLOCK_HEAP;
// the block id for the game data
// Ascii to font image id lookup table for frontend font.
// Same for WIN32 and PSX.
//
#[no_mangle]
pub static mut FEAsciiLookup: [UWORD; 256] =
    [IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_33 as libc::c_int as UWORD,
     IMAGE_TFONT_34 as libc::c_int as UWORD,
     IMAGE_TFONT_35 as libc::c_int as UWORD,
     IMAGE_TFONT_36 as libc::c_int as UWORD,
     IMAGE_TFONT_37 as libc::c_int as UWORD,
     IMAGE_TFONT_38 as libc::c_int as UWORD,
     IMAGE_TFONT_39 as libc::c_int as UWORD,
     IMAGE_TFONT_40 as libc::c_int as UWORD,
     IMAGE_TFONT_41 as libc::c_int as UWORD,
     IMAGE_TFONT_42 as libc::c_int as UWORD,
     IMAGE_TFONT_43 as libc::c_int as UWORD,
     IMAGE_TFONT_44 as libc::c_int as UWORD,
     IMAGE_TFONT_45 as libc::c_int as UWORD,
     IMAGE_TFONT_46 as libc::c_int as UWORD,
     IMAGE_TFONT_47 as libc::c_int as UWORD,
     IMAGE_TFONT_48 as libc::c_int as UWORD,
     IMAGE_TFONT_49 as libc::c_int as UWORD,
     IMAGE_TFONT_50 as libc::c_int as UWORD,
     IMAGE_TFONT_51 as libc::c_int as UWORD,
     IMAGE_TFONT_52 as libc::c_int as UWORD,
     IMAGE_TFONT_53 as libc::c_int as UWORD,
     IMAGE_TFONT_54 as libc::c_int as UWORD,
     IMAGE_TFONT_55 as libc::c_int as UWORD,
     IMAGE_TFONT_56 as libc::c_int as UWORD,
     IMAGE_TFONT_57 as libc::c_int as UWORD,
     IMAGE_TFONT_58 as libc::c_int as UWORD,
     IMAGE_TFONT_59 as libc::c_int as UWORD,
     IMAGE_TFONT_60 as libc::c_int as UWORD,
     IMAGE_TFONT_61 as libc::c_int as UWORD,
     IMAGE_TFONT_62 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_64 as libc::c_int as UWORD,
     IMAGE_TFONT_65 as libc::c_int as UWORD,
     IMAGE_TFONT_66 as libc::c_int as UWORD,
     IMAGE_TFONT_67 as libc::c_int as UWORD,
     IMAGE_TFONT_68 as libc::c_int as UWORD,
     IMAGE_TFONT_69 as libc::c_int as UWORD,
     IMAGE_TFONT_70 as libc::c_int as UWORD,
     IMAGE_TFONT_71 as libc::c_int as UWORD,
     IMAGE_TFONT_72 as libc::c_int as UWORD,
     IMAGE_TFONT_73 as libc::c_int as UWORD,
     IMAGE_TFONT_74 as libc::c_int as UWORD,
     IMAGE_TFONT_75 as libc::c_int as UWORD,
     IMAGE_TFONT_76 as libc::c_int as UWORD,
     IMAGE_TFONT_77 as libc::c_int as UWORD,
     IMAGE_TFONT_78 as libc::c_int as UWORD,
     IMAGE_TFONT_79 as libc::c_int as UWORD,
     IMAGE_TFONT_80 as libc::c_int as UWORD,
     IMAGE_TFONT_81 as libc::c_int as UWORD,
     IMAGE_TFONT_82 as libc::c_int as UWORD,
     IMAGE_TFONT_83 as libc::c_int as UWORD,
     IMAGE_TFONT_84 as libc::c_int as UWORD,
     IMAGE_TFONT_85 as libc::c_int as UWORD,
     IMAGE_TFONT_86 as libc::c_int as UWORD,
     IMAGE_TFONT_87 as libc::c_int as UWORD,
     IMAGE_TFONT_88 as libc::c_int as UWORD,
     IMAGE_TFONT_89 as libc::c_int as UWORD,
     IMAGE_TFONT_90 as libc::c_int as UWORD,
     IMAGE_TFONT_91 as libc::c_int as UWORD,
     IMAGE_TFONT_92 as libc::c_int as UWORD,
     IMAGE_TFONT_93 as libc::c_int as UWORD,
     IMAGE_TFONT_94 as libc::c_int as UWORD,
     IMAGE_TFONT_95 as libc::c_int as UWORD,
     IMAGE_TFONT_96 as libc::c_int as UWORD,
     IMAGE_TFONT_97 as libc::c_int as UWORD,
     IMAGE_TFONT_98 as libc::c_int as UWORD,
     IMAGE_TFONT_99 as libc::c_int as UWORD,
     IMAGE_TFONT_100 as libc::c_int as UWORD,
     IMAGE_TFONT_101 as libc::c_int as UWORD,
     IMAGE_TFONT_102 as libc::c_int as UWORD,
     IMAGE_TFONT_103 as libc::c_int as UWORD,
     IMAGE_TFONT_104 as libc::c_int as UWORD,
     IMAGE_TFONT_105 as libc::c_int as UWORD,
     IMAGE_TFONT_106 as libc::c_int as UWORD,
     IMAGE_TFONT_107 as libc::c_int as UWORD,
     IMAGE_TFONT_108 as libc::c_int as UWORD,
     IMAGE_TFONT_109 as libc::c_int as UWORD,
     IMAGE_TFONT_110 as libc::c_int as UWORD,
     IMAGE_TFONT_111 as libc::c_int as UWORD,
     IMAGE_TFONT_112 as libc::c_int as UWORD,
     IMAGE_TFONT_113 as libc::c_int as UWORD,
     IMAGE_TFONT_114 as libc::c_int as UWORD,
     IMAGE_TFONT_115 as libc::c_int as UWORD,
     IMAGE_TFONT_116 as libc::c_int as UWORD,
     IMAGE_TFONT_117 as libc::c_int as UWORD,
     IMAGE_TFONT_118 as libc::c_int as UWORD,
     IMAGE_TFONT_119 as libc::c_int as UWORD,
     IMAGE_TFONT_120 as libc::c_int as UWORD,
     IMAGE_TFONT_121 as libc::c_int as UWORD,
     IMAGE_TFONT_122 as libc::c_int as UWORD,
     IMAGE_TFONT_123 as libc::c_int as UWORD,
     IMAGE_TFONT_124 as libc::c_int as UWORD,
     IMAGE_TFONT_125 as libc::c_int as UWORD,
     IMAGE_TFONT_126 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_45 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_192 as libc::c_int as UWORD,
     IMAGE_TFONT_193 as libc::c_int as UWORD,
     IMAGE_TFONT_194 as libc::c_int as UWORD,
     IMAGE_TFONT_195 as libc::c_int as UWORD,
     IMAGE_TFONT_196 as libc::c_int as UWORD,
     IMAGE_TFONT_197 as libc::c_int as UWORD,
     IMAGE_TFONT_198 as libc::c_int as UWORD,
     IMAGE_TFONT_199 as libc::c_int as UWORD,
     IMAGE_TFONT_200 as libc::c_int as UWORD,
     IMAGE_TFONT_201 as libc::c_int as UWORD,
     IMAGE_TFONT_202 as libc::c_int as UWORD,
     IMAGE_TFONT_203 as libc::c_int as UWORD,
     IMAGE_TFONT_204 as libc::c_int as UWORD,
     IMAGE_TFONT_205 as libc::c_int as UWORD,
     IMAGE_TFONT_206 as libc::c_int as UWORD,
     IMAGE_TFONT_207 as libc::c_int as UWORD,
     IMAGE_TFONT_208 as libc::c_int as UWORD,
     IMAGE_TFONT_209 as libc::c_int as UWORD,
     IMAGE_TFONT_210 as libc::c_int as UWORD,
     IMAGE_TFONT_211 as libc::c_int as UWORD,
     IMAGE_TFONT_212 as libc::c_int as UWORD,
     IMAGE_TFONT_213 as libc::c_int as UWORD,
     IMAGE_TFONT_214 as libc::c_int as UWORD,
     IMAGE_TFONT_215 as libc::c_int as UWORD,
     IMAGE_TFONT_216 as libc::c_int as UWORD,
     IMAGE_TFONT_217 as libc::c_int as UWORD,
     IMAGE_TFONT_218 as libc::c_int as UWORD,
     IMAGE_TFONT_219 as libc::c_int as UWORD,
     IMAGE_TFONT_220 as libc::c_int as UWORD,
     IMAGE_TFONT_221 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_224 as libc::c_int as UWORD,
     IMAGE_TFONT_225 as libc::c_int as UWORD,
     IMAGE_TFONT_226 as libc::c_int as UWORD,
     IMAGE_TFONT_227 as libc::c_int as UWORD,
     IMAGE_TFONT_228 as libc::c_int as UWORD,
     IMAGE_TFONT_229 as libc::c_int as UWORD,
     IMAGE_TFONT_230 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_232 as libc::c_int as UWORD,
     IMAGE_TFONT_233 as libc::c_int as UWORD,
     IMAGE_TFONT_234 as libc::c_int as UWORD,
     IMAGE_TFONT_235 as libc::c_int as UWORD,
     IMAGE_TFONT_236 as libc::c_int as UWORD,
     IMAGE_TFONT_237 as libc::c_int as UWORD,
     IMAGE_TFONT_238 as libc::c_int as UWORD,
     IMAGE_TFONT_239 as libc::c_int as UWORD,
     IMAGE_TFONT_240 as libc::c_int as UWORD,
     IMAGE_TFONT_241 as libc::c_int as UWORD,
     IMAGE_TFONT_242 as libc::c_int as UWORD,
     IMAGE_TFONT_243 as libc::c_int as UWORD,
     IMAGE_TFONT_244 as libc::c_int as UWORD,
     IMAGE_TFONT_245 as libc::c_int as UWORD,
     IMAGE_TFONT_246 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_248 as libc::c_int as UWORD,
     IMAGE_TFONT_249 as libc::c_int as UWORD,
     IMAGE_TFONT_250 as libc::c_int as UWORD,
     IMAGE_TFONT_251 as libc::c_int as UWORD,
     IMAGE_TFONT_252 as libc::c_int as UWORD,
     IMAGE_TFONT_253 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD,
     IMAGE_TFONT_63 as libc::c_int as UWORD];
// Ascii to font image id lookup table for in game font.
// Same for WIN32 and PSX.
//
//#ifndef PSX
//UWORD AsciiLookup[256] =
//#else
// // We can use bytes as long as we ensure the font images are the 1st 256 in the image file.
#[no_mangle]
pub static mut AsciiLookup: [UWORD; 256] =
    [IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII42 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII33 as libc::c_int as UWORD,
     IMAGE_ASCII34 as libc::c_int as UWORD,
     IMAGE_ASCII35 as libc::c_int as UWORD,
     IMAGE_ASCII36 as libc::c_int as UWORD,
     IMAGE_ASCII37 as libc::c_int as UWORD,
     IMAGE_ASCII38 as libc::c_int as UWORD,
     IMAGE_ASCII39 as libc::c_int as UWORD,
     IMAGE_ASCII40 as libc::c_int as UWORD,
     IMAGE_ASCII41 as libc::c_int as UWORD,
     IMAGE_ASTERISK as libc::c_int as UWORD,
     IMAGE_ASCII43 as libc::c_int as UWORD,
     IMAGE_ASCII44 as libc::c_int as UWORD,
     IMAGE_ASCII45 as libc::c_int as UWORD,
     IMAGE_ASCII46 as libc::c_int as UWORD,
     IMAGE_ASCII47 as libc::c_int as UWORD,
     IMAGE_ASCII48 as libc::c_int as UWORD,
     IMAGE_ASCII49 as libc::c_int as UWORD,
     IMAGE_ASCII50 as libc::c_int as UWORD,
     IMAGE_ASCII51 as libc::c_int as UWORD,
     IMAGE_ASCII52 as libc::c_int as UWORD,
     IMAGE_ASCII53 as libc::c_int as UWORD,
     IMAGE_ASCII54 as libc::c_int as UWORD,
     IMAGE_ASCII55 as libc::c_int as UWORD,
     IMAGE_ASCII56 as libc::c_int as UWORD,
     IMAGE_ASCII57 as libc::c_int as UWORD,
     IMAGE_ASCII58 as libc::c_int as UWORD,
     IMAGE_ASCII59 as libc::c_int as UWORD,
     IMAGE_ASCII60 as libc::c_int as UWORD,
     IMAGE_ASCII61 as libc::c_int as UWORD,
     IMAGE_ASCII62 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII64 as libc::c_int as UWORD,
     IMAGE_ASCII65 as libc::c_int as UWORD,
     IMAGE_ASCII66 as libc::c_int as UWORD,
     IMAGE_ASCII67 as libc::c_int as UWORD,
     IMAGE_ASCII68 as libc::c_int as UWORD,
     IMAGE_ASCII69 as libc::c_int as UWORD,
     IMAGE_ASCII70 as libc::c_int as UWORD,
     IMAGE_ASCII71 as libc::c_int as UWORD,
     IMAGE_ASCII72 as libc::c_int as UWORD,
     IMAGE_ASCII73 as libc::c_int as UWORD,
     IMAGE_ASCII74 as libc::c_int as UWORD,
     IMAGE_ASCII75 as libc::c_int as UWORD,
     IMAGE_ASCII76 as libc::c_int as UWORD,
     IMAGE_ASCII77 as libc::c_int as UWORD,
     IMAGE_ASCII78 as libc::c_int as UWORD,
     IMAGE_ASCII79 as libc::c_int as UWORD,
     IMAGE_ASCII80 as libc::c_int as UWORD,
     IMAGE_ASCII81 as libc::c_int as UWORD,
     IMAGE_ASCII82 as libc::c_int as UWORD,
     IMAGE_ASCII83 as libc::c_int as UWORD,
     IMAGE_ASCII84 as libc::c_int as UWORD,
     IMAGE_ASCII85 as libc::c_int as UWORD,
     IMAGE_ASCII86 as libc::c_int as UWORD,
     IMAGE_ASCII87 as libc::c_int as UWORD,
     IMAGE_ASCII88 as libc::c_int as UWORD,
     IMAGE_ASCII89 as libc::c_int as UWORD,
     IMAGE_ASCII90 as libc::c_int as UWORD,
     IMAGE_ASCII91 as libc::c_int as UWORD,
     IMAGE_ASCII92 as libc::c_int as UWORD,
     IMAGE_ASCII93 as libc::c_int as UWORD,
     IMAGE_ASCII94 as libc::c_int as UWORD,
     IMAGE_ASCII95 as libc::c_int as UWORD,
     IMAGE_ASCII96 as libc::c_int as UWORD,
     IMAGE_ASCII97 as libc::c_int as UWORD,
     IMAGE_ASCII98 as libc::c_int as UWORD,
     IMAGE_ASCII99 as libc::c_int as UWORD,
     IMAGE_ASCII100 as libc::c_int as UWORD,
     IMAGE_ASCII101 as libc::c_int as UWORD,
     IMAGE_ASCII102 as libc::c_int as UWORD,
     IMAGE_ASCII103 as libc::c_int as UWORD,
     IMAGE_ASCII104 as libc::c_int as UWORD,
     IMAGE_ASCII105 as libc::c_int as UWORD,
     IMAGE_ASCII106 as libc::c_int as UWORD,
     IMAGE_ASCII107 as libc::c_int as UWORD,
     IMAGE_ASCII108 as libc::c_int as UWORD,
     IMAGE_ASCII109 as libc::c_int as UWORD,
     IMAGE_ASCII110 as libc::c_int as UWORD,
     IMAGE_ASCII111 as libc::c_int as UWORD,
     IMAGE_ASCII112 as libc::c_int as UWORD,
     IMAGE_ASCII113 as libc::c_int as UWORD,
     IMAGE_ASCII114 as libc::c_int as UWORD,
     IMAGE_ASCII115 as libc::c_int as UWORD,
     IMAGE_ASCII116 as libc::c_int as UWORD,
     IMAGE_ASCII117 as libc::c_int as UWORD,
     IMAGE_ASCII118 as libc::c_int as UWORD,
     IMAGE_ASCII119 as libc::c_int as UWORD,
     IMAGE_ASCII120 as libc::c_int as UWORD,
     IMAGE_ASCII121 as libc::c_int as UWORD,
     IMAGE_ASCII122 as libc::c_int as UWORD,
     IMAGE_ASCII123 as libc::c_int as UWORD,
     IMAGE_ASCII124 as libc::c_int as UWORD,
     IMAGE_ASCII125 as libc::c_int as UWORD,
     IMAGE_ASCII126 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII131 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII161 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII188 as libc::c_int as UWORD,
     IMAGE_ASCII189 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII191 as libc::c_int as UWORD,
     IMAGE_ASCII192 as libc::c_int as UWORD,
     IMAGE_ASCII193 as libc::c_int as UWORD,
     IMAGE_ASCII194 as libc::c_int as UWORD,
     IMAGE_ASCII195 as libc::c_int as UWORD,
     IMAGE_ASCII196 as libc::c_int as UWORD,
     IMAGE_ASCII197 as libc::c_int as UWORD,
     IMAGE_ASCII198 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII200 as libc::c_int as UWORD,
     IMAGE_ASCII201 as libc::c_int as UWORD,
     IMAGE_ASCII202 as libc::c_int as UWORD,
     IMAGE_ASCII203 as libc::c_int as UWORD,
     IMAGE_ASCII204 as libc::c_int as UWORD,
     IMAGE_ASCII205 as libc::c_int as UWORD,
     IMAGE_ASCII206 as libc::c_int as UWORD,
     IMAGE_ASCII207 as libc::c_int as UWORD,
     IMAGE_ASCII208 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII210 as libc::c_int as UWORD,
     IMAGE_ASCII211 as libc::c_int as UWORD,
     IMAGE_ASCII212 as libc::c_int as UWORD,
     IMAGE_ASCII213 as libc::c_int as UWORD,
     IMAGE_ASCII214 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII216 as libc::c_int as UWORD,
     IMAGE_ASCII217 as libc::c_int as UWORD,
     IMAGE_ASCII218 as libc::c_int as UWORD,
     IMAGE_ASCII219 as libc::c_int as UWORD,
     IMAGE_ASCII220 as libc::c_int as UWORD,
     IMAGE_ASCII221 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII223 as libc::c_int as UWORD,
     IMAGE_ASCII224 as libc::c_int as UWORD,
     IMAGE_ASCII225 as libc::c_int as UWORD,
     IMAGE_ASCII226 as libc::c_int as UWORD,
     IMAGE_ASCII227 as libc::c_int as UWORD,
     IMAGE_ASCII228 as libc::c_int as UWORD,
     IMAGE_ASCII229 as libc::c_int as UWORD,
     IMAGE_ASCII230 as libc::c_int as UWORD,
     IMAGE_ASCII231 as libc::c_int as UWORD,
     IMAGE_ASCII232 as libc::c_int as UWORD,
     IMAGE_ASCII233 as libc::c_int as UWORD,
     IMAGE_ASCII234 as libc::c_int as UWORD,
     IMAGE_ASCII235 as libc::c_int as UWORD,
     IMAGE_ASCII236 as libc::c_int as UWORD,
     IMAGE_ASCII237 as libc::c_int as UWORD,
     IMAGE_ASCII238 as libc::c_int as UWORD,
     IMAGE_ASCII239 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII241 as libc::c_int as UWORD,
     IMAGE_ASCII242 as libc::c_int as UWORD,
     IMAGE_ASCII243 as libc::c_int as UWORD,
     IMAGE_ASCII244 as libc::c_int as UWORD,
     IMAGE_ASCII245 as libc::c_int as UWORD,
     IMAGE_ASCII246 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII248 as libc::c_int as UWORD,
     IMAGE_ASCII249 as libc::c_int as UWORD,
     IMAGE_ASCII250 as libc::c_int as UWORD,
     IMAGE_ASCII251 as libc::c_int as UWORD,
     IMAGE_ASCII252 as libc::c_int as UWORD,
     IMAGE_ASCII253 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD,
     IMAGE_ASCII63 as libc::c_int as UWORD];
#[no_mangle]
pub static mut FrontImages: *mut IMAGEFILE =
    0 as *const IMAGEFILE as *mut IMAGEFILE;
#[no_mangle]
pub static mut DirectControl: BOOL = 0 as libc::c_int;
/*
 * Init.h
 *
 * Interface to the initialisation routines.
 *
 */
// the size of the file loading buffer
//extern int FEBigFont;
// Each module in the game should have a call from here to initialise
// any globals and statics to there default values each time the game
// or frontend restarts.
//
#[no_mangle]
pub unsafe extern "C" fn InitialiseGlobals() -> BOOL {
    frontendInitVars(); // Initialise frontend globals and statics.
    statsInitVars();
    structureInitVars();
    if messageInitVars() == 0 { return 0 as libc::c_int }
    if researchInitVars() == 0 { return 0 as libc::c_int }
    featureInitVars();
    radarInitVars();
    Edit3DInitVars();
    snapInitVars();
    driveInitVars(1 as libc::c_int);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn loadLevFile(mut filename: *const libc::c_char,
                                     mut datadir: libc::c_int) -> BOOL {
    let mut pBuffer: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut size: UDWORD = 0;
    if PHYSFS_exists(filename) == 0 ||
           loadFile(filename, &mut pBuffer, &mut size) == 0 {
        debug(LOG_ERROR,
              b"loadLevFile: File not found: %s\n\x00" as *const u8 as
                  *const libc::c_char, filename);
        return 0 as libc::c_int
        // only in NDEBUG case
    }
    if levParse(pBuffer, size as SDWORD, datadir) == 0 {
        debug(LOG_ERROR,
              b"loadLevFile: Parse error in %s\n\x00" as *const u8 as
                  *const libc::c_char, filename);
        return 0 as libc::c_int
    }
    memFreeRelease(pBuffer as *mut libc::c_void);
    pBuffer = 0 as *mut libc::c_char;
    return 1 as libc::c_int;
}
static mut searchPathRegistry: *mut wzSearchPath =
    0 as *const wzSearchPath as *mut wzSearchPath;
#[no_mangle]
pub unsafe extern "C" fn cleanSearchPath() {
    let mut curSearchPath: *mut wzSearchPath = searchPathRegistry;
    let mut tmpSearchPath: *mut wzSearchPath = 0 as *mut wzSearchPath;
    // Start at the lowest priority
    while !(*curSearchPath).lowerPriority.is_null() {
        curSearchPath = (*curSearchPath).lowerPriority
    }
    while !curSearchPath.is_null() {
        tmpSearchPath = (*curSearchPath).higherPriority;
        free(curSearchPath as *mut libc::c_void);
        curSearchPath = tmpSearchPath
    };
}
// Register searchPath above the path with next lower priority
#[no_mangle]
pub unsafe extern "C" fn registerSearchPath(mut path: *const libc::c_char,
                                            mut priority: libc::c_uint) {
    let mut curSearchPath: *mut wzSearchPath = searchPathRegistry;
    let mut tmpSearchPath: *mut wzSearchPath = 0 as *mut wzSearchPath;
    tmpSearchPath =
        malloc(::std::mem::size_of::<wzSearchPath>() as libc::c_ulong) as
            *mut wzSearchPath;
    strcpy((*tmpSearchPath).path.as_mut_ptr(), path);
    if *path.offset(strlen(path).wrapping_sub(1 as libc::c_int as
                                                  libc::c_uint) as isize) as
           libc::c_int != *PHYSFS_getDirSeparator() as libc::c_int {
        strcat((*tmpSearchPath).path.as_mut_ptr(), PHYSFS_getDirSeparator());
    }
    (*tmpSearchPath).priority = priority;
    debug(LOG_WZ,
          b"registerSearchPath: Registering %s at priority %i\x00" as
              *const u8 as *const libc::c_char, path, priority);
    if curSearchPath.is_null() {
        searchPathRegistry = tmpSearchPath;
        (*searchPathRegistry).lowerPriority = 0 as *mut _wzSearchPath;
        (*searchPathRegistry).higherPriority = 0 as *mut _wzSearchPath;
        return
    }
    while !(*curSearchPath).higherPriority.is_null() &&
              priority > (*curSearchPath).priority {
        curSearchPath = (*curSearchPath).higherPriority
    }
    while !(*curSearchPath).lowerPriority.is_null() &&
              priority < (*curSearchPath).priority {
        curSearchPath = (*curSearchPath).lowerPriority
    }
    if priority < (*curSearchPath).priority {
        (*tmpSearchPath).lowerPriority = (*curSearchPath).lowerPriority;
        (*tmpSearchPath).higherPriority = curSearchPath
    } else {
        (*tmpSearchPath).lowerPriority = curSearchPath;
        (*tmpSearchPath).higherPriority = (*curSearchPath).higherPriority
    }
    if !(*tmpSearchPath).lowerPriority.is_null() {
        (*(*tmpSearchPath).lowerPriority).higherPriority = tmpSearchPath
    }
    if !(*tmpSearchPath).higherPriority.is_null() {
        (*(*tmpSearchPath).higherPriority).lowerPriority = tmpSearchPath
    };
}
/*
 * \fn BOOL rebuildSearchPath( int mode )
 * \brief Rebuilds the PHYSFS searchPath with mode specific subdirs
 *
 * Priority:
 * maps > mods > plain_dir > warzone.wz
 */
#[no_mangle]
pub unsafe extern "C" fn rebuildSearchPath(mut mode: searchPathMode,
                                           mut force: BOOL) -> BOOL {
    static mut current_mode: searchPathMode = mod_clean;
    let mut curSearchPath: *mut wzSearchPath = searchPathRegistry;
    let mut tmpstr: [libc::c_char; 260] =
        *::std::mem::transmute::<&[u8; 260],
                                 &mut [libc::c_char; 260]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
    if mode as libc::c_uint != current_mode as libc::c_uint || force != 0 {
        current_mode = mode;
        rebuildSearchPath(mod_clean, 0 as libc::c_int);
        // DEBUG
        while !(*curSearchPath).lowerPriority.is_null()
              // Start at the lowest priority
              {
            curSearchPath = (*curSearchPath).lowerPriority
        }
        match mode as libc::c_uint {
            0 => {
                debug(LOG_WZ,
                      b"rebuildSearchPath: Cleaning up\x00" as *const u8 as
                          *const libc::c_char);
                while !curSearchPath.is_null() {
                    // DEBUG
                    // Remove maps and mods
                    removeSubdirs((*curSearchPath).path.as_mut_ptr(),
                                  b"maps\x00" as *const u8 as
                                      *const libc::c_char,
                                  0 as *mut *mut libc::c_char);
                    removeSubdirs((*curSearchPath).path.as_mut_ptr(),
                                  b"mods/global\x00" as *const u8 as
                                      *const libc::c_char,
                                  global_mods.as_mut_ptr());
                    removeSubdirs((*curSearchPath).path.as_mut_ptr(),
                                  b"mods/campaign\x00" as *const u8 as
                                      *const libc::c_char,
                                  campaign_mods.as_mut_ptr());
                    removeSubdirs((*curSearchPath).path.as_mut_ptr(),
                                  b"mods/multiplay\x00" as *const u8 as
                                      *const libc::c_char,
                                  multiplay_mods.as_mut_ptr());
                    // Remove multiplay patches
                    strcpy(tmpstr.as_mut_ptr(),
                           (*curSearchPath).path.as_mut_ptr());
                    strcat(tmpstr.as_mut_ptr(),
                           b"mp\x00" as *const u8 as *const libc::c_char);
                    PHYSFS_removeFromSearchPath(tmpstr.as_mut_ptr());
                    strcpy(tmpstr.as_mut_ptr(),
                           (*curSearchPath).path.as_mut_ptr());
                    strcat(tmpstr.as_mut_ptr(),
                           b"mp.wz\x00" as *const u8 as *const libc::c_char);
                    PHYSFS_removeFromSearchPath(tmpstr.as_mut_ptr());
                    // Remove plain dir
                    PHYSFS_removeFromSearchPath((*curSearchPath).path.as_mut_ptr());
                    // Remove warzone.wz
                    strcpy(tmpstr.as_mut_ptr(),
                           (*curSearchPath).path.as_mut_ptr());
                    strcat(tmpstr.as_mut_ptr(),
                           b"warzone.wz\x00" as *const u8 as
                               *const libc::c_char);
                    PHYSFS_removeFromSearchPath(tmpstr.as_mut_ptr());
                    curSearchPath = (*curSearchPath).higherPriority
                }
            }
            1 => {
                debug(LOG_WZ,
                      b"rebuildSearchPath: Switching to campaign mods\x00" as
                          *const u8 as *const libc::c_char);
                while !curSearchPath.is_null() {
                    // DEBUG
                    // Add global and campaign mods
                    PHYSFS_addToSearchPath((*curSearchPath).path.as_mut_ptr(),
                                           1 as libc::c_int);
                    addSubdirs((*curSearchPath).path.as_mut_ptr(),
                               b"mods/global\x00" as *const u8 as
                                   *const libc::c_char, 1 as libc::c_int,
                               global_mods.as_mut_ptr());
                    addSubdirs((*curSearchPath).path.as_mut_ptr(),
                               b"mods/campaign\x00" as *const u8 as
                                   *const libc::c_char, 1 as libc::c_int,
                               campaign_mods.as_mut_ptr());
                    PHYSFS_removeFromSearchPath((*curSearchPath).path.as_mut_ptr());
                    // Add plain dir
                    PHYSFS_addToSearchPath((*curSearchPath).path.as_mut_ptr(),
                                           1 as libc::c_int);
                    // Add warzone.wz
                    strcpy(tmpstr.as_mut_ptr(),
                           (*curSearchPath).path.as_mut_ptr());
                    strcat(tmpstr.as_mut_ptr(),
                           b"warzone.wz\x00" as *const u8 as
                               *const libc::c_char);
                    PHYSFS_addToSearchPath(tmpstr.as_mut_ptr(),
                                           1 as libc::c_int);
                    curSearchPath = (*curSearchPath).higherPriority
                }
            }
            2 => {
                debug(LOG_WZ,
                      b"rebuildSearchPath: Switching to multiplay mods\x00" as
                          *const u8 as *const libc::c_char);
                while !curSearchPath.is_null() {
                    // DEBUG
                    // Add maps and global and multiplay mods
                    PHYSFS_addToSearchPath((*curSearchPath).path.as_mut_ptr(),
                                           1 as libc::c_int);
                    addSubdirs((*curSearchPath).path.as_mut_ptr(),
                               b"maps\x00" as *const u8 as
                                   *const libc::c_char, 1 as libc::c_int,
                               0 as *mut *mut libc::c_char);
                    addSubdirs((*curSearchPath).path.as_mut_ptr(),
                               b"mods/global\x00" as *const u8 as
                                   *const libc::c_char, 1 as libc::c_int,
                               global_mods.as_mut_ptr());
                    addSubdirs((*curSearchPath).path.as_mut_ptr(),
                               b"mods/multiplay\x00" as *const u8 as
                                   *const libc::c_char, 1 as libc::c_int,
                               multiplay_mods.as_mut_ptr());
                    PHYSFS_removeFromSearchPath((*curSearchPath).path.as_mut_ptr());
                    // Add multiplay patches
                    strcpy(tmpstr.as_mut_ptr(),
                           (*curSearchPath).path.as_mut_ptr());
                    strcat(tmpstr.as_mut_ptr(),
                           b"mp\x00" as *const u8 as *const libc::c_char);
                    PHYSFS_addToSearchPath(tmpstr.as_mut_ptr(),
                                           1 as libc::c_int);
                    strcpy(tmpstr.as_mut_ptr(),
                           (*curSearchPath).path.as_mut_ptr());
                    strcat(tmpstr.as_mut_ptr(),
                           b"mp.wz\x00" as *const u8 as *const libc::c_char);
                    PHYSFS_addToSearchPath(tmpstr.as_mut_ptr(),
                                           1 as libc::c_int);
                    // Add plain dir
                    PHYSFS_addToSearchPath((*curSearchPath).path.as_mut_ptr(),
                                           1 as libc::c_int);
                    // Add warzone.wz
                    strcpy(tmpstr.as_mut_ptr(),
                           (*curSearchPath).path.as_mut_ptr());
                    strcat(tmpstr.as_mut_ptr(),
                           b"warzone.wz\x00" as *const u8 as
                               *const libc::c_char);
                    PHYSFS_addToSearchPath(tmpstr.as_mut_ptr(),
                                           1 as libc::c_int);
                    curSearchPath = (*curSearchPath).higherPriority
                }
            }
            _ => {
                debug(LOG_ERROR,
                      b"rebuildSearchPath: Can\'t switch to unknown mods %i\x00"
                          as *const u8 as *const libc::c_char,
                      mode as libc::c_uint);
                return 0 as libc::c_int
            }
        }
        PHYSFS_removeFromSearchPath(PHYSFS_getWriteDir());
        PHYSFS_addToSearchPath(PHYSFS_getWriteDir(), 0 as libc::c_int);
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn buildMapList() -> BOOL {
    let mut filelist: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut file: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut len: size_t = 0;
    if loadLevFile(b"gamedesc.lev\x00" as *const u8 as *const libc::c_char,
                   1 as libc::c_int) == 0 {
        return 0 as libc::c_int
    }
    loadLevFile(b"addon.lev\x00" as *const u8 as *const libc::c_char,
                2 as libc::c_int);
    filelist =
        PHYSFS_enumerateFiles(b"\x00" as *const u8 as *const libc::c_char);
    file = filelist;
    while !(*file).is_null() {
        len = strlen(*file);
        if len > 10 as libc::c_int as libc::c_uint &&
               strcasecmp((*file).offset(len.wrapping_sub(10 as libc::c_int as
                                                              libc::c_uint) as
                                             isize),
                          b".addon.lev\x00" as *const u8 as
                              *const libc::c_char) == 0 {
            debug(LOG_WZ,
                  b"Loading lev file: %s\n\x00" as *const u8 as
                      *const libc::c_char, *file);
            loadLevFile(*file, 2 as libc::c_int);
        }
        file = file.offset(1)
    }
    PHYSFS_freeList(filelist as *mut libc::c_void);
    return 1 as libc::c_int;
}
// User's home dir must be first so we allways see what we write
// ////////////////////////////////////////////////////////////////////////////
// ////////////////////////////////////////////////////////////////////////////
// Called once on program startup.
//
#[no_mangle]
pub unsafe extern "C" fn systemInitialise() -> BOOL {
    let mut sWInit: W_HEAPINIT =
        W_HEAPINIT{barInit: 0,
                   barExt: 0,
                   butInit: 0,
                   butExt: 0,
                   edbInit: 0,
                   edbExt: 0,
                   formInit: 0,
                   formExt: 0,
                   cFormInit: 0,
                   cFormExt: 0,
                   tFormInit: 0,
                   tFormExt: 0,
                   labInit: 0,
                   labExt: 0,
                   sldInit: 0,
                   sldExt:
                       0,}; // was 30 ... but what about the virtual keyboard
    memset(&mut sWInit as *mut W_HEAPINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_HEAPINIT>() as libc::c_ulong);
    sWInit.barInit = 40 as libc::c_int as UDWORD;
    sWInit.barExt = 5 as libc::c_int as UDWORD;
    sWInit.butInit = 50 as libc::c_int as UDWORD;
    sWInit.butExt = 5 as libc::c_int as UDWORD;
    sWInit.edbInit = 2 as libc::c_int as UDWORD;
    sWInit.edbExt = 1 as libc::c_int as UDWORD;
    sWInit.formInit = 10 as libc::c_int as UDWORD;
    sWInit.formExt = 2 as libc::c_int as UDWORD;
    sWInit.cFormInit = 50 as libc::c_int as UDWORD;
    sWInit.cFormExt = 5 as libc::c_int as UDWORD;
    sWInit.tFormInit = 3 as libc::c_int as UDWORD;
    sWInit.tFormExt = 2 as libc::c_int as UDWORD;
    sWInit.labInit = 15 as libc::c_int as UDWORD;
    sWInit.labExt = 3 as libc::c_int as UDWORD;
    sWInit.sldInit = 2 as libc::c_int as UDWORD;
    sWInit.sldExt = 1 as libc::c_int as UDWORD;
    if widgInitialise(&mut sWInit) == 0 { return 0 as libc::c_int }
    buildMapList();
    //loadLevels(DIR_CAMPAIGN);
    // Initialize render engine
    war_SetFog(0 as libc::c_int);
    if pie_Initialise() == 0 {
        debug(LOG_ERROR,
              b"Unable to initialise renderer\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    pie_SetTranslucent(war_GetTranslucent());
    pie_SetAdditive(war_GetAdditive());
    //	displayBufferSize = iV_GetDisplayWidth()*iV_GetDisplayHeight()*iV_GetDisplayBytesPP();
    displayBufferSize =
        pie_GetVideoBufferWidth().wrapping_mul(pie_GetVideoBufferHeight()).wrapping_mul(2
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            libc::c_uint);
    if displayBufferSize < 5000000 as libc::c_int as libc::c_uint {
        displayBufferSize = 5000000 as libc::c_int as UDWORD
    }
    DisplayBuffer = memMallocRelease(displayBufferSize) as *mut libc::c_char;
    if DisplayBuffer.is_null() {
        debug(LOG_ERROR,
              b"Unable to allocate memory for display buffer\x00" as *const u8
                  as *const libc::c_char);
        abort();
    }
    if audio_Init(Some(droidAudioTrackStopped as
                           unsafe extern "C" fn(_: *mut AUDIO_SAMPLE)
                               -> BOOL)) == 0 {
        debug(LOG_SOUND,
              b"Couldn\'t initialise audio system: continuing without audio\n\x00"
                  as *const u8 as *const libc::c_char);
    }
    if war_GetPlayAudioCDs() != 0 {
        cdAudio_Open(UserMusicPath.as_mut_ptr());
        mixer_Open();
    }
    if bDisableLobby == 0 && multiInitialise() == 0 {
        // ajl. Init net stuff
        return 0 as libc::c_int
    }
    if dataInitLoadFuncs() == 0 {
        // Pass all the data loading functions to the framework library
        return 0 as libc::c_int
    }
    if rayInitialise() == 0 {
        /* Initialise the ray tables */
        return 0 as libc::c_int
    }
    if fpathInitialise() == 0 { return 0 as libc::c_int }
    if astarInitialise() == 0 {
        // Initialise the findpath system
        return 0 as libc::c_int
    } // get favourite settings from the registry
    loadConfig(0 as libc::c_int);
    // create a block heap for the game data
    if blkCreate(&mut psGameHeap,
                 2 as libc::c_int * 1024 as libc::c_int * 1024 as libc::c_int,
                 1024 as libc::c_int * 1024 as libc::c_int) == 0 {
        return 0 as libc::c_int
    }
    // create a block heap for the campaign map
    if blkCreate(&mut psMapHeap, 1024 as libc::c_int * 1024 as libc::c_int,
                 32 as libc::c_int * 1024 as libc::c_int) == 0 {
        return 0 as libc::c_int
    }
    // create a block heap for the pre WRF data
    if blkCreate(&mut psMissionHeap,
                 2 as libc::c_int * 1024 as libc::c_int * 1024 as libc::c_int,
                 512 as libc::c_int * 1024 as libc::c_int) == 0 {
        return 0 as libc::c_int
    } // Reset the IV library.
    iV_Reset(1 as libc::c_int);
    initLoadingScreen(1 as libc::c_int, 0 as libc::c_int);
    return 1 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////////
// ////////////////////////////////////////////////////////////////////////////
// Called once at program shutdown.
//
#[no_mangle]
pub unsafe extern "C" fn systemShutdown() -> BOOL {
    //	unsigned int i;
    keyClearMappings();
    fpathShutDown();
    // free up all the load functions (all the data should already have been freed)
    resReleaseAll();
    /*
	for( i = 0; i < data_dirs_size; i++ )
	{
		free( data_dirs[i].name );
	}
	free( data_dirs );
*/
    // release the block heaps
    blkDestroy(psGameHeap);
    blkDestroy(psMapHeap);
    blkDestroy(psMissionHeap);
    if bDisableLobby == 0 && multiShutdown() == 0 {
        // ajl. init net stuff
        return 0 as libc::c_int
    }
    debug(LOG_MAIN,
          b"shutting down audio subsystems\x00" as *const u8 as
              *const libc::c_char);
    if war_GetPlayAudioCDs() != 0 {
        debug(LOG_MAIN,
              b"shutting down CD audio\x00" as *const u8 as
                  *const libc::c_char);
        cdAudio_Stop();
        cdAudio_Close();
        mixer_Close();
    }
    if audio_Disabled() == 0 as libc::c_int && audio_Shutdown() == 0 {
        return 0 as libc::c_int
    }
    debug(LOG_MAIN,
          b"shutting down graphics subsystem\x00" as *const u8 as
              *const libc::c_char);
    memFreeRelease(DisplayBuffer as *mut libc::c_void);
    DisplayBuffer = 0 as *mut libc::c_char;
    iV_ShutDown();
    levShutDown();
    widgShutDown();
    return 1 as libc::c_int;
}
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn init_ObjectDead(mut psObj: *mut libc::c_void)
 -> BOOL {
    let mut psBaseObj: *mut BASE_OBJECT = psObj as *mut BASE_OBJECT;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut psStructure: *mut STRUCTURE = 0 as *mut STRUCTURE;
    /* check is valid pointer */
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"init_ObjectDead: game object pointer invalid\n\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"init.c\x00" as *const u8 as *const libc::c_char,
              1159 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 16],
                                        &[libc::c_char; 16]>(b"init_ObjectDead\x00")).as_ptr(),
              b"PTRVALID(psBaseObj, sizeof(BASE_OBJECT))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (*psBaseObj).died == 1 as libc::c_int as libc::c_uint {
        match (*psBaseObj).type_0 as libc::c_uint {
            0 => {
                psDroid = psBaseObj as *mut DROID;
                (*psDroid).psCurAnim = 0 as *mut ANIM_OBJECT
            }
            1 => {
                psStructure = psBaseObj as *mut STRUCTURE;
                (*psStructure).psCurAnim = 0 as *mut ANIM_OBJECT
            }
            _ => {
                debug(LOG_ERROR,
                      b"init_ObjectAnimRemoved: unrecognised object type\x00"
                          as *const u8 as *const libc::c_char);
                abort();
            }
        }
    }
    return (*psBaseObj).died as BOOL;
}
// ////////////////////////////////////////////////////////////////////////////
// ////////////////////////////////////////////////////////////////////////////
// Called At Frontend Startup.
#[no_mangle]
pub unsafe extern "C" fn frontendInitialise(mut ResourceFile:
                                                *mut libc::c_char) -> BOOL {
    debug(LOG_MAIN,
          b"Initialising frontend : %s\x00" as *const u8 as
              *const libc::c_char, ResourceFile);
    // allocate memory from the pre data heap
    memSetBlockHeap(psGameHeap);
    if InitialiseGlobals() == 0 {
        // Initialise all globals and statics everywhere.
        return 0 as libc::c_int
    } // Reset the IV library.
    iV_Reset(1 as libc::c_int);
    if scrTabInitialise() == 0 {
        // Initialise the script system
        return 0 as libc::c_int
    }
    if stringsInitialise() == 0 {
        // Initialise the string system
        return 0 as libc::c_int
    }
    if objInitialise() == 0 {
        // Initialise the object system
        return 0 as libc::c_int
    }
    if anim_Init(Some(anim_GetShapeFunc as
                          unsafe extern "C" fn(_: *mut STRING)
                              -> *mut libc::c_void)) == 0 {
        return 0 as libc::c_int
    }
    if animObj_Init(Some(init_ObjectDead as
                             unsafe extern "C" fn(_: *mut libc::c_void)
                                 -> BOOL)) == 0 {
        return 0 as libc::c_int
    }
    if allocPlayerPower() == 0 {
        //set up the PlayerPower for each player - this should only be done ONCE now
        return 0 as libc::c_int
    }
    debug(LOG_MAIN,
          b"frontEndInitialise: loading resource file .....\x00" as *const u8
              as *const libc::c_char);
    if resLoad(ResourceFile, 0 as libc::c_int, DisplayBuffer,
               displayBufferSize as SDWORD, psGameHeap) == 0 {
        //need the object heaps to have been set up before loading in the save game
        return 0 as libc::c_int
    }
    if dispInitialise() == 0 {
        // Initialise the display system
        return 0 as libc::c_int
    }
    if bucketSetupList() == 0 {
        // reset object list
        return 0 as libc::c_int
    }
    FrontImages =
        resGetData(b"IMG\x00" as *const u8 as *const libc::c_char as
                       *mut STRING,
                   b"frend.img\x00" as *const u8 as *const libc::c_char as
                       *mut STRING) as *mut IMAGEFILE;
    FEFont =
        iV_CreateFontIndirect(FrontImages, FEAsciiLookup.as_mut_ptr(),
                              4 as libc::c_int);
    /* Shift the interface initialisation here temporarily so that it
   		can pick up the stats after they have been loaded */
    if intInitialise() == 0 {
        return 0 as libc::c_int
    } // get favourite settings from the registry
    loadConfig(1 as libc::c_int);
    // keymappings
	// clear out any existing mappings
    keyClearMappings(); // Set the default cursor shape.
    keyInitMappings(0 as
                        libc::c_int); // disable the open noise since distorted in 3dfx builds.
    pie_SetMouse(IntImages, IMAGE_CURSOR_DEFAULT as libc::c_int as UWORD);
    frameSetCursorFromRes(108 as libc::c_int as WORD);
    SetFormAudioIDs(-(1 as libc::c_int), ID_SOUND_WINDOWCLOSE as libc::c_int);
    memSetBlockHeap(0 as *mut _block_heap);
    initMiscVars();
    gameTimeInit();
    // hit me with some funky beats....
    if war_GetPlayAudioCDs() != 0 {
        cdAudio_PlayTrack(2 as libc::c_int);
        // track 2 = f.e. music,
    }
    return 1 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////////
// ////////////////////////////////////////////////////////////////////////////
// !PSX version. Called at frontend Shutdown, before game runs.
//
#[no_mangle]
pub unsafe extern "C" fn frontendShutdown() -> BOOL {
    debug(LOG_MAIN,
          b"Shuting down frontend\x00" as *const u8 as
              *const libc::c_char); // save settings to registry.
    saveConfig();
    closeConfig();
    //	if (!aiShutdown())
//	{
//		return FALSE;
//	}
    if mechShutdown() == 0 { return 0 as libc::c_int }
    releasePlayerPower();
    intShutDown();
    scrShutDown();
    //do this before shutting down the iV library
//	D3DFreeTexturePages();
    resReleaseAllData();
    if objShutdown() == 0 { return 0 as libc::c_int }
    ResearchRelease();
    if anim_Shutdown() == 0 { return 0 as libc::c_int }
    if animObj_Shutdown() == 0 { return 0 as libc::c_int }
    /*
	if (!dispShutdown())
	{
		return FALSE;
	}
*/
    pie_TexShutDown();
    // reset the block heap
    blkReset(psGameHeap);
    return 1 as libc::c_int;
}
/* *****************************************************************************/
/*                       Initialisation before data is loaded                 */
#[no_mangle]
pub unsafe extern "C" fn stageOneInitialise() -> BOOL {
    let mut psHeap: *mut BLOCK_HEAP = 0 as *mut BLOCK_HEAP;
    debug(LOG_MAIN,
          b"stageOneInitalise\x00" as *const u8 as *const libc::c_char);
    // Initialise all globals and statics everwhere.
    if InitialiseGlobals() == 0 {
        return 0 as libc::c_int
    } // Reset the IV library. (but not the palette)
    iV_Reset(0 as libc::c_int);
    if stringsInitialise() == 0 {
        /* Initialise the string system */
        return 0 as libc::c_int
    }
    if objInitialise() == 0 {
        /* Initialise the object system */
        return 0 as libc::c_int
    }
    if droidInit() == 0 { return 0 as libc::c_int }
    if initViewData() == 0 { return 0 as libc::c_int }
    if grpInitialise() == 0 { return 0 as libc::c_int }
    if aiInitialise() == 0 {
        /* Initialise the AI system */ // pregame
        return 0 as libc::c_int
    }
    // debug mode only so use normal MALLOC
    psHeap = memGetBlockHeap();
    memSetBlockHeap(0 as *mut _block_heap);
    memSetBlockHeap(psHeap);
    if anim_Init(Some(anim_GetShapeFunc as
                          unsafe extern "C" fn(_: *mut STRING)
                              -> *mut libc::c_void)) == 0 {
        return 0 as libc::c_int
    }
    if animObj_Init(Some(init_ObjectDead as
                             unsafe extern "C" fn(_: *mut libc::c_void)
                                 -> BOOL)) == 0 {
        return 0 as libc::c_int
    }
    if allocPlayerPower() == 0 {
        /*set up the PlayerPower for each player - this should only be done ONCE now*/
        return 0 as libc::c_int
    }
    if formationInitialise() == 0 {
        // Initialise the formation system
        return 0 as libc::c_int
    }
    // initialise the visibility stuff
    if visInitialise() == 0 { return 0 as libc::c_int }
    /* Initialise the movement system */
    if moveInitialise() == 0 { return 0 as libc::c_int }
    if proj_InitSystem() == 0 { return 0 as libc::c_int }
    if scrTabInitialise() == 0 {
        // Initialise the script system
        return 0 as libc::c_int
    }
    if gridInitialise() == 0 { return 0 as libc::c_int }
    if environInit() == 0 { return 0 as libc::c_int }
    // reset speed limiter
    moveSetFormationSpeedLimiting(1 as libc::c_int);
    initMission();
    initTransporters();
    //do this here so that the very first mission has it initialised
    initRunData();
    gameTimeInit();
    //need to reset the event timer too - AB 14/01/99
    eventTimeReset(gameTime.wrapping_div(100 as libc::c_int as libc::c_uint));
    // Set the cursor snap max distances.
    SetMaxDist(64 as libc::c_int, 64 as libc::c_int);
    return 1 as libc::c_int;
}
/* *****************************************************************************/
/*                       Shutdown after data is released                      */
#[no_mangle]
pub unsafe extern "C" fn stageOneShutDown() -> BOOL {
    debug(LOG_MAIN,
          b"stageOneShutDown\x00" as *const u8 as *const libc::c_char);
    // ffs
	//do this before shutting down the iV library
//	D3DFreeTexturePages();
    if audio_Disabled() == 0 as libc::c_int { audio_CheckAllUnloaded(); }
    proj_Shutdown();
    releaseMission();
    if aiShutdown() == 0 { return 0 as libc::c_int }
    if objShutdown() == 0 { return 0 as libc::c_int }
    grpShutDown();
    formationShutDown();
    releasePlayerPower();
    ResearchRelease();
    //free up the gateway stuff?
    gwShutDown();
    if mapShutdown() == 0 { return 0 as libc::c_int }
    scrShutDown();
    environShutDown();
    gridShutDown();
    if anim_Shutdown() == 0 { return 0 as libc::c_int }
    if animObj_Shutdown() == 0 { return 0 as libc::c_int }
    pie_TexShutDown();
    viewDataHeapShutDown();
    initMiscVars();
    return 1 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////////
// Initialise after the base data is loaded but before final level data is loaded
#[no_mangle]
pub unsafe extern "C" fn stageTwoInitialise() -> BOOL {
    debug(LOG_MAIN,
          b"stageTwoInitalise\x00" as *const u8 as *const libc::c_char);
    if bMultiPlayer != 0 {
        if multiTemplateSetup() == 0 { return 0 as libc::c_int }
    }
    if dispInitialise() == 0 {
        /* Initialise the display system */
        return 0 as libc::c_int
    }
    //	loadingScreenCallback();
    if InitRadar() == 0 {
        // After resLoad cause it needs the game palette initialised.
        return 0 as libc::c_int
    }
    //	loadingScreenCallback();
    if initMiscImds() == 0 {
        /* Set up the explosions */
        iV_ShutDown();
        debug(LOG_ERROR,
              b"Can\'t find all the explosions PCX\'s\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    /*
	if (!loadExtraIMDs())
	{
		return FALSE;
	}
*/
    /*if (!mechInitialise())		// Initialise the mechanics system
	{
		return FALSE;
	}*/
    if cmdDroidInit() == 0 { return 0 as libc::c_int }
    //	loadingScreenCallback();
    if bucketSetupList() == 0 {
        /* reset object list */
        return 0 as libc::c_int
    }
    /* Shift the interface initialisation here temporarily so that it
   		can pick up the stats after they have been loaded */
    //	loadingScreenCallback();
    if intInitialise() == 0 { return 0 as libc::c_int }
    //	loadingScreenCallback();
    //	if (!initTitle())
//	{
//		return(FALSE);
//	}
    if initMessage() == 0 {
        /* Initialise the message heaps */
        return 0 as libc::c_int
    }
    if gwInitialise() == 0 { return 0 as libc::c_int }
    // keymappings
    //	loadingScreenCallback();
    keyClearMappings();
    keyInitMappings(0 as libc::c_int);
    //	loadingScreenCallback();
    pie_SetMouse(IntImages,
                 IMAGE_CURSOR_DEFAULT as libc::c_int as
                     UWORD); // Set the default cursor shape.
    frameSetCursorFromRes(108 as libc::c_int as WORD);
    //	drawRadar3dfx(64,64);
//	iV_DownLoadRadar(radarBuffer3dfx);
    SetFormAudioIDs(ID_SOUND_WINDOWOPEN as libc::c_int,
                    ID_SOUND_WINDOWCLOSE as libc::c_int);
    //	mapNew(256,256);	// Generate the largest size of map needed for the game
//	if (!loadGame("final.gam"))
//	if (!loadGame("savetest.gam"))
//	{
//		return FALSE;
//	}
    //	intSetMapPos(43 << TILE_SHIFT, 43 << TILE_SHIFT);
    debug(LOG_MAIN,
          b"stageTwoInitialise: done\x00" as *const u8 as
              *const libc::c_char);
    return 1 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////////
// ////////////////////////////////////////////////////////////////////////////
// Free up after level specific data has been released but before base data is released
//
#[no_mangle]
pub unsafe extern "C" fn stageTwoShutDown() -> BOOL {
    debug(LOG_MAIN,
          b"stageTwoShutDown\x00" as *const u8 as *const libc::c_char);
    if war_GetPlayAudioCDs() != 0 { cdAudio_Stop(); }
    /* in stageThreeSgutDown now
	if (!missionShutDown())
	{
		return FALSE;
	}*/
    freeAllStructs();
    freeAllDroids();
    freeAllFeatures();
    freeAllFlagPositions();
    if messageShutdown() == 0 { return 0 as libc::c_int }
    if mechShutdown() == 0 { return 0 as libc::c_int }
    if ShutdownRadar() == 0 { return 0 as libc::c_int }
    intShutDown();
    cmdDroidShutDown();
    //free up the gateway stuff?
    gwShutDown();
    if mapShutdown() == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
/* ****************************************************************************/
/*      Initialise after all data is loaded                                  */
#[no_mangle]
pub unsafe extern "C" fn SetAllTilesVisible() {
    // Make all the tiles visible
    let mut psTile: *mut MAPTILE = psMapTiles;
    let mut i: UDWORD = 0;
    i = 0 as libc::c_int as UDWORD;
    while i < mapWidth.wrapping_mul(mapHeight) {
        (*psTile).tileVisBits =
            ((*psTile).tileVisBits as libc::c_int |
                 (1 as libc::c_int) << selectedPlayer) as UBYTE;
        psTile = psTile.offset(1);
        i = i.wrapping_add(1)
    };
}
/*

	Set all tiles within the scroll limits visible
	- this is for the playstation, so that at the end of each level everything is set to visible
	- this means that we don't have to save the visibilty area in the save game (this is good)
*/
#[no_mangle]
pub unsafe extern "C" fn SetScrollLimitsTilesVisible() {
    let mut psTile: *mut MAPTILE = 0 as *mut MAPTILE;
    let mut MapX: UWORD = 0;
    let mut MapY: UWORD = 0;
    MapY = scrollMinY as UWORD;
    while (MapY as libc::c_int) < scrollMaxY {
        psTile = mapTile(scrollMinX as UDWORD, MapY as UDWORD);
        MapX = scrollMinX as UWORD;
        while (MapX as libc::c_int) < scrollMaxX {
            (*psTile).tileVisBits =
                ((*psTile).tileVisBits as libc::c_int |
                     (1 as libc::c_int) << selectedPlayer) as UBYTE;
            psTile = psTile.offset(1);
            MapX = MapX.wrapping_add(1)
        }
        MapY = MapY.wrapping_add(1)
    };
}
// FromLoad flag used to indicate that this is the first call
// to stageThreeInitialise when loading a saved game so don't close the
// loading screen, onlyt relevant on PSX.
//
#[no_mangle]
pub unsafe extern "C" fn stageThreeInitialise() -> BOOL {
    //MAPTILE	*psTile;
//UDWORD	i,j;
    let mut psStr: *mut STRUCTURE = 0 as *mut STRUCTURE;
    debug(LOG_MAIN,
          b"stageThreeInitalise\x00" as *const u8 as *const libc::c_char);
    bTrackingTransporter = 0 as libc::c_int;
    loopMissionState = LMS_NORMAL;
    // reset the clock to normal speed
    gameTimeResetMod();
    if init3DView() == 0 {
        // Initialise 3d view stuff. After resLoad cause it needs the game palette initialised.
        return 0 as libc::c_int
    }
    effectResetUpdates();
    //initLighting();
    initLighting(0 as libc::c_int as UDWORD, 0 as libc::c_int as UDWORD,
                 mapWidth, mapHeight);
    if bMultiPlayer != 0 {
        // FIXME Is this really needed?
        debug(LOG_WZ,
              b"multiGameInit()\n\x00" as *const u8 as *const libc::c_char);
        multiGameInit();
        cmdDroidMultiExpBoost(1 as libc::c_int);
    } else {
        //ensure single player games do not have this set
        game.maxPlayers = 0 as libc::c_int as UBYTE
    } // reset the loading screen.
    preProcessVisibility();
    atmosInitSystem();
    closeLoadingScreen();
    if fpathInitialise() == 0 { return 0 as libc::c_int }
    clustInitialise();
    gridReset();
    //if mission screen is up, close it.
    if MissionResUp != 0 { intRemoveMissionResultNoAnim(); }
    //#ifndef PSX
//	if(bMultiPlayer)
//	{
//		multiGameInit();
//	}
//#endif
    // determine if to use radar
    psStr = apsStructLists[selectedPlayer as usize];
    while !psStr.is_null() {
        if (*(*psStr).pStructureType).type_0 ==
               REF_HQ as libc::c_int as libc::c_uint {
            radarOnScreen = 1 as libc::c_int;
            setHQExists(1 as libc::c_int, selectedPlayer);
            break ;
        } else { psStr = (*psStr).psNext }
    }
    // Re-inititialise some static variables.
    snapInitVars();
    driveInitVars(0 as libc::c_int);
    displayInitVars();
    setAllPauseStates(0 as libc::c_int);
    // ffs JS   (and its a global!)
    if getLevelLoadType() != GTYPE_SAVE_MIDMISSION as libc::c_int {
        eventFireCallbackTrigger(CALL_GAMEINIT as libc::c_int as
                                     TRIGGER_TYPE);
    }
    return 1 as libc::c_int;
}
/* ****************************************************************************/
/*      Shutdown before any data is released                                 */
#[no_mangle]
pub unsafe extern "C" fn stageThreeShutDown() -> BOOL {
    debug(LOG_MAIN,
          b"stageThreeShutDown\x00" as *const u8 as *const libc::c_char);
    // make sure any button tips are gone.
    widgReset(); // save options to registry (may have changed in game).
    audio_StopAll();
    saveConfig();
    if bMultiPlayer != 0 { multiGameShutdown(); }
    //#if 0
//	environShutDown();
//#endif
//	avCloseSystem();
    cmdDroidMultiExpBoost(0 as libc::c_int);
    eventReset();
    // reset the script values system
    scrvReset();
    //call this here before mission data is released
    if missionShutDown() == 0 { return 0 as libc::c_int }
    /*
		When this line wasn't at this point. The PSX version always failed on the next script after the tutorial ... unexplained why?
	*/
//	bInTutorial=FALSE;
    scrExternReset();
    //reset the run data so that doesn't need to be initialised in the scripts
    initRunData();
    // Remove any remaining enemy objects.
// Now done in mission state loop.
//	missionDestroyObjects();
    resetVTOLLandingPos();
    // Restore player colours since the scripts might of changed them.
    if bMultiPlayer == 0 {
        let mut temp: libc::c_int =
            getPlayerColour(selectedPlayer) as libc::c_int;
        initPlayerColours();
        setPlayerColour(selectedPlayer, temp as UDWORD);
    } else {
        initPlayerColours();
        // reset colours leaving multiplayer game.
    }
    setScriptWinLoseVideo(0 as libc::c_int as UBYTE);
    return 1 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////////
// ////////////////////////////////////////////////////////////////////////////
// Reset the state between expand maps or sub maps
//
#[no_mangle]
pub unsafe extern "C" fn gameReset() -> BOOL {
    debug(LOG_MAIN, b"gameReset\x00" as *const u8 as *const libc::c_char);
    return 1 as libc::c_int;
}
// Reset the game between campaigns
// Reset the game between campaigns
#[no_mangle]
pub unsafe extern "C" fn campaignReset() -> BOOL {
    debug(LOG_MAIN, b"campaignReset\x00" as *const u8 as *const libc::c_char);
    gwShutDown();
    mapShutdown();
    return 1 as libc::c_int;
}
// Reset the game when loading a save game
// Reset the game when loading a save game
#[no_mangle]
pub unsafe extern "C" fn saveGameReset() -> BOOL {
    //#ifdef MISSION_S
    debug(LOG_MAIN, b"saveGameReset\x00" as *const u8 as *const libc::c_char);
    if war_GetPlayAudioCDs() != 0 { cdAudio_Stop(); }
    /* in stageThreeSgutDown now
	if (!missionShutDown())
	{
		return FALSE;
	}*/
    freeAllStructs();
    freeAllDroids();
    freeAllFeatures();
    freeAllFlagPositions();
    //#ifdef NEW_SAVE added for V12 SAVE safe for all versions
    initMission();
    initTransporters();
    //#endif
	//free up the gateway stuff?
    gwShutDown();
    intResetScreen(1 as libc::c_int);
    intResetPreviousObj();
    if mapShutdown() == 0 { return 0 as libc::c_int }
    //clear out any messages
    freeMessages();
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn newMapInitialise() -> BOOL {
    debug(LOG_MAIN,
          b"newMapInitialise\x00" as *const u8 as *const libc::c_char);
    //NEW_SAVE removed for V11 Save removed for all versions
//	initViewPosition();
    // initialise the gateway stuff
//#ifndef PSX
	// this no longer necessary when RLE map zones are loaded
//	gwProcessMap();	// now loaded with map.
    // this is always necessary
/*	if (!gwLinkGateways())
	{
		return FALSE;
	}
*/
//#endif
    return 1 as libc::c_int;
}
// --- Miscellaneous Initialisation stuff that really should be done each restart
#[no_mangle]
pub unsafe extern "C" fn initMiscVars() {
    selectedPlayer = 0 as libc::c_int as UDWORD;
    godMode = 0 as libc::c_int;
    // ffs am
    radarOnScreen = 1 as libc::c_int;
    enableConsoleDisplay(1 as libc::c_int);
    setEnergyBarDisplay(1 as libc::c_int);
    setSelectedGroup(0xff as libc::c_int as UDWORD);
    processDebugMappings(0 as libc::c_int);
}
