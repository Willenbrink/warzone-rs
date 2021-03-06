extern BOOL DirectControl;
extern DROID *psDrivenDroid;

//static inline UWORD controlModeGet(void)
//{
//	return ControlMode;
//}
//
//
//static inline void controlModeSet(UWORD Mode)
//{
//	ControlMode = Mode;
//}
//
//
//static inline void	setDrivingStatus( BOOL val )
//{
//	bDriveMode = val;
//}
//
//static inline BOOL	getDrivingStatus( void )
//{
//	return(bDriveMode);
//}


static inline BOOL driveHasDriven(void)
{
	return (DirectControl) && (psDrivenDroid != NULL) ? TRUE : FALSE;
}


// Returns TRUE if drive mode is active.
//
static inline BOOL driveModeActive(void)
{
	return DirectControl;
}


// Return TRUE if the specified droid is the driven droid.
//
static inline BOOL driveIsDriven(DROID *psDroid)
{
	return (DirectControl) && (psDrivenDroid != NULL) && (psDroid == psDrivenDroid) ? TRUE : FALSE;
}


static inline BOOL driveIsFollower(DROID *psDroid)
{
	return (DirectControl) && (psDrivenDroid != NULL) && (psDroid != psDrivenDroid) && psDroid->selected ? TRUE : FALSE;
}


static inline DROID *driveGetDriven(void)
{
	return psDrivenDroid;
}



void driveInitVars(BOOL Restart);

BOOL StartDriverMode(DROID *psOldDroid);
void StopDriverMode(void);
//BOOL driveHasDriven(void);
//BOOL driveModeActive(void);
//BOOL driveIsDriven(DROID *psDroid);
//BOOL driveIsFollower(DROID *psDroid);
DROID *driveGetDriven(void);
BOOL driveDroidKilled(DROID *psDroid);
void driveSelectionChanged(void);
void driveNextDriver(void);
void driveUpdate(void);
void driveSetDroidMove(DROID *psDroid);
void setDrivingStatus( BOOL val );
BOOL getDrivingStatus( void );
void driveDisableControl(void);
void driveEnableControl(void);
void driveEnableInterface(BOOL AddReticule);
void driveDisableInterface(void);
BOOL driveInterfaceEnabled(void);
BOOL driveControlEnabled(void);
void driveProcessInterfaceButtons(void);
void driveAutoToggle(void);
void driveProcessAquireButton(void);
void driveProcessAquireTarget(void);
void driveMarkTarget(void);
void driveStartBuild(void);
BOOL driveAllowControl(void);
void driveEnableTactical(void);
void driveDisableTactical(void);
BOOL driveTacticalActive(void);
void driveTacticalSelectionChanged(void);
void driveProcessRadarInput(int x,int y);
BOOL driveWasDriving(void);
void driveDisableDriving(void);
void driveRestoreDriving(void);
SDWORD driveGetMoveSpeed(void);
SDWORD driveGetMoveDir(void);
BOOL driveSetDirectControl(BOOL Control);
BOOL driveSetWasDriving(BOOL Driving);

