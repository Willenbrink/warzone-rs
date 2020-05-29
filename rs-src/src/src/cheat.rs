use ::libc;
extern "C" {
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn addConsoleMessage(messageText: *mut STRING,
                         jusType: CONSOLE_TEXT_JUSTIFICATION) -> BOOL;
    #[no_mangle]
    fn kf_DebugDroidInfo();
    #[no_mangle]
    fn kf_BuildInfo();
    #[no_mangle]
    fn kf_FrameRate();
    #[no_mangle]
    fn kf_TogglePower();
    #[no_mangle]
    fn kf_AllAvailable();
    #[no_mangle]
    fn kf_AddMissionOffWorld();
    #[no_mangle]
    fn kf_ToggleGodMode();
    #[no_mangle]
    fn kf_ToggleDemoMode();
    #[no_mangle]
    fn kf_FinishResearch();
    #[no_mangle]
    fn kf_ToggleWeather();
    #[no_mangle]
    fn kf_KillSelected();
    #[no_mangle]
    fn kf_ToggleSensorDisplay();
    #[no_mangle]
    fn kf_ToggleShakeStatus();
    #[no_mangle]
    fn kf_ToggleMouseInvert();
    #[no_mangle]
    fn kf_SetKillerLevel();
    #[no_mangle]
    fn kf_SetEasyLevel();
    #[no_mangle]
    fn kf_SetNormalLevel();
    #[no_mangle]
    fn kf_SetToughUnitsLevel();
    #[no_mangle]
    fn kf_KillEnemy();
    #[no_mangle]
    fn kf_ToggleMissionTimer();
    #[no_mangle]
    fn kf_SetHardLevel();
    #[no_mangle]
    fn kf_NoFaults();
}
pub type STRING = libc::c_char;
pub type UDWORD = libc::c_uint;
pub type BOOL = libc::c_int;
pub type CONSOLE_TEXT_JUSTIFICATION = libc::c_uint;
pub const DEFAULT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 3;
pub const CENTRE_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 2;
pub const RIGHT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 1;
pub const LEFT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 0;
/* Handles cheat codes for Warzone */
/* Alex M 19th - Jan. 1999 */
pub type CHEAT_ENTRY = _cheat_entry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cheat_entry {
    pub pName: *mut STRING,
    pub function: Option<unsafe extern "C" fn() -> ()>,
}
#[no_mangle]
pub static mut cheatCodes: [CHEAT_ENTRY; 23] =
    unsafe {
        [{
             let mut init =
                 _cheat_entry{pName:
                                  b"give all\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              function:
                                  Some(kf_AllAvailable as
                                           unsafe extern "C" fn() -> ()),};
             init
         },
         {
             let mut init =
                 _cheat_entry{pName:
                                  b"deity\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              function:
                                  Some(kf_ToggleGodMode as
                                           unsafe extern "C" fn() -> ()),};
             init
         },
         {
             let mut init =
                 _cheat_entry{pName:
                                  b"droidinfo\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              function:
                                  Some(kf_DebugDroidInfo as
                                           unsafe extern "C" fn() -> ()),};
             init
         },
         {
             let mut init =
                 _cheat_entry{pName:
                                  b"sensors\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              function:
                                  Some(kf_ToggleSensorDisplay as
                                           unsafe extern "C" fn() -> ()),};
             init
         },
         {
             let mut init =
                 _cheat_entry{pName:
                                  b"let me win\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              function:
                                  Some(kf_AddMissionOffWorld as
                                           unsafe extern "C" fn() -> ()),};
             init
         },
         {
             let mut init =
                 _cheat_entry{pName:
                                  b"timedemo\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              function:
                                  Some(kf_FrameRate as
                                           unsafe extern "C" fn() -> ()),};
             init
         },
         {
             let mut init =
                 _cheat_entry{pName:
                                  b"kill\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              function:
                                  Some(kf_KillSelected as
                                           unsafe extern "C" fn() -> ()),};
             init
         },
         {
             let mut init =
                 _cheat_entry{pName:
                                  b"demo\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              function:
                                  Some(kf_ToggleDemoMode as
                                           unsafe extern "C" fn() -> ()),};
             init
         },
         {
             let mut init =
                 _cheat_entry{pName:
                                  b"john kettley\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              function:
                                  Some(kf_ToggleWeather as
                                           unsafe extern "C" fn() -> ()),};
             init
         },
         {
             let mut init =
                 _cheat_entry{pName:
                                  b"shakey\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              function:
                                  Some(kf_ToggleShakeStatus as
                                           unsafe extern "C" fn() -> ()),};
             init
         },
         {
             let mut init =
                 _cheat_entry{pName:
                                  b"mouseflip\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              function:
                                  Some(kf_ToggleMouseInvert as
                                           unsafe extern "C" fn() -> ()),};
             init
         },
         {
             let mut init =
                 _cheat_entry{pName:
                                  b"biffer baker\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              function:
                                  Some(kf_SetKillerLevel as
                                           unsafe extern "C" fn() -> ()),};
             init
         },
         {
             let mut init =
                 _cheat_entry{pName:
                                  b"easy\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              function:
                                  Some(kf_SetEasyLevel as
                                           unsafe extern "C" fn() -> ()),};
             init
         },
         {
             let mut init =
                 _cheat_entry{pName:
                                  b"normal\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              function:
                                  Some(kf_SetNormalLevel as
                                           unsafe extern "C" fn() -> ()),};
             init
         },
         {
             let mut init =
                 _cheat_entry{pName:
                                  b"hard\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              function:
                                  Some(kf_SetHardLevel as
                                           unsafe extern "C" fn() -> ()),};
             init
         },
         {
             let mut init =
                 _cheat_entry{pName:
                                  b"double up\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              function:
                                  Some(kf_SetToughUnitsLevel as
                                           unsafe extern "C" fn() -> ()),};
             init
         },
         {
             let mut init =
                 _cheat_entry{pName:
                                  b"whale fin\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              function:
                                  Some(kf_TogglePower as
                                           unsafe extern "C" fn() -> ()),};
             init
         },
         {
             let mut init =
                 _cheat_entry{pName:
                                  b"get off my land\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              function:
                                  Some(kf_KillEnemy as
                                           unsafe extern "C" fn() -> ()),};
             init
         },
         {
             let mut init =
                 _cheat_entry{pName:
                                  b"build info\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              function:
                                  Some(kf_BuildInfo as
                                           unsafe extern "C" fn() -> ()),};
             init
         },
         {
             let mut init =
                 _cheat_entry{pName:
                                  b"time toggle\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              function:
                                  Some(kf_ToggleMissionTimer as
                                           unsafe extern "C" fn() -> ()),};
             init
         },
         {
             let mut init =
                 _cheat_entry{pName:
                                  b"work harder\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              function:
                                  Some(kf_FinishResearch as
                                           unsafe extern "C" fn() -> ()),};
             init
         },
         {
             let mut init =
                 _cheat_entry{pName:
                                  b"no faults\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              function:
                                  Some(kf_NoFaults as
                                           unsafe extern "C" fn() -> ()),};
             init
         },
         {
             let mut init =
                 _cheat_entry{pName:
                                  b"end of list\x00" as *const u8 as
                                      *const libc::c_char as *mut STRING,
                              function: None,};
             init
         }]
    };
#[no_mangle]
pub unsafe extern "C" fn attemptCheatCode(mut pName: *mut STRING) -> BOOL {
    let mut index: UDWORD = 0;
    let mut errorString: [STRING; 255] = [0; 255];
    index = 0 as libc::c_int as UDWORD;
    while cheatCodes[index as usize].function.is_some() {
        if strcmp(pName, cheatCodes[index as usize].pName) == 0 as libc::c_int
           {
            // pointer to void* function
            /* We've got our man... */
            cheatCodes[index as
                           usize].function.expect("non-null function pointer")(); // run it
            /* And get out of here */
            return 1 as libc::c_int
        }
        index = index.wrapping_add(1)
    }
    /* We didn't find it */
    sprintf(errorString.as_mut_ptr(),
            b"%s?\x00" as *const u8 as *const libc::c_char, pName);
    addConsoleMessage(errorString.as_mut_ptr(), LEFT_JUSTIFY);
    return 0 as libc::c_int;
}
