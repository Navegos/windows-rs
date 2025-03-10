windows_targets::link!("advpack.dll" "system" fn AddDelBackupEntryA(lpcszfilelist : windows_sys::core::PCSTR, lpcszbackupdir : windows_sys::core::PCSTR, lpcszbasename : windows_sys::core::PCSTR, dwflags : u32) -> windows_sys::core::HRESULT);
windows_targets::link!("advpack.dll" "system" fn AddDelBackupEntryW(lpcszfilelist : windows_sys::core::PCWSTR, lpcszbackupdir : windows_sys::core::PCWSTR, lpcszbasename : windows_sys::core::PCWSTR, dwflags : u32) -> windows_sys::core::HRESULT);
windows_targets::link!("advpack.dll" "system" fn AdvInstallFileA(hwnd : super::super::Foundation:: HWND, lpszsourcedir : windows_sys::core::PCSTR, lpszsourcefile : windows_sys::core::PCSTR, lpszdestdir : windows_sys::core::PCSTR, lpszdestfile : windows_sys::core::PCSTR, dwflags : u32, dwreserved : u32) -> windows_sys::core::HRESULT);
windows_targets::link!("advpack.dll" "system" fn AdvInstallFileW(hwnd : super::super::Foundation:: HWND, lpszsourcedir : windows_sys::core::PCWSTR, lpszsourcefile : windows_sys::core::PCWSTR, lpszdestdir : windows_sys::core::PCWSTR, lpszdestfile : windows_sys::core::PCWSTR, dwflags : u32, dwreserved : u32) -> windows_sys::core::HRESULT);
windows_targets::link!("apphelp.dll" "system" fn ApphelpCheckShellObject(objectclsid : *const windows_sys::core::GUID, bshimifnecessary : windows_sys::core::BOOL, pullflags : *mut u64) -> windows_sys::core::BOOL);
windows_targets::link!("kernel32.dll" "system" fn CancelDeviceWakeupRequest(hdevice : super::super::Foundation:: HANDLE) -> windows_sys::core::BOOL);
windows_targets::link!("advpack.dll" "system" fn CloseINFEngine(hinf : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("api-ms-win-core-realtime-l1-1-2.dll" "system" fn ConvertAuxiliaryCounterToPerformanceCounter(ullauxiliarycountervalue : u64, lpperformancecountervalue : *mut u64, lpconversionerror : *mut u64) -> windows_sys::core::HRESULT);
windows_targets::link!("api-ms-win-core-realtime-l1-1-2.dll" "system" fn ConvertPerformanceCounterToAuxiliaryCounter(ullperformancecountervalue : u64, lpauxiliarycountervalue : *mut u64, lpconversionerror : *mut u64) -> windows_sys::core::HRESULT);
windows_targets::link!("dciman32.dll" "system" fn DCIBeginAccess(pdci : *mut DCISURFACEINFO, x : i32, y : i32, dx : i32, dy : i32) -> i32);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("dciman32.dll" "system" fn DCICloseProvider(hdc : super::super::Graphics::Gdi:: HDC));
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("dciman32.dll" "system" fn DCICreateOffscreen(hdc : super::super::Graphics::Gdi:: HDC, dwcompression : u32, dwredmask : u32, dwgreenmask : u32, dwbluemask : u32, dwwidth : u32, dwheight : u32, dwdcicaps : u32, dwbitcount : u32, lplpsurface : *mut *mut DCIOFFSCREEN) -> i32);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("dciman32.dll" "system" fn DCICreateOverlay(hdc : super::super::Graphics::Gdi:: HDC, lpoffscreensurf : *mut core::ffi::c_void, lplpsurface : *mut *mut DCIOVERLAY) -> i32);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("dciman32.dll" "system" fn DCICreatePrimary(hdc : super::super::Graphics::Gdi:: HDC, lplpsurface : *mut *mut DCISURFACEINFO) -> i32);
windows_targets::link!("dciman32.dll" "system" fn DCIDestroy(pdci : *mut DCISURFACEINFO));
windows_targets::link!("dciman32.dll" "system" fn DCIDraw(pdci : *mut DCIOFFSCREEN) -> i32);
windows_targets::link!("dciman32.dll" "system" fn DCIEndAccess(pdci : *mut DCISURFACEINFO));
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("dciman32.dll" "system" fn DCIEnum(hdc : super::super::Graphics::Gdi:: HDC, lprdst : *mut super::super::Foundation:: RECT, lprsrc : *mut super::super::Foundation:: RECT, lpfncallback : *mut core::ffi::c_void, lpcontext : *mut core::ffi::c_void) -> i32);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("dciman32.dll" "system" fn DCIOpenProvider() -> super::super::Graphics::Gdi:: HDC);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("dciman32.dll" "system" fn DCISetClipList(pdci : *mut DCIOFFSCREEN, prd : *mut super::super::Graphics::Gdi:: RGNDATA) -> i32);
windows_targets::link!("dciman32.dll" "system" fn DCISetDestination(pdci : *mut DCIOFFSCREEN, dst : *mut super::super::Foundation:: RECT, src : *mut super::super::Foundation:: RECT) -> i32);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("dciman32.dll" "system" fn DCISetSrcDestClip(pdci : *mut DCIOFFSCREEN, srcrc : *mut super::super::Foundation:: RECT, destrc : *mut super::super::Foundation:: RECT, prd : *mut super::super::Graphics::Gdi:: RGNDATA) -> i32);
windows_targets::link!("advpack.dll" "system" fn DelNodeA(pszfileordirname : windows_sys::core::PCSTR, dwflags : u32) -> windows_sys::core::HRESULT);
windows_targets::link!("advpack.dll" "system" fn DelNodeRunDLL32W(hwnd : super::super::Foundation:: HWND, hinstance : super::super::Foundation:: HINSTANCE, pszparms : windows_sys::core::PWSTR, nshow : i32) -> windows_sys::core::HRESULT);
windows_targets::link!("advpack.dll" "system" fn DelNodeW(pszfileordirname : windows_sys::core::PCWSTR, dwflags : u32) -> windows_sys::core::HRESULT);
windows_targets::link!("kernel32.dll" "system" fn DnsHostnameToComputerNameA(hostname : windows_sys::core::PCSTR, computername : windows_sys::core::PSTR, nsize : *mut u32) -> windows_sys::core::BOOL);
windows_targets::link!("kernel32.dll" "system" fn DnsHostnameToComputerNameW(hostname : windows_sys::core::PCWSTR, computername : windows_sys::core::PWSTR, nsize : *mut u32) -> windows_sys::core::BOOL);
windows_targets::link!("kernel32.dll" "system" fn DosDateTimeToFileTime(wfatdate : u16, wfattime : u16, lpfiletime : *mut super::super::Foundation:: FILETIME) -> windows_sys::core::BOOL);
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
windows_targets::link!("kernel32.dll" "system" fn EnableProcessOptionalXStateFeatures(features : u64) -> windows_sys::core::BOOL);
windows_targets::link!("advpack.dll" "system" fn ExecuteCabA(hwnd : super::super::Foundation:: HWND, pcab : *mut CABINFOA, preserved : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("advpack.dll" "system" fn ExecuteCabW(hwnd : super::super::Foundation:: HWND, pcab : *mut CABINFOW, preserved : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("advpack.dll" "system" fn ExtractFilesA(pszcabname : windows_sys::core::PCSTR, pszexpanddir : windows_sys::core::PCSTR, dwflags : u32, pszfilelist : windows_sys::core::PCSTR, lpreserved : *mut core::ffi::c_void, dwreserved : u32) -> windows_sys::core::HRESULT);
windows_targets::link!("advpack.dll" "system" fn ExtractFilesW(pszcabname : windows_sys::core::PCWSTR, pszexpanddir : windows_sys::core::PCWSTR, dwflags : u32, pszfilelist : windows_sys::core::PCWSTR, lpreserved : *mut core::ffi::c_void, dwreserved : u32) -> windows_sys::core::HRESULT);
windows_targets::link!("advpack.dll" "system" fn FileSaveMarkNotExistA(lpfilelist : windows_sys::core::PCSTR, lpdir : windows_sys::core::PCSTR, lpbasename : windows_sys::core::PCSTR) -> windows_sys::core::HRESULT);
windows_targets::link!("advpack.dll" "system" fn FileSaveMarkNotExistW(lpfilelist : windows_sys::core::PCWSTR, lpdir : windows_sys::core::PCWSTR, lpbasename : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
windows_targets::link!("advpack.dll" "system" fn FileSaveRestoreOnINFA(hwnd : super::super::Foundation:: HWND, psztitle : windows_sys::core::PCSTR, pszinf : windows_sys::core::PCSTR, pszsection : windows_sys::core::PCSTR, pszbackupdir : windows_sys::core::PCSTR, pszbasebackupfile : windows_sys::core::PCSTR, dwflags : u32) -> windows_sys::core::HRESULT);
windows_targets::link!("advpack.dll" "system" fn FileSaveRestoreOnINFW(hwnd : super::super::Foundation:: HWND, psztitle : windows_sys::core::PCWSTR, pszinf : windows_sys::core::PCWSTR, pszsection : windows_sys::core::PCWSTR, pszbackupdir : windows_sys::core::PCWSTR, pszbasebackupfile : windows_sys::core::PCWSTR, dwflags : u32) -> windows_sys::core::HRESULT);
windows_targets::link!("advpack.dll" "system" fn FileSaveRestoreW(hdlg : super::super::Foundation:: HWND, lpfilelist : windows_sys::core::PCWSTR, lpdir : windows_sys::core::PCWSTR, lpbasename : windows_sys::core::PCWSTR, dwflags : u32) -> windows_sys::core::HRESULT);
windows_targets::link!("kernel32.dll" "system" fn FileTimeToDosDateTime(lpfiletime : *const super::super::Foundation:: FILETIME, lpfatdate : *mut u16, lpfattime : *mut u16) -> windows_sys::core::BOOL);
windows_targets::link!("api-ms-win-dx-d3dkmt-l1-1-0.dll" "system" fn GdiEntry13() -> u32);
windows_targets::link!("kernel32.dll" "system" fn GetComputerNameA(lpbuffer : windows_sys::core::PSTR, nsize : *mut u32) -> windows_sys::core::BOOL);
windows_targets::link!("kernel32.dll" "system" fn GetComputerNameW(lpbuffer : windows_sys::core::PWSTR, nsize : *mut u32) -> windows_sys::core::BOOL);
windows_targets::link!("advapi32.dll" "system" fn GetCurrentHwProfileA(lphwprofileinfo : *mut HW_PROFILE_INFOA) -> windows_sys::core::BOOL);
windows_targets::link!("advapi32.dll" "system" fn GetCurrentHwProfileW(lphwprofileinfo : *mut HW_PROFILE_INFOW) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("dciman32.dll" "system" fn GetDCRegionData(hdc : super::super::Graphics::Gdi:: HDC, size : u32, prd : *mut super::super::Graphics::Gdi:: RGNDATA) -> u32);
windows_targets::link!("api-ms-win-core-featurestaging-l1-1-0.dll" "system" fn GetFeatureEnabledState(featureid : u32, changetime : FEATURE_CHANGE_TIME) -> FEATURE_ENABLED_STATE);
windows_targets::link!("api-ms-win-core-featurestaging-l1-1-1.dll" "system" fn GetFeatureVariant(featureid : u32, changetime : FEATURE_CHANGE_TIME, payloadid : *mut u32, hasnotification : *mut windows_sys::core::BOOL) -> u32);
windows_targets::link!("kernel32.dll" "system" fn GetFirmwareEnvironmentVariableA(lpname : windows_sys::core::PCSTR, lpguid : windows_sys::core::PCSTR, pbuffer : *mut core::ffi::c_void, nsize : u32) -> u32);
windows_targets::link!("kernel32.dll" "system" fn GetFirmwareEnvironmentVariableExA(lpname : windows_sys::core::PCSTR, lpguid : windows_sys::core::PCSTR, pbuffer : *mut core::ffi::c_void, nsize : u32, pdwattribubutes : *mut u32) -> u32);
windows_targets::link!("kernel32.dll" "system" fn GetFirmwareEnvironmentVariableExW(lpname : windows_sys::core::PCWSTR, lpguid : windows_sys::core::PCWSTR, pbuffer : *mut core::ffi::c_void, nsize : u32, pdwattribubutes : *mut u32) -> u32);
windows_targets::link!("kernel32.dll" "system" fn GetFirmwareEnvironmentVariableW(lpname : windows_sys::core::PCWSTR, lpguid : windows_sys::core::PCWSTR, pbuffer : *mut core::ffi::c_void, nsize : u32) -> u32);
windows_targets::link!("kernel32.dll" "system" fn GetPrivateProfileIntA(lpappname : windows_sys::core::PCSTR, lpkeyname : windows_sys::core::PCSTR, ndefault : i32, lpfilename : windows_sys::core::PCSTR) -> u32);
windows_targets::link!("kernel32.dll" "system" fn GetPrivateProfileIntW(lpappname : windows_sys::core::PCWSTR, lpkeyname : windows_sys::core::PCWSTR, ndefault : i32, lpfilename : windows_sys::core::PCWSTR) -> i32);
windows_targets::link!("kernel32.dll" "system" fn GetPrivateProfileSectionA(lpappname : windows_sys::core::PCSTR, lpreturnedstring : windows_sys::core::PSTR, nsize : u32, lpfilename : windows_sys::core::PCSTR) -> u32);
windows_targets::link!("kernel32.dll" "system" fn GetPrivateProfileSectionNamesA(lpszreturnbuffer : windows_sys::core::PSTR, nsize : u32, lpfilename : windows_sys::core::PCSTR) -> u32);
windows_targets::link!("kernel32.dll" "system" fn GetPrivateProfileSectionNamesW(lpszreturnbuffer : windows_sys::core::PWSTR, nsize : u32, lpfilename : windows_sys::core::PCWSTR) -> u32);
windows_targets::link!("kernel32.dll" "system" fn GetPrivateProfileSectionW(lpappname : windows_sys::core::PCWSTR, lpreturnedstring : windows_sys::core::PWSTR, nsize : u32, lpfilename : windows_sys::core::PCWSTR) -> u32);
windows_targets::link!("kernel32.dll" "system" fn GetPrivateProfileStringA(lpappname : windows_sys::core::PCSTR, lpkeyname : windows_sys::core::PCSTR, lpdefault : windows_sys::core::PCSTR, lpreturnedstring : windows_sys::core::PSTR, nsize : u32, lpfilename : windows_sys::core::PCSTR) -> u32);
windows_targets::link!("kernel32.dll" "system" fn GetPrivateProfileStringW(lpappname : windows_sys::core::PCWSTR, lpkeyname : windows_sys::core::PCWSTR, lpdefault : windows_sys::core::PCWSTR, lpreturnedstring : windows_sys::core::PWSTR, nsize : u32, lpfilename : windows_sys::core::PCWSTR) -> u32);
windows_targets::link!("kernel32.dll" "system" fn GetPrivateProfileStructA(lpszsection : windows_sys::core::PCSTR, lpszkey : windows_sys::core::PCSTR, lpstruct : *mut core::ffi::c_void, usizestruct : u32, szfile : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_targets::link!("kernel32.dll" "system" fn GetPrivateProfileStructW(lpszsection : windows_sys::core::PCWSTR, lpszkey : windows_sys::core::PCWSTR, lpstruct : *mut core::ffi::c_void, usizestruct : u32, szfile : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_targets::link!("kernel32.dll" "system" fn GetProfileIntA(lpappname : windows_sys::core::PCSTR, lpkeyname : windows_sys::core::PCSTR, ndefault : i32) -> u32);
windows_targets::link!("kernel32.dll" "system" fn GetProfileIntW(lpappname : windows_sys::core::PCWSTR, lpkeyname : windows_sys::core::PCWSTR, ndefault : i32) -> u32);
windows_targets::link!("kernel32.dll" "system" fn GetProfileSectionA(lpappname : windows_sys::core::PCSTR, lpreturnedstring : windows_sys::core::PSTR, nsize : u32) -> u32);
windows_targets::link!("kernel32.dll" "system" fn GetProfileSectionW(lpappname : windows_sys::core::PCWSTR, lpreturnedstring : windows_sys::core::PWSTR, nsize : u32) -> u32);
windows_targets::link!("kernel32.dll" "system" fn GetProfileStringA(lpappname : windows_sys::core::PCSTR, lpkeyname : windows_sys::core::PCSTR, lpdefault : windows_sys::core::PCSTR, lpreturnedstring : windows_sys::core::PSTR, nsize : u32) -> u32);
windows_targets::link!("kernel32.dll" "system" fn GetProfileStringW(lpappname : windows_sys::core::PCWSTR, lpkeyname : windows_sys::core::PCWSTR, lpdefault : windows_sys::core::PCWSTR, lpreturnedstring : windows_sys::core::PWSTR, nsize : u32) -> u32);
windows_targets::link!("kernel32.dll" "system" fn GetSystemRegistryQuota(pdwquotaallowed : *mut u32, pdwquotaused : *mut u32) -> windows_sys::core::BOOL);
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
windows_targets::link!("kernel32.dll" "system" fn GetThreadEnabledXStateFeatures() -> u64);
windows_targets::link!("advapi32.dll" "system" fn GetUserNameA(lpbuffer : windows_sys::core::PSTR, pcbbuffer : *mut u32) -> windows_sys::core::BOOL);
windows_targets::link!("advapi32.dll" "system" fn GetUserNameW(lpbuffer : windows_sys::core::PWSTR, pcbbuffer : *mut u32) -> windows_sys::core::BOOL);
windows_targets::link!("advpack.dll" "system" fn GetVersionFromFileA(lpszfilename : windows_sys::core::PCSTR, pdwmsver : *mut u32, pdwlsver : *mut u32, bversion : windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
windows_targets::link!("advpack.dll" "system" fn GetVersionFromFileExA(lpszfilename : windows_sys::core::PCSTR, pdwmsver : *mut u32, pdwlsver : *mut u32, bversion : windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
windows_targets::link!("advpack.dll" "system" fn GetVersionFromFileExW(lpszfilename : windows_sys::core::PCWSTR, pdwmsver : *mut u32, pdwlsver : *mut u32, bversion : windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
windows_targets::link!("advpack.dll" "system" fn GetVersionFromFileW(lpszfilename : windows_sys::core::PCWSTR, pdwmsver : *mut u32, pdwlsver : *mut u32, bversion : windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("dciman32.dll" "system" fn GetWindowRegionData(hwnd : super::super::Foundation:: HWND, size : u32, prd : *mut super::super::Graphics::Gdi:: RGNDATA) -> u32);
windows_targets::link!("kernel32.dll" "system" fn GlobalCompact(dwminfree : u32) -> usize);
windows_targets::link!("kernel32.dll" "system" fn GlobalFix(hmem : super::super::Foundation:: HGLOBAL));
windows_targets::link!("kernel32.dll" "system" fn GlobalUnWire(hmem : super::super::Foundation:: HGLOBAL) -> windows_sys::core::BOOL);
windows_targets::link!("kernel32.dll" "system" fn GlobalUnfix(hmem : super::super::Foundation:: HGLOBAL));
windows_targets::link!("kernel32.dll" "system" fn GlobalWire(hmem : super::super::Foundation:: HGLOBAL) -> *mut core::ffi::c_void);
windows_targets::link!("user32.dll" "system" fn IMPGetIMEA(param0 : super::super::Foundation:: HWND, param1 : *mut IMEPROA) -> windows_sys::core::BOOL);
windows_targets::link!("user32.dll" "system" fn IMPGetIMEW(param0 : super::super::Foundation:: HWND, param1 : *mut IMEPROW) -> windows_sys::core::BOOL);
windows_targets::link!("user32.dll" "system" fn IMPQueryIMEA(param0 : *mut IMEPROA) -> windows_sys::core::BOOL);
windows_targets::link!("user32.dll" "system" fn IMPQueryIMEW(param0 : *mut IMEPROW) -> windows_sys::core::BOOL);
windows_targets::link!("user32.dll" "system" fn IMPSetIMEA(param0 : super::super::Foundation:: HWND, param1 : *mut IMEPROA) -> windows_sys::core::BOOL);
windows_targets::link!("user32.dll" "system" fn IMPSetIMEW(param0 : super::super::Foundation:: HWND, param1 : *mut IMEPROW) -> windows_sys::core::BOOL);
windows_targets::link!("api-ms-win-core-apiquery-l2-1-0.dll" "system" fn IsApiSetImplemented(contract : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_targets::link!("kernel32.dll" "system" fn IsBadHugeReadPtr(lp : *const core::ffi::c_void, ucb : usize) -> windows_sys::core::BOOL);
windows_targets::link!("kernel32.dll" "system" fn IsBadHugeWritePtr(lp : *const core::ffi::c_void, ucb : usize) -> windows_sys::core::BOOL);
windows_targets::link!("advpack.dll" "system" fn IsNTAdmin(dwreserved : u32, lpdwreserved : *mut u32) -> windows_sys::core::BOOL);
windows_targets::link!("kernel32.dll" "system" fn IsNativeVhdBoot(nativevhdboot : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_targets::link!("advapi32.dll" "system" fn IsTokenUntrusted(tokenhandle : super::super::Foundation:: HANDLE) -> windows_sys::core::BOOL);
windows_targets::link!("advpack.dll" "system" fn LaunchINFSectionExW(hwnd : super::super::Foundation:: HWND, hinstance : super::super::Foundation:: HINSTANCE, pszparms : windows_sys::core::PCWSTR, nshow : i32) -> windows_sys::core::HRESULT);
windows_targets::link!("advpack.dll" "system" fn LaunchINFSectionW(hwndowner : super::super::Foundation:: HWND, hinstance : super::super::Foundation:: HINSTANCE, pszparams : windows_sys::core::PWSTR, nshow : i32) -> i32);
windows_targets::link!("kernel32.dll" "system" fn LocalCompact(uminfree : u32) -> usize);
windows_targets::link!("kernel32.dll" "system" fn LocalShrink(hmem : super::super::Foundation:: HLOCAL, cbnewsize : u32) -> usize);
windows_targets::link!("kernel32.dll" "system" fn MulDiv(nnumber : i32, nnumerator : i32, ndenominator : i32) -> i32);
windows_targets::link!("advpack.dll" "system" fn NeedReboot(dwrebootcheck : u32) -> windows_sys::core::BOOL);
windows_targets::link!("advpack.dll" "system" fn NeedRebootInit() -> u32);
windows_targets::link!("advpack.dll" "system" fn OpenINFEngineA(pszinffilename : windows_sys::core::PCSTR, pszinstallsection : windows_sys::core::PCSTR, dwflags : u32, phinf : *mut *mut core::ffi::c_void, pvreserved : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("advpack.dll" "system" fn OpenINFEngineW(pszinffilename : windows_sys::core::PCWSTR, pszinstallsection : windows_sys::core::PCWSTR, dwflags : u32, phinf : *mut *mut core::ffi::c_void, pvreserved : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("kernel32.dll" "system" fn OpenMutexA(dwdesiredaccess : u32, binherithandle : windows_sys::core::BOOL, lpname : windows_sys::core::PCSTR) -> super::super::Foundation:: HANDLE);
windows_targets::link!("kernel32.dll" "system" fn OpenSemaphoreA(dwdesiredaccess : u32, binherithandle : windows_sys::core::BOOL, lpname : windows_sys::core::PCSTR) -> super::super::Foundation:: HANDLE);
windows_targets::link!("api-ms-win-core-realtime-l1-1-2.dll" "system" fn QueryAuxiliaryCounterFrequency(lpauxiliarycounterfrequency : *mut u64) -> windows_sys::core::HRESULT);
windows_targets::link!("kernel32.dll" "system" fn QueryIdleProcessorCycleTime(bufferlength : *mut u32, processoridlecycletime : *mut u64) -> windows_sys::core::BOOL);
windows_targets::link!("kernel32.dll" "system" fn QueryIdleProcessorCycleTimeEx(group : u16, bufferlength : *mut u32, processoridlecycletime : *mut u64) -> windows_sys::core::BOOL);
windows_targets::link!("api-ms-win-core-realtime-l1-1-1.dll" "system" fn QueryInterruptTime(lpinterrupttime : *mut u64));
windows_targets::link!("api-ms-win-core-realtime-l1-1-1.dll" "system" fn QueryInterruptTimePrecise(lpinterrupttimeprecise : *mut u64));
windows_targets::link!("kernel32.dll" "system" fn QueryProcessCycleTime(processhandle : super::super::Foundation:: HANDLE, cycletime : *mut u64) -> windows_sys::core::BOOL);
windows_targets::link!("kernel32.dll" "system" fn QueryThreadCycleTime(threadhandle : super::super::Foundation:: HANDLE, cycletime : *mut u64) -> windows_sys::core::BOOL);
windows_targets::link!("kernel32.dll" "system" fn QueryUnbiasedInterruptTime(unbiasedtime : *mut u64) -> windows_sys::core::BOOL);
windows_targets::link!("api-ms-win-core-realtime-l1-1-1.dll" "system" fn QueryUnbiasedInterruptTimePrecise(lpunbiasedinterrupttimeprecise : *mut u64));
windows_targets::link!("api-ms-win-core-backgroundtask-l1-1-0.dll" "system" fn RaiseCustomSystemEventTrigger(customsystemeventtriggerconfig : *const CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG) -> u32);
windows_targets::link!("advpack.dll" "system" fn RebootCheckOnInstallA(hwnd : super::super::Foundation:: HWND, pszinf : windows_sys::core::PCSTR, pszsec : windows_sys::core::PCSTR, dwreserved : u32) -> windows_sys::core::HRESULT);
windows_targets::link!("advpack.dll" "system" fn RebootCheckOnInstallW(hwnd : super::super::Foundation:: HWND, pszinf : windows_sys::core::PCWSTR, pszsec : windows_sys::core::PCWSTR, dwreserved : u32) -> windows_sys::core::HRESULT);
windows_targets::link!("api-ms-win-core-featurestaging-l1-1-0.dll" "system" fn RecordFeatureError(featureid : u32, error : *const FEATURE_ERROR));
windows_targets::link!("api-ms-win-core-featurestaging-l1-1-0.dll" "system" fn RecordFeatureUsage(featureid : u32, kind : u32, addend : u32, originname : windows_sys::core::PCSTR));
windows_targets::link!("advpack.dll" "system" fn RegInstallA(hmod : super::super::Foundation:: HMODULE, pszsection : windows_sys::core::PCSTR, psttable : *const STRTABLEA) -> windows_sys::core::HRESULT);
windows_targets::link!("advpack.dll" "system" fn RegInstallW(hmod : super::super::Foundation:: HMODULE, pszsection : windows_sys::core::PCWSTR, psttable : *const STRTABLEW) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Registry")]
windows_targets::link!("advpack.dll" "system" fn RegRestoreAllA(hwnd : super::super::Foundation:: HWND, psztitlestring : windows_sys::core::PCSTR, hkbckupkey : super::Registry:: HKEY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Registry")]
windows_targets::link!("advpack.dll" "system" fn RegRestoreAllW(hwnd : super::super::Foundation:: HWND, psztitlestring : windows_sys::core::PCWSTR, hkbckupkey : super::Registry:: HKEY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Registry")]
windows_targets::link!("advpack.dll" "system" fn RegSaveRestoreA(hwnd : super::super::Foundation:: HWND, psztitlestring : windows_sys::core::PCSTR, hkbckupkey : super::Registry:: HKEY, pcszrootkey : windows_sys::core::PCSTR, pcszsubkey : windows_sys::core::PCSTR, pcszvaluename : windows_sys::core::PCSTR, dwflags : u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Registry")]
windows_targets::link!("advpack.dll" "system" fn RegSaveRestoreOnINFA(hwnd : super::super::Foundation:: HWND, psztitle : windows_sys::core::PCSTR, pszinf : windows_sys::core::PCSTR, pszsection : windows_sys::core::PCSTR, hhklmbackkey : super::Registry:: HKEY, hhkcubackkey : super::Registry:: HKEY, dwflags : u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Registry")]
windows_targets::link!("advpack.dll" "system" fn RegSaveRestoreOnINFW(hwnd : super::super::Foundation:: HWND, psztitle : windows_sys::core::PCWSTR, pszinf : windows_sys::core::PCWSTR, pszsection : windows_sys::core::PCWSTR, hhklmbackkey : super::Registry:: HKEY, hhkcubackkey : super::Registry:: HKEY, dwflags : u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Registry")]
windows_targets::link!("advpack.dll" "system" fn RegSaveRestoreW(hwnd : super::super::Foundation:: HWND, psztitlestring : windows_sys::core::PCWSTR, hkbckupkey : super::Registry:: HKEY, pcszrootkey : windows_sys::core::PCWSTR, pcszsubkey : windows_sys::core::PCWSTR, pcszvaluename : windows_sys::core::PCWSTR, dwflags : u32) -> windows_sys::core::HRESULT);
windows_targets::link!("kernel32.dll" "system" fn ReplacePartitionUnit(targetpartition : windows_sys::core::PCWSTR, sparepartition : windows_sys::core::PCWSTR, flags : u32) -> windows_sys::core::BOOL);
windows_targets::link!("kernel32.dll" "system" fn RequestDeviceWakeup(hdevice : super::super::Foundation:: HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("ntdll.dll" "system" fn RtlAnsiStringToUnicodeString(destinationstring : *mut super::super::Foundation:: UNICODE_STRING, sourcestring : *mut super::Kernel:: STRING, allocatedestinationstring : bool) -> super::super::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn RtlCharToInteger(string : *mut i8, base : u32, value : *mut u32) -> super::super::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("ntdll.dll" "system" fn RtlFreeAnsiString(ansistring : *mut super::Kernel:: STRING));
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("ntdll.dll" "system" fn RtlFreeOemString(oemstring : *mut super::Kernel:: STRING));
windows_targets::link!("ntdll.dll" "system" fn RtlFreeUnicodeString(unicodestring : *mut super::super::Foundation:: UNICODE_STRING));
windows_targets::link!("ntdll.dll" "system" fn RtlGetReturnAddressHijackTarget() -> usize);
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("ntdll.dll" "system" fn RtlInitAnsiString(destinationstring : *mut super::Kernel:: STRING, sourcestring : *mut i8));
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("ntdll.dll" "system" fn RtlInitAnsiStringEx(destinationstring : *mut super::Kernel:: STRING, sourcestring : *mut i8) -> super::super::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("ntdll.dll" "system" fn RtlInitString(destinationstring : *mut super::Kernel:: STRING, sourcestring : *mut i8));
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("ntdll.dll" "system" fn RtlInitStringEx(destinationstring : *mut super::Kernel:: STRING, sourcestring : *mut i8) -> super::super::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn RtlInitUnicodeString(destinationstring : *mut super::super::Foundation:: UNICODE_STRING, sourcestring : windows_sys::core::PCWSTR));
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("ntdll.dll" "system" fn RtlIsNameLegalDOS8Dot3(name : *mut super::super::Foundation:: UNICODE_STRING, oemname : *mut super::Kernel:: STRING, namecontainsspaces : *mut bool) -> bool);
windows_targets::link!("ntdll.dll" "system" fn RtlLocalTimeToSystemTime(localtime : *mut i64, systemtime : *mut i64) -> super::super::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn RtlRaiseCustomSystemEventTrigger(triggerconfig : *const CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG) -> u32);
windows_targets::link!("ntdll.dll" "system" fn RtlTimeToSecondsSince1970(time : *mut i64, elapsedseconds : *mut u32) -> bool);
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("ntdll.dll" "system" fn RtlUnicodeStringToAnsiString(destinationstring : *mut super::Kernel:: STRING, sourcestring : *mut super::super::Foundation:: UNICODE_STRING, allocatedestinationstring : bool) -> super::super::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("ntdll.dll" "system" fn RtlUnicodeStringToOemString(destinationstring : *mut super::Kernel:: STRING, sourcestring : *mut super::super::Foundation:: UNICODE_STRING, allocatedestinationstring : bool) -> super::super::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn RtlUnicodeToMultiByteSize(bytesinmultibytestring : *mut u32, unicodestring : windows_sys::core::PCWSTR, bytesinunicodestring : u32) -> super::super::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn RtlUniform(seed : *mut u32) -> u32);
windows_targets::link!("advpack.dll" "system" fn RunSetupCommandA(hwnd : super::super::Foundation:: HWND, szcmdname : windows_sys::core::PCSTR, szinfsection : windows_sys::core::PCSTR, szdir : windows_sys::core::PCSTR, lpsztitle : windows_sys::core::PCSTR, phexe : *mut super::super::Foundation:: HANDLE, dwflags : u32, pvreserved : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("advpack.dll" "system" fn RunSetupCommandW(hwnd : super::super::Foundation:: HWND, szcmdname : windows_sys::core::PCWSTR, szinfsection : windows_sys::core::PCWSTR, szdir : windows_sys::core::PCWSTR, lpsztitle : windows_sys::core::PCWSTR, phexe : *mut super::super::Foundation:: HANDLE, dwflags : u32, pvreserved : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("user32.dll" "system" fn SendIMEMessageExA(param0 : super::super::Foundation:: HWND, param1 : super::super::Foundation:: LPARAM) -> super::super::Foundation:: LRESULT);
windows_targets::link!("user32.dll" "system" fn SendIMEMessageExW(param0 : super::super::Foundation:: HWND, param1 : super::super::Foundation:: LPARAM) -> super::super::Foundation:: LRESULT);
windows_targets::link!("kernel32.dll" "system" fn SetEnvironmentStringsA(newenvironment : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_targets::link!("kernel32.dll" "system" fn SetFirmwareEnvironmentVariableA(lpname : windows_sys::core::PCSTR, lpguid : windows_sys::core::PCSTR, pvalue : *const core::ffi::c_void, nsize : u32) -> windows_sys::core::BOOL);
windows_targets::link!("kernel32.dll" "system" fn SetFirmwareEnvironmentVariableExA(lpname : windows_sys::core::PCSTR, lpguid : windows_sys::core::PCSTR, pvalue : *const core::ffi::c_void, nsize : u32, dwattributes : u32) -> windows_sys::core::BOOL);
windows_targets::link!("kernel32.dll" "system" fn SetFirmwareEnvironmentVariableExW(lpname : windows_sys::core::PCWSTR, lpguid : windows_sys::core::PCWSTR, pvalue : *const core::ffi::c_void, nsize : u32, dwattributes : u32) -> windows_sys::core::BOOL);
windows_targets::link!("kernel32.dll" "system" fn SetFirmwareEnvironmentVariableW(lpname : windows_sys::core::PCWSTR, lpguid : windows_sys::core::PCWSTR, pvalue : *const core::ffi::c_void, nsize : u32) -> windows_sys::core::BOOL);
windows_targets::link!("kernel32.dll" "system" fn SetHandleCount(unumber : u32) -> u32);
windows_targets::link!("kernel32.dll" "system" fn SetMessageWaitingIndicator(hmsgindicator : super::super::Foundation:: HANDLE, ulmsgcount : u32) -> windows_sys::core::BOOL);
windows_targets::link!("advpack.dll" "system" fn SetPerUserSecValuesA(pperuser : *mut PERUSERSECTIONA) -> windows_sys::core::HRESULT);
windows_targets::link!("advpack.dll" "system" fn SetPerUserSecValuesW(pperuser : *mut PERUSERSECTIONW) -> windows_sys::core::HRESULT);
windows_targets::link!("api-ms-win-core-featurestaging-l1-1-0.dll" "system" fn SubscribeFeatureStateChangeNotification(subscription : *mut FEATURE_STATE_CHANGE_SUBSCRIPTION, callback : PFEATURE_STATE_CHANGE_CALLBACK, context : *const core::ffi::c_void));
windows_targets::link!("advpack.dll" "system" fn TranslateInfStringA(pszinffilename : windows_sys::core::PCSTR, pszinstallsection : windows_sys::core::PCSTR, psztranslatesection : windows_sys::core::PCSTR, psztranslatekey : windows_sys::core::PCSTR, pszbuffer : windows_sys::core::PSTR, cchbuffer : u32, pdwrequiredsize : *mut u32, pvreserved : *const core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("advpack.dll" "system" fn TranslateInfStringExA(hinf : *mut core::ffi::c_void, pszinffilename : windows_sys::core::PCSTR, psztranslatesection : windows_sys::core::PCSTR, psztranslatekey : windows_sys::core::PCSTR, pszbuffer : windows_sys::core::PSTR, dwbuffersize : u32, pdwrequiredsize : *mut u32, pvreserved : *const core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("advpack.dll" "system" fn TranslateInfStringExW(hinf : *mut core::ffi::c_void, pszinffilename : windows_sys::core::PCWSTR, psztranslatesection : windows_sys::core::PCWSTR, psztranslatekey : windows_sys::core::PCWSTR, pszbuffer : windows_sys::core::PWSTR, dwbuffersize : u32, pdwrequiredsize : *mut u32, pvreserved : *const core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("advpack.dll" "system" fn TranslateInfStringW(pszinffilename : windows_sys::core::PCWSTR, pszinstallsection : windows_sys::core::PCWSTR, psztranslatesection : windows_sys::core::PCWSTR, psztranslatekey : windows_sys::core::PCWSTR, pszbuffer : windows_sys::core::PWSTR, cchbuffer : u32, pdwrequiredsize : *mut u32, pvreserved : *const core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("api-ms-win-core-featurestaging-l1-1-0.dll" "system" fn UnsubscribeFeatureStateChangeNotification(subscription : FEATURE_STATE_CHANGE_SUBSCRIPTION));
windows_targets::link!("advpack.dll" "system" fn UserInstStubWrapperA(hwnd : super::super::Foundation:: HWND, hinstance : super::super::Foundation:: HINSTANCE, pszparms : windows_sys::core::PCSTR, nshow : i32) -> windows_sys::core::HRESULT);
windows_targets::link!("advpack.dll" "system" fn UserInstStubWrapperW(hwnd : super::super::Foundation:: HWND, hinstance : super::super::Foundation:: HINSTANCE, pszparms : windows_sys::core::PCWSTR, nshow : i32) -> windows_sys::core::HRESULT);
windows_targets::link!("advpack.dll" "system" fn UserUnInstStubWrapperA(hwnd : super::super::Foundation:: HWND, hinstance : super::super::Foundation:: HINSTANCE, pszparms : windows_sys::core::PCSTR, nshow : i32) -> windows_sys::core::HRESULT);
windows_targets::link!("advpack.dll" "system" fn UserUnInstStubWrapperW(hwnd : super::super::Foundation:: HWND, hinstance : super::super::Foundation:: HINSTANCE, pszparms : windows_sys::core::PCWSTR, nshow : i32) -> windows_sys::core::HRESULT);
windows_targets::link!("user32.dll" "system" fn WINNLSEnableIME(param0 : super::super::Foundation:: HWND, param1 : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_targets::link!("user32.dll" "system" fn WINNLSGetEnableStatus(param0 : super::super::Foundation:: HWND) -> windows_sys::core::BOOL);
windows_targets::link!("user32.dll" "system" fn WINNLSGetIMEHotkey(param0 : super::super::Foundation:: HWND) -> u32);
windows_targets::link!("dciman32.dll" "system" fn WinWatchClose(hww : HWINWATCH));
windows_targets::link!("dciman32.dll" "system" fn WinWatchDidStatusChange(hww : HWINWATCH) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("dciman32.dll" "system" fn WinWatchGetClipList(hww : HWINWATCH, prc : *mut super::super::Foundation:: RECT, size : u32, prd : *mut super::super::Graphics::Gdi:: RGNDATA) -> u32);
windows_targets::link!("dciman32.dll" "system" fn WinWatchNotify(hww : HWINWATCH, notifycallback : WINWATCHNOTIFYPROC, notifyparam : super::super::Foundation:: LPARAM) -> windows_sys::core::BOOL);
windows_targets::link!("dciman32.dll" "system" fn WinWatchOpen(hwnd : super::super::Foundation:: HWND) -> HWINWATCH);
windows_targets::link!("wldp.dll" "system" fn WldpCanExecuteBuffer(host : *const windows_sys::core::GUID, options : WLDP_EXECUTION_EVALUATION_OPTIONS, buffer : *const u8, buffersize : u32, auditinfo : windows_sys::core::PCWSTR, result : *mut WLDP_EXECUTION_POLICY) -> windows_sys::core::HRESULT);
windows_targets::link!("wldp.dll" "system" fn WldpCanExecuteFile(host : *const windows_sys::core::GUID, options : WLDP_EXECUTION_EVALUATION_OPTIONS, filehandle : super::super::Foundation:: HANDLE, auditinfo : windows_sys::core::PCWSTR, result : *mut WLDP_EXECUTION_POLICY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("wldp.dll" "system" fn WldpCanExecuteStream(host : *const windows_sys::core::GUID, options : WLDP_EXECUTION_EVALUATION_OPTIONS, stream : * mut core::ffi::c_void, auditinfo : windows_sys::core::PCWSTR, result : *mut WLDP_EXECUTION_POLICY) -> windows_sys::core::HRESULT);
windows_targets::link!("wldp.dll" "system" fn WldpGetLockdownPolicy(hostinformation : *const WLDP_HOST_INFORMATION, lockdownstate : *mut u32, lockdownflags : u32) -> windows_sys::core::HRESULT);
windows_targets::link!("wldp.dll" "system" fn WldpIsClassInApprovedList(classid : *const windows_sys::core::GUID, hostinformation : *const WLDP_HOST_INFORMATION, isapproved : *mut windows_sys::core::BOOL, optionalflags : u32) -> windows_sys::core::HRESULT);
windows_targets::link!("wldp.dll" "system" fn WldpIsDynamicCodePolicyEnabled(isenabled : *mut windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
windows_targets::link!("wldp.dll" "system" fn WldpQueryDeviceSecurityInformation(information : *mut WLDP_DEVICE_SECURITY_INFORMATION, informationlength : u32, returnlength : *mut u32) -> windows_sys::core::HRESULT);
windows_targets::link!("wldp.dll" "system" fn WldpQueryDynamicCodeTrust(filehandle : super::super::Foundation:: HANDLE, baseimage : *const core::ffi::c_void, imagesize : u32) -> windows_sys::core::HRESULT);
windows_targets::link!("wldp.dll" "system" fn WldpSetDynamicCodeTrust(filehandle : super::super::Foundation:: HANDLE) -> windows_sys::core::HRESULT);
windows_targets::link!("kernel32.dll" "system" fn WritePrivateProfileSectionA(lpappname : windows_sys::core::PCSTR, lpstring : windows_sys::core::PCSTR, lpfilename : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_targets::link!("kernel32.dll" "system" fn WritePrivateProfileSectionW(lpappname : windows_sys::core::PCWSTR, lpstring : windows_sys::core::PCWSTR, lpfilename : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_targets::link!("kernel32.dll" "system" fn WritePrivateProfileStringA(lpappname : windows_sys::core::PCSTR, lpkeyname : windows_sys::core::PCSTR, lpstring : windows_sys::core::PCSTR, lpfilename : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_targets::link!("kernel32.dll" "system" fn WritePrivateProfileStringW(lpappname : windows_sys::core::PCWSTR, lpkeyname : windows_sys::core::PCWSTR, lpstring : windows_sys::core::PCWSTR, lpfilename : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_targets::link!("kernel32.dll" "system" fn WritePrivateProfileStructA(lpszsection : windows_sys::core::PCSTR, lpszkey : windows_sys::core::PCSTR, lpstruct : *const core::ffi::c_void, usizestruct : u32, szfile : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_targets::link!("kernel32.dll" "system" fn WritePrivateProfileStructW(lpszsection : windows_sys::core::PCWSTR, lpszkey : windows_sys::core::PCWSTR, lpstruct : *const core::ffi::c_void, usizestruct : u32, szfile : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_targets::link!("kernel32.dll" "system" fn WriteProfileSectionA(lpappname : windows_sys::core::PCSTR, lpstring : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_targets::link!("kernel32.dll" "system" fn WriteProfileSectionW(lpappname : windows_sys::core::PCWSTR, lpstring : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_targets::link!("kernel32.dll" "system" fn WriteProfileStringA(lpappname : windows_sys::core::PCSTR, lpkeyname : windows_sys::core::PCSTR, lpstring : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_targets::link!("kernel32.dll" "system" fn WriteProfileStringW(lpappname : windows_sys::core::PCWSTR, lpkeyname : windows_sys::core::PCWSTR, lpstring : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_targets::link!("kernel32.dll" "system" fn _hread(hfile : i32, lpbuffer : *mut core::ffi::c_void, lbytes : i32) -> i32);
windows_targets::link!("kernel32.dll" "system" fn _hwrite(hfile : i32, lpbuffer : windows_sys::core::PCSTR, lbytes : i32) -> i32);
windows_targets::link!("kernel32.dll" "system" fn _lclose(hfile : i32) -> i32);
windows_targets::link!("kernel32.dll" "system" fn _lcreat(lppathname : windows_sys::core::PCSTR, iattribute : i32) -> i32);
windows_targets::link!("kernel32.dll" "system" fn _llseek(hfile : i32, loffset : i32, iorigin : i32) -> i32);
windows_targets::link!("kernel32.dll" "system" fn _lopen(lppathname : windows_sys::core::PCSTR, ireadwrite : i32) -> i32);
windows_targets::link!("kernel32.dll" "system" fn _lread(hfile : i32, lpbuffer : *mut core::ffi::c_void, ubytes : u32) -> u32);
windows_targets::link!("kernel32.dll" "system" fn _lwrite(hfile : i32, lpbuffer : windows_sys::core::PCSTR, ubytes : u32) -> u32);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
windows_targets::link!("kernel32.dll" "system" fn uaw_lstrcmpW(string1 : *const u16, string2 : *const u16) -> i32);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
windows_targets::link!("kernel32.dll" "system" fn uaw_lstrcmpiW(string1 : *const u16, string2 : *const u16) -> i32);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
windows_targets::link!("kernel32.dll" "system" fn uaw_lstrlenW(string : *const u16) -> i32);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
windows_targets::link!("kernel32.dll" "system" fn uaw_wcschr(string : *const u16, character : u16) -> *mut u16);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
windows_targets::link!("kernel32.dll" "system" fn uaw_wcscpy(destination : *mut u16, source : *const u16) -> *mut u16);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
windows_targets::link!("kernel32.dll" "system" fn uaw_wcsicmp(string1 : *const u16, string2 : *const u16) -> i32);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
windows_targets::link!("kernel32.dll" "system" fn uaw_wcslen(string : *const u16) -> usize);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
windows_targets::link!("kernel32.dll" "system" fn uaw_wcsrchr(string : *const u16, character : u16) -> *mut u16);
pub const AADBE_ADD_ENTRY: u32 = 1u32;
pub const AADBE_DEL_ENTRY: u32 = 2u32;
pub const ACTCTX_FLAG_APPLICATION_NAME_VALID: u32 = 32u32;
pub const ACTCTX_FLAG_ASSEMBLY_DIRECTORY_VALID: u32 = 4u32;
pub const ACTCTX_FLAG_HMODULE_VALID: u32 = 128u32;
pub const ACTCTX_FLAG_LANGID_VALID: u32 = 2u32;
pub const ACTCTX_FLAG_PROCESSOR_ARCHITECTURE_VALID: u32 = 1u32;
pub const ACTCTX_FLAG_RESOURCE_NAME_VALID: u32 = 8u32;
pub const ACTCTX_FLAG_SET_PROCESS_DEFAULT: u32 = 16u32;
pub const ACTCTX_FLAG_SOURCE_IS_ASSEMBLYREF: u32 = 64u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ACTCTX_SECTION_KEYED_DATA_2600 {
    pub cbSize: u32,
    pub ulDataFormatVersion: u32,
    pub lpData: *mut core::ffi::c_void,
    pub ulLength: u32,
    pub lpSectionGlobalData: *mut core::ffi::c_void,
    pub ulSectionGlobalDataLength: u32,
    pub lpSectionBase: *mut core::ffi::c_void,
    pub ulSectionTotalLength: u32,
    pub hActCtx: super::super::Foundation::HANDLE,
    pub ulAssemblyRosterIndex: u32,
}
impl Default for ACTCTX_SECTION_KEYED_DATA_2600 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA {
    pub lpInformation: *mut core::ffi::c_void,
    pub lpSectionBase: *mut core::ffi::c_void,
    pub ulSectionLength: u32,
    pub lpSectionGlobalDataBase: *mut core::ffi::c_void,
    pub ulSectionGlobalDataLength: u32,
}
impl Default for ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ACTIVATION_CONTEXT_BASIC_INFORMATION {
    pub hActCtx: super::super::Foundation::HANDLE,
    pub dwFlags: u32,
}
impl Default for ACTIVATION_CONTEXT_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ACTIVATION_CONTEXT_BASIC_INFORMATION_DEFINED: u32 = 1u32;
pub const AC_LINE_BACKUP_POWER: u32 = 2u32;
pub const AC_LINE_OFFLINE: u32 = 0u32;
pub const AC_LINE_ONLINE: u32 = 1u32;
pub const AC_LINE_UNKNOWN: u32 = 255u32;
pub const ADN_DEL_IF_EMPTY: u32 = 1u32;
pub const ADN_DEL_UNC_PATHS: u32 = 8u32;
pub const ADN_DONT_DEL_DIR: u32 = 4u32;
pub const ADN_DONT_DEL_SUBDIRS: u32 = 2u32;
pub const AFSR_BACKNEW: u32 = 2u32;
pub const AFSR_EXTRAINCREFCNT: u32 = 2048u32;
pub const AFSR_NODELETENEW: u32 = 4u32;
pub const AFSR_NOMESSAGES: u32 = 8u32;
pub const AFSR_NOPROGRESS: u32 = 16u32;
pub const AFSR_RESTORE: u32 = 1u32;
pub const AFSR_UPDREFCNT: u32 = 512u32;
pub const AFSR_USEREFCNT: u32 = 1024u32;
pub const AIF_FORCE_FILE_IN_USE: u32 = 8u32;
pub const AIF_NOLANGUAGECHECK: u32 = 268435456u32;
pub const AIF_NOOVERWRITE: u32 = 16u32;
pub const AIF_NOSKIP: u32 = 2u32;
pub const AIF_NOVERSIONCHECK: u32 = 4u32;
pub const AIF_NO_VERSION_DIALOG: u32 = 32u32;
pub const AIF_QUIET: u32 = 536870912u32;
pub const AIF_REPLACEONLY: u32 = 1024u32;
pub const AIF_WARNIFSKIP: u32 = 1u32;
pub const ALINF_BKINSTALL: u32 = 32u32;
pub const ALINF_CHECKBKDATA: u32 = 128u32;
pub const ALINF_DELAYREGISTEROCX: u32 = 512u32;
pub const ALINF_NGCONV: u32 = 8u32;
pub const ALINF_QUIET: u32 = 4u32;
pub const ALINF_ROLLBACK: u32 = 64u32;
pub const ALINF_ROLLBKDOALL: u32 = 256u32;
pub const ALINF_UPDHLPDLLS: u32 = 16u32;
pub type APPLICATION_RECOVERY_CALLBACK = Option<unsafe extern "system" fn(pvparameter: *mut core::ffi::c_void) -> u32>;
pub const ARSR_NOMESSAGES: u32 = 8u32;
pub const ARSR_REGSECTION: u32 = 128u32;
pub const ARSR_REMOVREGBKDATA: u32 = 4096u32;
pub const ARSR_RESTORE: u32 = 1u32;
pub const ATOM_FLAG_GLOBAL: u32 = 2u32;
pub const AT_ARP: u32 = 640u32;
pub const AT_ENTITY: TDIENTITY_ENTITY_TYPE = 640u32;
pub const AT_NULL: u32 = 642u32;
pub const BACKUP_GHOSTED_FILE_EXTENTS: u32 = 11u32;
pub const BACKUP_INVALID: u32 = 0u32;
pub const BASE_SEARCH_PATH_DISABLE_SAFE_SEARCHMODE: u32 = 65536u32;
pub const BASE_SEARCH_PATH_ENABLE_SAFE_SEARCHMODE: u32 = 1u32;
pub const BASE_SEARCH_PATH_PERMANENT: u32 = 32768u32;
pub const BATTERY_FLAG_CHARGING: u32 = 8u32;
pub const BATTERY_FLAG_CRITICAL: u32 = 4u32;
pub const BATTERY_FLAG_HIGH: u32 = 1u32;
pub const BATTERY_FLAG_LOW: u32 = 2u32;
pub const BATTERY_FLAG_NO_BATTERY: u32 = 128u32;
pub const BATTERY_FLAG_UNKNOWN: u32 = 255u32;
pub const BATTERY_LIFE_UNKNOWN: u32 = 4294967295u32;
pub const BATTERY_PERCENTAGE_UNKNOWN: u32 = 255u32;
pub const BAUD_075: u32 = 1u32;
pub const BAUD_110: u32 = 2u32;
pub const BAUD_115200: u32 = 131072u32;
pub const BAUD_1200: u32 = 64u32;
pub const BAUD_128K: u32 = 65536u32;
pub const BAUD_134_5: u32 = 4u32;
pub const BAUD_14400: u32 = 4096u32;
pub const BAUD_150: u32 = 8u32;
pub const BAUD_1800: u32 = 128u32;
pub const BAUD_19200: u32 = 8192u32;
pub const BAUD_2400: u32 = 256u32;
pub const BAUD_300: u32 = 16u32;
pub const BAUD_38400: u32 = 16384u32;
pub const BAUD_4800: u32 = 512u32;
pub const BAUD_56K: u32 = 32768u32;
pub const BAUD_57600: u32 = 262144u32;
pub const BAUD_600: u32 = 32u32;
pub const BAUD_7200: u32 = 1024u32;
pub const BAUD_9600: u32 = 2048u32;
pub const BAUD_USER: u32 = 268435456u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CABINFOA {
    pub pszCab: windows_sys::core::PSTR,
    pub pszInf: windows_sys::core::PSTR,
    pub pszSection: windows_sys::core::PSTR,
    pub szSrcPath: [i8; 260],
    pub dwFlags: u32,
}
impl Default for CABINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CABINFOW {
    pub pszCab: windows_sys::core::PWSTR,
    pub pszInf: windows_sys::core::PWSTR,
    pub pszSection: windows_sys::core::PWSTR,
    pub szSrcPath: [u16; 260],
    pub dwFlags: u32,
}
impl Default for CABINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CATID_DeleteBrowsingHistory: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x31caf6e4_d6aa_4090_a050_a5ac8972e9ef);
pub const CBR_110: u32 = 110u32;
pub const CBR_115200: u32 = 115200u32;
pub const CBR_1200: u32 = 1200u32;
pub const CBR_128000: u32 = 128000u32;
pub const CBR_14400: u32 = 14400u32;
pub const CBR_19200: u32 = 19200u32;
pub const CBR_2400: u32 = 2400u32;
pub const CBR_256000: u32 = 256000u32;
pub const CBR_300: u32 = 300u32;
pub const CBR_38400: u32 = 38400u32;
pub const CBR_4800: u32 = 4800u32;
pub const CBR_56000: u32 = 56000u32;
pub const CBR_57600: u32 = 57600u32;
pub const CBR_600: u32 = 600u32;
pub const CBR_9600: u32 = 9600u32;
pub const CE_DNS: u32 = 2048u32;
pub const CE_IOE: u32 = 1024u32;
pub const CE_MODE: u32 = 32768u32;
pub const CE_OOP: u32 = 4096u32;
pub const CE_PTO: u32 = 512u32;
pub const CE_TXFULL: u32 = 256u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLIENT_ID {
    pub UniqueProcess: super::super::Foundation::HANDLE,
    pub UniqueThread: super::super::Foundation::HANDLE,
}
impl Default for CLIENT_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CL_NL_ENTITY: TDIENTITY_ENTITY_TYPE = 769u32;
pub const CL_NL_IP: u32 = 771u32;
pub const CL_NL_IPX: u32 = 769u32;
pub const CL_TL_ENTITY: TDIENTITY_ENTITY_TYPE = 1025u32;
pub const CL_TL_NBF: u32 = 1025u32;
pub const CL_TL_UDP: u32 = 1027u32;
pub const CODEINTEGRITY_OPTION_DEBUGMODE_ENABLED: u32 = 128u32;
pub const CODEINTEGRITY_OPTION_ENABLED: u32 = 1u32;
pub const CODEINTEGRITY_OPTION_FLIGHTING_ENABLED: u32 = 512u32;
pub const CODEINTEGRITY_OPTION_FLIGHT_BUILD: u32 = 256u32;
pub const CODEINTEGRITY_OPTION_HVCI_IUM_ENABLED: u32 = 8192u32;
pub const CODEINTEGRITY_OPTION_HVCI_KMCI_AUDITMODE_ENABLED: u32 = 2048u32;
pub const CODEINTEGRITY_OPTION_HVCI_KMCI_ENABLED: u32 = 1024u32;
pub const CODEINTEGRITY_OPTION_HVCI_KMCI_STRICTMODE_ENABLED: u32 = 4096u32;
pub const CODEINTEGRITY_OPTION_PREPRODUCTION_BUILD: u32 = 64u32;
pub const CODEINTEGRITY_OPTION_TESTSIGN: u32 = 2u32;
pub const CODEINTEGRITY_OPTION_TEST_BUILD: u32 = 32u32;
pub const CODEINTEGRITY_OPTION_UMCI_AUDITMODE_ENABLED: u32 = 8u32;
pub const CODEINTEGRITY_OPTION_UMCI_ENABLED: u32 = 4u32;
pub const CODEINTEGRITY_OPTION_UMCI_EXCLUSIONPATHS_ENABLED: u32 = 16u32;
pub const COMMPROP_INITIALIZED: u32 = 3879531822u32;
pub const CONTEXT_SIZE: u32 = 16u32;
pub const CO_NL_ENTITY: TDIENTITY_ENTITY_TYPE = 768u32;
pub const CO_TL_ENTITY: TDIENTITY_ENTITY_TYPE = 1024u32;
pub const CO_TL_NBF: u32 = 1024u32;
pub const CO_TL_SPP: u32 = 1030u32;
pub const CO_TL_SPX: u32 = 1026u32;
pub const CO_TL_TCP: u32 = 1028u32;
pub const CP_DIRECT: u32 = 2u32;
pub const CP_HWND: u32 = 0u32;
pub const CP_LEVEL: u32 = 3u32;
pub const CP_OPEN: u32 = 1u32;
pub const CREATE_FOR_DIR: u32 = 2u32;
pub const CREATE_FOR_IMPORT: u32 = 1u32;
pub const CRITICAL_SECTION_NO_DEBUG_INFO: u32 = 16777216u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG {
    pub Size: u32,
    pub TriggerId: windows_sys::core::PCWSTR,
}
impl Default for CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CameraUIControl: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x16d5a2be_b1c5_47b3_8eae_ccbcf452c7e8);
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct CameraUIControlCaptureMode(pub i32);
impl CameraUIControlCaptureMode {
    pub const PhotoOrVideo: Self = Self(0i32);
    pub const Photo: Self = Self(1i32);
    pub const Video: Self = Self(2i32);
}
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct CameraUIControlLinearSelectionMode(pub i32);
impl CameraUIControlLinearSelectionMode {
    pub const Single: Self = Self(0i32);
    pub const Multiple: Self = Self(1i32);
}
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct CameraUIControlMode(pub i32);
impl CameraUIControlMode {
    pub const Browse: Self = Self(0i32);
    pub const Linear: Self = Self(1i32);
}
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct CameraUIControlPhotoFormat(pub i32);
impl CameraUIControlPhotoFormat {
    pub const Jpeg: Self = Self(0i32);
    pub const Png: Self = Self(1i32);
    pub const JpegXR: Self = Self(2i32);
}
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct CameraUIControlVideoFormat(pub i32);
impl CameraUIControlVideoFormat {
    pub const Mp4: Self = Self(0i32);
    pub const Wmv: Self = Self(1i32);
}
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct CameraUIControlViewType(pub i32);
impl CameraUIControlViewType {
    pub const SingleItem: Self = Self(0i32);
    pub const ItemList: Self = Self(1i32);
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DATETIME {
    pub year: u16,
    pub month: u16,
    pub day: u16,
    pub hour: u16,
    pub min: u16,
    pub sec: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DCICMD {
    pub dwCommand: u32,
    pub dwParam1: u32,
    pub dwParam2: u32,
    pub dwVersion: u32,
    pub dwReserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DCICREATEINPUT {
    pub cmd: DCICMD,
    pub dwCompression: u32,
    pub dwMask: [u32; 3],
    pub dwWidth: u32,
    pub dwHeight: u32,
    pub dwDCICaps: u32,
    pub dwBitCount: u32,
    pub lpSurface: *mut core::ffi::c_void,
}
impl Default for DCICREATEINPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DCICREATEOFFSCREENSURFACE: u32 = 2u32;
pub const DCICREATEOVERLAYSURFACE: u32 = 3u32;
pub const DCICREATEPRIMARYSURFACE: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DCIENUMINPUT {
    pub cmd: DCICMD,
    pub rSrc: super::super::Foundation::RECT,
    pub rDst: super::super::Foundation::RECT,
    pub EnumCallback: isize,
    pub lpContext: *mut core::ffi::c_void,
}
impl Default for DCIENUMINPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DCIENUMSURFACE: u32 = 4u32;
pub const DCIESCAPE: u32 = 5u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DCIOFFSCREEN {
    pub dciInfo: DCISURFACEINFO,
    pub Draw: isize,
    pub SetClipList: isize,
    pub SetDestination: isize,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DCIOVERLAY {
    pub dciInfo: DCISURFACEINFO,
    pub dwChromakeyValue: u32,
    pub dwChromakeyMask: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DCISURFACEINFO {
    pub dwSize: u32,
    pub dwDCICaps: u32,
    pub dwCompression: u32,
    pub dwMask: [u32; 3],
    pub dwWidth: u32,
    pub dwHeight: u32,
    pub lStride: i32,
    pub dwBitCount: u32,
    pub dwOffSurface: usize,
    pub wSelSurface: u16,
    pub wReserved: u16,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
    pub dwReserved3: u32,
    pub BeginAccess: isize,
    pub EndAccess: isize,
    pub DestroySurface: isize,
}
impl Default for DCISURFACEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DCI_1632_ACCESS: u32 = 64u32;
pub const DCI_ASYNC: u32 = 1024u32;
pub const DCI_CANOVERLAY: u32 = 65536u32;
pub const DCI_CAN_STRETCHX: u32 = 4096u32;
pub const DCI_CAN_STRETCHXN: u32 = 16384u32;
pub const DCI_CAN_STRETCHY: u32 = 8192u32;
pub const DCI_CAN_STRETCHYN: u32 = 32768u32;
pub const DCI_CHROMAKEY: u32 = 32u32;
pub const DCI_DWORDALIGN: u32 = 256u32;
pub const DCI_DWORDSIZE: u32 = 128u32;
pub const DCI_ERR_CURRENTLYNOTAVAIL: i32 = -5i32;
pub const DCI_ERR_HEIGHTALIGN: i32 = -21i32;
pub const DCI_ERR_INVALIDCLIPLIST: i32 = -15i32;
pub const DCI_ERR_INVALIDPOSITION: i32 = -13i32;
pub const DCI_ERR_INVALIDRECT: i32 = -6i32;
pub const DCI_ERR_INVALIDSTRETCH: i32 = -14i32;
pub const DCI_ERR_OUTOFMEMORY: i32 = -12i32;
pub const DCI_ERR_SURFACEISOBSCURED: i32 = -16i32;
pub const DCI_ERR_TOOBIGHEIGHT: i32 = -9i32;
pub const DCI_ERR_TOOBIGSIZE: i32 = -11i32;
pub const DCI_ERR_TOOBIGWIDTH: i32 = -10i32;
pub const DCI_ERR_UNSUPPORTEDFORMAT: i32 = -7i32;
pub const DCI_ERR_UNSUPPORTEDMASK: i32 = -8i32;
pub const DCI_ERR_WIDTHALIGN: i32 = -20i32;
pub const DCI_ERR_XALIGN: i32 = -17i32;
pub const DCI_ERR_XYALIGN: i32 = -19i32;
pub const DCI_ERR_YALIGN: i32 = -18i32;
pub const DCI_FAIL_GENERIC: i32 = -1i32;
pub const DCI_FAIL_INVALIDSURFACE: i32 = -3i32;
pub const DCI_FAIL_UNSUPPORTED: i32 = -4i32;
pub const DCI_FAIL_UNSUPPORTEDVERSION: i32 = -2i32;
pub const DCI_OFFSCREEN: u32 = 1u32;
pub const DCI_OK: u32 = 0u32;
pub const DCI_OVERLAY: u32 = 2u32;
pub const DCI_PRIMARY: u32 = 0u32;
pub const DCI_STATUS_CHROMAKEYCHANGED: u32 = 16u32;
pub const DCI_STATUS_FORMATCHANGED: u32 = 4u32;
pub const DCI_STATUS_POINTERCHANGED: u32 = 1u32;
pub const DCI_STATUS_STRIDECHANGED: u32 = 2u32;
pub const DCI_STATUS_SURFACEINFOCHANGED: u32 = 8u32;
pub const DCI_STATUS_WASSTILLDRAWING: u32 = 32u32;
pub const DCI_SURFACE_TYPE: u32 = 15u32;
pub const DCI_VERSION: u32 = 256u32;
pub const DCI_VISIBLE: u32 = 16u32;
pub const DCI_WRITEONLY: u32 = 512u32;
pub const DEACTIVATE_ACTCTX_FLAG_FORCE_EARLY_DEACTIVATION: u32 = 1u32;
pub type DECISION_LOCATION = i32;
pub const DECISION_LOCATION_AUDIT: DECISION_LOCATION = 2i32;
pub const DECISION_LOCATION_ENFORCE_STATE_LIST: DECISION_LOCATION = 7i32;
pub const DECISION_LOCATION_ENTERPRISE_DEFINED_CLASS_ID: DECISION_LOCATION = 4i32;
pub const DECISION_LOCATION_FAILED_CONVERT_GUID: DECISION_LOCATION = 3i32;
pub const DECISION_LOCATION_GLOBAL_BUILT_IN_LIST: DECISION_LOCATION = 5i32;
pub const DECISION_LOCATION_NOT_FOUND: DECISION_LOCATION = 8i32;
pub const DECISION_LOCATION_PARAMETER_VALIDATION: DECISION_LOCATION = 1i32;
pub const DECISION_LOCATION_PROVIDER_BUILT_IN_LIST: DECISION_LOCATION = 6i32;
pub const DECISION_LOCATION_REFRESH_GLOBAL_DATA: DECISION_LOCATION = 0i32;
pub const DECISION_LOCATION_UNKNOWN: DECISION_LOCATION = 9i32;
pub const DELAYLOAD_GPA_FAILURE: u32 = 4u32;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct DELAYLOAD_INFO {
    pub Size: u32,
    pub DelayloadDescriptor: *mut IMAGE_DELAYLOAD_DESCRIPTOR,
    pub ThunkAddress: *mut IMAGE_THUNK_DATA32,
    pub TargetDllName: windows_sys::core::PCSTR,
    pub TargetApiDescriptor: DELAYLOAD_PROC_DESCRIPTOR,
    pub TargetModuleBase: *mut core::ffi::c_void,
    pub Unused: *mut core::ffi::c_void,
    pub LastError: u32,
}
#[cfg(target_arch = "x86")]
impl Default for DELAYLOAD_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct DELAYLOAD_INFO {
    pub Size: u32,
    pub DelayloadDescriptor: *mut IMAGE_DELAYLOAD_DESCRIPTOR,
    pub ThunkAddress: *mut IMAGE_THUNK_DATA64,
    pub TargetDllName: windows_sys::core::PCSTR,
    pub TargetApiDescriptor: DELAYLOAD_PROC_DESCRIPTOR,
    pub TargetModuleBase: *mut core::ffi::c_void,
    pub Unused: *mut core::ffi::c_void,
    pub LastError: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for DELAYLOAD_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DELAYLOAD_PROC_DESCRIPTOR {
    pub ImportDescribedByName: u32,
    pub Description: DELAYLOAD_PROC_DESCRIPTOR_0,
}
impl Default for DELAYLOAD_PROC_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DELAYLOAD_PROC_DESCRIPTOR_0 {
    pub Name: windows_sys::core::PCSTR,
    pub Ordinal: u32,
}
impl Default for DELAYLOAD_PROC_DESCRIPTOR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DELETE_BROWSING_HISTORY_COOKIES: u32 = 2u32;
pub const DELETE_BROWSING_HISTORY_DOWNLOADHISTORY: u32 = 64u32;
pub const DELETE_BROWSING_HISTORY_FORMDATA: u32 = 8u32;
pub const DELETE_BROWSING_HISTORY_HISTORY: u32 = 1u32;
pub const DELETE_BROWSING_HISTORY_PASSWORDS: u32 = 16u32;
pub const DELETE_BROWSING_HISTORY_PRESERVEFAVORITES: u32 = 32u32;
pub const DELETE_BROWSING_HISTORY_TIF: u32 = 4u32;
pub const DOCKINFO_DOCKED: u32 = 2u32;
pub const DOCKINFO_UNDOCKED: u32 = 1u32;
pub const DOCKINFO_USER_SUPPLIED: u32 = 4u32;
pub const DRIVE_CDROM: u32 = 5u32;
pub const DRIVE_FIXED: u32 = 3u32;
pub const DRIVE_NO_ROOT_DIR: u32 = 1u32;
pub const DRIVE_RAMDISK: u32 = 6u32;
pub const DRIVE_REMOTE: u32 = 4u32;
pub const DRIVE_REMOVABLE: u32 = 2u32;
pub const DRIVE_UNKNOWN: u32 = 0u32;
pub const DTR_CONTROL_DISABLE: u32 = 0u32;
pub const DTR_CONTROL_ENABLE: u32 = 1u32;
pub const DTR_CONTROL_HANDSHAKE: u32 = 2u32;
pub const DefaultBrowserSyncSettings: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3ac83423_3112_4aa6_9b5b_1feb23d0c5f9);
pub const EFSRPC_SECURE_ONLY: u32 = 8u32;
pub const EFS_DROP_ALTERNATE_STREAMS: u32 = 16u32;
pub const EFS_USE_RECOVERY_KEYS: u32 = 1u32;
pub const ENTITY_LIST_ID: u32 = 0u32;
pub const ENTITY_TYPE_ID: u32 = 1u32;
pub type ENUM_CALLBACK = Option<unsafe extern "system" fn(lpsurfaceinfo: *mut DCISURFACEINFO, lpcontext: *mut core::ffi::c_void)>;
pub const ER_ENTITY: TDIENTITY_ENTITY_TYPE = 896u32;
pub const ER_ICMP: u32 = 896u32;
pub const EVENTLOG_FULL_INFO: u32 = 0u32;
pub const EditionUpgradeBroker: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc4270827_4f39_45df_9288_12ff6b85a921);
pub const EditionUpgradeHelper: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x01776df3_b9af_4e50_9b1c_56e93116d704);
pub const EndpointIoControlType: TDI_TL_IO_CONTROL_TYPE = 0i32;
pub const FAIL_FAST_GENERATE_EXCEPTION_ADDRESS: u32 = 1u32;
pub const FAIL_FAST_NO_HARD_ERROR_DLG: u32 = 2u32;
pub type FEATURE_CHANGE_TIME = i32;
pub const FEATURE_CHANGE_TIME_MODULE_RELOAD: FEATURE_CHANGE_TIME = 1i32;
pub const FEATURE_CHANGE_TIME_READ: FEATURE_CHANGE_TIME = 0i32;
pub const FEATURE_CHANGE_TIME_REBOOT: FEATURE_CHANGE_TIME = 3i32;
pub const FEATURE_CHANGE_TIME_SESSION: FEATURE_CHANGE_TIME = 2i32;
pub type FEATURE_ENABLED_STATE = i32;
pub const FEATURE_ENABLED_STATE_DEFAULT: FEATURE_ENABLED_STATE = 0i32;
pub const FEATURE_ENABLED_STATE_DISABLED: FEATURE_ENABLED_STATE = 1i32;
pub const FEATURE_ENABLED_STATE_ENABLED: FEATURE_ENABLED_STATE = 2i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FEATURE_ERROR {
    pub hr: windows_sys::core::HRESULT,
    pub lineNumber: u16,
    pub file: windows_sys::core::PCSTR,
    pub process: windows_sys::core::PCSTR,
    pub module: windows_sys::core::PCSTR,
    pub callerReturnAddressOffset: u32,
    pub callerModule: windows_sys::core::PCSTR,
    pub message: windows_sys::core::PCSTR,
    pub originLineNumber: u16,
    pub originFile: windows_sys::core::PCSTR,
    pub originModule: windows_sys::core::PCSTR,
    pub originCallerReturnAddressOffset: u32,
    pub originCallerModule: windows_sys::core::PCSTR,
    pub originName: windows_sys::core::PCSTR,
}
impl Default for FEATURE_ERROR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type FEATURE_STATE_CHANGE_SUBSCRIPTION = *mut core::ffi::c_void;
pub const FIBER_FLAG_FLOAT_SWITCH: u32 = 1u32;
pub const FILE_CREATED: u32 = 2u32;
pub const FILE_DIR_DISALLOWED: u32 = 9u32;
pub const FILE_DOES_NOT_EXIST: u32 = 5u32;
pub const FILE_ENCRYPTABLE: u32 = 0u32;
pub const FILE_EXISTS: u32 = 4u32;
pub const FILE_FLAG_IGNORE_IMPERSONATED_DEVICEMAP: u32 = 131072u32;
pub const FILE_FLAG_OPEN_REQUIRING_OPLOCK: u32 = 262144u32;
pub const FILE_IS_ENCRYPTED: u32 = 1u32;
pub const FILE_MAXIMUM_DISPOSITION: u32 = 5u32;
pub const FILE_NO_COMPRESSION: u32 = 32768u32;
pub const FILE_OPENED: u32 = 1u32;
pub const FILE_OPEN_NO_RECALL: u32 = 4194304u32;
pub const FILE_OPEN_REMOTE_INSTANCE: u32 = 1024u32;
pub const FILE_OVERWRITTEN: u32 = 3u32;
pub const FILE_READ_ONLY: u32 = 8u32;
pub const FILE_RENAME_FLAG_POSIX_SEMANTICS: u32 = 2u32;
pub const FILE_RENAME_FLAG_REPLACE_IF_EXISTS: u32 = 1u32;
pub const FILE_RENAME_FLAG_SUPPRESS_PIN_STATE_INHERITANCE: u32 = 4u32;
pub const FILE_ROOT_DIR: u32 = 3u32;
pub const FILE_SKIP_COMPLETION_PORT_ON_SUCCESS: u32 = 1u32;
pub const FILE_SKIP_SET_EVENT_ON_HANDLE: u32 = 2u32;
pub const FILE_SUPERSEDED: u32 = 0u32;
pub const FILE_SYSTEM_ATTR: u32 = 2u32;
pub const FILE_SYSTEM_DIR: u32 = 4u32;
pub const FILE_SYSTEM_NOT_SUPPORT: u32 = 6u32;
pub const FILE_UNKNOWN: u32 = 5u32;
pub const FILE_USER_DISALLOWED: u32 = 7u32;
pub const FILE_VALID_MAILSLOT_OPTION_FLAGS: u32 = 50u32;
pub const FILE_VALID_OPTION_FLAGS: u32 = 16777215u32;
pub const FILE_VALID_PIPE_OPTION_FLAGS: u32 = 50u32;
pub const FILE_VALID_SET_FLAGS: u32 = 54u32;
pub const FIND_ACTCTX_SECTION_KEY_RETURN_ASSEMBLY_METADATA: u32 = 4u32;
pub const FIND_ACTCTX_SECTION_KEY_RETURN_FLAGS: u32 = 2u32;
pub const FIND_ACTCTX_SECTION_KEY_RETURN_HACTCTX: u32 = 1u32;
pub const FORMAT_MESSAGE_MAX_WIDTH_MASK: u32 = 255u32;
pub const FS_CASE_IS_PRESERVED: u32 = 2u32;
pub const FS_CASE_SENSITIVE: u32 = 1u32;
pub const FS_FILE_COMPRESSION: u32 = 16u32;
pub const FS_FILE_ENCRYPTION: u32 = 131072u32;
pub const FS_PERSISTENT_ACLS: u32 = 8u32;
pub const FS_UNICODE_STORED_ON_DISK: u32 = 4u32;
pub const FS_VOL_IS_COMPRESSED: u32 = 32768u32;
pub const GENERIC_ENTITY: TDIENTITY_ENTITY_TYPE = 0u32;
pub const GET_SYSTEM_WOW64_DIRECTORY_NAME_A_A: windows_sys::core::PCSTR = windows_sys::core::s!("GetSystemWow64DirectoryA");
pub const GET_SYSTEM_WOW64_DIRECTORY_NAME_A_T: windows_sys::core::PCWSTR = windows_sys::core::w!("GetSystemWow64DirectoryA");
pub const GET_SYSTEM_WOW64_DIRECTORY_NAME_A_W: windows_sys::core::PCWSTR = windows_sys::core::w!("GetSystemWow64DirectoryA");
pub const GET_SYSTEM_WOW64_DIRECTORY_NAME_T_A: windows_sys::core::PCWSTR = windows_sys::core::w!("GetSystemWow64DirectoryW");
pub const GET_SYSTEM_WOW64_DIRECTORY_NAME_T_T: windows_sys::core::PCWSTR = windows_sys::core::w!("GetSystemWow64DirectoryW");
pub const GET_SYSTEM_WOW64_DIRECTORY_NAME_T_W: windows_sys::core::PCWSTR = windows_sys::core::w!("GetSystemWow64DirectoryW");
pub const GET_SYSTEM_WOW64_DIRECTORY_NAME_W_A: windows_sys::core::PCSTR = windows_sys::core::s!("GetSystemWow64DirectoryW");
pub const GET_SYSTEM_WOW64_DIRECTORY_NAME_W_T: windows_sys::core::PCWSTR = windows_sys::core::w!("GetSystemWow64DirectoryW");
pub const GET_SYSTEM_WOW64_DIRECTORY_NAME_W_W: windows_sys::core::PCWSTR = windows_sys::core::w!("GetSystemWow64DirectoryW");
pub const GMEM_DDESHARE: u32 = 8192u32;
pub const GMEM_DISCARDABLE: u32 = 256u32;
pub const GMEM_DISCARDED: u32 = 16384u32;
pub const GMEM_INVALID_HANDLE: u32 = 32768u32;
pub const GMEM_LOCKCOUNT: u32 = 255u32;
pub const GMEM_LOWER: u32 = 4096u32;
pub const GMEM_MODIFY: u32 = 128u32;
pub const GMEM_NOCOMPACT: u32 = 16u32;
pub const GMEM_NODISCARD: u32 = 32u32;
pub const GMEM_NOTIFY: u32 = 16384u32;
pub const GMEM_NOT_BANKED: u32 = 4096u32;
pub const GMEM_SHARE: u32 = 8192u32;
pub const GMEM_VALID_FLAGS: u32 = 32626u32;
pub const GetSockOptIoControlType: TDI_TL_IO_CONTROL_TYPE = 2i32;
pub const HANJA_WINDOW: u32 = 2u32;
pub const HINSTANCE_ERROR: u32 = 32u32;
pub type HWINWATCH = *mut core::ffi::c_void;
pub const HW_PROFILE_GUIDLEN: u32 = 39u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HW_PROFILE_INFOA {
    pub dwDockInfo: u32,
    pub szHwProfileGuid: [i8; 39],
    pub szHwProfileName: [i8; 80],
}
impl Default for HW_PROFILE_INFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HW_PROFILE_INFOW {
    pub dwDockInfo: u32,
    pub szHwProfileGuid: [u16; 39],
    pub szHwProfileName: [u16; 80],
}
impl Default for HW_PROFILE_INFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IE4_BACKNEW: u32 = 2u32;
pub const IE4_EXTRAINCREFCNT: u32 = 2048u32;
pub const IE4_FRDOALL: u32 = 256u32;
pub const IE4_NODELETENEW: u32 = 4u32;
pub const IE4_NOENUMKEY: u32 = 32u32;
pub const IE4_NOMESSAGES: u32 = 8u32;
pub const IE4_NOPROGRESS: u32 = 16u32;
pub const IE4_NO_CRC_MAPPING: u32 = 64u32;
pub const IE4_REGSECTION: u32 = 128u32;
pub const IE4_REMOVREGBKDATA: u32 = 4096u32;
pub const IE4_RESTORE: u32 = 1u32;
pub const IE4_UPDREFCNT: u32 = 512u32;
pub const IE4_USEREFCNT: u32 = 1024u32;
pub const IE_BADID: i32 = -1i32;
pub const IE_BAUDRATE: i32 = -12i32;
pub const IE_BYTESIZE: i32 = -11i32;
pub const IE_DEFAULT: i32 = -5i32;
pub const IE_HARDWARE: i32 = -10i32;
pub const IE_MEMORY: i32 = -4i32;
pub const IE_NOPEN: i32 = -3i32;
pub const IE_OPEN: i32 = -2i32;
pub const IF_ENTITY: TDIENTITY_ENTITY_TYPE = 512u32;
pub const IF_GENERIC: u32 = 512u32;
pub const IF_MIB: u32 = 514u32;
pub const IGNORE: u32 = 0u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IMAGE_DELAYLOAD_DESCRIPTOR {
    pub Attributes: IMAGE_DELAYLOAD_DESCRIPTOR_0,
    pub DllNameRVA: u32,
    pub ModuleHandleRVA: u32,
    pub ImportAddressTableRVA: u32,
    pub ImportNameTableRVA: u32,
    pub BoundImportAddressTableRVA: u32,
    pub UnloadInformationTableRVA: u32,
    pub TimeDateStamp: u32,
}
impl Default for IMAGE_DELAYLOAD_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IMAGE_DELAYLOAD_DESCRIPTOR_0 {
    pub AllAttributes: u32,
    pub Anonymous: IMAGE_DELAYLOAD_DESCRIPTOR_0_0,
}
impl Default for IMAGE_DELAYLOAD_DESCRIPTOR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct IMAGE_DELAYLOAD_DESCRIPTOR_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IMAGE_THUNK_DATA32 {
    pub u1: IMAGE_THUNK_DATA32_0,
}
impl Default for IMAGE_THUNK_DATA32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IMAGE_THUNK_DATA32_0 {
    pub ForwarderString: u32,
    pub Function: u32,
    pub Ordinal: u32,
    pub AddressOfData: u32,
}
impl Default for IMAGE_THUNK_DATA32_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IMAGE_THUNK_DATA64 {
    pub u1: IMAGE_THUNK_DATA64_0,
}
impl Default for IMAGE_THUNK_DATA64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IMAGE_THUNK_DATA64_0 {
    pub ForwarderString: u64,
    pub Function: u64,
    pub Ordinal: u64,
    pub AddressOfData: u64,
}
impl Default for IMAGE_THUNK_DATA64_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IMEA_INIT: u32 = 1u32;
pub const IMEA_NEXT: u32 = 2u32;
pub const IMEA_PREV: u32 = 3u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IMEPROA {
    pub hWnd: super::super::Foundation::HWND,
    pub InstDate: DATETIME,
    pub wVersion: u32,
    pub szDescription: [u8; 50],
    pub szName: [u8; 80],
    pub szOptions: [u8; 30],
}
impl Default for IMEPROA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IMEPROW {
    pub hWnd: super::super::Foundation::HWND,
    pub InstDate: DATETIME,
    pub wVersion: u32,
    pub szDescription: [u16; 50],
    pub szName: [u16; 80],
    pub szOptions: [u16; 30],
}
impl Default for IMEPROW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct IMESTRUCT {
    pub fnc: u32,
    pub wParam: super::super::Foundation::WPARAM,
    pub wCount: u32,
    pub dchSource: u32,
    pub dchDest: u32,
    pub lParam1: super::super::Foundation::LPARAM,
    pub lParam2: super::super::Foundation::LPARAM,
    pub lParam3: super::super::Foundation::LPARAM,
}
pub const IME_BANJAtoJUNJA: u32 = 19u32;
pub const IME_ENABLE_CONVERT: u32 = 2u32;
pub const IME_ENTERWORDREGISTERMODE: u32 = 24u32;
pub const IME_GETCONVERSIONMODE: u32 = 17u32;
pub const IME_GETIMECAPS: u32 = 3u32;
pub const IME_GETOPEN: u32 = 5u32;
pub const IME_GETVERSION: u32 = 7u32;
pub const IME_JOHABtoKS: u32 = 21u32;
pub const IME_JUNJAtoBANJA: u32 = 20u32;
pub const IME_KStoJOHAB: u32 = 22u32;
pub const IME_MAXPROCESS: u32 = 32u32;
pub const IME_MODE_ALPHANUMERIC: u32 = 1u32;
pub const IME_MODE_CODEINPUT: u32 = 128u32;
pub const IME_MODE_DBCSCHAR: u32 = 16u32;
pub const IME_MODE_HANJACONVERT: u32 = 4u32;
pub const IME_MODE_HIRAGANA: u32 = 4u32;
pub const IME_MODE_KATAKANA: u32 = 2u32;
pub const IME_MODE_NOCODEINPUT: u32 = 256u32;
pub const IME_MODE_NOROMAN: u32 = 64u32;
pub const IME_MODE_ROMAN: u32 = 32u32;
pub const IME_MODE_SBCSCHAR: u32 = 2u32;
pub const IME_MOVEIMEWINDOW: u32 = 8u32;
pub const IME_REQUEST_CONVERT: u32 = 1u32;
pub const IME_RS_DISKERROR: u32 = 14u32;
pub const IME_RS_ERROR: u32 = 1u32;
pub const IME_RS_ILLEGAL: u32 = 6u32;
pub const IME_RS_INVALID: u32 = 17u32;
pub const IME_RS_NEST: u32 = 18u32;
pub const IME_RS_NOIME: u32 = 2u32;
pub const IME_RS_NOROOM: u32 = 10u32;
pub const IME_RS_NOTFOUND: u32 = 7u32;
pub const IME_RS_SYSTEMMODAL: u32 = 19u32;
pub const IME_RS_TOOLONG: u32 = 5u32;
pub const IME_SENDVKEY: u32 = 19u32;
pub const IME_SETCONVERSIONFONTEX: u32 = 25u32;
pub const IME_SETCONVERSIONMODE: u32 = 16u32;
pub const IME_SETCONVERSIONWINDOW: u32 = 8u32;
pub const IME_SETOPEN: u32 = 4u32;
pub const IME_SET_MODE: u32 = 18u32;
pub const INFO_CLASS_GENERIC: u32 = 256u32;
pub const INFO_CLASS_IMPLEMENTATION: u32 = 768u32;
pub const INFO_CLASS_PROTOCOL: u32 = 512u32;
pub const INFO_TYPE_ADDRESS_OBJECT: u32 = 512u32;
pub const INFO_TYPE_CONNECTION: u32 = 768u32;
pub const INFO_TYPE_PROVIDER: u32 = 256u32;
pub const INTERIM_WINDOW: u32 = 0u32;
pub const INVALID_ENTITY_INSTANCE: i32 = -1i32;
pub const IOCTL_TDI_TL_IO_CONTROL_ENDPOINT: u32 = 2162744u32;
pub const IR_CHANGECONVERT: u32 = 289u32;
pub const IR_CLOSECONVERT: u32 = 290u32;
pub const IR_DBCSCHAR: u32 = 352u32;
pub const IR_FULLCONVERT: u32 = 291u32;
pub const IR_IMESELECT: u32 = 304u32;
pub const IR_MODEINFO: u32 = 400u32;
pub const IR_OPENCONVERT: u32 = 288u32;
pub const IR_STRING: u32 = 320u32;
pub const IR_STRINGEND: u32 = 257u32;
pub const IR_STRINGEX: u32 = 384u32;
pub const IR_STRINGSTART: u32 = 256u32;
pub const IR_UNDETERMINE: u32 = 368u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JAVA_TRUST {
    pub cbSize: u32,
    pub flag: u32,
    pub fAllActiveXPermissions: windows_sys::core::BOOL,
    pub fAllPermissions: windows_sys::core::BOOL,
    pub dwEncodingType: u32,
    pub pbJavaPermissions: *mut u8,
    pub cbJavaPermissions: u32,
    pub pbSigner: *mut u8,
    pub cbSigner: u32,
    pub pwszZone: windows_sys::core::PCWSTR,
    pub guidZone: windows_sys::core::GUID,
    pub hVerify: windows_sys::core::HRESULT,
}
impl Default for JAVA_TRUST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct JIT_DEBUG_INFO {
    pub dwSize: u32,
    pub dwProcessorArchitecture: u32,
    pub dwThreadID: u32,
    pub dwReserved0: u32,
    pub lpExceptionAddress: u64,
    pub lpExceptionRecord: u64,
    pub lpContextRecord: u64,
}
pub const KEY_ALL_KEYS: WLDP_KEY = 2i32;
pub const KEY_OVERRIDE: WLDP_KEY = 1i32;
pub const KEY_UNKNOWN: WLDP_KEY = 0i32;
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy)]
pub struct LDR_DATA_TABLE_ENTRY {
    pub Reserved1: [*mut core::ffi::c_void; 2],
    pub InMemoryOrderLinks: super::Kernel::LIST_ENTRY,
    pub Reserved2: [*mut core::ffi::c_void; 2],
    pub DllBase: *mut core::ffi::c_void,
    pub Reserved3: [*mut core::ffi::c_void; 2],
    pub FullDllName: super::super::Foundation::UNICODE_STRING,
    pub Reserved4: [u8; 8],
    pub Reserved5: [*mut core::ffi::c_void; 3],
    pub Anonymous: LDR_DATA_TABLE_ENTRY_0,
    pub TimeDateStamp: u32,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for LDR_DATA_TABLE_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy)]
pub union LDR_DATA_TABLE_ENTRY_0 {
    pub CheckSum: u32,
    pub Reserved6: *mut core::ffi::c_void,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for LDR_DATA_TABLE_ENTRY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const LIS_NOGRPCONV: u32 = 2u32;
pub const LIS_QUIET: u32 = 1u32;
pub const LOGON32_PROVIDER_VIRTUAL: u32 = 4u32;
pub const LOGON32_PROVIDER_WINNT35: u32 = 1u32;
pub const LOGON_ZERO_PASSWORD_BUFFER: u32 = 2147483648u32;
pub const LPTx: u32 = 128u32;
pub const MAXINTATOM: u32 = 49152u32;
pub const MAX_COMPUTERNAME_LENGTH: u32 = 15u32;
pub const MAX_TDI_ENTITIES: u32 = 4096u32;
pub const MCW_DEFAULT: u32 = 0u32;
pub const MCW_HIDDEN: u32 = 16u32;
pub const MCW_RECT: u32 = 1u32;
pub const MCW_SCREEN: u32 = 4u32;
pub const MCW_VERTICAL: u32 = 8u32;
pub const MCW_WINDOW: u32 = 2u32;
pub const MICROSOFT_WINBASE_H_DEFINE_INTERLOCKED_CPLUSPLUS_OVERLOADS: u32 = 0u32;
pub const MICROSOFT_WINDOWS_WINBASE_H_DEFINE_INTERLOCKED_CPLUSPLUS_OVERLOADS: u32 = 0u32;
pub const MODE_WINDOW: u32 = 1u32;
pub const OFS_MAXPATHNAME: u32 = 128u32;
pub const OPERATION_API_VERSION: u32 = 1u32;
pub const OVERWRITE_HIDDEN: u32 = 4u32;
pub const PCF_16BITMODE: u32 = 512u32;
pub const PCF_DTRDSR: u32 = 1u32;
pub const PCF_INTTIMEOUTS: u32 = 128u32;
pub const PCF_PARITY_CHECK: u32 = 8u32;
pub const PCF_RLSD: u32 = 4u32;
pub const PCF_RTSCTS: u32 = 2u32;
pub const PCF_SETXCHAR: u32 = 32u32;
pub const PCF_SPECIALCHARS: u32 = 256u32;
pub const PCF_TOTALTIMEOUTS: u32 = 64u32;
pub const PCF_XONXOFF: u32 = 16u32;
pub type PDELAYLOAD_FAILURE_DLL_CALLBACK = Option<unsafe extern "system" fn(notificationreason: u32, delayloadinfo: *const DELAYLOAD_INFO) -> *mut core::ffi::c_void>;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PERUSERSECTIONA {
    pub szGUID: [i8; 59],
    pub szDispName: [i8; 128],
    pub szLocale: [i8; 10],
    pub szStub: [i8; 1040],
    pub szVersion: [i8; 32],
    pub szCompID: [i8; 128],
    pub dwIsInstalled: u32,
    pub bRollback: windows_sys::core::BOOL,
}
impl Default for PERUSERSECTIONA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PERUSERSECTIONW {
    pub szGUID: [u16; 59],
    pub szDispName: [u16; 128],
    pub szLocale: [u16; 10],
    pub szStub: [u16; 1040],
    pub szVersion: [u16; 32],
    pub szCompID: [u16; 128],
    pub dwIsInstalled: u32,
    pub bRollback: windows_sys::core::BOOL,
}
impl Default for PERUSERSECTIONW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PFEATURE_STATE_CHANGE_CALLBACK = Option<unsafe extern "system" fn(context: *const core::ffi::c_void)>;
pub type PFIBER_CALLOUT_ROUTINE = Option<unsafe extern "system" fn(lpparameter: *mut core::ffi::c_void) -> *mut core::ffi::c_void>;
pub type PQUERYACTCTXW_FUNC = Option<unsafe extern "system" fn(dwflags: u32, hactctx: super::super::Foundation::HANDLE, pvsubinstance: *const core::ffi::c_void, ulinfoclass: u32, pvbuffer: *mut core::ffi::c_void, cbbuffer: usize, pcbwrittenorrequired: *mut usize) -> windows_sys::core::BOOL>;
pub const PROCESS_CREATION_ALL_APPLICATION_PACKAGES_OPT_OUT: u32 = 1u32;
pub const PROCESS_CREATION_CHILD_PROCESS_OVERRIDE: u32 = 2u32;
pub const PROCESS_CREATION_CHILD_PROCESS_RESTRICTED: u32 = 1u32;
pub const PROCESS_CREATION_CHILD_PROCESS_RESTRICTED_UNLESS_SECURE: u32 = 4u32;
pub const PROCESS_CREATION_DESKTOP_APP_BREAKAWAY_DISABLE_PROCESS_TREE: u32 = 2u32;
pub const PROCESS_CREATION_DESKTOP_APP_BREAKAWAY_ENABLE_PROCESS_TREE: u32 = 1u32;
pub const PROCESS_CREATION_DESKTOP_APP_BREAKAWAY_OVERRIDE: u32 = 4u32;
pub const PROCESS_CREATION_MITIGATION_POLICY_DEP_ATL_THUNK_ENABLE: u32 = 2u32;
pub const PROCESS_CREATION_MITIGATION_POLICY_DEP_ENABLE: u32 = 1u32;
pub const PROCESS_CREATION_MITIGATION_POLICY_SEHOP_ENABLE: u32 = 4u32;
pub const PROC_THREAD_ATTRIBUTE_ADDITIVE: u32 = 262144u32;
pub const PROC_THREAD_ATTRIBUTE_INPUT: u32 = 131072u32;
pub const PROC_THREAD_ATTRIBUTE_NUMBER: u32 = 65535u32;
pub const PROC_THREAD_ATTRIBUTE_THREAD: u32 = 65536u32;
pub const PROTECTION_LEVEL_SAME: u32 = 4294967295u32;
pub const PST_FAX: u32 = 33u32;
pub const PST_LAT: u32 = 257u32;
pub const PST_MODEM: u32 = 6u32;
pub const PST_NETWORK_BRIDGE: u32 = 256u32;
pub const PST_PARALLELPORT: u32 = 2u32;
pub const PST_RS232: u32 = 1u32;
pub const PST_RS422: u32 = 3u32;
pub const PST_RS423: u32 = 4u32;
pub const PST_RS449: u32 = 5u32;
pub const PST_SCANNER: u32 = 34u32;
pub const PST_TCPIP_TELNET: u32 = 258u32;
pub const PST_UNSPECIFIED: u32 = 0u32;
pub const PST_X25: u32 = 259u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PUBLIC_OBJECT_BASIC_INFORMATION {
    pub Attributes: u32,
    pub GrantedAccess: u32,
    pub HandleCount: u32,
    pub PointerCount: u32,
    pub Reserved: [u32; 10],
}
impl Default for PUBLIC_OBJECT_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PUBLIC_OBJECT_TYPE_INFORMATION {
    pub TypeName: super::super::Foundation::UNICODE_STRING,
    pub Reserved: [u32; 22],
}
impl Default for PUBLIC_OBJECT_TYPE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PWINSTATIONQUERYINFORMATIONW = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: u32, param2: WINSTATIONINFOCLASS, param3: *mut core::ffi::c_void, param4: u32, param5: *mut u32) -> bool>;
pub type PWLDP_CANEXECUTEBUFFER_API = Option<unsafe extern "system" fn(host: *const windows_sys::core::GUID, options: WLDP_EXECUTION_EVALUATION_OPTIONS, buffer: *const u8, buffersize: u32, auditinfo: windows_sys::core::PCWSTR, result: *mut WLDP_EXECUTION_POLICY) -> windows_sys::core::HRESULT>;
pub type PWLDP_CANEXECUTEFILE_API = Option<unsafe extern "system" fn(host: *const windows_sys::core::GUID, options: WLDP_EXECUTION_EVALUATION_OPTIONS, filehandle: super::super::Foundation::HANDLE, auditinfo: windows_sys::core::PCWSTR, result: *mut WLDP_EXECUTION_POLICY) -> windows_sys::core::HRESULT>;
#[cfg(feature = "Win32_System_Com")]
pub type PWLDP_CANEXECUTESTREAM_API = Option<unsafe extern "system" fn(host: *const windows_sys::core::GUID, options: WLDP_EXECUTION_EVALUATION_OPTIONS, stream: *mut core::ffi::c_void, auditinfo: windows_sys::core::PCWSTR, result: *mut WLDP_EXECUTION_POLICY) -> windows_sys::core::HRESULT>;
pub type PWLDP_ISAPPAPPROVEDBYPOLICY_API = Option<unsafe extern "system" fn(packagefamilyname: windows_sys::core::PCWSTR, packageversion: u64) -> windows_sys::core::HRESULT>;
pub type PWLDP_ISDYNAMICCODEPOLICYENABLED_API = Option<unsafe extern "system" fn(pbenabled: *mut windows_sys::core::BOOL) -> windows_sys::core::HRESULT>;
pub type PWLDP_ISPRODUCTIONCONFIGURATION_API = Option<unsafe extern "system" fn(isproductionconfiguration: *mut windows_sys::core::BOOL) -> windows_sys::core::HRESULT>;
pub type PWLDP_ISWCOSPRODUCTIONCONFIGURATION_API = Option<unsafe extern "system" fn(isproductionconfiguration: *mut windows_sys::core::BOOL) -> windows_sys::core::HRESULT>;
pub type PWLDP_QUERYDEVICESECURITYINFORMATION_API = Option<unsafe extern "system" fn(information: *mut WLDP_DEVICE_SECURITY_INFORMATION, informationlength: u32, returnlength: *mut u32) -> windows_sys::core::HRESULT>;
pub type PWLDP_QUERYDYNAMICODETRUST_API = Option<unsafe extern "system" fn(filehandle: super::super::Foundation::HANDLE, baseimage: *const core::ffi::c_void, imagesize: u32) -> windows_sys::core::HRESULT>;
pub type PWLDP_QUERYPOLICYSETTINGENABLED2_API = Option<unsafe extern "system" fn(setting: windows_sys::core::PCWSTR, enabled: *mut windows_sys::core::BOOL) -> windows_sys::core::HRESULT>;
pub type PWLDP_QUERYPOLICYSETTINGENABLED_API = Option<unsafe extern "system" fn(setting: WLDP_POLICY_SETTING, enabled: *mut windows_sys::core::BOOL) -> windows_sys::core::HRESULT>;
pub type PWLDP_QUERYWINDOWSLOCKDOWNMODE_API = Option<unsafe extern "system" fn(lockdownmode: *mut WLDP_WINDOWS_LOCKDOWN_MODE) -> windows_sys::core::HRESULT>;
pub type PWLDP_QUERYWINDOWSLOCKDOWNRESTRICTION_API = Option<unsafe extern "system" fn(lockdownrestriction: *mut WLDP_WINDOWS_LOCKDOWN_RESTRICTION) -> windows_sys::core::HRESULT>;
pub type PWLDP_RESETPRODUCTIONCONFIGURATION_API = Option<unsafe extern "system" fn() -> windows_sys::core::HRESULT>;
pub type PWLDP_RESETWCOSPRODUCTIONCONFIGURATION_API = Option<unsafe extern "system" fn() -> windows_sys::core::HRESULT>;
pub type PWLDP_SETDYNAMICCODETRUST_API = Option<unsafe extern "system" fn(hfilehandle: super::super::Foundation::HANDLE) -> windows_sys::core::HRESULT>;
pub type PWLDP_SETWINDOWSLOCKDOWNRESTRICTION_API = Option<unsafe extern "system" fn(lockdownrestriction: WLDP_WINDOWS_LOCKDOWN_RESTRICTION) -> windows_sys::core::HRESULT>;
pub const QUERY_ACTCTX_FLAG_ACTCTX_IS_ADDRESS: u32 = 16u32;
pub const QUERY_ACTCTX_FLAG_ACTCTX_IS_HMODULE: u32 = 8u32;
pub const QUERY_ACTCTX_FLAG_NO_ADDREF: u32 = 2147483648u32;
pub const QUERY_ACTCTX_FLAG_USE_ACTIVE_ACTCTX: u32 = 4u32;
pub const RECOVERY_DEFAULT_PING_INTERVAL: u32 = 5000u32;
pub type REGINSTALLA = Option<unsafe extern "system" fn(hm: super::super::Foundation::HMODULE, pszsection: windows_sys::core::PCSTR, psttable: *const STRTABLEA) -> windows_sys::core::HRESULT>;
pub const REG_RESTORE_LOG_KEY: windows_sys::core::PCWSTR = windows_sys::core::w!("RegRestoreLogFile");
pub const REG_SAVE_LOG_KEY: windows_sys::core::PCWSTR = windows_sys::core::w!("RegSaveLogFile");
pub const REMOTE_PROTOCOL_INFO_FLAG_LOOPBACK: u32 = 1u32;
pub const REMOTE_PROTOCOL_INFO_FLAG_OFFLINE: u32 = 2u32;
pub const REMOTE_PROTOCOL_INFO_FLAG_PERSISTENT_HANDLE: u32 = 4u32;
pub const RESETDEV: u32 = 7u32;
pub const RESTART_MAX_CMD_LINE: u32 = 1024u32;
pub const RPI_FLAG_SMB2_SHARECAP_CLUSTER: u32 = 64u32;
pub const RPI_FLAG_SMB2_SHARECAP_CONTINUOUS_AVAILABILITY: u32 = 16u32;
pub const RPI_FLAG_SMB2_SHARECAP_DFS: u32 = 8u32;
pub const RPI_FLAG_SMB2_SHARECAP_SCALEOUT: u32 = 32u32;
pub const RPI_FLAG_SMB2_SHARECAP_TIMEWARP: u32 = 2u32;
pub const RPI_SMB2_FLAG_SERVERCAP_DFS: u32 = 1u32;
pub const RPI_SMB2_FLAG_SERVERCAP_DIRECTORY_LEASING: u32 = 32u32;
pub const RPI_SMB2_FLAG_SERVERCAP_LARGEMTU: u32 = 4u32;
pub const RPI_SMB2_FLAG_SERVERCAP_LEASING: u32 = 2u32;
pub const RPI_SMB2_FLAG_SERVERCAP_MULTICHANNEL: u32 = 8u32;
pub const RPI_SMB2_FLAG_SERVERCAP_PERSISTENT_HANDLES: u32 = 16u32;
pub const RPI_SMB2_SHAREFLAG_COMPRESS_DATA: u32 = 2u32;
pub const RPI_SMB2_SHAREFLAG_ENCRYPT_DATA: u32 = 1u32;
pub const RSC_FLAG_DELAYREGISTEROCX: u32 = 512u32;
pub const RSC_FLAG_INF: u32 = 1u32;
pub const RSC_FLAG_NGCONV: u32 = 8u32;
pub const RSC_FLAG_QUIET: u32 = 4u32;
pub const RSC_FLAG_SETUPAPI: u32 = 1024u32;
pub const RSC_FLAG_SKIPDISKSPACECHECK: u32 = 2u32;
pub const RSC_FLAG_UPDHLPDLLS: u32 = 16u32;
pub const RTS_CONTROL_DISABLE: u32 = 0u32;
pub const RTS_CONTROL_ENABLE: u32 = 1u32;
pub const RTS_CONTROL_HANDSHAKE: u32 = 2u32;
pub const RTS_CONTROL_TOGGLE: u32 = 3u32;
pub const RUNCMDS_DELAYPOSTCMD: u32 = 4u32;
pub const RUNCMDS_NOWAIT: u32 = 2u32;
pub const RUNCMDS_QUIET: u32 = 1u32;
pub const SCS_32BIT_BINARY: u32 = 0u32;
pub const SCS_64BIT_BINARY: u32 = 6u32;
pub const SCS_DOS_BINARY: u32 = 1u32;
pub const SCS_OS216_BINARY: u32 = 5u32;
pub const SCS_PIF_BINARY: u32 = 3u32;
pub const SCS_POSIX_BINARY: u32 = 4u32;
pub const SCS_THIS_PLATFORM_BINARY: u32 = 6u32;
pub const SCS_WOW_BINARY: u32 = 2u32;
pub const SHUTDOWN_NORETRY: u32 = 1u32;
pub const SP_BAUD: u32 = 2u32;
pub const SP_DATABITS: u32 = 4u32;
pub const SP_HANDSHAKING: u32 = 16u32;
pub const SP_PARITY: u32 = 1u32;
pub const SP_PARITY_CHECK: u32 = 32u32;
pub const SP_RLSD: u32 = 64u32;
pub const SP_SERIALCOMM: u32 = 1u32;
pub const SP_STOPBITS: u32 = 8u32;
pub const STARTF_HOLOGRAPHIC: u32 = 262144u32;
pub const STORAGE_INFO_FLAGS_ALIGNED_DEVICE: u32 = 1u32;
pub const STORAGE_INFO_FLAGS_PARTITION_ALIGNED_ON_DEVICE: u32 = 2u32;
pub const STORAGE_INFO_OFFSET_UNKNOWN: u32 = 4294967295u32;
pub const STREAM_CONTAINS_GHOSTED_FILE_EXTENTS: u32 = 16u32;
pub const STREAM_CONTAINS_PROPERTIES: u32 = 4u32;
pub const STREAM_CONTAINS_SECURITY: u32 = 2u32;
pub const STREAM_MODIFIED_WHEN_READ: u32 = 1u32;
pub const STREAM_NORMAL_ATTRIBUTE: u32 = 0u32;
pub const STREAM_SPARSE_ATTRIBUTE: u32 = 8u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct STRENTRYA {
    pub pszName: windows_sys::core::PSTR,
    pub pszValue: windows_sys::core::PSTR,
}
impl Default for STRENTRYA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct STRENTRYW {
    pub pszName: windows_sys::core::PWSTR,
    pub pszValue: windows_sys::core::PWSTR,
}
impl Default for STRENTRYW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct STRINGEXSTRUCT {
    pub dwSize: u32,
    pub uDeterminePos: u32,
    pub uDetermineDelimPos: u32,
    pub uYomiPos: u32,
    pub uYomiDelimPos: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct STRTABLEA {
    pub cEntries: u32,
    pub pse: *mut STRENTRYA,
}
impl Default for STRTABLEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct STRTABLEW {
    pub cEntries: u32,
    pub pse: *mut STRENTRYW,
}
impl Default for STRTABLEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SYSTEM_BASIC_INFORMATION {
    pub Reserved1: [u8; 24],
    pub Reserved2: [*mut core::ffi::c_void; 4],
    pub NumberOfProcessors: i8,
}
impl Default for SYSTEM_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SYSTEM_CODEINTEGRITY_INFORMATION {
    pub Length: u32,
    pub CodeIntegrityOptions: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SYSTEM_EXCEPTION_INFORMATION {
    pub Reserved1: [u8; 16],
}
impl Default for SYSTEM_EXCEPTION_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SYSTEM_INTERRUPT_INFORMATION {
    pub Reserved1: [u8; 24],
}
impl Default for SYSTEM_INTERRUPT_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SYSTEM_LOOKASIDE_INFORMATION {
    pub Reserved1: [u8; 32],
}
impl Default for SYSTEM_LOOKASIDE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SYSTEM_PERFORMANCE_INFORMATION {
    pub Reserved1: [u8; 312],
}
impl Default for SYSTEM_PERFORMANCE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SYSTEM_POLICY_INFORMATION {
    pub Reserved1: [*mut core::ffi::c_void; 2],
    pub Reserved2: [u32; 3],
}
impl Default for SYSTEM_POLICY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION {
    pub IdleTime: i64,
    pub KernelTime: i64,
    pub UserTime: i64,
    pub Reserved1: [i64; 2],
    pub Reserved2: u32,
}
impl Default for SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SYSTEM_PROCESS_INFORMATION {
    pub NextEntryOffset: u32,
    pub NumberOfThreads: u32,
    pub Reserved1: [u8; 48],
    pub ImageName: super::super::Foundation::UNICODE_STRING,
    pub BasePriority: i32,
    pub UniqueProcessId: super::super::Foundation::HANDLE,
    pub Reserved2: *mut core::ffi::c_void,
    pub HandleCount: u32,
    pub SessionId: u32,
    pub Reserved3: *mut core::ffi::c_void,
    pub PeakVirtualSize: usize,
    pub VirtualSize: usize,
    pub Reserved4: u32,
    pub PeakWorkingSetSize: usize,
    pub WorkingSetSize: usize,
    pub Reserved5: *mut core::ffi::c_void,
    pub QuotaPagedPoolUsage: usize,
    pub Reserved6: *mut core::ffi::c_void,
    pub QuotaNonPagedPoolUsage: usize,
    pub PagefileUsage: usize,
    pub PeakPagefileUsage: usize,
    pub PrivatePageCount: usize,
    pub Reserved7: [i64; 6],
}
impl Default for SYSTEM_PROCESS_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SYSTEM_REGISTRY_QUOTA_INFORMATION {
    pub RegistryQuotaAllowed: u32,
    pub RegistryQuotaUsed: u32,
    pub Reserved1: *mut core::ffi::c_void,
}
impl Default for SYSTEM_REGISTRY_QUOTA_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SYSTEM_STATUS_FLAG_POWER_SAVING_ON: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SYSTEM_THREAD_INFORMATION {
    pub Reserved1: [i64; 3],
    pub Reserved2: u32,
    pub StartAddress: *mut core::ffi::c_void,
    pub ClientId: CLIENT_ID,
    pub Priority: i32,
    pub BasePriority: i32,
    pub Reserved3: u32,
    pub ThreadState: u32,
    pub WaitReason: u32,
}
impl Default for SYSTEM_THREAD_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SYSTEM_TIMEOFDAY_INFORMATION {
    pub Reserved1: [u8; 48],
}
impl Default for SYSTEM_TIMEOFDAY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const S_ALLTHRESHOLD: u32 = 2u32;
pub const S_LEGATO: u32 = 1u32;
pub const S_NORMAL: u32 = 0u32;
pub const S_PERIOD1024: u32 = 1u32;
pub const S_PERIOD2048: u32 = 2u32;
pub const S_PERIOD512: u32 = 0u32;
pub const S_PERIODVOICE: u32 = 3u32;
pub const S_QUEUEEMPTY: u32 = 0u32;
pub const S_SERBDNT: i32 = -5i32;
pub const S_SERDCC: i32 = -7i32;
pub const S_SERDDR: i32 = -14i32;
pub const S_SERDFQ: i32 = -13i32;
pub const S_SERDLN: i32 = -6i32;
pub const S_SERDMD: i32 = -10i32;
pub const S_SERDPT: i32 = -12i32;
pub const S_SERDSH: i32 = -11i32;
pub const S_SERDSR: i32 = -15i32;
pub const S_SERDST: i32 = -16i32;
pub const S_SERDTP: i32 = -8i32;
pub const S_SERDVL: i32 = -9i32;
pub const S_SERDVNA: i32 = -1i32;
pub const S_SERMACT: i32 = -3i32;
pub const S_SEROFM: i32 = -2i32;
pub const S_SERQFUL: i32 = -4i32;
pub const S_STACCATO: u32 = 2u32;
pub const S_THRESHOLD: u32 = 1u32;
pub const S_WHITE1024: u32 = 5u32;
pub const S_WHITE2048: u32 = 6u32;
pub const S_WHITE512: u32 = 4u32;
pub const S_WHITEVOICE: u32 = 7u32;
pub const SetSockOptIoControlType: TDI_TL_IO_CONTROL_TYPE = 1i32;
pub const SocketIoControlType: TDI_TL_IO_CONTROL_TYPE = 3i32;
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct TCP_REQUEST_QUERY_INFORMATION_EX32_XP {
    pub ID: TDIObjectID,
    pub Context: [u32; 4],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for TCP_REQUEST_QUERY_INFORMATION_EX32_XP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TCP_REQUEST_QUERY_INFORMATION_EX_W2K {
    pub ID: TDIObjectID,
    pub Context: [u8; 16],
}
impl Default for TCP_REQUEST_QUERY_INFORMATION_EX_W2K {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TCP_REQUEST_QUERY_INFORMATION_EX_XP {
    pub ID: TDIObjectID,
    pub Context: [usize; 4],
}
impl Default for TCP_REQUEST_QUERY_INFORMATION_EX_XP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TCP_REQUEST_SET_INFORMATION_EX {
    pub ID: TDIObjectID,
    pub BufferSize: u32,
    pub Buffer: [u8; 1],
}
impl Default for TCP_REQUEST_SET_INFORMATION_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TC_GP_TRAP: u32 = 2u32;
pub const TC_HARDERR: u32 = 1u32;
pub const TC_NORMAL: u32 = 0u32;
pub const TC_SIGNAL: u32 = 3u32;
pub type TDIENTITY_ENTITY_TYPE = u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TDIEntityID {
    pub tei_entity: TDIENTITY_ENTITY_TYPE,
    pub tei_instance: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TDIObjectID {
    pub toi_entity: TDIEntityID,
    pub toi_class: u32,
    pub toi_type: u32,
    pub toi_id: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TDI_TL_IO_CONTROL_ENDPOINT {
    pub Type: TDI_TL_IO_CONTROL_TYPE,
    pub Level: u32,
    pub Anonymous: TDI_TL_IO_CONTROL_ENDPOINT_0,
    pub InputBuffer: *mut core::ffi::c_void,
    pub InputBufferLength: u32,
    pub OutputBuffer: *mut core::ffi::c_void,
    pub OutputBufferLength: u32,
}
impl Default for TDI_TL_IO_CONTROL_ENDPOINT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union TDI_TL_IO_CONTROL_ENDPOINT_0 {
    pub IoControlCode: u32,
    pub OptionName: u32,
}
impl Default for TDI_TL_IO_CONTROL_ENDPOINT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type TDI_TL_IO_CONTROL_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct THREAD_NAME_INFORMATION {
    pub ThreadName: super::super::Foundation::UNICODE_STRING,
}
pub const THREAD_PRIORITY_ERROR_RETURN: u32 = 2147483647u32;
pub const UMS_VERSION: u32 = 256u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct UNDETERMINESTRUCT {
    pub dwSize: u32,
    pub uDefIMESize: u32,
    pub uDefIMEPos: u32,
    pub uUndetTextLen: u32,
    pub uUndetTextPos: u32,
    pub uUndetAttrPos: u32,
    pub uCursorPos: u32,
    pub uDeltaStart: u32,
    pub uDetermineTextLen: u32,
    pub uDetermineTextPos: u32,
    pub uDetermineDelimPos: u32,
    pub uYomiTextLen: u32,
    pub uYomiTextPos: u32,
    pub uYomiDelimPos: u32,
}
pub type VALUENAME = i32;
pub const VALUENAME_BUILT_IN_LIST: VALUENAME = 2i32;
pub const VALUENAME_ENTERPRISE_DEFINED_CLASS_ID: VALUENAME = 1i32;
pub const VALUENAME_UNKNOWN: VALUENAME = 0i32;
pub type WINSTATIONINFOCLASS = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WINSTATIONINFORMATIONW {
    pub Reserved2: [u8; 70],
    pub LogonId: u32,
    pub Reserved3: [u8; 1140],
}
impl Default for WINSTATIONINFORMATIONW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WINWATCHNOTIFYPROC = Option<unsafe extern "system" fn(hww: HWINWATCH, hwnd: super::super::Foundation::HWND, code: u32, lparam: super::super::Foundation::LPARAM)>;
pub const WINWATCHNOTIFY_CHANGED: u32 = 4u32;
pub const WINWATCHNOTIFY_CHANGING: u32 = 3u32;
pub const WINWATCHNOTIFY_DESTROY: u32 = 2u32;
pub const WINWATCHNOTIFY_START: u32 = 0u32;
pub const WINWATCHNOTIFY_STOP: u32 = 1u32;
pub const WLDP_CANEXECUTEBUFFER_FN: windows_sys::core::PCSTR = windows_sys::core::s!("WldpCanExecuteBuffer");
pub const WLDP_CANEXECUTEFILE_FN: windows_sys::core::PCSTR = windows_sys::core::s!("WldpCanExecuteFile");
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WLDP_DEVICE_SECURITY_INFORMATION {
    pub UnlockIdSize: u32,
    pub UnlockId: *mut u8,
    pub ManufacturerIDLength: u32,
    pub ManufacturerID: windows_sys::core::PWSTR,
}
impl Default for WLDP_DEVICE_SECURITY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WLDP_DLL: windows_sys::core::PCWSTR = windows_sys::core::w!("WLDP.DLL");
pub type WLDP_EXECUTION_EVALUATION_OPTIONS = i32;
pub const WLDP_EXECUTION_EVALUATION_OPTION_EXECUTE_IN_INTERACTIVE_SESSION: WLDP_EXECUTION_EVALUATION_OPTIONS = 1i32;
pub const WLDP_EXECUTION_EVALUATION_OPTION_NONE: WLDP_EXECUTION_EVALUATION_OPTIONS = 0i32;
pub type WLDP_EXECUTION_POLICY = i32;
pub const WLDP_EXECUTION_POLICY_ALLOWED: WLDP_EXECUTION_POLICY = 1i32;
pub const WLDP_EXECUTION_POLICY_BLOCKED: WLDP_EXECUTION_POLICY = 0i32;
pub const WLDP_EXECUTION_POLICY_REQUIRE_SANDBOX: WLDP_EXECUTION_POLICY = 2i32;
pub const WLDP_FLAGS_SKIPSIGNATUREVALIDATION: u32 = 256u32;
pub const WLDP_GETLOCKDOWNPOLICY_FN: windows_sys::core::PCSTR = windows_sys::core::s!("WldpGetLockdownPolicy");
pub type WLDP_HOST = i32;
pub const WLDP_HOST_CMD: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x5baea1d6_6f1c_488e_8490_347fa5c5067f);
pub const WLDP_HOST_HTML: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb35a71b6_fe56_48d6_9543_2dff0ecded66);
pub type WLDP_HOST_ID = i32;
pub const WLDP_HOST_ID_ALL: WLDP_HOST_ID = 7i32;
pub const WLDP_HOST_ID_GLOBAL: WLDP_HOST_ID = 1i32;
pub const WLDP_HOST_ID_IE: WLDP_HOST_ID = 5i32;
pub const WLDP_HOST_ID_MAX: WLDP_HOST_ID = 8i32;
pub const WLDP_HOST_ID_MSI: WLDP_HOST_ID = 6i32;
pub const WLDP_HOST_ID_POWERSHELL: WLDP_HOST_ID = 4i32;
pub const WLDP_HOST_ID_UNKNOWN: WLDP_HOST_ID = 0i32;
pub const WLDP_HOST_ID_VBA: WLDP_HOST_ID = 2i32;
pub const WLDP_HOST_ID_WSH: WLDP_HOST_ID = 3i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WLDP_HOST_INFORMATION {
    pub dwRevision: u32,
    pub dwHostId: WLDP_HOST_ID,
    pub szSource: windows_sys::core::PCWSTR,
    pub hSource: super::super::Foundation::HANDLE,
}
impl Default for WLDP_HOST_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WLDP_HOST_INFORMATION_REVISION: u32 = 1u32;
pub const WLDP_HOST_JAVASCRIPT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x5629f0d5_1cca_4fed_a1a3_36a8c18d74c0);
pub const WLDP_HOST_MAX: WLDP_HOST = 2i32;
pub const WLDP_HOST_MSI: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x624eb611_6e7e_4eec_9bfe_f0ecdbfcf390);
pub const WLDP_HOST_OTHER: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x626cbec3_e1fa_4227_9800_ed210274cf7c);
pub const WLDP_HOST_POWERSHELL: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x8e9aaa7c_198b_4879_ae41_a50d47ad6458);
pub const WLDP_HOST_PYTHON: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xbfd557ef_2448_42ec_810b_0d9f09352d4a);
pub const WLDP_HOST_RUNDLL32: WLDP_HOST = 0i32;
pub const WLDP_HOST_SVCHOST: WLDP_HOST = 1i32;
pub const WLDP_HOST_WINDOWS_SCRIPT_HOST: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd30b84c5_29ce_4ff3_86ec_a30007a82e49);
pub const WLDP_HOST_XML: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x5594be58_c6bf_4295_82f4_d494d20e3a36);
pub const WLDP_ISAPPAPPROVEDBYPOLICY_FN: windows_sys::core::PCSTR = windows_sys::core::s!("WldpIsAppApprovedByPolicy");
pub const WLDP_ISCLASSINAPPROVEDLIST_FN: windows_sys::core::PCSTR = windows_sys::core::s!("WldpIsClassInApprovedList");
pub const WLDP_ISDYNAMICCODEPOLICYENABLED_FN: windows_sys::core::PCSTR = windows_sys::core::s!("WldpIsDynamicCodePolicyEnabled");
pub const WLDP_ISPRODUCTIONCONFIGURATION_FN: windows_sys::core::PCSTR = windows_sys::core::s!("WldpIsProductionConfiguration");
pub const WLDP_ISWCOSPRODUCTIONCONFIGURATION_FN: windows_sys::core::PCSTR = windows_sys::core::s!("WldpIsWcosProductionConfiguration");
pub type WLDP_KEY = i32;
pub const WLDP_LOCKDOWN_AUDIT_FLAG: u32 = 8u32;
pub const WLDP_LOCKDOWN_CONFIG_CI_AUDIT_FLAG: u32 = 2u32;
pub const WLDP_LOCKDOWN_CONFIG_CI_FLAG: u32 = 1u32;
pub const WLDP_LOCKDOWN_DEFINED_FLAG: u32 = 2147483648u32;
pub const WLDP_LOCKDOWN_EXCLUSION_FLAG: u32 = 16u32;
pub const WLDP_LOCKDOWN_OFF: u32 = 2147483648u32;
pub const WLDP_LOCKDOWN_UMCIENFORCE_FLAG: u32 = 4u32;
pub const WLDP_LOCKDOWN_UNDEFINED: u32 = 0u32;
pub type WLDP_POLICY_SETTING = i32;
pub const WLDP_POLICY_SETTING_AV_PERF_MODE: WLDP_POLICY_SETTING = 1000i32;
pub const WLDP_QUERYDANAMICCODETRUST_FN: windows_sys::core::PCSTR = windows_sys::core::s!("WldpQueryDynamicCodeTrust");
pub const WLDP_QUERYDEVICESECURITYINFORMATION_FN: windows_sys::core::PCSTR = windows_sys::core::s!("WldpQueryDeviceSecurityInformation");
pub const WLDP_QUERYDYNAMICCODETRUST_FN: windows_sys::core::PCSTR = windows_sys::core::s!("WldpQueryDynamicCodeTrust");
pub const WLDP_QUERYPOLICYSETTINGENABLED2_FN: windows_sys::core::PCSTR = windows_sys::core::s!("WldpQueryPolicySettingEnabled2");
pub const WLDP_QUERYPOLICYSETTINGENABLED_FN: windows_sys::core::PCSTR = windows_sys::core::s!("WldpQueryPolicySettingEnabled");
pub const WLDP_QUERYWINDOWSLOCKDOWNMODE_FN: windows_sys::core::PCSTR = windows_sys::core::s!("WldpQueryWindowsLockdownMode");
pub const WLDP_QUERYWINDOWSLOCKDOWNRESTRICTION_FN: windows_sys::core::PCSTR = windows_sys::core::s!("WldpQueryWindowsLockdownRestriction");
pub const WLDP_RESETPRODUCTIONCONFIGURATION_FN: windows_sys::core::PCSTR = windows_sys::core::s!("WldpResetProductionConfiguration");
pub const WLDP_RESETWCOSPRODUCTIONCONFIGURATION_FN: windows_sys::core::PCSTR = windows_sys::core::s!("WldpResetWcosProductionConfiguration");
pub const WLDP_SETDYNAMICCODETRUST_FN: windows_sys::core::PCSTR = windows_sys::core::s!("WldpSetDynamicCodeTrust");
pub const WLDP_SETWINDOWSLOCKDOWNRESTRICTION_FN: windows_sys::core::PCSTR = windows_sys::core::s!("WldpSetWindowsLockdownRestriction");
pub type WLDP_WINDOWS_LOCKDOWN_MODE = i32;
pub const WLDP_WINDOWS_LOCKDOWN_MODE_LOCKED: WLDP_WINDOWS_LOCKDOWN_MODE = 2i32;
pub const WLDP_WINDOWS_LOCKDOWN_MODE_MAX: WLDP_WINDOWS_LOCKDOWN_MODE = 3i32;
pub const WLDP_WINDOWS_LOCKDOWN_MODE_TRIAL: WLDP_WINDOWS_LOCKDOWN_MODE = 1i32;
pub const WLDP_WINDOWS_LOCKDOWN_MODE_UNLOCKED: WLDP_WINDOWS_LOCKDOWN_MODE = 0i32;
pub type WLDP_WINDOWS_LOCKDOWN_RESTRICTION = i32;
pub const WLDP_WINDOWS_LOCKDOWN_RESTRICTION_MAX: WLDP_WINDOWS_LOCKDOWN_RESTRICTION = 3i32;
pub const WLDP_WINDOWS_LOCKDOWN_RESTRICTION_NONE: WLDP_WINDOWS_LOCKDOWN_RESTRICTION = 0i32;
pub const WLDP_WINDOWS_LOCKDOWN_RESTRICTION_NOUNLOCK: WLDP_WINDOWS_LOCKDOWN_RESTRICTION = 1i32;
pub const WLDP_WINDOWS_LOCKDOWN_RESTRICTION_NOUNLOCK_PERMANENT: WLDP_WINDOWS_LOCKDOWN_RESTRICTION = 2i32;
pub const WM_CONVERTREQUEST: u32 = 266u32;
pub const WM_CONVERTRESULT: u32 = 267u32;
pub const WM_IMEKEYDOWN: u32 = 656u32;
pub const WM_IMEKEYUP: u32 = 657u32;
pub const WM_IME_REPORT: u32 = 640u32;
pub const WM_INTERIM: u32 = 268u32;
pub const WM_WNT_CONVERTREQUESTEX: u32 = 265u32;
pub const WinStationInformation: WINSTATIONINFOCLASS = 8i32;
