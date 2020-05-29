use ::libc;
extern "C" {
    #[no_mangle]
    fn abs(_: libc::c_int) -> libc::c_int;
    // The maximum time for one frame (stops the clock running away when debugging)
// changed to /6 by ajl. if this needs to go back to ticks/10 then tell me. 
    /* The current time in the game world */
    #[no_mangle]
    static mut gameTime: UDWORD;
    #[no_mangle]
    static mut colours: [uint8; 0];
    #[no_mangle]
    fn pie_Line(x0: libc::c_int, y0: libc::c_int, x1: libc::c_int,
                y1: libc::c_int, colour: uint32);
}
pub type UBYTE = libc::c_uchar;
pub type UWORD = libc::c_ushort;
pub type SWORD = libc::c_short;
pub type UDWORD = libc::c_uint;
pub type BOOL = libc::c_int;
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
pub type _object_type = libc::c_uint;
pub const OBJ_TARGET: _object_type = 4;
pub const OBJ_BULLET: _object_type = 3;
pub const OBJ_FEATURE: _object_type = 2;
pub const OBJ_STRUCTURE: _object_type = 1;
pub const OBJ_DROID: _object_type = 0;
pub type OBJECT_TYPE = _object_type;
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
pub type BASE_OBJECT = _base_object;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TARGET {
    pub Type: UWORD,
    pub psObj: *mut BASE_OBJECT,
}
static mut FoundCurrent: BOOL = 0;
static mut NumTargets: UWORD = 0;
static mut TargetCurrent: UWORD = 0;
static mut TargetCurrentID: UDWORD = 0;
static mut TargetList: [TARGET; 32] =
    [TARGET{Type: 0, psObj: 0 as *const BASE_OBJECT as *mut BASE_OBJECT,};
        32];
static mut TargetEndTime: UDWORD = 0;
static mut TargetingObject: *mut BASE_OBJECT =
    0 as *const BASE_OBJECT as *mut BASE_OBJECT;
