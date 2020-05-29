use ::libc;
extern "C" {
    pub type _formation;
    /* *
 * \fn const char *PHYSFS_getLastError(void)
 * \brief Get human-readable error information.
 *
 * \deprecated Use PHYSFS_getLastErrorCode() and PHYSFS_getErrorByCode() instead.
 *
 * \warning As of PhysicsFS 2.1, this function has been nerfed.
 *          Before PhysicsFS 2.1, this function was the only way to get
 *          error details beyond a given function's basic return value.
 *          This was meant to be a human-readable string in one of several
 *          languages, and was not useful for application parsing. This was
 *          a problem, because the developer and not the user chose the
 *          language at compile time, and the PhysicsFS maintainers had
 *          to (poorly) maintain a significant amount of localization work.
 *          The app couldn't parse the strings, even if they counted on a
 *          specific language, since some were dynamically generated.
 *          In 2.1 and later, this always returns a static string in
 *          English; you may use it as a key string for your own
 *          localizations if you like, as we'll promise not to change
 *          existing error strings. Also, if your application wants to
 *          look at specific errors, we now offer a better option:
 *          use PHYSFS_getLastErrorCode() instead.
 *
 * Get the last PhysicsFS error message as a human-readable, null-terminated
 *  string. This will return NULL if there's been no error since the last call
 *  to this function. The pointer returned by this call points to an internal
 *  buffer. Each thread has a unique error state associated with it, but each
 *  time a new error message is set, it will overwrite the previous one
 *  associated with that thread. It is safe to call this function at anytime,
 *  even before PHYSFS_init().
 *
 * PHYSFS_getLastError() and PHYSFS_getLastErrorCode() both reset the same
 *  thread-specific error state. Calling one will wipe out the other's
 *  data. If you need both, call PHYSFS_getLastErrorCode(), then pass that
 *  value to PHYSFS_getErrorByCode().
 *
 * As of PhysicsFS 2.1, this function only presents text in the English
 *  language, but the strings are static, so you can use them as keys into
 *  your own localization dictionary. These strings are meant to be passed on
 *  directly to the user.
 *
 * Generally, applications should only concern themselves with whether a
 *  given function failed; however, if your code require more specifics, you
 *  should use PHYSFS_getLastErrorCode() instead of this function.
 *
 *   \return READ ONLY string of last error message.
 *
 * \sa PHYSFS_getLastErrorCode
 * \sa PHYSFS_getErrorByCode
 */
    #[no_mangle]
    fn PHYSFS_getLastError() -> *const libc::c_char;
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
    /* i/o stuff... */
    /* *
 * \fn PHYSFS_File *PHYSFS_openWrite(const char *filename)
 * \brief Open a file for writing.
 *
 * Open a file for writing, in platform-independent notation and in relation
 *  to the write dir as the root of the writable filesystem. The specified
 *  file is created if it doesn't exist. If it does exist, it is truncated to
 *  zero bytes, and the writing offset is set to the start.
 *
 * Note that entries that are symlinks are ignored if
 *  PHYSFS_permitSymbolicLinks(1) hasn't been called, and opening a
 *  symlink with this function will fail in such a case.
 *
 *   \param filename File to open.
 *  \return A valid PhysicsFS filehandle on success, NULL on error. Use
 *          PHYSFS_getLastErrorCode() to obtain the specific error.
 *
 * \sa PHYSFS_openRead
 * \sa PHYSFS_openAppend
 * \sa PHYSFS_write
 * \sa PHYSFS_close
 */
    #[no_mangle]
    fn PHYSFS_openWrite(filename: *const libc::c_char) -> *mut PHYSFS_File;
    /* *
 * \fn PHYSFS_File *PHYSFS_openRead(const char *filename)
 * \brief Open a file for reading.
 *
 * Open a file for reading, in platform-independent notation. The search path
 *  is checked one at a time until a matching file is found, in which case an
 *  abstract filehandle is associated with it, and reading may be done.
 *  The reading offset is set to the first byte of the file.
 *
 * Note that entries that are symlinks are ignored if
 *  PHYSFS_permitSymbolicLinks(1) hasn't been called, and opening a
 *  symlink with this function will fail in such a case.
 *
 *   \param filename File to open.
 *  \return A valid PhysicsFS filehandle on success, NULL on error.
 *          Use PHYSFS_getLastErrorCode() to obtain the specific error.
 *
 * \sa PHYSFS_openWrite
 * \sa PHYSFS_openAppend
 * \sa PHYSFS_read
 * \sa PHYSFS_close
 */
    #[no_mangle]
    fn PHYSFS_openRead(filename: *const libc::c_char) -> *mut PHYSFS_File;
    /* *
 * \fn int PHYSFS_close(PHYSFS_File *handle)
 * \brief Close a PhysicsFS filehandle.
 *
 * This call is capable of failing if the operating system was buffering
 *  writes to the physical media, and, now forced to write those changes to
 *  physical media, can not store the data for some reason. In such a case,
 *  the filehandle stays open. A well-written program should ALWAYS check the
 *  return value from the close call in addition to every writing call!
 *
 *   \param handle handle returned from PHYSFS_open*().
 *  \return nonzero on success, zero on error. Use PHYSFS_getLastErrorCode()
 *          to obtain the specific error.
 *
 * \sa PHYSFS_openRead
 * \sa PHYSFS_openWrite
 * \sa PHYSFS_openAppend
 */
    #[no_mangle]
    fn PHYSFS_close(handle: *mut PHYSFS_File) -> libc::c_int;
    /* *
 * \fn PHYSFS_sint64 PHYSFS_read(PHYSFS_File *handle, void *buffer, PHYSFS_uint32 objSize, PHYSFS_uint32 objCount)
 * \brief Read data from a PhysicsFS filehandle
 *
 * The file must be opened for reading.
 *
 * \deprecated As of PhysicsFS 2.1, use PHYSFS_readBytes() instead. This
 *             function just wraps it anyhow. This function never clarified
 *             what would happen if you managed to read a partial object, so
 *             working at the byte level makes this cleaner for everyone,
 *             especially now that PHYSFS_Io interfaces can be supplied by the
 *             application.
 *
 *   \param handle handle returned from PHYSFS_openRead().
 *   \param buffer buffer to store read data into.
 *   \param objSize size in bytes of objects being read from (handle).
 *   \param objCount number of (objSize) objects to read from (handle).
 *  \return number of objects read. PHYSFS_getLastErrorCode() can shed light
 *          on the reason this might be < (objCount), as can PHYSFS_eof().
 *          -1 if complete failure.
 *
 * \sa PHYSFS_readBytes
 * \sa PHYSFS_eof
 */
    #[no_mangle]
    fn PHYSFS_read(handle: *mut PHYSFS_File, buffer: *mut libc::c_void,
                   objSize: PHYSFS_uint32, objCount: PHYSFS_uint32)
     -> PHYSFS_sint64;
    /* *
 * \fn PHYSFS_sint64 PHYSFS_write(PHYSFS_File *handle, const void *buffer, PHYSFS_uint32 objSize, PHYSFS_uint32 objCount)
 * \brief Write data to a PhysicsFS filehandle
 *
 * The file must be opened for writing.
 *
 * \deprecated As of PhysicsFS 2.1, use PHYSFS_writeBytes() instead. This
 *             function just wraps it anyhow. This function never clarified
 *             what would happen if you managed to write a partial object, so
 *             working at the byte level makes this cleaner for everyone,
 *             especially now that PHYSFS_Io interfaces can be supplied by the
 *             application.
 *
 *   \param handle retval from PHYSFS_openWrite() or PHYSFS_openAppend().
 *   \param buffer buffer of bytes to write to (handle).
 *   \param objSize size in bytes of objects being written to (handle).
 *   \param objCount number of (objSize) objects to write to (handle).
 *  \return number of objects written. PHYSFS_getLastErrorCode() can shed
 *          light on the reason this might be < (objCount). -1 if complete
 *          failure.
 *
 * \sa PHYSFS_writeBytes
 */
    #[no_mangle]
    fn PHYSFS_write(handle: *mut PHYSFS_File, buffer: *const libc::c_void,
                    objSize: PHYSFS_uint32, objCount: PHYSFS_uint32)
     -> PHYSFS_sint64;
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
    fn memMallocRelease(Size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn memFreeRelease(pMemToFree: *mut libc::c_void);
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
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_uint)
     -> *mut libc::c_void;
    #[no_mangle]
    fn saveFile(pFileName: *const libc::c_char,
                pFileData: *const libc::c_char, fileSize: UDWORD) -> BOOL;
    #[no_mangle]
    fn loadFile(pFileName: *const libc::c_char,
                ppFileData: *mut *mut libc::c_char, pFileSize: *mut UDWORD)
     -> BOOL;
    /* The lists of objects allocated */
    #[no_mangle]
    static mut apsDroidLists: [*mut DROID; 8];
    /* add the droid to the Droid Lists */
    #[no_mangle]
    fn addDroid(psDroidToAdd: *mut DROID, pList: *mut *mut DROID);
    /*subtract the power required */
    #[no_mangle]
    fn usePower(player: UDWORD, quantity: UDWORD) -> BOOL;
    //return the power when a structure/droid is deliberately destroyed
    #[no_mangle]
    fn addPower(player: UDWORD, quantity: UDWORD);
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
    /* The range for neighbouring objects */
    //used to stop structures being built too near the edge and droids being placed down - pickATile
    /* Info stored for each droid neighbour */
    // The neighbouring object
    // The square of the distance to the object
    //UDWORD			dist;			// The distance to the object
    /* Store for the objects near the droid currently being updated */
    // the structure that was last hit
    // initialise droid module
    // refresh the naybor list
// this only does anything if the naybor list is out of date
    //extern BOOL loadDroidPrograms(car *pProgramData, UDWORD bufferSize);
    /*initialise the template build and power points */
    /*Builds an instance of a Structure - the x/y passed in are in world coords.*/
    /* Set the asBits in a DROID structure given it's template. */
    // calculate the experience level of a droid
    /* Calculate the weight of a droid from it's template */
    /* Calculate the power points required to build/maintain a droid */
    /* Calculate the body points of a droid from it's template */
    /* Calculate the base body points of a droid without upgrades*/
    /* Calculate the base speed of a droid from it's template */
    /* Calculate the speed of a droid over a terrain */
    /* Calculate the points required to build the template */
    /* Calculate the points required to build the droid */
//UDWORD calcDroidBuild(DROID *psDroid);
    /* Calculate the power points required to build/maintain the droid */
    #[no_mangle]
    fn calcTemplatePower(psTemplate: *mut DROID_TEMPLATE) -> UDWORD;
    #[no_mangle]
    fn zonedPAT(x: UDWORD, y: UDWORD) -> BOOL;
    #[no_mangle]
    fn pickATileGen(x: *mut UDWORD, y: *mut UDWORD, numIterations: UBYTE,
                    function:
                        Option<unsafe extern "C" fn(_: UDWORD, _: UDWORD)
                                   -> BOOL>) -> BOOL;
    #[no_mangle]
    fn buildDroid(pTemplate: *mut DROID_TEMPLATE, x: UDWORD, y: UDWORD,
                  player: UDWORD, onMission: BOOL) -> *mut DROID;
    #[no_mangle]
    fn calcTemplateBuild(psTemplate: *mut DROID_TEMPLATE) -> UDWORD;
    #[no_mangle]
    fn addEffect(pos: *mut iVector, group: EFFECT_GROUP, type_0: EFFECT_TYPE,
                 specified: BOOL, imd: *mut iIMDShape, lit: BOOL);
    // ////////////////////////////////////////////////////////////////////////
// variables
    #[no_mangle]
    static mut NetPlay: NETPLAY;
    // change a players name.
    #[no_mangle]
    fn NETgetLocalPlayerData(dpid: DPID, pData: *mut libc::c_void,
                             pSize: *mut DWORD) -> BOOL;
    #[no_mangle]
    fn NETgetGlobalPlayerData(dpid: DPID, pData: *mut libc::c_void,
                              pSize: *mut DWORD) -> BOOL;
    #[no_mangle]
    fn NETsetLocalPlayerData(dpid: DPID, pData: *mut libc::c_void,
                             size: DWORD) -> BOOL;
    #[no_mangle]
    fn NETsetGlobalPlayerData(dpid: DPID, pData: *mut libc::c_void,
                              size: DWORD) -> BOOL;
    // encryption
    #[no_mangle]
    fn NETsetKey(c1: UDWORD, c2: UDWORD, c3: UDWORD, c4: UDWORD) -> BOOL;
    #[no_mangle]
    fn NETmangleData(input: *mut libc::c_long, result: *mut libc::c_long,
                     dataSize: UDWORD) -> BOOL;
    #[no_mangle]
    fn NETunmangleData(input: *mut libc::c_long, result: *mut libc::c_long,
                       dataSize: UDWORD) -> BOOL;
    #[no_mangle]
    static mut ingame: MULTIPLAYERINGAME;
    #[no_mangle]
    static mut selectedPlayer: UDWORD;
    #[no_mangle]
    static mut player2dpid: [DWORD; 8];
    #[no_mangle]
    fn IdToTemplate(tempId: UDWORD, player: UDWORD) -> *mut DROID_TEMPLATE;
    #[no_mangle]
    fn NameToTemplate(sName: *mut CHAR, player: UDWORD)
     -> *mut DROID_TEMPLATE;
    #[no_mangle]
    fn isHumanPlayer(player: UDWORD) -> BOOL;
    #[no_mangle]
    fn sendTemplate(t: *mut DROID_TEMPLATE) -> BOOL;
    #[no_mangle]
    fn addTemplate(player: UDWORD, psNew: *mut DROID_TEMPLATE) -> BOOL;
    /*
 * MultiStat.c
 *
 * Alex Lee , pumpkin studios, EIDOS
 *
 * load / update / store multiplayer statistics for league tables etc...
 * Also handle the force save/loads and default teams for each tech level.
 */
    #[no_mangle]
    static mut MultiPlayersPath: [libc::c_char; 255];
}
pub type size_t = libc::c_uint;
/* *
 * \typedef PHYSFS_uint32
 * \brief An unsigned, 32-bit integer type.
 */
