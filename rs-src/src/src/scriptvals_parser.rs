use ::libc;
extern "C" {
    #[no_mangle]
    fn vsprintf(_: *mut libc::c_char, _: *const libc::c_char,
                _: __builtin_va_list) -> libc::c_int;
    /* Return the length of S.  */
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_uint;
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    /* Use misc.  */
    /* Use misc or X/Open.  */
    /* Allocate SIZE bytes of memory.  */
    #[no_mangle]
    fn malloc(_: libc::c_uint) -> *mut libc::c_void;
    /* Free a block allocated by `malloc', `realloc' or `calloc'.  */
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
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
    /* Copy N bytes of SRC to DEST, guaranteeing
   correct behavior for overlapping strings.  */
    /* Copy no more than N bytes of SRC to DEST, stopping when C is found.
   Return the position in DEST one byte past where C was copied,
   or NULL if C was not found in the first N bytes of SRC.  */
    /* Misc || X/Open.  */
    /* Set N bytes of S to C.  */
    /* Compare N bytes of S1 and S2.  */
    /* Search N bytes of S for C.  */
    /* Search in S for C.  This is similar to `memchr' but there is no
   length limit.  */
    /* Search N bytes of S for the final occurrence of C.  */
    /* Copy SRC to DEST.  */
    /* Copy no more than N characters of SRC to DEST.  */
    /* Append SRC onto DEST.  */
    /* Append no more than N characters from SRC onto DEST.  */
    /* Compare S1 and S2.  */
    /* Compare N characters of S1 and S2.  */
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_uint) -> libc::c_int;
    #[no_mangle]
    fn resGetData(pType: *mut STRING, pID: *mut STRING) -> *mut libc::c_void;
    #[no_mangle]
    fn resPresent(pType: *mut STRING, pID: *mut STRING) -> BOOL;
    // Create a new context for a script
    #[no_mangle]
    fn eventNewContext(psCode: *mut SCRIPT_CODE, release: CONTEXT_RELEASE,
                       ppsContext: *mut *mut SCRIPT_CONTEXT) -> BOOL;
    // Add a new object to the trigger system
// Time is the application time at which all the triggers are to be started
    #[no_mangle]
    fn eventRunContext(psContext: *mut SCRIPT_CONTEXT, time: UDWORD) -> BOOL;
    // Set a global variable value for a context
    #[no_mangle]
    fn eventSetContextVar(psContext: *mut SCRIPT_CONTEXT, index: UDWORD,
                          type_0: INTERP_TYPE, data: UDWORD) -> BOOL;
    // The table of user types for the compiler
    #[no_mangle]
    static mut asTypeTable: [TYPE_SYMBOL; 0];
    // store array access data
    /* Set the current input buffer for the lexer */
    /* A simple error reporting routine */
    // parse a value file
    // Lookup a type
    // Lookup a variable identifier
    // Lookup an array identifier
    // Whether the script is run immediately or stored for later use
    // Add a new context to the list
    #[no_mangle]
    fn scrvAddContext(pID: *mut STRING, psContext: *mut SCRIPT_CONTEXT,
                      type_0: SCRV_TYPE) -> BOOL;
    #[no_mangle]
    fn scrvGetErrorData(pLine: *mut libc::c_int,
                        ppText: *mut *mut libc::c_char);
    #[no_mangle]
    fn scrvSetInputBuffer(pBuffer: *mut libc::c_char, size: UDWORD);
    // Link any object types to the actual pointer values
//extern BOOL scrvLinkValues(void);
    // Find a base object from it's id
    #[no_mangle]
    fn scrvGetBaseObj(id: UDWORD, ppsObj: *mut *mut BASE_OBJECT) -> BOOL;
    // Find a string from it's (string)id
    #[no_mangle]
    fn scrvGetString(pStringID: *mut STRING, ppString: *mut *mut STRING)
     -> BOOL;
    /* get a stat inc based on the name */
    #[no_mangle]
    fn getStructStatFromName(pName: *mut STRING) -> SDWORD;
    /*
 * Feature.h
 *
 * Definitions for the feature structures.
 *
 */
    //they're just not there anymore!!!!! Ye ha!
    /* The statistics for the features */
    //Value is stored for easy access to this feature in destroyDroid()/destroyStruct()
//extern UDWORD			droidFeature;
    /* Load the feature stats */
    /* Release the feature stats memory */
    // Set the tile no draw flags for a structure
    /* Create a feature on the map */
    /* Release the resources associated with a feature */
    /* Update routine for features */
    // free up a feature with no visual effects
    /* Remove a Feature and free it's memory */
    /* get a feature stat id from its name */
    #[no_mangle]
    fn getFeatureStatFromName(pName: *mut STRING) -> SDWORD;
    #[no_mangle]
    fn getCompFromResName(compType: UDWORD, pName: *mut STRING) -> SDWORD;
    #[no_mangle]
    fn getTemplateFromName(pName: *mut STRING) -> *mut DROID_TEMPLATE;
    #[no_mangle]
    fn audio_GetTrackID(szFileName: *mut libc::c_char) -> SDWORD;
    #[no_mangle]
    fn audio_SetTrackVals(szFileName: *mut libc::c_char, bLoop: BOOL,
                          piID: *mut libc::c_int, iVol: libc::c_int,
                          iPriority: libc::c_int, iAudibleRadius: libc::c_int,
                          VagID: libc::c_int) -> BOOL;
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
    /* The list of proximity displays allocated */
    //allocates the viewdata heap
    /* Initialise the message heaps */
    /* Release the message heaps */
    //destroys the viewdata heap
    /*Add a messgae to the list */
    /*remove a message */
    /* Remove all Messages*/
    /* removes all the proximity displays */
    /*load the view data for the messages from the file exported from the world editor*/
    /*get the view data that contains the text message pointer passed in */
    #[no_mangle]
    fn getViewData(pTextMsg: *mut STRING) -> *mut VIEWDATA;
    // find the level dataset
    #[no_mangle]
    fn levFindDataSet(pName: *mut STRING, ppsDataSet: *mut *mut LEVEL_DATASET)
     -> BOOL;
    /* For a given view data get the research this is related to */
    #[no_mangle]
    fn getResearch(pName: *mut STRING, resName: BOOL) -> *mut RESEARCH;
    /* A Bison parser, made by GNU Bison 3.5.4.  */
    /*
 * ScriptVals.y
 *
 * yacc grammar for loading script variable values
 *
 */
    /* Bison implementation for Yacc-like parsers in C

   Copyright (C) 1984, 1989-1990, 2000-2015, 2018-2020 Free Software Foundation,
   Inc.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <http://www.gnu.org/licenses/>.  */
    /* As a special exception, you may create a larger work that contains
   part or all of the Bison parser skeleton and distribute that work
   under terms of your choice, so long as that work isn't itself a
   parser generator using the skeleton or a modified version thereof
   as a parser skeleton.  Alternatively, if you modify or redistribute
   the parser skeleton itself, you may (at your option) remove this
   special exception, which will cause the skeleton and the resulting
   Bison output files to be licensed under the GNU General Public
   License without this special exception.

   This special exception was added by the Free Software Foundation in
   version 2.2 of Bison.  */
    #[no_mangle]
    fn scrv_lex() -> libc::c_int;
}
pub type __builtin_va_list = *mut libc::c_char;
pub type va_list = __builtin_va_list;
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
pub type SWORD = libc::c_short;
pub type UDWORD = libc::c_uint;
pub type SDWORD = libc::c_int;
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
pub type _interp_type = libc::c_uint;
pub const VAL_USERTYPESTART: _interp_type = 6;
pub const VAL_VOID: _interp_type = 5;
pub const VAL_EVENT: _interp_type = 4;
pub const VAL_TRIGGER: _interp_type = 3;
pub const VAL_STRING: _interp_type = 2;
pub const VAL_INT: _interp_type = 1;
pub const VAL_BOOL: _interp_type = 0;
pub type INTERP_TYPE = _interp_type;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _interp_val {
    pub type_0: INTERP_TYPE,
    pub v: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub bval: BOOL,
    pub ival: SDWORD,
    pub sval: *mut STRING,
    pub oval: *mut libc::c_void,
    pub pVoid: *mut libc::c_void,
}
pub type INTERP_VAL = _interp_val;
pub type STORAGE_TYPE = UBYTE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _var_debug {
    pub pIdent: *mut STRING,
    pub storage: STORAGE_TYPE,
}
pub type VAR_DEBUG = _var_debug;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _array_data {
    pub base: UDWORD,
    pub type_0: UBYTE,
    pub dimensions: UBYTE,
    pub elements: [UBYTE; 4],
}
pub type ARRAY_DATA = _array_data;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _array_debug {
    pub pIdent: *mut STRING,
    pub storage: UBYTE,
}
pub type ARRAY_DEBUG = _array_debug;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _script_debug {
    pub offset: UDWORD,
    pub line: UDWORD,
    pub pLabel: *mut STRING,
}
pub type SCRIPT_DEBUG = _script_debug;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _trigger_data {
    pub type_0: UWORD,
    pub code: UWORD,
    pub time: UDWORD,
}
pub type TRIGGER_DATA = _trigger_data;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _script_code {
    pub size: UDWORD,
    pub pCode: *mut UDWORD,
    pub numTriggers: UWORD,
    pub numEvents: UWORD,
    pub pTriggerTab: *mut UWORD,
    pub psTriggerData: *mut TRIGGER_DATA,
    pub pEventTab: *mut UWORD,
    pub pEventLinks: *mut SWORD,
    pub numGlobals: UWORD,
    pub numArrays: UWORD,
    pub arraySize: UDWORD,
    pub pGlobals: *mut INTERP_TYPE,
    pub ppsLocalVars: *mut *mut INTERP_TYPE,
    pub numLocalVars: *mut UDWORD,
    pub ppsLocalVarVal: *mut *mut INTERP_VAL,
    pub numParams: *mut UDWORD,
    pub psVarDebug: *mut VAR_DEBUG,
    pub psArrayInfo: *mut ARRAY_DATA,
    pub psArrayDebug: *mut ARRAY_DEBUG,
    pub debugEntries: UWORD,
    pub psDebug: *mut SCRIPT_DEBUG,
}
pub type SCRIPT_CODE = _script_code;
// function pointer for script variable saving
// if pBuffer is NULL the script system is just asking how much space the saved variable will require
// otherwise pBuffer points to an array to store the value in
pub type SCR_VAL_SAVE
    =
    Option<unsafe extern "C" fn(_: INTERP_TYPE, _: UDWORD,
                                _: *mut libc::c_char, _: *mut UDWORD)
               -> BOOL>;
// function pointer for script variable loading
pub type SCR_VAL_LOAD
    =
    Option<unsafe extern "C" fn(_: SDWORD, _: INTERP_TYPE,
                                _: *mut libc::c_char, _: UDWORD,
                                _: *mut UDWORD) -> BOOL>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _type_symbol {
    pub typeID: SWORD,
    pub accessType: SWORD,
    pub pIdent: *mut STRING,
    pub saveFunc: SCR_VAL_SAVE,
    pub loadFunc: SCR_VAL_LOAD,
}
/* Type for a user type symbol */
pub type TYPE_SYMBOL = _type_symbol;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _val_chunk {
    pub asVals: [INTERP_VAL; 20],
    pub psNext: *mut _val_chunk,
}
pub type VAL_CHUNK = _val_chunk;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _script_context {
    pub psCode: *mut SCRIPT_CODE,
    pub psGlobals: *mut VAL_CHUNK,
    pub triggerCount: SDWORD,
    pub release: SWORD,
    pub id: SWORD,
    pub psNext: *mut _script_context,
}
pub type SCRIPT_CONTEXT = _script_context;
pub type _scr_user_types = libc::c_uint;
pub const ST_MAXTYPE: _scr_user_types = 34;
pub const ST_POINTER_S: _scr_user_types = 33;
pub const ST_POINTER_T: _scr_user_types = 32;
pub const ST_POINTER_O: _scr_user_types = 31;
pub const ST_RESEARCH: _scr_user_types = 30;
pub const ST_GROUP: _scr_user_types = 29;
pub const ST_LEVEL: _scr_user_types = 28;
pub const ST_SOUND: _scr_user_types = 27;
pub const ST_TEXTSTRING: _scr_user_types = 26;
pub const ST_DROIDID: _scr_user_types = 25;
pub const ST_FEATURESTAT: _scr_user_types = 24;
pub const ST_STRUCTURESTAT: _scr_user_types = 23;
pub const ST_STRUCTUREID: _scr_user_types = 22;
pub const ST_TEMPLATE: _scr_user_types = 21;
pub const ST_BRAIN: _scr_user_types = 20;
pub const ST_REPAIR: _scr_user_types = 19;
pub const ST_WEAPON: _scr_user_types = 18;
pub const ST_CONSTRUCT: _scr_user_types = 17;
pub const ST_SENSOR: _scr_user_types = 16;
pub const ST_ECM: _scr_user_types = 15;
pub const ST_PROPULSION: _scr_user_types = 14;
pub const ST_BODY: _scr_user_types = 13;
pub const ST_COMPONENT: _scr_user_types = 12;
pub const ST_BASESTATS: _scr_user_types = 11;
pub const ST_FEATURE: _scr_user_types = 10;
pub const ST_STRUCTURE: _scr_user_types = 9;
pub const ST_DROID: _scr_user_types = 8;
pub const ST_BASEOBJECT: _scr_user_types = 7;
pub const ST_INTMESSAGE: _scr_user_types = 6;
// The type id to use in the type field of values
// Whether the type is an object or a simple value
// Type identifier
// load and save functions
// 
/* **********************************************************************************
 *
 * Event system functions
 */