static mut TargetingType: UWORD = 0;
// Initialise the targeting system.
//
#[no_mangle]
pub unsafe extern "C" fn targetInitialise() {
    TargetCurrent = 0 as libc::c_int as UWORD;
    TargetCurrentID = 0xffffffff as libc::c_uint;
}
// Reset the target list, call once per frame.
//
#[no_mangle]
pub unsafe extern "C" fn targetOpenList(mut psTargeting: *mut BASE_OBJECT) {
    NumTargets = 0 as libc::c_int as UWORD;
    FoundCurrent = 0 as libc::c_int;
    TargetingObject = psTargeting;
}
#[no_mangle]
pub unsafe extern "C" fn targetCloseList() {
    (FoundCurrent) == 0;
    //DBPRINTF(("%d %d %d\n",NumTargets,TargetCurrent,TargetCurrentID);
}
// // Aquire a new target.
// //
//BASE_OBJECT *targetAquireNew(void)
//{
//	BASE_OBJECT *psObj;
//
//	// First try and target the nearest threat.
//	if( (psObj=targetAquireNearestObj(TargetingObject,TARGET_TYPE_THREAT)) != NULL) {
//		DBPRINTF(("Targeting threat\n"));
//		return psObj;
//	}
//
//	// if that failed then attempt to target the nearest object.
//	if( (psObj=targetAquireNearestObj(TargetingObject,TARGET_TYPE_ANY)) != NULL) {
//		DBPRINTF(("Targeting any\n"));
//		return psObj;
//	}
//
//	TargetCurrent = 0;
//	TargetCurrentID = UDWORD_MAX;
//
//	return NULL;
//}
/*
	DROID_WEAPON,		// Weapon droid
	DROID_SENSOR,		// Sensor droid
	DROID_ECM,			// ECM droid
	DROID_CONSTRUCT,	// Constructor droid
	DROID_PERSON,		// person
	DROID_CYBORG,		// cyborg-type thang
	DROID_TRANSPORTER,	// guess what this is!
	DROID_COMMAND,		// Command droid
	DROID_REPAIR,		// Repair droid
	DROID_DEFAULT,		// Default droid
	DROID_ANY,			// Any droid, Only used as a parameter for intGotoNextDroidType(droidtype).
*/
// Tell the targeting system what type of droid is doing the targeting.
//
#[no_mangle]
pub unsafe extern "C" fn targetSetTargetable(mut DroidType: UWORD) {
    TargetingType = DroidType;
}
#[no_mangle]
pub unsafe extern "C" fn targetAdd(mut psObj: *mut BASE_OBJECT) { }
// Call whenever an object is removed. probably don't need this as
// the list is rebuilt every frame any way.
//
#[no_mangle]
pub unsafe extern "C" fn targetKilledObject(mut psObj: *mut BASE_OBJECT) { }
// // Aquire the target nearest to the specified screen coords.
// //
//BASE_OBJECT *targetAquireNearestScreen(SWORD x,SWORD y,UWORD TargetType)
//{
//	UWORD i;
//	UWORD Nearesti = 0;
//	UDWORD NearestDsq = UDWORD_MAX;
//	UDWORD dx,dy;
//	UDWORD Dsq;
//	BASE_OBJECT *NearestObj = NULL;
//	BASE_OBJECT *psObj;
//
//	for(i=0; i<NumTargets; i++) {
//		psObj = TargetList[i].psObj;
//		dx = abs(psObj->sDisplay.screenX - x);
//		dy = abs(psObj->sDisplay.screenY - y);
//		Dsq = dx*dx+dy*dy;
//		if(Dsq < NearestDsq) {
//			if(TargetList[i].Type & TargetType) {
//				NearestDsq = Dsq;
//				Nearesti = i;
//				NearestObj = psObj;
//			}
//		}
//	}
//
//	if(NearestObj != NULL) {
//		TargetCurrent = Nearesti;
//		if(TargetCurrentID != NearestObj->id) {
//			TargetCurrentID = NearestObj->id;
//			targetStartAnim();
//		}
//	}
//
//	return NearestObj;
//}
// Aquire the target nearest the vector from x,y to the top of the screen.
//
#[no_mangle]
pub unsafe extern "C" fn targetAquireNearestView(mut x: SWORD, mut y: SWORD,
                                                 mut TargetType: UWORD)
 -> *mut BASE_OBJECT {
    let mut i: UWORD = 0;
    let mut Nearesti: UWORD = 0 as libc::c_int as UWORD;
    //	UDWORD NearestDsq = UDWORD_MAX;
    let mut NearestDx: UDWORD = 0xffffffff as libc::c_uint;
    let mut dx: UDWORD = 0;
    let mut dy: UDWORD = 0;
    //	UDWORD Dsq;
    let mut NearestObj: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut psObj: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    i = 0 as libc::c_int as UWORD;
    while (i as libc::c_int) < NumTargets as libc::c_int {
        psObj = TargetList[i as usize].psObj;
        dx =
            abs((*psObj).sDisplay.screenX.wrapping_sub(x as libc::c_uint) as
                    libc::c_int) as UDWORD;
        dy =
            abs((*psObj).sDisplay.screenY.wrapping_sub(y as libc::c_uint) as
                    libc::c_int) as UDWORD;
        //		if(Dsq < NearestDsq) {
//			if(TargetList[i].Type & TargetType) {
//				NearestDsq = Dsq;
//				Nearesti = i;
//				NearestObj = psObj;
//			}
//		}
        dx =
            (dx as
                 libc::c_uint).wrapping_add(dy.wrapping_div(2 as libc::c_int
                                                                as
                                                                libc::c_uint))
                as UDWORD as UDWORD;
        if dx < NearestDx {
            if TargetList[i as usize].Type as libc::c_int &
                   TargetType as libc::c_int != 0 {
                NearestDx = dx;
                Nearesti = i;
                NearestObj = psObj
            }
        }
        i = i.wrapping_add(1)
    }
    if !NearestObj.is_null() {
        TargetCurrent = Nearesti;
        if TargetCurrentID != (*NearestObj).id {
            //		Dsq = dx*dx+dy*dy*4;
            //printf("set1 %d\n",TargetCurrentID);
            TargetCurrentID = (*NearestObj).id;
            targetStartAnim();
        }
    } else { TargetCurrentID = 0xffffffff as libc::c_uint }
    return NearestObj;
}
// // Aquire the target nearest to the specified object.
// //
//BASE_OBJECT *targetAquireNearestObj(BASE_OBJECT *psObj,UWORD TargetType)
//{
//	if(psObj != NULL) {
//		return targetAquireNearestScreen(psObj->sDisplay.screenX,psObj->sDisplay.screenY,TargetType);
//	} else {
//		return NULL;
//	}
//}
// Aquire the target nearest to the specified object.
//
#[no_mangle]
pub unsafe extern "C" fn targetAquireNearestObjView(mut psObj:
                                                        *mut BASE_OBJECT,
                                                    mut TargetType: UWORD)
 -> *mut BASE_OBJECT {
    if !psObj.is_null() {
        return targetAquireNearestView((*psObj).sDisplay.screenX as SWORD,
                                       (*psObj).sDisplay.screenY as SWORD,
                                       TargetType)
    } else { return 0 as *mut BASE_OBJECT };
}
// // Aquire the next target in the target list.
// //
//BASE_OBJECT *targetAquireNext(UWORD TargetType)
//{
//	BASE_OBJECT *Target = NULL;
//
//	if(NumTargets) {
//		UWORD OldCurrent = TargetCurrent;
//
//		TargetCurrent++;
//
//		while( (Target == NULL) && (TargetCurrent < NumTargets) ) {
//			// Target is of required type?
//			if(TargetList[TargetCurrent].Type & TargetType) {
//				Target = TargetList[TargetCurrent].psObj;
//			}
//
//			TargetCurrent++;
//		}
//
//
//		if(Target == NULL) {
//			TargetCurrent = 0;
//
//			while( (Target == NULL) && (TargetCurrent < OldCurrent) ) {
//				// Target is of required type?
//				if(TargetList[TargetCurrent].Type & TargetType) {
//					Target = TargetList[TargetCurrent].psObj;
//				}
//
//				TargetCurrent++;
//			}
//		}
//
//		if(TargetCurrent >= NumTargets) {
//			TargetCurrent = 0;
//		}
//	}
//
//	if(Target != NULL) {
//		if(TargetCurrentID != Target->id) {
//			TargetCurrentID = Target->id;
//			targetStartAnim();
//		}
//	}
//
//	return Target;
//}
// Get the currently targeted object.
//
#[no_mangle]
pub unsafe extern "C" fn targetGetCurrent() -> *mut BASE_OBJECT {
    if TargetCurrentID != 0xffffffff as libc::c_uint {
        return TargetList[TargetCurrent as usize].psObj
    }
    return 0 as *mut BASE_OBJECT;
}
// Start the box zoom animation.
//
#[no_mangle]
pub unsafe extern "C" fn targetStartAnim() {
    TargetEndTime =
        gameTime.wrapping_add((1000 as libc::c_int / 2 as libc::c_int) as
                                  libc::c_uint);
}
// Display a marker over the current target.
//
#[no_mangle]
pub unsafe extern "C" fn targetMarkCurrent() {
    let mut x: SWORD = 0;
    let mut y: SWORD = 0;
    let mut Offset: SWORD = 0;
    let mut x0: SWORD = 0;
    let mut y0: SWORD = 0;
    let mut x1: SWORD = 0;
    let mut y1: SWORD = 0;
    //printf("%d\n",TargetCurrentID);
    if TargetCurrentID == 0xffffffff as libc::c_uint { return }
    x = (*TargetList[TargetCurrent as usize].psObj).sDisplay.screenX as SWORD;
    y = (*TargetList[TargetCurrent as usize].psObj).sDisplay.screenY as SWORD;
    // Make it zoom in.
    if TargetEndTime > gameTime {
        Offset =
            (16 as libc::c_int as
                 libc::c_uint).wrapping_add(TargetEndTime.wrapping_sub(gameTime).wrapping_div(2
                                                                                                  as
                                                                                                  libc::c_int
                                                                                                  as
                                                                                                  libc::c_uint))
                as SWORD
    } else { Offset = 16 as libc::c_int as SWORD }
    x0 = (x as libc::c_int - Offset as libc::c_int) as SWORD;
    y0 = (y as libc::c_int - Offset as libc::c_int) as SWORD;
    x1 = (x as libc::c_int + Offset as libc::c_int) as SWORD;
    y1 = (y as libc::c_int + Offset as libc::c_int) as SWORD;
    pie_Line(x0 as libc::c_int, y0 as libc::c_int,
             x0 as libc::c_int + 8 as libc::c_int, y0 as libc::c_int,
             *colours.as_mut_ptr().offset(14 as libc::c_int as isize) as
                 uint32);
    pie_Line(x0 as libc::c_int, y0 as libc::c_int, x0 as libc::c_int,
             y0 as libc::c_int + 8 as libc::c_int,
             *colours.as_mut_ptr().offset(14 as libc::c_int as isize) as
                 uint32);
    pie_Line(x1 as libc::c_int, y0 as libc::c_int,
             x1 as libc::c_int - 8 as libc::c_int, y0 as libc::c_int,
             *colours.as_mut_ptr().offset(14 as libc::c_int as isize) as
                 uint32);
    pie_Line(x1 as libc::c_int, y0 as libc::c_int, x1 as libc::c_int,
             y0 as libc::c_int + 8 as libc::c_int,
             *colours.as_mut_ptr().offset(14 as libc::c_int as isize) as
                 uint32);
    pie_Line(x1 as libc::c_int, y1 as libc::c_int,
             x1 as libc::c_int - 8 as libc::c_int, y1 as libc::c_int,
             *colours.as_mut_ptr().offset(14 as libc::c_int as isize) as
                 uint32);
    pie_Line(x1 as libc::c_int, y1 as libc::c_int, x1 as libc::c_int,
             y1 as libc::c_int - 8 as libc::c_int,
             *colours.as_mut_ptr().offset(14 as libc::c_int as isize) as
                 uint32);
    pie_Line(x0 as libc::c_int, y1 as libc::c_int,
             x0 as libc::c_int + 8 as libc::c_int, y1 as libc::c_int,
             *colours.as_mut_ptr().offset(14 as libc::c_int as isize) as
                 uint32);
    pie_Line(x0 as libc::c_int, y1 as libc::c_int, x0 as libc::c_int,
             y1 as libc::c_int - 8 as libc::c_int,
             *colours.as_mut_ptr().offset(14 as libc::c_int as isize) as
                 uint32);
}