pub type PHYSFS_uint32 = libc::c_uint;
pub type PHYSFS_sint64 = libc::c_longlong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PHYSFS_File {
    pub opaque: *mut libc::c_void,
}
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
pub type SBYTE = libc::c_schar;
pub type STRING = libc::c_char;
pub type UWORD = libc::c_ushort;
pub type SWORD = libc::c_short;
pub type UDWORD = libc::c_uint;
pub type SDWORD = libc::c_int;
pub type BOOL = libc::c_int;
pub type CHAR = libc::c_char;
pub type DWORD = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct POINT {
    pub x: libc::c_int,
    pub y: libc::c_int,
}
/* !WIN32 */
pub type DPID = libc::c_int;
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
pub type int32 = libc::c_int;
pub type uint8 = libc::c_uchar;
pub type uint16 = libc::c_ushort;
pub type uint32 = libc::c_uint;
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
//UDWORD		dummy;
//	BOOL		drawnThisFrame;		// for sorting - have we drawn the imd already?
// last frame it was drawn
//	UDWORD		animFrame;			// anim Frame
//	SDWORD		bucketDepth;
//	BOOL		onScreen;
//	UDWORD		numTiles;
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
/* Only needed for Alex's movement update ? */
//	UDWORD	timeStarted;
//	UDWORD	arrivalTime;
//	UDWORD	pathStarted;
//	BOOL	startedMoving;
//	UDWORD	lastTime;
//	BOOL	speedChange;
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
pub type DROID_TYPE = _droid_type;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _component {
    pub nStat: UBYTE,
}
pub type COMPONENT = _component;
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
pub type DROID_TEMPLATE = _droid_template;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SAVEDPLAYERSTATS {
    pub name: [STRING; 255],
    pub stats: PLAYERSTATS,
    pub padding: [UBYTE; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _forcemember {
    pub pTempl: *mut DROID_TEMPLATE,
    pub psNext: *mut _forcemember,
}
pub type FORCE_MEMBER = _forcemember;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _force {
    pub pForceTemplates: *mut DROID_TEMPLATE,
    pub pMembers: *mut FORCE_MEMBER,
}
pub type FORCE = _force;
#[no_mangle]
pub static mut Force: FORCE =
    FORCE{pForceTemplates: 0 as *const DROID_TEMPLATE as *mut DROID_TEMPLATE,
          pMembers: 0 as *const FORCE_MEMBER as *mut FORCE_MEMBER,};
// position to retreat to
// number of units below which might run
// %health value below which to turn and run - FOR GROUPS ONLY
// basic chance to run
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
// pointer to template to use for this droid
// Pointer to next template
// remove a droid from force
// remove a droid from force
// ////////////////////////////////////////////////////////////////////////////
// FORCE SELECT STUFF
// ////////////////////////////////////////////////////////////////////////////
// ////////////////////////////////////////////////////////////////////////////
// funcs to edit the force.
#[no_mangle]
pub unsafe extern "C" fn addToForce(mut templ: *mut DROID_TEMPLATE) -> BOOL {
    let mut pF: *mut FORCE_MEMBER = 0 as *mut FORCE_MEMBER;
    let mut pT: *mut DROID_TEMPLATE = 0 as *mut DROID_TEMPLATE;
    let mut psTempl: *mut DROID_TEMPLATE = 0 as *mut DROID_TEMPLATE;
    if usePower(selectedPlayer, (*templ).powerPoints) == 0 {
        //subtract the power required to build
        return 0 as libc::c_int
        // go no further
    }
    // add template to list. if it doesn't exist.
    psTempl = Force.pForceTemplates; // find relevant template
    while !psTempl.is_null() && (*psTempl).ref_0 != (*templ).ref_0 {
        psTempl = (*psTempl).psNext
    }
    if psTempl.is_null() {
        pT =
            memMallocRelease(::std::mem::size_of::<DROID_TEMPLATE>() as
                                 libc::c_ulong) as *mut DROID_TEMPLATE;
        if pT.is_null() { return 0 as libc::c_int }
        memcpy(pT as *mut libc::c_void, templ as *const libc::c_void,
               ::std::mem::size_of::<DROID_TEMPLATE>() as libc::c_ulong);
        // calculate power etc for this template
//		initTemplatePoints();
        (*pT).buildPoints = calcTemplateBuild(pT);
        (*pT).powerPoints = calcTemplatePower(pT);
        (*pT).psNext = Force.pForceTemplates;
        Force.pForceTemplates = pT
    } else {
        pT = psTempl
        // set up to point to existing template.
    }
    // add droid.
    pF =
        memMallocRelease(::std::mem::size_of::<FORCE_MEMBER>() as
                             libc::c_ulong) as
            *mut FORCE_MEMBER; // create a slot in the force.
    if pF.is_null() { return 0 as libc::c_int } // add this droid.
    (*pF).pTempl = pT;
    (*pF).psNext = Force.pMembers;
    Force.pMembers = pF;
    return 1 as libc::c_int;
}
// the selected force.
//  Force defs.
// ////////////////////////////////////////////////////////////////////////////
//  Force defs.
// ////////////////////////////////////////////////////////////////////////////
#[no_mangle]
pub unsafe extern "C" fn removeFromForce(mut number: UDWORD) -> BOOL {
    let mut i: UDWORD = 0; // goto that force element;
    let mut templateid: UDWORD = 0; // return that much power.
    let mut pF: *mut FORCE_MEMBER = 0 as *mut FORCE_MEMBER;
    let mut pF2: *mut FORCE_MEMBER = 0 as *mut FORCE_MEMBER;
    let mut psCurr: *mut DROID_TEMPLATE = 0 as *mut DROID_TEMPLATE;
    let mut psPrev: *mut DROID_TEMPLATE = 0 as *mut DROID_TEMPLATE;
    let mut inuse: BOOL = 0;
    pF = Force.pMembers;
    i = 0 as libc::c_int as UDWORD;
    while i < number { pF = (*pF).psNext; i = i.wrapping_add(1) }
    addPower(selectedPlayer, (*(*pF).pTempl).powerPoints);
    if number == 0 as libc::c_int as libc::c_uint {
        // if first remove it,
        pF = Force.pMembers; // not first so linked list
        Force.pMembers = (*pF).psNext
    } else {
        pF = Force.pMembers;
        i = 0 as libc::c_int as UDWORD;
        while i < number {
            // needs modifying..
            pF2 = pF;
            pF = (*pF).psNext;
            i = i.wrapping_add(1)
        }
        (*pF2).psNext = (*pF).psNext
    }
    templateid = (*(*pF).pTempl).ref_0;
    memFreeRelease(pF as *mut libc::c_void);
    pF = 0 as *mut FORCE_MEMBER;
    // now check if template is still in use.
    inuse = 0 as libc::c_int;
    pF = Force.pMembers;
    while !pF.is_null() {
        if (*(*pF).pTempl).ref_0 == templateid { inuse = 1 as libc::c_int }
        pF = (*pF).psNext
    }
    if inuse == 0 {
        // remove template, no longer needed.
        psPrev = 0 as *mut DROID_TEMPLATE;
        psCurr = Force.pForceTemplates;
        while !psCurr.is_null() {
            if (*psCurr).ref_0 == templateid { break ; }
            psPrev = psCurr;
            psCurr = (*psCurr).psNext
        }
        if !psCurr.is_null() {
            // if we found itthen delete it.
            if !psPrev.is_null() {
                // Update list pointers.
                (*psPrev).psNext = (*psCurr).psNext
                // It's down the list somewhere
            } else {
                Force.pForceTemplates = (*psCurr).psNext
                // It's at the root
            }
            memFreeRelease(psCurr as *mut libc::c_void);
            psCurr = 0 as *mut DROID_TEMPLATE
            // Delete the template.
        }
    }
    return 1 as libc::c_int;
}
// add a droid (templ) to force
// add a droid (templ) to force
/*
// find a place for the force
VOID chooseForceLoc(UDWORD *pX,UDWORD *pY)
{
	FEATURE			*pFeat;
	UDWORD			x,y,chose,tcount=0;

	// pick a boulder on the arena map
	for(pFeat=apsFeatureLists[0];pFeat;pFeat=pFeat->psNext)//count boulders
	{
		if(pFeat->psStats->subType == FEAT_BOULDER)
		{
			tcount++;
		}
	}

	if(tcount)												// use boulders
	{
		chose = 0;
		while(chose == 0)									// dont pick zeroth boulder.
		{
			chose  = (rand()%(tcount+1));
		}

		tcount= 0;
		pFeat=apsFeatureLists[0];
		while(pFeat && (tcount!=chose))
		{
			if(pFeat->psStats->subType == FEAT_BOULDER)
			{
				tcount++;
			}
			if(tcount != chose)
			{
				pFeat = pFeat->psNext;
			}
		}
		x = pFeat->x >>TILE_SHIFT;
		y = pFeat->y >>TILE_SHIFT;
	}
	else													//dont use boulders
	{
		tcount = 0;
		x = rand()%mapWidth;								//choose an intitial x/y pos.
		y = rand()%mapHeight;
	}

	// set result
	*pX = x;
	*pY = y;
}
*/
// ////////////////////////////////////////////////////////////////////////////
// place the force on the map.
#[no_mangle]
pub unsafe extern "C" fn useTheForce(mut bAddTempl: BOOL) 
 //Luke
 {
    let mut pDr: *mut DROID = 0 as *mut DROID;
    let mut pTempl: *mut DROID_TEMPLATE = 0 as *mut DROID_TEMPLATE;
    let mut position: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut x1: UDWORD = 0;
    let mut y1: UDWORD = 0;
    let mut x: UDWORD = 0;
    let mut y: UDWORD = 0;
    let mut psTempl: *mut DROID_TEMPLATE = 0 as *mut DROID_TEMPLATE;
    //	if(game.type == DMATCH)
//	{
//		chooseForceLoc(&x,&y);
//	}
//	if(game.type == CAMPAIGN)
//	{
    if !apsDroidLists[selectedPlayer as usize].is_null() {
        //set drop off point to pos of other droids.
        x =
            ((*apsDroidLists[selectedPlayer as usize]).x as libc::c_int >>
                 7 as libc::c_int) as UDWORD;
        y =
            ((*apsDroidLists[selectedPlayer as usize]).y as libc::c_int >>
                 7 as libc::c_int) as UDWORD;
        // send each of the extra templates used in the force
        psTempl = Force.pForceTemplates;
        while !psTempl.is_null() {
            sendTemplate(psTempl);
            psTempl = (*psTempl).psNext
            // OUCH! REALLY COM HEAVY
        }
    }
    //	}
    x1 = x; // now we have a coord, place droids
    y1 = y;
    while !Force.pMembers.is_null() {
        // for each force member
        x = x1;
        y = y1;
        if pickATileGen(&mut x, &mut y, 20 as libc::c_int as UBYTE,
                        Some(zonedPAT as
                                 unsafe extern "C" fn(_: UDWORD, _: UDWORD)
                                     -> BOOL)) == 0 {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"UseTheForce: Unable to find a free location\x00" as
                          *const u8 as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"multistat.c\x00" as *const u8 as *const libc::c_char,
                      276 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 12],
                                                &[libc::c_char; 12]>(b"useTheForce\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
        }
        debug(LOG_NEVER,
              b"force droid dropping at :%d,%d\n\x00" as *const u8 as
                  *const libc::c_char, x, y);
        //		}
        psTempl =
            NameToTemplate((*(*Force.pMembers).pTempl).aName.as_mut_ptr(),
                           selectedPlayer);
        if psTempl.is_null() {
            (*(*Force.pMembers).pTempl).ref_0 =
                0xc0000 as libc::c_int as UDWORD;
            addTemplate(selectedPlayer, (*Force.pMembers).pTempl);
        }
        pTempl = (*Force.pMembers).pTempl;
        pDr =
            buildDroid(pTempl, x << 7 as libc::c_int, y << 7 as libc::c_int,
                       selectedPlayer, 0 as libc::c_int);
        removeFromForce(0 as libc::c_int as UDWORD);
        if !pDr.is_null() {
            // copy template
            // end of template copy
            /*		if(!psTempl)												// already exists.
		{
			if (HEAP_ALLOC(psTemplateHeap, &psTempl))
			{
				memcpy(psTempl, Force.pMembers->pTempl, sizeof(DROID_TEMPLATE));
				psTempl->ref = REF_TEMPLATE_START;					// templates are the odd one out!
				if (apsDroidTemplates[selectedPlayer])						// Add it to the list
				{
					for(psCurr = apsDroidTemplates[selectedPlayer];
						psCurr->psNext != NULL;
						psCurr = psCurr->psNext
						);
					psCurr->psNext = psTempl;
					psTempl->psNext = NULL;
				}
			}
		}
		else
		{
			apsDroidTemplates[selectedPlayer]=psTempl;
			psTempl->psNext = NULL;
		}
*/
            //		if(usePower(selectedPlayer, pTempl->powerPoints))
//		{
            // remove from force (to free power)
            addDroid(pDr, apsDroidLists.as_mut_ptr()); // add it to the world.
            position.x = (*pDr).x as int32; // Add an effect
            position.z = (*pDr).y as int32;
            position.y = (*pDr).z as int32;
            addEffect(&mut position, EFFECT_EXPLOSION,
                      EXPLOSION_TYPE_DISCOVERY, 0 as libc::c_int,
                      0 as *mut iIMDShape, 0 as libc::c_int);
        }
    };
}
// place the force in the game
// place the force in the game
// ////////////////////////////////////////////////////////////////////////////
// save out force to a file.
// file format is as follows.
//  number of templates
//  number of droids in force
//  templates
//  droids
#[no_mangle]
pub unsafe extern "C" fn saveForce(mut name: *mut libc::c_char,
                                   mut pfForce: *mut FORCE) -> BOOL {
    let mut fileName: [STRING; 255] =
        *::std::mem::transmute::<&[u8; 255],
                                 &mut [STRING; 255]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"); // open the file
    let mut pFileHandle: *mut PHYSFS_File = 0 as *mut PHYSFS_File;
    let mut pT: *mut DROID_TEMPLATE = 0 as *mut DROID_TEMPLATE;
    let mut pCount: *mut FORCE_MEMBER = 0 as *mut FORCE_MEMBER;
    let mut pMember: *mut FORCE_MEMBER = 0 as *mut FORCE_MEMBER;
    let mut count: UDWORD = 0 as libc::c_int as UDWORD;
    strcpy(fileName.as_mut_ptr(), name);
    pFileHandle = PHYSFS_openWrite(fileName.as_mut_ptr());
    if pFileHandle.is_null() {
        debug(LOG_ERROR,
              b"saveForce: Couldn\'t open %s: %s\x00" as *const u8 as
                  *const libc::c_char, fileName.as_mut_ptr(),
              PHYSFS_getLastError());
        return 0 as libc::c_int
    }
    // save header for force file.
    count = 0 as libc::c_int as UDWORD; // count templates
    pT = (*pfForce).pForceTemplates; // count droids
    while !pT.is_null() { count = count.wrapping_add(1); pT = (*pT).psNext }
    if PHYSFS_write(pFileHandle,
                    &mut count as *mut UDWORD as *const libc::c_void,
                    ::std::mem::size_of::<UDWORD>() as libc::c_ulong,
                    1 as libc::c_int as PHYSFS_uint32) !=
           1 as libc::c_int as libc::c_longlong {
        debug(LOG_ERROR,
              b"saveForce: Write failed for %s: %s\x00" as *const u8 as
                  *const libc::c_char, fileName.as_mut_ptr(),
              PHYSFS_getLastError());
        return 0 as libc::c_int
    }
    count = 0 as libc::c_int as UDWORD;
    pCount = (*pfForce).pMembers;
    while !pCount.is_null() {
        count = count.wrapping_add(1);
        pCount = (*pCount).psNext
    }
    if PHYSFS_write(pFileHandle,
                    &mut count as *mut UDWORD as *const libc::c_void,
                    ::std::mem::size_of::<UDWORD>() as libc::c_ulong,
                    1 as libc::c_int as PHYSFS_uint32) !=
           1 as libc::c_int as libc::c_longlong {
        debug(LOG_ERROR,
              b"saveForce: Write failed for %s: %s\x00" as *const u8 as
                  *const libc::c_char, fileName.as_mut_ptr(),
              PHYSFS_getLastError());
        return 0 as libc::c_int
    }
    // new method. refs to templates. USED FOR MULITLANG SUPP.
    pMember = (*pfForce).pMembers;
    while !pMember.is_null() {
        if PHYSFS_write(pFileHandle,
                        &mut (*(*pMember).pTempl).multiPlayerID as *mut UDWORD
                            as *const libc::c_void,
                        ::std::mem::size_of::<UDWORD>() as libc::c_ulong,
                        1 as libc::c_int as PHYSFS_uint32) !=
               1 as libc::c_int as libc::c_longlong {
            debug(LOG_ERROR,
                  b"saveForce: Write failed for %s: %s\x00" as *const u8 as
                      *const libc::c_char, fileName.as_mut_ptr(),
                  PHYSFS_getLastError());
            return 0 as libc::c_int
        }
        pMember = (*pMember).psNext
    }
    if PHYSFS_close(pFileHandle) == 0 {
        debug(LOG_ERROR,
              b"saveForce: Close failed for %s: %s\x00" as *const u8 as
                  *const libc::c_char, fileName.as_mut_ptr(),
              PHYSFS_getLastError());
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////////
// load a force from a file.
#[no_mangle]
pub unsafe extern "C" fn loadForce(mut name: *mut libc::c_char) -> BOOL {
    let mut fileName: [STRING; 255] =
        *::std::mem::transmute::<&[u8; 255],
                                 &mut [STRING; 255]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
    //	STRING			tname[255]="";
    let mut pFileHandle: *mut PHYSFS_File =
        0 as *mut PHYSFS_File; // check file exists
    let mut tcount: UDWORD = 0;
    let mut fcount: UDWORD = 0 as libc::c_int as UDWORD;
    let mut ref_0: UDWORD = 0;
    let mut psTempl: *mut DROID_TEMPLATE = 0 as *mut DROID_TEMPLATE;
    strcpy(fileName.as_mut_ptr(), name);
    debug(LOG_WZ, b"loadForce: %s\x00" as *const u8 as *const libc::c_char,
          fileName.as_mut_ptr());
    pFileHandle = PHYSFS_openRead(fileName.as_mut_ptr());
    if pFileHandle.is_null() {
        debug(LOG_ERROR,
              b"loadForce: Failed to open %s: %s\x00" as *const u8 as
                  *const libc::c_char, fileName.as_mut_ptr(),
              PHYSFS_getLastError());
        return 0 as libc::c_int
        // failed
    }
    while !Force.pMembers.is_null() {
        // clear current force
        removeFromForce(0 as libc::c_int as UDWORD);
    }
    //	DBERROR(("tem %d   :   mem %d",Force.pForceTemplates ,	Force.pMembers));
    Force.pForceTemplates = 0 as *mut DROID_TEMPLATE;
    Force.pMembers = 0 as *mut FORCE_MEMBER;
    // load in new force.
    if PHYSFS_read(pFileHandle,
                   &mut tcount as *mut UDWORD as *mut libc::c_void,
                   ::std::mem::size_of::<UDWORD>() as libc::c_ulong,
                   1 as libc::c_int as PHYSFS_uint32) !=
           1 as libc::c_int as libc::c_longlong {
        // get number of templates
        debug(LOG_ERROR,
              b"loadForce: Read failed for %s: %s\x00" as *const u8 as
                  *const libc::c_char, fileName.as_mut_ptr(),
              PHYSFS_getLastError());
        PHYSFS_close(pFileHandle);
        return 0 as libc::c_int
    }
    if PHYSFS_read(pFileHandle,
                   &mut fcount as *mut UDWORD as *mut libc::c_void,
                   ::std::mem::size_of::<UDWORD>() as libc::c_ulong,
                   1 as libc::c_int as PHYSFS_uint32) !=
           1 as libc::c_int as libc::c_longlong {
        // get number of droids in force
        debug(LOG_ERROR,
              b"loadForce: read failed for %s: %s\x00" as *const u8 as
                  *const libc::c_char, fileName.as_mut_ptr(),
              PHYSFS_getLastError());
        PHYSFS_close(pFileHandle);
        return 0 as libc::c_int
    }
    // new method.
	// get forces.
    while fcount > 0 as libc::c_int as libc::c_uint {
        if PHYSFS_read(pFileHandle,
                       &mut ref_0 as *mut UDWORD as *mut libc::c_void,
                       ::std::mem::size_of::<UDWORD>() as libc::c_ulong,
                       1 as libc::c_int as PHYSFS_uint32) !=
               1 as libc::c_int as libc::c_longlong {
            // read in a template ref code.
            debug(LOG_ERROR,
                  b"loadForce: read failed for %s: %s\x00" as *const u8 as
                      *const libc::c_char, fileName.as_mut_ptr(),
                  PHYSFS_getLastError());
            PHYSFS_close(pFileHandle);
            return 0 as libc::c_int
        }
        psTempl = IdToTemplate(ref_0, 4 as libc::c_int as UDWORD);
        if !psTempl.is_null() {
            addToForce(psTempl);
            // add it to the force.
        }
        fcount = fcount.wrapping_sub(1)
    }
    if PHYSFS_close(pFileHandle) == 0 {
        debug(LOG_ERROR,
              b"loadForce: Close failed for %s: %s\x00" as *const u8 as
                  *const libc::c_char, fileName.as_mut_ptr(),
              PHYSFS_getLastError());
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
// form disk
// ////////////////////////////////////////////////////////////////////////////
// STATS STUFF
// ////////////////////////////////////////////////////////////////////////////
// ////////////////////////////////////////////////////////////////////////////
// Get Player's stats
#[no_mangle]
pub unsafe extern "C" fn getMultiStats(mut player: UDWORD, mut bLocal: BOOL)
 -> PLAYERSTATS {
    static mut stat: PLAYERSTATS =
        PLAYERSTATS{played: 0,
                    wins: 0,
                    loses: 0,
                    totalKills: 0,
                    totalScore: 0,
                    recentKills: 0,
                    recentScore: 0,
                    killsToAdd: 0,
                    scoreToAdd: 0,};
    let mut statSize: DWORD =
        ::std::mem::size_of::<PLAYERSTATS>() as libc::c_ulong as DWORD;
    let mut playerDPID: DPID = 0;
    playerDPID = player2dpid[player as usize];
    if bLocal != 0 {
        NETgetLocalPlayerData(playerDPID,
                              &mut stat as *mut PLAYERSTATS as
                                  *mut libc::c_void, &mut statSize);
    } else {
        NETgetGlobalPlayerData(playerDPID,
                               &mut stat as *mut PLAYERSTATS as
                                   *mut libc::c_void, &mut statSize);
    }
    return stat;
}
// get from net
// ////////////////////////////////////////////////////////////////////////////
// Set Player's stats
#[no_mangle]
pub unsafe extern "C" fn setMultiStats(mut dp: DWORD,
                                       mut plStats: PLAYERSTATS,
                                       mut bLocal: BOOL) -> BOOL {
    let mut playerDPID: DPID = dp;
    if bLocal != 0 {
        NETsetLocalPlayerData(playerDPID,
                              &mut plStats as *mut PLAYERSTATS as
                                  *mut libc::c_void,
                              ::std::mem::size_of::<PLAYERSTATS>() as
                                  libc::c_ulong as DWORD);
    } else {
        NETsetGlobalPlayerData(playerDPID,
                               &mut plStats as *mut PLAYERSTATS as
                                   *mut libc::c_void,
                               ::std::mem::size_of::<PLAYERSTATS>() as
                                   libc::c_ulong as DWORD);
    }
    return 1 as libc::c_int;
}
// to disk 
// ////////////////////////////////////////////////////////////////////////////
// Load Player Stats
#[no_mangle]
pub unsafe extern "C" fn loadMultiStats(mut sPlayerName: *mut STRING,
                                        mut playerStats: *mut PLAYERSTATS)
 -> BOOL {
    let mut fileName: [STRING; 255] =
        *::std::mem::transmute::<&[u8; 255],
                                 &mut [STRING; 255]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
    let mut size: UDWORD = 0;
    let mut pFileData: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut blankstats: PLAYERSTATS =
        {
            let mut init =
                PLAYERSTATS{played: 0 as libc::c_int,
                            wins: 0,
                            loses: 0,
                            totalKills: 0,
                            totalScore: 0,
                            recentKills: 0,
                            recentScore: 0,
                            killsToAdd: 0,
                            scoreToAdd: 0,};
            init
        };
    let mut st: SAVEDPLAYERSTATS =
        SAVEDPLAYERSTATS{name: [0; 255],
                         stats:
                             PLAYERSTATS{played: 0,
                                         wins: 0,
                                         loses: 0,
                                         totalKills: 0,
                                         totalScore: 0,
                                         recentKills: 0,
                                         recentScore: 0,
                                         killsToAdd: 0,
                                         scoreToAdd: 0,},
                         padding: [0; 4],};
    let mut codedst: *mut SAVEDPLAYERSTATS = 0 as *mut SAVEDPLAYERSTATS;
    let mut tmp: [UDWORD; 4] = [0; 4];
    strcpy(fileName.as_mut_ptr(), MultiPlayersPath.as_mut_ptr());
    strcat(fileName.as_mut_ptr(), sPlayerName);
    strcat(fileName.as_mut_ptr(),
           b".sta\x00" as *const u8 as *const libc::c_char);
    debug(LOG_WZ,
          b"loadMultiStats: %s\x00" as *const u8 as *const libc::c_char,
          fileName.as_mut_ptr());
    // check player already exists
	// FIXME: integrate with physfs stuff, and add basic sanity
    if PHYSFS_exists(fileName.as_mut_ptr()) == 0 {
        saveMultiStats(sPlayerName, sPlayerName, &mut blankstats);
        // didnt exist so create.
    }
    loadFile(fileName.as_mut_ptr(), &mut pFileData, &mut size);
    codedst = pFileData as *mut SAVEDPLAYERSTATS;
    //decode packet;
    memcpy(&mut tmp as *mut [UDWORD; 4] as *mut libc::c_void,
           &mut NetPlay.cryptKey as *mut [UDWORD; 4] as *const libc::c_void,
           ::std::mem::size_of::<[UDWORD; 4]>() as libc::c_ulong);
    NETsetKey(11974 as libc::c_int as UDWORD, 224351 as libc::c_int as UDWORD,
              2023901 as libc::c_int as UDWORD,
              21080 as libc::c_int as UDWORD);
    NETunmangleData(codedst as *mut libc::c_long,
                    &mut st as *mut SAVEDPLAYERSTATS as *mut libc::c_long,
                    ::std::mem::size_of::<SAVEDPLAYERSTATS>() as
                        libc::c_ulong);
    NETsetKey(tmp[0 as libc::c_int as usize], tmp[1 as libc::c_int as usize],
              tmp[2 as libc::c_int as usize], tmp[3 as libc::c_int as usize]);
    //set stats.
    memcpy(playerStats as *mut libc::c_void,
           &mut st.stats as *mut PLAYERSTATS as *const libc::c_void,
           ::std::mem::size_of::<PLAYERSTATS>() as libc::c_ulong); // get
    //set the name. ASSUME STRING IS LONG ENOUGH!
    strcpy(sPlayerName, st.name.as_mut_ptr());
    memFreeRelease(pFileData as *mut libc::c_void);
    pFileData = 0 as *mut libc::c_char;
    // reset recent scores
    (*playerStats).recentKills = 0 as libc::c_int;
    (*playerStats).recentScore = 0 as libc::c_int;
    (*playerStats).killsToAdd = 0 as libc::c_int;
    (*playerStats).scoreToAdd = 0 as libc::c_int;
    // clear any skirmish stats.
    size = 0 as libc::c_int as UDWORD;
    while size < 8 as libc::c_int as libc::c_uint {
        ingame.skScores[size as usize][0 as libc::c_int as usize] =
            0 as libc::c_int as UDWORD;
        ingame.skScores[size as usize][1 as libc::c_int as usize] =
            0 as libc::c_int as UDWORD;
        size = size.wrapping_add(1)
    }
    return 1 as libc::c_int;
}
// stat defs
// the selected force.
// ////////////////////////////////////////////////////////////////////////////
//  Player Stat defs.
// ////////////////////////////////////////////////////////////////////////////
// Save Player Stats
#[no_mangle]
pub unsafe extern "C" fn saveMultiStats(mut sFileName: *mut STRING,
                                        mut sPlayerName: *mut STRING,
                                        mut playerStats: *mut PLAYERSTATS)
 -> BOOL {
    let mut fileName: [STRING; 255] =
        *::std::mem::transmute::<&[u8; 255],
                                 &mut [STRING; 255]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
    let mut codedst: SAVEDPLAYERSTATS =
        SAVEDPLAYERSTATS{name: [0; 255],
                         stats:
                             PLAYERSTATS{played: 0,
                                         wins: 0,
                                         loses: 0,
                                         totalKills: 0,
                                         totalScore: 0,
                                         recentKills: 0,
                                         recentScore: 0,
                                         killsToAdd: 0,
                                         scoreToAdd: 0,},
                         padding: [0; 4],};
    let mut st: SAVEDPLAYERSTATS =
        SAVEDPLAYERSTATS{name: [0; 255],
                         stats:
                             PLAYERSTATS{played: 0,
                                         wins: 0,
                                         loses: 0,
                                         totalKills: 0,
                                         totalScore: 0,
                                         recentKills: 0,
                                         recentScore: 0,
                                         killsToAdd: 0,
                                         scoreToAdd: 0,},
                         padding: [0; 4],};
    let mut tmp: [UDWORD; 4] = [0; 4];
    // prepare file.
    memcpy(&mut st.stats as *mut PLAYERSTATS as *mut libc::c_void,
           playerStats as *const libc::c_void,
           ::std::mem::size_of::<PLAYERSTATS>() as libc::c_ulong);
    memset(st.name.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           255 as libc::c_int as libc::c_uint);
    memset(st.padding.as_mut_ptr() as *mut libc::c_void, 1 as libc::c_int,
           4 as libc::c_int as libc::c_uint);
    strcpy(st.name.as_mut_ptr(), sPlayerName);
    //encode packet;
    memcpy(&mut tmp as *mut [UDWORD; 4] as *mut libc::c_void,
           &mut NetPlay.cryptKey as *mut [UDWORD; 4] as *const libc::c_void,
           ::std::mem::size_of::<[UDWORD; 4]>() as libc::c_ulong);
    NETsetKey(11974 as libc::c_int as UDWORD, 224351 as libc::c_int as UDWORD,
              2023901 as libc::c_int as UDWORD,
              21080 as libc::c_int as UDWORD);
    NETmangleData(&mut st as *mut SAVEDPLAYERSTATS as *mut libc::c_long,
                  &mut codedst as *mut SAVEDPLAYERSTATS as *mut libc::c_long,
                  ::std::mem::size_of::<SAVEDPLAYERSTATS>() as libc::c_ulong);
    NETsetKey(tmp[0 as libc::c_int as usize], tmp[1 as libc::c_int as usize],
              tmp[2 as libc::c_int as usize], tmp[3 as libc::c_int as usize]);
    strcpy(fileName.as_mut_ptr(), MultiPlayersPath.as_mut_ptr());
    strcat(fileName.as_mut_ptr(), sFileName);
    strcat(fileName.as_mut_ptr(),
           b".sta\x00" as *const u8 as *const libc::c_char);
    // FIXME: ugly cast
    saveFile(fileName.as_mut_ptr(),
             &mut codedst as *mut SAVEDPLAYERSTATS as *mut libc::c_char,
             ::std::mem::size_of::<SAVEDPLAYERSTATS>() as libc::c_ulong);
    return 1 as libc::c_int;
}
// send to net.
// ////////////////////////////////////////////////////////////////////////////
// score update functions
// update players damage stats.
#[no_mangle]
pub unsafe extern "C" fn updateMultiStatsDamage(mut attacker: UDWORD,
                                                mut defender: UDWORD,
                                                mut inflicted: UDWORD) {
    let mut st: PLAYERSTATS =
        PLAYERSTATS{played: 0,
                    wins: 0,
                    loses: 0,
                    totalKills: 0,
                    totalScore: 0,
                    recentKills: 0,
                    recentScore: 0,
                    killsToAdd: 0,
                    scoreToAdd: 0,}; // get stats
    if isHumanPlayer(attacker) != 0 {
        st = getMultiStats(attacker, 1 as libc::c_int);
        if NetPlay.bComms != 0 {
            st.scoreToAdd =
                (st.scoreToAdd as
                     libc::c_uint).wrapping_add((2 as libc::c_int as
                                                     libc::c_uint).wrapping_mul(inflicted))
                    as SDWORD as SDWORD
        } else {
            st.recentScore =
                (st.recentScore as
                     libc::c_uint).wrapping_add((2 as libc::c_int as
                                                     libc::c_uint).wrapping_mul(inflicted))
                    as SDWORD as SDWORD
        }
        setMultiStats(player2dpid[attacker as usize], st, 1 as libc::c_int);
    } else {
        ingame.skScores[attacker as usize][0 as libc::c_int as usize] =
            (ingame.skScores[attacker as usize][0 as libc::c_int as usize] as
                 libc::c_uint).wrapping_add((2 as libc::c_int as
                                                 libc::c_uint).wrapping_mul(inflicted))
                as UDWORD as UDWORD
        // increment skirmish players rough score.
    } // get stats
    if isHumanPlayer(defender) != 0 {
        st = getMultiStats(defender, 1 as libc::c_int);
        if NetPlay.bComms != 0 {
            st.scoreToAdd =
                (st.scoreToAdd as libc::c_uint).wrapping_sub(inflicted) as
                    SDWORD as SDWORD
        } else {
            st.recentScore =
                (st.recentScore as libc::c_uint).wrapping_sub(inflicted) as
                    SDWORD as SDWORD
        }
        setMultiStats(player2dpid[defender as usize], st, 1 as libc::c_int);
    } else {
        ingame.skScores[defender as usize][0 as libc::c_int as usize] =
            (ingame.skScores[defender as usize][0 as libc::c_int as usize] as
                 libc::c_uint).wrapping_sub(inflicted) as UDWORD as UDWORD
        // increment skirmish players rough score.
    };
}
// update games played.
#[no_mangle]
pub unsafe extern "C" fn updateMultiStatsGames() {
    let mut st: PLAYERSTATS =
        PLAYERSTATS{played: 0,
                    wins: 0,
                    loses: 0,
                    totalKills: 0,
                    totalScore: 0,
                    recentKills: 0,
                    recentScore: 0,
                    killsToAdd: 0,
                    scoreToAdd: 0,};
    st = getMultiStats(selectedPlayer, 1 as libc::c_int);
    st.played += 1;
    setMultiStats(player2dpid[selectedPlayer as usize], st, 1 as libc::c_int);
}
// games won
#[no_mangle]
pub unsafe extern "C" fn updateMultiStatsWins() {
    let mut st: PLAYERSTATS =
        PLAYERSTATS{played: 0,
                    wins: 0,
                    loses: 0,
                    totalKills: 0,
                    totalScore: 0,
                    recentKills: 0,
                    recentScore: 0,
                    killsToAdd: 0,
                    scoreToAdd: 0,};
    st = getMultiStats(selectedPlayer, 1 as libc::c_int);
    st.wins += 1;
    setMultiStats(player2dpid[selectedPlayer as usize], st, 1 as libc::c_int);
}
//games lost.
#[no_mangle]
pub unsafe extern "C" fn updateMultiStatsLoses() {
    let mut st: PLAYERSTATS =
        PLAYERSTATS{played: 0,
                    wins: 0,
                    loses: 0,
                    totalKills: 0,
                    totalScore: 0,
                    recentKills: 0,
                    recentScore: 0,
                    killsToAdd: 0,
                    scoreToAdd: 0,};
    st = getMultiStats(selectedPlayer, 1 as libc::c_int);
    st.loses += 1;
    setMultiStats(player2dpid[selectedPlayer as usize], st, 1 as libc::c_int);
}
// update kills
#[no_mangle]
pub unsafe extern "C" fn updateMultiStatsKills(mut psKilled: *mut BASE_OBJECT,
                                               mut player: UDWORD) {
    let mut st: PLAYERSTATS =
        PLAYERSTATS{played: 0,
                    wins: 0,
                    loses: 0,
                    totalKills: 0,
                    totalScore: 0,
                    recentKills: 0,
                    recentScore: 0,
                    killsToAdd: 0,
                    scoreToAdd: 0,};
    if isHumanPlayer(player) != 0 {
        st = getMultiStats(player, 1 as libc::c_int);
        if NetPlay.bComms != 0 {
            st.killsToAdd += 1
            // increase kill count;
        } else { st.recentKills += 1 }
        setMultiStats(player2dpid[player as usize], st, 1 as libc::c_int);
    } else {
        ingame.skScores[player as usize][1 as libc::c_int as usize] =
            ingame.skScores[player as
                                usize][1 as libc::c_int as
                                           usize].wrapping_add(1)
    };
}