// Whether a context is released when there are no active triggers for it
pub type _context_release = libc::c_uint;
// do not release the context
// release the context
pub const CR_NORELEASE: _context_release = 1;
pub const CR_RELEASE: _context_release = 0;
pub type CONTEXT_RELEASE = _context_release;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PLANE {
    pub a: FRACT,
    pub b: FRACT,
    pub c: FRACT,
    pub d: FRACT,
    pub vP: iVector,
}
/* **************************************************************************/
// These 1st three entries can NOT NOW be cast into a iVectorf *   (iVectorf on PC are doubles)
// these values form the plane equation ax+by+cz=d
// a point on the plane - in normal non-fract format
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
pub type SCREEN_DISP_DATA = _screen_disp_data;
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
pub type _object_type = libc::c_uint;
// for the camera tracking
// Comes out of guns, stupid :-)
pub const OBJ_TARGET: _object_type = 4;
// Things like roads, trees, bridges, fires
pub const OBJ_BULLET: _object_type = 3;
// All Buildings
pub const OBJ_FEATURE: _object_type = 2;
// Droids
pub const OBJ_STRUCTURE: _object_type = 1;
pub const OBJ_DROID: _object_type = 0;
pub type OBJECT_TYPE = _object_type;
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
pub type BASE_OBJECT = _base_object;
pub type _init_type = libc::c_uint;
pub const IT_STRING: _init_type = 2;
pub const IT_INDEX: _init_type = 1;
pub const IT_BOOL: _init_type = 0;
pub type INIT_TYPE = _init_type;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _var_init {
    pub type_0: INIT_TYPE,
    pub index: SDWORD,
    pub pString: *mut STRING,
}
/*
 * ScriptVals.h
 *
 * Common functions for the scriptvals loader
 */
// The possible types of initialisation values
// All the possible values that may be used to initialise a variable
pub type VAR_INIT = _var_init;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _array_indexes {
    pub dimensions: SDWORD,
    pub elements: [SDWORD; 4],
}
pub type ARRAY_INDEXES = _array_indexes;
/* Stored state numbers (used for stacks). */
pub type yy_state_t = yytype_int8;
/* !YY_SCRV_Y_TAB_H_INCLUDED  */
/* On compilers that do not define __PTRDIFF_MAX__ etc., make sure
   <limits.h> and (if available) <stdint.h> are included
   so that the code can choose integer types of a good width.  */
/* Narrow types that promote to a signed type and that can represent a
   signed or unsigned integer of at least N bits.  In tables they can
   save space and decrease cache pressure.  Promoting to a signed type
   helps avoid bugs in integer arithmetic.  */
pub type yytype_int8 = libc::c_schar;
/* Tokens.  */
/* Value type.  */
#[derive(Copy, Clone)]
#[repr(C)]
pub union YYSTYPE {
    pub bval: BOOL,
    pub tval: INTERP_TYPE,
    pub sval: *mut STRING,
    pub vindex: UDWORD,
    pub ival: SDWORD,
    pub sInit: VAR_INIT,
    pub arrayIndex: *mut ARRAY_INDEXES,
}
/* State numbers in computations.  */
pub type yy_state_fast_t = libc::c_int;
pub type RESEARCH = research_stats;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct research_stats {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
    pub techCode: UBYTE,
    pub techLevel: TECH_LEVEL,
    pub subGroup: UWORD,
    pub researchPoints: UWORD,
    pub researchPower: UDWORD,
    pub keyTopic: UBYTE,
    pub storeCount: UBYTE,
    pub numPRRequired: UBYTE,
    pub pPRList: *mut UWORD,
    pub numStructures: UBYTE,
    pub pStructList: *mut UWORD,
    pub numFunctions: UBYTE,
    pub pFunctionList: *mut *mut _function,
    pub numRedStructs: UBYTE,
    pub pRedStructs: *mut UWORD,
    pub numRedArtefacts: UBYTE,
    pub pRedArtefacts: *mut *mut COMP_BASE_STATS,
    pub numStructResults: UBYTE,
    pub pStructureResults: *mut UWORD,
    pub numArteResults: UBYTE,
    pub pArtefactResults: *mut *mut COMP_BASE_STATS,
    pub pReplacedArtefacts: *mut *mut COMP_BASE_STATS,
    pub pViewData: *mut _viewdata,
    pub iconID: UWORD,
    pub psStat: *mut BASE_STATS,
    pub pIMD: *mut iIMDShape,
    pub pIMD2: *mut iIMDShape,
}
pub type BASE_STATS = _base_stats;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _base_stats {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
}
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
pub type COMP_BASE_STATS = _comp_base_stats;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _comp_base_stats {
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
}
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
pub struct _function {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
    pub type_0: UBYTE,
}
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
pub type VIEWDATA = _viewdata;
pub type DROID_TEMPLATE = _droid_template;
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
pub const COMP_WEAPON: _component_type = 8;
pub const COMP_BRAIN: _component_type = 2;
pub const COMP_REPAIRUNIT: _component_type = 4;
pub const COMP_CONSTRUCT: _component_type = 7;
pub const COMP_SENSOR: _component_type = 6;
pub const COMP_ECM: _component_type = 5;
pub const COMP_PROPULSION: _component_type = 3;
pub const COMP_BODY: _component_type = 1;
pub type SCRV_TYPE = _scrv_type;
pub type _scrv_type = libc::c_uint;
pub const SCRV_NOEXEC: _scrv_type = 1;
pub const SCRV_EXEC: _scrv_type = 0;
// type of map
// number of players for the map
// index of WRF/WDG that loads the scenario file
// title for the level
// title for the level
// the WRF/WDG files for the level
// in load order
// LEVEL_DATASET that must be loaded for this level to load
// LEVEL_DATASET used when changing to this level from another
/* ! defined yyoverflow || YYERROR_VERBOSE */
/* A type that is properly aligned for any stack member.  */
#[derive(Copy, Clone)]
#[repr(C)]
pub union yyalloc {
    pub yyss_alloc: yy_state_t,
    pub yyvs_alloc: YYSTYPE,
}
pub type _component_type = libc::c_uint;
pub const COMP_NUMCOMPONENTS: _component_type = 9;
pub const COMP_UNKNOWN: _component_type = 0;
// The current script code
static mut psCurrScript: *mut SCRIPT_CODE =
    0 as *const SCRIPT_CODE as *mut SCRIPT_CODE;
// The current script context
static mut psCurrContext: *mut SCRIPT_CONTEXT =
    0 as *const SCRIPT_CONTEXT as *mut SCRIPT_CONTEXT;
// the current array indexes
static mut sCurrArrayIndexes: ARRAY_INDEXES =
    ARRAY_INDEXES{dimensions: 0, elements: [0; 4],};
/* C LALR(1) parser skeleton written by Richard Stallman, by
   simplifying the original so-called "semantic" parser.  */
// check that an array index is valid
#[no_mangle]
pub unsafe extern "C" fn scrvCheckArrayIndex(mut base: SDWORD,
                                             mut psIndexes:
                                                 *mut ARRAY_INDEXES,
                                             mut pIndex: *mut UDWORD)
 -> BOOL {
    /* All symbols defined below should begin with yy or YY, to avoid
   infringing on user name space.  This should be done even for local
   variables, as they might otherwise be expanded by user macros.
   There are some unavoidable exceptions within include files to
   define necessary library symbols; they are noted "INFRINGES ON
   USER NAME SPACE" below.  */
    let mut i: SDWORD = 0;
    let mut size: SDWORD = 0;
    if psCurrScript.is_null() || (*psCurrScript).psDebug.is_null() {
        return 0 as libc::c_int
    }
    /* Undocumented macros, especially those whose name start with YY_,
   are private implementation details.  Do not rely on them.  */
    if base < 0 as libc::c_int ||
           base >= (*psCurrScript).numArrays as libc::c_int {
        /* Identify Bison output.  */
        scrv_error(b"Array index out of range\x00" as *const u8 as
                       *const libc::c_char as *mut libc::c_char);
        return 0 as libc::c_int
    }
    /* Bison version.  */
    if (*psIndexes).dimensions !=
           (*(*psCurrScript).psArrayInfo.offset(base as isize)).dimensions as
               libc::c_int {
        /* Skeleton name.  */
        scrv_error(b"Invalid number of dimensions for array initialiser\x00"
                       as *const u8 as *const libc::c_char as
                       *mut libc::c_char);
        return 0 as libc::c_int
    }
    /* Pure parsers.  */
    i = 0 as libc::c_int;
    while i <
              (*(*psCurrScript).psArrayInfo.offset(base as isize)).dimensions
                  as libc::c_int {
        /* Push parsers.  */
        if (*psIndexes).elements[i as usize] < 0 as libc::c_int ||
               (*psIndexes).elements[i as usize] >=
                   (*(*psCurrScript).psArrayInfo.offset(base as
                                                            isize)).elements[i
                                                                                 as
                                                                                 usize]
                       as libc::c_int {
            /* Pull parsers.  */
            scrv_error(b"Invalid index for dimension %d\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char, i);
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Substitute the variable and function names.  */
    *pIndex = 0 as libc::c_int as UDWORD;
    size = 1 as libc::c_int;
    i =
        (*(*psCurrScript).psArrayInfo.offset(base as isize)).dimensions as
            libc::c_int - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        *pIndex =
            (*pIndex as
                 libc::c_uint).wrapping_add(((*psIndexes).elements[i as usize]
                                                 * size) as libc::c_uint) as
                UDWORD as UDWORD;
        size *=
            (*(*psCurrScript).psArrayInfo.offset(base as
                                                     isize)).elements[i as
                                                                          usize]
                as libc::c_int;
        i -= 1
    }
    /* First part of user prologue.  */
    *pIndex =
        (*pIndex as
             libc::c_uint).wrapping_add((*(*psCurrScript).psArrayInfo.offset(base
                                                                                 as
                                                                                 isize)).base)
            as UDWORD as UDWORD;
    return 1 as libc::c_int;
}
/* YYTRANSLATE[TOKEN-NUM] -- Symbol number corresponding to TOKEN-NUM
   as returned by yylex.  */
static mut yytranslate: [yytype_int8; 268] =
    [0 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 15 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 16 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 13 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 14 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     1 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     3 as libc::c_int as yytype_int8, 4 as libc::c_int as yytype_int8,
     5 as libc::c_int as yytype_int8, 6 as libc::c_int as yytype_int8,
     7 as libc::c_int as yytype_int8, 8 as libc::c_int as yytype_int8,
     9 as libc::c_int as yytype_int8, 10 as libc::c_int as yytype_int8,
     11 as libc::c_int as yytype_int8, 12 as libc::c_int as yytype_int8];
/* YYPACT[STATE-NUM] -- Index in YYTABLE of the portion describing
     STATE-NUM.  */
static mut yypact: [yytype_int8; 37] =
    [8 as libc::c_int as yytype_int8, 11 as libc::c_int as yytype_int8,
     0 as libc::c_int as yytype_int8, -(8 as libc::c_int) as yytype_int8,
     1 as libc::c_int as yytype_int8, -(8 as libc::c_int) as yytype_int8,
     -(8 as libc::c_int) as yytype_int8, -(8 as libc::c_int) as yytype_int8,
     13 as libc::c_int as yytype_int8, -(8 as libc::c_int) as yytype_int8,
     -(8 as libc::c_int) as yytype_int8, 9 as libc::c_int as yytype_int8,
     10 as libc::c_int as yytype_int8, 7 as libc::c_int as yytype_int8,
     7 as libc::c_int as yytype_int8, -(8 as libc::c_int) as yytype_int8,
     5 as libc::c_int as yytype_int8, -(7 as libc::c_int) as yytype_int8,
     -(8 as libc::c_int) as yytype_int8, 14 as libc::c_int as yytype_int8,
     -(5 as libc::c_int) as yytype_int8, 20 as libc::c_int as yytype_int8,
     -(8 as libc::c_int) as yytype_int8, 12 as libc::c_int as yytype_int8,
     -(8 as libc::c_int) as yytype_int8, -(8 as libc::c_int) as yytype_int8,
     2 as libc::c_int as yytype_int8, -(8 as libc::c_int) as yytype_int8,
     15 as libc::c_int as yytype_int8, 21 as libc::c_int as yytype_int8,
     -(8 as libc::c_int) as yytype_int8, -(8 as libc::c_int) as yytype_int8,
     -(8 as libc::c_int) as yytype_int8, -(8 as libc::c_int) as yytype_int8,
     -(8 as libc::c_int) as yytype_int8, 16 as libc::c_int as yytype_int8,
     -(8 as libc::c_int) as yytype_int8];
/* YYDEFACT[STATE-NUM] -- Default reduction number in state STATE-NUM.
     Performed when YYTABLE does not specify something else to do.  Zero
     means the default is an error.  */
static mut yydefact: [yytype_int8; 37] =
    [0 as libc::c_int as yytype_int8, 0 as libc::c_int as yytype_int8,
     0 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     0 as libc::c_int as yytype_int8, 8 as libc::c_int as yytype_int8,
     1 as libc::c_int as yytype_int8, 3 as libc::c_int as yytype_int8,
     0 as libc::c_int as yytype_int8, 4 as libc::c_int as yytype_int8,
     6 as libc::c_int as yytype_int8, 0 as libc::c_int as yytype_int8,
     0 as libc::c_int as yytype_int8, 9 as libc::c_int as yytype_int8,
     9 as libc::c_int as yytype_int8, 16 as libc::c_int as yytype_int8,
     0 as libc::c_int as yytype_int8, 0 as libc::c_int as yytype_int8,
     10 as libc::c_int as yytype_int8, 0 as libc::c_int as yytype_int8,
     0 as libc::c_int as yytype_int8, 0 as libc::c_int as yytype_int8,
     14 as libc::c_int as yytype_int8, 17 as libc::c_int as yytype_int8,
     5 as libc::c_int as yytype_int8, 11 as libc::c_int as yytype_int8,
     0 as libc::c_int as yytype_int8, 7 as libc::c_int as yytype_int8,
     0 as libc::c_int as yytype_int8, 0 as libc::c_int as yytype_int8,
     18 as libc::c_int as yytype_int8, 19 as libc::c_int as yytype_int8,
     20 as libc::c_int as yytype_int8, 12 as libc::c_int as yytype_int8,
     13 as libc::c_int as yytype_int8, 0 as libc::c_int as yytype_int8,
     15 as libc::c_int as yytype_int8];
/* YYPGOTO[NTERM-NUM].  */
static mut yypgoto: [yytype_int8; 12] =
    [-(8 as libc::c_int) as yytype_int8, -(8 as libc::c_int) as yytype_int8,
     24 as libc::c_int as yytype_int8, -(8 as libc::c_int) as yytype_int8,
     -(8 as libc::c_int) as yytype_int8, -(8 as libc::c_int) as yytype_int8,
     19 as libc::c_int as yytype_int8, -(6 as libc::c_int) as yytype_int8,
     -(8 as libc::c_int) as yytype_int8, -(8 as libc::c_int) as yytype_int8,
     -(8 as libc::c_int) as yytype_int8, -(8 as libc::c_int) as yytype_int8];
/* YYDEFGOTO[NTERM-NUM].  */
static mut yydefgoto: [yytype_int8; 12] =
    [-(1 as libc::c_int) as yytype_int8, 2 as libc::c_int as yytype_int8,
     3 as libc::c_int as yytype_int8, 11 as libc::c_int as yytype_int8,
     12 as libc::c_int as yytype_int8, 4 as libc::c_int as yytype_int8,
     17 as libc::c_int as yytype_int8, 18 as libc::c_int as yytype_int8,
     22 as libc::c_int as yytype_int8, 23 as libc::c_int as yytype_int8,
     19 as libc::c_int as yytype_int8, 33 as libc::c_int as yytype_int8];
/* YYTABLE[YYPACT[STATE-NUM]] -- What to do in state STATE-NUM.  If
     positive, shift that token.  If negative, reduce the rule whose
     number is the opposite.  If YYTABLE_NINF, syntax error.  */
static mut yytable: [yytype_int8; 34] =
    [6 as libc::c_int as yytype_int8, 15 as libc::c_int as yytype_int8,
     16 as libc::c_int as yytype_int8, 15 as libc::c_int as yytype_int8,
     16 as libc::c_int as yytype_int8, 30 as libc::c_int as yytype_int8,
     31 as libc::c_int as yytype_int8, 24 as libc::c_int as yytype_int8,
     32 as libc::c_int as yytype_int8, 27 as libc::c_int as yytype_int8,
     1 as libc::c_int as yytype_int8, 25 as libc::c_int as yytype_int8,
     8 as libc::c_int as yytype_int8, 9 as libc::c_int as yytype_int8,
     25 as libc::c_int as yytype_int8, 15 as libc::c_int as yytype_int8,
     16 as libc::c_int as yytype_int8, 5 as libc::c_int as yytype_int8,
     1 as libc::c_int as yytype_int8, 10 as libc::c_int as yytype_int8,
     21 as libc::c_int as yytype_int8, 26 as libc::c_int as yytype_int8,
     13 as libc::c_int as yytype_int8, 14 as libc::c_int as yytype_int8,
     28 as libc::c_int as yytype_int8, 35 as libc::c_int as yytype_int8,
     7 as libc::c_int as yytype_int8, 29 as libc::c_int as yytype_int8,
     0 as libc::c_int as yytype_int8, 0 as libc::c_int as yytype_int8,
     0 as libc::c_int as yytype_int8, 34 as libc::c_int as yytype_int8,
     36 as libc::c_int as yytype_int8, 20 as libc::c_int as yytype_int8];
static mut yycheck: [yytype_int8; 34] =
    [0 as libc::c_int as yytype_int8, 8 as libc::c_int as yytype_int8,
     9 as libc::c_int as yytype_int8, 8 as libc::c_int as yytype_int8,
     9 as libc::c_int as yytype_int8, 3 as libc::c_int as yytype_int8,
     4 as libc::c_int as yytype_int8, 14 as libc::c_int as yytype_int8,
     6 as libc::c_int as yytype_int8, 14 as libc::c_int as yytype_int8,
     10 as libc::c_int as yytype_int8, 17 as libc::c_int as yytype_int8,
     11 as libc::c_int as yytype_int8, 12 as libc::c_int as yytype_int8,
     20 as libc::c_int as yytype_int8, 8 as libc::c_int as yytype_int8,
     9 as libc::c_int as yytype_int8, 6 as libc::c_int as yytype_int8,
     10 as libc::c_int as yytype_int8, 6 as libc::c_int as yytype_int8,
     15 as libc::c_int as yytype_int8, 7 as libc::c_int as yytype_int8,
     13 as libc::c_int as yytype_int8, 13 as libc::c_int as yytype_int8,
     4 as libc::c_int as yytype_int8, 4 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 15 as libc::c_int as yytype_int8,
     -(1 as libc::c_int) as yytype_int8, -(1 as libc::c_int) as yytype_int8,
     -(1 as libc::c_int) as yytype_int8, 16 as libc::c_int as yytype_int8,
     16 as libc::c_int as yytype_int8, 14 as libc::c_int as yytype_int8];
/* YYSTOS[STATE-NUM] -- The (internal number of the) accessing
     symbol of state STATE-NUM.  */
static mut yystos: [yytype_int8; 37] =
    [0 as libc::c_int as yytype_int8, 10 as libc::c_int as yytype_int8,
     18 as libc::c_int as yytype_int8, 19 as libc::c_int as yytype_int8,
     22 as libc::c_int as yytype_int8, 6 as libc::c_int as yytype_int8,
     0 as libc::c_int as yytype_int8, 19 as libc::c_int as yytype_int8,
     11 as libc::c_int as yytype_int8, 12 as libc::c_int as yytype_int8,
     6 as libc::c_int as yytype_int8, 20 as libc::c_int as yytype_int8,
     21 as libc::c_int as yytype_int8, 13 as libc::c_int as yytype_int8,
     13 as libc::c_int as yytype_int8, 8 as libc::c_int as yytype_int8,
     9 as libc::c_int as yytype_int8, 23 as libc::c_int as yytype_int8,
     24 as libc::c_int as yytype_int8, 27 as libc::c_int as yytype_int8,
     23 as libc::c_int as yytype_int8, 15 as libc::c_int as yytype_int8,
     25 as libc::c_int as yytype_int8, 26 as libc::c_int as yytype_int8,
     14 as libc::c_int as yytype_int8, 24 as libc::c_int as yytype_int8,
     7 as libc::c_int as yytype_int8, 14 as libc::c_int as yytype_int8,
     4 as libc::c_int as yytype_int8, 15 as libc::c_int as yytype_int8,
     3 as libc::c_int as yytype_int8, 4 as libc::c_int as yytype_int8,
     6 as libc::c_int as yytype_int8, 28 as libc::c_int as yytype_int8,
     16 as libc::c_int as yytype_int8, 4 as libc::c_int as yytype_int8,
     16 as libc::c_int as yytype_int8];
#[no_mangle]
pub unsafe extern "C" fn scrvLookUpType(mut pIdent: *mut STRING,
                                        mut pType: *mut INTERP_TYPE) -> BOOL {
    let mut psCurr: *mut TYPE_SYMBOL = 0 as *mut TYPE_SYMBOL;
    psCurr = asTypeTable.as_mut_ptr();
    while (*psCurr).typeID as libc::c_int != 0 as libc::c_int {
        if strcmp((*psCurr).pIdent, pIdent) == 0 as libc::c_int {
            *pType = (*psCurr).typeID as INTERP_TYPE;
            return 1 as libc::c_int
        }
        psCurr = psCurr.offset(1)
        /* YYR2[YYN] -- Number of symbols on the right hand side of rule YYN.  */
    }
    return 0 as libc::c_int;
}
static mut yyr1: [yytype_int8; 21] =
    [0 as libc::c_int as yytype_int8, 17 as libc::c_int as yytype_int8,
     18 as libc::c_int as yytype_int8, 18 as libc::c_int as yytype_int8,
     20 as libc::c_int as yytype_int8, 19 as libc::c_int as yytype_int8,
     21 as libc::c_int as yytype_int8, 19 as libc::c_int as yytype_int8,
     22 as libc::c_int as yytype_int8, 23 as libc::c_int as yytype_int8,
     23 as libc::c_int as yytype_int8, 23 as libc::c_int as yytype_int8,
     24 as libc::c_int as yytype_int8, 25 as libc::c_int as yytype_int8,
     26 as libc::c_int as yytype_int8, 26 as libc::c_int as yytype_int8,
     27 as libc::c_int as yytype_int8, 27 as libc::c_int as yytype_int8,
     28 as libc::c_int as yytype_int8, 28 as libc::c_int as yytype_int8,
     28 as libc::c_int as yytype_int8];
static mut yyr2: [yytype_int8; 21] =
    [0 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     1 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     0 as libc::c_int as yytype_int8, 6 as libc::c_int as yytype_int8,
     0 as libc::c_int as yytype_int8, 7 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 0 as libc::c_int as yytype_int8,
     1 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     3 as libc::c_int as yytype_int8, 3 as libc::c_int as yytype_int8,
     1 as libc::c_int as yytype_int8, 4 as libc::c_int as yytype_int8,
     1 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     1 as libc::c_int as yytype_int8, 1 as libc::c_int as yytype_int8,
     1 as libc::c_int as yytype_int8];
// Lookup a variable identifier
#[no_mangle]
pub unsafe extern "C" fn scrvLookUpVar(mut pIdent: *mut STRING,
                                       mut pIndex: *mut UDWORD) -> BOOL {
    let mut i: UDWORD = 0;
    if psCurrScript.is_null() || (*psCurrScript).psDebug.is_null() {
        return 0 as libc::c_int
    }
    i = 0 as libc::c_int as UDWORD;
    while i < (*psCurrScript).numGlobals as libc::c_uint {
        if !(*(*psCurrScript).psVarDebug.offset(i as isize)).pIdent.is_null()
               &&
               strcmp((*(*psCurrScript).psVarDebug.offset(i as isize)).pIdent,
                      pIdent) == 0 as libc::c_int {
            *pIndex = i;
            return 1 as libc::c_int
        }
        i = i.wrapping_add(1)
    }
    return 0 as libc::c_int;
}
// Lookup an array identifier
#[no_mangle]
pub unsafe extern "C" fn scrvLookUpArray(mut pIdent: *mut STRING,
                                         mut pIndex: *mut UDWORD) -> BOOL {
    let mut i: UDWORD = 0;
    if psCurrScript.is_null() || (*psCurrScript).psDebug.is_null() {
        /* Error token number */
        return 0 as libc::c_int
    }
    i = 0 as libc::c_int as UDWORD;
    while i < (*psCurrScript).numArrays as libc::c_uint {
        if !(*(*psCurrScript).psArrayDebug.offset(i as
                                                      isize)).pIdent.is_null()
               &&
               strcmp((*(*psCurrScript).psArrayDebug.offset(i as
                                                                isize)).pIdent,
                      pIdent) == 0 as libc::c_int {
            *pIndex = i;
            return 1 as libc::c_int
        }
        i = i.wrapping_add(1)
    }
    return 0 as libc::c_int;
}
// Load a script value file
// Load a script value file
#[no_mangle]
pub unsafe extern "C" fn scrvLoad(mut pData: *mut libc::c_char,
                                  mut size: UDWORD) -> BOOL {
    scrvSetInputBuffer(pData, size);
    if scrv_parse() != 0 as libc::c_int { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
/* A simple error reporting routine */
#[no_mangle]
pub unsafe extern "C" fn scrv_error(mut pMessage: *mut libc::c_char,
                                    mut args: ...) {
    let mut line: libc::c_int = 0;
    let mut pText: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut aTxtBuf: [libc::c_char; 1024] = [0; 1024];
    let mut args_0: va_list = 0 as *mut libc::c_char;
    args_0 = args.clone();
    vsprintf(aTxtBuf.as_mut_ptr(), pMessage, args_0);
    scrvGetErrorData(&mut line, &mut pText);
    debug(LOG_ERROR,
          b"VLO parse error: %s at line %d, token: %d, text: \'%s\'\x00" as
              *const u8 as *const libc::c_char, aTxtBuf.as_mut_ptr(), line,
          scrv_char, pText);
}
/* YYERROR_VERBOSE */
/*-----------------------------------------------.
| Release the memory associated to this symbol.  |
`-----------------------------------------------*/
unsafe extern "C" fn yydestruct(mut yymsg: *const libc::c_char,
                                mut yytype: libc::c_int,
                                mut yyvaluep: *mut YYSTYPE) {
    if yymsg.is_null() {
        yymsg = b"Deleting\x00" as *const u8 as *const libc::c_char
    };
}
/* The lookahead symbol.  */
#[no_mangle]
pub static mut scrv_char: libc::c_int = 0;
/* The semantic value of the lookahead symbol.  */
#[no_mangle]
pub static mut scrv_lval: YYSTYPE = YYSTYPE{bval: 0,};
/* Number of syntax errors so far.  */
#[no_mangle]
pub static mut scrv_nerrs: libc::c_int = 0;
/*----------.
| yyparse.  |
`----------*/
#[no_mangle]
pub unsafe extern "C" fn scrv_parse() -> libc::c_int {
    let mut current_block: u64;
    let mut yystate: yy_state_fast_t = 0;
    /* Number of tokens to shift before error messages enabled.  */
    let mut yyerrstatus: libc::c_int = 0;
    /* The stacks and their tools:
       'yyss': related to states.
       'yyvs': related to semantic values.

       Refer to the stacks through separate pointers, to allow yyoverflow
       to reallocate them elsewhere.  */
    /* The state stack.  */
    let mut yyssa: [yy_state_t; 200] = [0; 200];
    let mut yyss: *mut yy_state_t = 0 as *mut yy_state_t;
    let mut yyssp: *mut yy_state_t = 0 as *mut yy_state_t;
    /* The semantic value stack.  */
    let mut yyvsa: [YYSTYPE; 200] = [YYSTYPE{bval: 0,}; 200];
    let mut yyvs: *mut YYSTYPE = 0 as *mut YYSTYPE;
    let mut yyvsp: *mut YYSTYPE = 0 as *mut YYSTYPE;
    let mut yystacksize: libc::c_int = 0;
    let mut yyn: libc::c_int = 0;
    let mut yyresult: libc::c_int = 0;
    /* Lookahead token as an internal (translated) token number.  */
    let mut yytoken: libc::c_int = 0 as libc::c_int;
    /* The variables used to return semantic value and location from the
     action routines.  */
    let mut yyval: YYSTYPE = YYSTYPE{bval: 0,};
    /* The number of symbols on the RHS of the reduced rule.
     Keep to zero when no symbol should be popped.  */
    let mut yylen: libc::c_int =
        0 as libc::c_int; /* Cause a token to be read.  */
    yyss = yyssa.as_mut_ptr();
    yyssp = yyss;
    yyvs = yyvsa.as_mut_ptr();
    yyvsp = yyvs;
    yystacksize = 200 as libc::c_int;
    yystate = 0 as libc::c_int;
    yyerrstatus = 0 as libc::c_int;
    scrv_nerrs = 0 as libc::c_int;
    scrv_char = -(2 as libc::c_int);
    's_88:
        loop 
             /*--------------------------------------------------------------------.
| yysetstate -- set current state (the top of the stack) to yystate.  |
`--------------------------------------------------------------------*/
             {
            (0 as libc::c_int != 0 &&
                 (0 as libc::c_int <= yystate && yystate < 37 as libc::c_int))
                as libc::c_int;
            *yyssp = yystate as yy_state_t;
            if yyss.offset(yystacksize as
                               isize).offset(-(1 as libc::c_int as isize)) <=
                   yyssp {
                /* Get the current used size of the three stacks, in elements.  */
                let mut yysize: libc::c_int =
                    yyssp.wrapping_offset_from(yyss) as libc::c_int +
                        1 as libc::c_int;
                /* defined YYSTACK_RELOCATE */
                /* Extend the stack our own way.  */
                if 10000 as libc::c_int <= yystacksize {
                    current_block = 8735508871533772611;
                    break ;
                }
                yystacksize *= 2 as libc::c_int;
                if (10000 as libc::c_int) < yystacksize {
                    yystacksize = 10000 as libc::c_int
                }
                let mut yyss1: *mut yy_state_t = yyss;
                let mut yyptr: *mut yyalloc =
                    malloc((yystacksize *
                                (::std::mem::size_of::<yy_state_t>() as
                                     libc::c_ulong as libc::c_int +
                                     ::std::mem::size_of::<YYSTYPE>() as
                                         libc::c_ulong as libc::c_int) +
                                (::std::mem::size_of::<yyalloc>() as
                                     libc::c_ulong as libc::c_int -
                                     1 as libc::c_int)) as libc::c_uint) as
                        *mut yyalloc;
                if yyptr.is_null() {
                    current_block = 8735508871533772611;
                    break ;
                }
                let mut yynewbytes: libc::c_int = 0;
                libc::memcpy(&mut (*yyptr).yyss_alloc as *mut yy_state_t as
                                 *mut libc::c_void,
                             yyss as *const libc::c_void,
                             (yysize as
                                  libc::c_uint).wrapping_mul(::std::mem::size_of::<yy_state_t>()
                                                                 as
                                                                 libc::c_ulong)
                                 as libc::size_t);
                yyss = &mut (*yyptr).yyss_alloc;
                yynewbytes =
                    yystacksize *
                        ::std::mem::size_of::<yy_state_t>() as libc::c_ulong
                            as libc::c_int +
                        (::std::mem::size_of::<yyalloc>() as libc::c_ulong as
                             libc::c_int - 1 as libc::c_int);
                yyptr =
                    yyptr.offset((yynewbytes /
                                      ::std::mem::size_of::<yyalloc>() as
                                          libc::c_ulong as libc::c_int) as
                                     isize);
                let mut yynewbytes_0: libc::c_int = 0;
                libc::memcpy(&mut (*yyptr).yyvs_alloc as *mut YYSTYPE as
                                 *mut libc::c_void,
                             yyvs as *const libc::c_void,
                             (yysize as
                                  libc::c_uint).wrapping_mul(::std::mem::size_of::<YYSTYPE>()
                                                                 as
                                                                 libc::c_ulong)
                                 as libc::size_t);
                yyvs = &mut (*yyptr).yyvs_alloc;
                yynewbytes_0 =
                    yystacksize *
                        ::std::mem::size_of::<YYSTYPE>() as libc::c_ulong as
                            libc::c_int +
                        (::std::mem::size_of::<yyalloc>() as libc::c_ulong as
                             libc::c_int - 1 as libc::c_int);
                yyptr =
                    yyptr.offset((yynewbytes_0 /
                                      ::std::mem::size_of::<yyalloc>() as
                                          libc::c_ulong as libc::c_int) as
                                     isize);
                if yyss1 != yyssa.as_mut_ptr() {
                    free(yyss1 as *mut libc::c_void);
                }
                yyssp =
                    yyss.offset(yysize as
                                    isize).offset(-(1 as libc::c_int as
                                                        isize));
                yyvsp =
                    yyvs.offset(yysize as
                                    isize).offset(-(1 as libc::c_int as
                                                        isize));
                if yyss.offset(yystacksize as
                                   isize).offset(-(1 as libc::c_int as isize))
                       <= yyssp {
                    current_block = 9549167192516842735;
                    break ;
                }
            }
            /* !defined yyoverflow && !defined YYSTACK_RELOCATE */
            if yystate == 6 as libc::c_int {
                /*-------------------------------------.
| yyacceptlab -- YYACCEPT comes here.  |
`-------------------------------------*/
                yyresult = 0 as libc::c_int;
                current_block = 16617850210136980057;
                break ;
            } else {
                /*-----------.
| yybackup.  |
`-----------*/
                /* Do appropriate processing given the current state.  Read a
     lookahead token if we need one and don't already have one.  */
                /* First try to decide what to do without reference to lookahead token.  */
                yyn = yypact[yystate as usize] as libc::c_int;
                if yyn == -(8 as libc::c_int) {
                    current_block = 3496058479340000382;
                } else {
                    /* Not known => get a lookahead token if don't already have one.  */
                    /* YYCHAR is either YYEMPTY or YYEOF or a valid lookahead symbol.  */
                    if scrv_char == -(2 as libc::c_int) {
                        scrv_char = scrv_lex()
                    }
                    if scrv_char <= 0 as libc::c_int {
                        yytoken = 0 as libc::c_int;
                        scrv_char = yytoken
                    } else {
                        yytoken =
                            if 0 as libc::c_int <= scrv_char &&
                                   scrv_char <= 267 as libc::c_int {
                                yytranslate[scrv_char as usize] as libc::c_int
                            } else { 2 as libc::c_int }
                    }
                    /* If the proper action on seeing token YYTOKEN is to reduce or to
     detect an error, take that action.  */
                    yyn += yytoken;
                    if yyn < 0 as libc::c_int || (33 as libc::c_int) < yyn ||
                           yycheck[yyn as usize] as libc::c_int != yytoken {
                        current_block = 3496058479340000382;
                    } else {
                        yyn = yytable[yyn as usize] as libc::c_int;
                        if yyn <= 0 as libc::c_int {
                            yyn = -yyn;
                            current_block = 10004144406045794244;
                        } else {
                            /* Count tokens shifted since error; after three, turn off error
     status.  */
                            if yyerrstatus != 0 { yyerrstatus -= 1 }
                            /* Shift the lookahead token.  */
                            yystate = yyn;
                            yyvsp = yyvsp.offset(1);
                            *yyvsp = scrv_lval;
                            /* Discard the shifted token.  */
                            scrv_char = -(2 as libc::c_int);
                            current_block = 2962149868552024647;
                        }
                    }
                }
                match current_block {
                    3496058479340000382 =>
                    /*-----------------------------------------------------------.
| yydefault -- do the default action for the current state.  |
`-----------------------------------------------------------*/
                    {
                        yyn = yydefact[yystate as usize] as libc::c_int;
                        if yyn == 0 as libc::c_int {
                            /*--------------------------------------.
| yyerrlab -- here on detecting error.  |
`--------------------------------------*/
                            /* Make sure we have latest lookahead translation.  See comments at
     user semantic actions for why this is necessary.  */
                            yytoken =
                                if scrv_char == -(2 as libc::c_int) {
                                    -(2 as libc::c_int)
                                } else if 0 as libc::c_int <= scrv_char &&
                                              scrv_char <= 267 as libc::c_int
                                 {
                                    yytranslate[scrv_char as usize] as
                                        libc::c_int
                                } else { 2 as libc::c_int };
                            /* If not already recovering from an error, report this error.  */
                            if yyerrstatus == 0 {
                                scrv_nerrs += 1;
                                scrv_error(b"syntax error\x00" as *const u8 as
                                               *const libc::c_char as
                                               *mut libc::c_char);
                            }
                            if yyerrstatus == 3 as libc::c_int {
                                /* If just tried and failed to reuse lookahead token after an
         error, discard it.  */
                                if scrv_char <= 0 as libc::c_int {
                                    /* Return failure if at end of input.  */
                                    if scrv_char == 0 as libc::c_int {
                                        current_block = 9549167192516842735;
                                        break ;
                                    }
                                } else {
                                    yydestruct(b"Error: discarding\x00" as
                                                   *const u8 as
                                                   *const libc::c_char,
                                               yytoken, &mut scrv_lval);
                                    scrv_char = -(2 as libc::c_int)
                                }
                            }
                            /* Else will try to reuse lookahead token after shifting the error
     token.  */
                            /*-------------------------------------------------------------.
| yyerrlab1 -- common code for both syntax error and YYERROR.  |
`-------------------------------------------------------------*/
                            yyerrstatus =
                                3 as
                                    libc::c_int; /* Each real token shifted decrements this.  */
                            loop  {
                                yyn = yypact[yystate as usize] as libc::c_int;
                                if !(yyn == -(8 as libc::c_int)) {
                                    yyn += 1 as libc::c_int;
                                    if 0 as libc::c_int <= yyn &&
                                           yyn <= 33 as libc::c_int &&
                                           yycheck[yyn as usize] as
                                               libc::c_int == 1 as libc::c_int
                                       {
                                        yyn =
                                            yytable[yyn as usize] as
                                                libc::c_int;
                                        if (0 as libc::c_int) < yyn {
                                            break ;
                                        }
                                    }
                                }
                                /* Pop the current state because it cannot handle the error token.  */
                                if yyssp == yyss {
                                    current_block = 9549167192516842735;
                                    break 's_88 ;
                                }
                                yydestruct(b"Error: popping\x00" as *const u8
                                               as *const libc::c_char,
                                           yystos[yystate as usize] as
                                               libc::c_int, yyvsp);
                                yyvsp =
                                    yyvsp.offset(-(1 as libc::c_int as
                                                       isize));
                                yyssp =
                                    yyssp.offset(-(1 as libc::c_int as
                                                       isize));
                                yystate = *yyssp as yy_state_fast_t
                            }
                            yyvsp = yyvsp.offset(1);
                            *yyvsp = scrv_lval;
                            /* Shift the error token.  */
                            yystate = yyn;
                            current_block = 2962149868552024647;
                        } else { current_block = 10004144406045794244; }
                    }
                    _ => { }
                }
                match current_block {
                    10004144406045794244 =>
                    /*-----------------------------.
| yyreduce -- do a reduction.  |
`-----------------------------*/
                    /* yyn is the number of a rule to reduce with.  */
                    {
                        yylen = yyr2[yyn as usize] as libc::c_int;
                        /* If YYLEN is nonzero, implement the default value of the action:
     '$$ = $1'.

     Otherwise, the following line sets YYVAL to garbage.
     This behavior is undocumented and Bison
     users should not rely upon it.  Assigning to YYVAL
     unconditionally makes the parser a bit smaller, and it avoids a
     GCC warning that YYVAL may be used uninitialized.  */
                        yyval =
                            *yyvsp.offset((1 as libc::c_int - yylen) as
                                              isize);
                        match yyn {
                            4 => {
                                if eventNewContext(psCurrScript, CR_RELEASE,
                                                   &mut psCurrContext) == 0 {
                                    scrv_error(b"Couldn\'t create context\x00"
                                                   as *const u8 as
                                                   *const libc::c_char as
                                                   *mut libc::c_char);
                                    current_block = 9549167192516842735;
                                    break ;
                                } else if scrvAddContext((*yyvsp.offset(-(1 as
                                                                              libc::c_int)
                                                                            as
                                                                            isize)).sval,
                                                         psCurrContext,
                                                         SCRV_EXEC) == 0 {
                                    scrv_error(b"Couldn\'t store context\x00"
                                                   as *const u8 as
                                                   *const libc::c_char as
                                                   *mut libc::c_char);
                                    current_block = 9549167192516842735;
                                    break ;
                                }
                            }
                            5 => {
                                if eventRunContext(psCurrContext,
                                                   gameTime.wrapping_div(100
                                                                             as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint))
                                       == 0 {
                                    current_block = 9549167192516842735;
                                    break ;
                                }
                            }
                            6 => {
                                if eventNewContext(psCurrScript, CR_NORELEASE,
                                                   &mut psCurrContext) == 0 {
                                    scrv_error(b"Couldn\'t create context\x00"
                                                   as *const u8 as
                                                   *const libc::c_char as
                                                   *mut libc::c_char);
                                    current_block = 9549167192516842735;
                                    break ;
                                } else if scrvAddContext((*yyvsp.offset(0 as
                                                                            libc::c_int
                                                                            as
                                                                            isize)).sval,
                                                         psCurrContext,
                                                         SCRV_NOEXEC) == 0 {
                                    scrv_error(b"Couldn\'t store context\x00"
                                                   as *const u8 as
                                                   *const libc::c_char as
                                                   *mut libc::c_char);
                                    current_block = 9549167192516842735;
                                    break ;
                                }
                            }
                            8 => {
                                let mut namelen: libc::c_int = 0;
                                let mut extpos: libc::c_int = 0;
                                let mut stringname: *mut libc::c_char =
                                    0 as *mut libc::c_char;
                                stringname =
                                    (*yyvsp.offset(0 as libc::c_int as
                                                       isize)).sval;
                                namelen = strlen(stringname) as libc::c_int;
                                extpos = namelen - 3 as libc::c_int;
                                if strncmp(&mut *stringname.offset(extpos as
                                                                       isize),
                                           b"blo\x00" as *const u8 as
                                               *const libc::c_char,
                                           3 as libc::c_int as libc::c_uint)
                                       == 0 as libc::c_int {
                                    if resPresent(b"BLO\x00" as *const u8 as
                                                      *const libc::c_char as
                                                      *mut STRING, stringname)
                                           == 1 as libc::c_int {
                                        psCurrScript =
                                            resGetData(b"BLO\x00" as *const u8
                                                           as
                                                           *const libc::c_char
                                                           as *mut STRING,
                                                       stringname) as
                                                *mut SCRIPT_CODE
                                    } else {
                                        *stringname.offset(extpos as isize) =
                                            's' as i32 as libc::c_char;
                                        psCurrScript =
                                            resGetData(b"SCRIPT\x00" as
                                                           *const u8 as
                                                           *const libc::c_char
                                                           as *mut STRING,
                                                       stringname) as
                                                *mut SCRIPT_CODE
                                    }
                                } else if strncmp(&mut *stringname.offset(extpos
                                                                              as
                                                                              isize),
                                                  b"slo\x00" as *const u8 as
                                                      *const libc::c_char,
                                                  3 as libc::c_int as
                                                      libc::c_uint) ==
                                              0 as libc::c_int {
                                    if resPresent(b"SCRIPT\x00" as *const u8
                                                      as *const libc::c_char
                                                      as *mut STRING,
                                                  stringname) ==
                                           1 as libc::c_int {
                                        psCurrScript =
                                            resGetData(b"SCRIPT\x00" as
                                                           *const u8 as
                                                           *const libc::c_char
                                                           as *mut STRING,
                                                       stringname) as
                                                *mut SCRIPT_CODE
                                    }
                                }
                                if psCurrScript.is_null() {
                                    scrv_error(b"Script file %s not found\x00"
                                                   as *const u8 as
                                                   *const libc::c_char as
                                                   *mut libc::c_char,
                                               stringname);
                                    current_block = 9549167192516842735;
                                    break ;
                                } else {
                                    yyval.sval =
                                        (*yyvsp.offset(0 as libc::c_int as
                                                           isize)).sval
                                }
                            }
                            12 => {
                                let mut psObj: *mut BASE_OBJECT =
                                    0 as *mut BASE_OBJECT;
                                let mut compIndex: SDWORD = 0;
                                let mut psTemplate: *mut DROID_TEMPLATE =
                                    0 as *mut DROID_TEMPLATE;
                                let mut psViewData: *mut VIEWDATA =
                                    0 as *mut VIEWDATA;
                                let mut pString: *mut STRING =
                                    0 as *mut STRING;
                                let mut psResearch: *mut RESEARCH =
                                    0 as *mut RESEARCH;
                                match (*yyvsp.offset(-(1 as libc::c_int) as
                                                         isize)).tval as
                                          libc::c_uint {
                                    1 => {
                                        if (*yyvsp.offset(0 as libc::c_int as
                                                              isize)).sInit.type_0
                                               as libc::c_uint !=
                                               IT_INDEX as libc::c_int as
                                                   libc::c_uint ||
                                               eventSetContextVar(psCurrContext,
                                                                  (*yyvsp.offset(-(2
                                                                                       as
                                                                                       libc::c_int)
                                                                                     as
                                                                                     isize)).vindex,
                                                                  (*yyvsp.offset(-(1
                                                                                       as
                                                                                       libc::c_int)
                                                                                     as
                                                                                     isize)).tval,
                                                                  (*yyvsp.offset(0
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize)).sInit.index
                                                                      as
                                                                      UDWORD)
                                                   == 0 {
                                            scrv_error(b"Typemismatch for variable %d\x00"
                                                           as *const u8 as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char,
                                                       (*yyvsp.offset(-(2 as
                                                                            libc::c_int)
                                                                          as
                                                                          isize)).vindex);
                                            current_block =
                                                9549167192516842735;
                                            break ;
                                        }
                                    }
                                    8 => {
                                        if (*yyvsp.offset(0 as libc::c_int as
                                                              isize)).sInit.type_0
                                               as libc::c_uint !=
                                               IT_INDEX as libc::c_int as
                                                   libc::c_uint {
                                            scrv_error(b"Typemismatch for variable %d\x00"
                                                           as *const u8 as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char,
                                                       (*yyvsp.offset(-(2 as
                                                                            libc::c_int)
                                                                          as
                                                                          isize)).vindex);
                                            current_block =
                                                9549167192516842735;
                                            break ;
                                        } else if scrvGetBaseObj((*yyvsp.offset(0
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    isize)).sInit.index
                                                                     as
                                                                     UDWORD,
                                                                 &mut psObj)
                                                      == 0 {
                                            scrv_error(b"Droid id %d not found\x00"
                                                           as *const u8 as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char,
                                                       (*yyvsp.offset(0 as
                                                                          libc::c_int
                                                                          as
                                                                          isize)).sInit.index
                                                           as UDWORD);
                                            current_block =
                                                9549167192516842735;
                                            break ;
                                        } else if (*psObj).type_0 as
                                                      libc::c_uint !=
                                                      OBJ_DROID as libc::c_int
                                                          as libc::c_uint {
                                            scrv_error(b"Object id %d is not a droid\x00"
                                                           as *const u8 as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char,
                                                       (*yyvsp.offset(0 as
                                                                          libc::c_int
                                                                          as
                                                                          isize)).sInit.index
                                                           as UDWORD);
                                            current_block =
                                                9549167192516842735;
                                            break ;
                                        } else if eventSetContextVar(psCurrContext,
                                                                     (*yyvsp.offset(-(2
                                                                                          as
                                                                                          libc::c_int)
                                                                                        as
                                                                                        isize)).vindex,
                                                                     (*yyvsp.offset(-(1
                                                                                          as
                                                                                          libc::c_int)
                                                                                        as
                                                                                        isize)).tval,
                                                                     psObj as
                                                                         UDWORD)
                                                      == 0 {
                                            scrv_error(b"Set Value Failed for %s\x00"
                                                           as *const u8 as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char,
                                                       (*yyvsp.offset(-(2 as
                                                                            libc::c_int)
                                                                          as
                                                                          isize)).vindex);
                                            current_block =
                                                9549167192516842735;
                                            break ;
                                        }
                                    }
                                    9 => {
                                        if (*yyvsp.offset(0 as libc::c_int as
                                                              isize)).sInit.type_0
                                               as libc::c_uint !=
                                               IT_INDEX as libc::c_int as
                                                   libc::c_uint {
                                            scrv_error(b"Typemismatch for variable %d\x00"
                                                           as *const u8 as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char,
                                                       (*yyvsp.offset(-(2 as
                                                                            libc::c_int)
                                                                          as
                                                                          isize)).vindex);
                                            current_block =
                                                9549167192516842735;
                                            break ;
                                        } else if scrvGetBaseObj((*yyvsp.offset(0
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    isize)).sInit.index
                                                                     as
                                                                     UDWORD,
                                                                 &mut psObj)
                                                      == 0 {
                                            scrv_error(b"Structure id %d not found\x00"
                                                           as *const u8 as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char,
                                                       (*yyvsp.offset(0 as
                                                                          libc::c_int
                                                                          as
                                                                          isize)).sInit.index
                                                           as UDWORD);
                                            current_block =
                                                9549167192516842735;
                                            break ;
                                        } else if (*psObj).type_0 as
                                                      libc::c_uint !=
                                                      OBJ_STRUCTURE as
                                                          libc::c_int as
                                                          libc::c_uint {
                                            scrv_error(b"Object id %d is not a structure\x00"
                                                           as *const u8 as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char,
                                                       (*yyvsp.offset(0 as
                                                                          libc::c_int
                                                                          as
                                                                          isize)).sInit.index
                                                           as UDWORD);
                                            current_block =
                                                9549167192516842735;
                                            break ;
                                        } else if eventSetContextVar(psCurrContext,
                                                                     (*yyvsp.offset(-(2
                                                                                          as
                                                                                          libc::c_int)
                                                                                        as
                                                                                        isize)).vindex,
                                                                     (*yyvsp.offset(-(1
                                                                                          as
                                                                                          libc::c_int)
                                                                                        as
                                                                                        isize)).tval,
                                                                     psObj as
                                                                         UDWORD)
                                                      == 0 {
                                            scrv_error(b"Set Value Failed for %s\x00"
                                                           as *const u8 as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char,
                                                       (*yyvsp.offset(-(2 as
                                                                            libc::c_int)
                                                                          as
                                                                          isize)).vindex);
                                            current_block =
                                                9549167192516842735;
                                            break ;
                                        }
                                    }
                                    10 => {
                                        if (*yyvsp.offset(0 as libc::c_int as
                                                              isize)).sInit.type_0
                                               as libc::c_uint !=
                                               IT_INDEX as libc::c_int as
                                                   libc::c_uint {
                                            scrv_error(b"Typemismatch for variable %d\x00"
                                                           as *const u8 as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char,
                                                       (*yyvsp.offset(-(2 as
                                                                            libc::c_int)
                                                                          as
                                                                          isize)).vindex);
                                            current_block =
                                                9549167192516842735;
                                            break ;
                                        } else if scrvGetBaseObj((*yyvsp.offset(0
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    isize)).sInit.index
                                                                     as
                                                                     UDWORD,
                                                                 &mut psObj)
                                                      == 0 {
                                            scrv_error(b"Feature id %d not found\x00"
                                                           as *const u8 as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char,
                                                       (*yyvsp.offset(0 as
                                                                          libc::c_int
                                                                          as
                                                                          isize)).sInit.index
                                                           as UDWORD);
                                            current_block =
                                                9549167192516842735;
                                            break ;
                                        } else if (*psObj).type_0 as
                                                      libc::c_uint !=
                                                      OBJ_FEATURE as
                                                          libc::c_int as
                                                          libc::c_uint {
                                            scrv_error(b"Object id %d is not a feature\x00"
                                                           as *const u8 as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char,
                                                       (*yyvsp.offset(0 as
                                                                          libc::c_int
                                                                          as
                                                                          isize)).sInit.index
                                                           as UDWORD);
                                            current_block =
                                                9549167192516842735;
                                            break ;
                                        } else if eventSetContextVar(psCurrContext,
                                                                     (*yyvsp.offset(-(2
                                                                                          as
                                                                                          libc::c_int)
                                                                                        as
                                                                                        isize)).vindex,
                                                                     (*yyvsp.offset(-(1
                                                                                          as
                                                                                          libc::c_int)
                                                                                        as
                                                                                        isize)).tval,
                                                                     psObj as
                                                                         UDWORD)
                                                      == 0 {
                                            scrv_error(b"Set Value Failed for %s\x00"
                                                           as *const u8 as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char,
                                                       (*yyvsp.offset(-(2 as
                                                                            libc::c_int)
                                                                          as
                                                                          isize)).vindex);
                                            current_block =
                                                9549167192516842735;
                                            break ;
                                        }
                                    }
                                    24 => {
                                        if (*yyvsp.offset(0 as libc::c_int as
                                                              isize)).sInit.type_0
                                               as libc::c_uint !=
                                               IT_STRING as libc::c_int as
                                                   libc::c_uint {
                                            scrv_error(b"Typemismatch for variable %d\x00"
                                                           as *const u8 as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char,
                                                       (*yyvsp.offset(-(2 as
                                                                            libc::c_int)
                                                                          as
                                                                          isize)).vindex);
                                            current_block =
                                                9549167192516842735;
                                            break ;
                                        } else {
                                            compIndex =
                                                getFeatureStatFromName((*yyvsp.offset(0
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          isize)).sInit.pString);
                                            if compIndex ==
                                                   -(1 as libc::c_int) {
                                                scrv_error(b"Feature Stat %s not found\x00"
                                                               as *const u8 as
                                                               *const libc::c_char
                                                               as
                                                               *mut libc::c_char,
                                                           (*yyvsp.offset(0 as
                                                                              libc::c_int
                                                                              as
                                                                              isize)).sInit.pString);
                                                current_block =
                                                    9549167192516842735;
                                                break ;
                                            } else if eventSetContextVar(psCurrContext,
                                                                         (*yyvsp.offset(-(2
                                                                                              as
                                                                                              libc::c_int)
                                                                                            as
                                                                                            isize)).vindex,
                                                                         (*yyvsp.offset(-(1
                                                                                              as
                                                                                              libc::c_int)
                                                                                            as
                                                                                            isize)).tval,
                                                                         compIndex
                                                                             as
                                                                             UDWORD)
                                                          == 0 {
                                                scrv_error(b"Set Value Failed for %s\x00"
                                                               as *const u8 as
                                                               *const libc::c_char
                                                               as
                                                               *mut libc::c_char,
                                                           (*yyvsp.offset(-(2
                                                                                as
                                                                                libc::c_int)
                                                                              as
                                                                              isize)).vindex);
                                                current_block =
                                                    9549167192516842735;
                                                break ;
                                            }
                                        }
                                    }
                                    0 => {
                                        if (*yyvsp.offset(0 as libc::c_int as
                                                              isize)).sInit.type_0
                                               as libc::c_uint !=
                                               IT_BOOL as libc::c_int as
                                                   libc::c_uint ||
                                               eventSetContextVar(psCurrContext,
                                                                  (*yyvsp.offset(-(2
                                                                                       as
                                                                                       libc::c_int)
                                                                                     as
                                                                                     isize)).vindex,
                                                                  (*yyvsp.offset(-(1
                                                                                       as
                                                                                       libc::c_int)
                                                                                     as
                                                                                     isize)).tval,
                                                                  (*yyvsp.offset(0
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize)).sInit.index
                                                                      as
                                                                      UDWORD)
                                                   == 0 {
                                            scrv_error(b"Typemismatch for variable %d\x00"
                                                           as *const u8 as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char,
                                                       (*yyvsp.offset(-(2 as
                                                                            libc::c_int)
                                                                          as
                                                                          isize)).vindex);
                                            current_block =
                                                9549167192516842735;
                                            break ;
                                        }
                                    }
                                    13 => {
                                        if (*yyvsp.offset(0 as libc::c_int as
                                                              isize)).sInit.type_0
                                               as libc::c_uint !=
                                               IT_STRING as libc::c_int as
                                                   libc::c_uint {
                                            scrv_error(b"Typemismatch for variable %d\x00"
                                                           as *const u8 as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char,
                                                       (*yyvsp.offset(-(2 as
                                                                            libc::c_int)
                                                                          as
                                                                          isize)).vindex);
                                            current_block =
                                                9549167192516842735;
                                            break ;
                                        } else {
                                            compIndex =
                                                getCompFromResName(COMP_BODY
                                                                       as
                                                                       libc::c_int
                                                                       as
                                                                       UDWORD,
                                                                   (*yyvsp.offset(0
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      isize)).sInit.pString);
                                            if compIndex ==
                                                   -(1 as libc::c_int) {
                                                scrv_error(b"body component %s not found\x00"
                                                               as *const u8 as
                                                               *const libc::c_char
                                                               as
                                                               *mut libc::c_char,
                                                           (*yyvsp.offset(0 as
                                                                              libc::c_int
                                                                              as
                                                                              isize)).sInit.pString);
                                                current_block =
                                                    9549167192516842735;
                                                break ;
                                            } else if eventSetContextVar(psCurrContext,
                                                                         (*yyvsp.offset(-(2
                                                                                              as
                                                                                              libc::c_int)
                                                                                            as
                                                                                            isize)).vindex,
                                                                         (*yyvsp.offset(-(1
                                                                                              as
                                                                                              libc::c_int)
                                                                                            as
                                                                                            isize)).tval,
                                                                         compIndex
                                                                             as
                                                                             UDWORD)
                                                          == 0 {
                                                scrv_error(b"Set Value Failed for %s\x00"
                                                               as *const u8 as
                                                               *const libc::c_char
                                                               as
                                                               *mut libc::c_char,
                                                           (*yyvsp.offset(-(2
                                                                                as
                                                                                libc::c_int)
                                                                              as
                                                                              isize)).vindex);
                                                current_block =
                                                    9549167192516842735;
                                                break ;
                                            }
                                        }
                                    }
                                    14 => {
                                        if (*yyvsp.offset(0 as libc::c_int as
                                                              isize)).sInit.type_0
                                               as libc::c_uint !=
                                               IT_STRING as libc::c_int as
                                                   libc::c_uint {
                                            scrv_error(b"Typemismatch for variable %d\x00"
                                                           as *const u8 as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char,
                                                       (*yyvsp.offset(-(2 as
                                                                            libc::c_int)
                                                                          as
                                                                          isize)).vindex);
                                            current_block =
                                                9549167192516842735;
                                            break ;
                                        } else {
                                            compIndex =
                                                getCompFromResName(COMP_PROPULSION
                                                                       as
                                                                       libc::c_int
                                                                       as
                                                                       UDWORD,
                                                                   (*yyvsp.offset(0
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      isize)).sInit.pString);
                                            if compIndex ==
                                                   -(1 as libc::c_int) {
                                                scrv_error(b"Propulsion component %s not found\x00"
                                                               as *const u8 as
                                                               *const libc::c_char
                                                               as
                                                               *mut libc::c_char,
                                                           (*yyvsp.offset(0 as
                                                                              libc::c_int
                                                                              as
                                                                              isize)).sInit.pString);
                                                current_block =
                                                    9549167192516842735;
                                                break ;
                                            } else if eventSetContextVar(psCurrContext,
                                                                         (*yyvsp.offset(-(2
                                                                                              as
                                                                                              libc::c_int)
                                                                                            as
                                                                                            isize)).vindex,
                                                                         (*yyvsp.offset(-(1
                                                                                              as
                                                                                              libc::c_int)
                                                                                            as
                                                                                            isize)).tval,
                                                                         compIndex
                                                                             as
                                                                             UDWORD)
                                                          == 0 {
                                                scrv_error(b"Set Value Failed for %s\x00"
                                                               as *const u8 as
                                                               *const libc::c_char
                                                               as
                                                               *mut libc::c_char,
                                                           (*yyvsp.offset(-(2
                                                                                as
                                                                                libc::c_int)
                                                                              as
                                                                              isize)).vindex);
                                                current_block =
                                                    9549167192516842735;
                                                break ;
                                            }
                                        }
                                    }
                                    15 => {
                                        if (*yyvsp.offset(0 as libc::c_int as
                                                              isize)).sInit.type_0
                                               as libc::c_uint !=
                                               IT_STRING as libc::c_int as
                                                   libc::c_uint {
                                            scrv_error(b"Typemismatch for variable %d\x00"
                                                           as *const u8 as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char,
                                                       (*yyvsp.offset(-(2 as
                                                                            libc::c_int)
                                                                          as
                                                                          isize)).vindex);
                                            current_block =
                                                9549167192516842735;
                                            break ;
                                        } else {
                                            compIndex =
                                                getCompFromResName(COMP_ECM as
                                                                       libc::c_int
                                                                       as
                                                                       UDWORD,
                                                                   (*yyvsp.offset(0
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      isize)).sInit.pString);
                                            if compIndex ==
                                                   -(1 as libc::c_int) {
                                                scrv_error(b"ECM component %s not found\x00"
                                                               as *const u8 as
                                                               *const libc::c_char
                                                               as
                                                               *mut libc::c_char,
                                                           (*yyvsp.offset(0 as
                                                                              libc::c_int
                                                                              as
                                                                              isize)).sInit.pString);
                                                current_block =
                                                    9549167192516842735;
                                                break ;
                                            } else if eventSetContextVar(psCurrContext,
                                                                         (*yyvsp.offset(-(2
                                                                                              as
                                                                                              libc::c_int)
                                                                                            as
                                                                                            isize)).vindex,
                                                                         (*yyvsp.offset(-(1
                                                                                              as
                                                                                              libc::c_int)
                                                                                            as
                                                                                            isize)).tval,
                                                                         compIndex
                                                                             as
                                                                             UDWORD)
                                                          == 0 {
                                                scrv_error(b"Set Value Failed for %s\x00"
                                                               as *const u8 as
                                                               *const libc::c_char
                                                               as
                                                               *mut libc::c_char,
                                                           (*yyvsp.offset(-(2
                                                                                as
                                                                                libc::c_int)
                                                                              as
                                                                              isize)).vindex);
                                                current_block =
                                                    9549167192516842735;
                                                break ;
                                            }
                                        }
                                    }
                                    16 => {
                                        if (*yyvsp.offset(0 as libc::c_int as
                                                              isize)).sInit.type_0
                                               as libc::c_uint !=
                                               IT_STRING as libc::c_int as
                                                   libc::c_uint {
                                            scrv_error(b"Typemismatch for variable %d\x00"
                                                           as *const u8 as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char,
                                                       (*yyvsp.offset(-(2 as
                                                                            libc::c_int)
                                                                          as
                                                                          isize)).vindex);
                                            current_block =
                                                9549167192516842735;
                                            break ;
                                        } else {
                                            compIndex =
                                                getCompFromResName(COMP_SENSOR
                                                                       as
                                                                       libc::c_int
                                                                       as
                                                                       UDWORD,
                                                                   (*yyvsp.offset(0
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      isize)).sInit.pString);
                                            if compIndex ==
                                                   -(1 as libc::c_int) {
                                                scrv_error(b"Sensor component %s not found\x00"
                                                               as *const u8 as
                                                               *const libc::c_char
                                                               as
                                                               *mut libc::c_char,
                                                           (*yyvsp.offset(0 as
                                                                              libc::c_int
                                                                              as
                                                                              isize)).sInit.pString);
                                                current_block =
                                                    9549167192516842735;
                                                break ;
                                            } else if eventSetContextVar(psCurrContext,
                                                                         (*yyvsp.offset(-(2
                                                                                              as
                                                                                              libc::c_int)
                                                                                            as
                                                                                            isize)).vindex,
                                                                         (*yyvsp.offset(-(1
                                                                                              as
                                                                                              libc::c_int)
                                                                                            as
                                                                                            isize)).tval,
                                                                         compIndex
                                                                             as
                                                                             UDWORD)
                                                          == 0 {
                                                scrv_error(b"Set Value Failed for %s\x00"
                                                               as *const u8 as
                                                               *const libc::c_char
                                                               as
                                                               *mut libc::c_char,
                                                           (*yyvsp.offset(-(2
                                                                                as
                                                                                libc::c_int)
                                                                              as
                                                                              isize)).vindex);
                                                current_block =
                                                    9549167192516842735;
                                                break ;
                                            }
                                        }
                                    }
                                    17 => {
                                        if (*yyvsp.offset(0 as libc::c_int as
                                                              isize)).sInit.type_0
                                               as libc::c_uint !=
                                               IT_STRING as libc::c_int as
                                                   libc::c_uint {
                                            scrv_error(b"Typemismatch for variable %d\x00"
                                                           as *const u8 as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char,
                                                       (*yyvsp.offset(-(2 as
                                                                            libc::c_int)
                                                                          as
                                                                          isize)).vindex);
                                            current_block =
                                                9549167192516842735;
                                            break ;
                                        } else {
                                            compIndex =
                                                getCompFromResName(COMP_CONSTRUCT
                                                                       as
                                                                       libc::c_int
                                                                       as
                                                                       UDWORD,
                                                                   (*yyvsp.offset(0
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      isize)).sInit.pString);
                                            if compIndex ==
                                                   -(1 as libc::c_int) {
                                                scrv_error(b"Construct component %s not found\x00"
                                                               as *const u8 as
                                                               *const libc::c_char
                                                               as
                                                               *mut libc::c_char,
                                                           (*yyvsp.offset(0 as
                                                                              libc::c_int
                                                                              as
                                                                              isize)).sInit.pString);
                                                current_block =
                                                    9549167192516842735;
                                                break ;
                                            } else if eventSetContextVar(psCurrContext,
                                                                         (*yyvsp.offset(-(2
                                                                                              as
                                                                                              libc::c_int)
                                                                                            as
                                                                                            isize)).vindex,
                                                                         (*yyvsp.offset(-(1
                                                                                              as
                                                                                              libc::c_int)
                                                                                            as
                                                                                            isize)).tval,
                                                                         compIndex
                                                                             as
                                                                             UDWORD)
                                                          == 0 {
                                                scrv_error(b"Set Value Failed for %s\x00"
                                                               as *const u8 as
                                                               *const libc::c_char
                                                               as
                                                               *mut libc::c_char,
                                                           (*yyvsp.offset(-(2
                                                                                as
                                                                                libc::c_int)
                                                                              as
                                                                              isize)).vindex);
                                                current_block =
                                                    9549167192516842735;
                                                break ;
                                            }
                                        }
                                    }
                                    19 => {
                                        if (*yyvsp.offset(0 as libc::c_int as
                                                              isize)).sInit.type_0
                                               as libc::c_uint !=
                                               IT_STRING as libc::c_int as
                                                   libc::c_uint {
                                            scrv_error(b"Typemismatch for variable %d\x00"
                                                           as *const u8 as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char,
                                                       (*yyvsp.offset(-(2 as
                                                                            libc::c_int)
                                                                          as
                                                                          isize)).vindex);
                                            current_block =
                                                9549167192516842735;
                                            break ;
                                        } else {
                                            compIndex =
                                                getCompFromResName(COMP_REPAIRUNIT
                                                                       as
                                                                       libc::c_int
                                                                       as
                                                                       UDWORD,
                                                                   (*yyvsp.offset(0
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      isize)).sInit.pString);
                                            if compIndex ==
                                                   -(1 as libc::c_int) {
                                                scrv_error(b"Repair component %s not found\x00"
                                                               as *const u8 as
                                                               *const libc::c_char
                                                               as
                                                               *mut libc::c_char,
                                                           (*yyvsp.offset(0 as
                                                                              libc::c_int
                                                                              as
                                                                              isize)).sInit.pString);
                                                current_block =
                                                    9549167192516842735;
                                                break ;
                                            } else if eventSetContextVar(psCurrContext,
                                                                         (*yyvsp.offset(-(2
                                                                                              as
                                                                                              libc::c_int)
                                                                                            as
                                                                                            isize)).vindex,
                                                                         (*yyvsp.offset(-(1
                                                                                              as
                                                                                              libc::c_int)
                                                                                            as
                                                                                            isize)).tval,
                                                                         compIndex
                                                                             as
                                                                             UDWORD)
                                                          == 0 {
                                                scrv_error(b"Set Value Failed for %s\x00"
                                                               as *const u8 as
                                                               *const libc::c_char
                                                               as
                                                               *mut libc::c_char,
                                                           (*yyvsp.offset(-(2
                                                                                as
                                                                                libc::c_int)
                                                                              as
                                                                              isize)).vindex);
                                                current_block =
                                                    9549167192516842735;
                                                break ;
                                            }
                                        }
                                    }
                                    20 => {
                                        if (*yyvsp.offset(0 as libc::c_int as
                                                              isize)).sInit.type_0
                                               as libc::c_uint !=
                                               IT_STRING as libc::c_int as
                                                   libc::c_uint {
                                            scrv_error(b"Typemismatch for variable %d\x00"
                                                           as *const u8 as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char,
                                                       (*yyvsp.offset(-(2 as
                                                                            libc::c_int)
                                                                          as
                                                                          isize)).vindex);
                                            current_block =
                                                9549167192516842735;
                                            break ;
                                        } else {
                                            compIndex =
                                                getCompFromResName(COMP_BRAIN
                                                                       as
                                                                       libc::c_int
                                                                       as
                                                                       UDWORD,
                                                                   (*yyvsp.offset(0
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      isize)).sInit.pString);
                                            if compIndex ==
                                                   -(1 as libc::c_int) {
                                                scrv_error(b"Brain component %s not found\x00"
                                                               as *const u8 as
                                                               *const libc::c_char
                                                               as
                                                               *mut libc::c_char,
                                                           (*yyvsp.offset(0 as
                                                                              libc::c_int
                                                                              as
                                                                              isize)).sInit.pString);
                                                current_block =
                                                    9549167192516842735;
                                                break ;
                                            } else if eventSetContextVar(psCurrContext,
                                                                         (*yyvsp.offset(-(2
                                                                                              as
                                                                                              libc::c_int)
                                                                                            as
                                                                                            isize)).vindex,
                                                                         (*yyvsp.offset(-(1
                                                                                              as
                                                                                              libc::c_int)
                                                                                            as
                                                                                            isize)).tval,
                                                                         compIndex
                                                                             as
                                                                             UDWORD)
                                                          == 0 {
                                                scrv_error(b"Set Value Failed for %s\x00"
                                                               as *const u8 as
                                                               *const libc::c_char
                                                               as
                                                               *mut libc::c_char,
                                                           (*yyvsp.offset(-(2
                                                                                as
                                                                                libc::c_int)
                                                                              as
                                                                              isize)).vindex);
                                                current_block =
                                                    9549167192516842735;
                                                break ;
                                            }
                                        }
                                    }
                                    18 => {
                                        if (*yyvsp.offset(0 as libc::c_int as
                                                              isize)).sInit.type_0
                                               as libc::c_uint !=
                                               IT_STRING as libc::c_int as
                                                   libc::c_uint {
                                            scrv_error(b"Typemismatch for variable %d\x00"
                                                           as *const u8 as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char,
                                                       (*yyvsp.offset(-(2 as
                                                                            libc::c_int)
                                                                          as
                                                                          isize)).vindex);
                                            current_block =
                                                9549167192516842735;
                                            break ;
                                        } else {
                                            compIndex =
                                                getCompFromResName(COMP_WEAPON
                                                                       as
                                                                       libc::c_int
                                                                       as
                                                                       UDWORD,
                                                                   (*yyvsp.offset(0
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      isize)).sInit.pString);
                                            if compIndex ==
                                                   -(1 as libc::c_int) {
                                                scrv_error(b"Weapon component %s not found\x00"
                                                               as *const u8 as
                                                               *const libc::c_char
                                                               as
                                                               *mut libc::c_char,
                                                           (*yyvsp.offset(0 as
                                                                              libc::c_int
                                                                              as
                                                                              isize)).sInit.pString);
                                                current_block =
                                                    9549167192516842735;
                                                break ;
                                            } else if eventSetContextVar(psCurrContext,
                                                                         (*yyvsp.offset(-(2
                                                                                              as
                                                                                              libc::c_int)
                                                                                            as
                                                                                            isize)).vindex,
                                                                         (*yyvsp.offset(-(1
                                                                                              as
                                                                                              libc::c_int)
                                                                                            as
                                                                                            isize)).tval,
                                                                         compIndex
                                                                             as
                                                                             UDWORD)
                                                          == 0 {
                                                scrv_error(b"Set Value Failed for %s\x00"
                                                               as *const u8 as
                                                               *const libc::c_char
                                                               as
                                                               *mut libc::c_char,
                                                           (*yyvsp.offset(-(2
                                                                                as
                                                                                libc::c_int)
                                                                              as
                                                                              isize)).vindex);
                                                current_block =
                                                    9549167192516842735;
                                                break ;
                                            }
                                        }
                                    }
                                    21 => {
                                        if (*yyvsp.offset(0 as libc::c_int as
                                                              isize)).sInit.type_0
                                               as libc::c_uint !=
                                               IT_STRING as libc::c_int as
                                                   libc::c_uint {
                                            scrv_error(b"Typemismatch for variable %d\x00"
                                                           as *const u8 as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char,
                                                       (*yyvsp.offset(-(2 as
                                                                            libc::c_int)
                                                                          as
                                                                          isize)).vindex);
                                            current_block =
                                                9549167192516842735;
                                            break ;
                                        } else {
                                            psTemplate =
                                                getTemplateFromName((*yyvsp.offset(0
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       isize)).sInit.pString);
                                            if psTemplate.is_null() {
                                                scrv_error(b"Template %s not found\x00"
                                                               as *const u8 as
                                                               *const libc::c_char
                                                               as
                                                               *mut libc::c_char,
                                                           (*yyvsp.offset(0 as
                                                                              libc::c_int
                                                                              as
                                                                              isize)).sInit.pString);
                                                current_block =
                                                    9549167192516842735;
                                                break ;
                                            } else if eventSetContextVar(psCurrContext,
                                                                         (*yyvsp.offset(-(2
                                                                                              as
                                                                                              libc::c_int)
                                                                                            as
                                                                                            isize)).vindex,
                                                                         (*yyvsp.offset(-(1
                                                                                              as
                                                                                              libc::c_int)
                                                                                            as
                                                                                            isize)).tval,
                                                                         psTemplate
                                                                             as
                                                                             UDWORD)
                                                          == 0 {
                                                scrv_error(b"Set Value Failed for %s\x00"
                                                               as *const u8 as
                                                               *const libc::c_char
                                                               as
                                                               *mut libc::c_char,
                                                           (*yyvsp.offset(-(2
                                                                                as
                                                                                libc::c_int)
                                                                              as
                                                                              isize)).vindex);
                                                current_block =
                                                    9549167192516842735;
                                                break ;
                                            }
                                        }
                                    }
                                    23 => {
                                        if (*yyvsp.offset(0 as libc::c_int as
                                                              isize)).sInit.type_0
                                               as libc::c_uint !=
                                               IT_STRING as libc::c_int as
                                                   libc::c_uint {
                                            scrv_error(b"Typemismatch for variable %d\x00"
                                                           as *const u8 as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char,
                                                       (*yyvsp.offset(-(2 as
                                                                            libc::c_int)
                                                                          as
                                                                          isize)).vindex);
                                            current_block =
                                                9549167192516842735;
                                            break ;
                                        } else {
                                            compIndex =
                                                getStructStatFromName((*yyvsp.offset(0
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         isize)).sInit.pString);
                                            if compIndex ==
                                                   -(1 as libc::c_int) {
                                                scrv_error(b"Structure Stat %s not found\x00"
                                                               as *const u8 as
                                                               *const libc::c_char
                                                               as
                                                               *mut libc::c_char,
                                                           (*yyvsp.offset(0 as
                                                                              libc::c_int
                                                                              as
                                                                              isize)).sInit.pString);
                                                current_block =
                                                    9549167192516842735;
                                                break ;
                                            } else if eventSetContextVar(psCurrContext,
                                                                         (*yyvsp.offset(-(2
                                                                                              as
                                                                                              libc::c_int)
                                                                                            as
                                                                                            isize)).vindex,
                                                                         (*yyvsp.offset(-(1
                                                                                              as
                                                                                              libc::c_int)
                                                                                            as
                                                                                            isize)).tval,
                                                                         compIndex
                                                                             as
                                                                             UDWORD)
                                                          == 0 {
                                                scrv_error(b"Set Value Failed for %s\x00"
                                                               as *const u8 as
                                                               *const libc::c_char
                                                               as
                                                               *mut libc::c_char,
                                                           (*yyvsp.offset(-(2
                                                                                as
                                                                                libc::c_int)
                                                                              as
                                                                              isize)).vindex);
                                                current_block =
                                                    9549167192516842735;
                                                break ;
                                            }
                                        }
                                    }
                                    22 => {
                                        if (*yyvsp.offset(0 as libc::c_int as
                                                              isize)).sInit.type_0
                                               as libc::c_uint !=
                                               IT_INDEX as libc::c_int as
                                                   libc::c_uint {
                                            scrv_error(b"Typemismatch for variable %d\x00"
                                                           as *const u8 as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char,
                                                       (*yyvsp.offset(-(2 as
                                                                            libc::c_int)
                                                                          as
                                                                          isize)).vindex);
                                            current_block =
                                                9549167192516842735;
                                            break ;
                                        } else if scrvGetBaseObj((*yyvsp.offset(0
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    isize)).sInit.index
                                                                     as
                                                                     UDWORD,
                                                                 &mut psObj)
                                                      == 0 {
                                            scrv_error(b"Structure id %d not found\x00"
                                                           as *const u8 as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char,
                                                       (*yyvsp.offset(0 as
                                                                          libc::c_int
                                                                          as
                                                                          isize)).sInit.index
                                                           as UDWORD);
                                            current_block =
                                                9549167192516842735;
                                            break ;
                                        } else if (*psObj).type_0 as
                                                      libc::c_uint !=
                                                      OBJ_STRUCTURE as
                                                          libc::c_int as
                                                          libc::c_uint {
                                            scrv_error(b"Object id %d is not a structure\x00"
                                                           as *const u8 as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char,
                                                       (*yyvsp.offset(0 as
                                                                          libc::c_int
                                                                          as
                                                                          isize)).sInit.index
                                                           as UDWORD);
                                            current_block =
                                                9549167192516842735;
                                            break ;
                                        } else if eventSetContextVar(psCurrContext,
                                                                     (*yyvsp.offset(-(2
                                                                                          as
                                                                                          libc::c_int)
                                                                                        as
                                                                                        isize)).vindex,
                                                                     (*yyvsp.offset(-(1
                                                                                          as
                                                                                          libc::c_int)
                                                                                        as
                                                                                        isize)).tval,
                                                                     (*yyvsp.offset(0
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)).sInit.index
                                                                         as
                                                                         UDWORD)
                                                      == 0 {
                                            scrv_error(b"Set Value Failed for %s\x00"
                                                           as *const u8 as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char,
                                                       (*yyvsp.offset(-(2 as
                                                                            libc::c_int)
                                                                          as
                                                                          isize)).vindex);
                                            current_block =
                                                9549167192516842735;
                                            break ;
                                        }
                                    }
                                    25 => {
                                        if (*yyvsp.offset(0 as libc::c_int as
                                                              isize)).sInit.type_0
                                               as libc::c_uint !=
                                               IT_INDEX as libc::c_int as
                                                   libc::c_uint {
                                            scrv_error(b"Typemismatch for variable %d\x00"
                                                           as *const u8 as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char,
                                                       (*yyvsp.offset(-(2 as
                                                                            libc::c_int)
                                                                          as
                                                                          isize)).vindex);
                                            current_block =
                                                9549167192516842735;
                                            break ;
                                        } else if scrvGetBaseObj((*yyvsp.offset(0
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    isize)).sInit.index
                                                                     as
                                                                     UDWORD,
                                                                 &mut psObj)
                                                      == 0 {
                                            scrv_error(b"Droid id %d not found\x00"
                                                           as *const u8 as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char,
                                                       (*yyvsp.offset(0 as
                                                                          libc::c_int
                                                                          as
                                                                          isize)).sInit.index
                                                           as UDWORD);
                                            current_block =
                                                9549167192516842735;
                                            break ;
                                        } else if (*psObj).type_0 as
                                                      libc::c_uint !=
                                                      OBJ_DROID as libc::c_int
                                                          as libc::c_uint {
                                            scrv_error(b"Object id %d is not a droid\x00"
                                                           as *const u8 as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char,
                                                       (*yyvsp.offset(0 as
                                                                          libc::c_int
                                                                          as
                                                                          isize)).sInit.index
                                                           as UDWORD);
                                            current_block =
                                                9549167192516842735;
                                            break ;
                                        } else if eventSetContextVar(psCurrContext,
                                                                     (*yyvsp.offset(-(2
                                                                                          as
                                                                                          libc::c_int)
                                                                                        as
                                                                                        isize)).vindex,
                                                                     (*yyvsp.offset(-(1
                                                                                          as
                                                                                          libc::c_int)
                                                                                        as
                                                                                        isize)).tval,
                                                                     (*yyvsp.offset(0
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)).sInit.index
                                                                         as
                                                                         UDWORD)
                                                      == 0 {
                                            scrv_error(b"Set Value Failed for %s\x00"
                                                           as *const u8 as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char,
                                                       (*yyvsp.offset(-(2 as
                                                                            libc::c_int)
                                                                          as
                                                                          isize)).vindex);
                                            current_block =
                                                9549167192516842735;
                                            break ;
                                        }
                                    }
                                    6 => {
                                        if (*yyvsp.offset(0 as libc::c_int as
                                                              isize)).sInit.type_0
                                               as libc::c_uint !=
                                               IT_STRING as libc::c_int as
                                                   libc::c_uint {
                                            scrv_error(b"Typemismatch for variable %d\x00"
                                                           as *const u8 as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char,
                                                       (*yyvsp.offset(-(2 as
                                                                            libc::c_int)
                                                                          as
                                                                          isize)).vindex);
                                            current_block =
                                                9549167192516842735;
                                            break ;
                                        } else {
                                            psViewData =
                                                getViewData((*yyvsp.offset(0
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize)).sInit.pString);
                                            if psViewData.is_null() {
                                                scrv_error(b"Message %s not found\x00"
                                                               as *const u8 as
                                                               *const libc::c_char
                                                               as
                                                               *mut libc::c_char,
                                                           (*yyvsp.offset(0 as
                                                                              libc::c_int
                                                                              as
                                                                              isize)).sInit.pString);
                                                current_block =
                                                    9549167192516842735;
                                                break ;
                                            } else if eventSetContextVar(psCurrContext,
                                                                         (*yyvsp.offset(-(2
                                                                                              as
                                                                                              libc::c_int)
                                                                                            as
                                                                                            isize)).vindex,
                                                                         (*yyvsp.offset(-(1
                                                                                              as
                                                                                              libc::c_int)
                                                                                            as
                                                                                            isize)).tval,
                                                                         psViewData
                                                                             as
                                                                             UDWORD)
                                                          == 0 {
                                                scrv_error(b"Set Value Failed for %s\x00"
                                                               as *const u8 as
                                                               *const libc::c_char
                                                               as
                                                               *mut libc::c_char,
                                                           (*yyvsp.offset(-(2
                                                                                as
                                                                                libc::c_int)
                                                                              as
                                                                              isize)).vindex);
                                                current_block =
                                                    9549167192516842735;
                                                break ;
                                            }
                                        }
                                    }
                                    26 => {
                                        if (*yyvsp.offset(0 as libc::c_int as
                                                              isize)).sInit.type_0
                                               as libc::c_uint !=
                                               IT_STRING as libc::c_int as
                                                   libc::c_uint {
                                            scrv_error(b"Typemismatch for variable %d\x00"
                                                           as *const u8 as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char,
                                                       (*yyvsp.offset(-(2 as
                                                                            libc::c_int)
                                                                          as
                                                                          isize)).vindex);
                                            current_block =
                                                9549167192516842735;
                                            break ;
                                        } else if scrvGetString((*yyvsp.offset(0
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize)).sInit.pString,
                                                                &mut pString)
                                                      == 0 {
                                            scrv_error(b"String %s not found\x00"
                                                           as *const u8 as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char,
                                                       (*yyvsp.offset(0 as
                                                                          libc::c_int
                                                                          as
                                                                          isize)).sInit.pString);
                                            current_block =
                                                9549167192516842735;
                                            break ;
                                        } else if eventSetContextVar(psCurrContext,
                                                                     (*yyvsp.offset(-(2
                                                                                          as
                                                                                          libc::c_int)
                                                                                        as
                                                                                        isize)).vindex,
                                                                     (*yyvsp.offset(-(1
                                                                                          as
                                                                                          libc::c_int)
                                                                                        as
                                                                                        isize)).tval,
                                                                     pString
                                                                         as
                                                                         UDWORD)
                                                      == 0 {
                                            scrv_error(b"Set Value Failed for %s\x00"
                                                           as *const u8 as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char,
                                                       (*yyvsp.offset(-(2 as
                                                                            libc::c_int)
                                                                          as
                                                                          isize)).vindex);
                                            current_block =
                                                9549167192516842735;
                                            break ;
                                        }
                                    }
                                    28 => {
                                        let mut psLevel: *mut LEVEL_DATASET =
                                            0 as *mut LEVEL_DATASET;
                                        if (*yyvsp.offset(0 as libc::c_int as
                                                              isize)).sInit.type_0
                                               as libc::c_uint !=
                                               IT_STRING as libc::c_int as
                                                   libc::c_uint {
                                            scrv_error(b"Typemismatch for variable %d\x00"
                                                           as *const u8 as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char,
                                                       (*yyvsp.offset(-(2 as
                                                                            libc::c_int)
                                                                          as
                                                                          isize)).vindex);
                                            current_block =
                                                9549167192516842735;
                                            break ;
                                        } else if levFindDataSet((*yyvsp.offset(0
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    isize)).sInit.pString,
                                                                 &mut psLevel)
                                                      == 0 {
                                            scrv_error(b"Level %s not found\x00"
                                                           as *const u8 as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char,
                                                       (*yyvsp.offset(0 as
                                                                          libc::c_int
                                                                          as
                                                                          isize)).sInit.pString);
                                            current_block =
                                                9549167192516842735;
                                            break ;
                                        } else if eventSetContextVar(psCurrContext,
                                                                     (*yyvsp.offset(-(2
                                                                                          as
                                                                                          libc::c_int)
                                                                                        as
                                                                                        isize)).vindex,
                                                                     (*yyvsp.offset(-(1
                                                                                          as
                                                                                          libc::c_int)
                                                                                        as
                                                                                        isize)).tval,
                                                                     (*psLevel).pName
                                                                         as
                                                                         UDWORD)
                                                      == 0 {
                                            scrv_error(b"Set Value Failed for %s\x00"
                                                           as *const u8 as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char,
                                                       (*yyvsp.offset(-(2 as
                                                                            libc::c_int)
                                                                          as
                                                                          isize)).vindex);
                                            current_block =
                                                9549167192516842735;
                                            break ;
                                        }
                                    }
                                    27 => {
                                        if (*yyvsp.offset(0 as libc::c_int as
                                                              isize)).sInit.type_0
                                               as libc::c_uint !=
                                               IT_STRING as libc::c_int as
                                                   libc::c_uint {
                                            scrv_error(b"Typemismatch for variable %d\x00"
                                                           as *const u8 as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char,
                                                       (*yyvsp.offset(-(2 as
                                                                            libc::c_int)
                                                                          as
                                                                          isize)).vindex);
                                            current_block =
                                                9549167192516842735;
                                            break ;
                                        } else {
                                            compIndex =
                                                audio_GetTrackID((*yyvsp.offset(0
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    isize)).sInit.pString);
                                            if compIndex ==
                                                   -(3 as libc::c_int) {
                                                audio_SetTrackVals((*yyvsp.offset(0
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      isize)).sInit.pString,
                                                                   0 as
                                                                       libc::c_int,
                                                                   &mut compIndex,
                                                                   100 as
                                                                       libc::c_int,
                                                                   1 as
                                                                       libc::c_int,
                                                                   1800 as
                                                                       libc::c_int,
                                                                   0 as
                                                                       libc::c_int);
                                            }
                                            if eventSetContextVar(psCurrContext,
                                                                  (*yyvsp.offset(-(2
                                                                                       as
                                                                                       libc::c_int)
                                                                                     as
                                                                                     isize)).vindex,
                                                                  (*yyvsp.offset(-(1
                                                                                       as
                                                                                       libc::c_int)
                                                                                     as
                                                                                     isize)).tval,
                                                                  compIndex as
                                                                      UDWORD)
                                                   == 0 {
                                                scrv_error(b"Set Value Failed for %s\x00"
                                                               as *const u8 as
                                                               *const libc::c_char
                                                               as
                                                               *mut libc::c_char,
                                                           (*yyvsp.offset(-(2
                                                                                as
                                                                                libc::c_int)
                                                                              as
                                                                              isize)).vindex);
                                                current_block =
                                                    9549167192516842735;
                                                break ;
                                            }
                                        }
                                    }
                                    30 => {
                                        if (*yyvsp.offset(0 as libc::c_int as
                                                              isize)).sInit.type_0
                                               as libc::c_uint !=
                                               IT_STRING as libc::c_int as
                                                   libc::c_uint {
                                            scrv_error(b"Typemismatch for variable %d\x00"
                                                           as *const u8 as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char,
                                                       (*yyvsp.offset(-(2 as
                                                                            libc::c_int)
                                                                          as
                                                                          isize)).vindex);
                                            current_block =
                                                9549167192516842735;
                                            break ;
                                        } else {
                                            psResearch =
                                                getResearch((*yyvsp.offset(0
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize)).sInit.pString,
                                                            1 as libc::c_int);
                                            if psResearch.is_null() {
                                                scrv_error(b"Research %s not found\x00"
                                                               as *const u8 as
                                                               *const libc::c_char
                                                               as
                                                               *mut libc::c_char,
                                                           (*yyvsp.offset(0 as
                                                                              libc::c_int
                                                                              as
                                                                              isize)).sInit.pString);
                                                current_block =
                                                    9549167192516842735;
                                                break ;
                                            } else if eventSetContextVar(psCurrContext,
                                                                         (*yyvsp.offset(-(2
                                                                                              as
                                                                                              libc::c_int)
                                                                                            as
                                                                                            isize)).vindex,
                                                                         (*yyvsp.offset(-(1
                                                                                              as
                                                                                              libc::c_int)
                                                                                            as
                                                                                            isize)).tval,
                                                                         psResearch
                                                                             as
                                                                             UDWORD)
                                                          == 0 {
                                                scrv_error(b"Set Value Failed for %s\x00"
                                                               as *const u8 as
                                                               *const libc::c_char
                                                               as
                                                               *mut libc::c_char,
                                                           (*yyvsp.offset(-(2
                                                                                as
                                                                                libc::c_int)
                                                                              as
                                                                              isize)).vindex);
                                                current_block =
                                                    9549167192516842735;
                                                break ;
                                            }
                                        }
                                    }
                                    _ => {
                                        scrv_error(b"Unknown type: %s\x00" as
                                                       *const u8 as
                                                       *const libc::c_char as
                                                       *mut libc::c_char,
                                                   (*asTypeTable.as_mut_ptr().offset((*yyvsp.offset(-(1
                                                                                                          as
                                                                                                          libc::c_int)
                                                                                                        as
                                                                                                        isize)).tval
                                                                                         as
                                                                                         isize)).pIdent);
                                        current_block = 9549167192516842735;
                                        break ;
                                    }
                                }
                            }
                            13 => {
                                sCurrArrayIndexes.dimensions =
                                    1 as libc::c_int;
                                sCurrArrayIndexes.elements[0 as libc::c_int as
                                                               usize] =
                                    (*yyvsp.offset(-(1 as libc::c_int) as
                                                       isize)).ival;
                                yyval.arrayIndex = &mut sCurrArrayIndexes
                            }
                            14 => {
                                yyval.arrayIndex =
                                    (*yyvsp.offset(0 as libc::c_int as
                                                       isize)).arrayIndex
                            }
                            15 => {
                                if (*(*yyvsp.offset(-(3 as libc::c_int) as
                                                        isize)).arrayIndex).dimensions
                                       >= 4 as libc::c_int {
                                    scrv_error(b"Too many dimensions for array\x00"
                                                   as *const u8 as
                                                   *const libc::c_char as
                                                   *mut libc::c_char);
                                    current_block = 9549167192516842735;
                                    break ;
                                } else {
                                    (*(*yyvsp.offset(-(3 as libc::c_int) as
                                                         isize)).arrayIndex).elements[(*(*yyvsp.offset(-(3
                                                                                                             as
                                                                                                             libc::c_int)
                                                                                                           as
                                                                                                           isize)).arrayIndex).dimensions
                                                                                          as
                                                                                          usize]
                                        =
                                        (*yyvsp.offset(-(1 as libc::c_int) as
                                                           isize)).ival;
                                    let ref mut fresh0 =
                                        (*(*yyvsp.offset(-(3 as libc::c_int)
                                                             as
                                                             isize)).arrayIndex).dimensions;
                                    *fresh0 += 1 as libc::c_int
                                }
                            }
                            16 => {
                                yyval.vindex =
                                    (*yyvsp.offset(0 as libc::c_int as
                                                       isize)).vindex
                            }
                            17 => {
                                let mut index: UDWORD = 0;
                                if scrvCheckArrayIndex((*yyvsp.offset(-(1 as
                                                                            libc::c_int)
                                                                          as
                                                                          isize)).vindex
                                                           as SDWORD,
                                                       (*yyvsp.offset(0 as
                                                                          libc::c_int
                                                                          as
                                                                          isize)).arrayIndex,
                                                       &mut index) == 0 {
                                    current_block = 9549167192516842735;
                                    break ;
                                }
                                yyval.vindex = index
                            }
                            18 => {
                                yyval.sInit.type_0 = IT_BOOL;
                                yyval.sInit.index =
                                    (*yyvsp.offset(0 as libc::c_int as
                                                       isize)).bval
                            }
                            19 => {
                                yyval.sInit.type_0 = IT_INDEX;
                                yyval.sInit.index =
                                    (*yyvsp.offset(0 as libc::c_int as
                                                       isize)).ival
                            }
                            20 => {
                                yyval.sInit.type_0 = IT_STRING;
                                yyval.sInit.pString =
                                    (*yyvsp.offset(0 as libc::c_int as
                                                       isize)).sval
                            }
                            _ => { }
                        }
                        /* User semantic actions sometimes alter yychar, and that requires
     that yytoken be updated with the new translation.  We take the
     approach of translating immediately before every use of yytoken.
     One alternative is translating here after every semantic action,
     but that translation would be missed if the semantic action invokes
     YYABORT, YYACCEPT, or YYERROR immediately after altering yychar or
     if it invokes YYBACKUP.  In the case of YYABORT or YYACCEPT, an
     incorrect destructor might then be invoked immediately.  In the
     case of YYERROR or YYBACKUP, subsequent parser actions might lead
     to an incorrect destructor call or verbose syntax error message
     before the lookahead is translated.  */
                        yyvsp = yyvsp.offset(-(yylen as isize));
                        yyssp = yyssp.offset(-(yylen as isize));
                        yylen = 0 as libc::c_int;
                        yyvsp = yyvsp.offset(1);
                        *yyvsp = yyval;
                        /* Now 'shift' the result of the reduction.  Determine what state
     that goes to, based on the state we popped back to and the rule
     number reduced by.  */
                        let yylhs: libc::c_int =
                            yyr1[yyn as usize] as libc::c_int -
                                17 as libc::c_int;
                        let yyi: libc::c_int =
                            yypgoto[yylhs as usize] as libc::c_int +
                                *yyssp as libc::c_int;
                        yystate =
                            if 0 as libc::c_int <= yyi &&
                                   yyi <= 33 as libc::c_int &&
                                   yycheck[yyi as usize] as libc::c_int ==
                                       *yyssp as libc::c_int {
                                yytable[yyi as usize] as libc::c_int
                            } else {
                                yydefgoto[yylhs as usize] as libc::c_int
                            }
                    }
                    _ => { }
                }
                /*------------------------------------------------------------.
| yynewstate -- push a new state, which is found in yystate.  |
`------------------------------------------------------------*/
                /* In all cases, when you get here, the value and location stacks
     have just been pushed.  So pushing a state here evens the stacks.  */
                yyssp = yyssp.offset(1)
            }
        }
    match current_block {
        8735508871533772611 =>
        /*-------------------------------------------------.
| yyexhaustedlab -- memory exhaustion comes here.  |
`-------------------------------------------------*/
        {
            scrv_error(b"memory exhausted\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char);
            yyresult = 2 as libc::c_int
        }
        9549167192516842735 =>
        /*-----------------------------------.
| yyabortlab -- YYABORT comes here.  |
`-----------------------------------*/
        {
            yyresult = 1 as libc::c_int
        }
        _ => { }
    }
    /* Fall through.  */
    /*-----------------------------------------------------.
| yyreturn -- parsing is finished, return the result.  |
`-----------------------------------------------------*/
    if scrv_char != -(2 as libc::c_int) {
        /* Make sure we have latest lookahead translation.  See comments at
         user semantic actions for why this is necessary.  */
        yytoken =
            if 0 as libc::c_int <= scrv_char &&
                   scrv_char <= 267 as libc::c_int {
                yytranslate[scrv_char as usize] as libc::c_int
            } else { 2 as libc::c_int };
        yydestruct(b"Cleanup: discarding lookahead\x00" as *const u8 as
                       *const libc::c_char, yytoken, &mut scrv_lval);
    }
    /* Do not reclaim the symbols of the rule whose action triggered
     this YYABORT or YYACCEPT.  */
    yyvsp = yyvsp.offset(-(yylen as isize));
    yyssp = yyssp.offset(-(yylen as isize));
    while yyssp != yyss {
        yydestruct(b"Cleanup: popping\x00" as *const u8 as
                       *const libc::c_char,
                   yystos[*yyssp as libc::c_int as usize] as libc::c_int,
                   yyvsp);
        yyvsp = yyvsp.offset(-(1 as libc::c_int as isize));
        yyssp = yyssp.offset(-(1 as libc::c_int as isize))
    }
    if yyss != yyssa.as_mut_ptr() { free(yyss as *mut libc::c_void); }
    return yyresult;
}
